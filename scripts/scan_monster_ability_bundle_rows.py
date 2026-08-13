#!/usr/bin/env python3
"""Measure the `ABILITY:Internal|AUTOMATIC|` ownership class across the monster lane.

# Why this exists

SD-29 `decisions.md §62.4` found an ability-ownership shape that neither
`scripts/classify_monster_ability_rows.py`'s row-named pass nor its
namespaced-prefix pass can see, and asked a successor to scan for it::

    **The predicate to scan other books for is an `ABILITY:Internal|AUTOMATIC|`
    token on a monster row.** Unscanned as of this round; scanning it is cheap
    and is the next round's first move.

Round 10 ran that scan.  §62.4 measured **16** units, in one file of one book,
because that is the file it was looking at.  The class is **229 units across
six books** -- five of them already registered.  This script is the scan, checked
in rather than left in a scratch path, because `§45.1`'s finding is that an
ephemeral path is not a citation: the round-1 shape table was derived by a
`/tmp` script that no longer exists, and a successor could not re-derive it.

# The shape

A monster row may state its abilities INDIRECTLY::

    support/isg_races_b4.lst:6            The First Blade
        ABILITY:Internal|AUTOMATIC|Race Traits ~ First Blade
    support/isg_abilities_races_b4.lst:8  Race Traits ~ First Blade  CATEGORY:Internal
        ABILITY:Special Ability|AUTOMATIC|…|First Blade ~ Powerful Blows (Slam)|…

The monster row names a ``CATEGORY:Internal`` **bundle** row; the bundle row
names the individual abilities.  The classifier's row-named pass reads
``ABILITY:Special Ability|AUTOMATIC|`` tokens *on monster rows* and never sees
the bundle; its prefix pass matches an ability namespace against a monster
**key**, and the namespaces here are the creature's short name while the keys are
longer.  Neither pass reaches them, so both call the rows orphans.

# What it prints, and what the number means

Per book: how many rows the classifier currently calls ORPHAN the bundle hop
would reach.  It is a **ceiling correction in the expensive direction** -- these
are units the lane could ship and does not, not units it has miscounted as
shipped.

**Following the hop is a mechanism change, not an ingest.**  It widens an
ownership pass, which changes which records five already-registered books ship
-- the "count change needs a sweep" hazard at its worst.  `§62.4` deferred it for
that reason and round 10 did the same, now with the real number attached.

Usage::

    python3 scripts/scan_monster_ability_bundle_rows.py            # every book with remaining units
    python3 scripts/scan_monster_ability_bundle_rows.py inner_sea_gods ...

Run from the repo root.  ``PCGEN_CORPUS_ROOT`` may point at a local PCGen
``data/`` checkout; it defaults to ``$HOME/workspace/repos/pcgen/data``.
"""

from __future__ import annotations

import importlib.util
import json
import os
import re
import sys

REPO_CLASSIFIER = "scripts/classify_monster_ability_rows.py"
INVENTORY = "docs/work-inventory.json"

# The token a monster row uses to name a `CATEGORY:Internal` bundle row.
BUNDLE_TOKEN = re.compile(r"ABILITY:Internal\|AUTOMATIC\|([^\t|]*)")
# `CATEGORY=Internal|<key>` and `<key>.MOD` are both spellings of the same
# bundle row's first column; strip both before comparing.
CATEGORY_PREFIX = re.compile(r"^CATEGORY=[^|]*\|")


def load_classifier():
    """Reuse the lane's own classifier rather than re-deriving ``orphan``.

    The whole point of the number is that it is measured against the screen the
    lane actually ships; a second, private definition of ``orphan`` here would
    let the two drift and would make the finding unfalsifiable.
    """
    spec = importlib.util.spec_from_file_location("monster_row_classifier", REPO_CLASSIFIER)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def resolve_book_file(book_dir: str, basename: str) -> str | None:
    """The transcriber's and generator's `resolve_book_file`, third spelling.

    A citation is a bare basename, not a path (`decisions.md §62.1`), so the
    file may sit under `support/`. Refuses an ambiguous basename rather than
    picking one, for the same reason the other two do.
    """
    hits = []
    for dirpath, _dirnames, filenames in os.walk(book_dir):
        if basename in filenames:
            hits.append(os.path.join(dirpath, basename))
    return hits[0] if len(hits) == 1 else None


def scan_book(book: str, units: list[dict], directory: str | None, orphans: set[str]) -> set[str]:
    if directory is None or not orphans:
        return set()
    monster_files = sorted({u["source_file"] for u in units if u["kind"] == "monster"})
    ability_files = sorted({u["source_file"] for u in units if u["kind"] == "monster_ability"})

    bundles: set[str] = set()
    for name in monster_files:
        path = resolve_book_file(directory, name)
        if path is None:
            continue
        with open(path, encoding="utf-8", errors="replace") as handle:
            for line in handle:
                bundles.update(BUNDLE_TOKEN.findall(line))
    if not bundles:
        return set()

    reached: set[str] = set()
    for name in ability_files:
        path = resolve_book_file(directory, name)
        if path is None:
            continue
        with open(path, encoding="utf-8", errors="replace") as handle:
            for line in handle:
                first_column = line.split("\t")[0]
                key = CATEGORY_PREFIX.sub("", first_column).split(".MOD")[0]
                if key not in bundles:
                    continue
                for token in re.findall(r"ABILITY:[^\t]*", line):
                    # `ABILITY:<category>|<nature>|<key>|<key>|…`, with `PRE`
                    # guards mixed in among the keys.
                    for part in token.split("|")[2:]:
                        part = part.strip()
                        if part.startswith("PRE") or part.startswith("!PRE"):
                            continue
                        if part in orphans:
                            reached.add(part)
    return reached


def main() -> None:
    classifier = load_classifier()
    inventory = json.load(open(INVENTORY, encoding="utf-8"))
    out_of_scope = {b["id"] for b in inventory["books"] if b["scope"] == "out_of_scope"}
    directories = classifier.book_dirs()

    by_book: dict[str, list[dict]] = {}
    for unit in inventory["units"]:
        if unit["kind"] in ("monster", "monster_ability") and unit["book"] not in out_of_scope:
            by_book.setdefault(unit["book"], []).append(unit)

    wanted = set(sys.argv[1:])
    total = 0
    print(f"{'book':26} {'orphans':>8} {'bundle-reachable':>17}")
    for book in sorted(by_book):
        if wanted and book not in wanted:
            continue
        report = classifier.classify_book(book, by_book[book], directories.get(book))
        orphans = set(report["orphan_keys"])
        reached = scan_book(book, by_book[book], directories.get(book), orphans)
        if reached:
            print(f"{book:26} {len(orphans):>8} {len(reached):>17}")
            total += len(reached)
    print()
    print(f"orphan rows the `ABILITY:Internal|AUTOMATIC|` hop would reach: {total}")
    print("These are a CEILING CORRECTION, not a backlog line: following the hop")
    print("widens an ownership pass and changes what every registered book ships.")


if __name__ == "__main__":
    main()
