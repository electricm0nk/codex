//! SD-19 corpus-aware compute seam.
//!
//! Wraps [`compute_pilot_base_chassis`] with corpus-derived spell-school
//! and equipment contributions, without modifying `pilot_compute.rs`
//! itself — every landed SD-18 cycle keeps calling the chassis function
//! unchanged. See
//! `programs/codex/requirements/SD-19-corpus-aware-compute-seam/technical-design.md`
//! §1 for the design.
//!
//! This module's own `Pf1SchoolId`/`RuleSetId` types come from `rules_tables`
//! (the SD-19 foundation slice); `PilotReceipt` in the doctrine doc's
//! illustrative code does not exist in this repo — the real chassis
//! function returns `PilotBaseChassisComputation`, which is what `base`
//! wraps here.
//!
//! Resolution here is deliberately **generic**: it reads a resolved
//! corpus record's own school/category rather than dispatching through
//! per-school/per-category files (operator-confirmed 2026-07-16, see the
//! review note atop `technical-design.md`). This satisfies the
//! seam-shapes-correctness proof at slice-ship, before any loop cycle has
//! landed. Future SD-19 cycles ground *evidence tier* (matrix row
//! promotion, exhaustive per-school/category coverage) rather than
//! writing new dispatch code.

use std::collections::BTreeMap;

use crate::rules_core::character_input::{ActiveState, CharacterInput, EquipmentSelection};
use crate::rules_core::equipment_effects::{compute_equipment_effects, EquipmentEffects};
use crate::rules_core::equipment_resolver::equipment_id_resolve;
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, PilotBaseChassisComputation};
use crate::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;
use crate::rules_core::spell_resolver::spell_id_resolve;

/// Corpus-augmented compute result. Wraps the existing chassis
/// computation and adds a corpus-derived section carrying the
/// spell-coverage and equipped-items contributions produced by the seam.
#[derive(Debug, Clone, PartialEq)]
pub struct CorpusPilotReceipt {
    /// The unchanged chassis computation from `compute_pilot_base_chassis`.
    pub base: PilotBaseChassisComputation,
    /// The corpus-derived contributions grounded by this call. Empty when
    /// the input carried no `spells_selected` or `equipment_selections`
    /// entries that resolved against the corpus.
    pub corpus_derived: CorpusDerivedSection,
}

/// Per-domain corpus-derived contributions.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CorpusDerivedSection {
    pub school_coverage: BTreeMap<Pf1SchoolId, SchoolCoverage>,
    pub equipped_items: Vec<ResolvedEquipment>,
    /// v0.6 alpha swarm item 1, shape (c): the real, corpus-resolved
    /// aggregate equipment-effect totals (`armor_class_delta`,
    /// `armor_check_penalty_total`, `max_dex_cap`, `spell_failure_chance`)
    /// for the character's currently `EquippedActive` items, via the same
    /// already-existing `equipment_effects::compute_equipment_effects`
    /// `contract::to_pilot_receipt` already calls for `PilotReceipt`.
    /// Deliberately NOT wired into the claim-gated `PilotBaseChassisComputation`
    /// pillars (`baseline_armor_class`, `baseline_melee_attack_bonus`,
    /// `selected_skill_modifiers`) this receipt's own `base` field carries —
    /// this is an additive, explicitly-not-claim-gated section, the same
    /// posture `equipped_items` above already has. Attack-bonus enhancement
    /// is deliberately excluded (see `EquipmentEffects.per_item`'s own
    /// `weapon_enhancement_bonus`, not surfaced here) — real per-item math
    /// exists, but nothing in `character_input.rs`'s schema records which
    /// weapon a modifier item attaches to, so aggregating it would risk
    /// misapplying one weapon's bonus to another. See
    /// `docs/release/v0.6/item-1-architecture-wall-design.md` for the full
    /// design pass this field implements the recommendation of.
    pub equipment_effects: EquipmentEffects,
}

