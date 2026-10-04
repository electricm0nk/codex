#!/usr/bin/env python3
"""SD-37 E4.3: planted mutations against the engine's `sf_seed` tests (every one must turn red).

Engine plants (on `sf_abilities.rs` / `sf_defense.rs`), each switching off ONE term of the
ability-score total:

- A1 the 17 rule dropped (every increase is +2): Mystic Wis 18 -> 20, Technomancer Int 18 -> 20.
- A2 the ability increases dropped: Mystic/Technomancer final = creation scores.
- A3 the held-row fold dropped (no race, subrace, theme or chosen racial term).
- A4 the chosen-ability mapping dropped (a `CHOOSE:PCSTAT` pick read as a skill): Soldier Str 14.
- A5 the point budget raised to 11 (an 11-point buy is accepted).
- A6 the creation cap raised to 19 (a 19 at creation is accepted).
- A7 the point-buy term dropped.

Every planted file is restored byte for byte (sha256 checked) and the restored tree must PASS.
Run from the repo root with CARGO_TARGET_DIR set. Exit 0 only when every plant FAILs and the
restored tree PASSes."""
import hashlib, subprocess, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[4]
TEST = ["cargo", "test", "--locked", "-j", "8", "--lib", "sf_seed", "--", "--test-threads=8"]
PC = "src/rules_core/pilot_compute/"
PLANTS = [
    ("A1", PC + "sf_abilities.rs", "if current[i] >= 17 {", "if false && current[i] >= 17 {"),
    ("A2", PC + "sf_abilities.rs", "for (at, chosen) in &abilities.increases {", "for (at, chosen) in abilities.increases.iter().filter(|_| false) {"),
    ("A3", PC + "sf_abilities.rs", "terms[i].extend(fold(held_rows(package, &sf, &|t| *t == BonusTarget::Ability(a), &mut not_folded)));",
     "let _ = fold(held_rows(package, &sf, &|t| *t == BonusTarget::Ability(a), &mut not_folded));"),
    ("A4", PC + "sf_defense.rs", "Some(a) => BonusTarget::Ability(a),", "Some(_) => BonusTarget::Skill(option.clone()),"),
    ("A5", PC + "sf_abilities.rs", "pub const POINT_BUY_BUDGET: i64 = 10;", "pub const POINT_BUY_BUDGET: i64 = 11;"),
    ("A6", PC + "sf_abilities.rs", "pub const MAX_SCORE_AT_CREATION: i64 = 18;", "pub const MAX_SCORE_AT_CREATION: i64 = 19;"),
    ("A7", PC + "sf_abilities.rs", "if abilities.point_buy[i] != 0 {", "if false && abilities.point_buy[i] != 0 {"),
]


def run_test():
    p = subprocess.run(TEST, cwd=REPO, capture_output=True, text=True)
    out = p.stdout + p.stderr
    why = [l.strip()[:220] for l in out.splitlines() if "engine " in l or "left:" in l or "right:" in l or "panicked" in l or "test result" in l][:8]
    return ("PASS" if p.returncode == 0 else "FAIL"), p.returncode, why


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    files = sorted({REPO / f for _, f, _, _ in PLANTS})
    saved = {f: f.read_bytes() for f in files}
    hashes = {f: sha(f) for f in files}
    report = [f"{f.relative_to(REPO)} sha256 {h}" for f, h in hashes.items()] + ["test: " + " ".join(TEST)]
    verdicts = {}
    try:
        for pid, rel, old, new in PLANTS:
            path = REPO / rel
            src = path.read_text()
            assert src.count(old) == 1, f"{pid}: anchor not found exactly once: {old!r}"
            path.write_text(src.replace(old, new))
            v, rc, why = run_test()
            path.write_bytes(saved[path])
            report.append(f"{pid} on {rel} ({old!r} -> {new!r}) -> sf_seed {v} (exit {rc})")
            report.extend("    " + w for w in why)
            verdicts[pid] = v
    finally:
        for f, b in saved.items():
            f.write_bytes(b)
    restored_ok = all(sha(f) == h for f, h in hashes.items())
    v, rc, _ = run_test()
    report.append(f"restored (sha256 {'identical' if restored_ok else 'DIFFERENT'}) -> sf_seed {v} (exit {rc})")
    ids = [p[0] for p in PLANTS]
    ok = all(verdicts.get(k) == "FAIL" for k in ids) and v == "PASS" and restored_ok
    report.append("verdict=" + ("PASS" if ok else "FAIL") + " " + " ".join(f"{k}={verdicts.get(k)}" for k in ids))
    print("\n".join(report))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
