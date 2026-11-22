#!/usr/bin/env python3
"""Tests for `scripts/ingest_ability.py`'s `decisions.md §24` rename path.

`decisions.md §3` fixture discipline, adapted per `§24b`-5: "the fixture
pins the Codex name and the record's mechanical content transcribed from
oracle bytes. The comment records the rename and the coordinate, not the
original name." `RENAMED_ABILITY_FIXTURE` below is that fixture. It was
transcribed by running `python3 scripts/ingest_ability.py` against the
pinned PCGen oracle and reading the emitted
`data/corpus/inner_sea_faiths/ability/codex_named_unit_ability_inner_sea_faiths_isf_abilities_faith_lst_117.json`
back — coordinate `inner_sea_faiths:isf_abilities_faith.lst:117`, renamed
because its row declares `NAMEISPI:YES`. **This file must never carry the
original printed name** — only the coordinate identifies which record the
fixture pins.
"""
from __future__ import annotations

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import ingest_ability as ia  # noqa: E402
from codex_neutral_name import neutral_key, neutral_name  # noqa: E402

# Coordinate: inner_sea_faiths:isf_abilities_faith.lst:117 (NAMEISPI:YES row).
RENAMED_ABILITY_FIXTURE = {
    "data": {
        "key": "Codex-Named Unit (ability_inner_sea_faiths_isf_abilities_faith_lst_117)",
        "name": "Codex-Named Unit (ability_inner_sea_faiths_isf_abilities_faith_lst_117)",
        "description": None,
        "raw_tokens": [
            {"key": "KEY", "value": "[redacted PI]"},
            {"key": "NAMEISPI", "value": "YES"},
            {"key": "CATEGORY", "value": "Internal"},
            {
                "key": "SPELLS",
                "value": (
                    "Divine Boon Choice|TIMES=3|CASTERLEVEL=TL|Crafter's Fortune|"
                    "PREMULT:1,[PREVARGTEQ:DeificBoonLVL,12],[PREVARGTEQ:ExaltedBoonLVL,3]"
                ),
            },
            {
                "key": "SPELLS",
                "value": (
                    "Divine Boon Choice|TIMES=2|CASTERLEVEL=TL|Make Whole|"
                    "PREMULT:1,[PREVARGTEQ:DeificBoonLVL,12],[PREVARGTEQ:ExaltedBoonLVL,3]"
                ),
            },
            {
                "key": "SPELLS",
                "value": (
                    "Divine Boon Choice|TIMES=1|CASTERLEVEL=TL|Minor Creation|"
                    "PREMULT:1,[PREVARGTEQ:DeificBoonLVL,12],[PREVARGTEQ:ExaltedBoonLVL,3]"
                ),
            },
            {
                "key": "SPELLS",
                "value": (
                    "Divine Boon|CASTERLEVEL=TL|Fabricate|"
                    "PREMULT:1,[PREVARGTEQ:DeificBoonLVL,20],[PREVARGTEQ:ExaltedBoonLVL,9]"
                ),
            },
            {"key": "BONUS", "value": "SKILL|Disable Device|4"},
            {
                "key": "ASPECT",
                "value": (
                    "SaveBonus|+2 sacred or profane bonus on saves vs. effects that cause "
                    "ability damage, ability drain, energy drain, exhaustion, fatigue, or "
                    "nonlethal damage.|PREMULT:1,[PREVARGTEQ:DeificBoonLVL,16],"
                    "[PREVARGTEQ:ExaltedBoonLVL,6]"
                ),
            },
        ],
    },
    "wiring_class": "static",
    "codex_generated_name": True,
    "rename": {
        "reason": "name_pi_blocked",
        "coordinate": "inner_sea_faiths:isf_abilities_faith.lst:117",
    },
    "license": "PI-REDACTED",
    "pi_field": "name",
    "pi_marker": "redacted",
}


