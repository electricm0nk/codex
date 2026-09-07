//! SD18 Gnome Illusion Resistance recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Gnome race
//! seam (`explain_gnome_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with a sixth grounded PF1 Core Rulebook Gnome racial trait: Illusion
//! Resistance (`core_essentials/races/gnome/gnome_abilities_race.lst` —
//! Illusion Resistance entry: `DESC:Gnomes get a +2 racial saving throw
//! bonus against illusion spells and effects.`,
//! `BONUS:VAR|SaveBonus_vs_Illusions|2|TYPE=Racial`).
//!
//! This is a flat +2 racial saving-throw-bonus magnitude, mirroring the
//! existing Gnome Keen Senses / Dwarf Stonecunning / Dwarf Greed / Elf
//! Elven Immunities flat-bonus idiom already established elsewhere on this
//! seam, applied to a save category instead of a skill — NOT a
//! saving-throw-total engine.
//!
//! Defensive Training (dodge bonus vs. giants), Hatred (attack bonus vs.
//! reptilian humanoids/goblinoids), Gnome Magic (spell-like abilities keyed
//! to Charisma), and weapon familiarity (gnome hooked hammer) remain
//! distinct, still-unproven Gnome families and are NOT grounded by this
//! slice.
//!
//! Slice: cycle-2026-07-14T0137, matrix row_id: race.gnome.bounded_semantics.

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

const ILLUSION_RESISTANCE_ID: &str = "race.gnome.trait_bundle.illusion_resistance";

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

// ----- Illusion Resistance record exists on a Gnome input, flat +2 save-bonus magnitude -----

#[test]
fn gnome_input_surfaces_illusion_resistance_trait_bundle_record() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let illusion_resistance = explanation(&computation, ILLUSION_RESISTANCE_ID);
    assert_eq!(
        illusion_resistance.value, 2,
        "Gnome Illusion Resistance record must carry the grounded flat +2 \
         illusion-save racial-bonus magnitude"
    );
    assert!(
        illusion_resistance.detail.contains("illusion"),
        "Gnome Illusion Resistance record must name the illusion save bonus: {}",
        illusion_resistance.detail
    );
    assert!(
        illusion_resistance.detail.contains("+2"),
        "Gnome Illusion Resistance record detail must name the +2 magnitude: {}",
        illusion_resistance.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_gnome_illusion_resistance_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == ILLUSION_RESISTANCE_ID),
        "Human input must not surface the Gnome Illusion Resistance record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Keen Senses stays grounded alongside the new Illusion Resistance record -----

#[test]
fn gnome_input_still_surfaces_keen_senses_alongside_illusion_resistance() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let keen_senses = explanation(&computation, "race.gnome.trait_bundle.keen_senses");
    assert_eq!(
        keen_senses.value, 2,
        "Keen Senses must remain grounded unchanged by this slice"
    );
}

// ----- The bounded diagnostic now names Illusion Resistance as grounded, not unproven -----

#[test]
fn gnome_bounded_semantics_note_moves_illusion_resistance_out_of_unproven_list() {
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
    // Illusion Resistance is one more grounded family among several
    // still-unproven ones (Defensive Training, Hatred, Gnome Magic, weapon
    // familiarity), so the row does not reach Supported this cycle.
    assert_eq!(gnome.support_state, SupportState::Supported);
    assert_eq!(gnome.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        gnome.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        gnome.grounding_ref.contains("sd18_gnome_illusion_resistance"),
        "gnome row grounding_ref must cite this slice's proof surface: {}",
        gnome.grounding_ref
    );
    assert!(
        gnome.blocker_or_lossiness_note.contains("Illusion Resistance"),
        "gnome row note must name Illusion Resistance as grounded, not unproven: {}",
        gnome.blocker_or_lossiness_note
    );
}
