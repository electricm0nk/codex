#!/usr/bin/env python3
"""Regression tests for the `ingest_generic_kind.py --remediate` structural
gap-close (SD-32 card 11, `t9-onboarding` follow-up cycle, 2026-08-23).

**The defect this closes.** `ingest_generic_kind.py`'s ordinary writer is
gated on `join_status == "no_record"` against a shape-ledger snapshot: once
a unit is ingested it is no longer `no_record`, so the ordinary writer can
never re-touch a record it already shipped -- even when the record carries a
Product Identity leak the CURRENT scrub logic would now catch. This was
named live by the `pi-key-rawtokens-followup` cycle's own receipt, which
found and confirmed 9 real leaks (`feat_generic` x7 `adventurers_guide`,
`monster_generic` x2 `inner_sea_bestiary`) it could not fix for exactly this
reason.

`--remediate` (added by this cycle) re-derives every already-shipped,
SELF-OWNED (`codex_generated_name` key present -- see
`ingest_generic_kind.py::find_owned_generic_files`) `<kind>_generic` record
from its own pinned-oracle citation and re-applies the current redaction
pipeline in place.

**This test file does two things:**
1. Confirms the 9 named leaks, plus the `race_trait_generic` leak the same
   remediation run also reached, no longer carry a blacklist hit on disk
   (a corpus-content regression test, the same shape
   `test_pi_key_rawtokens_defect1_regen.py` already uses).
2. **Mutation-proves** the regression test itself: `RemediationMutationProofTest`
   reintroduces a leak into an in-memory copy of a remediated record (never
   the shipped file) and asserts the SAME assertion helper goes RED for it,
   then confirms the real on-disk record is unaffected -- proving the check
   can fail, not just pass.

Never types a real blacklist term literally; uses `normalized_term_hit`
(the same scan `decisions.md §19a` mandates) as the oracle for "clean".
"""
from __future__ import annotations

import copy
import json
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from sd32_t9_pi_review_feat_equipment import normalized_term_hit  # noqa: E402
from pi_scrub import blacklist_term_hit_including_concatenated  # noqa: E402

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REDACTED = "[redacted PI]"

# The 9 records confirmed as real leaks by the `pi-key-rawtokens-followup`
# cycle's own receipt, plus the one `race_trait_generic` leak this cycle's
# `declared_pi_shipping_audit` re-derivation found in the same shape
# (already-shipped, self-owned, DESC declared PI but raw_tokens' own DESC
# copy not yet redacted by the code version that originally wrote it).
REMEDIATED_RECORD_PATHS = [
    "data/corpus/adventurers_guide/feat_generic/duelist_of_the_roaring_falls.json",
    "data/corpus/adventurers_guide/feat_generic/duelist_of_the_shrouded_lake.json",
    "data/corpus/adventurers_guide/feat_generic/extra_spontaneous_spell_mastery.json",
    "data/corpus/adventurers_guide/feat_generic/falling_water_gambit.json",
    "data/corpus/adventurers_guide/feat_generic/nameless_one.json",
    "data/corpus/adventurers_guide/feat_generic/redistributed_might.json",
    "data/corpus/adventurers_guide/feat_generic/sirian_s_masterstroke.json",
    "data/corpus/inner_sea_bestiary/monster_generic/chemnosit.json",
    "data/corpus/inner_sea_bestiary/monster_generic/volnagur.json",
    "data/corpus/inner_sea_races/race_trait_generic/"
    "codex_named_unit_race_trait_inner_sea_races_isr_abilities_race_lst_67.json",
]


def _load(rel_path: str) -> dict:
    with open(os.path.join(REPO_ROOT, rel_path), encoding="utf-8") as fh:
        return json.load(fh)


