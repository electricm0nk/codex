//! Starfinder import (SD-42 `decisions.md §8`, `§21` Q-A): a file of PCGen's `Starfinder` game mode
//! reads the Starfinder package's records, is saved as a Starfinder character (`gameSystem`), and the
//! Starfinder engine computes it; a companion file (the drone) is refused by name.
//!
//! **Does not cover:** whether each imported total equals PCGen's (E1.7's parity harness); Starfinder
//! spells (none of the 3 Starfinder samples casts; a spell line becomes a custom spell), class-feature
//! picks (fighting styles, gear boosts, mechanic tricks: custom feats, not engine choices), a human's
//! `+2 Racial Stat Bonus` pick (no sample is human); the app's own write of `gameSystem` (E4.1); any
//! Starfinder `.pcg` outside the 3 samples of the pinned oracle.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex::saved_character::equipment_sets::{CopyKey, Location, location_of};
use codex_ingest::pcg_import::catalog::Catalog;
use codex_ingest::pcg_import::map::{SF_SOURCE_PACKAGE_ID, character_input_of, convert, starfinder_build, starfinder_verdict};
use codex_ingest::pcg_import::parse::parse;
use codex_ingest::pcg_import::wire::ImportFile;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| Catalog::load(&repo_root()).expect("the checkout holds converted rules"))
}

fn sample(name: &str) -> Option<String> {
    let dir = match std::env::var("PCGEN_REPO_DIR") {
        Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(std::env::var("HOME").expect("HOME")).join("workspace/repos/pcgen"),
    };
    let path = dir.join("code/testsuite/PCGfiles").join(format!("{name}.pcg"));
    match std::fs::read(&path) {
        Ok(bytes) => Some(String::from_utf8_lossy(&bytes).into_owned()),
        Err(_) => {
            println!("SKIPPED: the oracle's {name}.pcg is absent at {} (decisions.md §22)", path.display());
            None
        }
    }
}

fn verdict(file: &ImportFile) -> Result<(), String> {
    let package = catalog().for_game_mode("Starfinder").expect("the Starfinder catalog").starfinder_package().expect("its package");
    starfinder_verdict(package, &character_input_of(file))
}

#[test]
fn sf_soldier_imports_as_starfinder() {
    let Some(text) = sample("sf_soldier") else { return };
    let file = convert(&text, "sf_soldier.pcg", catalog()).expect("converts");
    assert_eq!(file.game_system, "starfinder-1e");
    assert_eq!(file.character_input.source_package_id, SF_SOURCE_PACKAGE_ID);
    let chosen = &file.character_input.chosen;
    assert_eq!(chosen.race_id, "core:race:android");
    assert_eq!((chosen.class_levels[0].class_id.as_str(), chosen.class_levels[0].level), ("core:class:soldier", 10));
    assert!(chosen.selected_feats.iter().any(|id| id == "core:ability:ace_pilot"), "the theme: {:?}", chosen.selected_feats);
    assert!(chosen.selected_feats.iter().any(|id| id == "core:feat:barricade"), "{:?}", chosen.selected_feats);
    assert!(chosen.selected_choices.iter().any(|c| c.choice_set_id == "core:class:soldier" && c.selection_id == "DEX"), "the key ability");
    // One allocation per SKILL line whose CLASSBOUGHT buys ranks; the `Display ~` lines buy none.
    let ranked = parse(&text).iter().filter(|line| line.key() == "SKILL" && line.get("CLASSBOUGHT").is_some()).count();
    assert_eq!(chosen.skill_allocations.len(), ranked, "{:?}", chosen.skill_allocations);
    assert!(chosen.skill_allocations.iter().any(|s| s.skill_id == "acrobatics" && s.ranks == 10));
    assert!(file.report.unmapped_classes.is_empty() && file.report.unmapped_race.is_none());
    assert!(!file.report.warnings.iter().any(|w| w.contains("game mode")), "{:?}", file.report.warnings);
    assert!(!file.bio.notes.contains("Classes with no Codex record"), "{}", file.bio.notes);
    verdict(&file).expect("the Starfinder engine computes the soldier");

    // The file's 1000 credits left are the engine's credits remaining, not Table 11-5's wealth less the gear.
    let package = catalog().for_game_mode("Starfinder").unwrap().starfinder_package().unwrap();
    let (build, loadout) = starfinder_build(package, &character_input_of(&file));
    let carried = codex::rules_core::pilot_compute::sf_loadout::compute(package, &build, &loadout).expect("the loadout computes");
    assert_eq!(carried.credits_remaining.total, 1000);
}

#[test]
fn sf_mechanic_imports_with_its_gear() {
    let Some(text) = sample("sf_mechanic") else { return };
    let file = convert(&text, "sf_mechanic.pcg", catalog()).expect("converts");
    assert_eq!(file.game_system, "starfinder-1e");
    verdict(&file).expect("the Starfinder engine computes the mechanic");

    let lines = parse(&text);
    let equip: Vec<_> = lines.iter().filter(|line| line.key() == "EQUIPNAME").collect();
    let pieces: u64 = equip.iter().map(|line| line.get("QUANTITY").and_then(|q| q.trim().parse::<f64>().ok()).map_or(1, |q| q.round() as u64)).sum();
    let rows = &file.character_input.chosen.equipment_selections;
    assert_eq!(rows.len() + file.custom.equipment.len(), equip.len(), "every line is a held row or a Create item");
    assert!(rows.iter().all(|row| row.item_id.contains(":equipment:")), "Starfinder records: {rows:?}");
    // The configurable Computer states no price the engine can carry: a Create item at its place.
    let computer = file.custom.equipment.iter().find(|record| record.name.starts_with("Computer (")).expect("the computer is a Create item");
    let computer_key = CopyKey { item_id: format!("custom:{}", computer.id), bonus: 0, ordinal: 0 };
    assert_eq!(location_of(file.equipment_sets.active_set(), &computer_key), Location::Equipped);
    let stacked: u64 = file.equipment_sets.copies.values().map(|facts| u64::from(facts.quantity.expect("a stack states its pieces"))).sum();
    assert_eq!(stacked, pieces, "pieces conserved (the Credit line holds 100,050)");

    // Worn is Equipped, armour included; Carried stays Carried.
    let state = |id: &str| rows.iter().find(|row| row.item_id == id).map(|row| row.active_state.as_str());
    assert_eq!(state("core:equipment:abadarcorp_travel_suit_silver"), Some("EquippedActive"));
    assert_eq!(state("armory:equipment:travel_treads_mk_2"), Some("SelectedInactive"));
    assert!(file.equipment_sets.active_set().locations.iter().all(|entry| entry.location == Location::Equipped), "no Pathfinder body slot");
}

#[test]
fn sf_drone_per_q_a() {
    let Some(text) = sample("sf_mechanic_drone") else { return };
    let err = convert(&text, "sf_mechanic_drone.pcg", catalog()).expect_err("a drone file is a companion");
    assert!(err.starts_with("sf_mechanic_drone.pcg is a Drone file, a companion of "), "{err}");
    assert!(err.contains("import the master (sf_mechanic.pcg)"), "{err}");
    assert!(err.contains("The Starfinder engine does not compute it alone: sf_chassis."), "the engine's own refusal: {err}");
}
