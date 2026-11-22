#!/usr/bin/env python3
"""Classify every `monster_ability` corpus row by the link that would reach it.

# Why this exists

SD-29 `decisions.md §45.1` records the lesson this script exists to make cheap:
a round-1 deferral note ranked four books by the **work inventory's evidence
token** -- a statement about what the *engine* has compiled -- when the question
was what the *corpus rows* are.  Those are different questions, and the
cheaper-sounding token named the harder work.  `scripts/classify_race_trait_rows.py`
is the race-trait lane's answer to that; this is the monster lane's.

Round 1 of `epic-5-monster-lane-extend` (`progress.md` `## Cycle SD29-E5-F2-002`
§4) derived the per-book link-shape table with a `/tmp/.../shape_all.py` that no
longer exists.  §45.1's own finding is that **an ephemeral path is not a
citation**; a successor must be able to re-derive the table rather than trust
it.  This is that script, checked in.

# What it classifies, and why the classes are the ones that matter

A `monster_ability` record only reaches a player through a monster: the desktop
monster catalog lists monsters, and an ability is shown as a row underneath the
monster that owns it (`monster_catalog.rs`).  So the operative question for
every ability row is *which monster claims it*, and there are exactly three
answers in the corpus:

``row-named``
    Some monster row in the same book names it in an
    ``ABILITY:Special Ability|AUTOMATIC|<key>`` token.  Bonus Bestiary's shape.

``prefix``
    The ability's own ``KEY:`` is namespaced ``<Monster> ~ <Ability>`` and the
    prefix is a monster of this book.  Monster Codex's shape (`Seru ~ Poison`).

``orphan``
    Neither.  Nothing in the book's own monster rows reaches it.  **An orphan is
    a reach-gate cost, not a transcription one** -- ingesting it produces a
    record that loads and is never shown, which is precisely the stub class
    `decisions.md §44.2` was written about.

Both link shapes mirror `scripts/transcribe_monster_tables.py`'s own
`parse_special_ability_refs` + namespaced-prefix pass, so the classification
predicts what a transcription of that book would actually produce.

# The ceiling this measures

A book with monsters and few orphans is ingestable at its stated unit count.  A
book with **zero monster rows** and N ability rows has N orphans by
construction: there is no monster in that book to hang them on, so no per-monster
cycle can ground them at all.  That is a scope finding about the lane, not a
backlog item -- `loop-instruction.md`'s "Hard stops" names a per-monster cycle
against a zero-monster book as a reportable hard stop.

Usage::

    python3 scripts/classify_monster_ability_rows.py            # every book with remaining units
    python3 scripts/classify_monster_ability_rows.py book_of_the_damned_volume_1 ...

``PCGEN_CORPUS_ROOT`` may point at a local PCGen ``data/`` checkout; it defaults
to ``$HOME/workspace/repos/pcgen/data``.  Run from the repo root: the unit set
is ``docs/work-inventory.json``, never a raw line count over the ``.lst``.
"""

from __future__ import annotations

import json
import os
import re
import sys

INVENTORY = "docs/work-inventory.json"


def corpus_root() -> str:
    return os.environ.get(
        "PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")
    )


def book_dirs() -> dict[str, str]:
    """Book id -> absolute PCGen directory, found by directory basename.

    Derived from the tree rather than from a hand-kept table, so a book this
    script has never seen classifies without an edit here.
    """
    found: dict[str, str] = {}
    for dirpath, dirnames, _ in os.walk(corpus_root()):
        for name in dirnames:
            found.setdefault(name, os.path.join(dirpath, name))
    return found


def read_row(path: str, line_no: int) -> list[str]:
    """The 1-based line at `line_no`, split into its tab-separated tokens."""
    with open(path, encoding="utf-8", errors="replace") as handle:
        line = handle.read().split("\n")[line_no - 1]
    return [token.strip() for token in line.split("\t") if token.strip()]


