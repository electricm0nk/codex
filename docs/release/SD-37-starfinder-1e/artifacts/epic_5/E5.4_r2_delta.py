#!/usr/bin/env python3
"""E5.4 attempt-2 structural delta for the Starfinder package (decisions.md §21 (b) and (c)).

usage: E5.4_r2_delta.py <new sheet_rules dir> <baseline sheet_rules dir>

Pinned delta classes (every differing file must fall in one, else it is `other_moves`):
  automatic_grant_waiver  a rule whose ONLY change is `applies`: new = AtLeast{n:1, of:[W..., old]},
                          the old gate kept as the last term and every W = `Holds` a rule that
                          the rule's own `granted_by` names (§21(c): a Starfinder AUTOMATIC grant
                          holds its target whatever the target's own prerequisites say);
  drone_class_hp          `core/class/drone.json`: one new `Hp` line `#bonus1` (the mapping table's
                          `drone_hp_progression` term, §21(b)); every other sibling unchanged
                          except its `#bonusN` suffix moved to `#bonusN+1`;
  report                  `_report.json` (rules +1, degraded records -1, the CURRENTMAX
                          degradation count -1) and `_tokens.json` (only the drone class's
                          degradation entry dropped).
Exit 0 only if other_moves == 0.
"""
import json
import os
import sys
from collections import Counter

NEW, BASE = sys.argv[1], sys.argv[2]


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def files(root):
    out = set()
    for d, _, fs in os.walk(root):
        for f in fs:
            out.add(os.path.relpath(os.path.join(d, f), root))
    return out


def waiver_only(new_rules, old_rules):
    if not isinstance(new_rules, list) or len(new_rules) != len(old_rules):
        return False
    changed = 0
    for n, o in zip(new_rules, old_rules):
        if n == o:
            continue
        if {k: v for k, v in n.items() if k != "applies"} != {k: v for k, v in o.items() if k != "applies"}:
            return False
        a = n["applies"]
        if not (isinstance(a, dict) and "AtLeast" in a and a["AtLeast"]["n"] == 1):
            return False
        of = a["AtLeast"]["of"]
        if of[-1] != o["applies"] or len(of) < 2:
            return False
        granters = {g["by"].get("Rule") for g in n.get("granted_by") or []}
        for w in of[:-1]:
            terms = w["All"] if isinstance(w, dict) and "All" in w else [w]
            holds = [t for t in terms if isinstance(t, dict) and "Holds" in t]
            if len(holds) != 1 or holds[0]["Holds"]["what"].get("Rule") not in granters:
                return False
        changed += 1
    return changed > 0


def drone_class_hp(new_rules, old_rules):
    hp = [r for r in new_rules if r.get("target") == "Hp"]
    if len(hp) != 1 or hp[0]["id"] != "core:class:drone#bonus1" or len(new_rules) != len(old_rules) + 1:
        return False
    rest = [r for r in new_rules if r is not hp[0]]
    for n, o in zip(rest, old_rules):
        o = dict(o)
        if "#bonus" in o["id"]:
            base, k = o["id"].rsplit("#bonus", 1)
            o["id"] = f"{base}#bonus{int(k) + 1}"
        if n != o:
            return False
    return True


def report_ok(new, old):
    n, o = dict(new), dict(old)
    if n.pop("rules_written") != o.pop("rules_written") + 1:
        return False
    if n.pop("degraded_records") != o.pop("degraded_records") - 1:
        return False
    nd, od = n.pop("degraded_by_token_type"), o.pop("degraded_by_token_type")
    key = "BONUS:HP|CURRENTMAX (no Starfinder mapping-table term)"
    if nd.get(key) != od.get(key) - 1 or {k: v for k, v in nd.items() if k != key} != {k: v for k, v in od.items() if k != key}:
        return False
    return n == o


classes = Counter()
other = []
new_files, base_files = files(NEW), files(BASE)
for rel in sorted(new_files | base_files):
    a, b = os.path.join(NEW, rel), os.path.join(BASE, rel)
    if rel not in new_files or rel not in base_files:
        other.append(f"{rel}: {'added' if rel in new_files else 'removed'}")
        continue
    with open(a, "rb") as fa, open(b, "rb") as fb:
        if fa.read() == fb.read():
            continue
    if rel == "_report.json":
        if report_ok(load(a), load(b)):
            classes["report"] += 1
            continue
    elif rel == "_tokens.json":
        na, nb = open(a, encoding="utf-8").read().splitlines(), open(b, encoding="utf-8").read().splitlines()
        diff = [(x, y) for x, y in zip(na, nb) if x != y]
        if len(na) == len(nb) and len(diff) == 1 and '"id":"core\\u003aclass\\u003adrone"' in diff[0][0] \
                and json.loads(diff[0][1].rstrip(",")).get("degradations") and "degradations" not in json.loads(diff[0][0].rstrip(",")):
            classes["report"] += 1
            continue
    elif rel == os.path.join("core", "class", "drone.json"):
        if drone_class_hp(load(a), load(b)):
            classes["drone_class_hp"] += 1
            continue
    else:
        if waiver_only(load(a), load(b)):
            classes["automatic_grant_waiver"] += 1
            continue
    other.append(rel)

for k in sorted(classes):
    print(f"{k}: {classes[k]}")
print(f"other_moves={len(other)}")
for o in other:
    print(f"  OTHER {o}")
sys.exit(0 if not other else 1)
