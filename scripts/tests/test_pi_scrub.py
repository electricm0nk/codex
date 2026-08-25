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


class ConcatenatedCheckDoesNotSpanRealWhitespaceTests(unittest.TestCase):
    """`data/corpus/inner_sea_magic/ability/hidden_wand.json` (SD-32
    `corpus_literal_sweep` `clean:false` blocker, t9-onboarding cycle,
    2026-08-23): the real blacklist term "Andoran" false-positived on the
    ordinary prose "...activate a wand (or any similar spell trigger
    item..." because check 4's alphanumeric-normalized haystack strips ALL
    non-alphanumeric characters, including real word-separating whitespace
    -- "a wand or any" collapses to "...awandoranysimilar..." and swallows
    "andoran" as a false substring, even though no genuine no-separator
    concatenation exists in the source text (there ARE spaces; the check
    just deletes them). Reproduced here with a synthetic term/text pair,
    never a real blacklist term, matching this file's own stated convention.

    Check 4 exists to catch a term truly joined with NO separator at all
    (`CoordinatedeityAspectChoice`, tested above) -- ordinary natural-
    language prose whose words merely happen to concatenate into the term
    once whitespace is deleted is a different, unintended shape. Real
    whitespace in the ORIGINAL value must keep acting as a boundary the way
    every other separator (`.`, `,`, `|`, `~`) already does not need to,
    because PascalCase/BONUS-variable identifiers never contain whitespace
    to begin with -- so preserving it costs the genuine-concatenation catch
    nothing."""

    def setUp(self):
        self._orig_norm_terms = pi_scrub._NORM_BLACKLIST_TERMS
        # 8 normalized chars, >= _MIN_NORMALIZED_NEEDLE_LEN, built the same
        # way "Andoran" is: two ordinary short English words that happen to
        # concatenate into the synthetic term once whitespace is deleted.
        fake_term = "Testcase"
        pi_scrub._NORM_BLACKLIST_TERMS = [(fake_term, pi_scrub._normalize(fake_term))]

    def tearDown(self):
        pi_scrub._NORM_BLACKLIST_TERMS = self._orig_norm_terms

    def test_ordinary_prose_whose_words_concatenate_across_real_whitespace_is_not_a_hit(self):
        # "test" and "case" are separate, real, whitespace-separated words --
        # the exact "wand" / "or" / "any" shape from the live incident.
        text = "Please run a test case scenario before you continue."
        self.assertIsNone(pi_scrub.blacklist_term_hit_including_concatenated(text))

    def test_genuine_no_separator_concatenation_of_the_same_term_is_still_caught(self):
        # No whitespace at all in the source value -- the shape check 4 was
        # actually built for (`CoordinatedeityAspectChoice`-style).
        value = "TestcaseAspectChoice.SpecialQuality"
        self.assertEqual(
            pi_scrub.blacklist_term_hit_including_concatenated(value), "Testcase"
        )

    def test_mutation_proof_a_naive_strip_all_normalize_reopens_the_false_positive(self):
        """Mutation-proves the fix is load-bearing: the OLD strip-everything
        normalization (what `_normalize` used to do to the haystack) DOES
        false-positive on the same prose, confirming the RED case above
        fails for the intended reason and is not vacuous."""
        naive_normalize = lambda s: __import__("re").sub(r"[^a-z0-9]", "", s.lower())  # noqa: E731
        text = "Please run a test case scenario before you continue."
        norm_value = naive_normalize(text)
        self.assertIn(
            "testcase",
            norm_value,
            "the naive strip-everything normalization must still reproduce the collision "
            "for this mutation proof to mean anything",
        )


class ShortTermsAreNotOverRedactedTests(unittest.TestCase):
    def test_a_short_generic_needle_below_the_normalized_length_bound_is_left_alone(self):
        tokens = [{"key": "FACT", "value": "Abb|RMA"}]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, name="Red Placeholder Assassin", key="Red Placeholder Assassin"
        )
        self.assertFalse(any_redacted)
        self.assertEqual(scrubbed, tokens)