def special_ability_refs(row: list[str]) -> list[str]:
    """Ability keys named by this monster row's `ABILITY:Special Ability` tokens.

    Identical predicate to `transcribe_monster_tables.parse_special_ability_refs`
    -- a classification that used a looser rule than the transcriber would
    over-report reachability, which is the direction that ships stubs.
    """
    refs: list[str] = []
    for field in row:
        if not field.startswith("ABILITY:Special Ability|"):
            continue
        for name in field.split("|")[2:]:
            name = name.strip()
            if not name or name.startswith("PRE") or "=" in name:
                continue
            if name not in refs:
                refs.append(name)
    return refs


REMAINING = {"not-started", "engine-does-not-hold"}

PI_SCREEN_RS = "src/rules_core/pi_screening.rs"


def pi_blacklist_terms() -> list[str]:
    """`pi_screening::PI_BLACKLIST_TERMS`, parsed out of the Rust source.

    Derived, never re-typed -- the same reading
    `scripts/transcribe_monster_tables.py` does, for the same reason: one list
    is authoritative and a copy drifts the first time
    `docs/governance/ogl-pi-blacklist.md` §3's per-book override adds a term.
    """
    text = open(PI_SCREEN_RS, encoding="utf-8").read()
    start = text.index("pub const PI_BLACKLIST_TERMS")
    body = text[start : text.index("\n];", start)]
    terms = re.findall(r'"([^"]+)"', body)
    if len(terms) < 20:
        raise SystemExit(
            f"{PI_SCREEN_RS}: parsed only {len(terms)} PI terms, which cannot be right"
        )
    return terms


def is_product_identity(row: list[str], key: str, name: str, terms: list[str]) -> bool:
    """Whether this corpus row's own IDENTITY is Product Identity.

    Two independent signals, and the first is the one this program was blind to
    until SD-29 Epic 5 round 3 (`decisions.md §50.1`):

    ``NAMEISPI:YES``
        PCGen's own per-record declaration that the record's NAME is Product
        Identity.  Inner Sea World Guide carries five; **only three of the five
        are also on the blacklist**, so a term-list-only screen passes two.

    a blacklist term in the key or name
        What `gen_book_cache`'s hard stop already catches.

    A row matching either can never be ingested by a per-monster cycle: a KEY
    is the one field redaction cannot touch, so the record is dropped rather
    than screened, and reclassifying is an operator decision.  Counting such a
    row as "reachable" over-reports the lane's real remainder.
    """
    for field in row:
        if field.startswith("NAMEISPI:") and field[len("NAMEISPI:") :].strip() == "YES":
            return True
    haystack = f"{key} {name}"
    return any(term in haystack for term in terms)


def is_copy_row(row: list[str]) -> bool:
    """Whether this monster row is a `<Base>.COPY=<Variant>` derived row.

    A copy row states a DELTA on another record, not a stat block: PCGen copies
    the base record whole and applies the few tokens the copy row carries.
    Transcribed verbatim it produces a card with a challenge rating and no size,
    speed, type or page, and `gen_book_cache::verified_citation_line` refuses it
    outright because the row's first column is not the record's name.

    So a copy row is not reachable through this chassis, for the same *kind* of
    reason a `NAMEISPI:YES` row is not: no per-monster cycle can ship it as it
    stands. Counting one as reachable over-reports the lane's remainder, which
    is the error `decisions.md` §50.7 corrected for Product Identity and this
    corrects for inheritance. Corpus-wide there are exactly two, both in
    `bestiary_2` -- the `origin: copy` units in `docs/work-inventory.json`.
    """
    return bool(row) and ".COPY=" in row[0]


