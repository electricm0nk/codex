//! SD-41 E2.1, G-1's second leg (`acceptance-and-verification.md` G-1, `decisions.md §8`): a
//! pre-SD-41 character root (`tests/fixtures/sd41-old-save/`, Aldric Ironhand's seed files, no
//! `equipment_sets.json`) loads; the first load writes the Default set; and Armor Class, carried
//! weight, load level and every `defense.armor_class.*` row equal what the base (`tranche-21` @
//! `0d04f5730d`) read from the same files (its `pf_seed_render_hash` render of Aldric).
//!
//! This crate is a binary with no lib target, so its modules are compiled into this test by path,
//! as `main.rs` declares them. The target runs without the libtest harness (`Cargo.toml`
//! `harness = false`): `cfg(test)` is off, so the modules' own suites, which the bin target runs,
//! are neither compiled nor run a second time here. `main` runs the one check and exits non-zero on
//! a failed assertion.
#![allow(dead_code, unused_imports, non_snake_case)]

#[path = "../src/browser_handoff.rs"]
mod browser_handoff;
#[path = "../src/character_custom.rs"]
mod character_custom;
#[path = "../src/character_hub.rs"]
mod character_hub;
#[path = "../src/character_ledger.rs"]
mod character_ledger;
#[path = "../src/class_catalog.rs"]
mod class_catalog;
#[path = "../src/class_facts.rs"]
mod class_facts;
#[path = "../src/class_catalog_generic.rs"]
mod class_catalog_generic;
#[path = "../src/class_feature_descriptions.rs"]
mod class_feature_descriptions;
#[path = "../src/class_feature_feat_bridge.rs"]
mod class_feature_feat_bridge;
#[path = "../src/class_feature_pool_picker.rs"]
mod class_feature_pool_picker;
#[path = "../src/dm_console_export.rs"]
mod dm_console_export;
#[path = "../src/encounter_rating.rs"]
mod encounter_rating;
#[path = "../src/class_spell_levels.rs"]
mod class_spell_levels;
#[path = "../src/corpus_fixtures.rs"]
mod corpus_fixtures;
#[path = "../src/corpus_full.rs"]
mod corpus_full;
#[path = "../src/corpus_ingest_diagnostic.rs"]
mod corpus_ingest_diagnostic;
#[path = "../src/cpr_adapter.rs"]
mod cpr_adapter;
#[path = "../src/cpr_catalog.rs"]
mod cpr_catalog;
#[path = "../src/cpr_creation.rs"]
mod cpr_creation;
#[path = "../src/cpr_improve.rs"]
mod cpr_improve;
#[path = "../src/cpr_sheet_print.rs"]
mod cpr_sheet_print;
#[path = "../src/equipment_catalog.rs"]
mod equipment_catalog;
#[path = "../src/feat_catalog.rs"]
mod feat_catalog;
#[path = "../src/authoring_workbench.rs"]
mod authoring_workbench;
#[path = "../src/companion_catalog.rs"]
mod companion_catalog;
#[path = "../src/companion_pool_catalog.rs"]
mod companion_pool_catalog;
#[path = "../src/intelligent_item_catalog.rs"]
mod intelligent_item_catalog;
#[path = "../src/monster_catalog.rs"]
mod monster_catalog;
#[path = "../src/pf1_adapter.rs"]
mod pf1_adapter;
#[path = "../src/race_catalog.rs"]
mod race_catalog;
#[path = "../src/race_trait_picker.rs"]
mod race_trait_picker;
#[path = "../src/reference_library_catalog.rs"]
mod reference_library_catalog;
#[path = "../src/rule_system_adapter.rs"]
mod rule_system_adapter;
#[path = "../src/sf_adapter.rs"]
mod sf_adapter;
#[path = "../src/sf_catalog.rs"]
mod sf_catalog;
#[path = "../src/sf_choices.rs"]
mod sf_choices;
#[path = "../src/sf_creation.rs"]
mod sf_creation;
#[path = "../src/sf_drone_print.rs"]
mod sf_drone_print;
#[path = "../src/sf_level_up.rs"]
mod sf_level_up;
#[path = "../src/sf_oracle_parity.rs"]
mod sf_oracle_parity;
#[path = "../src/sf_sheet_print.rs"]
mod sf_sheet_print;
#[path = "../src/spell_catalog.rs"]
mod spell_catalog;
#[path = "../src/stub_adapter.rs"]
mod stub_adapter;
#[path = "../src/trait_picker.rs"]
mod trait_picker;
#[path = "../src/ui_probe.rs"]
mod ui_probe;
#[path = "../src/characterHub/mod.rs"]
mod characterHub;
#[path = "../src/print/mod.rs"]
mod print;
#[path = "../src/update/mod.rs"]
mod update;
use codex::rules_core::converted_prose;

use codex::saved_character::equipment_sets;
use std::path::{Path, PathBuf};

fn old_save_root() -> PathBuf {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sd41-old-save");
    let root = std::env::temp_dir().join(format!("codex-sd41-old-save-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("temp root");
    for entry in std::fs::read_dir(&fixture).expect("the fixture root") {
        let entry = entry.expect("a fixture file");
        std::fs::copy(entry.path(), root.join(entry.file_name())).expect("copy a fixture file");
    }
    root
}

fn main() {
    old_save_loads_unchanged();
    println!("test old_save_loads_unchanged ... ok");
}

fn old_save_loads_unchanged() {
    let root = old_save_root();
    assert!(!root.join(equipment_sets::EQUIPMENT_SETS_FILE).exists(), "the fixture is a pre-SD-41 root");

    let sheet = character_hub::load_saved_character_at_root(&root).expect("the old save loads");

    let sets = equipment_sets::load(&root).expect("the sets file reads").expect("the first load wrote it");
    let names: Vec<&str> = sets.sets.iter().map(|set| set.name.as_str()).collect();
    assert_eq!((sets.active.as_str(), names), ("Default", vec!["Default"]));
    let summary = sheet.equipment_sets.as_ref().expect("the response names the sets");
    assert_eq!((summary.active.as_str(), summary.names.clone()), ("Default", vec!["Default".to_owned()]));

    // Read from the same files on the base: `snapshot.baselineArmorClass` 16, 29 lb, Light, and the seven rows.
    assert_eq!(sheet.snapshot.as_ref().expect("Aldric computes").baseline_armor_class, 16, "armor_class");
    assert_eq!(sheet.corpus_derived.encumbrance.total_carried_weight_lbs, 29.0, "total_carried_weight_lbs");
    assert_eq!(sheet.corpus_derived.encumbrance.level, "Light", "level");
    let armor_class_rows: Vec<(&str, i16)> = sheet
        .explanations
        .iter()
        .filter(|row| row.id.starts_with("defense.armor_class."))
        .map(|row| (row.id.as_str(), row.value))
        .collect();
    assert_eq!(
        armor_class_rows,
        vec![
            ("defense.armor_class.armor", 4),
            ("defense.armor_class.shield", 0),
            ("defense.armor_class.dexterity", 1),
            ("defense.armor_class.size", 0),
            ("defense.armor_class.natural", 0),
            ("defense.armor_class.deflection", 0),
            ("defense.armor_class.misc", 1),
        ]
    );
    std::fs::remove_dir_all(&root).ok();
}
