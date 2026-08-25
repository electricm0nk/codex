#!/usr/bin/env python3
"""Tests for `scripts/family_vocabulary_reconcile.py` (SD-32 Gate 1, card
`family-vocabulary-reconciliation`, `decisions.md §12a`).

Proves:

1. The canonical family table is READ from `shape_ledger.py`'s own
   `FAMILIES`/`_family_metadata()`, never a hand-copy -- a drift guard
   (`FamilyVocabularyDriftTest`) monkeypatches `shape_ledger.FAMILIES` and
   proves the canonical table changes with it (RED if this script ever
   starts hand-copying family data instead of reading it).
2. The MT-to-canonical mapping covers every canonical family id and states
   a delta for every family that has an MT counterpart, `None` for F0
   (which has none).
3. The engine-coverage reconciliation (`reconcile_engine_coverage`) counts
   only F4-shaped bare-identifier segments, and correctly separates
   resolvable (has a producer elsewhere in the corpus) from unresolvable.
4. Fail-closed posture: an empty/formula-free corpus reports an error, not
   a false "0/0" or "100%" result.
"""

import json
import os
import sys
import tempfile
import unittest
from unittest import mock

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import family_vocabulary_reconcile as FVR  # noqa: E402
import shape_ledger as SL  # noqa: E402


def _write_record(path, raw_tokens):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        json.dump({"data": {"raw_tokens": raw_tokens}, "source": {"path": "x.lst", "line": 1}}, fh)


class CanonicalFamilyTableTest(unittest.TestCase):
    def test_reads_every_family_from_shape_ledger_including_extensions(self):
        ledger = {"families": {}}
        table = FVR.canonical_family_table(ledger)
        ids = {row["id"] for row in table}
        expected = {fid for fid, *_ in SL.FAMILIES} | {SL.FAMILY_F0_NO_FORMULA, SL.FAMILY_F8_OTHER}
        self.assertEqual(ids, expected)

    def test_counts_come_from_the_passed_ledger_not_hardcoded(self):
        ledger = {"families": {"F1": {"count": 999}}}
        table = FVR.canonical_family_table(ledger)
        f1 = next(r for r in table if r["id"] == "F1")
        self.assertEqual(f1["count"], 999)

    def test_priority_order_matches_shape_ledger_families_order(self):
        ledger = {"families": {}}
        table = FVR.canonical_family_table(ledger)
        priority_ids = [r["id"] for r in table if r["priority_rank"] is not None]
        expected_order = [fid for fid, *_ in SL.FAMILIES]
        self.assertEqual(priority_ids, expected_order)


class FamilyVocabularyDriftTest(unittest.TestCase):
    """The drift guard item 5 requires: prove this goes red if the ledger's
    families and canonical definition fork. Since the canonical table is
    read directly from `shape_ledger.FAMILIES`, this is exercised by
    mutating that in-memory table and confirming the canonical output
    reflects the mutation immediately -- if it did NOT, the canonical table
    would be a stale hand-copy, which is exactly the fork this module
    exists to prevent."""

    def test_canonical_table_reflects_a_shape_ledger_families_mutation(self):
        ledger = {"families": {}}
        before = FVR.canonical_family_table(ledger)
        before_f1_label = next(r for r in before if r["id"] == "F1")["label"]

        mutated_families = [
            (fid, "MUTATED LABEL FOR DRIFT TEST" if fid == "F1" else label, pred, pw)
            for fid, label, pred, pw in SL.FAMILIES
        ]
        with mock.patch.object(SL, "FAMILIES", mutated_families):
            after = FVR.canonical_family_table(ledger)
        after_f1_label = next(r for r in after if r["id"] == "F1")["label"]

        # RED proof: if this script ever hand-copies family labels instead
        # of reading SL.FAMILIES live, `after_f1_label` would still equal
        # `before_f1_label` and this assertion would fail.
        self.assertNotEqual(before_f1_label, after_f1_label)
        self.assertEqual(after_f1_label, "MUTATED LABEL FOR DRIFT TEST")
        # And it reverts cleanly once the mock context exits (proving the
        # "mutate, prove red, revert" cycle the acceptance bar requires).
        reverted = FVR.canonical_family_table(ledger)
        self.assertEqual(next(r for r in reverted if r["id"] == "F1")["label"], before_f1_label)


