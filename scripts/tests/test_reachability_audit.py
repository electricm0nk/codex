"""
Self-test for `scripts/reachability_audit.py` (SD31-E0-F1, `decisions.md §4`).

WHY THIS EXISTS
----------------
`SD-30 state-goals-and-lessons.md §3.1`: this repo has shipped three gates
that could not fail, each caught only by running it against a known-answer
case. This test feeds the audit a FABRICATED dead-end -- a
`(wiring_class, status)` pair `_doneness_verdict_uncapped()` has no rule for
-- and confirms both that it lands in the audit's own `dead_end_cells` /
`unmapped_cells_with_units` output AND that the audit's CLI exits non-zero
for it. Companion tests confirm the clean case exits zero, that a
dead-ended `wiring_class` is reported but does not by itself fail the gate
(a capability gap is owned by an epic, not a defect in this script), and
that the corpus-live checks are stated as PROPERTIES rather than as pins on
a live figure -- no class carrying units may be stranded, and the ceiling
must agree with its own dead-end arithmetic -- and that the real corpus carries zero unmapped
cells today (the post-remediation state `test_pf1e_dashboard_producer.py`'s
`test_full_grid_yields_no_unmapped_cells` already proves for the producer's
own table -- this is the standing, corpus-live check that it stays that
way).

Run: python3 -m unittest scripts/tests/test_reachability_audit.py
Wired as the `reachability-audit-selftest` stage in `scripts/verify.sh`.

Prove-it-can-fail discipline (same as `test_pf1e_dashboard_producer.py`):
change `wiring_classes.add(wc)` / `status_vocab.add(st)` in `audit()` to NOT
widen the grid with observed-but-unlisted words, and
`test_fabricated_unmapped_cell_is_reported` goes red because the fabricated
cell is silently absent from the grid instead of raising.
"""
from __future__ import annotations

import collections
import importlib.util
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import unittest

# reachability_audit.py is always this test's sibling-of-a-sibling
# (scripts/reachability_audit.py), resolved relative to __file__ -- same
# convention test_pf1e_dashboard_producer.py uses to find its own subject.
_MODULE_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "reachability_audit.py"
)
_spec = importlib.util.spec_from_file_location("reachability_audit", _MODULE_PATH)
audit_mod = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(audit_mod)

_STANDARD_STATUS_VOCAB = {
    "grounded": "", "literal-verified": "", "fixture-verified": "",
    "ingested-magnitude": "", "text-complete": "", "deferred-with-reason": "",
    "engine-does-not-hold": "", "not-started": "", "unknown": "",
}


def _doc(units, status_vocabulary=None):
    return {
        "generated_at": "2026-08-15T00:00:00Z",
        "status_vocabulary": status_vocabulary or dict(_STANDARD_STATUS_VOCAB),
        "units": units,
    }