def _assert_record_carries_no_blacklist_hit(test: unittest.TestCase, record: dict) -> None:
    """Every raw_tokens VALUE and the top-level description/name/key, if not
    already the standing redaction marker, must be clean under the SAME
    scan `pi_key_rawtokens_audit.py`/`scrub_name_pi_tokens` use -- including
    the alphanumeric-normalized concatenated-term check
    (`blacklist_term_hit_including_concatenated`), not only the word-bounded
    one, so this assertion is at least as strict as the production scrub."""
    data = record["data"]
    for field in ("name", "key", "description"):
        value = data.get(field)
        if value and value != REDACTED:
            hit = normalized_term_hit(value) or blacklist_term_hit_including_concatenated(value)
            test.assertIsNone(hit, f"field {field!r} carries an unredacted blacklist hit: {value!r}")
    for tok in data.get("raw_tokens", []):
        value = tok.get("value")
        if value and value != REDACTED:
            hit = normalized_term_hit(value) or blacklist_term_hit_including_concatenated(value)
            test.assertIsNone(
                hit, f"raw_tokens[{tok.get('key')!r}] carries an unredacted blacklist hit: {value!r}"
            )


class RemediatedRecordsNoLongerLeakTest(unittest.TestCase):
    """The 10 records this cycle's `--remediate` run rewrote in place. Before
    this cycle, `duelist_of_the_roaring_falls.json` shipped `PREABILITY` and
    `BENEFIT` tokens carrying the real, unredacted prerequisite/benefit text
    (git history: this cycle's own commit is the fix) -- confirmed live via
    `git show HEAD~1:<path>` during authoring, not assumed."""

    def test_all_ten_remediated_records_carry_no_blacklist_hit(self):
        for rel_path in REMEDIATED_RECORD_PATHS:
            with self.subTest(rel_path=rel_path):
                record = _load(rel_path)
                _assert_record_carries_no_blacklist_hit(self, record)

    def test_declared_pi_records_have_matching_raw_tokens_desc(self):
        """The `DESC-PI-SHIPPED-IN-RAW-TOKENS` shape this cycle also closed
        (`cargo run --bin declared_pi_shipping_audit`): a record whose
        `pi_field` includes `description` must carry the redaction marker in
        its `raw_tokens`' own `DESC` entries too, not only `data.description`."""
        for rel_path in REMEDIATED_RECORD_PATHS:
            record = _load(rel_path)
            pi_field = record.get("pi_field") or ""
            if "description" not in pi_field.split(","):
                continue
            with self.subTest(rel_path=rel_path):
                desc_tokens = [t["value"] for t in record["data"]["raw_tokens"] if t["key"] == "DESC"]
                self.assertTrue(desc_tokens, f"{rel_path}: expected at least one DESC raw_token")
                self.assertTrue(
                    all(v == REDACTED for v in desc_tokens),
                    f"{rel_path}: raw_tokens DESC not fully redacted: {desc_tokens!r}",
                )


class RemediationMutationProofTest(unittest.TestCase):
    """Mutation-proves `_assert_record_carries_no_blacklist_hit` (and, by
    construction, `RemediatedRecordsNoLongerLeakTest` above) actually can
    fail: reintroduce a leak into an IN-MEMORY copy of a real remediated
    record, assert the check goes RED, then confirm the on-disk record
    itself is untouched by this test."""

    def test_check_goes_red_when_a_leak_is_reintroduced(self):
        rel_path = "data/corpus/adventurers_guide/feat_generic/duelist_of_the_roaring_falls.json"
        original = _load(rel_path)
        # Sanity: the real, remediated, on-disk record is clean first.
        _assert_record_carries_no_blacklist_hit(self, original)

        mutated = copy.deepcopy(original)
        for tok in mutated["data"]["raw_tokens"]:
            if tok["key"] == "PREABILITY":
                # Reintroduce the leak shape this cycle fixed: the
                # redaction marker replaced by real, blacklisted
                # prerequisite text (built from an already-blacklisted term
                # via string concatenation here so no literal PI term is
                # ever written into this test file's own source -- and kept
                # apart from any OTHER prefix text, since concatenating
                # unrelated fragments can accidentally spell a SECOND,
                # different blacklist term across the join, the same
                # collision-class risk `ogl-pi-blacklist.md §2.3a`'s
                # word-boundary rule exists to guard against elsewhere).
                tok["value"] = "Al" + "dori Dueling Disciple"

        with self.assertRaises(AssertionError):
            _assert_record_carries_no_blacklist_hit(self, mutated)

        # The real on-disk record was never touched by this test.
        reloaded = _load(rel_path)
        self.assertEqual(reloaded, original)


if __name__ == "__main__":
    unittest.main()
