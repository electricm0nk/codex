#!/usr/bin/env python3
"""Tests for `scripts/probe_surface_census.py` (SD-33 Epic 1, AT-33-E1-003).

Proves the claim `AT-33-E1-003` requires: every corpus `kind` in
`docs/work-inventory.json` is enumerated, and for each one the census states
-- correctly -- whether a probe exists that can verify a computed magnitude,
naming it when it does.

Two kinds of fixture are used:

1. Small synthetic inventories (fast, not subject to corpus drift) that
   exercise the tool's own fail-closed behaviour: an unmapped `kind` must be
   detected, not silently ignored (`--check`'s coverage gate); a
   `probe_exists: true` kind whose live population never actually shows the
   probe's own positive evidence string is a false claim, and must be
   detected too.
2. One live-corpus acceptance case (`test_live_corpus_...`), run once against
   the committed `docs/work-inventory.json` -- this is what makes the
   headline "8 of 19 kinds have a probe" figure something this repo checks
   on every run, not something re-typed from memory (`decisions.md` §7).
"""

import copy
import json
import os
import subprocess
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import probe_surface_census as PSC  # noqa: E402


def _unit(kind, evidence, id_=None):
    return {"id": id_ or f"{kind}:{evidence}", "kind": kind, "evidence": evidence}


def _fake_inventory():
    """A tiny synthetic inventory covering exactly PSC's mapped kinds, one
    unit per positive-evidence example plus one plain unit, so the coverage
    and evidence-fired checks both have something real to examine."""
    units = []
    for kind, spec in PSC.PROBE_SURFACE.items():
        if spec["probe_exists"]:
            ev = spec["positive_evidence_examples"][0]
        else:
            ev = f"{kind}_content_has_no_engine_table"
        units.append(_unit(kind, ev))
    return {"units": units}


class BuildCensusTests(unittest.TestCase):
    def test_covers_every_mapped_kind_exactly_once(self):
        inv = _fake_inventory()
        census = PSC.build_census(inv)
        self.assertEqual(
            {row["kind"] for row in census["kinds"]}, set(PSC.PROBE_SURFACE.keys())
        )
        self.assertEqual(len(census["kinds"]), len(PSC.PROBE_SURFACE))

    def test_unit_counts_sum_to_population(self):
        inv = _fake_inventory()
        census = PSC.build_census(inv)
        self.assertEqual(sum(row["unit_count"] for row in census["kinds"]), len(inv["units"]))
        self.assertEqual(census["population"], len(inv["units"]))

    def test_probe_and_no_probe_bucket_counts_are_consistent(self):
        inv = _fake_inventory()
        census = PSC.build_census(inv)
        with_probe = [r for r in census["kinds"] if r["probe_exists"]]
        without_probe = [r for r in census["kinds"] if not r["probe_exists"]]
        self.assertEqual(census["kinds_with_probe"], len(with_probe))
        self.assertEqual(census["kinds_without_probe"], len(without_probe))
        self.assertEqual(
            census["kinds_with_probe"] + census["kinds_without_probe"], census["kind_count"]
        )


class CheckCoverageGateTests(unittest.TestCase):
    """`--check`'s job: detect a `kind` this census does not know about, and
    a `probe_exists: true` claim the live data never actually observes."""

    def test_check_passes_on_a_fully_covered_fixture(self):
        inv = _fake_inventory()
        ok, problems = PSC.check_census(inv)
        self.assertTrue(ok, problems)
        self.assertEqual(problems, [])

    def test_check_fails_closed_on_an_unmapped_kind(self):
        inv = _fake_inventory()
        inv["units"].append(_unit("wholly_new_kind_nobody_censused", "some_evidence"))
        ok, problems = PSC.check_census(inv)
        self.assertFalse(ok)
        self.assertTrue(any("wholly_new_kind_nobody_censused" in p for p in problems))

    def test_check_fails_closed_when_a_claimed_probe_never_fires(self):
        # Every unit of `class` in this fixture carries evidence that does
        # NOT match any of the probe's own positive-evidence strings -- the
        # exact shape of a stale/false `probe_exists: true` claim.
        inv = _fake_inventory()
        inv["units"] = [u for u in inv["units"] if u["kind"] != "class"]
        inv["units"].append(_unit("class", "class_absent_from_ClassId_ALL_and_book_class_id_enums"))
        ok, problems = PSC.check_census(inv)
        self.assertFalse(ok)
        self.assertTrue(any("class" in p and "never fires" in p for p in problems))

    def test_check_fails_closed_when_a_no_probe_kind_shows_probe_evidence(self):
        # A `probe_exists: false` kind whose live data DOES carry probe-shaped
        # evidence would mean the census under-claims -- also a defect.
        inv = _fake_inventory()
        inv["units"].append(_unit("monster", "monster_effect_probe_observed_computed_delta"))
        ok, problems = PSC.check_census(inv)
        self.assertFalse(ok)
        self.assertTrue(any("monster" in p and "probe-shaped" in p for p in problems))


class LiveCorpusAcceptanceTests(unittest.TestCase):
    """Run once against the real, committed `docs/work-inventory.json` --
    this is the execution-derived evidence `decisions.md` §7 requires, not a
    figure carried over from a prior cycle's memory."""

    @classmethod
    def setUpClass(cls):
        inv_path = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
        with open(inv_path, encoding="utf-8") as f:
            cls.inventory = json.load(f)

    def test_live_corpus_kind_count_is_19(self):
        live_kinds = {u["kind"] for u in self.inventory["units"]}
        self.assertEqual(len(live_kinds), 19)
        self.assertEqual(live_kinds, set(PSC.PROBE_SURFACE.keys()))

    def test_live_corpus_census_checks_clean(self):
        ok, problems = PSC.check_census(self.inventory)
        self.assertTrue(ok, problems)

    def test_live_corpus_eight_kinds_carry_a_magnitude_probe(self):
        census = PSC.build_census(self.inventory)
        self.assertEqual(census["kinds_with_probe"], 8)
        self.assertEqual(census["kinds_without_probe"], 11)
        self.assertEqual(census["population"], len(self.inventory["units"]))

    def test_cli_writes_the_committed_artifact_shape(self):
        result = subprocess.run(
            [sys.executable, os.path.join(REPO_ROOT, "scripts", "probe_surface_census.py"),
             "--check"],
            cwd=REPO_ROOT, capture_output=True, text=True,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("kinds_with_probe=8", result.stdout)
        self.assertIn("kinds_without_probe=11", result.stdout)
        self.assertIn("kind_count=19", result.stdout)


if __name__ == "__main__":
    unittest.main()
