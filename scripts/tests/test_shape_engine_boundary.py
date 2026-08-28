#!/usr/bin/env python3
"""Tests for `scripts/shape_engine_boundary.py` (SD-34 Epic 1, AT-34-E1-004).

Proves the load-bearing claim: a shape engine turns a formula string into a
number and does not place/attach/display the record -- that gate is the
engine's four-condition promotion ladder in `src/bin/v06_work_inventory.rs`,
whose line-cited content is re-verified, not assumed, on every run.

Uses small synthetic inventory fixtures for the counting logic, same
precedent as `test_completion_atlas.py` / `test_missing_engine_tables.py`.
The citation check is exercised against the real, live source file (there is
only one `v06_work_inventory.rs` to cite), including a genuine RED->GREEN
mutation proof that the fail-closed path fires for the intended reason.
"""

import os
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_engine_boundary as SEB  # noqa: E402


def _unit(id_, magnitude_tokens, status):
    return {"id": id_, "magnitude_token_count": magnitude_tokens, "status": status}


class TestMagnitudeBearing(unittest.TestCase):
    def test_zero_token_units_excluded(self):
        units = [_unit("u1", 0, "engine-does-not-hold"), _unit("u2", 2, "grounded")]
        self.assertEqual([u["id"] for u in SEB.magnitude_bearing(units)], ["u2"])

    def test_missing_field_treated_as_zero(self):
        units = [{"id": "u1", "status": "grounded"}, _unit("u2", 1, "grounded")]
        self.assertEqual([u["id"] for u in SEB.magnitude_bearing(units)], ["u2"])

    def test_multiple_tokens_still_counted_once(self):
        units = [_unit("u1", 5, "grounded")]
        self.assertEqual(len(SEB.magnitude_bearing(units)), 1)


class TestNotHeldByEngine(unittest.TestCase):
    def test_only_engine_does_not_hold_status_counts(self):
        units = [
            _unit("u1", 1, "engine-does-not-hold"),
            _unit("u2", 1, "grounded"),
            _unit("u3", 1, "literal-verified"),
            _unit("u4", 1, "ingested-magnitude"),
        ]
        self.assertEqual([u["id"] for u in SEB.not_held_by_engine(units)], ["u1"])

    def test_scoped_to_the_magnitude_bearing_population_passed_in(self):
        # A zero-token engine-does-not-hold unit is not part of "the shape engine's
        # own feedstock" -- callers are expected to pass `magnitude_bearing()`
        # output in, not the raw unit list.
        mag = SEB.magnitude_bearing(
            [_unit("u1", 0, "engine-does-not-hold"), _unit("u2", 1, "engine-does-not-hold")]
        )
        self.assertEqual([u["id"] for u in SEB.not_held_by_engine(mag)], ["u2"])


class TestBuildReportOnLiveSource(unittest.TestCase):
    """The citation must resolve against the real, committed
    `src/bin/v06_work_inventory.rs` -- this is the whole point of the
    instrument, so it is not faked with a fixture."""

    def test_citation_resolves_at_head(self):
        units = [_unit("u1", 1, "engine-does-not-hold"), _unit("u2", 1, "grounded")]
        report = SEB.build_report(units)
        self.assertTrue(report["citation_ok"])
        self.assertEqual(report["promotion_ladder_anchor_line"], 10857)
        self.assertIn("has_real_description", report["promotion_ladder_source"])
        self.assertIn("class_feature_pool_catalog_holds", report["promotion_ladder_source"])

    def test_citation_failures_empty_at_head(self):
        self.assertEqual(SEB.citation_failures(), [])

    def test_live_counts_match_the_committed_fact(self):
        # The exact numbers `technical-design.md §3` / `decisions.md §2a`
        # state as fact, re-derived from the real committed inventory.
        units = SEB._load_units()
        mag = SEB.magnitude_bearing(units)
        self.assertEqual(len(mag), 26396)
        self.assertEqual(len(SEB.not_held_by_engine(mag)), 9475)


class TestCitationFailsClosedForTheIntendedReason(unittest.TestCase):
    """RED->GREEN: prove the fail-closed path fires because the cited
    line's CONTENT stopped matching -- not because of an unrelated error
    (`risks-and-open-questions.md §10`)."""

    def setUp(self):
        self._orig_lines = dict(SEB.PROMOTION_LADDER_LINES)

    def tearDown(self):
        SEB.PROMOTION_LADDER_LINES.clear()
        SEB.PROMOTION_LADDER_LINES.update(self._orig_lines)

    def test_wrong_expected_content_is_caught_not_silently_passed(self):
        # RED: assert a line 10857 must contain text it does not.
        SEB.PROMOTION_LADDER_LINES[10857] = "this text does not appear on that line"
        failures = SEB.citation_failures()
        self.assertEqual(len(failures), 1)
        self.assertIn("10857", failures[0])
        self.assertIn("this text does not appear on that line", failures[0])

        with self.assertRaises(SEB.StaleCitationError):
            SEB.build_report([_unit("u1", 1, "engine-does-not-hold")])

    def test_out_of_range_line_is_caught(self):
        SEB.PROMOTION_LADDER_LINES[99999999] = "unreachable"
        failures = SEB.citation_failures()
        self.assertTrue(any("out of range" in f for f in failures))

    def test_restored_lines_pass_again_GREEN(self):
        # GREEN: after tearDown-equivalent restoration mid-test, the real
        # content passes again -- proves the RED above was about content,
        # not a broken test harness.
        SEB.PROMOTION_LADDER_LINES[10857] = "this text does not appear on that line"
        self.assertNotEqual(SEB.citation_failures(), [])
        SEB.PROMOTION_LADDER_LINES.clear()
        SEB.PROMOTION_LADDER_LINES.update(self._orig_lines)
        self.assertEqual(SEB.citation_failures(), [])


class TestRenderMarkdownEmbedsReDeriveCommands(unittest.TestCase):
    def test_every_figure_carries_a_command(self):
        units = [_unit("u1", 1, "engine-does-not-hold"), _unit("u2", 1, "grounded")]
        report = SEB.build_report(units)
        md = SEB.render_markdown(report)
        self.assertIn("python3 scripts/shape_engine_boundary.py --check", md)
        self.assertIn("python3 -c", md)
        self.assertIn(str(report["magnitude_bearing"]), md)
        self.assertIn(str(report["not_held_by_engine"]), md)
        self.assertIn("10857", md)
        self.assertIn("denominator", md)


if __name__ == "__main__":
    unittest.main()
