//! SD18 Fighter level-19 widening grounding proof.
//!
//! Widens the accepted Human Fighter level-1..level-18 chassis
//! (`tests/sd18_fighter_level18_widening.rs`, the loop's most recent Fighter
//! ceiling) to Fighter level 19 -- the third class to reach level 19 in the
//! §3.2 level-19 sweep, after Barbarian and Cleric -- mirroring the sibling
//! per-level-gate idiom (`supported_fighter_level` is generalized from
//! `1..=18` to `1..=19` via `MAX_SUPPORTED_FIGHTER_LEVEL = 19`). Both PF1 CRB
//! primary sources (`d20pfsrd.com/classes/core-classes/fighter/` and
//! `aonprd.com/ClassDisplay.aspx?ItemName=Fighter`) were read directly
//! (raw curl + tag-strip, not summarized) before writing any code or test,
//! fetching the full levels-16-through-20 block in one pass so the level-19
//! row's neighbors were visible in context (guards against
//! level-misattribution), and agreed byte-for-byte:
//!
//! - level 17: "+17/+12/+7/+2 | +10 | +5 | +5 | Weapon training [4]"
//! - level 18: "+18/+13/+8/+3 | +11 | +6 | +6 | Bonus feat, bravery +5"
//! - level 19: "+19/+14/+9/+4 | +11 | +6 | +6 | Armor mastery"
//! - level 20: "+20/+15/+10/+5 | +12 | +6 | +6 | Bonus feat, Weapon mastery"
//!
//! This confirms: base attack bonus GENUINELY RISES to 19 (full BAB, up from
//! 18), while ALL THREE base saves STAY NUMERICALLY UNCHANGED from level 18
//! (good Fortitude `19 / 2 + 2 = 11`, poor Reflex/Will both `19 / 3 = 6`),
//! integer-division coincidences -- checked directly, not assumed. The
//! Special column reads "Armor mastery" only: NOT a bonus-feat cadence level
//! (1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20 -- 19 is absent), NOT a Weapon
//! Training rank-rise level (ranks rise at 5, 9, 13, 17; the next rise, if
//! any, is beyond level 20), and NOT an Armor Training rank-rise level
//! (ranks rise at 3, 7, 11, 15; the PF1 Core Rulebook names no fifth rank),
//! so none of those three pillars widen at level 19, and Bravery's magnitude
//! stays +5 (an integer-division coincidence with level 18, since the next
//! rise is level 22, beyond the PF1 level cap).
//!
//! Armor Mastery (Ex) itself IS a genuinely new named class feature (per
//! aonprd.com: "At 19th level, a fighter gains DR 5/-- whenever he is
//! wearing armor or using a shield"). This is grounded as a bounded
//! flat-magnitude record only (`class_feature.fighter.armor_mastery`, value
//! 5 at or above level 19), mirroring EXACTLY the already-proven Barbarian
//! Damage Reduction idiom (`class_feature.barbarian.damage_reduction`): no
//! damage-resolution engine and no incoming-damage total exists anywhere in
//! this codebase to apply it, and no worn-armor-or-shield condition check is
//! computed, so this grounds no actual damage reduction. No new subsystem is
//! invented.
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, a general
//! feat-effect/prerequisite engine, or any actual damage-reduction
//! application, and it does not ground Fighter level 20 (Bonus feat, Weapon
//! mastery -- the capstone). It also preserves the accepted Fighter
//! level-1..level-18 truth (unchanged) and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level18_sd18_widening_deterministic_input.txt"
);

const FIGHTER_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level19_sd18_widening_deterministic_input.txt"
);

// ----- Base attack bonus genuinely rises at level 19; all three base saves stay unchanged -----

#[test]
fn fighter_level19_base_attack_rises_saves_stay_unchanged() {
    let input = load(FIGHTER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-19 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 19,
        "Fighter level 19 full-BAB progression must genuinely rise to 19, up from 18 at level \
         18: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(
        fortitude.value, 11,
        "Fighter level 19 good Fortitude (19/2+2) must stay 11, unchanged from level 18, an \
         integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Fighter level 19 poor Reflex (19/3) must stay 6, unchanged from level 18, an \
         integer-division coincidence"
    );

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(
        will.value, 6,
        "Fighter level 19 poor Will (19/3) must stay 6, unchanged from level 18, an \
         integer-division coincidence"
    );
}

// ----- No new bonus-feat seam at level 19; Bravery stays unchanged -----

#[test]
fn fighter_level19_no_new_bonus_feat_seam_bravery_unchanged() {
    let input = load(FIGHTER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_19_bonus_feat"),
        "level 19 is NOT a Fighter bonus-feat cadence level (1, 2, 4, 6, 8, 10, 12, 14, 16, 18, \
         20), so no tenth bonus-feat seam should appear: {:?}",
        computation.explanations
    );

    // The level-18 bonus-feat seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_18_bonus_feat"),
        "level-19 Fighter must still carry the level-18 bonus-feat seam: {:?}",
        computation.explanations
    );

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(
        bravery.value, 5,
        "Bravery (1 + (19-2)/4 = 1 + 4 = 5) must stay +5 at level 19, unchanged from level 18, \
         an integer-division coincidence (the next rise is level 22, beyond the PF1 level cap)"
    );
}

