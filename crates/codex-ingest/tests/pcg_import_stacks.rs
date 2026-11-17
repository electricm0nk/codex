//! One stack per placement of an `EQUIPNAME` line, located from the `CALCEQUIPSET` set's tree
//! (SD-42 `decisions.md §5`, SD-a, SD-c, SD-d).
//!
//! The synthetic `fixtures/sd42_stacks.pcg` is written by hand in the samples' syntax: 84 rations in a
//! pouch in a backpack, a worn armour, a wielded sword, a Torso outfit, a Not Carried bedroll, a
//! `QUANTITY:0` torch, a `QUANTITY:100050` line, and one javelin line split over two places.
//!
//! **Does not cover:** sets other than `CALCEQUIPSET`; PCGen's bundle items, whose `QUANTITY` counts
//! bundles while `CopyFacts.quantity` counts pieces (discovered card E1.8); custom records past the
//! 200 a character holds (no copy is made for them); any `.pcg` outside the fixture and the 7 samples.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex::rules_core::body_slots::BodySlot;
use codex::saved_character::equipment_sets::{CopyKey, EquipmentSets, Location, location_of, project};
use codex_ingest::pcg_import::catalog::Catalog;
use codex_ingest::pcg_import::map::{PCGEN_SLOTS, convert};
use codex_ingest::pcg_import::parse::parse;
use codex_ingest::pcg_import::wire::ImportFile;

const SAMPLES: [&str; 7] = ["pf_Cleric", "pf_Paladin", "pf_Rogue", "pf_goldielocks", "sf_mechanic", "sf_mechanic_drone", "sf_soldier"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| Catalog::load(&repo_root()).expect("the checkout holds converted rules"))
}

fn oracle_repo() -> PathBuf {
    match std::env::var("PCGEN_REPO_DIR") {
        Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(std::env::var("HOME").expect("HOME")).join("workspace/repos/pcgen"),
    }
}

fn fixture() -> &'static ImportFile {
    static FILE: OnceLock<ImportFile> = OnceLock::new();
    FILE.get_or_init(|| {
        let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sd42_stacks.pcg")).expect("fixture");
        convert(&text, "sd42_stacks.pcg", catalog()).expect("converts")
    })
}

/// The copies of `item_id` the sets file holds, in ordinal order.
fn copies_of(sets: &EquipmentSets, item_id: &str) -> Vec<CopyKey> {
    let mut keys: Vec<CopyKey> = sets
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
    keys.sort();
    keys
}

fn the_copy(sets: &EquipmentSets, item_id: &str) -> CopyKey {
    let copies = copies_of(sets, item_id);
    assert_eq!(copies.len(), 1, "{item_id}: one stack; sets: {:?}", sets.copies.keys().collect::<Vec<_>>());
    copies[0].clone()
}

fn quantity(sets: &EquipmentSets, copy: &CopyKey) -> Option<u32> {
    sets.copies.get(&copy.map_key()).and_then(|facts| facts.quantity)
}

#[test]
fn stacks_one_copy_per_line_with_quantity() {
    let file = fixture();
    let sets = &file.equipment_sets;
    let rations = the_copy(sets, "Rations (Trail/Per Day)");
    assert_eq!(quantity(sets, &rations), Some(84), "QUANTITY:84.0 is one stack of 84");
    let rows = file.character_input.chosen.equipment_selections.iter().filter(|row| row.item_id == "Rations (Trail/Per Day)").count();
    assert_eq!(rows, 1, "one equipmentSelections row for the stack");
    // One line in two places is two stacks (SD-c): 3 carried, 2 in the backpack.
    let javelins = copies_of(sets, "Javelin");
    assert_eq!(javelins.len(), 2, "{javelins:?}");
    let mut placed: Vec<(Option<u32>, Location)> = javelins.iter().map(|copy| (quantity(sets, copy), location_of(sets.active_set(), copy))).collect();
    placed.sort_by_key(|(pieces, _)| *pieces);
    assert_eq!(placed[0], (Some(2), Location::Inside { container: the_copy(sets, "Backpack") }));
    assert_eq!(placed[1], (Some(3), Location::Carried));
    assert_eq!(sets.active, "Default Set", "the CALCEQUIPSET set's own name");
}

#[test]
fn stacks_nested_container_location() {
    let sets = &fixture().equipment_sets;
    let set = sets.active_set();
    let backpack = the_copy(sets, "Backpack");
    let pouch = the_copy(sets, "Pouch (Belt)");
    let rations = the_copy(sets, "Rations (Trail/Per Day)");
    assert_eq!(location_of(set, &backpack), Location::Equipped, "EQUIPSET:Equipped");
    assert_eq!(location_of(set, &pouch), Location::Inside { container: backpack.clone() });
    assert_eq!(location_of(set, &rations), Location::Inside { container: pouch }, "two containers deep");
}

