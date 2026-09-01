//! Sorcerer Draconic Bloodline "Dragon Resistances" (3rd-level bloodline power)
//! magnitude grounding.
//!
//! Corpus source: pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/
//! cr_abilities_class.lst, KEY:Draconic Bloodline ~ Dragon Resistances, gated
//! PREVARGTEQ:Sorcerer_Draconic_BloodlineProgressionLVL,3. Two separate corpus
//! `BONUS:VAR` lines both feed the same `Sorcerer_DraconicDragonResistances_
//! NaturalArmorBonus` variable and accumulate ADDITIVELY (a real, repeated PCGen
//! corpus pattern, not a single-line override):
//!   BONUS:VAR|...NaturalArmorBonus|min(floor((LVL-3)/6)+1,3)
//!   BONUS:VAR|...NaturalArmorBonus|1|PREVARGTEQ:...,15
//! so the natural armor total is 1 at level 3, 2 at level 9, and 4 (3 capped + 1
//! additional) at level 15 and higher -- NOT 3, which grounding only the first line
//! would (incorrectly) yield. The energy resistance bonus is a separate variable:
//! `min(floor((LVL-3)/6)+1,2)*5` -- 5 at level 3, 10 at level 9 and higher.
//!
//! Both magnitudes are grounded as type-agnostic facts: the numbers are identical
//! regardless of which energy type (acid/cold/electricity/fire) the Draconic
//! bloodline's dragon type ultimately names, so this seam deliberately does not pick
//! a canonical energy type -- that label is a deferred, unresolved sub-choice, named
//! by a non-claim-blocking diagnostic.
//!
//! Recognized only for the canonical deterministic Draconic bloodline selection
//! (`choice:sorcerer_bloodline -> bloodline:draconic`), mirroring exactly how the
//! Arcane bloodline's own class-skill grant
//! (`tests/sd13_sorcerer_bloodline_class_skill_choice.rs`) is only recognized when
//! the Arcane bloodline itself was the recognized selection.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
mod common;
use common::load;

const SORCERER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");
const SORCERER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level3_sd13_deterministic_input.txt");
const SORCERER_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level9_sd13_deterministic_input.txt");
const SORCERER_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level15_sd18_widening_deterministic_input.txt"
);
const SORCERER_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level20_sd18_widening_deterministic_input.txt"
);

const NATURAL_ARMOR_EXPLANATION_ID: &str =
    "class_feature.sorcerer.draconic_bloodline.dragon_resistances.natural_armor_bonus";
const RESISTANCE_EXPLANATION_ID: &str =
    "class_feature.sorcerer.draconic_bloodline.dragon_resistances.resistance_bonus";
const ENERGY_TYPE_UNRESOLVED_DIAGNOSTIC_ID: &str =
    "class_feature.sorcerer.draconic_bloodline.dragon_resistances.energy_type_unresolved";

fn draconic(fixture: &str) -> String {
    fixture.replace("bloodline:arcane", "bloodline:draconic")
}

fn find_explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> Option<&'a ComputationExplanation> {
    computation.explanations.iter().find(|e| e.id == id)
}

fn find_diagnostic<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> Option<&'a ComputationDiagnostic> {
    computation.diagnostics.iter().find(|d| d.id == id)
}

