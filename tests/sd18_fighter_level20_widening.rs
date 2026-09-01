//! SD18 Fighter level-20 widening grounding proof.
//!
//! Widens the accepted Human Fighter level-1..level-19 chassis
//! (`tests/sd18_fighter_level19_widening.rs`, the loop's most recent Fighter
//! ceiling) to Fighter level 20 -- the FINAL level within PF1's 1-20
//! character-level cap, and the loop's FIFTH §3.2 level-20 landing after
//! Cleric, Wizard, Barbarian, and Bard -- mirroring the sibling per-level-gate
//! idiom (`supported_fighter_level` is generalized from `1..=19` to `1..=20`
//! via `MAX_SUPPORTED_FIGHTER_LEVEL = 20`). Both PF1 CRB primary sources
//! (`d20pfsrd.com/classes/core-classes/fighter/` and
//! `aonprd.com/ClassDisplay.aspx?ItemName=Fighter`) were read directly (raw
//! curl + tag-strip, not summarized) before writing any code or test,
//! fetching the full class table in one pass so the level-20 row's neighbors
//! were visible in context (guards against level-misattribution), and agreed
//! byte-for-byte:
//!
//! - level 18: "+18/+13/+8/+3 | +11 | +6 | +6 | Bonus feat, bravery +5"
//! - level 19: "+19/+14/+9/+4 | +11 | +6 | +6 | Armor mastery"
//! - level 20: "+20/+15/+10/+5 | +12 | +6 | +6 | Bonus feat, Weapon mastery"
//!
//! This confirms: base attack bonus GENUINELY RISES to 20 (full BAB, up from
//! 19) and good Fortitude GENUINELY RISES to 12 (`20/2+2=12`, up from 11),
//! while poor Reflex/Will both STAY at 6 (`20/3=6`, an integer-division
//! coincidence with level 19) -- checked directly, not assumed. Level 20 IS a
//! Fighter bonus-feat cadence level (1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20),
//! so this widening surfaces a TENTH named bonus-feat progression seam (the
//! canonical Critical Mastery selection's prerequisites -- two other critical
//! feats -- are honestly met by the canonical loadout: Improved Critical,
//! Critical Focus, and Staggering Critical are all already-selected fighter
//! bonus feats). Level 20 is NEITHER a Weapon Training rank-rise level (ranks
//! rise at 5, 9, 13, 17; the PF1 Core Rulebook names no fifth rank) NOR an
//! Armor Training rank-rise level (ranks rise at 3, 7, 11, 15; the PF1 Core
//! Rulebook names no fifth rank), so neither pillar widens, and Bravery's
//! magnitude stays +5 (an integer-division coincidence with level 19, since
//! the next rise, level 22, is beyond the PF1 level cap).
//!
//! Weapon Mastery (Ex) itself IS a genuinely new named class feature (per
//! both primary sources, byte-for-byte agreement): "At 20th level, a fighter
//! chooses one weapon, such as the longsword, greataxe, or longbow. Any
//! attacks made with that weapon automatically confirm all critical threats
//! and have their damage multiplier increased by 1 (x2 becomes x3, for
//! example). In addition, he cannot be disarmed while wielding a weapon of
//! this type." This is grounded as a bounded, non-fabricated grant-only
//! identity/magnitude record only (`class_feature.fighter.weapon_mastery`,
//! critical-multiplier-increase magnitude 1 at or above level 20), mirroring
//! EXACTLY the already-proven Fighter Armor Mastery idiom
//! (`class_feature.fighter.armor_mastery`): no critical-hit-confirmation
//! engine, no damage-multiplier-application engine, and no
//! disarm-resolution engine exists anywhere in this codebase, so this
//! grounds no actual automatic-critical-confirmation, no actual
//! damage-multiplier change, and no actual disarm immunity. No new subsystem
//! is invented.
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, a general
//! feat-effect/prerequisite engine, the Armor Mastery damage-reduction
//! application, or any actual critical-hit/disarm resolution. This closes
//! Fighter's per-level arithmetic-widening frontier: level 20 is the final
//! level within PF1's 1-20 cap. It also preserves the accepted Fighter
//! level-1..level-19 truth (unchanged) and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level19_sd18_widening_deterministic_input.txt"
);

