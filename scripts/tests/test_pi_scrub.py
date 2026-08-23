#!/usr/bin/env python3
"""Tests for `scripts/pi_scrub.py` -- the ONE shared `scrub_name_pi_tokens`
implementation, extracted after `ingest_ability.py` and `ingest_generic_kind.py`
carried two independently-maintained copies that drifted (SD-32 T9-onboarding-
cause-closure cycle; see the module docstring).

All PI-shaped strings in this file are synthetic placeholders, never a real
Product Identity term -- the blacklist-term test builds its own throwaway
6+-letter "term" and monkeypatches it into the module's normalized-term table
rather than importing a real deity/place name from `PI_BLACKLIST_TERMS`.
"""
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import pi_scrub  # noqa: E402


class IdentityConcatenationTests(unittest.TestCase):
    """The `RedMantisAssassinLVL` shape: the record's OWN name/key,
    concatenated with no separator into another token's value."""

    def test_pascalcase_identifier_embedding_the_records_own_name_is_redacted(self):
        tokens = [
            {"key": "DEFINE", "value": "PlaceholderDeityNameLVL|0"},
            {"key": "FACT", "value": "Abb|PDN"},  # short, generic -- left alone
        ]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, "Placeholder Deity Name", "Placeholder Deity Name"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], pi_scrub.REDACTED_PI_MARKER)
        self.assertEqual(scrubbed[1]["value"], "Abb|PDN")

    def test_separated_occurrence_still_caught_by_the_space_preserving_check(self):
        tokens = [{"key": "PREREQ", "value": "Must worship Placeholder Deity Name"}]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, "Something", "Concept ~ Placeholder Deity Name"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], pi_scrub.REDACTED_PI_MARKER)


class BlacklistTermConcatenationTests(unittest.TestCase):
    """The shape found live in an already-shipped `codex_named_unit_*`
    record: a BLACKLISTED term (not the record's own identity) concatenated
    PascalCase-style into another token's value, defeating
    `normalized_term_hit`'s word-boundary requirement (which exists to avoid
    the recorded short-blacklist-term-inside-an-ordinary-word false
    positive) because the character
    immediately following the term is a letter, never a boundary.

    Uses a synthetic 9-letter placeholder term substituted into the module's
    own normalized-blacklist table for the duration of the test -- never a
    real Product Identity string."""

    def setUp(self):
        self._orig_norm_terms = pi_scrub._NORM_BLACKLIST_TERMS
        fake_term = "Coordinatedeity"  # synthetic, not a real PI term
        pi_scrub._NORM_BLACKLIST_TERMS = [(fake_term, pi_scrub._normalize(fake_term))]

    def tearDown(self):
        pi_scrub._NORM_BLACKLIST_TERMS = self._orig_norm_terms

    def test_concatenated_blacklist_term_with_no_word_boundary_is_redacted(self):
        # "CoordinatedeityAspectChoice" -- the term immediately followed by
        # another capitalized word, no separator, no boundary character.
        tokens = [{"key": "TYPE", "value": "CoordinatedeityAspectChoice.SpecialQuality"}]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, name="Unrelated Display Name", key="Unrelated Display Name"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], pi_scrub.REDACTED_PI_MARKER)

    def test_word_bounded_occurrence_of_the_same_term_is_still_redacted(self):
        """Sanity: the ordinary, separated case (already covered by
        `normalized_term_hit`) is not broken by adding the concatenated-form
        check."""
        tokens = [{"key": "DESC", "value": "You worship Coordinatedeity in this aspect."}]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, name="Unrelated Display Name", key="Unrelated Display Name"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], pi_scrub.REDACTED_PI_MARKER)

    def test_mutation_proof_removing_check_4_lets_the_concatenated_form_through(self):
        """Mutation-proves the concatenated-blacklist-term check (check 4)
        is actually load-bearing: with `_NORM_BLACKLIST_TERMS` emptied (the
        mutation), the concatenated-form leak is NOT caught -- confirming
        the test above fails for the intended reason, not by accident."""
        pi_scrub._NORM_BLACKLIST_TERMS = []  # mutation: disable check 4 only
        tokens = [{"key": "TYPE", "value": "CoordinatedeityAspectChoice.SpecialQuality"}]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, name="Unrelated Display Name", key="Unrelated Display Name"
        )
        self.assertFalse(any_redacted)
        self.assertEqual(scrubbed[0]["value"], "CoordinatedeityAspectChoice.SpecialQuality")


class ShortTermsAreNotOverRedactedTests(unittest.TestCase):
    def test_a_short_generic_needle_below_the_normalized_length_bound_is_left_alone(self):
        tokens = [{"key": "FACT", "value": "Abb|RMA"}]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, name="Red Placeholder Assassin", key="Red Placeholder Assassin"
        )
        self.assertFalse(any_redacted)
        self.assertEqual(scrubbed, tokens)


class NeverMutatesInputTests(unittest.TestCase):
    def test_input_list_and_dicts_are_not_mutated(self):
        tokens = [{"key": "KEY", "value": "Placeholder Deity Name"}]
        pi_scrub.scrub_name_pi_tokens(tokens, "x", "Placeholder Deity Name")
        self.assertEqual(tokens[0]["value"], "Placeholder Deity Name")


if __name__ == "__main__":
    unittest.main()