// ----- Weapon Training and Armor Training both stay unchanged at level 19 -----

#[test]
fn fighter_level19_weapon_training_and_armor_training_stay_unchanged() {
    let input = load(FIGHTER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 4,
        "Weapon Training's first-group attack bonus (1 + (19-5)/4) must stay +4 at level 19, \
         unchanged from level 18: {}",
        weapon_training.detail
    );

    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(
        armor_training.value, 4,
        "Fighter Armor Training rank must stay 4 at level 19: the PF1 Core Rulebook names no \
         fifth Armor Training rank: {}",
        armor_training.detail
    );

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.weapon_training_group_5"),
        "level 19 must not fabricate a fifth weapon-training-group seam: {:?}",
        computation.explanations
    );
}

// ----- Armor Mastery: a genuinely new level-19 class feature, flat-magnitude only -----

#[test]
fn fighter_level19_armor_mastery_flat_magnitude_record() {
    let input = load(FIGHTER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let armor_mastery = explanation(&computation, "class_feature.fighter.armor_mastery");
    assert_eq!(
        armor_mastery.value, 5,
        "Armor Mastery grants DR 5/-- at level 19 (verified against d20pfsrd and aonprd), \
         grounded as a flat, non-fabricated magnitude record only: {}",
        armor_mastery.detail
    );
    assert!(
        armor_mastery.detail.contains("damage-resolution"),
        "Armor Mastery's record must honestly disclaim the absent damage-resolution engine, \
         mirroring the Barbarian Damage Reduction idiom: {}",
        armor_mastery.detail
    );

    // The level-18 fixture must not carry this record at all (correct PF1
    // Core Rulebook level-gate absence).
    let level18_input = load(FIGHTER_LEVEL18_FIXTURE);
    let level18_computation = compute_pilot_base_chassis(&level18_input);
    assert!(
        !level18_computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.armor_mastery"),
        "level-18 Fighter must not fabricate the level-19 Armor Mastery record: {:?}",
        level18_computation.explanations
    );
}

// ----- Baseline melee attack bonus rises by exactly the base-attack-bonus delta; AC unchanged -----

#[test]
fn fighter_level19_baseline_melee_attack_bonus_rises_armor_class_unchanged() {
    let input = load(FIGHTER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Baseline melee attack bonus rises by the base-attack-bonus delta (+1)
    // only, since Weapon Training's first-group bonus stays unchanged at
    // level 19: from 26 at level 18 to 27 at level 19.
    assert_eq!(computation.baseline_melee_attack_bonus, 28);

    // Baseline armor class is unchanged: Armor Training stays rank 4 (no
    // fifth rank exists) and Armor Mastery is a damage-reduction record, not
    // an armor-class contributor, so neither changes any derived Climb/Swim
    // total or armor-class value on this fixture.
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

// ----- Negative control: the level-18 fixture is unaffected by this widening -----

#[test]
fn fighter_level18_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 18, "Fighter level 18 base attack bonus must stay 18");

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(bravery.value, 5, "Fighter level 18 Bravery must stay +5");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.armor_mastery"),
        "level-18 Fighter must not gain the level-19 Armor Mastery record: {:?}",
        computation.explanations
    );

    assert_eq!(computation.baseline_melee_attack_bonus, 27);
}

// ----- Negative control: multiclass Fighter is not promoted -----

#[test]
fn multiclass_fighter_level19_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL19_FIXTURE.replace(
        "class_level=class:fighter:19",
        "class_level=class:fighter:19\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Fighter+Rogue multiclass mix into a
    // genuinely supported combination (via the table-driven
    // compute_generic_table_chassis path for Rogue), so this negative control is
    // superseded, not violated: Fighter level 19 / Rogue level 1 now gets a
    // real, integrated class_chassis.base_attack_bonus (Fighter 19's full BAB
    // 19 + Rogue 1's 3/4 BAB floor(1*3/4)=0, summed = 19), and with this
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
        computation.base_attack_bonus, 19,
        "Fighter 19 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 19"
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Fighter 19 / Rogue 1 with this fixture's full deterministic posture has no \
         remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix note names the level-19 widening -----

#[test]
fn matrix_fighter_row_names_level_19_widening() {
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
        fighter.grounding_ref.contains("sd18_fighter_level19_widening"),
        "fighter row must cite the live SD18 level-19 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 19") || note.contains("level-19"),
        "fighter partial note must name the level-19 widening: {note}"
    );
}
