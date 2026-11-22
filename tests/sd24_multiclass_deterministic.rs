//! SD-24 Epic 5 (criterion 5.2): Deterministic test surface -- 30
//! character-advancement cycles for the Fighter+Wizard multiclass dispatch
//! criterion 5.1 already landed (`fighter_level_in_mix` /
//! `wizard_level_in_mix` in `pilot_compute.rs`; the widened entry gates in
//! `level_up::fighter` / `level_up::wizard`).
//!
//! Per `technical-design.md §2.2`'s own definition of the 30 cycles:
//!
//! - 10 cycles: solo Fighter, character level 1 -> 10.
//! - 10 cycles: solo Wizard, character level 1 -> 10.
//! - 10 cycles: Fighter+Wizard split at total level 5 (Fighter 4 / Wizard 1),
//!   advancing each side to level 10 -- 5 Fighter-side cycles (Fighter
//!   5..9 / Wizard 1, total level 6..10) and 5 Wizard-side cycles (Wizard
//!   5..9 / Fighter 1, total level 6..10). The split step itself (total
//!   level 5) is criterion 5.1's own test surface
//!   (`sd24_multiclass_fighter_wizard_split.rs`) and is not re-counted here.
//!
//! Every cycle asserts, against PF1's canonical formulas computed
//! independently in this file (not copied from `pilot_compute.rs`'s own
//! internals), that:
//!
//! - the base chassis computation is never claim-blocked
//!   (`class_chassis.unsupported`), and
//! - the base attack bonus / base save totals match canonical PF1 stacking
//!   exactly: Fighter is full BAB / good Fortitude / poor Reflex+Will;
//!   Wizard is half BAB / poor Fortitude+Reflex / good Will; a multiclass
//!   mix sums each class's own *unrounded* fractional save contribution and
//!   floors only once for the total (SD-21 E7.29's rule, the same one
//!   `sd24_multiclass_fighter_wizard_split.rs` and the two `*_lv10.rs` tests
//!   already exercise at their own single endpoints).
//!
//! Per-cycle input/output is captured in
//! `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_5/multiclass-fixture.md`.

use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
mod common;
use common::load;

const FIGHTER9_WIZARD1_LV10_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter9_wizard1_sd24_multiclass_lv10_input.txt"
);
const WIZARD9_FIGHTER1_LV10_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard9_fighter1_sd24_multiclass_lv10_input.txt"
);

/// A solo clone of `base`'s own posture (feats/choices/abilities), at a
/// single `class_id`/`level` -- the same isolated-clone technique
/// `sd24_multiclass_fighter_lv10.rs` / `sd24_multiclass_wizard_lv10.rs`
/// already use for their own level-1-9 walks.
fn solo(base: &CharacterInput, class_id: &str, level: u8) -> CharacterInput {
    let mut probe = base.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: class_id.to_owned(),
        level,
    }];
    probe
}

/// A Fighter+Wizard mix clone of `base`'s own posture at arbitrary per-class
/// levels -- generalizes the split fixtures' own fixed-level shape to any
/// point along the split-advance walk.
fn mix(base: &CharacterInput, fighter_level: u8, wizard_level: u8) -> CharacterInput {
    let mut probe = base.clone();
    probe.chosen.class_levels = vec![
        CharacterClassLevel {
            class_id: "class:fighter".to_owned(),
            level: fighter_level,
        },
        CharacterClassLevel {
            class_id: "class:wizard".to_owned(),
            level: wizard_level,
        },
    ];
    probe
}

fn assert_not_claim_blocked(computation: &PilotBaseChassisComputation, context: &str) {
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_chassis.unsupported" && d.claim_blocking),
        "{context} must not be claim-blocked on its own base chassis: {:?}",
        computation.diagnostics
    );
}

// ----- Canonical PF1 formulas, computed independently of `pilot_compute.rs` -----

fn fighter_bab(level: u8) -> i16 {
    // Fighter: full (1/1) base attack bonus progression.
    i16::from(level)
}

fn wizard_bab(level: u8) -> i16 {
    // Wizard: half (1/2) base attack bonus progression.
    i16::from(level / 2)
}

fn good_save_fraction(level: u8) -> f64 {
    f64::from(level) / 2.0 + 2.0
}

fn poor_save_fraction(level: u8) -> f64 {
    f64::from(level) / 3.0
}

// ----- 10 cycles: solo Fighter, level 1 -> 10 -----

#[test]
fn cycles_01_to_10_solo_fighter_level_1_to_10_matches_canonical_chassis() {
    let base = load(FIGHTER9_WIZARD1_LV10_FIXTURE);

    for level in 1..=10u8 {
        let input = solo(&base, "class:fighter", level);
        let computation = compute_pilot_base_chassis(&input);
        assert_not_claim_blocked(&computation, &format!("solo Fighter level {level}"));

        assert_eq!(
            computation.base_attack_bonus,
            fighter_bab(level),
            "solo Fighter level {level} base attack bonus mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.fortitude,
            good_save_fraction(level).floor() as i16,
            "solo Fighter level {level} good Fortitude mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.reflex,
            poor_save_fraction(level).floor() as i16,
            "solo Fighter level {level} poor Reflex mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.will,
            poor_save_fraction(level).floor() as i16,
            "solo Fighter level {level} poor Will mismatch: {:?}",
            computation.explanations
        );
    }
}

// ----- 10 cycles: solo Wizard, level 1 -> 10 -----

