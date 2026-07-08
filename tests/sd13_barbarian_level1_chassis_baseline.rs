//! SD13-E3 Barbarian level-1 martial chassis baseline proof.
//!
//! Proves the SD13-E3 barbarian slice: the live rules-core surface ingests a
//! deterministic Human `class:barbarian:1` input, leaves direct computed
//! evidence that acknowledges the bounded level-1 martial chassis identity
//! rather than treating it as an undocumented packet placeholder, and yet
//! stays explicitly claim-blocked on the four still-missing martial burdens
//! (base-attack progression, base-save progression, fast-movement +10 ft.
//! speed extension, illiteracy trait). It also pins the matrix
//! reclassification of the barbarian row from `Unverified` / `Observed` to
//! `Partial` / `Computed`.
//!
//! It is intentionally not a martial class engine. It grounds no Fighter-
//! shaped `level_1_pilot` base-attack/base-save chassis (Barbarian has full
//! BAB and good Fortitude but the slice does not implement it), no rage
//! execution, no weapon familiarity, no level-2+ martial progression, no
//! skill-list expansion (barbarian class skills), and no illiteracy trait
//! engine. It also preserves the accepted Fighter 1-3 truth, the Rogue
//! blocked negative control, the Paladin/Ranger blocked hybrid negative
//! controls, and the Human race/interaction truth.

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

const BARBARIAN_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level1_sd13_deterministic_input.txt"
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

// ----- Direct runtime evidence: the martial chassis identity is acknowledged -----

#[test]
fn barbarian_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Barbarian chassis identity is recognized on
    // the compute path, not silently dropped as an undocumented packet placeholder.
    let chassis =
        explanation(&computation, "class_chassis.barbarian.bounded_progression");
    assert!(
        chassis.detail.contains("class:barbarian") && chassis.detail.contains("level 1"),
        "barbarian chassis recognition must name the class:barbarian:1 identity: {}",
        chassis.detail
    );
    // It is recognition only; it must not fabricate a Fighter-style computed chassis.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "barbarian baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "barbarian baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (STR 16 -> +3).
    assert_eq!(computation.ability_modifiers.strength, 3);
}

// ----- Still blocked: honest, class-specific burden diagnostics -----

#[test]
fn barbarian_level1_stays_blocked_naming_four_martial_burdens() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Four named burder-of-claim-blocking diagnostics. Each pillar of the
    // barbarian level-1 bounded chassis surface must be named explicitly so the
    // diagnosis is auditable, rather than hiding behind a generic "unsupported
    // class" label.
    let base_attack = claim_blocking(
        &computation,
        "class_feature.barbarian.bounded_progression.base_attack.unsupported",
    );
    assert!(
        base_attack.message.contains("base attack"),
        "barbarian base-attack blocker must name the 'base attack' burden: {}",
        base_attack.message
    );

    let base_save = claim_blocking(
        &computation,
        "class_feature.barbarian.bounded_progression.base_save.unsupported",
    );
    assert!(
        base_save.message.contains("base save"),
        "barbarian base-save blocker must name the 'base save' burden: {}",
        base_save.message
    );

    let fast_movement = claim_blocking(
        &computation,
        "class_feature.barbarian.bounded_progression.fast_movement.unsupported",
    );
    assert!(
        fast_movement.message.contains("fast movement"),
        "barbarian fast-movement blocker must name the 'fast movement' burden: {}",
        fast_movement.message
    );

    let illiteracy = claim_blocking(
        &computation,
        "class_feature.barbarian.bounded_progression.illiteracy.unsupported",
    );
    assert!(
        illiteracy.message.contains("illiteracy"),
        "barbarian illiteracy blocker must name the 'illiteracy' trait: {}",
        illiteracy.message
    );

    // The integrated posture is blocked, never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked barbarian baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the barbarian path -----

