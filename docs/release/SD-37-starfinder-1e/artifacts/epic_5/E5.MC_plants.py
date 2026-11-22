#!/usr/bin/env python3
"""E5.MC criterion plants: one granted feature dropped from a seed's converted records, four shapes,
one per seed. Each must turn E5.1's criterion test red FOR THE INTENDED REASON (the panic names the
dropped rule id under "not printed"), and E5.1's fixture check (fixture = fresh walk) red too.
Every file is restored byte-identical; a restored run must be green.

usage (repo root, CARGO_TARGET_DIR and PATH set): python3 E5.MC_plants.py
"""
import hashlib
import json
import os
import subprocess
import sys

PKG = "data/starfinder-1e/sheet_rules/core/ability"
DESKTOP = "apps/desktop/src-tauri"
TEST = "sf_seed_printed_features_equal_the_features_the_converted_records_grant"
FIXTURE_CHECK = ["python3", "docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.1_seed_features.py", ".", "--check"]


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def edit_json(path, fn):
    rules = json.load(open(path, encoding="utf-8"))
    fn(rules[0])
    with open(path, "w", encoding="utf-8") as f:
        f.write(json.dumps(rules, ensure_ascii=False, separators=(",", ":")))


def pop_grants(r):
    r.pop("granted_by")


def grant_at_six(r):
    # the Class edge and its ClassLevel gate both move from level 1 to level 6 (seed is level 5)
    g = r["granted_by"][0]
    g["by"]["Class"]["at_level"] = 6
    g["when"]["All"][1]["Compare"]["rhs"]["Const"] = 6


def reattribute(r):
    # the grant is re-attributed to another class (operative), so the soldier no longer holds it
    g = r["granted_by"][0]
    g["by"]["Class"]["id"] = "operative"
    g["when"]["All"][1]["Compare"]["lhs"]["ClassLevel"] = "operative"


PLANTS = [
    ("MC1 SF-Mystic-5: Healing Touch loses every grant edge", "SF-Mystic-5",
     "core:ability:mystic_class_feature_healing_touch", f"{PKG}/mystic_class_feature_healing_touch.json",
     lambda p: edit_json(p, pop_grants)),
    ("MC2 SF-Technomancer-5: Spell Cache granted at level 6, not 1", "SF-Technomancer-5",
     "core:ability:technomancer_class_feature_spell_cache", f"{PKG}/technomancer_class_feature_spell_cache.json",
     lambda p: edit_json(p, grant_at_six)),
    ("MC3 SF-Envoy-3: Envoy Improvisation record file deleted", "SF-Envoy-3",
     "core:ability:envoy_class_feature_envoy_improvisation", f"{PKG}/envoy_class_feature_envoy_improvisation.json",
     lambda p: os.remove(p)),
    ("MC4 SF-Soldier-3: Combat Feat grant re-attributed to operative", "SF-Soldier-3",
     "core:ability:soldier_class_feature_combat_feat", f"{PKG}/soldier_class_feature_combat_feat.json",
     lambda p: edit_json(p, reattribute)),
]


def desktop():
    r = subprocess.run(["cargo", "test", "--locked", "-j", "8", TEST, "--", "--test-threads=8"],
                       cwd=DESKTOP, capture_output=True, text=True)
    out = r.stdout + r.stderr
    return r.returncode, out


def fixture():
    r = subprocess.run(FIXTURE_CHECK, capture_output=True, text=True)
    return r.returncode, (r.stdout + r.stderr)


def main():
    red = 0
    for name, seed, rid, path, plant in PLANTS:
        saved = open(path, "rb").read()
        before = sha(path)
        plant(path)
        assert not os.path.exists(path) or sha(path) != before, f"{name}: the plant changed nothing"
        try:
            dcode, dout = desktop()
            fcode, fout = fixture()
        finally:
            open(path, "wb").write(saved)
        assert sha(path) == before, f"{name}: not restored"
        panic = [l for l in dout.splitlines() if "not printed" in l]
        reason = bool(panic) and seed in panic[0] and rid in panic[0].split("not printed", 1)[1]
        ok = dcode != 0 and reason and fcode != 0
        red += ok
        print(f"{name}: desktop exit={dcode} intended_reason={reason} fixture_check exit={fcode} -> {'RED (as required)' if ok else 'NOT CAUGHT / wrong reason'}")
        for l in panic[:1]:
            print("    ", l.strip()[:400])
        for l in [l for l in fout.splitlines() if l.strip()][-2:]:
            print("     fixture:", l.strip()[:300])
    dcode, dout = desktop()
    fcode, _ = fixture()
    tail = [l for l in dout.splitlines() if "test result" in l]
    print(f"restored: desktop exit={dcode} {tail[-1:] if tail else ''} fixture_check exit={fcode}")
    ok = red == len(PLANTS) and dcode == 0 and fcode == 0
    print(f"plants_red={red}/{len(PLANTS)} verdict={'PASS' if ok else 'FAIL'}")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