class FabricatedDeadEndTest(unittest.TestCase):
    """The audit must be proven able to fail before it is trusted."""

    def test_fabricated_unmapped_wiring_class_is_reported(self):
        # `not-started`/`engine-does-not-hold`/`unknown`/`deferred-with-reason` are
        # handled generically ahead of the per-wiring_class dispatch in
        # `_doneness_verdict_uncapped()` (they resolve for ANY wiring_class,
        # bogus or not) -- only an EVIDENTIARY status (`grounded`,
        # `text-complete`, `ingested-magnitude`, `literal-verified`,
        # `fixture-verified`) reaches the final `raise ValueError(f"doneness:
        # unknown wiring_class {wiring_class!r}")` for an unrecognised
        # wiring_class. `grounded` is the fabricated dead end here for
        # exactly that reason.
        units = [
            {"id": "fab-1", "book": "core_rulebook", "kind": "spell",
             "wiring_class": "quantum-entangled", "status": "grounded"},
        ]
        result = audit_mod.audit(_doc(units))
        unmapped = {d["cell"]: d for d in result["dead_end_cells"] if d["reason"] == "unmapped"}
        self.assertIn("quantum-entangled|grounded", unmapped)
        self.assertEqual(unmapped["quantum-entangled|grounded"]["unit_count"], 1)
        self.assertFalse(result["ok"], "a count>0 unmapped cell must fail the audit")
        self.assertEqual(len(result["unmapped_cells_with_units"]), 1)

    def test_fabricated_unmapped_wiring_class_at_a_status_the_generic_rules_catch_is_no_done_path_not_unmapped(self):
        # The mirror image of the case above: `not-started` for a bogus
        # wiring_class resolves via the generic top-level rule (NOT an
        # unmapped cell) -- but since NO evidentiary status can ever reach
        # `done` for this wiring_class either, the wiring_class as a whole
        # is still a `no-done-path` dead end, just not an `unmapped` one.
        units = [
            {"id": "fab-1b", "book": "core_rulebook", "kind": "spell",
             "wiring_class": "quantum-entangled", "status": "not-started"},
        ]
        result = audit_mod.audit(_doc(units))
        cells = {d["cell"]: d for d in result["dead_end_cells"]}
        self.assertEqual(cells["quantum-entangled|not-started"]["reason"], "no-done-path")
        self.assertEqual(result["unmapped_cells_with_units"], [])
        self.assertTrue(result["ok"])

    def test_fabricated_unmapped_status_is_reported(self):
        units = [
            {"id": "fab-2", "book": "core_rulebook", "kind": "spell",
             "wiring_class": "static", "status": "martian-verified"},
        ]
        result = audit_mod.audit(_doc(units))
        unmapped = {d["cell"]: d for d in result["dead_end_cells"] if d["reason"] == "unmapped"}
        self.assertIn("static|martian-verified", unmapped)
        self.assertFalse(result["ok"])

    def test_clean_document_passes(self):
        units = [
            {"id": "ok-1", "book": "core_rulebook", "kind": "spell",
             "wiring_class": "static", "status": "literal-verified"},
        ]
        result = audit_mod.audit(_doc(units))
        self.assertTrue(result["ok"], result["unmapped_cells_with_units"])
        self.assertEqual(result["unmapped_cells_with_units"], [])
        self.assertEqual(result["reachable_ceiling"], 1.0)

    def test_a_dead_ended_class_is_reported_but_does_not_fail_ok(self):
        """A `wiring_class` with no done-reaching status in the grid must show
        up as a `no-done-path` dead end and depress the reachable ceiling, but
        must NOT flip `ok` to False -- only an unmapped cell (an actual
        audit-table bug) does that.

        SYNTHETIC vocabulary, deliberately. `ambiguous` is used as the
        dead-ended class because it was one on the real corpus through SD-34,
        and `_STANDARD_STATUS_VOCAB` above omits AT-35-E2-003's
        `sheet-complete` rung, which is what gave it a done path in the live
        inventory. The point of this case is the MECHANISM, not the class:
        the live-corpus claim is made as a property by
        `test_real_inventory_strands_no_wiring_class_that_carries_units`."""
        units = [
            {"id": "amb-1", "book": "core_rulebook", "kind": "spell",
             "wiring_class": "ambiguous", "status": "grounded"},
            {"id": "amb-2", "book": "core_rulebook", "kind": "spell",
             "wiring_class": "static", "status": "literal-verified"},
        ]
        result = audit_mod.audit(_doc(units))
        cells = {d["cell"]: d for d in result["dead_end_cells"]}
        self.assertIn("ambiguous|grounded", cells)
        self.assertEqual(cells["ambiguous|grounded"]["reason"], "no-done-path")
        self.assertEqual(cells["ambiguous|grounded"]["unit_count"], 1)
        self.assertTrue(result["ok"])
        self.assertAlmostEqual(result["reachable_ceiling"], 0.5)
        self.assertAlmostEqual(result["reachable_ceiling_by_kind"]["spell"], 0.5)

    def test_cli_exits_nonzero_on_fabricated_unmapped_cell(self):
        with tempfile.TemporaryDirectory() as td:
            doc_path = os.path.join(td, "fab.json")
            with open(doc_path, "w", encoding="utf-8") as f:
                json.dump(_doc([
                    {"id": "fab-3", "book": "core_rulebook", "kind": "spell",
                     "wiring_class": "quantum-entangled", "status": "grounded"},
                ]), f)
            proc = subprocess.run(
                [sys.executable, str(_MODULE_PATH), "--inventory", doc_path],
                capture_output=True, text=True,
            )
            self.assertNotEqual(proc.returncode, 0, proc.stdout + proc.stderr)
            self.assertIn("quantum-entangled|grounded", proc.stdout)

    def test_cli_exits_zero_on_clean_document(self):
        with tempfile.TemporaryDirectory() as td:
            doc_path = os.path.join(td, "clean.json")
            with open(doc_path, "w", encoding="utf-8") as f:
                json.dump(_doc([
                    {"id": "ok-3", "book": "core_rulebook", "kind": "spell",
                     "wiring_class": "static", "status": "literal-verified"},
                ]), f)
            proc = subprocess.run(
                [sys.executable, str(_MODULE_PATH), "--inventory", doc_path],
                capture_output=True, text=True,
            )
            self.assertEqual(proc.returncode, 0, proc.stdout + proc.stderr)

    def test_cli_json_out_writes_the_same_result(self):
        with tempfile.TemporaryDirectory() as td:
            doc_path = os.path.join(td, "clean.json")
            json_out = os.path.join(td, "out.json")
            with open(doc_path, "w", encoding="utf-8") as f:
                json.dump(_doc([
                    {"id": "ok-4", "book": "core_rulebook", "kind": "spell",
                     "wiring_class": "computed", "status": "grounded"},
                ]), f)
            proc = subprocess.run(
                [sys.executable, str(_MODULE_PATH), "--inventory", doc_path,
                 "--json-out", json_out],
                capture_output=True, text=True,
            )
            self.assertEqual(proc.returncode, 0, proc.stdout + proc.stderr)
            with open(json_out, encoding="utf-8") as f:
                payload = json.load(f)
            self.assertTrue(payload["ok"])
            self.assertEqual(payload["reachable_ceiling"], 1.0)


