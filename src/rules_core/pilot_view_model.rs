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
    /// The flat DR magnitude from a grounded class-feature DR explanation
    /// (currently only Barbarian's `class_feature.barbarian.damage_reduction`
    /// -- v0.6 alpha swarm, risks-and-open-questions.md item 6), or `None`
    /// when no such record is present or its magnitude is the level-gate
    /// absence value of 0 (real PF1 has no "DR 0"; omitted, not zeroed, per
    /// this codebase's existing convention for absent-vs-zero facts). This
    /// is a bounded display-only value: no damage-resolution engine applies
    /// it to any actual incoming-damage total.
    pub damage_reduction: Option<i16>,
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
                damage_reduction: receipt
                    .computation
                    .explanations
                    .iter()
                    .find(|explanation| explanation.id == "class_feature.barbarian.damage_reduction")
                    .map(|explanation| explanation.value)
                    .filter(|&value| value > 0),
            },
            skill: PilotSkillViewModel {
                selected_modifier: receipt.computation.selected_skill_modifiers,
            },
        }
    }
}

/// v0.6 alpha swarm (risks-and-open-questions.md item 6): DR data already
/// existed as a raw `ComputationExplanation` (Barbarian's
/// `class_feature.barbarian.damage_reduction`, `pilot_compute.rs`) but
/// wasn't promoted to a first-class `PilotSnapshot` field, so the Defense
/// tab had nothing structured to render. These tests exercise
/// `PilotSnapshot::from_receipt` directly with a synthetic receipt (the
/// production compute path never happens to produce this state today,
/// since Barbarian isn't yet a chassis-dispatch-supported class -- see
/// `has_supported_class_chassis` -- so this proves the wiring itself,
/// independent of when/whether Barbarian becomes chassis-supported).
#[cfg(test)]
mod damage_reduction_exposure_tests {
    use super::*;
    use crate::rules_core::pilot_compute::PilotBaseChassisComputation;

    fn receipt_with_explanations(explanations: Vec<ComputationExplanation>) -> PilotHeadlessReceipt {
        PilotHeadlessReceipt {
            case_id: None,
            source_package_id: "test".to_owned(),
            status: HeadlessReceiptStatus::Computed,
            computation: PilotBaseChassisComputation {
                ability_modifiers: AbilityModifiers::default(),
                base_attack_bonus: 0,
                base_saves: BaseSaves::default(),
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSaves::default(),
                selected_skill_modifiers: SelectedSkillModifiers::default(),
                explanations,
                diagnostics: Vec::new(),
            },
        }
    }

    #[test]
    fn surfaces_a_real_grounded_damage_reduction_value() {
        let receipt = receipt_with_explanations(vec![ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: 3,
            detail: "Barbarian DR 3/-".to_owned(),
        }]);

        let snapshot = PilotSnapshot::from_receipt(&receipt);

        assert_eq!(snapshot.defense.damage_reduction, Some(3));
    }

    #[test]
    fn omits_a_zero_level_gate_absence_value_rather_than_surfacing_a_fake_zero() {
        let receipt = receipt_with_explanations(vec![ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: 0,
            detail: "Barbarian DR absent below level 7".to_owned(),
        }]);

        let snapshot = PilotSnapshot::from_receipt(&receipt);

        assert_eq!(
            snapshot.defense.damage_reduction, None,
            "real PF1 has no \"DR 0\" -- the level-gate absence record must be omitted, not \
             surfaced as a fake zero"
        );
    }

    #[test]
    fn omits_damage_reduction_when_no_such_explanation_exists_at_all() {
        let receipt = receipt_with_explanations(Vec::new());

        let snapshot = PilotSnapshot::from_receipt(&receipt);

        assert_eq!(snapshot.defense.damage_reduction, None);
    }
}