class FixturePinsTheCodexNameNotTheOriginalTest(unittest.TestCase):
    def test_fixture_name_matches_the_pure_coordinate_derivation(self):
        expected = neutral_name("ability", "inner_sea_faiths", "isf_abilities_faith.lst", 117)
        self.assertEqual(RENAMED_ABILITY_FIXTURE["data"]["name"], expected)
        self.assertEqual(
            RENAMED_ABILITY_FIXTURE["data"]["key"],
            neutral_key("ability", "inner_sea_faiths", "isf_abilities_faith.lst", 117),
        )

    def test_fixture_carries_no_original_identity_column(self):
        # The KEY token must be redacted, not the row's own original key --
        # `scrub_name_pi_tokens` is the mechanism under test elsewhere in
        # this file; this fixture just pins its OUTPUT.
        key_token = next(t for t in RENAMED_ABILITY_FIXTURE["data"]["raw_tokens"] if t["key"] == "KEY")
        self.assertEqual(key_token["value"], "[redacted PI]")

    def test_fixture_marks_the_rename_visibly(self):
        self.assertTrue(RENAMED_ABILITY_FIXTURE["codex_generated_name"])
        self.assertEqual(RENAMED_ABILITY_FIXTURE["rename"]["reason"], "name_pi_blocked")
        self.assertEqual(
            RENAMED_ABILITY_FIXTURE["rename"]["coordinate"],
            "inner_sea_faiths:isf_abilities_faith.lst:117",
        )
        # §24b-4: the rename record stops at the coordinate.
        self.assertEqual(set(RENAMED_ABILITY_FIXTURE["rename"].keys()), {"reason", "coordinate"})

    def test_fixture_mechanical_tokens_are_transcribed_verbatim(self):
        """Every non-identity token is the byte-verbatim mechanical content
        -- `§24b`-5's "record's mechanical content transcribed from oracle
        bytes." Spot-check the BONUS token, which carries no PI risk and so
        must be untouched."""
        bonus_token = next(t for t in RENAMED_ABILITY_FIXTURE["data"]["raw_tokens"] if t["key"] == "BONUS")
        self.assertEqual(bonus_token["value"], "SKILL|Disable Device|4")


class ScrubNamePiTokensTest(unittest.TestCase):
    """Direct unit tests of `ingest_ability.py::scrub_name_pi_tokens`,
    using entirely synthetic (non-PI) placeholder strings."""

    def test_a_key_token_restating_the_original_key_is_redacted(self):
        tokens = [
            {"key": "KEY", "value": "Concept Category ~ Placeholder Deity Name"},
            {"key": "BONUS", "value": "SKILL|Perception|2"},
        ]
        scrubbed, any_redacted = ia.scrub_name_pi_tokens(
            tokens, name="Placeholder Deity Name's Obedience", key="Concept Category ~ Placeholder Deity Name"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], ia.REDACTED_PI_MARKER)
        self.assertEqual(scrubbed[1]["value"], "SKILL|Perception|2")  # untouched -- no PI

    def test_a_segment_of_the_key_appearing_alone_is_also_redacted(self):
        """A `~`-delimited segment of `key` (e.g. just the deity's own name)
        appearing standalone in another field must still be caught."""
        tokens = [{"key": "PREREQ", "value": "Must worship Placeholder Deity Name"}]
        scrubbed, any_redacted = ia.scrub_name_pi_tokens(
            tokens, name="Something", key="Concept Category ~ Placeholder Deity Name"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], ia.REDACTED_PI_MARKER)

    def test_an_unrelated_token_is_left_alone(self):
        tokens = [{"key": "BONUS", "value": "SKILL|Stealth|4"}]
        scrubbed, any_redacted = ia.scrub_name_pi_tokens(
            tokens, name="Placeholder Deity Name's Obedience", key="Concept Category ~ Placeholder Deity Name"
        )
        self.assertFalse(any_redacted)
        self.assertEqual(scrubbed, tokens)

    def test_never_mutates_the_input_list(self):
        tokens = [{"key": "KEY", "value": "Concept Category ~ Placeholder Deity Name"}]
        original = [dict(t) for t in tokens]
        ia.scrub_name_pi_tokens(tokens, name="x", key="Concept Category ~ Placeholder Deity Name")
        self.assertEqual(tokens, original)


if __name__ == "__main__":
    unittest.main()
