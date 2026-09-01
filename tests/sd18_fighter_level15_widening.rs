//! SD18 Fighter level-15 widening grounding proof.
//!
//! Widens the accepted Human Fighter level-1..level-14 chassis
//! (`tests/sd18_fighter_level14_widening.rs`, the loop's most recent Fighter
//! ceiling) to Fighter level 15 -- the loop's THIRD §3.2 level-15 landing,
//! after Barbarian (`tests/sd18_barbarian_level15_widening.rs`,
//! cycle-2026-07-15T2800) and Rogue (`tests/sd18_rogue_level15_widening.rs`,
//! cycle-2026-07-15T2900) -- mirroring the sibling per-level-gate idiom
//! (`supported_fighter_level` is generalized from `1..=14` to `1..=15` via
//! `MAX_SUPPORTED_FIGHTER_LEVEL = 15`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, and agreed byte-for-byte:
//!
//! - level 15 base attack bonus is +15/+10/+5 (full BAB, genuinely risen
//!   from +14); good Fortitude STAYS +9 (`15 / 2 + 2 = 9`, an
//!   integer-division coincidence with level 14), while poor Reflex and poor
//!   Will both GENUINELY RISE to +5 (`15 / 3 = 5`, up from +4 at level 14) --
//!   checked directly, not assumed.
//! - the PF1 Core Rulebook Fighter class table's level-15 "Special" column
//!   reads "Armor training 4" only (both primary sources agree, no other
//!   entry). Level 15 is NOT a bonus-feat cadence level (1, 2, 4, 6, 8, 10,
//!   12, 14, ...), so no eighth bonus-feat seam appears; Weapon Training's
//!   rank formula (`1 + (level - 5) / 4`) STAYS at rank 3
//!   (`1 + (15 - 5) / 4 = 1 + 2 = 3`), an integer-division coincidence with
//!   level 14 (the next rise is level 17); Bravery's already-generic formula
//!   (`1 + (level - 2) / 4`) STAYS +4 (`1 + 13 / 4 = 4`), an
//!   integer-division coincidence with level 14 (the next rise is level 18).
//!   Armor Training's already-level-generic rank function (already carrying
//!   three tiers: rank 1 at level 3, rank 2 at level 7, rank 3 at level 11)
//!   genuinely rises to rank 4 at level 15, raising the armor-check-penalty
//!   reduction and maximum-Dexterity-bonus increase magnitudes by one more
//!   named tier -- a magnitude widening on the same pre-existing formula
//!   shape, mirroring the Armor Training 3 landing at level 11 exactly. On
//!   the deterministic Chain Shirt fixture, the armor-check-penalty
//!   reduction was already capped at 0 by Armor Training 2, so this rise
//!   changes no derived Climb/Swim total, and the raised maximum Dexterity
//!   bonus changes no derived armor class either, since the deterministic
//!   +2 Dexterity contribution stays well below both the old and new caps --
//!   the rank rise itself is still a genuine, non-fabricated magnitude
//!   widening on the already-grounded pillar, not a new subsystem.
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, a general
//! feat-effect/prerequisite engine, or any Greater Weapon Specialization
//! damage-roll application, and it does not ground Fighter level 16+. It
//! also preserves the accepted Fighter level-1..level-14 truth (unchanged)
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level14_sd18_widening_deterministic_input.txt"
);

const FIGHTER_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level15_sd18_widening_deterministic_input.txt"
);

// ----- Base attack bonus and poor Reflex/Will genuinely rise; Fortitude stays at level 15 -----

#[test]
fn fighter_level15_base_attack_and_poor_saves_rise_fortitude_stays() {
    let input = load(FIGHTER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-15 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 15,
        "Fighter level 15 full-BAB progression must genuinely rise to 15, up from 14 at level \
         14: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Fighter level 15 good Fortitude (15/2+2) must stay 9, unchanged from level 14"
    );

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Fighter level 15 poor Reflex (15/3) must genuinely rise to 5, up from 4 at level 14"
    );

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(
        will.value, 5,
        "Fighter level 15 poor Will (15/3) must genuinely rise to 5, up from 4 at level 14"
    );
}

// ----- Armor Training genuinely rises to rank 4 at level 15 -----

