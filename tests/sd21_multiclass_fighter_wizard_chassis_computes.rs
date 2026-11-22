//! SD-21 Epic 7 (multiclass stacking): E7.28 per-class-dispatch extension to
//! length-2+ `class_levels` mixes via a new `compute_multiclass_base_chassis`.
//!
//! Confirmed live 2026-07-18 (pre-launch verification, Epic 7 addendum): no
//! multiclass logic exists anywhere in `src/rules_core/` prior to this slice —
//! every `supported_*_level` gate matches only single-element `class_levels`, so
//! any length-2+ mix (even a mix of the two classes Epic 6 already grounds,
//! Fighter and Wizard) trips the universal `class_chassis.unsupported`
//! diagnostic and gets a fabricated `base_attack_bonus: 0` /
//! `base_saves: BaseSaves::default()`, exactly like an unrecognized single class
//! did before Epic 6.
//!
//! This slice extends `compute_class_chassis`'s dispatch (the single call site
//! `compute_pilot_base_chassis` already routes through) to recognize a length-2+
//! mix by running each class's own `compute_<class>_chassis` — Epic 6's
//! already-grounded Fighter and Wizard functions — in per-class isolation (a
//! synthetic single-class `CharacterInput` per class level) and combining the
//! results: base attack bonus is summed (safe as a plain sum here, since at
//! most one of Fighter's Full-BAB or Wizard's Half-BAB progression ever
//! contributes a fractional remainder across this pair, so naive-sum and
//! fractional-sum agree). Base saves are combined for now via the same
//! straightforward per-class-round-then-sum shape; E7.29 is the dedicated
//! follow-up cycle that replaces this with PF1's fractional-progression
//! save-stacking rule, since the two approaches diverge for other level pairs
//! (not exercised by this file's Fighter 4 / Wizard 4 reproducer).
//!
//! Widens `has_supported_class_chassis` (already Epic-6-widened from
//! Fighter-only to Fighter-or-Wizard) to also recognize a supported multiclass
//! mix, so `compute_total_saves` folds ability modifiers into the newly-real
//! multiclass base saves instead of staying claim-blocked.

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::pilot_compute::{
    BaseSaves,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
mod common;
use common::{load, explanation};

const FIGHTER_LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
}

/// Builds a length-2 Fighter <fighter_level> / Wizard <wizard_level> input by
/// overriding the Fighter level-1 fixture's single `class_levels` entry into a
/// two-entry mix, mirroring the clone-and-mutate idiom the Epic 6 negative
/// control (`sd21_wizard_chassis_computes.rs`) already used for its length-2
/// probe.
fn multiclass_fighter_wizard(fighter_level: u8, wizard_level: u8) -> CharacterInput {
    let mut input = load(FIGHTER_LEVEL_1_FIXTURE);
    let mut fighter = input.chosen.class_levels[0].clone();
    fighter.level = fighter_level;
    let mut wizard = fighter.clone();
    wizard.class_id = "class:wizard".to_string();
    wizard.level = wizard_level;
    input.chosen.class_levels = vec![fighter, wizard];
    input
}

#[test]
fn fighter4_wizard4_no_longer_trips_the_universal_class_chassis_unsupported_diagnostic() {
    let input = multiclass_fighter_wizard(4, 4);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(&computation, "class_chassis.unsupported"),
        "a Fighter 4 / Wizard 4 multiclass mix is now a dispatch-supported chassis: {:?}",
        computation.diagnostics
    );
}

#[test]
fn fighter4_wizard4_base_attack_bonus_and_base_saves_are_genuinely_computed() {
    let input = multiclass_fighter_wizard(4, 4);
    let computation = compute_pilot_base_chassis(&input);

    // Fighter 4 (Full BAB): 4. Wizard 4 (Half BAB): 4 / 2 = 2. Summed: 6.
    assert_eq!(
        computation.base_attack_bonus, 6,
        "Fighter 4 / Wizard 4 base attack bonus must be the genuine per-class sum, \
         not fabricated: {computation:?}"
    );
    // Fort: Fighter4 good (4/2+2=4) + Wizard4 poor (4/3=1) = 5.
    // Ref: Fighter4 poor (4/3=1) + Wizard4 poor (4/3=1) = 2.
    // Will: Fighter4 poor (4/3=1) + Wizard4 good (4/2+2=4) = 5.
    assert_eq!(
        computation.base_saves,
        BaseSaves {
            fortitude: 5,
            reflex: 2,
            will: 5,
        },
        "Fighter 4 / Wizard 4 base saves must be genuinely computed: {computation:?}"
    );

    let base_attack = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(base_attack.value, 6);
    assert_eq!(
        computation
            .explanations
            .iter()
            .filter(|e| e.id == "class_chassis.base_attack_bonus")
            .count(),
        1,
        "the combined multiclass base attack bonus must be a single explanation entry, \
         not one leaked per isolated per-class sub-computation (no id clobbering): {:?}",
        computation.explanations
    );
}

