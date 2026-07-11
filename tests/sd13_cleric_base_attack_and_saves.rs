//! SD13-E5 Cleric level-1 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Paladin, Druid) already has and Cleric has never
//! had: the base attack bonus and base save progression at Cleric level 1. Both
//! formulas are verified against the PF1 Core Rulebook Cleric class table (d20pfsrd and
//! the legacy Paizo PRD mirror, cross-checked against the level 2-5 BAB values to
//! disambiguate the fraction, since level 1 alone floors both a 1/2 and a 3/4
//! progression to the same +0) before writing any code:
//! - base attack bonus: 3/4 BAB, the same formula shape as Rogue/Monk/Druid
//!   (`classlevel * 3 / 4`), +0 at level 1;
//! - base save progression: good Fortitude, good Will, poor Reflex
//!   (`classlevel/2+2` for the two good saves, `classlevel/3` for the one poor save),
//!   +2 / +0 / +2 at level 1 — the same save shape as Druid, confirmed independently
//!   rather than assumed from Druid's own pattern.
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's, Monk's, and Druid's own base-attack/base-save
//! grounding: neither record is wired into
//! `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`, or
//! `compute_combat_baseline`, so the integrated pilot surface still reports a blocked
//! posture on this input.
//!
//! This slice does NOT touch domain spell-list contents, the prepared divine spell
//! posture burden, the Rebuke Death heal amount, or Cleric level 2+ — those stay
//! named-but-unproven exactly as before. Channel Energy, domain choice, the flat
//! domain spell slot count, and the two grounded domain powers (Touch of Good, Rebuke
//! Death partial) are unaffected.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{SupportState, seeded_sd13_e1_f1_current_truth};

const CLERIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level1_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.cleric.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.cleric.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.cleric.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.cleric.base_save.will";

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
fn cleric_level1_grounds_base_attack_bonus() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Cleric class table: 3/4 BAB, same formula shape as
    // Rogue/Monk/Druid (classlevel * 3 / 4). At level 1: 1 * 3 / 4 = 0.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 0,
        "Cleric level 1 base attack bonus (3/4 BAB) must be +0: {base_attack:?}"
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
fn cleric_level1_grounds_base_save_progression() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Cleric class table: good Fortitude, good Will, poor Reflex.
    // At level 1: Fortitude/Will = classlevel/2+2 = 2, Reflex = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 2, "Cleric level 1 good Fortitude save must be +2");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 0, "Cleric level 1 poor Reflex save must be +0");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Cleric level 1 good Will save must be +2");

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
fn cleric_level1_base_attack_and_saves_are_not_wired_into_integrated_totals() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // ...but the integrated Fighter-shaped chassis compute path (still unsupported
    // for Cleric) is untouched: no fabricated base_attack_bonus field, and no
    // supported Fighter-style base-attack chassis explanation leaks in.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "the standalone cleric base-attack explanation must not be wired into the integrated \
         base_attack_bonus field"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "cleric baseline must not surface a supported Fighter base-attack chassis explanation"
    );
}

// ----- Existing Cleric grounded pillars and blockers are unaffected -----

#[test]
fn cleric_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Channel Energy, domain choice, the flat domain spell slot count, and the two
    // grounded domain powers stay grounded exactly as before.
    assert!(has_explanation(&computation, "class_chassis.cleric.channel_energy_dice"));
    assert!(has_explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day"
    ));
    assert!(has_explanation(&computation, "class_chassis.cleric.domain_choice"));
    assert!(has_explanation(&computation, "class_chassis.cleric.domain_spell_slot"));
    assert!(has_explanation(
        &computation,
        "class_chassis.cleric.domain_power_good_touch_of_good_bonus"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day"
    ));

    // Both claim-blocking burdens (domain powers, prepared divine spell posture)
    // still fire; this slice grounds no domain spell-list contents and no spell math.
    let domain_powers = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.cleric.domain_powers.unsupported")
        .expect("domain powers blocker must still fire");
    assert!(domain_powers.claim_blocking);
    let prepared = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.cleric.prepared_divine.unsupported")
        .expect("prepared divine spell posture blocker must still fire");
    assert!(prepared.claim_blocking);
}

// ----- Cleric level 2 was later widened into the supported tranche -----

#[test]
fn cleric_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_cleric_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk level-range
    // gate idiom) and extended the base-attack/base-save formulas; this negative
    // control is superseded, not violated — pin the new truth here too so this
    // file stays internally consistent.
    let level_2 = CLERIC_FIXTURE.replace("class:cleric:1", "class:cleric:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-2 Cleric is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-2 Cleric is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_druid_do_not_gain_cleric_base_attack_or_save_grounding() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&fighter_computation, BASE_SAVE_FORTITUDE_ID));

    let druid_fixture = CLERIC_FIXTURE.replace("class:cleric:1", "class:druid:1");
    let druid = load(&druid_fixture);
    let druid_computation = compute_pilot_base_chassis(&druid);
    assert!(!has_explanation(&druid_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&druid_computation, BASE_SAVE_FORTITUDE_ID));
}

// ----- Control plane: the matrix row's note names the newly grounded pillar -----

#[test]
fn matrix_cleric_row_note_names_base_attack_and_base_save_as_grounded() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric row must exist");

    assert_eq!(cleric.support_state, SupportState::Partial);
    assert_ne!(cleric.support_state, SupportState::Supported);
    for token in ["base attack", "base save", "standalone"] {
        assert!(
            cleric.blocker_or_lossiness_note.contains(token),
            "cleric blocker note must name '{token}' now that base attack/base save are \
             grounded: {}",
            cleric.blocker_or_lossiness_note
        );
    }
    // The still-unproven burdens stay named.
    for token in ["domain spell-list", "prepared"] {
        assert!(
            cleric.blocker_or_lossiness_note.contains(token),
            "cleric blocker note must still name the unproven '{token}' burden: {}",
            cleric.blocker_or_lossiness_note
        );
    }
}
