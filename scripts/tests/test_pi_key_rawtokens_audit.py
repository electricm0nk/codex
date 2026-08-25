#!/usr/bin/env python3
"""Tests for `scripts/pi_key_rawtokens_audit.py` -- the generic corpus-wide
`data.key`/`data.raw_tokens` PI screen (`decisions.md §17` gap-close).

Never types a real blacklist term literally; every test indexes into the
imported `PI_BLACKLIST_TERMS` list instead.
"""
from __future__ import annotations

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import pi_key_rawtokens_audit as audit  # noqa: E402
from sd32_t9_pi_review_feat_equipment import PI_BLACKLIST_TERMS  # noqa: E402

_TERM = PI_BLACKLIST_TERMS[10]  # a real blacklisted deity name, referenced by index only


class ScreenRecordTest(unittest.TestCase):
    def test_a_clean_record_has_no_confirmed_or_candidate_hits(self):
        record = {
            "data": {
                "key": "Adept",
                "name": "Adept",
                "raw_tokens": [{"key": "BONUS", "value": "SKILL|Perception|2"}],
            }
        }
        result = audit.screen_record(record)
        self.assertEqual(result["confirmed_terms"], set())

    def test_a_blacklist_term_in_a_non_desc_token_is_confirmed(self):
        record = {
            "data": {
                "key": "Adept",
                "name": "Adept",
                "raw_tokens": [{"key": "SPELLLEVEL", "value": f"PREDEITY:1,{_TERM}"}],
            }
        }
        result = audit.screen_record(record)
        self.assertEqual(result["confirmed_terms"], {_TERM})

    def test_a_blacklist_term_in_key_is_confirmed(self):
        record = {"data": {"key": f"Concept ~ {_TERM}", "name": "Concept", "raw_tokens": []}}
        result = audit.screen_record(record)
        self.assertEqual(result["confirmed_terms"], {_TERM})

    def test_an_already_redacted_token_is_not_a_leak(self):
        record = {
            "data": {
                "key": "Adept",
                "name": "Adept",
                "raw_tokens": [{"key": "SPELLLEVEL", "value": "[redacted PI]"}],
            }
        }
        result = audit.screen_record(record)
        self.assertEqual(result["confirmed_terms"], set())

    def test_a_capitalized_non_blacklist_word_is_a_candidate_not_confirmed(self):
        record = {
            "data": {
                "key": "Adept",
                "name": "Adept",
                "raw_tokens": [{"key": "PREREQ", "value": "Must belong to House Thrune"}],
            }
        }
        result = audit.screen_record(record)
        self.assertEqual(result["confirmed_terms"], set())
        self.assertIn("Thrune", result["candidate_terms"])

    def test_common_english_capitalized_words_are_not_candidates(self):
        result = audit.candidate_terms("The Special Ability grants a Bonus When triggered")
        self.assertEqual(result, set())


class NameAlreadyFlaggedTest(unittest.TestCase):
    """§17a validation catch: a record whose `data.name` is ALREADY
    `[redacted PI]` (an earlier screen already caught it) must not be
    reported as a NEW confirmed leak just because the literal marker
    string contains no blacklist term. 26 of a 30-record sample were this
    false-positive shape before this fix."""

    def test_the_redaction_marker_itself_counts_as_already_flagged(self):
        self.assertTrue(audit.name_already_flagged(audit.REDACTED_PI_MARKER))

    def test_a_fresh_blacklist_hit_counts_as_already_flagged(self):
        self.assertTrue(audit.name_already_flagged(_TERM))

    def test_a_genuinely_clean_name_is_not_flagged(self):
        self.assertFalse(audit.name_already_flagged("Death"))

    def test_an_empty_name_is_not_flagged(self):
        self.assertFalse(audit.name_already_flagged(""))


if __name__ == "__main__":
    unittest.main()


if __name__ == "__main__":
    unittest.main()
