#!/usr/bin/env python3
"""E5.2 planted mutations: each must turn its check red; every file is restored byte-identical.

usage (repo root, CARGO_TARGET_DIR and PATH set): python3 E5.2_plants.py <baseline sheet_rules dir>

Checks: desktop = `cargo test --locked -j 8 -- --test-threads=8 sf_sheet_print sf_adapter`
(apps/desktop/src-tauri); delta = E5.2_delta.py against the baseline; sf_check =
`sheet_rule_convert -- --system starfinder-1e --check` (the converter would write a different
package).
"""
import hashlib
import json
import subprocess
import sys

BASE = sys.argv[1]
PKG = "data/starfinder-1e/sheet_rules"
DESKTOP = "apps/desktop/src-tauri"
PRINT_RS = f"{DESKTOP}/src/sf_sheet_print.rs"
CONVERT_RS = "crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs"
INPUT_RS = "src/rules_core/character_input.rs"


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


def misdecode_full_text(r):
    # The full description (segment 1; segment 0 is the `Never` short form, which no sheet or
    # catalog prints beside the full text): its first word with the oracle's pre-repair quote.
    seg = r["prose"][1]
    seg["pieces"][0]["Text"] = "â€œ" + seg["pieces"][0]["Text"]


def off_mystic_list(r):
    r["granted_by"] = [g for g in r["granted_by"] if g["by"].get("ClassSpellList", {}).get("id") != "mystic"]


PLANTS = [
    ("P1_detect_thoughts_misdecoded_again", f"{PKG}/core/spell/detect_thoughts.json", "desktop",
     lambda p: edit_json(p, "core:spell:detect_thoughts", misdecode_full_text)),
    ("P2_command_off_the_mystic_list", f"{PKG}/core/spell/command.json", "desktop",
     lambda p: edit_json(p, "core:spell:command", off_mystic_list)),
    ("P3_quick_draw_not_printed", f"{PKG}/core/feat/quick_draw.json", "desktop",
     lambda p: edit_json(p, "core:feat:quick_draw", lambda r: r.update(print=False))),
    ("P4_spell_evaluated_with_no_caster", PRINT_RS, "desktop",
     lambda p: edit_text(p, "let ctx = EvalContext { holder_class: Some(class.clone()),", "let ctx = EvalContext { holder_class: None,")),
    ("P5_rule_id_spell_parse_reverted", INPUT_RS, "desktop",
     lambda p: edit_text(p, '[_, "spell", _, source_class_id] =>', '[_, "spell_planted", _, source_class_id] =>')),
    ("P6_unclassified_package_move", f"{PKG}/core/spell/magic_missile.json", "delta",
     lambda p: edit_json(p, "core:spell:magic_missile", lambda r: r.update(label="Magic Missile (planted)"))),
    ("P7_converter_repair_disabled", CONVERT_RS, "sf_check",
     lambda p: edit_text(p, "    if system == GameSystem::Pathfinder1e || !tokens.iter().any(|(_, v)| matches!(sf_unmisdecoded(v)",
                         "    if true || !tokens.iter().any(|(_, v)| matches!(sf_unmisdecoded(v)")),
]


def run(check):
    if check == "desktop":
        cmd = ["cargo", "test", "--locked", "-j", "8", "--", "--test-threads=8", "sf_sheet_print", "sf_adapter"]
        r = subprocess.run(cmd, cwd=DESKTOP, capture_output=True, text=True)
    elif check == "delta":
        cmd = ["python3", "docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.2_delta.py", PKG, BASE]
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
