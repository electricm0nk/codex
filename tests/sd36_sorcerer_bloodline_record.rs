//! SD-36 Epic F3c4 (FS-17, engine half): a sorcerer bloodline the bespoke Sorcerer module does
//! not model prints from its CONVERTED record, and the SD-32 generic pool-group pass no longer
//! prints bloodline member values at levels the record does not grant them.
//!
//! F3c4b converted each bloodline's `CATEGORY:Sorcerer Bloodline` pick row as a `pool_option`
//! granted by `Standard Bloodline`'s choice. This step links the character's Path-A pick
//! (`choice:sorcerer_bloodline -> bloodline:<x>`) to that option (one rule,
//! `sheet_rule::link_path_a_picks`), so the held set holds the option, the bloodline record and
//! its lines at the levels the record states.
//!
//! Hand-worked on the shared census fixture (Human; Str 16 + 2 = 18 (+4), Dex 14 (+2),
//! Con 14 (+2), Int 10, Wis 12, Cha 8 (-1); 1 rank each in Climb / Intimidate / Swim; chain shirt
//! worn, armor check penalty -2, CRB p.150):
//!
//! Draconic sorcerer 5 (CRB p.71-72 Table 3-14, p.75 Draconic):
//!   BAB +2; base saves Fort +1, Ref +1, Will +4.
//!   HP (d6, max at 1st, then the average 4, each + Con 2): 8 + 4 x 6 = 32.
//!   Class skill: Perception (Draconic, CRB p.75).
//!   Dragon Resistances (3rd): +1 natural armor (+2 at 9th, +4 at 15th).
//!   Not yet: Breath Weapon (9th), Wings (15th), Power of Wyrms (20th).
//!
//! Aquatic sorcerer (APG p.136): class skill Swim.
//!   Swim = 1 rank + Str 4 + class skill 3 (CRB p.87) - ACP 2 = 6.
//!   Climb = 1 + 4 - 2 = 3; Intimidate (a sorcerer class skill, CRB p.71) = 1 - 1 + 3 = 3.

use codex::rules_core::class_census::load_sweep_fixture;
use codex::rules_core::class_seeds::input_for;
use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::durability::compute_max_hp;
use codex::rules_core::pilot_compute::class_skill_sheet_rules::{class_skill_view_for, ClassSkillAnswer};
use codex::rules_core::pilot_compute::{build_pilot_headless_receipt, HeadlessReceiptStatus, PilotHeadlessReceipt};
use codex::rules_core::sheet_rule::SheetLine;
use codex::rules_core::sheet_rule_package;

fn sorcerer(bloodline: &str, level: u8) -> CharacterInput {
    let fixture = load_sweep_fixture().expect("fixture");
    let mut input = input_for(&fixture, "sorcerer", level);
    for c in input.chosen.selected_choices.iter_mut() {
        if c.choice_set_id == "choice:sorcerer_bloodline" {
            c.selection_id = format!("bloodline:{bloodline}");
        }
    }
    if bloodline != "arcane" {
        input.chosen.selected_choices.retain(|c| c.choice_set_id != "choice:sorcerer_arcane_bond");
    }
    input
}

fn value(receipt: &PilotHeadlessReceipt, id: &str) -> Option<i16> {
    receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
}

fn detail(receipt: &PilotHeadlessReceipt, id: &str) -> Option<String> {
    receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.detail.clone())
}

fn blockers(receipt: &PilotHeadlessReceipt) -> Vec<String> {
    receipt.computation.diagnostics.iter().filter(|d| d.claim_blocking).map(|d| format!("{}: {}", d.id, d.message)).collect()
}

fn sheet(input: &CharacterInput) -> Vec<SheetLine> {
    let package = sheet_rule_package::package().as_ref().expect("package");
    let receipt = build_pilot_headless_receipt(input);
    receipt.computation.with_sheet_rules(input, package, &[]).sheet_lines
}

fn generic_ids(receipt: &PilotHeadlessReceipt, fragment: &str) -> Vec<(String, i16)> {
    receipt
        .computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with("class_feature.sorcerer.bloodline.generic.") && e.id.contains(fragment))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

