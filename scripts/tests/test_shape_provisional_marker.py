#!/usr/bin/env python3
"""Tests for `scripts/shape_provisional_marker.py` (`decisions.md §27`/
`§27a`/`§27b`, `kanban.md` row 17).

Proves the contract's three load-bearing claims:

1. `stamp_provisional_default` always sets marker + reason TOGETHER, and
   refuses a reason-less stamp (a silent default is exactly what `§27`
   forbids).
2. `is_provisional_default` reads it back correctly and defaults to False
   on an untouched record -- absence of the marker is never mistaken for
   presence.
3. `scan_corpus_for_provisional_defaults` finds every marked record in a
   synthetic corpus tree and reports a missing reason as `None` rather
   than raising or silently dropping the hit.

Uses a small synthetic `tempfile` corpus tree, never the live corpus --
same discipline `test_shape_ledger.py`'s docstring states.
"""

import json
import os
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_provisional_marker as SPM  # noqa: E402


class StampAndReadTest(unittest.TestCase):
    def test_stamp_sets_marker_and_reason_together(self):
        record = {"data": {"key": "Foo"}}
        SPM.stamp_provisional_default(record, "delivery-only TYPE row, no facet segment")
        self.assertTrue(SPM.is_provisional_default(record))
        self.assertEqual(
            SPM.provisional_reason(record), "delivery-only TYPE row, no facet segment"
        )

    def test_empty_reason_is_rejected(self):
        record = {"data": {"key": "Foo"}}
        with self.assertRaises(ValueError):
            SPM.stamp_provisional_default(record, "")
        with self.assertRaises(ValueError):
            SPM.stamp_provisional_default(record, "   ")
        # And the record was never mutated by the rejected call.
        self.assertFalse(SPM.is_provisional_default(record))

    def test_untouched_record_is_not_provisional(self):
        record = {"data": {"key": "Foo", "raw_tokens": []}}
        self.assertFalse(SPM.is_provisional_default(record))
        self.assertIsNone(SPM.provisional_reason(record))

    def test_record_with_no_data_object_is_not_provisional(self):
        self.assertFalse(SPM.is_provisional_default({}))

    def test_idempotent_same_reason(self):
        record = {"data": {"key": "Foo"}}
        SPM.stamp_provisional_default(record, "reason A")
        SPM.stamp_provisional_default(record, "reason A")
        self.assertEqual(SPM.provisional_reason(record), "reason A")

    def test_creates_data_object_if_absent(self):
        record = {}
        SPM.stamp_provisional_default(record, "reason A")
        self.assertTrue(SPM.is_provisional_default(record))


class ClearProvisionalDefaultTest(unittest.TestCase):
    """`clear_provisional_default` -- row 17's own paired counterpart, used
    once a defaulted unit's real shape has been derived and confirmed
    (`decisions.md §27a`/§27b: "record it as such, remove the provisional
    marker")."""

    def test_clears_both_marker_fields(self):
        record = {"data": {"key": "Foo"}}
        SPM.stamp_provisional_default(record, "reason A")
        SPM.clear_provisional_default(record)
        self.assertFalse(SPM.is_provisional_default(record))
        self.assertIsNone(SPM.provisional_reason(record))

    def test_leaves_other_data_fields_untouched(self):
        record = {"data": {"key": "Foo", "facet": "SpecialQuality"}}
        SPM.stamp_provisional_default(record, "reason A")
        SPM.clear_provisional_default(record)
        self.assertEqual(record["data"]["key"], "Foo")
        self.assertEqual(record["data"]["facet"], "SpecialQuality")

    def test_no_op_on_a_never_stamped_record(self):
        record = {"data": {"key": "Foo"}}
        SPM.clear_provisional_default(record)
        self.assertFalse(SPM.is_provisional_default(record))

    def test_no_op_on_a_record_with_no_data_object(self):
        record = {}
        SPM.clear_provisional_default(record)  # must not raise
        self.assertFalse(SPM.is_provisional_default(record))


class ScanCorpusTest(unittest.TestCase):
    def _write(self, root, book, kind, name, data, source_line=1):
        d = os.path.join(root, book, kind)
        os.makedirs(d, exist_ok=True)
        rec = {
            "data": data,
            "source": {"path": f"{book}.lst", "line": source_line},
        }
        with open(os.path.join(d, f"{name}.json"), "w", encoding="utf-8") as fh:
            json.dump(rec, fh)

    def test_finds_marked_record_and_ignores_unmarked(self):
        with tempfile.TemporaryDirectory() as root:
            self._write(root, "bestiary", "monster_ability", "marked", {
                "key": "X", SPM.PROVISIONAL_DEFAULT_FIELD: True,
                SPM.PROVISIONAL_DEFAULT_REASON_FIELD: "delivery-only TYPE row",
            })
            self._write(root, "bestiary", "monster_ability", "unmarked", {"key": "Y"})
            hits = SPM.scan_corpus_for_provisional_defaults(root)
            self.assertEqual(len(hits), 1)
            self.assertEqual(hits[0]["book"], "bestiary")
            self.assertEqual(hits[0]["kind"], "monster_ability")
            self.assertEqual(hits[0]["id_or_key"], "X")
            self.assertEqual(hits[0]["reason"], "delivery-only TYPE row")

    def test_marker_true_with_missing_reason_is_reported_as_none_not_dropped(self):
        """A malformed marker (set outside this module's contract, e.g. by
        a future cycle that reaches into `data` directly instead of
        calling `stamp_provisional_default`) must still surface as a hit
        -- silently dropping it would hide the very defect `§1a` exists to
        catch."""
        with tempfile.TemporaryDirectory() as root:
            self._write(root, "bestiary", "monster_ability", "malformed", {
                "key": "Z", SPM.PROVISIONAL_DEFAULT_FIELD: True,
            })
            hits = SPM.scan_corpus_for_provisional_defaults(root)
            self.assertEqual(len(hits), 1)
            self.assertIsNone(hits[0]["reason"])

    def test_books_restriction_only_walks_named_books(self):
        with tempfile.TemporaryDirectory() as root:
            self._write(root, "bestiary", "monster_ability", "a", {
                "key": "A", SPM.PROVISIONAL_DEFAULT_FIELD: True,
                SPM.PROVISIONAL_DEFAULT_REASON_FIELD: "r",
            })
            self._write(root, "bestiary_2", "monster_ability", "b", {
                "key": "B", SPM.PROVISIONAL_DEFAULT_FIELD: True,
                SPM.PROVISIONAL_DEFAULT_REASON_FIELD: "r",
            })
            hits = SPM.scan_corpus_for_provisional_defaults(root, books={"bestiary"})
            self.assertEqual(len(hits), 1)
            self.assertEqual(hits[0]["book"], "bestiary")

    def test_empty_corpus_returns_empty_list_not_an_error(self):
        with tempfile.TemporaryDirectory() as root:
            self.assertEqual(SPM.scan_corpus_for_provisional_defaults(root), [])

    def test_nonexistent_root_returns_empty_list(self):
        self.assertEqual(
            SPM.scan_corpus_for_provisional_defaults("/nonexistent/path/xyz"), []
        )


if __name__ == "__main__":
    unittest.main()
