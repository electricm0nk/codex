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

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::parse_equipment_file;
use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection,
};
use codex::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::support_state_matrix::{
    EvidenceTier, MatrixSubjectType, SupportState, seeded_current_truth,
};

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
        let identity = equipment_key_token(record).unwrap_or(&record.name);
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

#[test]
fn magic_items_matrix_row_reflects_full_coverage() {
    let matrix = seeded_current_truth();
    let row = matrix
        .rows
        .iter()
        .find(|r| r.subject_type == MatrixSubjectType::Equipment(EquipmentCategory::MagicItems))
        .expect("expected an Equipment(MagicItems) row in the seeded matrix");

    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.grounding_ref.contains("sd19_equipment_magic_items"),
        "expected the row's grounding_ref to cite this cycle's proof test, got: {}",
        row.grounding_ref
    );
    assert!(
        row.blocker_or_lossiness_note.contains("1556")
            || row.blocker_or_lossiness_note.to_lowercase().contains("every"),
        "expected the row's note to describe full coverage, got: {}",
        row.blocker_or_lossiness_note
    );
}
