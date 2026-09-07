//! SD18 Gnome Hatred recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Gnome race
//! seam (`explain_gnome_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with an eighth grounded PF1 Core Rulebook Gnome racial trait: Hatred, a
//! flat +1 racial bonus on attack rolls against humanoid creatures of the
//! reptilian and goblinoid subtypes
//! (`core_essentials/races/gnome/gnome_abilities_race.lst:24` —
//! `BONUS:VAR|Gnome_Hatred_AttackBonus|1`).
//!
//! This mirrors the already-landed Gnome Keen Senses / Illusion Resistance /
//! Defensive Training idiom exactly (a single flat bonus magnitude carried
//! as a bounded recognition record): the recognized value names only the
//! flat attack-bonus magnitude, not an attack-roll-total engine and not a
//! reptilian/goblinoid-subtype-detection engine (no "is the target a
//! reptilian humanoid or goblinoid" resolution is fabricated). No such
//! engine exists anywhere in this codebase, so no check resolution is
//! fabricated from this record. Ability modifiers, size, speed, senses,
//! Keen Senses, Illusion Resistance, and Defensive Training (already
//! grounded) remain distinct and unchanged.
//!
//! Slice: cycle-2026-07-14T1700, matrix row_id: race.gnome.bounded_semantics.

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

const HATRED_ID: &str = "race.gnome.trait_bundle.hatred";

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

// ----- Hatred record exists on a Gnome input, flat +1 magnitude -----

#[test]
fn gnome_input_surfaces_hatred_trait_bundle_record() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let hatred = explanation(&computation, HATRED_ID);
    assert_eq!(
        hatred.value, 1,
        "Gnome Hatred record must carry the grounded flat +1 attack-bonus magnitude"
    );
    assert!(
        hatred.detail.contains("reptilian"),
        "Gnome Hatred record must name the reptilian subtype: {}",
        hatred.detail
    );
    assert!(
        hatred.detail.contains("goblinoid"),
        "Gnome Hatred record must name the goblinoid subtype: {}",
        hatred.detail
    );
    assert!(
        hatred.detail.contains("+1"),
        "Gnome Hatred record detail must name the +1 magnitude: {}",
        hatred.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_hatred_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.explanations.iter().any(|e| e.id == HATRED_ID),
        "Human input must not surface the Gnome Hatred record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Keen Senses, Illusion Resistance, and Defensive Training stay grounded alongside Hatred -----

#[test]
fn gnome_input_still_surfaces_prior_families_alongside_hatred() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let keen_senses = explanation(&computation, "race.gnome.trait_bundle.keen_senses");
    assert_eq!(
        keen_senses.value, 2,
        "Keen Senses must remain grounded unchanged by this slice"
    );
    let illusion_resistance = explanation(
        &computation,
        "race.gnome.trait_bundle.illusion_resistance",
    );
    assert_eq!(
        illusion_resistance.value, 2,
        "Illusion Resistance must remain grounded unchanged by this slice"
    );
    let defensive_training = explanation(
        &computation,
        "race.gnome.trait_bundle.defensive_training",
    );
    assert_eq!(
        defensive_training.value, 4,
        "Defensive Training must remain grounded unchanged by this slice"
    );
}

// ----- The bounded diagnostic now names Hatred as grounded, not unproven -----

#[test]
fn gnome_bounded_semantics_note_moves_hatred_out_of_unproven_list() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.gnome.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.gnome.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in ["Gnome Magic", "weapon familiarity"] {
        assert!(
            bounded.message.contains(token),
            "race.gnome.bounded_semantics must still name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("Hatred"),
        "race.gnome.bounded_semantics must name Hatred as grounded: {}",
        bounded.message
    );
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
    // Hatred is one more grounded family among several still-unproven ones
    // (Gnome Magic, weapon familiarity), so the row does not reach Supported
    // this cycle.
    assert_eq!(gnome.support_state, SupportState::Supported);
    assert_eq!(gnome.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        gnome.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        gnome.grounding_ref.contains("sd18_gnome_hatred"),
        "gnome row grounding_ref must cite this slice's proof surface: {}",
        gnome.grounding_ref
    );
    assert!(
        gnome.blocker_or_lossiness_note.contains("Hatred"),
        "gnome row note must name Hatred as grounded, not unproven: {}",
        gnome.blocker_or_lossiness_note
    );
}
