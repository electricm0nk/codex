#!/usr/bin/env python3
"""Enumerate and pin the F7b package deltas (SD-36 Epic F7b, converter step).

Every delta must be a `prose` field delta that `f7b_shapes.classify_prose` explains, or a
`provenance` delta that `f7b_shapes.classify_provenance` explains (module doc of `f7b_shapes.py`
names the classes). Each is pinned as (rule id, field, class, sha256 of the new value). Anything
else -- a rule id added or removed, any other field, a `_vars/` table added or removed -- is
printed and nothing is written.

Usage (from the repo root):
    python3 f7b_delta_pins.py <baseline package dir> <fresh package dir> <out json>
The baseline is the package at tranche/16 HEAD (git archive), the structural diff's own baseline.
"""
from __future__ import annotations

import json
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import f7b_shapes as shapes  # noqa: E402
import structural_diff as sd  # noqa: E402

OWNER = "core_rulebook:spell:fireball"


def main() -> int:
    base_dir, fresh_dir, out_path = sys.argv[1], sys.argv[2], sys.argv[3]
    base, fresh = sd.load_tree(base_dir), sd.load_tree(fresh_dir)
    base_rules, fresh_rules = sd.rules_by_id(base["rule_files"]), sd.rules_by_id(fresh["rule_files"])
    problems: list[str] = []
    if set(base_rules) != set(fresh_rules):
        problems.append(f"rule ids moved: +{len(set(fresh_rules) - set(base_rules))} -{len(set(base_rules) - set(fresh_rules))}")
    bv = {p for p in base["other_files"] if p.startswith("_vars/")}
    fv = {p for p in fresh["other_files"] if p.startswith("_vars/")}
    if bv != fv:
        problems.append(f"_vars/ tables moved: +{sorted(fv - bv)[:5]} -{sorted(bv - fv)[:5]}")
    unconverted = shapes.defect_records(fresh["other_files"], "inline-formula-unconverted")
    former = shapes.defect_records(base["other_files"], "inline-formula-in-prose")
    pins: list[list] = []
    by_class: Counter = Counter()
    fields: Counter = Counter()
    for rid in sorted(set(base_rules) & set(fresh_rules)):
        old, new = base_rules[rid], fresh_rules[rid]
        for field in sd.diff_rule(old, new):
            if field == "prose":
                cls = shapes.classify_prose(rid, old.get("prose", []), new.get("prose", []), rid in unconverted, rid in former)
            elif field == "provenance":
                cls = shapes.classify_provenance(old.get("provenance"), new.get("provenance"))
            else:
                cls = None
            if cls is None:
                problems.append(f"unexplained {rid}: {field}")
                continue
            pins.append([rid, field, cls, sd.f3b2_field_sha(new.get(field))])
            fields[field] += 1
            for c in cls.split("+"):
                by_class[c] += 1
        if sd.edge_diff(old.get("granted_by"), new.get("granted_by")) != ([], []) or old.get("grants") != new.get("grants"):
            problems.append(f"edges or grants moved: {rid}")
    if problems:
        for p in problems[:40]:
            print(p)
        print(f"problems={len(problems)}; nothing written")
        return 1
    out = {
        "owner": OWNER,
        "_note": "SD-36 F7b: prose formulas render as typed pieces; out-of-inventory prose conditions decided (f7b_shapes.py)",
        "counts": {"records": len({p[0] for p in pins}), "by_field": dict(sorted(fields.items())), "by_class": dict(sorted(by_class.items()))},
        "_count": len(pins),
        "pins": pins,
    }
    with open(out_path, "w", encoding="utf-8") as fh:
        json.dump(out, fh, indent=0, sort_keys=False)
        fh.write("\n")
    print(json.dumps(out["counts"], indent=1))
    print(f"pins={len(pins)} -> {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
