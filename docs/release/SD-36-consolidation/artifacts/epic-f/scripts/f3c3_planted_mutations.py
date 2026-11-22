#!/usr/bin/env python3
"""F3c3 planted mutations (SD-36 Epic F3c3): each mutates ONE thing in a scratch copy of the fresh
package, runs structural_diff.py against tranche/16, records the verdict, and restores the copy.

Usage: python3 f3c3_planted_mutations.py <scratch dir> <repo root>
  <scratch dir>/base-t16/data/sheet_rules must hold tranche/16's package
  (`git archive cc21cac195 data/sheet_rules | tar -x -C <scratch dir>/base-t16`); the copy is made
  under <scratch dir>/f3c3-mut. Output: f3c3-planted-mutations.txt.
"""
import json, os, shutil, subprocess, sys
S, repo = sys.argv[1], sys.argv[2]
base = f"{S}/base-t16/data/sheet_rules"
copy = f"{S}/f3c3-mut/sheet_rules"
sd = f"{repo}/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py"
if os.path.exists(f"{S}/f3c3-mut"):
    shutil.rmtree(f"{S}/f3c3-mut")
shutil.copytree(f"{repo}/data/sheet_rules", copy)

def run(name):
    p = subprocess.run([sys.executable, sd, copy, "--baseline", base], capture_output=True, text=True)
    lines = [l for l in p.stdout.splitlines() if l.startswith("verdict=") or "F3c3 " in l and ("missing" in l or "moved" in l or "unpinned" in l or "rows, pinned" in l)]
    print(f"{name}: exit={p.returncode} {lines[-1] if lines else ''}")
    for l in lines[:-1][:3]:
        print(f"    {l.strip()}")
    sys.stdout.flush()

def edit(rel, fn):
    path = f"{copy}/{rel}"
    raw = open(path, "rb").read()
    data = json.loads(raw)
    fn(data)
    open(path, "w").write(json.dumps(data, separators=(",", ":")))
    return lambda: open(path, "wb").write(raw)

run("control (unmodified)")
r = edit("ultimate_psionics/subclass/psion_egoist.json", lambda d: d[0].__setitem__("grants", [g for g in d[0]["grants"] if g != {"FactGrant": {"ClassSkill": "spellcraft"}}]))
run("M1 egoist loses its Spellcraft class-skill grant"); r()
r = edit("ultimate_psionics/class/psion.json", lambda d: d.__setitem__(slice(None), [x for x in d if not x["id"].endswith("#subclass")]))
run("M2 psion's subclass choice dropped from the class record"); r()
r = edit("ultimate_psionics/class_feature/psychometabolism_class_skills.json", lambda d: d[0].__setitem__("granted_by", []))
run("M3 the Egoist -> Psychometabolism Class Skills edge dropped"); r()
def drop_contrib(d):
    d["contributions"] = [c for c in d["contributions"] if c.get("rule_id") != "ultimate_psionics:subclass:psion_egoist"]
r = edit("_vars/v308fe013e550a1ef.json", drop_contrib)
run("M4 Egoist's Psychometabolism Discipline LVL contribution dropped"); r()
r = edit("_defects/subclass-token-unconverted.json", lambda d: d.pop())
run("M5 one subclass-token-unconverted row dropped"); r()
extra = f"{copy}/core_rulebook/subclass/wizard_planted.json"
json.dump([{"id": "core_rulebook:subclass:wizard_planted", "label": "Planted", "value": "Text", "granted_by": [], "grants": []}], open(extra, "w"))
run("M6 an unpinned subclass rule planted"); os.remove(extra)
r = edit("core_rulebook/subclass/wizard_evoker.json", lambda d: d[0].__setitem__("tags", ["Sorcerer Subclass"]))
run("M7 Evoker's tag moved off the Wizard choice"); r()
shutil.move(f"{copy}/ultimate_psionics/subclass", f"{S}/f3c3-mut/psion-subclass-aside")
run("M8 every Psion subclass file deleted"); shutil.move(f"{S}/f3c3-mut/psion-subclass-aside", f"{copy}/ultimate_psionics/subclass")
run("control (restored)")
