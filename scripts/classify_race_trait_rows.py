#!/usr/bin/env python3
"""Classify a book's racial-trait rows the way the engine will, before ingesting it.

Why this exists
---------------
SD-29's race-trait lane round 1 (`decisions.md §44.4`) ranked four candidate books
by the *work-inventory evidence token* on their units — a statement about what the
engine has compiled — and got the order exactly backwards, because the question
that decides a book's cost is what its **rows** are:

* a row that sets a ``FACT:<Race>_Replace<Trait>|True`` token becomes
  ``TraitRole::Alternate`` and reaches a player through the existing picker with
  **no new mechanism at all**;
* a row gated only by a positive ``PREFACT``/``PREABILITY`` becomes
  ``TraitRole::FlagGranted`` and needs whatever grants it to exist;
* a row with no readable gate becomes ``TraitRole::Unclassified`` and **never
  applies**, so ingesting it alone ships a record that loads and does nothing —
  the stub class `decisions.md §44.2` describes.

Run this against a candidate book *before* committing a round to it. The
predicates below mirror ``race_resolver::classify`` and
``ingest_race_traits::parse_row``; where they disagree, the Rust is the truth and
this script is the bug.

Usage
-----
    python3 scripts/classify_race_trait_rows.py isr_abilities_race.lst
    python3 scripts/classify_race_trait_rows.py ha_abilities_race.lst --verbose

Files are located by recursive glob under ``$PCGEN_CORPUS_ROOT`` (default
``~/workspace/repos/pcgen/data``), so a bare basename is enough.
"""

import argparse
import glob
import os
import sys

# The 30 races the product models (`ingest_race_traits::IN_SCOPE_RACES`,
# SD-27 `decisions.md §25.3`, widened 18 -> 24 by SD-31 Epic 1-F2 and
# 24 -> 30 by SD-31-E6-F4-003, both re-synced here 2026-08-16 -- this
# screening copy had drifted to the original 18 across both widenings,
# undercounting every subsequent screen's "in-scope rows" figure). A trait
# of any other race cannot ground: the resolver returns None without a
# chassis, whatever the ingest writes.
IN_SCOPE_RACES = [
    "Dwarf", "Elf", "Gnome", "Half-Elf", "Half-Orc", "Halfling", "Human",
    "Aasimar", "Drow", "Duergar", "Goblin", "Hobgoblin", "Kobold", "Merfolk",
    "Orc", "Svirfneblin", "Tengu", "Tiefling",
    "Fetchling", "Grippli", "Ifrit", "Oread", "Sylph", "Undine",
    "Catfolk", "Kitsune", "Ratfolk", "Strix", "Suli", "Wayang",
]
RACIAL_TRAIT_SUFFIX = " Racial Trait"
RACIAL_DEFAULT_SUFFIX = " Racial Default"


def fields_of(line):
    """PrettyLST pads columns with tab runs, so empty fields are structure."""
    return [f.strip() for f in line.split("\t") if f.strip()]


def is_mod_row(fields):
    """Only field 0 decides; `.MOD` inside a token value is not a mod row."""
    return bool(fields) and ".MOD" in fields[0]


def types_of(fields):
    out = []
    for f in fields:
        if f.startswith("TYPE:"):
            out += [t.strip() for t in f[5:].split(".") if t.strip()]
    return out


def classify(fields, types):
    """Mirrors `race_resolver::classify`, in its own precedence order."""
    race = next((t[: -len(RACIAL_TRAIT_SUFFIX)] for t in types
                 if t.endswith(RACIAL_TRAIT_SUFFIX)), None)
    if race is None:
        return None, None
    if any(t == race + RACIAL_DEFAULT_SUFFIX for t in types):
        return race, "default"
    sets_flag = any(
        f.startswith("FACT:") and "_Replace" in f and f.lower().endswith("|true")
        for f in fields
    )
    if sets_flag:
        return race, "alternate"
    # A *positive* gate only. A leading `!` is a suppressor, which is the
    # standard-trait shape, not a grant.
    positive_gate = any(
        f.startswith("PREFACT:") or f.startswith("PREABILITY:") for f in fields
    )
    if positive_gate:
        return race, "flag_granted"
    return race, "unclassified"


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("lst", nargs="+", help="one or more .lst basenames or paths")
    ap.add_argument("--verbose", action="store_true",
                    help="print every non-alternate row's key and verdict")
    args = ap.parse_args()

    root = os.environ.get("PCGEN_CORPUS_ROOT",
                          os.path.expanduser("~/workspace/repos/pcgen/data"))
    exit_code = 0

    for pattern in args.lst:
        paths = ([pattern] if os.path.isfile(pattern)
                 else sorted(glob.glob(root + "/**/" + pattern, recursive=True)))
        if not paths:
            print("no such .lst under %s: %s" % (root, pattern), file=sys.stderr)
            exit_code = 1
            continue

        for path in paths:
            counts = {"default": 0, "alternate": 0, "flag_granted": 0, "unclassified": 0}
            mod_rows_with_a_race_type = 0
            out_of_scope = 0
            detail = []

            with open(path, encoding="utf-8", errors="replace") as fh:
                for lineno, line in enumerate(fh, 1):
                    stripped = line.strip()
                    if not stripped or stripped.startswith("#"):
                        continue
                    fields = fields_of(line)
                    types = types_of(fields)
                    race, role = classify(fields, types)
                    if role is None:
                        continue
                    # A `.MOD` row declares nothing and can never become a
                    # record, however it is typed.
                    if is_mod_row(fields):
                        mod_rows_with_a_race_type += 1
                        continue
                    if race not in IN_SCOPE_RACES:
                        out_of_scope += 1
                        continue
                    counts[role] += 1
                    if role != "alternate":
                        key = next((f[4:] for f in fields if f.startswith("KEY:")), fields[0])
                        detail.append((lineno, key, role))

            in_scope = sum(counts.values())
            print(os.path.relpath(path, root))
            print("   in-scope rows %d | default %d | alternate %d | flag_granted %d | unclassified %d"
                  % (in_scope, counts["default"], counts["alternate"],
                     counts["flag_granted"], counts["unclassified"]))
            print("   out-of-scope races %d | .MOD rows carrying a race TYPE %d (never ingested)"
                  % (out_of_scope, mod_rows_with_a_race_type))
            print("   => %d of %d rows need no new mechanism" % (counts["alternate"], in_scope))
            if args.verbose:
                for lineno, key, role in detail:
                    print("      :%-5d %-50s %s" % (lineno, key, role))

    return exit_code


if __name__ == "__main__":
    sys.exit(main())
