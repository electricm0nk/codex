//! Task #66: Wizard's Abjuration arcane school -- Wizard half only.
//!
//! Grounds the three magnitude-bearing Abjuration School power records from
//! `cr_abilities_class.lst` (independently verified this session against
//! `$HOME/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst`,
//! `KEY:Abjuration School ~ *`):
//!
//! - Resistance (`KEY:Abjuration School ~ Resistance`): `BONUS:VAR|
//!   AbjurationResistanceBonus|5` (unconditional from the power's own
//!   level-1 unlock, `PREVARGTEQ:AbjurationProgressionSchoolLVL,1` on the
//!   Abjuration School record's `ABILITY:` grant line) plus a second
//!   `BONUS:VAR|AbjurationResistanceBonus|5|PREVARGTEQ:
//!   AbjurationProgressionSchoolLVL,11` that stacks to 10 at level 11+. At
//!   level 20 the record's DESC text (not a `BONUS:VAR` formula) replaces
//!   "resistance" with "immunity" -- a non-numeric capstone. The corpus's
//!   `#Immunity` KEY record is commented out (inactive), matching this
//!   task's pre-verified fact that Immunity "stays deferred/named only": no
//!   Resistance explanation fires at level 20+.
//! - Protective Ward (`KEY:Abjuration School ~ Protective Ward`): three flat,
//!   non-dice `BONUS:VAR` formulas --
//!   `AbjurationProtectiveWardTimes|ArcaneSchoolPowerTimes` (itself
//!   `DEFINE:ArcaneSchoolPowerTimes|0` `BONUS:VAR|ArcaneSchoolPowerTimes|
//!   INT+3` on the shared "Arcane School Tracker" internal record --
//!   confirmed the SAME "3 + Intelligence modifier" idiom the pre-existing
//!   Force Missile grounding already uses, per `INT` resolving to the
//!   ability MODIFIER in this codebase's BONUS:VAR convention, confirmed by
//!   `sd13_wizard_evocation_school_powers.rs`'s own fixture-vs-expected-value
//!   check), `AbjurationProtectiveWardDuration|INT` (the bare ability
//!   modifier, no floor encoded in the corpus formula itself), and
//!   `AbjurationProtectiveWardBonus|(AbjurationSchoolLVL/5)+1`.
//! - Energy Absorption (`KEY:Abjuration School ~ Energy Absorption`):
//!   `BONUS:VAR|AbjurationEnergyAbsorption|AbjurationSchoolLVL*3`, gated on
//!   the power's own level-6 unlock (`PREVARGTEQ:
//!   AbjurationProgressionSchoolLVL,6` on the Abjuration School record's
//!   `ABILITY:` grant line for Energy Absorption).
//!
//! The chain (also independently confirmed in the corpus this session, in
//! the "Arcane School Support" block): `AbjurationSchoolLVL` <-
//! `ArcaneSchoolLVL` <- `WizardLVL` (`BONUS:VAR|ArcaneSchoolLVL|WizardLVL`
//! on the internal "Arcane School Tracker" record), and
//! `AbjurationProgressionSchoolLVL` <- `ArcaneSchoolProgressionLVL` <-
//! `WizardLVL` identically. This slice builds ONLY that Wizard-fed path.
//! `ArcaneSchoolLVL` is ALSO fed by `ArcanistLvl` via the Arcanist Exploit
//! "School Understanding" -- that path, and School Understanding itself
//! (a chooser-in-chooser, per task #55's scoping), are explicitly NOT built
//! here. No Arcanist wiring, no School Understanding, and no numeric
//! Immunity value are added by this slice.
//!
//! Explanation ids live under the shared `class_feature.school.abjuration.*`
//! namespace (not a Wizard-specific id), per this session's ruling to match
//! the `class_feature.familiar.*` / `class_feature.domain.*` shared-ladder
//! precedent -- positioning the namespace correctly for when Arcanist's own
//! half (via School Understanding) is built later.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::{load, explanation, has_explanation};

const LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_abjuration_school_input.txt");
const LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level6_abjuration_school_input.txt");
const LEVEL11_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level11_abjuration_school_input.txt");
const LEVEL20_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level20_abjuration_school_input.txt");
const EVOCATION_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt");

const RESISTANCE_ID: &str = "class_feature.school.abjuration.resistance";
const PROTECTIVE_WARD_USES_PER_DAY_ID: &str =
    "class_feature.school.abjuration.protective_ward_uses_per_day";
const PROTECTIVE_WARD_DURATION_ID: &str =
    "class_feature.school.abjuration.protective_ward_duration";
const PROTECTIVE_WARD_DEFLECTION_BONUS_ID: &str =
    "class_feature.school.abjuration.protective_ward_deflection_bonus";
const ENERGY_ABSORPTION_ID: &str = "class_feature.school.abjuration.energy_absorption";

// ----- Resistance: flat 5 from level 1, stacks to flat 10 at level 11+ -----

#[test]
fn level1_grounds_resistance_at_5() {
    let input = load(LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let resistance = explanation(&computation, RESISTANCE_ID);
    assert_eq!(resistance.value, 5, "level 1 Abjuration Resistance must ground a flat +5");
    assert!(resistance.detail.contains("Resistance"));
}

#[test]
fn level6_resistance_still_5() {
    let input = load(LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let resistance = explanation(&computation, RESISTANCE_ID);
    assert_eq!(resistance.value, 5, "level 6 Abjuration Resistance stays flat +5 (bump is at 11)");
}

#[test]
fn level11_resistance_stacks_to_10() {
    let input = load(LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let resistance = explanation(&computation, RESISTANCE_ID);
    assert_eq!(
        resistance.value, 10,
        "level 11 Abjuration Resistance must stack to a flat +10 (two +5 BONUS:VAR lines)"
    );
}

#[test]
fn level20_resistance_grounds_no_value_since_it_converts_to_undgrounded_immunity() {
    let input = load(LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, RESISTANCE_ID),
        "at level 20 the corpus DESC replaces resistance with immunity (a tokenless, \
         non-numeric capstone left deferred/named-only); no Resistance magnitude may be \
         claimed"
    );
}

// ----- Protective Ward: three flat, level-scaled magnitudes -----

#[test]
fn level1_grounds_protective_ward_magnitudes() {
    let input = load(LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // ArcaneSchoolPowerTimes = INT modifier + 3. Fixture Intelligence is 17 + 2
    // Human racial -> modifier +4 -> 3 + 4 = 7 (same idiom as Force Missile).
    let uses = explanation(&computation, PROTECTIVE_WARD_USES_PER_DAY_ID);
    assert_eq!(uses.value, 7, "level 1 Protective Ward uses-per-day pool must be 3 + Int-mod = 7");

    // AbjurationProtectiveWardDuration|INT -- the bare Intelligence modifier.
    let duration = explanation(&computation, PROTECTIVE_WARD_DURATION_ID);
    assert_eq!(duration.value, 4, "level 1 Protective Ward duration must be the Int modifier (4)");

    // AbjurationProtectiveWardBonus|(AbjurationSchoolLVL/5)+1. At level 1: (1/5)+1 = 1.
    let bonus = explanation(&computation, PROTECTIVE_WARD_DEFLECTION_BONUS_ID);
    assert_eq!(bonus.value, 1, "level 1 Protective Ward deflection bonus must be (1/5)+1 = 1");
}

#[test]
fn level6_protective_ward_deflection_bonus_rises() {
    let input = load(LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, PROTECTIVE_WARD_DEFLECTION_BONUS_ID);
    assert_eq!(bonus.value, 2, "level 6 Protective Ward deflection bonus must be (6/5)+1 = 2");

    // Uses-per-day and duration are level-independent (only Intelligence-driven).
    let uses = explanation(&computation, PROTECTIVE_WARD_USES_PER_DAY_ID);
    assert_eq!(uses.value, 7);
    let duration = explanation(&computation, PROTECTIVE_WARD_DURATION_ID);
    assert_eq!(duration.value, 4);
}

#[test]
fn level11_protective_ward_deflection_bonus_rises_again() {
    let input = load(LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let bonus = explanation(&computation, PROTECTIVE_WARD_DEFLECTION_BONUS_ID);
    assert_eq!(bonus.value, 3, "level 11 Protective Ward deflection bonus must be (11/5)+1 = 3");
}

#[test]
fn level20_protective_ward_still_grounds_all_three_magnitudes() {
    let input = load(LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, PROTECTIVE_WARD_DEFLECTION_BONUS_ID);
    assert_eq!(bonus.value, 5, "level 20 Protective Ward deflection bonus must be (20/5)+1 = 5");
    let uses = explanation(&computation, PROTECTIVE_WARD_USES_PER_DAY_ID);
    assert_eq!(uses.value, 7);
    let duration = explanation(&computation, PROTECTIVE_WARD_DURATION_ID);
    assert_eq!(duration.value, 4);
}

// ----- Energy Absorption: unlocks at level 6, flat AbjurationSchoolLVL*3 -----

#[test]
fn level1_grounds_no_energy_absorption_before_it_unlocks() {
    let input = load(LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, ENERGY_ABSORPTION_ID),
        "Energy Absorption unlocks at AbjurationProgressionSchoolLVL 6 (PREVARGTEQ:...,6); \
         a level 1 wizard must not gain it"
    );
}

#[test]
fn level6_grounds_energy_absorption_at_18() {
    let input = load(LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let absorption = explanation(&computation, ENERGY_ABSORPTION_ID);
    assert_eq!(absorption.value, 18, "level 6 Energy Absorption must be AbjurationSchoolLVL*3 = 18");
}

#[test]
fn level11_energy_absorption_rises_to_33() {
    let input = load(LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let absorption = explanation(&computation, ENERGY_ABSORPTION_ID);
    assert_eq!(absorption.value, 33, "level 11 Energy Absorption must be 11*3 = 33");
}

#[test]
fn level20_energy_absorption_rises_to_60() {
    let input = load(LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);
    let absorption = explanation(&computation, ENERGY_ABSORPTION_ID);
    assert_eq!(absorption.value, 60, "level 20 Energy Absorption must be 20*3 = 60");
}

// ----- Negative controls: no canonical Abjuration selection, no grounding -----

#[test]
fn without_canonical_abjuration_selection_no_school_power_is_grounded() {
    let stripped: String = LEVEL1_FIXTURE
        .lines()
        .filter(|line| {
            !line.contains("choice:wizard_school_specialization")
                && !line.contains("choice:wizard_opposed_schools")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&stripped);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        RESISTANCE_ID,
        PROTECTIVE_WARD_USES_PER_DAY_ID,
        PROTECTIVE_WARD_DURATION_ID,
        PROTECTIVE_WARD_DEFLECTION_BONUS_ID,
        ENERGY_ABSORPTION_ID,
    ] {
        assert!(
            !has_explanation(&computation, id),
            "without the canonical Abjuration specialization choice, '{id}' must not be grounded"
        );
    }
}

#[test]
fn an_evocation_specialist_gains_no_abjuration_school_power_grounding() {
    let input = load(EVOCATION_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        RESISTANCE_ID,
        PROTECTIVE_WARD_USES_PER_DAY_ID,
        PROTECTIVE_WARD_DURATION_ID,
        PROTECTIVE_WARD_DEFLECTION_BONUS_ID,
        ENERGY_ABSORPTION_ID,
    ] {
        assert!(
            !has_explanation(&computation, id),
            "an Evocation specialist must not gain any Abjuration school-power grounding \
             (id: '{id}')"
        );
    }
    // The pre-existing Evocation grounding must still fire, undisturbed.
    assert!(has_explanation(&computation, "class_chassis.wizard.intense_bonus_damage"));
    assert!(has_explanation(&computation, "class_chassis.wizard.force_missile_uses_per_day"));
}