def classify_book(book: str, units: list[dict], directory: str | None) -> dict:
    """Classify a book's REMAINING ability rows against ALL its monster rows.

    The two predicates are deliberately different.  The *counts* answer "what
    does this lane still owe", so they are over remaining units only.  The
    *link* answers "is there a monster in this book that owns this row", and a
    monster that is already grounded owns its abilities just as well as one this
    lane has yet to transcribe -- restricting the link to remaining monsters
    would report Bestiary 1's 46 SD-22 monsters' abilities as orphans, which
    they are not.
    """
    monsters = [u for u in units if u["kind"] == "monster"]
    abilities = [
        u
        for u in units
        if u["kind"] == "monster_ability" and u["status"] in REMAINING
    ]
    ability_keys = {u["corpus_key"] for u in units if u["kind"] == "monster_ability"}

    # Product Identity is read BEFORE the link, because a PI monster is one no
    # cycle can ship and therefore one that cannot own anything. Resolving the
    # link against every monster ROW instead reported 11 of Inner Sea World
    # Guide's 16 remaining abilities as reachable when their owners are the five
    # `NAMEISPI:YES` rows -- 16 units of over-reported reachability in one book
    # (`decisions.md §50.7`).
    terms = pi_blacklist_terms()
    pi_monster_keys: set[str] = set()
    pi_ability_keys: set[str] = set()
    copy_monster_keys: set[str] = set()
    if directory is not None:
        for unit in units:
            path = os.path.join(directory, unit["source_file"])
            if not os.path.exists(path):
                continue
            row = read_row(path, unit["source_line"])
            if is_product_identity(row, unit["corpus_key"], unit["name"], terms):
                if unit["kind"] == "monster":
                    pi_monster_keys.add(unit["corpus_key"])
                else:
                    pi_ability_keys.add(unit["corpus_key"])
            elif unit["kind"] == "monster" and is_copy_row(row):
                copy_monster_keys.add(unit["corpus_key"])

    unshippable = pi_monster_keys | copy_monster_keys
    shippable_monsters = [u for u in monsters if u["corpus_key"] not in unshippable]
    monster_keys = {u["corpus_key"] for u in shippable_monsters}

    named: set[str] = set()
    unresolved_refs: list[str] = []
    if directory is not None:
        for unit in shippable_monsters:
            path = os.path.join(directory, unit["source_file"])
            if not os.path.exists(path):
                continue
            for ref in special_ability_refs(read_row(path, unit["source_line"])):
                if ref in ability_keys:
                    named.add(ref)
                else:
                    unresolved_refs.append(ref)

    row_named = prefix = orphan = pi = 0
    orphan_keys: list[str] = []
    for unit in abilities:
        key = unit["corpus_key"]
        if key in pi_ability_keys:
            pi += 1
        elif key in named:
            row_named += 1
        elif " ~ " in key and key.split(" ~ ")[0] in monster_keys:
            prefix += 1
        else:
            orphan += 1
            orphan_keys.append(key)
    remaining_monsters = [u for u in monsters if u["status"] in REMAINING]
    pi_monsters_remaining = sum(
        1 for u in remaining_monsters if u["corpus_key"] in pi_monster_keys
    )
    copy_monsters_remaining = sum(
        1 for u in remaining_monsters if u["corpus_key"] in copy_monster_keys
    )
    return {
        "book": book,
        "monsters": len(remaining_monsters),
        "monsters_all": len(monsters),
        "abilities": len(abilities),
        "row_named": row_named,
        "prefix": prefix,
        "orphan": orphan,
        "pi": pi + pi_monsters_remaining,
        "pi_monsters": pi_monsters_remaining,
        "copy": copy_monsters_remaining,
        "orphan_keys": orphan_keys,
        "external_ability_refs": sorted(set(unresolved_refs)),
        "directory_found": directory is not None,
    }


