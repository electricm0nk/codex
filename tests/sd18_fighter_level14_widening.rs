//! SD18 Fighter level-14 widening grounding proof.
//!
//! Widens the accepted Human Fighter level-1..level-13 chassis
//! (`tests/sd18_fighter_level13_widening.rs`, the loop's most recent Fighter
//! ceiling) to Fighter level 14 -- entering the level-14 sweep opened by
//! Barbarian (`tests/sd18_barbarian_level14_widening.rs`, cycle-2026-07-15T1900)
//! -- mirroring the sibling per-level-gate idiom (`supported_fighter_level` is
//! generalized from `1..=13` to `1..=14` via `MAX_SUPPORTED_FIGHTER_LEVEL = 14`).
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys aonprd.com
//! mirror) were read directly before writing any code or test, and agreed
//! byte-for-byte:
//!
//! - level 14 base attack bonus is +14/+9/+4 (full BAB, genuinely risen from
//!   +13) while good Fortitude genuinely rises to +9 (`14 / 2 + 2 = 9`, up
//!   from 8); poor Reflex and poor Will both STAY +4 (`14 / 3 = 4`), an
//!   integer-division coincidence with level 13 -- checked directly, not
//!   assumed.
//! - the PF1 Core Rulebook Fighter class table's level-14 "Special" column
//!   reads "Bonus feat, bravery +4" (both primary sources agree). Bravery's
//!   already-generic formula (`1 + (level - 2) / 4`) genuinely rises to +4 at
//!   level 14 (up from +3 at level 13) with no code change -- a pure
//!   magnitude rise on the already-grounded flat Will-vs-fear bonus record.
//!   Fighter's bonus-feat cadence (1, 2, 4, 6, 8, 10, 12, 14, ...) includes
//!   14, so this slice adds a SEVENTH named bonus-feat progression seam --
//!   mirroring the existing `FIGHTER_LEVEL_12_BONUS_FEAT_CHOICE_ID` idiom
//!   exactly -- surfaced as an explanation-only record contributing no
//!   computed mechanical value. The canonical Greater Weapon Specialization
//!   selection's prerequisites (fighter level 12, Weapon Focus and Weapon
//!   Specialization with the chosen weapon) are honestly met by the
//!   canonical loadout: Weapon Focus (longsword) is the level-1 fighter
//!   bonus feat and Weapon Specialization (longsword) is the level-12
//!   fighter bonus feat.
//! - Weapon Training's rank formula (`1 + (level - 5) / 4`) STAYS at rank 3
//!   at level 14 (`1 + (14 - 5) / 4 = 1 + 2 = 3`), an integer-division
//!   coincidence with level 13 (the next rise is level 17) -- checked
//!   directly, not assumed. Armor Training also stays at rank 3 (the next
//!   rise is level 15).
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, a general
//! feat-effect/prerequisite engine, or Greater Weapon Specialization's own
//! damage-roll application, and it does not ground Fighter level 15+. It
//! also preserves the accepted Fighter level-1..level-13 truth (unchanged)
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level13_sd18_widening_deterministic_input.txt"
);

const FIGHTER_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level14_sd18_widening_deterministic_input.txt"
);

// ----- Base attack bonus and good Fortitude genuinely rise; poor saves stay at level 14 -----

#[test]
fn fighter_level14_base_attack_bonus_and_fortitude_rise_poor_saves_stay() {
    let input = load(FIGHTER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-14 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 14,
        "Fighter level 14 full-BAB progression must genuinely rise to 14, up from 13 at level \
         13: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Fighter level 14 good Fortitude (14/2+2) must genuinely rise to 9, up from 8 at level \
         13"
    );

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Fighter level 14 poor Reflex (14/3) must stay 4, unchanged from level 13"
    );

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(
        will.value, 4,
        "Fighter level 14 poor Will (14/3) must stay 4, unchanged from level 13"
    );
}

// ----- Weapon Training stays at rank 3; no new weapon-training group lands at level 14 -----

#[test]
fn fighter_level14_weapon_training_rank_stays_unchanged() {
    let input = load(FIGHTER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 3,
        "Weapon Training's first-group attack bonus (1 + (14-5)/4) must stay +3 at level 14, an \
         integer-division coincidence with level 13 (the next rise is level 17): {}",
        weapon_training.detail
    );

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.weapon_training_group_4"),
        "level 14 is not a weapon-training rank-rise level, so no fourth weapon-training-group \
         seam should appear: {:?}",
        computation.explanations
    );
}

