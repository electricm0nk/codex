//! SD18 Halfling Luck recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Halfling race
//! seam (`explain_halfling_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with an eighth grounded PF1 Core Rulebook Halfling racial trait: Halfling
//! Luck, a flat +1 racial bonus on all saving throws
//! (`core_essentials/races/halfling/halfling_abilities_race.lst` — Halfling
//! Luck entry, `BONUS:VAR|Halfling_HalflingLuck_SaveBonus|1|TYPE=Racial`).
//!
//! This mirrors the already-landed Dwarf Hardy / Halfling Fearless flat
//! racial saving-throw-bonus idiom exactly (a flat magnitude on a named save
//! scope, not a saving-throw-total engine): the recognized value names only
//! the flat racial-bonus magnitude, not a saving-throw resolution engine.
//! Weapon familiarity (sling and "halfling" weapons) remains the sole
//! distinct, still-unproven Halfling family and is NOT grounded by this
//! slice.
//!
//! Slice: cycle-2026-07-14T1407, matrix row_id: race.halfling.bounded_semantics.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const HALFLING_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_halfling_fighter_level1_sd13_deterministic_input.txt");

const HALFLING_LUCK_ID: &str = "race.halfling.trait_bundle.halfling_luck";

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

// ----- Halfling Luck record exists on a Halfling input, flat +1 magnitude -----

#[test]
fn halfling_input_surfaces_halfling_luck_trait_bundle_record() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let luck = explanation(&computation, HALFLING_LUCK_ID);
    assert_eq!(
        luck.value, 1,
        "Halfling Luck record must carry the grounded flat +1 all-saving-throws \
         racial-bonus magnitude"
    );
    assert!(
        luck.detail.contains("saving throw"),
        "Halfling Luck record must name the saving-throw scope: {}",
        luck.detail
    );
    assert!(
        luck.detail.contains("+1"),
        "Halfling Luck record detail must name the +1 magnitude: {}",
        luck.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_halfling_luck_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == HALFLING_LUCK_ID),
        "Human input must not surface the Halfling Luck record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Fearless, Sure-Footed and Keen Senses stay intact (non-regression on sibling families) -----

#[test]
fn halfling_input_still_surfaces_sibling_families_alongside_halfling_luck() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fearless = explanation(&computation, "race.halfling.trait_bundle.fearless");
    assert_eq!(fearless.value, 2);

    let sure_footed = explanation(&computation, "race.halfling.trait_bundle.sure_footed");
    assert_eq!(sure_footed.value, 2);

    let keen_senses = explanation(&computation, "race.halfling.trait_bundle.keen_senses");
    assert_eq!(keen_senses.value, 2);
}

// ----- The bounded diagnostic now names Halfling Luck as grounded, not unproven -----

#[test]
fn halfling_bounded_semantics_note_moves_halfling_luck_out_of_unproven_list() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.halfling.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.halfling.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven family from before this slice must
    // still be named honestly.
    assert!(
        bounded.message.contains("weapon familiarity"),
        "race.halfling.bounded_semantics must still name the still-unproven 'weapon \
         familiarity' trait: {}",
        bounded.message
    );
    assert!(
        bounded.message.contains("Halfling Luck"),
        "race.halfling.bounded_semantics must name Halfling Luck: {}",
        bounded.message
    );
}

// ----- Control plane: the matrix row stays Partial/Computed, widened note -----

#[test]
fn matrix_halfling_row_stays_partial_computed_and_grounding_ref_names_this_slice() {
    let matrix = seeded_current_truth();
    let halfling = matrix
        .row("race.halfling.bounded_semantics")
        .expect("halfling row must exist");

    // NOTE (2026-07-16, SD-19 Full-matrix closure): this slice's own honest
    // Partial -> Partial widening claim below is historically accurate for THIS
    // slice, but the row was later promoted to Supported/ProductVisible by the
    // separate SD-19 Race Trait Catalog browser UI-surfacing work, which is why
    // the assertions after this comment now read Supported/ProductVisible.
    // Honest promotion: Partial -> Partial widening, NOT a jump to Supported.
    // Halfling Luck is one more grounded family among the still-unproven ones
    // (weapon familiarity), so the row does not reach Supported this cycle.
    assert_eq!(halfling.support_state, SupportState::Supported);
    assert_eq!(halfling.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        halfling.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        halfling.grounding_ref.contains("sd18_halfling_luck"),
        "halfling row grounding_ref must cite this slice's proof surface: {}",
        halfling.grounding_ref
    );
    assert!(
        halfling.blocker_or_lossiness_note.contains("Halfling Luck"),
        "halfling row note must still mention Halfling Luck by name: {}",
        halfling.blocker_or_lossiness_note
    );
    assert!(
        halfling
            .blocker_or_lossiness_note
            .contains("weapon familiarity"),
        "halfling row note must name the distinct, still-unproven weapon familiarity \
         trait: {}",
        halfling.blocker_or_lossiness_note
    );
}
