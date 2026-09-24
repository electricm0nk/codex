//! SD-36 Epic F3b3 (4): the selected-skill check reads each class's class skills from its
//! CONVERTED record (`class_skill_sheet_rules`), not from a hand-kept class list.
//!
//! F3b found Barbarian printed Climb/Swim without the +3 class-skill bonus although its
//! converted record (`core_rulebook:class_feature:class_skills_barbarian`,
//! `cr_abilities_class.lst:2831`, CSKILL Acrobatics|Climb|TYPE=Craft|Handle Animal|Intimidate|
//! Knowledge (Nature)|Perception|Ride|Survival|Swim) grants all three.
//!
//! Hand-worked (the shared census fixture: Human, Str 16 + 2 = 18 (+4), Cha 8 (-1), 1 rank each
//! in Climb / Intimidate / Swim, chain shirt worn): CRB p.87 a class skill with at least 1 rank
//! gets +3; CRB p.150 Table 6-6 chain shirt armor check penalty -2 (Climb and Swim).
//!   Barbarian 1 Climb      = 1 + 4 + 3 - 2 = 6
//!   Barbarian 1 Intimidate = 1 - 1 + 3     = 3
//!   Barbarian 1 Swim       = 1 + 4 + 3 - 2 = 6

use codex::rules_core::class_census::{census, load_sweep_fixture};
use codex::rules_core::class_seeds::input_for;
use codex::rules_core::pilot_compute::class_skill_sheet_rules::{class_skill_view, ClassSkillAnswer};
use codex::rules_core::pilot_compute::{build_pilot_headless_receipt, PilotHeadlessReceipt};

fn value(receipt: &PilotHeadlessReceipt, id: &str) -> Option<i16> {
    receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
}

fn detail(receipt: &PilotHeadlessReceipt, id: &str) -> Option<String> {
    receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.detail.clone())
}

#[test]
fn barbarian_gets_the_class_skill_bonus_its_record_grants() {
    let fixture = load_sweep_fixture().expect("fixture");
    let receipt = build_pilot_headless_receipt(&input_for(&fixture, "barbarian", 1));
    assert_eq!(value(&receipt, "skill.selected_modifier.climb"), Some(6), "{:?}", detail(&receipt, "skill.selected_modifier.climb"));
    assert_eq!(value(&receipt, "skill.selected_modifier.intimidate"), Some(3), "{:?}", detail(&receipt, "skill.selected_modifier.intimidate"));
    assert_eq!(value(&receipt, "skill.selected_modifier.swim"), Some(6), "{:?}", detail(&receipt, "skill.selected_modifier.swim"));
}

/// The scan: every census class at level 1 whose converted record answers Known, and whose
/// single-class receipt prints a selected-skill line. For each of Climb / Intimidate / Swim the
/// line carries the class-skill bonus exactly when the record grants the skill. Printed with its
/// denominator; asserted 0 mismatches.
#[test]
fn every_census_class_prints_the_class_skill_bonus_its_record_grants() {
    let fixture = load_sweep_fixture().expect("fixture");
    let mut checked = 0usize;
    let mut known = 0usize;
    let mut mismatches = Vec::new();
    let classes = census();
    for entry in classes.values() {
        let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
        let ClassSkillAnswer::Known(view) = class_skill_view(slug, 1) else { continue };
        known += 1;
        let receipt = build_pilot_headless_receipt(&input_for(&fixture, slug, 1));
        for skill in ["climb", "intimidate", "swim"] {
            let Some(line) = detail(&receipt, &format!("skill.selected_modifier.{skill}")) else { continue };
            checked += 1;
            let printed = line.contains("class-skill bonus (+3)");
            if printed != view.contains(skill) {
                mismatches.push(format!("{} {skill}: record {} / sheet {}", entry.class_id, view.contains(skill), printed));
            }
        }
    }
    println!(
        "census ids {}; record Known at level 1: {known}; (class, skill) lines checked: {checked}; mismatches: {}",
        classes.len(),
        mismatches.len()
    );
    assert!(checked > 0);
    assert!(mismatches.is_empty(), "{} of {checked} lines disagree with the record: {mismatches:#?}", mismatches.len());
}

