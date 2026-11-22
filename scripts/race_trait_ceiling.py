#!/usr/bin/env python3
"""Derive the race-trait lane's real ceiling, and split it from the residue.

Why this exists
---------------
`docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md` §44.4 established that
the lane's denominator (every ``race_trait``-kinded unit in ``docs/work-inventory.json``)
is not its workload: a racial trait whose race has no chassis is unreachable by
construction, because ``RaceCorpus::resolve`` returns ``None`` without one. Rounds 1
and 3 each derived that ceiling in a throwaway script; §45.1's own lesson is that an
ephemeral path is not a citation, so round 4 checked one in.

It answers two questions and keeps them separate:

1. **The ceiling** — how many corpus rows *could* ground, ever.
2. **The remainder** — of those, how many have not, joined to each unit's own
   recorded status by ``(book, source_file, source_line)``.

Two TYPE suffixes are in the ceiling, and the second one is why this script exists
rather than a one-line grep (§48.2):

* ``TYPE:<Race> Racial Trait`` — the ordinary racial-trait row. 553 corpus-wide
  under the 18-race roster this figure was first measured against (SD-29 round
  4); ``IN_SCOPE_RACES`` now tracks ``ingest_race_traits.rs``'s own roster
  (34 races as of SD31-E6-F4-006, read at import time rather than
  re-transcribed — see ``read_in_scope_races``'s doc comment), so a fresh run
  reports the CURRENT total, not this historical one.
* ``TYPE:<Race> Subrace``      — a *heritage selector*: the row a player picks,
  which grants the replacement rows gated on it. 18 corpus-wide, all in
  ``core_essentials``. These are ``race_trait``-kinded units in the work inventory
  exactly like the racial-trait rows (``file_kind()`` types them by filename), so
  counting them in the denominator and not in the ceiling made the lane look
  further from done than it was.

Scope
-----
Book directories are read from ``docs/work-inventory.json`` — its ``corpus_root``
subdirectories plus its ``additional_book_dirs`` — intersected with the books it
actually reports. Run with ``--whole-tree`` to scan every book under the PCGen data
root instead; the difference is a real scope finding rather than noise (§48.8).

Usage
-----
    python3 scripts/race_trait_ceiling.py
    python3 scripts/race_trait_ceiling.py --whole-tree
"""

import argparse
import collections
import json
import os
import re
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def read_in_scope_races(repo_root=REPO_ROOT):
    """The race roster this ceiling counts, read off `ingest_race_traits.rs`'s
    own `IN_SCOPE_RACES` declaration -- never hand-transcribed.

    SD-31 wave-21 correction: this used to be an 18-race set pinned to
    `SD-27 decisions.md §25.3` and never touched again, while the Rust
    constant it claimed to copy widened three more times (18 -> 24 -> 30 ->
    34) without this file changing once -- `scripts/tests/
    test_race_trait_ceiling.py` names all three widenings and the ceiling
    this drift was silently under-reporting. `scripts/classify_race_trait_
    rows.py` carries the identical hand-sync pattern and suffered the
    identical drift (re-synced once at 30, already stale again at 34) --
    the structural fix is reading the declaration, not re-copying it a
    fourth time.

    Same contract as `v06_work_inventory::app_race_corpus_books`: an
    unreadable or unparseable declaration returns an EMPTY set, never a
    guessed one. This ceiling then reports 0 rather than a stale number,
    which is the safe direction for an instrument other cards size their
    workload from.
    """
    path = os.path.join(repo_root, "src", "bin", "ingest_race_traits.rs")
    try:
        with open(path, encoding="utf-8") as handle:
            src = handle.read()
    except OSError:
        return set()
    marker = "const IN_SCOPE_RACES:"
    start = src.find(marker)
    if start == -1:
        return set()
    end = src.find("];", start)
    if end == -1:
        return set()
    return set(re.findall(r'"([^"]+)"', src[start:end]))


IN_SCOPE_RACES = read_in_scope_races()
RACIAL_TRAIT_SUFFIX = " Racial Trait"
SUBRACE_SUFFIX = " Subrace"

INVENTORY = "docs/work-inventory.json"


def file_is_race_trait_kinded(basename):
    """Mirror of `v06_work_inventory::file_kind`'s `race_trait` arm.

    Order matters there and here: `_abilities_class` wins over `_abilities_race`,
    and a companion/familiar marker anywhere in the basename wins over both.
    """
    if "_abilities_class" in basename:
        return False
    if "_abilities_race" in basename:
        return not ("companion" in basename or "familiar" in basename)
    return False


def type_tokens(fields):
    out = []
    for field in fields:
        if field.startswith("TYPE:"):
            out += [t.strip() for t in field[len("TYPE:"):].split(".") if t.strip()]
    return out


