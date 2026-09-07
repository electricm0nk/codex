//! SD-24 Epic 5 (criterion 5.1): Fighter + Wizard multiclass dispatch --
//! the Wizard-dominant advancement walk, level 1 -> 10.
//!
//! Criterion 5.1's own acceptance text: "GREEN: advancement from level 1 ->
//! 10 succeeds; BAB stacking follows canonical PF1 best-progression; saves
//! use best-fractional-progression." This test walks a Human Wizard through
//! every total character level 1-9 as a solo Wizard, then takes its first
//! Fighter level at total level 10 (Wizard 9 / Fighter 1) -- the mirror
//! image of `sd24_multiclass_fighter_lv10.rs`'s Fighter-side walk, proving
//! the split-class transition is symmetric regardless of which class is
//! the newly-joining one.
//!
//! Per-level correctness for solo Wizard levels 1-9 is already exhaustively
//! proven elsewhere (SD13/SD18's own per-level fixtures and tests); this
//! walk's own job is narrower and specific to this criterion: confirm the
//! base chassis computation never fails (never surfaces a claim-blocking
//! `class_chassis.unsupported`) at ANY of the ten steps, including the
//! split step itself, that Wizard's own explanations keep firing at every
//! step (this cycle's actual fix), and that the final level-10 BAB/save
//! totals match PF1's canonical additive multiclass stacking rule exactly.

use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use codex::rules_core::level_up::fighter::compute_fighter_level_up_grants;
use codex::rules_core::level_up::wizard::compute_wizard_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::load;

const WIZARD9_FIGHTER1_LV10_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard9_fighter1_sd24_multiclass_lv10_input.txt"
);

/// A solo Wizard clone of the level-10 mix fixture's own `chosen` state, at
/// `level` (1-9). Reuses the level-10 fixture's own feat/choice/ability
/// posture rather than re-deriving a separate fixture per level -- the same
/// isolated-clone technique `level_up/wizard.rs`'s own
/// `wizard_chassis_explanations` already uses.
fn solo_wizard_at(base: &CharacterInput, level: u8) -> CharacterInput {
    let mut probe = base.clone();
    probe.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:wizard".to_owned(),
        level,
    }];
    probe
}

#[test]
fn wizard_dominant_advancement_from_level_1_to_10_never_fails() {
    let base = load(WIZARD9_FIGHTER1_LV10_FIXTURE);

    // Levels 1-9: solo Wizard. Level 10: Wizard 9 / Fighter 1 -- the split.
    for level in 1..=9u8 {
        let input = solo_wizard_at(&base, level);
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_chassis.unsupported" && d.claim_blocking),
            "solo Wizard level {level} must not be claim-blocked on its own \
             base chassis: {:?}",
            computation.diagnostics
        );
        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
            "solo Wizard level {level} must still ground its own level-1 \
             prepared arcane spell-bearing recognition: {:?}",
            computation.explanations
        );
    }

    let computation_at_10 = compute_pilot_base_chassis(&base);
    assert!(
        !computation_at_10
            .diagnostics
            .iter()
            .any(|d| d.id == "class_chassis.unsupported" && d.claim_blocking),
        "the split step (Wizard 9 / Fighter 1, total level 10) must not be \
         claim-blocked: {:?}",
        computation_at_10.diagnostics
    );
    assert!(
        computation_at_10
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "Wizard's own spell-baseline recognition must still surface once \
         Fighter joins the mix at the split -- this is this cycle's actual \
         fix (`wizard_level_in_mix`): {:?}",
        computation_at_10.explanations
    );
}

#[test]
fn wizard9_fighter1_level10_base_attack_bonus_is_the_additive_sum() {
    let base = load(WIZARD9_FIGHTER1_LV10_FIXTURE);
    let computation = compute_pilot_base_chassis(&base);

    // Wizard 9 (1/2 BAB): floor(9/2) = 4. Fighter 1 (full BAB): classlevel = 1.
    assert_eq!(
        computation.base_attack_bonus, 5,
        "Wizard 9 (BAB 4) + Fighter 1 (BAB 1) must sum to 5: {:?}",
        computation.explanations
    );
}

#[test]
fn wizard9_fighter1_level10_base_saves_use_sum_fractions_then_round_down_once() {
    let base = load(WIZARD9_FIGHTER1_LV10_FIXTURE);
    let computation = compute_pilot_base_chassis(&base);

    // Wizard 9: poor Fortitude (9/3=3.0), poor Reflex (9/3=3.0), good Will
    // (9/2+2=6.5). Fighter 1: good Fortitude (1/2+2=2.5), poor Reflex
    // (1/3=0.333), poor Will (1/3=0.333).
    // Fortitude 3.0+2.5=5.5 -> 5; Reflex 3.0+0.333=3.333 -> 3;
    // Will 6.5+0.333=6.833 -> 6.
    assert_eq!(computation.base_saves.fortitude, 5, "{:?}", computation.explanations);
    assert_eq!(computation.base_saves.reflex, 3, "{:?}", computation.explanations);
    assert_eq!(computation.base_saves.will, 6, "{:?}", computation.explanations);
}

#[test]
fn level10_split_step_grants_are_attributed_to_the_class_that_actually_changed() {
    let base = load(WIZARD9_FIGHTER1_LV10_FIXTURE);

    // Fighter's own sub-level changed (absent -> 1) at this step.
    let fighter_plan = compute_fighter_level_up_grants(&base, 0, 1);
    assert!(
        !fighter_plan.automatic_features.is_empty(),
        "Fighter's own LevelUpPlan must be non-empty at the split: {:?}",
        fighter_plan.automatic_features
    );

    // Wizard's own sub-level did NOT change at this step (stays at 9).
    let wizard_plan = compute_wizard_level_up_grants(&base, 9, 9);
    assert!(
        wizard_plan.automatic_features.is_empty(),
        "Wizard's own LevelUpPlan must stay empty for the unchanged Wizard \
         side of this split: {:?}",
        wizard_plan.automatic_features
    );
}
