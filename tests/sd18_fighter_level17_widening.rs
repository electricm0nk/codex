//! SD18 Fighter level-17 widening grounding proof.
//!
//! Widens the accepted Human Fighter level-1..level-16 chassis
//! (`tests/sd18_fighter_level16_widening.rs`, the loop's most recent Fighter
//! ceiling) to Fighter level 17 -- the fourth class to reach level 17 in the
//! §3.2 level-17 sweep, after Ranger, Bard, and Rogue -- mirroring the
//! sibling per-level-gate idiom (`supported_fighter_level` is generalized
//! from `1..=16` to `1..=17` via `MAX_SUPPORTED_FIGHTER_LEVEL = 17`). Both
//! PF1 CRB primary sources (`www.aonprd.com/ClassDisplay.aspx?ItemName=Fighter`
//! and `legacy.aonprd.com/coreRuleBook/classes/fighter.html`) were read
//! directly before writing any code or test, and agreed byte-for-byte on
//! levels 14-18:
//!
//! - level 17 base attack bonus is +17/+12/+7/+2 (full BAB, genuinely risen
//!   from +16) while both poor Reflex and poor Will STAY +5 (`17 / 3 = 5`,
//!   an integer-division coincidence with level 16); good Fortitude also
//!   STAYS +10 (`17 / 2 + 2 = 10`, an integer-division coincidence with
//!   level 16) -- checked directly, not assumed.
//! - the PF1 Core Rulebook Fighter class table's level-17 "Special" column
//!   reads "Weapon training 4" (both primary sources agree, byte-for-byte,
//!   so a third source was not required; neighboring levels 16 -- "Bonus
//!   feat" -- and 18 -- "Bonus feat, bravery +5" -- were also fetched and
//!   rule out misattribution). Weapon Training's rank formula
//!   (`1 + (level - 5) / 4`) is already generic over `level` and genuinely
//!   rises to rank 4 at level 17 with no formula change; the first group's
//!   (Heavy Blades) attack-roll bonus, already folded into the baseline
//!   melee attack bonus, rises from +3 to +4 automatically. Landing rank 4
//!   correctly requires a FOURTH weapon-group choice slot, mirroring the
//!   existing `FIGHTER_WEAPON_TRAINING_GROUP_3_CHOICE_ID` idiom exactly,
//!   since PF1 grants a newly chosen group at each weapon-training rank.
//!   This slice adds `FIGHTER_WEAPON_TRAINING_GROUP_4_CHOICE_ID` (canonically
//!   Hammers), surfaced as an explanation-only record: no hammer is part of
//!   the deterministic Longsword loadout, so the +1 (`rank - 3 = 1`) is
//!   never folded into any computed total, exactly like the second (Bows)
//!   and third (Polearms) groups.
//! - Fighter gains no new bonus feat at level 17 (the cadence is 1, 2, 4, 6,
//!   8, 10, 12, 14, 16, ...; 17 is not in it) -- checked directly, not
//!   assumed.
//! - Bravery stays +4 (`1 + (17 - 2) / 4 = 4`), an integer-division
//!   coincidence with level 16 (the next rise is level 18).
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, a general
//! feat-effect/prerequisite engine, or any bonus-feat application, and it
//! does not ground Fighter level 18+. It also preserves the accepted Fighter
//! level-1..level-16 truth (unchanged) and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level16_sd18_widening_deterministic_input.txt"
);

const FIGHTER_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level17_sd18_widening_deterministic_input.txt"
);

// ----- Base attack bonus genuinely rises; all base saves stay unchanged at level 17 -----

#[test]
fn fighter_level17_base_attack_bonus_rises_saves_stay() {
    let input = load(FIGHTER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-17 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 17,
        "Fighter level 17 full-BAB progression must genuinely rise to 17, up from 16 at level \
         16: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Fighter level 17 good Fortitude (17/2+2) must stay 10, an integer-division coincidence \
         with level 16"
    );

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Fighter level 17 poor Reflex (17/3) must stay 5, an integer-division coincidence with \
         level 16"
    );

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(
        will.value, 5,
        "Fighter level 17 poor Will (17/3) must stay 5, an integer-division coincidence with \
         level 16"
    );
}

// ----- Weapon Training rises to rank 4; the first group's bonus rises with it -----

#[test]
fn fighter_level17_weapon_training_rank_rises_to_four() {
    let input = load(FIGHTER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 4,
        "Weapon Training's first-group attack bonus (1 + (17-5)/4) must genuinely rise to +4 \
         at level 17, up from +3 at level 16: {}",
        weapon_training.detail
    );
}

// ----- The fourth weapon-training group is newly surfaced as an explanation-only record -----

