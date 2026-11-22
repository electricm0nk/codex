#!/usr/bin/env python3
"""Tests for `scripts/sd32_t9_corpus_wide_pi_rescan.py`'s `kind_from_path`.

SD-32 card 11, t9-onboarding-pi-final-leaks-and-generators cycle
(`decisions.md §17a`): the function's original implementation read
`rel.parts[1]`, which for a repo-rooted path shaped
`data/corpus/<book>/<kind>/<file>.json` is always the literal string
"corpus" -- not the kind. Every per-kind row the script ever printed before
this fix silently read `kind=corpus`, collapsing every kind into one
bucket; the script's own consumers worked around this by piping the path
list through `awk -F'/' '{print $4}'` instead of trusting the function.
This test proves the fix directly against the function, not a downstream
workaround.
"""
from __future__ import annotations

import os
import sys
from pathlib import Path
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import sd32_t9_corpus_wide_pi_rescan as rescan  # noqa: E402


class KindFromPathTests(unittest.TestCase):
    def test_kind_is_the_fourth_path_component_not_the_literal_corpus(self):
        rel = Path("data/corpus/inner_sea_gods/equipment/some_record.json")
        self.assertEqual(rescan.kind_from_path(rel), "equipment")

    def test_a_different_kind_and_book_still_resolves_correctly(self):
        rel = Path("data/corpus/inner_sea_magic/class_feature/varisian_pilgrim/caravan_bond.json")
        self.assertEqual(rescan.kind_from_path(rel), "class_feature")

    def test_a_nested_subdirectory_under_kind_still_resolves_to_the_kind_not_the_subdir(self):
        rel = Path("data/corpus/inner_sea_world_guide/template/human_ethnicity_garundi.json")
        self.assertEqual(rescan.kind_from_path(rel), "template")

    def test_the_old_buggy_behaviour_would_have_returned_the_literal_string_corpus(self):
        # Mutation proof (`§1a`): reproduce the exact pre-fix expression
        # inline (never re-import a reverted copy of the module) and show
        # it disagrees with the fixed function on the same input -- proving
        # this test would have failed red against the old code.
        rel = Path("data/corpus/inner_sea_gods/equipment/some_record.json")
        old_buggy_result = rel.parts[1]
        self.assertEqual(old_buggy_result, "corpus")
        self.assertNotEqual(old_buggy_result, rescan.kind_from_path(rel))

    def test_a_short_path_falls_back_to_the_unknown_marker(self):
        self.assertEqual(rescan.kind_from_path(Path("data/corpus")), "?")


if __name__ == "__main__":
    unittest.main()