def main() -> None:
    inventory = json.load(open(INVENTORY, encoding="utf-8"))
    wanted = set(sys.argv[1:])

    by_book: dict[str, list[dict]] = {}
    for unit in inventory["units"]:
        if unit["kind"] not in ("monster", "monster_ability"):
            continue
        if wanted and unit["book"] not in wanted:
            continue
        by_book.setdefault(unit["book"], []).append(unit)

    dirs = book_dirs()
    rows = [
        classify_book(book, by_book[book], dirs.get(book))
        for book in sorted(by_book)
    ]
    rows = [r for r in rows if r["monsters"] + r["abilities"] > 0]
    rows.sort(key=lambda r: -(r["monsters"] + r["abilities"]))

    width = max((len(r["book"]) for r in rows), default=4)
    print(
        f"{'book'.ljust(width)}  {'mon':>4} {'abil':>5} {'row-named':>9} "
        f"{'prefix':>6} {'ORPHAN':>6} {'PI':>4} {'COPY':>4}"
    )
    for r in rows:
        print(
            f"{r['book'].ljust(width)}  {r['monsters']:>4} {r['abilities']:>5} "
            f"{r['row_named']:>9} {r['prefix']:>6} {r['orphan']:>6} {r['pi']:>4} "
            f"{r['copy']:>4}"
            + ("" if r["directory_found"] else "   [pcgen dir NOT FOUND]")
        )

    total_units = sum(r["monsters"] + r["abilities"] for r in rows)
    total_orphan = sum(r["orphan"] for r in rows)
    total_pi = sum(r["pi"] for r in rows)
    total_copy = sum(r["copy"] for r in rows)
    # **`monsters_all`, NOT `monsters`.** `monsters` counts only the book's
    # REMAINING monster rows, so a book whose monsters have all been INGESTED
    # reads as zero here -- and this line's own parenthetical then says "no
    # monster in the book to own them" about a book that ships several.
    #
    # The two classes are structurally different and only one of them is a
    # ceiling. A book with no monster rows AT ALL (Bestiary 5, Bestiary 6) can
    # never own its ability rows through any pass, and `loop-instruction.md`'s
    # "Hard stops" names a per-monster cycle against one as a reportable hard
    # stop. A book whose monsters are all shipped (`horror_adventures` after
    # SD-29 Epic 5's final round, `inner_sea_gods`, `ultimate_psionics`,
    # `bestiary_3`) has owners; its orphans are unowned for some OTHER reason --
    # most of them `scan_monster_ability_bundle_rows.py`'s bundle class.
    #
    # Measured, because the size of the error is the argument for the fix: the
    # single line reported `931 across 14 books` on the tree that split it,
    # against a real structural figure of `703 across 10`. SD-29's closure run 2
    # receipt published the same conflation one ingest earlier as `866 across
    # 13` (= 703 structural + 163 exhausted). An ingest that GROUNDS a book's
    # monsters used to make this number RISE, which is the signature of a proxy
    # measuring something other than what it names.
    zero_monster = [r for r in rows if r["monsters_all"] == 0]
    zero_monster_units = sum(r["abilities"] for r in zero_monster)
    exhausted = [r for r in rows if r["monsters_all"] > 0 and r["monsters"] == 0]
    exhausted_units = sum(r["abilities"] for r in exhausted)
    print()
    print(f"remaining monster+monster_ability units     : {total_units}")
    print(f"orphan monster_ability rows                 : {total_orphan}")
    print(
        f"  of which in ZERO-monster books            : {zero_monster_units} "
        f"across {len(zero_monster)} books (no monster row in the book at all, "
        f"so nothing can ever own them)"
    )
    print(
        f"  of which in monster-EXHAUSTED books       : {exhausted_units} "
        f"across {len(exhausted)} books (every monster row SHIPS; these orphans "
        f"are unowned for another reason -- see scan_monster_ability_bundle_rows.py)"
    )
    print(f"Product Identity rows (never shippable)      : {total_pi}")
    print(f"`.COPY=` delta rows (no stat block of their own): {total_copy}")
    print(
        "reachable remainder (units - orphans - PI - COPY): "
        f"{total_units - total_orphan - total_pi - total_copy}"
    )


if __name__ == "__main__":
    main()
