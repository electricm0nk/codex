//! A line Codex has no record for is a Create item held at its place, not a text-only record
//! (SD-42 `decisions.md §7`, `§21` Q-M).
//!
//! The synthetic `fixtures/pcg_unmapped.pcg` holds, in the samples' syntax: 3 of an unknown trinket
//! (no base, no record), an unknown satchel holding 4 rations (a catalog item) and 2 of an unknown
//! charm, and 2 camels Not Carried with `LOADCOMPANIONS:N`.
//!
//! **Does not cover:** an unknown line's `CUSTOMIZATION` (`NAME`, `SPROP`, eqmods) on its Create item;
//! a capacity the file states (PCGen's `.pcg` carries none: 0 of 89 `EQUIPNAME` lines in the operator's
//! file and 0 of the 7 samples hold `CONTAINS`); bundle lines (E1.8); companion files (not followed);
//! the app writing `custom.json` (E4.1); any `.pcg` outside this fixture.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex::saved_character::equipment_sets::{CopyKey, Location, location_of};
use codex_ingest::pcg_import::catalog::Catalog;
use codex_ingest::pcg_import::map::convert;
use codex_ingest::pcg_import::wire::{CustomRecord, ImportFile};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture() -> &'static ImportFile {
    static FILE: OnceLock<ImportFile> = OnceLock::new();
    FILE.get_or_init(|| {
        let catalog = Catalog::load(&repo_root()).expect("the checkout holds converted rules");
        let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pcg_unmapped.pcg")).expect("fixture");
        convert(&text, "pcg_unmapped.pcg", &catalog).expect("converts")
    })
}

fn record(name: &str) -> &'static CustomRecord {
    fixture().custom.equipment.iter().find(|record| record.name == name).unwrap_or_else(|| panic!("{name}: a custom item; {:?}", fixture().custom.equipment))
}

fn the_copy(item_id: &str) -> CopyKey {
    let sets = &fixture().equipment_sets;
    let keys: Vec<CopyKey> = sets
        .copies
        .keys()
        .filter_map(|key| {
            let mut parts = key.rsplitn(3, '|');
            let ordinal = parts.next()?.parse().ok()?;
            let bonus = parts.next()?.parse().ok()?;
            let id = parts.next()?;
            (id == item_id).then(|| CopyKey { item_id: id.to_owned(), bonus, ordinal })
        })
        .collect();
    assert_eq!(keys.len(), 1, "{item_id}: one stack; {:?}", sets.copies.keys().collect::<Vec<_>>());
    keys[0].clone()
}

fn pieces(copy: &CopyKey) -> Option<u32> {
    fixture().equipment_sets.copies.get(&copy.map_key()).and_then(|facts| facts.quantity)
}

fn location(copy: &CopyKey) -> Location {
    location_of(fixture().equipment_sets.active_set(), copy)
}

#[test]
fn unmapped_item_is_a_create_item_at_its_location() {
    let trinket = record("SD42 Unknown Trinket");
    assert_eq!((trinket.weight_lbs, trinket.price_gp, trinket.container_capacity_lbs), (Some(2.0), Some(15.0), None));
    assert!(!trinket.stats.iter().any(|stat| stat.label == "Worn" || stat.label == "Carried"), "no text-only place: {:?}", trinket.stats);
    let copy = the_copy(&format!("custom:{}", trinket.id));
    assert_eq!(pieces(&copy), Some(3));
    // In no entry of the file's set: outside the set, PCGen's Not Carried (`decisions.md §20`, E1.10).
    assert_eq!(location(&copy), Location::NotCarried);
    assert!(
        fixture().report.warnings.iter().any(|warning| warning == "SD42 Unknown Trinket: no Codex record for SD42 Unknown Trinket"),
        "{:?}",
        fixture().report.warnings
    );
}

#[test]
fn unmapped_container_holds_its_contents() {
    let satchel = record("Oddment Satchel");
    // No capacity in the file: the satchel holds exactly what the file puts in it (4 x 1 lb + 2 x 0.5 lb).
    assert_eq!((satchel.weight_lbs, satchel.price_gp, satchel.container_capacity_lbs), (Some(1.5), Some(4.0), Some(5.0)));
    let satchel_copy = the_copy(&format!("custom:{}", satchel.id));
    assert_eq!(location(&satchel_copy), Location::Equipped);
    let inside = Location::Inside { container: satchel_copy };
    let rations = the_copy("Rations (Trail/Per Day)");
    assert_eq!((location(&rations), pieces(&rations)), (inside.clone(), Some(4)));
    let charm = the_copy(&format!("custom:{}", record("Whetstone Charm").id));
    assert_eq!((location(&charm), pieces(&charm)), (inside, Some(2)));
}

#[test]
fn companion_line_is_a_gear_line() {
    let file = fixture();
    let camel_id = file
        .character_input
        .chosen
        .equipment_selections
        .iter()
        .map(|row| row.item_id.clone())
        .find(|id| id.to_lowercase().contains("camel"))
        .unwrap_or_else(|| format!("custom:{}", record("Camel").id));
    let camel = the_copy(&camel_id);
    assert_eq!((location(&camel), pieces(&camel)), (Location::NotCarried, Some(2)));
    assert!(!file.custom.feats.iter().any(|feat| feat.name.contains("Camel")), "a companion line is gear, not a feat");
}
