#!/usr/bin/env python3
"""SD-36 Epic F1c render receipt: why each ADDED rule id is held on F1c and not on tranche/16.

  f1c_render_attribute.py <pairs.json> <tranche16 sheet_rules dir> <f1c sheet_rules dir>

For every distinct rule id in pairs.json `added_other` (principal id when the added id is a
`#` sibling), compares the record on both packages:
  new_record        the id does not exist in the tranche/16 package (D3 Unchained records,
                    D6 choice offers, ...)
  new_grant_edge    it exists, but its `granted_by` edges differ (a converter grant change:
                    D1 type grants, D8 pool picks, ...)
  gate_changed      same `granted_by`, different `applies`
  same_record       the record is byte-identical on both: the change is in what the engine
                    holds or prints (D7 always-held seeding, TakenOnClass class lines, or
                    render_sheet's line gate)
Prints one row per id: bucket, id, rows (builds x lines).
"""
import collections, json, os, sys


def load(root, rid):
    book, kind, slug = rid.split("#")[0].split(":")
    p = os.path.join(root, book, kind, slug + ".json")
    if not os.path.exists(p):
        return None
    for r in json.load(open(p)):
        if r["id"] == rid:
            return r
    return None


def main(pairs, t16, f1c):
    rows = json.load(open(pairs))["rows"]["added_other"]
    count = collections.Counter(r[1] for r in rows)
    out = collections.defaultdict(list)
    for rid, n in sorted(count.items()):
        a, b = load(t16, rid), load(f1c, rid)
        if a is None:
            bucket = "new_record"
        elif a.get("granted_by") != b.get("granted_by"):
            bucket = "new_grant_edge"
        elif a.get("applies") != b.get("applies"):
            bucket = "gate_changed"
        elif a == b:
            bucket = "same_record"
        else:
            bucket = "other_field_changed"
        out[bucket].append((rid, n))
    for k, v in out.items():
        print(f"== {k}: {len(v)} ids, {sum(n for _, n in v)} rows")
        for rid, n in v:
            print(f"   {n:4d} {rid}")


if __name__ == "__main__":
    main(*sys.argv[1:4])
