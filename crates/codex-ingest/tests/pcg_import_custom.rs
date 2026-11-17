//! Custom PCGen items keep their name and every property (SD-42 `decisions.md §6`, SD-e).
//!
//! The synthetic `fixtures/pcg_custom.pcg` holds a plain Longsword and six customised lines written
//! by hand in the samples' syntax (no operator line): "Mourn" (a Longsword with a name, the short masterwork
//! code, `_DAMAGE|2d4` and two `SPROP` lines), a Darkwood Buckler (the long material key newer
//! PCGen writes), a Small dagger, a traveler's outfit whose `COSTMOD` brings its `COST` to 0, a
//! heavy flail with an eqmod Buy Custom does not offer and an unknown `DATA` key, and a
//! quarterstaff whose second head (`ALTEQMOD`) deals `1d8`.
//!
//! **Does not cover:** pricing or weighing the eqmods in the app (E4.1) and the engine reading the
//! overrides (E4.2); short eqmod codes outside the Core Rulebook's aliases (they read as text); a
//! line whose `COST`/`WT` matches a record the converter cannot resolve (no override is written);
//! any `.pcg` outside this fixture (the 7 samples run through `pcg_import_stacks`). It is its own
//! file because a second Longsword line would change E1.2's one-Longsword fixture.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex::rules_core::size::SizeCategory;
use codex::saved_character::equipment_sets::CopyFacts;
use codex_ingest::pcg_import::catalog::Catalog;
use codex_ingest::pcg_import::map::convert;
use codex_ingest::pcg_import::wire::{EquipmentWire, ImportFile};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture() -> &'static ImportFile {
    static FILE: OnceLock<ImportFile> = OnceLock::new();
    FILE.get_or_init(|| {
        let catalog = Catalog::load(&repo_root()).expect("the checkout holds converted rules");
        let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pcg_custom.pcg")).expect("fixture");
        convert(&text, "pcg_custom.pcg", &catalog).expect("converts")
    })
}

/// The one stack of `item_id` (its ordinal-0 copy, bonus 0).
fn facts(item_id: &str) -> &'static CopyFacts {
    let copies = &fixture().equipment_sets.copies;
    copies.get(&format!("{item_id}|0|0")).unwrap_or_else(|| panic!("{item_id}: no stack; {:?}", copies.keys().collect::<Vec<_>>()))
}

fn named(name: &str) -> &'static CopyFacts {
    let found: Vec<&CopyFacts> = fixture().equipment_sets.copies.values().filter(|facts| facts.name.as_deref() == Some(name)).collect();
    assert_eq!(found.len(), 1, "{name}: one stack carries the name");
    found[0]
}

/// The last `equipmentSelections` row of `item_id` (file order).
fn row(item_id: &str) -> &'static EquipmentWire {
    fixture().character_input.chosen.equipment_selections.iter().rev().find(|row| row.item_id == item_id).unwrap_or_else(|| panic!("{item_id}: no row"))
}

#[test]
fn custom_name_is_kept() {
    let mourn = named("Mourn");
    assert_eq!(mourn.quantity, Some(1));
    assert_eq!(facts("Longsword").name, None, "the uncustomised longsword keeps the record's name");
    assert_eq!(facts("Dagger").name.as_deref(), Some("Dagger (Small)"));
    assert_eq!(facts("Outfit (Traveler's)").name, None, "a NAME equal to the base is not a custom name");
}

#[test]
fn custom_damage_override_is_read() {
    assert_eq!(named("Mourn").damage_override.as_deref(), Some("2d4"), "_DAMAGE|2d4 in EQMOD");
    assert_eq!(facts("Quarterstaff").damage_override.as_deref(), Some("1d8"), "_DAMAGE|1d8 in ALTEQMOD");
    assert_eq!(facts("Longsword").damage_override, None);
}

#[test]
fn custom_eqmods_map_to_option_keys() {
    let mourn = row("Longsword");
    assert_eq!(mourn.eqmods, vec!["Special Quality ~ Masterwork ~ Weapon".to_owned()], "MWORKW → the masterwork key; _DAMAGE is not an eqmod");
    assert!(mourn.applied_modifiers.is_empty());
    assert_eq!(row("Buckler").eqmods, vec!["Material ~ Darkwood".to_owned()], "a long key passes through");
    assert_eq!(row("Dagger").eqmods, vec!["Material ~ Steel".to_owned()], "STEEL → its material key");
    let warnings = &fixture().report.warnings;
    assert!(!warnings.iter().any(|warning| warning.starts_with("Mourn") || warning.starts_with("Darkwood Buckler")), "{warnings:?}");
}

#[test]
fn custom_unknown_eqmod_is_a_property_and_a_warning() {
    let flail = facts("Flail (Heavy)");
    assert_eq!(flail.properties, vec!["EQMOD DISARM".to_owned(), "CRITRANGE 19".to_owned()]);
    assert!(row("Flail (Heavy)").eqmods.is_empty());
    let warnings = &fixture().report.warnings;
    assert!(warnings.iter().any(|warning| warning == "Heavy Flail: EQMOD DISARM kept as text"), "{warnings:?}");
    assert!(warnings.iter().any(|warning| warning == "Heavy Flail: CRITRANGE kept as text"), "{warnings:?}");
}

#[test]
fn custom_size_cost_weight_overrides() {
    let dagger = facts("Dagger");
    assert_eq!(dagger.size, Some(SizeCategory::Small), "SIZE=S");
    assert_eq!(dagger.weight_override_lbs, Some(0.5), "WT 0.5 differs from the record's 1 lb");
    assert_eq!(dagger.cost_override_copper, None, "COST 2 is the record's price");
    assert_eq!(facts("Outfit (Traveler's)").cost_override_copper, Some(0), "COSTMOD -1 folded into the line's COST 0");
    let mourn = named("Mourn");
    assert_eq!((mourn.cost_override_copper, mourn.weight_override_lbs), (Some(31_500), None), "masterwork 315 gp; the record's 4 lb");
    let plain = facts("Longsword");
    assert_eq!((plain.size, plain.cost_override_copper, plain.weight_override_lbs), (None, None, None), "an uncustomised line writes no override");
}

#[test]
fn custom_sprop_lines_are_properties() {
    assert_eq!(named("Mourn").properties, vec!["Keen edge".to_owned(), "Glows faintly".to_owned()]);
    let json = serde_json::to_value(fixture()).expect("serialises");
    let back: ImportFile = serde_json::from_value(json).expect("the wire reads back");
    assert_eq!(&back, fixture());
}
