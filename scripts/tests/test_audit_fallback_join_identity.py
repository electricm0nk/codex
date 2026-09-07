#!/usr/bin/env python3
"""Tests for `scripts/audit_fallback_join_identity.py` (SD-32 kanban row 22,
`epic-12-fallback-join-correctness-audit`).

`shape_ledger.py`'s `key_index`/`cross_book_key_index` fallbacks exist for
real reasons (a citation-redirect, a cross-book widen-access row -- see
their own docstrings) and must keep working. But nothing in `classify_unit`
itself stops a fallback from resolving to the WRONG record if its in-memory
index ever desyncs from the corpus on disk -- exactly the shape of the
THREE join defects this bundle already found and fixed (book-alias,
citation-redirect, kind-blind join). This module proves the audit's
identity check actually catches that failure mode, not just that it passes
on well-formed input.
"""

import json
import os
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import audit_fallback_join_identity as AUD  # noqa: E402


def _unit(id_, kind, book, source_file, source_line, corpus_key, status="engine-does-not-hold"):
    return {
        "id": id_,
        "kind": kind,
        "book": book,
        "status": status,
        "source_file": source_file,
        "source_line": source_line,
        "corpus_key": corpus_key,
    }


def _write_record(path, key, raw_tokens, source_path, source_line):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        json.dump(
            {
                "data": {"key": key, "raw_tokens": raw_tokens},
                "source": {"path": source_path, "line": source_line},
            },
            fh,
        )


class ResolveRecordPathForKeyTest(unittest.TestCase):
    def test_finds_a_real_record_under_the_book_and_kind_directory(self):
        with tempfile.TemporaryDirectory() as tmp:
            _write_record(
                os.path.join(tmp, "some_book", "spell", "widget.json"),
                key="Widget",
                raw_tokens=[],
                source_path="widget.lst",
                source_line=10,
            )
            hits = AUD._resolve_record_path_for_key(tmp, "some_book", "spell", "Widget")
            self.assertEqual(len(hits), 1)

    def test_reports_no_hits_when_the_key_exists_only_under_a_different_kind(self):
        """A same-named key that lives under a DIFFERENT kind directory must
        never satisfy the check -- this is the identity-boundary the whole
        audit exists to defend (decisions.md's kind-blind-join defect,
        +1,024 units, was exactly this shape: a different kind's record at
        the same coordinate silently answering the join)."""
        with tempfile.TemporaryDirectory() as tmp:
            _write_record(
                os.path.join(tmp, "some_book", "equipment", "widget.json"),
                key="Widget",
                raw_tokens=[],
                source_path="widget.lst",
                source_line=10,
            )
            hits = AUD._resolve_record_path_for_key(tmp, "some_book", "spell", "Widget")
            self.assertEqual(hits, [])

    def test_reports_no_hits_when_the_key_exists_only_under_a_different_book(self):
        with tempfile.TemporaryDirectory() as tmp:
            _write_record(
                os.path.join(tmp, "other_book", "spell", "widget.json"),
                key="Widget",
                raw_tokens=[],
                source_path="widget.lst",
                source_line=10,
            )
            hits = AUD._resolve_record_path_for_key(tmp, "some_book", "spell", "Widget")
            self.assertEqual(hits, [])


