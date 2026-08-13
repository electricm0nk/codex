#!/usr/bin/env python3
"""Why each still-`in-progress` equipment / equipment_modifier unit misses its bar.

`in-progress`'s own definition promises "the bar is reachable with an
instrument that exists". `decisions.md` Decision 10 records that this is false
for the overwhelming majority of the bucket. This script is that finding's
derivation: it reads only

  * `docs/work-inventory.json` — the generator's own output
    (`cargo run --release --bin v06_work_inventory`), and
  * the real on-disk `data/corpus/<book>/equipment/` records that
    `probe_equipment_effect_wiring` itself reads,

and partitions the bucket by the reason the probe could not observe a delta.
Nothing here is transcribed from a previous run.

Run:  python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/why-in-progress-equipment-stalls.py
"""
import collections
import json
import os

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", ".."))
CORPUS = os.path.join(REPO, "data", "corpus")
INVENTORY = os.path.join(REPO, "docs", "work-inventory.json")

# Engine book slug -> `data/corpus/` directory name. Mirrors
# `v06_work_inventory.rs`'s `OBSERVABLE_BOOK_DIRS` + `CORPUS_DIR_ALIASES`;
# these are the only books with an ingested equipment corpus at all, which is
# itself one of the findings below.
DIR_FOR_BOOK = {
    "core_rulebook": "core_rulebook",
    "advanced_players_guide": "advanced_players_guide",
    "advanced_class_guide": "advanced_class_guide",
    "advanced_race_guide": "advanced_race_guide",
    "pathfinder_unchained": "pathfinder_unchained",
    "bestiary": "beastiary",
}


def index_book(corpus_dir):
    """key -> record and name -> record, for one book's equipment corpus."""
    idx = {}
    root = os.path.join(CORPUS, corpus_dir, "equipment")
    for dirpath, _, files in os.walk(root):
        for name in files:
            if not name.endswith(".json"):
                continue
            try:
                with open(os.path.join(dirpath, name)) as fh:
                    data = json.load(fh).get("data") or {}
            except (OSError, ValueError):
                continue
            for candidate in (data.get("key"), data.get("name")):
                if candidate:
                    idx.setdefault(candidate, data)
    return idx


def main():
    records = {}
    for book, corpus_dir in DIR_FOR_BOOK.items():
        records[book] = index_book(corpus_dir)
        print(f"corpus records indexed: {book:24} {len(records[book])}")
    print()

    with open(INVENTORY) as fh:
        doc = json.load(fh)
    units = [u for u in doc["units"] if (u.get("book") or "") != "beginner_box"]

    # The `in-progress` population for these two kinds is exactly
    # `computed` + (`ingested-magnitude` | `text-complete`) — see the verdict
    # table in `derive-movable-mass.py`, which validates itself against the
    # live dashboard payload.
    stalled = [u for u in units
               if u["kind"] in ("equipment", "equipment_modifier")
               and u.get("wiring_class") == "computed"
               and u["status"] in ("ingested-magnitude", "text-complete")]

    reasons = collections.Counter()
    families = collections.Counter()
    by_book = collections.Counter()
    for unit in stalled:
        idx = records.get(unit["book"])
        if idx is None:
            reasons["no data/corpus/<book>/equipment directory exists at all"] += 1
            by_book[unit["book"]] += 1
            continue
        record = idx.get(unit["corpus_key"]) or idx.get(unit["name"])
        if record is None:
            reasons["book has a corpus, but no record under this key or name"] += 1
            continue
        chains = record.get("raw_bonus_chains") or []
        if not chains:
            reasons["record resolves, carries NO bonus chain at all"] += 1
            continue
        reasons["record resolves, bonus chain in a family the effect model "
                "does not read"] += 1
        for chain in chains:
            quals = chain.get("qualifiers") or []
            if quals:
                families[quals[0]] += 1

    print(f"still in-progress equipment / equipment_modifier: {len(stalled)}\n")
    for reason, n in sorted(reasons.items(), key=lambda x: -x[1]):
        print(f"  {n:5}  {reason}")
    print("\n  no-corpus books, by book:")
    for book, n in sorted(by_book.items(), key=lambda x: -x[1]):
        print(f"    {n:5}  {book}")
    print("\nBONUS families on records that resolve but stay unwired "
          "(one record may carry several):")
    for family, n in sorted(families.items(), key=lambda x: -x[1])[:20]:
        print(f"  {n:5}  BONUS:{family}")


if __name__ == "__main__":
    main()
