//! SD13-E4-F7 Bard level-1 spontaneous arcane spell-bearing baseline proof.
//!
//! Proves the second honest SD13-F7 spell-bearing slice after Sorcerer: the live
//! rules-core surface ingests a deterministic Human `class:bard:1` input, leaves
//! direct computed evidence that recognizes the Bard level-1 spell-bearing class
//! identity rather than treating it as an undocumented packet placeholder, and yet
//! stays explicitly claim-blocked with two distinct diagnostics: one for the named
//! Bardic Knowledge + Bardic Music chassis-class-feature burden and one for the
//! spontaneous known-spell / slot posture burden. It also pins the matrix
//! reclassification of the Bard row from `Unverified` / `Observed` to
//! `Blocked` / `Computed`, while proving Sorcerer stays `Blocked` / `Computed`,
//! Wizard stays `Unverified` / `Observed`, and the accepted Paladin/Ranger hybrid
//! rows stay `Blocked` / `Computed`.
//!
//! It is intentionally not a Bard-class-feature engine and not a spell engine. It
//! fabricates no Bardic Knowledge check resolution, no Bardic Music / Inspire
//! Courage execution, no spell slots, no spells known, no spell DCs, no bonus
//! spells, no prepared posture, no school choice, and no general spell totals,
//! and it grounds no Bard level 2+. It also preserves the accepted Human race
//! seam on the spell-bearing path.

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

const BARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.bard";
const BARDIC_CLASS_FEATURE_BLOCKER_ID: &str =
    "class_feature.bard.bardic_knowledge_and_music.unsupported";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.bard.spontaneous_known_and_per_day.unsupported";

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

// ----- Direct runtime evidence: the spell-bearing identity is acknowledged -----

#[test]
fn bard_level1_leaves_direct_spell_baseline_recognition_evidence() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Bard spell-bearing identity is recognized
    // on the compute path, not silently dropped as an undocumented packet placeholder.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:bard") && recognition.detail.contains("level 1"),
        "bard recognition must name the class:bard:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "bard recognition must name the spell-bearing identity: {}",
        recognition.detail
    );

    // It is recognition only: it must carry no fabricated mechanical value (+0) and must
    // not fabricate a Fighter-style computed chassis.
    assert_eq!(
        recognition.value, 0,
        "bard spell baseline recognition must carry no fabricated value (+0)"
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "spell baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "spell baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (CHA 15 -> +2).
    assert_eq!(computation.ability_modifiers.charisma, 2);
}

#[test]
fn bard_level1_fabricates_no_spell_or_class_feature_math() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No explanation may fabricate spell slots, spells known, DCs, bonus spells, prepared
    // posture, school choice, or general spell totals. No explanation may fabricate Bardic
    // Knowledge check resolution or Bardic Music / Inspire Courage execution. The single
    // recognition record is the only spell-bearing explanation, and it carries +0.
    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID
                || (!explanation.id.contains("spell") && !explanation.id.contains("bardic")),
            "no fabricated spell or bardic-class-feature explanation is allowed beyond the \
             +0 recognition: {explanation:?}"
        );
    }
    // The recognition itself asserts it fabricates no spell math and no Bardic math.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Still blocked: two distinct honest, class-specific burden diagnostics -----

#[test]
fn bard_level1_stays_blocked_on_bardic_class_feature_burden() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The Bardic Knowledge + Bardic Music chassis-class-feature burden must be named
    // explicitly, not hidden behind a generic "unsupported caster" label.
    let bardic = claim_blocking(&computation, BARDIC_CLASS_FEATURE_BLOCKER_ID);
    assert!(
        bardic.message.contains("bardic knowledge") && bardic.message.contains("bardic music"),
        "bard class-feature blocker must name both Bardic Knowledge and Bardic Music: {}",
        bardic.message
    );
}

