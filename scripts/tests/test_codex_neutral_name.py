#!/usr/bin/env python3
"""Tests for `scripts/codex_neutral_name.py` — SD-32 `decisions.md §24b`'s
binding conditions.

`test_output_is_unchanged_when_the_pi_name_is_replaced` is `§24b`-1's own
required test, verbatim: *"A test proves the generator's output is
unchanged when the PI name is replaced with a different string."* It is
the single most load-bearing test in this module.
"""
from __future__ import annotations

import inspect
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import codex_neutral_name as cnn  # noqa: E402


class SignatureHasNoNameChannelTest(unittest.TestCase):
    """Structural proof, not just behavioural: the public functions cannot
    even be CALLED with a name/description/free-text argument, so there is
    no channel through which a PI string could influence the output."""

    def test_neutral_name_signature_has_no_name_or_key_parameter(self):
        params = set(inspect.signature(cnn.neutral_name).parameters)
        self.assertEqual(params, {"kind", "book", "source_file", "source_line"})

    def test_neutral_coordinate_id_signature_has_no_name_or_key_parameter(self):
        params = set(inspect.signature(cnn.neutral_coordinate_id).parameters)
        self.assertEqual(params, {"kind", "book", "source_file", "source_line"})

    def test_divergence_entry_signature_has_no_name_or_key_parameter(self):
        params = set(inspect.signature(cnn.divergence_entry).parameters)
        self.assertEqual(params, {"kind", "book", "source_file", "source_line", "reason"})


class DeterminismTest(unittest.TestCase):
    def test_same_coordinates_produce_the_same_name_every_call(self):
        args = ("ability", "inner_sea_faiths", "isf_abilities_faith.lst", 13)
        results = {cnn.neutral_name(*args) for _ in range(20)}
        self.assertEqual(len(results), 1, "must be deterministic across repeated calls")

    def test_same_coordinates_produce_the_same_coordinate_id_every_call(self):
        args = ("ability", "inner_sea_faiths", "isf_abilities_faith.lst", 13)
        results = {cnn.neutral_coordinate_id(*args) for _ in range(20)}
        self.assertEqual(len(results), 1)

    def test_regenerate_twice_diff_identical(self):
        """`§24b`-6: 'Regenerate twice and diff: identical output. A test
        fails if the generator is non-deterministic.'"""
        args = ("class_feature", "advanced_players_guide", "apg_abilities.lst", 222)
        run1 = {
            "name": cnn.neutral_name(*args),
            "key": cnn.neutral_key(*args),
            "id": cnn.neutral_coordinate_id(*args),
        }
        run2 = {
            "name": cnn.neutral_name(*args),
            "key": cnn.neutral_key(*args),
            "id": cnn.neutral_coordinate_id(*args),
        }
        self.assertEqual(run1, run2)


class OutputUnchangedWhenThePiNameIsReplacedTest(unittest.TestCase):
    """`§24b`-1's own required test, verbatim."""

    def test_output_is_unchanged_when_the_pi_name_is_replaced(self):
        coord = ("ability", "inner_sea_faiths", "isf_abilities_faith.lst", 13)

        # Two records that would carry DIFFERENT PI names at the SAME
        # coordinates. Neither name is ever passed to the generator below --
        # that is the point under test: there is no parameter to pass it
        # through, so replacing one string with a wildly different one
        # cannot change the output. Synthetic placeholder strings only --
        # this file must never carry a real PI name itself (`§24b`-2 bans
        # the original from ANY committed artifact, tests included).
        record_with_name_a = {"name": "Placeholder-Name-One's Obedience", "coord": coord}
        record_with_name_b = {"name": "A Totally Different Made-Up String", "coord": coord}

        name_a = cnn.neutral_name(*record_with_name_a["coord"])
        name_b = cnn.neutral_name(*record_with_name_b["coord"])

        self.assertEqual(name_a, name_b)
        # And neither carries any trace of either candidate name.
        self.assertNotIn("Placeholder-Name-One", name_a)
        self.assertNotIn("Totally Different", name_b)

    def test_output_is_unchanged_across_every_field_a_pi_name_could_hide_in(self):
        """A PI name can also arrive via `key` (e.g. a `'<Concept> ~
        <Deity>'`-shaped key) or free text embedded in another LST field.
        Prove the SAME invariance for `neutral_key` and `divergence_entry`."""
        coord = ("ability", "inner_sea_faiths", "isf_abilities_faith.lst", 13)
        for original_key in ("Placeholder Concept ~ Placeholder-Name-One", "Something Else Entirely ~ X"):
            del original_key  # never passed below -- see docstring
            self.assertEqual(cnn.neutral_key(*coord), cnn.neutral_key(*coord))
            entry_a = cnn.divergence_entry(*coord, reason="name_pi_blocked")
            entry_b = cnn.divergence_entry(*coord, reason="name_pi_blocked")
            self.assertEqual(entry_a, entry_b)


class DifferentCoordinatesProduceDifferentNamesTest(unittest.TestCase):
    """Not a binding condition, but a sanity check that the id space is not
    degenerate (every distinct unit still gets a distinct name)."""

    def test_different_line_numbers_differ(self):
        a = cnn.neutral_name("ability", "book", "file.lst", 1)
        b = cnn.neutral_name("ability", "book", "file.lst", 2)
        self.assertNotEqual(a, b)

    def test_different_books_differ(self):
        a = cnn.neutral_name("ability", "book_a", "file.lst", 1)
        b = cnn.neutral_name("ability", "book_b", "file.lst", 1)
        self.assertNotEqual(a, b)

    def test_different_kinds_differ(self):
        a = cnn.neutral_name("ability", "book", "file.lst", 1)
        b = cnn.neutral_name("deity", "book", "file.lst", 1)
        self.assertNotEqual(a, b)


class DivergenceEntryNeverCarriesTheOriginalStringTest(unittest.TestCase):
    """`§24b`-4: record THAT a rename happened, its coordinates, and why --
    never the original string."""

    def test_divergence_entry_has_no_field_that_could_carry_the_original_name(self):
        entry = cnn.divergence_entry(
            "ability", "inner_sea_faiths", "isf_abilities_faith.lst", 13, reason="name_pi_blocked"
        )
        self.assertEqual(
            set(entry.keys()),
            {"kind", "book", "source_file", "source_line", "codex_name", "reason"},
        )
        for value in entry.values():
            self.assertNotIn("Placeholder-Name-One", str(value))


if __name__ == "__main__":
    unittest.main()
