//! Oracle-validation and parity-harness surface (GE-05 / SD-26 Epic 2).
//!
//! Exposes the GE05-E2-F1 golden-case fixture schema, the GE06-E3-F1 selected
//! parity-dimension adapter, the Oracle-Harness comparator (SD-26 Criterion
//! 2.1), the normalization-rule engine (SD-26 Criterion 2.2), the parity
//! report writer (SD-26 Criterion 2.3), and the Rust-side PCGen runner
//! wrapper (SD-26 Criterion 2.4).

pub mod comparator;
pub mod golden_fixture;
pub mod normalization;
pub mod parity_report;
pub mod pcgen_runner;
/// SD-35 `AT-35-E6-001`: the `kind=race_trait` FORMULA bar check, moved off the
/// live side (`decisions.md` §11).
pub mod race_trait_formula_bar_check;
pub mod selected_parity_dimensions;
