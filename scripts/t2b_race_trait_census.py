#!/usr/bin/env python3
"""SD-32 card 11 lane T2b census — book-by-book breakdown of the 2,472 open
race_trait units, re-derived fresh (no doc-quoted list) against
docs/work-inventory.json and apps/desktop/src-tauri/src/race_catalog.rs's
RACE_CORPUS_BOOKS. See docs/release/SD-32-compute-library-and-cause-closure/
artifacts/gate-3-closure-invariant/card11-t2b-census-census.md for the full
memo this script backs.
"""
import json
import re
import sys
from collections import Counter

RACE_CATALOG = "apps/desktop/src-tauri/src/race_catalog.rs"
WORK_INVENTORY = "docs/work-inventory.json"


def load_race_corpus_books():
    src = open(RACE_CATALOG).read()
    m = re.search(r"RACE_CORPUS_BOOKS:\s*&\[&str\]\s*=\s*&\[(.*?)\];", src, re.S)
    assert m, "RACE_CORPUS_BOOKS not found"
    body = m.group(1)
    return set(re.findall(r'"([a-z0-9_]+)"', body))


def load_residual_units():
    d = json.load(open(WORK_INVENTORY))
    return [
        x for x in d["units"]
        if x.get("kind") == "race_trait" and x.get("evidence") == "race_trait_race_not_modelled"
    ]


def main():
    registered = load_race_corpus_books()
    units = load_residual_units()
    by_book = Counter(u["book"] for u in units)

    print(f"# total residual T2b units: {len(units)}")
    print(f"# RACE_CORPUS_BOOKS registered ({len(registered)}): {sorted(registered)}")
    print()

    unreg_total = 0
    reg_total = 0
    print("book,count,registered_in_race_catalog")
    for book, n in sorted(by_book.items(), key=lambda kv: -kv[1]):
        is_reg = book in registered
        print(f"{book},{n},{is_reg}")
        if is_reg:
            reg_total += n
        else:
            unreg_total += n

    print()
    print(f"# unregistered-book pile: {unreg_total}")
    print(f"# registered-book pile: {reg_total}")
    print(f"# sum check: {unreg_total + reg_total} (must equal {len(units)})")

    # Sub-split the registered-book (718-shaped) pile: category-header rows
    # (no race named -- name matches known category-header patterns) vs
    # "Adopted Race ~ <X>" selector rows vs other.
    header_patterns = [
        re.compile(r"^Racial SLA ~ "),
        re.compile(r"^Unchained Evolution ~ "),
        re.compile(r"^Favored Class Bonus ~ "),
        re.compile(r"^Race Subtype ~ "),
    ]
    reg_units = [u for u in units if u["book"] in registered]
    header_count = 0
    adopted_count = 0
    other_count = 0
    header_rows = []
    adopted_rows = []
    other_rows = []
    for u in reg_units:
        key = u.get("corpus_key", "") or u.get("name", "")
        if key.startswith("Adopted Race ~ "):
            adopted_count += 1
            adopted_rows.append(u)
        elif any(p.match(key) for p in header_patterns):
            header_count += 1
            header_rows.append(u)
        else:
            other_count += 1
            other_rows.append(u)

    print()
    print(f"# registered-book pile breakdown (of {reg_total}):")
    print(f"#   category-header rows (by-design exclusion candidates): {header_count}")
    print(f"#   'Adopted Race ~ <X>' selector rows: {adopted_count}")
    print(f"#   other (uncategorized): {other_count}")

    if "--dump-other" in sys.argv:
        print()
        print("# OTHER rows (uncategorized) -- book: corpus_key:")
        for u in other_rows:
            print(f"#   {u['book']}: {u.get('corpus_key', u.get('name'))}")

    if "--dump-adopted-books" in sys.argv:
        print()
        c = Counter(u["book"] for u in adopted_rows)
        print("# Adopted Race rows by book:")
        for b, n in sorted(c.items(), key=lambda kv: -kv[1]):
            print(f"#   {b}: {n}")


if __name__ == "__main__":
    main()