class MtMappingTableTest(unittest.TestCase):
    def test_every_canonical_family_present_and_f0_has_no_mt_counterpart(self):
        ledger = {"families": {}}
        canonical = FVR.canonical_family_table(ledger)
        mapping = FVR.mt_mapping_table(canonical)
        ids = {row["id"] for row in mapping}
        self.assertEqual(ids, {row["id"] for row in canonical})
        f0 = next(r for r in mapping if r["id"] == SL.FAMILY_F0_NO_FORMULA)
        self.assertIsNone(f0["mt_count"])
        self.assertIsNone(f0["delta"])
        self.assertIsNotNone(f0["note"])

    def test_delta_is_canonical_minus_mt(self):
        ledger = {"families": {"F1": {"count": 2000}}}
        canonical = FVR.canonical_family_table(ledger)
        mapping = FVR.mt_mapping_table(canonical)
        f1 = next(r for r in mapping if r["id"] == "F1")
        self.assertEqual(f1["mt_count"], 1747)
        self.assertEqual(f1["delta"], 2000 - 1747)


class EngineCoverageReconciliationTest(unittest.TestCase):
    def test_only_f4_shaped_segments_counted_and_producer_resolvability_checked(self):
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "book_a", "class_feature")
            # r1: a BONUS:VAR write whose FORMULA segment is the bare identifier
            # "AlchemistBombPool" (F4-shaped: no LVL suffix, not an ability
            # abbreviation) -> puts "AlchemistBombPool" in the F4 population.
            _write_record(
                os.path.join(book_dir, "r1.json"),
                [{"key": "BONUS", "value": "VAR|AlchemistBombLVL|AlchemistBombPool"}],
            )
            # r2: a DEFINE whose VARIABLE NAME (not its formula) is
            # "AlchemistBombPool" -> makes it a producer TARGET elsewhere in
            # the corpus, so r1's reference to it is resolvable.
            _write_record(
                os.path.join(book_dir, "r2.json"),
                [{"key": "DEFINE", "value": "AlchemistBombPool|5"}],
            )
            # r3: F4-shaped ("LonelyCounter" is a bare identifier), but
            # NOTHING anywhere in the corpus ever writes to "LonelyCounter"
            # -> unresolved.
            _write_record(
                os.path.join(book_dir, "r3.json"),
                [{"key": "BONUS", "value": "VAR|SomeOtherTarget|LonelyCounter"}],
            )
            # r4: NOT F4-shaped (arithmetic, not a bare identifier) -> excluded
            # from the F4 population entirely.
            _write_record(
                os.path.join(book_dir, "r4.json"),
                [{"key": "DEFINE", "value": "Whatever|LonelyCounter+2"}],
            )
            # r5/r6: a second F4-shaped identifier, "MasterChymistBonus",
            # whose ONLY producer anywhere in the corpus is a BONUS:VAR
            # write (never a DEFINE) -- exercises the BONUS:VAR branch of
            # `_producer_targets` specifically (the real-corpus
            # `AlchemistBombLVL` shape MEASURE-TWICE.md §3.1 names: a third
            # producer living on an entirely different record from the
            # DEFINE base).
            _write_record(
                os.path.join(book_dir, "r5.json"),
                [{"key": "BONUS", "value": "VAR|SomeTarget|MasterChymistBonus"}],
            )
            _write_record(
                os.path.join(book_dir, "r6.json"),
                [{"key": "BONUS", "value": "VAR|MasterChymistBonus|2"}],
            )
            result = FVR.reconcile_engine_coverage(tmp)
            # {AlchemistBombPool, LonelyCounter, MasterChymistBonus}
            self.assertEqual(result["population"], 3)
            # AlchemistBombPool (DEFINE producer) + MasterChymistBonus (BONUS:VAR producer)
            self.assertEqual(result["resolved"], 2)
            self.assertAlmostEqual(result["resolved_pct"], round(100 * 2 / 3, 1))

    def test_empty_corpus_fails_closed(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.makedirs(os.path.join(tmp, "book_a"))
            result = FVR.reconcile_engine_coverage(tmp)
            self.assertEqual(result["population"], 0)
            self.assertIn("error", result)
            self.assertIn("no coverage", result["error"])


if __name__ == "__main__":
    unittest.main()