#[test]
fn a_sorcerer_with_the_draconic_bloodline_computes_at_level_5() {
    let input = sorcerer("draconic", 5);
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "{:#?}", blockers(&receipt));
    assert_eq!(value(&receipt, "class_chassis.base_attack_bonus"), Some(2));
    assert_eq!(value(&receipt, "class_chassis.base_save.fortitude"), Some(1));
    assert_eq!(value(&receipt, "class_chassis.base_save.reflex"), Some(1));
    assert_eq!(value(&receipt, "class_chassis.base_save.will"), Some(4));
    assert_eq!(compute_max_hp(&input.chosen.class_levels, 2), Some(32));
    // Dragon Resistances' +1 natural armor (bespoke, CRB p.75) reaches AC: 17 (the Arcane
    // sorcerer's AC on the same fixture) + 1.
    assert_eq!(value(&receipt, "class_feature.sorcerer.draconic_bloodline.dragon_resistances.natural_armor_bonus"), Some(1));
    assert_eq!(value(&receipt, "defense.baseline_armor_class"), Some(18));
    let lines = sheet(&input);
    let ids: Vec<&str> = lines.iter().map(|l| l.id.as_str()).collect();
    // The bloodline class skill: Perception, held through the record's gated grant.
    assert!(ids.contains(&"core_rulebook:ability:perception"), "{ids:#?}");
    for held in [
        "core_rulebook:class_feature:sorcerer_bloodline_draconic",
        "core_rulebook:class_feature:draconic_bloodline_bloodline_arcana",
        "core_rulebook:class_feature:draconic_bloodline_bonus_spells",
        "core_rulebook:class_feature:draconic_bloodline_bloodline_powers",
        "core_rulebook:class_feature:draconic_bloodline_dragon_resistances",
    ] {
        assert!(ids.contains(&held), "{held} is printed at sorcerer 5: {ids:#?}");
    }
    for absent in [
        "core_rulebook:class_feature:draconic_bloodline_breath_weapon",
        "core_rulebook:class_feature:draconic_bloodline_wings",
        "core_rulebook:class_feature:draconic_bloodline_power_of_wyrms",
    ] {
        assert!(!ids.contains(&absent), "{absent} is not granted at sorcerer 5: {ids:#?}");
    }
    let resistances = lines.iter().find(|l| l.id == "core_rulebook:class_feature:draconic_bloodline_dragon_resistances").unwrap();
    assert_eq!(resistances.printed, "+1", "{resistances:?}");
}

#[test]
fn a_draconic_sorcerer_9_holds_breath_weapon_and_the_second_natural_armor_step() {
    let lines = sheet(&sorcerer("draconic", 9));
    let find = |id: &str| lines.iter().find(|l| l.id == id);
    assert!(find("core_rulebook:class_feature:draconic_bloodline_breath_weapon").is_some());
    assert_eq!(find("core_rulebook:class_feature:draconic_bloodline_dragon_resistances").map(|l| l.printed.as_str()), Some("+2"));
    assert!(find("core_rulebook:class_feature:draconic_bloodline_power_of_wyrms").is_none());
}

#[test]
fn an_aquatic_sorcerer_prints_swim_6_on_the_census_fixture() {
    for level in [1u8, 5, 20] {
        let receipt = build_pilot_headless_receipt(&sorcerer("aquatic", level));
        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "aquatic {level}: {:#?}", blockers(&receipt));
        assert_eq!(value(&receipt, "skill.selected_modifier.swim"), Some(6), "{:?}", detail(&receipt, "skill.selected_modifier.swim"));
        assert_eq!(value(&receipt, "skill.selected_modifier.climb"), Some(3), "{:?}", detail(&receipt, "skill.selected_modifier.climb"));
        assert_eq!(value(&receipt, "skill.selected_modifier.intimidate"), Some(3), "{:?}", detail(&receipt, "skill.selected_modifier.intimidate"));
    }
}

/// The Arcane sorcerer's Swim is unchanged (Arcane grants a Knowledge skill, not Swim):
/// 1 + 4 - 2 = 3.
#[test]
fn an_arcane_sorcerer_still_prints_swim_3() {
    let receipt = build_pilot_headless_receipt(&sorcerer("arcane", 5));
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "{:#?}", blockers(&receipt));
    assert_eq!(value(&receipt, "skill.selected_modifier.swim"), Some(3));
}

/// The class-skill union reads the character's own pick: Draconic adds Perception (CRB p.75),
/// Aquatic adds Swim (APG p.136); Arcane adds neither.
#[test]
fn the_class_skill_union_reads_the_characters_bloodline_pick() {
    let known = |bloodline: &str, level: u8| match class_skill_view_for(&sorcerer(bloodline, level), "sorcerer", level) {
        ClassSkillAnswer::Known(view) => view,
        ClassSkillAnswer::Unknown { reason } => panic!("{bloodline}: {reason}"),
    };
    assert!(known("draconic", 5).contains("perception"));
    assert!(!known("draconic", 5).contains("swim"));
    assert!(known("aquatic", 1).contains("swim"));
    assert!(!known("arcane", 5).contains("perception") && !known("arcane", 5).contains("swim"));
    for skill in ["bluff", "intimidate", "spellcraft"] {
        assert!(known("draconic", 5).contains(skill), "the sorcerer's own class skill {skill} stays");
    }
}

// --- (2) the SD-32 generic pool-group pass: no bloodline member at a level the record does not grant.

