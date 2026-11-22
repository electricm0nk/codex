#!/usr/bin/env python3
"""Tests for `scripts/ingest_ability.py`'s raw-tokens blacklist screen for
records whose bare `name`/`key` are NOT PI (`decisions.md §17` gap-close).

**The gap.** Before this fix, a record whose name/key are clean skipped the
"name is PI" branch entirely, and only its `DESC` token was screened
(declared `DESCISPI:YES` or a blacklist hit in the free-text description).
Every OTHER raw-token VALUE — `PREDEITY:`, `TYPE:`, `SPELLLEVEL:`, etc. —
was stored verbatim, unscanned. Two already-shipped records proved this
live: `data/corpus/inner_sea_gods/ability/adept.json` (a `SPELLLEVEL` token
carries `PREDEITY:1,<deity>`) and
`data/corpus/inner_sea_magic/ability/diplomatic_student.json` (`TYPE` and
`PREABILITY` tokens both carry the same institution name), while both
records' bare `name` is clean. `scrub_blacklist_pi_tokens` below is the
generic fix: every token VALUE is scanned against the SIGNED-OFF 60-term
blacklist (`decisions.md §19`), independent of whether the record's own
name triggered a rename.

This file deliberately never types a real blacklist term literally — every
test indexes into the imported `PI_BLACKLIST_TERMS` list instead, per this
cycle's own instruction not to carry a real PI string in test code.
"""
from __future__ import annotations

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import ingest_ability as ia  # noqa: E402
from sd32_t9_pi_review_feat_equipment import PI_BLACKLIST_TERMS  # noqa: E402

# A real blacklisted term, referenced by index only -- never typed literally
# in this file's source.
_TERM = PI_BLACKLIST_TERMS[14]


class ScrubBlacklistPiTokensTest(unittest.TestCase):
    """Direct unit tests of `ingest_ability.py::scrub_blacklist_pi_tokens`."""

    def test_a_non_desc_token_carrying_a_blacklist_term_is_redacted(self):
        tokens = [
            {"key": "PREDEITY", "value": f"1,{_TERM}"},
            {"key": "BONUS", "value": "SKILL|Perception|2"},
        ]
        scrubbed, any_redacted = ia.scrub_blacklist_pi_tokens(tokens, desc_already_redacted=False)
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], ia.REDACTED_PI_MARKER)
        self.assertEqual(scrubbed[1]["value"], "SKILL|Perception|2")  # untouched -- no PI

    def test_a_clean_record_is_left_alone(self):
        tokens = [{"key": "BONUS", "value": "SKILL|Stealth|4"}]
        scrubbed, any_redacted = ia.scrub_blacklist_pi_tokens(tokens, desc_already_redacted=False)
        self.assertFalse(any_redacted)
        self.assertEqual(scrubbed, tokens)

    def test_an_already_redacted_desc_token_is_not_double_processed(self):
        tokens = [{"key": "DESC", "value": ia.REDACTED_PI_MARKER}]
        scrubbed, any_redacted = ia.scrub_blacklist_pi_tokens(tokens, desc_already_redacted=True)
        self.assertFalse(any_redacted)
        self.assertEqual(scrubbed[0]["value"], ia.REDACTED_PI_MARKER)

    def test_a_desc_token_not_yet_redacted_is_still_screened(self):
        """desc_already_redacted=False means the DESC token, if present,
        still goes through the same scan as every other token -- a record
        whose DESC carries a blacklist term but was not flagged by the
        declared/description-specific screen must still be caught here."""
        tokens = [{"key": "DESC", "value": f"Blessed by {_TERM}."}]
        scrubbed, any_redacted = ia.scrub_blacklist_pi_tokens(tokens, desc_already_redacted=False)
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], ia.REDACTED_PI_MARKER)

    def test_never_mutates_the_input_list(self):
        tokens = [{"key": "PREDEITY", "value": f"1,{_TERM}"}]
        original = [dict(t) for t in tokens]
        ia.scrub_blacklist_pi_tokens(tokens, desc_already_redacted=False)
        self.assertEqual(tokens, original)


class RecordsEqualIgnoringTimestampTest(unittest.TestCase):
    """A generic re-run must not touch a file whose content did not
    actually change -- see `scrub_blacklist_pi_tokens`'s remediation and
    `records_equal_ignoring_timestamp`'s own docstring for why."""

    def test_identical_except_timestamp_is_equal(self):
        a = {"ingested_at": "2026-01-01T00:00:00Z", "data": {"key": "X"}}
        b = {"ingested_at": "2026-02-02T00:00:00Z", "data": {"key": "X"}}
        self.assertTrue(ia.records_equal_ignoring_timestamp(a, b))

    def test_a_real_content_difference_is_not_equal(self):
        a = {"ingested_at": "2026-01-01T00:00:00Z", "data": {"key": "X"}}
        b = {"ingested_at": "2026-01-01T00:00:00Z", "data": {"key": "Y"}}
        self.assertFalse(ia.records_equal_ignoring_timestamp(a, b))


if __name__ == "__main__":
    unittest.main()
