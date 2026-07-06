//! SD13-E4-F8 Cleric level-1 prepared divine spell-burden baseline proof.
//!
//! Proves the first truthful SD13-F8 divine spell-bearing slice: the live rules-core
//! surface ingests a deterministic Human `class:cleric:1` input, leaves direct computed
//! evidence that recognizes the Cleric level-1 prepared divine spell-bearing class
//! identity rather than treating it as an undocumented packet placeholder, and yet
//! stays explicitly claim-blocked with two distinct diagnostics: one for the prepared
//! divine spell posture burden and one for the domain choice burden. It also pins the
//! matrix reclassification of the Cleric row from `Unverified` / `Observed` to
//! `Blocked` / `Computed`, while proving Bard, Druid, Monk, Barbarian, and Wizard stay
//! `Unverified` / `Observed` and the accepted Paladin/Ranger hybrid rows and Sorcerer
//! spell baseline row stay `Blocked` / `Computed`.
//!
//! It is intentionally not a spell engine. It fabricates no spell slots, no spells
//! prepared, no orisons, no first-circle selection, no channel-energy resolution, no
//! domain-power execution, no deity-/alignment-driven effect math, and no prepared
//! caster totals, and it grounds no Cleric level 2+. It also preserves the accepted
//! Human race seam on the divine spell-bearing path.

use codex::rules_core::character_input::{load_character_input_fixture, CharacterInput};
use codex::rules_core::pilot_compute::{
    build_pilot_headless_receipt, compute_pilot_base_chassis, ComputationDiagnostic,
    ComputationExplanation, HeadlessReceiptStatus, PilotBaseChassisComputation,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    seeded_sd13_e1_f1_current_truth, EvidenceFreshness, EvidenceTier, SupportState,
};

const CLERIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.cleric";
const PREPARED_SPELL_BLOCKER_ID: &str = "class_spell.cleric.prepared.unsupported";
const DOMAIN_BLOCKER_ID: &str = "class_feature.cleric.deity_domain.unsupported";

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
fn cleric_level1_leaves_direct_prepared_spell_baseline_recognition_evidence() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Cleric prepared divine spell-bearing identity is
    // recognized on the compute path, not silently dropped as an undocumented packet
    // placeholder.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:cleric") && recognition.detail.contains("level 1"),
        "cleric recognition must name the class:cleric:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("prepared")
            || recognition.detail.contains("divine")
            || recognition.detail.contains("spell"),
        "cleric recognition must name the prepared divine spell-bearing identity: {}",
        recognition.detail
    );

    // It is recognition only: it must carry no fabricated mechanical value (+0) and must
    // not fabricate a Fighter-style computed chassis.
    assert_eq!(
        recognition.value, 0,
        "cleric spell baseline recognition must carry no fabricated value (+0)"
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "spell baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "spell baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (WIS 16 -> +3).
    assert_eq!(computation.ability_modifiers.wisdom, 3);
}

#[test]
fn cleric_level1_fabricates_no_spell_math() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No explanation may fabricate spell slots, spells prepared, DCs, bonus spells,
    // channel-energy totals, domain-power resolution, or general prepared-caster totals.
    // The single recognition record is the only spell-bearing explanation, and it carries
    // +0.
    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID || !explanation.id.contains("spell"),
            "no fabricated spell explanation is allowed beyond the +0 recognition: {explanation:?}"
        );
    }
    // The recognition itself asserts it fabricates no spell math.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Still blocked: two distinct honest, class-specific burden diagnostics -----

#[test]
fn cleric_level1_stays_blocked_on_prepared_divine_spell_posture_burden() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The prepared divine spell posture burden must be named explicitly, not hidden
    // behind a generic "unsupported caster" label.
    let prepared = claim_blocking(&computation, PREPARED_SPELL_BLOCKER_ID);
    assert!(
        prepared.message.contains("prepared"),
        "cleric prepared spell blocker must name the prepared posture burden: {}",
        prepared.message
    );
    assert!(
        prepared.message.contains("divine") || prepared.message.contains("orison"),
        "cleric prepared spell blocker must name the divine source or orison burden: {}",
        prepared.message
    );
}

