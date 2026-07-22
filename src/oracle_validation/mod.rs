//! Oracle-validation and parity-harness surface (GE-05 / SD-26 Epic 2).
//!
//! Exposes the GE05-E2-F1 golden-case fixture schema, the GE06-E3-F1 selected
//! parity-dimension adapter, the Oracle-Harness comparator (SD-26 Criterion
//! 2.1), the normalization-rule engine (SD-26 Criterion 2.2), and the
//! Rust-side PCGen runner wrapper (SD-26 Criterion 2.4). Parity-report
//! rendering (SD-26 Criterion 2.3) remains out of scope and lands in a later
//! SD-26 Epic 2 cycle.

pub mod comparator;
pub mod golden_fixture;
pub mod normalization;
pub mod pcgen_runner;
pub mod selected_parity_dimensions;
