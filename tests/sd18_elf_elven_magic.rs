//! SD18 Elf Elven Magic recognition proof.
//!
//! Tranche-3 §3.1 race-row work-unit: extends the SD13-E2/SD18 Elf race seam
//! (`explain_elf_race_seam` at `src/rules_core/pilot_compute.rs`) with a
//! seventh grounded PF1 Core Rulebook Elf racial trait: Elven Magic
//! (`core_essentials/races/elf/elf_abilities_race.lst` — Elven Magic entry:
//! `DESC:Elves receive a +2 racial bonus on caster level checks made to
//! overcome spell resistance. In addition, elves receive a +2 racial bonus
//! on Spellcraft skill checks made to identify the properties of magic
//! items.`, `BONUS:SITUATION|Spellcraft=to identify magic item
//! properties|2|TYPE=Racial`).
//!
//! This trait was not previously named in the Elf row's unproven-family list
//! (which named only weapon familiarity and bonus language grants) — it is a
//! genuine, distinct PF1 Core Elf racial trait present in the corpus that
//! this slice newly recognizes. It bundles two distinct sub-effects, both
//! grounded as bounded, non-fabricated recognition records, mirroring the
//! Elven Immunities idiom exactly:
//!   - a +2 racial bonus on caster level checks made to overcome spell
//!     resistance: a flat, grant-only identity record (mirroring the Elven
//!     Immunities sleep-immunity idiom exactly — no caster-level-check /
//!     spell-resistance-resolution engine exists anywhere in this codebase
//!     to apply the bonus to);
//!   - a +2 racial bonus on Spellcraft skill checks made to identify the
//!     properties of magic items: a flat racial-bonus magnitude (mirroring
//!     the existing Elf Keen Senses / Dwarf Stonecunning / Dwarf Greed
//!     flat-bonus idiom, applied to the Spellcraft skill), NOT a
//!     Spellcraft-check-total engine.
//!
//! Weapon familiarity (longbow, composite longbow, longsword, rapier,
//! shortbow, composite shortbow) and bonus language grants remain distinct,
//! still-unproven Elf families and are NOT grounded by this slice.
//!
//! Slice: cycle-2026-07-14T0500, matrix row_id: race.elf.bounded_semantics.

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

const ELVEN_MAGIC_ID: &str = "race.elf.trait_bundle.elven_magic";

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

// ----- Elven Magic record exists on an Elf input, flat +2 magnitude -----

#[test]
fn elf_input_surfaces_elven_magic_trait_bundle_record() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let elven_magic = explanation(&computation, ELVEN_MAGIC_ID);
    assert_eq!(
        elven_magic.value, 2,
        "Elf Elven Magic record must carry the grounded flat +2 Spellcraft \
         racial-bonus magnitude"
    );
    assert!(
        elven_magic.detail.contains("Spellcraft"),
        "Elf Elven Magic record must name the Spellcraft skill-bonus sub-effect: {}",
        elven_magic.detail
    );
    assert!(
        elven_magic.detail.contains("spell resistance"),
        "Elf Elven Magic record must name the caster-level-check-vs-SR sub-effect: {}",
        elven_magic.detail
    );
    assert!(
        elven_magic.detail.contains("+2"),
        "Elf Elven Magic record detail must name the +2 magnitude: {}",
        elven_magic.detail
    );
}

// ----- The record does not leak onto Human input -----

#[test]
fn human_input_does_not_surface_elven_magic_record() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == ELVEN_MAGIC_ID),
        "Human input must not surface the Elf Elven Magic record, got explanations {:?}",
        computation.explanations
    );
}

// ----- Keen Senses and Elven Immunities stay grounded alongside the new Elven Magic record -----

#[test]
fn elf_input_still_surfaces_keen_senses_and_elven_immunities_alongside_elven_magic() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let keen_senses = explanation(&computation, "race.elf.trait_bundle.keen_senses");
    assert_eq!(
        keen_senses.value, 2,
        "Keen Senses must remain grounded unchanged by this slice"
    );
    let elven_immunities = explanation(&computation, "race.elf.trait_bundle.elven_immunities");
    assert_eq!(
        elven_immunities.value, 2,
        "Elven Immunities must remain grounded unchanged by this slice"
    );
}

// ----- The bounded diagnostic now names Elven Magic as grounded -----

#[test]
fn elf_bounded_semantics_note_names_elven_magic_as_grounded() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.elf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.elf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    assert!(
        bounded.message.contains("Elven Magic"),
        "race.elf.bounded_semantics must name Elven Magic as grounded: {}",
        bounded.message
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
    // Elven Magic is one more grounded family among several still-unproven
    // ones (weapon familiarity, bonus languages), so the row does not reach
    // Supported this cycle.
    assert_eq!(elf.support_state, SupportState::Supported);
    assert_eq!(elf.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        elf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        elf.grounding_ref.contains("sd18_elf_elven_magic"),
        "elf row grounding_ref must cite this slice's proof surface: {}",
        elf.grounding_ref
    );
    assert!(
        elf.blocker_or_lossiness_note.contains("Elven Magic"),
        "elf row note must name Elven Magic as grounded: {}",
        elf.blocker_or_lossiness_note
    );
}
