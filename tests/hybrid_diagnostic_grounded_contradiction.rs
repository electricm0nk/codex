//! Regression proof for a genuine self-contradiction found by direct code
//! tracing in `explain_hybrid_level1_chassis` (`src/rules_core/pilot_compute.rs`).
//!
//! `explain_hybrid_level1_chassis` is dispatched unconditionally for every
//! character. For a Human Paladin or Ranger at exactly level 1, single-class,
//! it used to push a `claim_blocking: true` diagnostic
//! (`class_feature.hybrid.<class>.unsupported`) flatly asserting the class's
//! non-spell class-feature burden "are not implemented in this bounded hybrid
//! chassis baseline".
//!
//! But a separate, later per-class decomposition function is ALSO dispatched
//! unconditionally for the exact same input and grounds real, non-fabricated
//! values for part of that same burden:
//!   - Paladin: `explain_paladin_level1_chassis_and_spell_burden_separation`
//!     grounds `class_chassis.paladin.smite_evil_attack_bonus` and
//!     `class_chassis.paladin.smite_evil_damage_bonus` for real.
//!   - Ranger: `explain_ranger_level1_chassis_and_class_feature_separation`
//!     grounds `class_chassis.ranger.track` and
//!     `class_chassis.ranger.favored_enemy_skill_bonus` /
//!     `..._attack_damage_bonus` for real.
//!
//! So a Human Paladin/Ranger level-1's computed output used to contain both a
//! diagnostic flatly asserting the feature is unimplemented AND an
//! explanation record with that exact feature's real grounded values --
//! a direct contradiction a real user could see, since Paladin and Ranger
//! are both `Computed`-status classes a user can actually build and save.
//!
//! This file proves: for a Human Paladin/Ranger level 1, the
//! `class_feature.hybrid.<class>.unsupported` diagnostic is either absent, or
//! (if ever reintroduced) must not claim non-implementation of a feature that
//! is simultaneously grounded for real elsewhere in the same computation.
//! Before the fix, this test fails: the diagnostic is present, claims Smite
//! Evil / Track / Favored Enemy are unimplemented, and the grounded
//! explanations are present at the same time. After the fix, the diagnostic
//! is gone entirely and the test passes vacuously-but-honestly on that arm,
//! while still positively confirming the grounded explanations are present
//! and real (non-placeholder, capable of nonzero values).

use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
mod common;
use common::load;

const PALADIN_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");
const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected grounded explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

#[test]
fn paladin_level1_hybrid_feature_diagnostic_does_not_contradict_grounded_smite_evil() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Smite Evil is genuinely grounded for this exact input: real, non-fabricated,
    // capable of nonzero values (attack bonus tracks Charisma modifier, damage bonus
    // tracks class level -- both nonzero-capable, not hardcoded placeholders).
    let attack_bonus = explanation(&computation, "class_chassis.paladin.smite_evil_attack_bonus");
    let damage_bonus = explanation(&computation, "class_chassis.paladin.smite_evil_damage_bonus");
    assert_eq!(
        damage_bonus.value, 1,
        "paladin level 1's Smite Evil damage bonus is a real, non-fabricated value equal to \
         class level (1), not a zero placeholder"
    );
    assert!(
        attack_bonus.value >= 0,
        "smite evil attack bonus must be a real computed value, not absent"
    );

    // The hybrid diagnostic must not simultaneously exist and claim Smite Evil is
    // unimplemented while the above records prove otherwise.
    if let Some(feature_diagnostic) = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.hybrid.paladin.unsupported")
    {
        assert!(
            !(feature_diagnostic.claim_blocking
                && feature_diagnostic.message.to_lowercase().contains("smite")),
            "CONTRADICTION: 'class_feature.hybrid.paladin.unsupported' is claim-blocking and \
             claims Smite Evil is unimplemented ({:?}), while \
             'class_chassis.paladin.smite_evil_attack_bonus'/'_damage_bonus' are simultaneously \
             present with real, non-fabricated values ({attack_bonus:?}, {damage_bonus:?})",
            feature_diagnostic.message
        );
    }
}

#[test]
fn ranger_level1_hybrid_feature_diagnostic_does_not_contradict_grounded_track_and_favored_enemy() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Track and the Favored Enemy flat surface are genuinely grounded for this exact
    // input: real, non-fabricated, capable of nonzero values.
    let track = explanation(&computation, "class_chassis.ranger.track");
    let favored_enemy_skill =
        explanation(&computation, "class_chassis.ranger.favored_enemy_skill_bonus");
    let favored_enemy_attack = explanation(
        &computation,
        "class_chassis.ranger.favored_enemy_attack_damage_bonus",
    );
    assert_eq!(
        track.value, 1,
        "ranger level 1's Track bonus is a real, non-fabricated value (max(level/2, 1) = 1), \
         not a zero placeholder"
    );
    assert_eq!(
        favored_enemy_skill.value, 2,
        "ranger level 1's Favored Enemy skill bonus is a real, non-fabricated flat +2, not a \
         zero placeholder"
    );
    assert_eq!(
        favored_enemy_attack.value, 2,
        "ranger level 1's Favored Enemy attack/damage bonus is a real, non-fabricated flat +2, \
         not a zero placeholder"
    );

    // The hybrid diagnostic must not simultaneously exist and claim favored enemy /
    // tracking are unimplemented while the above records prove otherwise.
    if let Some(feature_diagnostic) = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.hybrid.ranger.unsupported")
    {
        let lower = feature_diagnostic.message.to_lowercase();
        assert!(
            !(feature_diagnostic.claim_blocking
                && (lower.contains("favored enemy") || lower.contains("tracking"))),
            "CONTRADICTION: 'class_feature.hybrid.ranger.unsupported' is claim-blocking and \
             claims favored enemy/tracking are unimplemented ({:?}), while \
             'class_chassis.ranger.track'/'favored_enemy_skill_bonus'/'favored_enemy_attack_damage_bonus' \
             are simultaneously present with real, non-fabricated values ({track:?}, \
             {favored_enemy_skill:?}, {favored_enemy_attack:?})",
            feature_diagnostic.message
        );
    }
}
