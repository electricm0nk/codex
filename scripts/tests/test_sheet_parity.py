"""Self-tests for `scripts/oracle_harness/sheet_parity.py` (SD-35 AT-35-E2-005).

Fixture discipline: every input here is a hand-written export text and a hand-written
`ours` document -- the comparison logic is tested on shapes it must get right (a folded
same-type pair, a stacking type, a save's ability modifier removed from `CHECK.n.MISC`, a
DESC-number match, a planted disagreement that must surface as `disagree` and never fold into
`agree`), never on the live corpus. Run: `python3 -m unittest scripts/tests/test_sheet_parity.py`.
"""

from __future__ import annotations

import os
import sys
import unittest

sys.path.insert(0, os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "oracle_harness"))
import sheet_parity as SP  # noqa: E402

EXPORT = """NAME=Test
STAT.0.NAME=STR
STAT.0.SCORE=14
STAT.0.MOD=+2
STAT.1.NAME=DEX
STAT.1.SCORE=10
STAT.1.MOD=+0
STAT.2.NAME=CON
STAT.2.SCORE=8
STAT.2.MOD=-1
STAT.3.NAME=INT
STAT.3.SCORE=18
STAT.3.MOD=+4
STAT.4.NAME=WIS
STAT.4.SCORE=14
STAT.4.MOD=+2
STAT.5.NAME=CHA
STAT.5.SCORE=16
STAT.5.MOD=+3
AC.TOTAL=12
AC.ARMOR=0
AC.NATURALARMOR=0
AC.DEFLECTION=0
AC.DODGE=1
AC.MISC=0
ACCHECK=-2
BAB=+1
ATTACK.MELEE.TOTAL=+3
ATTACK.MELEE.MISC=+0
INITIATIVEMISC=+0
CHECK.0.NAME=Fortitude
CHECK.0.TOTAL=+3
CHECK.0.BASE=+2
CHECK.0.MISC=+1
CHECK.1.NAME=Reflex
CHECK.1.TOTAL=+0
CHECK.1.BASE=+0
CHECK.1.MISC=+0
CHECK.2.NAME=Will
CHECK.2.TOTAL=+2
CHECK.2.BASE=+0
CHECK.2.MISC=+2
SKILL.0.NAME=Acrobatics
SKILL.0.TOTAL=4
SKILL.0.MISC=2
SKILL.0.ACHECK=YES
SKILL.1.NAME=Climb
SKILL.1.TOTAL=3
SKILL.1.MISC=1
SKILL.1.ACHECK=YES
SKILL.2.NAME=Diplomacy
SKILL.2.TOTAL=3
SKILL.2.MISC=3
SKILL.2.ACHECK=NONE
MOVE.0.NAME=Walk
MOVE.0.RATE=30 ft.
SA.0.NAME=Rage
SA.0.DESC=A barbarian can rage for 6 rounds per day.
This is a second paragraph with the number 12 in it.
SA.1.NAME=Stunning Fist
SA.1.DESC=Fortitude save DC 15 negates.
SA.2.NAME=Detect Evil
SA.2.DESC=At will, you can use Detect Evil, as the Spell. You can concentrate on a single individual within 60 feet, as if having studied it for 3 rounds.
"""


def ours(lines, **chassis_over):
    chassis = {
        "ability_mods": [2, 0, -1, 4, 2, 3],
        "bab": 1,
        "base_saves": [2, 0, 0],
        "total_saves": [3, 0, 2],
        "baseline_ac": 12,
        "baseline_melee_attack": 3,
        "skills": {"climb": 3},
    }
    chassis.update(chassis_over)
    return {
        "character": "test",
        "ability_scores": [14, 10, 8, 18, 14, 16],
        "chassis": chassis,
        "facts": {"speeds": {"Walk": 30}},
        "lines": lines,
    }


def line(rule_id, label, value, target=None, bonus_type=None, also=None):
    return {
        "id": rule_id,
        "kind": rule_id.split(":")[1],
        "label": label,
        "value": {"Resolved": value},
        "printed": str(value),
        "also": also or [],
        "prose": "",
        "condition": None,
        "target": target,
        "bonus_type": bonus_type,
        "form": "number",
    }


