#!/usr/bin/env python3
"""E5.3 planted mutations: each must turn its check red; every file is restored byte-identical.

usage (repo root, CARGO_TARGET_DIR and PATH set, oracle resolved):
    python3 E5.3_plants.py <baseline sheet_rules dir> <oracle data root>

Checks: desktop = `cargo test --locked -j 8 -- --test-threads=8 sf_sheet_print sf_adapter`
(apps/desktop/src-tauri); delta = E5.3_delta.py against the baseline; sf_check =
`sheet_rule_convert -- --system starfinder-1e --check` (the converter would write a different
package).
"""
import hashlib
import json
import subprocess
import sys

BASE = sys.argv[1]
ORACLE = sys.argv[2]
PKG = "data/starfinder-1e/sheet_rules"
DESKTOP = "apps/desktop/src-tauri"
PRINT_RS = f"{DESKTOP}/src/sf_sheet_print.rs"
ADAPTER_RS = f"{DESKTOP}/src/sf_adapter.rs"
CONVERT_RS = "crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs"
SENSORS = f"{PKG}/core/equipment_modifier/armor_infrared_sensors.json"


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


def reprice(r):
    for seg in r["prose"]:
        if seg["family"] == {"StatBlock": "Price"}:
            seg["pieces"][0]["Text"] = "201"


PLANTS = [
    ("P1_quantity_printed_as_one", PRINT_RS, "desktop",
     lambda p: edit_text(p, 'format!("quantity {}", item.quantity)', 'format!("quantity {}", 1)')),
    ("P2_carried_gear_not_printed", PRINT_RS, "desktop",
     lambda p: edit_text(p, ".filter(|s| s.active_state != ActiveState::Absent)", ".filter(|s| s.active_state == ActiveState::EquippedActive)")),
    ("P3_upgrade_slots_not_enforced", PRINT_RS, "desktop",
     lambda p: edit_text(p, "if used_here > available {", "if used_here > available + 1 {")),
    ("P4_fusion_fit_not_checked", PRINT_RS, "desktop",
     lambda p: edit_text(p, "if stat_row(rule, DAMAGE_ROW).is_none() {", "if false {")),
    ("P5_one_augmentation_per_system_off", PRINT_RS, "desktop",
     lambda p: edit_text(p, "            if item.installed {", "            if false {")),
    ("P6_equipped_item_not_held", ADAPTER_RS, "desktop",
     lambda p: edit_text(p, "if selection.active_state == ActiveState::EquippedActive && !picks.contains(&selection.item_id) {",
                         "if false && !picks.contains(&selection.item_id) {")),
    ("P7_converter_modifier_cost_off", CONVERT_RS, "sf_check",
     lambda p: edit_text(p, 'matches!(ctx.record.kind.as_str(), "equipment" | "equipment_modifier")', 'matches!(ctx.record.kind.as_str(), "equipment")')),
    ("P8_converter_modifier_stat_rows_off", CONVERT_RS, "sf_check",
     lambda p: edit_text(p, 'if key == "SPROP"\n                && let Some((label, text))', 'if key == "SPROP_PLANTED"\n                && let Some((label, text))')),
    ("P9_upgrade_price_off_the_oracle", SENSORS, "delta",
     lambda p: edit_json(p, "core:equipment_modifier:armor_infrared_sensors", reprice)),
    ("P10_unclassified_package_move", SENSORS, "delta",
     lambda p: edit_json(p, "core:equipment_modifier:armor_infrared_sensors", lambda r: r.update(label="Infrared sensors (planted)"))),
]


def run(check):
    if check == "desktop":
        cmd = ["cargo", "test", "--locked", "-j", "8", "--", "--test-threads=8", "sf_sheet_print", "sf_adapter"]
        r = subprocess.run(cmd, cwd=DESKTOP, capture_output=True, text=True)
    elif check == "delta":
        cmd = ["python3", "docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.3_delta.py", PKG, BASE, ORACLE]
        r = subprocess.run(cmd, capture_output=True, text=True)
    else:
        cmd = ["cargo", "run", "--locked", "-j", "8", "-q", "-p", "codex-ingest", "--bin", "sheet_rule_convert", "--",
               "--system", "starfinder-1e", "--check"]
        r = subprocess.run(cmd, capture_output=True, text=True)
    keep = ("test result", "verdict=", "panicked", "FAILED", "differ", "stale", "error")
    tail = [l for l in (r.stdout + r.stderr).splitlines() if any(k in l for k in keep)]
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
        print(f"{name}: check={check} exit={code} -> {results[name]}", flush=True)
        for l in tail[:5]:
            print("   ", l.strip()[:300], flush=True)
    restored = {c: run(c)[0] for c in ("desktop", "delta", "sf_check")}
    print(f"restored: desktop exit={restored['desktop']} delta exit={restored['delta']} sf_check exit={restored['sf_check']}")
    red = sum(v == "FAIL" for v in results.values())
    ok = red == len(PLANTS) and all(v == 0 for v in restored.values())
    print(f"plants_red={red}/{len(PLANTS)} verdict={'PASS' if ok else 'FAIL'}")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