/// A canonical Paizo-table-cell reference. Non-`None` proves the corpus
/// record the seam resolved lives at a specific cell of the rule-set's
/// source-book table, not just "a corpus record exists."
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellRef {
    pub rule_set: RuleSetId,
    pub table: String,
    pub row_key: String,
    pub column_key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SchoolCoverage {
    pub school: Pf1SchoolId,
    /// Corpus spell identities (see `spell_resolver`'s doc comment on why
    /// this is the spell's `name`, not a `KEY:` token), sorted.
    pub spells: Vec<String>,
    pub table_cell: Option<TableCellRef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedEquipment {
    pub item_id: String,
    pub equipment_record_name: String,
    pub equipment_record_key: String,
    pub derived_stats: DerivedEquipmentStats,
    pub table_cell: Option<TableCellRef>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DerivedEquipmentStats {
    pub armor_bonus: Option<i16>,
    pub attack_bonus: Option<i16>,
    pub max_dex: Option<i16>,
    pub spell_failure: Option<f32>,
}

/// The corpus-aware compute seam. Computes the unchanged chassis, then
/// resolves every `spells_selected` / `equipment_selections` entry
/// against the corpus, grouping resolved spells by school and wrapping
/// resolved equipment with its (currently unpopulated — bounded-baseline
/// non-goal, see `scope-draft.md` §1.1) derived stats.
pub fn compute_pilot_with_corpus(
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> CorpusPilotReceipt {
    let base = compute_pilot_base_chassis(input);

    let mut school_coverage: BTreeMap<Pf1SchoolId, SchoolCoverage> = BTreeMap::new();
    for selection in &input.chosen.spells_selected {
        let Some((record, table_cell)) =
            spell_id_resolve(&selection.spell_id, RuleSetId::Crb, corpus)
        else {
            continue;
        };
        let Some(school) = record
            .school
            .as_deref()
            .and_then(Pf1SchoolId::from_corpus_str)
        else {
            continue;
        };
        let entry = school_coverage.entry(school).or_insert_with(|| SchoolCoverage {
            school,
            spells: Vec::new(),
            table_cell: table_cell.clone(),
        });
        if !entry.spells.contains(&selection.spell_id) {
            entry.spells.push(selection.spell_id.clone());
        }
        if entry.table_cell.is_none() {
            entry.table_cell = table_cell;
        }
    }
    for coverage in school_coverage.values_mut() {
        coverage.spells.sort();
    }

    let mut equipped_items = Vec::new();
    for selection in &input.chosen.equipment_selections {
        let Some((record, table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            continue;
        };
        let key = crate::rules_core::equipment_resolver::equipment_key_token(record)
            .unwrap_or(&record.name)
            .to_string();
        equipped_items.push(ResolvedEquipment {
            item_id: selection.item_id.clone(),
            equipment_record_name: record.name.clone(),
            equipment_record_key: key,
            derived_stats: DerivedEquipmentStats::default(),
            table_cell,
        });
    }

    // v0.6 alpha swarm item 1, shape (c): mirrors `contract::to_pilot_receipt`'s
    // own `EquippedActive`-only filtering exactly (a `SelectedInactive` or
    // `Absent` item contributes no armor/attack/skill effect, only
    // `equipped_items` above tracks identity for everything owned).
    let equipped: Vec<EquipmentSelection> = input
        .chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .cloned()
        .collect();
    let equipment_effects = compute_equipment_effects(&equipped, corpus);

    CorpusPilotReceipt {
        base,
        corpus_derived: CorpusDerivedSection {
            school_coverage,
            equipped_items,
            equipment_effects,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use crate::rules_core::character_input::{
        AbilityScores, CharacterClassLevel, ChosenCharacterState,
    };
    use crate::rules_core::source_content::SourceRef;

    /// Real verbatim tokens for a Chain Shirt, matching
    /// `tests/sd20_contract_equipment_wiring.rs`'s own fixture exactly
    /// (`ACCHECK:-2`) -- same real corpus record, reused rather than
    /// re-derived.
    const CHAIN_SHIRT_FIXTURE_TEXT: &str = "Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";

    fn corpus_with_chain_shirt() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", CHAIN_SHIRT_FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    fn fighter_input_with(equipment_selections: Vec<EquipmentSelection>) -> CharacterInput {
        CharacterInput {
            case_id: Some("pilot-compute-corpus-equipment-effects-test".to_string()),
            source_package_id: "test".to_string(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_string(),
                class_levels: vec![CharacterClassLevel { class_id: "class:fighter".to_string(), level: 1 }],
                ability_scores: AbilityScores {
                    strength: 16,
                    dexterity: 14,
                    constitution: 14,
                    intelligence: 10,
                    wisdom: 12,
                    charisma: 8,
                },
                selected_feats: Vec::new(),
                skill_allocations: Vec::new(),
                equipment_selections,
                selected_choices: Vec::new(),
                spells_selected: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    /// v0.6 alpha swarm item 1, shape (c): the new
    /// `CorpusDerivedSection.equipment_effects` field surfaces a real,
    /// corpus-resolved armor-check penalty for an equipped item -- the gap
    /// the item-1 design pass identified (the token was already present on
    /// the resolved record, just never read into a struct field).
    #[test]
    fn corpus_derived_section_carries_the_real_armor_check_penalty_for_equipped_active_armor() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(vec![EquipmentSelection {
            item_id: "Chain Shirt (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
        }]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(
            receipt.corpus_derived.equipment_effects.armor_check_penalty_total, -2,
            "Chain Shirt's real ACCHECK is -2"
        );
        assert_eq!(receipt.corpus_derived.equipment_effects.armor_class_delta, 4);
    }

    /// A resolvable item that is merely `SelectedInactive` (owned, not
    /// worn) must contribute no armor-check penalty -- proves the
    /// `EquippedActive` filter is real, not vacuous.
    #[test]
    fn corpus_derived_section_excludes_a_selected_inactive_items_armor_check_penalty() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(vec![EquipmentSelection {
            item_id: "Chain Shirt (Base)".to_string(),
            equipped_or_active: false,
            active_state: ActiveState::SelectedInactive,
        }]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(receipt.corpus_derived.equipment_effects.armor_check_penalty_total, 0);
        assert_eq!(receipt.corpus_derived.equipment_effects.armor_class_delta, 0);
        assert!(
            receipt.corpus_derived.equipment_effects.per_item.is_empty(),
            "an inactive selection must produce no per-item equipment-effect entry"
        );
    }

    /// A build with no equipment at all must show a real, honest zero, not
    /// an error or a fabricated value.
    #[test]
    fn corpus_derived_section_defaults_to_zero_armor_check_penalty_with_no_equipment() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(Vec::new());

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(receipt.corpus_derived.equipment_effects.armor_check_penalty_total, 0);
    }
}
