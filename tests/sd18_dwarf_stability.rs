//! SD18 Dwarf Stability recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Dwarf race
//! seam (`explain_dwarf_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with an eighth grounded PF1 Core Rulebook Dwarf racial trait: Stability,
//! a flat +4 racial bonus to Combat Maneuver Defense when resisting a bull
//! rush or trip attempt while standing on the ground
//! (`core_essentials/races/dwarf/dwarf_abilities_race.lst:26` —
//! `BONUS:VAR|CMD_BullRush,CMD_Trip|4|TYPE=Racial`).
//!
//! This mirrors the already-landed Dwarf Hardy idiom exactly (a single flat
//! bonus magnitude applied to two named derived-stat targets): the
//! recognized value names only the flat CMD-bonus magnitude, not a
//! Combat-Maneuver-Defense-total or bull-rush/trip-resolution engine. No
//! such engine exists anywhere in this codebase, so no check resolution is
//! fabricated from this record. Stonecunning, Greed, and Hardy (already
//! grounded) remain distinct and unchanged.
//!
//! Slice: cycle-2026-07-14T1500, matrix row_id: race.dwarf.bounded_semantics.

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

const DWARF_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_dwarf_fighter_level1_sd13_deterministic_input.txt");

const STABILITY_ID: &str = "race.dwarf.trait_bundle.stability";

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

// ----- Stability record exists on a Dwarf input, flat +4 magnitude -----

#[test]
fn dwarf_input_surfaces_stability_trait_bundle_record() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let stability = explanation(&computation, STABILITY_ID);
    assert_eq!(
        stability.value, 4,
        "Dwarf Stability record must carry the grounded flat +4 CMD-bonus magnitude"
    );
    assert!(
        stability.detail.contains("bull rush"),
        "Dwarf Stability record must name the bull rush maneuver: {}",
        stability.detail
    );
    assert!(
        stability.detail.contains("trip"),
        "Dwarf Stability record must name the trip maneuver: {}",
        stability.detail
    );
    assert!(
        stability.detail.contains("+4"),
        "Dwarf Stability record detail must name the +4 magnitude: {}",
        stability.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_stability_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.explanations.iter().any(|e| e.id == STABILITY_ID),
        "Human input must not surface the Dwarf Stability record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Hardy, Greed, and Stonecunning stay grounded alongside the new Stability record -----

#[test]
fn dwarf_input_still_surfaces_hardy_greed_and_stonecunning_alongside_stability() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

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

// ----- The bounded diagnostic now names Stability as grounded, not unproven -----

#[test]
fn dwarf_bounded_semantics_note_moves_stability_out_of_unproven_list() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.dwarf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.dwarf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in ["Defensive Training", "Hatred", "weapon familiarity"] {
        assert!(
            bounded.message.contains(token),
            "race.dwarf.bounded_semantics must still name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("Stability"),
        "race.dwarf.bounded_semantics must name Stability as grounded: {}",
        bounded.message
    );
}

// ----- Control plane: the matrix row stays Partial/Computed, widened note -----

#[test]
fn matrix_dwarf_row_stays_partial_computed_and_grounding_ref_names_this_slice() {
    let matrix = seeded_current_truth();
    let dwarf = matrix
        .row("race.dwarf.bounded_semantics")
        .expect("dwarf row must exist");

    // NOTE (2026-07-16, SD-19 Full-matrix closure): this slice's own honest
    // Partial -> Partial widening claim below is historically accurate for THIS
    // slice, but the row was later promoted to Supported/ProductVisible by the
    // separate SD-19 Race Trait Catalog browser UI-surfacing work, which is why
    // the assertions after this comment now read Supported/ProductVisible.
    // Honest promotion: Partial -> Partial widening, NOT a jump to Supported.
    // Stability is one more grounded family among several still-unproven ones
    // (Defensive Training, Hatred, weapon familiarity), so the row does not
    // reach Supported this cycle.
    assert_eq!(dwarf.support_state, SupportState::Supported);
    assert_eq!(dwarf.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        dwarf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        dwarf.grounding_ref.contains("sd18_dwarf_stability"),
        "dwarf row grounding_ref must cite this slice's proof surface: {}",
        dwarf.grounding_ref
    );
    assert!(
        dwarf.blocker_or_lossiness_note.contains("Stability"),
        "dwarf row note must name Stability as grounded, not unproven: {}",
        dwarf.blocker_or_lossiness_note
    );
}
