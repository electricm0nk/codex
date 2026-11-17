//! The Default set round-trips every `.pcg` import fixture (`decisions.md` §8): for each file the
//! converter reads, `project(default_from(x), x)` equals the imported `active_state` copy by copy.
//! The population is `pcg_import::fixtures_tests::fixtures()`'s (that function is private to the
//! crate's own test module, so it is walked here the same way): the repo's 13 eqm fixtures and the
//! pinned oracle's `code/testsuite/PCGfiles`.
//!
//! **Does not cover:** the desktop's derived slot per copy (each worn copy is given `BodySlot::None`
//! and then `BodySlot::Armor`, the two `default_from` branches; the slot never changes the
//! projection), custom items (`custom:<id>`, E2.1), or a `.pcg` no fixture holds.

use std::path::{Path, PathBuf};

use codex::rules_core::body_slots::{BodySlot, Enhanceable};
use codex::rules_core::character_input::ActiveState;
use codex::saved_character::equipment_sets::{copy_keys, default_from, project, HeldCopy};
use codex_ingest::pcg_import::catalog::Catalog;
use codex_ingest::pcg_import::map::convert;
use codex_ingest::pcg_import::wire::EquipmentWire;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn oracle_dir() -> PathBuf {
    match std::env::var("PCGEN_REPO_DIR") {
        Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(std::env::var("HOME").expect("HOME")).join("workspace/repos/pcgen"),
    }
}

fn pcg_files_under(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            pcg_files_under(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "pcg") {
            out.push(path);
        }
    }
}

fn active_state(wire: &EquipmentWire) -> ActiveState {
    match wire.active_state.as_str() {
        "EquippedActive" => ActiveState::EquippedActive,
        "SelectedInactive" => ActiveState::SelectedInactive,
        "Absent" => ActiveState::Absent,
        other => panic!("not an active state: {other}"),
    }
}

/// The `+N` of a copy by its enhancement eqmod.
fn bonus_of(wire: &EquipmentWire) -> u8 {
    (1..=10u8)
        .filter(|n| [Enhanceable::Armor, Enhanceable::Shield, Enhanceable::Weapon].iter().any(|kind| wire.applied_modifiers.contains(&kind.modifier_key(*n))))
        .max()
        .unwrap_or(0)
}

#[test]
fn equipment_sets_default_round_trips_every_pcg_fixture() {
    let mut files = Vec::new();
    pcg_files_under(&repo_root().join("scripts/oracle_harness/eqm-fixtures"), &mut files);
    assert_eq!(files.len(), 13, "scripts/oracle_harness/eqm-fixtures/*.pcg");
    pcg_files_under(&oracle_dir(), &mut files);
    assert!(files.len() - 13 >= 42, "the pinned oracle's code/testsuite/PCGfiles holds 42 .pcg; found {}", files.len() - 13);
    files.sort();
    let catalog = Catalog::load(&repo_root()).expect("the checkout holds converted rules");
    let (mut converted, mut refused, mut with_equipment, mut copies) = (0usize, 0usize, 0usize, 0usize);
    let mut states = std::collections::BTreeSet::new();
    for path in &files {
        let text = String::from_utf8_lossy(&std::fs::read(path).expect("fixture")).into_owned();
        let Ok(file) = convert(&text, &path.display().to_string(), &catalog) else {
            refused += 1;
            continue;
        };
        converted += 1;
        let selections = &file.character_input.chosen.equipment_selections;
        if !selections.is_empty() {
            with_equipment += 1;
        }
        let keys = copy_keys(selections.iter().map(|wire| (wire.item_id.as_str(), bonus_of(wire))));
        let saved: Vec<ActiveState> = selections.iter().map(active_state).collect();
        states.extend(saved.iter().map(|state| format!("{state:?}")));
        for slot in [BodySlot::None, BodySlot::Armor] {
            let held: Vec<HeldCopy> = keys.iter().zip(&saved).map(|(copy, state)| HeldCopy { copy: copy.clone(), active_state: *state, slot }).collect();
            let sets = default_from(&held);
            assert_eq!(project(sets.active_set(), &keys), saved, "{} (slot {slot:?})", path.display());
        }
        copies += keys.len();
    }
    println!(
        "equipment_sets_pcg_population: {converted} of {converted} fixtures round-trip ({} files, {refused} refused by the converter as older than the format; {with_equipment} hold equipment, {copies} copies; states {states:?})",
        files.len()
    );
    assert!(refused >= 1, "none_PdfBuster.pcg is refused by the converter");
    assert!(with_equipment >= 13, "the 13 repo eqm fixtures hold equipment: {with_equipment}");
}
