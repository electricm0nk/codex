//! SD18 Dwarf Defensive Training recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Dwarf race
//! seam (`explain_dwarf_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with a ninth grounded PF1 Core Rulebook Dwarf racial trait: Defensive
//! Training, a flat +4 dodge bonus to Armor Class against monsters of the
//! giant subtype
//! (`core_essentials/races/dwarf/dwarf_abilities_race.lst:22` —
//! `BONUS:VAR|RacialDefensiveTrainingBonus|4`).
//!
//! This mirrors the already-landed Dwarf Stability idiom (a single flat
//! bonus magnitude applied to a named derived-stat target): the recognized
//! value names only the flat dodge-bonus-to-AC magnitude, not an
//! Armor-Class-total or giant-subtype-detection engine. No such engine
//! exists anywhere in this codebase, so no check resolution is fabricated
//! from this record. Stonecunning, Greed, Hardy, and Stability (already
//! grounded) remain distinct and unchanged.
//!
//! Slice: cycle-2026-07-14T1900, matrix row_id: race.dwarf.bounded_semantics.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
mod common;
use common::{load, explanation};

const DWARF_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_dwarf_fighter_level1_sd13_deterministic_input.txt");

const DEFENSIVE_TRAINING_ID: &str = "race.dwarf.trait_bundle.defensive_training";

fn diagnostic<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        })
}

// ----- Defensive Training record exists on a Dwarf input, flat +4 magnitude -----

#[test]
fn dwarf_input_surfaces_defensive_training_trait_bundle_record() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let defensive_training = explanation(&computation, DEFENSIVE_TRAINING_ID);
    assert_eq!(
        defensive_training.value, 4,
        "Dwarf Defensive Training record must carry the grounded flat +4 dodge-bonus magnitude"
    );
    assert!(
        defensive_training.detail.contains("giant"),
        "Dwarf Defensive Training record must name the giant subtype: {}",
        defensive_training.detail
    );
    assert!(
        defensive_training.detail.contains("dodge"),
        "Dwarf Defensive Training record must name the dodge bonus: {}",
        defensive_training.detail
    );
    assert!(
        defensive_training.detail.contains("+4"),
        "Dwarf Defensive Training record detail must name the +4 magnitude: {}",
        defensive_training.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_defensive_training_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == DEFENSIVE_TRAINING_ID),
        "Human input must not surface the Dwarf Defensive Training record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Stability, Hardy, Greed, and Stonecunning stay grounded alongside the new record -----

#[test]
fn dwarf_input_still_surfaces_prior_families_alongside_defensive_training() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let stability = explanation(&computation, "race.dwarf.trait_bundle.stability");
    assert_eq!(
        stability.value, 4,
        "Stability must remain grounded unchanged by this slice"
    );
    let hardy = explanation(&computation, "race.dwarf.trait_bundle.hardy");
    assert_eq!(
        hardy.value, 2,
        "Hardy must remain grounded unchanged by this slice"
    );
    let greed = explanation(&computation, "race.dwarf.trait_bundle.greed");
    assert_eq!(
        greed.value, 2,
        "Greed must remain grounded unchanged by this slice"
    );
    let stonecunning = explanation(&computation, "race.dwarf.trait_bundle.stonecunning");
    assert_eq!(
        stonecunning.value, 2,
        "Stonecunning must remain grounded unchanged by this slice"
    );
}

// ----- The bounded diagnostic now names Defensive Training as grounded, not unproven -----

#[test]
fn dwarf_bounded_semantics_note_moves_defensive_training_out_of_unproven_list() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.dwarf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.dwarf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in ["Hatred", "weapon familiarity"] {
        assert!(
            bounded.message.contains(token),
            "race.dwarf.bounded_semantics must still name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("Defensive Training"),
        "race.dwarf.bounded_semantics must name Defensive Training as grounded: {}",
        bounded.message
    );
}

// ----- Control plane: the matrix row stays Partial/Computed, widened note -----

