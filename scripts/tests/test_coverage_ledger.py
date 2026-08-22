#!/usr/bin/env python3
"""Tests for `scripts/coverage_ledger.py` (SD-31 wave 30, lane 1).

Proves the load-bearing claim the tool exists to make: that "every not-done
unit belongs to a named group, or the tool says exactly which ones don't"
is not merely asserted in the script's docstring but actually holds --
including the negative case, per the wave brief's explicit instruction to
"prove the tool can fail: feed it a classification table with a deliberate
hole and confirm it reports the uncovered units rather than silently
passing."

Uses a small synthetic `docs/work-inventory.json`-shaped document rather
than the live 38k-unit corpus, so these tests stay fast and are not subject
to the live inventory's own drift across waves (`work-inventory.json` is
explicitly frozen this wave; these tests must not depend on its exact
current contents to pass or fail).
"""

import json
import os
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import coverage_ledger as CL  # noqa: E402


def _unit(id_, kind, book, status, wiring_class, **extra):
    u = {
        "id": id_,
        "kind": kind,
        "book": book,
        "status": status,
        "wiring_class": wiring_class,
        "name": extra.pop("name", id_.split(":")[-1]),
        "corpus_key": extra.pop("corpus_key", id_.split(":")[-1]),
        "type_facet": extra.pop("type_facet", ""),
        "source_file": extra.pop("source_file", "some_file.lst"),
        "evidence": extra.pop("evidence", None),
    }
    u.update(extra)
    return u


# Six units, chosen so every doneness_verdict branch used is unambiguous
# regardless of `kind`-based capping (see doneness_verdict's docstring):
#   not-started : status alone decides, before any wiring_class branch
#   unmeasurable: status == "unknown" decides, same reason
#   deferred    : status == "deferred-with-reason" decides, same reason
#   held        : static/derived + grounded
#   done        : static/derived + literal-verified (excluded from every
#                 population this tool builds -- included here only to
#                 prove DONE units are dropped before matching runs at all)
UNIT_NOT_STARTED_RACE = _unit("book_a:race:alpha", "race", "book_a", "not-started", "static")
UNIT_UNMEASURABLE_FEAT = _unit("book_a:feat:beta", "feat", "book_a", "unknown", "static")
UNIT_DEFERRED_CF = _unit("book_a:class_feature:gamma", "class_feature", "book_a", "deferred-with-reason", "derived")
UNIT_HELD_SPELL = _unit("book_b:spell:delta", "spell", "book_b", "grounded", "static")
UNIT_DONE_EQUIPMENT = _unit("book_b:equipment:epsilon", "equipment", "book_b", "literal-verified", "static")
UNIT_EXCLUDED_BOOK = _unit("beginner_box:feat:zeta", "feat", "beginner_box", "not-started", "static")

ALL_UNITS = [
    UNIT_NOT_STARTED_RACE,
    UNIT_UNMEASURABLE_FEAT,
    UNIT_DEFERRED_CF,
    UNIT_HELD_SPELL,
    UNIT_DONE_EQUIPMENT,
    UNIT_EXCLUDED_BOOK,
]


def _inventory(units):
    return {"units": units}


class NotDonePopulationTest(unittest.TestCase):
    def test_done_and_excluded_book_units_are_dropped(self):
        pop = CL.not_done_population(_inventory(ALL_UNITS))
        ids = {u["id"] for u in pop}
        self.assertEqual(
            ids,
            {
                UNIT_NOT_STARTED_RACE["id"],
                UNIT_UNMEASURABLE_FEAT["id"],
                UNIT_DEFERRED_CF["id"],
                UNIT_HELD_SPELL["id"],
            },
        )
        self.assertNotIn(UNIT_DONE_EQUIPMENT["id"], ids, "done units must never enter the population")
        self.assertNotIn(UNIT_EXCLUDED_BOOK["id"], ids, "EXCLUDED_BOOKS units must never enter the population")


class LoadClassificationTableValidationTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)

    def _write(self, doc):
        path = os.path.join(self._tmp.name, "groups.json")
        with open(path, "w", encoding="utf-8") as fh:
            json.dump(doc, fh)
        return path

    def test_valid_table_loads(self):
        path = self._write(
            {
                "groups": [
                    {"id": "G1", "label": "x", "todo_entry": "todo/sweeps.md#S1", "match": {"kind": ["race"]}}
                ]
            }
        )
        groups = CL.load_classification_table(path)
        self.assertEqual(len(groups), 1)

    def test_missing_groups_key_raises(self):
        path = self._write({})
        with self.assertRaises(CL.ClassificationTableError):
            CL.load_classification_table(path)

    def test_duplicate_group_id_raises(self):
        row = {"id": "G1", "label": "x", "todo_entry": "t", "match": {"kind": ["race"]}}
        path = self._write({"groups": [row, dict(row)]})
        with self.assertRaises(CL.ClassificationTableError):
            CL.load_classification_table(path)

    def test_unknown_match_key_raises(self):
        path = self._write(
            {"groups": [{"id": "G1", "label": "x", "todo_entry": "t", "match": {"not_a_real_field": ["x"]}}]}
        )
        with self.assertRaises(CL.ClassificationTableError):
            CL.load_classification_table(path)

    def test_missing_required_key_raises(self):
        path = self._write({"groups": [{"id": "G1", "label": "x", "match": {"kind": ["race"]}}]})
        with self.assertRaises(CL.ClassificationTableError):
            CL.load_classification_table(path)

    def test_bad_regex_raises(self):
        path = self._write(
            {"groups": [{"id": "G1", "label": "x", "todo_entry": "t", "match": {"id_regex": "(unclosed"}}]}
        )
        with self.assertRaises(CL.ClassificationTableError):
            CL.load_classification_table(path)


class BuildLedgerFullCoverageTest(unittest.TestCase):
    """The tool must report zero uncovered when the table genuinely covers
    the whole population -- proving it does not report false positives."""

    def test_full_coverage_reports_zero_uncovered(self):
        groups = [
            {
                "id": "G-RACE",
                "label": "race kind",
                "todo_entry": "todo/sweeps.md#S2",
                "match": {"kind": ["race"]},
            },
            {
                "id": "G-REST",
                "label": "everything else",
                "todo_entry": "todo/sweeps.md#S3",
                "match": {"verdict": ["unmeasurable", "deferred", "held"]},
            },
        ]
        units = CL.not_done_population(_inventory(ALL_UNITS))
        ledger = CL.build_ledger(units, groups)
        self.assertEqual(ledger["uncovered_count"], 0)
        self.assertEqual(ledger["uncovered"], [])
        self.assertEqual(ledger["covered_distinct"], ledger["population"])
        self.assertEqual(ledger["groups_without_todo_entry"], [])


class BuildLedgerDeliberateHoleTest(unittest.TestCase):
    """Direct answer to the wave brief's instruction: feed a classification
    table with a deliberate hole and confirm the tool reports the uncovered
    units by id, rather than silently passing."""

    def test_uncovered_units_named_exactly(self):
        # Only covers `race` -- the other three not-done units in ALL_UNITS
        # (feat/unmeasurable, class_feature/deferred, spell/held) are a
        # deliberate hole.
        groups = [
            {
                "id": "G-RACE-ONLY",
                "label": "race kind only (deliberate hole for the rest)",
                "todo_entry": "todo/sweeps.md#S2",
                "match": {"kind": ["race"]},
            }
        ]
        units = CL.not_done_population(_inventory(ALL_UNITS))
        ledger = CL.build_ledger(units, groups)
        self.assertEqual(ledger["uncovered_count"], 3)
        self.assertEqual(
            set(ledger["uncovered"]),
            {UNIT_UNMEASURABLE_FEAT["id"], UNIT_DEFERRED_CF["id"], UNIT_HELD_SPELL["id"]},
        )
        # The tool must NAME them, not just count them.
        self.assertTrue(all(isinstance(x, str) for x in ledger["uncovered"]))


class OverlapTest(unittest.TestCase):
    def test_unit_matched_by_two_groups_is_reported_as_overlap(self):
        groups = [
            {"id": "G-A", "label": "a", "todo_entry": "t1", "match": {"kind": ["race"]}},
            {"id": "G-B", "label": "b", "todo_entry": "t2", "match": {"book": ["book_a"]}},
        ]
        units = CL.not_done_population(_inventory(ALL_UNITS))
        ledger = CL.build_ledger(units, groups)
        # UNIT_NOT_STARTED_RACE is book_a AND kind race -> matches both.
        overlap_ids = {o["id"] for o in ledger["overlap"]}
        self.assertIn(UNIT_NOT_STARTED_RACE["id"], overlap_ids)
        self.assertEqual(ledger["overlap_count"], 1)
        row = next(r for r in ledger["rows"] if r["id"] == UNIT_NOT_STARTED_RACE["id"])
        self.assertEqual(set(row["groups"]), {"G-A", "G-B"})


