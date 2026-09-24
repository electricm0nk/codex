#!/usr/bin/env python3
"""Enumerate and verify the F3b2b package delta classes (SD-36 Epic F3b2b, converter step 2).

The F3b2b converter change (sd36/epic-f2-f3) moves existing RECORD FIELDS against the tranche/16
package beyond what F3b2 pinned. This script walks every field delta tranche/16 -> fresh, skips the
ones an F3b2 pin already accepts (`structural_diff.f3b2_delta_holds`), classifies the rest with
`structural_diff.f3b2b_classify` (the shape check `structural_diff.py` re-runs), and writes the
pinned (rule id, field, value) triples `structural_diff.py` reads
(`structural_diff_f3b2b_deltas.json`). A delta that fits no class is printed and the script exits 1
without writing.

Classes:

  f3b2b_undeclared_note   `provenance` gains `undeclared_in_pinned_tree` (the names the record reads
                          as Const(0) under oracle semantics) and nothing else. Pinned: the list.
  f3b2b_closure_complete  `closure_complete` absent -> true on a class principal. Pinned: true.

Also pinned: `required_added_edges`, every `granted_by` edge present in the fresh package and absent
from the PRIOR step's package (F3b2, `b1fdc45fca`) -- the references this step newly resolves. The
structural diff only forbids REMOVALS against tranche/16, so without the pin losing one of these
would pass.

Usage (from the repo root):
    python3 f3b2b_delta_pins.py <tranche/16 package dir> data/sheet_rules <out json> <F3b2 package dir>
"""
from __future__ import annotations

import json
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff as sd  # noqa: E402


def main() -> int:
    base_root, fresh_root, out, prior_root = sys.argv[1:5]
    prior = sd.rules_by_id(sd.load_tree(prior_root)["rule_files"])
    base = sd.rules_by_id(sd.load_tree(base_root)["rule_files"])
    fresh = sd.rules_by_id(sd.load_tree(fresh_root)["rule_files"])
    classes: dict[str, list] = defaultdict(list)
    unexplained = []
    for rid in sorted(set(base) & set(fresh)):
        for field in sd.diff_rule(base[rid], fresh[rid]):
            if sd.f3b2_delta_holds(rid, field, base[rid], fresh[rid]) is not None:
                continue
            got = sd.f3b2b_classify(rid, field, base[rid], fresh[rid])
            if got is None:
                unexplained.append((rid, field))
            else:
                classes[got[0]].append([rid, field, got[1]])
    required_edges = []
    for rid in sorted(set(prior) & set(fresh)):
        removed, added = sd.edge_diff(prior[rid].get("granted_by"), fresh[rid].get("granted_by"))
        if removed:
            print(f"REMOVED since the prior step {rid}: {removed}")
            unexplained.append((rid, "granted_by"))
        required_edges.extend([rid, key] for key in added)
    for rid, field in unexplained:
        print(f"UNEXPLAINED {rid}: {field}")
    if unexplained:
        print(f"{len(unexplained)} unexplained field deltas; nothing written")
        return 1
    data = {
        "_purpose": "SD-36 Epic F3b2b: the package field deltas of the F3b2b converter step vs the tranche/16 package that no F3b2 pin accepts, each classified by f3b2b_delta_pins.py; structural_diff.py accepts a field delta only on an exact pinned (rule id, field) whose class shape holds and whose fresh value equals the pin.",
        "_command": "python3 f3b2b_delta_pins.py <tranche/16 package dir> data/sheet_rules structural_diff_f3b2b_deltas.json <F3b2 package dir, git archive b1fdc45fca>",
        "classes": {name: {"_count": len(pins), "pins": pins} for name, pins in sorted(classes.items())},
        "required_added_edges": {"_count": len(required_edges), "pins": required_edges},
    }
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1, sort_keys=False)
        fh.write("\n")
    for name, pins in sorted(classes.items()):
        print(f"{name}: {len(pins)} pins on {len({p[0] for p in pins})} rule ids")
    print(f"required_added_edges: {len(required_edges)} edges on {len({e[0] for e in required_edges})} rule ids")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
