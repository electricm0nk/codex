//! SD18 Dwarf Hardy recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Dwarf race
//! seam (`explain_dwarf_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with a seventh grounded PF1 Core Rulebook Dwarf racial trait: Hardy, a
//! flat +2 racial bonus on saving throws against poison, spells, and
//! spell-like abilities
//! (`core_essentials/races/dwarf/dwarf_abilities_race.lst:25` —
//! `BONUS:VAR|SaveBonus_vs_Poison|2|TYPE=Racial` and
//! `BONUS:VAR|SaveBonus_vs_Spells|2|TYPE=Racial`).
//!
//! This mirrors the already-landed Elf Elven Immunities idiom exactly (a
//! flat racial saving-throw-bonus magnitude applied to a save category
//! instead of a skill): the recognized value names only the flat
//! save-bonus magnitude, not a saving-throw-total engine. No
//! saving-throw-resolution engine exists anywhere in this codebase, so no
//! check resolution is fabricated from this record. Stonecunning and Greed
//! (already grounded) remain distinct and unchanged.
//!
//! Slice: cycle-2026-07-14T0400, matrix row_id: race.dwarf.bounded_semantics.

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

const HARDY_ID: &str = "race.dwarf.trait_bundle.hardy";

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

// ----- Hardy record exists on a Dwarf input, flat +2 magnitude -----

#[test]
fn dwarf_input_surfaces_hardy_trait_bundle_record() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let hardy = explanation(&computation, HARDY_ID);
    assert_eq!(
        hardy.value, 2,
        "Dwarf Hardy record must carry the grounded flat +2 saving-throw-bonus magnitude"
    );
    assert!(
        hardy.detail.contains("poison"),
        "Dwarf Hardy record must name the poison save category: {}",
        hardy.detail
    );
    assert!(
        hardy.detail.contains("spell"),
        "Dwarf Hardy record must name the spells/spell-like-abilities save category: {}",
        hardy.detail
    );
    assert!(
        hardy.detail.contains("+2"),
        "Dwarf Hardy record detail must name the +2 magnitude: {}",
        hardy.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_hardy_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.explanations.iter().any(|e| e.id == HARDY_ID),
        "Human input must not surface the Dwarf Hardy record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Greed and Stonecunning stay grounded alongside the new Hardy record -----

#[test]
fn dwarf_input_still_surfaces_greed_and_stonecunning_alongside_hardy() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

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

// ----- The bounded diagnostic now names Hardy as grounded, not unproven -----

#[test]
fn dwarf_bounded_semantics_note_moves_hardy_out_of_unproven_list() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.dwarf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.dwarf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in [
        "Defensive Training",
        "Stability",
        "Hatred",
        "weapon familiarity",
    ] {
        assert!(
            bounded.message.contains(token),
            "race.dwarf.bounded_semantics must still name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("Hardy"),
        "race.dwarf.bounded_semantics must name Hardy as grounded: {}",
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
    // Hardy is one more grounded family among several still-unproven ones
    // (Defensive Training, Stability, Hatred, weapon familiarity), so the
    // row does not reach Supported this cycle.
    assert_eq!(dwarf.support_state, SupportState::Supported);
    assert_eq!(dwarf.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        dwarf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        dwarf.grounding_ref.contains("sd18_dwarf_hardy"),
        "dwarf row grounding_ref must cite this slice's proof surface: {}",
        dwarf.grounding_ref
    );
    assert!(
        dwarf.blocker_or_lossiness_note.contains("Hardy"),
        "dwarf row note must name Hardy as grounded, not unproven: {}",
        dwarf.blocker_or_lossiness_note
    );
}
