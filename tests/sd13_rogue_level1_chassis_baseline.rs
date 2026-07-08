//! SD13-E3 Rogue level-1 chassis baseline proof.
//!
//! Proves the SD13-E3 rogue slice (mirroring the Barbarian/Monk level-1
//! martial-baseline pattern): the live rules-core surface ingests a
//! deterministic Human `class:rogue:1` input, leaves direct computed evidence
//! that acknowledges the bounded level-1 chassis identity rather than treating
//! it as an undocumented packet placeholder. The SD13-E3 pillar-grounding
//! slice now grounds three of the four named burdens directly: base-attack
//! progression (3/4 BAB), base-save progression (good Reflex, poor Fortitude,
//! poor Will), and sneak attack (die-count only, `+1d6` at level 1). Only
//! trapfinding remains claim-blocked. It also pins the matrix reclassification
//! of the rogue row, which stays `Partial` / `Computed` with a narrowed
//! blocker note naming only trapfinding.
//!
//! It is intentionally not a Rogue class engine. The new base-attack/base-save
//! explanations are standalone `class_chassis.rogue.*` records, not wired into
//! `compute_fighter_chassis`, `compute_total_saves`, or
//! `compute_combat_baseline` — `defense.total_save.*` is still never computed
//! for Rogue. The sneak-attack explanation grounds only the die-count facet
//! (`1`), not damage-roll execution or the flanking/Dexterity-denial
//! trigger-condition engine. This slice grounds no trapfinding
//! Perception/Disable Device bonus, no rogue talent, and no level-2+
//! progression. It also preserves the accepted Fighter 1-3 truth, the
//! Barbarian/Monk partial/computed truth, the Paladin/Ranger blocked hybrid
//! negative controls, and the Human race/interaction truth.
//! `tests/ge06_pilot_total_saves.rs::unsupported_chassis_blocks_total_saves`
//! keeps passing unmodified: this slice adds grounded pillar explanations
//! only, and never computes `defense.total_save.*` for Rogue.

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

// ----- Now grounded: base-attack, base-save, and sneak-attack pillars -----

#[test]
fn rogue_level1_base_attack_bonus_is_grounded_and_no_longer_blocked() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 0,
        "Rogue level 1 3/4-BAB progression (1 * 3 / 4) must equal 0: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "rogue base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.id
            == "class_feature.rogue.bounded_progression.base_attack.unsupported"),
        "the base-attack unsupported diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn rogue_level1_base_saves_are_grounded_and_no_longer_blocked() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 0, "Rogue level 1 poor Fortitude (1/3) must equal 0");
    assert!(
        fortitude.detail.to_lowercase().contains("poor"),
        "rogue Fortitude explanation must name it as a poor save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 2, "Rogue level 1 good Reflex (1/2+2) must equal 2");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "rogue Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 0, "Rogue level 1 poor Will (1/3) must equal 0");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "rogue Will explanation must name it as a poor save: {}",
        will.detail
    );

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.rogue.bounded_progression.base_save.unsupported"),
        "the base-save unsupported diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn rogue_level1_sneak_attack_die_count_is_grounded_and_no_longer_blocked() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 1,
        "Rogue level 1 sneak attack die count must be 1 (i.e. 1d6): {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.contains("1d6"),
        "rogue sneak-attack explanation must name the +1d6 damage die: {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.to_lowercase().contains("die count")
            || sneak_attack.detail.to_lowercase().contains("die-count"),
        "rogue sneak-attack explanation must explicitly scope itself to the die-count facet \
         only, not damage-roll execution or the trigger-condition engine: {}",
        sneak_attack.detail
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.rogue.bounded_progression.sneak_attack.unsupported"),
        "the sneak-attack unsupported diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

// ----- Still blocked: trapfinding is the sole remaining named burden -----

#[test]
fn rogue_level1_stays_blocked_naming_trapfinding_only() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

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
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
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
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "level-2 Rogue must not gain any bounded level-1 rogue chassis explanation: {:?}",
        computation.explanations
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
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "multiclass Rogue must not gain any bounded level-1 single-class rogue chassis \
         explanation: {:?}",
        computation.explanations
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
fn matrix_rogue_row_is_partial_computed_and_names_trapfinding_only() {
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
    assert!(
        note.contains("trapfinding"),
        "rogue partial note must still name the 'trapfinding' burden: {note}"
    );
    assert!(
        note.contains("only trapfinding remains unproven")
            || note.contains("only the trapfinding burden remains unproven"),
        "rogue partial note must explicitly say trapfinding is the sole remaining burden: {note}"
    );
    for grounded in ["base attack", "base save", "sneak attack"] {
        assert!(
            note.contains(grounded),
            "rogue partial note must still mention '{grounded}' as grounded: {note}"
        );
    }
    assert!(
        !note.contains("four named pillar burdens remain unproven"),
        "rogue partial note must not repeat the stale 'four burdens unproven' claim: {note}"
    );
    assert!(
        note.contains("defense.total_save"),
        "rogue partial note must still name the separate, still-true total_save gap: {note}"
    );
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

    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Blocked,
        "paladin row must stay Blocked after the rogue slice"
    );

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row must keep its later-accepted Partial posture after the rogue slice"
    );

    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the rogue slice must not promote any row to Supported or Lossy"
    );
}
