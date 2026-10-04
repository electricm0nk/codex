#!/usr/bin/env python3
"""SD-37 E4.4: planted mutations (every one must turn red).

Engine plants (`sf_spells.rs`) against `cargo test --lib sf_seed`, each switching off ONE term:
- S1 bonus spells dropped (Mystic/Technomancer 1st 5 -> 4, 2nd 3 -> 2).
- S2 the key ability modifier dropped from the DC (15/16/17 -> 11/12/13).
- S3 the held `SpellDc` fold dropped (Spell Focus +1 gone).
- S4 every spell level castable (3rd-6th get totals).
Package plants (the converted rows the engine reads; `data/starfinder-1e/sheet_rules`), against
the same tests:
- P1 the Empath "Connection Spell - 2" detect thoughts line worth 0 (Mystic known 1st 5 -> 4).
- P2 the mystic 1st-level spells-per-day row starts at 3, not 2 (Mystic per day 1st 5 -> 6).
- P3 Spell Focus's DC row scoped to another class (no +1 on a mystic or technomancer DC).
Converter plants (`crates/codex-ingest`), against `sheet_rule_convert --system starfinder-1e
--check` (the committed package must be the converter's output, so switching off a lowering
must make the check fail):
- C1 the ownerless-`CL` = character level arm off (Spell Focus's DC row degrades back to words).
- C2 the class spell-progression lowering off.
- C3 the connection-spell hop off.

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
PC = "src/rules_core/pilot_compute/sf_spells.rs"
PKG = "data/starfinder-1e/sheet_rules/"
CONV = "crates/codex-ingest/src/pcgen_import/sheet_rule/"
PLANTS = [
    ("S1", TEST, PC, "let bonus = bonus_spells(score, level);", "let bonus = 0 * bonus_spells(score, level);"),
    ("S2", TEST, PC, "value: modifier, source: SRD_DC.into() },", "value: 0 * modifier, source: SRD_DC.into() },"),
    ("S3", TEST, PC, "dc_terms.extend(fold(held_rows(", "drop(fold(held_rows("),
    ("S4", TEST, PC, "let castable = own_row_admits(package, sf, class_id, &known_target);", "let castable = own_row_admits(package, sf, class_id, &known_target) || true;"),
    ("P1", TEST, PKG + "core/ability/empath.json",
     '"id":"core:ability:empath#spell_known_empath_connection_spell_2_1_detect_thoughts","label":"Empath Connection Spell - 2: detect thoughts (level 1 Mystic spell known)","value":{"Number":{"Const":1}}',
     '"id":"core:ability:empath#spell_known_empath_connection_spell_2_1_detect_thoughts","label":"Empath Connection Spell - 2: detect thoughts (level 1 Mystic spell known)","value":{"Number":{"Const":0}}'),
    ("P2", TEST, PKG + "core/class/mystic.json",
     '"label":"Mystic (level 1 spells per day)","value":{"Number":{"Sum":[{"Const":2}',
     '"label":"Mystic (level 1 spells per day)","value":{"Number":{"Sum":[{"Const":3}'),
    ("P3", TEST, PKG + "core/feat/spell_focus.json", '"target":{"SpellDc":"All"}', '"target":{"SpellDc":{"Class":"envoy"}}'),
    ("C1", CHECK, CONV + "formula.rs",
     "None if ctx.tree.system == codex::rules_core::game_system::GameSystem::Starfinder1e => Ok(Expr::Level),",
     "None if false && ctx.tree.system == codex::rules_core::game_system::GameSystem::Starfinder1e => Ok(Expr::Level),"),
    ("C2", CHECK, CONV + "convert.rs",
     'if ctx.tree.system == GameSystem::Starfinder1e && record.kind == "class" {\n        let class = super::ctx::own_class_id(record);',
     'if false && ctx.tree.system == GameSystem::Starfinder1e && record.kind == "class" {\n        let class = super::ctx::own_class_id(record);'),
    ("C3", CHECK, CONV + "convert.rs",
     "&& let Some(spells) = internal_spell_known_grants(ctx.tree, t)",
     "&& let Some(spells) = internal_spell_known_grants(ctx.tree, t).filter(|_| false)"),
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
