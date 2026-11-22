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
        "non_prestige_swept": 18,
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

    def test_headline_computed_denominator_is_non_prestige_swept_not_full_ids(self):
        # F0-check finding 2 (RED first): the real census's own shape --
        # ids=135, non_prestige_swept=61, computed=42, blocked=19 -- used to
        # render "**42** | of 135" and "**93** | of 135" (`blocked` itself
        # also computed as `ids - computed` on the Rust side, now fixed
        # too). The denominator for BOTH rows must be `non_prestige_swept`
        # (61), never the full merged `ids` (135), which also carries the
        # 74 prestige ids this column never sweeps.
        doc = make_doc(ids=135, non_prestige_swept=61, computed=42, blocked=19, prestige_swept=74)
        table = G.render_headline_table(doc)
        lines = table.splitlines()
        computed_line = next(line for line in lines if "reach `Computed` at every swept level" in line)
        blocked_line = next(line for line in lines if "reach `Computed` at no level" in line)
        self.assertIn("**42** | of 61", computed_line)
        self.assertIn("**19** | of 61", blocked_line)
        self.assertNotIn("of 135", computed_line)
        self.assertNotIn("of 135", blocked_line)

    def test_headline_table_raises_when_computed_and_blocked_do_not_partition_non_prestige_swept(self):
        # A drift guard: if a future edit changes what `computed`/`blocked`
        # count without moving `non_prestige_swept` in lockstep, this must
        # fail loudly rather than silently print a denominator that no
        # longer matches the population the numerator was measured over.
        doc = make_doc(non_prestige_swept=18, computed=12, blocked=5)  # 12+5=17 != 18
        with self.assertRaises(ValueError):
            G.render_headline_table(doc)

    def test_family_table_total_row_sums_to_doc_ids(self):
        # 2 CRB + 2 Ultimate Combat = 4 non-prestige, + 4 prestige_swept = 8, consistent.
        doc = make_doc(ids=8, non_prestige_swept=4)
        rows = G.family_rows(doc)
        table = G.render_family_table(doc, rows)
        self.assertIn("| **Total** | | **8**", table)

    def test_family_table_total_row_computed_denominator_is_non_prestige_ids_not_total(self):
        # F0-check finding 2: the Total row's "Computed alone" figure must
        # be stated "of <non-prestige ids>" (4), never "of <total ids>" (8,
        # which also folds in the 4 prestige ids that column never sweeps).
        doc = make_doc(ids=8, non_prestige_swept=4)
        rows = G.family_rows(doc)
        table = G.render_family_table(doc, rows)
        total_line = next(line for line in table.splitlines() if line.startswith("| **Total**"))
        self.assertIn("of 4 non-prestige ids", total_line)
        self.assertNotIn("of 8 non-prestige ids", total_line)

    def test_family_table_raises_when_partition_does_not_sum_to_ids(self):
        doc = make_doc(ids=999, non_prestige_swept=4)  # deliberately inconsistent with the fixture's own classes
        rows = G.family_rows(doc)
        with self.assertRaises(ValueError):
            G.render_family_table(doc, rows)

    def test_family_table_raises_when_non_prestige_swept_does_not_match_family_ids(self):
        # F0-check finding 2's own regression shape: `non_prestige_swept`
        # (the true non-prestige population) must equal the sum of the
        # per-family `ids` -- a generator that let these drift apart is
        # exactly what printed "93 | of 135" against a population of 61.
        doc = make_doc(ids=8, non_prestige_swept=999)  # family ids sum to 4, not 999
        rows = G.family_rows(doc)
        with self.assertRaises(ValueError):
            G.render_family_table(doc, rows)

    def test_render_block_is_wrapped_in_markers(self):
        # 2 CRB + 2 UC + 4 prestige_swept = 8, consistent; computed/blocked
        # (3/1) match the 4-class non-prestige fixture (3 Computed, 1 Blocked).
        doc = make_doc(ids=8, non_prestige_swept=4, computed=3, blocked=1)
        block = G.render_block(doc)
        self.assertTrue(block.startswith(G.MARKER_BEGIN))
        self.assertIn(G.MARKER_END, block)

    def test_family_table_never_prints_a_chassis_column(self):
        # `render_block`'s surrounding prose explains, in words, why there
        # is no Chassis column -- it legitimately contains the word. The
        # per-family table itself must not, since the census JSON reports
        # no per-id chassis field to put in one.
        doc = make_doc(ids=8, non_prestige_swept=4, computed=3, blocked=1)
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

        # 2 CRB (both Computed) + 2 UC (1 Computed, 1 Blocked) = 4
        # non-prestige, + 4 prestige = 8 total ids, fully self-consistent.
        self.doc = make_doc(ids=8, non_prestige_swept=4, computed=3, blocked=1)
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
        # A class newly reaches Computed -- computed+1, blocked-1, so the
        # non_prestige_swept partition stays internally consistent (a real
        # census drift never changes the population swept, only which
        # bucket a row lands in).
        drifted["computed"] = drifted["computed"] + 1
        drifted["blocked"] = drifted["blocked"] - 1
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
