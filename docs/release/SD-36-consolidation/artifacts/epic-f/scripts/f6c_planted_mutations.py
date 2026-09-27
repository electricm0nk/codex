#!/usr/bin/env python3
"""F6c planted mutations (SD-36 Epic F6c, converter step): each mutates ONE thing in a scratch copy
of the fresh package, runs structural_diff.py against the tranche/16 baseline, records the verdict,
and restores the copy.

Usage: python3 f6c_planted_mutations.py <scratch dir> <repo root> <tranche/16 package dir>
  (`git archive tranche/16 data/sheet_rules | tar -x -C <dir>`); the copy is made under
  <scratch dir>/f6c-mut. Output: f6c-planted-mutations.txt.
"""
import json, os, shutil, subprocess, sys
S, repo, base = sys.argv[1], sys.argv[2], sys.argv[3]
copy = f"{S}/f6c-mut/sheet_rules"
sd = f"{repo}/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py"
if os.path.exists(f"{S}/f6c-mut"):
    shutil.rmtree(f"{S}/f6c-mut")
shutil.copytree(f"{repo}/data/sheet_rules", copy)


def run(name):
    p = subprocess.run([sys.executable, sd, copy, "--baseline", base], capture_output=True, text=True)
    out = p.stdout.splitlines()
    verdict = [l for l in out if l.startswith("verdict=")]
    detail = [l.strip() for l in out if l.strip().startswith(("F6c ", "F4pre pinned", "baseline F"))]
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


def sibling(data, rid):
    return next(r for r in data if r["id"] == rid)


HK = "adventurers_guide/class/hellknight.json"
CY = "inner_sea_magic/class/cyphermage.json"
SHAMAN = "advanced_class_guide/class/shaman.json"
SH = "advanced_class_guide:class:shaman#bonus1"
CLERIC = "core_rulebook/class/cleric.json"
CL = "core_rulebook:class:cleric#bonus1"
run("control (unmodified)")
r = edit(HK, lambda d: d[0]["provenance"]["printing"].__setitem__("newest", "inner_sea_world_guide:class:hellknight"))
run("M1 the hellknight verdict names the older printing"); r()
r = edit(CY, lambda d: d[0]["provenance"]["printing"].__setitem__("newest", "inner_sea_magic:class:cyphermage"))
run("M2 a verdict the resolver never proved (cyphermage)"); r()
r = edit(CY, lambda d: d[0]["provenance"]["printing"].__setitem__("source_date", "2018-01"))
run("M3 a printing's SOURCEDATE moved"); r()
r = edit(CY, lambda d: d[0]["provenance"].pop("printing"))
run("M4 a pinned printing withdrawn"); r()
r = edit("core_rulebook/class/fighter.json", lambda d: d[0]["provenance"].__setitem__("printing", {"printings": ["core_rulebook:class:fighter"]}))
run("M5 an unpinned printing on a class no other book states"); r()
r = edit(SHAMAN, lambda d: sibling(d, SH).__setitem__("print", True))
run("M6 the shaman's withheld count prints again"); r()
r = edit(SHAMAN, lambda d: sibling(d, SH).__setitem__("offers", {"id": SH, "count": {"Const": 1}, "from": "Domains"}))
run("M7 the shaman's withheld count offers the domains again"); r()
r = edit(CLERIC, lambda d: (sibling(d, CL).pop("offers"), sibling(d, CL).__setitem__("print", False)))
run("M8 the cleric's domain count withheld (its domains are the player's picks)"); r()
r = edit("advanced_class_guide/domain/life_spirit.json", lambda d: d[0].__setitem__("print", True))
run("M9 a spirit-magic domain record prints again"); r()
r = edit("core_rulebook/domain/air.json", lambda d: d[0].__setitem__("print", False))
run("M10 a domain another class grants (the druid's Air) stops printing"); r()
run("control (restored)")
