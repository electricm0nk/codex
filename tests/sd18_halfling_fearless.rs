//! SD18 Halfling Fearless recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Halfling race
//! seam (`explain_halfling_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with a seventh grounded PF1 Core Rulebook Halfling racial trait: Fearless,
//! a flat +2 racial bonus on saving throws against fear
//! (`core_essentials/races/halfling/halfling_abilities_race.lst` — Fearless
//! entry, `BONUS:VAR|SaveBonus_vs_Fear|2|TYPE=Racial`).
//!
//! This mirrors the already-landed Dwarf Hardy flat racial saving-throw-bonus
//! idiom exactly (a flat magnitude on a named save category, not a
//! saving-throw-total engine): the recognized value names only the flat
//! racial-bonus magnitude, not a saving-throw resolution engine. Halfling
//! Luck (a separate luck bonus on all saving throws) and weapon familiarity
//! (sling and "halfling" weapons) remain distinct, still-unproven families
//! and are NOT grounded by this slice.
//!
//! Slice: cycle-2026-07-14T0800, matrix row_id: race.halfling.bounded_semantics.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
mod common;
use common::{load, explanation};

const HALFLING_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_halfling_fighter_level1_sd13_deterministic_input.txt");

const FEARLESS_ID: &str = "race.halfling.trait_bundle.fearless";

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

// ----- Fearless record exists on a Halfling input, flat +2 magnitude -----

#[test]
fn halfling_input_surfaces_fearless_trait_bundle_record() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fearless = explanation(&computation, FEARLESS_ID);
    assert_eq!(
        fearless.value, 2,
        "Halfling Fearless record must carry the grounded flat +2 \
         saving-throw-vs-fear racial-bonus magnitude"
    );
    assert!(
        fearless.detail.contains("fear"),
        "Halfling Fearless record must name the fear saving-throw category: {}",
        fearless.detail
    );
    assert!(
        fearless.detail.contains("+2"),
        "Halfling Fearless record detail must name the +2 magnitude: {}",
        fearless.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_halfling_fearless_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.explanations.iter().any(|e| e.id == FEARLESS_ID),
        "Human input must not surface the Halfling Fearless record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Sure-Footed and Keen Senses stay intact (non-regression on sibling families) -----

#[test]
fn halfling_input_still_surfaces_sure_footed_and_keen_senses_alongside_fearless() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sure_footed = explanation(&computation, "race.halfling.trait_bundle.sure_footed");
    assert_eq!(sure_footed.value, 2);

    let keen_senses = explanation(&computation, "race.halfling.trait_bundle.keen_senses");
    assert_eq!(keen_senses.value, 2);
}

// ----- The bounded diagnostic now names Fearless as grounded, not unproven -----

#[test]
fn halfling_bounded_semantics_note_moves_fearless_out_of_unproven_list() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.halfling.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.halfling.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in ["Halfling Luck", "weapon familiarity"] {
        assert!(
            bounded.message.contains(token),
            "race.halfling.bounded_semantics must still name the still-unproven '{token}' \
             trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("Fearless"),
        "race.halfling.bounded_semantics must name Fearless: {}",
        bounded.message
    );
}

// ----- Control plane: the matrix row stays Partial/Computed, widened note -----