/// SD-36 Epic F3c2 (1): the two classes F3b3 left Blocked on
/// `skill.selected_modifier.class_skill_unknown`.
///
/// **Expert** (CRB p.450: "any 10 skills" are class skills). Its converted record carries the
/// pick (`core_rulebook:class:expert#bonus4`, `BONUS:ABILITYPOOL|Expert Class Skills|10`,
/// cr_classes.lst:549) and the chooser (`core_rulebook:class_feature:expert_class_skills`,
/// cr_abilities_class.lst:2735, `CHOOSE:SKILL|ALL`, `CSKILL:LIST` -> `ClassSkillChosen`). The
/// class's ten picks are a choice, so its Path-A canonical default is seeded through
/// `class_seeds` like every other choice: the first ten single skills of the CRB skill list
/// (Acrobatics, Appraise, Bluff, Climb, Diplomacy, Disable Device, Disguise, Escape Artist, Fly,
/// Handle Animal). Hand-worked on the shared census fixture (Human, Str 18 (+4), Cha 8 (-1),
/// 1 rank each, chain shirt ACP -2, CRB p.150):
///   Expert 1 Climb      = 1 + 4 + 3 - 2 = 6   (a class skill: the +3, CRB p.87)
///   Expert 1 Intimidate = 1 - 1         = 0   (not picked)
///   Expert 1 Swim       = 1 + 4 - 2     = 3   (not picked)
///
/// **Psion** stays Unknown by name: its base class skills (Autohypnosis, Craft, Knowledge,
/// Profession, Spellcraft) exist only on the discipline `SUBCLASS:` lines
/// (ultimate_psionics up_classes.lst:221-248), which the converter does not carry, and the
/// discipline's own `<Discipline> Class Skills` records have no granting edge. The record has no
/// answer, so the refusal keeps firing.
#[test]
fn expert_prints_its_canonical_class_skill_picks_and_psion_stays_refused_by_name() {
    let fixture = load_sweep_fixture().expect("fixture");
    let ClassSkillAnswer::Known(view) = class_skill_view("expert", 1) else {
        panic!("expert: {:?}", class_skill_view("expert", 1));
    };
    for skill in ["acrobatics", "appraise", "bluff", "climb", "diplomacy", "disable_device", "disguise", "escape_artist", "fly", "handle_animal"] {
        assert!(view.contains(skill), "{skill}: {view:?}");
    }
    assert_eq!(view.skills.len(), 10, "CRB p.450: ten skills: {view:?}");
    assert!(view.groups.is_empty() && !view.contains("intimidate") && !view.contains("swim"), "{view:?}");
    for level in [1u8, 20] {
        let receipt = build_pilot_headless_receipt(&input_for(&fixture, "expert", level));
        assert_eq!(value(&receipt, "skill.selected_modifier.climb"), Some(6), "{:?}", detail(&receipt, "skill.selected_modifier.climb"));
        assert_eq!(value(&receipt, "skill.selected_modifier.intimidate"), Some(0), "{:?}", detail(&receipt, "skill.selected_modifier.intimidate"));
        assert_eq!(value(&receipt, "skill.selected_modifier.swim"), Some(3), "{:?}", detail(&receipt, "skill.selected_modifier.swim"));
        assert!(
            !receipt.computation.diagnostics.iter().any(|d| d.id == "skill.selected_modifier.class_skill_unknown"),
            "expert {level}: {:?}",
            receipt.computation.diagnostics
        );
    }

    assert!(matches!(class_skill_view("psion", 1), ClassSkillAnswer::Unknown { .. }));
    let receipt = build_pilot_headless_receipt(&input_for(&fixture, "psion", 1));
    assert!(
        receipt.computation.diagnostics.iter().any(|d| d.id == "skill.selected_modifier.class_skill_unknown" && d.claim_blocking),
        "psion: {:?}",
        receipt.computation.diagnostics
    );
}