class AuditUnitsIdentityBoundaryTest(unittest.TestCase):
    """Exercises `audit_units` directly with injected indices, so a
    deliberately-desynced fallback index (the exact shape a real defect
    takes: the dict says "match", the corpus on disk says otherwise) can be
    proven caught -- and proven to have been catchable, by first showing the
    genuine-match case reports zero mismatches."""

    def _corpus_with_one_record(self, tmp):
        _write_record(
            os.path.join(tmp, "some_book", "spell", "widget.json"),
            key="Widget",
            raw_tokens=[{"key": "DEFINE", "value": "Foo|0"}],
            source_path="widget_overlay.lst",  # deliberately NOT the unit's own citation
            source_line=99,
        )

    def test_a_genuine_key_index_fallback_reports_zero_mismatches(self):
        with tempfile.TemporaryDirectory() as tmp:
            self._corpus_with_one_record(tmp)
            unit = _unit("some_book:spell:widget", "spell", "some_book", "widget_base.lst", 5, "Widget")
            corpus_index = {}  # primary join deliberately misses
            key_index = {("some_book", "spell", "Widget"): [{"key": "DEFINE", "value": "Foo|0"}]}
            result = AUD.audit_units([unit], tmp, corpus_index, key_index, {})
            self.assertEqual(result["tier_counts"]["key_index"], 1)
            self.assertEqual(result["mismatches"], [])

    def test_a_key_index_entry_with_no_supporting_on_disk_record_is_a_mismatch(self):
        """Simulates the failure mode this row exists to guard against: the
        in-memory `key_index` claims a hit for a (book, kind, key) that no
        real corpus record actually backs (a stale/desynced index -- the
        same shape a wrong-record answer would take). MUST be caught."""
        with tempfile.TemporaryDirectory() as tmp:
            self._corpus_with_one_record(tmp)  # real record is (some_book, spell, "Widget")
            unit = _unit("some_book:spell:phantom", "spell", "some_book", "phantom_base.lst", 5, "Phantom")
            corpus_index = {}
            # A desynced index: claims "Phantom" resolves, but no on-disk
            # record for (some_book, spell, "Phantom") exists anywhere.
            key_index = {("some_book", "spell", "Phantom"): [{"key": "DEFINE", "value": "Foo|0"}]}
            result = AUD.audit_units([unit], tmp, corpus_index, key_index, {})
            self.assertEqual(result["tier_counts"]["key_index"], 1)
            self.assertEqual(len(result["mismatches"]), 1)
            self.assertEqual(result["mismatches"][0]["id"], "some_book:spell:phantom")

    def test_a_cross_book_entry_with_no_supporting_on_disk_record_is_a_mismatch(self):
        with tempfile.TemporaryDirectory() as tmp:
            self._corpus_with_one_record(tmp)
            unit = _unit("other_book:spell:phantom", "spell", "other_book", "phantom_base.lst", 5, "Phantom")
            corpus_index = {}
            key_index = {}
            cross_book_key_index = {("spell", "Phantom"): ("some_book", [{"key": "DEFINE", "value": "Foo|0"}])}
            result = AUD.audit_units([unit], tmp, corpus_index, key_index, cross_book_key_index)
            self.assertEqual(result["tier_counts"]["cross_book"], 1)
            self.assertEqual(len(result["mismatches"]), 1)

    def test_a_genuine_cross_book_fallback_reports_zero_mismatches(self):
        with tempfile.TemporaryDirectory() as tmp:
            self._corpus_with_one_record(tmp)
            unit = _unit("other_book:spell:widget", "spell", "other_book", "widget_base.lst", 5, "Widget")
            corpus_index = {}
            key_index = {}
            cross_book_key_index = {("spell", "Widget"): ("some_book", [{"key": "DEFINE", "value": "Foo|0"}])}
            result = AUD.audit_units([unit], tmp, corpus_index, key_index, cross_book_key_index)
            self.assertEqual(result["tier_counts"]["cross_book"], 1)
            self.assertEqual(result["mismatches"], [])

    def test_primary_hit_never_consults_a_fallback_index_at_all(self):
        with tempfile.TemporaryDirectory() as tmp:
            unit = _unit("b:spell:widget", "spell", "b", "widget_base.lst", 5, "Widget")
            corpus_index = {("b", "spell", "widget_base.lst", 5): [{"key": "DEFINE", "value": "Foo|0"}]}
            result = AUD.audit_units([unit], tmp, corpus_index, {}, {})
            self.assertEqual(result["tier_counts"]["primary"], 1)
            self.assertEqual(result["fallback_only"], 0)

    def test_no_record_when_no_tier_resolves(self):
        with tempfile.TemporaryDirectory() as tmp:
            unit = _unit("b:spell:ghost", "spell", "b", "ghost.lst", 5, "Ghost")
            result = AUD.audit_units([unit], tmp, {}, {}, {})
            self.assertEqual(result["tier_counts"]["no_record"], 1)
            self.assertEqual(result["mismatches"], [])


if __name__ == "__main__":
    unittest.main()
