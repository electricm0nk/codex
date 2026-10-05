#!/usr/bin/env python3
"""E5.4 structural delta for the Starfinder package (the drone companion).

usage: E5.4_delta.py <new sheet_rules dir> <baseline sheet_rules dir>

Pinned delta classes (every differing file must fall in one, else it is `other_moves`):
  companion_mod_added      a new `<book>/companion_mod/<role>.json` file;
  drone_pool_option_added  a new `pool_option` file whose id is a drone pick member
                           (`drone_chassis_selection_`, `drone_skill_unit_`, `drone_feat_`);
  grant_edge_added         a rule whose ONLY change is `granted_by` entries added (the old list a
                           subsequence of the new), each by a
                           NEW rule (a companion modifier / new pool option) or by the drone race
                           (`MONSTERCLASS`, the drone class);
  offers_added             a rule whose ONLY change is an `offers` added over the drone skill unit
                           pool (`internal` / `Drone Skill Unit`);
  drone_race_speed         `core:race:drone`: its `Speed` rows, and nothing else;
  var_table                a `_vars/` table whose change is only contributions/declarers from new
                           rules, or a new table; `var_names.json` new names only;
  defects                  `_defects/` rows added that name only new drone rules or the drone race
                           (its `Swim` variable, which no row DEFINEs), or the new
                           `companion-mod-role-unoffered.json`; `_report.json` (counts).
Exit 0 only if other_moves == 0.
"""
import json
import os
import sys
from collections import Counter

NEW, BASE = sys.argv[1], sys.argv[2]
DRONE_OPTION_PREFIXES = ("core:pool_option:drone_chassis_selection_", "core:pool_option:drone_skill_unit_", "core:pool_option:drone_feat_")


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def files(root):
    out = set()
    for d, _, fs in os.walk(root):
        for f in fs:
            out.add(os.path.relpath(os.path.join(d, f), root))
    return out


new_files, base_files = files(NEW), files(BASE)
added = sorted(new_files - base_files)
removed = sorted(base_files - new_files)
changed = sorted(f for f in new_files & base_files if open(os.path.join(NEW, f), "rb").read() != open(os.path.join(BASE, f), "rb").read())

new_rule_ids = set()
for f in added:
    if f.startswith("_"):
        continue
    for r in load(os.path.join(NEW, f)):
        new_rule_ids.add(r["id"])

classes = Counter()
other = []
for f in removed:
    other.append(("removed", f))
for f in added:
    if "/companion_mod/" in f:
        classes["companion_mod_added"] += 1
    elif "/pool_option/" in f and all(r["id"].startswith(DRONE_OPTION_PREFIXES) for r in load(os.path.join(NEW, f))):
        classes["drone_pool_option_added"] += 1
    elif f.startswith("_vars/"):
        classes["var_table"] += 1
    elif f == "_defects/companion-mod-role-unoffered.json":
        classes["defects"] += 1
    else:
        other.append(("added", f))


def granter(g):
    by = g["by"]
    return by.get("Rule") or by.get("Choice")


for f in changed:
    a, b = load(os.path.join(NEW, f)), load(os.path.join(BASE, f))
    if f == "_report.json":
        classes["defects"] += 1
        continue
    if f.startswith("_defects/"):
        extra = [x for x in a if x not in b]
        if all(x in a for x in b) and extra and all(any(n in x for n in new_rule_ids | {"core:race:drone"}) for x in extra):
            classes["defects"] += 1
        else:
            other.append(("defects", f))
        continue
    if f.startswith("_vars/"):
        drop = lambda t: {k: v for k, v in t.items() if k not in ("contributions", "declared_by", "provenance")}
        extra_c = [c for c in a["contributions"] if c not in b["contributions"]]
        extra_d = [d for d in a["declared_by"] if d not in b["declared_by"]]
        if drop(a) == drop(b) and all(c in a["contributions"] for c in b["contributions"]) and all(c["rule_id"] in new_rule_ids for c in extra_c) and all(d in new_rule_ids for d in extra_d):
            classes["var_table"] += 1
        else:
            other.append(("var_table", f))
        continue
    if f == "var_names.json":
        if all(a.get(k) == v for k, v in b.items()):
            classes["var_table"] += 1
        else:
            other.append(("var_names", f))
        continue
    by_id_b = {r["id"]: r for r in b}
    ok = len(a) == len(b) and set(by_id_b) == {r["id"] for r in a}
    kinds = set()
    for r in a if ok else []:
        o = by_id_b[r["id"]]
        if r == o:
            continue
        diff = {k for k in set(r) | set(o) if r.get(k) != o.get(k)}
        if diff == {"granted_by"}:
            gb_new, gb_old = r.get("granted_by") or [], o.get("granted_by") or []
            it = iter(gb_new)
            subsequence = all(any(g == x for x in it) for g in gb_old)
            extra = [g for g in gb_new if g not in gb_old]
            if subsequence and len(gb_new) == len(gb_old) + len(extra) and extra and all(granter(g) in new_rule_ids or granter(g) == "core:race:drone" for g in extra):
                kinds.add("grant_edge_added")
                continue
        if diff == {"offers"} and o.get("offers") is None and r["offers"]["from"].get("Rules", {}).get("tags") == ["Drone Skill Unit"]:
            kinds.add("offers_added")
            continue
        if r["id"] == "core:race:drone" and diff == {"prose"}:
            keep = lambda segs: [s for s in segs if s.get("family") != {"StatBlock": "Speed"}]
            if keep(r["prose"]) == keep(o.get("prose") or []):
                kinds.add("drone_race_speed")
                continue
        ok = False
    if ok and kinds:
        for k in kinds:
            classes[k] += 1
    else:
        other.append(("rule", f))

print("added", len(added), "removed", len(removed), "changed", len(changed))
for k, v in sorted(classes.items()):
    print(f"class {k}: {v} files")
print(f"other_moves={len(other)}")
for o in other[:40]:
    print("  OTHER", *o)
sys.exit(0 if not other else 1)
