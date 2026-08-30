//! SD-20 Epic 2-7 -> boundary-contract wiring project, Cycle 2
//! (`contract:spellbook_wiring`, per `adaptive-squishing-mccarthy.md`).
//!
//! Before this cycle, `PilotReceipt` never called Epic 2's real
//! `spellbook::compute_spellbook_coverage` at all (`tests/
//! sd20_tabletop_readiness_integration.rs`'s Finding 1) -- spell coverage
//! was reachable only via a direct call to the engine, never through the
//! boundary contract. This file proves two things about the fix:
//!
//! (a) **Parity**: `to_pilot_receipt(...).spellbook` is exactly what a
//!     direct call to `compute_spellbook_coverage(&input, &corpus)`
//!     produces for the same input/corpus -- the wiring composes the
//!     engine's real output, it does not reinterpret or duplicate it.
//!     Exercised with a real caster (a level-1 Wizard with `Shield`
//!     prepared), the same scenario `tests/fixtures/wire/sd20/
//!     spellbook_parity.json` uses, so the parity assertion is not
//!     vacuously true over an empty `SpellbookCoverage`.
//! (b) **Honest emptiness, promoted to the wired path**: a Fighter with
//!     no `spells_selected` produces a genuinely empty
//!     `SpellbookCoverage` (`epic_probe_spellbook_is_honestly_empty_for_a_non_caster_fighter`'s
//!     precedent, now proven through `to_pilot_receipt` itself) and zero
//!     `sheet.spellbook.*` cells -- not a fabricated placeholder cell.
//!
//! Per `adaptive-squishing-mccarthy.md`'s "Not every epic output becomes
//! a sheet cell" design decision, only `spell_save_dc` becomes
//! `sheet.spellbook.*` cells (one dynamic cell per present `BTreeMap` key:
//! `sheet.spellbook.spell_save_dc.<class_id>`). `spells_prepared`,
//! `spells_known`, and `school_specialization` do not reduce to
//! `PrintedSheetCellValue::Number(i16) | Blocked` cleanly and stay
//! reachable via `receipt.spellbook` directly.
//!
//! **`slots_total`/`slots_used` were removed from `SpellbookCoverage`
//! entirely** (epic-31-spell-wiring gap closure, 2026-08-07, `decisions.md`
//! Decision 37): they were declared but never populated by
//! `compute_spellbook_coverage`; populating them would have duplicated the already-real,
//! already-tested `class_spell.*.total_spells_per_day.*` chassis
//! computation (`pilot_compute.rs`) that the desktop app's "Spells per
//! day" section already renders via `spellsPerDayModel.ts`. The fields
//! were deleted rather than filled -- see that decision for the full
//! reasoning.

use codex::pcgen_import::ir_converter::convert_spell_record;
use codex::pcgen_import::lst_parser::spell::parse_lst_spell_row;
use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    SpellSelection,
};
use codex::rules_core::contract::{
    printed_sheet_cell_map, to_pilot_receipt, PrintedSheetCellValue,
};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::spellbook::compute_spellbook_coverage;

fn empty_corpus(name: &str) -> SourcePackageContent<'static> {
    let source_ref = SourceRef {
        lst_file: name.to_string(),
        line: 1,
    };
    SourcePackageContent::empty(name.to_string(), source_ref)
}

/// A level-1 Wizard with `Shield` prepared -- the same scenario
/// `tests/fixtures/wire/sd20/spellbook_parity.json` uses, built directly
/// in Rust (per this cycle's brief: "reuse spellbook_parity.json's
/// scenario or build a similar one").
fn wizard_with_shield_prepared() -> (CharacterInput, SourcePackageContent<'static>) {
    let raw_line = "Shield\tSCHOOL:Abjuration\tCLASSES:Sorcerer=1,Wizard=1\tDESCRIPTION:Invisible shield of force.";
    let parsed = parse_lst_spell_row("sd20_contract_spellbook_wiring", 1, raw_line);
    let record = parsed
        .record
        .expect("fixture-shaped raw_lst_row must parse into a real LstSpellRecord");
    let source_ref = SourceRef {
        lst_file: "sd20_contract_spellbook_wiring".to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd20_contract_spellbook_wiring", source_ref);
    let record: &'static _ = Box::leak(Box::new(record));
    corpus.push(convert_spell_record(record));

    let input = CharacterInput {
        case_id: Some("sd20_contract_spellbook_wiring".to_string()),
        source_package_id: "sd20_contract_spellbook_wiring".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:wizard".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 17,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: vec![SpellSelection {
                spell_id: "Shield".to_string(),
                source_class_id: "class:wizard".to_string(),
                acquisition_mode: AcquisitionMode::Prepared,
            }],
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    };

    (input, corpus)
}

/// A level-1 Fighter with no spells at all.
fn fighter_with_no_spells() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_spellbook_wiring_fighter".to_string()),
        source_package_id: "sd20_contract_spellbook_wiring_fighter".to_string(),
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
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn pilot_receipt_spellbook_field_matches_a_direct_compute_spellbook_coverage_call() {
    let (input, corpus) = wizard_with_shield_prepared();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    let direct = compute_spellbook_coverage(&input, &corpus);

    assert_eq!(
        receipt.spellbook, direct,
        "PilotReceipt.spellbook must be exactly what a direct compute_spellbook_coverage(&input, \
         &corpus) call produces for the same input/corpus -- the wiring must compose the engine's \
         real output, not reinterpret or duplicate it"
    );

    // Sanity: this fixture's scenario really does resolve a real
    // (non-empty) spells_prepared + spell_save_dc, so the parity
    // assertion above is not vacuously true over an empty
    // SpellbookCoverage.
    assert_eq!(direct.spells_prepared.len(), 1);
    assert_eq!(
        direct.spell_save_dc.get("class:wizard").copied(),
        Some(14),
        "10 + spell level 1 + INT mod (+3 for a 17) = 14"
    );
}

#[test]
fn spell_save_dc_cell_exists_and_sources_from_spellbook_spell_save_dc() {
    let (input, corpus) = wizard_with_shield_prepared();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);
    let cells = printed_sheet_cell_map(&receipt);

    let dc_cell = cells
        .iter()
        .find(|cell| cell.cell_id == "sheet.spellbook.spell_save_dc.class:wizard")
        .expect("sheet.spellbook.spell_save_dc.class:wizard cell must exist for this caster");

    assert_eq!(dc_cell.value, PrintedSheetCellValue::Number(14));
}

#[test]
fn fighter_with_no_spells_produces_an_honestly_empty_spellbook_and_zero_spellbook_cells() {
    let input = fighter_with_no_spells();
    let corpus = empty_corpus("sd20_contract_spellbook_wiring_fighter");

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    assert_eq!(
        receipt.spellbook,
        codex::rules_core::spellbook::SpellbookCoverage::default(),
        "a Fighter with no spells_selected must produce a genuinely empty SpellbookCoverage \
         through the wired path, not a fabricated one -- promoting \
         epic_probe_spellbook_is_honestly_empty_for_a_non_caster_fighter's direct-call precedent \
         to the real to_pilot_receipt path"
    );

    let cells = printed_sheet_cell_map(&receipt);
    assert!(
        !cells
            .iter()
            .any(|cell| cell.cell_id.starts_with("sheet.spellbook.")),
        "a non-caster must produce zero sheet.spellbook.* cells of any kind: {:?}",
        cells
            .iter()
            .filter(|cell| cell.cell_id.starts_with("sheet.spellbook."))
            .collect::<Vec<_>>()
    );
}
