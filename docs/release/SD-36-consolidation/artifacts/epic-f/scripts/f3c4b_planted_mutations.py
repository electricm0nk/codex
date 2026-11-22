#!/usr/bin/env python3
"""F3c4b planted mutations (SD-36 Epic F3c4b): each mutates ONE thing in a scratch copy of the fresh
package, runs structural_diff.py against tranche/16, records the verdict, and restores the copy.

Usage: python3 f3c4b_planted_mutations.py <scratch dir> <repo root> <tranche/16 package dir>
  (`git archive cc21cac195 data/sheet_rules | tar -x -C <dir>`); the copy is made under
  <scratch dir>/f3c4b-mut. Output: f3c4b-planted-mutations.txt.
"""
import json, os, shutil, subprocess, sys
S, repo, base = sys.argv[1], sys.argv[2], sys.argv[3]
copy = f"{S}/f3c4b-mut/sheet_rules"
sd = f"{repo}/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py"
if os.path.exists(f"{S}/f3c4b-mut"):
    shutil.rmtree(f"{S}/f3c4b-mut")
shutil.copytree(f"{repo}/data/sheet_rules", copy)


def run(name):
    p = subprocess.run([sys.executable, sd, copy, "--baseline", base], capture_output=True, text=True)
    out = p.stdout.splitlines()
    verdict = [l for l in out if l.startswith("verdict=")]
    detail = [l.strip() for l in out if ("F3c4b " in l and ("missing" in l or "moved" in l or "unpinned" in l or "rows, pinned" in l)) or l.startswith("unexpected field deltas: ") or l.startswith("removed granted_by edges: ")]
    print(f"{name}: exit={p.returncode} {verdict[-1] if verdict else ''}")
    for l in [d for d in detail if not d.endswith(": 0") and "edges: 0 " not in d][:3]:
        print(f"    {l}")
    sys.stdout.flush()


def edit(rel, fn):
    path = f"{copy}/{rel}"
    raw = open(path, "rb").read()
    data = json.loads(raw)
    fn(data)
    open(path, "w").write(json.dumps(data, separators=(",", ":")))
    return lambda: open(path, "wb").write(raw)


DRACONIC = "core_rulebook/pool_option/sorcerer_bloodline_draconic_bloodline.json"
CHOOSER = "core_rulebook:class_feature:sorcerer_standard_bloodline_selection"
run("control (unmodified)")
r = edit(DRACONIC, lambda d: d[0].__setitem__("granted_by", [g for g in d[0]["granted_by"] if g["by"] != {"Choice": CHOOSER}]))
run("M1 Draconic's pick option loses its Choice(Standard Bloodline) edge"); r()
r = edit("core_rulebook/class_feature/sorcerer_bloodline_draconic.json", lambda d: d[0].__setitem__("granted_by", [g for g in d[0]["granted_by"] if g["by"] != {"Rule": "core_rulebook:pool_option:sorcerer_bloodline_draconic_bloodline"}]))
run("M2 the Draconic option -> Sorcerer Bloodline ~ Draconic edge dropped"); r()
var = "_vars/v763e68437947d610.json"  # Sorcerer_Draconic_BloodlineClassSkill1


def drop_contrib(d):
    d["contributions"] = [c for c in d["contributions"] if c.get("rule_id") != "core_rulebook:pool_option:sorcerer_bloodline_draconic_bloodline"]


r = edit(var, drop_contrib)
run("M3 Draconic's BloodlineClassSkill1 contribution dropped"); r()
r = edit("_defects/pool-option-unconverted.json", lambda d: d.pop())
run("M4 one pool-option-unconverted row dropped"); r()
extra = f"{copy}/core_rulebook/pool_option/sorcerer_bloodline_planted.json"
json.dump([{"id": "core_rulebook:pool_option:sorcerer_bloodline_planted", "label": "Planted", "value": "Text", "granted_by": [{"by": {"Choice": CHOOSER}, "when": "Always"}], "grants": []}], open(extra, "w"))
run("M5 an unpinned pool option planted"); os.remove(extra)
r = edit(DRACONIC, lambda d: d[0].__setitem__("pool", "special_ability"))
run("M6 Draconic's option moved out of the sorcerer_bloodline pool (content sha)"); r()
r = edit("core_rulebook/class_feature/bloodline_tracker.json", lambda d: d[0].__setitem__("granted_by", []))
run("M7 the Standard Bloodline -> Bloodline Tracker edge dropped"); r()
r = edit("core_rulebook/pool_option/feat_power_attack_flurry.json", lambda d: d[0].__setitem__("granted_by", [g for g in d[0]["granted_by"] if "Rule" not in g["by"]]))
run("M8 Power Attack (Flurry)'s replacement edges dropped (the replaced fallback edge is then a removal)"); r()
r = edit("advanced_class_guide/ability/aberrant_bloodline.json", lambda d: d[0].__setitem__("applies", "Always"))
run("M9 a pinned MissingRule -> Rule gate delta replaced by a different value"); r()
r = edit("core_rulebook/class/expert.json", lambda d: d[0].pop("closure_complete", None))
run("M10 a pinned closure_complete attestation withdrawn"); r()
shutil.move(f"{copy}/core_rulebook/pool_option", f"{S}/f3c4b-mut/cr-pool-option-aside")
run("M11 every core_rulebook pool option deleted"); shutil.move(f"{S}/f3c4b-mut/cr-pool-option-aside", f"{copy}/core_rulebook/pool_option")
run("control (restored)")
