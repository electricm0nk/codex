//! SD-24 Epic 5 (criterion 5.1): Fighter + Wizard multiclass dispatch --
//! the split-class transition.
//!
//! Criterion 5.1's own acceptance text: "RED: advancement from level 1 -> 10
//! fails at level 5 (the multiclass split-class transition level) for the
//! Fighter-side or Wizard-side. GREEN: advancement from level 1 -> 10
//! succeeds; BAB stacking follows canonical PF1 best-progression; saves use
//! best-fractional-progression."
//!
//! This test drives the literal split point: a Human Fighter 4 that takes
//! its first Wizard level at total character level 5 (Fighter 4 / Wizard
//! 1), the smallest concrete instance of the "split-class transition" the
//! criterion names.
//!
//! Two independent pillars are exercised:
//!
//! 1. The base-attack-bonus / base-save chassis pillar
//!    (`pilot_compute::compute_pilot_base_chassis`), whose multiclass
//!    stacking (`compute_multiclass_base_chassis`, SD-21 E7.28/E7.29) was
//!    already landed before this cycle -- these assertions are a
//!    regression check confirming the split does not disturb it, not this
//!    cycle's own fix.
//!
//! 2. Wizard's own per-class explanations
//!    (`explain_wizard_level1_prepared_spell_baseline`) and the Level Up
//!    grant model (`level_up::wizard::compute_wizard_level_up_grants`) --
//!    both of which, before this cycle, silently dropped every Wizard-
//!    specific fact the instant Wizard joined a multiclass mix (gated on
//!    `supported_wizard_level`, a single-class-only check), mirroring the
//!    identical gap `fighter_level_in_mix` (SD-21 E7.30) already closed for
//!    Fighter's own features. This is the RED this cycle closes.

use codex::rules_core::character_input::CharacterClassLevel;
use codex::rules_core::level_up::fighter::compute_fighter_level_up_grants;
use codex::rules_core::level_up::wizard::compute_wizard_level_up_grants;
use codex::rules_core::pilot_compute::{BaseSaves, compute_pilot_base_chassis};
mod common;
use common::load;

const FIGHTER4_WIZARD1_SPLIT_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter4_wizard1_sd24_multiclass_split_input.txt"
);

// ----- Pillar 1: base chassis (BAB / save) stacking at the split level -----

#[test]
fn fighter4_wizard1_split_base_attack_bonus_is_the_additive_sum() {
    let input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fighter 4 (full BAB): classlevel = 4. Wizard 1 (1/2 BAB): floor(1/2) = 0.
    // PF1's canonical multiclass rule: base attack bonus from each class
    // simply adds together.
    assert_eq!(
        computation.base_attack_bonus, 4,
        "Fighter 4 (BAB 4) + Wizard 1 (BAB 0) must sum to 4: {:?}",
        computation.explanations
    );
}

#[test]
fn fighter4_wizard1_split_base_saves_use_sum_fractions_then_round_down_once() {
    let input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fighter 4: good Fortitude (4/2+2=4.0), poor Reflex (4/3=1.333), poor
    // Will (4/3=1.333). Wizard 1: poor Fortitude (1/3=0.333), poor Reflex
    // (1/3=0.333), good Will (1/2+2=2.5). PF1's canonical multiclass save
    // rule sums each class's own UN-ROUNDED fractional contribution before
    // rounding down once for the total (not a naive per-class-round-then-sum):
    // Fortitude 4.0+0.333=4.333 -> 4; Reflex 1.333+0.333=1.667 -> 1;
    // Will 1.333+2.5=3.833 -> 3.
    assert_eq!(
        computation.base_saves.fortitude, 4,
        "combined fractional Fortitude 4.333 must floor to 4: {:?}",
        computation.explanations
    );
    assert_eq!(
        computation.base_saves.reflex, 1,
        "combined fractional Reflex 1.667 must floor to 1: {:?}",
        computation.explanations
    );
    assert_eq!(
        computation.base_saves.will, 3,
        "combined fractional Will 3.833 must floor to 3: {:?}",
        computation.explanations
    );
}

#[test]
fn fighter4_wizard1_split_base_chassis_is_never_claim_blocked() {
    let input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_chassis.unsupported" && d.claim_blocking),
        "a supported Fighter+Wizard mix must not surface \
         class_chassis.unsupported: {:?}",
        computation.diagnostics
    );
}

// ----- Pillar 2: Wizard's own explanations keep firing inside the mix -----

#[test]
fn fighter4_wizard1_split_wizard_spell_baseline_still_surfaces_inside_the_mix() {
    let input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let has = |id: &str| computation.explanations.iter().any(|e| e.id == id);

    assert!(
        has("class_chassis.spell_baseline.wizard"),
        "Wizard's level-1 prepared arcane spell-bearing recognition must \
         still surface once Wizard joins a supported Fighter+Wizard mix: {:?}",
        computation.explanations
    );
    assert!(
        has("class_chassis.wizard.scribe_scroll"),
        "Wizard's Scribe Scroll bonus-feat recognition must still surface \
         inside the mix: {:?}",
        computation.explanations
    );
    assert!(
        has("class_chassis.wizard.specialization_choice"),
        "Wizard's canonical school specialization recognition must still \
         surface inside the mix: {:?}",
        computation.explanations
    );
}

