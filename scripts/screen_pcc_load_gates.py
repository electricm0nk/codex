#!/usr/bin/env python3
"""Screen every remaining corpus unit against the PCC line that LOADS its file.

# Why this exists

SD-29 Epic 5's final round found a unit that three checked-in screens all called
workable and that this repo can never ingest.

``docs/work-inventory.json`` had ``occult_adventures:monster:kami_shikigami`` as
a real ``monster`` unit; ``scripts/classify_monster_ability_rows.py`` scored it
``reachable``; SD-29's closure run 2 receipt carried it forward as **1 of the
lane's 10 REAL workable units**.  All three were reading the ``.lst`` row, and
the row is fine.  The disqualifying fact is one line away, in the book's pcc::

    _occult_adventures.pcc:75
        RACE:support/oa_races_b3.lst|!PRECAMPAIGN:1,INCLUDES=Bestiary 3

A **negated** gate.  PCGen loads that file only when Bestiary 3 is ABSENT, and
the file's own header says why -- ``SOURCELONG:Bestiary 3``, ``SOURCESHORT:B3``:
it is a republication of Bestiary 3 content, carried so Occult Adventures stands
alone without it.  This repo registered ``RuleSetId::B3`` in SD-29 Epic 5 round 5
and already ships ``Kami (Shikigami)`` from ``bestiary_3``
(``rules_tables::bestiary_3::monster_data``).  Ingesting the ``occult_adventures``
row would have written a SECOND record for one creature under a second book
label.

# The general shape, and why a positive-gate-only reader misses half of it

Round 9 (`decisions.md §64`) made "read the PCC LOAD LINE, not the ``.lst``" a
standing check, and every use of it since has been a POSITIVE gate:
``PRECAMPAIGN:1,INCLUDES=<Book>`` with ``<Book>`` absent, so the file is out of
scope (``RuleSetId::Ha``'s ``support/ha_abilities_race_oa.lst``, ``B5``'s
``support/b5_races_companion_oa.lst``).  Those are excluded by a book this repo
does NOT have.

The negated form is excluded by a book this repo DOES have, and it gets *more*
likely to fire as the repo ingests more books.  A screen written for the positive
form alone silently passes it.  Both forms are here.

# What "registered" means, and why it is derived rather than listed

A gate names a CAMPAIGN (``INCLUDES=Bestiary 3``), not a corpus directory
(``bestiary_3``).  This script bridges the two by reading the ``CAMPAIGN:`` line
of **every** ``.pcc`` in each PCGen book directory and treating a campaign as
registered when this repo has a ``data/corpus/<book>/`` for that directory.
Nothing is typed twice: add a book and the mapping follows, which is
`decisions.md §45.1`'s rule that a screen must stay falsifiable rather than
become a table someone maintains.

Conditions this screen cannot model (``INCLUDESBOOKTYPE=``, and anything else
carrying an ``=`` that is not ``INCLUDES=``) are reported as ``unresolved``
rather than guessed at -- a screen that guesses in the permissive direction ships
records, which is the direction that costs.

Usage::

    python3 scripts/screen_pcc_load_gates.py                    # every kind
    python3 scripts/screen_pcc_load_gates.py monster monster_ability

Run from the repo root.  ``PCGEN_CORPUS_ROOT`` may point at a local PCGen
``data/`` checkout; it defaults to ``$HOME/workspace/repos/pcgen/data``.

Exit code is 0 whatever it finds: this is a screen, not a gate.
"""

from __future__ import annotations

import json
import os
import re
import sys

INVENTORY = "docs/work-inventory.json"
CORPUS_DIR = "data/corpus"

REMAINING = {"not-started", "engine-does-not-hold"}

# A pcc content line: `<TAG>:<path>|<condition>|<condition>...`
PCC_LOAD = re.compile(r"^[A-Z]+:([^|\t\r\n]+)(.*)$")
# `PRECAMPAIGN:n,<conditions>` and its negated `!PRECAMPAIGN:n,<conditions>`.
GATE = re.compile(r"(!?)PRECAMPAIGN:\d+,([^|\t\r\n]*)")


def corpus_root() -> str:
    return os.environ.get(
        "PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")
    )