#[test]
fn a_draconic_sorcerer_5_prints_no_breath_weapon_dc() {
    let receipt = build_pilot_headless_receipt(&sorcerer("draconic", 5));
    assert_eq!(generic_ids(&receipt, "breath_weapon"), Vec::<(String, i16)>::new(), "Breath Weapon is a 9th-level power (CRB p.75)");
}

#[test]
fn a_draconic_sorcerer_5_prints_no_power_of_wyrms_blindsense() {
    let receipt = build_pilot_headless_receipt(&sorcerer("draconic", 5));
    assert_eq!(generic_ids(&receipt, "power_of_wyrms"), Vec::<(String, i16)>::new(), "Power of Wyrms is a 20th-level power (CRB p.75)");
}

#[test]
fn a_draconic_sorcerer_5_prints_no_natural_armor_2() {
    let receipt = build_pilot_headless_receipt(&sorcerer("draconic", 5));
    assert_eq!(generic_ids(&receipt, "dragon_resistances"), Vec::<(String, i16)>::new(), "+1 at 3rd, +2 only from 9th (CRB p.75)");
    assert_eq!(value(&receipt, "class_feature.sorcerer.draconic_bloodline.dragon_resistances.natural_armor_bonus"), Some(1));
}

#[test]
fn an_arcane_sorcerer_5_prints_no_arcane_apotheosis() {
    let receipt = build_pilot_headless_receipt(&sorcerer("arcane", 5));
    assert_eq!(generic_ids(&receipt, "arcane_apotheosis"), Vec::<(String, i16)>::new(), "Arcane Apotheosis is a 20th-level power (CRB p.74)");
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "{:#?}", blockers(&receipt));
}

/// An invented bloodline links to nothing: the bespoke blocker stands (never a silent pass).
#[test]
fn an_invented_bloodline_stays_blocked_by_name() {
    let receipt = build_pilot_headless_receipt(&sorcerer("not_a_real_bloodline", 5));
    assert!(
        receipt.computation.diagnostics.iter().any(|d| d.claim_blocking && d.id == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"),
        "{:#?}",
        blockers(&receipt)
    );
}

/// The scan: every converted sorcerer bloodline record (32), at sorcerer 1. The record's own
/// class-skill edges (a `Granter::Rule(<record>)` edge onto a rule that grants
/// `Fact::ClassSkill`) name its bloodline class skill independently of the class-skill reader.
/// For Climb and Swim (Intimidate is a sorcerer class skill already, CRB p.71) the printed line
/// carries the +3 exactly when the record names the skill -- or the sheet is Blocked by name.
/// Printed with its denominator; asserted 0 mismatches.
#[test]
fn every_bloodline_prints_the_class_skill_bonus_its_record_grants() {
    use codex::rules_core::sheet_rule::{Effect, Fact, Granter};
    let package = sheet_rule_package::package().as_ref().expect("package");
    let mut records: Vec<String> = package
        .slugs_of_kind("class_feature")
        .filter(|s| s.starts_with("sorcerer_bloodline_") && !s.starts_with("sorcerer_bloodline_feat_"))
        .map(str::to_owned)
        .collect();
    records.sort();
    let (mut checked, mut blocked, mut named) = (0usize, 0usize, 0usize);
    let mut mismatches = Vec::new();
    for slug in &records {
        let id = package.find("class_feature", slug).unwrap().clone();
        let bloodline = slug.strip_prefix("sorcerer_bloodline_").unwrap();
        let record_skills: Vec<String> = package
            .rules
            .values()
            .filter(|r| r.granted_by.iter().any(|g| matches!(&g.by, Granter::Rule(x) if x == &id)))
            .flat_map(|r| r.grants.iter())
            .filter_map(|e| match e {
                Effect::FactGrant(Fact::ClassSkill(s)) => Some(s.clone()),
                _ => None,
            })
            .collect();
        named += usize::from(!record_skills.is_empty());
        let receipt = build_pilot_headless_receipt(&sorcerer(bloodline, 1));
        if receipt.status != HeadlessReceiptStatus::Computed {
            blocked += 1;
            continue;
        }
        for skill in ["climb", "swim"] {
            let line = detail(&receipt, &format!("skill.selected_modifier.{skill}")).unwrap_or_default();
            checked += 1;
            let printed = line.contains("class-skill bonus (+3)");
            if printed != record_skills.iter().any(|s| s == skill) {
                mismatches.push(format!("{bloodline} {skill}: record {record_skills:?} / sheet {printed}"));
            }
        }
    }
    println!(
        "bloodline records {}; naming a class skill {named}; Computed at 1: {}; (bloodline, skill) lines checked {checked}; mismatches {}",
        records.len(),
        records.len() - blocked,
        mismatches.len()
    );
    assert_eq!(records.len(), 32);
    assert!(mismatches.is_empty(), "{mismatches:#?}");
}
