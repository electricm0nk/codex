"""Self-test for `scripts/observer/provenance.py` -- Decision 14's
provenance schema (SD31-D14-PROV-001, 2026-08-17).

Every gate is exercised on BOTH a clean case (passes) and a deliberately
broken one (fails), per this package's "a gate that cannot fail proves
nothing" doctrine -- see each TestCase's own class docstring for which
invariant it proves.

Run: python3 -m unittest scripts.tests.test_provenance
"""
from __future__ import annotations

import importlib.util
import pathlib
import sys
import unittest

_MODULE_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "observer" / "provenance.py"
)
_spec = importlib.util.spec_from_file_location("provenance", _MODULE_PATH)
provenance = importlib.util.module_from_spec(_spec)
# Must be registered in sys.modules BEFORE exec_module: provenance.py uses
# `@dataclasses.dataclass`, and the dataclasses machinery resolves forward-
# referenced type annotations via `sys.modules[cls.__module__]` -- a module
# object that was never registered under its own `__name__` makes that
# lookup return None and crashes with a bare AttributeError on import,
# unrelated to anything this test actually checks.
sys.modules["provenance"] = provenance
_spec.loader.exec_module(provenance)


def rec(obj, book, status, basis=""):
    return provenance.ProvenanceRecord(object_id=obj, book=book, status=status, basis=basis)


class RecordValidationTests(unittest.TestCase):
    def test_unknown_status_is_rejected(self):
        with self.assertRaises(ValueError):
            rec("dwarf", "core_rulebook", "made-up-status")

    def test_every_canonical_status_constructs(self):
        for status in provenance.CANONICAL_STATUSES:
            rec("x", "book", status)  # must not raise

    def test_unclassified_is_a_valid_bookkeeping_status(self):
        rec("x", "book", provenance.UNCLASSIFIED)  # must not raise


class TotalityTests(unittest.TestCase):
    """Invariant 1: every (object, book) pair carries exactly one status,
    no default."""

    def test_a_fully_covered_universe_passes(self):
        universe = {("dwarf", "core_rulebook"), ("elf", "core_rulebook")}
        records = [
            rec("dwarf", "core_rulebook", provenance.ORIGIN),
            rec("elf", "core_rulebook", provenance.ORIGIN),
        ]
        self.assertEqual(provenance.check_totality(records, universe), [])

    def test_a_missing_pair_is_caught(self):
        universe = {("dwarf", "core_rulebook"), ("elf", "core_rulebook")}
        records = [rec("dwarf", "core_rulebook", provenance.ORIGIN)]
        violations = provenance.check_totality(records, universe)
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0].kind, "MISSING")

    def test_a_duplicated_pair_is_caught(self):
        universe = {("dwarf", "core_rulebook")}
        records = [
            rec("dwarf", "core_rulebook", provenance.ORIGIN),
            rec("dwarf", "core_rulebook", provenance.DUPLICATE),
        ]
        violations = provenance.check_totality(records, universe)
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0].kind, "DUPLICATE")

    def test_unclassified_satisfies_totality(self):
        # UNCLASSIFIED is a real, explicit value -- it must not itself
        # read as "missing."
        universe = {("catfolk", "bestiary_3")}
        records = [rec("catfolk", "bestiary_3", provenance.UNCLASSIFIED)]
        self.assertEqual(provenance.check_totality(records, universe), [])

    def test_a_record_outside_the_universe_is_caught(self):
        universe = {("dwarf", "core_rulebook")}
        records = [
            rec("dwarf", "core_rulebook", provenance.ORIGIN),
            rec("ghost_dwarf", "some_book", provenance.ORIGIN),
        ]
        violations = provenance.check_totality(records, universe)
        kinds = {v.kind for v in violations}
        self.assertIn("OUT_OF_UNIVERSE", kinds)


