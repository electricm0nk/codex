#!/usr/bin/env python3
"""F3c5 planted mutations (SD-36 Epic F3c5): each mutates ONE thing in a scratch copy of the fresh
package, runs structural_diff.py against tranche/16, records the verdict, and restores the copy.

Usage: python3 f3c5_planted_mutations.py <scratch dir> <repo root> <tranche/16 package dir>
  (`git archive cc21cac195 data/sheet_rules | tar -x -C <dir>`); the copy is made under
  <scratch dir>/f3c5-mut. Output: f3c5-planted-mutations.txt.
"""
import json, os, shutil, subprocess, sys
S, repo, base = sys.argv[1], sys.argv[2], sys.argv[3]
copy = f"{S}/f3c5-mut/sheet_rules"
sd = f"{repo}/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py"
if os.path.exists(f"{S}/f3c5-mut"):
    shutil.rmtree(f"{S}/f3c5-mut")
shutil.copytree(f"{repo}/data/sheet_rules", copy)


def run(name):
    p = subprocess.run([sys.executable, sd, copy, "--baseline", base], capture_output=True, text=True)
    out = p.stdout.splitlines()
    verdict = [l for l in out if l.startswith("verdict=")]
    detail = [l.strip() for l in out if l.strip().startswith("F3c5 ") and ("missing" in l or "moved" in l or "unpinned" in l or "rows, pinned" in l or "withdrawn" in l)] + [l for l in out if l.startswith("unexpected field deltas: ") or l.startswith("removed grants: ")]
    print(f"{name}: exit={p.returncode} {verdict[-1] if verdict else ''}")
    for l in [d for d in detail if not d.endswith(": 0") and not d.startswith("removed grants: 0")][:3]:
        print(f"    {l}")
    sys.stdout.flush()


def edit(rel, fn):
    path = f"{copy}/{rel}"
    raw = open(path, "rb").read()
    data = json.loads(raw)
    fn(data)
    open(path, "w").write(json.dumps(data, separators=(",", ":")))
    return lambda: open(path, "wb").write(raw)


BITE = "core_rulebook/class_feature/dragon_disciple_dragon_bite.json"
CLAWS = "core_rulebook/class_feature/draconic_bloodline_claws.json"
run("control (unmodified)")
r = edit(BITE, lambda d: d[0].__setitem__("grants", []))
run("M1 Dragon Bite loses its NaturalAttack(Bite) fact"); r()
r = edit(BITE, lambda d: d[0].__setitem__("grants", [{"FactGrant": {"NaturalAttack": "Claw"}}]))
run("M2 Dragon Bite's fact names another attack (Claw)"); r()
r = edit(BITE, lambda d: d[0].__setitem__("grants", [{"FactGrant": {"Proficiency": {"Weapon": "Bite"}}}]))
run("M3 Dragon Bite's fact becomes a weapon proficiency"); r()
r = edit("core_rulebook/class/dragon_disciple.json", lambda d: d[0].pop("closure_complete", None))
run("M4 Dragon Disciple's closure_complete attestation withdrawn"); r()
r = edit(CLAWS, lambda d: d.pop())
run("M5 Draconic Claws' #weapon0 damage-step sibling dropped"); r()
r = edit(CLAWS, lambda d: d[1].__setitem__("applies", d[0]["applies"]))
run("M6 the damage-step sibling loses its >= 7 gate"); r()
r = edit(CLAWS, lambda d: d[0].__setitem__("applies", d[1]["applies"]))
run("M7 the Claws principal regains the >= 7 gate (held from 7 again)"); r()
r = edit("_defects/unresolved-references.json", lambda d: d.append("core_rulebook:class_feature:dragon_disciple_dragon_bite: Internal|Bite"))
run("M8 the Internal|Bite unresolved row reappears"); r()
r = edit("_vars/v58ee7a23ec1c122a.json", lambda d: d.__setitem__("contributions", []))
run("M9 the Rake Size table a Rake fact's gate reads loses its contribution"); r()
extra = f"{copy}/core_rulebook/class_feature/dragon_disciple_dragon_bite_planted.json"
json.dump([{"id": "core_rulebook:class_feature:dragon_disciple_dragon_bite#weapon9", "label": "Planted", "value": "Text", "granted_by": [], "grants": []}], open(extra, "w"))
run("M10 an unpinned #weapon sibling planted on a record carrying a NaturalAttack fact"); os.remove(extra)
run("control (restored)")