def row_category(line):
    """`racial_trait`, `subrace`, or None -- the predicates the ingest uses.

    `.MOD` is read off field 0 only: PCGen writes `var("STAT.3.MOD...")` inside
    token values and that is not a mod row (`ingest_race_traits::is_mod_row`).
    """
    fields = [f.strip() for f in line.split("\t") if f.strip()]
    if not fields or ".MOD" in fields[0]:
        return None
    toks = type_tokens(fields)
    for tok in toks:
        if tok.endswith(RACIAL_TRAIT_SUFFIX):
            if tok[: -len(RACIAL_TRAIT_SUFFIX)].strip() in IN_SCOPE_RACES:
                return "racial_trait"
            return None
    for tok in toks:
        if tok.endswith(SUBRACE_SUFFIX):
            if tok[: -len(SUBRACE_SUFFIX)].strip() in IN_SCOPE_RACES:
                return "subrace"
    return None


def book_dirs(inventory, whole_tree):
    root = inventory["corpus_root"]
    dirs = []
    for entry in sorted(os.listdir(root)):
        path = os.path.join(root, entry)
        if os.path.isdir(path):
            dirs.append((entry, path))
    for extra in inventory["additional_book_dirs"]:
        dirs.append((os.path.basename(extra), extra))
    if whole_tree:
        data_root = os.path.dirname(os.path.dirname(os.path.dirname(root)))
        return [("<whole tree>", data_root)]
    known = {b["id"] for b in inventory["books"]}
    return [(i, p) for (i, p) in dirs if i in known]


def scan(dirs):
    """(book, basename, lineno) -> category, for every ceiling row."""
    found = {}
    for book, root in dirs:
        for dirpath, _, files in os.walk(root):
            for name in files:
                if not name.endswith(".lst") or not file_is_race_trait_kinded(name):
                    continue
                path = os.path.join(dirpath, name)
                with open(path, encoding="utf-8", errors="replace") as handle:
                    for lineno, line in enumerate(handle, 1):
                        stripped = line.strip()
                        if not stripped or stripped.startswith("#"):
                            continue
                        category = row_category(line)
                        if category:
                            found[(book, name, lineno)] = category
    return found


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--whole-tree", action="store_true",
                        help="scan every book under the PCGen data root, not just this bundle's")
    args = parser.parse_args()

    if not os.path.exists(INVENTORY):
        sys.exit(f"run from the repo root: {INVENTORY} not found")
    inventory = json.load(open(INVENTORY, encoding="utf-8"))

    found = scan(book_dirs(inventory, args.whole_tree))
    by_category = collections.Counter(found.values())
    per_book = collections.Counter(book for (book, _, _) in found)

    print("CEILING")
    print(f"  TYPE:<one of {len(IN_SCOPE_RACES)} races> Racial Trait rows : {by_category['racial_trait']}")
    print(f"  TYPE:<one of {len(IN_SCOPE_RACES)} races> Subrace rows      : {by_category['subrace']}")
    print(f"  total                                    : {len(found)}")
    print()
    for book, count in sorted(per_book.items(), key=lambda kv: -kv[1]):
        print(f"  {count:>5}  {book}")

    if args.whole_tree:
        print("\n(--whole-tree: no status join, since the inventory does not cover these books)")
        return

    by_status = collections.Counter()
    per_book_status = collections.defaultdict(collections.Counter)
    remaining = collections.defaultdict(list)
    matched = 0
    for unit in inventory["units"]:
        if unit["kind"] != "race_trait":
            continue
        key = (unit["book"], unit["source_file"], unit["source_line"])
        if key not in found:
            continue
        matched += 1
        by_status[unit["status"]] += 1
        per_book_status[unit["book"]][unit["status"]] += 1
        if unit["status"] != "grounded":
            remaining[unit["book"]].append(unit["corpus_key"])

    print()
    print("STATUS, joined by (book, source_file, source_line)")
    print(f"  units matched into the ceiling : {matched}")
    print(f"  by status                      : {dict(by_status)}")
    if matched != len(found):
        print(f"  WARNING: {len(found) - matched} ceiling rows have no matching inventory unit")
    print()
    for book in sorted(per_book_status):
        print(f"  {book:<26} {dict(per_book_status[book])}")
    print()
    print("REMAINING, by book and key")
    for book in sorted(remaining):
        print(f"  {book} ({len(remaining[book])})")
        for key in sorted(remaining[book]):
            print(f"      {key}")

    total = sum(b["kinds"]["race_trait"]["units"] for b in inventory["books"]
                if "race_trait" in b["kinds"])
    print()
    print(f"chassis-blocked residue: {total} race_trait units - {len(found)} ceiling "
          f"= {total - len(found)} that no race-trait ingest can ground")


if __name__ == "__main__":
    main()