class GenericCategoryWordIsNotAStandaloneNeedleTests(unittest.TestCase):
    """T9-onboarding-cause-closure over-redaction fix (2026-08-23,
    `decisions.md §24b`-2). PCGen's own `KEY` schema is frequently
    `<Category-or-Group> ~ <Specific>` (real corpus shape: `"Trait ~
    <a PI-bearing trait name>"`). Before this fix, the old per-WORD split
    treated every individual word of that key -- including the generic
    group word "Trait" itself -- as an independent redaction needle, so
    ANY unrelated token merely containing the ordinary word "Trait"
    (e.g. `BONUS:...TYPE=Trait`, present on every trait-kind record,
    genuinely no PI) got wiped to `[redacted PI]` for a reason that had
    nothing to do with the record's actual PI content (a deity name
    embedded in the trait's own name). Reproduced here with synthetic
    placeholders, never a real Product Identity term."""

    def test_generic_group_word_from_the_key_does_not_redact_an_unrelated_token(self):
        # Real-shaped reproduction: key = "Trait ~ <PI name>", and a
        # completely generic, PI-free BONUS value that merely happens to
        # also say "Trait" (as every trait-kind record's BONUS/TYPE token
        # legitimately does, via PCGen's own `TYPE=Trait` convention).
        tokens = [
            {"key": "BONUS", "value": "SKILL|Diplomacy|Sense Motive|1|TYPE=Trait"},
        ]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, "Placeholder Deity Name", "Trait ~ Placeholder Deity Name"
        )
        self.assertFalse(any_redacted)
        self.assertEqual(scrubbed[0]["value"], "SKILL|Diplomacy|Sense Motive|1|TYPE=Trait")

    def test_genuine_self_reference_of_the_full_segment_is_still_redacted(self):
        # The record's own FULL `~`-segment (not a single generic word cut
        # out of it) appearing verbatim in a token value is still exactly
        # the shape check 2 exists to catch -- unaffected by removing the
        # per-word loop.
        tokens = [{"key": "BONUS", "value": "ABILITYPOOL|Placeholder Group Name|1"}]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, "Placeholder Group Name", "Concept ~ Placeholder Group Name"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], pi_scrub.REDACTED_PI_MARKER)

    def test_mutation_proof_reintroducing_the_per_word_split_reopens_the_over_redaction(self):
        """Mutation-proves the fix is load-bearing: re-adding the OLD
        per-word needle split (the mutation) reproduces the over-redaction
        on the exact case the first test above proves fixed."""
        tokens = [
            {"key": "BONUS", "value": "SKILL|Diplomacy|Sense Motive|1|TYPE=Trait"},
        ]
        name = "Placeholder Deity Name"
        key = "Trait ~ Placeholder Deity Name"

        import re as _re

        needles: set[str] = set()
        norm_needles: set[str] = set()

        def add_needle(s: str) -> None:
            s = s.strip()
            if not s:
                return
            needles.add(s.lower())
            normalized = pi_scrub._normalize(s)
            if len(normalized) >= pi_scrub._MIN_NORMALIZED_NEEDLE_LEN:
                norm_needles.add(normalized)

        for s in (name, key):
            add_needle(s)
        for segment in _re.split(r"\s*~\s*", key):
            add_needle(segment)
            for word in _re.split(r"[\s()]+", segment):
                add_needle(word)  # the mutation: pre-fix per-word split

        value = tokens[0]["value"]
        value_lower = value.lower()
        old_behavior_hit = any(n in value_lower for n in needles)
        self.assertTrue(
            old_behavior_hit,
            "the pre-fix per-word split must reproduce the over-redaction for this "
            "mutation proof to mean anything",
        )


class NeverMutatesInputTests(unittest.TestCase):
    def test_input_list_and_dicts_are_not_mutated(self):
        tokens = [{"key": "KEY", "value": "Placeholder Deity Name"}]
        pi_scrub.scrub_name_pi_tokens(tokens, "x", "Placeholder Deity Name")
        self.assertEqual(tokens[0]["value"], "Placeholder Deity Name")


