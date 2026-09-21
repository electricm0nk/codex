#!/usr/bin/env python3
"""Regression test for `scripts/gen_class_status_table.py`.

SD-36 Epic F, batch F0, step F0e (`docs/release/SD-36-consolidation/
epic-f-class-completion.md` §2, F0.3). Pure unit tests against synthetic
census documents and a synthetic `status.md`-shaped fixture -- never
invokes the real `class_census` bin (that would make this suite pay for a
`cargo run` on every `python3 -m unittest` call), and never depends on the
real, committed `docs/architecture/status.md` (so a later hand-edit to
that file's prose cannot make this suite red for an unrelated reason).
The real file's own drift is instead covered live by `scripts/verify.sh`'s
`class-census` stage running `--check` against the real fixture.
"""

import copy
import os
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import gen_class_status_table as G  # noqa: E402


def make_doc(**overrides):
    doc = {
        "generated_at": "2026-09-21T00:00:00Z",
        "generated_by": "cargo run --bin class_census -- --json <path>",
        "source_of_truth": "codex::rules_core::pilot_compute::build_pilot_headless_receipt",
        "ids": 18,
        "computed": 12,
        "blocked": 6,
        "prestige_swept": 4,
        "prestige_alone_blocked": 4,
        "prestige_mix_computed": 1,
        "mix_panel_swept": 3,
        "mix_panel_computed": 3,
        "mix_panel_blocked": 0,
        "classes": [
            {"class_id": "class:fighter", "family": "CRB", "books": ["core_rulebook"],
             "registries": ["ClassId"], "max_level": 20, "status": "Computed",
             "levels_computed": 20, "levels_blocked": 0, "blocking_diagnostics": []},
            {"class_id": "class:wizard", "family": "CRB", "books": ["core_rulebook"],
             "registries": ["ClassId"], "max_level": 20, "status": "Computed",
             "levels_computed": 20, "levels_blocked": 0, "blocking_diagnostics": []},
            {"class_id": "class:samurai", "family": "Ultimate Combat", "books": ["ultimate_combat"],
             "registries": ["UcClassId"], "max_level": 20, "status": "Blocked",
             "levels_computed": 0, "levels_blocked": 20, "blocking_diagnostics": []},
            {"class_id": "class:gunslinger", "family": "Ultimate Combat", "books": ["ultimate_combat"],
             "registries": ["UcClassId"], "max_level": 20, "status": "Computed",
             "levels_computed": 20, "levels_blocked": 0, "blocking_diagnostics": []},
        ],
        "prestige": [],
        "mix_panel": [],
        "mix_panel_blocking_histogram": {},
    }
    doc.update(overrides)
    return doc


class FamilyRowsTests(unittest.TestCase):
    def test_groups_by_family_in_known_order_and_counts_computed(self):
        rows = G.family_rows(make_doc())
        labels = [r["label"] for r in rows]
        # CRB before Ultimate Combat, matching KNOWN_FAMILY_ORDER, even
        # though the fixture lists Ultimate Combat classes first in the
        # `classes` array (order comes from the fixed family list, not
        # array order).
        self.assertEqual(labels, ["CRB", "Ultimate Combat"])
        crb = rows[0]
        self.assertEqual(crb["ids"], 2)
        self.assertEqual(crb["computed"], 2)
        uc = rows[1]
        self.assertEqual(uc["ids"], 2)
        self.assertEqual(uc["computed"], 1)  # samurai blocked, gunslinger computed

    def test_family_with_zero_ids_is_omitted_not_fabricated_as_a_zero_row(self):
        rows = G.family_rows(make_doc())
        labels = {r["label"] for r in rows}
        self.assertNotIn("ACG", labels)  # no ACG class in this fixture at all

    def test_unknown_family_label_raises_loudly_rather_than_dropping_silently(self):
        doc = make_doc()
        doc["classes"].append({
            "class_id": "class:mystery", "family": "Some New Family Nobody Registered",
            "books": ["nowhere"], "registries": [], "max_level": 20, "status": "Blocked",
            "levels_computed": 0, "levels_blocked": 20, "blocking_diagnostics": [],
        })
        with self.assertRaises(ValueError) as ctx:
            G.family_rows(doc)
        self.assertIn("Some New Family Nobody Registered", str(ctx.exception))


class RenderTests(unittest.TestCase):
    def test_headline_table_states_every_figure_with_its_denominator(self):
        table = G.render_headline_table(make_doc())
        self.assertIn("**18**", table)
        self.assertIn("**12**", table)
        self.assertIn("of 18", table)
        self.assertIn("**1**", table)
        self.assertIn("of 4", table)  # prestige_mix_computed / prestige_swept
        self.assertIn("`prestige_mix_computed`", table)

    def test_family_table_total_row_sums_to_doc_ids(self):
        doc = make_doc(ids=8)  # 2 CRB + 2 Ultimate Combat + 4 prestige_swept = 8, consistent
        rows = G.family_rows(doc)
        table = G.render_family_table(doc, rows)
        self.assertIn("| **Total** | | **8** |", table)

    def test_family_table_raises_when_partition_does_not_sum_to_ids(self):
        doc = make_doc(ids=999)  # deliberately inconsistent with the fixture's own classes
        rows = G.family_rows(doc)
        with self.assertRaises(ValueError):
            G.render_family_table(doc, rows)

    def test_render_block_is_wrapped_in_markers(self):
        doc = make_doc(ids=8)  # 2 CRB + 2 UC + 4 prestige_swept = 8, consistent
        block = G.render_block(doc)
        self.assertTrue(block.startswith(G.MARKER_BEGIN))
        self.assertIn(G.MARKER_END, block)

    def test_family_table_never_prints_a_chassis_column(self):
        # `render_block`'s surrounding prose explains, in words, why there
        # is no Chassis column -- it legitimately contains the word. The
        # per-family table itself must not, since the census JSON reports
        # no per-id chassis field to put in one.
        doc = make_doc(ids=8)
        rows = G.family_rows(doc)
        table = G.render_family_table(doc, rows)
        self.assertNotIn("Chassis", table)


