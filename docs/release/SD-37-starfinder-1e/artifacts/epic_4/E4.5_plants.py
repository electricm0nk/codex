#!/usr/bin/env python3
"""SD-37 E4.5: planted mutations (every one must turn red).

Engine plants against `cargo test --lib sf_seed`, each switching off ONE term:
- L1 the worn armour not carried (credits spent and bulk lose the armour).
- L2 light items counted 1 each, not 10 to a bulk.
- L3 an item's quantity ignored in its price (2 batteries priced as 1).
- L4 Table 11-5 read one row late (starting credits of the next level).
- L5 the unencumbered limit is the Strength score, not half of it.
Package plants (`data/starfinder-1e/sheet_rules`), against the same tests:
- P1 the Defiance Series, Squad price 1220 -> 1200 (Soldier credits spent/remaining).
- P2 the basic medkit's bulk 1 -> L (Mystic and Technomancer bulk 1 -> 0).
Converter plant (`crates/codex-ingest`), against `sheet_rule_convert --system starfinder-1e
--check` (the committed package must be the converter's output):
- C1 the Starfinder `COST:` -> `StatBlock "Price"` arm off.

Every planted file is restored byte for byte (sha256 checked) and the restored tree must PASS
both commands. Run from the repo root with CARGO_TARGET_DIR set. Exit 0 only when every plant
FAILs and the restored tree PASSes."""
import hashlib
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[4]
TEST = ["cargo", "test", "--locked", "-j", "8", "--lib", "sf_seed", "--", "--test-threads=8"]
CHECK = ["cargo", "run", "--locked", "-j", "8", "-q", "-p", "codex-ingest", "--bin", "sheet_rule_convert", "--", "--system", "starfinder-1e", "--check"]
PC = "src/rules_core/pilot_compute/sf_loadout.rs"
PKG = "data/starfinder-1e/sheet_rules/"
CONV = "crates/codex-ingest/src/pcgen_import/sheet_rule/"
PLANTS = [
    ("L1", TEST, PC, "if let Some(armor) = &build.armor {", "if let Some(armor) = build.armor.as_ref().filter(|_| false) {"),
    ("L2", TEST, PC, "value: light / 10,", "value: light,"),
    ("L3", TEST, PC, "value: p * i64::from(*quantity), source: rule.id.clone() });", "value: p, source: rule.id.clone() });"),
    ("L4", TEST, "src/rules_core/money.rs", "SF_WEALTH_BY_LEVEL_CREDITS.get(l - 1)", "SF_WEALTH_BY_LEVEL_CREDITS.get(l)"),
    ("L5", TEST, "src/rules_core/encumbrance.rs", "BulkLimits { unencumbered_max: strength / 2, overburdened_above: strength }",
     "BulkLimits { unencumbered_max: strength, overburdened_above: strength }"),
    ("P1", TEST, PKG + "core/equipment/defiance_series_squad.json",
     '{"StatBlock":"Price"},"pieces":[{"Text":"1220"}]', '{"StatBlock":"Price"},"pieces":[{"Text":"1200"}]'),
    ("P2", TEST, PKG + "core/equipment/medkit_basic.json", '"Text":"Bulk: 1"', '"Text":"Bulk: L"'),
    ("C1", CHECK, CONV + "convert.rs",
     '"COST" if ctx.tree.system == GameSystem::Starfinder1e && ctx.record.kind == "equipment" =>',
     '"COST" if false && ctx.tree.system == GameSystem::Starfinder1e && ctx.record.kind == "equipment" =>'),
]


def run(cmd):
    p = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True)
    out = p.stdout + p.stderr
    why = [l.strip()[:220] for l in out.splitlines()
           if "engine " in l or "left:" in l or "right:" in l or "panicked" in l or "test result" in l or "verdict=" in l or "mismatch" in l][:8]
    return ("PASS" if p.returncode == 0 else "FAIL"), p.returncode, why


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    files = sorted({REPO / f for _, _, f, _, _ in PLANTS})
    saved = {f: f.read_bytes() for f in files}
    hashes = {f: sha(f) for f in files}
    report = [f"{f.relative_to(REPO)} sha256 {h}" for f, h in hashes.items()]
    report += ["test: " + " ".join(TEST), "check: " + " ".join(CHECK)]
    verdicts = {}
    try:
        for pid, cmd, rel, old, new in PLANTS:
            path = REPO / rel
            src = path.read_text()
            assert src.count(old) == 1, f"{pid}: anchor not found exactly once: {old!r}"
            path.write_text(src.replace(old, new))
            v, rc, why = run(cmd)
            path.write_bytes(saved[path])
            report.append(f"{pid} on {rel} -> {'sf_seed' if cmd is TEST else 'convert --check'} {v} (exit {rc})")
            report.extend("    " + w for w in why)
            verdicts[pid] = v
    finally:
        for f, b in saved.items():
            f.write_bytes(b)
    restored_ok = all(sha(f) == h for f, h in hashes.items())
    vt, rct, _ = run(TEST)
    vc, rcc, _ = run(CHECK)
    report.append(f"restored (sha256 {'identical' if restored_ok else 'DIFFERENT'}) -> sf_seed {vt} (exit {rct}); convert --check {vc} (exit {rcc})")
    ids = [p[0] for p in PLANTS]
    ok = all(verdicts.get(k) == "FAIL" for k in ids) and vt == "PASS" and vc == "PASS" and restored_ok
    report.append("verdict=" + ("PASS" if ok else "FAIL") + " " + " ".join(f"{k}={verdicts.get(k)}" for k in ids))
    print("\n".join(report))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