def book_dirs() -> dict[str, str]:
    """Book id -> absolute PCGen directory, found by directory basename."""
    found: dict[str, str] = {}
    for dirpath, dirnames, _ in os.walk(corpus_root()):
        for name in dirnames:
            found.setdefault(name, os.path.join(dirpath, name))
    return found


def pcc_paths(directory: str) -> list[str]:
    """EVERY `.pcc` in the book directory, not the first one.

    A PCGen book directory routinely declares SEVERAL campaigns from one tree --
    `bestiary/` carries `bestiary.pcc` (`CAMPAIGN:Bestiary`), a
    `(Player Options Only)` variant and a `(PFS)` variant. Reading one `.pcc` and
    calling its `CAMPAIGN:` "the book's name" made this screen report `this repo
    does NOT have Bestiary` about a book it ships 330 monsters from. Caught by
    running the screen against cases whose answer was already known, which is the
    only reason it was caught at all.
    """
    return [
        os.path.join(directory, name)
        for name in sorted(os.listdir(directory))
        if name.endswith(".pcc")
    ]


def campaign_names(directory: str) -> set[str]:
    """Every `CAMPAIGN:` this book directory declares, across all its pccs."""
    names: set[str] = set()
    for pcc in pcc_paths(directory):
        try:
            with open(pcc, encoding="utf-8", errors="replace") as handle:
                for line in handle:
                    if line.startswith("CAMPAIGN:"):
                        names.add(line[len("CAMPAIGN:") :].strip())
                        break
        except OSError:
            continue
    return names


# `Bestiary 3 (Player Options Only)` (a `CAMPAIGN:` declaration) and
# `Bestiary 3 ~ Player Options Only` (the spelling a gate uses) are the same
# book. Both suffix forms are stripped so the two sides compare.
VARIANT_SUFFIX = re.compile(r"\s*(\(.*\)|~.*)$")


def normalize(campaign: str) -> str:
    return VARIANT_SUFFIX.sub("", campaign).strip()


# Corpus directories this repo spells differently from PCGen's own directory.
#
# ONE entry, and it is a documented ruling rather than a convenience:
# `rules_tables::monster_chassis`'s Bestiary 1 row records that `corpus_book` is
# `beastiary` and NOT `bestiary`, because that directory has been spelled that
# way since SD-22 and registering the source spelling would write a SECOND corpus
# directory for a book that already has one.
CORPUS_DIR_ALIASES = {"bestiary": "beastiary"}


def registered_campaigns(dirs: dict[str, str]) -> dict[str, str]:
    """Normalized campaign name -> book id, for every book this repo ingested.

    "Ingested" is `data/corpus/<book>/` existing, which is what every consumer of
    a corpus record already means by it -- not a hand-kept roster.
    """
    out: dict[str, str] = {}
    for book, directory in dirs.items():
        corpus_book = CORPUS_DIR_ALIASES.get(book, book)
        if not os.path.isdir(os.path.join(CORPUS_DIR, corpus_book)):
            continue
        for name in campaign_names(directory):
            out.setdefault(normalize(name), corpus_book)
    return out


def load_gates(directory: str) -> dict[str, list[tuple[bool, str]]]:
    """`.lst` basename -> the gates on the pcc line that loads it.

    Keyed by BASENAME because `docs/work-inventory.json` records every unit's
    `source_file` as a bare basename whatever subdirectory it sits in -- the same
    widening `scripts/transcribe_monster_tables.py::resolve_book_file` makes on
    the read side.

    A basename loaded by MORE than one pcc in the directory accumulates every
    line's gates. That is deliberate and it is why the verdict is a disjunction
    over conditions but a conjunction over lines: a file this repo could reach
    through any registered campaign is in scope.
    """
    gates: dict[str, list[tuple[bool, str]]] = {}
    for pcc in pcc_paths(directory):
        try:
            handle = open(pcc, encoding="utf-8", errors="replace")
        except OSError:
            continue
        with handle:
            for raw in handle:
                line = raw.strip()
                if not line or line.startswith("#"):
                    continue
                match = PCC_LOAD.match(line)
                if match is None:
                    continue
                path, rest = match.group(1), match.group(2)
                if not path.endswith(".lst"):
                    continue
                found = [
                    (bool(negated), condition.strip())
                    for negated, condition in GATE.findall(rest)
                ]
                if found:
                    gates.setdefault(os.path.basename(path), []).extend(found)
                else:
                    gates.setdefault(os.path.basename(path), [])
    return gates


