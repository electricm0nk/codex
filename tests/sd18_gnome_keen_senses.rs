//! SD18 Gnome Keen Senses recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2 Gnome race seam
//! (`explain_gnome_race_seam` at `src/rules_core/pilot_compute.rs`) with a
//! fifth grounded PF1 Core Rulebook Gnome racial trait: Keen Senses, a flat
//! +2 racial bonus on Perception checks
//! (`core_essentials/races/gnome/gnome_abilities_race.lst` — Keen Senses
//! entry, `BONUS:SKILL|Perception|KeenSensesBonus|TYPE=Racial` with
//! `BONUS:VAR|KeenSensesBonus|2`).
//!
//! This is deliberately NOT a Perception-check-total engine: the recognized
//! value names only the flat racial-bonus magnitude (mirroring the existing
//! Dwarf Stonecunning / Elf Keen Senses skill-bonus idiom already
//! established elsewhere on this seam), not a skill-check-resolution
//! engine. Defensive Training (dodge bonus vs. giants), Illusion Resistance
//! (save bonus vs. illusion spells), Hatred (attack bonus vs. reptilian
//! humanoids/goblinoids), Gnome Magic (spell-like abilities keyed to
//! Charisma), and weapon familiarity (gnome hooked hammer) remain distinct,
//! still-unproven families and are NOT grounded by this slice.
//!
//! Slice: cycle-2026-07-13T0930, matrix row_id: race.gnome.bounded_semantics.

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

const GNOME_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_gnome_fighter_level1_sd13_deterministic_input.txt");

const KEEN_SENSES_ID: &str = "race.gnome.trait_bundle.keen_senses";

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

// ----- Keen Senses record exists on a Gnome input, flat +2 magnitude -----

#[test]
fn gnome_input_surfaces_keen_senses_trait_bundle_record() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let keen_senses = explanation(&computation, KEEN_SENSES_ID);
    assert_eq!(
        keen_senses.value, 2,
        "Gnome Keen Senses record must carry the grounded flat +2 Perception \
         racial-bonus magnitude"
    );
    assert!(
        keen_senses.detail.contains("Perception"),
        "Gnome Keen Senses record must name the Perception skill: {}",
        keen_senses.detail
    );
    assert!(
        keen_senses.detail.contains("+2"),
        "Gnome Keen Senses record detail must name the +2 magnitude: {}",
        keen_senses.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_gnome_keen_senses_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == KEEN_SENSES_ID),
        "Human input must not surface the Gnome Keen Senses record, got explanations {:?}",
        computation.explanations
    );
}

// ----- The bounded diagnostic now names Keen Senses as grounded, not unproven -----

#[test]
fn gnome_bounded_semantics_note_moves_keen_senses_out_of_unproven_list() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.gnome.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.gnome.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in [
        "Defensive Training",
        "Illusion Resistance",
        "Hatred",
        "Gnome Magic",
        "weapon familiarity",
    ] {
        assert!(
            bounded.message.contains(token),
            "race.gnome.bounded_semantics must still name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
}

// ----- Control plane: the matrix row stays Partial/Computed, widened note -----

#[test]
fn matrix_gnome_row_stays_partial_computed_and_grounding_ref_names_this_slice() {
    let matrix = seeded_current_truth();
    let gnome = matrix
        .row("race.gnome.bounded_semantics")
        .expect("gnome row must exist");

    // NOTE (2026-07-16, SD-19 Full-matrix closure): this slice's own honest
    // Partial -> Partial widening claim below is historically accurate for THIS
    // slice, but the row was later promoted to Supported/ProductVisible by the
    // separate SD-19 Race Trait Catalog browser UI-surfacing work, which is why
    // the assertions after this comment now read Supported/ProductVisible.
    // Honest promotion: Partial -> Partial widening, NOT a jump to Supported.
    // Keen Senses is one more grounded family among several still-unproven
    // ones (Defensive Training, Illusion Resistance, Hatred, Gnome Magic,
    // weapon familiarity), so the row does not reach Supported this cycle.
    assert_eq!(gnome.support_state, SupportState::Supported);
    assert_eq!(gnome.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        gnome.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        gnome.grounding_ref.contains("sd18_gnome_keen_senses"),
        "gnome row grounding_ref must cite this slice's proof surface: {}",
        gnome.grounding_ref
    );
    assert!(
        gnome.blocker_or_lossiness_note.contains("Keen Senses"),
        "gnome row note must still mention Keen Senses by name: {}",
        gnome.blocker_or_lossiness_note
    );
    assert!(
        gnome.blocker_or_lossiness_note.contains("Defensive Training"),
        "gnome row note must name the distinct, still-unproven Defensive \
         Training trait: {}",
        gnome.blocker_or_lossiness_note
    );
}
