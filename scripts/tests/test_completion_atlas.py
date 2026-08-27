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


class TestDoneEvidenceViolations(unittest.TestCase):
    """AT-34-E1-002 condition 3: a DONE unit whose evidence does not support it."""

    def test_supported_done_evidence_passes(self):
        self.assertTrue(CA._done_evidence_is_supported("companion_held_and_corpus_record_carries_real_description"))

    def test_empty_evidence_is_unsupported(self):
        self.assertFalse(CA._done_evidence_is_supported(""))
        self.assertFalse(CA._done_evidence_is_supported(None))

    def test_unfinished_bucket_marker_in_done_evidence_is_a_violation(self):
        # RED->GREEN proof shape: a DONE unit carrying an A-bucket marker is
        # exactly the mistake condition 3 exists to catch.
        self.assertFalse(CA._done_evidence_is_supported("has_no_engine_table"))
        self.assertFalse(CA._done_evidence_is_supported("class_feature_option_pool_record_not_held_by_engine"))

    def test_explanation_id_alone_is_not_flagged(self):
        # Confirmed against the live corpus: 245 real DONE units legitimately
        # carry `explanation_id` in their evidence string. Flagging it would
        # be condition 6's own mistake shape (a substring read as meaning
        # something it does not) turned inward on condition 3.
        self.assertTrue(
            CA._done_evidence_is_supported("explanation_id_observed_and_corpus_record_carries_real_description")
        )

    def test_done_evidence_violations_finds_mismatched_unit(self):
        units = [
            _unit("g1", "grounded", "companion_held_and_corpus_record_carries_real_description"),
            _unit("g2", "grounded", "has_no_engine_table"),  # planted violation
        ]
        self.assertEqual(CA._done_evidence_violations(units), ["g2"])


class TestMissingClearingMechanisms(unittest.TestCase):
    """AT-34-E1-002 condition 4: a bucket with no named clearing mechanism."""

    def test_real_definitions_all_have_clears(self):
        self.assertEqual(CA._missing_clearing_mechanisms(), [])

    def test_missing_clears_detected(self):
        mutated = {b: dict(v) for b, v in CA.BUCKET_DEFINITIONS.items()}
        mutated["A"]["clears"] = ""
        self.assertEqual(CA._missing_clearing_mechanisms(mutated), ["A"])

    def test_absent_clears_key_detected(self):
        mutated = {b: dict(v) for b, v in CA.BUCKET_DEFINITIONS.items()}
        del mutated["Z"]["clears"]
        self.assertEqual(CA._missing_clearing_mechanisms(mutated), ["Z"])


class TestCitationFailures(unittest.TestCase):
    """AT-34-E1-002 condition 6: the file:line citation must resolve AND its
    content must actually contain the claimed marker."""

    def test_real_citations_all_resolve_and_match(self):
        # This is the live acceptance evidence for condition 6: every
        # bucket's citation is checked against the real, current
        # src/bin/v06_work_inventory.rs on disk -- not assumed.
        self.assertEqual(CA._citation_failures(), [])

    def test_missing_citation_detected(self):
        mutated = {b: dict(v) for b, v in CA.BUCKET_DEFINITIONS.items()}
        mutated["A"].pop("citation", None)
        failures = CA._citation_failures(mutated)
        self.assertEqual(len(failures), 1)
        self.assertIn("A", failures[0])

    def test_wrong_line_number_detected(self):
        mutated = {b: dict(v) for b, v in CA.BUCKET_DEFINITIONS.items()}
        mutated["A"]["citation"] = dict(mutated["A"]["citation"])
        mutated["A"]["citation"]["line"] = 10**9  # out of range
        failures = CA._citation_failures(mutated)
        self.assertEqual(len(failures), 1)
        self.assertIn("does not resolve", failures[0])

    def test_content_mismatch_detected_even_when_line_resolves(self):
        # The line resolves (it exists) but no longer contains the claimed
        # marker -- proves this asserts on CONTENT, not just path/line
        # (risks-and-open-questions.md §10).
        mutated = {b: dict(v) for b, v in CA.BUCKET_DEFINITIONS.items()}
        mutated["A"]["citation"] = dict(mutated["A"]["citation"])
        mutated["A"]["citation"]["must_contain"] = "this_marker_definitely_does_not_appear_on_that_line"
        failures = CA._citation_failures(mutated)
        self.assertEqual(len(failures), 1)
        self.assertIn("no longer contains", failures[0])

    def test_nonexistent_file_detected(self):
        mutated = {b: dict(v) for b, v in CA.BUCKET_DEFINITIONS.items()}
        mutated["A"]["citation"] = {"file": "src/bin/does_not_exist_98765.rs", "line": 1, "must_contain": "x"}
        failures = CA._citation_failures(mutated)
        self.assertEqual(len(failures), 1)
        self.assertIn("does not resolve", failures[0])


class TestStalenessGate(unittest.TestCase):
    """AT-34-E1-002 condition 5: a `derived_at` SHA that is not an ancestor
    of HEAD."""

    def test_head_is_its_own_ancestor(self):
        head = CA._head_sha()
        self.assertTrue(CA._is_ancestor(head))

    def test_ancestor_commit_is_an_ancestor(self):
        import subprocess
        parent = subprocess.run(
            ["git", "rev-parse", "HEAD~1"], cwd=CA.REPO_ROOT,
            capture_output=True, text=True, check=True,
        ).stdout.strip()
        self.assertTrue(CA._is_ancestor(parent))

    def test_bogus_sha_is_not_an_ancestor(self):
        self.assertFalse(CA._is_ancestor("0000000000000000000000000000000000dead"))
        self.assertFalse(CA._is_ancestor("unknown"))
        self.assertFalse(CA._is_ancestor(None))
        self.assertFalse(CA._is_ancestor(""))

    def test_staleness_violation_none_when_no_artifact(self):
        self.assertIsNone(CA._staleness_violation("/tmp/does-not-exist-completion-atlas.json"))

    def test_staleness_violation_flags_bogus_prior_sha(self):
        import json as _json
        import tempfile
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
            _json.dump({"derived_at": "0000000000000000000000000000000000dead"}, fh)
            path = fh.name
        try:
            result = CA._staleness_violation(path)
            self.assertIsNotNone(result)
            self.assertIn("not an ancestor", result)
        finally:
            os.remove(path)

    def test_staleness_violation_clear_for_real_ancestor(self):
        import json as _json
        import tempfile
        head = CA._head_sha()
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
            _json.dump({"derived_at": head}, fh)
            path = fh.name
        try:
            self.assertIsNone(CA._staleness_violation(path))
        finally:
            os.remove(path)

    def test_committed_artifact_is_not_stale(self):
        # Live acceptance evidence: the artifact as currently committed on
        # disk (before this run's own --check overwrites it) must still be
        # an ancestor of HEAD.
        self.assertIsNone(CA._staleness_violation())


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
