//! SD-19 §2.5 equipment card: magic_items — full coverage.
//!
//! Per the operator's amended loop instruction (2026-07-16, "i want to
//! make sure that we brought in ALL spells, ALL armor, ALL weapons, ALL
//! equipment, not just samples"), this proves **every** real-corpus
//! `cr_equip_magic_items.lst` record — not a representative sample — is
//! (a) resolvable via `equipment_id_resolve`, (b) present in
//! `CorpusPilotReceipt.corpus_derived.equipped_items` after a call to
//! `compute_pilot_with_corpus`, and (c) carries a non-`None` `TableCellRef`
//! grounding it to the CRB table store's now-complete
//! `equipment_data::magic_items::MAGIC_ITEMS_TABLE`.
//!
//! Supersedes the prior representative-sample cycle (`1689b16`), which
//! *discovered* the `.COPY=` merge-collapse defect this full-coverage
//! cycle depends on having fixed (`22eeed9`) — that cycle deliberately
//! drew its sample from non-`.COPY=` records to avoid landing on top of
//! a known bug; this cycle now exercises every `.COPY=` variant too.
//! Also required the equipment resolver's exact-name-match fix (added
//! alongside this cycle).
//!
//! Reads the real PCGen corpus directly (the per-cycle `CORPUS_ROOT`
//! pattern from `decisions.md` §6.6), skipping with a documented
//! `eprintln!` when `CORPUS_ROOT` is unset.

use std::path::PathBuf;

use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
use codex_ingest::pcgen_import::lst_parser::equipment::parse_equipment_file;
use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection,
};
use codex::rules_core::equipment_resolver::equipment_id_resolve;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// The record's corpus identity: its own `KEY:` when the source line carried
/// one, else its name.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 13: this was
/// `equipment_resolver::equipment_key_token`, which took an ingest-format
/// parser row and lived on the live side. The rule is settled as
/// `CorpusEquipmentRecord::identity` now, so a test holding a parser row asks
/// the converter for it.
fn row_identity(record: &codex_ingest::pcgen_import::lst_parser::equipment::EquipmentRecord) -> String {
    codex_ingest::pcgen_import::ir_converter::equipment_record_to_corpus(record).identity
}

fn corpus_root() -> Option<PathBuf> {
    match std::env::var("CORPUS_ROOT") {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() { Some(path) } else { None }
        }
        Err(_) => None,
    }
}

fn cr_equip_magic_items_path(root: &std::path::Path) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_magic_items.lst")
}

fn base_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd19_equipment_magic_items".to_string()),
        source_package_id: "sd19_equipment_magic_items".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "fighter".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
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
fn every_real_corpus_item_resolves_reaches_equipped_items_and_grounds_through_table_cell() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping (set \
             CORPUS_ROOT=$HOME/workspace/repos/pcgen/data to enable)"
        );
        return;
    };
    let cr_equip_magic_items = cr_equip_magic_items_path(&root);
    if !cr_equip_magic_items.is_file() {
        eprintln!(
            "canonical cr_equip_magic_items.lst not present at {}; skipping",
            cr_equip_magic_items.display()
        );
        return;
    }

    let parsed = parse_equipment_file(&cr_equip_magic_items)
        .expect("cr_equip_magic_items.lst must parse");
    assert!(
        !parsed.entries.is_empty(),
        "corpus-existence check must find magic_items records"
    );
    assert_eq!(
        parsed.entries.len(),
        1556,
        "expected 1556 distinct magic_items records post-merge-fix; if this \
         changes the corpus or the parser changed — regenerate \
         equipment_data::magic_items"
    );

    let source_ref = SourceRef {
        lst_file: cr_equip_magic_items.display().to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd19_equipment_magic_items", source_ref);
    for record in &parsed.entries {
        corpus.push(convert_equipment_record(record));
    }

    let mut input = base_input();
    for record in &parsed.entries {
        let identity = row_identity(record);
        let identity = identity.as_str();
        let resolved = equipment_id_resolve(identity, RuleSetId::Crb, &corpus);
        let (resolved_record, table_cell) = resolved.unwrap_or_else(|| {
            panic!("expected equipment_id_resolve to resolve '{identity}'")
        });
        assert_eq!(&resolved_record.name, &record.name);
        assert!(
            table_cell.is_some(),
            "expected '{identity}' to ground through the full CRB table store"
        );
        input.chosen.equipment_selections.push(EquipmentSelection {
            item_id: identity.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        });
    }

    let receipt = compute_pilot_with_corpus(&input, &corpus);
    assert_eq!(
        receipt.corpus_derived.equipped_items.len(),
        parsed.entries.len(),
        "expected every one of the {} magic_items records to resolve into its own \
         equipped_items entry",
        parsed.entries.len()
    );
    for item in &receipt.corpus_derived.equipped_items {
        assert!(
            item.table_cell.is_some(),
            "expected equipped_items entry for '{}' to carry a table_cell",
            item.item_id
        );
        assert!(!item.equipment_record_name.is_empty());
    }
}

