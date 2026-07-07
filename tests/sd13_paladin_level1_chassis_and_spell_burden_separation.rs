//! SD13-E3/E4 Paladin level-1 chassis-and-spell-burden separation proof.
//!
//! Proves the deeper Paladin-only decomposition that sits on top of the accepted
//! SD13-F6 hybrid baseline: the live rules-core surface emits one distinct
//! claim-blocking diagnostic per still-missing Paladin non-spell class-feature
//! burden (smite evil, lay on hands, divine grace, mercy) plus one distinct
//! claim-blocking diagnostic for the later partial-caster spell burden, and the
//! support-state matrix row for `class.paladin.hybrid_chassis_and_spell_burden`
//! reclassifies its blocker note to name each separated burden explicitly
//! instead of the single combined "non-spell class-feature" string left by F6.
//!
//! It is intentionally not a hybrid class engine. It grounds no Paladin level
//! 2+, no smite / lay-on-hands / divine-grace / mercy math, no partial-caster
//! slot math, no deity resolution, no domain mechanics, no alignment-driven
//! target resolution, no healing-resource accounting, and no spell posture
//! computation. It also preserves the accepted Fighter 1-3 truth, the Rogue
//! blocked negative control, the shared F6 hybrid blockers (so the F6 test
//! continues to pass), the Sorcerer F7 baseline truth, and the Human race /
//! interaction seam.

use codex::rules_core::character_input::{load_character_input_fixture, CharacterInput};
use codex::rules_core::pilot_compute::{
    build_pilot_headless_receipt, compute_pilot_base_chassis, ComputationDiagnostic,
    HeadlessReceiptStatus, PilotBaseChassisComputation,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    seeded_sd13_e1_f1_current_truth, EvidenceFreshness, EvidenceTier, SupportState,
};

const PALADIN_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// Exact per-burden Paladin-only diagnostics this slice proves. Each one names a
// distinct still-missing non-spell class-feature burden or the later
// partial-caster spell burden, so Paladin's chassis and spell burdens are
// separable on the runtime path.
const PALADIN_SMITE_EVIL_ID: &str = "class_feature.paladin.smite_evil.unsupported";
const PALADIN_LAY_ON_HANDS_ID: &str = "class_feature.paladin.lay_on_hands.unsupported";
const PALADIN_DIVINE_GRACE_ID: &str = "class_feature.paladin.divine_grace.unsupported";
const PALADIN_MERCY_ID: &str = "class_feature.paladin.mercy.unsupported";
const PALADIN_PARTIAL_CASTER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

// F6 hybrid blockers are accepted truth and must still be claim-blocking for
// Ranger regression preservation. The F6 test asserts both of these ids.
const F6_HYBRID_PALADIN_FEATURE_ID: &str = "class_feature.hybrid.paladin.unsupported";
const F6_HYBRID_PALADIN_SPELL_ID: &str = "class_spell.hybrid.paladin.unsupported";

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

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
}

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Per-burden Paladin chassis blockers must be present and claim-blocking -----

#[test]
fn paladin_level1_emits_separate_smite_lay_on_hands_divine_grace_mercy_blockers() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        PALADIN_SMITE_EVIL_ID,
        PALADIN_LAY_ON_HANDS_ID,
        PALADIN_DIVINE_GRACE_ID,
        PALADIN_MERCY_ID,
    ] {
        assert!(
            has_diagnostic(&computation, id),
            "paladin must surface separate claim-blocking diagnostic '{id}', got {:?}",
            computation.diagnostics
        );
        let diag = claim_blocking(&computation, id);
        assert!(
            !diag.message.is_empty(),
            "per-burden paladin blocker '{id}' must carry a non-empty message"
        );
    }
}

#[test]
fn paladin_smite_evil_blocker_names_alignment_and_target_resolution_burden() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let smite = claim_blocking(&computation, PALADIN_SMITE_EVIL_ID);
    assert!(
        smite.message.contains("smite"),
        "paladin smite blocker must name the smite burden: {}",
        smite.message
    );
    // The smite burden is inseparable from alignment target resolution. The
    // blocker must name that the alignment / target resolution is also out of
    // scope, so no fake smite execution can sneak in.
    assert!(
        smite.message.contains("alignment")
            || smite.message.contains("target")
            || smite.message.contains("evil"),
        "paladin smite blocker must name alignment/target burden: {}",
        smite.message
    );
}

