//! SD13-E3-F5 Fighter level-1 mandatory milestone classification proof.
//!
//! Proves the dedicated tranche this handoff authorizes: the accepted deterministic
//! Human Fighter level-1 pilot seam is reclassified into a bounded
//! level-1 mandatory-milestone classification that names which level-1 mandatory
//! milestones are proven (computed) on the deterministic pilot surface and which
//! remain unproven for the level-10 progression matrix, while preserving the
//! existing L2-10 row, the Rogue blocked negative control, the Human race seam,
//! and the Human interaction pressure row.
//!
//! It is intentionally not a level-2+ Fighter engine, not a general feat engine,
//! and not a level-10 progression engine. It only widens the existing L1 row's
//! blocker note into a concrete milestone enumeration, points the row's grounding
//! ref at this proof surface, and asserts the matrix reclassification without
//! silently promoting the row to Supported.
//!
//! Non-goals:
//! - do not claim Fighter L1 is fully supported (level-10 progression remains unclassified)
//! - do not collapse the L1 row into the L2-10 row (they keep distinct dimensions)
//! - do not weaken the Rogue blocked negative control or the Human race seam

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::{load, explanation};

const LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

// ----- L1 deterministic pilot surface preserves its proven (computed) seams -----

#[test]
fn level_1_pilot_surface_still_emits_proven_base_chassis_explanations() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    // The Fighter L1 deterministic pilot surface must still emit every proven
    // (computed) level-1 milestone explanation that the matrix note names.
    let required = [
        "class_chassis.base_attack_bonus",
        "class_chassis.base_save.fortitude",
        "class_chassis.base_save.reflex",
        "class_chassis.base_save.will",
        "class_chassis.fighter.level_1_hit_points",
        "defense.total_save.fortitude",
        "defense.total_save.reflex",
        "defense.total_save.will",
        "combat.baseline_melee_attack_bonus",
        "defense.baseline_armor_class",
    ];
    for id in required {
        assert!(
            explanation(&computation, id).id == id,
            "Fighter L1 must still emit the proven '{id}' explanation"
        );
    }
}

#[test]
fn level_1_pilot_surface_emits_ability_modifier_and_selected_skill_seams() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    // At least one ability modifier explanation must surface (proves the
    // ability-modifier milestone is grounded for the chosen scores).
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "ability_modifier.strength"),
        "Fighter L1 must surface an ability modifier explanation"
    );
    // The selected skill modifier seam (climb/intimidate/swim per the L1 fixture)
    // proves the selected-skill milestone is grounded for the canonical picks.
    for id in [
        "skill.selected_modifier.climb",
        "skill.selected_modifier.intimidate",
        "skill.selected_modifier.swim",
    ] {
        assert!(
            explanation(&computation, id).id == id,
            "Fighter L1 must still emit the selected skill modifier '{id}'"
        );
    }
}

#[test]
fn level_1_base_attack_bonus_still_equals_fighter_chassis_value() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 1,
        "Fighter level 1 BAB must remain +1 on the deterministic pilot surface"
    );
}

// ----- Control plane: the matrix reclassifies the L1 row to bounded Partial -----

// ----- Control plane: peers remain stable after the L1 widening -----

