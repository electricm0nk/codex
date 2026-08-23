#!/usr/bin/env python3
"""SD-32 card 11, T2b classifier-fix cycle -- corpus-wide stress test of the
KEY-prefix discriminator before it is wired into `refine_kind`
(`src/bin/v06_work_inventory.rs`).

Hypothesis under test (from `decisions.md §16` and the w1-d lane's
`t2b-bestiary_3-measurement-receipt.md` §6 item 1): a `_abilities_race.lst`
row is monster/template content, not a real racial trait, when its `KEY:`
prefix (the text before the first ` ~ `) exactly matches the first field of a
`CR:`-bearing row in the SAME book's own `*_races.lst`, or the first field of
ANY row in the same book's own `*_templates.lst`.

This script re-derives, for EVERY book in the pinned oracle that carries an
`*_abilities_race.lst` file (not just the 26 unregistered T2b books), whether
that hypothesis:

1. Ever matches a KNOWN-real player racial trait row -- the known trap named
   in the w1-d receipt (`Favored Enemy ~ Humanoid (<Race>)`, whose OWN TYPE
   shares an inner `SpecialAttack` dot-segment with the monster vocabulary)
   plus every other real trait row belonging to a book's own playable races
   (cross-referenced from `RACE_CORPUS_BOOKS`/`IN_SCOPE_RACES`-shaped books:
   core_rulebook, advanced_players_guide, advanced_race_guide, bestiary_2,
   bestiary_5, bestiary_6, inner_sea_races, core_essentials).
2. How many rows it moves book-wide, corroborating (or not) the w1-d receipt's
   683-exact-match finding for bestiary_3 specifically.

Run: python3 scripts/t2b_refine_kind_key_prefix_stress_test.py
"""
import os
import re
import sys

# "on" (default) matches CR:-bearing races.lst names AND templates.lst names.
# "off" matches CR:-bearing races.lst names only -- pass `off` as argv[1] to
# isolate templates.lst's contribution (and its false-positive risk) from the
# races.lst signal.
TEMPLATES_MODE = sys.argv[1] if len(sys.argv) > 1 else "on"

ORACLE_ROOT = (
    "docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/"
    "operator-supplied/pcgen/data/pathfinder"
)

# Real playable-race books this program ingests race_trait from today --
# cross-referenced from `RACE_CORPUS_BOOKS`/`IN_SCOPE_RACES` doc comments in
# `race_catalog.rs`/`ingest_races.rs`. These are the books where a false
# positive would be most dangerous (real content in scope).
KNOWN_RACE_BOOKS_DIRS = {
    "core_rulebook": "paizo/roleplaying_game/core_rulebook",
    "advanced_players_guide": "paizo/roleplaying_game/advanced_players_guide",
    "advanced_race_guide": "paizo/roleplaying_game/advanced_race_guide",
    "advanced_class_guide": "paizo/roleplaying_game/advanced_class_guide",
    # NOTE: bestiary/bestiary_1 is deliberately NOT listed here. It is not a
    # real playable-race book -- confirmed corpus-wide (0 CR:-less
    # `b1_races.lst` entries; `not-ingested-figures-are-classifier-noise`
    # memory note) -- every one of its 528 CR:-only hits below is a correct
    # reclassification (monster special-ability content), not a false
    # positive. Listing it here would just relabel real work as "trap found".
    "bestiary_2": "paizo/roleplaying_game/bestiary_2",
    "bestiary_5": "paizo/roleplaying_game/bestiary_5",
    "bestiary_6": "paizo/roleplaying_game/bestiary_6",
    "inner_sea_races": "paizo/campaign_setting/inner_sea_races",
    "core_essentials": "paizo/roleplaying_game/core_essentials",
    "ultimate_wilderness": "paizo/roleplaying_game/ultimate_wilderness",
}


