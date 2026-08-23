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
2. **A genuinely uncovered real object (AT-32-G3-001, decisions.md §14a).**
   `classify_unit()` in `scripts/shape_ledger.py` always returns a family
   id by construction (it falls through to F0/F8 rather than ever
   returning `None`), so on a real inventory `unclassified_count` can never
   organically go non-zero -- that structural universality is Gate 1's own
   design, not a loophole here.

   **`decisions.md` §14a is the finding of record that this class' prior
   version got this wrong**: it proved AT-32-G3-001 by `mock.patch`-ing
   `shape_ledger.build_ledger` to fabricate a row with `family: None`, a
   state no real object can ever produce -- 80 fabricated units pointing at
   a nonexistent corpus file returned `exit 0, PASS`. That is precisely the
   `decisions.md` §1a anti-gaming defect: a gate proven red only by
   patching the code under test reports safety it does not provide. The
   mock-based version of this test class has been deleted, not retained
   alongside a real one.

   The tests below prove the gate's **real** invariant --
   `no_record`'s share of the population must not exceed a committed
   budget (`decisions.md` §14b) -- goes red through the actual
   `run_gate`/`build_ledger`/`classify_unit`/`build_corpus_index` path,
   with **no mock, no monkeypatch, and no override of any function under
   test**. Real (if synthetic) units are fed at a real, unreachable corpus
   location, so the join organically produces `no_record` rows the same
   way a real corpus gap would.

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


class NoRecordBudgetInvariantTest(unittest.TestCase):
    """AT-32-G3-001, decisions.md §14a/§14b: the standing gate must be able
    to go red when an object appears that no shape covers. `no_record` is
    that invariant -- a unit whose join finds no corpus record at all is
    precisely an object no shape covers -- and every test below reaches it
    through the REAL `run_gate` -> `build_corpus_index` -> `build_ledger`
    -> `classify_unit` path, feeding real (if synthetic) units at a real,
    unreachable corpus location. No function under test is mocked or
    monkeypatched anywhere in this class -- that is precisely the defect
    `decisions.md §14a` records against this class' prior version."""

    def test_orchestrator_reproduction_now_fails(self):
        """The literal repro from `decisions.md §14a`/the reopening brief:
        80 real units across 8 kinds, all pointing at a nonexistent corpus
        file, real `corpus_root='/nonexistent'` (never patched). Before
        this fix: exit 0, PASS. After: the join organically produces 80
        `no_record` rows -- 100% of this run's population, which exceeds
        the committed baseline share -- and the gate must go red."""
        units = [
            _unit(f"b:{k}:{i}", k, "b", "not-started", "static", "totally_fake_file.lst", i)
            for k in ("ability", "skill", "template", "deity", "power", "domain", "language", "kit")
            for i in range(1, 11)
        ]
        status, report = G.run_gate({"units": units}, corpus_root="/nonexistent")

        self.assertNotEqual(status, 0, "80 genuinely no_record objects must now fail the gate")
        self.assertEqual(report["unclassified_count"], 0, "F0 still absorbs them -- that part is unchanged")
        self.assertEqual(report["no_record_count"], 80)
        self.assertTrue(report["no_record_budget_exceeded"])

    def test_real_no_record_unit_fails_a_tight_budget(self):
        """A single genuinely-uncovered real object, with the budget
        tightened to zero tolerance -- proves the invariant fires on the
        smallest possible real gap, not only a large synthetic one."""
        units = [_unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)]
        status, report = G.run_gate(
            {"units": units},
            corpus_root="/nonexistent",
            no_record_budget_count=0,
            no_record_budget_population=1,
        )
        self.assertNotEqual(status, 0)
        self.assertEqual(report["no_record_count"], 1)
        self.assertTrue(report["no_record_budget_exceeded"])

    def test_new_kind_no_record_object_fails_the_gate_too(self):
        """Card 15 (`decisions.md §12b`): the integration cycle widens
        `docs/work-inventory.json` with brand-new kinds (`ability`, `skill`,
        `template`, ...) that never existed when this gate was written.
        `classify_unit()` is kind-agnostic (it joins purely on
        `book`/`source_file`/`source_line`, never branching on `kind` --
        `shape_ledger.py`'s own `classify_unit` docstring), so this proves
        the real join organically catches a future object of a KIND THIS
        GATE HAS NEVER SEEN, not only a familiar `spell`/`feat` row."""
        units = [_unit("b:ability:x", "ability", "b", "not-started", "static", "f.lst", 1)]
        status, report = G.run_gate(
            {"units": units},
            corpus_root="/nonexistent",
            no_record_budget_count=0,
            no_record_budget_population=1,
        )
        self.assertNotEqual(status, 0)
        self.assertEqual(report["no_record_count"], 1)

    def test_no_record_within_budget_still_passes(self):
        """The inverse of the above: a real corpus with a matched record
        for every unit passes cleanly even though the budget mechanism now
        exists -- the gate does not become permanently red just because it
        gained a new check."""
        with tempfile.TemporaryDirectory() as tmp:
            corpus_root = os.path.join(tmp, "corpus")
            book_dir = os.path.join(corpus_root, "book_a", "spell")
            os.makedirs(book_dir)
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(
                    {
                        "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|2"}]},
                        "source": {"path": "book_a_spells.lst", "line": 1},
                    },
                    fh,
                )
            units = [_unit("book_a:spell:x", "spell", "book_a", "not-started", "static", "book_a_spells.lst", 1)]
            status, report = G.run_gate({"units": units}, corpus_root=corpus_root)
        self.assertEqual(status, 0)
        self.assertEqual(report["no_record_count"], 0)
        self.assertFalse(report["no_record_budget_exceeded"])

    def test_pile_mismatch_fails_the_gate(self):
        """Sum-the-piles: if the per-family rollup does not add back to the
        population, the gate must fail even when unclassified_count reads
        0 -- catching a `build_ledger` regression that drops rows silently.
        This exercises `evaluate_ledger` directly with a hand-built,
        deliberately-malformed ledger dict -- ordinary unit testing of a
        pure function's edge case, not a patch of any code under test
        (`shape_ledger.build_ledger` is never touched)."""
        ledger = {
            "population": 6,  # families below only sum to 1 -- deliberate mismatch
            "rows": [],
            "families": {"F1": {"label": "x", "proof_width": "x", "count": 1}},
            "unclassified_count": 0,
            "unclassified": [],
            "join_status_counts": {"matched": 1},
        }
        status, report = G.evaluate_ledger(ledger)
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
