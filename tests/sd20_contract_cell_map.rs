//! SD-20 Epic 1 (boundary contract): printed-sheet cell map — cycle 3.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.1 and
//! `technical-design.md` §1.1's "Cells" clause, the printed-sheet cell map
//! is a row-by-row map of the printed PF1 character sheet, each cell
//! pointing at exactly one `PilotReceipt` field. A cell whose source field
//! is claim-blocked renders `PrintedSheetCellValue::Blocked` ("blocked —
//! see diagnostics") rather than a fabricated number. This is the third
//! Epic-1 work-unit (after `CharacterInput` types and `PilotReceipt`
//! types, per the loop instruction's Step 2).

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    SpellSelection,
};
use codex::rules_core::contract::{
    printed_sheet_cell_map, to_pilot_receipt, PrintedSheetCell, PrintedSheetCellValue,
};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

fn empty_corpus() -> SourcePackageContent<'static> {
    let source_ref = SourceRef {
        lst_file: "sd20_contract_cell_map".to_string(),
        line: 1,
    };
    SourcePackageContent::empty("sd20_contract_cell_map", source_ref)
}

fn fighter_level_1_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_cell_map".to_string()),
        source_package_id: "sd20_contract_cell_map".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            // "class:fighter" (not the bare "fighter" some sibling SD-19/SD-20
            // fixtures use) is required to hit `compute_fighter_chassis`'s
            // `FIGHTER_CLASS_ID` match and produce a genuinely *supported*
            // chassis — this test needs a real (non-`class_chassis.unsupported`)
            // posture to prove the cell map renders real numbers, not just
            // that it round-trips whatever the chassis function happens to
            // return.
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 14,
                dexterity: 12,
                constitution: 13,
                intelligence: 10,
                wisdom: 10,
                charisma: 8,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn wizard_level_1_input() -> CharacterInput {
    let mut input = fighter_level_1_input();
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "wizard".to_string(),
        level: 1,
    }];
    input.chosen.spells_selected = vec![SpellSelection {
        spell_id: "Not A Real Spell".to_string(),
        source_class_id: "wizard".to_string(),
        acquisition_mode: AcquisitionMode::Known,
    }];
    input
}

fn cell_value<'a>(cells: &'a [PrintedSheetCell], cell_id: &str) -> &'a PrintedSheetCellValue {
    &cells
        .iter()
        .find(|c| c.cell_id == cell_id)
        .unwrap_or_else(|| panic!("expected a printed-sheet cell with id {cell_id}"))
        .value
}

#[test]
fn printed_sheet_cell_map_renders_real_values_for_a_supported_chassis() {
    let corpus = empty_corpus();
    let input = fighter_level_1_input();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    let cells = printed_sheet_cell_map(&receipt);

    assert_eq!(
        *cell_value(&cells, "sheet.base_attack_bonus"),
        PrintedSheetCellValue::Number(receipt.chassis.base_attack_bonus)
    );
    assert_eq!(
        *cell_value(&cells, "sheet.save.fortitude"),
        PrintedSheetCellValue::Number(receipt.chassis.total_saves.fortitude)
    );
    // This fixture's Fighter has a genuinely supported *class chassis*
    // (base_attack_bonus / total_saves above are real), but it has no
    // equipped weapon/armor and no Dodge/Weapon Focus feats, so
    // `compute_combat_baseline`'s own deterministic-posture check
    // independently fails and pushes claim-blocking
    // `combat.baseline_unsupported` — contract.rs's `printed_sheet_cell_map`
    // correctly renders `Blocked` for `sheet.armor_class` /
    // `sheet.melee_attack_bonus` here (fixed alongside SD-20 Epic 8's
    // Finding 2; asserting these as `Number(0)` would itself be the same
    // fabricated-zero bug that fix closed).
    assert_eq!(
        *cell_value(&cells, "sheet.armor_class"),
        PrintedSheetCellValue::Blocked
    );
    assert_eq!(
        *cell_value(&cells, "sheet.melee_attack_bonus"),
        PrintedSheetCellValue::Blocked
    );
    assert_eq!(
        *cell_value(&cells, "sheet.ability_modifier.strength"),
        PrintedSheetCellValue::Number(receipt.chassis.ability_modifiers.strength)
    );
    // Likewise, this fixture has no skill allocations at all, so
    // `compute_selected_skill_modifiers`'s own deterministic-posture check
    // independently fails and pushes claim-blocking
    // `skill.selected_modifier.unsupported`, correctly blocking
    // `sheet.skill.climb` here.
    assert_eq!(
        *cell_value(&cells, "sheet.skill.climb"),
        PrintedSheetCellValue::Blocked
    );

    // Every cell must cite exactly one PilotReceipt field (no cell left
    // pointing at nothing) — the contract's auditability requirement.
    assert!(cells.iter().all(|c| !c.source_field.is_empty()));
    // No cell is silently dropped: the map is non-empty.
    assert!(!cells.is_empty());
}

#[test]
fn printed_sheet_cell_map_blocks_chassis_dependent_cells_but_not_independent_ones_when_unsupported()
{
    let corpus = empty_corpus();
    // A wizard-only input has no supported single-class Fighter chassis
    // posture; `class_chassis.unsupported` fires claim_blocking: true, and
    // the chassis's own base_attack_bonus / total_saves / baseline_* /
    // selected_skill_modifiers fields are zeroed (not real data) as a
    // result. The cell map must not show that zero as if it were a real
    // number — it must render `Blocked` for each cell sourced from those
    // fields, per technical-design.md §1.1: "If a cell's source field is
    // claim-blocked, the GUI renders 'blocked — see diagnostics' rather
    // than a fabricated number."
    let input = wizard_level_1_input();
    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);
    assert!(receipt
        .diagnostics
        .iter()
        .any(|d| d.id == "class_chassis.unsupported" && d.claim_blocking));

    let cells = printed_sheet_cell_map(&receipt);

    for blocked_cell_id in [
        "sheet.base_attack_bonus",
        "sheet.save.fortitude",
        "sheet.save.reflex",
        "sheet.save.will",
        "sheet.armor_class",
        "sheet.melee_attack_bonus",
        "sheet.skill.climb",
        "sheet.skill.intimidate",
        "sheet.skill.swim",
    ] {
        assert_eq!(
            *cell_value(&cells, blocked_cell_id),
            PrintedSheetCellValue::Blocked,
            "expected {blocked_cell_id} to render Blocked, not a fabricated zero"
        );
    }

    // Ability modifiers are computed directly from ability scores,
    // independent of chassis support — they must NOT be blocked just
    // because the rest of the chassis is unsupported (blanket-blocking
    // every cell would itself be a form of fabricated/imprecise output).
    assert_eq!(
        *cell_value(&cells, "sheet.ability_modifier.strength"),
        PrintedSheetCellValue::Number(receipt.chassis.ability_modifiers.strength)
    );
    assert_eq!(
        *cell_value(&cells, "sheet.ability_modifier.charisma"),
        PrintedSheetCellValue::Number(receipt.chassis.ability_modifiers.charisma)
    );
}