class MarkerSpliceTests(unittest.TestCase):
    FIXTURE = (
        "# Status\n\n"
        "## Class/level compute coverage\n\n"
        "intro prose that must survive untouched\n\n"
        f"{G.MARKER_BEGIN}\nOLD CONTENT HERE\n{G.MARKER_END}\n\n"
        "## Next section\n\ntrailing prose that must also survive untouched\n"
    )

    def test_extract_current_block_returns_the_marked_region(self):
        current = G.extract_current_block(self.FIXTURE)
        self.assertIn("OLD CONTENT HERE", current)
        self.assertTrue(current.startswith(G.MARKER_BEGIN))
        self.assertTrue(current.endswith(G.MARKER_END))

    def test_extract_current_block_raises_when_markers_absent(self):
        with self.assertRaises(ValueError):
            G.extract_current_block("# Status\n\nno markers anywhere here\n")

    def test_extract_current_block_raises_when_markers_duplicated(self):
        doubled = self.FIXTURE + self.FIXTURE
        with self.assertRaises(ValueError):
            G.extract_current_block(doubled)

    def test_apply_block_replaces_only_the_marked_region(self):
        new_block = f"{G.MARKER_BEGIN}\nNEW CONTENT\n{G.MARKER_END}"
        result = G.apply_block(self.FIXTURE, new_block)
        self.assertIn("NEW CONTENT", result)
        self.assertNotIn("OLD CONTENT HERE", result)
        self.assertIn("intro prose that must survive untouched", result)
        self.assertIn("trailing prose that must also survive untouched", result)

    def test_apply_block_raises_when_markers_absent(self):
        with self.assertRaises(ValueError):
            G.apply_block("no markers", f"{G.MARKER_BEGIN}\nx\n{G.MARKER_END}")


class CheckModeDriftTests(unittest.TestCase):
    """End-to-end through `main()` against a temp status.md and a `--json`
    fixture on disk -- the drift test F0.3's acceptance row calls for,
    without ever shelling out to `cargo run`."""

    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmpdir.cleanup)
        self.status_md = os.path.join(self.tmpdir.name, "status.md")
        self.json_path = os.path.join(self.tmpdir.name, "census.json")

        self.doc = make_doc(ids=8)  # 2 CRB + 2 UC + 4 prestige = 8, consistent
        with open(self.json_path, "w", encoding="utf-8") as fh:
            import json
            json.dump(self.doc, fh)

        with open(self.status_md, "w", encoding="utf-8") as fh:
            fh.write(
                "# Status\n\n## Class/level compute coverage\n\n"
                f"{G.MARKER_BEGIN}\nplaceholder\n{G.MARKER_END}\n\n## Next\n"
            )

    def _write_generated(self):
        rc = G.main(["--json", self.json_path, "--status-md", self.status_md])
        self.assertEqual(rc, 0)

    def test_write_mode_populates_the_markers(self):
        self._write_generated()
        with open(self.status_md, "r", encoding="utf-8") as fh:
            content = fh.read()
        self.assertIn("**8**", content)
        self.assertNotIn("placeholder", content)
        self.assertIn("## Next", content)  # surrounding doc untouched

    def test_check_mode_passes_immediately_after_a_matching_write(self):
        self._write_generated()
        rc = G.main(["--json", self.json_path, "--status-md", self.status_md, "--check"])
        self.assertEqual(rc, 0)

    def test_check_mode_fails_on_drift_when_the_census_moves(self):
        self._write_generated()
        drifted = copy.deepcopy(self.doc)
        drifted["computed"] = drifted["computed"] + 1  # a class newly reaches Computed
        drifted_path = os.path.join(self.tmpdir.name, "census-drifted.json")
        with open(drifted_path, "w", encoding="utf-8") as fh:
            import json
            json.dump(drifted, fh)
        rc = G.main(["--json", drifted_path, "--status-md", self.status_md, "--check"])
        self.assertEqual(rc, 1)

    def test_check_mode_fails_when_markers_are_missing(self):
        with open(self.status_md, "w", encoding="utf-8") as fh:
            fh.write("# Status\n\nno markers in this file\n")
        rc = G.main(["--json", self.json_path, "--status-md", self.status_md, "--check"])
        self.assertEqual(rc, 1)


if __name__ == "__main__":
    unittest.main()
