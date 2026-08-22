#!/usr/bin/env python3
"""Self-test for `scripts/shape_coverage_standing_gate.py` (SD-32 Gate 3,
card `gate-3-closure-invariant`, AT-32-G3-001/002/003).

WHY THIS EXISTS
----------------
Decision 1a (carried from SD-31 decisions §50, restated for this bundle in
`decisions.md`): "A gate that cannot fail is worse than no gate." This test
proves the standing gate can actually go red in both of the two ways
AT-32-G3-001/002 require, not merely that it can print PASS on today's
inventory:

1. **Empty predicate (AT-32-G3-002).** An inventory with zero not-done
   units (or none at all) must fail closed -- "no coverage", nonzero exit
   -- exactly the discipline `scripts/coverage_ledger.py` and
   `scripts/shape_ledger.py` already carry, and the same discipline this
   gate must not regress on.
2. **A FABRICATED uncovered object (AT-32-G3-001).** `classify_unit()` in
   `scripts/shape_ledger.py` always returns a family id by construction (it
   falls through to F0/F8 rather than ever returning `None`), so on a real
   inventory `unclassified_count` can never organically go non-zero -- that
   structural universality is Gate 1's own design, not a loophole here.
   This test therefore does what `test_reachability_audit.py`'s own
   docstring calls "prove it can fail before it is trusted": it monkeypatches
   `shape_ledger.classify_unit` to emit one row with a `None` family (an
   object literally no shape covers) and confirms `run_gate()` reports it
   in `unclassified_count`, fails the piles-reconcile check, and returns a
   nonzero exit -- the mechanism the gate depends on to ever go red at all.

Run: python3 -m unittest scripts/tests/test_shape_coverage_standing_gate.py
Wired as the `shape-coverage-standing-gate-selftest` stage in
`scripts/verify.sh`.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_coverage_standing_gate as G  # noqa: E402
import shape_ledger as SL  # noqa: E402


def _unit(id_, kind, book, status, wiring_class, source_file, source_line):
    return {
        "id": id_,
        "kind": kind,
        "book": book,
        "status": status,
        "wiring_class": wiring_class,
        "source_file": source_file,
        "source_line": source_line,
    }


class FailClosedOnEmptyTest(unittest.TestCase):
    """AT-32-G3-002: the standing gate fails closed on an empty predicate."""

    def _run_cli(self, args, stdin_text=None):
        return subprocess.run(
            [sys.executable, os.path.join(REPO_ROOT, "scripts", "shape_coverage_standing_gate.py"), *args],
            input=stdin_text,
            capture_output=True,
            text=True,
        )

    def test_dev_null_reports_no_coverage_and_nonzero_exit(self):
        result = self._run_cli(["--inventory", "/dev/null"])
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no coverage", (result.stdout + result.stderr).lower())

    def test_empty_units_list_reports_no_coverage_and_nonzero_exit(self):
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
            json.dump({"units": []}, fh)
            path = fh.name
        try:
            result = self._run_cli(["--inventory", path])
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("no coverage", (result.stdout + result.stderr).lower())
        finally:
            os.unlink(path)

    def test_stdin_empty_object_reports_no_coverage(self):
        # This is the literal AT-32-G3-002 verification command shape:
        # `echo '{}' | python3 scripts/shape_coverage_standing_gate.py`.
        result = self._run_cli([], stdin_text="{}\n")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no coverage", (result.stdout + result.stderr).lower())

    def test_run_gate_function_empty_inventory_returns_nonzero(self):
        status, report = G.run_gate({"units": []}, corpus_root="/nonexistent")
        self.assertNotEqual(status, 0)
        self.assertIn("error", report)
        self.assertIn("no coverage", report["error"].lower())


class FabricatedUncoveredObjectTest(unittest.TestCase):
    """AT-32-G3-001: the standing gate must be able to go red when an
    object appears that no shape covers -- proven with a fabricated gap
    per `test_reachability_audit.py`'s "prove it can fail" discipline,
    since a real inventory can never organically exercise this path
    (shape_ledger.classify_unit always returns a family)."""

    def test_fabricated_unclassified_row_fails_the_gate(self):
        units = [
            _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1),
            _unit("b:spell:y", "spell", "b", "not-started", "static", "f.lst", 2),
        ]
        inventory = {"units": units}
        real_build_ledger = SL.build_ledger

        def fabricated_build_ledger(units_arg, corpus_index):
            ledger = real_build_ledger(units_arg, corpus_index)
            # Inject one row this classifier structurally could never
            # produce on its own: an object no named family covers.
            ledger["rows"].append({"id": "b:spell:fabricated-gap", "kind": "spell", "book": "b", "family": None, "join_status": "no_record"})
            ledger["population"] += 1
            ledger["unclassified_count"] += 1
            ledger["unclassified"].append("b:spell:fabricated-gap")
            return ledger

        with mock.patch.object(SL, "build_ledger", side_effect=fabricated_build_ledger):
            status, report = G.run_gate(inventory, corpus_root="/nonexistent")

        self.assertNotEqual(status, 0, "a fabricated uncovered object must fail the gate")
        self.assertEqual(report["unclassified_count"], 1)

    def test_pile_mismatch_fails_the_gate(self):
        """Sum-the-piles: if the per-family rollup does not add back to the
        population, the gate must fail even when unclassified_count reads
        0 -- catching a build_ledger regression that drops rows silently."""
        units = [_unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)]
        inventory = {"units": units}
        real_build_ledger = SL.build_ledger

        def mismatched_build_ledger(units_arg, corpus_index):
            ledger = real_build_ledger(units_arg, corpus_index)
            ledger["population"] += 5  # families no longer sum to population
            return ledger

        with mock.patch.object(SL, "build_ledger", side_effect=mismatched_build_ledger):
            status, report = G.run_gate(inventory, corpus_root="/nonexistent")

        self.assertNotEqual(status, 0, "a pile mismatch must fail the gate even with unclassified_count == 0")
        self.assertFalse(report["piles_reconcile"])


class RealCaseGreenTest(unittest.TestCase):
    """A clean case -- every unit classified, piles reconcile -- passes."""

    def test_clean_inventory_passes(self):
        with tempfile.TemporaryDirectory() as tmp:
            corpus_root = os.path.join(tmp, "corpus")
            book_dir = os.path.join(corpus_root, "book_a", "spell")
            os.makedirs(book_dir)
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(
                    {
                        "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WizardLVL/2"}]},
                        "source": {"path": "book_a_spells.lst", "line": 1},
                    },
                    fh,
                )
            inventory = {
                "units": [
                    _unit("book_a:spell:lvl", "spell", "book_a", "not-started", "static", "book_a_spells.lst", 1),
                ]
            }
            status, report = G.run_gate(inventory, corpus_root=corpus_root)
            self.assertEqual(status, 0)
            self.assertEqual(report["unclassified_count"], 0)
            self.assertTrue(report["piles_reconcile"])
            self.assertEqual(report["population"], 1)


class CorpusShaCitationTest(unittest.TestCase):
    """AT-32-G3-003: the report names the corpus SHA the count was
    re-derived against, read from scripts/pcgen-oracle-pin.env."""

    def test_read_oracle_sha_matches_pin_file(self):
        sha = G.read_oracle_sha()
        with open(os.path.join(REPO_ROOT, "scripts", "pcgen-oracle-pin.env")) as fh:
            pin_text = fh.read()
        self.assertIsNotNone(sha)
        self.assertIn(sha, pin_text)

    def test_report_carries_corpus_sha(self):
        units = [_unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)]
        status, report = G.run_gate({"units": units}, corpus_root="/nonexistent")
        self.assertIn("corpus_sha", report)
        self.assertTrue(report["corpus_sha"])


if __name__ == "__main__":
    unittest.main()
