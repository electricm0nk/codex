#!/usr/bin/env python3
"""E4.5 structural delta + price check for the Starfinder package.

usage: E4.5_delta.py <new sheet_rules dir> <baseline sheet_rules dir> <oracle data root>

1. Delta classes over every differing file: `price_row_added` = the only change in the file is
   one added `StatBlock "Price"` prose segment on an equipment principal rule; anything else is
   `other_moves` (must be 0). Rule ids, values, targets and every other field are compared.
2. Second implementation of the price: for every equipment principal rule in the new package,
   the oracle `COST:` tokens on its `provenance.closure_rows` lines (last statement wins) read
   straight from the `.lst` files, compared to the package `Price` row.
Exit 0 only if other_moves == 0 and price mismatches == 0.
"""
import json
import os
import sys
from collections import Counter


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def price_rows(rule):
    out = []
    for seg in rule.get("prose", []) or []:
        fam = seg.get("family")
        if isinstance(fam, dict) and fam.get("StatBlock") == "Price":
            out.append("".join(p.get("Text", "") for p in seg["pieces"] if isinstance(p, dict)))
    return out


def without_price(rule):
    r = dict(rule)
    r["prose"] = [
        s for s in (rule.get("prose") or [])
        if not (isinstance(s.get("family"), dict) and s["family"].get("StatBlock") == "Price")
    ]
    if not r["prose"] and "prose" not in rule:
        r.pop("prose")
    return r


def main():
    new_dir, base_dir, oracle = sys.argv[1], sys.argv[2], sys.argv[3]
    classes = Counter()
    other = []
    files = 0
    for root, _, names in os.walk(new_dir):
        for n in names:
            if not n.endswith(".json"):
                continue
            p = os.path.join(root, n)
            rel = os.path.relpath(p, new_dir)
            b = os.path.join(base_dir, rel)
            if not os.path.exists(b):
                other.append(f"{rel}: new file")
                continue
            with open(p, "rb") as f1, open(b, "rb") as f2:
                if f1.read() == f2.read():
                    continue
            files += 1
            new, old = load(p), load(b)
            if not isinstance(new, list) or "/equipment/" not in "/" + rel:
                other.append(f"{rel}: non-equipment file differs")
                continue
            if [r["id"] for r in new] != [r["id"] for r in old]:
                other.append(f"{rel}: rule ids moved")
                continue
            ok = True
            for rn, ro in zip(new, old):
                if rn == ro:
                    continue
                stripped = without_price(rn)
                if "prose" not in ro and stripped.get("prose") == []:
                    stripped.pop("prose")
                if stripped != ro or "#" in rn["id"] or len(price_rows(rn)) != 1 or price_rows(ro):
                    other.append(f"{rel}: {rn['id']} changed beyond one added Price row")
                    ok = False
            if ok:
                classes["price_row_added"] += 1
    for root, _, names in os.walk(base_dir):
        for n in names:
            rel = os.path.relpath(os.path.join(root, n), base_dir)
            if not os.path.exists(os.path.join(new_dir, rel)):
                other.append(f"{rel}: file removed")

    # Second implementation: oracle COST per equipment principal rule.
    lines_cache = {}

    def oracle_line(spec):
        path, _, num = spec.rpartition(":")
        if path not in lines_cache:
            with open(os.path.join(oracle, path), encoding="utf-8", errors="replace") as f:
                lines_cache[path] = f.read().split("\n")
        return lines_cache[path][int(num) - 1]

    principals = 0
    with_price = 0
    oracle_priced = 0
    mismatches = []
    for root, _, names in os.walk(new_dir):
        if os.path.basename(root) != "equipment":
            continue
        for n in names:
            for rule in load(os.path.join(root, n)):
                if "#" in rule["id"]:
                    continue
                principals += 1
                got = price_rows(rule)
                want = None
                for spec in rule.get("provenance", {}).get("closure_rows", []):
                    for tok in oracle_line(spec).split("\t"):
                        tok = tok.strip()
                        if tok.startswith("COST:"):
                            want = tok[5:]
                if want is not None:
                    oracle_priced += 1
                if got:
                    with_price += 1
                expected = [want] if want is not None and want.isdigit() else []
                if got != expected:
                    mismatches.append(f"{rule['id']}: package {got} oracle COST {want!r}")
    print(f"differing_files={files} " + " ".join(f"{k}={v}" for k, v in sorted(classes.items())) + f" other_moves={len(other)}")
    for o in other[:40]:
        print("  OTHER", o)
    print(f"equipment_principals={principals} oracle_cost_rows={oracle_priced} package_price_rows={with_price} price_mismatches={len(mismatches)}")
    for m in mismatches[:40]:
        print("  MISMATCH", m)
    verdict = "PASS" if not other and not mismatches else "FAIL"
    print(f"verdict={verdict}")
    sys.exit(0 if verdict == "PASS" else 1)


if __name__ == "__main__":
    main()
