//! SD18 Fighter level-12 widening grounding proof.
//!
//! Widens the accepted Human Fighter level-1..level-11 chassis
//! (`tests/sd18_fighter_level11_armor_training3.rs`, the loop's most recent
//! Fighter ceiling) to Fighter level 12 -- mirroring the sibling-class
//! level-range-gate idiom (`supported_fighter_level` is generalized from
//! `1..=11` to `1..=12` via `MAX_SUPPORTED_FIGHTER_LEVEL = 12`, exactly as
//! `cycle-2026-07-14T1814` widened `MAX_SUPPORTED_BARBARIAN_LEVEL`,
//! `cycle-2026-07-14T2359` widened `MAX_SUPPORTED_BARD_LEVEL`,
//! `cycle-2026-07-15T0200` widened `MAX_SUPPORTED_CLERIC_LEVEL`, and
//! `cycle-2026-07-15T0500` widened `MAX_SUPPORTED_DRUID_LEVEL`, all from 11
//! to 12). Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com Fighter class table) were read directly before writing any
//! code or test:
//!
//! - level 12 base attack bonus is +12 (full BAB = classlevel, genuinely
//!   risen from +11 at level 11) -- confirmed by the same level-generic
//!   formula already grounded at levels 1-11, not re-derived.
//! - base saves genuinely rise: Fortitude +8 (`12 / 2 + 2 = 8`, up from +7),
//!   Reflex +4 (`12 / 3 = 4`, up from +3), and Will +4 (`12 / 3 = 4`, up from
//!   +3) -- all three checked directly against both primary sources, not
//!   assumed.
//! - the PF1 Core Rulebook Fighter class table's level-12 "Special" column
//!   reads "Bonus feat" only (both primary sources agree; no named pillar
//!   beyond the bonus-feat cadence). This slice surfaces the level-12
//!   bonus-feat slot exactly as the level-2/4/6/8/10 slots were surfaced --
//!   a named selection seam contributing no computed mechanical value -- and
//!   grounds no general feat-effect or prerequisite engine. The canonical
//!   Weapon Specialization selection's prerequisites (fighter level 4 and
//!   Weapon Focus with the chosen weapon) are honestly met by the canonical
//!   loadout: Weapon Focus (longsword) is the level-1 fighter bonus feat.
//! - Armor Training stays at rank 3 (unchanged from level 11; Armor Training
//!   does not rise again within this bounded surface) and Weapon Training
//!   stays at rank 2 (unchanged from level 11; `1 + (12 - 5) / 4 = 2`,
//!   Weapon Training 3 does not arrive until level 13) -- both integer
//!   division coincidences with level 11, checked not assumed.
//! - Bravery's magnitude stays +3 (`1 + (12 - 2) / 4 = 3`) -- also an
//!   integer-division coincidence with level 11.
//!
//! It deliberately does not touch the Weapon Training damage-roll half, a
//! fear-condition/save-resolution engine for Bravery, a general
//! feat-effect/prerequisite engine, or Weapon Specialization's own
//! damage-roll application, and it does not ground Fighter level 13+. It
//! also preserves the accepted Fighter level-1..level-11 truth (unchanged)
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level11_sd18_armor_training3_deterministic_input.txt"
);

const FIGHTER_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level12_sd18_widening_deterministic_input.txt"
);

// ----- Base attack bonus and base saves genuinely rise at level 12 -----

#[test]
fn fighter_level12_base_attack_bonus_and_saves_genuinely_rise() {
    let input = load(FIGHTER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-12 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        bab.value, 12,
        "Fighter level 12 full-BAB progression must genuinely rise to 12, up from 11 at level \
         11: {}",
        bab.detail
    );

    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(
        fortitude.value, 8,
        "Fighter level 12 good Fortitude (12/2+2) must genuinely rise to 8, up from 7 at level \
         11"
    );

    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Fighter level 12 poor Reflex (12/3) must genuinely rise to 4, up from 3 at level 11"
    );

    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(
        will.value, 4,
        "Fighter level 12 poor Will (12/3) must genuinely rise to 4, up from 3 at level 11"
    );
}

// ----- The level-12 bonus feat seam is surfaced, contributing no mechanical value -----

#[test]
fn fighter_level12_grants_a_new_bonus_feat_seam() {
    let input = load(FIGHTER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let seam = explanation(&computation, "class_feature.fighter.level_12_bonus_feat");
    assert_eq!(
        seam.value, 0,
        "the level-12 bonus-feat seam names the selection only and contributes no computed \
         mechanical value: {seam:?}"
    );
    assert!(
        seam.detail.contains("weapon_specialization"),
        "level-12 bonus-feat seam must name the canonical Weapon Specialization selection: {}",
        seam.detail
    );

    // The level-10 seam still carries over.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_10_bonus_feat"),
        "level-12 Fighter must still carry the level-10 bonus-feat seam: {:?}",
        computation.explanations
    );
}

