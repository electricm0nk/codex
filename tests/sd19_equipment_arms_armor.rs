//! SD-19 §2.5 equipment card: arms_armor — full coverage.
//!
//! Per the operator's amended loop instruction (2026-07-16, "i want to
//! make sure that we brought in ALL spells, ALL armor, ALL weapons, ALL
//! equipment, not just samples"), this proves **every** real-corpus
//! `cr_equip_arms_armor.lst` record — not a representative sample — is
//! (a) resolvable via `equipment_id_resolve`, (b) present in
//! `CorpusPilotReceipt.corpus_derived.equipped_items` after a call to
//! `compute_pilot_with_corpus`, and (c) carries a non-`None` `TableCellRef`
//! grounding it to the CRB table store's now-complete
//! `equipment_data::arms_armor::ARMS_ARMOR_TABLE` (generated from this
//! same real corpus file — see that module's own doc comment for the
//! generation method).
//!
//! This supersedes the prior representative-sample cycle (`e08607e`),
//! which was correct against the original scope doc's "representative
//! sample" criterion but not against the amended "every item" criterion.
//! It required the SD-17 parser-merge fix (`22eeed9`) to land first —
//! without it, `.COPY=` records and coincidentally-same-named plain
//! records collapsed into merged `EquipmentRecord`s, making full
//! per-item resolution structurally impossible.
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

fn cr_equip_arms_armor_path(root: &std::path::Path) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst")
}

fn base_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd19_equipment_arms_armor".to_string()),
        source_package_id: "sd19_equipment_arms_armor".to_string(),
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
    let cr_equip_arms_armor = cr_equip_arms_armor_path(&root);
    if !cr_equip_arms_armor.is_file() {
        eprintln!(
            "canonical cr_equip_arms_armor.lst not present at {}; skipping",
            cr_equip_arms_armor.display()
        );
        return;
    }

    let parsed = parse_equipment_file(&cr_equip_arms_armor)
        .expect("cr_equip_arms_armor.lst must parse");
    assert!(
        !parsed.entries.is_empty(),
        "corpus-existence check must find arms_armor records"
    );
    assert_eq!(
        parsed.entries.len(),
        310,
        "expected 310 distinct arms_armor records post-merge-fix; if this \
         changes the corpus or the parser changed — regenerate \
         equipment_data::arms_armor"
    );

    let source_ref = SourceRef {
        lst_file: cr_equip_arms_armor.display().to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd19_equipment_arms_armor", source_ref);
    for record in &parsed.entries {
        corpus.push(convert_equipment_record(record));
    }

    // Every record's own canonical identity (KEY: token, falling back to
    // name) must resolve, and its table_cell must now be grounded (the
    // table store was generated from this exact corpus file).
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
        "expected every one of the {} arms_armor records to resolve into its own \
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
fn arms_armor_matrix_row_reflects_full_coverage() {
    let matrix = seeded_current_truth();
    let row = matrix
        .rows
        .iter()
        .find(|r| r.subject_type == MatrixSubjectType::Equipment(EquipmentCategory::ArmsArmor))
        .expect("expected an Equipment(ArmsArmor) row in the seeded matrix");

    // Full reachability + full table-cell grounding achieved this cycle,
    // and now promoted to Supported/Product-visible: the desktop app's
    // Equipment Catalog browser (apps/desktop/src/equipmentCatalog/
    // EquipmentCatalogScreen.tsx) surfaces the *full* category via the
    // list_equipment_catalog Tauri command, satisfying the loop
    // instruction's own definition of Supported/Product-visible.
    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.grounding_ref.contains("sd19_equipment_arms_armor"),
        "expected the row's grounding_ref to cite this cycle's proof test, got: {}",
        row.grounding_ref
    );
    assert!(
        row.blocker_or_lossiness_note.contains("310")
            || row.blocker_or_lossiness_note.to_lowercase().contains("every"),
        "expected the row's note to describe full coverage, got: {}",
        row.blocker_or_lossiness_note
    );
}
