#!/usr/bin/env python3
"""Tests for `scripts/race_trait_ceiling.py`'s `IN_SCOPE_RACES` roster.

WHY THIS FILE EXISTS (SD-31 wave-21 race_trait lane). The script's own
`IN_SCOPE_RACES` was a hand-transcribed copy of `ingest_race_traits.rs`'s
`IN_SCOPE_RACES`, pinned at 18 races and annotated "SD-27 decisions.md
§25.3". That copy was never updated across two later widenings the Rust
constant's own doc comment records (18 -> 24, SD-31 Epic 1-F2; 24 -> 30,
SD-31-E6-F4-003) and a third that landed without ever touching this file at
all (30 -> 34, SD31-E6-F4-006's Gillman/Nagaji/Vanara/Vishkanya follow-on
batch) -- the exact "two lists never reconciled" shape `docs/release/
SD-31-corpus-closure-grind/decisions.md` names as "Decision 36's pattern"
everywhere else it recurs (spell/equipment book registries, both rebuilt to
read the resolver's own declaration instead of a second hand list).

`scripts/classify_race_trait_rows.py` suffered the identical drift and was
independently re-synced by hand at SD-31-E6-F4-003 (see its own `IN_SCOPE_
RACES` comment) -- and drifted again the moment the 34th race landed,
because a hand-sync fixes one snapshot, not the recurrence. This module
closes it the way `v06_work_inventory::app_race_corpus_books` closes the
identical hazard for the race-book registry: read the roster off the
product's own authoritative declaration at runtime, never re-transcribe it.
"""

import os
import re
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
INGEST_RACE_TRAITS_RS = os.path.join(REPO_ROOT, "src", "bin", "ingest_race_traits.rs")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import race_trait_ceiling as ceiling  # noqa: E402


def _independently_parsed_roster():
    """A SEPARATE, deliberately naive re-parse of the same Rust declaration.

    Does not call anything in `race_trait_ceiling` -- a test that only
    checks the module under test agrees with itself proves nothing about
    whether it agrees with the real source.
    """
    with open(INGEST_RACE_TRAITS_RS, encoding="utf-8") as handle:
        src = handle.read()
    start = src.index("const IN_SCOPE_RACES:")
    end = src.index("];", start)
    return set(re.findall(r'"([^"]+)"', src[start:end]))


class InScopeRacesRosterTest(unittest.TestCase):
    def test_matches_the_authoritative_ingest_race_traits_declaration(self):
        expected = _independently_parsed_roster()
        # Canary against a silently-empty parse on either side: the real
        # roster has never shrunk and is currently 34 races. A test that
        # only asserted equality would pass vacuously if BOTH sides read
        # nothing.
        self.assertGreaterEqual(len(expected), 30)
        self.assertEqual(ceiling.IN_SCOPE_RACES, expected)

    def test_includes_every_race_named_by_each_widening_this_roster_missed(self):
        # SD-31 Epic 1-F2 (18 -> 24)
        for race in ("Fetchling", "Grippli", "Ifrit", "Oread", "Sylph", "Undine"):
            self.assertIn(race, ceiling.IN_SCOPE_RACES)
        # SD-31-E6-F4-003 (24 -> 30)
        for race in ("Catfolk", "Kitsune", "Ratfolk", "Strix", "Suli", "Wayang"):
            self.assertIn(race, ceiling.IN_SCOPE_RACES)
        # SD31-E6-F4-006 follow-on (30 -> 34) -- the widening that never
        # touched this file at all
        for race in ("Gillman", "Nagaji", "Vanara", "Vishkanya"):
            self.assertIn(race, ceiling.IN_SCOPE_RACES)

    def test_unparseable_declaration_yields_empty_set_not_a_stale_guess(self):
        # Under-claiming on a broken read is the safe direction (the same
        # contract `app_race_corpus_books` documents for the identical
        # hazard on the race-book registry) -- proven here by pointing the
        # reader at a repo root with no such file.
        self.assertEqual(ceiling.read_in_scope_races(repo_root="/nonexistent-repo-root"), set())


if __name__ == "__main__":
    unittest.main()
