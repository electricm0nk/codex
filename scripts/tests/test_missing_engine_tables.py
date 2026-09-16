#!/usr/bin/env python3
"""Tests for `scripts/missing_engine_tables.py` (SD-34 Epic 1, AT-34-E1-003).

Proves the load-bearing claim: bucket A (`has_no_engine_table` evidence) is
enumerated **per kind**, with a per-book breakdown, the engine surface (the
exact `engine_does_not_hold(...)` call site in `src/bin/v06_work_inventory.rs`)
a real table would replace, and the set of books that would reach zero
bucket-A once that kind's table exists.

Uses small synthetic inventory fixtures so these tests stay fast and are not
subject to corpus drift across cycles (`test_completion_atlas.py` and
`test_box_ledger.py` set the same precedent in this repo). The live
population is exercised separately, as acceptance evidence, by running the
committed CLI against the committed `docs/work-inventory.json` -- it was
8,463 units across 9 kinds as of `AT-34-E1-003`; building the seven
`simple_kind_tables` and wiring them into `classify()` for real
(`AT-34-E2-001`/`AT-34-E2-004`) took it to 449 units across 2 kinds
(`companion`, `power`).
"""

import os
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import missing_engine_tables as MET  # noqa: E402


def _unit(id_, kind, book, bucket_a=True):
    evidence = f"{kind}_content_has_no_engine_table" if bucket_a else "grounded"
    status = "engine-does-not-hold" if bucket_a else "grounded"
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
            _unit("u2", "companion", "bestiary"),
            _unit("u3", "companion", "bestiary"),
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
            _unit("u1", "power", "ultimate_psionics"),
            _unit("u2", "companion", "ultimate_psionics"),
        ]
        result = MET.build_report(units)
        self.assertNotIn("ultimate_psionics", result["kinds"]["power"]["zero_bucket_a_books"])
        self.assertNotIn("ultimate_psionics", result["kinds"]["companion"]["zero_bucket_a_books"])

    def test_book_cleared_by_a_kind_even_if_another_book_is_mixed(self):
        units = [
            _unit("u1", "power", "ultimate_psionics"),
            _unit("u2", "companion", "ultimate_psionics"),
            _unit("u3", "power", "bestiary"),
        ]
        result = MET.build_report(units)
        self.assertIn("bestiary", result["kinds"]["power"]["zero_bucket_a_books"])
        self.assertNotIn("ultimate_psionics", result["kinds"]["power"]["zero_bucket_a_books"])


class TestEngineSurfaceCitation(unittest.TestCase):
    def test_every_bucket_a_kind_has_a_citation(self):
        units = [_unit("u1", k, "some_book") for k in MET.ENGINE_SURFACE_CITATIONS]
        result = MET.build_report(units)
        for k in MET.ENGINE_SURFACE_CITATIONS:
            surface = result["kinds"][k]["engine_surface"]
            self.assertEqual(surface["file"], "src/bin/v06_work_inventory.rs")
            self.assertEqual(surface["context_fn"], "classify")
            self.assertIn("has_no_engine_table", surface["anchor"])
            # Derived by search at check time (AT-35-E1-002), never pinned.
            self.assertIsInstance(surface["resolved_line"], int)
            self.assertGreater(surface["resolved_line"], 0)
            self.assertNotIn("line", surface)

    def test_citation_resolves_at_head(self):
        # The real, committed file -- not a fixture. Guards against the
        # citation drifting out of sync with a future refactor
        # (`decisions.md §12` L1, same shape as completion_atlas condition 6).
        failures = MET.citation_failures()
        self.assertEqual(failures, [], f"stale citations: {failures}")


_SYNTHETIC = [
    "fn classify(unit: &Unit) -> Verdict {",
    "    match unit.kind {",
    '        Kind::Companion => engine_does_not_hold("companion_content_has_no_engine_table"),',
    '        Kind::Power => engine_does_not_hold("power_content_has_no_engine_table"),',
    "    }",
    "}",
]