// ----- Armor Training and Weapon Training carry over unchanged -----

#[test]
fn fighter_level12_armor_training_and_weapon_training_stay_unchanged() {
    let input = load(FIGHTER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert_eq!(
        armor_training.value, 3,
        "Armor Training rank must stay 3 at level 12 (unchanged from level 11): {armor_training:?}"
    );

    let weapon_training = explanation(&computation, "class_feature.fighter.weapon_training");
    assert_eq!(
        weapon_training.value, 2,
        "Weapon Training's first-group attack bonus (1 + (12-5)/4) must stay +2 at level 12"
    );

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    assert_eq!(bravery.value, 3, "Bravery (1 + (12-2)/4) must stay +3 at level 12");
}

// ----- Baseline melee attack bonus rises by exactly the base-attack-bonus delta -----

#[test]
fn fighter_level12_baseline_melee_attack_bonus_rises() {
    let input = load(FIGHTER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Baseline melee attack bonus rises by exactly the base-attack-bonus
    // delta (+1, from 17 at level 11 to 18 at level 12), since neither
    // Weapon Training's rank nor Weapon Focus changes at level 12.
    assert_eq!(computation.baseline_melee_attack_bonus, 19);

    // Baseline armor class is unchanged: Armor Training's rank (and thus
    // maximum Dexterity bonus) does not rise again at level 12, and the
    // deterministic +2 Dexterity contribution stays well below the cap.
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.selected_skill_modifiers.climb, 8);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 8);
}

// ----- Negative control: the level-11 fixture is unaffected by this widening -----

#[test]
fn fighter_level11_truth_is_unchanged_by_this_slice() {
    let input = load(FIGHTER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 11, "Fighter level 11 base attack bonus must stay 11");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.level_12_bonus_feat"),
        "level-11 Fighter must not gain the level-12 bonus-feat seam"
    );

    assert_eq!(computation.baseline_melee_attack_bonus, 18);
}

// ----- Negative control: level 21 stays claim-blocked (PF1 has no 21st character level) -----
//
// SD18 (tests/sd18_fighter_level13_widening.rs, tests/sd18_fighter_level14_widening.rs,
// tests/sd18_fighter_level15_widening.rs, tests/sd18_fighter_level16_widening.rs,
// tests/sd18_fighter_level17_widening.rs, tests/sd18_fighter_level18_widening.rs,
// tests/sd18_fighter_level19_widening.rs) further widened the bounded
// tranche from level 12 to level 13 (Weapon Training 3, a third
// weapon-group choice slot), then to level 14 (a seventh bonus-feat cadence
// slot and the Bravery magnitude rise), then to level 15 (Armor Training
// 4), then to level 16 (an eighth bonus-feat cadence slot), then to level
// 17 (Weapon Training 4, a fourth weapon-group choice slot), then to level
// 18 (a ninth bonus-feat cadence slot and a further Bravery magnitude
// rise), and then to level 19 (the Armor Mastery flat-magnitude damage
// reduction record), and SD18 (tests/sd18_fighter_level20_widening.rs)
// further widened the bounded tranche to level 20 (a tenth bonus-feat
// cadence slot and the Weapon Mastery grant-only capstone record) -- the
// FINAL level within PF1's 1-20 character-level cap -- so this negative
// control now sits just above the current bound (level 21, which does not
// exist as a PF1 character level) rather than at level 13, level 14,
// level 15, level 16, level 17, level 18, level 19, or level 20.

#[test]
fn fighter_level_21_stays_claim_blocked() {
    let level_21 = FIGHTER_LEVEL12_FIXTURE.replace("class:fighter:12", "class:fighter:21");
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
fn multiclass_fighter_level12_is_not_promoted_by_this_slice() {
    let multiclass = FIGHTER_LEVEL12_FIXTURE.replace(
        "class_level=class:fighter:12",
        "class_level=class:fighter:12\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Fighter+Rogue multiclass mix into a
    // genuinely supported combination (via the table-driven
    // compute_generic_table_chassis path for Rogue), so this negative control is
    // superseded, not violated: Fighter level 12 / Rogue level 1 now gets a
    // real, integrated class_chassis.base_attack_bonus (Fighter 12's full BAB
    // 12 + Rogue 1's 3/4 BAB floor(1*3/4)=0, summed = 12), and with this
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
        computation.base_attack_bonus, 12,
        "Fighter 12 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 12"
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Fighter 12 / Rogue 1 with this fixture's full deterministic posture has no \
         remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_fighter_row_names_level_12_widening() {
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
        fighter.grounding_ref.contains("sd18_fighter_level12_widening"),
        "fighter row must cite the live SD18 level-12 widening proof surface: {}",
        fighter.grounding_ref
    );
    let note = fighter.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "fighter partial note must name the level-12 widening: {note}"
    );
}
