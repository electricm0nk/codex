//! SD-20 Epic 2-7 -> boundary-contract wiring project, Cycle 4
//! (`contract:equipment_wiring`, per `adaptive-squishing-mccarthy.md`).
//!
//! Before this cycle, `PilotReceipt` never called Epic 5's real
//! `equipment_effects::compute_equipment_effects` at all (`tests/
//! sd20_tabletop_readiness_integration.rs`'s Finding 1) -- equipment
//! effects (AC bonus, max-dex cap, spell-failure chance) were reachable
//! only via a direct call to the engine, never through the boundary
//! contract. This file proves three things about the fix:
//!
//! (a) **Parity**: `to_pilot_receipt(...).equipment_effects` matches
//!     exactly what a direct call to
//!     `compute_equipment_effects(&equipped, &corpus)` produces for the
//!     same pre-filtered `equipped` slice -- the wiring composes the
//!     engine's real output, it does not reinterpret or duplicate it.
//!     Exercised with the Fighter tabletop fixture's real equipped items
//!     (Longsword, Chain Shirt, the Masterwork weapon quality -- the same
//!     three real CRB records `tests/sd20_tabletop_readiness_integration.rs`'s
//!     `epic_probe_equipment_effects_are_real_but_not_reflected_in_corpus_derived`
//!     probes directly), so the parity assertion is not vacuously true
//!     over a default `EquipmentEffects`.
//! (b) **`max_dex_cap` cell discipline**: `sheet.equipment.max_dex_cap` is
//!     present when the equipped loadout carries a max-dex cap (the Chain
//!     Shirt scenario) and *entirely absent* -- not a fabricated
//!     `Blocked` or `Number(0)` -- when it does not (an unarmored
//!     loadout). `sheet.equipment.armor_class_delta` is always present
//!     (it is a plain `i16`, not `Option`, so there is always a real
//!     value -- `0` for "no armor bonus" is honest, not fabricated).
//! (c) **`EquippedActive` filtering**: an equipment selection whose
//!     `active_state` is `SelectedInactive` (not `EquippedActive`) is
//!     excluded from the computation entirely -- it contributes nothing
//!     to `armor_class_delta`/`max_dex_cap`/`spell_failure_chance` and
//!     produces no `per_item` entry, even though the underlying corpus
//!     record resolves cleanly (proving the exclusion is genuinely driven
//!     by `active_state`, not by resolver failure).
//!
//! Per `adaptive-squishing-mccarthy.md`'s Cycle 4 scope: `spell_failure_chance:
//! Option<f32>` is deliberately EXCLUDED from `printed_sheet_cell_map`
//! cells this cycle -- a fractional percentage doesn't cleanly fit
//! `PrintedSheetCellValue::Number(i16)`. It stays reachable via
//! `receipt.equipment_effects.spell_failure_chance` directly, matching
//! the same "not every epic output becomes a sheet cell" precedent
//! `spellbook`/`feats` already set.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection,
};
use codex::rules_core::contract::{printed_sheet_cell_map, to_pilot_receipt, PrintedSheetCellValue};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// The same real CRB equipment records
/// `tests/sd20_tabletop_readiness_integration.rs`'s
/// `FIGHTER_GEAR_FIXTURE_TEXT` uses (Longsword, Chain Shirt, the
/// Masterwork weapon quality), plus one extra shield record
/// (`Buckler (Base)`) this file uses only to prove `active_state`
/// filtering excludes a real, resolvable record when it is not
/// `EquippedActive`.
const FIGHTER_GEAR_FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
Buckler\tKEY:Buckler (Base)\tTYPE:Shield\tCOST:5\tWT:5\tACCHECK:-1\tBONUS:COMBAT|AC|1|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
";

fn corpus_with_fighter_gear() -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", FIGHTER_GEAR_FIXTURE_TEXT);
    assert!(
        result.diagnostics.is_empty(),
        "fixture text must parse cleanly: {:?}",
        result.diagnostics
    );
    let source_ref = SourceRef {
        lst_file: "cr_equip_arms_armor.lst".to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
    for record in result.entries {
        let record: &'static EquipmentRecord = Box::leak(Box::new(record));
        corpus.push(convert_equipment_record(record));
    }
    corpus
}

fn base_fighter_input(equipment_selections: Vec<EquipmentSelection>) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_equipment_wiring_fighter".to_string()),
        source_package_id: "sd20_contract_equipment_wiring_fighter".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_string(),
                level: 1,
            }],
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
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn equipped(item_id: &str) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    }
}

fn selected_inactive(item_id: &str) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_string(),
        equipped_or_active: false,
        active_state: ActiveState::SelectedInactive,
        applied_modifiers: Vec::new(),
    }
}