#[test]
fn fighter_level17_grants_a_fourth_weapon_training_group_seam() {
    let input = load(FIGHTER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fourth_group = explanation(&computation, "class_feature.fighter.weapon_training_group_4");
    assert_eq!(
        fourth_group.value, 1,
        "the fourth weapon-training group (rank - 3 = 1) must be surfaced as an \
         explanation-only record: {fourth_group:?}"
    );
    assert!(
        fourth_group.detail.contains("group:hammers"),
        "fourth weapon-training group seam must name the canonical Hammers selection: {}",
        fourth_group.detail
    );

    // The second and third groups' seams still carry over unchanged.
    let second_group =
        explanation(&computation, "class_feature.fighter.weapon_training_group_2");
    assert_eq!(
        second_group.value, 3,
        "the second weapon-training group (rank - 1 = 3) must carry over, genuinely risen from \
         +2 at level 16 alongside the rank rise: {second_group:?}"
    );
    let third_group = explanation(&computation, "class_feature.fighter.weapon_training_group_3");
    assert_eq!(
        third_group.value, 2,
        "the third weapon-training group (rank - 2 = 2) must carry over, genuinely risen from \
         +1 at level 16 alongside the rank rise: {third_group:?}"
    );
}

// ----- No new bonus feat lands at level 17; Bravery stays unchanged -----

#[test]
fn fighter_level17_grants_no_new_bonus_feat_bravery_stays_unchanged() {
    let input = load(FIGHTER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_17_bonus_feat"),
        "level 17 is not a bonus-feat cadence level (1, 2, 4, 6, 8, 10, 12, 14, 16, ...), so no \
         new bonus-feat seam should appear: {:?}",
        computation.explanations
    );

    // The level-16 seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_16_bonus_feat"),
        "level-17 Fighter must still carry the level-16 bonus-feat seam: {:?}",
        computation.explanations
    );

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(bravery.value, 4, "Bravery (1 + (17-2)/4) must stay +4 at level 17");
}

// ----- Baseline melee attack bonus rises by exactly the combined delta -----

#[test]
fn fighter_level17_baseline_melee_attack_bonus_rises() {
    let input = load(FIGHTER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Baseline melee attack bonus rises by the base-attack-bonus delta (+1)
    // plus the first-group Weapon Training delta (+1), from 23 at level 16
    // to 25 at level 17.
    assert_eq!(computation.baseline_melee_attack_bonus, 26);

    // Baseline armor class is unchanged: Armor Training's rank does not rise
    // again at level 17 (it caps at rank 4, level 15).
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

// ----- Negative control: the level-16 fixture is unaffected by this widening -----

#[test]
fn fighter_level16_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 16, "Fighter level 16 base attack bonus must stay 16");

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(weapon_training.value, 3, "Fighter level 16 Weapon Training must stay +3");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.weapon_training_group_4"),
        "level-16 Fighter must not gain the level-17 fourth weapon-training-group seam"
    );

    assert_eq!(computation.baseline_melee_attack_bonus, 24);
}

// ----- Negative control: level 21 stays claim-blocked (PF1 has no 21st character level) -----
//
// SD18 (tests/sd18_fighter_level18_widening.rs, tests/sd18_fighter_level19_widening.rs,
// tests/sd18_fighter_level20_widening.rs) further widened the bounded tranche
// from level 17 to level 18 (a ninth bonus-feat cadence slot and a further
// Bravery magnitude rise), then to level 19 (the Armor Mastery
// flat-magnitude damage reduction record), and then to level 20 (a tenth
// bonus-feat cadence slot and the Weapon Mastery grant-only capstone
// record) -- the FINAL level within PF1's 1-20 character-level cap -- so
// this negative control now sits just above the current bound (level 21,
// which does not exist as a PF1 character level) rather than at level 18,
// level 19, or level 20.

#[test]
fn fighter_level_21_stays_claim_blocked() {
    let level_21 = FIGHTER_LEVEL17_FIXTURE.replace("class:fighter:17", "class:fighter:21");
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
fn multiclass_fighter_level17_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL17_FIXTURE.replace(
        "class_level=class:fighter:17",
        "class_level=class:fighter:17\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Fighter+Rogue multiclass mix into a
    // genuinely supported combination (via the table-driven
    // compute_generic_table_chassis path for Rogue), so this negative control is
    // superseded, not violated: Fighter level 17 / Rogue level 1 now gets a
    // real, integrated class_chassis.base_attack_bonus (Fighter 17's full BAB
    // 17 + Rogue 1's 3/4 BAB floor(1*3/4)=0, summed = 17), and with this
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
        computation.base_attack_bonus, 17,
        "Fighter 17 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 17"
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Fighter 17 / Rogue 1 with this fixture's full deterministic posture has no \
         remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_fighter_row_names_level_17_widening() {
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
        fighter.grounding_ref.contains("sd18_fighter_level17_widening"),
        "fighter row must cite the live SD18 level-17 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 17") || note.contains("level-17"),
        "fighter partial note must name the level-17 widening: {note}"
    );
}