#[test]
fn barbarian_baseline_preserves_human_race_seam() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "barbarian baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "barbarian baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    // The bounded Human race-semantics note stays present and non-claim-blocking.
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "barbarian baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_paladin_ranger_do_not_gain_barbarian_recognition() {
    // A supported Fighter must not gain a barbarian-baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(
            &fighter_computation,
            "class_chassis.barbarian.bounded_progression"
        ),
        "the Fighter chassis must not surface a barbarian-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "the Fighter chassis must not surface barbarian class-feature burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // Paladin must stay a blocked hybrid baseline, never a barbarian baseline.
    let paladin = load(include_str!(
        "fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt"
    ));
    let paladin_computation = compute_pilot_base_chassis(&paladin);
    assert!(
        !has_explanation(
            &paladin_computation,
            "class_chassis.barbarian.bounded_progression"
        ),
        "Paladin must not surface a barbarian-baseline recognition record"
    );
    assert!(
        !paladin_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "Paladin must not surface barbarian class-feature burden diagnostics"
    );

    // Ranger must stay a blocked hybrid baseline, never a barbarian baseline.
    let ranger = load(include_str!(
        "fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt"
    ));
    let ranger_computation = compute_pilot_base_chassis(&ranger);
    assert!(
        !has_explanation(
            &ranger_computation,
            "class_chassis.barbarian.bounded_progression"
        ),
        "Ranger must not surface a barbarian-baseline recognition record"
    );
    assert!(
        !ranger_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "Ranger must not surface barbarian class-feature burden diagnostics"
    );

    // Rogue must stay a plain blocked negative control, never a barbarian baseline.
    let rogue_fixture = BARBARIAN_FIXTURE.replace("class:barbarian:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        !has_explanation(
            &rogue_computation,
            "class_chassis.barbarian.bounded_progression"
        ),
        "Rogue must not surface a barbarian-baseline recognition record"
    );
    assert!(
        !rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "Rogue must not surface barbarian class-feature burden diagnostics"
    );
}

#[test]
fn barbarian_level_2_is_not_promoted_by_this_slice() {
    // The slice is bounded to level 1; a level-2 Barbarian must not gain the
    // level-1 martial recognition record and stays blocked.
    let level_2 = BARBARIAN_FIXTURE.replace("class:barbarian:1", "class:barbarian:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.barbarian.bounded_progression"),
        "level-2 Barbarian must not gain the bounded level-1 martial recognition record"
    );
    // A level-2 barbarian must NOT surface any of the four named level-1 burden
    // diagnostics; level-2 promotion is reserved for a later SD13-E3 slice.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "level-2 Barbarian must not surface the level-1 barbarian burden diagnostics: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Barbarian must stay claim-blocked in this slice"
    );
}

#[test]
fn multiclass_barbarian_is_not_promoted_by_this_slice() {
    // A multiclass mix (Barbarian + Fighter) must not gain the bounded level-1
    // single-class martial recognition record and stays blocked.
    let multiclass = BARBARIAN_FIXTURE.replace(
        "class_level=class:barbarian:1",
        "class_level=class:barbarian:1\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.barbarian.bounded_progression"),
        "multiclass Barbarian must not gain the bounded level-1 single-class martial recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies the barbarian row to Partial/Computed -----

#[test]
fn matrix_barbarian_row_is_partial_computed_and_names_four_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let barbarian = matrix
        .row("class.barbarian.bounded_progression")
        .expect("barbarian bounded_progression row must exist");

    // Moves off the pure Unverified/Observed placeholder, but only to Partial/Computed.
    // The slice is bounded; we are not claiming Supported.
    assert_eq!(barbarian.support_state, SupportState::Partial);
    assert_eq!(barbarian.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        barbarian.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        barbarian
            .grounding_ref
            .contains("sd13_barbarian_level1_chassis_baseline"),
        "barbarian row must cite the SD13-E3 barbarian proof surface: {}",
        barbarian.grounding_ref
    );
    // The note must name all four still-missing burdens explicitly.
    let note = barbarian.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "barbarian partial row must carry a note");
    for token in ["base attack", "base save", "fast movement", "illiteracy"] {
        assert!(
            note.contains(token),
            "barbarian partial note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    // Fighter rows stay Partial/Computed.
    for id in ["class.fighter.level_1_pilot", "class.fighter.levels_2_10"] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {id} must stay Partial after the barbarian slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }

    // Paladin and Ranger stay Blocked/Computed (hybrid negative controls).
    for id in [
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "row {id} must stay Blocked after the barbarian slice"
        );
    }

    // Rogue was later promoted to Partial/Computed by its own SD13-E3 chassis
    // recognition slice.
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(rogue.support_state, SupportState::Partial);

    // No row is silently promoted to Supported or Lossy by this slice.
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the barbarian slice must not promote any row to Supported or Lossy"
    );
}