def verdict(
    gates: list[tuple[bool, str]], registered: dict[str, str]
) -> tuple[str, str]:
    """(status, why) for one file's gate list.

    **`PRECAMPAIGN:1,<a>,<b>` is a DISJUNCTION**, not a sequence of independent
    conditions: the leading count is how many of the listed campaigns must be
    present, and every gate in this corpus spells it `1`. A reader that returned
    on the first condition would exclude a file loaded under
    `PRECAMPAIGN:1,INCLUDES=Bestiary 2,INCLUDES=Bestiary 2 ~ Player Options Only`
    whenever the repo had the second spelling and not the first -- which is the
    normal case here, since the variant is what this repo ingested.

    Fails toward `unresolved`. A condition this screen cannot model is reported,
    never assumed satisfied: guessing in the permissive direction ships records,
    and that is the direction that costs.
    """
    if not gates:
        return "in-scope", "loaded unconditionally"

    for negated, condition in gates:
        named: list[str] = []
        unmodelled: str | None = None
        for part in condition.split(","):
            part = part.strip()
            if not part:
                continue
            if part.startswith("INCLUDES="):
                named.append(normalize(part[len("INCLUDES=") :]))
            elif "=" in part:
                unmodelled = part
            else:
                # `_bestiary_5.pcc:69`'s bare `PRECAMPAIGN:1,Occult Adventures`.
                named.append(normalize(part))
        if unmodelled is not None:
            return "unresolved", f"condition this screen does not model: {unmodelled}"
        if not named:
            continue
        present = [n for n in named if n in registered]
        if negated and present:
            have = present[0]
            return (
                "EXCLUDED",
                f"negated gate `!PRECAMPAIGN:1,{condition}` and this repo HAS "
                f"{have} (data/corpus/{registered[have]}) -- PCGen would not load "
                f"this file",
            )
        if not negated and not present:
            return (
                "EXCLUDED",
                f"gate `PRECAMPAIGN:1,{condition}` and this repo has none of "
                f"{named} -- PCGen would not load this file",
            )
    return "in-scope", "every gate on the load line is satisfied"


def main() -> None:
    wanted_kinds = set(sys.argv[1:])
    inventory = json.load(open(INVENTORY, encoding="utf-8"))
    dirs = book_dirs()
    registered = registered_campaigns(dirs)

    gates_by_book: dict[str, dict[str, list[tuple[bool, str]]]] = {}
    findings: list[tuple[str, str, str, str, str]] = []
    unresolved: list[tuple[str, str, str]] = []
    screened = 0

    for unit in inventory["units"]:
        if unit["status"] not in REMAINING:
            continue
        if wanted_kinds and unit["kind"] not in wanted_kinds:
            continue
        book = unit["book"]
        directory = dirs.get(book)
        if directory is None:
            continue
        if book not in gates_by_book:
            gates_by_book[book] = load_gates(directory)
        gates = gates_by_book[book].get(unit["source_file"])
        if gates is None:
            continue
        screened += 1
        status, why = verdict(gates, registered)
        if status == "EXCLUDED":
            findings.append((book, unit["kind"], unit["id"], unit["source_file"], why))
        elif status == "unresolved":
            unresolved.append((book, unit["source_file"], why))

    print(f"campaigns this repo has registered : {len(registered)}")
    print(f"remaining units screened           : {screened}")
    print()

    if findings:
        by_file: dict[tuple[str, str, str, str], int] = {}
        for book, kind, _unit_id, source_file, why in findings:
            key = (book, kind, source_file, why)
            by_file[key] = by_file.get(key, 0) + 1
        print("EXCLUDED BY THE PCC LOAD LINE -- these are NOT workable units:")
        for (book, kind, source_file, why), count in sorted(by_file.items()):
            print(f"  {book} / {kind} / {source_file}: {count} unit(s)")
            print(f"      {why}")
        print()
        for book, kind, unit_id, _source_file, _why in sorted(findings):
            print(f"    {unit_id}")
        print()

    if unresolved:
        print("UNRESOLVED conditions (reported, never assumed satisfied):")
        for book, source_file, why in sorted(set(unresolved)):
            print(f"  {book} / {source_file}: {why}")
        print()

    print(f"TOTAL remaining units excluded by a PCC load gate: {len(findings)}")


if __name__ == "__main__":
    main()