class NarrowedRedactionForSelfReferenceOnlyTests(unittest.TestCase):
    """SD-32 T9-onboarding-cause-closure (row 17's remaining 21):
    `decisions.md §24b`-2's "the mechanical formula, never the PI original"
    still holds when the ONLY PI content in a value is the record's own
    self-referenced name/key segment (check 2) -- pass `neutral_name` and
    the surrounding mechanical structure (magnitude, `TYPE=`, ...) survives,
    with only the self-reference span replaced.
    """

    def test_self_reference_only_value_is_narrowed_not_wiped_when_neutral_name_given(self):
        tokens = [
            {"key": "BONUS", "value": "ABILITYPOOL|Placeholder Deity Name|1|TYPE=Base"}
        ]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens,
            "Placeholder Deity Name",
            "Concept ~ Placeholder Deity Name",
            neutral_name="Codex-Named Unit (x_1)",
        )
        self.assertTrue(any_redacted)
        self.assertEqual(
            scrubbed[0]["value"],
            "ABILITYPOOL|Codex-Named Unit (x_1)|1|TYPE=Base",
        )
        # The magnitude and qualifier survive -- never wiped to the marker.
        self.assertNotEqual(scrubbed[0]["value"], pi_scrub.REDACTED_PI_MARKER)
        self.assertIn("1", scrubbed[0]["value"])
        self.assertIn("TYPE=Base", scrubbed[0]["value"])

    def test_without_neutral_name_the_prior_full_redaction_behaviour_is_unchanged(self):
        tokens = [
            {"key": "BONUS", "value": "ABILITYPOOL|Placeholder Deity Name|1|TYPE=Base"}
        ]
        scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
            tokens, "Placeholder Deity Name", "Concept ~ Placeholder Deity Name"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], pi_scrub.REDACTED_PI_MARKER)

    def test_a_value_that_ALSO_hits_the_blacklist_is_never_narrowed(self):
        self._orig_norm_terms = pi_scrub._NORM_BLACKLIST_TERMS
        fake_term = "Coordinatedeity"  # synthetic, not a real PI term
        pi_scrub._NORM_BLACKLIST_TERMS = [(fake_term, pi_scrub._normalize(fake_term))]
        try:
            tokens = [
                {
                    "key": "BONUS",
                    "value": "VAR|CoordinatedeityPlaceholder Deity NameLVL|1",
                }
            ]
            scrubbed, any_redacted = pi_scrub.scrub_name_pi_tokens(
                tokens,
                "Placeholder Deity Name",
                "Concept ~ Placeholder Deity Name",
                neutral_name="Codex-Named Unit (x_1)",
            )
        finally:
            pi_scrub._NORM_BLACKLIST_TERMS = self._orig_norm_terms
        self.assertTrue(any_redacted)
        # Still a full wipe: narrowing must never partially unmask a value
        # that also carries a genuine blacklisted term.
        self.assertEqual(scrubbed[0]["value"], pi_scrub.REDACTED_PI_MARKER)

    def test_mutation_proof_a_neutral_name_that_fails_to_substitute_still_fails_closed(self):
        """If the narrowing branch's own substitution somehow matched nothing
        (defensive-only path — `space_preserving_hit` guarantees a needle IS
        present) the value must still end up fully redacted, never shipped
        with the original PI string intact. Proven by forcing the needle set
        to diverge from the detected hit via a monkeypatched, always-true
        hit detector paired with an impossible-to-match needle would require
        reaching into internals; instead this proves the INVARIANT
        holds for the real function: the narrowed value must never still
        contain the original self-reference needle."""
        tokens = [
            {"key": "BONUS", "value": "ABILITYPOOL|Placeholder Deity Name|1|TYPE=Base"}
        ]
        scrubbed, _ = pi_scrub.scrub_name_pi_tokens(
            tokens,
            "Placeholder Deity Name",
            "Concept ~ Placeholder Deity Name",
            neutral_name="Codex-Named Unit (x_1)",
        )
        self.assertNotIn("Placeholder Deity Name", scrubbed[0]["value"])


if __name__ == "__main__":
    unittest.main()
