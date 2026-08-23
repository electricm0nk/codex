#!/usr/bin/env python3
"""SD-32 card 11, shape T9 — re-derive the per-record onboarding backlog census.

Re-derive command (from repo root, worktree base pinned to decisions.md §13's
commit or later, PCGEN_CORPUS_ROOT pointed at the repo-local oracle slot):

    cargo build --locked --release --bin v06_work_inventory
    PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
        <target>/release/v06_work_inventory --stdout-only > fresh_inventory.json
    python3 scripts/sd32_t9_census.py fresh_inventory.json

Filters `docs/work-inventory.json`-shaped output to the six evidence-code
families `epic-breakdown.md`/`THE-BOX.md` name for T9, then breaks the result
down by book and by kind within each book — the fixed onboarding cost is
per-book/per-file, not per-record (docs/retro/ E13 calibration), so a
per-record total is the wrong unit to size work by.
"""
import json
import re
import sys
from collections import defaultdict

EVIDENCE_FAMILIES = {
    "spell": re.compile(r"^spell_key_absent_from_spell_list"),
    "companion": re.compile(r"^companion_absent_from_"),
    "feat": re.compile(r"^feat_key_absent_from_catalog"),
    "monster_ability": re.compile(r"^monster_ability_absent_from_"),
    "equipment": re.compile(r"^equipment_key_absent_from_equipment_tables"),
    "monster": re.compile(r"^monster_absent_from_"),
}


def main() -> None:
    path = sys.argv[1] if len(sys.argv) > 1 else "fresh_inventory.json"
    data = json.load(open(path))
    units = data["units"]

    by_kind = defaultdict(list)
    for u in units:
        kind = u.get("kind")
        ev = u.get("evidence", "") or ""
        pat = EVIDENCE_FAMILIES.get(kind)
        if pat and pat.match(ev):
            by_kind[kind].append(u)

    total = sum(len(v) for v in by_kind.values())
    print("=== T9 totals by kind ===")
    for k in EVIDENCE_FAMILIES:
        print(f"{k}\t{len(by_kind[k])}")
    print(f"TOTAL\t{total}")

    book_kind = defaultdict(lambda: defaultdict(int))
    for k, lst in by_kind.items():
        for u in lst:
            book = u.get("book") or u.get("source_book") or "UNKNOWN"
            book_kind[book][k] += 1

    print()
    print("=== per-book breakdown (book, total, {kind: count}) ===")
    for book in sorted(book_kind):
        kinds = dict(book_kind[book])
        row_total = sum(kinds.values())
        print(f"{book}\t{row_total}\t{kinds}")

    print()
    print(f"books touched: {len(book_kind)}")


if __name__ == "__main__":
    main()
