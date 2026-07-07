//! SD13-E3 Monk level-1 martial chassis baseline proof.
//!
//! Proves the SD13-E3 monk slice (mirroring the Barbarian level-1 martial-baseline
//! pattern): the live rules-core surface ingests a deterministic Human `class:monk:1`
//! input, leaves direct computed evidence that acknowledges the bounded level-1
//! martial chassis identity rather than treating it as an undocumented packet
//! placeholder, and yet stays explicitly claim-blocked on the four still-missing
//! martial burdens (base-attack progression, base-save progression, unarmed strike /
//! Flurry of Blows, and AC Bonus / level-1 bonus feat). It also pins the matrix
//! reclassification of the monk row from `Unverified` / `Observed` to `Partial` /
//! `Computed`.
//!
//! It is intentionally not a martial class engine. It grounds no Fighter-shaped
//! `level_1_pilot` base-attack/base-save chassis (Monk has 3/4 BAB and good
//! Fortitude/Reflex/Will but the slice does not implement it), no unarmed strike
//! damage die, no Flurry of Blows execution, no AC Bonus computation, no level-1
//! bonus feat grant from the restricted Monk feat list, no ki pool, and no level-2+
//! martial progression. It also preserves the accepted Fighter 1-3 truth, the Rogue
//! blocked negative control, the Barbarian partial/computed truth, the
//! Paladin/Ranger blocked hybrid negative controls, and the Human race/interaction
//! truth.

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

const MONK_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level1_sd13_deterministic_input.txt");

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
fn monk_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let chassis = explanation(&computation, "class_chassis.monk.bounded_progression");
    assert!(
        chassis.detail.contains("class:monk") && chassis.detail.contains("level 1"),
        "monk chassis recognition must name the class:monk:1 identity: {}",
        chassis.detail
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "monk baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "monk baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (WIS 17 -> +3).
    assert_eq!(computation.ability_modifiers.wisdom, 3);
}

// ----- Still blocked: honest, class-specific burden diagnostics -----

#[test]
fn monk_level1_stays_blocked_naming_four_martial_burdens() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.base_attack.unsupported",
    );
    assert!(
        base_attack.message.contains("base attack"),
        "monk base-attack blocker must name the 'base attack' burden: {}",
        base_attack.message
    );

    let base_save = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.base_save.unsupported",
    );
    assert!(
        base_save.message.contains("base save"),
        "monk base-save blocker must name the 'base save' burden: {}",
        base_save.message
    );

    let unarmed_flurry = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.unarmed_strike_and_flurry.unsupported",
    );
    assert!(
        unarmed_flurry.message.contains("unarmed strike")
            && unarmed_flurry.message.contains("Flurry of Blows"),
        "monk unarmed-strike/flurry blocker must name both burdens: {}",
        unarmed_flurry.message
    );

    let ac_bonus_feat = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.ac_bonus_and_bonus_feat.unsupported",
    );
    assert!(
        ac_bonus_feat.message.contains("AC Bonus") && ac_bonus_feat.message.contains("bonus feat"),
        "monk AC-bonus/bonus-feat blocker must name both burdens: {}",
        ac_bonus_feat.message
    );

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked monk baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the monk path -----

#[test]
fn monk_baseline_preserves_human_race_seam() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "monk baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "monk baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "monk baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_barbarian_paladin_ranger_do_not_gain_monk_recognition() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, "class_chassis.monk.bounded_progression"),
        "the Fighter chassis must not surface a monk-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.monk.")),
        "the Fighter chassis must not surface monk class-feature burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    let barbarian = load(include_str!(
        "fixtures/rules_core/pf1_human_barbarian_level1_sd13_deterministic_input.txt"
    ));
    let barbarian_computation = compute_pilot_base_chassis(&barbarian);
    assert!(
        !has_explanation(&barbarian_computation, "class_chassis.monk.bounded_progression"),
        "Barbarian must not surface a monk-baseline recognition record"
    );
    assert!(
        !barbarian_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.monk.")),
        "Barbarian must not surface monk class-feature burden diagnostics"
    );

    let paladin = load(include_str!(
        "fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt"
    ));
    let paladin_computation = compute_pilot_base_chassis(&paladin);
    assert!(
        !has_explanation(&paladin_computation, "class_chassis.monk.bounded_progression"),
        "Paladin must not surface a monk-baseline recognition record"
    );

    let ranger = load(include_str!(
        "fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt"
    ));
    let ranger_computation = compute_pilot_base_chassis(&ranger);
    assert!(
        !has_explanation(&ranger_computation, "class_chassis.monk.bounded_progression"),
        "Ranger must not surface a monk-baseline recognition record"
    );

    let rogue_fixture = MONK_FIXTURE.replace("class:monk:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        !has_explanation(&rogue_computation, "class_chassis.monk.bounded_progression"),
        "Rogue must not surface a monk-baseline recognition record"
    );
    assert!(
        !rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.monk.")),
        "Rogue must not surface monk class-feature burden diagnostics"
    );
}

#[test]
fn monk_level_2_is_not_promoted_by_this_slice() {
    let level_2 = MONK_FIXTURE.replace("class:monk:1", "class:monk:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.monk.bounded_progression"),
        "level-2 Monk must not gain the bounded level-1 martial recognition record"
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.monk.")),
        "level-2 Monk must not surface the level-1 monk burden diagnostics: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Monk must stay claim-blocked in this slice"
    );
}

#[test]
fn multiclass_monk_is_not_promoted_by_this_slice() {
    let multiclass = MONK_FIXTURE.replace(
        "class_level=class:monk:1",
        "class_level=class:monk:1\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.monk.bounded_progression"),
        "multiclass Monk must not gain the bounded level-1 single-class martial recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Monk must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies the monk row to Partial/Computed -----

#[test]
fn matrix_monk_row_is_partial_computed_and_names_four_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk bounded_progression row must exist");

    assert_eq!(monk.support_state, SupportState::Partial);
    assert_eq!(monk.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        monk.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        monk.grounding_ref
            .contains("sd13_monk_level1_chassis_baseline"),
        "monk row must cite the SD13-E3 monk proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "monk partial row must carry a note");
    for token in [
        "base attack",
        "base save",
        "unarmed strike",
        "Flurry of Blows",
        "AC Bonus",
        "bonus feat",
    ] {
        assert!(
            note.contains(token),
            "monk partial note must name the '{token}' burden: {note}"
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
            "row {id} must stay Partial after the monk slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }

    let barbarian = matrix
        .row("class.barbarian.bounded_progression")
        .expect("barbarian row must exist");
    assert_eq!(
        barbarian.support_state,
        SupportState::Partial,
        "barbarian row must keep its accepted Partial posture after the monk slice"
    );

    for id in [
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "row {id} must stay Blocked after the monk slice"
        );
    }

    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(rogue.support_state, SupportState::Blocked);

    for id in [
        "class.sorcerer.progression_and_spell_burden",
        "class.bard.progression_and_spell_burden",
        "class.wizard.progression_and_spell_burden",
        "class.cleric.progression_and_spell_burden",
        "class.druid.progression_and_spell_burden",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "row {id} must stay Blocked after the monk slice"
        );
    }

    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the monk slice must not promote any row to Supported or Lossy"
    );
}
