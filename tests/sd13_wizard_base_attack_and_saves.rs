//! SD13-E5 Wizard level-1 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Paladin, Druid, Cleric, Bard, Sorcerer) already has
//! and Wizard has never had: the base attack bonus and base save progression at Wizard
//! level 1. Both formulas are verified against the PF1 Core Rulebook Wizard class table
//! (d20pfsrd and the legacy Paizo PRD mirror, cross-checked against the raw level 1-6
//! table rows since level 1 alone floors several different fractions to the same value)
//! before writing any code:
//! - base attack bonus: 1/2 BAB (`classlevel / 2`), +0 at level 1 — the SAME shape as
//!   Sorcerer, UNLIKE the 3/4 BAB shared by Rogue/Monk/Druid/Cleric/Bard; Wizard's own
//!   class table reads +0/+1/+1/+2/+2/+3 at levels 1-6, which is the 1/2 BAB
//!   progression, not 3/4 (which would read +0/+1/+2/+3/+3/+4);
//! - base save progression: good Will only, poor Fortitude, poor Reflex
//!   (`classlevel/2+2` for the one good save, `classlevel/3` for the two poor saves),
//!   +0 / +0 / +2 (Fortitude/Reflex/Will) at level 1 — confirmed against the raw Wizard
//!   class table rather than assumed from Sorcerer's own matching pattern.
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's, Monk's, Druid's, Cleric's, Bard's, and
//! Sorcerer's own base-attack/base-save grounding: neither record is wired into
//! `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`, or
//! `compute_combat_baseline`, so the integrated pilot surface still reports a blocked
//! posture on this input.
//!
//! This slice does NOT touch the opposed-school two-slot preparation cost, the prepared
//! spellbook/spells-per-day posture burden, or Wizard level 2+ — those stay
//! named-but-unproven exactly as before. School specialization choice recognition, the
//! specialist bonus slot, Intense Spells, and Force Missile are unaffected.

use codex::rules_core::pilot_compute::{BaseSaves, compute_pilot_base_chassis};
mod common;
use common::{load, explanation, has_explanation};

const WIZARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.wizard.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.wizard.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.wizard.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.wizard.base_save.will";

// ----- Grounded: the base attack bonus pillar -----

#[test]
fn wizard_level1_grounds_base_attack_bonus() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Wizard class table: 1/2 BAB (classlevel / 2), the same shape as
    // Sorcerer, UNLIKE the 3/4 BAB shared by Rogue/Monk/Druid/Cleric/Bard. At level 1:
    // 1 / 2 = 0.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 0,
        "Wizard level 1 base attack bonus (1/2 BAB) must be +0: {base_attack:?}"
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
fn wizard_level1_grounds_base_save_progression() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Wizard class table: good Will only, poor Fortitude, poor
    // Reflex. At level 1: Will = classlevel/2+2 = 2, Fortitude/Reflex = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 0, "Wizard level 1 poor Fortitude save must be +0");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 0, "Wizard level 1 poor Reflex save must be +0");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Wizard level 1 good Will save must be +2");

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

// ----- The grounded records were standalone until SD-21 E6.26 wired them in for real -----

#[test]
fn wizard_level1_base_attack_and_saves_are_now_wired_into_integrated_totals() {
    // SUPERSEDED, NOT VIOLATED (SD-21 Epic 6, criterion 26, 2026-07-18): at the time
    // this file's slice landed, `compute_pilot_base_chassis` dispatched only to
    // `compute_fighter_chassis`, so this test pinned the-then-live bug (any non-Fighter
    // class fabricated a `base_attack_bonus: 0` / absent generic explanation) as a
    // negative control. `compute_pilot_base_chassis` now dispatches per-class via
    // `compute_class_chassis`, and a new `compute_wizard_chassis` composes
    // `rules_tables::crb::class_tables::class_tables()`'s already-verified Wizard row
    // to wire `base_attack_bonus` / `base_saves` for real — mirroring the identical
    // "superseded, not violated" idiom this file's own
    // `wizard_level_2/3/4/5_was_later_widened_into_the_supported_tranche` negative
    // controls already use for the level-range widening. The standalone
    // `class_chassis.wizard.*` records this file grounds stay exactly as they were:
    // this slice adds a second, generic-id (`class_chassis.base_attack_bonus`, no
    // `.wizard.` infix, mirroring Fighter's own historical un-prefixed naming) record
    // that IS the one wired into the integrated fields, so both coexist without
    // conflict.
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records still exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // ...and the integrated chassis compute path is now genuinely wired for Wizard too:
    // at level 1 the 1/2-BAB formula still floors to +0 (coincidentally the same
    // number the old fabricated zero pinned), but it is now a real, dispatch-supported
    // value with its own generic explanation record backing it, not a fabricated
    // absence.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "Wizard level 1 base attack bonus (1/2 BAB, classlevel / 2) is genuinely +0 at level 1"
    );
    let base_attack = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(base_attack.value, 0);
    assert_eq!(
        computation.base_saves,
        BaseSaves {
            fortitude: 0,
            reflex: 0,
            will: 2,
        },
        "Wizard level 1 base saves (poor Fort/Ref, good Will) are now genuinely computed"
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_chassis.unsupported"),
        "a single-class Wizard is now a dispatch-supported chassis: {:?}",
        computation.diagnostics
    );
}

// ----- Existing Wizard grounded pillars and blockers are unaffected -----

#[test]
fn wizard_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The prepared arcane spell-bearing recognition record stays grounded exactly as
    // before.
    assert!(has_explanation(&computation, "class_chassis.spell_baseline.wizard"));

    // Both claim-blocking burdens (school specialization, prepared spellbook posture)
    // still fire; this slice grounds no school-opposition math and no spell math.
    let school_powers = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported")
        .expect("school-power and opposed-school-cost blocker must still fire");
    assert!(school_powers.claim_blocking);
    let prepared_spellbook = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported")
        .expect("prepared spellbook posture blocker must still fire");
    assert!(prepared_spellbook.claim_blocking);
}

// ----- Wizard level 2 was later widened into the supported tranche -----

#[test]
fn wizard_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_wizard_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/
    // Sorcerer level-range gate idiom) and extended the base-attack/base-save
    // formulas; this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent.
    let level_2 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-2 Wizard is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-2 Wizard is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));
}

// ----- Wizard level 3 was later widened into the supported tranche -----

#[test]
fn wizard_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_wizard_level3_progression.rs) widened the level-range gate to
    // level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/
    // Sorcerer level-range gate idiom) and extended the base-attack/base-save
    // formulas; this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent.
    let level_3 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-3 Wizard is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));
}

// ----- Wizard level 4 was later widened into the supported tranche -----

#[test]
fn wizard_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_wizard_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/
    // Sorcerer level-range gate idiom) and extended the base-attack/base-save
    // formulas; this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent.
    let level_4 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-4 Wizard is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));
}

// ----- Wizard level 5 was later widened into the supported tranche -----

#[test]
fn wizard_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was out of scope and stayed
    // unrecognized. A later SD13-E5 slice (tests/sd13_wizard_level5_progression.rs)
    // widened the level-range gate to level 5 and extended the base-attack/base-save
    // formulas; this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent.
    let level_5 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, BASE_ATTACK_ID),
        "level-5 Wizard is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));
}