const FIGHTER_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level20_sd18_widening_deterministic_input.txt"
);

// ----- Base attack bonus and good Fortitude genuinely rise at level 20; poor saves stay unchanged -----

#[test]
fn fighter_level20_base_attack_and_fortitude_rise_poor_saves_stay_unchanged() {
    let input = load(FIGHTER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-20 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 20,
        "Fighter level 20 full-BAB progression must genuinely rise to 20, up from 19 at level \
         19: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(
        fortitude.value, 12,
        "Fighter level 20 good Fortitude (20/2+2) must genuinely rise to 12, up from 11 at \
         level 19"
    );

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Fighter level 20 poor Reflex (20/3) must stay 6, unchanged from level 19, an \
         integer-division coincidence"
    );

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(
        will.value, 6,
        "Fighter level 20 poor Will (20/3) must stay 6, unchanged from level 19, an \
         integer-division coincidence"
    );
}

// ----- New TENTH bonus-feat seam at level 20; Bravery stays unchanged -----

#[test]
fn fighter_level20_new_bonus_feat_seam_bravery_unchanged() {
    let input = load(FIGHTER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let level_20_feat = explanation(&computation, "class_feature.fighter.level_20_bonus_feat");
    assert_eq!(
        level_20_feat.value, 0,
        "the level-20 bonus-feat seam grounds the choice slot only, not a general feat-effect \
         engine, so it contributes no computed mechanical value (+0)"
    );

    // The level-18 bonus-feat seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_18_bonus_feat"),
        "level-20 Fighter must still carry the level-18 bonus-feat seam: {:?}",
        computation.explanations
    );

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(
        bravery.value, 5,
        "Bravery (1 + (20-2)/4 = 1 + 4 = 5) must stay +5 at level 20, unchanged from level 19, \
         an integer-division coincidence (the next rise is level 22, beyond the PF1 level cap)"
    );
}

// ----- Weapon Training and Armor Training both stay unchanged at level 20 -----

#[test]
fn fighter_level20_weapon_training_and_armor_training_stay_unchanged() {
    let input = load(FIGHTER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 4,
        "Weapon Training's first-group attack bonus (1 + (20-5)/4) must stay +4 at level 20, \
         unchanged from level 19: {}",
        weapon_training.detail
    );

    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(
        armor_training.value, 4,
        "Fighter Armor Training rank must stay 4 at level 20: the PF1 Core Rulebook names no \
         fifth Armor Training rank: {}",
        armor_training.detail
    );

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.weapon_training_group_5"),
        "level 20 must not fabricate a fifth weapon-training-group seam: {:?}",
        computation.explanations
    );
}

// ----- Armor Mastery carries over unchanged at level 20 -----

