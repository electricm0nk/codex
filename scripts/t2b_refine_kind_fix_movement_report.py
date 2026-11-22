#!/usr/bin/env python3
"""SD-32 card 11 T2b classifier-fix cycle -- movement report comparing
`docs/work-inventory.json` before and after `refine_kind`'s KEY-prefix fix
(`decisions.md §16`). Reports movement in BOTH directions (units moved OUT of
`race_trait` and units moved IN, if any), joined by (book, source_file,
source_line) since a kind change also changes the unit's own id.

Run: python3 scripts/t2b_refine_kind_fix_movement_report.py <before.json> <after.json>
"""
import json
import sys
from collections import Counter, defaultdict


def load(path):
    d = json.load(open(path))
    by_coord = {}
    for u in d["units"]:
        coord = (u.get("book"), u.get("source_file"), u.get("source_line"))
        by_coord[coord] = u
    return by_coord


def main():
    before_path, after_path = sys.argv[1], sys.argv[2]
    before = load(before_path)
    after = load(after_path)

    transitions = Counter()
    moved_out_of_race_trait_by_book = defaultdict(int)
    moved_into_race_trait_by_book = defaultdict(int)

    all_coords = set(before) | set(after)
    for coord in all_coords:
        b = before.get(coord)
        a = after.get(coord)
        if b is None or a is None:
            continue  # coordinate appeared/disappeared for an unrelated reason (not this fix's concern)
        bk, ak = b.get("kind"), a.get("kind")
        if bk != ak:
            transitions[(bk, ak)] += 1
            book = coord[0]
            if bk == "race_trait":
                moved_out_of_race_trait_by_book[book] += 1
            if ak == "race_trait":
                moved_into_race_trait_by_book[book] += 1

    print("=== Kind transitions (before -> after), all coordinates present in both files ===")
    for (bk, ak), n in sorted(transitions.items(), key=lambda kv: -kv[1]):
        print(f"{bk} -> {ak}: {n}")

    print()
    print("=== race_trait -> * (moved OUT of race_trait), by book ===")
    total_out = 0
    for book, n in sorted(moved_out_of_race_trait_by_book.items(), key=lambda kv: -kv[1]):
        print(f"  {book}: {n}")
        total_out += n
    print(f"TOTAL moved out of race_trait: {total_out}")

    print()
    print("=== * -> race_trait (moved IN to race_trait), by book ===")
    total_in = 0
    for book, n in sorted(moved_into_race_trait_by_book.items(), key=lambda kv: -kv[1]):
        print(f"  {book}: {n}")
        total_in += n
    print(f"TOTAL moved into race_trait: {total_in}")

    if total_in == 0:
        print()
        print("No units moved INTO race_trait -- this fix is a one-directional widening of the")
        print("MonsterAbility match, never a narrowing of it, confirmed by full join rather than assumed.")


if __name__ == "__main__":
    main()
