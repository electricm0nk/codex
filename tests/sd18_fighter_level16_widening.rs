//! SD18 Fighter level-16 widening grounding proof.
//!
//! Widens the accepted Human Fighter level-1..level-15 chassis
//! (`tests/sd18_fighter_level15_widening.rs`, the loop's most recent Fighter
//! ceiling) to Fighter level 16 -- continuing the §3.2 level-16 sweep opened
//! by Barbarian (`tests/sd18_barbarian_level16_widening.rs`,
//! cycle-2026-07-15T4600) -- mirroring the sibling per-level-gate idiom
//! (`supported_fighter_level` is generalized from `1..=15` to `1..=16` via
//! `MAX_SUPPORTED_FIGHTER_LEVEL = 16`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, and agreed byte-for-byte:
//!
//! - level 16 base attack bonus is +16/+11/+6/+1 (full BAB, genuinely risen
//!   from +15); good Fortitude GENUINELY RISES to +10 (`16 / 2 + 2 = 10`, up
//!   from +9 at level 15), while poor Reflex and poor Will both STAY +5
//!   (`16 / 3 = 5`, an integer-division coincidence with level 15) --
//!   checked directly, not assumed.
//! - the PF1 Core Rulebook Fighter class table's level-16 "Special" column
//!   reads "Bonus feat" only (both primary sources agree, no other entry).
//!   Level 16 IS a Fighter bonus-feat cadence level (1, 2, 4, 6, 8, 10, 12,
//!   14, 16, ...), so this widening surfaces an EIGHTH named bonus-feat
//!   progression seam (`choice:fighter_bonus_feat_16`), mirroring the
//!   level-2 through level-14 seams exactly -- no general feat-effect or
//!   prerequisite engine invented. The canonical Critical Focus selection's
//!   prerequisite (base attack bonus +9) is honestly met by the level-16
//!   base attack bonus of +16. Weapon Training's rank formula
//!   (`1 + (level - 5) / 4`) STAYS at rank 3 (`1 + (16 - 5) / 4 = 1 + 2 = 3`),
//!   an integer-division coincidence with level 15 (the next rise is level
//!   17); Armor Training stays at rank 4 (the PF1 Core Rulebook names no
//!   fifth Armor Training rank); Bravery's already-generic formula
//!   (`1 + (level - 2) / 4`) STAYS +4 (`1 + 14 / 4 = 4`), an
//!   integer-division coincidence with level 15 (the next rise is level 18).
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, a general
//! feat-effect/prerequisite engine, or any Critical Focus critical-confirm
//! application, and it does not ground Fighter level 17+. It also preserves
//! the accepted Fighter level-1..level-15 truth (unchanged) and the
//! multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const FIGHTER_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level15_sd18_widening_deterministic_input.txt"
);

const FIGHTER_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level16_sd18_widening_deterministic_input.txt"
);

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

// ----- Base attack bonus and good Fortitude genuinely rise; poor Reflex/Will stay at level 16 -----

#[test]
fn fighter_level16_base_attack_and_fortitude_rise_poor_saves_stay() {
    let input = load(FIGHTER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-16 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 16,
        "Fighter level 16 full-BAB progression must genuinely rise to 16, up from 15 at level \
         15: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Fighter level 16 good Fortitude (16/2+2) must genuinely rise to 10, up from 9 at level \
         15"
    );

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Fighter level 16 poor Reflex (16/3) must stay 5, an integer-division coincidence with \
         level 15"
    );

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(
        will.value, 5,
        "Fighter level 16 poor Will (16/3) must stay 5, an integer-division coincidence with \
         level 15"
    );
}

// ----- Eighth bonus-feat seam appears at level 16 -----

#[test]
fn fighter_level16_eighth_bonus_feat_seam_appears() {
    let input = load(FIGHTER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_16_bonus_feat"),
        "level 16 is a Fighter bonus-feat cadence level, so an eighth bonus-feat seam should \
         appear: {:?}",
        computation.explanations
    );

    // The level-14 bonus-feat seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_14_bonus_feat"),
        "level-16 Fighter must still carry the level-14 bonus-feat seam: {:?}",
        computation.explanations
    );
}

