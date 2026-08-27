#!/usr/bin/env python3
"""Tests for `scripts/missing_engine_tables.py` (SD-34 Epic 1, AT-34-E1-003).

Proves the load-bearing claim: bucket A (8,463 units, `has_no_engine_table`
evidence) is enumerated **per kind**, with a per-book breakdown, the engine
surface (the exact `not_ingested(...)` call site in
`src/bin/v06_work_inventory.rs`) a real table would replace, and the set of
books that would reach zero bucket-A once that kind's table exists.

Uses small synthetic inventory fixtures so these tests stay fast and are not
subject to corpus drift across cycles (`test_completion_atlas.py` and
`test_box_ledger.py` set the same precedent in this repo). The live 8,463-
unit population is exercised separately, as acceptance evidence, by running
the committed CLI against the committed `docs/work-inventory.json`.
"""

import os
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import missing_engine_tables as MET  # noqa: E402


def _unit(id_, kind, book, bucket_a=True):
    evidence = f"{kind}_content_has_no_engine_table" if bucket_a else "grounded"
    status = "not-ingested" if bucket_a else "grounded"
    return {"id": id_, "kind": kind, "book": book, "status": status, "evidence": evidence}


class TestPerKindBreakdown(unittest.TestCase):
    def test_single_kind_single_book(self):
        units = [_unit("u1", "power", "ultimate_psionics")]
        result = MET.build_report(units)
        self.assertEqual(result["kinds"]["power"]["count"], 1)
        self.assertEqual(result["kinds"]["power"]["by_book"], {"ultimate_psionics": 1})

    def test_non_bucket_a_unit_excluded(self):
        units = [
            _unit("u1", "power", "ultimate_psionics"),
            _unit("u2", "power", "ultimate_psionics", bucket_a=False),
        ]
        result = MET.build_report(units)
        self.assertEqual(result["kinds"]["power"]["count"], 1)

    def test_population_sums_across_kinds(self):
        units = [
            _unit("u1", "power", "ultimate_psionics"),
            _unit("u2", "trait", "ultimate_campaign"),
            _unit("u3", "trait", "advanced_players_guide"),
        ]
        result = MET.build_report(units)
        self.assertEqual(result["population"], 3)
        self.assertEqual(sum(k["count"] for k in result["kinds"].values()), 3)

    def test_unclassified_kind_is_a_defect_not_silently_dropped(self):
        # A kind absent from ENGINE_SURFACE_CITATIONS must fail closed, not
        # silently drop the unit from the report (`decisions.md §12` L1).
        units = [_unit("u1", "monster_ability", "bestiary")]
        with self.assertRaises(MET.UnknownKindError):
            MET.build_report(units)


class TestZeroABooks(unittest.TestCase):
    def test_book_cleared_when_only_kind_present(self):
        units = [_unit("u1", "power", "ultimate_psionics")]
        result = MET.build_report(units)
        self.assertIn("ultimate_psionics", result["kinds"]["power"]["zero_bucket_a_books"])

    def test_book_not_cleared_when_another_kind_also_present(self):
        units = [
            _unit("u1", "ability", "core_rulebook"),
            _unit("u2", "template", "core_rulebook"),
        ]
        result = MET.build_report(units)
        self.assertNotIn("core_rulebook", result["kinds"]["ability"]["zero_bucket_a_books"])
        self.assertNotIn("core_rulebook", result["kinds"]["template"]["zero_bucket_a_books"])

    def test_book_cleared_by_a_kind_even_if_another_book_is_mixed(self):
        units = [
            _unit("u1", "ability", "core_rulebook"),
            _unit("u2", "template", "core_rulebook"),
            _unit("u3", "ability", "inner_sea_faiths"),
        ]
        result = MET.build_report(units)
        self.assertIn("inner_sea_faiths", result["kinds"]["ability"]["zero_bucket_a_books"])
        self.assertNotIn("core_rulebook", result["kinds"]["ability"]["zero_bucket_a_books"])


class TestEngineSurfaceCitation(unittest.TestCase):
    def test_every_bucket_a_kind_has_a_citation(self):
        units = [_unit("u1", k, "some_book") for k in MET.ENGINE_SURFACE_CITATIONS]
        result = MET.build_report(units)
        for k in MET.ENGINE_SURFACE_CITATIONS:
            surface = result["kinds"][k]["engine_surface"]
            self.assertEqual(surface["file"], "src/bin/v06_work_inventory.rs")
            self.assertIsInstance(surface["line"], int)
            self.assertIn("has_no_engine_table", surface["must_contain"])

    def test_citation_resolves_at_head(self):
        # The real, committed file -- not a fixture. Guards against the
        # citation drifting out of sync with a future refactor
        # (`decisions.md §12` L1, same shape as completion_atlas condition 6).
        failures = MET.citation_failures()
        self.assertEqual(failures, [], f"stale citations: {failures}")


class TestLiveInventory(unittest.TestCase):
    """Acceptance-level checks against the real, committed corpus."""

    def test_live_population_and_kind_count(self):
        units = MET._load_units()
        report = MET.build_report(units)
        self.assertEqual(report["population"], 8463)
        self.assertEqual(len(report["kinds"]), 9)

    def test_live_core_rulebook_slice_matches_technical_design(self):
        units = MET._load_units()
        report = MET.build_report(units)
        expected_core = {
            "ability": 471, "template": 262, "skill": 110, "domain": 34,
            "language": 22, "deity": 21, "companion": 14,
        }
        for kind, expected in expected_core.items():
            self.assertEqual(
                report["kinds"][kind]["by_book"].get("core_rulebook", 0),
                expected,
                f"kind={kind}",
            )
        # power and trait are NOT exercised by core_rulebook.
        self.assertNotIn("core_rulebook", report["kinds"]["power"]["by_book"])
        self.assertNotIn("core_rulebook", report["kinds"]["trait"]["by_book"])

    def test_live_ultimate_campaign_is_almost_single_bucket(self):
        units = MET._load_units()
        report = MET.build_report(units)
        # trait: 154 of ultimate_campaign's bucket-A population is `trait`.
        self.assertEqual(
            report["kinds"]["trait"]["by_book"].get("ultimate_campaign", 0), 154,
        )


if __name__ == "__main__":
    unittest.main()