#[test]
fn fighter_level15_armor_training_rises_to_rank_four() {
    let input = load(FIGHTER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(
        armor_training.value, 4,
        "Fighter Armor Training rank must genuinely rise to 4 at level 15, up from 3 at level \
         14: {}",
        armor_training.detail
    );
}

// ----- Weapon Training and Bravery both stay unchanged at level 15 (integer-division coincidences) -----

#[test]
fn fighter_level15_weapon_training_and_bravery_stay_unchanged() {
    let input = load(FIGHTER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 3,
        "Weapon Training's first-group attack bonus (1 + (15-5)/4) must stay +3 at level 15, an \
         integer-division coincidence with level 14 (the next rise is level 17): {}",
        weapon_training.detail
    );

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(
        bravery.value, 4,
        "Bravery (1 + (15-2)/4) must stay +4 at level 15, an integer-division coincidence with \
         level 14 (the next rise is level 18)"
    );

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_15_bonus_feat"),
        "level 15 is not a Fighter bonus-feat cadence level, so no eighth bonus-feat seam \
         should appear: {:?}",
        computation.explanations
    );

    // The level-14 bonus-feat seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_14_bonus_feat"),
        "level-15 Fighter must still carry the level-14 bonus-feat seam: {:?}",
        computation.explanations
    );
}

// ----- Baseline melee attack bonus rises by exactly the base-attack-bonus delta; AC unchanged -----

#[test]
fn fighter_level15_baseline_melee_attack_bonus_rises_armor_class_unchanged() {
    let input = load(FIGHTER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Baseline melee attack bonus rises by the base-attack-bonus delta (+1)
    // only, since Weapon Training's first-group bonus stays unchanged at
    // level 15: from 21 at level 14 to 22 at level 15.
    assert_eq!(computation.baseline_melee_attack_bonus, 23);

    // Baseline armor class is unchanged: the armor-check-penalty reduction
    // was already capped at 0 by Armor Training 2, and the deterministic +2
    // Dexterity contribution stays below both the old (+7) and new (+8)
    // maximum-Dexterity-bonus caps, so the rank-4 rise changes no derived
    // armor-class value on this fixture.
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

// ----- Negative control: the level-14 fixture is unaffected by this widening -----

#[test]
fn fighter_level14_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 14, "Fighter level 14 base attack bonus must stay 14");

    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(armor_training.value, 3, "Fighter level 14 Armor Training must stay rank 3");

    assert_eq!(computation.baseline_melee_attack_bonus, 22);
}

// ----- Negative control: level 16 is now supported (SD18, tests/sd18_fighter_level16_widening.rs) -----
//
// SD18 (tests/sd18_fighter_level16_widening.rs) widened the bounded tranche
// from level 15 to level 16 (an eighth bonus-feat cadence slot), so this
// slice's own level-16 boundary is superseded; the negative-control
// boundary for "beyond the bounded row" now lives in
// tests/sd18_fighter_level16_widening.rs at level 17. This slice keeps only
// the level-14/level-15 truth checks above unchanged.

// ----- Negative control: multiclass Fighter is not promoted -----

#[test]
fn multiclass_fighter_level15_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL15_FIXTURE.replace(
        "class_level=class:fighter:15",
        "class_level=class:fighter:15\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Fighter+Rogue multiclass mix into a
    // genuinely supported combination (via the table-driven
    // compute_generic_table_chassis path for Rogue), so this negative control is
    // superseded, not violated: Fighter level 15 / Rogue level 1 now gets a
    // real, integrated class_chassis.base_attack_bonus (Fighter 15's full BAB
    // 15 + Rogue 1's 3/4 BAB floor(1*3/4)=0, summed = 15), and with this
    // fixture's full deterministic combat/skill posture already matching, there is
    // no remaining claim-blocking diagnostic at all -- the receipt is genuinely
    // Computed, not Blocked.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.base_attack_bonus"),
        "multiclass Fighter/Rogue now genuinely gains the integrated base-attack-bonus \
         explanation: {:?}",
        computation.explanations
    );
    assert_eq!(
        computation.base_attack_bonus, 15,
        "Fighter 15 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 15"
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Fighter 15 / Rogue 1 with this fixture's full deterministic posture has no \
         remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_fighter_row_names_level_15_widening() {
    let matrix = seeded_current_truth();
    let fighter = matrix
        .row("class.fighter.levels_2_10")
        .expect("fighter levels_2_10 row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(fighter.support_state, SupportState::Supported);
    assert_eq!(fighter.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(fighter.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        fighter.grounding_ref.contains("sd18_fighter_level15_widening"),
        "fighter row must cite the live SD18 level-15 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "fighter partial note must name the level-15 widening: {note}"
    );
}
