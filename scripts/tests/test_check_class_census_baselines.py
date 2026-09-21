#!/usr/bin/env python3
"""Regression test for `scripts/check_class_census_baselines.py`.

F0-check finding 6 (RED first): a synthetic census document with each of
the four `BASELINE_CENSUS_*` counts dropped by one, one at a time, must
make the gate go red BY NAME (not just non-zero exit) -- this is the
negative-control proof finding 6 asked for: "the current stage has no
negative-control proof that it fails on a drop."
"""

import os
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import check_class_census_baselines as C  # noqa: E402

BASELINES = {
    "BASELINE_CENSUS_IDS": 135,
    "BASELINE_CENSUS_COMPUTED": 42,
    "BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED": 74,
    "BASELINE_CENSUS_MIX_COMPUTED": 185,
}


def make_doc(**overrides):
    doc = {
        "ids": 135,
        "computed": 42,
        "blocked": 19,
        "prestige_alone_blocked": 74,
        "mix_panel_computed": 185,
    }
    doc.update(overrides)
    return doc


class CheckTests(unittest.TestCase):
    def test_every_baseline_exactly_met_passes_with_no_failures(self):
        self.assertEqual(C.check(make_doc(), BASELINES), [])

    def test_every_baseline_exceeded_passes_with_no_failures(self):
        doc = make_doc(ids=136, computed=43, prestige_alone_blocked=75, mix_panel_computed=186)
        self.assertEqual(C.check(doc, BASELINES), [])

    def test_ids_drop_by_one_fails_by_name(self):
        failures = C.check(make_doc(ids=134), BASELINES)
        self.assertEqual(len(failures), 1)
        self.assertIn("ids 134 below baseline 135", failures[0])
        self.assertIn("BASELINE_CENSUS_IDS", failures[0])

    def test_computed_drop_by_one_fails_by_name(self):
        failures = C.check(make_doc(computed=41), BASELINES)
        self.assertEqual(len(failures), 1)
        self.assertIn("computed 41 below baseline 42", failures[0])
        self.assertIn("BASELINE_CENSUS_COMPUTED", failures[0])

    def test_prestige_alone_blocked_drop_by_one_fails_by_name(self):
        # The exact regression finding 6 named: this baseline had no
        # consumer at all before the fix, so 74 -> 0 would not fail the
        # stage. Even a drop by one must now be caught.
        failures = C.check(make_doc(prestige_alone_blocked=73), BASELINES)
        self.assertEqual(len(failures), 1)
        self.assertIn("prestige_alone_blocked 73 below baseline 74", failures[0])
        self.assertIn("BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED", failures[0])

    def test_prestige_alone_blocked_dropping_to_zero_fails(self):
        failures = C.check(make_doc(prestige_alone_blocked=0), BASELINES)
        self.assertEqual(len(failures), 1)
        self.assertIn("prestige_alone_blocked 0 below baseline 74", failures[0])

    def test_mix_panel_computed_drop_by_one_fails_by_name(self):
        # The other previously-unwired baseline (finding 6): 185 -> 0 would
        # not fail the stage before this fix.
        failures = C.check(make_doc(mix_panel_computed=184), BASELINES)
        self.assertEqual(len(failures), 1)
        self.assertIn("mix_panel_computed 184 below baseline 185", failures[0])
        self.assertIn("BASELINE_CENSUS_MIX_COMPUTED", failures[0])

    def test_mix_panel_computed_dropping_to_zero_fails(self):
        failures = C.check(make_doc(mix_panel_computed=0), BASELINES)
        self.assertEqual(len(failures), 1)
        self.assertIn("mix_panel_computed 0 below baseline 185", failures[0])

    def test_multiple_simultaneous_drops_report_one_line_each(self):
        doc = make_doc(computed=41, mix_panel_computed=184)
        failures = C.check(doc, BASELINES)
        self.assertEqual(len(failures), 2)
        joined = "\n".join(failures)
        self.assertIn("computed 41", joined)
        self.assertIn("mix_panel_computed 184", joined)

    def test_missing_required_field_raises_keyerror_not_silently_defaulting(self):
        doc = make_doc()
        del doc["prestige_alone_blocked"]
        with self.assertRaises(KeyError):
            C.check(doc, BASELINES)


class MainCliTests(unittest.TestCase):
    def _write(self, tmp_path, doc):
        import json

        with open(tmp_path, "w", encoding="utf-8") as fh:
            json.dump(doc, fh)

    def test_main_exits_zero_and_prints_actuals_when_all_baselines_met(self):
        import tempfile

        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, "census.json")
            self._write(path, make_doc())
            rc = C.main([
                path,
                "--baseline-ids", "135",
                "--baseline-computed", "42",
                "--baseline-prestige-alone-blocked", "74",
                "--baseline-mix-computed", "185",
            ])
            self.assertEqual(rc, 0)

    def test_main_exits_one_when_prestige_alone_blocked_drops(self):
        import tempfile

        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, "census.json")
            self._write(path, make_doc(prestige_alone_blocked=0))
            rc = C.main([
                path,
                "--baseline-ids", "135",
                "--baseline-computed", "42",
                "--baseline-prestige-alone-blocked", "74",
                "--baseline-mix-computed", "185",
            ])
            self.assertEqual(rc, 1)

    def test_main_exits_one_on_unparseable_json(self):
        import tempfile

        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, "census.json")
            with open(path, "w", encoding="utf-8") as fh:
                fh.write("not json")
            rc = C.main([
                path,
                "--baseline-ids", "135",
                "--baseline-computed", "42",
                "--baseline-prestige-alone-blocked", "74",
                "--baseline-mix-computed", "185",
            ])
            self.assertEqual(rc, 1)


if __name__ == "__main__":
    unittest.main()