#[test]
fn bard_level1_stays_blocked_on_spontaneous_spell_posture_burden() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The spontaneous known-spell / slot posture burden must be a separate, explicit,
    // claim-blocking diagnostic.
    let spontaneous = claim_blocking(&computation, SPONTANEOUS_BLOCKER_ID);
    assert!(
        spontaneous.message.contains("spontaneous")
            && spontaneous.message.contains("spells known")
            && spontaneous.message.contains("spells per day"),
        "bard spell blocker must name the spontaneous known-spell / slot posture burden: {}",
        spontaneous.message
    );

    // The two burdens are genuinely distinct diagnostics.
    assert_ne!(
        BARDIC_CLASS_FEATURE_BLOCKER_ID, SPONTANEOUS_BLOCKER_ID,
        "bardic-class-feature and spontaneous burdens must be separate diagnostics"
    );
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("bard"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "bard must leave exactly two class-specific claim-blocking diagnostics: {:?}",
        computation.diagnostics
    );
}

#[test]
fn bard_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(BARD_FIXTURE);

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

// ----- The accepted Human race seam is preserved on the spell-bearing path -----

#[test]
fn spell_baseline_preserves_human_race_seam() {
    let input = load(BARD_FIXTURE);
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
fn fighter_sorcerer_and_rogue_do_not_gain_bard_recognition() {
    // A supported Fighter must not gain a bard spell-baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a bard spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("bard")),
        "Fighter must not surface bard burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // The accepted Sorcerer baseline must stay a Sorcerer baseline, never a Bard baseline.
    let sorcerer = load(include_str!(
        "fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt"
    ));
    let sorcerer_computation = compute_pilot_base_chassis(&sorcerer);
    assert!(
        !has_explanation(&sorcerer_computation, RECOGNITION_ID)
            && !sorcerer_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("bard")),
        "Sorcerer must not surface any bard recognition or burden diagnostics: {:?}",
        sorcerer_computation.diagnostics
    );

    // A Rogue must stay a plain blocked negative control, never a Bard baseline.
    let rogue_fixture = BARD_FIXTURE.replace("class:bard:1", "class:rogue:1");
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
                .any(|d| d.id.contains("bard")),
        "Rogue must not surface any bard recognition or burden diagnostics: {:?}",
        rogue_computation.diagnostics
    );
}

#[test]
fn bard_level_2_is_not_promoted_by_this_slice() {
    // The slice is bounded to level 1; a level-2 Bard must not gain the level-1
    // spell-baseline recognition record and stays blocked.
    let level_2 = BARD_FIXTURE.replace("class:bard:1", "class:bard:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, RECOGNITION_ID),
        "level-2 Bard must not gain the bounded level-1 spell-baseline recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies the Bard row to Blocked/Computed -----

#[test]
fn matrix_bard_row_is_blocked_computed_and_names_both_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard row must exist");

    // Moves off the pure Unverified/Observed placeholder, but only to Blocked/Computed.
    assert_eq!(bard.support_state, SupportState::Blocked);
    assert_eq!(bard.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        bard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        bard.grounding_ref
            .contains("sd13_bard_level1_spell_baseline"),
        "bard row must cite the SD13-F7 spell-baseline proof surface: {}",
        bard.grounding_ref
    );
    // The note must name both the Bardic class-feature burden and the spontaneous spell posture.
    let note = bard.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "bard blocked row must carry a note");
    for token in [
        "bardic knowledge",
        "bardic music",
        "spontaneous",
        "spells known",
        "spells per day",
    ] {
        assert!(
            note.contains(token),
            "bard blocked note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_wizard_row_reflects_current_truth_after_bard_slice() {
    // The Bard slice itself left Wizard Unverified/Observed; the later SD13-E4-R3
    // slice executed the Wizard row's own merge-receipt obligation, promoting it to
    // Blocked/Computed. This negative control now pins that current truth rather
    // than the Bard-slice-only snapshot.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Blocked,
        "wizard row must be Blocked after the SD13-E4-R3 promotion"
    );
    assert_eq!(
        wizard.evidence_tier,
        EvidenceTier::Computed,
        "wizard row must be Computed after the SD13-E4-R3 promotion"
    );
}

#[test]
fn matrix_keeps_sorcerer_paladin_and_ranger_blocked_computed_after_bard_slice() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    for row_id in [
        "class.sorcerer.progression_and_spell_burden",
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "row {row_id} must stay Blocked after the Bard slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }
}

#[test]
fn matrix_does_not_promote_any_row_to_supported_or_lossy_after_bard_slice() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the Bard slice must not promote any row to Supported or Lossy"
    );
}
