//! Oracle-harness comparator (SD-26 Epic 2, Criterion 2.1).
//!
//! Compares a normalized old-system (PCGen) oracle output against the
//! new-system (Codex) selected parity dimensions and reports, per dimension,
//! whether the two agree. This is the first piece of the Oracle-Harness
//! Comparator described in `technical-design.md §2`; normalization-rule
//! authoring (Criterion 2.2, `normalization.rs`), parity-report rendering
//! (2.3, `parity_report.rs`), the PCGen runner (2.4, `pcgen_runner.rs`), and the
//! pilot-case verification cycle (2.5) all build on this module's public
//! surface in later cycles. Keep `NormalizedOutput`, `NormalizedDimensionValue`,
//! `compare`, `ComparisonResult`, `DimensionMatch`, `DimensionMismatch`, and
//! `MismatchReason` stable — later cycles depend on them.
//!
//! Concurrency: serial (per `epic-breakdown.md` Criterion 2.1).
//!
//! This slice applies direct value equality only (exact match on `value_i16`,
//! exact match on `value_string`). Criterion 2.2's normalization-rule engine
//! will refine what "matches" means (e.g. trimming whitespace, integer
//! coercion) as a pre-comparison normalization step on the PCGen side; it does
//! not need to change this function's signature or the shape of
//! `ComparisonResult`.

use crate::oracle_validation::selected_parity_dimensions::{
    SelectedDimension, SelectedParityDimensions,
};

/// A single normalized old-system (PCGen) oracle dimension value, already
/// reduced to the same string/int value shape [`SelectedDimension`] uses so the
/// comparator can align dimensions by ID without a schema-translation step.
///
/// This slice (Criterion 2.1) does not itself normalize raw PCGen output —
/// that rule engine is Criterion 2.2 (`normalization.rs`). Callers construct
/// `NormalizedOutput` from already-normalized values (e.g. a PCGen-runner
/// capture, Criterion 2.4, that has already had whitespace/format
/// normalization applied by Criterion 2.2's rule engine).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedDimensionValue {
    /// Stable dimension ID, matching the IDs `SelectedParityDimensions` uses
    /// (e.g. `combat.baseline_melee_attack_bonus`).
    pub id: String,
    /// String-typed value (used for identity dimensions).
    pub value_string: Option<String>,
    /// Numeric value (used for computed output dimensions).
    pub value_i16: Option<i16>,
}

/// A normalized old-system (PCGen) oracle output for one case: the full set of
/// per-dimension values the comparator aligns against the Codex selected
/// dimensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedOutput {
    pub dim_values: Vec<NormalizedDimensionValue>,
}

/// One dimension where the PCGen and Codex values agreed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimensionMatch {
    pub dimension_id: String,
    pub pcgen_value_string: Option<String>,
    pub pcgen_value_i16: Option<i16>,
    pub codex_value_string: Option<String>,
    pub codex_value_i16: Option<i16>,
}

/// Why a dimension did not match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MismatchReason {
    /// Both sides had the dimension but the values differed.
    ValueMismatch,
    /// The PCGen normalized output had the dimension but Codex's selected
    /// dimensions did not.
    MissingFromCodex,
    /// Codex's selected dimensions had the dimension but the PCGen normalized
    /// output did not.
    MissingFromPcgen,
}

/// One dimension where the PCGen and Codex values disagreed, or one side was
/// missing the dimension entirely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimensionMismatch {
    pub dimension_id: String,
    pub pcgen_value_string: Option<String>,
    pub pcgen_value_i16: Option<i16>,
    pub codex_value_string: Option<String>,
    pub codex_value_i16: Option<i16>,
    pub reason: MismatchReason,
}

/// Result of comparing a normalized PCGen oracle output against Codex's
/// selected parity dimensions for one case.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ComparisonResult {
    pub matches: Vec<DimensionMatch>,
    pub mismatches: Vec<DimensionMismatch>,
}

impl ComparisonResult {
    /// True when every aligned dimension matched and neither side had a
    /// dimension the other lacked.
    pub fn all_matched(&self) -> bool {
        self.mismatches.is_empty()
    }
}

/// Compares a normalized PCGen oracle output against Codex's selected parity
/// dimensions, aligning by dimension ID.
///
/// Criterion 2.1's public surface: `NormalizedOutput`, `SelectedParityDimensions`,
/// `compare`, and `ComparisonResult` are the stable contract later Epic 2
/// cycles build against (2.2 normalization, 2.3 parity-report, 2.4 PCGen
/// runner, 2.5 pilot verification).
pub fn compare(
    canon_pcg: &NormalizedOutput,
    codex: &SelectedParityDimensions,
) -> ComparisonResult {
    let mut matches = Vec::new();
    let mut mismatches = Vec::new();
    let mut matched_codex_ids: Vec<&str> = Vec::new();

    for pcgen_dim in &canon_pcg.dim_values {
        match codex.dimensions.iter().find(|c| c.id == pcgen_dim.id) {
            Some(codex_dim) => {
                matched_codex_ids.push(codex_dim.id.as_str());
                if values_match(pcgen_dim, codex_dim) {
                    matches.push(DimensionMatch {
                        dimension_id: pcgen_dim.id.clone(),
                        pcgen_value_string: pcgen_dim.value_string.clone(),
                        pcgen_value_i16: pcgen_dim.value_i16,
                        codex_value_string: codex_dim.value_string.clone(),
                        codex_value_i16: codex_dim.value_i16,
                    });
                } else {
                    mismatches.push(DimensionMismatch {
                        dimension_id: pcgen_dim.id.clone(),
                        pcgen_value_string: pcgen_dim.value_string.clone(),
                        pcgen_value_i16: pcgen_dim.value_i16,
                        codex_value_string: codex_dim.value_string.clone(),
                        codex_value_i16: codex_dim.value_i16,
                        reason: MismatchReason::ValueMismatch,
                    });
                }
            }
            None => {
                mismatches.push(DimensionMismatch {
                    dimension_id: pcgen_dim.id.clone(),
                    pcgen_value_string: pcgen_dim.value_string.clone(),
                    pcgen_value_i16: pcgen_dim.value_i16,
                    codex_value_string: None,
                    codex_value_i16: None,
                    reason: MismatchReason::MissingFromCodex,
                });
            }
        }
    }

    for codex_dim in &codex.dimensions {
        if !matched_codex_ids.contains(&codex_dim.id.as_str()) {
            mismatches.push(DimensionMismatch {
                dimension_id: codex_dim.id.clone(),
                pcgen_value_string: None,
                pcgen_value_i16: None,
                codex_value_string: codex_dim.value_string.clone(),
                codex_value_i16: codex_dim.value_i16,
                reason: MismatchReason::MissingFromPcgen,
            });
        }
    }

    ComparisonResult { matches, mismatches }
}

/// Direct value equality between a PCGen dimension value and its aligned
/// Codex dimension value. See the module-level doc comment: this is
/// intentionally exact-equality only; Criterion 2.2 owns normalization.
fn values_match(pcgen: &NormalizedDimensionValue, codex: &SelectedDimension) -> bool {
    pcgen.value_i16 == codex.value_i16 && pcgen.value_string == codex.value_string
}
