#!/usr/bin/env python3
"""SD-37 E4.1: classify the SF package delta of the racial-HP lowering.

usage: E4.1_race_hp_delta.py <new package dir> <baseline package dir>
Pass (exit 0) only when every differing rule file differs by exactly one added `#race_hp`
rule (target Hp, Const value) with every pre-existing rule byte-equal in value, and every
other difference is `_report.json`, `var_names.json` or `_defects/sf-race-hit-points-unresolved.json`.
Prints the race_hp value distribution (denominator: race files in the new package).
"""
import json, os, sys, filecmp
from collections import Counter

new, old = sys.argv[1], sys.argv[2]
allowed_other = {"_report.json", "var_names.json", os.path.join("_defects", "sf-race-hit-points-unresolved.json")}
bad, added, values, races = [], 0, Counter(), 0
def files(root):
    out = set()
    for d, _, ns in os.walk(root):
        for n in ns:
            out.add(os.path.relpath(os.path.join(d, n), root))
    return out
fn, fo = files(new), files(old)
for rel in sorted(fn | fo):
    if "/race/" in rel and rel in fn:
        races += 1
    if rel in fn and rel in fo and filecmp.cmp(os.path.join(new, rel), os.path.join(old, rel), shallow=False):
        continue
    if rel in allowed_other:
        continue
    if rel not in fn or rel not in fo:
        bad.append(f"only in one side: {rel}")
        continue
    a = json.load(open(os.path.join(new, rel)))
    b = json.load(open(os.path.join(old, rel)))
    extra = [r for r in a if r["id"].endswith("#race_hp")]
    rest = [r for r in a if not r["id"].endswith("#race_hp")]
    if "/race/" not in rel or len(extra) != 1 or rest != b:
        bad.append(f"moved: {rel}")
        continue
    r = extra[0]
    if r["target"] != "Hp" or list(r["value"]["Number"].keys()) != ["Const"]:
        bad.append(f"race_hp shape: {rel}")
        continue
    added += 1
    values[r["value"]["Number"]["Const"]] += 1
print(f"race_files={races} race_hp_added={added} values={dict(sorted(values.items()))} other_moves={len(bad)}")
for b_ in bad:
    print("  ", b_)
sys.exit(1 if bad else 0)