/// (a) + (c): the Fighter's real equipped gear (Longsword, Chain Shirt,
/// Masterwork weapon quality, all `EquippedActive`) plus a real, resolvable
/// Buckler that is `SelectedInactive` -- proving both parity against a
/// direct `compute_equipment_effects` call over the pre-filtered
/// `EquippedActive` slice, and that the inactive Buckler contributes
/// nothing.
#[test]
fn pilot_receipt_equipment_effects_matches_direct_call_over_equipped_active_only() {
    let input = base_fighter_input(vec![
        equipped("Longsword (Base)"),
        equipped("Chain Shirt (Base)"),
        equipped("Special Quality ~ Masterwork ~ Weapon"),
        selected_inactive("Buckler (Base)"),
    ]);
    let corpus = corpus_with_fighter_gear();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // Independently pre-filter to EquippedActive only, exactly as the
    // brief specifies, and call the engine directly for the parity check.
    let equipped_active: Vec<EquipmentSelection> = input
        .chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .cloned()
        .collect();
    assert_eq!(
        equipped_active.len(),
        3,
        "expected exactly 3 EquippedActive selections (Longsword, Chain Shirt, Masterwork); \
         Buckler must not count"
    );
    let direct = compute_equipment_effects(&equipped_active, &corpus);

    assert_eq!(
        receipt.equipment_effects, direct,
        "receipt.equipment_effects must match a direct compute_equipment_effects(&equipped, &corpus) \
         call over the same EquippedActive-filtered slice exactly"
    );

    // Real values, not vacuous: Chain Shirt's real +4 AC / max dex 4 /
    // 20% spell failure (mirrors
    // sd20_tabletop_readiness_integration.rs's epic_probe_equipment_effects_*).
    assert_eq!(receipt.equipment_effects.per_item.len(), 3);
    assert_eq!(receipt.equipment_effects.armor_class_delta, 4);
    assert_eq!(receipt.equipment_effects.max_dex_cap, Some(4));
    assert_eq!(receipt.equipment_effects.spell_failure_chance, Some(20.0));

    // (c): the inactive Buckler resolves against the corpus cleanly (it's
    // a real record), yet contributes no per_item entry and no AC bonus --
    // proving exclusion is driven by active_state, not resolver failure.
    assert!(
        !receipt
            .equipment_effects
            .per_item
            .iter()
            .any(|item| item.equipment_record_key == "Buckler (Base)"),
        "a SelectedInactive selection must not appear in equipment_effects.per_item at all"
    );

    // Cell-map assertions: armor_class_delta always present; max_dex_cap
    // present (Some(4)) because the Chain Shirt carries a cap.
    let cells = printed_sheet_cell_map(&receipt);
    let ac_delta_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.equipment.armor_class_delta")
        .expect("sheet.equipment.armor_class_delta must always be present");
    assert_eq!(ac_delta_cell.value, PrintedSheetCellValue::Number(4));

    let max_dex_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.equipment.max_dex_cap")
        .expect("sheet.equipment.max_dex_cap must be present when a max-dex cap exists");
    assert_eq!(max_dex_cell.value, PrintedSheetCellValue::Number(4));
}

/// (b): an unarmored loadout (Longsword + Masterwork weapon quality, no
/// armor/shield) has no max-dex cap at all -- `sheet.equipment.max_dex_cap`
/// must be entirely absent from the cell map (not `Blocked`, not a
/// fabricated `Number(0)`), while `sheet.equipment.armor_class_delta`
/// stays present with the honest value `0`.
#[test]
fn max_dex_cap_cell_is_omitted_entirely_when_no_cap_exists() {
    let input = base_fighter_input(vec![
        equipped("Longsword (Base)"),
        equipped("Special Quality ~ Masterwork ~ Weapon"),
    ]);
    let corpus = corpus_with_fighter_gear();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    assert_eq!(receipt.equipment_effects.max_dex_cap, None);
    assert_eq!(receipt.equipment_effects.armor_class_delta, 0);

    let cells = printed_sheet_cell_map(&receipt);
    assert!(
        !cells
            .iter()
            .any(|cell| cell.cell_id == "sheet.equipment.max_dex_cap"),
        "max_dex_cap cell must be entirely absent when no cap exists, not fabricated: {:?}",
        cells
            .iter()
            .find(|cell| cell.cell_id == "sheet.equipment.max_dex_cap")
    );

    let ac_delta_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.equipment.armor_class_delta")
        .expect("sheet.equipment.armor_class_delta must always be present, even at 0");
    assert_eq!(ac_delta_cell.value, PrintedSheetCellValue::Number(0));
}

/// (c), isolated: a `SelectedInactive` Chain Shirt alongside an
/// `EquippedActive` Longsword contributes nothing to `equipment_effects`
/// -- proving the filter genuinely gates on `active_state ==
/// EquippedActive`, not merely "was mentioned in equipment_selections".
#[test]
fn selected_inactive_equipment_is_excluded_from_equipment_effects() {
    let input = base_fighter_input(vec![
        equipped("Longsword (Base)"),
        selected_inactive("Chain Shirt (Base)"),
    ]);
    let corpus = corpus_with_fighter_gear();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    assert_eq!(
        receipt.equipment_effects.per_item.len(),
        1,
        "only the EquippedActive Longsword should resolve; the SelectedInactive Chain Shirt must not"
    );
    assert_eq!(
        receipt.equipment_effects.armor_class_delta, 0,
        "the SelectedInactive Chain Shirt's +4 AC bonus must not be counted"
    );
    assert_eq!(receipt.equipment_effects.max_dex_cap, None);
    assert_eq!(receipt.equipment_effects.spell_failure_chance, None);
}
