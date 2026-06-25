//! GE-06 pilot view-model contract over the merged headless receipt.
//!
//! Projects the bounded rules-core receipt into a machine-checkable UI-consumer
//! surface that preserves real computed values when available and explicit blocked
//! posture plus diagnostics when computation is blocked. This adapter adds no UI,
//! no transport, and no new rules truth.

use super::pilot_compute::{
    AbilityModifiers, BaseSaves, ComputationDiagnostic, ComputationExplanation,
    HeadlessReceiptStatus, PilotHeadlessReceipt, SelectedSkillModifiers,
};
use super::pilot_failure::{FailureClassifier, PrimaryOwner};

/// Bounded GE-06 pilot view model derived from the merged headless receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotViewModel {
    pub case_id: Option<String>,
    pub source_package_id: String,
    pub status: HeadlessReceiptStatus,
    pub primary_owner: PrimaryOwner,
    pub snapshot: Option<PilotSnapshot>,
    pub explanations: Vec<ComputationExplanation>,
    pub diagnostics: Vec<ComputationDiagnostic>,
}

/// Computed pilot snapshot emitted only when the receipt is `Computed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotSnapshot {
    pub ability_modifiers: AbilityModifiers,
    pub base_attack_bonus: i16,
    pub base_saves: BaseSaves,
    pub combat: PilotCombatViewModel,
    pub defense: PilotDefenseViewModel,
    pub skill: PilotSkillViewModel,
}

/// Bounded combat surface for the pilot view model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotCombatViewModel {
    pub baseline_melee_attack_bonus: i16,
}

/// Bounded defense surface for the pilot view model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotDefenseViewModel {
    pub baseline_armor_class: i16,
    pub total_save: BaseSaves,
}

/// Bounded skill surface for the pilot view model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotSkillViewModel {
    pub selected_modifier: SelectedSkillModifiers,
}

impl PilotViewModel {
    /// Project the merged headless receipt into the bounded pilot view-model contract.
    pub fn from_receipt(receipt: &PilotHeadlessReceipt) -> Self {
        let primary_owner = FailureClassifier::new(receipt).primary_owner();
        let snapshot = match receipt.status {
            HeadlessReceiptStatus::Computed => Some(PilotSnapshot::from_receipt(receipt)),
            HeadlessReceiptStatus::Blocked => None,
        };

        Self {
            case_id: receipt.case_id.clone(),
            source_package_id: receipt.source_package_id.clone(),
            status: receipt.status,
            primary_owner,
            snapshot,
            explanations: receipt.computation.explanations.clone(),
            diagnostics: receipt.computation.diagnostics.clone(),
        }
    }
}

impl PilotSnapshot {
    fn from_receipt(receipt: &PilotHeadlessReceipt) -> Self {
        Self {
            ability_modifiers: receipt.computation.ability_modifiers,
            base_attack_bonus: receipt.computation.base_attack_bonus,
            base_saves: receipt.computation.base_saves,
            combat: PilotCombatViewModel {
                baseline_melee_attack_bonus: receipt.computation.baseline_melee_attack_bonus,
            },
            defense: PilotDefenseViewModel {
                baseline_armor_class: receipt.computation.baseline_armor_class,
                total_save: receipt.computation.total_saves,
            },
            skill: PilotSkillViewModel {
                selected_modifier: receipt.computation.selected_skill_modifiers,
            },
        }
    }
}
