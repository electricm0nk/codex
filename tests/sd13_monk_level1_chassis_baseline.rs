//! SD13-E3 Monk level-1 martial chassis baseline proof.
//!
//! Proves the SD13-E3 monk slice (mirroring the Barbarian level-1 martial-baseline
//! pattern): the live rules-core surface ingests a deterministic Human `class:monk:1`
//! input, leaves direct computed evidence that acknowledges the bounded level-1
//! martial chassis identity rather than treating it as an undocumented packet
//! placeholder, and now grounds three pillar burdens directly (base-attack
//! progression, base-save progression, and AC Bonus / Wisdom-to-AC), while staying
//! explicitly claim-blocked on the two still-missing burdens (unarmed strike /
//! Flurry of Blows, and the level-1 bonus feat grant). It also pins the matrix
//! reclassification of the monk row from `Unverified` / `Observed` to `Partial` /
//! `Computed`.
//!
//! It is intentionally not a martial class engine. It grounds the Monk 3/4-BAB
//! progression (`classlevel * 3 / 4`), the all-three-good base-save progression
//! (`classlevel / 2 + 2` for Fortitude, Reflex, and Will), and the flat level-1
//! Wisdom-to-AC value (Wisdom modifier, if positive, added to AC), but it grounds
//! no unarmed strike damage die, no Flurry of Blows execution, no level-1 bonus
//! feat grant from the restricted Monk feat list, no ki pool, no level-4+ AC Bonus
//! dodge-bonus progression, no "unarmored and unencumbered" runtime state-check
//! engine, and no level-2+ martial progression. It also preserves the accepted
//! Fighter 1-3 truth, the Rogue blocked negative control, the Barbarian
//! partial/computed truth, the Paladin/Ranger blocked hybrid negative controls, and
//! the Human race/interaction truth.

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

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
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

// ----- Grounded: base-attack, base-save, and AC Bonus pillar burdens -----

#[test]
fn monk_level1_grounds_base_attack_bonus() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 0,
        "monk level 1 base attack bonus is 3/4-BAB: 1 * 3 / 4 = 0"
    );
    assert!(
        base_attack.detail.contains("3/4-BAB"),
        "monk base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );

    assert!(
        !has_diagnostic(
            &computation,
            "class_feature.monk.bounded_progression.base_attack.unsupported"
        ),
        "the old base-attack blocker diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn monk_level1_grounds_base_save_progression() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.monk.base_save.fortitude");
    let reflex = explanation(&computation, "class_chassis.monk.base_save.reflex");
    let will = explanation(&computation, "class_chassis.monk.base_save.will");
    assert_eq!(fortitude.value, 2, "monk level 1 good Fortitude: 1/2+2 = 2");
    assert_eq!(reflex.value, 2, "monk level 1 good Reflex: 1/2+2 = 2");
    assert_eq!(will.value, 2, "monk level 1 good Will: 1/2+2 = 2");
    assert!(
        fortitude.detail.contains("good") && reflex.detail.contains("good") && will.detail.contains("good"),
        "monk base-save explanations must call out that all three saves are good: {:?} {:?} {:?}",
        fortitude.detail,
        reflex.detail,
        will.detail
    );

    assert!(
        !has_diagnostic(
            &computation,
            "class_feature.monk.bounded_progression.base_save.unsupported"
        ),
        "the old base-save blocker diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn monk_level1_grounds_ac_bonus() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // WIS 17 -> +3 modifier, so AC Bonus = max(3, 0) = 3.
    let ac_bonus = explanation(&computation, "class_chassis.monk.ac_bonus");
    assert_eq!(
        ac_bonus.value, 3,
        "monk AC Bonus is the positive Wisdom modifier added to AC: max(3, 0) = 3"
    );
    assert!(
        ac_bonus.detail.contains("Wisdom"),
        "monk AC Bonus explanation must name the Wisdom bonus source: {}",
        ac_bonus.detail
    );
}

// ----- Still blocked: unarmed strike / Flurry of Blows -----

#[test]
fn monk_level1_stays_blocked_on_unarmed_strike_and_flurry() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

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

// ----- Still blocked: narrowed bonus-feat-only diagnostic -----

#[test]
fn monk_level1_stays_blocked_naming_bonus_feat_only() {
    let input = load(MONK_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(
            &computation,
            "class_feature.monk.bounded_progression.ac_bonus_and_bonus_feat.unsupported"
        ),
        "the old combined AC-Bonus/bonus-feat blocker diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );

    let bonus_feat = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.bonus_feat.unsupported",
    );
    assert!(
        bonus_feat.message.contains("bonus feat"),
        "monk bonus-feat blocker must name the 'bonus feat' burden: {}",
        bonus_feat.message
    );
    assert!(
        !bonus_feat.message.contains("AC Bonus"),
        "the narrowed bonus-feat blocker must not claim AC Bonus is unimplemented: {}",
        bonus_feat.message
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
fn matrix_monk_row_is_partial_computed_and_names_remaining_burdens() {
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
    // Base-attack, base-save, and AC Bonus are now grounded; only unarmed strike /
    // Flurry of Blows and the bonus feat grant remain named as still-unproven.
    for token in ["unarmed strike", "Flurry of Blows", "bonus feat"] {
        assert!(
            note.contains(token),
            "monk partial note must name the still-unproven '{token}' burden: {note}"
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

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Partial,
        "paladin row must keep its later-accepted Partial posture after the monk slice"
    );

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row must keep its later-accepted Partial posture after the monk slice"
    );

    // Rogue was later promoted to Partial/Computed by its own SD13-E3 chassis
    // recognition slice.
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(rogue.support_state, SupportState::Partial);

    // Sorcerer, Bard, Cleric, and Druid were later promoted to Partial/Computed by
    // their own SD13-E4 decomposition slices (Eschew Materials, Bardic Knowledge,
    // Channel Energy, Wild Empathy).
    for id in [
        "class.sorcerer.progression_and_spell_burden",
        "class.bard.progression_and_spell_burden",
        "class.cleric.progression_and_spell_burden",
        "class.druid.progression_and_spell_burden",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {id} must be Partial after its own SD13-E4 decomposition slice"
        );
    }

    // Wizard was later promoted to Partial/Computed by its own SD13-E4 Scribe
    // Scroll decomposition slice.
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Partial,
        "wizard row must keep its later-accepted Partial posture after the monk slice"
    );

    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the monk slice must not promote any row to Supported or Lossy"
    );
}
