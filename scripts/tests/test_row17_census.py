#!/usr/bin/env python3
"""Tests for `scripts/row17_census.py` (`kanban.md` row 17,
`decisions.md §27`/§27a/§27b).

Uses a small synthetic `docs/work-inventory.json` + `data/corpus/` tree
(tempfile), never the live 34k-unit corpus — same discipline
`test_shape_ledger.py` and `test_shape_provisional_marker.py` use.

Proves:

1. `row17_honest_size` arithmetic (`fallthrough` + `provisional_default`
   within the not-done population) is exactly right on a hand-built
   fixture with a known answer.
2. The census can go RED: mutating a genuinely-derived unit's own corpus
   record to look defaulted (PI-redacted formula value) moves the count;
   reverting brings it back. This is the "prove your census can go red"
   requirement from the dispatch brief, run against the real on-disk
   corpus-walking code path (`build_corpus_index`), not just the
   in-memory `shape_ledger` unit tests.
3. `--check` exits 1 when a provisional-default marker is missing its
   required reason, and 0 otherwise.
"""

import json
import os
import subprocess
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import row17_census as RC  # noqa: E402
import shape_provisional_marker as SPM  # noqa: E402
import pi_scrub as PS  # noqa: E402


def _write_corpus_record(root, book, kind, name, data, source_file, source_line):
    d = os.path.join(root, book, kind)
    os.makedirs(d, exist_ok=True)
    rec = {
        "population": "in_scope",
        "completeness": "full",
        "data": data,
        "source": {"path": f"pathfinder/{book}/{source_file}", "line": source_line},
    }
    with open(os.path.join(d, f"{name}.json"), "w", encoding="utf-8") as fh:
        json.dump(rec, fh)


def _write_inventory(path, units):
    doc = {
        "generated_at": "2026-08-23T00:00:00Z",
        "generated_by": "test-fixture",
        "schema_version": 1,
        "units": units,
    }
    with open(path, "w", encoding="utf-8") as fh:
        json.dump(doc, fh)


class BuildCensusFixtureTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.corpus_root = os.path.join(self.tmp.name, "corpus")
        self.inventory_path = os.path.join(self.tmp.name, "work-inventory.json")

        # Five not-done units, one of each shape this census must count:
        #   1. genuinely derived (real family)
        #   2. measured_empty (record exists, zero formula tokens)
        #   3. measured_pi_redacted -- genuinely PI, correctly redacted;
        #      a REAL answer, T9-onboarding-cause-closure (2026-08-23,
        #      row 17's remaining 21) / decisions.md §27a -- NOT row 17's
        #      placeholder population.
        #   4. fallthrough -- a genuine, non-PI parse failure; still row
        #      17's real, actionable population.
        #   5. engine_does_not_hold (no corpus record at all)
        units = [
            {"id": "b:spell:derived", "kind": "spell", "book": "b", "status": "engine-does-not-hold",
             "wiring_class": "static", "source_file": "real.lst", "source_line": 1},
            {"id": "b:spell:empty", "kind": "spell", "book": "b", "status": "engine-does-not-hold",
             "wiring_class": "static", "source_file": "empty.lst", "source_line": 1},
            {"id": "b:trait:redacted", "kind": "trait", "book": "b", "status": "engine-does-not-hold",
             "wiring_class": "static", "source_file": "red.lst", "source_line": 1},
            {"id": "b:feat:malformed", "kind": "feat", "book": "b", "status": "engine-does-not-hold",
             "wiring_class": "static", "source_file": "bad.lst", "source_line": 1},
            {"id": "b:spell:missing", "kind": "spell", "book": "b", "status": "engine-does-not-hold",
             "wiring_class": "static", "source_file": "missing.lst", "source_line": 1},
        ]
        _write_inventory(self.inventory_path, units)

        _write_corpus_record(
            self.corpus_root, "b", "spell", "derived",
            {"key": "Real Spell", "raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|2"}]},
            "real.lst", 1,
        )
        _write_corpus_record(
            self.corpus_root, "b", "spell", "empty",
            {"key": "Empty Spell", "raw_tokens": []},
            "empty.lst", 1,
        )
        _write_corpus_record(
            self.corpus_root, "b", "trait", "redacted",
            {"key": "Codex-Named Unit", "raw_tokens": [{"key": "BONUS", "value": PS.REDACTED_PI_MARKER}]},
            "red.lst", 1,
        )
        _write_corpus_record(
            self.corpus_root, "b", "feat", "malformed",
            {"key": "Malformed Feat", "raw_tokens": [{"key": "DEFINE", "value": "OnlyOneField"}]},
            "bad.lst", 1,
        )
        # `missing.lst` unit deliberately has NO corpus record at all.

    def test_honest_size_arithmetic_on_known_fixture(self):
        census = RC.build_census(self.inventory_path, self.corpus_root)
        t = census["totals"]
        self.assertEqual(t["population"], 5)
        self.assertEqual(t["derived"], 1)
        self.assertEqual(t["measured_empty"], 1)
        self.assertEqual(t["measured_pi_redacted"], 1)
        self.assertEqual(t["fallthrough"], 1)
        self.assertEqual(t["fallthrough_pi_redacted"], 0)
        self.assertEqual(t["engine_does_not_hold"], 1)
        self.assertEqual(t["provisional_default_in_not_done_population"], 0)
        self.assertEqual(t["row17_honest_size"], 1)  # just the (non-PI) fallthrough unit

    def test_census_goes_red_on_mutation_and_green_on_revert(self):
        """Prove-RED requirement: mutate a genuinely-derived unit's own
        on-disk corpus record so it looks like a genuine (non-PI) parse
        failure, confirm the honest size count moves, then revert and
        confirm it moves back.

        Deliberately mutates to a MALFORMED token, not the PI-redaction
        marker: T9-onboarding-cause-closure (2026-08-23, row 17's remaining
        21) moved PI-redacted values OUT of `row17_honest_size` (they are a
        real answer, `measured_pi_redacted`) -- so a PI-redaction mutation
        would no longer move this count, and that is `test_pi_redacted_
        mutation_moves_measured_pi_redacted_not_honest_size` below's job to
        prove instead."""
        before = RC.build_census(self.inventory_path, self.corpus_root)
        self.assertEqual(before["totals"]["row17_honest_size"], 1)
        self.assertEqual(before["totals"]["derived"], 1)

        derived_path = os.path.join(self.corpus_root, "b", "spell", "derived.json")
        with open(derived_path) as fh:
            original = json.load(fh)
        mutated = json.loads(json.dumps(original))
        mutated["data"]["raw_tokens"] = [{"key": "DEFINE", "value": "OnlyOneField"}]
        with open(derived_path, "w", encoding="utf-8") as fh:
            json.dump(mutated, fh)

        try:
            during = RC.build_census(self.inventory_path, self.corpus_root)
            self.assertEqual(during["totals"]["row17_honest_size"], 2)  # RED: moved
            self.assertEqual(during["totals"]["derived"], 0)
        finally:
            with open(derived_path, "w", encoding="utf-8") as fh:
                json.dump(original, fh)

        after = RC.build_census(self.inventory_path, self.corpus_root)
        self.assertEqual(after["totals"]["row17_honest_size"], 1)  # GREEN: reverted
        self.assertEqual(after["totals"]["derived"], 1)

    def test_pi_redacted_mutation_moves_measured_pi_redacted_not_honest_size(self):
        """Companion mutation proof: mutating a genuinely-derived unit to
        the PI-redaction marker moves `measured_pi_redacted`, and leaves
        `row17_honest_size` UNCHANGED -- proving the T9-onboarding-cause-
        closure classification fix is load-bearing, not a static label."""
        before = RC.build_census(self.inventory_path, self.corpus_root)
        self.assertEqual(before["totals"]["measured_pi_redacted"], 1)
        self.assertEqual(before["totals"]["row17_honest_size"], 1)

        derived_path = os.path.join(self.corpus_root, "b", "spell", "derived.json")
        with open(derived_path) as fh:
            original = json.load(fh)
        mutated = json.loads(json.dumps(original))
        mutated["data"]["raw_tokens"] = [{"key": "BONUS", "value": PS.REDACTED_PI_MARKER}]
        with open(derived_path, "w", encoding="utf-8") as fh:
            json.dump(mutated, fh)

        try:
            during = RC.build_census(self.inventory_path, self.corpus_root)
            self.assertEqual(during["totals"]["measured_pi_redacted"], 2)  # moved
            self.assertEqual(during["totals"]["row17_honest_size"], 1)  # UNCHANGED
            self.assertEqual(during["totals"]["derived"], 0)
        finally:
            with open(derived_path, "w", encoding="utf-8") as fh:
                json.dump(original, fh)

        after = RC.build_census(self.inventory_path, self.corpus_root)
        self.assertEqual(after["totals"]["measured_pi_redacted"], 1)  # reverted
        self.assertEqual(after["totals"]["derived"], 1)

    def test_provisional_default_marker_counted_in_population(self):
        _write_corpus_record(
            self.corpus_root, "b", "trait", "provisional",
            {
                "key": "Bare Delivery Trait",
                "raw_tokens": [{"key": "TYPE", "value": "SpellLike"}],
                SPM.PROVISIONAL_DEFAULT_FIELD: True,
                SPM.PROVISIONAL_DEFAULT_REASON_FIELD: "delivery-only TYPE, no facet segment",
            },
            "prov.lst", 1,
        )
        with open(self.inventory_path) as fh:
            doc = json.load(fh)
        doc["units"].append({
            "id": "b:trait:provisional", "kind": "trait", "book": "b", "status": "engine-does-not-hold",
            "wiring_class": "static", "source_file": "prov.lst", "source_line": 1,
        })
        _write_inventory(self.inventory_path, doc["units"])

        census = RC.build_census(self.inventory_path, self.corpus_root)
        t = census["totals"]
        self.assertEqual(t["provisional_default_in_not_done_population"], 1)
        self.assertEqual(t["row17_honest_size"], 2)  # 1 fallthrough + 1 provisional

    def test_check_flag_fails_on_missing_reason(self):
        _write_corpus_record(
            self.corpus_root, "b", "trait", "malformed",
            {"key": "Malformed", "raw_tokens": [], SPM.PROVISIONAL_DEFAULT_FIELD: True},
            "mal.lst", 1,
        )
        result = subprocess.run(
            [sys.executable, os.path.join(REPO_ROOT, "scripts", "row17_census.py"),
             "--inventory", self.inventory_path, "--corpus-root", self.corpus_root, "--check"],
            capture_output=True, text=True,
        )
        self.assertEqual(result.returncode, 1)
        self.assertIn("CONTRACT VIOLATION", result.stdout)

    def test_check_flag_passes_when_no_malformed_markers(self):
        result = subprocess.run(
            [sys.executable, os.path.join(REPO_ROOT, "scripts", "row17_census.py"),
             "--inventory", self.inventory_path, "--corpus-root", self.corpus_root, "--check"],
            capture_output=True, text=True,
        )
        self.assertEqual(result.returncode, 0)


if __name__ == "__main__":
    unittest.main()
