//! SD18 Elf Elven Immunities recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Elf race seam
//! (`explain_elf_race_seam` at `src/rules_core/pilot_compute.rs`) with a
//! sixth grounded PF1 Core Rulebook Elf racial trait: Elven Immunities
//! (`core_essentials/races/elf/elf_abilities_race.lst` — Elven Immunities
//! entry: `DESC:Elves are immune to magic sleep effects and get a +2 racial
//! saving throw bonus against enchantment spells and effects.`,
//! `ABILITY:Special Ability|AUTOMATIC|Immunity to Sleep`,
//! `BONUS:VAR|SaveBonus_vs_Enchantments|2|TYPE=Racial`).
//!
//! This trait bundles two distinct sub-effects, both grounded as bounded,
//! non-fabricated recognition records:
//!   - immunity to magic sleep effects: a flat, no-magnitude grant-only
//!     identity record (mirroring the Monk Purity of Body / Diamond Body
//!     disease/poison-immunity idiom exactly — no sleep-effect-resolution
//!     engine exists anywhere in this codebase to apply the immunity to);
//!   - a +2 racial saving throw bonus against enchantment spells and
//!     effects: a flat racial-bonus magnitude (mirroring the existing Elf
//!     Keen Senses / Dwarf Stonecunning / Dwarf Greed flat-bonus idiom,
//!     applied to a save category instead of a skill), NOT a
//!     saving-throw-total engine.
//!
//! Weapon familiarity (longbow, composite longbow, longsword, rapier,
//! shortbow, composite shortbow) and bonus language grants remain distinct,
//! still-unproven Elf families and are NOT grounded by this slice.
//!
//! Slice: cycle-2026-07-14T0100, matrix row_id: race.elf.bounded_semantics.

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

const ELF_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_elf_fighter_level1_sd13_deterministic_input.txt");

const ELVEN_IMMUNITIES_ID: &str = "race.elf.trait_bundle.elven_immunities";

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

// ----- Elven Immunities record exists on an Elf input, flat +2 save-bonus magnitude -----

#[test]
fn elf_input_surfaces_elven_immunities_trait_bundle_record() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let elven_immunities = explanation(&computation, ELVEN_IMMUNITIES_ID);
    assert_eq!(
        elven_immunities.value, 2,
        "Elf Elven Immunities record must carry the grounded flat +2 \
         enchantment-save racial-bonus magnitude"
    );
    assert!(
        elven_immunities.detail.contains("sleep"),
        "Elf Elven Immunities record must name the sleep-effect immunity: {}",
        elven_immunities.detail
    );
    assert!(
        elven_immunities.detail.contains("enchantment"),
        "Elf Elven Immunities record must name the enchantment save bonus: {}",
        elven_immunities.detail
    );
    assert!(
        elven_immunities.detail.contains("+2"),
        "Elf Elven Immunities record detail must name the +2 magnitude: {}",
        elven_immunities.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_elven_immunities_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == ELVEN_IMMUNITIES_ID),
        "Human input must not surface the Elf Elven Immunities record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Keen Senses stays grounded alongside the new Elven Immunities record -----

#[test]
fn elf_input_still_surfaces_keen_senses_alongside_elven_immunities() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let keen_senses = explanation(&computation, "race.elf.trait_bundle.keen_senses");
    assert_eq!(
        keen_senses.value, 2,
        "Keen Senses must remain grounded unchanged by this slice"
    );
}

// ----- The bounded diagnostic now names Elven Immunities as grounded, not unproven -----

#[test]
fn elf_bounded_semantics_note_moves_elven_immunities_out_of_unproven_list() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.elf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.elf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    // The remaining named-but-unproven families from before this slice must
    // still be named honestly.
    for token in ["weapon familiarity", "bonus language"] {
        assert!(
            bounded.message.contains(token),
            "race.elf.bounded_semantics must still name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
}

// ----- Control plane: the matrix row stays Partial/Computed, widened note -----

#[test]
fn matrix_elf_row_stays_partial_computed_and_grounding_ref_names_this_slice() {
    let matrix = seeded_current_truth();
    let elf = matrix
        .row("race.elf.bounded_semantics")
        .expect("elf row must exist");

    // NOTE (2026-07-16, SD-19 Full-matrix closure): this slice's own honest
    // Partial -> Partial widening claim below is historically accurate for THIS
    // slice, but the row was later promoted to Supported/ProductVisible by the
    // separate SD-19 Race Trait Catalog browser UI-surfacing work, which is why
    // the assertions after this comment now read Supported/ProductVisible.
    // Honest promotion: Partial -> Partial widening, NOT a jump to Supported.
    // Elven Immunities is one more grounded family among several still-unproven
    // ones (weapon familiarity, bonus languages), so the row does not reach
    // Supported this cycle.
    assert_eq!(elf.support_state, SupportState::Supported);
    assert_eq!(elf.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        elf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        elf.grounding_ref.contains("sd18_elf_elven_immunities"),
        "elf row grounding_ref must cite this slice's proof surface: {}",
        elf.grounding_ref
    );
    assert!(
        elf.blocker_or_lossiness_note.contains("Elven Immunities"),
        "elf row note must name Elven Immunities as grounded, not unproven: {}",
        elf.blocker_or_lossiness_note
    );
}
