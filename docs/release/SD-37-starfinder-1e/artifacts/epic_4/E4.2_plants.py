#!/usr/bin/env python3
"""SD-37 E4.2: planted mutations against the engine's `sf_seed` tests (every one must turn red).

Converter plants (each switches ONE of E4.2's three converter changes off; the package is
regenerated with `--dump` into a scratch root and read by the engine through CODEX_REPO_ROOT, so
the committed package is untouched):

- C1 the Internal selection hop off (`convert.rs`): races stop granting their default traits --
  Scrounger (Envoy Engineering/Stealth/Survival) and Flat Affect (Technomancer Sense Motive).
- C2 the zero-DEFINE global declarations off (`mod.rs`): CS_First_<Skill> and
  MysticChannelSkillBonus fold to 0 -- theme knowledge +1 (Soldier Athletics, Mystic
  Mysticism, Envoy Culture) and channel skill (Mystic Perception, Sense Motive).
- C3 the Internal ABILITYPOOL members off (`pool_option.rs`): the Scholar's chosen skill is not a
  package record -- Technomancer Physical Science.

Engine plants (on `sf_defense.rs` / `sf_skills.rs`):

- E1 max-Dex cap dropped (Soldier EAC/KAC), E2 armour check penalty dropped, E3 trained class
  skill +3 dropped, E4 trained-only gate dropped (an untrained trained-only skill gets a total),
  E5 the held-row fold dropped (every racial/theme/class-feature bonus lost).

Every planted file is restored byte for byte (sha256 checked) and the restored tree must PASS.
Run from the repo root with CARGO_TARGET_DIR set; SCRATCH = a scratch dir outside the repo.
Exit 0 only when every plant FAILs and the restored tree PASSes."""
import hashlib, os, shutil, subprocess, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[4]
SCRATCH = Path(os.environ["SCRATCH"])
TEST = ["cargo", "test", "--locked", "-j", "8", "--lib", "sf_seed", "--", "--test-threads=8"]
DUMP = ["cargo", "run", "--locked", "-j", "8", "-q", "-p", "codex-ingest", "--bin", "sheet_rule_convert", "--",
        "--system", "starfinder-1e", "--dump"]
SR = "crates/codex-ingest/src/pcgen_import/sheet_rule/"
CONVERTER_PLANTS = [
    ("C1", SR + "convert.rs", "&& let Some(inner) = internal_hop_grants(ctx.tree, t)",
     "&& let Some(inner) = internal_hop_grants(ctx.tree, t).filter(|_| false)"),
    ("C2", SR + "mod.rs", "if declared_by.is_empty() && always_held::defined_at_zero_only_on(",
     "if false && declared_by.is_empty() && always_held::defined_at_zero_only_on("),
    ("C3", SR + "pool_option.rs", "if tree.system == codex::rules_core::game_system::GameSystem::Starfinder1e {",
     "if false && tree.system == codex::rules_core::game_system::GameSystem::Starfinder1e {"),
]
PC = "src/rules_core/pilot_compute/"
ENGINE_PLANTS = [
    ("E1", PC + "sf_defense.rs", "value: dex.min(cap), source: a.id.clone() }", "value: dex, source: a.id.clone() }"),
    ("E2", PC + "sf_skills.rs", "if record.tags.iter().any(|t| t == \"ACHECK\") && acp != 0 {",
     "if false && record.tags.iter().any(|t| t == \"ACHECK\") && acp != 0 {"),
    ("E3", PC + "sf_skills.rs", "if class_skill && ranks >= 1 {", "if false && class_skill && ranks >= 1 {"),
    ("E4", PC + "sf_skills.rs", "if matches!(open, Gate::Exclude) {", "if false && matches!(open, Gate::Exclude) {"),
    ("E5", PC + "sf_skills.rs", "terms.extend(fold(held_rows(package, sf, &wants, &mut not_folded)));",
     "let _ = fold(held_rows(package, sf, &wants, &mut not_folded));"),
]


def run_test(env_root=None):
    env = dict(os.environ)
    env.pop("CODEX_REPO_ROOT", None)
    if env_root:
        env["CODEX_REPO_ROOT"] = str(env_root)
    p = subprocess.run(TEST, cwd=REPO, capture_output=True, text=True, env=env)
    out = p.stdout + p.stderr
    why = [l.strip()[:220] for l in out.splitlines() if "engine " in l or "Refusal" in l or "mismatch" in l][:8]
    return ("PASS" if p.returncode == 0 else "FAIL"), p.returncode, why


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def plant(path, old, new):
    src = path.read_text()
    assert src.count(old) == 1, f"{path}: plant anchor not found exactly once: {old!r}"
    path.write_text(src.replace(old, new))


def main():
    files = sorted({REPO / f for _, f, _, _ in CONVERTER_PLANTS + ENGINE_PLANTS})
    saved = {f: f.read_bytes() for f in files}
    hashes = {f: sha(f) for f in files}
    report = [f"{f.relative_to(REPO)} sha256 {h}" for f, h in hashes.items()] + ["test: " + " ".join(TEST)]
    verdicts = {}
    try:
        for pid, rel, old, new in CONVERTER_PLANTS:
            path = REPO / rel
            plant(path, old, new)
            root = SCRATCH / f"e42-{pid}"
            shutil.rmtree(root, ignore_errors=True)
            d = subprocess.run(DUMP + [str(root / "data/starfinder-1e/sheet_rules")], cwd=REPO, capture_output=True, text=True)
            path.write_bytes(saved[path])
            if d.returncode != 0:
                report.append(f"{pid} dump failed (exit {d.returncode}): {d.stderr[-300:]}")
                verdicts[pid] = "DUMP-FAILED"
                continue
            v, rc, why = run_test(root)
            report.append(f"{pid} on {rel} ({old!r} -> {new!r}) -> sf_seed {v} (exit {rc})")
            report.extend("    " + w for w in why)
            verdicts[pid] = v
            shutil.rmtree(root, ignore_errors=True)
        for pid, rel, old, new in ENGINE_PLANTS:
            path = REPO / rel
            plant(path, old, new)
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
    ids = [p[0] for p in CONVERTER_PLANTS + ENGINE_PLANTS]
    ok = all(verdicts.get(k) == "FAIL" for k in ids) and v == "PASS" and restored_ok
    report.append("verdict=" + ("PASS" if ok else "FAIL") + " " + " ".join(f"{k}={verdicts.get(k)}" for k in ids))
    print("\n".join(report))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
