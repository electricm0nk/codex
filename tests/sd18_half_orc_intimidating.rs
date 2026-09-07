//! SD18 Half-Orc Intimidating recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2 Half-Orc race seam
//! (`explain_half_orc_race_seam` at `src/rules_core/pilot_compute.rs`) with a
//! fifth grounded PF1 Core Rulebook Half-Orc racial trait: Intimidating, a
//! flat +2 racial bonus on Intimidate checks
//! (`core_essentials/races/half_orc/halforc_abilities_race.lst` — Intimidating
//! entry, `BONUS:SKILL|Intimidate|2|TYPE=Racial`).
//!
//! This is deliberately NOT an Intimidate-check-total engine: the recognized
//! value names only the flat racial-bonus magnitude (mirroring the existing
//! Dwarf Stonecunning / Elf Keen Senses / Gnome Keen Senses / Half-Elf Keen
//! Senses skill-bonus idiom already established elsewhere on this seam), not
//! a skill-check-resolution engine. Orc Ferocity (fighting on for one more
//! round below 0 hit points) and weapon familiarity (orc double axe,
//! falchion, and treating any weapon with "orc" in its name as martial)
//! remain distinct, still-unproven families and are NOT grounded by this
//! slice.
//!
//! Slice: cycle-2026-07-13T1600, matrix row_id: race.half_orc.bounded_semantics.

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

const HALF_ORC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_deterministic_input.txt");

const INTIMIDATING_ID: &str = "race.half_orc.trait_bundle.intimidating";

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

// ----- Intimidating record exists on a Half-Orc input, flat +2 magnitude -----

#[test]
fn half_orc_input_surfaces_intimidating_trait_bundle_record() {
    let input = load(HALF_ORC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intimidating = explanation(&computation, INTIMIDATING_ID);
    assert_eq!(
        intimidating.value, 2,
        "Half-Orc Intimidating record must carry the grounded flat +2 Intimidate \
         racial-bonus magnitude"
    );
    assert!(
        intimidating.detail.contains("Intimidate"),
        "Half-Orc Intimidating record must name the Intimidate skill: {}",
        intimidating.detail
    );
    assert!(
        intimidating.detail.contains("+2"),
        "Half-Orc Intimidating record detail must name the +2 magnitude: {}",
        intimidating.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_half_orc_intimidating_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == INTIMIDATING_ID),
        "Human input must not surface the Half-Orc Intimidating record, got explanations {:?}",
        computation.explanations
    );
}

// ----- The bounded diagnostic now names Intimidating as grounded, not unproven -----

#[test]
fn half_orc_bounded_semantics_note_moves_intimidating_out_of_unproven_list() {
    let input = load(HALF_ORC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.half_orc.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.half_orc.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in ["Orc Ferocity", "weapon familiarity"] {
        assert!(
            bounded.message.contains(token),
            "race.half_orc.bounded_semantics must still name the still-unproven '{token}' \
             trait: {}",
            bounded.message
        );
    }
    assert!(
        !bounded.message.contains("Intimidating (a bonus on Intimidate checks)"),
        "race.half_orc.bounded_semantics must no longer list Intimidating among the \
         unproven families: {}",
        bounded.message
    );
}

// ----- Control plane: the matrix row stays Partial/Computed, widened note -----

#[test]
fn matrix_half_orc_row_stays_partial_computed_and_grounding_ref_names_this_slice() {
    let matrix = seeded_current_truth();
    let half_orc = matrix
        .row("race.half_orc.bounded_semantics")
        .expect("half_orc row must exist");

    // NOTE (2026-07-16, SD-19 Full-matrix closure): this slice's own honest
    // Partial -> Partial widening claim below is historically accurate for THIS
    // slice, but the row was later promoted to Supported/ProductVisible by the
    // separate SD-19 Race Trait Catalog browser UI-surfacing work, which is why
    // the assertions after this comment now read Supported/ProductVisible.
    // Honest promotion: Partial -> Partial widening, NOT a jump to Supported.
    // Intimidating is one more grounded family among several still-unproven
    // ones (Orc Ferocity, weapon familiarity), so the row does not reach
    // Supported this cycle.
    assert_eq!(half_orc.support_state, SupportState::Supported);
    assert_eq!(half_orc.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        half_orc.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        half_orc.grounding_ref.contains("sd18_half_orc_intimidating"),
        "half_orc row grounding_ref must cite this slice's proof surface: {}",
        half_orc.grounding_ref
    );
    assert!(
        half_orc.blocker_or_lossiness_note.contains("Intimidating"),
        "half_orc row note must still mention Intimidating by name: {}",
        half_orc.blocker_or_lossiness_note
    );
    assert!(
        half_orc.blocker_or_lossiness_note.contains("Orc Ferocity"),
        "half_orc row note must name the distinct, still-unproven Orc Ferocity \
         trait: {}",
        half_orc.blocker_or_lossiness_note
    );
}
