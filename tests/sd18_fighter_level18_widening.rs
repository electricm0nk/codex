//! SD18 Fighter level-18 widening grounding proof.
//!
//! Widens the accepted Human Fighter level-1..level-17 chassis
//! (`tests/sd18_fighter_level17_widening.rs`, the loop's most recent Fighter
//! ceiling) to Fighter level 18 -- the fourth class to reach level 18 in the
//! §3.2 level-18 sweep, after Wizard, Cleric, and Paladin -- mirroring the
//! sibling per-level-gate idiom (`supported_fighter_level` is generalized
//! from `1..=17` to `1..=18` via `MAX_SUPPORTED_FIGHTER_LEVEL = 18`). Both
//! PF1 CRB primary sources (`d20pfsrd.com/classes/core-classes/fighter/` and
//! `aonprd.com/ClassDisplay.aspx?ItemName=Fighter`) were read directly before
//! writing any code or test, fetching the full levels-16-through-19 block in
//! one pass so the level-18 row's neighbors were visible in context (guards
//! against level-misattribution), and agreed byte-for-byte:
//!
//! - level 16: "+16/+11/+6/+1 | +10 | +5 | +5 | Bonus feat"
//! - level 17: "+17/+12/+7/+2 | +10 | +5 | +5 | Weapon training [4]"
//!   (d20pfsrd's own abbreviated wording, "Weapon training", vs aonprd's
//!   "Weapon training 4", is immaterial prose formatting, not a numeric or
//!   level-18 conflict, so no third source was required)
//! - level 18: "+18/+13/+8/+3 | +11 | +6 | +6 | Bonus feat, bravery +5"
//!   (both sources byte-for-byte identical)
//! - level 19: "+19/+14/+9/+4 | +11 | +6 | +6 | Armor mastery"
//!
//! This confirms: base attack bonus GENUINELY RISES to 18 (full BAB, up from
//! 17); good Fortitude GENUINELY RISES to 11 (`18 / 2 + 2 = 11`, up from 10
//! at level 17); poor Reflex and poor Will both GENUINELY RISE to 6
//! (`18 / 3 = 6`, up from 5 at level 17) -- checked directly, not assumed.
//! The Special column reads "Bonus feat, bravery +5": Bravery's
//! already-generic formula (`1 + (level - 2) / 4`) genuinely rises to +5
//! with no formula change (`1 + (18 - 2) / 4 = 5`), and this widening
//! surfaces a NINTH named bonus-feat progression seam
//! (`choice:fighter_bonus_feat_18`), mirroring the level-2 through level-16
//! seams exactly -- no general feat-effect or prerequisite engine invented.
//! The canonical Staggering Critical selection's prerequisites (Critical
//! Focus and base attack bonus +13) are honestly met by the canonical
//! loadout: Critical Focus is the level-16 fighter bonus feat and the
//! level-18 base attack bonus is +18. Weapon Training's rank formula
//! (`1 + (level - 5) / 4`) STAYS at rank 4 (`1 + (18 - 5) / 4 = 1 + 3 = 4`),
//! an integer-division coincidence with level 17 (PF1 names no fifth Weapon
//! Training rank in this range); Armor Training stays at rank 4 (the PF1
//! Core Rulebook names no fifth Armor Training rank).
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, a general
//! feat-effect/prerequisite engine, or any Staggering Critical
//! critical-confirm application, and it does not ground Fighter level 19+.
//! It also preserves the accepted Fighter level-1..level-17 truth (unchanged)
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level17_sd18_widening_deterministic_input.txt"
);

const FIGHTER_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level18_sd18_widening_deterministic_input.txt"
);

// ----- Base attack bonus and all three base saves genuinely rise at level 18 -----

#[test]
fn fighter_level18_base_attack_and_all_saves_rise() {
    let input = load(FIGHTER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-18 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 18,
        "Fighter level 18 full-BAB progression must genuinely rise to 18, up from 17 at level \
         17: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(
        fortitude.value, 11,
        "Fighter level 18 good Fortitude (18/2+2) must genuinely rise to 11, up from 10 at \
         level 17"
    );

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Fighter level 18 poor Reflex (18/3) must genuinely rise to 6, up from 5 at level 17"
    );

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(
        will.value, 6,
        "Fighter level 18 poor Will (18/3) must genuinely rise to 6, up from 5 at level 17"
    );
}

// ----- Ninth bonus-feat seam appears at level 18; Bravery genuinely rises to +5 -----

