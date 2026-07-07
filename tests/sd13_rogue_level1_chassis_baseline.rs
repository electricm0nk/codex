//! SD13-E3 Rogue level-1 chassis baseline proof.
//!
//! Proves the SD13-E3 rogue slice (mirroring the Barbarian/Monk level-1
//! martial-baseline pattern): the live rules-core surface ingests a
//! deterministic Human `class:rogue:1` input, leaves direct computed evidence
//! that acknowledges the bounded level-1 chassis identity rather than treating
//! it as an undocumented packet placeholder, and yet stays explicitly
//! claim-blocked on the four still-missing burdens (base-attack progression,
//! base-save progression, sneak attack, trapfinding). It also pins the matrix
//! reclassification of the rogue row from `Blocked` / `Computed` to `Partial`
//! / `Computed`.
//!
//! It is intentionally not a Rogue class engine. It grounds no Fighter-shaped
//! `level_1_pilot` base-attack/base-save chassis (Rogue has 3/4 BAB and good
//! Reflex but the slice does not implement it), no sneak attack damage-die
//! execution, no trapfinding Perception/Disable Device bonus, no rogue talent,
//! and no level-2+ progression. It also preserves the accepted Fighter 1-3
//! truth, the Barbarian/Monk partial/computed truth, the Paladin/Ranger
//! blocked hybrid negative controls, and the Human race/interaction truth.
//! `tests/ge06_pilot_total_saves.rs::unsupported_chassis_blocks_total_saves`
//! keeps passing unmodified: this slice adds recognition and named burden
//! diagnostics only, and never computes `defense.total_save.*` for Rogue.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, HeadlessReceiptStatus,
    PilotBaseChassisComputation, build_pilot_headless_receipt, compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const ROGUE_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level1_sd13_deterministic_input.txt");
const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

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
) -> &'a ComputationExplanation {
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

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Direct runtime evidence: the chassis identity is acknowledged -----

#[test]
fn rogue_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let chassis = explanation(&computation, "class_chassis.rogue.bounded_progression");
    assert!(
        chassis.detail.contains("class:rogue") && chassis.detail.contains("level 1"),
        "rogue chassis recognition must name the class:rogue:1 identity: {}",
        chassis.detail
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "rogue baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "rogue baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (DEX 17 -> +3).
    assert_eq!(computation.ability_modifiers.dexterity, 3);
}

// ----- Still blocked: honest, class-specific burden diagnostics -----

#[test]
fn rogue_level1_stays_blocked_naming_four_burdens() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = claim_blocking(
        &computation,
        "class_feature.rogue.bounded_progression.base_attack.unsupported",
    );
    assert!(
        base_attack.message.contains("base attack"),
        "rogue base-attack blocker must name the 'base attack' burden: {}",
        base_attack.message
    );

    let base_save = claim_blocking(
        &computation,
        "class_feature.rogue.bounded_progression.base_save.unsupported",
    );
    assert!(
        base_save.message.contains("base save"),
        "rogue base-save blocker must name the 'base save' burden: {}",
        base_save.message
    );

    let sneak_attack = claim_blocking(
        &computation,
        "class_feature.rogue.bounded_progression.sneak_attack.unsupported",
    );
    assert!(
        sneak_attack.message.contains("sneak attack"),
        "rogue sneak-attack blocker must name the 'sneak attack' burden: {}",
        sneak_attack.message
    );

    let trapfinding = claim_blocking(
        &computation,
        "class_feature.rogue.bounded_progression.trapfinding.unsupported",
    );
    assert!(
        trapfinding.message.contains("trapfinding"),
        "rogue trapfinding blocker must name the 'trapfinding' burden: {}",
        trapfinding.message
    );

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked rogue baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the rogue path -----

#[test]
fn rogue_baseline_preserves_human_race_seam() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "rogue baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "rogue baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "rogue baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, "class_chassis.rogue.bounded_progression"),
        "the Fighter chassis must not surface a rogue-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.rogue.")),
        "the Fighter chassis must not surface rogue burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );
}

#[test]
fn rogue_level_2_is_not_promoted_by_this_slice() {
    let level_2 = ROGUE_FIXTURE.replace("class:rogue:1", "class:rogue:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.rogue.bounded_progression"),
        "level-2 Rogue must not gain the bounded level-1 chassis recognition record"
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.rogue.")),
        "level-2 Rogue must not surface the level-1 rogue burden diagnostics: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Rogue must stay claim-blocked in this slice"
    );
}

#[test]
fn multiclass_rogue_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_FIXTURE.replace(
        "class_level=class:rogue:1",
        "class_level=class:rogue:1\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.rogue.bounded_progression"),
        "multiclass Rogue must not gain the bounded level-1 single-class chassis recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- The pre-existing GE-06 negative control keeps passing unmodified -----

#[test]
fn rogue_still_produces_a_claim_blocking_diagnostic_and_no_total_saves() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "rogue chassis must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("defense.total_save.")),
        "rogue chassis must withhold total-save explanations: {:?}",
        computation.explanations
    );
}

// ----- Control plane: the matrix reclassifies the rogue row to Partial/Computed -----

#[test]
fn matrix_rogue_row_is_partial_computed_and_names_four_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Partial);
    assert_eq!(rogue.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        rogue.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        rogue
            .grounding_ref
            .contains("sd13_rogue_level1_chassis_baseline"),
        "rogue row must cite the SD13-E3 rogue proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "rogue partial row must carry a note");
    for token in ["base attack", "base save", "sneak attack", "trapfinding"] {
        assert!(
            note.contains(token),
            "rogue partial note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    for id in ["class.fighter.level_1_pilot", "class.fighter.levels_2_10"] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {id} must stay Partial after the rogue slice"
        );
    }

    for id in ["class.barbarian.bounded_progression", "class.monk.bounded_progression"] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {id} must keep its accepted Partial posture after the rogue slice"
        );
    }

    for id in [
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "row {id} must stay Blocked after the rogue slice"
        );
    }

    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the rogue slice must not promote any row to Supported or Lossy"
    );
}