// ----- Weapon Training, Armor Training, and Bravery all stay unchanged at level 16 (integer-division coincidences / no fifth Armor Training rank) -----

#[test]
fn fighter_level16_weapon_training_armor_training_and_bravery_stay_unchanged() {
    let input = load(FIGHTER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 3,
        "Weapon Training's first-group attack bonus (1 + (16-5)/4) must stay +3 at level 16, an \
         integer-division coincidence with level 15 (the next rise is level 17): {}",
        weapon_training.detail
    );

    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(
        armor_training.value, 4,
        "Fighter Armor Training rank must stay 4 at level 16: the PF1 Core Rulebook names no \
         fifth Armor Training rank: {}",
        armor_training.detail
    );

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(
        bravery.value, 4,
        "Bravery (1 + (16-2)/4) must stay +4 at level 16, an integer-division coincidence with \
         level 15 (the next rise is level 18)"
    );
}

// ----- Baseline melee attack bonus rises by exactly the base-attack-bonus delta; AC unchanged -----

#[test]
fn fighter_level16_baseline_melee_attack_bonus_rises_armor_class_unchanged() {
    let input = load(FIGHTER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Baseline melee attack bonus rises by the base-attack-bonus delta (+1)
    // only, since Weapon Training's first-group bonus stays unchanged at
    // level 16: from 22 at level 15 to 23 at level 16.
    assert_eq!(computation.baseline_melee_attack_bonus, 23);

    // Baseline armor class is unchanged: Armor Training stays rank 4 (no
    // fifth rank exists), so the armor-check-penalty reduction and maximum
    // Dexterity bonus stay at their level-15 values, and neither changes any
    // derived Climb/Swim total or armor-class value on this fixture.
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.selected_skill_modifiers.climb, 7);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 7);
}

// ----- Negative control: the level-15 fixture is unaffected by this widening -----

#[test]
fn fighter_level15_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 15, "Fighter level 15 base attack bonus must stay 15");

    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(armor_training.value, 4, "Fighter level 15 Armor Training must stay rank 4");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_16_bonus_feat"),
        "level-15 Fighter must not gain the level-16 bonus-feat seam: {:?}",
        computation.explanations
    );

    assert_eq!(computation.baseline_melee_attack_bonus, 22);
}

// ----- Negative control: level 17 stays claim-blocked (beyond the bounded L2-16 row) -----

#[test]
fn fighter_level_17_stays_claim_blocked() {
    let level_17 = FIGHTER_LEVEL16_FIXTURE.replace("class:fighter:16", "class:fighter:17");
    let input = load(&level_17);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-17 Fighter must stay claim-blocked beyond the bounded levels-2-16 row: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.base_attack_bonus"),
        "level-17 Fighter must not fabricate a base-attack-bonus explanation"
    );
}

// ----- Negative control: multiclass Fighter is not promoted -----

#[test]
fn multiclass_fighter_level16_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL16_FIXTURE.replace(
        "class_level=class:fighter:16",
        "class_level=class:fighter:16\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.base_attack_bonus"),
        "multiclass Fighter must not gain the bounded level-16 base-attack-bonus explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Fighter must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-16 widening -----

#[test]
fn matrix_fighter_row_names_level_16_widening() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let fighter = matrix
        .row("class.fighter.levels_2_10")
        .expect("fighter levels_2_10 row must exist");

    assert_eq!(fighter.support_state, SupportState::Partial);
    assert_eq!(fighter.evidence_tier, EvidenceTier::Computed);
    assert_eq!(fighter.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        fighter.grounding_ref.contains("sd18_fighter_level16_widening"),
        "fighter row must cite the live SD18 level-16 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 16") || note.contains("level-16"),
        "fighter partial note must name the level-16 widening: {note}"
    );
}
