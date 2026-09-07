//! SD-24 Epic 5 (criterion 5.1): Fighter + Wizard multiclass dispatch --
//! the Fighter-dominant advancement walk, level 1 -> 10.
//!
//! Criterion 5.1's own acceptance text: "GREEN: advancement from level 1 ->
//! 10 succeeds; BAB stacking follows canonical PF1 best-progression; saves
//! use best-fractional-progression." This test walks a Human Fighter
//! through every total character level 1-9 as a solo Fighter, then takes
//! its first Wizard level at total level 10 (Fighter 9 / Wizard 1) -- the
//! Fighter-side half of the two possible split shapes (mirrored by
//! `sd24_multiclass_wizard_lv10.rs`'s Wizard-side half).
//!
//! Per-level correctness for solo Fighter levels 1-9 is already exhaustively
//! proven elsewhere (SD13/SD18's own per-level fixtures and tests); this
//! walk's own job is narrower and specific to this criterion: confirm the
//! base chassis computation never fails (never surfaces a claim-blocking
//! `class_chassis.unsupported`) at ANY of the ten steps, including the
//! split step itself, and that the final level-10 BAB/save totals match
//! PF1's canonical additive multiclass stacking rule exactly.

use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use codex::rules_core::level_up::fighter::compute_fighter_level_up_grants;
use codex::rules_core::level_up::wizard::compute_wizard_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::load;

const FIGHTER9_WIZARD1_LV10_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter9_wizard1_sd24_multiclass_lv10_input.txt"
);

/// A solo Fighter clone of the level-10 mix fixture's own `chosen` state, at
/// `level` (1-9). Reuses the level-10 fixture's own feat/choice/ability
/// posture rather than re-deriving a separate fixture per level -- the same
/// isolated-clone technique `level_up/fighter.rs`'s own
/// `fighter_chassis_explanations` already uses.
fn solo_fighter_at(base: &CharacterInput, level: u8) -> CharacterInput {
    let mut probe = base.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:fighter".to_owned(),
        level,
    }];
    probe
}

#[test]
fn fighter_dominant_advancement_from_level_1_to_10_never_fails() {
    let base = load(FIGHTER9_WIZARD1_LV10_FIXTURE);

    // Levels 1-9: solo Fighter. Level 10: Fighter 9 / Wizard 1 -- the split.
    for level in 1..=9u8 {
        let input = solo_fighter_at(&base, level);
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_chassis.unsupported" && d.claim_blocking),
            "solo Fighter level {level} must not be claim-blocked on its \
             own base chassis: {:?}",
            computation.diagnostics
        );
    }

    let computation_at_10 = compute_pilot_base_chassis(&base);
    assert!(
        !computation_at_10
            .diagnostics
            .iter()
            .any(|d| d.id == "class_chassis.unsupported" && d.claim_blocking),
        "the split step (Fighter 9 / Wizard 1, total level 10) must not be \
         claim-blocked: {:?}",
        computation_at_10.diagnostics
    );
}

#[test]
fn fighter9_wizard1_level10_base_attack_bonus_is_the_additive_sum() {
    let base = load(FIGHTER9_WIZARD1_LV10_FIXTURE);
    let computation = compute_pilot_base_chassis(&base);

    // Fighter 9 (full BAB): classlevel = 9. Wizard 1 (1/2 BAB): floor(1/2) = 0.
    assert_eq!(
        computation.base_attack_bonus, 9,
        "Fighter 9 (BAB 9) + Wizard 1 (BAB 0) must sum to 9: {:?}",
        computation.explanations
    );
}

#[test]
fn fighter9_wizard1_level10_base_saves_use_sum_fractions_then_round_down_once() {
    let base = load(FIGHTER9_WIZARD1_LV10_FIXTURE);
    let computation = compute_pilot_base_chassis(&base);

    // Fighter 9: good Fortitude (9/2+2=6.5), poor Reflex (9/3=3.0), poor
    // Will (9/3=3.0). Wizard 1: poor Fortitude (1/3=0.333), poor Reflex
    // (1/3=0.333), good Will (1/2+2=2.5).
    // Fortitude 6.5+0.333=6.833 -> 6; Reflex 3.0+0.333=3.333 -> 3;
    // Will 3.0+2.5=5.5 -> 5.
    assert_eq!(computation.base_saves.fortitude, 6, "{:?}", computation.explanations);
    assert_eq!(computation.base_saves.reflex, 3, "{:?}", computation.explanations);
    assert_eq!(computation.base_saves.will, 5, "{:?}", computation.explanations);
}

#[test]
fn level10_split_step_grants_are_attributed_to_the_class_that_actually_changed() {
    let base = load(FIGHTER9_WIZARD1_LV10_FIXTURE);

    // Wizard's own sub-level changed (absent -> 1) at this step.
    let wizard_plan = compute_wizard_level_up_grants(&base, 0, 1);
    assert!(
        !wizard_plan.automatic_features.is_empty(),
        "Wizard's own LevelUpPlan must be non-empty at the split: {:?}",
        wizard_plan.automatic_features
    );

    // Fighter's own sub-level did NOT change at this step (stays at 9).
    let fighter_plan = compute_fighter_level_up_grants(&base, 9, 9);
    assert!(
        fighter_plan.automatic_features.is_empty(),
        "Fighter's own LevelUpPlan must stay empty for the unchanged \
         Fighter side of this split: {:?}",
        fighter_plan.automatic_features
    );
}
