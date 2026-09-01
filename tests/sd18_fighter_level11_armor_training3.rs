//! SD18 Fighter level-11 widening grounding proof.
//!
//! Widens the accepted deterministic Human Fighter level-1..level-10 chassis
//! (`tests/sd13_fighter_level9_level10_progression.rs`, the SD13 tranche's
//! declared ceiling) to Fighter level 11 — the fifth SD-18 §3.2 class-row
//! widening, mirroring the sibling-class level-range-gate idiom
//! (`supported_fighter_level` is generalized from `1..=10` to `1..=11` via
//! `MAX_SUPPORTED_FIGHTER_LEVEL = 11`, exactly as `cycle-2026-07-13T1255`
//! widened `MAX_SUPPORTED_BARBARIAN_LEVEL`, `cycle-2026-07-13T1830` widened
//! `MAX_SUPPORTED_BARD_LEVEL`, `cycle-2026-07-13T2007` widened
//! `MAX_SUPPORTED_CLERIC_LEVEL`, and `cycle-2026-07-13T1851` widened
//! `MAX_SUPPORTED_DRUID_LEVEL`, all from 10 to 11). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Fighter class table) were read
//! directly before writing any code or test:
//!
//! - level 11 base attack bonus is +11 (full BAB = classlevel, genuinely
//!   risen from +10 at level 10) — confirmed by the same level-generic
//!   formula already grounded at levels 1-10, not re-derived.
//! - base saves stay Fortitude +7 (`11 / 2 + 2 = 7`), Reflex +3 (`11 / 3 =
//!   3`), and Will +3 (`11 / 3 = 3`) — all three are integer-division
//!   coincidences with level 10, checked not assumed.
//! - the PF1 Core Rulebook Fighter class table's level-11 "Special" column
//!   reads "Armor training 3" (verified independently against both primary
//!   sources, checked rather than assumed away): Armor Training's rank
//!   GENUINELY RISES to 3 (from 2 at level 10), which raises the Chain
//!   Shirt's maximum Dexterity bonus to +7 (from +6 at Armor Training 2) —
//!   a genuine magnitude rise on the already-grounded Armor Training pillar,
//!   mirroring exactly how Armor Training 1 -> 2 was grounded at level 7.
//!   The armor-check-penalty reduction also rises to 3, but the effective
//!   penalty on the deterministic Chain Shirt was already capped at 0 by
//!   Armor Training 2 (`(-2 + 2).min(0) = 0`, and `(-2 + 3).min(0)` is still
//!   `0`), so no Climb/Swim selected-skill total changes on this fixture;
//!   and the raised max-Dexterity-bonus cap does not change baseline armor
//!   class either, since the deterministic +2 Dexterity contribution was
//!   already well below both the old and new caps.
//! - Bravery's magnitude stays +3 (`1 + (11 - 2) / 4 = 3`) and Weapon
//!   Training's first-group attack bonus stays +2 (`1 + (11 - 5) / 4 = 2`) —
//!   both integer-division coincidences with level 10.
//! - no new bonus feat lands at level 11 (the cadence is 1, 2, 4, 6, 8, 10,
//!   12, ...; the next feat is level 12), verified independently rather than
//!   assumed.
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, or a general
//! feat-effect/prerequisite engine (all stay named-but-unproven, unchanged
//! from levels 1-10), and it does not ground Fighter level 12+. It also
//! preserves the accepted Fighter level-1..level-10 truth (unchanged), and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level10_sd13_deterministic_input.txt");

const FIGHTER_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level11_sd18_armor_training3_deterministic_input.txt"
);

// ----- Base attack bonus genuinely rises; base saves carry over unchanged -----

