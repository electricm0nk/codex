//! SD13-E4 Druid level-1 prepared divine spell-burden baseline proof.
//!
//! Proves the fifth honest SD13-E4 spell-bearing slice (after Sorcerer, Bard, Wizard,
//! and Cleric): the live rules-core surface ingests a deterministic Human `class:druid:1`
//! input, leaves direct computed evidence that recognizes the Druid level-1 prepared
//! divine spell-bearing class identity rather than treating it as an undocumented
//! packet placeholder, and yet stays explicitly claim-blocked with two distinct
//! diagnostics: one for the nature bond and wild empathy class-feature burden (nature
//! bond choice between an animal companion and a domain, nature sense, wild empathy)
//! and one for the prepared divine spell posture burden (spells prepared from the full
//! Druid list, spontaneous summon nature's ally conversion, spell slots per day, bonus
//! spells from a high Wisdom, spell save DCs). The slice stays single-class,
//! level-1-only, Human-only, and grounds no spell math, no nature-bond power execution,
//! and no wild-empathy check resolution.
//!
//! It also promotes the matrix reclassification inline (mirroring the Sorcerer / Bard /
//! Cleric pattern): the in-source carrier moves the Druid row to `Blocked` / `Computed`
//! / `RefreshableFromLiveProof`, grounded on this test file, with a blocker note naming
//! both burdens.
//!
//! It is intentionally not a spell engine. It fabricates no nature-bond power
//! execution, no wild-empathy check resolution, no spellbook content, no spells
//! prepared, no spell slots per day, no spell DCs, no bonus spells, and it grounds no
//! Druid level 2+.

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

const DRUID_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.druid";
const NATURE_BOND_BLOCKER_ID: &str = "class_feature.druid.nature_bond_and_wild_empathy.unsupported";
const PREPARED_BLOCKER_ID: &str = "class_spell.druid.prepared_divine.unsupported";

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

// ----- Direct runtime evidence: the prepared divine spell-bearing identity is acknowledged -----

#[test]
fn druid_level1_leaves_direct_prepared_divine_spell_baseline_recognition_evidence() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:druid") && recognition.detail.contains("level 1"),
        "druid recognition must name the class:druid:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "druid recognition must name the spell-bearing identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("divine"),
        "druid recognition must distinguish the divine identity from the arcane Sorcerer/Wizard/Bard identities: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("prepared"),
        "druid recognition must distinguish the prepared divine identity from spontaneous arcane casters: {}",
        recognition.detail
    );

    assert_eq!(
        recognition.value, 0,
        "druid prepared divine spell baseline recognition must carry no fabricated value (+0)"
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "prepared divine spell baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "prepared divine spell baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (WIS 17 -> +3).
    assert_eq!(computation.ability_modifiers.wisdom, 3);
}

#[test]
fn druid_level1_fabricates_no_spell_math() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID || !explanation.id.contains("spell"),
            "no fabricated spell explanation is allowed beyond the +0 recognition: {explanation:?}"
        );
    }
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Still blocked: two distinct honest, class-specific burden diagnostics -----

#[test]
fn druid_level1_stays_blocked_on_nature_bond_and_wild_empathy_burden() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bond = claim_blocking(&computation, NATURE_BOND_BLOCKER_ID);
    assert!(
        bond.message.contains("nature bond") && bond.message.contains("wild empathy"),
        "druid nature-bond blocker must name the nature bond / wild empathy burden: {}",
        bond.message
    );
}

#[test]
fn druid_level1_stays_blocked_on_prepared_divine_spell_posture_burden() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let prepared = claim_blocking(&computation, PREPARED_BLOCKER_ID);
    assert!(
        prepared.message.contains("prepared") && prepared.message.contains("spell slot"),
        "druid prepared divine spell blocker must name the prepared / spell slots burden: {}",
        prepared.message
    );

    assert_ne!(
        NATURE_BOND_BLOCKER_ID, PREPARED_BLOCKER_ID,
        "nature-bond and prepared burdens must be separate diagnostics"
    );
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("druid"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "druid must leave exactly two class-specific claim-blocking diagnostics: {:?}",
        computation.diagnostics
    );
}

#[test]
fn druid_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(DRUID_FIXTURE);

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked prepared divine spell baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the prepared divine spell-bearing path -----

#[test]
fn spell_baseline_preserves_human_race_seam() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "prepared divine spell baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "prepared divine spell baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "prepared divine spell baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative controls: the druid baseline must not leak onto other classes/levels -----

#[test]
fn fighter_sorcerer_wizard_and_cleric_do_not_gain_druid_recognition() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a druid prepared-divine-spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("druid")),
        "Fighter must not surface druid burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    for other_class in ["class:sorcerer:1", "class:wizard:1", "class:cleric:1"] {
        let other_fixture = DRUID_FIXTURE.replace("class:druid:1", other_class);
        let other = load(&other_fixture);
        let other_computation = compute_pilot_base_chassis(&other);
        assert!(
            !has_explanation(&other_computation, RECOGNITION_ID)
                && !other_computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id.contains("druid")),
            "{other_class} must not surface any druid recognition or burden diagnostics: {:?}",
            other_computation.diagnostics
        );
    }
}

#[test]
fn druid_level_2_is_not_promoted_by_this_slice() {
    let level_2 = DRUID_FIXTURE.replace("class:druid:1", "class:druid:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, RECOGNITION_ID),
        "level-2 Druid must not gain the bounded level-1 prepared-divine-spell-baseline recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix row is promoted inline (Sorcerer/Bard/Cleric pattern) -----

#[test]
fn matrix_druid_row_is_blocked_computed_and_names_both_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid row must exist");

    assert_eq!(druid.support_state, SupportState::Blocked);
    assert_eq!(druid.evidence_tier, EvidenceTier::Computed);
    assert_eq!(druid.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        druid
            .grounding_ref
            .contains("sd13_druid_level1_spell_baseline"),
        "carrier grounding_ref must cite this slice's proof surface"
    );
    assert!(
        druid.blocker_or_lossiness_note.contains("nature bond")
            && druid.blocker_or_lossiness_note.contains("wild empathy")
            && druid.blocker_or_lossiness_note.contains("prepared"),
        "druid blocker note must name the nature bond, wild empathy, and prepared divine spell burdens: {}",
        druid.blocker_or_lossiness_note
    );
}

#[test]
fn matrix_preserves_sorcerer_bard_wizard_cleric_and_hybrid_blocked_computed_truth() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    for row_id in [
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
        "class.sorcerer.progression_and_spell_burden",
        "class.bard.progression_and_spell_burden",
        "class.wizard.progression_and_spell_burden",
        "class.cleric.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "row {row_id} must stay Blocked after the Druid slice"
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
        "the Druid slice must not promote any row to Supported or Lossy"
    );
}
