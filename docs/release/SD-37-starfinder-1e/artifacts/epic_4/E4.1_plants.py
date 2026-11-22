#!/usr/bin/env python3
"""SD-37 E4.1: re-plant M1..M4 (decisions.md §8) against the engine's `sf_seed` test.

E3.MC found M2..M4 load-bearing only on E3.3's fixture model: the converted package held no HD,
RaceHP or Con term. E4.1 named a live source for each, so each mutation is planted where its term
now lives, and `cargo test --lib sf_seed` must go red:

- M1 swap ALTHP<->CURRENTMAX, M4 drop RaceHP: planted on the mapping table THE CONVERTER reads
  (`sf_mapping_mutations.py`'s m1/m4), package regenerated with `--dump` into a scratch root and
  read by the engine through CODEX_REPO_ROOT (the committed package is untouched).
- M2 drop HD, M3 route CON*TL into Hit Points: the HD term is the class's `Hit die` row read by
  `sf_chassis.rs`, and Con x level is its Stamina system rule, so both are planted on
  `src/rules_core/pilot_compute/sf_chassis.rs` (restored byte for byte, sha256 checked).
- For completeness, M2/M3 are ALSO planted on the table (they do not reach the package; reported).

Run from the repo root with CARGO_TARGET_DIR set; SCRATCH = a scratch dir outside the repo.
Exit 0 only when M1..M4 (live-source plants) each FAIL, and the restored tree PASSes."""
import hashlib, importlib.util, json, os, shutil, subprocess, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[4]
SCRATCH = Path(os.environ["SCRATCH"])
spec = importlib.util.spec_from_file_location("m", HERE.parent / "epic_3" / "token-mapping" / "sf_mapping_mutations.py")
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
TABLE = HERE.parent / "epic_3" / "token-mapping" / "sf-mapping-table.v1.json"
ENGINE = REPO / "src/rules_core/pilot_compute/sf_chassis.rs"
TEST = ["cargo", "test", "--locked", "-j", "8", "--lib", "sf_seed", "--", "--test-threads=8"]
DUMP = ["cargo", "run", "--locked", "-j", "8", "-q", "-p", "codex-ingest", "--bin", "sheet_rule_convert", "--",
        "--system", "starfinder-1e", "--dump"]
ENGINE_PLANTS = {
    "M2": ("value: hit_die(principal)? * levels,", "value: 0 * hit_die(principal)? * levels,"),
    "M3": ('stamina.push(SfTerm { label: "Constitution modifier × level".into()',
           'hp.push(SfTerm { label: "Constitution modifier × level".into()'),
}


def test(env_root=None):
    env = dict(os.environ)
    env.pop("CODEX_REPO_ROOT", None)
    if env_root:
        env["CODEX_REPO_ROOT"] = str(env_root)
    p = subprocess.run(TEST, cwd=REPO, capture_output=True, text=True, env=env)
    out = p.stdout + p.stderr
    why = [l.strip() for l in out.splitlines() if "engine " in l or "SfChassisRefusal" in l][:4]
    return ("PASS" if p.returncode == 0 else "FAIL"), p.returncode, why


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    t0, e0 = TABLE.read_bytes(), ENGINE.read_bytes()
    h_t, h_e = sha(TABLE), sha(ENGINE)
    report, live = [f"table sha256 {h_t}", f"engine sha256 {h_e}", "test: " + " ".join(TEST)], {}
    try:
        for mid, desc, fn in m.MUTATIONS:
            t = json.loads(t0); fn(t)
            TABLE.write_text(json.dumps(t, indent=2, ensure_ascii=False) + "\n")
            root = SCRATCH / f"e41-{mid}"
            shutil.rmtree(root, ignore_errors=True)
            pkg = root / "data/starfinder-1e/sheet_rules"
            d = subprocess.run(DUMP + [str(pkg)], cwd=REPO, capture_output=True, text=True)
            TABLE.write_bytes(t0)
            if d.returncode != 0:
                report.append(f"{mid} table plant: dump failed (exit {d.returncode}): {d.stderr[-300:]}")
                continue
            v, rc, why = test(root)
            report.append(f"{mid} ({desc}) on the converter's table -> sf_seed {v} (exit {rc})")
            report.extend("    " + w for w in why)
            if mid in ("M1", "M4"):
                live[mid] = v
        for mid, (old, new) in ENGINE_PLANTS.items():
            src = e0.decode()
            assert src.count(old) == 1, f"{mid}: plant anchor not found exactly once"
            ENGINE.write_text(src.replace(old, new))
            v, rc, why = test()
            ENGINE.write_bytes(e0)
            report.append(f"{mid} on sf_chassis.rs ({old!r} -> {new!r}) -> sf_seed {v} (exit {rc})")
            report.extend("    " + w for w in why)
            live[mid] = v
    finally:
        TABLE.write_bytes(t0); ENGINE.write_bytes(e0)
    restored_ok = sha(TABLE) == h_t and sha(ENGINE) == h_e
    v, rc, _ = test()
    report.append(f"restored (table and engine sha256 {'identical' if restored_ok else 'DIFFERENT'}) -> sf_seed {v} (exit {rc})")
    ok = all(live.get(k) == "FAIL" for k in ("M1", "M2", "M3", "M4")) and v == "PASS" and restored_ok
    report.append("verdict=" + ("PASS" if ok else "FAIL") + " " + " ".join(f"{k}={live.get(k)}" for k in ("M1", "M2", "M3", "M4")))
    print("\n".join(report))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