fn assert_dragon_resistances(fixture: &str, expected_natural_armor: i16, expected_resistance: i16) {
    let input = load(&draconic(fixture));
    let computation = compute_pilot_base_chassis(&input);

    let natural_armor = find_explanation(&computation, NATURAL_ARMOR_EXPLANATION_ID)
        .expect("Dragon Resistances natural armor bonus must be grounded for a Draconic bloodline Sorcerer");
    assert_eq!(
        natural_armor.value, expected_natural_armor,
        "natural armor bonus mismatch: {}",
        natural_armor.detail
    );

    let resistance = find_explanation(&computation, RESISTANCE_EXPLANATION_ID)
        .expect("Dragon Resistances energy resistance bonus must be grounded for a Draconic bloodline Sorcerer");
    assert_eq!(
        resistance.value, expected_resistance,
        "resistance bonus mismatch: {}",
        resistance.detail
    );

    let energy_type_unresolved = find_diagnostic(&computation, ENERGY_TYPE_UNRESOLVED_DIAGNOSTIC_ID)
        .expect("the energy-type sub-choice must be named as deferred/unresolved");
    assert!(
        !energy_type_unresolved.claim_blocking,
        "the deferred energy-type diagnostic must not be claim-blocking, since the two \
         magnitudes are otherwise fully grounded: {energy_type_unresolved:?}"
    );
    for damage_type in ["acid", "cold", "electricity", "fire"] {
        assert!(
            !natural_armor.detail.to_lowercase().contains(damage_type)
                && !resistance.detail.to_lowercase().contains(damage_type),
            "grounded magnitude records must not pick a canonical energy type: {} / {}",
            natural_armor.detail,
            resistance.detail
        );
    }
}

#[test]
fn dragon_resistances_at_level_3_is_1_natural_armor_and_5_resistance() {
    assert_dragon_resistances(SORCERER_LEVEL3_FIXTURE, 1, 5);
}

#[test]
fn dragon_resistances_at_level_9_is_2_natural_armor_and_10_resistance() {
    assert_dragon_resistances(SORCERER_LEVEL9_FIXTURE, 2, 10);
}

#[test]
fn dragon_resistances_at_level_15_is_4_natural_armor_and_10_resistance() {
    // Level 15 is the case that would be WRONG (3, not 4) if only the first
    // BONUS:VAR|...NaturalArmorBonus|min(floor((LVL-3)/6)+1,3) line were grounded
    // without also grounding the second, additively-stacking
    // BONUS:VAR|...NaturalArmorBonus|1|PREVARGTEQ:...,15 line.
    assert_dragon_resistances(SORCERER_LEVEL15_FIXTURE, 4, 10);
}

#[test]
fn dragon_resistances_at_level_20_stays_4_natural_armor_and_10_resistance() {
    assert_dragon_resistances(SORCERER_LEVEL20_FIXTURE, 4, 10);
}

#[test]
fn dragon_resistances_below_level_3_is_a_correct_level_gate_absence() {
    // Draconic Bloodline itself is chosen at level 1, but Dragon Resistances is a
    // 3rd-level bloodline power. Below the gate both magnitudes must still be
    // reported, at value 0, as a level-gate absence -- mirroring the Barbarian Trap
    // Sense / Damage Reduction two-branch idiom already used elsewhere in this file.
    assert_dragon_resistances(SORCERER_LEVEL1_FIXTURE, 0, 0);
}

#[test]
fn dragon_resistances_is_not_fabricated_for_a_non_draconic_bloodline() {
    // The canonical level-3 fixture's default bloodline selection is Arcane, not
    // Draconic. No Draconic-specific grant may appear.
    let input = load(SORCERER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        find_explanation(&computation, NATURAL_ARMOR_EXPLANATION_ID).is_none(),
        "Dragon Resistances natural armor bonus must not be fabricated for the Arcane bloodline"
    );
    assert!(
        find_explanation(&computation, RESISTANCE_EXPLANATION_ID).is_none(),
        "Dragon Resistances resistance bonus must not be fabricated for the Arcane bloodline"
    );
}

#[test]
fn dragon_resistances_is_not_fabricated_when_no_bloodline_choice_is_present() {
    let fixture: String = SORCERER_LEVEL3_FIXTURE
        .lines()
        .filter(|line| !line.starts_with("choice=choice:sorcerer_bloodline:"))
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        find_explanation(&computation, NATURAL_ARMOR_EXPLANATION_ID).is_none(),
        "Dragon Resistances natural armor bonus must not be fabricated with no bloodline choice"
    );
    assert!(
        find_explanation(&computation, RESISTANCE_EXPLANATION_ID).is_none(),
        "Dragon Resistances resistance bonus must not be fabricated with no bloodline choice"
    );
}
