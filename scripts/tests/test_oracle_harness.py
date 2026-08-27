#!/usr/bin/env python3
"""Tests for `scripts/oracle_harness/` (SD-33 Epic 2, AT-33-E2-003).

Proves the load-bearing claim AT-33-E2-003 makes: given a unit, the harness
returns `(ours, oracle, agree|disagree|unverifiable)`, and `unverifiable` is
a **first-class** return -- never an error silently folded into `agree`.

Fixture discipline (`epic-breakdown.md` AT-33-E2-003, `stc-authoring`): a
fixture's expected value is transcribed from bytes the harness's own read
path does NOT touch. Every literal `oracle=...` value below was read by eye
from the real, committed BatchExporter output
(`docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt`,
AT-33-E2-002) and typed as a Python literal here -- this test file never
opens that file at run time, and `oracle_export.parse_oracle_export` (the
harness's actual read path for that file, exercised separately below in
`OracleExportParsingTest`, deliberately using an *inline* string literal
rather than the committed file, for the identical reason) is never called
by the comparison tests in `CompareUnitTest`. The two are proven
independently so neither can mirror the other.
"""

import os
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
from oracle_harness import compare as OC  # noqa: E402
from oracle_harness import oracle_export as OE  # noqa: E402


class NormalizeNumericTest(unittest.TestCase):
    def test_plain_int_string(self):
        self.assertEqual(OC.normalize_numeric("12"), 12)

    def test_signed_positive_string(self):
        # PCGen's own export convention for mods/BAB/saves: "+3", "+1".
        self.assertEqual(OC.normalize_numeric("+3"), 3)

    def test_signed_negative_string(self):
        self.assertEqual(OC.normalize_numeric("-2"), -2)

    def test_int_passthrough(self):
        self.assertEqual(OC.normalize_numeric(12), 12)

    def test_non_numeric_returns_none(self):
        self.assertIsNone(OC.normalize_numeric("Fighter"))


class CompareUnitTest(unittest.TestCase):
    """Every `oracle=` literal below was hand-transcribed from the real,
    committed `pf1_fighter_l1.computed.txt` bytes (AT-33-E2-002) -- read by
    eye, not parsed by this test or by any code under test."""

    def test_agree_numeric(self):
        # Real committed export line: HP=12.
        result = OC.compare_unit("pf1_fighter_l1.HP", ours=12, oracle="12")
        self.assertEqual(result["verdict"], "agree")
        self.assertEqual(result["ours"], 12)
        self.assertEqual(result["oracle"], 12)

    def test_agree_signed_numeric(self):
        # Real committed export line: BAB=+1.
        result = OC.compare_unit("pf1_fighter_l1.BAB", ours=1, oracle="+1")
        self.assertEqual(result["verdict"], "agree")

    def test_agree_string(self):
        # Real committed export line: CLASS.0.NAME=Fighter.
        result = OC.compare_unit("pf1_fighter_l1.CLASS_NAME", ours="Fighter", oracle="Fighter")
        self.assertEqual(result["verdict"], "agree")

    def test_disagree_known_case(self):
        """A KNOWN-DISAGREEING case (`epic-breakdown.md` AT-33-E2-003: "a
        harness that has never returned disagree has not been shown to be
        capable of it"). `ours=2` is a synthetic wrong value standing in for
        an engine defect -- the real committed oracle value for this unit is
        BAB=+1 (same transcribed line as `test_agree_signed_numeric`); this
        case deliberately supplies a different `ours` to prove disagreement
        is actually detected, not just theoretically possible."""
        result = OC.compare_unit("pf1_fighter_l1.BAB", ours=2, oracle="+1")
        self.assertEqual(result["verdict"], "disagree")
        self.assertEqual(result["ours"], 2)
        self.assertEqual(result["oracle"], 1)

    def test_unverifiable_no_oracle_value(self):
        """`unverifiable` must be a first-class, non-error return -- never
        swallowed into `agree` -- when no oracle value exists for a unit at
        all (e.g. a token the export template never captured)."""
        result = OC.compare_unit("pf1_fighter_l1.SPELL_DC", ours=15, oracle=None)
        self.assertEqual(result["verdict"], "unverifiable")
        self.assertEqual(result["ours"], 15)
        self.assertIsNone(result["oracle"])

    def test_unverifiable_is_not_an_exception(self):
        """A missing oracle value must not raise -- 'unverifiable' is a
        return value, not an error path swallowed by a try/except into
        'agree'."""
        try:
            result = OC.compare_unit("some.unit", ours=1, oracle=None)
        except Exception as exc:  # pragma: no cover - failure path
            self.fail(f"compare_unit raised on a missing oracle value: {exc!r}")
        self.assertEqual(result["verdict"], "unverifiable")

    def test_result_shape_matches_box_ledger_contract(self):
        """`scripts/box_ledger.py::load_oracle_results` (AT-33-E1-002
        condition 3) expects records shaped `(unit_id, ours, oracle,
        verdict)`. Prove the harness's return dict actually carries all
        four keys box_ledger reads."""
        result = OC.compare_unit("pf1_fighter_l1.HP", ours=12, oracle="12")
        self.assertEqual(
            set(result.keys()), {"unit_id", "ours", "oracle", "verdict"}
        )
        self.assertEqual(result["unit_id"], "pf1_fighter_l1.HP")


