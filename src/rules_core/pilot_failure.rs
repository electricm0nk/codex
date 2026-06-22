//! GE-06 pilot failure classifier and owner mapping.
//!
//! Maps the merged GE06-E2-F3 headless receipt into one primary failure owner
//! from the bounded GE-06 vocabulary: ModelFlaw, ImporterFlaw, EngineFlaw,
//! OracleGap, or UiGap.
//!
//! The classifier preserves the full vocabulary while classifying only what the
//! merged receipt can currently prove. It refuses IntegrationIssue as a terminal
//! bucket and remains honest about the limited evidence the current receipt carries.

use super::pilot_compute::{HeadlessReceiptStatus, PilotHeadlessReceipt};

/// Primary failure owner categories from the GE-06 doctrine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryOwner {
    /// The canonical model or input contract has a gap or semantic collapse.
    ModelFlaw,
    /// The import pipeline or provenance surface has a gap.
    ImporterFlaw,
    /// The rules-core computation, explanation, or diagnostic surface has a gap.
    EngineFlaw,
    /// The receipt was computed successfully but lacks oracle comparison evidence.
    OracleGap,
    /// The UI or product-visible surface has a gap or missing visibility.
    UiGap,
}

/// Classification result from mapping a receipt into one primary owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureClassification {
    /// The required primary failure owner.
    pub primary_owner: PrimaryOwner,
    /// Optional contributing-owner context or reason when useful.
    pub contributing_context: Option<String>,
}

impl FailureClassification {
    /// Classify a merged headless receipt into one primary failure owner.
    ///
    /// Returns a FailureClassification with one required primary owner and
    /// optional contributing context. The classification is grounded in the
    /// observable receipt state: its status (Computed vs Blocked) plus whether the
    /// receipt preserves claim-blocking diagnostics on the blocked path.
    pub fn classify(receipt: &PilotHeadlessReceipt) -> Self {
        let has_claim_blocking_diagnostics = receipt
            .computation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.claim_blocking);

        match (receipt.status, has_claim_blocking_diagnostics) {
            (HeadlessReceiptStatus::Computed, false) => FailureClassification {
                primary_owner: PrimaryOwner::OracleGap,
                contributing_context: Some(
                    "Computed receipt with no comparison evidence".to_string(),
                ),
            },
            (HeadlessReceiptStatus::Blocked, true) => FailureClassification {
                primary_owner: PrimaryOwner::EngineFlaw,
                contributing_context: Some(
                    "Blocked by claim-blocking rules diagnostics".to_string(),
                ),
            },
            (HeadlessReceiptStatus::Computed, true) => FailureClassification {
                primary_owner: PrimaryOwner::EngineFlaw,
                contributing_context: Some(
                    "Computed receipt carried unexpected claim-blocking diagnostics".to_string(),
                ),
            },
            (HeadlessReceiptStatus::Blocked, false) => FailureClassification {
                primary_owner: PrimaryOwner::EngineFlaw,
                contributing_context: Some(
                    "Blocked receipt without claim-blocking diagnostics violates the receipt contract"
                        .to_string(),
                ),
            },
        }
    }
}
