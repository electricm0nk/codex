//! The PCGen import writes the sets, the Create items and the Notes (SD-42 `decisions.md §5, §7, §10`).
//!
//! Each test runs the converter the `pcg_import` sidecar runs (`codex_ingest::pcg_import::run`, a
//! dev-dependency) on a committed synthetic `.pcg`, then the app's own `import_pcg_json` into a fresh
//! root, so what is asserted is what a player's import writes.
//!
//! **Does not cover:** the sidecar binary itself (the `pcg_sidecar_*` tests run it); the 4 Pathfinder
//! samples and the operator's file (E4.5, E4.13); the screen (E4.2); Starfinder gear on the screen.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codex::saved_character::equipment_sets::{self, CopyKey, Location};
use codex_ingest::pcg_import::wire::ImportFile;

use super::*;

const SAVED_AT: &str = "2026-11-18T00:00:00Z";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("crates/codex-ingest/tests/fixtures").join(name)
}

fn tempdir(label: &str) -> PathBuf {
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("codex-import-sets-{label}-{}-{unique}", std::process::id()));
    std::fs::create_dir_all(&path).unwrap();
    path
}

/// Converts `pcg` as the sidecar does, then imports it as character `id` under `characters`.
fn import(pcg: &Path, characters: &Path, id: &str) -> (PathBuf, ImportFile, ImportPcgResponse) {
    let out = tempdir(&format!("{id}-json")).join("out.json");
    let wire = codex_ingest::pcg_import::run(pcg, &out, &repo_root()).expect("the converter reads the fixture");
    let text = std::fs::read_to_string(&out).unwrap();
    let root = characters.join(id);
    let response = import_pcg_json(&text, &root, id, SAVED_AT, "0.0.0-test").expect("the import runs");
    assert!(matches!(response.outcome, CreateCharacterResponse::Saved { .. }), "{:?}", response.outcome);
    (root, wire, response)
}

/// The held row standing for `copy`; exactly one.
fn row_of<'a>(rows: &'a [HeldEquipmentCopyDto], copy: &CopyKey) -> &'a HeldEquipmentCopyDto {
    let found: Vec<&HeldEquipmentCopyDto> = rows.iter().filter(|row| row.copies.contains(copy)).collect();
    assert_eq!(found.len(), 1, "{copy:?}: {found:?}");
    found[0]
}

fn row_named<'a>(rows: &'a [HeldEquipmentCopyDto], name: &str) -> &'a HeldEquipmentCopyDto {
    let found: Vec<&HeldEquipmentCopyDto> = rows.iter().filter(|row| row.name == name).collect();
    assert_eq!(found.len(), 1, "{name}: {:?}", rows.iter().map(|row| &row.name).collect::<Vec<_>>());
    found[0]
}

#[test]
fn import_writes_one_held_row_per_line_with_quantity() {
    let (root, wire, _) = import(&fixture("pcg_stacks.pcg"), &tempdir("stacks"), "stacks");
    let written = equipment_sets::load(&root).unwrap().expect("the import wrote equipment_sets.json");
    assert_eq!(written, wire.equipment_sets, "the file is the wire's block, not a Default set");

    let rows = list_held_equipment_at_root(&root).unwrap();
    let active = written.active_set();
    for (map_key, facts) in &wire.equipment_sets.copies {
        let Some(copy) = active.locations.iter().map(|entry| &entry.copy).find(|copy| &copy.map_key() == map_key) else { continue };
        let row = row_of(&rows, copy);
        assert_eq!(row.location, equipment_sets::location_of(active, copy), "{map_key}");
        if let Some(pieces) = facts.quantity {
            assert_eq!(row.quantity, pieces, "{map_key}");
        }
        if let Some(name) = &facts.name {
            assert_eq!(&row.name, name, "{map_key}");
        }
    }
    assert_eq!(row_named(&rows, "Rations, Trail").quantity, 84, "84 rations are one row of 84");
}

#[test]
fn import_nested_container_locations_project() {
    let (root, _, response) = import(&fixture("pcg_stacks.pcg"), &tempdir("nested"), "nested");
    let rows = list_held_equipment_at_root(&root).unwrap();
    let backpack = row_named(&rows, "Backpack");
    let pouch = row_named(&rows, "Pouch, Belt");
    let rations = row_named(&rows, "Rations, Trail");
    assert_eq!(backpack.location, Location::Equipped);
    assert_eq!(pouch.location, Location::Inside { container: backpack.copies[0].clone() });
    assert_eq!(rations.location, Location::Inside { container: pouch.copies[0].clone() }, "two containers deep");
    assert_eq!(row_named(&rows, "Bedroll").location, Location::NotCarried);

    // The first load reads the imported set as it stands, and the import answered with that load's weight.
    let before = std::fs::read(root.join("equipment_sets.json")).unwrap();
    let loaded = load_saved_character_at_root(&root).unwrap();
    assert!(loaded.snapshot.is_some(), "the sheet computes");
    assert_eq!(std::fs::read(root.join("equipment_sets.json")).unwrap(), before, "the first load rewrites nothing");
    let CreateCharacterResponse::Saved { corpus_derived, .. } = response.outcome else { unreachable!() };
    assert_eq!(corpus_derived.encumbrance.total_carried_weight_lbs, loaded.corpus_derived.encumbrance.total_carried_weight_lbs);
}