class EmptyMatchFailsClosedTest(unittest.TestCase):
    def test_group_with_empty_match_matches_nothing(self):
        groups = [{"id": "G-EMPTY", "label": "e", "todo_entry": "t", "match": {}}]
        units = CL.not_done_population(_inventory(ALL_UNITS))
        ledger = CL.build_ledger(units, groups)
        self.assertEqual(ledger["group_rollup"][0]["count"], 0)
        self.assertEqual(ledger["uncovered_count"], len(units))


class RegexAndUnitIdsMatchTest(unittest.TestCase):
    def test_id_regex_matches(self):
        groups = [
            {"id": "G-RX", "label": "r", "todo_entry": "t", "match": {"id_regex": r":class_feature:"}}
        ]
        units = CL.not_done_population(_inventory(ALL_UNITS))
        ledger = CL.build_ledger(units, groups)
        matched = {r["id"] for r in ledger["rows"] if "G-RX" in r["groups"]}
        self.assertEqual(matched, {UNIT_DEFERRED_CF["id"]})

    def test_unit_ids_explicit_list_matches(self):
        groups = [
            {
                "id": "G-EXPLICIT",
                "label": "e",
                "todo_entry": "t",
                "match": {"unit_ids": [UNIT_HELD_SPELL["id"]]},
            }
        ]
        units = CL.not_done_population(_inventory(ALL_UNITS))
        ledger = CL.build_ledger(units, groups)
        matched = {r["id"] for r in ledger["rows"] if "G-EXPLICIT" in r["groups"]}
        self.assertEqual(matched, {UNIT_HELD_SPELL["id"]})


class GroupsWithoutTodoEntryTest(unittest.TestCase):
    def test_empty_todo_entry_is_flagged(self):
        groups = [{"id": "G-NOTE", "label": "n", "todo_entry": "", "match": {"kind": ["race"]}}]
        units = CL.not_done_population(_inventory(ALL_UNITS))
        ledger = CL.build_ledger(units, groups)
        self.assertEqual(ledger["groups_without_todo_entry"], ["G-NOTE"])
        self.assertFalse(ledger["group_rollup"][0]["has_todo_entry"])


