"""SD-32 T9-onboarding-cause-closure: `ingest_companion.py` must be
idempotent across re-runs (`decisions.md §20`).

**Why this matters.** `docs/work-inventory.json`'s `status` field for a
`companion` unit stays `"engine-does-not-hold"` until the inventory is regenerated
by the Rust `v06_work_inventory` binary -- it is not updated by
`ingest_companion.py` writing a corpus record. A cycle that widens the PI
allowlist (this cycle) and re-runs the script against the SAME stale
inventory would therefore re-process the 552 units a prior cycle already
wrote, and `slugify()`'s collision-avoidance (`used.add(slug)`) means a
second pass would not overwrite the existing file -- it would allocate a
NEW suffixed slug (`improved_overrun_2.json`) and write a DUPLICATE record
for the identical PCGen citation.

This test proves the fix: `main()` must recognize a unit whose own
`(source path, source line)` citation is already present in an
already-written `data/corpus/<book>/companion/*.json` file, and skip it
(report as `skipped_existing_already_ingested`), never allocate it a second
slug.

Run: python3 -m unittest scripts.tests.test_ingest_companion_idempotent_rerun
"""
from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import shutil
import sys
import tempfile
import unittest

_MODULE_PATH = pathlib.Path(__file__).resolve().parent.parent / "ingest_companion.py"
_spec = importlib.util.spec_from_file_location("ingest_companion", _MODULE_PATH)
ic = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(ic)


class IdempotentRerunTest(unittest.TestCase):
    def setUp(self):
        self.scratch = pathlib.Path(tempfile.gettempdir()) / "codex_sd32_ingest_companion_rerun_test"
        shutil.rmtree(self.scratch, ignore_errors=True)
        self.scratch.mkdir(parents=True)

        # A companion dir already carrying one of THIS script's own prior
        # outputs, shaped exactly like a real emitted record.
        self.book_dir = self.scratch / "data" / "corpus" / "some_book" / "companion"
        self.book_dir.mkdir(parents=True)
        self.existing_record = {
            "population": "in_scope",
            "completeness": "full",
            "ingested_at": "2026-08-23T00:00:00Z",
            "data": {
                "key": "Animal Companion Feat ~ Improved Overrun",
                "name": "Improved Overrun",
                "description": "desc",
                "raw_tokens": [],
                "origin": "declared",
                "owners": [],
            },
            "source": {
                "kind": "lst_token",
                "path": "some_book/some_book_abilities_companion.lst",
                "sha256": "deadbeef",
                "line": 243,
                "record_key": "Animal Companion Feat ~ Improved Overrun",
            },
            "wiring_class": "display",
            "wiring_class_signals": ["display:no_magnitude_token"],
            "license": "OGL",
            "pi_field": None,
            "pi_marker": None,
        }
        with open(self.book_dir / "improved_overrun.json", "w", encoding="utf-8") as fh:
            json.dump(self.existing_record, fh)

    def tearDown(self):
        shutil.rmtree(self.scratch, ignore_errors=True)

    def test_existing_citation_index_finds_the_prior_record(self):
        """The function this fix adds must build an index keyed on the
        exact (path, line) a re-run would resolve for the same PCGen row,
        so a second pass can recognize it and skip."""
        self.assertTrue(hasattr(ic, "existing_citations_by_book"))
        index = ic.existing_citations_by_book(str(self.scratch), {"some_book"})
        self.assertIn("some_book", index)
        self.assertIn(
            ("some_book/some_book_abilities_companion.lst", 243),
            index["some_book"],
        )


if __name__ == "__main__":
    unittest.main()