#[test]
fn paladin_lay_on_hands_blocker_names_healing_resource_burden() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let lay_on_hands = claim_blocking(&computation, PALADIN_LAY_ON_HANDS_ID);
    assert!(
        lay_on_hands.message.contains("lay on hands") || lay_on_hands.message.contains("heal"),
        "paladin lay-on-hands blocker must name the lay-on-hands / healing burden: {}",
        lay_on_hands.message
    );
}

#[test]
fn paladin_divine_grace_blocker_names_save_bonus_burden() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let divine_grace = claim_blocking(&computation, PALADIN_DIVINE_GRACE_ID);
    assert!(
        divine_grace.message.contains("divine grace") || divine_grace.message.contains("save"),
        "paladin divine-grace blocker must name the divine-grace / save-bonus burden: {}",
        divine_grace.message
    );
}

#[test]
fn paladin_mercy_blocker_names_conditional_modifier_burden() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let mercy = claim_blocking(&computation, PALADIN_MERCY_ID);
    assert!(
        mercy.message.contains("mercy"),
        "paladin mercy blocker must name the mercy burden: {}",
        mercy.message
    );
}

// ----- Partial-caster spell burden is separable from the per-feature blockers -----

#[test]
fn paladin_partial_caster_blocker_is_separate_and_partial_caster_specific() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let spell = claim_blocking(&computation, PALADIN_PARTIAL_CASTER_ID);
    // The slice must name the partial-caster posture explicitly so the later
    // spell-burden closure work cannot confuse Paladin with a full divine caster
    // (Cleric / Druid) or with a hybrid-but-not-yet-spell-bearing hybrid
    // (which Paladin is not).
    assert!(
        spell.message.contains("partial")
            || spell.message.contains("half-spell")
            || spell.message.contains("half caster")
            || spell.message.contains("two levels")
            || spell.message.contains("slower"),
        "paladin partial-caster blocker must name the partial-caster posture: {}",
        spell.message
    );
    assert!(
        !spell.message.contains("full caster"),
        "paladin partial-caster blocker must not collapse into a full-caster claim: {}",
        spell.message
    );
}

#[test]
fn paladin_separated_blockers_do_not_emerge_for_ranger_or_fighter() {
    // Negative control: the separated Paladin-only blockers must not leak onto
    // the Ranger hybrid baseline — that lane owns its own F6 blockers and the
    // Ranger-only decomposition slice (when it lands).
    let ranger_input = load(RANGER_FIXTURE);
    let ranger_computation = compute_pilot_base_chassis(&ranger_input);
    for id in [
        PALADIN_SMITE_EVIL_ID,
        PALADIN_LAY_ON_HANDS_ID,
        PALADIN_DIVINE_GRACE_ID,
        PALADIN_MERCY_ID,
        PALADIN_PARTIAL_CASTER_ID,
    ] {
        assert!(
            !has_diagnostic(&ranger_computation, id),
            "ranger must not gain a Paladin-only per-burden blocker '{id}'"
        );
    }

    // And the Fighter must stay on the Fighter-shaped accepted truth, never
    // gain any hybrid or partial-caster blocker.
    let fighter_input = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter_input);
    for id in [
        PALADIN_SMITE_EVIL_ID,
        PALADIN_LAY_ON_HANDS_ID,
        PALADIN_DIVINE_GRACE_ID,
        PALADIN_MERCY_ID,
        PALADIN_PARTIAL_CASTER_ID,
        F6_HYBRID_PALADIN_FEATURE_ID,
        F6_HYBRID_PALADIN_SPELL_ID,
    ] {
        assert!(
            !has_diagnostic(&fighter_computation, id),
            "fighter must not gain a Paladin-only or hybrid blocker '{id}'"
        );
    }
}

// ----- F6 acceptance truth must still be intact -----

#[test]
fn paladin_f6_hybrid_blockers_remain_intact_under_separation() {
    // The F6 hybrid blocker ids must keep being claim-blocking. This slice is
    // an extension, never a downgrade, of the F6 acceptance surface.
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_diagnostic(&computation, F6_HYBRID_PALADIN_FEATURE_ID),
        "F6 hybrid class-feature blocker must remain claim-blocking"
    );
    assert!(
        has_diagnostic(&computation, F6_HYBRID_PALADIN_SPELL_ID),
        "F6 hybrid spell blocker must remain claim-blocking"
    );

    // The F6 chassis recognition explanation must still be present so the F6
    // test does not lose its identity-proof surface.
    assert!(
        has_explanation(&computation, "class_chassis.hybrid_baseline.paladin"),
        "F6 chassis recognition explanation must remain"
    );
}

