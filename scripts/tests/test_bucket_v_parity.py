#!/usr/bin/env python3
"""Self-test for `scripts/oracle_harness/bucket_v_parity.py` (SD-35 AT-35-E4-002).

Covers the pure decisions the bucket-V oracle run makes -- the ones a wrong answer would make
a verdict silently wrong rather than loudly broken:

  * a unit's own row is located in the pinned data by `source_file` + `source_line`, and the
    identity is CHECKED, including PCGen's `.MOD` and `.COPY=<KEY>` row syntaxes;
  * an identity that does not match fails closed with a named reason, never a verdict;
  * which numbers count as "declared by the oracle" (a magnitude-bearing token's body, not a
    `SOURCEPAGE`);
  * the role a sheet line's numbers are compared in -- value first, then the rendered words;
  * the mechanical `cause` a disagreement is classified by.

Run: `python3 -m unittest scripts.tests.test_bucket_v_parity` (or `python3 -m unittest
discover -s scripts/tests -p 'test_bucket_v_parity.py'`).
"""

import os
import sys
import tempfile
import unittest

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO, "scripts", "oracle_harness"))

import bucket_v_parity as bv  # noqa: E402


def _lst(tmp, name, rows):
    path = os.path.join(tmp, name)
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(rows) + "\n")
    return {name: [path]}


class PinnedRowTests(unittest.TestCase):
    def test_plain_row_matches_on_its_key_and_reads_its_magnitudes(self):
        with tempfile.TemporaryDirectory() as tmp:
            idx = _lst(
                tmp,
                "x_abilities_class.lst",
                ["# header", "Bravery\tKEY:Fighter ~ Bravery\tCATEGORY:Special Ability\tBONUS:SAVE|Will|5\tSOURCEPAGE:p.99"],
            )
            row = bv.pinned_row({"corpus_key": "Fighter ~ Bravery", "source_file": "x_abilities_class.lst", "source_line": 2}, idx)
        self.assertTrue(row["identity_matches_corpus_key"])
        self.assertEqual(row["category"], "Special Ability")
        self.assertIn(5, row["declared_numbers"])
        self.assertNotIn(99, row["declared_numbers"], "a SOURCEPAGE number is not a magnitude the oracle declares")

    def test_copy_row_identity_is_read_out_of_the_copy_suffix(self):
        with tempfile.TemporaryDirectory() as tmp:
            idx = _lst(tmp, "cr_equipmods.lst", ["Special Quality ~ Wield Size / 1 Step Greater.COPY=PLUS1STEP\tPLUS:1"])
            row = bv.pinned_row({"corpus_key": "PLUS1STEP", "source_file": "cr_equipmods.lst", "source_line": 1}, idx)
        self.assertTrue(row["identity_matches_corpus_key"])
        self.assertEqual(row["key"], "PLUS1STEP")

    def test_mod_row_identity_drops_the_category_prefix_and_the_mod_suffix(self):
        with tempfile.TemporaryDirectory() as tmp:
            idx = _lst(tmp, "acg_abilities_race.lst", ["CATEGORY=Special Ability|Bloodrager ~ Bloodrage.MOD\tBONUS:VAR|X|1"])
            row = bv.pinned_row({"corpus_key": "Bloodrager ~ Bloodrage", "source_file": "acg_abilities_race.lst", "source_line": 1}, idx)
        self.assertTrue(row["identity_matches_corpus_key"])

    def test_a_row_that_is_not_this_unit_fails_closed_with_a_reason(self):
        with tempfile.TemporaryDirectory() as tmp:
            idx = _lst(tmp, "x.lst", ["Something Else\tKEY:Other ~ Thing"])
            row = bv.pinned_row({"corpus_key": "Fighter ~ Bravery", "source_file": "x.lst", "source_line": 1}, idx)
        self.assertFalse(row["identity_matches_corpus_key"])
        verdict = bv.verdict_for({"id": "a:b:c", "kind": "class_feature", "book": "core_rulebook", "corpus_key": "Fighter ~ Bravery", "pinned": row}, None, {}, {})
        self.assertEqual(verdict["verdict"], "oracle-unverifiable")
        self.assertEqual(verdict["reason"], "pinned-row-identity-mismatch")

    def test_a_source_file_absent_from_the_pin_is_named_not_guessed(self):
        row = bv.pinned_row({"corpus_key": "K", "source_file": "nope.lst", "source_line": 1}, {})
        self.assertEqual(row["reason"], "not-in-pinned-data")

    def test_a_line_number_past_the_end_of_the_file_is_named(self):
        with tempfile.TemporaryDirectory() as tmp:
            idx = _lst(tmp, "x.lst", ["only one row"])
            row = bv.pinned_row({"corpus_key": "K", "source_file": "x.lst", "source_line": 99}, idx)
        self.assertEqual(row["reason"], "line-out-of-range")


