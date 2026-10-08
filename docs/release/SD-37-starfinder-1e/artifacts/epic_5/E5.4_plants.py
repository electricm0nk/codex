#!/usr/bin/env python3
"""E5.4 planted mutations: each must turn its check red; every file is restored byte-identical.

usage (repo root, CARGO_TARGET_DIR and PATH set): python3 E5.4_plants.py [--print]

Package plants (check = `cargo test -p codex-ingest --test sf_drone_companion`):
  P1 the companion modifier loses its master gate;      P2 a drone skill unit option is deleted;
  P3 the hover chassis loses its kit scores row;        P4 Limited AI loses its modifier edge;
  P5 the drone class loses its race edge.
Print plants (--print; check = desktop `cargo test sf_drone_print`; needs the engine+print patch):
  P6 the master link is dropped (no master variables reach the drone);
  P7 the drone's choices are dropped (no chassis / skill unit / feat / mod held).
Attempt-2 plants (decisions.md §21 (b), (c)):
  P8 Flight System loses the hover chassis' waiver (package)            -> sf_drone_companion red;
  P9 the drone's hit-point line is dropped (package)                    -> sf_drone_companion red;
  P10 the table's hit_die_offset row routes to hit points (mapping)     -> hp_pool_row unit test red;
  P11 the print's flight-system waiver is dropped (--print, package)    -> desktop sf_drone_print red;
  P12 one waiver's kept gate is altered (classifier)                    -> E5.4_r2_delta.py exit 1.
"""
import hashlib, json, os, subprocess, sys

PKG = "data/starfinder-1e/sheet_rules/core"
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
def edit_json(path, rid, fn):
    rules = json.load(open(path, encoding="utf-8"))
    for r in rules:
        if r["id"] == rid: fn(r)
    open(path, "w", encoding="utf-8").write(json.dumps(rules, ensure_ascii=False, separators=(",", ":")))
def edit_text(path, old, new):
    s = open(path, encoding="utf-8").read(); assert s.count(old) == 1, (path, old)
    open(path, "w", encoding="utf-8").write(s.replace(old, new))
def ingest():
    return subprocess.run(["cargo", "test", "--locked", "-j", "8", "-p", "codex-ingest", "--test", "sf_drone_companion", "--", "--test-threads=8"], capture_output=True, text=True).returncode
def desktop():
    return subprocess.run(["cargo", "test", "--locked", "-j", "8", "sf_drone_print", "--", "--test-threads=8"], cwd="apps/desktop/src-tauri", capture_output=True, text=True).returncode

PRINT_RS = "apps/desktop/src-tauri/src/sf_drone_print.rs"
TABLE = "docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf-mapping-table.v1.json"
def mapping_unit():
    return subprocess.run(["cargo", "test", "--locked", "-j", "8", "-p", "codex-ingest", "--lib", "hp_pool_row", "--", "--test-threads=8"], capture_output=True, text=True).returncode
def r2_delta():
    base = os.environ["E54_BASE_SHEET_RULES"]  # the card base's data/starfinder-1e/sheet_rules, extracted
    return subprocess.run(["python3", "docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.4_r2_delta.py", "data/starfinder-1e/sheet_rules", base], capture_output=True, text=True).returncode
def unwaive(r):
    r["applies"] = r["applies"]["AtLeast"]["of"][-1]
def alter_kept_gate(r):
    r["applies"]["AtLeast"]["of"][-1] = "Always"
plants = [
    ("P1 modifier gate dropped", f"{PKG}/companion_mod/drone.json", lambda p: edit_json(p, "core:companion_mod:drone", lambda r: r.__setitem__("applies", "Always")), ingest),
    ("P2 skill unit option deleted", f"{PKG}/pool_option/drone_skill_unit_perception.json", lambda p: os.remove(p), ingest),
    ("P3 kit scores row dropped", f"{PKG}/pool_option/drone_chassis_selection_hover.json", lambda p: edit_json(p, "core:pool_option:drone_chassis_selection_hover", lambda r: r.__setitem__("prose", [])), ingest),
    ("P4 Limited AI edge dropped", f"{PKG}/ability/limited_ai.json", lambda p: edit_json(p, "core:ability:limited_ai", lambda r: r.__setitem__("granted_by", [])), ingest),
    ("P5 drone class race edge dropped", f"{PKG}/class/drone.json", lambda p: edit_json(p, "core:class:drone", lambda r: r.__setitem__("granted_by", [])), ingest),
    ("P8 flight-system waiver dropped", f"{PKG}/ability/drone_mod_flight_system.json", lambda p: edit_json(p, "core:ability:drone_mod_flight_system", unwaive), ingest),
    ("P9 drone hit-point line dropped", f"{PKG}/class/drone.json", lambda p: edit_json(p, "core:class:drone#bonus1", lambda r: r.__setitem__("target", "Stamina")), ingest),
    ("P10 hit_die_offset routed to hit points", TABLE, lambda p: edit_text(p, '"id": "hit_die_offset"', '"id": "hit_points"'), mapping_unit),
    ("P12 a waiver's kept gate altered", f"{PKG}/feat/deadly_aim.json", lambda p: edit_json(p, "core:feat:deadly_aim", alter_kept_gate), r2_delta),
]
if "--print" in sys.argv:
    plants += [
        ("P6 master link dropped", PRINT_RS, lambda p: edit_text(p, "            master_vars: master_vars.clone(),\n", ""), desktop),
        ("P7 drone choices dropped", PRINT_RS, lambda p: edit_text(p, "            choices: build\n                .choices\n", "            choices: BTreeMap::<String, Vec<String>>::new()\n"), desktop),
        ("P11 print: flight-system waiver dropped", f"{PKG}/ability/drone_mod_flight_system.json", lambda p: edit_json(p, "core:ability:drone_mod_flight_system", unwaive), desktop),
    ]
red = 0
for name, path, plant, check in plants:
    before = open(path, "rb").read(); h = sha(path)
    plant(path)
    code = check()
    open(path, "wb").write(before)
    assert sha(path) == h
    print(f"{name}: check exit {code} -> {'FAIL (red, as required)' if code != 0 else 'PASS (plant NOT caught)'}")
    red += code != 0
print(f"{red} of {len(plants)} plants red; files restored byte-identical")
sys.exit(0 if red == len(plants) else 1)