class RealCorpusStandingGateTest(unittest.TestCase):
    """The standing-gate case: run against the real, live work-inventory.json."""

    def test_real_inventory_has_no_unmapped_cells(self):
        doc = audit_mod.load_inventory(audit_mod.DEFAULT_INVENTORY)
        result = audit_mod.audit(doc)
        self.assertEqual(
            result["unmapped_cells_with_units"], [],
            "the real corpus must not carry an unmapped (wiring_class, status) "
            "cell with on-board units -- those units would be absent from "
            "every rollup",
        )
        self.assertTrue(result["ok"])

    def test_real_inventory_strands_no_wiring_class_that_carries_units(self):
        """PROPERTY, not a value pin.

        The audit exists to catch a `wiring_class` whose units can never
        reach `done` at ANY status the corpus actually uses -- those units
        are stranded: no amount of work on them can move the rollup. The
        standing invariant is therefore that no dead-ended class carries
        on-board units, stated as a property so it holds however the classes
        are named and however many there are.

        WHY THIS IS NOT THE ASSERTION THAT USED TO BE HERE. Until SD-35
        Epic 2 this test pinned the literal set `{"ambiguous"}` -- SD-34's
        known capability gap. AT-35-E2-003's `sheet-complete` rung gave
        `ambiguous` a done-reaching status (the sheet rule: an unresolvable
        record renders as the rule's words and the unit is done), the set
        went empty, and the equality pin went red against a CORRECT engine.
        Re-pinning on today's empty set would simply re-arm the same trap in
        the other direction, so the assertion below is red only when a real
        dead end exists. Prove-it-can-fail: the fabricated-dead-end tests
        above exercise the same `no-done-path` code path on a synthetic doc.
        """
        doc = audit_mod.load_inventory(audit_mod.DEFAULT_INVENTORY)
        result = audit_mod.audit(doc)
        no_done = {d["wiring_class"] for d in result["dead_end_cells"]
                   if d["reason"] == "no-done-path"}
        # Count units the same way audit() does -- excluded books are out of
        # the grid, so they must be out of this denominator too.
        excluded = audit_mod.PRODUCER.EXCLUDED_BOOKS
        counts = collections.Counter(
            u.get("wiring_class") or "ambiguous"
            for u in (doc.get("units") or [])
            if (u.get("book") or "unknown") not in excluded
        )
        stranded = {wc: counts[wc] for wc in no_done if counts.get(wc)}
        self.assertEqual(
            stranded, {},
            "a wiring_class carrying on-board units has no status in the "
            "current grid that reaches `done` -- every unit in it is "
            "stranded and no cycle can close it; give the class a "
            "done-reaching status or reclassify the units",
        )

    def test_real_inventory_reachable_ceiling_agrees_with_its_own_dead_ends(self):
        """PROPERTY: the headline ceiling is exactly the arithmetic of the
        dead-end cells it is derived from, on the live corpus. A ceiling that
        drifts from its own dead-end total is the instrument lying, which is
        the failure mode `SD-30 state-goals-and-lessons.md §3.1` names."""
        doc = audit_mod.load_inventory(audit_mod.DEFAULT_INVENTORY)
        result = audit_mod.audit(doc)
        excluded = audit_mod.PRODUCER.EXCLUDED_BOOKS
        total = len([u for u in (doc.get("units") or [])
                     if (u.get("book") or "unknown") not in excluded])
        self.assertGreater(total, 0)
        self.assertEqual(
            result["reachable_ceiling"],
            round(1 - result["dead_end_unit_total"] / total, 6),
        )
        self.assertGreater(result["reachable_ceiling"], 0.0)
        self.assertLessEqual(result["reachable_ceiling"], 1.0)


if __name__ == "__main__":
    unittest.main()
