//! GE-06 pilot view-model contract.
//!
//! Projects the merged GE-06 pilot headless receipt into a machine-checkable
//! UI-consumer boundary. This adapter preserves pilot identity, real computed
//! snapshot values when the receipt is Computed, explicit blocked posture with
//! real diagnostics when the receipt is Blocked, the primary failure owner from
//! the classifier, and explanation payloads or stable references for surfaced
//! values.

use super::pilot_compute::{
    AbilityModifiers, BaseSaves, ComputationDiagnostic, ComputationExplanation,
    HeadlessReceiptStatus, PilotBaseChassisComputation, PilotHeadlessReceipt,
    SelectedSkillModifiers,
};
use super::pilot_failure::{FailureClassifier, PrimaryOwner};

/// The bounded pilot view-model contract for UI consumption.
///
/// This surface is the projection from the headless receipt into a shape
/// suitable for UI consumers. It preserves real identity, status, and diagnostics.
/// When Computed, it includes a snapshot of the deterministic values. When Blocked,
/// it explicitly refuses to emit a faux success snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotViewModel {
    /// Pilot case identity from the real receipt.
    pub case_id: Option<String>,
    /// Source package identity from the real receipt.
    pub source_package_id: String,
    /// Status: whether the integrated path produced evidence (Computed) or is blocked.
    pub status: HeadlessReceiptStatus,
    /// Primary failure owner from the classifier lane.
    pub primary_owner: PrimaryOwner,
    /// Real snapshot of computed values. Present only when status is Computed.
    /// Explicitly None when Blocked to refuse faux success values.
    pub snapshot: Option<PilotViewModelSnapshot>,
    /// Claim-blocking diagnostics from the receipt. Preserved regardless of status.
    pub diagnostics: Vec<ComputationDiagnostic>,
}

/// The bounded snapshot of deterministic computed values from the receipt.
///
/// This is emitted only when the receipt is Computed. It preserves all
/// real values, explanations, and non-blocking diagnostics from the underlying
/// computation without invention or fabrication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotViewModelSnapshot {
    /// Ability modifiers from the computation.
    pub ability_modifiers: AbilityModifiers,
    /// Class base attack bonus from the computation.
    pub base_attack_bonus: i16,
    /// Class base saves from the computation.
    pub base_saves: BaseSaves,
    /// Baseline melee attack bonus for the deterministic loadout.
    pub baseline_melee_attack_bonus: i16,
    /// Baseline armor class for the deterministic posture.
    pub baseline_armor_class: i16,
    /// Total saving throws (base + ability modifiers).
    pub total_saves: BaseSaves,
    /// Selected deterministic skill modifiers.
    pub selected_skill_modifiers: SelectedSkillModifiers,
    /// Explanation records for the computed values.
    pub explanations: Vec<ComputationExplanation>,
    /// Non-claim-blocking diagnostics from the computation.
    pub diagnostics: Vec<ComputationDiagnostic>,
}

/// Blocked view-model state (placeholder for future use).
/// Currently unused but reserved for explicit blocked-posture projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotViewModelBlocked {
    pub primary_owner: PrimaryOwner,
    pub diagnostics: Vec<ComputationDiagnostic>,
}

impl PilotViewModel {
    /// Project the real headless receipt into the view-model contract.
    ///
    /// This adapter preserves case identity, source package, status, and primary owner.
    /// When the receipt is Computed, it emits a real snapshot. When Blocked, it
    /// preserves explicit blocked posture and diagnostics without inventing a
    /// success snapshot.
    pub fn from_receipt(
        receipt: &PilotHeadlessReceipt,
        classifier: &FailureClassifier,
    ) -> Self {
        let primary_owner = classifier.primary_owner();
        let status = receipt.status;

        // Build snapshot only when Computed. Explicitly refuse faux values when Blocked.
        let snapshot = match status {
            HeadlessReceiptStatus::Computed => Some(Self::build_snapshot(&receipt.computation)),
            HeadlessReceiptStatus::Blocked => None,
        };

        Self {
            case_id: receipt.case_id.clone(),
            source_package_id: receipt.source_package_id.clone(),
            status,
            primary_owner,
            snapshot,
            diagnostics: receipt.computation.diagnostics.clone(),
        }
    }

    /// Build the snapshot from a computed receipt.
    fn build_snapshot(computation: &PilotBaseChassisComputation) -> PilotViewModelSnapshot {
        PilotViewModelSnapshot {
            ability_modifiers: computation.ability_modifiers,
            base_attack_bonus: computation.base_attack_bonus,
            base_saves: computation.base_saves,
            baseline_melee_attack_bonus: computation.baseline_melee_attack_bonus,
            baseline_armor_class: computation.baseline_armor_class,
            total_saves: computation.total_saves,
            selected_skill_modifiers: computation.selected_skill_modifiers,
            explanations: computation.explanations.clone(),
            diagnostics: computation
                .diagnostics
                .iter()
                .filter(|d| !d.claim_blocking)
                .cloned()
                .collect(),
        }
    }
}
