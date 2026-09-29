#!/usr/bin/env python3
"""Tests for `scripts/completion_atlas.py` (SD-34 Epic 1, AT-34-E1-001).

Proves the load-bearing claim the atlas exists to make: every unit in
`docs/work-inventory.json` lands in **exactly one** of the ten buckets fixed
by `decisions.md §2`, with a real `unclassified` count computed from the
data (never assumed zero) and `overlap` computed rather than hardcoded.

Uses small synthetic inventory fixtures, not the live 49,450-unit corpus, so
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

    def test_sheet_complete_is_done(self):
        # SD-35 AT-35-E2-003: `sheet-complete` (decisions.md §1, the sheet
        # rule) is a DONE status -- a rendered sheet line is the terminal state.
        self.assertEqual(CA._bucket_of(_unit("u14", "sheet-complete", "sheet_rule_rendered:number")), "DONE")

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

    def test_engine_does_not_hold_splits_a_by_evidence(self):
        self.assertEqual(
            CA._bucket_of(_unit("u9", "engine-does-not-hold", "ability_content_has_no_engine_table")),
            "A",
        )

    def test_engine_does_not_hold_splits_b_by_evidence(self):
        self.assertEqual(
            CA._bucket_of(_unit("u10", "engine-does-not-hold", "class_feature_owner_matched_by_name_but_record_not_held_by_engine")),
            "B",
        )
        self.assertEqual(
            CA._bucket_of(_unit("u10b", "engine-does-not-hold", "race_trait_absent_from_race_traits")),
            "B",
        )
        self.assertEqual(
            CA._bucket_of(_unit("u10c", "engine-does-not-hold", "race_trait_race_not_modelled")),
            "B",
        )

    def test_engine_does_not_hold_splits_c_by_evidence(self):
        self.assertEqual(
            CA._bucket_of(_unit("u11", "engine-does-not-hold", "no_explanation_id_and_no_diagnostic_names_this_feature")),
            "C",
        )

    def test_engine_does_not_hold_falls_through_to_d(self):
        self.assertEqual(
            CA._bucket_of(_unit("u12", "engine-does-not-hold", "class_feature_of_unmodelled_corpus_class:warrior")),
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
            _unit("a1", "engine-does-not-hold", "ability_content_has_no_engine_table"),
            _unit("b1", "engine-does-not-hold", "not_held_by_engine"),
            _unit("c1", "engine-does-not-hold", "no_explanation_id_and_no_diagnostic_names_this_feature"),
            _unit("d1", "engine-does-not-hold", "class_feature_of_unmodelled_corpus_class:warrior"),
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

    def test_sheet_complete_evidence_must_name_a_rendered_form(self):
        # SD-35 AT-35-E2-003: a `sheet-complete` unit's evidence is
        # `sheet_rule_rendered:<form>` with `<form>` one of the three sheet
        # forms (decisions.md §1) -- anything else is the atlas trusting the
        # status word instead of what produced it.
        for form in ("number", "dice", "words"):
            self.assertTrue(
                CA._done_evidence_is_supported(f"sheet_rule_rendered:{form}", status="sheet-complete"), form
            )
        for bad in (
            "sheet_rule_rendered:",
            "sheet_rule_rendered:hidden",
            "companion_held_and_corpus_record_carries_real_description",
            "",
            None,
        ):
            self.assertFalse(CA._done_evidence_is_supported(bad, status="sheet-complete"), repr(bad))
        # The other DONE statuses keep their existing bar: a grounded unit
        # does not need the sheet-rule marker.
        self.assertTrue(
            CA._done_evidence_is_supported(
                "companion_held_and_corpus_record_carries_real_description", status="grounded"
            )
        )

    def test_done_evidence_violations_finds_sheet_complete_without_rendered_form(self):
        units = [
            _unit("s1", "sheet-complete", "sheet_rule_rendered:words"),
            _unit("s2", "sheet-complete", "class_probe_observed_computed_delta_on_the_rendered_snapshot"),  # planted
        ]
        self.assertEqual(CA._done_evidence_violations(units), ["s2"])


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


class TestEvidenceSourceIsHistorical(unittest.TestCase):
    """SD-36 D3: the generator (`src/bin/v06_work_inventory.rs`) is retired
    and `docs/work-inventory.json` is a permanently frozen snapshot, so the
    old content-anchor resolution machinery (`resolve_content_anchor`,
    `_citation_failures`, `resolved_citations`, condition 6) is gone -- there
    is no live source left to drift-check against. `evidence_source` stays
    as a prose-only historical record of which generator status/evidence
    string each bucket keyed on."""

    def test_every_bucket_has_an_evidence_source_and_no_citation_key(self):
        for b in CA.BUCKET_ORDER:
            definition = CA.BUCKET_DEFINITIONS[b]
            self.assertTrue(definition.get("evidence_source"), b)
            self.assertNotIn("citation", definition, b)

    def test_retired_resolution_machinery_is_gone(self):
        for name in ("resolve_content_anchor", "_citation_failures", "resolved_citations", "read_source_lines"):
            self.assertFalse(hasattr(CA, name), f"{name} should have been removed with the generator")


class TestByKindAndByEvidence(unittest.TestCase):
    def test_by_kind_partitions_each_kind_and_drops_nothing(self):
        units = [
            dict(_unit("u1", "grounded", "ok"), kind="feat"),
            dict(_unit("u2", "engine-does-not-hold", "has_no_engine_table"), kind="feat"),
            dict(_unit("u3", "not-started", "x"), kind="spell"),
            _unit("u4", "grounded", "ok"),  # no kind -> grouped under None
        ]
        rows = CA.by_kind(units)
        self.assertEqual(rows["feat"]["examined"], 2)
        self.assertEqual(rows["feat"]["counts"]["DONE"], 1)
        self.assertEqual(rows["feat"]["counts"]["A"], 1)
        self.assertEqual(rows["spell"]["counts"]["Z"], 1)
        self.assertEqual(rows[None]["examined"], 1)
        self.assertEqual(sum(r["examined"] for r in rows.values()), len(units))

    def test_by_kind_honours_book_filter(self):
        units = [
            dict(_unit("u1", "grounded", "ok", book="b1"), kind="feat"),
            dict(_unit("u2", "grounded", "ok", book="b2"), kind="feat"),
        ]
        self.assertEqual(CA.by_kind(units, book="b1")["feat"]["examined"], 1)

    def test_by_evidence_counts_each_evidence_string_per_bucket(self):
        units = [
            _unit("u1", "engine-does-not-hold", "has_no_engine_table"),
            _unit("u2", "engine-does-not-hold", "has_no_engine_table"),
            _unit("u3", "engine-does-not-hold", "absent_from_table"),
            _unit("u4", "unmeasurable", "why"),
        ]
        rows = CA.by_evidence(units)
        self.assertEqual(rows["A"]["has_no_engine_table"], 2)
        self.assertEqual(rows["B"]["absent_from_table"], 1)
        self.assertEqual(rows["U"]["why"], 1)
        self.assertEqual(set(CA.by_evidence(units, bucket="A")), {"A"})
        self.assertEqual(
            sum(sum(c.values()) for c in rows.values()), len(units)
        )

    def test_by_evidence_cli_rejects_unknown_bucket(self):
        self.assertEqual(CA.main(["--by-evidence", "--bucket", "Q"]), 2)


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
        # `AT-34-E2-004`: bucket A was 8,463 units before Epic 2 built the
        # eight tables; building them (`AT-34-E2-001`) and wiring the real
        # classification (`AT-34-E2-004`) reduces it to the two kinds Epic 2
        # deliberately does not close -- `power` (421, all `ultimate_psionics`,
        # costed for Epic 5) and the 28 `bestiary`-book `companion` units
        # whose REPORTED book itself carries no chassis registration at all
        # (as opposed to the `core_rulebook`/`ultimate_campaign` shape this
        # criterion fixed, where the reported book DOES have a table).
        # Re-derive: `python3 -c "import json; from collections import
        # Counter; inv=json.load(open('docs/work-inventory.json')); c=Counter();
        # [c.update([u['kind']]) for u in inv['units'] if 'has_no_engine_table'
        # in (u.get('evidence') or '')]; print(c)"` -> `{'power': 421,
        # 'companion': 28}`.
        # SD-35: bucket A is a population the bundle DRAINS (Epic 2's converter,
        # Epic 5's `power` chassis), so an equality here is a pin that must be
        # hand-edited every cycle -- the shape that went stale for six SD-34
        # waves. The gate is the ceiling plus the kinds, which still fails
        # CLOSED on a regression that puts units back into bucket A or on an
        # instrument that starts over-counting.
        # `449` is the SD-34 AT-34-E2-004 high-water mark. Live at SD-35
        # `4e321d2c6c`: 1. Re-derive: `python3 scripts/completion_atlas.py
        # --check` -> the `A:` line.
        inv = CA._load_inventory()
        result = CA.partition(inv["units"])
        self.assertLessEqual(result["counts"].get("A", 0), 449)

    def test_bucket_u_matches_named_population(self):
        # Formerly pinned to a hand-derived literal (202, `AT-34-E3-003`,
        # `decisions.md §17`/§bucket-U-cycle-2 history). `docs/work-inventory.json`
        # is now FROZEN (SD-36 Epic B, operator ruling D3 --
        # `docs/work-inventory.FROZEN.md`) at a snapshot whose `status`
        # Counter carries zero `unmeasurable` units -- the 202 figure was
        # measured against a since-superseded corpus state and does not
        # reproduce against the frozen one (independently confirmed:
        # `python3 -c "import json; from collections import Counter;
        # inv=json.load(open('docs/work-inventory.json'));
        # print(Counter(u['status'] for u in inv['units'])['unmeasurable'])"`
        # -> `0`).
        #
        # Per operator ruling (SD-36 Epic B fix cycle, 2026-09-17): assert
        # bucket U against a second, independent implementation of the same
        # count rather than a number pinned to one corpus snapshot, so the
        # test keeps proving `_bucket_of`'s `unmeasurable -> "U"` mapping
        # (the load-bearing claim) without re-encoding a value that only
        # ever held for one frozen state of the data it reads.
        inv = CA._load_inventory()
        result = CA.partition(inv["units"])
        independent_u_count = sum(
            1 for u in inv["units"] if u.get("status") == "unmeasurable"
        )
        self.assertEqual(result["counts"].get("U", 0), independent_u_count)


if __name__ == "__main__":
    unittest.main()
