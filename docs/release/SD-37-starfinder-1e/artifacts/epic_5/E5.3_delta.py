#!/usr/bin/env python3
"""E5.3 structural delta + oracle check for the Starfinder package (equipment modifiers).

usage: E5.3_delta.py <new sheet_rules dir> <baseline sheet_rules dir> <oracle data root>

1. Delta classes over every differing file. A file is classified only if it is an
   `equipment_modifier` principal whose change is exactly, per segment:
     price_row_added   one `StatBlock "Price"` segment added;
     item_level_row    a `Special` "ItemLevel=<n>" segment became `StatBlock "Item level"` <n>;
     bulk_row          a `Special` "Bulk=<x>"/"BULK=<x>" segment became `StatBlock "Quality"`
                       "Bulk: <x>";
   with every other segment (in order) and every other field byte-equal. A new
   `_defects/sf-price-unresolved.json` is checked row by row (each row must name an
   equipment modifier whose oracle COST is not one whole number). Anything else is
   `other_moves` (must be 0).
2. Second implementation of the values: for every equipment-modifier principal in the new
   package, the oracle `COST:` and `SPROP:ItemLevel=`/`SPROP:Bulk=` tokens on its
   `provenance.closure_rows` lines, read straight from the `.lst` files, compared to the
   package rows (a non-integer COST must print no Price row).
Exit 0 only if other_moves == 0 and mismatches == 0.
"""
import json
import os
import re
import sys
from collections import Counter


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def text(seg):
    return "".join(p.get("Text", "") for p in seg["pieces"] if isinstance(p, dict))


def stat(seg):
    fam = seg.get("family")
    return fam.get("StatBlock") if isinstance(fam, dict) else None


SPROP_STAT = re.compile(r"^(ItemLevel|Bulk|BULK)=(\d+|L|-)$")


def classify(old, new):
    """Delta classes of one rule pair, or None if the change is anything else."""
    rest_old = []
    moved = Counter()
    for seg in old.get("prose", []) or []:
        m = SPROP_STAT.match(text(seg)) if seg.get("family") == "Special" else None
        if m:
            moved["item_level_row" if m.group(1) == "ItemLevel" else "bulk_row"] += 1
            continue
        rest_old.append(seg)
    rest_new = []
    added = Counter()
    for seg in new.get("prose", []) or []:
        s = stat(seg)
        if s == "Price":
            added["price_row_added"] += 1
            continue
        if s == "Item level" and re.fullmatch(r"\d+", text(seg)):
            added["item_level_row"] += 1
            continue
        if s == "Quality" and re.fullmatch(r"Bulk: (\d+|L|-)", text(seg)):
            added["bulk_row"] += 1
            continue
        rest_new.append(seg)
    if rest_old != rest_new or added["price_row_added"] > 1:
        return None
    if added["item_level_row"] != moved["item_level_row"] or added["bulk_row"] != moved["bulk_row"]:
        return None
    o, n = dict(old), dict(new)
    o.pop("prose", None)
    n.pop("prose", None)
    if o != n:
        return None
    return added