class ExactlyOneAuthoritativeTests(unittest.TestCase):
    """Invariant 2: exactly one AUTHORITATIVE pair per object."""

    def test_a_single_origin_passes(self):
        records = [rec("dwarf", "core_rulebook", provenance.ORIGIN)]
        self.assertEqual(provenance.check_exactly_one_authoritative(records), [])

    def test_origin_flipped_to_superseded_plus_errata_source_passes(self):
        records = [
            rec("dwarf", "core_rulebook", provenance.SUPERSEDED),
            rec("dwarf", "advanced_race_guide", provenance.ERRATA_SOURCE),
        ]
        self.assertEqual(provenance.check_exactly_one_authoritative(records), [])

    def test_zero_authoritative_pairs_is_unowned(self):
        records = [rec("dwarf", "core_rulebook", provenance.DUPLICATE)]
        violations = provenance.check_exactly_one_authoritative(records)
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0].kind, "UNOWNED")

    def test_two_authoritative_pairs_means_the_comparison_never_ran(self):
        # Real defect shape: origin never flipped when the errata-source
        # was added.
        records = [
            rec("dwarf", "core_rulebook", provenance.ORIGIN),
            rec("dwarf", "advanced_race_guide", provenance.ERRATA_SOURCE),
        ]
        violations = provenance.check_exactly_one_authoritative(records)
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0].kind, "AMBIGUOUS_AUTHORITY")

    def test_all_unclassified_pairs_are_excluded_not_flagged_unowned(self):
        records = [rec("catfolk", "bestiary_3", provenance.UNCLASSIFIED),
                   rec("catfolk", "advanced_race_guide", provenance.UNCLASSIFIED)]
        self.assertEqual(provenance.check_exactly_one_authoritative(records), [])


class ComputeDenominatorTests(unittest.TestCase):
    """Invariant 3: denominator = authoritative + variant, derived."""

    def test_origin_and_variant_count_duplicate_and_superseded_do_not(self):
        records = [
            rec("dwarf", "core_rulebook", provenance.ORIGIN),
            rec("grey_dwarf", "advanced_race_guide", provenance.VARIANT),
            rec("elf", "core_rulebook", provenance.DUPLICATE),
            rec("gnome", "core_rulebook", provenance.SUPERSEDED),
        ]
        result = provenance.compute_denominator(records)
        self.assertEqual(result["denominator"], 2)
        self.assertEqual(result["by_status"][provenance.ORIGIN], 1)
        self.assertEqual(result["by_status"][provenance.VARIANT], 1)
        self.assertEqual(result["by_status"][provenance.DUPLICATE], 1)

    def test_errata_source_counts_toward_denominator_origin_does_not_once_superseded(self):
        records = [
            rec("dwarf", "core_rulebook", provenance.SUPERSEDED),
            rec("dwarf", "advanced_race_guide", provenance.ERRATA_SOURCE),
        ]
        result = provenance.compute_denominator(records)
        self.assertEqual(result["denominator"], 1)

    def test_unclassified_never_counts(self):
        records = [rec("catfolk", "bestiary_3", provenance.UNCLASSIFIED)]
        self.assertEqual(provenance.compute_denominator(records)["denominator"], 0)

    def test_is_a_pure_fold_same_input_same_output(self):
        records = [rec("dwarf", "core_rulebook", provenance.ORIGIN)]
        a = provenance.compute_denominator(records)
        b = provenance.compute_denominator(records)
        self.assertEqual(a, b)


class PackagingArtifactAndStructuralSignatureTests(unittest.TestCase):
    """Invariant 4: packaging-artifact -> zero; descoped-structural signed."""

    def test_no_packaging_artifact_passes(self):
        records = [rec("android", "core_rulebook", provenance.ORIGIN)]
        self.assertEqual(provenance.check_packaging_artifact_trending_to_zero(records), [])

    def test_a_remaining_packaging_artifact_is_reported(self):
        records = [rec("android", "core_essentials", provenance.PACKAGING_ARTIFACT)]
        violations = provenance.check_packaging_artifact_trending_to_zero(records)
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0].kind, "PACKAGING_ARTIFACT_REMAINING")

    def test_a_signed_structural_exclusion_passes(self):
        records = [rec("gnome", "some_book", provenance.DESCOPED_STRUCTURAL)]
        signed = {("gnome", "some_book")}
        self.assertEqual(provenance.check_descoped_structural_signed(records, signed), [])

    def test_an_unsigned_structural_exclusion_is_caught(self):
        # A cycle may PROPOSE, never grant on its own -- §3.
        records = [rec("gnome", "some_book", provenance.DESCOPED_STRUCTURAL)]
        violations = provenance.check_descoped_structural_signed(records, signed_pairs=set())
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0].kind, "UNSIGNED_STRUCTURAL_EXCLUSION")

    def test_descoped_licensing_needs_no_signature(self):
        records = [rec("x", "book", provenance.DESCOPED_LICENSING)]
        self.assertEqual(provenance.check_descoped_structural_signed(records, signed_pairs=set()), [])


