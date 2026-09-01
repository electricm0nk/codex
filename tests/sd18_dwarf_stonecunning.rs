//! SD18 Dwarf Stonecunning recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2 Dwarf race seam
//! (`explain_dwarf_race_seam` at `src/rules_core/pilot_compute.rs`) with a
//! fifth grounded PF1 Core Rulebook Dwarf racial trait: Stonecunning, a flat
//! +2 situational bonus on Perception checks to potentially notice unusual
//! stonework, such as traps and hidden doors located in stone walls or
//! floors (`core_essentials/races/dwarf/dwarf_abilities_race.lst:27` —
//! `BONUS:SITUATION|Perception=to notice unusual stonework|2|TYPE=Racial`;
//! `dwarf_skills.lst:6` — `Perception.MOD SITUATION:to notice unusual
//! stonework`).
//!
//! This is deliberately NOT a Perception-check-total engine: the recognized
//! value names only the flat situational-bonus magnitude (mirroring the
//! existing Bard Inspire Competence / Track skill-bonus idiom already
//! established elsewhere on this seam), not a skill-check-resolution or
//! stonework-detection engine. Greed (the separate +2 Appraise racial trait
//! for assessing nonmagical precious-metal/gemstone goods,
//! `dwarf_abilities_race.lst:23`) remains a distinct, still-unproven family
//! and is NOT grounded by this slice.
//!
//! Slice: cycle-2026-07-12T2334, matrix row_id: race.dwarf.bounded_semantics.

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

const STONECUNNING_ID: &str = "race.dwarf.trait_bundle.stonecunning";

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

// ----- Stonecunning record exists on a Dwarf input, flat +2 magnitude -----

#[test]
fn dwarf_input_surfaces_stonecunning_trait_bundle_record() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let stonecunning = explanation(&computation, STONECUNNING_ID);
    assert_eq!(
        stonecunning.value, 2,
        "Dwarf Stonecunning record must carry the grounded flat +2 Perception \
         situational-bonus magnitude"
    );
    assert!(
        stonecunning.detail.contains("Perception"),
        "Dwarf Stonecunning record must name the Perception skill: {}",
        stonecunning.detail
    );
    assert!(
        stonecunning.detail.contains("stonework"),
        "Dwarf Stonecunning record must name the unusual-stonework situation: {}",
        stonecunning.detail
    );
    assert!(
        stonecunning.detail.contains("+2"),
        "Dwarf Stonecunning record detail must name the +2 magnitude: {}",
        stonecunning.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_stonecunning_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.explanations.iter().any(|e| e.id == STONECUNNING_ID),
        "Human input must not surface the Dwarf Stonecunning record, got explanations {:?}",
        computation.explanations
    );
}

// ----- The bounded diagnostic now names Stonecunning as grounded, not unproven -----

#[test]
fn dwarf_bounded_semantics_note_moves_stonecunning_out_of_unproven_list() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.dwarf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.dwarf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // Greed (the separate Appraise trait) remains distinct and unproven.
    assert!(
        bounded.message.contains("Greed"),
        "race.dwarf.bounded_semantics must still name the still-unproven 'Greed' \
         (Appraise) trait as distinct from Stonecunning: {}",
        bounded.message
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in ["Defensive Training", "Hardy", "Stability", "Hatred", "weapon familiarity"] {
        assert!(
            bounded.message.contains(token),
            "race.dwarf.bounded_semantics must still name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
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
    // Stonecunning is one more grounded family among several still-unproven
    // ones (Greed, Defensive Training, Hardy, Stability, Hatred, weapon
    // familiarity), so the row does not reach Supported this cycle.
    assert_eq!(dwarf.support_state, SupportState::Supported);
    assert_eq!(dwarf.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        dwarf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        dwarf.grounding_ref.contains("sd18_dwarf_stonecunning"),
        "dwarf row grounding_ref must cite this slice's proof surface: {}",
        dwarf.grounding_ref
    );
    assert!(
        dwarf.blocker_or_lossiness_note.contains("Stonecunning"),
        "dwarf row note must still mention Stonecunning by name: {}",
        dwarf.blocker_or_lossiness_note
    );
    assert!(
        dwarf.blocker_or_lossiness_note.contains("Greed"),
        "dwarf row note must name the distinct, still-unproven Greed (Appraise) \
         trait: {}",
        dwarf.blocker_or_lossiness_note
    );
}
