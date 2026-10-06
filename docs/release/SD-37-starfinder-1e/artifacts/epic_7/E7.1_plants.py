#!/usr/bin/env python3
"""E7.1 planted mutations: each must turn its check red; every file is restored byte-identical.

usage (repo root; CARGO_TARGET_DIR and PATH set; E71_BASE_SHEET_RULES = the card base's
data/starfinder-1e/sheet_rules, extracted): python3 E7.1_plants.py

Converter / package (the global Default, `always_held::convert_unconverted_globals`):
  P1 Effective LVL loses the global's contribution (package)  -> root sf_attack red (rifle damage 0)
  P2 the global record is no longer always held (package)     -> codex-ingest sf_global_default red
  P3 a var table moves outside the classes (package)           -> E7.1_delta.py exit 1
Attack reader (`sf_attack.rs`):
  P4 the operative property is ignored                         -> root sf_attack red
  P5 the non-proficiency -4 is dropped                         -> root sf_attack red
  P6 melee damage loses the Strength modifier                  -> desktop sf_oracle_parity red
Parity gate (`sf_oracle_parity.rs`, `scripts/oracle_harness/sf_parity/`):
  P7 one oracle value moves (sf_soldier_3 hp 25 -> 26)          -> desktop sf_oracle_parity red (unexplained)
  P8 one ledger row's values move (stale + unexplained)         -> desktop sf_oracle_parity red
  P9 a build's identity moves in the oracle (a skill's ranks)   -> desktop sf_oracle_parity red (identity)
  P10 the not-in-oracle list loses a row                        -> desktop sf_oracle_parity red
Sheet (`starfinderSheetModel.ts`):
  P11 the Weapons table is not laid out                         -> frontend starfinderSheet.test.ts red
"""
import hashlib, json, os, subprocess, sys

PKG = "data/starfinder-1e/sheet_rules"
A = "docs/release/SD-37-starfinder-1e/artifacts/epic_7"
ORACLE = "scripts/oracle_harness/sf_parity"
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
def edit_text(path, old, new):
    s = open(path, encoding="utf-8").read(); assert s.count(old) == 1, (path, old)
    open(path, "w", encoding="utf-8").write(s.replace(old, new))
def edit_var(path, fn):
    v = json.load(open(path, encoding="utf-8")); fn(v)
    open(path, "w", encoding="utf-8").write(json.dumps(v, ensure_ascii=False, separators=(",", ":")))
def var_path(label):
    for n in sorted(os.listdir(f"{PKG}/_vars")):
        p = f"{PKG}/_vars/{n}"
        if json.load(open(p))["label"] == label: return p
    raise SystemExit(f"no var {label}")
def run(cmd, cwd=None):
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True).returncode
def root_attack(): return run(["cargo", "test", "--locked", "-j", "8", "--lib", "sf_attack", "--", "--test-threads=8"])
def ingest_default(): return run(["cargo", "test", "--locked", "-j", "8", "-p", "codex-ingest", "--test", "sf_global_default", "--", "--test-threads=8"])
def delta(): return run(["python3", f"{A}/E7.1_delta.py", PKG, os.environ["E71_BASE_SHEET_RULES"]])
def parity(): return run(["cargo", "test", "--locked", "-j", "8", "sf_oracle_parity", "--", "--test-threads=8"], cwd="apps/desktop/src-tauri")
def sheet(): return run(["./node_modules/.bin/tsx", "src/characterHub/starfinderSheet.test.ts"], cwd="apps/desktop")

ATTACK = "src/rules_core/pilot_compute/sf_attack.rs"
EFF = var_path("Effective LVL")
plants = [
    ("P1 Effective LVL loses the global's contribution", EFF,
     lambda p: edit_var(p, lambda v: v.__setitem__("contributions", [c for c in v["contributions"] if c["rule_id"] != "core:ability:default"])), root_attack),
    ("P2 the global record is not always held", f"{PKG}/core/ability/default.json",
     lambda p: edit_text(p, '"always_held":true', '"always_held":false'), ingest_default),
    ("P3 a var table moves outside the classes", EFF,
     lambda p: edit_var(p, lambda v: v.__setitem__("label", "Effective Level (planted)")), delta),
    ("P4 the operative property is ignored", ATTACK,
     lambda p: edit_text(p, "SfAttackKind::Melee if operative && dex_mod > str_mod =>", "SfAttackKind::Melee if false && operative && dex_mod > str_mod =>"), root_attack),
    ("P5 the non-proficiency -4 is dropped", ATTACK,
     lambda p: edit_text(p, "        if !held_attack_row {\n", "        if false && !held_attack_row {\n"), root_attack),
    ("P6 melee damage loses the Strength modifier", ATTACK,
     lambda p: edit_text(p, 'SfAttackKind::Melee => damage_terms.push(SfTerm { label: "Strength modifier".into(), value: str_mod,', 'SfAttackKind::Melee => damage_terms.push(SfTerm { label: "Strength modifier".into(), value: 0,'), parity),
    ("P7 one oracle value moves", f"{ORACLE}/sf_soldier_3.oracle.txt", lambda p: edit_text(p, "\nhp=25\n", "\nhp=26\n"), parity),
    ("P8 one ledger row's values move", f"{ORACLE}/explained.tsv",
     lambda p: edit_text(p, "SF-Soldier-3\tbulk\t4\t4.3\t", "SF-Soldier-3\tbulk\t5\t4.3\t"), parity),
    ("P9 a build's identity moves in the oracle", f"{ORACLE}/sf_envoy_1.oracle.txt",
     lambda p: edit_text(p, "skill.Bluff=", "skill.Bluff=99|ranks=2.0|untrained=Y\nskill.Unused="), parity),
    ("P10 the not-in-oracle list loses a row", f"{ORACLE}/not_in_oracle.tsv",
     lambda p: edit_text(p, "SF-Soldier-3\trow.sf.credits.starting\n", ""), parity),
    ("P11 the Weapons table is not laid out", "apps/desktop/src/characterHub/starfinderSheetModel.ts",
     lambda p: edit_text(p, "    id: 'weapons',\n    title: 'Weapons',", "    id: 'weapons',\n    title: 'Arms',"), sheet),
]
only = [a for a in sys.argv[1:] if a.startswith("P")]
if only:
    plants = [p for p in plants if p[0].split()[0] in only]
red = 0
for name, path, plant, check in plants:
    before = open(path, "rb").read(); h = sha(path)
    plant(path)
    try:
        code = check()
    finally:
        open(path, "wb").write(before)
    assert sha(path) == h
    print(f"{name}: check exit {code} -> {'FAIL (red, as required)' if code != 0 else 'PASS (plant NOT caught)'}", flush=True)
    red += code != 0
print(f"{red} of {len(plants)} plants red; files restored byte-identical")
sys.exit(0 if red == len(plants) else 1)