def find_book_dirs():
    """Every directory under the oracle root that carries at least one
    `*_abilities_race.lst` file, recursively -- not assumed from a fixed
    list."""
    dirs = set()
    for root, _, files in os.walk(ORACLE_ROOT):
        for f in files:
            if "_abilities_race" in f and f.endswith(".lst") and "companion" not in f and "familiar" not in f:
                dirs.add(root)
    return sorted(dirs)


def read_lst_rows(path):
    rows = []
    if not os.path.exists(path):
        return rows
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            if line.startswith("#") or not line.strip():
                continue
            fields = line.rstrip("\n").split("\t")
            first = fields[0].strip()
            if not first or first.startswith("SOURCELONG") or first.startswith("###"):
                continue
            rows.append((first, fields))
    return rows


def cr_bearing_race_names(dir_path):
    names = set()
    for f in os.listdir(dir_path) if os.path.isdir(dir_path) else []:
        if f.endswith("_races.lst") and "companion" not in f and "familiar" not in f:
            for first, fields in read_lst_rows(os.path.join(dir_path, f)):
                if any("CR:" in field for field in fields):
                    names.add(first)
    return names


def template_names(dir_path):
    names = set()
    for f in os.listdir(dir_path) if os.path.isdir(dir_path) else []:
        if f.endswith("_templates.lst"):
            for first, _ in read_lst_rows(os.path.join(dir_path, f)):
                names.add(first)
    return names


def key_prefix(fields):
    for field in fields:
        if field.startswith("KEY:"):
            val = field[len("KEY:"):]
            return val.split(" ~ ", 1)[0].strip()
    return None


def main():
    book_dirs = find_book_dirs()
    print(f"# {len(book_dirs)} directories carry an *_abilities_race.lst file (corpus-wide, all publishers)\n")

    total_would_move = 0
    total_rows_checked = 0
    false_positive_hits = []

    for d in book_dirs:
        monster_names = cr_bearing_race_names(d)
        tmpl_names = template_names(d)
        names = monster_names if TEMPLATES_MODE == "off" else (monster_names | tmpl_names)
        if not names:
            continue
        for f in os.listdir(d):
            if "_abilities_race" not in f or not f.endswith(".lst"):
                continue
            if "companion" in f or "familiar" in f:
                continue
            rows = read_lst_rows(os.path.join(d, f))
            for first, fields in rows:
                total_rows_checked += 1
                key = key_prefix(fields)
                if key is None:
                    key = first
                if key in names:
                    total_would_move += 1
                    # Flag if this directory is one of the KNOWN real-race
                    # books, so we can hand-inspect for false positives.
                    rel = os.path.relpath(d, ORACLE_ROOT)
                    for book, book_rel in KNOWN_RACE_BOOKS_DIRS.items():
                        if rel == book_rel:
                            false_positive_hits.append((book, f, first, key))

    print(f"total *_abilities_race.lst rows checked (corpus-wide): {total_rows_checked}")
    print(f"total rows whose KEY-prefix matches a same-dir CR:-bearing race or template name: {total_would_move}")
    print()
    print(f"# Candidate false positives inside KNOWN real-race-book directories: {len(false_positive_hits)}")
    for book, f, first, key in false_positive_hits[:50]:
        print(f"    {book} / {f}: {first!r} (KEY-prefix {key!r})")
    if len(false_positive_hits) > 50:
        print(f"    ... and {len(false_positive_hits) - 50} more")

    # Specifically test the named trap: does ANY row in a known real-race
    # book whose corpus_key starts "Favored Enemy ~ Humanoid" get moved?
    print()
    print("# Named-trap check: Favored Enemy ~ Humanoid (<Race>) rows in known race books")
    trap_hits = 0
    for book, key, _ in [(h[0], h[2], h[3]) for h in false_positive_hits]:
        if key.startswith("Favored Enemy"):
            trap_hits += 1
    print(f"Favored Enemy rows caught by the false-positive scan above: {trap_hits}")


if __name__ == "__main__":
    main()