#[test]
fn fighter_level11_base_attack_bonus_genuinely_rises() {
    let input = load(FIGHTER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-11 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 11,
        "Fighter level 11 full-BAB progression must genuinely rise to 11, up from 10 at level \
         10: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(fortitude.value, 7, "Fighter level 11 good Fortitude (11/2+2) must stay 7");

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(reflex.value, 3, "Fighter level 11 poor Reflex (11/3) must stay 3");

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(will.value, 3, "Fighter level 11 poor Will (11/3) must stay 3");
}

// ----- Armor Training 3 genuinely raises the max-Dexterity-bonus pillar -----

#[test]
fn fighter_level11_armor_training_3_genuinely_raises_max_dex() {
    let input = load(FIGHTER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(
        seam.value, 3,
        "Armor Training 3 must carry rank 3 (genuinely risen from rank 2 at level 10): {seam:?}"
    );
    assert!(
        seam.detail.contains("Armor Training 3"),
        "armor-training seam must name Armor Training 3: {}",
        seam.detail
    );
    assert!(
        seam.detail.contains('7'),
        "armor-training seam must name the raised maximum Dexterity bonus of 7 (from 6 at \
         Armor Training 2): {}",
        seam.detail
    );
}

#[test]
fn fighter_level11_armor_check_penalty_and_derived_ac_stay_unchanged() {
    let input = load(FIGHTER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The armor-check-penalty reduction is already capped at 0 by Armor
    // Training 2, so Armor Training 3's further reduction changes no
    // derived value: Climb/Swim stay exactly as they were at level 10.
    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);

    // Baseline armor class is unchanged: the deterministic +2 Dexterity
    // contribution is already well below both the old (+6) and new (+7)
    // maximum-Dexterity-bonus cap.
    assert_eq!(computation.baseline_armor_class, 17);

    // Baseline melee attack bonus rises by exactly the base-attack-bonus
    // delta (+1), since Weapon Training's own rank does not change at 11.
    assert_eq!(computation.baseline_melee_attack_bonus, 18);
}

// ----- Bravery and Weapon Training carry over unchanged -----

#[test]
fn fighter_level11_bravery_and_weapon_training_carry_over_unchanged() {
    let input = load(FIGHTER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(bravery.value, 3, "Bravery (1 + (11-2)/4) must stay +3 at level 11");

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 2,
        "Weapon Training's first-group attack bonus (1 + (11-5)/4) must stay +2 at level 11"
    );
}

// ----- No new bonus feat lands at level 11 -----

#[test]
fn fighter_level11_grants_no_new_bonus_feat() {
    let input = load(FIGHTER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("bonus_feat_11") || e.id.contains("bonus_feat_12")),
        "no bonus-feat seam should exist at level 11 (the cadence's next feat is level 12): {:?}",
        computation.explanations
    );
    // The level-10 seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_10_bonus_feat"),
        "level-11 Fighter must still carry the level-10 bonus-feat seam: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn fighter_level10_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 10, "Fighter level 10 base attack bonus must stay 10");

    let seam = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(seam.value, 2, "Fighter level 10 Armor Training rank must stay 2");

    assert_eq!(computation.baseline_armor_class, 17);
    assert_eq!(computation.baseline_melee_attack_bonus, 17);
}

// ----- Negative control: level 21 stays claim-blocked (PF1 has no 21st character level) -----
//
// SD18 (tests/sd18_fighter_level12_widening.rs, tests/sd18_fighter_level13_widening.rs,
// tests/sd18_fighter_level14_widening.rs, tests/sd18_fighter_level15_widening.rs,
// tests/sd18_fighter_level16_widening.rs, tests/sd18_fighter_level17_widening.rs,
// tests/sd18_fighter_level18_widening.rs, tests/sd18_fighter_level19_widening.rs)
// further widened the bounded tranche from level 11 to level 12 (a sixth
// bonus-feat cadence slot), then to level 13 (Weapon Training 3), then to
// level 14 (a seventh bonus-feat cadence slot and the Bravery magnitude
// rise), then to level 15 (Armor Training 4), then to level 16 (an eighth
// bonus-feat cadence slot), then to level 17 (Weapon Training 4), then to
// level 18 (a ninth bonus-feat cadence slot and a further Bravery magnitude
// rise), and then to level 19 (the Armor Mastery flat-magnitude damage
// reduction record), and SD18 (tests/sd18_fighter_level20_widening.rs)
// further widened the bounded tranche to level 20 (a tenth bonus-feat
// cadence slot and the Weapon Mastery grant-only capstone record) -- the
// FINAL level within PF1's 1-20 character-level cap -- so this negative
// control now sits just above the current bound (level 21, which does not
// exist as a PF1 character level) rather than at level 12, level 13,
// level 14, level 15, level 16, level 17, level 18, level 19, or level 20.

#[test]
fn fighter_level_21_stays_claim_blocked() {
    let level_21 = FIGHTER_LEVEL11_FIXTURE.replace("class:fighter:11", "class:fighter:21");
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
fn multiclass_fighter_level11_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL11_FIXTURE.replace(
        "class_level=class:fighter:11",
        "class_level=class:fighter:11\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Fighter+Rogue multiclass mix into a
    // genuinely supported combination (via the table-driven
    // compute_generic_table_chassis path for Rogue), so this negative control is
    // superseded, not violated: Fighter level 11 / Rogue level 1 now gets a
    // real, integrated class_chassis.base_attack_bonus (Fighter 11's full BAB
    // 11 + Rogue 1's 3/4 BAB floor(1*3/4)=0, summed = 11), and with this
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
        computation.base_attack_bonus, 11,
        "Fighter 11 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 11"
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Fighter 11 / Rogue 1 with this fixture's full deterministic posture has no \
         remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_fighter_row_names_level_11_widening() {
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
        fighter.grounding_ref.contains("sd18_fighter_level11_armor_training3"),
        "fighter row must cite the live SD18 level-11 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "fighter partial note must name the level-11 widening: {note}"
    );
}
