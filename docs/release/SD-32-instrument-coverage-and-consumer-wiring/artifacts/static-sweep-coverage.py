#!/usr/bin/env python3
"""How much of the `static`/`held` population the corpus-literal sweep reaches.

`corpus_literal_sweep` answers "do the shipped bytes equal the corpus bytes?"
for every record it examines. It deliberately does not decide anyone's doneness
verdict, and it reports a corpus-wide tally rather than a per-unit result. So a
CLEAN sweep on its own does not tell you *which* `static` units now have the
evidence their bar names -- and "the sweep is green" was read once as "the
static population is cleared", which it is not.

This script joins the two sides and reports three populations, which must be
kept apart because they carry different strengths of evidence:

  TOKEN-COMPARED  the unit has a shipped `data/corpus` record carrying a raw
                  token population, so pass 2b of the sweep byte-compared its
                  magnitudes against the cited `.lst` line. This is the
                  `static` bar, in full.

  DIGEST-ONLY     the unit has a shipped record, but with no token population.
                  Pass 2a verified the cited corpus FILE's sha256, which proves
                  the source did not drift under the record -- it does NOT
                  prove the record's own magnitudes match the literal. Strictly
                  weaker than the bar and never to be counted as meeting it.

  UNREACHED       no shipped record exists at the unit's book/file/line, so the
                  sweep never saw it. Its magnitudes live in a compiled
                  `rules_tables` module instead. No amount of re-running the
                  sweep moves these; they need a corpus record first.

Join key is `(book, source_file, source_line)`. The book on the shipped side is
the LAST path component before the `.lst` filename, NOT a fixed index: corpus
paths are `pathfinder/paizo/<line>/<book>/<file>.lst` and `<line>` is
`roleplaying_game` for some books and `campaign_setting` for others, so a fixed
split index silently returns 0 matches for half the corpus. That bug produced a
confident "0 of 4801 covered" reading on 2026-08-13 before the join was
validated against a file known to be shared by both sides.

Read-only. Reads `docs/work-inventory.json` and `data/corpus/**`, writes nothing.

Run:  python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/static-sweep-coverage.py
"""
import collections
import glob
import json
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", ".."))
INVENTORY = os.path.join(REPO, "docs", "work-inventory.json")
CORPUS = os.path.join(REPO, "data", "corpus")

# Transcribed from the producer's `_doneness_verdict_uncapped()`: the statuses
# it accepts for `static`/`derived`, every one of which it maps to `held`.
HELD_STATUSES = {"ingested-magnitude", "grounded", "text-complete"}
EXCLUDED_BOOKS = {"beginner_box"}


def raw_tokens(record):
    """The record's transcribed token population, wherever it is carried."""
    for container in (record, record.get("data") or {}):
        for key in ("raw_tokens", "tokens"):
            if container.get(key):
                return container[key]
    return None


def load_shipped():
    """Index every data-bearing shipped record by (book, basename, line)."""
    shipped = collections.defaultdict(list)
    for path in glob.glob(os.path.join(CORPUS, "**", "*.json"), recursive=True):
        if os.path.basename(path) == "LICENSE.json":
            continue
        with open(path) as fh:
            record = json.load(fh)
        if not isinstance(record.get("data"), dict):
            continue
        source = record.get("source") or {}
        parts = (source.get("path") or "").split("/")
        book = parts[-2] if len(parts) >= 2 else None
        shipped[(book, os.path.basename(parts[-1] if parts else ""),
                 source.get("line"))].append(record)
    return shipped


def validate_join(units, shipped):
    """Fail loudly rather than report a confident zero.

    A join that matches nothing is far more likely to be a broken key than a
    real absence, and this one already produced exactly that false reading
    once. The assertion is deliberately weak (some overlap must exist), because
    a strong one would encode today's counts.
    """
    matched = sum(1 for u in units
                  if (u["book"], u.get("source_file"), u.get("source_line")) in shipped)
    if matched == 0:
        sys.exit("FAIL: the join matched 0 units. That is a broken key, not a "
                 "finding -- check the book derivation before citing any number.")
    return matched


def main():
    with open(INVENTORY) as fh:
        inventory = json.load(fh)
    units = [u for u in inventory["units"]
             if (u.get("book") or "") not in EXCLUDED_BOOKS]
    static_held = [u for u in units
                   if u.get("wiring_class") == "static" and u["status"] in HELD_STATUSES]

    shipped = load_shipped()
    validate_join(static_held, shipped)

    print(f"work-inventory.json generated_at: {inventory['generated_at']}")
    print(f"static + held units (excluding {sorted(EXCLUDED_BOOKS)}): {len(static_held)}\n")

    buckets = {"TOKEN-COMPARED": collections.Counter(),
               "DIGEST-ONLY": collections.Counter(),
               "UNREACHED": collections.Counter()}
    unreached_books = collections.Counter()
    for unit in static_held:
        key = (unit["book"], unit.get("source_file"), unit.get("source_line"))
        records = shipped.get(key)
        if not records:
            buckets["UNREACHED"][unit["kind"]] += 1
            unreached_books[unit["book"]] += 1
        elif raw_tokens(records[0]):
            buckets["TOKEN-COMPARED"][unit["kind"]] += 1
        else:
            buckets["DIGEST-ONLY"][unit["kind"]] += 1

    for name in ("TOKEN-COMPARED", "DIGEST-ONLY", "UNREACHED"):
        counter = buckets[name]
        print(f"=== {name}: {sum(counter.values())} ===")
        for kind, n in sorted(counter.items(), key=lambda x: -x[1]):
            print(f"  {n:6}  {kind}")
        print()

    print("=== UNREACHED by book (these need a corpus record, not a re-run) ===")
    for book, n in unreached_books.most_common():
        print(f"  {n:6}  {book}")

    print("\nNOTE: none of these units can reach `done` today. The producer's "
          "`doneness_verdict()` table has no `done` rung for `static` -- see "
          "decisions.md Decision 2. TOKEN-COMPARED is the population that would "
          "move if, and only if, that rung is granted.")


if __name__ == "__main__":
    main()