// ----- A seventh bonus feat lands at level 14; Bravery genuinely rises to +4 -----

#[test]
fn fighter_level14_grants_seventh_bonus_feat_bravery_rises() {
    let input = load(FIGHTER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus_feat = explanation(&computation, "class_feature.fighter.level_14_bonus_feat");
    assert_eq!(
        bonus_feat.value, 0,
        "the level-14 bonus-feat slot is a progression seam only, contributing no computed \
         mechanical value: {bonus_feat:?}"
    );

    // The level-12 seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_12_bonus_feat"),
        "level-14 Fighter must still carry the level-12 bonus-feat seam: {:?}",
        computation.explanations
    );

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(
        bravery.value, 4,
        "Bravery (1 + (14-2)/4) must genuinely rise to +4 at level 14, up from +3 at level 13"
    );
}

// ----- Baseline melee attack bonus rises by exactly the base-attack-bonus delta -----

#[test]
fn fighter_level14_baseline_melee_attack_bonus_rises() {
    let input = load(FIGHTER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Baseline melee attack bonus rises by the base-attack-bonus delta (+1)
    // only, since Weapon Training's first-group bonus stays unchanged at
    // level 14: from 20 at level 13 to 21 at level 14.
    assert_eq!(computation.baseline_melee_attack_bonus, 22);

    // Baseline armor class is unchanged: Armor Training's rank (and thus
    // maximum Dexterity bonus) does not rise again at level 14.
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

// ----- Negative control: the level-13 fixture is unaffected by this widening -----

#[test]
fn fighter_level13_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 13, "Fighter level 13 base attack bonus must stay 13");

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(bravery.value, 3, "Fighter level 13 Bravery must stay +3");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_14_bonus_feat"),
        "level-13 Fighter must not gain the level-14 bonus-feat seam"
    );

    assert_eq!(computation.baseline_melee_attack_bonus, 21);
}

// ----- Negative control: level 21 stays claim-blocked (PF1 has no 21st character level) -----
//
// SD18 (tests/sd18_fighter_level15_widening.rs, tests/sd18_fighter_level16_widening.rs,
// tests/sd18_fighter_level17_widening.rs, tests/sd18_fighter_level18_widening.rs,
// tests/sd18_fighter_level19_widening.rs) further widened the bounded
// tranche from level 14 to level 15 (Armor Training 4), then to level 16
// (an eighth bonus-feat cadence slot), then to level 17 (Weapon Training 4,
// a fourth weapon-group choice slot), then to level 18 (a ninth bonus-feat
// cadence slot and a further Bravery magnitude rise), and then to level 19
// (the Armor Mastery flat-magnitude damage reduction record), and SD18
// (tests/sd18_fighter_level20_widening.rs) further widened the bounded
// tranche to level 20 (a tenth bonus-feat cadence slot and the Weapon
// Mastery grant-only capstone record) -- the FINAL level within PF1's
// 1-20 character-level cap -- so this negative control now sits just
// above the current bound (level 21, which does not exist as a PF1
// character level) rather than at level 15, level 16, level 17, level 18,
// level 19, or level 20.

#[test]
fn fighter_level_21_stays_claim_blocked() {
    let level_21 = FIGHTER_LEVEL14_FIXTURE.replace("class:fighter:14", "class:fighter:21");
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
fn multiclass_fighter_level14_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL14_FIXTURE.replace(
        "class_level=class:fighter:14",
        "class_level=class:fighter:14\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Fighter+Rogue multiclass mix into a
    // genuinely supported combination (via the table-driven
    // compute_generic_table_chassis path for Rogue), so this negative control is
    // superseded, not violated: Fighter level 14 / Rogue level 1 now gets a
    // real, integrated class_chassis.base_attack_bonus (Fighter 14's full BAB
    // 14 + Rogue 1's 3/4 BAB floor(1*3/4)=0, summed = 14), and with this
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
        computation.base_attack_bonus, 14,
        "Fighter 14 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 14"
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Fighter 14 / Rogue 1 with this fixture's full deterministic posture has no \
         remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_fighter_row_names_level_14_widening() {
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
        fighter.grounding_ref.contains("sd18_fighter_level14_widening"),
        "fighter row must cite the live SD18 level-14 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 14") || note.contains("level-14"),
        "fighter partial note must name the level-14 widening: {note}"
    );
}
