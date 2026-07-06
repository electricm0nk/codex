//! SD13-E4-F7 Sorcerer level-1 spell-bearing baseline proof (post-F8 invariants).
//!
//! Proves the SD13-F7 spell-bearing invariants that SURVIVE the SD13-E4-F8 bloodline
//! + spontaneous spell-slot follow-up slice:
//!
//! - The live rules-core surface still ingests a deterministic Human
//!   `class:sorcerer:1` input and leaves direct computed evidence that recognizes the
//!   Sorcerer level-1 spell-bearing class identity (the F7 chassis recognition record
//!   `class_chassis.spell_baseline.sorcerer`), rather than treating it as an
//!   undocumented packet placeholder.
//! - It still fabricates no Fighter-shaped computed chassis (no base attack bonus,
//!   no `class_chassis.base_attack_bonus` explanation) — the spell-bearing path
//!   remains distinct from the Fighter chassis path the F7 baseline anchored.
//! - It still preserves the accepted Human race seam on the spell-bearing path
//!   (ability-bonus target, bonus-feat grant, bounded non-claim-blocking Human
//!   race note).
//! - The deterministic Human Sorcerer level-1 posture stays Blocked on the
//!   integrated headless receipt, because the bounded F8 slice names only the
//!   level-1 bloodline + spontaneous math and the level-2+ progression + broader
//!   spell-support surface as the remaining gap.
//! - Level-2 Sorcerer, Fighter, and Rogue stay negative controls: the level-1
//!   chassis recognition does not leak onto level-2 Sorcerer, the Fighter chassis
//!   does not surface a Sorcerer recognition, and the Rogue chassis stays a plain
//!   blocked negative control.
//! - The matrix keeps Bard and Wizard at `Unverified` / `Observed` and the
//!   accepted Paladin / Ranger hybrid rows at `Blocked` / `Computed`, while the
//!   Sorcerer row itself has been reclassified by the F8 follow-up slice from
//!   `Blocked` / `Computed` to `Partial` / `Computed`.
//!
//! This file does NOT re-assert the two pre-F8 burden diagnostics
//! (`class_feature.sorcerer.bloodline.unsupported` and
//! `class_spell.sorcerer.spontaneous.unsupported`) or the pre-F8 "no spell math"
//! invariant: those are now obsolete because the F8 slice computes the bounded
//! level-1 bloodline selection + Arcane Bond level-1 power and the spontaneous
//! spells known / slots per day / save DC math directly. The post-F8 invariants
//! for those surfaces live in `tests/sd13_sorcerer_bloodline_and_spontaneous_slice.rs`.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, PilotBaseChassisComputation, build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const SORCERER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.sorcerer";

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a codex::rules_core::pilot_compute::ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Direct runtime evidence: the spell-bearing identity is acknowledged -----

#[test]
fn sorcerer_level1_leaves_direct_spell_baseline_recognition_evidence() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Sorcerer spell-bearing identity is recognized
    // on the compute path, not silently dropped as an undocumented packet placeholder.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:sorcerer") && recognition.detail.contains("level 1"),
        "sorcerer recognition must name the class:sorcerer:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "sorcerer recognition must name the spell-bearing identity: {}",
        recognition.detail
    );

    // The chassis recognition itself carries no fabricated mechanical value (+0) and
    // the spell-bearing path does not fabricate a Fighter-style computed chassis.
    assert_eq!(
        recognition.value, 0,
        "sorcerer spell baseline recognition must carry no fabricated value (+0)"
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "spell baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "spell baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (CHA 17 -> +3).
    assert_eq!(computation.ability_modifiers.charisma, 3);
}

// ----- The bounded, level-1-only chassis recognition does not leak onto level 2 -----

#[test]
fn sorcerer_level_2_is_not_promoted_by_this_slice() {
    // The F7 chassis recognition is bounded to level 1; a level-2 Sorcerer must not
    // gain the level-1 spell-baseline recognition record and stays blocked.
    let level_2 = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:sorcerer:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, RECOGNITION_ID),
        "level-2 Sorcerer must not gain the bounded level-1 spell-baseline recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- The integrated posture remains Blocked on the bounded remaining gap -----

#[test]
fn sorcerer_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(SORCERER_FIXTURE);

    // The integrated posture is blocked, never a counterfeit computed success:
    // the F8 slice proves only the bounded level-1 bloodline + spontaneous math
    // and names the level-2+ progression + broader spell-support surface as the
    // remaining gap. That gap keeps the integrated posture Blocked.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked spell baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the spell-bearing path -----

#[test]
fn spell_baseline_preserves_human_race_seam() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "spell baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "spell baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "spell baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative controls: the spell baseline must not leak onto other classes -----

#[test]
fn fighter_and_rogue_do_not_gain_sorcerer_recognition() {
    // A supported Fighter must not gain a sorcerer spell-baseline recognition record
    // or any F7-surviving Sorcerer-specific diagnostic.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a sorcerer spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("sorcerer")),
        "Fighter must not surface sorcerer burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // A Rogue must stay a plain blocked negative control, never a sorcerer baseline.
    let rogue_fixture = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        rogue_computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Rogue chassis must remain claim-blocked"
    );
    assert!(
        !has_explanation(&rogue_computation, RECOGNITION_ID)
            && !rogue_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("sorcerer")),
        "Rogue must not surface any sorcerer recognition or burden diagnostics: {:?}",
        rogue_computation.diagnostics
    );
}

// ----- Control plane: the Sorcerer row has been reclassified to Partial/Computed by F8 -----

#[test]
fn matrix_sorcerer_row_is_partial_computed_after_f8_followup() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer row must exist");

    // The F8 follow-up slice lifts the Sorcerer row from Blocked/Computed to
    // Partial/Computed. This F7 file pins the reclassified truth; the
    // F8-specific invariants (bloodline + spontaneous explanations, the new
    // remaining-gap diagnostic) live in tests/sd13_sorcerer_bloodline_and_spontaneous_slice.rs.
    assert_eq!(
        sorcerer.support_state,
        SupportState::Partial,
        "sorcerer row must be Partial after the F8 follow-up slice"
    );
    assert_eq!(
        sorcerer.evidence_tier,
        EvidenceTier::Computed,
        "sorcerer row stays Computed on the evidence tier axis"
    );
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "sorcerer row stays refreshable-from-live-proof on the freshness axis"
    );
    // The blocker note must name the bounded level-2+ progression gap that the
    // F8 follow-up slice names as the only remaining gap.
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level"),
        "sorcerer Partial-row note must name the level-2+ progression gap: {note}"
    );
    assert!(
        note.contains("bloodline") && note.contains("spontaneous"),
        "sorcerer Partial-row note must name the bloodline + spontaneous surfaces: {note}"
    );
}

#[test]
fn matrix_keeps_bard_and_wizard_unverified_observed() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    for row_id in [
        "class.bard.progression_and_spell_burden",
        "class.wizard.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Unverified,
            "row {row_id} must stay Unverified after the F8 follow-up slice"
        );
        assert_eq!(
            row.evidence_tier,
            EvidenceTier::Observed,
            "row {row_id} must stay Observed after the F8 follow-up slice"
        );
    }
}

#[test]
fn matrix_preserves_paladin_and_ranger_hybrid_blocked_computed_truth() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    for row_id in [
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "hybrid row {row_id} must stay Blocked after the F8 follow-up slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }
}

#[test]
fn matrix_does_not_promote_any_row_to_supported_or_lossy() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "no slice must promote any row to Supported or Lossy"
    );
}