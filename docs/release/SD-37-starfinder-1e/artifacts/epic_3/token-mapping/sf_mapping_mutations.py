#!/usr/bin/env python3
"""Planted mutations M1–M4 on the SF mapping table (`decisions.md §8`, SD-37 E3.3 acceptance).

Each mutation edits sf-mapping-table.v1.json IN PLACE, runs the seed-fixture test, and restores
the file byte for byte (checked with a sha256). Pass = every mutation turns the seed fixtures red
(cargo exits non-zero and the failure names a seed field) and the restored table runs green.

  M1  swap ALTHP <-> CURRENTMAX: every hit_points term whose token is BONUS:HP|CURRENTMAX… moves
      to stamina and every stamina term whose token is BONUS:HP|ALTHP… moves to hit_points
  M2  drop the HD term from Hit Points
  M3  route CON*TL into Hit Points instead of Stamina
  M4  drop RaceHP from Hit Points

Usage (repo root, CARGO_TARGET_DIR set per the dispatch prefix):
  python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf_mapping_mutations.py [--log <file>]
Exit 0 only when M1..M4 -> FAIL each and restored -> PASS.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[5]
TABLE = HERE / "sf-mapping-table.v1.json"
TEST = ["cargo", "test", "--locked", "-j", "8", "-p", "codex-ingest", "--test", "sf_mapping_table",
        "seed_fixtures_match_the_srd_hand_values_and_the_oracle", "--", "--test-threads=8", "--exact"]


def rows(t):
    return {r["id"]: r for r in t["rows"]}


def m1(t):
    r = rows(t)
    hp, sp = r["hit_points"], r["stamina"]
    to_sp = [x for x in hp["terms"] if x["token"].startswith("BONUS:HP|CURRENTMAX")]
    to_hp = [x for x in sp["terms"] if x["token"].startswith("BONUS:HP|ALTHP")]
    assert to_sp and to_hp, "M1 found nothing to swap"
    hp["terms"] = [x for x in hp["terms"] if x not in to_sp] + to_hp
    sp["terms"] = [x for x in sp["terms"] if x not in to_hp] + to_sp


def m2(t):
    hp = rows(t)["hit_points"]
    n = len(hp["terms"])
    hp["terms"] = [x for x in hp["terms"] if x["shape"] != "hit_die_per_level"]
    assert len(hp["terms"]) == n - 1, "M2 found no HD term"


def m3(t):
    r = rows(t)
    con = [x for x in r["stamina"]["terms"] if x["token"].endswith("|CON*TL")]
    assert len(con) == 1, "M3 found no CON*TL term"
    r["stamina"]["terms"].remove(con[0])
    r["hit_points"]["terms"].append(con[0])


def m4(t):
    hp = rows(t)["hit_points"]
    n = len(hp["terms"])
    hp["terms"] = [x for x in hp["terms"] if not x["token"].endswith("|RaceHP")]
    assert len(hp["terms"]) == n - 1, "M4 found no RaceHP term"


MUTATIONS = [("M1", "swap ALTHP<->CURRENTMAX", m1), ("M2", "drop HD from Hit Points", m2),
             ("M3", "route CON*TL into Hit Points", m3), ("M4", "drop RaceHP from Hit Points", m4)]


def run_test():
    p = subprocess.run(TEST, cwd=REPO, capture_output=True, text=True)
    out = p.stdout + p.stderr
    lines = [l for l in out.splitlines() if " mapping gives " in l or "refused:" in l or "no row claims" in l]
    passed = p.returncode == 0 and "1 passed" in out
    return passed, p.returncode, lines, out


def main():
    log_path = Path(sys.argv[sys.argv.index("--log") + 1]) if "--log" in sys.argv else None
    original = TABLE.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    report = [f"table sha256 before: {digest}", f"test: {' '.join(TEST)}"]
    verdicts = {}
    try:
        for mid, desc, fn in MUTATIONS:
            t = json.loads(original)
            fn(t)
            TABLE.write_text(json.dumps(t, indent=2, ensure_ascii=False) + "\n")
            passed, code, lines, _ = run_test()
            verdicts[mid] = "PASS" if passed else "FAIL"
            report.append(f"{mid} ({desc}) -> {verdicts[mid]} (cargo exit {code}; {len(lines)} seed-field disagreements)")
            report.extend(f"    {l.strip()}" for l in lines)
            TABLE.write_bytes(original)
    finally:
        TABLE.write_bytes(original)
    restored = hashlib.sha256(TABLE.read_bytes()).hexdigest()
    passed, code, lines, out = run_test()
    verdicts["restored"] = "PASS" if passed else "FAIL"
    report.append(f"restored -> {verdicts['restored']} (cargo exit {code}); table sha256 after: {restored} "
                  f"({'identical' if restored == digest else 'DIFFERENT'})")
    ok = all(verdicts[m] == "FAIL" for m, _, _ in MUTATIONS) and verdicts["restored"] == "PASS" and restored == digest
    report.append("verdict=" + ("PASS" if ok else "FAIL") + " " + " ".join(f"{k}={v}" for k, v in verdicts.items()))
    text = "\n".join(report) + "\n"
    print(text, end="")
    if log_path:
        log_path.write_text(text)
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
