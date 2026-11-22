#!/usr/bin/env python3
"""Tests for `scripts/classify_race_trait_rows.py`'s `IN_SCOPE_RACES` roster.

WHY THIS FILE EXISTS (SD-31 wave-21 race_trait lane). Same defect as
`scripts/tests/test_race_trait_ceiling.py` documents for its sibling script,
one file over: this module's `IN_SCOPE_RACES` was hand-transcribed from
`ingest_race_traits.rs`'s own `IN_SCOPE_RACES` and re-synced once, by hand,
at SD-31-E6-F4-003 (18 -> 30) -- and was already stale again the moment
SD31-E6-F4-006 (Gillman/Nagaji/Vanara/Vishkanya, 30 -> 34) landed without
ever touching this file. A hand-sync fixes one snapshot, not the recurring
pattern; reading the declaration closes it structurally.
"""

import os
import re
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
INGEST_RACE_TRAITS_RS = os.path.join(REPO_ROOT, "src", "bin", "ingest_race_traits.rs")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import classify_race_trait_rows as classify_rows  # noqa: E402


def _independently_parsed_roster():
    with open(INGEST_RACE_TRAITS_RS, encoding="utf-8") as handle:
        src = handle.read()
    start = src.index("const IN_SCOPE_RACES:")
    end = src.index("];", start)
    return set(re.findall(r'"([^"]+)"', src[start:end]))


class InScopeRacesRosterTest(unittest.TestCase):
    def test_matches_the_authoritative_ingest_race_traits_declaration(self):
        expected = _independently_parsed_roster()
        self.assertGreaterEqual(len(expected), 30)
        self.assertEqual(set(classify_rows.IN_SCOPE_RACES), expected)

    def test_includes_the_four_newest_follow_on_races(self):
        for race in ("Gillman", "Nagaji", "Vanara", "Vishkanya"):
            self.assertIn(race, classify_rows.IN_SCOPE_RACES)


if __name__ == "__main__":
    unittest.main()
