//! Oracle-harness comparator proof (SD-26 Epic 2, Criterion 2.1).
//!
//! Proves that `oracle_validation::comparator::compare` aligns a normalized
//! old-system (PCGen) oracle output against the new-system (Codex) selected
//! parity dimensions by dimension ID, and correctly classifies agreement,
//! value disagreement, and one-sided dimension presence. Uses the real GE06
//! pilot headless receipt (PF1 CRB Human Fighter level 1) as the Codex side so
//! this exercises the actual `SelectedParityDimensions` shape, not a synthetic
//! stand-in.

use codex::oracle_validation::comparator::{
    compare, MismatchReason, NormalizedDimensionValue, NormalizedOutput,
};
use codex::oracle_validation::selected_parity_dimensions::SelectedParityDimensions;
use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn codex_selected_dimensions() -> SelectedParityDimensions {
    let result = load_character_input_fixture(DETERMINISTIC_FIXTURE);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    let input = result
        .character_input
        .expect("valid fixture should produce a character input record");
    let receipt = build_pilot_headless_receipt(&input);
    SelectedParityDimensions::from_receipt(&receipt)
}

/// A `NormalizedOutput` whose values agree, dimension-for-dimension, with the
/// real pilot receipt's selected dimensions (per `ge06_selected_parity_dimensions.rs`).
fn agreeing_pcgen_output() -> NormalizedOutput {
    NormalizedOutput {
        dim_values: vec![
            numeric("combat.baseline_melee_attack_bonus", 5),
            numeric("defense.baseline_armor_class", 17),
            numeric("defense.total_save.fortitude", 4),
            numeric("defense.total_save.reflex", 2),
            numeric("defense.total_save.will", 1),
            numeric("skill.selected_modifier.climb", 5),
            numeric("skill.selected_modifier.intimidate", 3),
            numeric("skill.selected_modifier.swim", 5),
            text(
                "character.identity",
                "pf1-crb-human-fighter-level1",
            ),
        ],
    }
}

fn numeric(id: &str, value: i16) -> NormalizedDimensionValue {
    NormalizedDimensionValue {
        id: id.to_string(),
        value_string: None,
        value_i16: Some(value),
    }
}

fn text(id: &str, value: &str) -> NormalizedDimensionValue {
    NormalizedDimensionValue {
        id: id.to_string(),
        value_string: Some(value.to_string()),
        value_i16: None,
    }
}

#[test]
fn all_dimensions_agree_yields_no_mismatches() {
    let codex = codex_selected_dimensions();
    let pcgen = agreeing_pcgen_output();

    let result = compare(&pcgen, &codex);

    assert_eq!(result.matches.len(), 9, "expected all 9 dimensions to match");
    assert!(
        result.mismatches.is_empty(),
        "expected no mismatches, got: {:?}",
        result.mismatches
    );
    assert!(result.all_matched());

    let armor_match = result
        .matches
        .iter()
        .find(|m| m.dimension_id == "defense.baseline_armor_class")
        .expect("armor class dimension should be present in matches");
    assert_eq!(armor_match.pcgen_value_i16, Some(17));
    assert_eq!(armor_match.codex_value_i16, Some(17));
}

#[test]
fn a_disagreeing_value_is_reported_as_value_mismatch() {
    let codex = codex_selected_dimensions();
    let mut pcgen = agreeing_pcgen_output();

    // Flip the PCGen armor-class value so it disagrees with Codex's 17.
    let armor = pcgen
        .dim_values
        .iter_mut()
        .find(|d| d.id == "defense.baseline_armor_class")
        .expect("fixture defines armor class dimension");
    armor.value_i16 = Some(99);

    let result = compare(&pcgen, &codex);

    assert_eq!(result.matches.len(), 8, "eight dimensions still agree");
    assert_eq!(result.mismatches.len(), 1, "exactly one dimension disagrees");
    assert!(!result.all_matched());

    let mismatch = &result.mismatches[0];
    assert_eq!(mismatch.dimension_id, "defense.baseline_armor_class");
    assert_eq!(mismatch.pcgen_value_i16, Some(99));
    assert_eq!(mismatch.codex_value_i16, Some(17));
    assert_eq!(mismatch.reason, MismatchReason::ValueMismatch);
}

#[test]
fn a_pcgen_only_dimension_is_reported_as_missing_from_codex() {
    let codex = codex_selected_dimensions();
    let mut pcgen = agreeing_pcgen_output();
    pcgen
        .dim_values
        .push(numeric("combat.baseline_ranged_attack_bonus", 5));

    let result = compare(&pcgen, &codex);

    assert_eq!(result.matches.len(), 9);
    assert_eq!(result.mismatches.len(), 1);

    let mismatch = &result.mismatches[0];
    assert_eq!(mismatch.dimension_id, "combat.baseline_ranged_attack_bonus");
    assert_eq!(mismatch.pcgen_value_i16, Some(5));
    assert_eq!(mismatch.codex_value_i16, None);
    assert_eq!(mismatch.reason, MismatchReason::MissingFromCodex);
}

#[test]
fn a_codex_only_dimension_is_reported_as_missing_from_pcgen() {
    let codex = codex_selected_dimensions();
    let mut pcgen = agreeing_pcgen_output();
    pcgen
        .dim_values
        .retain(|d| d.id != "skill.selected_modifier.swim");

    let result = compare(&pcgen, &codex);

    assert_eq!(result.matches.len(), 8);
    assert_eq!(result.mismatches.len(), 1);

    let mismatch = &result.mismatches[0];
    assert_eq!(mismatch.dimension_id, "skill.selected_modifier.swim");
    assert_eq!(mismatch.pcgen_value_i16, None);
    assert_eq!(mismatch.codex_value_i16, Some(5));
    assert_eq!(mismatch.reason, MismatchReason::MissingFromPcgen);
}
