#!/usr/bin/env python3
"""F3p planted mutations (SD-36 Epic F3 polish P5): each mutates ONE thing in a scratch copy of the
fresh package, runs structural_diff.py against the tranche/16 baseline, records the verdict, and
restores the copy.

Usage: python3 f3p_planted_mutations.py <scratch dir> <repo root> <tranche/16 package dir>
  (`git archive 070c253e93 data/sheet_rules | tar -x -C <dir>`); the copy is made under
  <scratch dir>/f3p-mut. Output: f3p-planted-mutations.txt.
"""
import json, os, shutil, subprocess, sys
S, repo, base = sys.argv[1], sys.argv[2], sys.argv[3]
copy = f"{S}/f3p-mut/sheet_rules"
sd = f"{repo}/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py"
if os.path.exists(f"{S}/f3p-mut"):
    shutil.rmtree(f"{S}/f3p-mut")
shutil.copytree(f"{repo}/data/sheet_rules", copy)
EWP = "core_rulebook:feat:exotic_weapon_proficiency"


def run(name):
    p = subprocess.run([sys.executable, sd, copy, "--baseline", base], capture_output=True, text=True)
    out = p.stdout.splitlines()
    verdict = [l for l in out if l.startswith("verdict=")]
    detail = [l.strip() for l in out if l.strip().startswith("F3p ") and ("withdrawn" in l or "missing" in l or "option terms" in l)] + [l for l in out if l.startswith("unexpected field deltas: ") or l.startswith("removed granted_by edges: ")]
    print(f"{name}: exit={p.returncode} {verdict[-1] if verdict else ''}")
    for l in [d for d in detail if not d.endswith(": 0") and "(never allowed" not in d or d.startswith("removed granted_by edges: ") and not d.startswith("removed granted_by edges: 0")][:3]:
        print(f"    {l}")
    sys.stdout.flush()


def edit(rel, fn):
    path = f"{copy}/{rel}"
    raw = open(path, "rb").read()
    data = json.loads(raw)
    fn(data)
    open(path, "w").write(json.dumps(data, separators=(",", ":")))
    return lambda: open(path, "wb").write(raw)


def ewp_edge(d):
    return next(e for e in d[0]["granted_by"] if e["by"] == {"Rule": EWP})


FIRE = "ultimate_combat/class_feature/exotic_weapon_proficiency_firearms.json"
run("control (unmodified)")
r = edit(FIRE, lambda d: ewp_edge(d).__setitem__("when", {"Holds": {"what": {"Rule": EWP}, "count": 1}}))
run("M1 the firearms grant's option gate withdrawn (bare Exotic Weapon Proficiency again)"); r()
r = edit(FIRE, lambda d: ewp_edge(d)["when"]["All"][1]["Chosen"].__setitem__("option", "katana"))
run("M2 the firearms grant's option becomes katana"); r()
r = edit(FIRE, lambda d: ewp_edge(d)["when"]["All"][1]["Chosen"].__setitem__("choice", "core_rulebook:feat:martial_weapon_proficiency"))
run("M3 the option term names another chooser"); r()
r = edit(FIRE, lambda d: ewp_edge(d)["when"]["All"].append({"Chosen": {"choice": EWP, "option": "pistol"}}))
run("M4 a second option term added to the pinned gate"); r()
r = edit("core_rulebook/feat/power_attack.json", lambda d: d[0].__setitem__("applies", {"All": [d[0].get("applies") or "Always", {"Holds": {"what": {"Rule": EWP}, "count": 1}}, {"Chosen": {"choice": EWP, "option": "firearms"}}]}))
run("M5 an unpinned option gate planted on Power Attack"); r()
pinned_tables = json.load(open(f"{repo}/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff_f3p_deltas.json"))["var_tables"]["pins"]
table = pinned_tables[0][0]


def drop_first_option(node):
    if isinstance(node, dict):
        if set(node) == {"All"} and any(isinstance(t, dict) and "Chosen" in t for t in node["All"]):
            node["All"] = [t for t in node["All"] if not (isinstance(t, dict) and "Chosen" in t)]
            return True
        return any(drop_first_option(v) for v in node.values())
    if isinstance(node, list):
        return any(drop_first_option(v) for v in node)
    return False


r = edit(table, drop_first_option)
run(f"M6 a pinned _vars table ({table}) loses its option term"); r()
SEL = "requires a martial option chosen for Weapon Focus"


def selector_to_option(node):
    if isinstance(node, dict):
        for k, v in list(node.items()):
            if v == {"Situational": {"text": SEL}}:
                node[k] = {"Chosen": {"choice": "core_rulebook:feat:weapon_focus", "option": "type_martial"}}
                return True
            if selector_to_option(v):
                return True
    if isinstance(node, list):
        for i, v in enumerate(node):
            if v == {"Situational": {"text": SEL}}:
                node[i] = {"Chosen": {"choice": "core_rulebook:feat:weapon_focus", "option": "type_martial"}}
                return True
            if selector_to_option(v):
                return True
    return False


sel_rel = next(p for p in (os.path.relpath(os.path.join(d, f), copy) for d, _, fs in os.walk(copy) for f in fs if f.endswith(".json") and not d.endswith("_vars")) if SEL in open(os.path.join(copy, p)).read())
r = edit(sel_rel, selector_to_option)
run(f"M8 a TYPE= selector condition ({sel_rel}) becomes an option id no pick carries (type_martial)"); r()
r = edit(FIRE, lambda d: d[0]["granted_by"].remove(ewp_edge(d)))
run("M7 the firearms grant's Exotic Weapon Proficiency edge dropped"); r()
run("control (restored)")