class OracleExportParsingTest(unittest.TestCase):
    """Exercises the harness's actual read path (`parse_oracle_export`)
    against an inline string literal, not the committed export file --
    fixture discipline again: this class's expected values are typed here,
    not read back from what the function under test itself would parse."""

    def test_parses_key_equals_value_lines(self):
        text = "NAME=SD33 E2 Fighter\nHP=12\nBAB=+1\nCLASS.0.NAME=Fighter\n"
        parsed = OE.parse_oracle_export(text)
        self.assertEqual(parsed["HP"], "12")
        self.assertEqual(parsed["BAB"], "+1")
        self.assertEqual(parsed["CLASS.0.NAME"], "Fighter")
        self.assertEqual(parsed["NAME"], "SD33 E2 Fighter")

    def test_ignores_blank_and_comment_lines(self):
        text = "# a comment\n\nHP=12\n   \nAC.TOTAL=12\n"
        parsed = OE.parse_oracle_export(text)
        self.assertEqual(parsed, {"HP": "12", "AC.TOTAL": "12"})

    def test_missing_key_absent_not_empty_string(self):
        """A key the template never emitted must be absent from the parsed
        dict (so `compare_unit` sees `oracle=None` via `.get()`, not an
        empty string that would `normalize_numeric` to None anyway but for
        the wrong reason)."""
        parsed = OE.parse_oracle_export("HP=12\n")
        self.assertNotIn("SPELL_DC", parsed)


class RunComparisonTest(unittest.TestCase):
    """End-to-end: given an oracle export blob and an `ours` mapping,
    `run_comparison` returns one record per requested unit in the exact
    shape `box_ledger.load_oracle_results` consumes, including at least one
    of each of the three verdicts in a single run."""

    def test_all_three_verdicts_in_one_run(self):
        oracle_text = "HP=12\nBAB=+1\nAC.TOTAL=12\n"
        ours = {
            "pf1_fighter_l1.HP": ("HP", 12),        # agree
            "pf1_fighter_l1.BAB": ("BAB", 2),        # disagree (real oracle is +1)
            "pf1_fighter_l1.SPELL_DC": ("SPELL_DC", 15),  # unverifiable: no such key
        }
        records = OC.run_comparison(ours, oracle_text)
        by_id = {r["unit_id"]: r for r in records}
        self.assertEqual(by_id["pf1_fighter_l1.HP"]["verdict"], "agree")
        self.assertEqual(by_id["pf1_fighter_l1.BAB"]["verdict"], "disagree")
        self.assertEqual(by_id["pf1_fighter_l1.SPELL_DC"]["verdict"], "unverifiable")
        # Every record carries the box_ledger.py contract shape.
        for r in records:
            self.assertEqual(set(r.keys()), {"unit_id", "ours", "oracle", "verdict"})


if __name__ == "__main__":
    unittest.main()
