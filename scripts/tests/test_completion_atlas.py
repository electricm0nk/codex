#!/usr/bin/env python3
"""Tests for `scripts/completion_atlas.py` (SD-34 Epic 1, AT-34-E1-001).

Proves the load-bearing claim the atlas exists to make: every unit in
`docs/work-inventory.json` lands in **exactly one** of the ten buckets fixed
by `decisions.md §2`, with a real `unclassified` count computed from the
data (never assumed zero) and `overlap` computed rather than hardcoded.

Uses small synthetic inventory fixtures, not the live 49,438-unit corpus, so
these tests stay fast and are not subject to corpus drift across cycles
(`test_box_ledger.py` sets the same precedent in this repo). The live
corpus is exercised separately, as acceptance evidence, by running the
committed CLI against the committed `docs/work-inventory.json` -- not
inside this fast unit-test file.
"""

import os
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import completion_atlas as CA  # noqa: E402


def _unit(id_, status, evidence=None, book="test_book"):
    return {"id": id_, "status": status, "evidence": evidence, "book": book}


class TestBucketOf(unittest.TestCase):
    def test_done_statuses(self):
        self.assertEqual(CA._bucket_of(_unit("u1", "grounded")), "DONE")
        self.assertEqual(CA._bucket_of(_unit("u2", "text-complete")), "DONE")

    def test_verified_statuses(self):
        self.assertEqual(CA._bucket_of(_unit("u3", "literal-verified")), "V")
        self.assertEqual(CA._bucket_of(_unit("u4", "fixture-verified")), "V")

    def test_ingested_magnitude_is_m(self):
        self.assertEqual(CA._bucket_of(_unit("u5", "ingested-magnitude")), "M")

    def test_unmeasurable_is_u(self):
        self.assertEqual(CA._bucket_of(_unit("u6", "unmeasurable", "text_only_but_corpus_record_carries_no_description_to_show_a_player")), "U")

    def test_deferred_is_x(self):
        self.assertEqual(CA._bucket_of(_unit("u7", "deferred-with-reason")), "X")

    def test_not_started_is_z(self):
        self.assertEqual(CA._bucket_of(_unit("u8", "not-started")), "Z")

    def test_not_ingested_splits_a_by_evidence(self):
        self.assertEqual(
            CA._bucket_of(_unit("u9", "not-ingested", "ability_content_has_no_engine_table")),
            "A",
        )

    def test_not_ingested_splits_b_by_evidence(self):
        self.assertEqual(
            CA._bucket_of(_unit("u10", "not-ingested", "class_feature_owner_matched_by_name_but_record_not_held_by_engine")),
            "B",
        )
        self.assertEqual(
            CA._bucket_of(_unit("u10b", "not-ingested", "race_trait_absent_from_race_traits")),
            "B",
        )
        self.assertEqual(
            CA._bucket_of(_unit("u10c", "not-ingested", "race_trait_race_not_modelled")),
            "B",
        )

    def test_not_ingested_splits_c_by_evidence(self):
        self.assertEqual(
            CA._bucket_of(_unit("u11", "not-ingested", "no_explanation_id_and_no_diagnostic_names_this_feature")),
            "C",
        )

    def test_not_ingested_falls_through_to_d(self):
        self.assertEqual(
            CA._bucket_of(_unit("u12", "not-ingested", "class_feature_of_unmodelled_corpus_class:warrior")),
            "D",
        )

    def test_unknown_status_is_unclassified(self):
        # RED case: this is the intended-failure shape for the fail-closed
        # check in cmd_check -- a status the atlas has never seen must
        # come back None, never silently guessed into a bucket.
        self.assertIsNone(CA._bucket_of(_unit("u13", "some-future-status-nobody-named-yet")))


class TestPartition(unittest.TestCase):
    def test_all_ten_buckets_reachable_and_sum_to_population(self):
        units = [
            _unit("g1", "grounded"),
            _unit("g2", "text-complete"),
            _unit("v1", "literal-verified"),
            _unit("v2", "fixture-verified"),
            _unit("m1", "ingested-magnitude"),
            _unit("u1", "unmeasurable", "text_only_but_corpus_record_carries_no_description_to_show_a_player"),
            _unit("x1", "deferred-with-reason"),
            _unit("z1", "not-started"),
            _unit("a1", "not-ingested", "ability_content_has_no_engine_table"),
            _unit("b1", "not-ingested", "not_held_by_engine"),
            _unit("c1", "not-ingested", "no_explanation_id_and_no_diagnostic_names_this_feature"),
            _unit("d1", "not-ingested", "class_feature_of_unmodelled_corpus_class:warrior"),
        ]
        result = CA.partition(units)
        self.assertEqual(result["examined"], len(units))
        self.assertEqual(result["unclassified_ids"], [])
        self.assertEqual(result["overlap_ids"], [])
        total_bucketed = sum(result["counts"].values())
        self.assertEqual(total_bucketed, len(units))
        self.assertEqual(set(result["counts"].keys()), set(CA.BUCKET_ORDER))

    def test_unclassified_is_real_not_assumed(self):
        # RED->GREEN proof: a unit with a status the atlas has no rule for
        # must show up in unclassified, not silently vanish or get counted
        # as DONE by default.
        units = [_unit("mystery", "totally-unknown-status")]
        result = CA.partition(units)
        self.assertEqual(result["unclassified_ids"], ["mystery"])
        self.assertEqual(sum(result["counts"].values()), 0)

    def test_overlap_detected_on_duplicate_ids(self):
        units = [
            _unit("dup", "grounded"),
            _unit("dup", "grounded"),
        ]
        result = CA.partition(units)
        self.assertEqual(result["overlap_ids"], ["dup"])

    def test_book_filter_scopes_examined_population(self):
        units = [
            _unit("a1", "grounded", book="book_a"),
            _unit("a2", "grounded", book="book_a"),
            _unit("b1", "grounded", book="book_b"),
        ]
        result = CA.partition(units, book="book_a")
        self.assertEqual(result["examined"], 2)


class TestLiveInventoryCheck(unittest.TestCase):
    """Runs the CLI against the real committed inventory -- the acceptance
    evidence AT-34-E1-001 actually names."""

    def test_live_check_reports_zero_unclassified_and_zero_overlap(self):
        inv = CA._load_inventory()
        result = CA.partition(inv["units"])
        self.assertEqual(result["unclassified_ids"], [])
        self.assertEqual(result["overlap_ids"], [])
        self.assertEqual(result["examined"], inv["totals"]["units"])

    def test_bucket_a_matches_named_population(self):
        # decisions.md / epic-breakdown.md: bucket A is 8,463 units.
        inv = CA._load_inventory()
        result = CA.partition(inv["units"])
        self.assertEqual(result["counts"].get("A", 0), 8463)

    def test_bucket_u_matches_named_population(self):
        inv = CA._load_inventory()
        result = CA.partition(inv["units"])
        self.assertEqual(result["counts"].get("U", 0), 321)


if __name__ == "__main__":
    unittest.main()