class CliStrictExitCodeTest(unittest.TestCase):
    """Proves the tool can actually FAIL a build (`--strict`), not just log
    a warning nobody reads."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.inv_path = os.path.join(self._tmp.name, "work-inventory.json")
        with open(self.inv_path, "w", encoding="utf-8") as fh:
            json.dump(_inventory(ALL_UNITS), fh)

    def _groups_path(self, doc):
        path = os.path.join(self._tmp.name, "groups.json")
        with open(path, "w", encoding="utf-8") as fh:
            json.dump(doc, fh)
        return path

    def test_strict_passes_with_full_coverage_and_todo_entries(self):
        groups_path = self._groups_path(
            {
                "groups": [
                    {"id": "G-RACE", "label": "r", "todo_entry": "t1", "match": {"kind": ["race"]}},
                    {
                        "id": "G-REST",
                        "label": "rest",
                        "todo_entry": "t2",
                        "match": {"verdict": ["unmeasurable", "deferred", "held"]},
                    },
                ]
            }
        )
        rc = CL.main(["--inventory", self.inv_path, "--groups", groups_path, "--strict"])
        self.assertEqual(rc, 0)

    def test_strict_fails_on_uncovered_units(self):
        groups_path = self._groups_path(
            {"groups": [{"id": "G-RACE", "label": "r", "todo_entry": "t1", "match": {"kind": ["race"]}}]}
        )
        rc = CL.main(["--inventory", self.inv_path, "--groups", groups_path, "--strict"])
        self.assertEqual(rc, 1)

    def test_strict_fails_on_missing_todo_entry_even_with_full_coverage(self):
        groups_path = self._groups_path(
            {
                "groups": [
                    {"id": "G-RACE", "label": "r", "todo_entry": "", "match": {"kind": ["race"]}},
                    {
                        "id": "G-REST",
                        "label": "rest",
                        "todo_entry": "t2",
                        "match": {"verdict": ["unmeasurable", "deferred", "held"]},
                    },
                ]
            }
        )
        rc = CL.main(["--inventory", self.inv_path, "--groups", groups_path, "--strict"])
        self.assertEqual(rc, 1)

    def test_non_strict_still_exits_zero_with_uncovered_units(self):
        groups_path = self._groups_path(
            {"groups": [{"id": "G-RACE", "label": "r", "todo_entry": "t1", "match": {"kind": ["race"]}}]}
        )
        rc = CL.main(["--inventory", self.inv_path, "--groups", groups_path])
        self.assertEqual(rc, 0)

    def test_out_and_uncovered_out_files_are_written(self):
        groups_path = self._groups_path(
            {"groups": [{"id": "G-RACE", "label": "r", "todo_entry": "t1", "match": {"kind": ["race"]}}]}
        )
        out_path = os.path.join(self._tmp.name, "ledger.json")
        uncovered_path = os.path.join(self._tmp.name, "uncovered.json")
        rc = CL.main(
            [
                "--inventory",
                self.inv_path,
                "--groups",
                groups_path,
                "--out",
                out_path,
                "--uncovered-out",
                uncovered_path,
            ]
        )
        self.assertEqual(rc, 0)
        with open(out_path, encoding="utf-8") as fh:
            ledger = json.load(fh)
        self.assertIn("rows", ledger)
        self.assertIn("group_rollup", ledger)
        with open(uncovered_path, encoding="utf-8") as fh:
            uncovered = json.load(fh)
        self.assertEqual(
            set(uncovered),
            {UNIT_UNMEASURABLE_FEAT["id"], UNIT_DEFERRED_CF["id"], UNIT_HELD_SPELL["id"]},
        )


class EvidenceVisibleOriginMatchTest(unittest.TestCase):
    """Wave 30 integration: the wave-30 adversarial review fed this tool a
    real lane-6 group keyed on `evidence` and got
    `ClassificationTableError: unknown key(s) ['evidence']` -- 4 of that
    lane's 6 corpus-wide populations (keyed on `evidence`, `visible`, or
    `origin`) could not be expressed as a coverage group at all. These three
    tests prove the fix: each of the three newly-supported keys can express
    a real group, on the same units used throughout this file."""

    def setUp(self):
        self.units = [
            _unit(
                "book_a:race:alpha",
                "race",
                "book_a",
                "not-started",
                "static",
                evidence="no_compiled_rule_set_for_book",
                visible=True,
                origin="native",
            ),
            _unit(
                "book_a:feat:beta",
                "feat",
                "book_a",
                "unknown",
                "static",
                evidence="feat_key_absent_from_catalog",
                visible=False,
                origin="copy",
            ),
        ]
        self.inv = _inventory(self.units)

    def test_evidence_regex_matches(self):
        groups = CL.load_classification_table(
            _write_groups_file(
                self,
                {
                    "groups": [
                        {
                            "id": "G1",
                            "label": "book gate",
                            "todo_entry": "todo/levers.md#L10",
                            "match": {"evidence_regex": "^no_compiled_rule_set_for_book$"},
                        }
                    ]
                },
            )
        )
        units = CL.not_done_population(self.inv)
        ledger = CL.build_ledger(units, groups)
        self.assertEqual(ledger["group_rollup"][0]["count"], 1)
        self.assertEqual(ledger["uncovered_count"], 1)

    def test_visible_list_matches(self):
        groups = CL.load_classification_table(
            _write_groups_file(
                self,
                {
                    "groups": [
                        {
                            "id": "G1",
                            "label": "visible false",
                            "todo_entry": "todo/blocked.md#B8",
                            "match": {"visible": [False]},
                        }
                    ]
                },
            )
        )
        units = CL.not_done_population(self.inv)
        ledger = CL.build_ledger(units, groups)
        self.assertEqual(ledger["group_rollup"][0]["count"], 1)
        self.assertEqual(ledger["uncovered_count"], 1)

    def test_origin_list_matches(self):
        groups = CL.load_classification_table(
            _write_groups_file(
                self,
                {
                    "groups": [
                        {
                            "id": "G1",
                            "label": "copy origin",
                            "todo_entry": "todo/sweeps.md#S14",
                            "match": {"origin": ["copy"]},
                        }
                    ]
                },
            )
        )
        units = CL.not_done_population(self.inv)
        ledger = CL.build_ledger(units, groups)
        self.assertEqual(ledger["group_rollup"][0]["count"], 1)
        self.assertEqual(ledger["uncovered_count"], 1)


def _write_groups_file(testcase, doc):
    tmp = tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False)
    json.dump(doc, tmp)
    tmp.close()
    testcase.addCleanup(os.unlink, tmp.name)
    return tmp.name


if __name__ == "__main__":
    unittest.main()
