//! SD13-E4-F8 Druid level-1 divine-prepared spell-burden baseline proof.
//!
//! Proves the bounded SD13-F8 spell-bearing slice for the Druid: the live rules-core
//! surface ingests a deterministic Human `class:druid:1` input, leaves direct computed
//! evidence that recognizes the Druid level-1 divine-prepared spell-bearing class
//! identity rather than treating it as an undocumented packet placeholder, and yet
//! stays explicitly claim-blocked with three distinct diagnostics — one for the
//! divine-prepared posture burden, one for the nature-bond burden, and one for the
//! animal-companion lineage burden. It also pins the matrix reclassification of the
//! Druid row from `Unverified` / `Observed` to `Blocked` / `Computed`, while proving
//! Bard and Wizard stay `Unverified` / `Observed` and the accepted Sorcerer and
//! Paladin/Ranger rows stay at their prior `Blocked` / `Computed` truth.
//!
//! It is intentionally not a spell engine. It fabricates no spell slots, spells
//! known/prepared, spell DCs, bonus spells, domain choice, school choice, nature-bond
//! selection, animal-companion selection, animal-companion stats, or general spell
//! totals, and it grounds no Druid level 2+. It also preserves the accepted Human
//! race seam on the divine-prepared spell-bearing path.
//!
//! It deliberately does NOT use the Cleric domain/bond burden ids — the Druid
//! nature-bond and animal-companion lineage are distinct from any Cleric domain/bond
//! burden, and conflating them would silently regress the per-class honesty the SD-13
//! matrix enforces.

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

const DRUID_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.druid";
const DIVINE_PREPARED_BLOCKER_ID: &str = "class_spell.druid.divine_prepared.unsupported";
const NATURE_BOND_BLOCKER_ID: &str = "class_feature.druid.nature_bond.unsupported";
const ANIMAL_COMPANION_BLOCKER_ID: &str = "class_feature.druid.animal_companion.unsupported";

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

// ----- Direct runtime evidence: the divine-prepared spell-bearing identity is acknowledged -----

#[test]
fn druid_level1_leaves_direct_divine_prepared_spell_baseline_recognition_evidence() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Druid divine-prepared spell-bearing identity
    // is recognized on the compute path, not silently dropped as an undocumented packet
    // placeholder.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:druid") && recognition.detail.contains("level 1"),
        "druid recognition must name the class:druid:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("divine-prepared") && recognition.detail.contains("spell"),
        "druid recognition must name the divine-prepared spell-bearing identity: {}",
        recognition.detail
    );

    // It is recognition only: it must carry no fabricated mechanical value (+0) and must
    // not fabricate a Fighter-style computed chassis.
    assert_eq!(
        recognition.value, 0,
        "druid divine-prepared baseline recognition must carry no fabricated value (+0)"
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "druid spell baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "druid divine-prepared baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (WIS 14 -> +2).
    assert_eq!(computation.ability_modifiers.wisdom, 2);
}

#[test]
fn druid_level1_fabricates_no_spell_math_and_no_companion_math() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No explanation may fabricate spell slots, spells known/prepared, spell DCs, bonus
    // spells, prepared posture, domain choice, school choice, or any general spell totals.
    // The single recognition record is the only spell-bearing explanation, and it
    // carries +0.
    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID || !explanation.id.contains("spell"),
            "no fabricated spell explanation is allowed beyond the +0 recognition: {explanation:?}"
        );
        assert!(
            !explanation.id.contains("companion"),
            "druid baseline must not fabricate animal-companion math: {explanation:?}"
        );
    }
    // The recognition itself asserts it fabricates no spell or companion math.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Still blocked: three distinct honest, class-specific burden diagnostics -----

#[test]
fn druid_level1_stays_blocked_on_divine_prepared_posture_burden() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The divine-prepared posture burden must be named explicitly, not hidden behind a
    // generic "unsupported caster" label.
    let divine_prepared = claim_blocking(&computation, DIVINE_PREPARED_BLOCKER_ID);
    assert!(
        divine_prepared.message.contains("divine-prepared")
            && divine_prepared.message.contains("spells prepared")
            && divine_prepared.message.contains("spell slot"),
        "druid divine-prepared blocker must name the divine-prepared posture burden: {}",
        divine_prepared.message
    );
}

#[test]
fn druid_level1_stays_blocked_on_nature_bond_burden() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The nature-bond burden must be a separate, explicit, claim-blocking diagnostic,
    // distinct from the divine-prepared posture and from any Cleric domain/bond
    // diagnostic.
    let nature_bond = claim_blocking(&computation, NATURE_BOND_BLOCKER_ID);
    assert!(
        nature_bond.message.contains("nature bond")
            && nature_bond.message.contains("animal companion")
            && nature_bond.message.contains("domain"),
        "druid nature-bond blocker must name the nature-bond burden family: {}",
        nature_bond.message
    );

    // The nature-bond diagnostic must not be confused with any Cleric domain/bond
    // diagnostic id; they are different burdens for different classes.
    assert!(
        !nature_bond.id.contains("cleric"),
        "druid nature-bond diagnostic must not reuse a Cleric domain/bond id: {nature_bond:?}"
    );
}

