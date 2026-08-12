//! GE06-E3-F1 selected parity-dimension adapter.
//!
//! Adapts the merged GE06-E2-F3 headless receipt into a machine-checkable
//! selected-dimension surface for the mandatory pilot dimensions only. Preserves
//! current new-system values or references and maintains a `Computed` claim-tier
//! floor without implying oracle-checked parity.
//!
//! v0.6 alpha swarm item 4 follow-up added `from_pilot_receipt` (see its own
//! doc comment) alongside the original `from_receipt` rather than extending
//! `from_receipt`'s signature -- `from_receipt` takes the corpus-free
//! headless `PilotHeadlessReceipt`, which structurally cannot carry
//! encumbrance (needs real corpus-resolved per-item weight) or a
//! character's class levels (needed for max HP); widening it would have
//! required either the same corpus-threading architecture change that
//! dropped the AC posture-widening slice, or a breaking signature change to
//! `from_receipt`'s existing callers. Adding a new, additive entry point
//! instead means zero blast radius on `from_receipt`'s 3 existing callers.

use crate::rules_core::character_input::CharacterClassLevel;
use crate::rules_core::contract::PilotReceipt;
use crate::rules_core::durability::compute_max_hp;
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
        let numeric_dimensions: [(&str, i16); 9] = [
            ("combat.base_attack_bonus", computation.base_attack_bonus),
            (
                "combat.baseline_melee_attack_bonus",
                computation.baseline_melee_attack_bonus,
            ),
            (
                "defense.baseline_armor_class",
                computation.baseline_armor_class,
            ),
            (
                "defense.total_save.fortitude",
                computation.total_saves.fortitude,
            ),
            ("defense.total_save.reflex", computation.total_saves.reflex),
            ("defense.total_save.will", computation.total_saves.will),
            (
                "skill.selected_modifier.climb",
                computation.selected_skill_modifiers.climb,
            ),
            (
                "skill.selected_modifier.intimidate",
                computation.selected_skill_modifiers.intimidate,
            ),
            (
                "skill.selected_modifier.swim",
                computation.selected_skill_modifiers.swim,
            ),
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

        dimensions.extend(
            numeric_dimensions
                .into_iter()
                .map(|(id, value)| SelectedDimension {
                    id: id.to_string(),
                    value_string: None,
                    value_i16: Some(value),
                    source_package_id: receipt.source_package_id.clone(),
                }),
        );

        Self {
            dimensions,
            claim_tier_floor: ClaimTierFloor::Computed,
        }
    }

    /// Projects the corpus-aware `contract::PilotReceipt` (not the
    /// corpus-free headless receipt `from_receipt` reads) into
    /// `from_receipt`'s same 9 mandatory dimensions (including
    /// `combat.base_attack_bonus`, the raw class-table BAB distinct from
    /// `combat.baseline_melee_attack_bonus`'s Strength/feat-inclusive total —
    /// self-directed backend scan, v0.6 alpha swarm), plus
    /// `durability.max_hp` and the 3 `encumbrance.*` dimensions v0.6 alpha
    /// swarm item 4 added PCGen-side extraction for
    /// (`scripts/pcgen-normalize-output.py`). Both need data
    /// `PilotHeadlessReceipt` cannot carry: max HP needs `class_levels`
    /// (not on the headless receipt at all -- passed in separately here,
    /// since `PilotReceipt` itself doesn't echo the `CharacterInput` that
    /// produced it either), and encumbrance needs real corpus-resolved
    /// per-item weight, which `PilotReceipt.encumbrance` has *already*
    /// resolved (`contract::to_pilot_receipt` is corpus-aware) -- this
    /// function only reads that already-computed value, it does not touch
    /// corpus itself.
    ///
    /// `durability.max_hp` is omitted (not emitted as a zero) when
    /// `compute_max_hp` returns `None` -- multiclass builds and classes
    /// outside `table_class_id`'s recognized set (see `durability.rs`'s own
    /// doc comment) honestly have no computed max HP to report, rather than
    /// fabricating one.
    ///
    /// `case_id`/`source_package_id` are passed explicitly rather than read
    /// from the receipt: `PilotReceipt` doesn't carry either (unlike
    /// `PilotHeadlessReceipt`, which does) since `contract::to_pilot_receipt`
    /// doesn't echo its `CharacterInput` input back onto its output either.
    pub fn from_pilot_receipt(
        receipt: &PilotReceipt,
        class_levels: &[CharacterClassLevel],
        case_id: Option<&str>,
        source_package_id: &str,
    ) -> Self {
        let chassis = &receipt.chassis;
        let mut numeric_dimensions: Vec<(&str, Option<i16>)> = vec![
            ("combat.base_attack_bonus", Some(chassis.base_attack_bonus)),
            (
                "combat.baseline_melee_attack_bonus",
                Some(chassis.baseline_melee_attack_bonus),
            ),
            (
                "defense.baseline_armor_class",
                Some(chassis.baseline_armor_class),
            ),
            (
                "defense.total_save.fortitude",
                Some(chassis.total_saves.fortitude),
            ),
            ("defense.total_save.reflex", Some(chassis.total_saves.reflex)),
            ("defense.total_save.will", Some(chassis.total_saves.will)),
            (
                "skill.selected_modifier.climb",
                Some(chassis.selected_skill_modifiers.climb),
            ),
            (
                "skill.selected_modifier.intimidate",
                Some(chassis.selected_skill_modifiers.intimidate),
            ),
            (
                "skill.selected_modifier.swim",
                Some(chassis.selected_skill_modifiers.swim),
            ),
            (
                "durability.max_hp",
                compute_max_hp(class_levels, chassis.ability_modifiers.constitution),
            ),
            (
                "encumbrance.carrying_capacity.light_max_lbs",
                Some(receipt.encumbrance.thresholds.light_max_lbs.round() as i16),
            ),
            (
                "encumbrance.carrying_capacity.medium_max_lbs",
                Some(receipt.encumbrance.thresholds.medium_max_lbs.round() as i16),
            ),
            (
                "encumbrance.carrying_capacity.heavy_max_lbs",
                Some(receipt.encumbrance.thresholds.heavy_max_lbs.round() as i16),
            ),
            (
                "encumbrance.total_carried_weight_lbs",
                Some(receipt.encumbrance.total_carried_weight_lbs.round() as i16),
            ),
        ];
        numeric_dimensions.retain(|(_, value)| value.is_some());

        let mut dimensions = Vec::with_capacity(numeric_dimensions.len() + 1);

        if let Some(case_id) = case_id {
            dimensions.push(SelectedDimension {
                id: "character.identity".to_string(),
                value_string: Some(case_id.to_string()),
                value_i16: None,
                source_package_id: source_package_id.to_string(),
            });
        }

        dimensions.extend(numeric_dimensions.into_iter().map(|(id, value)| SelectedDimension {
            id: id.to_string(),
            value_string: None,
            value_i16: value,
            source_package_id: source_package_id.to_string(),
        }));

        Self {
            dimensions,
            claim_tier_floor: ClaimTierFloor::Computed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};
    use crate::rules_core::contract::to_pilot_receipt;
    use crate::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
    use crate::rules_core::source_content::SourcePackageContent;

    const FIXTURE: &str =
        include_str!("../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

    fn load_input() -> CharacterInput {
        let result = load_character_input_fixture(FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly: {:?}", result.diagnostics);
        result.character_input.expect("valid fixture should produce a character input record")
    }

    fn empty_corpus() -> SourcePackageContent<'static> {
        SourcePackageContent::empty(
            "core_rulebook",
            crate::rules_core::source_content::SourceRef {
                lst_file: "test.lst".to_string(),
                line: 1,
            },
        )
    }

    #[test]
    fn from_pilot_receipt_includes_the_original_eight_dimensions_plus_base_attack_bonus_durability_and_encumbrance() {
        let input = load_input();
        let corpus = empty_corpus();
        let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
        let pilot_receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

        let dimensions = SelectedParityDimensions::from_pilot_receipt(
            &pilot_receipt,
            &input.chosen.class_levels,
            input.case_id.as_deref(),
            &input.source_package_id,
        );

        let ids: Vec<&str> = dimensions.dimensions.iter().map(|d| d.id.as_str()).collect();
        for expected_id in [
            "character.identity",
            "combat.base_attack_bonus",
            "combat.baseline_melee_attack_bonus",
            "defense.baseline_armor_class",
            "defense.total_save.fortitude",
            "defense.total_save.reflex",
            "defense.total_save.will",
            "skill.selected_modifier.climb",
            "skill.selected_modifier.intimidate",
            "skill.selected_modifier.swim",
            "durability.max_hp",
            "encumbrance.carrying_capacity.light_max_lbs",
            "encumbrance.carrying_capacity.medium_max_lbs",
            "encumbrance.carrying_capacity.heavy_max_lbs",
            "encumbrance.total_carried_weight_lbs",
        ] {
            assert!(ids.contains(&expected_id), "missing dimension '{expected_id}': {ids:?}");
        }

        // Human Fighter level 1, d10 max + CON mod +2 (CG-03: STR gets the
        // +2 racial bonus, not CON here) = 12.
        let max_hp = dimensions.dimensions.iter().find(|d| d.id == "durability.max_hp").unwrap();
        assert_eq!(max_hp.value_i16, Some(12));

        // Fighter level 1 raw BAB is +1 (cr_classes.lst BASEAB|classlevel),
        // distinct from combat.baseline_melee_attack_bonus (which folds in
        // the Strength modifier and any feats/equipment) -- confirmed
        // empirically against a real PCGen run of the same fixture shape,
        // whose /character/attack/melee/bab element also reads +1.
        let base_attack_bonus =
            dimensions.dimensions.iter().find(|d| d.id == "combat.base_attack_bonus").unwrap();
        assert_eq!(base_attack_bonus.value_i16, Some(1));
    }

    #[test]
    fn from_pilot_receipt_omits_durability_max_hp_for_a_multiclass_build_rather_than_fabricating_one() {
        let mut input = load_input();
        input.chosen.class_levels.push(CharacterClassLevel {
            class_id: "class:rogue".to_owned(),
            level: 1,
        });
        let corpus = empty_corpus();
        let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
        let pilot_receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

        let dimensions = SelectedParityDimensions::from_pilot_receipt(
            &pilot_receipt,
            &input.chosen.class_levels,
            input.case_id.as_deref(),
            &input.source_package_id,
        );

        assert!(
            !dimensions.dimensions.iter().any(|d| d.id == "durability.max_hp"),
            "a multiclass build must omit durability.max_hp, not fabricate a value: {:?}",
            dimensions.dimensions
        );
    }

    #[test]
    fn from_receipt_includes_the_raw_base_attack_bonus_distinct_from_the_melee_total() {
        let input = load_input();
        let receipt = crate::rules_core::pilot_compute::build_pilot_headless_receipt(&input);
        let dimensions = SelectedParityDimensions::from_receipt(&receipt);

        let base_attack_bonus =
            dimensions.dimensions.iter().find(|d| d.id == "combat.base_attack_bonus")
                .expect("from_receipt should carry a combat.base_attack_bonus dimension");
        assert_eq!(base_attack_bonus.value_i16, Some(1), "Fighter level 1 raw BAB is +1");

        let melee_total = dimensions
            .dimensions
            .iter()
            .find(|d| d.id == "combat.baseline_melee_attack_bonus")
            .unwrap();
        assert_ne!(
            base_attack_bonus.value_i16, melee_total.value_i16,
            "these must be distinct dimensions, not the same value wired twice"
        );
    }

    #[test]
    fn from_receipt_is_unchanged_and_still_compiles_against_its_existing_three_dot_rs_callers() {
        // Not a behavioral assertion -- proves from_receipt's signature is
        // untouched (this file's own doc comment's central claim) by simply
        // calling it exactly as its existing tests/*.rs callers do.
        let input = load_input();
        let receipt = crate::rules_core::pilot_compute::build_pilot_headless_receipt(&input);
        let dimensions = SelectedParityDimensions::from_receipt(&receipt);
        assert!(!dimensions.dimensions.is_empty());
    }
}
