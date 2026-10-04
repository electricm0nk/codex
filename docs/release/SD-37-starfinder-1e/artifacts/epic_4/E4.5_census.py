#!/usr/bin/env python3
"""E4.5 census (repo root): oracle `COST:` token shapes and package bulk-row shapes.

usage: E4.5_census.py <oracle data root>   (eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)";
                                            python3 E4.5_census.py "$PCGEN_REPO_DIR/data")
1. Every tab field starting `COST:` in `<oracle>/starfinder/**/*.lst` (non-comment lines): integer /
   decimal / other, with the file and first field of every non-integer one.
2. Every equipment principal rule in `data/starfinder-1e/sheet_rules/*/equipment/*.json`: whether
   it has a `StatBlock "Quality"` row reading `Bulk: <x>`, and the shape of <x>.
"""
import glob
import json
import os
import re
import sys
from collections import Counter

oracle = sys.argv[1]
shapes = Counter()
odd = []
for dp, _, fns in os.walk(os.path.join(oracle, "starfinder")):
    for fn in fns:
        if not fn.endswith(".lst"):
            continue
        p = os.path.join(dp, fn)
        for line in open(p, encoding="utf-8", errors="replace"):
            if line.startswith("#"):
                continue
            fields = line.rstrip("\n").split("\t")
            for tok in fields:
                if tok.startswith("COST:"):
                    v = tok[5:]
                    s = "int" if re.fullmatch(r"\d+", v) else ("dec" if re.fullmatch(r"\d*\.\d+", v) else "other")
                    shapes[s] += 1
                    if s != "int":
                        odd.append((os.path.relpath(p, oracle), fields[0], v))
print("oracle COST shapes:", dict(shapes))
for o in odd:
    print("  non-integer:", o)

principals = 0
with_bulk = 0
bulk_shapes = Counter()
bulk_other = []
for f in sorted(glob.glob("data/starfinder-1e/sheet_rules/*/equipment/*.json")):
    for r in json.load(open(f)):
        if "#" in r["id"]:
            continue
        principals += 1
        vals = []
        for seg in r.get("prose", []) or []:
            fam = seg["family"]
            if isinstance(fam, dict) and fam.get("StatBlock") == "Quality":
                t = "".join(x.get("Text", "") for x in seg["pieces"] if isinstance(x, dict))
                head, _, rest = t.partition(":")
                if head.strip().lower() == "bulk":
                    vals.append(rest.strip())
        if vals:
            with_bulk += 1
            v = vals[-1]
            s = "int" if re.fullmatch(r"\d+", v) else (v if v in ("L", "-", "—") else "other")
            bulk_shapes[s] += 1
            if s == "other":
                bulk_other.append((r["id"], v))
print(f"equipment principals={principals} with_bulk_row={with_bulk} without={principals - with_bulk}")
print("bulk shapes:", dict(bulk_shapes))
for o in bulk_other:
    print("  other:", o)