def main():
    new_dir, base_dir, oracle = sys.argv[1], sys.argv[2], sys.argv[3]
    classes = Counter()
    other = []
    files = 0
    defect_rows = []
    for root, _, names in os.walk(new_dir):
        for n in names:
            if not n.endswith(".json"):
                continue
            p = os.path.join(root, n)
            rel = os.path.relpath(p, new_dir)
            b = os.path.join(base_dir, rel)
            if rel == os.path.join("_defects", "sf-price-unresolved.json") and not os.path.exists(b):
                defect_rows = load(p)
                classes["price_unresolved_defect_rows"] += len(defect_rows)
                continue
            if not os.path.exists(b):
                other.append(f"{rel}: new file")
                continue
            with open(p, "rb") as f1, open(b, "rb") as f2:
                if f1.read() == f2.read():
                    continue
            files += 1
            new, old = load(p), load(b)
            if not isinstance(new, list) or "/equipment_modifier/" not in "/" + rel:
                other.append(f"{rel}: a file other than an equipment modifier differs")
                continue
            if [r["id"] for r in new] != [r["id"] for r in old]:
                other.append(f"{rel}: rule ids moved")
                continue
            for rn, ro in zip(new, old):
                if rn == ro:
                    continue
                got = classify(ro, rn) if "#" not in rn["id"] else None
                if got is None:
                    other.append(f"{rel}: {rn['id']} changed beyond the price/level/bulk rows")
                    continue
                classes.update({k: v for k, v in got.items() if v})
                classes["records_moved"] += 1
    for root, _, names in os.walk(base_dir):
        for n in names:
            rel = os.path.relpath(os.path.join(root, n), base_dir)
            if not os.path.exists(os.path.join(new_dir, rel)):
                other.append(f"{rel}: file removed")

    # Second implementation: the oracle tokens per equipment-modifier principal.
    lines_cache = {}

    def oracle_line(spec):
        path, _, num = spec.rpartition(":")
        if path not in lines_cache:
            with open(os.path.join(oracle, path), encoding="utf-8", errors="replace") as f:
                lines_cache[path] = f.read().split("\n")
        return lines_cache[path][int(num) - 1]

    principals = 0
    counts = Counter()
    mismatches = []
    for root, _, names in os.walk(new_dir):
        if os.path.basename(root) != "equipment_modifier":
            continue
        for n in names:
            for rule in load(os.path.join(root, n)):
                if "#" in rule["id"]:
                    continue
                principals += 1
                cost, level, bulk = None, [], []
                for spec in rule.get("provenance", {}).get("closure_rows", []):
                    for tok in oracle_line(spec).split("\t"):
                        tok = tok.strip()
                        if tok.startswith("COST:"):
                            cost = tok[5:].strip()
                        m = SPROP_STAT.match(tok[6:]) if tok.startswith("SPROP:") else None
                        if m and m.group(1) == "ItemLevel":
                            level.append(m.group(2))
                        elif m:
                            bulk.append("Bulk: " + m.group(2))
                segs = rule.get("prose", []) or []
                got_price = [text(s) for s in segs if stat(s) == "Price"]
                got_level = [text(s) for s in segs if stat(s) == "Item level"]
                got_bulk = [text(s) for s in segs if stat(s) == "Quality" and text(s).startswith("Bulk: ")]
                want_price = [cost] if cost is not None and re.fullmatch(r"\d+", cost) else []
                counts["oracle_cost"] += cost is not None
                counts["package_price_rows"] += len(got_price)
                counts["package_item_level_rows"] += len(got_level)
                counts["package_bulk_rows"] += len(got_bulk)
                if got_price != want_price or got_level != level or got_bulk != bulk:
                    mismatches.append(f"{rule['id']}: package {got_price}/{got_level}/{got_bulk} oracle {cost!r}/{level}/{bulk}")
                if cost is not None and not want_price and not any(rule["id"] in row for row in defect_rows):
                    mismatches.append(f"{rule['id']}: oracle COST {cost!r} unresolved but not in the defect file")
    for row in defect_rows:
        if ":equipment_modifier:" not in row:
            other.append(f"_defects/sf-price-unresolved.json: row {row!r} is not an equipment modifier")
    print(f"differing_files={files} " + " ".join(f"{k}={v}" for k, v in sorted(classes.items())) + f" other_moves={len(other)}")
    for o in other[:40]:
        print("  OTHER", o)
    print(f"equipment_modifier_principals={principals} " + " ".join(f"{k}={v}" for k, v in sorted(counts.items())) + f" mismatches={len(mismatches)}")
    for m in mismatches[:40]:
        print("  MISMATCH", m)
    verdict = "PASS" if not other and not mismatches else "FAIL"
    print(f"verdict={verdict}")
    sys.exit(0 if verdict == "PASS" else 1)


if __name__ == "__main__":
    main()
