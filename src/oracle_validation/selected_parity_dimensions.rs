//! GE06-E3-F1 selected parity-dimension adapter.
//!
//! Adapts the merged GE06-E2-F3 headless receipt into a machine-checkable
//! selected-dimension surface for the mandatory pilot dimensions only. Preserves
//! current new-system values or references and maintains a `Computed` claim-tier
//! floor without implying oracle-checked parity.

use crate::rules_core::pilot_compute::PilotHeadlessReceipt;

/// Claim-tier floor for the emitted selected-dimension carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimTierFloor {
    /// The selected-dimension carrier contains computed evidence only.
    /// Parity comparison has not yet been performed.
    Computed,
}

/// One selected-dimension value from the integrated headless receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedDimension {
    /// Stable dimension ID (e.g., `character.identity`, `combat.baseline_melee_attack_bonus`).
    pub id: String,
    /// String-typed value (used for identity dimensions).
    pub value_string: Option<String>,
    /// Numeric value (used for computed output dimensions).
    pub value_i16: Option<i16>,
    /// Source package ID for this dimension (e.g., `pf1.core_rulebook`).
    pub source_package_id: String,
}

/// A bounded carrier of selected mandatory pilot dimensions projected from the
/// merged headless receipt. Preserves current new-system values and keeps the
/// claim-tier floor at `Computed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedParityDimensions {
    /// The mandatory selected pilot dimensions for the integrated slice.
    pub dimensions: Vec<SelectedDimension>,
    /// Claim-tier floor: this carrier contains computed evidence only.
    pub claim_tier_floor: ClaimTierFloor,
}

impl SelectedParityDimensions {
    /// Projects a merged headless receipt into selected parity dimensions for
    /// the mandatory pilot dimensions only.
    pub fn from_receipt(receipt: &PilotHeadlessReceipt) -> Self {
        let computation = &receipt.computation;
        let numeric_dimensions: [(&str, i16); 8] = [
            ("combat.baseline_melee_attack_bonus", computation.baseline_melee_attack_bonus),
            ("defense.baseline_armor_class", computation.baseline_armor_class),
            ("defense.total_save.fortitude", computation.total_saves.fortitude),
            ("defense.total_save.reflex", computation.total_saves.reflex),
            ("defense.total_save.will", computation.total_saves.will),
            ("skill.selected_modifier.climb", computation.selected_skill_modifiers.climb),
            ("skill.selected_modifier.intimidate", computation.selected_skill_modifiers.intimidate),
            ("skill.selected_modifier.swim", computation.selected_skill_modifiers.swim),
        ];

        let mut dimensions = Vec::with_capacity(numeric_dimensions.len() + 1);

        // character.identity dimension: preserves pilot identity from the receipt
        if let Some(case_id) = &receipt.case_id {
            dimensions.push(SelectedDimension {
                id: "character.identity".to_string(),
                value_string: Some(case_id.clone()),
                value_i16: None,
                source_package_id: receipt.source_package_id.clone(),
            });
        }

        dimensions.extend(numeric_dimensions.into_iter().map(|(id, value)| {
            SelectedDimension {
                id: id.to_string(),
                value_string: None,
                value_i16: Some(value),
                source_package_id: receipt.source_package_id.clone(),
            }
        }));

        Self {
            dimensions,
            claim_tier_floor: ClaimTierFloor::Computed,
        }
    }
}
