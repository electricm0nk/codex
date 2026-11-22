#!/usr/bin/env python3
"""E5.1 planted mutations: each must turn its check red; every file is restored byte-identical.

usage (repo root, CARGO_TARGET_DIR and PATH set): python3 E5.1_plants.py <baseline sheet_rules dir> <oracle data root>

Desktop check = `cargo test --locked -j 8 -- --test-threads=8 sf_sheet_print sf_adapter` in
apps/desktop/src-tauri (the E5.1 tests + the adapter's); delta check = E5.1_delta.py.
"""
import hashlib
import json
import os
import subprocess
import sys

BASE, ORACLE = sys.argv[1], sys.argv[2]
PKG = "data/starfinder-1e/sheet_rules"
DESKTOP = "apps/desktop/src-tauri"
PRINT_RS = f"{DESKTOP}/src/sf_sheet_print.rs"


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def edit_json(path, rule_id, fn):
    rules = json.load(open(path, encoding="utf-8"))
    for r in rules:
        if r["id"] == rule_id:
            fn(r)
    with open(path, "w", encoding="utf-8") as f:
        f.write(json.dumps(rules, ensure_ascii=False, separators=(",", ":")))


def edit_text(path, old, new):
    s = open(path, encoding="utf-8").read()
    assert s.count(old) == 1, (path, old)
    open(path, "w", encoding="utf-8").write(s.replace(old, new))


def speed_zero(r):
    for seg in r["prose"]:
        if seg["family"] == {"StatBlock": "Speed"}:
            seg["pieces"][1]["Text"] = "0"


PLANTS = [
    ("P1_gear_boost_grant_dropped", f"{PKG}/core/ability/soldier_class_feature_gear_boost.json", "desktop",
     lambda p: edit_json(p, "core:ability:soldier_class_feature_gear_boost", lambda r: r.pop("granted_by"))),
    ("P2_race_speed_zero", f"{PKG}/core/race/human.json", "desktop",
     lambda p: edit_json(p, "core:race:human", speed_zero)),
    ("P3_display_key_label", f"{PKG}/core/ability/empath.json", "desktop",
     lambda p: edit_json(p, "core:ability:empath", lambda r: r.update(label="Empath (Display ~ Perception)"))),
    ("P4_weapon_rows_printed", PRINT_RS, "desktop",
     lambda p: edit_text(p, "    lines.retain(|l| package.rule(&l.id).is_none_or(|r| r.pool != WEAPON_POOL));\n", "")),
    ("P5_hit_die_printed", PRINT_RS, "desktop",
     lambda p: edit_text(p, ".filter(|row| !row.starts_with(HIT_DIE_ROW))", ".filter(|row| !row.starts_with(HIT_DIE_ROW) || true)")),
    ("P6_unclassified_package_move", f"{PKG}/core/race/human.json", "delta",
     lambda p: edit_json(p, "core:race:human", lambda r: r.update(label="Human (planted)"))),
]


def run(check):
    if check == "desktop":
        cmd = ["cargo", "test", "--locked", "-j", "8", "--", "--test-threads=8", "sf_sheet_print", "sf_adapter"]
        r = subprocess.run(cmd, cwd=DESKTOP, capture_output=True, text=True)
    else:
        cmd = ["python3", "docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.1_delta.py", PKG, BASE, ORACLE]
        r = subprocess.run(cmd, capture_output=True, text=True)
    tail = [l for l in (r.stdout + r.stderr).splitlines() if "test result" in l or "verdict=" in l or "panicked" in l]
    return r.returncode, tail


def main():
    results = {}
    for name, path, check, plant in PLANTS:
        before = sha(path)
        saved = open(path, "rb").read()
        plant(path)
        assert sha(path) != before, f"{name}: the plant changed nothing"
        try:
            code, tail = run(check)
        finally:
            open(path, "wb").write(saved)
        assert sha(path) == before, f"{name}: not restored"
        results[name] = "FAIL" if code != 0 else "PASS"
        print(f"{name}: check={check} exit={code} -> {results[name]}")
        for l in tail[:4]:
            print("   ", l.strip()[:300])
    restored = {c: run(c)[0] for c in ("desktop", "delta")}
    print(f"restored: desktop exit={restored['desktop']} delta exit={restored['delta']}")
    red = sum(v == "FAIL" for v in results.values())
    ok = red == len(PLANTS) and all(v == 0 for v in restored.values())
    print(f"plants_red={red}/{len(PLANTS)} verdict={'PASS' if ok else 'FAIL'}")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
