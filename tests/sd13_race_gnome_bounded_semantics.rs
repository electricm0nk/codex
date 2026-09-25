//! SD13-E2 Gnome bounded race-semantics classification slice.
//!
//! Originally proved the first truthful SD13-E2 race-semantics classification
//! for the Gnome row: at the time, the live rules-core surface ingested a
//! deterministic `race:gnome` + `class:fighter:1` input and left only the
//! generic non-claim-blocking `race.semantics.unverified` diagnostic, with no
//! Gnome-specific trait math grounded.
//!
//! The SD13-E2 Gnome bounded race-semantics recognition slice
//! (`tests/sd13_gnome_race_semantics_recognition.rs`) executed the promotion
//! path this file's original guards anticipated: it landed grounded evidence
//! for four race-semantic families (ability modifiers, size, speed, senses) and
//! updated the row state in the typed matrix carrier. This file now pins that
//! promoted truth for the negative-control fixture below (a `race:gnome`
//! Fighter input that predates the recognition slice and still deliberately
//! omits every Human-specific selection): the Gnome race seam no longer emits
//! the generic diagnostic for this fixture, and the matrix row is `Partial` /
//! `Computed` rather than `Unverified` / `Observed`.
//!
//! It is intentionally not a Gnome race engine. It grounds no Gnome defensive
//! training, illusion resistance, hatred, keen senses, Gnome Magic, or weapon
//! familiarity math.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::load;

const GNOME_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_gnome_fighter_level1_sd13_deterministic_input.txt");

fn has_explanation(
    computation: &codex::rules_core::pilot_compute::PilotBaseChassisComputation,
    id: &str,
) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Direct runtime evidence: the bounded Gnome pilot classification -----

#[test]
fn gnome_pilot_produces_computed_outputs_through_a_non_human_race() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The bounded pilot still runs through a Gnome chassis: the deterministic
    // compute path is not claim-blocked by a non-Human race, so it must
    // surface computed Fighter level-1 chassis evidence (base attack bonus
    // and the three base saves) using the established per-stat chassis
    // explanation ids.
    for fighter_chassis_id in [
        "class_chassis.base_attack_bonus",
        "class_chassis.base_save.fortitude",
        "class_chassis.base_save.reflex",
        "class_chassis.base_save.will",
    ] {
        assert!(
            has_explanation(&computation, fighter_chassis_id),
            "Gnome pilot must still surface Fighter level-1 chassis evidence '{fighter_chassis_id}': {:?}",
            computation.explanations
        );
    }

    // The Gnome race seam now surfaces its own bounded, non-claim-blocking note
    // (SD13-E2 recognition slice) instead of the generic unverified-race
    // diagnostic every other non-Human race still receives.
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.gnome.bounded_semantics" && !d.claim_blocking),
        "Gnome pilot must emit the bounded Gnome race semantics note: {:?}",
        computation.diagnostics
    );

    // The Gnome race seam must NOT emit any Human-only race explanation records.
    // Gnome-specific trait math beyond the four recognized dimensions
    // (defensive training, illusion resistance, hatred, keen senses, Gnome
    // Magic, weapon familiarity) is not implemented and must not be fabricated.
    for human_only in [
        "race.human.ability_bonus_target",
        "race.human.bonus_feat_grant",
    ] {
        assert!(
            !has_explanation(&computation, human_only),
            "Gnome pilot must not fabricate Human-only race explanations: '{human_only}' present in {:?}",
            computation.explanations
        );
    }

    // The Human-only bounded note must not be emitted for a Gnome chassis.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics"),
        "Gnome pilot must not surface the Human-only bounded race note: {:?}",
        computation.diagnostics
    );
}
