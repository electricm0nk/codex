#!/usr/bin/env python3
"""Tests for `scripts/shape_ledger.py` (SD-32 Gate 1, card `gate-1-shape-closure`).

Proves the two load-bearing claims AT-32-G1-001/002 make:

1. Every not-done unit is classified into exactly one family -- including
   the honest F0 (no formula content) and F8 (residual) extension families
   -- so `unclassified_count` is structurally 0, not merely 0 by luck on
   today's inventory.
2. The tool fails closed on an empty/unreadable inventory (AT-32-G1-002):
   it must refuse to report "0 unclassified" when there was nothing to
   classify, mirroring `test_coverage_ledger.py`'s negative-case
   discipline for the same shape of claim.

Uses small synthetic inventory/corpus fixtures rather than the live 38k-unit
corpus, so these tests stay fast and are not subject to corpus drift
(same discipline as `test_coverage_ledger.py`'s own docstring).
"""

import json
import os
import subprocess
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_ledger as SL  # noqa: E402


def _unit(id_, kind, book, status, wiring_class, source_file, source_line, **extra):
    u = {
        "id": id_,
        "kind": kind,
        "book": book,
        "status": status,
        "wiring_class": wiring_class,
        "source_file": source_file,
        "source_line": source_line,
    }
    u.update(extra)
    return u


class ExtractFormulaSegmentTest(unittest.TestCase):
    def test_define_takes_second_field(self):
        self.assertEqual(SL.extract_formula_segment("DEFINE", "MesmeristPool|0"), "0")

    def test_define_missing_second_field_returns_none(self):
        self.assertIsNone(SL.extract_formula_segment("DEFINE", "MesmeristPool"))

    def test_bonus_var_takes_third_field(self):
        self.assertEqual(
            SL.extract_formula_segment("BONUS", "VAR|TDeftFingersBonus|max(MesmeristLVL/2,1)"),
            "max(MesmeristLVL/2,1)",
        )

    def test_bonus_skill_takes_third_field_ignores_trailing_type(self):
        self.assertEqual(
            SL.extract_formula_segment("BONUS", "SKILL|Perception|8|TYPE=Racial"), "8"
        )

    def test_bonus_too_short_returns_none(self):
        self.assertIsNone(SL.extract_formula_segment("BONUS", "SKILL|Perception"))

    def test_non_define_non_bonus_returns_none(self):
        self.assertIsNone(SL.extract_formula_segment("ABILITY", "Special Ability|X"))


class ClassifyFormulaTest(unittest.TestCase):
    def test_flat_constant(self):
        self.assertEqual(SL.classify_formula("2"), "F1")
        self.assertEqual(SL.classify_formula("-4"), "F1")
        self.assertEqual(SL.classify_formula("10%"), "F1")

    def test_per_level_scaling(self):
        self.assertEqual(SL.classify_formula("MesmeristLVL/2"), "F2")
        self.assertEqual(SL.classify_formula("WizardLVL"), "F2")

    def test_ability_modifier(self):
        self.assertEqual(SL.classify_formula("WIS"), "F3")
        self.assertEqual(SL.classify_formula("-CON"), "F3")

    def test_named_counter_plain_identifier(self):
        self.assertEqual(SL.classify_formula("BloodlineLVL_UNRELATED_NAME_no_lvl_suffix"), "F4")
        self.assertEqual(SL.classify_formula("TInjectionsUses"), "F4")

    def test_clamped_per_level(self):
        self.assertEqual(SL.classify_formula("max(MesmeristLVL/2,1)"), "F5")
        self.assertEqual(SL.classify_formula("min(floor((Sorcerer_Psychic_BloodlinePower3LVL+3)/6*2),4)"), "F5")

    def test_classlevel(self):
        self.assertEqual(SL.classify_formula('classlevel("Wizard")'), "F6")

    def test_conditional_step(self):
        self.assertEqual(
            SL.classify_formula("if(Sorcerer_CF_BloodlineArcana==0,1,0)"), "F7"
        )

    def test_skill_rank_derived(self):
        self.assertEqual(SL.classify_formula('skillinfo("RANK","Bluff")'), "F9")
        self.assertEqual(SL.classify_formula("TOTALRANK"), "F9")
        # skillinfo wins priority over a co-occurring LVL term
        self.assertEqual(
            SL.classify_formula('if(skillinfo("RANK","Bluff")>=WizardLVL,1,0)'), "F9"
        )

    def test_level_threshold_step_count(self):
        self.assertEqual(
            SL.classify_formula("if(WizardLVL>=5,1,0)+if(WizardLVL>=10,1,0)+if(WizardLVL>=15,1,0)"),
            "F10",
        )

    def test_residual_falls_to_f8(self):
        # Multi-term arithmetic mixing identifiers this classifier does
        # not recognise -- named F8 per the module docstring, not silently
        # dropped into F0/F1.
        self.assertEqual(SL.classify_formula('var("COUNT[EQTYPE.ARMOR.EQUIPPED]")*2'), "F8")

    def test_empty_string_is_f0_not_f8(self):
        self.assertEqual(SL.classify_formula(""), SL.FAMILY_F0_NO_FORMULA)

    def test_every_family_id_referenced_has_metadata(self):
        # Every id this function can return must have a label + proof_width
        # in the family metadata table -- AT-32-G1-003's requirement that
        # every family states its proof width, checked structurally.
        meta = SL._family_metadata()
        possible_ids = {fid for fid, *_ in SL.FAMILIES} | {SL.FAMILY_F0_NO_FORMULA, SL.FAMILY_F8_OTHER}
        for fid in possible_ids:
            self.assertIn(fid, meta, f"family {fid} has no metadata entry")
            self.assertTrue(meta[fid]["label"])
            self.assertTrue(meta[fid]["proof_width"])


