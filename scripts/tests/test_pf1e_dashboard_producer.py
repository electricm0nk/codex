"""
Self-test for the dashboard producer's doneness-verdict table
(`scripts/observer/pf1e_dashboard_producer.py`) -- launch-readiness
remediation Step 4D, blocker B6.

WHY THIS EXISTS
----------------
`doneness_verdict()` / `_doneness_verdict_uncapped()` raise `ValueError` on
any `(wiring_class, status)` pair they have no rule for -- by design, so a
new status word landing in the generator without a matching rule is a loud
crash, not a silent default. `compute_wiring_class_summary()` catches that
exception per-unit and records it in `doneness_unmapped` rather than letting
one novel unit kill the whole 5-minute cron tick -- but a table that quietly
grows an unmapped cell is still a real defect (blocker B6: the
`(ambiguous, literal-verified|fixture-verified)` cells were unmapped until
this remediation). This test grids over the FULL `WIRING_CLASS_VALUES x`
status-vocabulary cross product on a fabricated document -- not the real
corpus, which may not exercise every cell -- and asserts nothing lands in
`doneness_unmapped`.

Run: python3 -m unittest scripts/tests/test_pf1e_dashboard_producer.py
Wired as the `producer-selftest` stage in both `verify.sh` stage sets.

Prove-it-can-fail discipline (same as `test_fetch_pcgen_oracle.sh`): comment
out the `ambiguous` branch's `"literal-verified", "fixture-verified"` tuple
members in `_doneness_verdict_uncapped()` and re-run -- `test_full_grid_...`
and `test_ambiguous_literal_verified_is_held` both go red.
"""
from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import tempfile
import unittest.mock
import unittest

# producer.py is always this test's sibling-of-a-sibling
# (scripts/observer/pf1e_dashboard_producer.py); resolved relative to
# __file__, the same convention the producer itself uses to find
# observer.py, rather than an absolute path that would break on a
# differently-rooted checkout.
_PRODUCER_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "observer" / "pf1e_dashboard_producer.py"
)
_spec = importlib.util.spec_from_file_location("pf1e_dashboard_producer", _PRODUCER_PATH)
producer = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(producer)

# Kept in sync BY HAND with `STATUS_VOCABULARY` in `src/bin/v06_work_inventory.rs`
# -- the Rust generator's status words are the one authoritative list; this
# test grids over its own copy rather than reaching across the Rust/Python
# boundary at test time, so a NEW status word landing in the generator
# without a matching `doneness_verdict()` rule is caught here as an
# unmapped cell in the grid below, not silently skipped because the test
# never knew the word existed.
STATUS_WORDS = (
    "grounded",
    "literal-verified",
    "fixture-verified",
    "ingested-magnitude",
    "text-complete",
    "deferred-with-reason",
    "not-ingested",
    "not-started",
    "unknown",
)


def _fabricated_doc_path(tmpdir: str) -> str:
    """One unit per `(wiring_class, status)` cell -- 5 x 9 = 45 units.

    `kind="spell"`, deliberately: `spell` left `NO_GROUNDING_PROBE` (see the
    producer's own SD30-E0-F2 comment on `STATUS_LABEL`/`NO_GROUNDING_PROBE`,
    re-derived live 2026-08-14), so `doneness_verdict()`'s kind-cap never
    fires and cannot mask a raising cell behind a capped `held` -- this test
    wants to see the RAW `_doneness_verdict_uncapped()` table's coverage.
    """
    units = []
    for wc in producer.WIRING_CLASS_VALUES:
        for st in STATUS_WORDS:
            units.append({
                "id": f"fab:{wc}:{st}",
                "book": "core_rulebook",
                "kind": "spell",
                "wiring_class": wc,
                "status": st,
            })
    doc = {"generated_at": "2026-08-15T00:00:00Z", "units": units}
    doc_path = os.path.join(tmpdir, "fab-work-inventory.json")
    with open(doc_path, "w", encoding="utf-8") as f:
        json.dump(doc, f)
    return doc_path


class DonenessVerdictGridTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.doc_path = _fabricated_doc_path(self._tmp.name)
        # A dedicated scratch cache path -- never the real
        # WIRING_CLASS_CACHE -- so this test can neither read a stale real
        # cache nor pollute it with a fabricated one.
        self.cache_path = os.path.join(self._tmp.name, "fab-wiring-cache.json")

    def test_full_grid_yields_no_unmapped_cells(self):
        """The whole 5 wiring_class x 9 status grid (45 cells) must map to a
        real doneness verdict -- `doneness_unmapped` must come back empty."""
        summary = producer.compute_wiring_class_summary(
            doc_path=self.doc_path, cache_path=self.cache_path
        )
        self.assertTrue(summary.get("available"), summary.get("note"))
        self.assertEqual(
            summary.get("doneness_unmapped", "MISSING (field absent)"),
            {},
            "the full WIRING_CLASS_VALUES x status_vocabulary grid must map every "
            f"cell to a verdict; unmapped: {summary.get('doneness_unmapped')}",
        )
        # Sanity: 45 units in, 45 units accounted for across done+unmapped.
        total_doneness = sum(summary.get("doneness", {}).values())
        total_unmapped = sum(summary.get("doneness_unmapped", {}).values())
        self.assertEqual(total_doneness + total_unmapped, len(producer.WIRING_CLASS_VALUES) * len(STATUS_WORDS))

    def test_ambiguous_literal_verified_is_held(self):
        self.assertEqual(
            producer.doneness_verdict("ambiguous", "literal-verified", "spell"),
            producer.DONENESS_HELD,
        )

    def test_ambiguous_fixture_verified_is_held(self):
        self.assertEqual(
            producer.doneness_verdict("ambiguous", "fixture-verified", "spell"),
            producer.DONENESS_HELD,
        )

    def test_static_literal_verified_is_done(self):
        """Control: the ORIGINAL done rung (SD-32 decisions.md §2) still
        works -- `static` unambiguously reaches `done` on the same status
        word `ambiguous` only reaches `held` on, above."""
        self.assertEqual(
            producer.doneness_verdict("static", "literal-verified", "spell"),
            producer.DONENESS_DONE,
        )

    def test_unmapped_cell_raises(self):
        """Negative case: a status word with genuinely no rule anywhere
        still raises -- this table must not have quietly become total."""
        with self.assertRaises(ValueError):
            producer.doneness_verdict("ambiguous", "bogus-status-word", "spell")


