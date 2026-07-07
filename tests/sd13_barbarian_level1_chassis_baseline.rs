//! SD13-E3 Barbarian level-1 martial chassis baseline proof.
//!
//! Proves the SD13-E3 barbarian slice: the live rules-core surface ingests a
//! deterministic Human `class:barbarian:1` input, leaves direct computed
//! evidence that acknowledges the bounded level-1 martial chassis identity
//! rather than treating it as an undocumented packet placeholder, and now
//! grounds three of the four named martial pillar burdens directly: base-
//! attack progression, base-save progression, and the fast-movement +10 ft.
//! speed value. The fourth burden (the illiteracy trait) stays explicitly
//! claim-blocked. It also pins the matrix reclassification of the barbarian
//! row from `Unverified` / `Observed` to `Partial` / `Computed`.
//!
//! It is intentionally not a martial class engine. The grounded base-attack
//! and base-save explanations mirror the Fighter formula shape (full BAB,
//! good Fortitude, poor Reflex/Will) but are standalone records: they are not
//! wired into `PilotBaseChassisComputation.base_attack_bonus` or into
//! `compute_total_saves`/`compute_combat_baseline`, so the integrated pilot
//! surface still reports a blocked posture. The grounded fast-movement
//! explanation asserts only the flat +10 ft. value; it grounds no
//! armor/encumbrance-state check engine (no such engine exists anywhere in
//! this codebase yet). This slice still grounds no rage execution, no weapon
//! familiarity, no level-2+ martial progression, no skill-list expansion
//! (barbarian class skills), and no illiteracy trait engine. It also
//! preserves the accepted Fighter 1-3 truth, the Rogue blocked negative
//! control, the Paladin/Ranger blocked hybrid negative controls, and the
//! Human race/interaction truth.

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

// ----- Grounded: base-attack, base-save, and fast-movement pillar burdens -----

#[test]
fn barbarian_level1_grounds_base_attack_base_save_and_fast_movement() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Base-attack progression is now grounded as a standalone explanation record
    // (full BAB, same formula shape as Fighter's cr_classes.lst:139 base-attack
    // progression), and its old "unsupported" diagnostic no longer exists.
    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 1, "Barbarian level 1 full BAB must be +1");
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.barbarian.bounded_progression.base_attack.unsupported"),
        "the grounded base-attack burden must no longer surface its old unsupported diagnostic: {:?}",
        computation.diagnostics
    );

    // Base-save progression is now grounded: good Fortitude, poor Reflex/Will.
    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Barbarian level 1 good Fortitude save must be +2");
    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 0, "Barbarian level 1 poor Reflex save must be +0");
    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 0, "Barbarian level 1 poor Will save must be +0");
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.barbarian.bounded_progression.base_save.unsupported"),
        "the grounded base-save burden must no longer surface its old unsupported diagnostic: {:?}",
        computation.diagnostics
    );

    // Fast movement is now grounded as a flat +10 ft. value only, not a computed
    // armor/encumbrance-state check.
    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Barbarian fast movement must be +10 ft.");
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.barbarian.bounded_progression.fast_movement.unsupported"),
        "the grounded fast-movement burden must no longer surface its old unsupported diagnostic: {:?}",
        computation.diagnostics
    );

    // The grounded records are standalone: they must not leak into the integrated
    // base-attack-bonus/base-saves fields, which stay owned by the (unsupported for
    // Barbarian) Fighter-shaped chassis compute path.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "the standalone barbarian base-attack explanation must not be wired into the integrated base_attack_bonus field"
    );
}

// ----- Still blocked: the illiteracy trait burden -----

#[test]
fn barbarian_level1_stays_blocked_on_illiteracy() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let illiteracy = claim_blocking(
        &computation,
        "class_feature.barbarian.bounded_progression.illiteracy.unsupported",
    );
    assert!(
        illiteracy.message.contains("illiteracy"),
        "barbarian illiteracy blocker must name the 'illiteracy' trait: {}",
        illiteracy.message
    );

    // The integrated posture is still blocked overall (the Fighter-shaped chassis
    // compute path still claim-blocks Barbarian, and illiteracy remains
    // claim-blocking), never a counterfeit computed success.
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
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "the Fighter chassis must not surface barbarian-namespaced chassis explanations: {:?}",
        fighter_computation.explanations
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
    assert!(
        !paladin_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "Paladin must not surface barbarian-namespaced chassis explanations"
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
    assert!(
        !ranger_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "Ranger must not surface barbarian-namespaced chassis explanations"
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
    assert!(
        !rogue_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "Rogue must not surface barbarian-namespaced chassis explanations"
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
    // A level-2 barbarian must NOT surface the illiteracy burden diagnostic, nor any
    // of the three now-grounded level-1 explanation records; level-2 promotion is
    // reserved for a later SD13-E3 slice.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "level-2 Barbarian must not surface the level-1 barbarian burden diagnostics: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "level-2 Barbarian must not surface the level-1 barbarian chassis explanations: {:?}",
        computation.explanations
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
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "multiclass Barbarian must not surface the level-1 barbarian chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies the barbarian row to Partial/Computed -----

#[test]
fn matrix_barbarian_row_is_partial_computed_and_names_illiteracy_as_still_unproven() {
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
    // Base-attack, base-save, and fast-movement are now grounded; only illiteracy
    // remains named as still-unproven.
    let note = barbarian.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "barbarian partial row must carry a note");
    assert!(
        note.contains("illiteracy"),
        "barbarian partial note must name the still-unproven 'illiteracy' burden: {note}"
    );
    for token in ["base attack", "base save", "fast movement"] {
        assert!(
            note.contains(token),
            "barbarian partial note must still name the now-grounded '{token}' burden: {note}"
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

    // Paladin stays Blocked/Computed (hybrid negative control).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Blocked,
        "paladin row must stay Blocked after the barbarian slice"
    );

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row must keep its later-accepted Partial posture after the barbarian slice"
    );

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