class BuildCorpusIndexTest(unittest.TestCase):
    def test_indexes_by_book_basename_line_and_filters_define_bonus(self):
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "test_book", "spell")
            os.makedirs(book_dir)
            record = {
                "data": {
                    "raw_tokens": [
                        {"key": "DESC", "value": "irrelevant, not a formula token"},
                        {"key": "BONUS", "value": "VAR|Foo|WIS"},
                        {"key": "DEFINE", "value": "Bar|0"},
                    ]
                },
                "source": {"path": "some/path/test_spells.lst", "line": 42},
            }
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(record, fh)
            # LICENSE.json must be skipped even if present
            with open(os.path.join(book_dir, "LICENSE.json"), "w") as fh:
                json.dump({"data": {"raw_tokens": [{"key": "BONUS", "value": "SHOULD|NOT|APPEAR"}]}}, fh)

            index = SL.build_corpus_index(tmp, books={"test_book"})
            key = ("test_book", "test_spells.lst", 42)
            self.assertIn(key, index)
            toks = index[key]
            self.assertEqual(len(toks), 2)
            self.assertTrue(all(t["key"] in ("BONUS", "DEFINE") for t in toks))


class ClassifyUnitTest(unittest.TestCase):
    def test_no_join_match_is_f0_no_record(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "missing.lst", 1)
        row = SL.classify_unit(unit, corpus_index={})
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)
        self.assertEqual(row["join_status"], "no_record")

    def test_join_match_no_formula_tokens_is_f0(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)
        index = {("b", "f.lst", 1): []}
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)
        self.assertEqual(row["join_status"], "no_formula_tokens")

    def test_join_match_picks_highest_priority_family(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)
        index = {
            ("b", "f.lst", 1): [
                {"key": "BONUS", "value": "VAR|Foo|WIS"},  # F3
                {"key": "BONUS", "value": 'VAR|Bar|skillinfo("RANK","Bluff")'},  # F9, higher priority
            ]
        }
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["family"], "F9")
        self.assertEqual(row["join_status"], "matched")

    def test_missing_source_fields_is_no_record(self):
        unit = {"id": "b:spell:x", "kind": "spell", "book": "b"}
        row = SL.classify_unit(unit, corpus_index={})
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)
        self.assertEqual(row["join_status"], "no_record")


class BuildLedgerTest(unittest.TestCase):
    def test_unclassified_count_is_always_zero_for_nonempty_population(self):
        units = [
            _unit("b:spell:x", "spell", "b", "not-started", "static", "missing.lst", 1),
            _unit("b:spell:y", "spell", "b", "not-started", "static", "f.lst", 1),
        ]
        index = {("b", "f.lst", 1): [{"key": "BONUS", "value": "VAR|Foo|WIS"}]}
        ledger = SL.build_ledger(units, index)
        self.assertEqual(ledger["unclassified_count"], 0)
        self.assertEqual(ledger["population"], 2)
        self.assertIn(SL.FAMILY_F0_NO_FORMULA, ledger["families"])
        self.assertIn("F3", ledger["families"])


