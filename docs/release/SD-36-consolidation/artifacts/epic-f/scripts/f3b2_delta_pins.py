#!/usr/bin/env python3
"""Enumerate and verify the F3b2 package delta classes (SD-36 Epic F3b2, converter step).

The F3b2 converter change (sd36/epic-f2-f3) changes existing RECORD FIELDS against the tranche/16
package. This script classifies every field delta into exactly one class with
`structural_diff.f3b2_classify` (the same shape check `structural_diff.py` re-runs), and writes the
pinned (rule id, field, value) triples `structural_diff.py` reads
(`structural_diff_f3b2_deltas.json`). A delta that fits no class -- other than the growth-only
`granted_by`/`grants` fields `structural_diff.py` diffs as edge sets -- is printed and the script
exits 1 without writing.

Classes:

  f3b2_skill_ranks               `prose` gains exactly one `StatBlock "Skill ranks per level"` row of
                                 one plain-number Text piece (from STARTSKILLPTS). Pinned: the number.
  f3b2_closure_complete          `closure_complete` absent -> true on a class principal. Pinned: true.
  f3b2_placeholder_key_resolved  `applies`/`prose` identical except MissingRule -> Rule nodes (a
                                 reference to a placeholder-keyed record now resolving). Pinned: the
                                 fresh field's sha256.

Usage (from the repo root):
    python3 f3b2_delta_pins.py <tranche/16 package dir> data/sheet_rules <out json>
"""
from __future__ import annotations

import json
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff as sd  # noqa: E402


def main() -> int:
    base_root, fresh_root, out = sys.argv[1:4]
    base = sd.rules_by_id(sd.load_tree(base_root)["rule_files"])
    fresh = sd.rules_by_id(sd.load_tree(fresh_root)["rule_files"])
    classes: dict[str, list] = defaultdict(list)
    unexplained = []
    for rid in sorted(set(base) & set(fresh)):
        for field in sd.diff_rule(base[rid], fresh[rid]):
            got = sd.f3b2_classify(rid, field, base[rid], fresh[rid])
            if got is None:
                unexplained.append((rid, field))
            else:
                classes[got[0]].append([rid, field, got[1]])
    for rid, field in unexplained:
        print(f"UNEXPLAINED {rid}: {field}")
    if unexplained:
        print(f"{len(unexplained)} unexplained field deltas; nothing written")
        return 1
    data = {
        "_purpose": "SD-36 Epic F3b2: the package field deltas of the F3b2 converter step vs the tranche/16 package, each classified by f3b2_delta_pins.py; structural_diff.py accepts a field delta only on an exact pinned (rule id, field) whose class shape holds and whose fresh value equals the pin.",
        "_command": "python3 f3b2_delta_pins.py <tranche/16 package dir> data/sheet_rules structural_diff_f3b2_deltas.json",
        "classes": {name: {"_count": len(pins), "pins": pins} for name, pins in sorted(classes.items())},
    }
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1, sort_keys=False)
        fh.write("\n")
    for name, pins in sorted(classes.items()):
        print(f"{name}: {len(pins)} pins on {len({p[0] for p in pins})} rule ids")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
