//! SD13-E5 Druid level-1 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Paladin) already has and Druid has never had:
//! the base attack bonus and base save progression at Druid level 1. Both formulas are
//! verified against the PF1 Core Rulebook Druid class table (d20pfsrd and the legacy
//! Paizo PRD mirror, cross-checked against the level 4/5 BAB values to disambiguate the
//! fraction, since level 1 alone floors both a 1/2 and a 3/4 progression to the same
//! +0) before writing any code:
//! - base attack bonus: 3/4 BAB, the same formula shape as Rogue/Monk
//!   (`classlevel * 3 / 4`), +0 at level 1;
//! - base save progression: good Fortitude, good Will, poor Reflex
//!   (`classlevel/2+2` for the two good saves, `classlevel/3` for the one poor save),
//!   +2 / +0 / +2 at level 1.
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's and Monk's own base-attack/base-save grounding:
//! neither record is wired into `PilotBaseChassisComputation.base_attack_bonus`,
//! `compute_total_saves`, or `compute_combat_baseline`, so the integrated pilot surface
//! still reports a blocked posture on this input.
//!
//! This slice does NOT touch the companion stat block, the prepared divine spell
//! posture burden, or Druid level 2+ — those stay named-but-unproven exactly as
//! before. Wild Empathy, Nature Sense, and the nature-bond choice recognition
//! (grounded by earlier SD13-E4/E5 slices) are unaffected.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{SupportState, seeded_sd13_e1_f1_current_truth};

const DRUID_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level1_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.druid.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.druid.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.druid.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.druid.base_save.will";

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

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Grounded: the base attack bonus pillar -----

#[test]
fn druid_level1_grounds_base_attack_bonus() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Druid class table: 3/4 BAB, same formula shape as
    // Rogue/Monk (classlevel * 3 / 4). At level 1: 1 * 3 / 4 = 0.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 0,
        "Druid level 1 base attack bonus (3/4 BAB) must be +0: {base_attack:?}"
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.contains("classlevel * 3 / 4"),
        "base attack detail must cite the 3/4-BAB formula: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("standalone"),
        "base attack detail must state it is a standalone record: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("base_attack_bonus")
            || base_attack.detail.contains("compute_combat_baseline"),
        "base attack detail must name the integrated field/seam it is NOT wired into: {}",
        base_attack.detail
    );
}

// ----- Grounded: the base save progression pillar -----

#[test]
fn druid_level1_grounds_base_save_progression() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Druid class table: good Fortitude, good Will, poor Reflex.
    // At level 1: Fortitude/Will = classlevel/2+2 = 2, Reflex = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 2, "Druid level 1 good Fortitude save must be +2");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 0, "Druid level 1 poor Reflex save must be +0");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Druid level 1 good Will save must be +2");

    for (record, label) in [
        (fortitude, "Fortitude"),
        (reflex, "Reflex"),
        (will, "Will"),
    ] {
        assert!(
            record.detail.contains("standalone"),
            "{label} base-save detail must state it is a standalone record: {}",
            record.detail
        );
        assert!(
            record.detail.contains("compute_total_saves"),
            "{label} base-save detail must name compute_total_saves as the seam it is NOT wired \
             into: {}",
            record.detail
        );
    }
}

// ----- The grounded records are standalone: not wired into any integrated total -----

#[test]
fn druid_level1_base_attack_and_saves_are_not_wired_into_integrated_totals() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // ...but the integrated Fighter-shaped chassis compute path (still unsupported
    // for Druid) is untouched: no fabricated base_attack_bonus field, and no
    // supported Fighter-style base-attack chassis explanation leaks in.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "the standalone druid base-attack explanation must not be wired into the integrated \
         base_attack_bonus field"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "druid baseline must not surface a supported Fighter base-attack chassis explanation"
    );
}

// ----- Existing Druid grounded pillars and blockers are unaffected -----

#[test]
fn druid_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Wild Empathy, Nature Sense, and the nature-bond choice recognition stay
    // grounded exactly as before.
    assert!(has_explanation(&computation, "class_chassis.druid.wild_empathy"));
    assert!(has_explanation(&computation, "class_chassis.druid.nature_sense"));
    assert!(has_explanation(&computation, "class_chassis.druid.nature_bond_choice"));

    // Both claim-blocking burdens (animal companion execution, prepared divine
    // spell posture) still fire; this slice grounds no companion stat block and no
    // spell math.
    let animal_companion = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.druid.animal_companion.unsupported")
        .expect("animal companion blocker must still fire");
    assert!(animal_companion.claim_blocking);
    let prepared = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.druid.prepared_divine.unsupported")
        .expect("prepared divine spell posture blocker must still fire");
    assert!(prepared.claim_blocking);
}

// ----- Druid level 2 was later widened into the supported tranche -----

#[test]
fn druid_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_druid_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard
    // level-range gate idiom) and extended the base-attack/base-save formulas;
    // this negative control is superseded, not violated — pin the new truth
    // here too so this file stays internally consistent.
    let level_2 = DRUID_FIXTURE.replace("class:druid:1", "class:druid:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-2 Druid is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-2 Druid is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
}

// ----- Druid level 3 was later widened into the supported tranche -----

#[test]
fn druid_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 stayed unrecognized. A later
    // SD13-E5 slice (tests/sd13_druid_level3_progression.rs) widened the
    // level-range gate to level 3 and extended the base-attack/base-save
    // formulas; this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent.
    let level_3 = DRUID_FIXTURE.replace("class:druid:1", "class:druid:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-3 Druid is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-3 Druid is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
}

// ----- Druid level 4 was later widened into the supported tranche -----

#[test]
fn druid_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 stayed unrecognized. A later
    // SD13-E5 slice (tests/sd13_druid_level4_progression.rs) widened the
    // level-range gate to level 4 and extended the base-attack/base-save
    // formulas; this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent. The equivalent
    // level-5 negative control now lives in the new
    // tests/sd13_druid_level4_progression.rs file where the coverage moved.
    let level_4 = DRUID_FIXTURE.replace("class:druid:1", "class:druid:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-4 Druid is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-4 Druid is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_barbarian_do_not_gain_druid_base_attack_or_save_grounding() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&fighter_computation, BASE_SAVE_FORTITUDE_ID));

    let barbarian_fixture = DRUID_FIXTURE.replace("class:druid:1", "class:barbarian:1");
    let barbarian = load(&barbarian_fixture);
    let barbarian_computation = compute_pilot_base_chassis(&barbarian);
    assert!(!has_explanation(&barbarian_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&barbarian_computation, BASE_SAVE_FORTITUDE_ID));
}

// ----- Control plane: the matrix row's note names the newly grounded pillar -----

#[test]
fn matrix_druid_row_note_names_base_attack_and_base_save_as_grounded() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid row must exist");

    assert_eq!(druid.support_state, SupportState::Partial);
    assert_ne!(druid.support_state, SupportState::Supported);
    for token in ["base attack", "base save", "standalone"] {
        assert!(
            druid.blocker_or_lossiness_note.contains(token),
            "druid blocker note must name '{token}' now that base attack/base save are \
             grounded: {}",
            druid.blocker_or_lossiness_note
        );
    }
    // The still-unproven burdens stay named.
    for token in ["animal companion", "prepared"] {
        assert!(
            druid.blocker_or_lossiness_note.contains(token),
            "druid blocker note must still name the unproven '{token}' burden: {}",
            druid.blocker_or_lossiness_note
        );
    }
}