#[test]
fn fighter_level18_ninth_bonus_feat_seam_and_bravery_rise() {
    let input = load(FIGHTER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_18_bonus_feat"),
        "level 18 is a Fighter bonus-feat cadence level, so a ninth bonus-feat seam should \
         appear: {:?}",
        computation.explanations
    );

    // The level-16 bonus-feat seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_16_bonus_feat"),
        "level-18 Fighter must still carry the level-16 bonus-feat seam: {:?}",
        computation.explanations
    );

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(
        bravery.value, 5,
        "Bravery (1 + (18-2)/4) must genuinely rise to +5 at level 18, up from +4 at level 17"
    );
}

// ----- Weapon Training and Armor Training both stay unchanged at level 18 -----

#[test]
fn fighter_level18_weapon_training_and_armor_training_stay_unchanged() {
    let input = load(FIGHTER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 4,
        "Weapon Training's first-group attack bonus (1 + (18-5)/4) must stay +4 at level 18, an \
         integer-division coincidence with level 17: {}",
        weapon_training.detail
    );

    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(
        armor_training.value, 4,
        "Fighter Armor Training rank must stay 4 at level 18: the PF1 Core Rulebook names no \
         fifth Armor Training rank: {}",
        armor_training.detail
    );

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.weapon_training_group_5"),
        "level 18 must not fabricate a fifth weapon-training-group seam: {:?}",
        computation.explanations
    );
}

// ----- Baseline melee attack bonus rises by exactly the base-attack-bonus delta; AC unchanged -----

#[test]
fn fighter_level18_baseline_melee_attack_bonus_rises_armor_class_unchanged() {
    let input = load(FIGHTER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Baseline melee attack bonus rises by the base-attack-bonus delta (+1)
    // only, since Weapon Training's first-group bonus stays unchanged at
    // level 18: from 25 at level 17 to 26 at level 18.
    assert_eq!(computation.baseline_melee_attack_bonus, 27);

    // Baseline armor class is unchanged: Armor Training stays rank 4 (no
    // fifth rank exists), so the armor-check-penalty reduction and maximum
    // Dexterity bonus stay at their level-17 values, and neither changes any
    // derived Climb/Swim total or armor-class value on this fixture.
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

// ----- Negative control: the level-17 fixture is unaffected by this widening -----

#[test]
fn fighter_level17_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 17, "Fighter level 17 base attack bonus must stay 17");

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(bravery.value, 4, "Fighter level 17 Bravery must stay +4");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_18_bonus_feat"),
        "level-17 Fighter must not gain the level-18 bonus-feat seam: {:?}",
        computation.explanations
    );

    assert_eq!(computation.baseline_melee_attack_bonus, 26);
}

// ----- Negative control: level 21 stays claim-blocked (PF1 has no 21st character level) -----
//
// SD18 (tests/sd18_fighter_level19_widening.rs, tests/sd18_fighter_level20_widening.rs)
// further widened the bounded tranche from level 18 to level 19 (the Armor
// Mastery flat-magnitude damage reduction record), and then to level 20 (a
// tenth bonus-feat cadence slot and the Weapon Mastery grant-only capstone
// record) -- the FINAL level within PF1's 1-20 character-level cap -- so
// this negative control now sits just above the current bound (level 21,
// which does not exist as a PF1 character level) rather than at level 19
// or level 20.

#[test]
fn fighter_level_21_stays_claim_blocked() {
    let level_21 = FIGHTER_LEVEL18_FIXTURE.replace("class:fighter:18", "class:fighter:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-21 Fighter must stay claim-blocked beyond the bounded levels-2-20 row: {:?}",
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
fn multiclass_fighter_level18_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL18_FIXTURE.replace(
        "class_level=class:fighter:18",
        "class_level=class:fighter:18\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Fighter+Rogue multiclass mix into a
    // genuinely supported combination (via the table-driven
    // compute_generic_table_chassis path for Rogue), so this negative control is
    // superseded, not violated: Fighter level 18 / Rogue level 1 now gets a
    // real, integrated class_chassis.base_attack_bonus (Fighter 18's full BAB
    // 18 + Rogue 1's 3/4 BAB floor(1*3/4)=0, summed = 18), and with this
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
        computation.base_attack_bonus, 18,
        "Fighter 18 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 18"
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Fighter 18 / Rogue 1 with this fixture's full deterministic posture has no \
         remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix note names the level-18 widening -----

#[test]
fn matrix_fighter_row_names_level_18_widening() {
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
        fighter.grounding_ref.contains("sd18_fighter_level18_widening"),
        "fighter row must cite the live SD18 level-18 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 18") || note.contains("level-18"),
        "fighter partial note must name the level-18 widening: {note}"
    );
}
