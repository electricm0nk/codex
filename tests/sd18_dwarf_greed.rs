//! SD18 Dwarf Greed recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Dwarf race
//! seam (`explain_dwarf_race_seam` at `src/rules_core/pilot_compute.rs`)
//! with a sixth grounded PF1 Core Rulebook Dwarf racial trait: Greed, a flat
//! +2 situational bonus on Appraise checks to determine the price of
//! nonmagical goods that contain precious metals or gemstones
//! (`core_essentials/races/dwarf/dwarf_abilities_race.lst:23` —
//! `BONUS:SITUATION|Appraise=to assess nonmagical metals or gemstones|2|TYPE=Racial`;
//! `dwarf_skills.lst:5` — `Appraise.MOD SITUATION:to assess nonmagical
//! metals or gemstones`).
//!
//! This is deliberately NOT an Appraise-check-total engine: the recognized
//! value names only the flat situational-bonus magnitude (mirroring the
//! already-landed Stonecunning idiom on this same seam), not a
//! skill-check-resolution or goods-valuation engine. Stonecunning (the
//! separate +2 Perception racial trait, already grounded by the
//! cycle-2026-07-12T2334 slice) remains distinct.
//!
//! Slice: cycle-2026-07-14T0007, matrix row_id: race.dwarf.bounded_semantics.

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

const GREED_ID: &str = "race.dwarf.trait_bundle.greed";

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

// ----- Greed record exists on a Dwarf input, flat +2 magnitude -----

#[test]
fn dwarf_input_surfaces_greed_trait_bundle_record() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let greed = explanation(&computation, GREED_ID);
    assert_eq!(
        greed.value, 2,
        "Dwarf Greed record must carry the grounded flat +2 Appraise \
         situational-bonus magnitude"
    );
    assert!(
        greed.detail.contains("Appraise"),
        "Dwarf Greed record must name the Appraise skill: {}",
        greed.detail
    );
    assert!(
        greed.detail.contains("precious metal") || greed.detail.contains("gemstone"),
        "Dwarf Greed record must name the precious-metal/gemstone situation: {}",
        greed.detail
    );
    assert!(
        greed.detail.contains("+2"),
        "Dwarf Greed record detail must name the +2 magnitude: {}",
        greed.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_greed_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.explanations.iter().any(|e| e.id == GREED_ID),
        "Human input must not surface the Dwarf Greed record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Stonecunning stays grounded alongside the new Greed record -----

#[test]
fn dwarf_input_still_surfaces_stonecunning_alongside_greed() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let stonecunning = explanation(&computation, "race.dwarf.trait_bundle.stonecunning");
    assert_eq!(
        stonecunning.value, 2,
        "Stonecunning must remain grounded unchanged by this slice"
    );
}

// ----- The bounded diagnostic now names Greed as grounded, not unproven -----

#[test]
fn dwarf_bounded_semantics_note_moves_greed_out_of_unproven_list() {
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
        "Hardy",
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
    // Greed is one more grounded family among several still-unproven ones
    // (Defensive Training, Hardy, Stability, Hatred, weapon familiarity), so
    // the row does not reach Supported this cycle.
    assert_eq!(dwarf.support_state, SupportState::Supported);
    assert_eq!(dwarf.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        dwarf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        dwarf.grounding_ref.contains("sd18_dwarf_greed"),
        "dwarf row grounding_ref must cite this slice's proof surface: {}",
        dwarf.grounding_ref
    );
    assert!(
        dwarf.blocker_or_lossiness_note.contains("Greed"),
        "dwarf row note must name Greed as grounded, not unproven: {}",
        dwarf.blocker_or_lossiness_note
    );
}
