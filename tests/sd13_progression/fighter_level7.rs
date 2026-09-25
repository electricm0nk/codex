//! SD13-E3 Fighter level 7 milestone progression proof.
//!
//! Widens the accepted deterministic Human Fighter pilot seam from the bounded
//! levels 1-6 tranche to level 7: base attack / base save progression continues
//! generically, and Armor Training 2 is surfaced as an explicit progression seam,
//! mirroring the level-3 Armor Training 1 pattern. Armor Training 2 further
//! reduces the worn Chain Shirt's armor-check penalty (from -1 at Armor Training 1
//! to 0 at Armor Training 2), which raises the Climb and Swim selected-skill
//! totals by +1 each; it also further raises the maximum Dexterity bonus, but that
//! stays above the deterministic +2 Dexterity contribution, so it changes no
//! armor-class value on this fixture. The Fighter bonus-feat cadence (1, 2, 4, 6,
//! 8, 10, ...) grants no new feat at level 7, so no new bonus-feat seam is
//! asserted here.
//!
//! It is intentionally not a broad martial engine. It grounds only the level-7
//! base attack / base save delta and the Armor Training 2 seam. It asserts no
//! level-8+ Fighter burden (the next bonus feat is level 8), no spell burden, no
//! non-Fighter positive support, and no general feat/prerequisite engine.

use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use crate::common::{load, explanation, has_explanation};

const LEVEL_7_FIXTURE: &str =
    include_str!("../fixtures/rules_core/pf1_human_fighter_level7_sd13_deterministic_input.txt");
const LEVEL_6_FIXTURE: &str =
    include_str!("../fixtures/rules_core/pf1_human_fighter_level6_sd13_deterministic_input.txt");

// ----- Milestone: level 7 is no longer blanket-blocked -----

#[test]
fn level_7_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-7 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-7 base chassis: full BAB +7, good Fortitude +5, poor Reflex/Will +2.
    assert_eq!(computation.base_attack_bonus, 7);
    assert_eq!(computation.base_saves.fortitude, 5);
    assert_eq!(computation.base_saves.reflex, 2);
    assert_eq!(computation.base_saves.will, 2);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 7);
    assert!(
        bab.detail.contains("level 7"),
        "level-7 BAB explanation must name the level-7 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier (CON +2, DEX +2, WIS +1).
    assert_eq!(computation.total_saves.fortitude, 7);
    assert_eq!(computation.total_saves.reflex, 4);
    assert_eq!(computation.total_saves.will, 3);

    // Baseline combat: +7 BAB + STR +3 + Weapon Focus +1 + Weapon Training +1 = 12.
    assert_eq!(computation.baseline_melee_attack_bonus, 13);
    // Armor class is unchanged: the deterministic +2 Dexterity contribution is
    // already below both the Armor Training 1 and Armor Training 2 max-Dex caps.
    assert_eq!(computation.baseline_armor_class, 17);

    // Armor Training 2 reduces the effective Chain Shirt armor-check penalty from
    // -1 (Armor Training 1) to 0, so Climb and Swim (which apply the armor-check
    // penalty) each gain +1 over the level-3-6 tranche. Intimidate is
    // Charisma-based and carries no armor-check penalty, so it is unchanged.
    // CG-03 fix: STR modifier is now +4 (base 16 + 2 Human racial), not +3, adding
    // a further +1 on top of the Armor Training 2 rise.
    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

#[test]
fn level_7_armor_training_2_seam_is_explicit_and_bounded() {
    let input = load(LEVEL_7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(
        seam.value, 2,
        "level-7 armor-training seam must carry rank 2: {seam:?}"
    );
    assert!(
        seam.detail.contains("Armor Training 2"),
        "level-7 armor-training seam must name Armor Training 2: {}",
        seam.detail
    );
}

#[test]
fn level_7_still_carries_every_earlier_seam() {
    let input = load(LEVEL_7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_feature.fighter.level_2_bonus_feat",
        "class_feature.fighter.level_4_bonus_feat",
        "class_feature.fighter.level_6_bonus_feat",
        "class_feature.fighter.weapon_training",
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-7 Fighter must still carry the earlier seam '{id}': {:?}",
            computation.explanations
        );
    }
}

// ----- Milestone: downstream propagation of the widened bounded truth -----

#[test]
fn level_7_propagates_computed_receipt_and_view_model() {
    let input = load(LEVEL_7_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "widened level-7 Fighter tranche must propagate a computed receipt"
    );

    assert!(
        has_explanation(&receipt.computation, "race.human.ability_bonus_target"),
        "widened level-7 receipt must preserve the Human ability-bonus race seam: {:?}",
        receipt.computation.explanations
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
    assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("computed level-7 receipt must yield a snapshot");
    assert_eq!(
        snapshot.base_attack_bonus, receipt.computation.base_attack_bonus,
        "view-model snapshot must carry the widened level-7 base attack bonus"
    );
}

// ----- Negative control: level 8 was later widened into the supported tranche -----

#[test]
fn level_8_fighter_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 8 (the next bonus feat) was the
    // next unproven milestone and stayed claim-blocked. A later SD13-E3 slice
    // (tests/sd13_fighter_level8_progression.rs) widened level 8 into the
    // supported tranche; this negative control is superseded, not violated —
    // pin the new truth here too so this file stays internally consistent.
    let level_8 = LEVEL_7_FIXTURE.replace("class:fighter:7", "class:fighter:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-8 Fighter is supported since the SD13-E3 level-8 slice: {:?}",
        computation.diagnostics
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "level-8 Fighter must surface a computed base-attack-bonus explanation"
    );
}

// ----- Control plane: the matrix widens the levels-2-10 row's proven range to level 7 -----

// Preserve the level-6-only fixture as a still-valid, unchanged reference point.
#[test]
fn level_6_fixture_still_loads_and_computes_unaffected_by_the_level_7_widening() {
    let input = load(LEVEL_6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    assert_eq!(computation.base_attack_bonus, 6);
    assert_eq!(computation.baseline_melee_attack_bonus, 12);
    assert!(!computation.diagnostics.iter().any(|d| d.claim_blocking));
}