#[test]
fn fighter_level20_armor_mastery_carries_over_unchanged() {
    let input = load(FIGHTER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let armor_mastery = explanation(&computation, "class_feature.fighter.armor_mastery");
    assert_eq!(
        armor_mastery.value, 5,
        "Armor Mastery's DR 5/-- record must carry over unchanged at level 20: {}",
        armor_mastery.detail
    );
}

// ----- Weapon Mastery: a genuinely new level-20 capstone class feature, grant-only identity/magnitude record -----

#[test]
fn fighter_level20_weapon_mastery_grant_only_identity_record() {
    let input = load(FIGHTER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let weapon_mastery = explanation(&computation, "class_feature.fighter.weapon_mastery");
    assert_eq!(
        weapon_mastery.value, 1,
        "Weapon Mastery's critical-multiplier-increase magnitude is +1 at level 20 (verified \
         against d20pfsrd and aonprd), grounded as a flat, non-fabricated magnitude record \
         only: {}",
        weapon_mastery.detail
    );
    assert!(
        weapon_mastery.detail.contains("critical")
            && weapon_mastery.detail.contains("disarm"),
        "Weapon Mastery's record must honestly disclaim the absent critical-confirmation and \
         disarm-resolution engines, mirroring the Armor Mastery idiom: {}",
        weapon_mastery.detail
    );

    // The level-19 fixture must not carry this record at all (correct PF1
    // Core Rulebook level-gate absence).
    let level19_input = load(FIGHTER_LEVEL19_FIXTURE);
    let level19_computation = compute_pilot_base_chassis(&level19_input);
    assert!(
        !level19_computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.weapon_mastery"),
        "level-19 Fighter must not fabricate the level-20 Weapon Mastery record: {:?}",
        level19_computation.explanations
    );
}

// ----- Baseline melee attack bonus rises by exactly the base-attack-bonus delta; AC unchanged -----

#[test]
fn fighter_level20_baseline_melee_attack_bonus_rises_armor_class_unchanged() {
    let input = load(FIGHTER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Baseline melee attack bonus rises by the base-attack-bonus delta (+1)
    // only, since Weapon Training's first-group bonus stays unchanged at
    // level 20: from 27 at level 19 to 28 at level 20.
    assert_eq!(computation.baseline_melee_attack_bonus, 29);

    // Baseline armor class is unchanged: Armor Training stays rank 4 (no
    // fifth rank exists), Armor Mastery is a damage-reduction record (not an
    // armor-class contributor), and Weapon Mastery is a critical-multiplier/
    // disarm-immunity record (also not an armor-class contributor), so none
    // of these change any derived Climb/Swim total or armor-class value on
    // this fixture.
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

// ----- Negative control: the level-19 fixture is unaffected by this widening -----

#[test]
fn fighter_level19_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 19, "Fighter level 19 base attack bonus must stay 19");

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(bravery.value, 5, "Fighter level 19 Bravery must stay +5");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.weapon_mastery"),
        "level-19 Fighter must not gain the level-20 Weapon Mastery record: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_20_bonus_feat"),
        "level-19 Fighter must not gain the level-20 bonus-feat seam: {:?}",
        computation.explanations
    );

    assert_eq!(computation.baseline_melee_attack_bonus, 28);
}

// ----- Negative control: level 21 stays claim-blocked (PF1 has no 21st character level) -----

#[test]
fn fighter_level_21_stays_claim_blocked() {
    let level_21 = FIGHTER_LEVEL20_FIXTURE.replace("class:fighter:20", "class:fighter:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-21 Fighter must stay claim-blocked beyond the bounded levels-2-20 row (PF1 has \
         no 21st character level): {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.base_attack_bonus"),
        "level-21 Fighter must not fabricate a base-attack-bonus explanation"
    );
}

// ----- Negative control: multiclass Fighter is not promoted -----

#[test]
fn multiclass_fighter_level20_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL20_FIXTURE.replace(
        "class_level=class:fighter:20",
        "class_level=class:fighter:20\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Fighter+Rogue multiclass mix into a
    // genuinely supported combination (via the table-driven
    // compute_generic_table_chassis path for Rogue), so this negative control is
    // superseded, not violated: Fighter level 20 / Rogue level 1 now gets a
    // real, integrated class_chassis.base_attack_bonus (Fighter 20's full BAB
    // 20 + Rogue 1's 3/4 BAB floor(1*3/4)=0, summed = 20), and with this
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
        computation.base_attack_bonus, 20,
        "Fighter 20 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 20"
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Fighter 20 / Rogue 1 with this fixture's full deterministic posture has no \
         remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_fighter_row_names_level_20_widening() {
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
        fighter.grounding_ref.contains("sd18_fighter_level20_widening"),
        "fighter row must cite the live SD18 level-20 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "fighter partial note must name the level-20 widening: {note}"
    );
}
