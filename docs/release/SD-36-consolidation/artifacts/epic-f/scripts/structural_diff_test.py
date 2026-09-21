"""Pin for `structural_diff.py` becoming a gate (SD-36 Epic F1 re-check round 1, finding 2).

Reproduces, on synthetic fixtures (never the tracked `data/sheet_rules` package), the six
mutations the re-check ran by hand against a real fresh `--dump`:

    (a) a `value` field changed                          -> unexpected field delta
    (b) a rule removed entirely                           -> removed rule id
    (c) a `label` field renamed                            -> unexpected field delta
    (d) `_report.json` `records` count moved by one         -> moved count
    (e) a rule's sole `granted_by` edge deleted              -> removed granted_by edge
    (f) a rule's `grants` content replaced by a fabrication  -> removed grant

Before the fix, (a)/(c) were reported but did not gate (script always exited 0); (b) was
reported as a removed rule id but likewise never gated; (d) was printed as an unflagged pair;
(e) and (f) produced NO change in the report at all. This pin proves all six now gate (exit 1)
and are each named in the printed report.

Run:
    python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff_test.py
"""
from __future__ import annotations

import io
import json
import os
import sys
import tempfile
import unittest
from contextlib import redirect_stdout

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff  # noqa: E402


def write_tree(root: str, rules_by_file: dict[str, list[dict]], report: dict) -> None:
    for rel, rules in rules_by_file.items():
        path = os.path.join(root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as fh:
            json.dump(rules, fh)
    with open(os.path.join(root, "_report.json"), "w", encoding="utf-8") as fh:
        json.dump(report, fh)


BASE_REPORT = {"records": 49450, "converted": 40000, "refused": 9450, "rules_written": 71863, "var_tables": 100}


def base_rules() -> dict[str, list[dict]]:
    return {
        "core_rulebook/class_feature/arcane_bloodline_school_power.json": [
            {
                "id": "core_rulebook:class_feature:arcane_bloodline_school_power",
                "label": "Arcane Bloodline School Power",
                "value": {"Number": {"Const": 1}},
                "granted_by": [],
                "grants": [],
            }
        ],
        "core_rulebook/class_feature/wizard_weapon_and_armor_proficiency.json": [
            {
                "id": "core_rulebook:class_feature:wizard_weapon_and_armor_proficiency",
                "label": "Weapon and Armor Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [],
            }
        ],
        "core_rulebook/class_feature/single_simple_weapon_proficiency.json": [
            {
                "id": "core_rulebook:class_feature:single_simple_weapon_proficiency",
                "label": "Simple Weapon Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [],
            }
        ],
        "ultimate_combat/class_feature/samurai_proficiencies.json": [
            {
                "id": "ultimate_combat:class_feature:samurai_proficiencies",
                "label": "Samurai Proficiencies",
                "value": "Text",
                "granted_by": [{"by": {"Rule": "ultimate_combat:ability:samurai"}, "when": None}],
                "grants": [{"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}}],
            }
        ],
    }


class StructuralDiffGateTest(unittest.TestCase):
    def run_diff(self, base: dict[str, list[dict]], fresh: dict[str, list[dict]], fresh_report: dict | None = None) -> tuple[int, str]:
        with tempfile.TemporaryDirectory() as baseline_dir, tempfile.TemporaryDirectory() as scratch_dir:
            write_tree(baseline_dir, base, BASE_REPORT)
            write_tree(scratch_dir, fresh, fresh_report if fresh_report is not None else BASE_REPORT)
            argv = sys.argv
            sys.argv = ["structural_diff.py", scratch_dir, "--baseline", baseline_dir]
            buf = io.StringIO()
            try:
                with redirect_stdout(buf):
                    code = structural_diff.main()
            finally:
                sys.argv = argv
            return code, buf.getvalue()

    def test_unchanged_tree_passes(self):
        rules = base_rules()
        code, out = self.run_diff(rules, rules)
        self.assertEqual(code, 0, out)
        self.assertIn("verdict=PASS", out)

    def test_a_value_field_change_gates(self):
        base = base_rules()
        fresh = base_rules()
        fresh["core_rulebook/class_feature/arcane_bloodline_school_power.json"][0]["value"] = {"Number": {"Const": 7}}
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas: 1", out)
        self.assertIn("arcane_bloodline_school_power: value", out)

    def test_b_removed_rule_gates(self):
        base = base_rules()
        fresh = base_rules()
        del fresh["core_rulebook/class_feature/wizard_weapon_and_armor_proficiency.json"]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed rule ids: 1", out)
        self.assertIn("wizard_weapon_and_armor_proficiency", out)

    def test_c_renamed_field_gates(self):
        base = base_rules()
        fresh = base_rules()
        rec = fresh["core_rulebook/class_feature/single_simple_weapon_proficiency.json"][0]
        rec["labell"] = rec.pop("label")
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas:", out)
        self.assertNotIn("unexpected field deltas: 0", out)

    def test_d_moved_record_count_gates(self):
        rules = base_rules()
        fresh_report = dict(BASE_REPORT)
        fresh_report["records"] = BASE_REPORT["records"] - 1
        code, out = self.run_diff(rules, rules, fresh_report=fresh_report)
        self.assertEqual(code, 1, out)
        self.assertIn("records moved: 49450 -> 49449", out)

    def test_e_deleted_granted_by_edge_gates(self):
        base = base_rules()
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["granted_by"] = []
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed granted_by edges: 1", out)
        self.assertIn("samurai_proficiencies", out)

    def test_f_fabricated_grant_gates(self):
        base = base_rules()
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponGroup": "TOTALLY FABRICATED"}}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed grants: 1", out)
        self.assertIn("samurai_proficiencies", out)

    def test_f1_2_gating_wrap_of_the_same_fact_does_not_gate(self):
        """F1-2 wraps a bare `FactGrant` in a `GatedFactGrant` around the identical fact when a
        PRE-gate applies -- a real, sanctioned transformation this branch ships (not a removal).
        Reproduces the false positive the round-1 fix's raw-JSON `edge_diff` produced against the
        real corpus (`brawler_weapon_and_armor_proficiency`) before `grant_signature` replaced it."""
        base = base_rules()
        fresh = base_rules()
        rec = fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {
                "GatedFactGrant": {
                    "fact": {"Proficiency": {"WeaponGroup": "Samurai"}},
                    "when": {"Compare": {"lhs": {"Var": "v1"}, "op": "Eq", "rhs": {"Const": 0}}},
                }
            }
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("removed grants: 0", out)

    def test_f1_3_bare_tag_to_weapon_set_upgrade_does_not_gate(self):
        """F1-3 expands a bare `WeaponGroup`/`WeaponTag` into an equivalent `WeaponSet` naming the
        SAME tag (case-insensitive) with a resolved member list -- a real, sanctioned
        transformation (not a removal). Reproduces the false positive the round-1 fix produced
        against the real corpus (`musketeer_weapon_proficiency`'s `OnehandedFirearm`) before
        `grant_signature` replaced the raw-JSON edge diff."""
        base = base_rules()
        base["advanced_class_guide/class_feature/musketeer_weapon_proficiency.json"] = [
            {
                "id": "advanced_class_guide:class_feature:musketeer_weapon_proficiency",
                "label": "Musketeer Weapon Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [{"FactGrant": {"Proficiency": {"WeaponGroup": "OnehandedFirearm"}}}],
            }
        ]
        fresh = base_rules()
        fresh["advanced_class_guide/class_feature/musketeer_weapon_proficiency.json"] = [
            {
                "id": "advanced_class_guide:class_feature:musketeer_weapon_proficiency",
                "label": "Musketeer Weapon Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [
                    {
                        "FactGrant": {
                            "Proficiency": {
                                "WeaponSet": {"label": "OnehandedFirearm", "members": ["Pistol", "Musket"]}
                            }
                        }
                    }
                ],
            }
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("removed grants: 0", out)

    def test_added_edge_and_grant_are_allowed_and_do_not_gate(self):
        base = base_rules()
        fresh = base_rules()
        rec = fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["granted_by"] = list(rec["granted_by"]) + [{"by": {"Rule": "ultimate_combat:ability:other_source"}, "when": None}]
        rec["grants"] = list(rec["grants"]) + [{"FactGrant": {"Proficiency": {"WeaponGroup": "Extra"}}}]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("verdict=PASS", out)

    def test_report_only_flag_keeps_exit_zero(self):
        base = base_rules()
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["granted_by"] = []
        with tempfile.TemporaryDirectory() as baseline_dir, tempfile.TemporaryDirectory() as scratch_dir:
            write_tree(baseline_dir, base, BASE_REPORT)
            write_tree(scratch_dir, fresh, BASE_REPORT)
            argv = sys.argv
            sys.argv = ["structural_diff.py", scratch_dir, "--baseline", baseline_dir, "--report-only"]
            buf = io.StringIO()
            try:
                with redirect_stdout(buf):
                    code = structural_diff.main()
            finally:
                sys.argv = argv
            self.assertEqual(code, 0, buf.getvalue())
            self.assertIn("verdict=FAIL", buf.getvalue())


if __name__ == "__main__":
    unittest.main()