// ----- The integrated posture stays Blocked, never fakes Computed -----

#[test]
fn paladin_level1_still_yields_blocked_headless_receipt_and_view_model() {
    let input = load(PALADIN_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Blocked,
        "separated per-burden blockers must keep the integrated paladin posture Blocked"
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked paladin posture must not emit a computed snapshot"
    );
}

// ----- Control plane: matrix row carries the separated burden note -----

#[test]
fn matrix_paladin_row_note_names_each_separated_burden() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must exist");

    // Posture stays Blocked / Computed — this slice is not a promotion.
    assert_eq!(paladin.support_state, SupportState::Blocked);
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_paladin_level1_chassis_and_spell_burden_separation"),
        "paladin row must cite this slice as its proof surface: {}",
        paladin.grounding_ref
    );

    // The matrix note must now name each per-feature burden explicitly, not
    // just the combined "non-spell class-feature" string from F6.
    let note = paladin.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "paladin blocked row must carry a note");
    for token in ["smite", "lay on hands", "divine grace", "mercy"] {
        assert!(
            note.contains(token),
            "paladin blocked note must name the '{token}' burden: {note}"
        );
    }
    // The note must also name the partial-caster posture distinctly from the
    // per-feature chassis burdens, so the later SD13-E4 spell-burden closure
    // cannot accidentally collapse Paladin into a Cleric shape.
    assert!(
        note.contains("partial-caster") || note.contains("partial caster"),
        "paladin blocked note must name the partial-caster posture: {note}"
    );
}

#[test]
fn matrix_ranger_row_does_not_borrow_paladin_separated_burden_note() {
    // The non-goal is explicit: do not collapse Paladin into Ranger. The
    // matrix must keep them on distinct rows with their own burden notes.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist");

    let ranger_note = ranger.blocker_or_lossiness_note;
    for token in [
        "smite evil",
        "lay on hands",
        "divine grace",
        "mercy",
        "partial-caster",
        "partial caster",
    ] {
        assert!(
            !ranger_note.contains(token),
            "ranger blocked note must not borrow paladin per-feature burden '{token}': {ranger_note}"
        );
    }
}

#[test]
fn matrix_preserves_fighter_rogue_sorcerer_and_other_class_truth() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    // Fighter accepted posture is preserved.
    for id in ["class.fighter.level_1_pilot", "class.fighter.levels_2_10"] {
        let row = matrix
            .row(id)
            .unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {id} must stay Partial after the paladin-decomposition slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }

    // Rogue was later promoted to Partial/Computed by its own SD13-E3 chassis
    // recognition slice.
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(rogue.support_state, SupportState::Partial);

    // Sorcerer stays at its accepted F7 posture.
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer row must exist");
    assert_eq!(sorcerer.support_state, SupportState::Blocked);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::Computed);

    // Bard carries its own accepted SD13-E4-F7 posture (Blocked/Computed) and
    // Barbarian carries its accepted SD13-E3 martial-chassis posture
    // (Partial/Computed); this slice preserves both without re-promoting them.
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard row must exist");
    assert_eq!(
        bard.support_state,
        SupportState::Blocked,
        "bard row must keep its accepted Blocked posture after the paladin-decomposition slice"
    );
    assert_eq!(bard.evidence_tier, EvidenceTier::Computed);
    let barbarian = matrix
        .row("class.barbarian.bounded_progression")
        .expect("barbarian row must exist");
    assert_eq!(
        barbarian.support_state,
        SupportState::Partial,
        "barbarian row must keep its accepted Partial posture after the paladin-decomposition slice"
    );
    assert_eq!(barbarian.evidence_tier, EvidenceTier::Computed);

    // Wizard, Cleric, and Druid were later promoted to Blocked/Computed by their
    // own follow-up slices; verified separately below.
    for id in [
        "class.wizard.progression_and_spell_burden",
        "class.cleric.progression_and_spell_burden",
        "class.druid.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(id)
            .unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(row.support_state, SupportState::Blocked);
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }

    // Monk was later promoted to Partial/Computed by its own follow-up SD13-E3 slice.
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk row must exist");
    assert_eq!(monk.support_state, SupportState::Partial);
    assert_eq!(monk.evidence_tier, EvidenceTier::Computed);

    // No row is silently promoted to Supported by this slice.
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported),
        "the paladin-decomposition slice must not promote any row to Supported"
    );
}
