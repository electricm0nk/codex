//! Oracle-harness normalization-rule engine (SD-26 Epic 2, Criterion 2.2).
//!
//! Reduces a raw old-system (PCGen) text capture into the
//! [`NormalizedOutput`](crate::oracle_validation::comparator::NormalizedOutput)
//! shape Criterion 2.1's comparator already consumes. This is a
//! pre-comparison step on the PCGen side only: it does not change
//! `compare`'s signature or the shape of `ComparisonResult` (see
//! `comparator.rs`'s module doc and `technical-design.md §2.2`).
//!
//! Rule set, per `technical-design.md §2.2`:
//! - `trailing-whitespace-strip` — trims leading/trailing whitespace from the
//!   raw text value.
//! - `integer-coercion` — parses the (already-trimmed) text value as an
//!   `i16` when possible, promoting it from a string value to a numeric one
//!   (e.g. PCGen's spell-level field `"1"` normalizes to integer `1`).
//!
//! Rules apply in the order given, threading a working `(value_string,
//! value_i16)` pair through each rule so later rules see earlier rules'
//! output (e.g. integer-coercion parses the already-trimmed string).
//!
//! Later Epic 2 cycles that build on this module: 2.3 (`parity_report.rs`)
//! renders which rules applied per dimension; 2.4 (`pcgen_runner.rs`) is the
//! expected producer of [`RawPcgenOutput`] from an actual PCGen invocation.

use crate::oracle_validation::comparator::{NormalizedDimensionValue, NormalizedOutput};

/// One raw, not-yet-normalized old-system (PCGen) dimension value, as
/// captured verbatim from PCGen's text output (e.g. by Criterion 2.4's
/// runner) before any normalization rule has run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawDimensionValue {
    /// Stable dimension ID, matching the IDs used elsewhere in
    /// `oracle_validation` (e.g. `combat.baseline_melee_attack_bonus`).
    pub id: String,
    /// The raw text value as captured from PCGen, unmodified.
    pub raw_value: String,
}

/// A raw old-system (PCGen) oracle output for one case: the full set of
/// per-dimension raw text values to be normalized before comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawPcgenOutput {
    pub dim_values: Vec<RawDimensionValue>,
}

/// One normalization rule's behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationRuleKind {
    /// Trims leading/trailing whitespace from the string value.
    TrimWhitespace,
    /// Parses the string value as an `i16`; on success, promotes the value
    /// to numeric (clearing the string value) and leaves it unchanged
    /// otherwise.
    IntegerCoercion,
}

/// One named normalization rule. The `id` is the stable identifier the
/// parity report (Criterion 2.3) cites (e.g. `normalization.rs:N`'s
/// `trailing-whitespace-strip` / `integer-coercion` rows).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizationRule {
    pub id: String,
    pub kind: NormalizationRuleKind,
}

/// The default normalization rule set (`technical-design.md §2.2`), applied
/// in order: trim whitespace, then coerce integers.
pub fn default_normalization_rules() -> Vec<NormalizationRule> {
    vec![
        NormalizationRule {
            id: "trailing-whitespace-strip".to_string(),
            kind: NormalizationRuleKind::TrimWhitespace,
        },
        NormalizationRule {
            id: "integer-coercion".to_string(),
            kind: NormalizationRuleKind::IntegerCoercion,
        },
    ]
}

/// Applies `rules`, in order, to one raw dimension value, producing the
/// normalized value the comparator aligns by ID.
pub fn normalize_dimension_value(
    raw: &RawDimensionValue,
    rules: &[NormalizationRule],
) -> NormalizedDimensionValue {
    let mut value_string = Some(raw.raw_value.clone());
    let mut value_i16: Option<i16> = None;

    for rule in rules {
        match rule.kind {
            NormalizationRuleKind::TrimWhitespace => {
                if let Some(s) = value_string.as_mut() {
                    let trimmed = s.trim();
                    if trimmed.len() != s.len() {
                        *s = trimmed.to_string();
                    }
                }
            }
            NormalizationRuleKind::IntegerCoercion => {
                if let Some(s) = value_string.as_ref()
                    && let Ok(parsed) = s.parse::<i16>() {
                        value_i16 = Some(parsed);
                        value_string = None;
                    }
            }
        }
    }

    NormalizedDimensionValue {
        id: raw.id.clone(),
        value_string,
        value_i16,
    }
}

/// Applies `rules` to every dimension in a raw PCGen output, producing the
/// full [`NormalizedOutput`] Criterion 2.1's `compare` expects.
pub fn normalize(raw: &RawPcgenOutput, rules: &[NormalizationRule]) -> NormalizedOutput {
    NormalizedOutput {
        dim_values: raw
            .dim_values
            .iter()
            .map(|dim| normalize_dimension_value(dim, rules))
            .collect(),
    }
}