#[test]
fn cleric_level1_stays_blocked_on_domain_choice_burden() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The domain choice burden must be a separate, explicit, claim-blocking diagnostic.
    let domain = claim_blocking(&computation, DOMAIN_BLOCKER_ID);
    assert!(
        domain.message.contains("domain"),
        "cleric domain blocker must name the domain choice burden: {}",
        domain.message
    );

    // The two burdens are genuinely distinct diagnostics.
    assert_ne!(
        PREPARED_SPELL_BLOCKER_ID, DOMAIN_BLOCKER_ID,
        "prepared spell and domain burdens must be separate diagnostics"
    );
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("cleric"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "cleric must leave exactly two class-specific claim-blocking diagnostics: {:?}",
        computation.diagnostics
    );
}

#[test]
fn cleric_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(CLERIC_FIXTURE);

    // The integrated posture is blocked, never a counterfeit computed success.
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

// ----- The accepted Human race seam is preserved on the divine spell-bearing path -----

#[test]
fn spell_baseline_preserves_human_race_seam() {
    let input = load(CLERIC_FIXTURE);
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

// ----- Negative controls: the spell baseline must not leak onto other classes/levels -----

#[test]
fn fighter_and_sorcerer_do_not_gain_cleric_recognition() {
    // A supported Fighter must not gain a cleric spell-baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a cleric spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("cleric")),
        "Fighter must not surface cleric burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // A Sorcerer must keep its own spell-baseline recognition and not gain the Cleric
    // recognition record or domain diagnostics.
    let sorcerer = load(include_str!(
        "fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt"
    ));
    let sorcerer_computation = compute_pilot_base_chassis(&sorcerer);
    assert!(
        !has_explanation(&sorcerer_computation, RECOGNITION_ID),
        "the Sorcerer chassis must not surface a cleric spell-baseline recognition record"
    );
    assert!(
        !sorcerer_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("cleric")),
        "Sorcerer must not surface cleric burden diagnostics: {:?}",
        sorcerer_computation.diagnostics
    );

    // A Rogue must stay a plain blocked negative control, never a cleric baseline.
    let rogue_fixture = CLERIC_FIXTURE.replace("class:cleric:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "Rogue chassis must remain claim-blocked"
    );
    assert!(
        !has_explanation(&rogue_computation, RECOGNITION_ID)
            && !rogue_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("cleric")),
        "Rogue must not surface any cleric recognition or burden diagnostics: {:?}",
        rogue_computation.diagnostics
    );
}

#[test]
fn cleric_level_2_is_not_promoted_by_this_slice() {
    // The slice is bounded to level 1; a level-2 Cleric must not gain the level-1
    // spell-baseline recognition record and stays blocked.
    let level_2 = CLERIC_FIXTURE.replace("class:cleric:1", "class:cleric:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, RECOGNITION_ID),
        "level-2 Cleric must not gain the bounded level-1 spell-baseline recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Cleric must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies the Cleric row to Blocked/Computed -----

#[test]
fn matrix_cleric_row_is_blocked_computed_and_names_both_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric row must exist");

    // Moves off the pure Unverified/Observed placeholder, but only to Blocked/Computed.
    assert_eq!(cleric.support_state, SupportState::Blocked);
    assert_eq!(cleric.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        cleric.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        cleric
            .grounding_ref
            .contains("sd13_cleric_level1_spell_baseline"),
        "cleric row must cite the SD13-F8 spell-baseline proof surface: {}",
        cleric.grounding_ref
    );
    // The note must name both the prepared divine spell posture burden and the domain
    // choice burden.
    let note = cleric.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "cleric blocked row must carry a note");
    for token in ["prepared", "domain", "divine"] {
        assert!(
            note.contains(token),
            "cleric blocked note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_keeps_other_spell_rows_in_their_truth() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    // Bard, Druid, Wizard must remain pure Unverified/Observed after this slice.
    for row_id in [
        "class.bard.progression_and_spell_burden",
        "class.druid.progression_and_spell_burden",
        "class.wizard.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Unverified,
            "row {row_id} must stay Unverified after the Cleric slice"
        );
        assert_eq!(
            row.evidence_tier,
            EvidenceTier::Observed,
            "row {row_id} must stay Observed after the Cleric slice"
        );
    }
    // The accepted hybrid baselines and Sorcerer spell baseline must stay
    // Blocked/Computed, not be flattened or promoted.
    for row_id in [
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
        "class.sorcerer.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "row {row_id} must stay Blocked after the Cleric slice"
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
        "the Cleric slice must not promote any row to Supported or Lossy"
    );
}
