#!/usr/bin/env python3
"""Tests for `scripts/shape_engine_boundary.py` (SD-34 Epic 1, AT-34-E1-004;
content anchors by SD-35 AT-35-E1-002).

Proves the load-bearing claim: a shape engine turns a formula string into a
number and does not place/attach/display the record -- that gate is the
engine's four-condition promotion ladder in `src/bin/v06_work_inventory.rs`,
whose content is re-verified by search, not assumed, on every run.

Uses small synthetic inventory fixtures for the counting logic, same
precedent as `test_completion_atlas.py` / `test_missing_engine_tables.py`.
The citation check is exercised against the real, live source file (there is
only one `v06_work_inventory.rs` to cite) AND against synthetic source text
for the two RED->GREEN proofs AT-35-E1-002 names: moving the cited function
50 lines keeps the anchor green; changing one cited condition fails it.
"""

import os
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_engine_boundary as SEB  # noqa: E402


def _unit(id_, magnitude_tokens, status):
    return {"id": id_, "magnitude_token_count": magnitude_tokens, "status": status}


# A miniature `classify` carrying the real ladder text, plus the sibling
# `if has_real_description` block that shares its first three lines -- the
# shape the live file has, so uniqueness is exercised, not assumed.
_LADDER = SEB.PROMOTION_LADDER_ANCHOR["anchor"]
_SYNTHETIC_SOURCE = (
    ["fn simple_kind_verdict() {", "    let x = 1;", "}", "",
     "fn classify(unit: &Unit) -> Verdict {",
     "    if something {",
     "        if has_real_description",
     "            && is_display_wiring_class_for_promotion(wc_class)",
     "            && !universal_sheet_modifier",
     "            && facts.explanation_ids.contains(&grounding_explanation_id)",
     "        {",
     "            return early();",
     "        }",
     "    }",
     "    // the real ladder",
     "    if " + _LADDER[0][3:]]  # `if has_real_description`
    + ["        " + line for line in _LADDER[1:]]
    + ["    {", "        return promoted();", "    }", "    fallthrough()", "}", "",
       "fn classify_class_feature_delta() {", "    unrelated()", "}"]
)


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
    """The anchor must resolve against the real, committed
    `src/bin/v06_work_inventory.rs` -- this is the whole point of the
    instrument, so it is not faked with a fixture."""

    def test_citation_resolves_at_head(self):
        units = [_unit("u1", 1, "engine-does-not-hold"), _unit("u2", 1, "grounded")]
        report = SEB.build_report(units)
        self.assertTrue(report["citation_ok"])
        # The line is DERIVED by search at check time (AT-35-E1-002) -- never
        # pinned here, which is exactly what let SD-34 waves 44-51 drift it.
        # Assert it is real and that it points at the ladder's last line.
        resolved = SEB.resolve_promotion_ladder()
        self.assertTrue(resolved["ok"], resolved)
        self.assertEqual(report["promotion_ladder_anchor_line"], resolved["end_line"])
        self.assertGreater(report["promotion_ladder_anchor_line"], 0)
        self.assertEqual(report["promotion_ladder_context_fn"], "classify")
        self.assertIn("has_real_description", report["promotion_ladder_source"])
        self.assertIn("class_feature_pool_catalog_holds", report["promotion_ladder_source"])
        self.assertEqual(
            [line.strip() for line in report["promotion_ladder_source"].strip().split("\n")],
            SEB.PROMOTION_LADDER_ANCHOR["anchor"],
        )

    def test_citation_failures_empty_at_head(self):
        self.assertEqual(SEB.citation_failures(), [])

    def test_live_counts_match_the_committed_fact(self):
        # The exact numbers `technical-design.md §3` / `decisions.md §2a`
        # state as fact, re-derived from the real committed inventory.
        units = SEB._load_units()
        mag = SEB.magnitude_bearing(units)
        self.assertEqual(len(mag), 26396)
        # SD-34 wave 51: `9475` was stale, and had been carried as a KNOWN,
        # deliberately-deferred open item since wave 44 (`progress.md`, wave 45
        # entry: "left `shape_engine_boundary.py`'s own pre-existing, unrelated
        # population-count drift (now 9475 pinned vs 8996 live) named, not
        # fixed"). Closed here rather than carried a seventh wave: a red pin in
        # this file is not harmless, because it keeps the WHOLE instrument's
        # test red, which is exactly how its promotion-ladder citation went two
        # further waves (49, 50) without anyone noticing it had gone stale too.
        # Re-derived live, and confirmed NOT moved by this wave's own work:
        # `not_held_by_engine` is 8784 in BOTH this wave's before and after
        # `docs/work-inventory.json` snapshots (this wave closes 102
        # `engine-does-not-hold` units, but every one carries
        # `magnitude_token_count == 0`, so none of them is in the
        # magnitude-bearing population this figure counts over at all).
        self.assertEqual(len(SEB.not_held_by_engine(mag)), 8784)