class RoleTests(unittest.TestCase):
    def test_the_value_column_is_the_first_role(self):
        form, role, nums, why = bv.ours_numbers({"line": {"form": "number", "printed": "+5", "prose": "a 10 foot reach"}})
        self.assertEqual((form, role, nums, why), ("number", "value", [5], None))

    def test_the_rendered_words_are_the_role_when_the_value_column_is_empty(self):
        form, role, nums, why = bv.ours_numbers({"line": {"form": "words", "printed": "", "prose": "deals 1d4+2 points"}})
        self.assertEqual(role, "prose")
        self.assertEqual(nums, ["1d4", 2], "a dice expression compares as its own token, not as two scalars")

    def test_a_words_line_with_no_number_has_nothing_to_compare(self):
        _form, role, nums, why = bv.ours_numbers({"line": {"form": "words", "printed": "", "prose": "you are proficient"}})
        self.assertIsNone(role)
        self.assertEqual(nums, [])
        self.assertEqual(why, "line-carries-no-number")

    def test_a_print_false_rule_is_named_as_such_not_as_a_missing_line(self):
        _form, _role, _nums, why = bv.ours_numbers({"line": None, "in_package": True, "suppressed": True})
        self.assertEqual(why, "rule-is-print-false-nothing-reaches-the-sheet")


class VerdictTests(unittest.TestCase):
    UNIT = {
        "id": "core_rulebook:class_feature:x",
        "kind": "class_feature",
        "book": "core_rulebook",
        "corpus_key": "X",
        "pinned": {
            "row": "X\tKEY:X\tBONUS:SAVE|Will|5",
            "identity_matches_corpus_key": True,
            "declared_numbers": [5],
            "desc_numbers": [],
            "path": "p",
            "source_line": 1,
            "key": "X",
            "name": "X",
        },
    }

    def test_our_number_among_the_oracles_agrees(self):
        v = bv.verdict_for(self.UNIT, {"line": {"form": "number", "printed": "+5", "prose": ""}}, {}, {})
        self.assertEqual((v["verdict"], v["tier"]), ("oracle-agree", "source"))

    def test_our_number_absent_disagrees_and_is_classified(self):
        v = bv.verdict_for(self.UNIT, {"line": {"form": "number", "printed": "+7", "prose": "a +5 bonus"}}, {}, {})
        self.assertEqual(v["verdict"], "oracle-disagree")
        self.assertEqual(v["missing"], [7])
        self.assertEqual(v["cause"], "value-role-number-the-oracle-never-prints-words-agree")

    def test_words_that_disagree_are_classified_apart_from_a_value_column_artefact(self):
        v = bv.verdict_for(self.UNIT, {"line": {"form": "words", "printed": "", "prose": "a +9 bonus"}}, {}, {})
        self.assertEqual(v["verdict"], "oracle-disagree")
        self.assertEqual(v["cause"], "rendered-words-disagree")

    def test_the_export_tier_wins_over_the_source_tier_when_a_carrier_exported_the_unit(self):
        manifest = {"c1": {"units": [self.UNIT["id"]]}}
        exports = {"c1": {"X": "you gain a +7 bonus"}}
        v = bv.verdict_for(self.UNIT, {"line": {"form": "number", "printed": "+7", "prose": ""}}, exports, manifest)
        self.assertEqual((v["verdict"], v["tier"]), ("oracle-agree", "export"))

    def test_an_export_that_never_granted_the_ability_falls_back_and_says_so(self):
        manifest = {"c1": {"units": [self.UNIT["id"]]}}
        v = bv.verdict_for(self.UNIT, {"line": {"form": "number", "printed": "+5", "prose": ""}}, {"c1": {}}, manifest)
        self.assertEqual(v["export_reason"], "pcgen-did-not-grant-the-ability")
        self.assertEqual(v["tier"], "source")


class ExportParsingTests(unittest.TestCase):
    def test_the_export_is_keyed_by_the_abilitys_own_key(self):
        parsed = bv.parse_export("NAME=x\nCATEGORY|Special Ability|2\nABILITY|Fighter ~ Bravery|+5 to Will\nnoise\n")
        self.assertEqual(parsed, {"Fighter ~ Bravery": "+5 to Will"})


if __name__ == "__main__":
    unittest.main()