class ParseExportTest(unittest.TestCase):
    def test_continuation_lines_join_the_previous_value(self):
        parsed = SP.parse_export(EXPORT)
        self.assertIn("second paragraph", parsed["SA.0.DESC"])
        self.assertEqual(parsed["SA.1.NAME"], "Stunning Fist")
        self.assertEqual(SP.as_int(parsed["STAT.2.MOD"]), -1)
        self.assertEqual(SP.as_int(parsed["MOVE.0.RATE"]), 30)


class FoldTest(unittest.TestCase):
    def test_same_type_takes_the_max_and_stacking_types_sum(self):
        enh = {"name": "Enhancement", "mode": "Plain"}
        self.assertEqual(SP.fold([(2, enh), (3, enh)]), 3)
        dodge = {"name": "Dodge", "mode": "Plain"}
        self.assertEqual(SP.fold([(1, dodge), (1, dodge)]), 2)
        self.assertEqual(SP.fold([(2, None), (-1, enh), (2, enh)]), 3)


class CompareTest(unittest.TestCase):
    def setUp(self):
        self.export = SP.parse_export(EXPORT)

    def rows_for(self, lines, **over):
        return {r["unit"]: r for r in SP.compare_character(ours(lines, **over), self.export)}

    def test_skill_contribution_folds_and_matches_the_misc_component(self):
        rows = self.rows_for(
            [
                line("b:feat:acrobatic", "Acrobatic", 2, {"Skill": "acrobatics"}),
                line("b:trait:nimble", "Nimble", 2, {"Skill": "acrobatics"}, {"name": "Trait", "mode": "Plain"}),
            ]
        )
        r = rows["target:Skill:acrobatics"]
        self.assertEqual((r["ours"], r["oracle"], r["verdict"]), (4, 4, "agree"))
        self.assertEqual(r["oracle_key"], "SKILL.0.MISC-ACCHECK", "Acrobatics is an armor-check skill: MISC=2 carries the -2 penalty")

    def test_save_contribution_is_compared_without_the_ability_modifier(self):
        rows = self.rows_for([line("b:feat:great_fortitude", "Great Fortitude", 2, {"Save": "Fortitude"})])
        r = rows["target:Save:Fortitude"]
        # CHECK.0.MISC=+1 carries CON -1 -> the misc component is 2.
        self.assertEqual((r["ours"], r["oracle"], r["verdict"]), (2, 2, "agree"))

    def test_dodge_feeds_the_ac_dodge_component(self):
        rows = self.rows_for([line("b:feat:dodge", "Dodge", 1, "Ac", {"name": "Dodge", "mode": "Plain"})])
        r = rows["target:Ac:Dodge"]
        self.assertEqual((r["ours"], r["oracle"], r["verdict"], r["oracle_key"]), (1, 1, "agree", "AC.DODGE"))
        rows = self.rows_for([line("b:equipment:chain_shirt", "Chain Shirt", 4, "Ac", {"name": "Armor", "mode": "Plain"})])
        r = rows["target:Ac:Armor"]
        self.assertEqual((r["ours"], r["verdict"], r["oracle_key"]), (4, "disagree", "AC.ARMOR"))

    def test_an_absent_chassis_posture_is_unverifiable_not_a_zero_total(self):
        rows = self.rows_for([], baseline_ac=0, baseline_melee_attack=0, skills={"climb": 0, "swim": 0})
        self.assertEqual(rows["armor_class"]["verdict"], "unverifiable")
        self.assertEqual(rows["skill.climb"]["oracle_key"], "engine-posture-absent")

    def test_standalone_number_agrees_when_the_description_prints_it(self):
        rows = self.rows_for(
            [
                line("b:class_feature:barbarian_rage", "Rage", 6),
                line("b:class_feature:monk_stunning_fist", "Stunning Fist", 0, also=[["DC 15", {"Resolved": 15}]]),
            ]
        )
        self.assertEqual(rows["b:class_feature:barbarian_rage:value"]["verdict"], "agree")
        self.assertEqual(rows["b:class_feature:monk_stunning_fist:also:DC 15"]["verdict"], "agree")
        self.assertEqual(rows["b:class_feature:monk_stunning_fist:value"]["verdict"], "disagree")

    def test_a_planted_disagreement_surfaces_and_is_named(self):
        rows = self.rows_for([line("b:feat:acrobatic", "Acrobatic", 3, {"Skill": "acrobatics"})])
        r = rows["target:Skill:acrobatics"]
        self.assertEqual(r["verdict"], "disagree")
        self.assertEqual((r["ours"], r["oracle"]), (3, 4))
        self.assertIn("expr", r)
        summary = SP.summarize(list(rows.values()))
        self.assertEqual(summary["lines"]["disagree"], 1)
        self.assertEqual(summary["lines"]["compared"], 1)

    def test_a_target_pcgen_exports_nothing_for_is_unverifiable_not_agree(self):
        rows = self.rows_for([line("b:class_feature:x", "X", 2, {"Pool": "ki"})])
        r = rows["target:Pool:ki"]
        self.assertEqual(r["verdict"], "unverifiable")
        self.assertTrue(r["oracle_key"].startswith("no-component-export"))
        rows = self.rows_for([line("b:class_feature:y", "Unnamed Thing", 2)])
        self.assertEqual(rows["b:class_feature:y:value"]["verdict"], "unverifiable")

    def test_skill_misc_drops_the_armor_check_penalty_on_armor_check_skills(self):
        rows = self.rows_for([line("b:skill:climb", "Climb", 3, {"Skill": "climb"}, {"name": "ClassSkill", "mode": "Plain"})])
        r = rows["target:Skill:climb"]
        # SKILL.1.MISC=1 carries the chain shirt's -2 armor check penalty; the class-skill +3 is what the rule line contributes.
        self.assertEqual((r["ours"], r["oracle"], r["verdict"], r["oracle_key"]), (3, 3, "agree", "SKILL.1.MISC-ACCHECK"))
        rows = self.rows_for([line("b:skill:diplomacy", "Diplomacy", 3, {"Skill": "diplomacy"})])
        r = rows["target:Skill:diplomacy"]
        self.assertEqual((r["oracle"], r["verdict"], r["oracle_key"]), (3, "agree", "SKILL.2.MISC"))
        # an export without the ACHECK rows (an older template) compares the raw MISC
        export = SP.parse_export(EXPORT.replace("SKILL.1.ACHECK=YES\n", "").replace("ACCHECK=-2\n", ""))
        rows = {r["unit"]: r for r in SP.compare_character(ours([line("b:skill:climb", "Climb", 3, {"Skill": "climb"})]), export)}
        self.assertEqual((rows["target:Skill:climb"]["oracle"], rows["target:Skill:climb"]["oracle_key"], rows["target:Skill:climb"]["verdict"]), (1, "SKILL.1.MISC", "disagree"))

    def test_an_also_value_is_joined_in_its_own_role(self):
        rows = self.rows_for(
            [
                line("b:class_feature:paladin_detect_evil", "Detect Evil", 0, also=[["CL 1", {"Resolved": 1}], ["DC 13", {"Resolved": 13}], ["3/day", {"Resolved": 3}]]),
                line("b:class_feature:monk_stunning_fist", "Stunning Fist", 0, also=[["DC 15", {"Resolved": 15}], ["DC 3", {"Resolved": 3}]]),
                line("b:class_feature:barbarian_rage", "Rage", 0, also=[["6 rounds per days", {"Resolved": 6}], ["12/day", {"Resolved": 12}]]),
            ]
        )
        # Detect Evil's description prints 60 feet and 3 rounds, never a caster level, a DC or a per-day count.
        cl = rows["b:class_feature:paladin_detect_evil:also:CL 1"]
        self.assertEqual((cl["verdict"], cl["oracle_key"]), ("unverifiable", "DESC-has-no-CasterLevel:detect evil"))
        self.assertEqual(rows["b:class_feature:paladin_detect_evil:also:DC 13"]["oracle_key"], "DESC-has-no-SaveDc:detect evil")
        self.assertEqual(rows["b:class_feature:paladin_detect_evil:also:3/day"]["oracle_key"], "DESC-has-no-Uses:detect evil")
        self.assertEqual(rows["b:class_feature:monk_stunning_fist:also:DC 15"]["verdict"], "agree")
        self.assertEqual(rows["b:class_feature:monk_stunning_fist:also:DC 3"]["verdict"], "disagree")
        self.assertEqual(rows["b:class_feature:barbarian_rage:also:6 rounds per days"]["verdict"], "agree")
        r = rows["b:class_feature:barbarian_rage:also:12/day"]
        self.assertEqual((r["verdict"], r["oracle"]), ("disagree", [6]))
        summary = SP.summarize(list(rows.values()))
        self.assertEqual(summary["unverifiable_reasons"].get("DESC-has-no-CasterLevel"), 1)

    def test_chassis_totals_compare_and_a_wrong_one_disagrees(self):
        rows = self.rows_for([])
        self.assertEqual(rows["ability_mod.int"]["verdict"], "agree")
        self.assertEqual(rows["save.fortitude.total"]["verdict"], "agree")
        self.assertEqual(rows["skill.climb"]["verdict"], "agree")
        rows = self.rows_for([], bab=2)
        self.assertEqual(rows["base_attack_bonus"]["verdict"], "disagree")


