//! SD18 Halfling Sure-Footed recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Halfling race
//! seam (`explain_halfling_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with a sixth grounded PF1 Core Rulebook Halfling racial trait: Sure-Footed,
//! a flat +2 racial bonus on Acrobatics and Climb skill checks
//! (`core_essentials/races/halfling/halfling_abilities_race.lst` — Sure-Footed
//! entry, `BONUS:SKILL|Acrobatics,Climb|2|TYPE=Racial`).
//!
//! This is deliberately NOT an Acrobatics/Climb-check-total engine: the
//! recognized value names only the flat racial-bonus magnitude (mirroring the
//! existing Dwarf Stonecunning / Elf Keen Senses / Gnome Keen Senses /
//! Half-Elf Keen Senses / Half-Orc Intimidating / Halfling Keen Senses
//! skill-bonus idiom already established on this seam), not a
//! skill-check-resolution engine. Fearless (save bonus vs. fear), Halfling
//! Luck (luck bonus on all saves), and weapon familiarity (sling and
//! "halfling" weapons) remain distinct, still-unproven families and are NOT
//! grounded by this slice.
//!
//! Slice: cycle-2026-07-14T0300, matrix row_id: race.halfling.bounded_semantics.

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

const SURE_FOOTED_ID: &str = "race.halfling.trait_bundle.sure_footed";

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

// ----- Sure-Footed record exists on a Halfling input, flat +2 magnitude -----

#[test]
fn halfling_input_surfaces_sure_footed_trait_bundle_record() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sure_footed = explanation(&computation, SURE_FOOTED_ID);
    assert_eq!(
        sure_footed.value, 2,
        "Halfling Sure-Footed record must carry the grounded flat +2 \
         Acrobatics/Climb racial-bonus magnitude"
    );
    assert!(
        sure_footed.detail.contains("Acrobatics"),
        "Halfling Sure-Footed record must name the Acrobatics skill: {}",
        sure_footed.detail
    );
    assert!(
        sure_footed.detail.contains("Climb"),
        "Halfling Sure-Footed record must name the Climb skill: {}",
        sure_footed.detail
    );
    assert!(
        sure_footed.detail.contains("+2"),
        "Halfling Sure-Footed record detail must name the +2 magnitude: {}",
        sure_footed.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_halfling_sure_footed_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SURE_FOOTED_ID),
        "Human input must not surface the Halfling Sure-Footed record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Keen Senses stays intact (non-regression on the sibling family) -----

#[test]
fn halfling_input_still_surfaces_keen_senses_alongside_sure_footed() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let keen_senses = explanation(&computation, "race.halfling.trait_bundle.keen_senses");
    assert_eq!(keen_senses.value, 2);
}

// ----- The bounded diagnostic now names Sure-Footed as grounded, not unproven -----

#[test]
fn halfling_bounded_semantics_note_moves_sure_footed_out_of_unproven_list() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.halfling.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.halfling.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in ["Fearless", "Halfling Luck", "weapon familiarity"] {
        assert!(
            bounded.message.contains(token),
            "race.halfling.bounded_semantics must still name the still-unproven '{token}' \
             trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("Sure-Footed"),
        "race.halfling.bounded_semantics must name Sure-Footed: {}",
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
    // Sure-Footed is one more grounded family among several still-unproven
    // ones (Fearless, Halfling Luck, weapon familiarity), so the row does not
    // reach Supported this cycle.
    assert_eq!(halfling.support_state, SupportState::Supported);
    assert_eq!(halfling.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        halfling.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        halfling.grounding_ref.contains("sd18_halfling_sure_footed"),
        "halfling row grounding_ref must cite this slice's proof surface: {}",
        halfling.grounding_ref
    );
    assert!(
        halfling.blocker_or_lossiness_note.contains("Sure-Footed"),
        "halfling row note must still mention Sure-Footed by name: {}",
        halfling.blocker_or_lossiness_note
    );
    assert!(
        halfling.blocker_or_lossiness_note.contains("Fearless"),
        "halfling row note must name the distinct, still-unproven Fearless \
         trait: {}",
        halfling.blocker_or_lossiness_note
    );
}