#[test]
fn druid_level1_stays_blocked_on_animal_companion_lineage_burden() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The animal-companion lineage burden must be a third distinct, explicit,
    // claim-blocking diagnostic — separate from the divine-prepared posture and from
    // the nature-bond burden.
    let companion = claim_blocking(&computation, ANIMAL_COMPANION_BLOCKER_ID);
    assert!(
        companion.message.contains("animal companion")
            && companion.message.contains("lineage")
            && companion.message.contains("level"),
        "druid animal-companion lineage blocker must name the companion lineage burden: {}",
        companion.message
    );
}

#[test]
fn druid_level1_carries_three_distinct_class_specific_burden_diagnostics() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The three burdens are genuinely distinct diagnostics.
    assert_ne!(
        DIVINE_PREPARED_BLOCKER_ID, NATURE_BOND_BLOCKER_ID,
        "divine-prepared and nature-bond burdens must be separate diagnostics"
    );
    assert_ne!(
        DIVINE_PREPARED_BLOCKER_ID, ANIMAL_COMPANION_BLOCKER_ID,
        "divine-prepared and animal-companion burdens must be separate diagnostics"
    );
    assert_ne!(
        NATURE_BOND_BLOCKER_ID, ANIMAL_COMPANION_BLOCKER_ID,
        "nature-bond and animal-companion burdens must be separate diagnostics"
    );

    // The Druid must leave exactly three class-specific claim-blocking diagnostics.
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.contains("druid"))
        .count();
    assert_eq!(
        distinct_blocking, 3,
        "druid must leave exactly three class-specific claim-blocking diagnostics: {:?}",
        computation.diagnostics
    );
}

#[test]
fn druid_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(DRUID_FIXTURE);

    // The integrated posture is blocked, never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked divine-prepared baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the divine-prepared path -----

#[test]
fn divine_prepared_baseline_preserves_human_race_seam() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "druid divine-prepared baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "druid divine-prepared baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "druid divine-prepared baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative controls: the divine-prepared baseline must not leak onto other classes/levels -----

#[test]
fn fighter_and_sorcerer_do_not_gain_druid_recognition() {
    // A supported Fighter must not gain a druid divine-prepared baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a druid divine-prepared baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("druid")),
        "Fighter must not surface druid burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // An accepted Sorcerer must not gain a druid recognition record (Sorcerer stays on
    // its own spell baseline with bloodline + spontaneous posture burdens).
    let sorcerer = load(include_str!(
        "fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt"
    ));
    let sorcerer_computation = compute_pilot_base_chassis(&sorcerer);
    assert!(
        !has_explanation(&sorcerer_computation, RECOGNITION_ID)
            && !sorcerer_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("druid")),
        "Sorcerer must not surface any druid recognition or burden diagnostics: {:?}",
        sorcerer_computation.diagnostics
    );
}

#[test]
fn druid_level_2_is_not_promoted_by_this_slice() {
    // The slice is bounded to level 1; a level-2 Druid must not gain the level-1
    // divine-prepared baseline recognition record and stays blocked.
    let level_2 = DRUID_FIXTURE.replace("class:druid:1", "class:druid:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, RECOGNITION_ID),
        "level-2 Druid must not gain the bounded level-1 divine-prepared baseline recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies the Druid row to Blocked/Computed -----

#[test]
fn matrix_druid_row_is_blocked_computed_and_names_all_three_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid row must exist");

    // Moves off the pure Unverified/Observed placeholder, but only to Blocked/Computed.
    assert_eq!(druid.support_state, SupportState::Blocked);
    assert_eq!(druid.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        druid.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        druid
            .grounding_ref
            .contains("sd13_druid_level1_spell_baseline"),
        "druid row must cite the SD13-F8 divine-prepared baseline proof surface: {}",
        druid.grounding_ref
    );
    // The note must name all three burdens: divine-prepared posture, nature-bond,
    // and animal-companion lineage.
    let note = druid.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "druid blocked row must carry a note");
    for token in [
        "divine-prepared",
        "spells prepared",
        "nature-bond",
        "animal companion",
        "lineage",
    ] {
        assert!(
            note.contains(token),
            "druid blocked note must name the '{token}' burden: {note}"
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
            "row {row_id} must stay Unverified after the Druid slice"
        );
        assert_eq!(
            row.evidence_tier,
            EvidenceTier::Observed,
            "row {row_id} must stay Observed after the Druid slice"
        );
    }
}

#[test]
fn matrix_preserves_sorcerer_and_hybrid_blocked_computed_truth() {
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
            "row {row_id} must stay Blocked after the Druid slice"
        );
        assert_eq!(
            row.evidence_tier,
            EvidenceTier::Computed,
            "row {row_id} must stay Computed after the Druid slice"
        );
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
