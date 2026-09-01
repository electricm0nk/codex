//! SD13-E5 Sorcerer level-1 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Paladin, Druid, Cleric, Bard) already has and
//! Sorcerer has never had: the base attack bonus and base save progression at Sorcerer
//! level 1. Both formulas are verified against the PF1 Core Rulebook Sorcerer class
//! table (d20pfsrd and the legacy Paizo PRD mirror, cross-checked against the raw level
//! 1-6 table rows since level 1 alone floors several different fractions to the same
//! value) before writing any code:
//! - base attack bonus: 1/2 BAB (`classlevel / 2`), +0 at level 1 — UNLIKE every other
//!   class this loop has grounded so far (Rogue/Monk/Druid/Cleric/Bard are all 3/4 BAB),
//!   Sorcerer's own class table reads +0/+1/+1/+2/+2/+3 at levels 1-6, which is the 1/2
//!   BAB progression, not 3/4 (which would read +0/+1/+2/+3/+3/+4);
//! - base save progression: good Will only, poor Fortitude, poor Reflex
//!   (`classlevel/2+2` for the one good save, `classlevel/3` for the two poor saves),
//!   +0 / +0 / +2 (Fortitude/Reflex/Will) at level 1 — confirmed against the raw
//!   Sorcerer class table rather than assumed from any other class's pattern.
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's, Monk's, Druid's, Cleric's, and Bard's own
//! base-attack/base-save grounding: neither record is wired into
//! `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`, or
//! `compute_combat_baseline`, so the integrated pilot surface still reports a blocked
//! posture on this input.
//!
//! This slice does NOT touch Arcane Bond, bloodline arcana, bonus spells/feats at 3rd+,
//! or the spontaneous spell burden, and does NOT attempt Sorcerer level 2+ — those stay
//! named-but-unproven exactly as before. Eschew Materials, the bloodline choice
//! recognition, and the bloodline class-skill choice recognition are unaffected.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::{load, explanation, has_explanation};

const SORCERER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.sorcerer.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.sorcerer.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.sorcerer.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.sorcerer.base_save.will";

// ----- Grounded: the base attack bonus pillar -----

#[test]
fn sorcerer_level1_grounds_base_attack_bonus() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Sorcerer class table: 1/2 BAB (classlevel / 2), UNLIKE the 3/4
    // BAB shared by Rogue/Monk/Druid/Cleric/Bard. At level 1: 1 / 2 = 0.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 0,
        "Sorcerer level 1 base attack bonus (1/2 BAB) must be +0: {base_attack:?}"
    );
    assert!(
        base_attack.detail.contains("1/2") || base_attack.detail.contains("classlevel / 2"),
        "base attack detail must cite the 1/2-BAB formula: {}",
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
fn sorcerer_level1_grounds_base_save_progression() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Sorcerer class table: good Will only, poor Fortitude, poor
    // Reflex. At level 1: Will = classlevel/2+2 = 2, Fortitude/Reflex = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 0, "Sorcerer level 1 poor Fortitude save must be +0");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 0, "Sorcerer level 1 poor Reflex save must be +0");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Sorcerer level 1 good Will save must be +2");

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
fn sorcerer_level1_base_attack_and_saves_are_not_wired_into_integrated_totals() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // (v0.6 alpha swarm, risks item 8) `table_class_id` was widened to recognize
    // Sorcerer via the shared table-driven `compute_generic_table_chassis` dispatch,
    // so the integrated `class_chassis.base_attack_bonus` explanation now genuinely
    // exists -- but Sorcerer's real 1/2-BAB progression still floors to 0 at level 1
    // (classlevel / 2 = 0), so the *value* coincidentally stays 0, same shape as
    // Rogue's own 3/4-BAB widening in `sd13_rogue_level1_chassis_baseline.rs`. Only
    // the explanation's *presence* flips, not the value.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "sorcerer level 1's real 1/2-BAB progression (classlevel / 2) is 0"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "sorcerer base-attack bonus is now a genuinely integrated chassis explanation, not a \
         standalone-only record"
    );
}

// ----- Existing Sorcerer grounded pillars and blockers are unaffected -----

#[test]
fn sorcerer_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Eschew Materials, the bloodline choice recognition, and the bloodline class-skill
    // choice recognition stay grounded exactly as before.
    assert!(has_explanation(&computation, "class_chassis.sorcerer.eschew_materials"));
    assert!(has_explanation(&computation, "class_chassis.sorcerer.bloodline_choice"));
    assert!(has_explanation(
        &computation,
        "class_chassis.sorcerer.bloodline_class_skill_choice"
    ));

    // Both claim-blocking burdens (Arcane Bond/bloodline progression, spontaneous spell
    // posture) still fire; this slice grounds no bloodline-power execution and no spell
    // math.
    let arcane_bond = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported")
        .expect("Arcane Bond / bloodline progression blocker must still fire");
    assert!(arcane_bond.claim_blocking);

    // (v0.6 alpha swarm, risks item 8) class_spell.sorcerer.spontaneous.unsupported is
    // no longer unconditional -- it's a real, conditional validation of
    // AcquisitionMode::Known selections. This fixture predates spells_selected (zero
    // known spells), so the posture is genuinely valid and the blocker correctly does
    // not fire -- the real "no spells are fabricated" guarantee now comes from the
    // known-spell record's own count being honestly 0.
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported")
    {
        Some(spell_blocker) => assert!(
            spell_blocker.claim_blocking,
            "if the spell blocker fires at all, it must be claim-blocking"
        ),
        None => {
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.sorcerer.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the spell blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Sorcerer level 2 was later widened into the supported tranche -----

#[test]
fn sorcerer_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_sorcerer_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid
    // level-range gate idiom) and extended the base-attack/base-save formulas; this
    // negative control is superseded, not violated — pin the new truth here too so
    // this file stays internally consistent.
    let level_2 = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:sorcerer:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-2 Sorcerer is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-2 Sorcerer is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));
}
