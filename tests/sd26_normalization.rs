//! Oracle-harness normalization-rule engine proof (SD-26 Epic 2, Criterion 2.2).
//!
//! Proves that `oracle_validation::normalization` reduces raw PCGen text
//! captures into the `NormalizedOutput` shape Criterion 2.1's comparator
//! expects, applying the documented default rule set (`technical-design.md
//! §2.2`): trailing-whitespace-strip, then integer-coercion. Also proves the
//! engine composes end-to-end with `comparator::compare`, since normalization
//! is a pre-comparison step on the PCGen side, not a change to the
//! comparator's signature or `ComparisonResult` shape.

use codex::oracle_validation::comparator::{compare, MismatchReason};
use codex::oracle_validation::normalization::{
    default_normalization_rules, normalize, normalize_dimension_value, NormalizationRule,
    NormalizationRuleKind, RawDimensionValue, RawPcgenOutput,
};
use codex::oracle_validation::selected_parity_dimensions::{
    ClaimTierFloor, SelectedDimension, SelectedParityDimensions,
};

fn raw(id: &str, value: &str) -> RawDimensionValue {
    RawDimensionValue {
        id: id.to_string(),
        raw_value: value.to_string(),
    }
}

#[test]
fn trim_whitespace_rule_strips_leading_and_trailing_whitespace() {
    let rules = vec![NormalizationRule {
        id: "trailing-whitespace-strip".to_string(),
        kind: NormalizationRuleKind::TrimWhitespace,
    }];

    let normalized = normalize_dimension_value(&raw("spell.name", "  Fireball  "), &rules);

    assert_eq!(normalized.value_string, Some("Fireball".to_string()));
    assert_eq!(normalized.value_i16, None);
}

#[test]
fn integer_coercion_rule_converts_numeric_strings_to_i16() {
    let rules = vec![NormalizationRule {
        id: "integer-coercion".to_string(),
        kind: NormalizationRuleKind::IntegerCoercion,
    }];

    let normalized = normalize_dimension_value(&raw("spell.level", "1"), &rules);

    assert_eq!(normalized.value_i16, Some(1));
    assert_eq!(
        normalized.value_string, None,
        "a value promoted to i16 should not also carry a string value"
    );
}

#[test]
fn default_rules_apply_trim_before_integer_coercion() {
    let rules = default_normalization_rules();

    let normalized = normalize_dimension_value(&raw("defense.baseline_armor_class", "  17  "), &rules);

    assert_eq!(
        normalized.value_i16,
        Some(17),
        "trimming must happen before integer parsing or the surrounding whitespace \
         would make the numeric parse fail"
    );
}

#[test]
fn non_numeric_value_remains_a_trimmed_string_after_default_rules() {
    let rules = default_normalization_rules();

    let normalized = normalize_dimension_value(&raw("character.identity", "  pf1-crb-human-fighter-level1  "), &rules);

    assert_eq!(
        normalized.value_string,
        Some("pf1-crb-human-fighter-level1".to_string())
    );
    assert_eq!(normalized.value_i16, None);
}

#[test]
fn empty_rule_set_leaves_the_raw_string_untouched() {
    let normalized = normalize_dimension_value(&raw("spell.name", "  Fireball  "), &[]);

    assert_eq!(normalized.value_string, Some("  Fireball  ".to_string()));
    assert_eq!(normalized.value_i16, None);
}

#[test]
fn normalize_reduces_a_full_raw_output_dimension_for_dimension() {
    let raw_output = RawPcgenOutput {
        dim_values: vec![
            raw("defense.baseline_armor_class", " 17 "),
            raw("character.identity", " pf1-crb-human-fighter-level1 "),
        ],
    };

    let normalized = normalize(&raw_output, &default_normalization_rules());

    assert_eq!(normalized.dim_values.len(), 2);
    let armor = normalized
        .dim_values
        .iter()
        .find(|d| d.id == "defense.baseline_armor_class")
        .expect("armor dimension present");
    assert_eq!(armor.value_i16, Some(17));

    let identity = normalized
        .dim_values
        .iter()
        .find(|d| d.id == "character.identity")
        .expect("identity dimension present");
    assert_eq!(
        identity.value_string,
        Some("pf1-crb-human-fighter-level1".to_string())
    );
}

#[test]
fn normalized_raw_output_composes_end_to_end_with_the_comparator() {
    // A raw PCGen capture with untrimmed whitespace around a numeric value that
    // would fail an exact-equality comparison against Codex's already-clean
    // i16 value if normalization were skipped.
    let raw_output = RawPcgenOutput {
        dim_values: vec![raw("defense.baseline_armor_class", " 17 ")],
    };
    let normalized = normalize(&raw_output, &default_normalization_rules());

    let codex = SelectedParityDimensions {
        dimensions: vec![SelectedDimension {
            id: "defense.baseline_armor_class".to_string(),
            value_string: None,
            value_i16: Some(17),
            source_package_id: "pf1.core_rulebook".to_string(),
        }],
        claim_tier_floor: ClaimTierFloor::Computed,
    };

    let result = compare(&normalized, &codex);

    assert!(
        result.all_matched(),
        "normalized raw PCGen output should agree with Codex's clean value, got: {:?}",
        result.mismatches
    );
    assert!(result.mismatches.iter().all(|m| m.reason != MismatchReason::ValueMismatch));
}
