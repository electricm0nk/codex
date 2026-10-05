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
plants = [
    ("P1 modifier gate dropped", f"{PKG}/companion_mod/drone.json", lambda p: edit_json(p, "core:companion_mod:drone", lambda r: r.__setitem__("applies", "Always")), ingest),
    ("P2 skill unit option deleted", f"{PKG}/pool_option/drone_skill_unit_perception.json", lambda p: os.remove(p), ingest),
    ("P3 kit scores row dropped", f"{PKG}/pool_option/drone_chassis_selection_hover.json", lambda p: edit_json(p, "core:pool_option:drone_chassis_selection_hover", lambda r: r.__setitem__("prose", [])), ingest),
    ("P4 Limited AI edge dropped", f"{PKG}/ability/limited_ai.json", lambda p: edit_json(p, "core:ability:limited_ai", lambda r: r.__setitem__("granted_by", [])), ingest),
    ("P5 drone class race edge dropped", f"{PKG}/class/drone.json", lambda p: edit_json(p, "core:class:drone", lambda r: r.__setitem__("granted_by", [])), ingest),
]
if "--print" in sys.argv:
    plants += [
        ("P6 master link dropped", PRINT_RS, lambda p: edit_text(p, "            master_vars: master_vars.clone(),\n", ""), desktop),
        ("P7 drone choices dropped", PRINT_RS, lambda p: edit_text(p, "            choices: build\n                .choices\n", "            choices: BTreeMap::<String, Vec<String>>::new()\n"), desktop),
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