#[test]
fn fighter4_wizard1_split_fighter_class_features_are_unaffected_regression() {
    // Regression: SD-21 E7.30 already reconciled Fighter's own class
    // features (fighter_level_in_mix) for a supported mix. This cycle's
    // Wizard-side fix must not disturb that.
    let input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.fighter.bravery"),
        "Fighter 4's Bravery must still surface inside the mix (regression \
         check on SD-21 E7.30): {:?}",
        computation.explanations
    );
}

// ----- Pillar 2 (continued): Level Up grant model at the split step -----

#[test]
fn wizard_level_up_grants_fire_at_the_split_from_absent_to_level_1() {
    let input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);

    // Wizard's own sub-level transition at the split: absent (0, i.e. not
    // yet part of the build) -> 1 (Wizard's first level, the moment it
    // joins the mix).
    let plan = compute_wizard_level_up_grants(&input, 0, 1);

    assert!(
        !plan.automatic_features.is_empty(),
        "Wizard's own LevelUpPlan must NOT come back empty at the split -- \
         before this cycle's fix, a length-2 class_levels mix failed this \
         module's single-class-only entry gate and silently returned an \
         honestly-empty (but wrong-for-this-input) plan: {:?}",
        plan
    );

    let has_grant = |needle: &str| {
        plan.automatic_features
            .iter()
            .any(|g| g.effects.iter().any(|e| e.description.contains(needle)))
    };
    assert!(
        has_grant("prepared arcane spell-bearing baseline"),
        "Wizard's level-1 prepared arcane spell-bearing recognition must be \
         a newly-granted automatic feature at the split: {:?}",
        plan.automatic_features
    );
}

#[test]
fn fighter_level_up_grants_stay_empty_for_the_wizard_side_of_the_split() {
    let input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);

    // Fighter's own sub-level does NOT change at this split (it stays at 4
    // -- only Wizard's own level changed, from absent to 1). Fighter's own
    // LevelUpPlan for this same step must therefore be empty: dispatch
    // attributes the split's new grants to the class that actually changed,
    // not to both.
    let plan = compute_fighter_level_up_grants(&input, 4, 4);

    assert!(
        plan.automatic_features.is_empty(),
        "Fighter's own sub-level is unchanged (4 -> 4) at the Wizard-side \
         split step, so Fighter's plan must stay empty, not double-grant \
         already-earned features: {:?}",
        plan.automatic_features
    );
}

// ----- (v0.6 swarm update): Fighter+Rogue is now a supported multiclass mix -----

#[test]
fn multiclass_fighter_rogue_mix_is_now_supported() {
    // Was `multiclass_fighter_rogue_mix_is_still_unsupported`, pinning the
    // pre-v0.6-swarm truth that only Fighter+Wizard was a recognized
    // multiclass pair. The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened `table_class_id` to also recognize
    // Rogue via the table-driven `compute_generic_table_chassis` path, so
    // Fighter+Rogue is now genuinely supported too -- renamed and flipped
    // rather than left with a stale, misleading name.
    let mut input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);
    input.chosen.class_levels = vec![
        CharacterClassLevel {
            class_id: "class:fighter".to_owned(),
            level: 4,
        },
        CharacterClassLevel {
            class_id: "class:rogue".to_owned(),
            level: 1,
        },
    ];
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_chassis.unsupported"),
        "Fighter+Rogue is now a supported multiclass mix, so class_chassis.unsupported must not \
         fire: {:?}",
        computation.diagnostics
    );
    assert_eq!(
        computation.base_attack_bonus, 4,
        "Fighter 4 (full BAB) + Rogue 1 (3/4 BAB, floor(1*3/4)=0) = 4"
    );
    assert_eq!(
        computation.base_saves,
        BaseSaves {
            fortitude: 4,
            reflex: 3,
            will: 1,
        },
        "Fortitude: Fighter4 good (4/2+2=4.0) + Rogue1 poor (1/3=0.333) = 4.333 -> 4; \
         Reflex: Fighter4 poor (4/3=1.333) + Rogue1 good (1/2+2=2.5) = 3.833 -> 3; \
         Will: Fighter4 poor (4/3=1.333) + Rogue1 poor (1/3=0.333) = 1.667 -> 1"
    );

    // Fighter's own level-up grant plan is unrelated to class-mix recognition: with
    // from_level == to_level (4 -> 4), no level transition occurred, so no grants
    // are expected regardless of which second class is in the mix.
    let fighter_plan = compute_fighter_level_up_grants(&input, 4, 4);
    assert!(
        fighter_plan.automatic_features.is_empty(),
        "no level transition occurred (4 -> 4), so no automatic features should be granted: {:?}",
        fighter_plan.automatic_features
    );
}
