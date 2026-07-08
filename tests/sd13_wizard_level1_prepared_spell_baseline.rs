//! SD13-E4-R3 Wizard level-1 prepared arcane spell-burden baseline proof.
//!
//! Proves the third honest SD13-E4 spell-bearing slice (after Sorcerer and Bard): the
//! live rules-core surface
//! ingests a deterministic Human `class:wizard:1` input, leaves direct computed
//! evidence that recognizes the Wizard level-1 prepared arcane spell-bearing class
//! identity rather than treating it as an undocumented packet placeholder, and yet
//! stays explicitly claim-blocked with two distinct diagnostics: one for the school
//! specialization burden (specialization choice, opposed schools, specialty school
//! bonus) and one for the prepared spell posture burden (spellbook content, spells
//! prepared per day, spell slots per day, bonus slots, spell save DCs). The slice
//! stays single-class, level-1-only, Human-only, and grounds no spell math, no
//! school-opposition mechanics, and no specialty school bonus.
//!
//! A later SD13-E4 Wizard decomposition slice splits the school-specialization
//! diagnostic into two: Scribe Scroll (the free, specialization-independent bonus
//! feat every 1st-level Wizard is granted, letting them create scrolls of spells
//! they know) is grounded for real as a bounded, non-numeric grant-only
//! explanation, while the specialization CHOICE burden (the chosen school, the two
//! opposed schools, and the specialty school bonus spell slot) stays its own named,
//! claim-blocking diagnostic. The prepared spellbook / spells-prepared / spell-slot
//! posture burden is untouched by this decomposition and remains entirely unproven.
//!
//! It also pins the matrix reclassification for the Wizard row. At slice time the
//! in-source carrier stayed `Unverified` / `Observed` and the transition was owned
//! by the merge receipt; that receipt obligation was executed after the tranche 2.6
//! closeout merged to develop (2026-07-07, merge a774a2b), so the carrier row became
//! `Blocked` / `Computed` / `RefreshableFromLiveProof`, grounded on this proof
//! surface, with a blocker note naming both burdens. The Scribe Scroll decomposition
//! then promotes the row to `Partial` / `Computed` / `RefreshableFromLiveProof`
//! (grounding_ref unchanged), mirroring how the Ranger Track grounding alone flipped
//! that row from Blocked to Partial: one of the two named Wizard burdens is now
//! grounded, while the specialization-choice burden and the entire prepared spell
//! posture burden stay unproven. Bard keeps its accepted SD13-E4-F7 posture
//! (`Blocked` / `Computed`); the accepted Paladin / Sorcerer hybrid and spontaneous
//! rows stay `Blocked` / `Computed`.
//!
//! It is intentionally not a spell engine. It fabricates no spellbook content, no
//! spells prepared, no spell slots per day, no spell DCs, no bonus spells, no
//! school-opposed spell restrictions, and no specialty school bonus, and it grounds
//! no Wizard level 2+.

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

const WIZARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.wizard";
const SCRIBE_SCROLL_EXPLANATION_ID: &str = "class_chassis.wizard.scribe_scroll";
const SPECIALIZATION_CHOICE_BLOCKER_ID: &str =
    "class_feature.wizard.specialization_choice.unsupported";
const PREPARED_BLOCKER_ID: &str = "class_spell.wizard.prepared_spellbook.unsupported";

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

// ----- Direct runtime evidence: the prepared spell-bearing identity is acknowledged -----

#[test]
fn wizard_level1_leaves_direct_prepared_spell_baseline_recognition_evidence() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Wizard prepared spell-bearing identity is
    // recognized on the compute path, not silently dropped as an undocumented packet
    // placeholder.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:wizard") && recognition.detail.contains("level 1"),
        "wizard recognition must name the class:wizard:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "wizard recognition must name the spell-bearing identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("prepared"),
        "wizard recognition must distinguish the prepared arcane identity from the Sorcerer spontaneous identity: {}",
        recognition.detail
    );

    // It is recognition only: it must carry no fabricated mechanical value (+0) and must
    // not fabricate a Fighter-style computed chassis.
    assert_eq!(
        recognition.value, 0,
        "wizard prepared spell baseline recognition must carry no fabricated value (+0)"
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "prepared spell baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "prepared spell baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (INT 17 -> +3).
    assert_eq!(computation.ability_modifiers.intelligence, 3);
}

