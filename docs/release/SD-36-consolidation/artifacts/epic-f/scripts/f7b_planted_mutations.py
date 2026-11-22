#!/usr/bin/env python3
"""F7b planted mutations (SD-36 Epic F7b, converter step): each mutates ONE thing in a scratch copy
of the regenerated package, runs structural_diff.py against the tranche/16 baseline, records the
verdict, and restores the copy. Every mutation must FAIL; both controls must PASS.

Usage: python3 f7b_planted_mutations.py <scratch dir> <repo root> <tranche/16 package dir>
  (`git archive tranche/16 data/sheet_rules | tar -x -C <dir>`); the copy is made under
  <scratch dir>/f7b-mut. Output: f7b-planted-mutations.txt.
"""
import json, os, shutil, subprocess, sys
S, repo, base = sys.argv[1], sys.argv[2], sys.argv[3]
copy = f"{S}/f7b-mut/sheet_rules"
sd = f"{repo}/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py"
if os.path.exists(f"{S}/f7b-mut"):
    shutil.rmtree(f"{S}/f7b-mut")
shutil.copytree(f"{repo}/data/sheet_rules", copy)


def run(name):
    p = subprocess.run([sys.executable, sd, copy, "--baseline", base], capture_output=True, text=True)
    out = p.stdout.splitlines()
    verdict = [l for l in out if l.startswith("verdict=")]
    detail = [l.strip() for l in out if l.strip().startswith(("F7b pinned", "F7b:"))]
    detail += [l.strip() for l in out if l.startswith("unexpected field deltas: ") and not l.endswith(": 0")]
    print(f"{name}: exit={p.returncode} {verdict[-1] if verdict else ''}")
    for l in detail[:3]:
        print(f"    {l}")
    sys.stdout.flush()


def edit(rel, fn):
    path = f"{copy}/{rel}"
    raw = open(path, "rb").read()
    data = json.loads(raw)
    fn(data)
    open(path, "w").write(json.dumps(data, separators=(",", ":")))
    return lambda: open(path, "wb").write(raw)


def rule(data, rid):
    return next(r for r in data if r["id"] == rid)


def base_rule(rel, rid):
    return next(r for r in json.load(open(f"{base}/{rel}")) if r["id"] == rid)


FB = "core_rulebook/spell/fireball.json"
FBID = "core_rulebook:spell:fireball"
PR = "occult_adventures/spell/primal_regression.json"
PRID = "occult_adventures:spell:primal_regression"
AM = "advanced_class_guide/equipment/amulet_of_the_blooded_aberrant.json"
AMID = "advanced_class_guide:equipment:amulet_of_the_blooded_aberrant"
NSA = "advanced_class_guide/spell/naturalist_summon_nature_s_ally_i.json"
NSAID = "advanced_class_guide:spell:naturalist_summon_nature_s_ally_i"
FEAT = "advanced_class_guide/feat/amateur_investigator.json"
FEATID = "advanced_class_guide:feat:amateur_investigator"
DA = "core_rulebook/feat/deadly_aim.json"
DAID = "core_rulebook:feat:deadly_aim"


def dice_count_segment(r):
    return next(s for s in r["prose"] if any("DiceCount" in p for p in s["pieces"]))


run("control (unmodified)")
r = edit(FB, lambda d: next(p for p in dice_count_segment(rule(d, FBID))["pieces"] if "DiceCount" in p)["DiceCount"]["count"]["Min"][0].__setitem__("Const", 20))
run("M1 Fireball's rendered damage cap moves from 10d6 to 20d6"); r()


def respell(d):
    seg = next(s for s in rule(d, PRID)["prose"] if s["family"] == {"StatBlock": "Duration"})
    seg["pieces"] = [{"Text": "(CASTERLEVEL) minutes"}]


r = edit(PR, respell)
run("M2 a lowered duration spelled as the source formula again (primal regression)"); r()
r = edit(FB, lambda d: rule(d, FBID)["prose"].insert(3, [s for s in base_rule(FB, FBID)["prose"] if s.get("applies") not in (None, "Never")][0]))
run("M3 Fireball's out-of-inventory mythic line printed again"); r()


def escape_feat(d):
    for s in rule(d, FEATID)["prose"]:
        for p in s["pieces"]:
            if "Text" in p:
                p["Text"] = p["Text"].replace("[", "(").replace("]", ")")


r = edit(FEAT, escape_feat)
run("M4 brackets escaped in a feat's text (not spell text)"); r()
r = edit(DA, lambda d: rule(d, DAID)["prose"].pop(1))
run("M5 an ordinary gated prose line dropped (Deadly Aim)"); r()


def recolor(d):
    for s in rule(d, FBID)["prose"]:
        for p in s["pieces"]:
            if "Text" in p and "fire damage per caster level" in p["Text"]:
                p["Text"] = p["Text"].replace("fire damage per caster level", "cold damage per caster level")
                return


r = edit(FB, recolor)
run("M6 the words around a pinned delta change (Fireball fire -> cold)"); r()
r = edit(AM, lambda d: rule(d, AMID).__setitem__("prose", base_rule(AM, AMID)["prose"]))
run("M7 a decided out-of-inventory condition left undecided (amulet of the blooded, aberrant)"); r()
r = edit(NSA, lambda d: rule(d, NSAID)["provenance"].__setitem__("undeclared_in_pinned_tree", []))
run("M8 the oracle-zero provenance note withdrawn (naturalist summon nature's ally I)"); r()
run("control (restored)")