#[test]
fn import_create_items_are_held_and_weighed() {
    let (root, wire, response) = import(&fixture("pcg_unmapped.pcg"), &tempdir("create"), "create");
    let custom = load_character_custom_at_root(&root).unwrap();
    let record = |name: &str| custom.equipment.iter().find(|record| record.name == name).unwrap_or_else(|| panic!("{name}: {:?}", custom.equipment));
    let trinket = record("SD42 Unknown Trinket");
    assert_eq!((trinket.weight_lbs, trinket.price_gp), (Some(2.0), Some(15.0)), "custom.json keeps the line's WT and COST");
    let satchel = record("Oddment Satchel");
    assert_eq!(satchel.container_capacity_lbs, Some(5.0));
    assert_eq!(custom.equipment.len(), wire.custom.equipment.len());

    let rows = list_held_equipment_at_root(&root).unwrap();
    let trinket_row = row_of(&rows, &crate::character_custom::custom_copy(trinket));
    assert_eq!((trinket_row.quantity, &trinket_row.location), (3, &Location::NotCarried));
    let satchel_copy = crate::character_custom::custom_copy(satchel);
    assert_eq!(row_of(&rows, &satchel_copy).location, Location::Equipped);
    assert_eq!(row_named(&rows, "Rations, Trail").location, Location::Inside { container: satchel_copy });

    // Weighed with its pieces: the 3 trinkets (2 lb each) left behind weigh nothing; carried, 6 lb.
    let CreateCharacterResponse::Saved { corpus_derived, .. } = response.outcome else { unreachable!() };
    let before = corpus_derived.encumbrance.total_carried_weight_lbs;
    assert_eq!(before, load_saved_character_at_root(&root).unwrap().corpus_derived.encumbrance.total_carried_weight_lbs);
    let trinket_id = trinket.id.clone();
    set_custom_item_state_at_root(&root, &trinket_id, ActiveState::SelectedInactive, SAVED_AT).unwrap();
    let mut sets = equipment_sets::load(&root).unwrap().unwrap();
    place(active_set_mut(&mut sets), &crate::character_custom::custom_copy(trinket), Location::Carried);
    equipment_sets::save(&root, &sets).unwrap();
    let after = load_saved_character_at_root(&root).unwrap().corpus_derived.encumbrance.total_carried_weight_lbs;
    assert!((after - before - 6.0).abs() < 1e-9, "{before} -> {after}");
}

#[test]
fn import_notes_land_in_bio() {
    let (root, wire, _) = import(&fixture("pcg_unmapped.pcg"), &tempdir("notes"), "notes");
    let bio = load_character_bio_at_root(&root).unwrap();
    assert!(bio.notes.starts_with("## Imported from PCGen"), "{}", bio.notes);
    assert!(bio.notes.contains("SD42 Unknown Trinket"), "{}", bio.notes);
    assert_eq!(bio.notes, wire.bio.notes);
}

#[test]
fn import_custom_eqmods_are_the_buy_custom_modifiers() {
    let (root, _, _) = import(&fixture("pcg_custom.pcg"), &tempdir("eqmods"), "eqmods");
    let envelope = SavedCharacterStore::load(&root).unwrap();
    let rows = list_held_equipment_at_root(&root).unwrap();
    let modifiers_of = |name: &str| {
        let row = row_named(&rows, name);
        let index = selection_copy_keys(&envelope.character_input.chosen.equipment_selections).iter().position(|copy| copy == &row.copies[0]).unwrap();
        envelope.character_input.chosen.equipment_selections[index].applied_modifiers.clone()
    };
    assert_eq!(modifiers_of("Mourn"), ["Special Quality ~ Masterwork ~ Weapon"], "MWORKW is Buy Custom's masterwork");
    assert_eq!(modifiers_of("Darkwood Buckler"), ["Material ~ Darkwood"]);
    assert!(row_named(&rows, "Darkwood Buckler").abilities.iter().any(|line| line.starts_with("Darkwood")), "{:?}", row_named(&rows, "Darkwood Buckler").abilities);
}

#[test]
fn import_never_touches_another_root() {
    let characters = tempdir("two-roots");
    let (first, _, _) = import(&fixture("pcg_stacks.pcg"), &characters, "first");
    let files = |root: &Path| -> BTreeMap<PathBuf, Vec<u8>> {
        std::fs::read_dir(root).unwrap().map(|entry| entry.unwrap().path()).filter(|path| path.is_file()).map(|path| (path.clone(), std::fs::read(&path).unwrap())).collect()
    };
    let before = files(&first);
    assert!(before.contains_key(&first.join("equipment_sets.json")));
    let (second, _, _) = import(&fixture("pcg_stacks.pcg"), &characters, "second");
    assert_ne!(first, second);
    assert_eq!(files(&first), before, "the second import left the first untouched");
    let listed = SavedCharacterStore::list_all(&characters).unwrap();
    assert_eq!(listed.characters.len(), 2, "two imports of one file are two characters");
}

#[test]
fn import_starfinder_file_saves_a_starfinder_character() {
    let repo = match std::env::var("PCGEN_REPO_DIR") {
        Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(std::env::var("HOME").unwrap()).join("workspace/repos/pcgen"),
    };
    let sample = repo.join("code/testsuite/PCGfiles/sf_soldier.pcg");
    if !sample.is_file() {
        println!("SKIPPED: the oracle sample is absent at {} (decisions.md §22)", sample.display());
        return;
    }
    let (root, wire, _) = import(&sample, &tempdir("sf"), "sf");
    assert_eq!(wire.game_system, codex::rules_core::game_system::GameSystem::Starfinder1e.id());
    let envelope = SavedCharacterStore::load(&root).unwrap();
    assert_eq!(envelope.game_system, wire.game_system, "the envelope takes the wire's game system");
    assert_eq!(equipment_sets::load(&root).unwrap().expect("a sets file"), wire.equipment_sets);
    let loaded = crate::rule_system_adapter::load_saved_character_via_envelope(&root).unwrap();
    assert_eq!(loaded.summary.display_label, envelope.display_label);
}
