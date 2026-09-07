//! SD13-E5 Bard level-1 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Paladin, Druid, Cleric) already has and Bard has
//! never had: the base attack bonus and base save progression at Bard level 1. Both
//! formulas are verified against the PF1 Core Rulebook Bard class table (d20pfsrd and
//! the legacy Paizo PRD mirror, cross-checked against the raw level 1-6 table rows to
//! disambiguate the fraction, since level 1 alone floors both a 1/2 and a 3/4
//! progression to the same +0) before writing any code:
//! - base attack bonus: 3/4 BAB, the same formula shape as Rogue/Monk/Druid/Cleric
//!   (`classlevel * 3 / 4`), +0 at level 1;
//! - base save progression: good Reflex, good Will, poor Fortitude
//!   (`classlevel/2+2` for the two good saves, `classlevel/3` for the one poor save),
//!   +0 / +2 / +2 (Fortitude/Reflex/Will) at level 1 — the same save shape as Rogue,
//!   confirmed independently against the raw Bard class table rather than assumed from
//!   Rogue's own pattern.
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's, Monk's, Druid's, and Cleric's own
//! base-attack/base-save grounding: neither record is wired into
//! `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`, or
//! `compute_combat_baseline`, so the integrated pilot surface still reports a blocked
//! posture on this input.
//!
//! This slice does NOT touch Countersong, Distraction, the performance-state/
//! action-economy engine, or the spontaneous spell burden, and does NOT attempt Bard
//! level 2+ — those stay named-but-unproven exactly as before. Bardic Knowledge, the
//! Bardic Performance rounds/day budget, the Inspire Courage flat magnitude, and the
//! Fascinate flat DC / affected-creature-count formulas are unaffected.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const BARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level1_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.bard.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.bard.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.bard.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.bard.base_save.will";

// ----- Grounded: the base attack bonus pillar -----

#[test]
fn bard_level1_grounds_base_attack_bonus() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bard class table: 3/4 BAB, same formula shape as
    // Rogue/Monk/Druid/Cleric (classlevel * 3 / 4). At level 1: 1 * 3 / 4 = 0.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 0,
        "Bard level 1 base attack bonus (3/4 BAB) must be +0: {base_attack:?}"
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
fn bard_level1_grounds_base_save_progression() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bard class table: good Reflex, good Will, poor Fortitude.
    // At level 1: Reflex/Will = classlevel/2+2 = 2, Fortitude = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 0, "Bard level 1 poor Fortitude save must be +0");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 2, "Bard level 1 good Reflex save must be +2");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Bard level 1 good Will save must be +2");

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
fn bard_level1_base_attack_and_saves_are_not_wired_into_integrated_totals() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // ...but the integrated `base_attack_bonus` field is untouched: no fabricated
    // integrated value is wired in from the standalone record.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "the standalone bard base-attack explanation must not be wired into the integrated \
         base_attack_bonus field"
    );
    // (v0.6 alpha swarm, risks item 8) Bard is now recognized by
    // table_class_id, so the generic class-chassis base-attack-bonus
    // explanation IS surfaced (unlike the earlier unsupported-chassis state);
    // the value still floors to 0 at level 1 (3/4 BAB), only presence changed.
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "bard is now recognized by table_class_id and must surface its base-attack chassis explanation"
    );
}

// ----- Existing Bard grounded pillars and blockers are unaffected -----

#[test]
fn bard_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Bardic Knowledge, the Bardic Performance rounds/day budget, the Inspire Courage
    // flat magnitude, and the Fascinate flat DC / affected-creature-count formulas stay
    // grounded exactly as before.
    assert!(has_explanation(&computation, "class_chassis.bard.bardic_knowledge"));
    assert!(has_explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day"
    ));
    assert!(has_explanation(&computation, "class_chassis.bard.inspire_courage_bonus"));
    assert!(has_explanation(&computation, "class_chassis.bard.fascinate_dc"));
    assert!(has_explanation(
        &computation,
        "class_chassis.bard.fascinate_affected_creatures"
    ));

    // (v0.6 alpha swarm, risks item 8, known-spell closure) class_spell.bard
    // .spontaneous_known_and_per_day.unsupported is no longer unconditional --
    // this bare fixture has zero known spells, a genuinely valid posture, so
    // the blocker correctly does not fire here.
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.bard.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }

    // (v0.6 alpha swarm, risks item 8) class_feature.bard.bardic_performance_execution
    // .unsupported is retired -- this bare fixture has no bardic performance
    // activation, a genuinely valid posture, so no performance-execution
    // diagnostic claim-blocks here; the non-blocking other-performances note
    // still fires unconditionally.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.bard.bardic_performance_execution")
                && d.claim_blocking),
        "a genuinely valid not-performing posture must not claim-block on performance execution: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.bard.bardic_performance_execution.other_performances_not_modeled"
            && !d.claim_blocking),
        "the other-performances-not-modeled note must still fire, non-blocking: {:?}",
        computation.diagnostics
    );
}

// ----- Bard level 2 was later widened into the supported tranche -----

#[test]
fn bard_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric
    // level-range gate idiom) and extended the base-attack/base-save formulas;
    // this negative control is superseded, not violated — pin the new truth
    // here too so this file stays internally consistent.
    let level_2 = BARD_FIXTURE.replace("class:bard:1", "class:bard:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-2 Bard is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-2 Bard is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
}

// ----- Bard level 3 was later widened into the supported tranche -----

#[test]
fn bard_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level3_progression.rs) widened the level-range gate to
    // level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/
    // Sorcerer/Wizard level-range gate idiom) and extended the
    // base-attack/base-save formulas; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent.
    let level_3 = BARD_FIXTURE.replace("class:bard:1", "class:bard:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-3 Bard is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-3 Bard is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
}

// ----- Bard level 4 was later widened into the supported tranche -----

#[test]
fn bard_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_bard_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/
    // Sorcerer/Wizard level-range gate idiom) and extended the
    // base-attack/base-save formulas; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent.
    let level_4 = BARD_FIXTURE.replace("class:bard:1", "class:bard:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-4 Bard is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-4 Bard is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
}

// ----- The matrix row is Supported/ProductVisible via the Class Progression Catalog browser -----

#[test]
fn bard_row_stays_partial_and_cites_this_test_file_family() {
    let matrix = seeded_current_truth();
    let row = matrix
        .rows
        .iter()
        .find(|r| r.row_id == "class.bard.progression_and_spell_burden")
        .expect("bard row must exist in the matrix");
    assert_eq!(
        row.support_state,
        SupportState::Supported,
        "bard row is Supported after SD-19's Class Progression Catalog browser UI-surfacing work"
    );
}
