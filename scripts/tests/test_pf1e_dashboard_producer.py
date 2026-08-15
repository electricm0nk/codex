"""
Self-test for the dashboard producer's doneness-verdict table
(`scripts/observer/pf1e_dashboard_producer.py`) -- launch-readiness
remediation Step 4D, blocker B6.

WHY THIS EXISTS
----------------
`doneness_verdict()` / `_doneness_verdict_uncapped()` raise `ValueError` on
any `(wiring_class, status)` pair they have no rule for -- by design, so a
new status word landing in the generator without a matching rule is a loud
crash, not a silent default. `compute_wiring_class_summary()` catches that
exception per-unit and records it in `doneness_unmapped` rather than letting
one novel unit kill the whole 5-minute cron tick -- but a table that quietly
grows an unmapped cell is still a real defect (blocker B6: the
`(ambiguous, literal-verified|fixture-verified)` cells were unmapped until
this remediation). This test grids over the FULL `WIRING_CLASS_VALUES x`
status-vocabulary cross product on a fabricated document -- not the real
corpus, which may not exercise every cell -- and asserts nothing lands in
`doneness_unmapped`.

Run: python3 -m unittest scripts/tests/test_pf1e_dashboard_producer.py
Wired as the `producer-selftest` stage in both `verify.sh` stage sets.

Prove-it-can-fail discipline (same as `test_fetch_pcgen_oracle.sh`): comment
out the `ambiguous` branch's `"literal-verified", "fixture-verified"` tuple
members in `_doneness_verdict_uncapped()` and re-run -- `test_full_grid_...`
and `test_ambiguous_literal_verified_is_held` both go red.
"""
from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import tempfile
import unittest

# producer.py is always this test's sibling-of-a-sibling
# (scripts/observer/pf1e_dashboard_producer.py); resolved relative to
# __file__, the same convention the producer itself uses to find
# observer.py, rather than an absolute path that would break on a
# differently-rooted checkout.
_PRODUCER_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "observer" / "pf1e_dashboard_producer.py"
)
_spec = importlib.util.spec_from_file_location("pf1e_dashboard_producer", _PRODUCER_PATH)
producer = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(producer)

# Kept in sync BY HAND with `STATUS_VOCABULARY` in `src/bin/v06_work_inventory.rs`
# -- the Rust generator's status words are the one authoritative list; this
# test grids over its own copy rather than reaching across the Rust/Python
# boundary at test time, so a NEW status word landing in the generator
# without a matching `doneness_verdict()` rule is caught here as an
# unmapped cell in the grid below, not silently skipped because the test
# never knew the word existed.
STATUS_WORDS = (
    "grounded",
    "literal-verified",
    "fixture-verified",
    "ingested-magnitude",
    "text-complete",
    "deferred-with-reason",
    "not-ingested",
    "not-started",
    "unknown",
)


def _fabricated_doc_path(tmpdir: str) -> str:
    """One unit per `(wiring_class, status)` cell -- 5 x 9 = 45 units.

    `kind="spell"`, deliberately: `spell` left `NO_GROUNDING_PROBE` (see the
    producer's own SD30-E0-F2 comment on `STATUS_LABEL`/`NO_GROUNDING_PROBE`,
    re-derived live 2026-08-14), so `doneness_verdict()`'s kind-cap never
    fires and cannot mask a raising cell behind a capped `held` -- this test
    wants to see the RAW `_doneness_verdict_uncapped()` table's coverage.
    """
    units = []
    for wc in producer.WIRING_CLASS_VALUES:
        for st in STATUS_WORDS:
            units.append({
                "id": f"fab:{wc}:{st}",
                "book": "core_rulebook",
                "kind": "spell",
                "wiring_class": wc,
                "status": st,
            })
    doc = {"generated_at": "2026-08-15T00:00:00Z", "units": units}
    doc_path = os.path.join(tmpdir, "fab-work-inventory.json")
    with open(doc_path, "w", encoding="utf-8") as f:
        json.dump(doc, f)
    return doc_path


class DonenessVerdictGridTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.doc_path = _fabricated_doc_path(self._tmp.name)
        # A dedicated scratch cache path -- never the real
        # WIRING_CLASS_CACHE -- so this test can neither read a stale real
        # cache nor pollute it with a fabricated one.
        self.cache_path = os.path.join(self._tmp.name, "fab-wiring-cache.json")

    def test_full_grid_yields_no_unmapped_cells(self):
        """The whole 5 wiring_class x 9 status grid (45 cells) must map to a
        real doneness verdict -- `doneness_unmapped` must come back empty."""
        summary = producer.compute_wiring_class_summary(
            doc_path=self.doc_path, cache_path=self.cache_path
        )
        self.assertTrue(summary.get("available"), summary.get("note"))
        self.assertEqual(
            summary.get("doneness_unmapped", "MISSING (field absent)"),
            {},
            "the full WIRING_CLASS_VALUES x status_vocabulary grid must map every "
            f"cell to a verdict; unmapped: {summary.get('doneness_unmapped')}",
        )
        # Sanity: 45 units in, 45 units accounted for across done+unmapped.
        total_doneness = sum(summary.get("doneness", {}).values())
        total_unmapped = sum(summary.get("doneness_unmapped", {}).values())
        self.assertEqual(total_doneness + total_unmapped, len(producer.WIRING_CLASS_VALUES) * len(STATUS_WORDS))

    def test_ambiguous_literal_verified_is_held(self):
        self.assertEqual(
            producer.doneness_verdict("ambiguous", "literal-verified", "spell"),
            producer.DONENESS_HELD,
        )

    def test_ambiguous_fixture_verified_is_held(self):
        self.assertEqual(
            producer.doneness_verdict("ambiguous", "fixture-verified", "spell"),
            producer.DONENESS_HELD,
        )

    def test_static_literal_verified_is_done(self):
        """Control: the ORIGINAL done rung (SD-32 decisions.md §2) still
        works -- `static` unambiguously reaches `done` on the same status
        word `ambiguous` only reaches `held` on, above."""
        self.assertEqual(
            producer.doneness_verdict("static", "literal-verified", "spell"),
            producer.DONENESS_DONE,
        )

    def test_unmapped_cell_raises(self):
        """Negative case: a status word with genuinely no rule anywhere
        still raises -- this table must not have quietly become total."""
        with self.assertRaises(ValueError):
            producer.doneness_verdict("ambiguous", "bogus-status-word", "spell")


if __name__ == "__main__":
    unittest.main()
