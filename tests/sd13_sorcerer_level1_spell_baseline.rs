//! SD13-E4-F7 Sorcerer level-1 spontaneous spell-burden baseline proof.
//!
//! Proves the first truthful SD13-F7 spell-bearing slice: the live rules-core surface
//! ingests a deterministic Human `class:sorcerer:1` input, leaves direct computed
//! evidence that recognizes the Sorcerer level-1 spell-bearing class identity rather
//! than treating it as an undocumented packet placeholder, and yet stays explicitly
//! claim-blocked with two distinct diagnostics: one for the bloodline burden and one
//! for the spontaneous known-spell / slot posture burden. It also pins the matrix
//! reclassification of the Sorcerer row from `Unverified` / `Observed` to `Blocked` /
//! `Computed`, while proving Bard and Wizard stay `Unverified` / `Observed` and the
//! accepted Paladin/Ranger hybrid rows stay `Blocked` / `Computed`.
//!
//! It is intentionally not a spell engine. It fabricates no spell slots, spells known,
//! spell DCs, bonus spells, prepared posture, school choice, or general spell totals,
//! and it grounds no Sorcerer level 2+. It also preserves the accepted Human race seam
//! on the spell-bearing path.

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

const SORCERER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.sorcerer";
const BLOODLINE_BLOCKER_ID: &str = "class_feature.sorcerer.bloodline.unsupported";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";

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

    // It is recognition only: it must carry no fabricated mechanical value (+0) and must
    // not fabricate a Fighter-style computed chassis.
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

#[test]
fn sorcerer_level1_fabricates_no_spell_math() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No explanation may fabricate spell slots, spells known, DCs, bonus spells, prepared
    // posture, school choice, or general spell totals. The single recognition record is
    // the only spell-bearing explanation, and it carries +0.
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
fn sorcerer_level1_stays_blocked_on_bloodline_burden() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The bloodline burden must be named explicitly, not hidden behind a generic
    // "unsupported caster" label.
    let bloodline = claim_blocking(&computation, BLOODLINE_BLOCKER_ID);
    assert!(
        bloodline.message.contains("bloodline"),
        "sorcerer bloodline blocker must name the bloodline burden: {}",
        bloodline.message
    );
}

#[test]
fn sorcerer_level1_stays_blocked_on_spontaneous_spell_posture_burden() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The spontaneous known-spell / slot posture burden must be a separate, explicit,
    // claim-blocking diagnostic.
    let spontaneous = claim_blocking(&computation, SPONTANEOUS_BLOCKER_ID);
    assert!(
        spontaneous.message.contains("spontaneous")
            && spontaneous.message.contains("spells known")
            && spontaneous.message.contains("spell slot"),
        "sorcerer spell blocker must name the spontaneous known-spell / slot posture burden: {}",
        spontaneous.message
    );

    // The two burdens are genuinely distinct diagnostics.
    assert_ne!(
        BLOODLINE_BLOCKER_ID, SPONTANEOUS_BLOCKER_ID,
        "bloodline and spontaneous burdens must be separate diagnostics"
    );
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("sorcerer"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "sorcerer must leave exactly two class-specific claim-blocking diagnostics: {:?}",
        computation.diagnostics
    );
}

#[test]
fn sorcerer_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(SORCERER_FIXTURE);

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

// ----- Negative controls: the spell baseline must not leak onto other classes/levels -----

#[test]
fn fighter_and_rogue_do_not_gain_sorcerer_recognition() {
    // A supported Fighter must not gain a sorcerer spell-baseline recognition record.
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

#[test]
fn sorcerer_level_2_is_not_promoted_by_this_slice() {
    // The slice is bounded to level 1; a level-2 Sorcerer must not gain the level-1
    // spell-baseline recognition record and stays blocked.
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

// ----- Control plane: the matrix reclassifies the Sorcerer row to Blocked/Computed -----

#[test]
fn matrix_sorcerer_row_is_blocked_computed_and_names_both_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer row must exist");

    // Moves off the pure Unverified/Observed placeholder, but only to Blocked/Computed.
    assert_eq!(sorcerer.support_state, SupportState::Blocked);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd13_sorcerer_level1_spell_baseline"),
        "sorcerer row must cite the SD13-F7 spell-baseline proof surface: {}",
        sorcerer.grounding_ref
    );
    // The note must name both the bloodline burden and the spontaneous spell posture.
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "sorcerer blocked row must carry a note");
    for token in ["bloodline", "spontaneous", "spells known", "spell slot"] {
        assert!(
            note.contains(token),
            "sorcerer blocked note must name the '{token}' burden: {note}"
        );
    }
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
            "row {row_id} must stay Unverified after the Sorcerer slice"
        );
        assert_eq!(
            row.evidence_tier,
            EvidenceTier::Observed,
            "row {row_id} must stay Observed after the Sorcerer slice"
        );
    }
}

#[test]
fn matrix_preserves_paladin_hybrid_blocked_and_ranger_hybrid_partial_truth() {
    // The Sorcerer slice itself does not touch the Paladin or Ranger hybrid rows.
    // The SD-13 closeout tranche (this slice + the Sorcerer slice + the Ranger
    // class-feature slice + the Paladin class-feature slice, etc.) updates the
    // matrix truth for the hybrid rows; this carrier test reflects the post-tranche
    // truth that all 21 children of the SD-13 closeout parent GATE will merge.
    //
    // Paladin row: SD-13-E3-F6 recognized the level-1 chassis; the SD-13-E3
    // Paladin class-feature slice (separate card in this tranche) will lift the
    // non-spell burden. Until that slice lands, the Paladin row stays Blocked.
    //
    // Ranger row: SD-13-E3-F6 recognized the level-1 chassis and the SD-13-E3
    // Ranger class-feature slice (this card) lifted the bounded non-spell burden
    // (favored enemy, combat style, tracking) off the claim-blocking path by
    // surfacing each as a named recognition seam. The Ranger row is now
    // Partial/Computed with only the spell burden remaining in the blocker note.
    let matrix = seeded_sd13_e1_f1_current_truth();

    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Blocked,
        "paladin row must stay Blocked until the Paladin class-feature slice lands"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);

    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row must report Partial after the Ranger class-feature slice"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::Computed);
}

#[test]
fn matrix_does_not_promote_any_row_to_supported_or_lossy() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    assert!(
        !matrix.rows.iter().any(|r| r.support_state == SupportState::Supported
            || r.support_state == SupportState::Lossy),
        "the Sorcerer slice must not promote any row to Supported or Lossy"
    );
}