class RosterTest(unittest.TestCase):
    def test_roster_members_pair_engine_and_pcgen_forms(self):
        members = SP.roster_members()
        self.assertEqual(len(members), len(SP.CLASSES) * len(SP.LEVELS) + len(SP.RACES) - 1)
        text = SP.engine_fixture_text("human_wizard_l10", "human", "wizard", 10)
        self.assertIn("class_level=class:wizard:10", text)
        self.assertIn("ability=intelligence:18", text)
        pcg = SP.pcg_text("human_wizard_l10", "Human", "Wizard", 10)
        self.assertIn("CLASS:Wizard|LEVEL:10", pcg)
        self.assertIn("STAT:INT|SCORE:18", pcg)
        self.assertIn("+2 Strength", pcg)
        self.assertNotIn("+2 Strength", SP.pcg_text("x", "Dwarf", "Fighter", 1))

    def test_racial_stat_bonuses_parse_from_the_pinned_row_shape(self):
        row = "+2 Constitution, +2 Wisdom, -2 Charisma\tKEY:Dwarf ~ Ability Scores\tCATEGORY:Special Ability\tBONUS:STAT|CON,WIS|2|TYPE=Racial\tBONUS:STAT|CHA|-2|TYPE=Racial\tDESC:x"
        self.assertEqual(SP.parse_racial_stat_bonuses(row), {"CON": 2, "WIS": 2, "CHA": -2})
        human = "+2 to One Ability Score\tKEY:Human ~ Ability Scores\tBONUS:ABILITYPOOL|Ability Bonus|1"
        self.assertEqual(SP.parse_racial_stat_bonuses(human), {})
        gated = "x\tBONUS:STAT|STR|2|TYPE=Racial|PREVARGTEQ:Foo,1"
        self.assertEqual(SP.parse_racial_stat_bonuses(gated), {}, "a gated bonus is not a fixed adjustment")

    def test_engine_fixture_prebakes_the_fixed_racial_adjustment_and_the_pcg_does_not(self):
        text = SP.engine_fixture_text("dwarf_fighter_l1", "dwarf", "fighter", 1, {"CON": 2, "WIS": 2, "CHA": -2})
        for expected in ("ability=constitution:10", "ability=wisdom:16", "ability=charisma:14", "ability=strength:12"):
            self.assertIn(expected, text)
        self.assertIn("STAT:CON|SCORE:8", SP.pcg_text("dwarf_fighter_l1", "Dwarf", "Fighter", 1))
        self.assertIn("ability=constitution:8", SP.engine_fixture_text("human_wizard_l1", "human", "wizard", 1, {}))

    def test_pinned_checkout_rows_when_the_checkout_is_present(self):
        if not os.path.isdir(os.path.join(SP.pcgen_repo_dir(), SP.PCGEN_RACES_DIR)):
            self.skipTest("no pinned PCGen checkout on this box")
        self.assertEqual(SP.racial_ability_adjustments("Dwarf"), {"CON": 2, "WIS": 2, "CHA": -2})
        self.assertEqual(SP.racial_ability_adjustments("Halfling"), {"DEX": 2, "CHA": 2, "STR": -2})
        self.assertEqual(SP.racial_ability_adjustments("Human"), {})

    def test_skill_names(self):
        self.assertEqual(SP.skill_name("knowledge_arcana"), "Knowledge (Arcana)")
        self.assertEqual(SP.skill_name("sleight_of_hand"), "Sleight of Hand")
        self.assertEqual(SP.skill_name("use_magic_device"), "Use Magic Device")


if __name__ == "__main__":
    unittest.main()