#[test]
fn cycles_11_to_20_solo_wizard_level_1_to_10_matches_canonical_chassis() {
    let base = load(WIZARD9_FIGHTER1_LV10_FIXTURE);

    for level in 1..=10u8 {
        let input = solo(&base, "class:wizard", level);
        let computation = compute_pilot_base_chassis(&input);
        assert_not_claim_blocked(&computation, &format!("solo Wizard level {level}"));

        assert_eq!(
            computation.base_attack_bonus,
            wizard_bab(level),
            "solo Wizard level {level} base attack bonus mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.fortitude,
            poor_save_fraction(level).floor() as i16,
            "solo Wizard level {level} poor Fortitude mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.reflex,
            poor_save_fraction(level).floor() as i16,
            "solo Wizard level {level} poor Reflex mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.will,
            good_save_fraction(level).floor() as i16,
            "solo Wizard level {level} good Will mismatch: {:?}",
            computation.explanations
        );

        // This cycle's own genuine coverage: Wizard's class-specific
        // explanation must keep firing at every solo step too (regression
        // guard alongside the mix-side assertion below).
        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
            "solo Wizard level {level} must ground its own level-1 prepared \
             arcane spell-bearing recognition: {:?}",
            computation.explanations
        );
    }
}

// ----- 10 cycles: Fighter+Wizard split-advance, Fighter-side (5) + Wizard-side (5) -----

#[test]
fn cycles_21_to_25_fighter_side_split_advance_level_6_to_10_matches_canonical_chassis() {
    let base = load(FIGHTER9_WIZARD1_LV10_FIXTURE);

    // Split at total level 5 (Fighter 4 / Wizard 1, criterion 5.1's own test
    // surface) already proven; this walk covers the 5 Fighter-side
    // advancement cycles beyond the split, total level 6 -> 10.
    for fighter_level in 5..=9u8 {
        let total_level = fighter_level + 1;
        let input = mix(&base, fighter_level, 1);
        let computation = compute_pilot_base_chassis(&input);
        assert_not_claim_blocked(
            &computation,
            &format!("Fighter {fighter_level} / Wizard 1 (total level {total_level})"),
        );

        let expected_bab = fighter_bab(fighter_level) + wizard_bab(1);
        assert_eq!(
            computation.base_attack_bonus, expected_bab,
            "Fighter {fighter_level} / Wizard 1 base attack bonus mismatch: {:?}",
            computation.explanations
        );

        let expected_fort = (good_save_fraction(fighter_level) + poor_save_fraction(1)).floor() as i16;
        let expected_ref = (poor_save_fraction(fighter_level) + poor_save_fraction(1)).floor() as i16;
        let expected_will = (poor_save_fraction(fighter_level) + good_save_fraction(1)).floor() as i16;
        assert_eq!(
            computation.base_saves.fortitude, expected_fort,
            "Fighter {fighter_level} / Wizard 1 Fortitude mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.reflex, expected_ref,
            "Fighter {fighter_level} / Wizard 1 Reflex mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.will, expected_will,
            "Fighter {fighter_level} / Wizard 1 Will mismatch: {:?}",
            computation.explanations
        );

        // Wizard's own class-specific explanation must keep surfacing at
        // every point along the Fighter-side split-advance walk, not just
        // at the two endpoints criterion 5.1's own tests already cover.
        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
            "Fighter {fighter_level} / Wizard 1 must keep Wizard's own \
             spell-baseline recognition surfaced inside the mix: {:?}",
            computation.explanations
        );
    }
}

#[test]
fn cycles_26_to_30_wizard_side_split_advance_level_6_to_10_matches_canonical_chassis() {
    let base = load(WIZARD9_FIGHTER1_LV10_FIXTURE);

    // Mirror image: split at total level 5 (Wizard 4 / Fighter 1) already
    // implied by criterion 5.1's own split test (Fighter-dominant); this
    // walk covers the 5 Wizard-side advancement cycles beyond the split,
    // total level 6 -> 10.
    for wizard_level in 5..=9u8 {
        let total_level = wizard_level + 1;
        let input = mix(&base, 1, wizard_level);
        let computation = compute_pilot_base_chassis(&input);
        assert_not_claim_blocked(
            &computation,
            &format!("Wizard {wizard_level} / Fighter 1 (total level {total_level})"),
        );

        let expected_bab = fighter_bab(1) + wizard_bab(wizard_level);
        assert_eq!(
            computation.base_attack_bonus, expected_bab,
            "Wizard {wizard_level} / Fighter 1 base attack bonus mismatch: {:?}",
            computation.explanations
        );

        let expected_fort = (good_save_fraction(1) + poor_save_fraction(wizard_level)).floor() as i16;
        let expected_ref = (poor_save_fraction(1) + poor_save_fraction(wizard_level)).floor() as i16;
        let expected_will = (poor_save_fraction(1) + good_save_fraction(wizard_level)).floor() as i16;
        assert_eq!(
            computation.base_saves.fortitude, expected_fort,
            "Wizard {wizard_level} / Fighter 1 Fortitude mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.reflex, expected_ref,
            "Wizard {wizard_level} / Fighter 1 Reflex mismatch: {:?}",
            computation.explanations
        );
        assert_eq!(
            computation.base_saves.will, expected_will,
            "Wizard {wizard_level} / Fighter 1 Will mismatch: {:?}",
            computation.explanations
        );

        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
            "Wizard {wizard_level} / Fighter 1 must keep Wizard's own \
             spell-baseline recognition surfaced inside the mix: {:?}",
            computation.explanations
        );
    }
}
