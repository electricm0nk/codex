#!/usr/bin/env python3
"""Regression tests for the `pi-key-rawtokens-screen` follow-up cycle
(SD-32 card 11, `t9-onboarding` actor, 2026-08-23) -- confirms the four
originally-reported leaks and their generators' cause-level fix.

**Scope**: this is a corpus-content regression test, not a unit test of a
function -- it reads the ACTUAL shipped `data/corpus/**` records the
`domain`/`language`/`equipment` generators wrote after this cycle's fix and
asserts they carry no confirmed hit against the SIGNED-OFF 60(+1)-term
blacklist. The RED state this GREEN result replaces is documented, with the
exact commands and diffs, in this cycle's own cycle receipt -- re-running the
generators against a checkout that predates this cycle's commit reproduces
it directly (`git log --oneline -- data/corpus/core_rulebook/domain/death.json`
shows the leak-fixing commit).

Never types a real blacklist term literally; uses `normalized_term_hit`
(the same scan `decisions.md §19a` mandates) as the oracle for "clean".
"""
from __future__ import annotations

import json
import os
import re
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from sd32_t9_pi_review_feat_equipment import normalized_term_hit  # noqa: E402

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REDACTED = "[redacted PI]"


def _load(rel_path: str) -> dict:
    with open(os.path.join(REPO_ROOT, rel_path), encoding="utf-8") as fh:
        return json.load(fh)


def _assert_record_carries_no_blacklist_hit(test: unittest.TestCase, record: dict) -> None:
    """Every raw_tokens VALUE and the top-level description/name/key, if not
    already the standing redaction marker, must be clean under the same
    scan `pi_key_rawtokens_audit.py`/`ingest_ability.py::scrub_blacklist_pi_tokens`
    use. Mirrors `pi_key_rawtokens_audit.py::screen_record`'s own check."""
    data = record["data"]
    for field in ("name", "key", "description"):
        value = data.get(field)
        if value and value != REDACTED:
            hit = normalized_term_hit(value)
            test.assertIsNone(hit, f"field {field!r} carries an unredacted blacklist hit: {value!r}")
    for tok in data.get("raw_tokens", []):
        value = tok.get("value")
        if value and value != REDACTED:
            hit = normalized_term_hit(value)
            test.assertIsNone(
                hit, f"raw_tokens[{tok.get('key')!r}] carries an unredacted blacklist hit: {value!r}"
            )


class TheFourOriginallyReportedRecordsTest(unittest.TestCase):
    """`decisions.md §19`'s 60-term list, confirmed corpus-wide by the
    `pi-key-rawtokens-screen` cycle's own report
    (`artifacts/gate-3-closure-invariant/pi-key-rawtokens-corpus-report.md`
    §2). Three were real leaks in a `PREDEITY:`/`DESC:`-style token the
    generator's description-only screen never reached; this follow-up
    cycle's fix (unconditional `scrub_blacklist_pi_tokens` for `domain`/
    `language`, and the `pi_screening.rs::PI_BLACKLIST_TERMS` case-typo
    addition for `equipment`) closes all three. The fourth (`spell`) is
    re-derived below as a FALSE POSITIVE of the audit tool's own OCR-fold
    normalization, not a real leak -- see
    `SpellFalsePositiveTest` below."""

    def test_domain_death_no_longer_leaks(self):
        record = _load("data/corpus/core_rulebook/domain/death.json")
        _assert_record_carries_no_blacklist_hit(self, record)
        # The specific tokens that leaked are now the standing marker, not
        # silently dropped -- `§3` fixture discipline: redaction is visible.
        spelllevel_values = [t["value"] for t in record["data"]["raw_tokens"] if t["key"] == "SPELLLEVEL"]
        self.assertTrue(spelllevel_values, "the record must still carry its SPELLLEVEL tokens")
        self.assertTrue(all(v == REDACTED for v in spelllevel_values))
        self.assertEqual(record["license"], "PI-REDACTED")

    def test_language_nightsong_no_longer_leaks(self):
        record = _load("data/corpus/inner_sea_temples/language/nightsong.json")
        _assert_record_carries_no_blacklist_hit(self, record)
        predeity_values = [t["value"] for t in record["data"]["raw_tokens"] if t["key"] == "PREDEITY"]
        self.assertTrue(predeity_values, "the record must still carry its PREDEITY token")
        self.assertTrue(all(v == REDACTED for v in predeity_values))
        self.assertEqual(record["license"], "PI-REDACTED")

    def test_equipment_wayfinder_of_zephyrs_no_longer_leaks(self):
        record = _load("data/corpus/inner_sea_gods/equipment/wayfinder_of_zephyrs.json")
        _assert_record_carries_no_blacklist_hit(self, record)
        self.assertEqual(record["data"]["description"], REDACTED)
        desc_tokens = [t["value"] for t in record["data"]["raw_tokens"] if t["key"] == "DESC"]
        self.assertTrue(desc_tokens)
        self.assertTrue(all(v == REDACTED for v in desc_tokens))
        self.assertEqual(record["license"], "PI-REDACTED")


class SpellFalsePositiveTest(unittest.TestCase):
    """`§17a` self-correction, then `decisions.md §26`'s fix: the fourth
    originally-reported record (`advanced_players_guide/spell/bard_s_escape.json`)
    was NEVER a real leak. `normalized_term_hit` used to flag it only
    because its OCR-confusion fold (`rn` -> `m`) canonicalized the
    blacklist term "Jarn" to "jam" -- the same string as an ordinary
    English word that appears in this record's genuine, OGL prose ("...out
    of a tight jam..."). `decisions.md §26` fixed this at the cause
    (`pi_scrub.py`'s `_RN_FOLD_EXEMPT_TERMS_CASEFOLD`): the scan no longer
    flags this record at all. `test_pi_key_rawtokens_defect1_regen.py` no
    longer needs to document a live false positive as tolerated -- it
    proves the fix directly."""

    def test_the_record_no_longer_false_positives_under_the_fixed_scan(self):
        record = _load("data/corpus/advanced_players_guide/spell/bard_s_escape.json")
        _assert_record_carries_no_blacklist_hit(self, record)
        # This record was never touched by any cycle's fix and correctly
        # remains OGL -- the fix was to the SCAN, never to the corpus.
        self.assertEqual(record["license"], "OGL")

    def test_reproduces_the_false_positive_against_an_unfixed_matcher(self):
        """Confirms the defect this test class documents was real, without
        depending on the corpus record's exact bytes staying identical
        forever: an unfixed (no rn-fold exemption) re-implementation of the
        same canonicalize+word-boundary scan still flags the record's real
        description text, exactly as the pre-`§26` shared function did."""
        record = _load("data/corpus/advanced_players_guide/spell/bard_s_escape.json")
        description = record["data"]["description"]
        unfixed_canon_text = description.casefold().replace("rn", "m")
        unfixed_canon_jarn = "jarn".replace("rn", "m")  # "jam", no exemption
        self.assertRegex(
            unfixed_canon_text,
            r"(?<![a-z0-9])" + re.escape(unfixed_canon_jarn) + r"(?![a-z0-9])",
            "sanity: an unfixed matcher must still reproduce the false positive "
            "this test class exists to guard against",
        )
        # And the flagged term is not actually present anywhere in the
        # record's real bytes -- confirming the (reproduced) hit was always
        # an artifact of the fold, never a real Product Identity leak.
        self.assertNotIn("jarn", json.dumps(record).lower())


if __name__ == "__main__":
    unittest.main()