class TestContentAnchorRedGreen(unittest.TestCase):
    """AT-35-E1-002's two proofs, on synthetic source text so the live file
    is never mutated by a test: a refactor that MOVES the cited function
    keeps the citation green; a change to one cited condition fails it."""

    def test_synthetic_source_resolves_GREEN(self):
        resolved = SEB.resolve_promotion_ladder(lines=_SYNTHETIC_SOURCE)
        self.assertTrue(resolved["ok"], resolved)
        # It resolved to the REAL ladder, not the sibling block that shares
        # the first three lines.
        self.assertEqual(_SYNTHETIC_SOURCE[resolved["line"] - 1].strip(), "if has_real_description")
        self.assertIn("class_feature_pool_catalog_holds", _SYNTHETIC_SOURCE[resolved["end_line"] - 1])
        self.assertEqual(SEB.citation_failures(lines=_SYNTHETIC_SOURCE), [])

    def test_moving_the_function_fifty_lines_stays_GREEN(self):
        before = SEB.resolve_promotion_ladder(lines=_SYNTHETIC_SOURCE)
        moved = ["// padding %d" % i for i in range(50)] + _SYNTHETIC_SOURCE
        after = SEB.resolve_promotion_ladder(lines=moved)
        self.assertTrue(after["ok"], after)
        self.assertEqual(after["line"], before["line"] + 50)
        self.assertEqual(after["source"], before["source"])
        self.assertEqual(SEB.citation_failures(lines=moved), [])

    def test_changing_one_cited_condition_is_RED(self):
        idx = SEB.resolve_promotion_ladder(lines=_SYNTHETIC_SOURCE)["end_line"] - 1
        mutated = list(_SYNTHETIC_SOURCE)
        mutated[idx] = mutated[idx].replace(
            "class_feature_pool_catalog_holds", "class_feature_pool_catalog_might_hold"
        )
        failures = SEB.citation_failures(lines=mutated)
        self.assertEqual(len(failures), 1, failures)
        self.assertIn("no longer contains", failures[0])
        self.assertIn("class_feature_pool_catalog_holds", failures[0])

    def test_a_second_copy_of_the_ladder_is_RED_as_ambiguous(self):
        # Two identical ladders inside `classify` means the citation no
        # longer names ONE construction site -- fail closed rather than
        # silently pick the first.
        resolved = SEB.resolve_promotion_ladder(lines=_SYNTHETIC_SOURCE)
        block = _SYNTHETIC_SOURCE[resolved["line"] - 1:resolved["end_line"]]
        duplicated = (
            _SYNTHETIC_SOURCE[:resolved["end_line"]]
            + ["    {", "    }"]
            + block
            + _SYNTHETIC_SOURCE[resolved["end_line"]:]
        )
        failures = SEB.citation_failures(lines=duplicated)
        self.assertEqual(len(failures), 1, failures)
        self.assertIn("ambiguous", failures[0])

    def test_function_vanishing_is_RED(self):
        renamed = [line.replace("fn classify(", "fn classify_renamed(") for line in _SYNTHETIC_SOURCE]
        failures = SEB.citation_failures(lines=renamed)
        self.assertEqual(len(failures), 1, failures)
        self.assertIn("does not resolve", failures[0])

    def test_build_report_fails_closed_on_a_stale_anchor(self):
        # The fail-closed path fires for the intended reason: the anchor's
        # content, not a harness error.
        orig = dict(SEB.PROMOTION_LADDER_ANCHOR)
        try:
            SEB.PROMOTION_LADDER_ANCHOR["anchor"] = ["this text does not appear in classify"]
            with self.assertRaises(SEB.StaleCitationError):
                SEB.build_report([_unit("u1", 1, "engine-does-not-hold")])
        finally:
            SEB.PROMOTION_LADDER_ANCHOR.clear()
            SEB.PROMOTION_LADDER_ANCHOR.update(orig)
        # GREEN again once restored -- proves the RED above was about content.
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
        self.assertIn(str(report["promotion_ladder_anchor_line"]), md)
        self.assertIn("fn classify", md)
        self.assertIn("denominator", md)


if __name__ == "__main__":
    unittest.main()
