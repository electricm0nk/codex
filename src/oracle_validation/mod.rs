//! Oracle-validation and parity-harness surface (GE-05 / SD-26 Epic 2).
//!
//! Exposes the GE05-E2-F1 golden-case fixture schema, the GE06-E3-F1 selected
//! parity-dimension adapter, the Oracle-Harness comparator (SD-26 Criterion
//! 2.1), and the normalization-rule engine (SD-26 Criterion 2.2).
//! Parity-report rendering and the PCGen runner (SD-26 Criteria 2.3-2.4)
//! remain out of scope and land in later SD-26 Epic 2 cycles.

pub mod comparator;
pub mod golden_fixture;
pub mod normalization;
pub mod selected_parity_dimensions;
