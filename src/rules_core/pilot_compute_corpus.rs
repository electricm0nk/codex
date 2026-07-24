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
    /// `armor_check_penalty_total`, `max_dex_cap`, `spell_failure_chance`,
    /// and the bounded single-weapon `attack_bonus_delta`) for the
    /// character's currently `EquippedActive` items, via the same
    /// already-existing `equipment_effects::compute_equipment_effects`
    /// `contract::to_pilot_receipt` already calls for `PilotReceipt`.
    /// Deliberately NOT wired into the claim-gated `PilotBaseChassisComputation`
    /// pillars (`baseline_armor_class`, `baseline_melee_attack_bonus`,
    /// `selected_skill_modifiers`) this receipt's own `base` field carries —
    /// this is an additive, explicitly-not-claim-gated section, the same
    /// posture `equipped_items` above already has.
    pub equipment_effects: EquipmentEffects,
    /// v0.6 alpha swarm (QA finding, 2026-07-24): every `spells_selected`
    /// entry whose `spell_id` did NOT resolve against `corpus` -- verbatim,
    /// not deduplicated against `school_coverage`. Before this field, an
    /// unresolved selection simply vanished from every corpus-derived
    /// output with no trace at all (the loop below `continue`s past it) --
    /// for the desktop app specifically, whose only bundled `corpus` is a
    /// deliberately tiny ~4-record demo fixture
    /// (`apps/desktop/src-tauri/src/corpus_fixtures.rs`), this meant a
    /// real, disk-persisted selection outside that tiny bundle looked
    /// identical to "nothing selected" -- a silent, honest-looking display
    /// bug, not a data-loss bug (the underlying `CharacterInput` field was
    /// never touched). This field makes that absence traceable rather than
    /// silent, matching this crate's "never fabricate, never silently
    /// drop" discipline. Whether/how a caller surfaces this to a user is
    /// its own decision -- this field only guarantees the information
    /// exists to make that decision with.
    pub unresolved_spell_ids: Vec<String>,
    /// Mirrors `unresolved_spell_ids` exactly, for
    /// `equipment_selections[].item_id` that did not resolve against
    /// `corpus`.
    pub unresolved_equipment_item_ids: Vec<String>,
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
    let mut unresolved_spell_ids = Vec::new();
    for selection in &input.chosen.spells_selected {
        let Some((record, table_cell)) =
            spell_id_resolve(&selection.spell_id, RuleSetId::Crb, corpus)
        else {
            unresolved_spell_ids.push(selection.spell_id.clone());
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
    let mut unresolved_equipment_item_ids = Vec::new();
    for selection in &input.chosen.equipment_selections {
        let Some((record, table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            unresolved_equipment_item_ids.push(selection.item_id.clone());
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
            unresolved_spell_ids,
            unresolved_equipment_item_ids,
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
            applied_modifiers: Vec::new(),
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
            applied_modifiers: Vec::new(),
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

    /// v0.6 alpha swarm (QA finding, 2026-07-24): a real, disk-persisted
    /// equipment selection that does not resolve against `corpus` (e.g.
    /// the desktop app's tiny bundled demo corpus) must be traceable, not
    /// silently vanish from every corpus-derived output with no signal at
    /// all. A resolvable item and an unresolvable one are both present in
    /// the same input, proving the unresolved list doesn't just echo
    /// everything back.
    #[test]
    fn corpus_derived_section_tracks_an_equipment_selection_that_does_not_resolve() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(vec![
            EquipmentSelection {
                item_id: "Chain Shirt (Base)".to_string(),
                equipped_or_active: true,
                active_state: ActiveState::EquippedActive,
                applied_modifiers: Vec::new(),
            },
            EquipmentSelection {
                item_id: "Wand of Cure Light Wounds".to_string(),
                equipped_or_active: true,
                active_state: ActiveState::EquippedActive,
                applied_modifiers: Vec::new(),
            },
        ]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(
            receipt.corpus_derived.unresolved_equipment_item_ids,
            vec!["Wand of Cure Light Wounds".to_string()],
            "the unresolvable selection must be traceable, not silently dropped"
        );
        assert_eq!(
            receipt.corpus_derived.equipped_items.len(),
            1,
            "the resolvable Chain Shirt must still resolve normally"
        );
    }

    /// Mirrors the equipment case exactly, for `spells_selected`.
    #[test]
    fn corpus_derived_section_tracks_a_spell_selection_that_does_not_resolve() {
        let corpus = corpus_with_chain_shirt();
        let mut input = fighter_input_with(Vec::new());
        input.chosen.spells_selected.push(crate::rules_core::character_input::SpellSelection {
            spell_id: "Magic Missile".to_string(),
            source_class_id: "class:wizard".to_string(),
            acquisition_mode: crate::rules_core::character_input::AcquisitionMode::Known,
        });

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert_eq!(
            receipt.corpus_derived.unresolved_spell_ids,
            vec!["Magic Missile".to_string()],
            "a real spell selection absent from this corpus must be traceable, not silently dropped"
        );
        assert!(receipt.corpus_derived.school_coverage.is_empty());
    }

    /// Every selection resolving cleanly must leave both unresolved lists
    /// genuinely empty, not just unpopulated by omission.
    #[test]
    fn corpus_derived_section_leaves_unresolved_lists_empty_when_everything_resolves() {
        let corpus = corpus_with_chain_shirt();
        let input = fighter_input_with(vec![EquipmentSelection {
            item_id: "Chain Shirt (Base)".to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }]);

        let receipt = compute_pilot_with_corpus(&input, &corpus);

        assert!(receipt.corpus_derived.unresolved_equipment_item_ids.is_empty());
        assert!(receipt.corpus_derived.unresolved_spell_ids.is_empty());
    }
}