class DonenessIndependenceTests(unittest.TestCase):
    """Invariant 5: a provenance change must move ZERO doneness fields."""

    def test_unchanged_doneness_passes(self):
        before = {"u1": "held", "u2": "done"}
        after = {"u1": "held", "u2": "done"}
        self.assertEqual(provenance.assert_provenance_change_does_not_move_doneness(before, after), [])

    def test_a_unit_leaving_the_denominator_is_not_itself_a_violation(self):
        before = {"u1": "held"}
        after = {}
        self.assertEqual(provenance.assert_provenance_change_does_not_move_doneness(before, after), [])

    def test_a_doneness_verdict_that_moves_is_caught(self):
        before = {"u1": "held"}
        after = {"u1": "done"}
        violations = provenance.assert_provenance_change_does_not_move_doneness(before, after)
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0].kind, "DONENESS_MOVED_BY_PROVENANCE_CHANGE")


class DenominatorChangeReportTests(unittest.TestCase):
    """Invariant 6: any denominator change is reported as its own number,
    with the count per status."""

    def test_no_change_reports_zero_delta(self):
        snap = provenance.compute_denominator([rec("dwarf", "core_rulebook", provenance.ORIGIN)])
        delta = provenance.report_denominator_change(snap, snap)
        self.assertEqual(delta["denominator_delta"], 0)
        self.assertEqual(delta["per_status_delta"], {})

    def test_a_real_change_reports_the_total_and_per_status_delta(self):
        before = provenance.compute_denominator([rec("dwarf", "core_rulebook", provenance.ORIGIN)])
        after = provenance.compute_denominator([
            rec("dwarf", "core_rulebook", provenance.ORIGIN),
            rec("elf", "core_rulebook", provenance.ORIGIN),
        ])
        delta = provenance.report_denominator_change(before, after)
        self.assertEqual(delta["denominator_delta"], 1)
        self.assertEqual(delta["per_status_delta"][provenance.ORIGIN], 1)


class ClassifyUnambiguousTests(unittest.TestCase):
    """Population: unambiguous cases only, everything else UNCLASSIFIED."""

    def test_a_single_book_object_is_origin(self):
        pairs = [("dwarf", "core_rulebook", "race")]
        records = provenance.classify_unambiguous(pairs, packaging_artifact_books=set())
        self.assertEqual(len(records), 1)
        self.assertEqual(records[0].status, provenance.ORIGIN)

    def test_a_multi_book_object_is_left_unclassified(self):
        pairs = [
            ("catfolk", "bestiary_3", "race"),
            ("catfolk", "advanced_race_guide", "race"),
        ]
        records = provenance.classify_unambiguous(pairs, packaging_artifact_books=set())
        self.assertEqual(len(records), 2)
        self.assertTrue(all(r.status == provenance.UNCLASSIFIED for r in records))

    def test_a_packaging_artifact_book_is_flagged_regardless_of_book_count(self):
        pairs = [("android", "core_essentials", "race")]
        records = provenance.classify_unambiguous(pairs, packaging_artifact_books={"core_essentials"})
        self.assertEqual(records[0].status, provenance.PACKAGING_ARTIFACT)

    def test_totality_holds_over_the_output(self):
        pairs = [
            ("dwarf", "core_rulebook", "race"),
            ("catfolk", "bestiary_3", "race"),
            ("catfolk", "advanced_race_guide", "race"),
            ("android", "core_essentials", "race"),
        ]
        records = provenance.classify_unambiguous(pairs, packaging_artifact_books={"core_essentials"})
        universe = {(obj, book) for obj, book, _kind in pairs}
        self.assertEqual(provenance.check_totality(records, universe), [])

    def test_every_populated_record_carries_a_basis(self):
        pairs = [
            ("dwarf", "core_rulebook", "race"),
            ("catfolk", "bestiary_3", "race"),
            ("catfolk", "advanced_race_guide", "race"),
        ]
        records = provenance.classify_unambiguous(pairs, packaging_artifact_books=set())
        self.assertTrue(all(r.basis for r in records), "every record must state why it got its status")


if __name__ == "__main__":
    unittest.main()