class TestContentAnchorRedGreen(unittest.TestCase):
    """AT-35-E1-002: moving `classify` 50 lines keeps both anchors green;
    changing one cited arm fails exactly that kind."""

    def test_synthetic_source_is_GREEN(self):
        self.assertEqual(MET.citation_failures(lines=_SYNTHETIC), [])

    def test_moving_the_function_fifty_lines_stays_GREEN(self):
        moved = ["// %d" % i for i in range(50)] + _SYNTHETIC
        self.assertEqual(MET.citation_failures(lines=moved), [])

    def test_changing_one_cited_arm_is_RED_for_that_kind_only(self):
        mutated = list(_SYNTHETIC)
        mutated[2] = mutated[2].replace("companion_content_has_no_engine_table", "companion_has_a_table_now")
        failures = MET.citation_failures(lines=mutated)
        self.assertEqual(len(failures), 1, failures)
        self.assertTrue(failures[0].startswith("companion:"), failures[0])
        self.assertIn("no longer contains", failures[0])


class TestLiveInventory(unittest.TestCase):
    """Acceptance-level checks against the real, committed corpus."""

    def test_live_population_and_kind_count(self):
        # `AT-34-E2-004`: 8,463/9 (AT-34-E1-003 baseline) -> 449/2 once the
        # seven `simple_kind_tables` are wired into `classify()` for real --
        # only `companion` (28, `bestiary`-book, no chassis registration at
        # all for that book) and `power` (421, Epic 5's to build) remain.
        # SD-35 drains this population too (Epic 2's converter renders the
        # sheet line; Epic 5 builds the `power` chassis), so the assertion is
        # the ceiling and the kind set, not an equality that has to be
        # hand-edited every cycle. It still fails CLOSED: a new kind, or a
        # count above the SD-34 mark, breaks it.
        # `449` / `{companion, power}` is the SD-34 AT-34-E2-004 high-water
        # mark. Live at SD-35 `4e321d2c6c`: population=1, kinds={power}.
        # Re-derive: `python3 scripts/missing_engine_tables.py --check`.
        units = MET._load_units()
        report = MET.build_report(units)
        self.assertLessEqual(report["population"], 449)
        self.assertLessEqual(len(report["kinds"]), 2)
        self.assertTrue(set(report["kinds"]) <= {"companion", "power"}, report["kinds"])

    def test_live_core_rulebook_and_ultimate_campaign_have_zero_bucket_a(self):
        # `AT-34-E2-004`'s own evidence bar, restated as a pinned test: the
        # two vehicle books reach bucket A zero. Neither book appears in
        # ANY remaining kind's `by_book` breakdown.
        units = MET._load_units()
        report = MET.build_report(units)
        for kind, kind_report in report["kinds"].items():
            self.assertNotIn("core_rulebook", kind_report["by_book"], f"kind={kind}")
            self.assertNotIn("ultimate_campaign", kind_report["by_book"], f"kind={kind}")

    def test_live_remaining_population_is_power_and_bestiary_companion_only(self):
        # The BOOK set is the invariant that does not drain: whatever survives
        # under `power` is `ultimate_psionics`, and whatever survives under
        # `companion` is `bestiary`. A unit from any other book appearing here
        # is the regression this test catches. Counts are ceilings against the
        # SD-34 marks (421 / 28); live at SD-35 `4e321d2c6c`: power 1,
        # companion absent. Re-derive:
        # `python3 scripts/missing_engine_tables.py --check`.
        units = MET._load_units()
        report = MET.build_report(units)
        ceilings = {"power": ("ultimate_psionics", 421), "companion": ("bestiary", 28)}
        for kind, (book, ceiling) in ceilings.items():
            if kind not in report["kinds"]:
                continue
            by_book = report["kinds"][kind]["by_book"]
            self.assertEqual(set(by_book), {book}, f"kind={kind}")
            self.assertLessEqual(by_book[book], ceiling, f"kind={kind}")


if __name__ == "__main__":
    unittest.main()