class FailClosedOnEmptyTest(unittest.TestCase):
    """AT-32-G1-002: the shape ledger fails closed on empty predicates."""

    def _run(self, inventory_path):
        return subprocess.run(
            [sys.executable, os.path.join(REPO_ROOT, "scripts", "shape_ledger.py"), "--inventory", inventory_path],
            capture_output=True,
            text=True,
        )

    def test_dev_null_reports_no_coverage_and_nonzero_exit(self):
        result = self._run("/dev/null")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no coverage", (result.stdout + result.stderr).lower())

    def test_empty_units_list_reports_no_coverage_and_nonzero_exit(self):
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
            json.dump({"units": []}, fh)
            path = fh.name
        try:
            result = self._run(path)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("no coverage", (result.stdout + result.stderr).lower())
        finally:
            os.unlink(path)

    def test_all_done_units_also_reports_no_coverage(self):
        # not_done_population() filters DONE units to zero -- proves the
        # fail-closed path is reachable via real filtering, not only via a
        # literally-empty units list.
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
            json.dump(
                {
                    "units": [
                        _unit(
                            "b:equipment:e", "equipment", "b", "literal-verified", "static", "f.lst", 1
                        )
                    ]
                },
                fh,
            )
            path = fh.name
        try:
            result = self._run(path)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("no coverage", (result.stdout + result.stderr).lower())
        finally:
            os.unlink(path)


class GateEndToEndTest(unittest.TestCase):
    """Runs the tool end to end against a small synthetic inventory +
    corpus tree, mirroring AT-32-G1-001's verification command shape."""

    def test_end_to_end_produces_zero_unclassified_and_expected_families(self):
        with tempfile.TemporaryDirectory() as tmp:
            corpus_root = os.path.join(tmp, "corpus")
            book_dir = os.path.join(corpus_root, "book_a", "spell")
            os.makedirs(book_dir)
            records = {
                "unit_flat.json": {
                    "data": {"raw_tokens": [{"key": "BONUS", "value": "SKILL|Perception|2|TYPE=Racial"}]},
                    "source": {"path": "x/book_a_spells.lst", "line": 1},
                },
                "unit_lvl.json": {
                    "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WizardLVL/2"}]},
                    "source": {"path": "x/book_a_spells.lst", "line": 2},
                },
                "unit_noformula.json": {
                    "data": {"raw_tokens": []},
                    "source": {"path": "x/book_a_spells.lst", "line": 3},
                },
            }
            for fname, rec in records.items():
                with open(os.path.join(book_dir, fname), "w") as fh:
                    json.dump(rec, fh)

            inventory = {
                "units": [
                    _unit("book_a:spell:flat", "spell", "book_a", "not-started", "static", "book_a_spells.lst", 1),
                    _unit("book_a:spell:lvl", "spell", "book_a", "not-started", "static", "book_a_spells.lst", 2),
                    _unit(
                        "book_a:spell:noformula",
                        "spell",
                        "book_a",
                        "not-started",
                        "static",
                        "book_a_spells.lst",
                        3,
                    ),
                    _unit("book_a:spell:unreached", "spell", "book_a", "not-started", "static", "missing.lst", 99),
                ]
            }
            inv_path = os.path.join(tmp, "inventory.json")
            with open(inv_path, "w") as fh:
                json.dump(inventory, fh)
            out_path = os.path.join(tmp, "ledger.json")

            result = subprocess.run(
                [
                    sys.executable,
                    os.path.join(REPO_ROOT, "scripts", "shape_ledger.py"),
                    "--inventory",
                    inv_path,
                    "--corpus-root",
                    corpus_root,
                    "--output",
                    out_path,
                ],
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            with open(out_path) as fh:
                ledger = json.load(fh)
            self.assertEqual(ledger["unclassified_count"], 0)
            self.assertEqual(ledger["population"], 4)
            self.assertEqual(ledger["families"]["F1"]["count"], 1)
            self.assertEqual(ledger["families"]["F2"]["count"], 1)
            self.assertEqual(ledger["families"][SL.FAMILY_F0_NO_FORMULA]["count"], 2)


if __name__ == "__main__":
    unittest.main()