#[test]
fn wizard_level1_fabricates_no_spell_math() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No explanation may fabricate spellbook content, spells prepared, spell slots,
    // DCs, bonus spells, school opposition, specialty school bonus, or general spell
    // totals. The single recognition record is the only spell-bearing explanation, and
    // it carries +0.
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
fn wizard_level1_stays_blocked_on_specialization_choice_burden() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The specialization CHOICE burden must be named explicitly, not hidden behind a
    // generic "unsupported caster" label, and it must not have been silently resolved
    // by the Scribe Scroll grounding below.
    let specialization = claim_blocking(&computation, SPECIALIZATION_CHOICE_BLOCKER_ID);
    assert!(
        specialization.message.contains("school")
            && specialization.message.contains("opposed")
            && specialization.message.contains("specialty"),
        "wizard specialization-choice blocker must name the school / opposed / specialty school burden: {}",
        specialization.message
    );

    // The old combined diagnostic id must no longer appear: it was split, not kept
    // alongside the new one.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.wizard.school_specialization.unsupported"),
        "the old combined school_specialization diagnostic id must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn wizard_level1_scribe_scroll_is_grounded_as_specialization_independent_grant() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Scribe Scroll is grounded for real: a specialization-independent bonus feat
    // grant every 1st-level Wizard receives.
    let scribe_scroll = explanation(&computation, SCRIBE_SCROLL_EXPLANATION_ID);
    assert!(
        scribe_scroll.detail.contains("Scribe Scroll"),
        "scribe scroll explanation must name Scribe Scroll by name: {}",
        scribe_scroll.detail
    );
    assert!(
        scribe_scroll.detail.contains("bonus feat"),
        "scribe scroll explanation must name it as a bonus feat grant: {}",
        scribe_scroll.detail
    );

    // It is a boolean grant, not a numeric formula: it must carry no fabricated
    // mechanical value (+0), matching every other recognition-only explanation in
    // this bounded slice.
    assert_eq!(
        scribe_scroll.value, 0,
        "scribe scroll grant must carry no fabricated numeric value (+0)"
    );

    // It must not leak into computing the specialty school bonus spell slot or the
    // prepared spellbook posture: those stay claim-blocked, and no explanation may
    // fabricate that math.
    assert!(
        !scribe_scroll.detail.contains("specialty school bonus spell slot")
            && !scribe_scroll.detail.to_lowercase().contains("spells prepared per day"),
        "scribe scroll grounding must not silently compute specialty-slot or prepared-posture math: {}",
        scribe_scroll.detail
    );
    claim_blocking(&computation, SPECIALIZATION_CHOICE_BLOCKER_ID);
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

#[test]
fn wizard_level1_stays_blocked_on_prepared_spell_posture_burden() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The prepared spell posture burden must be a separate, explicit, claim-blocking
    // diagnostic that names the spellbook, spells prepared, and spell slots.
    let prepared = claim_blocking(&computation, PREPARED_BLOCKER_ID);
    assert!(
        prepared.message.contains("spellbook")
            && prepared.message.contains("prepared")
            && prepared.message.contains("spell slot"),
        "wizard prepared spell blocker must name the spellbook / spells prepared / spell slots burden: {}",
        prepared.message
    );

    // The two remaining burdens are genuinely distinct diagnostics.
    assert_ne!(
        SPECIALIZATION_CHOICE_BLOCKER_ID, PREPARED_BLOCKER_ID,
        "specialization-choice and prepared burdens must be separate diagnostics"
    );
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("wizard"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "wizard must leave exactly two class-specific claim-blocking diagnostics (specialization \
         choice + prepared spellbook), now that Scribe Scroll is grounded rather than blocked: {:?}",
        computation.diagnostics
    );
}

#[test]
fn wizard_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(WIZARD_FIXTURE);

    // The integrated posture is blocked, never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked prepared spell baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the prepared spell-bearing path -----

#[test]
fn spell_baseline_preserves_human_race_seam() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "prepared spell baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "prepared spell baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "prepared spell baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative controls: the prepared spell baseline must not leak onto other classes/levels -----

