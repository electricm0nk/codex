#!/usr/bin/env python3
"""F4pre planted mutations (SD-36 Epic F4pre, converter step): each mutates ONE thing in a scratch
copy of the fresh package, runs structural_diff.py against the tranche/16 baseline, records the
verdict, and restores the copy.

Usage: python3 f4pre_planted_mutations.py <scratch dir> <repo root> <tranche/16 package dir>
  (`git archive tranche/16 data/sheet_rules | tar -x -C <dir>`); the copy is made under
  <scratch dir>/f4pre-mut. Output: f4pre-planted-mutations.txt.
"""
import json, os, shutil, subprocess, sys
S, repo, base = sys.argv[1], sys.argv[2], sys.argv[3]
copy = f"{S}/f4pre-mut/sheet_rules"
sd = f"{repo}/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py"
if os.path.exists(f"{S}/f4pre-mut"):
    shutil.rmtree(f"{S}/f4pre-mut")
shutil.copytree(f"{repo}/data/sheet_rules", copy)


def run(name):
    p = subprocess.run([sys.executable, sd, copy, "--baseline", base], capture_output=True, text=True)
    out = p.stdout.splitlines()
    verdict = [l for l in out if l.startswith("verdict=")]
    detail = [l.strip() for l in out if l.strip().startswith("F4pre ") and ("withdrawn" in l or "missing" in l or "is not the" in l)]
    detail += [l.strip() for l in out if l.startswith("unexpected field deltas: ") and not l.endswith(": 0")]
    print(f"{name}: exit={p.returncode} {verdict[-1] if verdict else ''}")
    for l in detail[:3]:
        print(f"    {l}")
    sys.stdout.flush()


def edit(rel, fn, index=None):
    path = f"{copy}/{rel}"
    raw = open(path, "rb").read()
    data = json.loads(raw)
    fn(data if index is None else data[index])
    open(path, "w").write(json.dumps(data, separators=(",", ":")))
    return lambda: open(path, "wb").write(raw)


def sibling(data, rid):
    return next(r for r in data if r["id"] == rid)


CLERIC = "core_rulebook/class/cleric.json"
CL = "core_rulebook:class:cleric#bonus1"
SPIRIT = "advanced_class_guide/class_feature/shaman_spirit.json"
run("control (unmodified)")
r = edit(CLERIC, lambda d: sibling(d, CL).pop("offers"))
run("M1 the cleric's domain choice withdrawn"); r()
r = edit(CLERIC, lambda d: sibling(d, CL)["offers"].__setitem__("count", {"Const": 3}))
run("M2 the cleric's domain count becomes 3 (not the record's ClericDomainCount)"); r()
r = edit(SPIRIT, lambda d: d[0]["offers"]["from"]["Rules"].__setitem__("tags", ["ShamanWanderingSpirit"]))
run("M3 the shaman spirit choice offers another category's members"); r()
r = edit(SPIRIT, lambda d: d[0]["offers"].__setitem__("from", "Domains"))
run("M4 the shaman spirit pick offers the domains"); r()
r = edit(SPIRIT, lambda d: d[0]["offers"].__setitem__("id", "advanced_class_guide:class_feature:shaman"))
run("M5 the shaman spirit choice keyed under another rule"); r()
r = edit("core_rulebook/feat/power_attack.json", lambda d: d[0].__setitem__("offers", {"id": d[0]["id"], "count": {"Const": 1}, "from": {"Rules": {"pool": "feat", "tags": ["Combat"], "requires": "Always"}}}))
run("M6 an unpinned offer planted on a rule that is no pick"); r()
r = edit(SPIRIT, lambda d: d[0]["offers"]["from"]["Rules"].__setitem__("requires", {"Holds": {"what": {"Rule": "core_rulebook:feat:dodge"}, "count": 1}}))
run("M7 the shaman spirit option set gains a requirement"); r()
run("control (restored)")