#[test]
fn stacks_slot_table_covers_the_oracle_slots() {
    let sets = &fixture().equipment_sets;
    let set = sets.active_set();
    assert_eq!(location_of(set, &the_copy(sets, "Chain Shirt")), Location::Slot { slot: BodySlot::Armor });
    assert_eq!(location_of(set, &the_copy(sets, "Longsword")), Location::Slot { slot: BodySlot::WieldMain }, "Primary Hand");
    assert_eq!(location_of(set, &the_copy(sets, "Outfit (Explorer's)")), Location::Equipped, "Torso holds no Codex slot");
    assert_eq!(location_of(set, &the_copy(sets, "Bedroll")), Location::NotCarried);

    // Every row's state is the set's projection, so the app's load keeps each location.
    let chosen = &fixture().character_input.chosen;
    let state = |id: &str| chosen.equipment_selections.iter().find(|row| row.item_id == id).map(|row| row.active_state.clone()).unwrap();
    assert_eq!((state("Chain Shirt"), state("Bedroll"), state("Rations (Trail/Per Day)")), ("EquippedActive".into(), "Absent".into(), "SelectedInactive".into()));

    let list = oracle_repo().join("system/gameModes/Pathfinder/equipmentslots.lst");
    let Ok(text) = std::fs::read_to_string(&list) else {
        println!("SKIPPED: the oracle's equipmentslots.lst is absent at {} (decisions.md §22)", list.display());
        return;
    };
    let names: Vec<&str> = text.lines().filter_map(|line| line.strip_prefix("EQSLOT:")).map(|rest| rest.split('\t').next().unwrap_or("").trim()).collect();
    assert_eq!(names.len(), 26, "EQSLOT lines in the pinned oracle's Pathfinder list");
    let missing: Vec<&&str> = names.iter().filter(|name| !PCGEN_SLOTS.iter().any(|(word, _)| word == *name)).collect();
    assert!(missing.is_empty(), "slot words the table lacks: {missing:?}");
    println!("oracle EQSLOT lines {}; PCGEN_SLOTS rows {}", names.len(), PCGEN_SLOTS.len());
}

#[test]
fn stacks_zero_quantity_is_noted_not_held() {
    let file = fixture();
    assert!(copies_of(&file.equipment_sets, "Torch").is_empty());
    assert!(!file.character_input.chosen.equipment_selections.iter().any(|row| row.item_id == "Torch"));
    assert!(file.report.warnings.iter().any(|warning| warning == "Torch: 0 copies in the file"), "{:?}", file.report.warnings);
}

#[test]
fn stacks_large_quantity_round_trips() {
    let file = fixture();
    let caltrops = the_copy(&file.equipment_sets, "Caltrops");
    assert_eq!(quantity(&file.equipment_sets, &caltrops), Some(100_050), "no cap");
    let json = serde_json::to_value(file).expect("serialises");
    let sets: EquipmentSets = serde_json::from_value(json["equipmentSets"].clone()).expect("equipmentSets reads as equipment_sets.json");
    assert_eq!(&sets, &file.equipment_sets);
    let back: ImportFile = serde_json::from_value(json).expect("the wire reads back");
    assert_eq!(&back, file);
}

/// Each sample: every non-zero line is held in at least one stack, the pieces in the stacks equal
/// the file's `QUANTITY` summed over those lines, and each row's state is the set's projection.
#[test]
fn stacks_samples_emit_one_copy_per_line() {
    let dir = oracle_repo().join("code/testsuite/PCGfiles");
    if !dir.is_dir() {
        println!("SKIPPED: the oracle's PCGfiles are absent at {} (decisions.md §22)", dir.display());
        return;
    }
    for sample in SAMPLES {
        let path = dir.join(format!("{sample}.pcg"));
        let text = String::from_utf8_lossy(&std::fs::read(&path).expect("sample")).into_owned();
        let file = convert(&text, &format!("{sample}.pcg"), catalog()).expect("converts");
        let serialised = serde_json::to_value(&file).expect("serialises");
        let sets: EquipmentSets = serde_json::from_value(serialised["equipmentSets"].clone()).expect("reads as equipment_sets.json");

        let lines = parse(&text);
        let quantities: Vec<u64> = lines
            .iter()
            .filter(|line| line.key() == "EQUIPNAME")
            .map(|line| line.get("QUANTITY").and_then(|q| q.trim().parse::<f64>().ok()).map_or(1, |q| q.round() as u64))
            .collect();
        let held_lines = quantities.iter().filter(|q| **q > 0).count();
        let pieces: u64 = quantities.iter().sum();
        let copies = sets.copies.len();
        let stacked: u64 = sets.copies.values().map(|facts| u64::from(facts.quantity.expect("every stack states its pieces"))).sum();
        println!("{sample}: lines {} zero {} copies {copies} pieces {pieces} stacked {stacked}", quantities.len(), quantities.len() - held_lines);
        assert!(copies >= held_lines, "{sample}: {copies} stacks for {held_lines} held lines");
        assert_eq!(stacked, pieces, "{sample}: pieces conserved");

        let set = sets.active_set();
        let selections = &file.character_input.chosen.equipment_selections;
        let keys = codex::saved_character::equipment_sets::copy_keys(selections.iter().map(|row| {
            let bonus = (1..=10u8).filter(|n| row.applied_modifiers.iter().any(|m| m.contains(&format!("+{n} ~")))).max().unwrap_or(0);
            (row.item_id.as_str(), bonus)
        }));
        for (row, key) in selections.iter().zip(&keys) {
            let projected = format!("{:?}", project(set, std::slice::from_ref(key))[0]);
            assert_eq!(row.active_state, projected, "{sample}: {}", key.map_key());
        }
    }
}