#[test]
fn fighter_and_sorcerer_do_not_gain_wizard_recognition() {
    // A supported Fighter must not gain a wizard prepared-spell-baseline recognition
    // record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a wizard prepared-spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("wizard")),
        "Fighter must not surface wizard burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // A Sorcerer must stay a spontaneous blocked baseline, never a wizard baseline.
    let sorcerer_fixture = WIZARD_FIXTURE.replace("class:wizard:1", "class:sorcerer:1");
    let sorcerer = load(&sorcerer_fixture);
    let sorcerer_computation = compute_pilot_base_chassis(&sorcerer);
    assert!(
        sorcerer_computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "Sorcerer chassis must remain claim-blocked"
    );
    assert!(
        !has_explanation(&sorcerer_computation, RECOGNITION_ID)
            && !sorcerer_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("wizard")),
        "Sorcerer must not surface any wizard recognition or burden diagnostics: {:?}",
        sorcerer_computation.diagnostics
    );
}

#[test]
fn wizard_level_2_is_not_promoted_by_this_slice() {
    // The slice is bounded to level 1; a level-2 Wizard must not gain the level-1
    // prepared-spell-baseline recognition record and stays blocked.
    let level_2 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, RECOGNITION_ID),
        "level-2 Wizard must not gain the bounded level-1 prepared-spell-baseline recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Wizard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: Scribe Scroll grounding promotes the row to Partial -----

#[test]
fn matrix_wizard_row_is_partial_computed_after_scribe_scroll_grounding() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");

    // The merge-receipt obligation pinned by the SD13-E4-R3 slice already carried the
    // row to Blocked/Computed; this Scribe Scroll decomposition slice grounds one of
    // the two named burdens for real (mirroring the Ranger Track promotion), flipping
    // the row from Blocked to Partial. It is not Supported: the specialization-choice
    // burden and the entire prepared spell posture burden remain claim-blocking on the
    // runtime path.
    assert_eq!(
        wizard.support_state,
        SupportState::Partial,
        "wizard row must be promoted to Partial: Scribe Scroll is grounded, but specialization \
         choice and the prepared spell posture burden stay claim-blocking"
    );
    assert_eq!(
        wizard.evidence_tier,
        EvidenceTier::Computed,
        "wizard row must carry Computed evidence: the prepared arcane recognition and Scribe \
         Scroll grant seams are live"
    );
    assert_eq!(
        wizard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "wizard row must be refreshable from this re-runnable proof surface"
    );
    // grounding_ref stays unchanged: the same test file grounds both the merge-receipt
    // posture and this decomposition.
    assert!(
        wizard
            .grounding_ref
            .contains("sd13_wizard_level1_prepared_spell_baseline"),
        "wizard row grounding_ref must cite this proof surface unchanged: {}",
        wizard.grounding_ref
    );
    // The blocker note must name the Scribe Scroll grounding and both still-unproven
    // burdens explicitly.
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        !note.is_empty(),
        "partial wizard row must carry a non-empty blocker note"
    );
    for token in [
        "Scribe Scroll",
        "specialization choice",
        "spellbook",
        "spells prepared",
        "spell slots",
    ] {
        assert!(
            note.contains(token),
            "wizard blocker note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_preserves_bard_blocked_computed_truth() {
    // Bard landed its own SD13-E4-F7 spell-baseline slice and was later promoted
    // to Partial/Computed by a further SD13-E4 decomposition slice (Bardic
    // Knowledge grounded for real); the Wizard slice must preserve that posture,
    // not re-promote or silently demote it.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard row must exist");
    assert_eq!(
        bard.support_state,
        SupportState::Partial,
        "bard row must keep its later-accepted Partial posture after the Wizard slice"
    );
    assert_eq!(
        bard.evidence_tier,
        EvidenceTier::Computed,
        "bard row must keep its accepted Computed evidence tier after the Wizard slice"
    );
    assert_eq!(
        bard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "bard row must stay refreshable from its live spell-baseline proof surface"
    );
}

#[test]
fn matrix_preserves_hybrid_paladin_and_sorcerer_blocked_computed_truth() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .unwrap_or_else(|| panic!("row class.paladin.hybrid_chassis_and_spell_burden must exist"));
    assert_eq!(
        paladin.support_state,
        SupportState::Blocked,
        "paladin row must stay Blocked after the Wizard slice"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );

    // Sorcerer was later promoted to Partial/Computed by its own SD13-E4
    // decomposition slice (Eschew Materials grounded for real).
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .unwrap_or_else(|| panic!("row class.sorcerer.progression_and_spell_burden must exist"));
    assert_eq!(
        sorcerer.support_state,
        SupportState::Partial,
        "sorcerer row must be Partial after its own SD13-E4 decomposition slice"
    );
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row must keep its later-accepted Partial posture after the Wizard slice"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
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
        "the Wizard slice must not promote any row to Supported or Lossy"
    );
}