#[test]
fn fighter4_wizard4_total_saves_are_no_longer_claim_blocked() {
    let input = multiclass_fighter_wizard(4, 4);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(&computation, "defense.total_save.unsupported"),
        "total saves are now supported for a Fighter / Wizard multiclass mix: {:?}",
        computation.diagnostics
    );
}

#[test]
fn single_class_fighter_dispatch_is_unaffected_by_the_multiclass_extension() {
    let input = load(FIGHTER_LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(&computation, "class_chassis.unsupported"),
        "single-class Fighter must remain a genuinely supported dispatch target"
    );
    let base_attack = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        base_attack.value, computation.base_attack_bonus,
        "the multiclass extension must not change single-class Fighter's own chassis result"
    );
}

/// SD-21 E7.29: PF1's actual multiclass base-save rule is NOT a naive
/// per-class-round-then-sum (round each class's save down separately, then add
/// the two already-rounded integers) -- it sums each class's *un-rounded*
/// fractional save contribution across every class in the mix, and rounds down
/// only once for the total. A Fighter 3 / Wizard 2 mix is the reproducer that
/// distinguishes the two on Fortitude:
///
/// Fortitude: Fighter 3 good-save fraction (3/2 + 2 = 3.5) + Wizard 2 poor-save
/// fraction (2/3 = 0.667) = 4.167, floored once = 4. The naive shape instead
/// rounds each class down first (Fighter 3 fort = floor(3/2)+2 = 3, Wizard 2
/// fort = floor(2/3) = 0) and sums the integers = 3 -- one short of the
/// correct 4.
///
/// Reflex and Will do not diverge for this particular level pair (Reflex: both
/// classes are poor Reflex with fractional remainders that don't cross an
/// integer boundary together; Will: Fighter 3's poor-Will fraction is exactly
/// 1.0 and Wizard 2's good-Will fraction is exactly 3.0, so neither carries a
/// remainder to lose). Both are asserted anyway as same-mix controls that the
/// fractional fix doesn't perturb the values the naive shape already got
/// right.
#[test]
fn fighter3_wizard2_base_saves_use_fractional_progression_stacking_not_a_naive_sum() {
    let input = multiclass_fighter_wizard(3, 2);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        computation.base_saves,
        BaseSaves {
            fortitude: 4,
            reflex: 1,
            will: 4,
        },
        "Fighter 3 / Wizard 2 base saves must use PF1's sum-fractions-then-round-down-once \
         multiclass rule, not a naive per-class-round-then-sum (which would give fortitude \
         3 instead of the correct 4): {computation:?}"
    );
}

/// SD-21 E7.30: reconcile per-class feature integration for a multiclass mix.
///
/// `explain_fighter_class_features` (Fighter's Bravery / bonus-feat grants)
/// gates on `supported_fighter_level`, which requires a single-element
/// `class_levels` slice -- so before this cycle, a Fighter's own class-feature
/// explanations silently stopped firing the moment any other class joined the
/// mix, even though E7.28/E7.29 already made that same Fighter's BAB/save
/// chassis contribution genuinely computed. This is exactly the kind of silent
/// grant-loss "clobbering" this criterion closes: introducing Wizard into the
/// mix must not make Fighter's own already-earned class features vanish.
///
/// Fighter 4 (within a Fighter 4 / Wizard 4 mix) is past the Bravery 2
/// threshold, so `class_feature.fighter.bravery` must fire using Fighter's own
/// sub-level (4) within the mix, not the whole mix's length.
#[test]
fn fighter4_wizard4_fighter_class_features_still_fire_in_the_multiclass_mix() {
    let input = multiclass_fighter_wizard(4, 4);
    let computation = compute_pilot_base_chassis(&input);

    let bravery = explanation(&computation, "class_feature.fighter.bravery");
    // fighter_bravery_bonus(4) = 1 + (4 - 2) / 4 = 1 (integer division 2 / 4 = 0).
    assert_eq!(
        bravery.value, 1,
        "Fighter's own Bravery class feature must still fire, using Fighter's own \
         level (4) within the multiclass mix, not the mix's overall shape: {computation:?}"
    );
}

/// A companion negative control: the reconciliation must not have the reverse
/// problem either -- Fighter's per-class features firing in a multiclass mix
/// must not duplicate or otherwise clobber the combined chassis explanations
/// E7.28/E7.29 already ground (each keeps its own distinct, non-overlapping id
/// namespace: `class_chassis.*` for the combined pillar, `class_feature.fighter.*`
/// for Fighter's own features).
#[test]
fn fighter4_wizard4_class_feature_reconciliation_does_not_clobber_the_combined_chassis_explanations()
 {
    let input = multiclass_fighter_wizard(4, 4);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_chassis.base_attack_bonus",
        "class_chassis.base_save.fortitude",
        "class_chassis.base_save.reflex",
        "class_chassis.base_save.will",
    ] {
        assert_eq!(
            computation
                .explanations
                .iter()
                .filter(|e| e.id == id)
                .count(),
            1,
            "'{id}' must remain a single combined explanation entry even once Fighter's \
             own per-class features also fire in the mix: {:?}",
            computation.explanations
        );
    }
}
