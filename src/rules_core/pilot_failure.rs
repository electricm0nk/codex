//! GE-06 primary failure-owner classifier over the merged headless receipt.
//!
//! Maps the GE06-E2-F3 receipt (computed status + claim-blocking diagnostics) into
//! one required primary failure owner: model flaw, importer flaw, engine flaw,
//! oracle gap, or UI gap.
//!
//! This classifier remains honest about what the merged receipt can currently prove.
//! It does not invent broader telemetry or a general incident framework; it only
//! classifies the receipt states the deterministic path can truthfully support.

use super::pilot_compute::{HeadlessReceiptStatus, PilotHeadlessReceipt};

/// The five primary failure owners required by GE-06 doctrine. This is the required
/// vocabulary for classifying integrated failures; there is no terminal
/// `IntegrationIssue` sink.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryOwner {
    /// A flaw in the canonical model itself: the pilot cannot be represented
    /// without semantic collapse even before import or computation.
    ModelFlaw,
    /// A flaw in the importer: the source content exists but cannot be converted
    /// to canonical form with full provenance.
    ImporterFlaw,
    /// A flaw in the rules engine: canonical content exists but derived values,
    /// explanations, or required computations are wrong or missing.
    EngineFlaw,
    /// An oracle gap: computed outputs exist but selected old-vs-new comparison
    /// evidence is absent or not yet claimed.
    OracleGap,
    /// A UI gap: headless outputs exist but the UI hides diagnostics, depends on
    /// mock state, or otherwise fails to expose the real domain behavior.
    UiGap,
}

/// Classifier that maps the merged headless receipt into one primary failure owner.
/// The classification respects the first-broken-contract rule: it identifies the
/// earliest point in the integrated path where a contract failed, not the last
/// visible symptom.
pub struct FailureClassifier<'a> {
    receipt: &'a PilotHeadlessReceipt,
}

impl<'a> FailureClassifier<'a> {
    /// Create a new classifier over the given receipt.
    pub fn new(receipt: &'a PilotHeadlessReceipt) -> Self {
        Self { receipt }
    }

    /// Classify the receipt into one required primary owner.
    ///
    /// The first implementation remains honest about what the merged receipt can
    /// currently prove:
    /// - Computed receipt (no claim-blocking diagnostics): OracleGap
    ///   (we have outputs but no oracle comparison yet)
    /// - Blocked receipt (has claim-blocking diagnostics): EngineFlaw
    ///   (the engine failed to compute the required outputs)
    ///
    /// As the receipt surface grows to carry importer, model, and UI signals,
    /// later implementations can classify other owner categories while preserving
    /// the same interface.
    pub fn primary_owner(&self) -> PrimaryOwner {
        match self.receipt.status {
            HeadlessReceiptStatus::Computed => PrimaryOwner::OracleGap,
            HeadlessReceiptStatus::Blocked => PrimaryOwner::EngineFlaw,
        }
    }
}