class BuildUnitShardsPiRedactionTest(unittest.TestCase):
    """Decision 12 (2026-08-17): `build_unit_shards` must withhold a unit's
    NAME when its own (book, source_file, source_line) row declares
    `NAMEISPI:YES` in the pinned oracle -- and must keep the row (count,
    status, wiring_class all unaffected) rather than dropping it. Mutation-
    proof: a declared-PI unit and a clean unit sit side by side; only the
    declared one is redacted."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        # Fake pinned-oracle checkout: one book, one LST file, one
        # NAMEISPI:YES row and one clean row.
        oracle_root = os.path.join(self._tmp.name, "oracle")
        book_dir = os.path.join(
            oracle_root, "pathfinder", "paizo", "roleplaying_game", "ultimate_equipment"
        )
        os.makedirs(book_dir)
        with open(os.path.join(book_dir, "ue_equip.lst"), "w", encoding="utf-8") as f:
            f.write("Sturdy Rope\tCOST:1\tWT:5\n")
            f.write("Otyugh Hide\tNAMEISPI:YES\tCOST:1415\n")

        doc = {
            "generated_at": "2026-08-17T00:00:00Z",
            "units": [
                {
                    "id": "ultimate_equipment:equipment:sturdy_rope",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Sturdy Rope",
                    "source_file": "ue_equip.lst",
                    "source_line": 1,
                    "status": "grounded",
                    "wiring_class": "static",
                },
                {
                    "id": "ultimate_equipment:equipment:otyugh_hide",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Otyugh Hide",
                    "source_file": "ue_equip.lst",
                    "source_line": 2,
                    "status": "grounded",
                    "wiring_class": "static",
                },
            ],
        }
        self.doc_path = os.path.join(self._tmp.name, "fab-work-inventory.json")
        with open(self.doc_path, "w", encoding="utf-8") as f:
            json.dump(doc, f)
        self.shard_dir = os.path.join(self._tmp.name, "shards")

        self._env_patch = unittest.mock.patch.dict(
            os.environ, {"PCGEN_CORPUS_ROOT": oracle_root}
        )
        self._env_patch.start()
        self.addCleanup(self._env_patch.stop)

    def _rows_by_name(self, kind_shard_path):
        with open(kind_shard_path, encoding="utf-8") as f:
            shard = json.load(f)
        idx = shard["fields"].index("name")
        return [row[idx] for row in shard["rows"]]

    def test_declared_pi_name_is_withheld_but_the_row_survives(self):
        index = producer.build_unit_shards(doc_path=self.doc_path, shard_dir=self.shard_dir)
        self.assertTrue(index.get("available"), index.get("note"))
        self.assertTrue(index.get("pi_oracle_available"), "the fake oracle checkout must be found")
        self.assertEqual(index.get("pi_redacted_names"), 1)

        equipment = index["kinds"]["equipment"]
        # The row count is UNCHANGED -- both units still counted.
        self.assertEqual(equipment["units"], 2)

        shard_path = os.path.join(self.shard_dir, equipment["shard"])
        names = self._rows_by_name(shard_path)
        self.assertIn("Sturdy Rope", names, "the clean unit's real name must ship")
        self.assertNotIn("Otyugh Hide", names, "the declared-PI name must never ship")
        self.assertIn(
            producer.pi_redaction.REDACTED_PI_MARKER, names,
            "the declared-PI row must still be present, with its name withheld",
        )


class BuildUnitShardsWordBoundaryAndTypeFacetTest(unittest.TestCase):
    """FIX-DASHBOARD-PI (2026-08-17): `build_unit_shards` must ALSO catch a
    declared-PI name EMBEDDED in a `name` field with no `NAMEISPI:YES` row
    of its own (word-boundary, via `_PiScreen`) and a declared-PI name
    embedded in the raw `type_facet` compound identifier (plain substring,
    no allow-list) -- neither existed before this fix. Mutation-proof: a
    clean row and a leaking row of each kind sit side by side; only the
    leaking ones are redacted."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        oracle_root = os.path.join(self._tmp.name, "oracle")
        book_dir = os.path.join(
            oracle_root, "pathfinder", "paizo", "roleplaying_game", "ultimate_equipment"
        )
        os.makedirs(book_dir)
        with open(os.path.join(book_dir, "ue_equip.lst"), "w", encoding="utf-8") as f:
            # "Rendmoor" is declared PI on its OWN row -- the embedding unit
            # below carries no such declaration on ITS row, so only the
            # word-boundary/substring layer can catch it.
            f.write("Rendmoor\tNAMEISPI:YES\tCOST:1\n")

        doc = {
            "generated_at": "2026-08-17T00:00:00Z",
            "units": [
                {
                    "id": "ultimate_equipment:equipment:sturdy_rope",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Sturdy Rope",
                    "type_facet": "Goods.Adventuring Gear",
                    "source_file": "ue_equip.lst",
                    "source_line": 99,
                    "status": "grounded",
                    "wiring_class": "static",
                },
                {
                    "id": "ultimate_equipment:equipment:blade_of_rendmoor",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Blade of Rendmoor",
                    "type_facet": "Weapon.Martial.Sword",
                    "source_file": "ue_equip.lst",
                    "source_line": 98,
                    "status": "grounded",
                    "wiring_class": "static",
                },
                {
                    "id": "ultimate_equipment:equipment:rendmoor_helm",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Ordinary Helm",
                    "type_facet": "ClassFeatures.Rendmoor Guard.SpecialQuality",
                    "source_file": "ue_equip.lst",
                    "source_line": 97,
                    "status": "grounded",
                    "wiring_class": "static",
                },
            ],
        }
        self.doc_path = os.path.join(self._tmp.name, "fab-work-inventory.json")
        with open(self.doc_path, "w", encoding="utf-8") as f:
            json.dump(doc, f)
        self.shard_dir = os.path.join(self._tmp.name, "shards")

        self._env_patch = unittest.mock.patch.dict(os.environ, {"PCGEN_CORPUS_ROOT": oracle_root})
        self._env_patch.start()
        self.addCleanup(self._env_patch.stop)

    def _rows(self, kind_shard_path):
        with open(kind_shard_path, encoding="utf-8") as f:
            return json.load(f)

    def test_word_boundary_embed_and_type_facet_substring_are_both_redacted(self):
        index = producer.build_unit_shards(doc_path=self.doc_path, shard_dir=self.shard_dir)
        self.assertTrue(index.get("available"), index.get("note"))
        equipment = index["kinds"]["equipment"]
        self.assertEqual(equipment["units"], 3, "no row is ever dropped for a name/type_facet hit")

        shard = self._rows(os.path.join(self.shard_dir, equipment["shard"]))
        name_idx = shard["fields"].index("name")
        tf_idx = shard["fields"].index("type_facet")
        # `build_unit_shards` preserves unit order within a kind, so pairing
        # the fabricated units (which carry a unique `source_line`) with the
        # shard's own rows positionally is a stable lookup.
        rows_by_line = {u["source_line"]: row for u, row in zip(doc_units_in_order(self.doc_path), shard["rows"])}

        clean = rows_by_line[99]
        self.assertEqual(clean[name_idx], "Sturdy Rope")
        self.assertEqual(clean[tf_idx], "Goods.Adventuring Gear", "an ordinary type_facet must ship unredacted")

        self.assertEqual(rows_by_line[98][name_idx], producer.pi_redaction.REDACTED_PI_MARKER,
                          "\"Blade of Rendmoor\" embeds the declared-PI name \"Rendmoor\" as a word")

        self.assertEqual(rows_by_line[97][name_idx], "Ordinary Helm",
                          "the name field itself carries no leak")
        self.assertEqual(rows_by_line[97][tf_idx], producer.pi_redaction.REDACTED_PI_MARKER,
                          "type_facet embeds \"Rendmoor\" as a substring and must be withheld")


def doc_units_in_order(doc_path):
    with open(doc_path, encoding="utf-8") as f:
        return json.load(f)["units"]


class ParseLstFirstFieldPiRedactionTest(unittest.TestCase):
    """`_parse_lst_first_field` feeds `_book_item_roster` (equipment/feats/
    spells rosters) AND `_prestige_class_roadmap`'s `ag_variants` --
    Decision 12's row-149 exposure. A declared-PI row's name must be
    withheld from the returned list while the row still occupies its own
    slot (dedup keys on the real name, not the redacted marker)."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.path = os.path.join(self._tmp.name, "fab_classes.lst")
        with open(self.path, "w", encoding="utf-8") as f:
            f.write("Ordinary Fighter\tHD:10\n")
            f.write("Aldori Swordlord\tNAMEISPI:YES\tHD:10\n")

    def test_declared_pi_row_is_redacted_but_still_present(self):
        names = producer._parse_lst_first_field(self.path)
        self.assertIn("Ordinary Fighter", names)
        self.assertNotIn("Aldori Swordlord", names)
        self.assertIn(producer.pi_redaction.REDACTED_PI_MARKER, names)
        self.assertEqual(len(names), 2, "the declared-PI row must not be dropped")


if __name__ == "__main__":
    unittest.main()
