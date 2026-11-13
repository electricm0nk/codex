//! Equipment sets: where each owned copy is in each named set, kept in
//! `<character root>/equipment_sets.json` beside `envelope.txt` (`decisions.md` §5, §6, §8, §30).
//!
//! The saved input's `equipment=` lines are not changed: the active set is turned into their
//! `active_state`s by [`project`], the one bridge from a set to the engine. A copy a set does not
//! name is Carried in it, so the file holds only what differs from Carried.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::rules_core::body_slots::BodySlot;
use crate::rules_core::character_input::ActiveState;
use crate::saved_character::SavedCharacterStoreError;

/// The sidecar's file name under a character root.
pub const EQUIPMENT_SETS_FILE: &str = "equipment_sets.json";
/// The one schema this code reads and writes.
pub const EQUIPMENT_SETS_SCHEMA: u32 = 1;
/// The set an old save gets on first load (§8), and the one deleting the active set falls back to.
pub const DEFAULT_SET_NAME: &str = "Default";
/// A set name's longest length, in characters.
pub const SET_NAME_MAX_CHARS: usize = 40;

/// One owned copy: the `ordinal`-th (0-based) copy of `item_id` at `bonus` in the saved input's
/// order (§5.2). Custom items are `custom:<id>`, bonus 0, ordinal 0.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyKey {
    pub item_id: String,
    pub bonus: u8,
    pub ordinal: u32,
}

impl CopyKey {
    /// The key of the sidecar's `copies` map: `"<itemId>|<bonus>|<ordinal>"`.
    pub fn map_key(&self) -> String {
        format!("{}|{}|{}", self.item_id, self.bonus, self.ordinal)
    }
}

/// Where a copy is in one set (§5.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Location {
    /// Worn in one of the real body slots (never `BodySlot::None`; that is `Equipped`).
    Slot { slot: BodySlot },
    /// Worn, taking no slot.
    Equipped,
    /// With the character, not worn.
    Carried,
    /// Owned, left behind.
    NotCarried,
    /// In a container copy; the container's own location decides carried-ness.
    Inside { container: CopyKey },
}

/// One copy a set names, and where it is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetEntry {
    pub copy: CopyKey,
    pub location: Location,
}

/// One named set. A copy it does not name is Carried in it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EquipmentSet {
    pub name: String,
    pub locations: Vec<SetEntry>,
}

/// What the sidecar records about one copy across every set (§24, §30): its custom name and its
/// piece count. An absent `quantity` is one bundle of the record (`base_quantity` pieces).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyFacts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

/// The whole `equipment_sets.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EquipmentSets {
    pub schema: u32,
    pub active: String,
    pub sets: Vec<EquipmentSet>,
    #[serde(default)]
    pub copies: BTreeMap<String, CopyFacts>,
}

/// A copy as the load path sees it, for [`default_from`]: its key, its saved `active_state`, and
/// the slot the held list derives for it (`BodySlot::None` when it takes none).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeldCopy {
    pub copy: CopyKey,
    pub active_state: ActiveState,
    pub slot: BodySlot,
}

impl EquipmentSets {
    /// One set, active, and no per-copy facts.
    pub fn new(set: EquipmentSet) -> Self {
        Self { schema: EQUIPMENT_SETS_SCHEMA, active: set.name.clone(), sets: vec![set], copies: BTreeMap::new() }
    }

    /// The active set. `load` refuses a file whose `active` names no set, and the edits here keep
    /// it naming one.
    pub fn active_set(&self) -> &EquipmentSet {
        self.sets.iter().find(|set| set.name == self.active).unwrap_or(&self.sets[0])
    }

    /// `name` trimmed, when it is 1–40 characters and no other set holds it (ignoring case);
    /// `renaming` is the set being renamed, which may keep its own name.
    pub fn check_set_name(&self, name: &str, renaming: Option<&str>) -> Result<String, String> {
        let name = name.trim();
        let chars = name.chars().count();
        if chars == 0 {
            return Err("a set needs a name".to_owned());
        }
        if chars > SET_NAME_MAX_CHARS {
            return Err(format!("a set name is at most {SET_NAME_MAX_CHARS} characters; \"{name}\" has {chars}"));
        }
        let taken = self.sets.iter().any(|set| Some(set.name.as_str()) != renaming && set.name.to_lowercase() == name.to_lowercase());
        if taken {
            return Err(format!("there is already a set named \"{name}\""));
        }
        Ok(name.to_owned())
    }

    /// Adds a set: everything Carried, or a copy of the active set's locations.
    pub fn add_set(&mut self, name: &str, copy_active: bool) -> Result<(), String> {
        let name = self.check_set_name(name, None)?;
        let locations = if copy_active { self.active_set().locations.clone() } else { Vec::new() };
        self.sets.push(EquipmentSet { name, locations });
        Ok(())
    }

    /// Renames a set; the active set stays active under its new name.
    pub fn rename_set(&mut self, from: &str, to: &str) -> Result<(), String> {
        let index = self.index_of(from)?;
        let to = self.check_set_name(to, Some(from))?;
        if self.active == from {
            self.active = to.clone();
        }
        self.sets[index].name = to;
        Ok(())
    }

    /// Deletes a set. The last set cannot be deleted; deleting the active set activates `Default`
    /// or, without one, the first remaining.
    pub fn delete_set(&mut self, name: &str) -> Result<(), String> {
        let index = self.index_of(name)?;
        if self.sets.len() == 1 {
            return Err(format!("\"{name}\" is the last set and cannot be deleted"));
        }
        self.sets.remove(index);
        if self.active == name {
            self.active = self.sets.iter().find(|set| set.name == DEFAULT_SET_NAME).unwrap_or(&self.sets[0]).name.clone();
        }
        Ok(())
    }

    fn index_of(&self, name: &str) -> Result<usize, String> {
        self.sets.iter().position(|set| set.name == name).ok_or_else(|| format!("there is no set named \"{name}\""))
    }
}

/// Reads `<root>/equipment_sets.json`; `None` when the root has none yet (an old save: E2.1 writes
/// the Default set). A file of another schema, or whose `active` names no set, is refused.
pub fn load(root: &Path) -> Result<Option<EquipmentSets>, SavedCharacterStoreError> {
    let path = root.join(EQUIPMENT_SETS_FILE);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(store_error(format!("could not read {}: {err}", path.display()))),
    };
    let sets: EquipmentSets = serde_json::from_str(&text).map_err(|err| store_error(format!("{} is not an equipment-sets file: {err}", path.display())))?;
    if sets.schema != EQUIPMENT_SETS_SCHEMA {
        return Err(store_error(format!("{} is schema {}; this version reads schema {EQUIPMENT_SETS_SCHEMA}", path.display(), sets.schema)));
    }
    if !sets.sets.iter().any(|set| set.name == sets.active) {
        return Err(store_error(format!("{}: the active set \"{}\" is not one of its sets", path.display(), sets.active)));
    }
    Ok(Some(sets))
}

/// Writes `<root>/equipment_sets.json` through a `.tmp` sibling and a rename, as
/// `SavedCharacterStore::save` writes the envelope.
pub fn save(root: &Path, sets: &EquipmentSets) -> Result<(), SavedCharacterStoreError> {
    let path = root.join(EQUIPMENT_SETS_FILE);
    let tmp = root.join(format!("{EQUIPMENT_SETS_FILE}.tmp"));
    let text = serde_json::to_string_pretty(sets).map_err(|err| store_error(format!("could not write the equipment sets: {err}")))?;
    fs::write(&tmp, text).map_err(|err| store_error(format!("could not write {}: {err}", tmp.display())))?;
    fs::rename(&tmp, &path).map_err(|err| store_error(format!("could not write {}: {err}", path.display())))
}

fn store_error(message: String) -> SavedCharacterStoreError {
    SavedCharacterStoreError { message }
}

/// Where `copy` is in `set`; a copy the set does not name is Carried.
pub fn location_of(set: &EquipmentSet, copy: &CopyKey) -> Location {
    set.locations.iter().find(|entry| &entry.copy == copy).map_or(Location::Carried, |entry| entry.location.clone())
}

/// Keys for copies in saved order: each `(item_id, bonus)` pair's ordinal counts from 0.
pub fn copy_keys<'a>(copies: impl IntoIterator<Item = (&'a str, u8)>) -> Vec<CopyKey> {
    let mut seen: BTreeMap<(String, u8), u32> = BTreeMap::new();
    copies
        .into_iter()
        .map(|(item_id, bonus)| {
            let next = seen.entry((item_id.to_owned(), bonus)).or_insert(0);
            let ordinal = *next;
            *next += 1;
            CopyKey { item_id: item_id.to_owned(), bonus, ordinal }
        })
        .collect()
}

/// The `active_state` of each of `copies` under `set` (§6), logging any copy caught in a container
/// cycle (projected Carried; moves refuse cycles, so one here is a hand-edited file).
pub fn project(set: &EquipmentSet, copies: &[CopyKey]) -> Vec<ActiveState> {
    let (states, cycles) = project_with_cycles(set, copies);
    for copy in cycles {
        eprintln!("equipment sets: set \"{}\" puts {} inside a container that is inside it; projected as Carried", set.name, copy.map_key());
    }
    states
}

/// [`project`], and the copies (in `copies` order) whose container chain loops back on itself.
pub fn project_with_cycles(set: &EquipmentSet, copies: &[CopyKey]) -> (Vec<ActiveState>, Vec<CopyKey>) {
    let mut cycles = Vec::new();
    let states = copies
        .iter()
        .map(|copy| {
            let mut seen = BTreeSet::new();
            let mut current = copy.clone();
            let mut inside = false;
            loop {
                if !seen.insert(current.clone()) {
                    cycles.push(copy.clone());
                    break ActiveState::SelectedInactive;
                }
                match location_of(set, &current) {
                    Location::Slot { .. } | Location::Equipped if !inside => break ActiveState::EquippedActive,
                    Location::Slot { .. } | Location::Equipped | Location::Carried => break ActiveState::SelectedInactive,
                    Location::NotCarried => break ActiveState::Absent,
                    Location::Inside { container } => {
                        inside = true;
                        current = container;
                    }
                }
            }
        })
        .collect();
    (states, cycles)
}

/// The Default set for a root with no sidecar (§8): worn with a real slot → `Slot`, worn with
/// none → `Equipped`, left behind → `NotCarried`, carried → unnamed.
pub fn default_from(copies: &[HeldCopy]) -> EquipmentSets {
    let locations = copies
        .iter()
        .filter_map(|held| {
            let location = match (held.active_state, held.slot) {
                (ActiveState::EquippedActive, BodySlot::None) => Location::Equipped,
                (ActiveState::EquippedActive, slot) => Location::Slot { slot },
                (ActiveState::Absent, _) => Location::NotCarried,
                (ActiveState::SelectedInactive, _) => return None,
            };
            Some(SetEntry { copy: held.copy.clone(), location })
        })
        .collect();
    EquipmentSets::new(EquipmentSet { name: DEFAULT_SET_NAME.to_owned(), locations })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::body_slots::BodySlot;
    use crate::rules_core::character_input::{load_character_input_fixture, ActiveState, EquipmentSelection};
    use crate::saved_character::local_store::SavedCharacterStore;
    use std::path::{Path, PathBuf};

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time").as_nanos();
        let path = std::env::temp_dir().join(format!("codex-equipment-sets-{label}-{}-{unique}", std::process::id()));
        std::fs::create_dir_all(&path).expect("temp dir");
        path
    }

    fn key(item_id: &str, ordinal: u32) -> CopyKey {
        CopyKey { item_id: item_id.to_owned(), bonus: 0, ordinal }
    }

    fn set_of(entries: Vec<(CopyKey, Location)>) -> EquipmentSet {
        EquipmentSet { name: "Default".to_owned(), locations: entries.into_iter().map(|(copy, location)| SetEntry { copy, location }).collect() }
    }

    #[test]
    fn equipment_sets_round_trip_file() {
        let root = tempdir("round-trip");
        assert_eq!(load(&root).expect("absent file reads"), None, "no file = no sets yet (E2.1 writes Default)");
        let mut sets = EquipmentSets::new(set_of(vec![
            (key("item:chain_shirt", 0), Location::Slot { slot: BodySlot::Armor }),
            (key("item:shield", 0), Location::NotCarried),
            (key("item:rations", 0), Location::Inside { container: key("custom:food", 0) }),
            (key("item:ring_of_protection", 0), Location::Equipped),
        ]));
        sets.copies.insert(key("item:arrow_flight", 0).map_key(), CopyFacts { name: Some("Lucky arrows".to_owned()), quantity: Some(60) });
        save(&root, &sets).expect("saves");
        assert_eq!(load(&root).expect("reads"), Some(sets.clone()));
        let text = std::fs::read_to_string(root.join(EQUIPMENT_SETS_FILE)).expect("file");
        let json: serde_json::Value = serde_json::from_str(&text).expect("json");
        assert_eq!(json["schema"], 1);
        assert_eq!(json["active"], "Default");
        assert_eq!(json["sets"][0]["locations"][0]["copy"]["itemId"], "item:chain_shirt");
        assert_eq!(json["sets"][0]["locations"][0]["location"], serde_json::json!({ "kind": "slot", "slot": "armor" }));
        assert_eq!(json["sets"][0]["locations"][1]["location"], serde_json::json!({ "kind": "notCarried" }));
        assert_eq!(json["sets"][0]["locations"][2]["location"]["container"]["itemId"], "custom:food");
        assert_eq!(json["copies"]["item:arrow_flight|0|0"], serde_json::json!({ "name": "Lucky arrows", "quantity": 60 }));
        let leftovers: Vec<_> = std::fs::read_dir(&root).expect("dir").flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        assert_eq!(leftovers, vec![EQUIPMENT_SETS_FILE.to_owned()], "atomic write leaves no .tmp");
        std::fs::write(root.join(EQUIPMENT_SETS_FILE), text.replace("\"schema\": 1", "\"schema\": 2")).expect("write");
        assert!(load(&root).unwrap_err().message.contains("schema 2"), "an unknown schema is refused, not guessed");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn equipment_sets_unnamed_copy_is_carried() {
        let set = set_of(vec![(key("item:longsword", 0), Location::Slot { slot: BodySlot::WieldMain })]);
        assert_eq!(location_of(&set, &key("item:dagger", 0)), Location::Carried);
        assert_eq!(location_of(&set, &key("item:longsword", 1)), Location::Carried, "another ordinal is another copy");
        assert_eq!(location_of(&set, &key("item:longsword", 0)), Location::Slot { slot: BodySlot::WieldMain });
    }

    #[test]
    fn equipment_sets_projection_table() {
        let copies = [key("a", 0), key("b", 0), key("c", 0), key("d", 0), key("e", 0), key("f", 0), key("g", 0)];
        let set = set_of(vec![
            (key("a", 0), Location::Slot { slot: BodySlot::Head }),
            (key("b", 0), Location::Equipped),
            (key("c", 0), Location::Carried),
            (key("d", 0), Location::NotCarried),
            (key("e", 0), Location::Inside { container: key("c", 0) }),
            (key("f", 0), Location::Inside { container: key("d", 0) }),
            (key("g", 0), Location::Inside { container: key("a", 0) }),
        ]);
        assert_eq!(
            project(&set, &copies),
            vec![
                ActiveState::EquippedActive,
                ActiveState::EquippedActive,
                ActiveState::SelectedInactive,
                ActiveState::Absent,
                ActiveState::SelectedInactive,
                ActiveState::Absent,
                ActiveState::SelectedInactive,
            ]
        );
        assert_eq!(project(&set_of(vec![]), &[key("z", 0)]), vec![ActiveState::SelectedInactive], "unnamed = Carried");
    }

    #[test]
    fn equipment_sets_projection_nested_container_follows_outer() {
        let (pack, pouch, coin) = (key("item:backpack", 0), key("item:belt_pouch", 0), key("item:gem", 0));
        let inner = vec![(pouch.clone(), Location::Inside { container: pack.clone() }), (coin.clone(), Location::Inside { container: pouch.clone() })];
        let copies = [pack.clone(), pouch.clone(), coin.clone()];
        let mut left = inner.clone();
        left.push((pack.clone(), Location::NotCarried));
        assert_eq!(project(&set_of(left), &copies), vec![ActiveState::Absent; 3], "a pack left behind takes its contents");
        let mut worn = inner;
        worn.push((pack.clone(), Location::Slot { slot: BodySlot::Shoulders }));
        assert_eq!(
            project(&set_of(worn), &copies),
            vec![ActiveState::EquippedActive, ActiveState::SelectedInactive, ActiveState::SelectedInactive],
            "contents of a worn pack are carried, not worn"
        );
    }

    #[test]
    fn equipment_sets_projection_cycle_is_carried_and_logged() {
        let (a, b, c) = (key("a", 0), key("b", 0), key("c", 0));
        let set = set_of(vec![
            (a.clone(), Location::Inside { container: b.clone() }),
            (b.clone(), Location::Inside { container: a.clone() }),
            (c.clone(), Location::Inside { container: c.clone() }),
        ]);
        let (states, cycles) = project_with_cycles(&set, &[a.clone(), b.clone(), c.clone()]);
        assert_eq!(states, vec![ActiveState::SelectedInactive; 3]);
        assert_eq!(cycles, vec![a.clone(), b.clone(), c.clone()], "every copy caught in a cycle is reported");
        assert_eq!(project(&set, &[a, b, c]), vec![ActiveState::SelectedInactive; 3], "never a panic");
    }

    /// The `+N` of a selection by its enhancement eqmod (`body_slots::Enhanceable::modifier_key`).
    fn bonus_of(selection: &EquipmentSelection) -> u8 {
        use crate::rules_core::body_slots::Enhanceable;
        (1..=10u8)
            .filter(|n| [Enhanceable::Armor, Enhanceable::Shield, Enhanceable::Weapon].iter().any(|kind| selection.applied_modifiers.contains(&kind.modifier_key(*n))))
            .max()
            .unwrap_or(0)
    }

    /// `project(default_from(x), x) == x.active_states`, copy by copy, with every worn copy given
    /// `slot`; returns how many copies were checked.
    fn assert_round_trip(label: &str, selections: &[EquipmentSelection], slot: BodySlot) -> usize {
        let keys = copy_keys(selections.iter().map(|s| (s.item_id.as_str(), bonus_of(s))));
        let held: Vec<HeldCopy> = keys.iter().zip(selections).map(|(copy, s)| HeldCopy { copy: copy.clone(), active_state: s.active_state, slot }).collect();
        let sets = default_from(&held);
        assert_eq!(sets.active, DEFAULT_SET_NAME);
        assert_eq!(sets.sets.len(), 1);
        let projected = project(sets.active_set(), &keys);
        for ((copy, selection), state) in keys.iter().zip(selections).zip(&projected) {
            assert_eq!(*state, selection.active_state, "{label}: {copy:?}");
        }
        keys.len()
    }

    #[test]
    fn equipment_sets_default_from_seed_round_trips() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
        let mut files = Vec::new();
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            for entry in std::fs::read_dir(dir).expect("fixtures dir").flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "txt") {
                    out.push(path);
                }
            }
        }
        walk(&fixtures, &mut files);
        files.sort();
        let (mut inputs, mut copies, mut states) = (0usize, 0usize, std::collections::BTreeSet::new());
        for path in &files {
            let text = std::fs::read_to_string(path).expect("fixture");
            if !text.lines().any(|line| line.trim_start().starts_with("equipment=")) {
                continue;
            }
            let Some(input) = load_character_input_fixture(&text).character_input else { continue };
            let selections = &input.chosen.equipment_selections;
            states.extend(selections.iter().map(|s| format!("{:?}", s.active_state)));
            for slot in [BodySlot::None, BodySlot::Armor] {
                copies += assert_round_trip(&path.display().to_string(), selections, slot);
            }
            inputs += 1;
        }
        let saved = SavedCharacterStore::load(&fixtures.join("sd14/pf1_human_fighter_level1_saved_character")).expect("saved root");
        for slot in [BodySlot::None, BodySlot::Armor] {
            copies += assert_round_trip("sd14 saved root", &saved.character_input.chosen.equipment_selections, slot);
        }
        inputs += 1;
        println!("equipment_sets_default_from_seed_round_trips: {inputs} inputs, {} copy checks, states {states:?}", copies);
        assert!(inputs >= 100, "the on-disk fixture inputs holding an equipment line: {inputs}");
        assert_eq!(states.len(), 3, "all three active states are exercised: {states:?}");
    }

    #[test]
    fn equipment_sets_default_from_maps_each_state() {
        let held = [
            HeldCopy { copy: key("item:chain_shirt", 0), active_state: ActiveState::EquippedActive, slot: BodySlot::Armor },
            HeldCopy { copy: key("item:cloak", 0), active_state: ActiveState::EquippedActive, slot: BodySlot::None },
            HeldCopy { copy: key("item:rope", 0), active_state: ActiveState::SelectedInactive, slot: BodySlot::None },
            HeldCopy { copy: key("item:shield", 0), active_state: ActiveState::Absent, slot: BodySlot::Shield },
        ];
        let sets = default_from(&held);
        let set = sets.active_set();
        assert_eq!(set.locations.len(), 3, "a carried copy is left unnamed");
        assert_eq!(location_of(set, &held[0].copy), Location::Slot { slot: BodySlot::Armor });
        assert_eq!(location_of(set, &held[1].copy), Location::Equipped);
        assert_eq!(location_of(set, &held[2].copy), Location::Carried);
        assert_eq!(location_of(set, &held[3].copy), Location::NotCarried);
        assert_eq!(copy_keys([("item:dagger", 0), ("item:dagger", 1), ("item:dagger", 0)]), vec![key("item:dagger", 0), CopyKey { item_id: "item:dagger".to_owned(), bonus: 1, ordinal: 0 }, key("item:dagger", 1)]);
    }

    #[test]
    fn equipment_sets_name_rules() {
        let mut sets = EquipmentSets::new(set_of(vec![(key("a", 0), Location::NotCarried)]));
        assert_eq!(sets.add_set("  Combat  ", false), Ok(()));
        assert_eq!(sets.sets[1].name, "Combat", "trimmed");
        assert!(sets.sets[1].locations.is_empty(), "a new set starts as everything Carried");
        assert!(sets.add_set("combat", false).is_err(), "unique, ignoring case");
        assert!(sets.add_set("   ", false).is_err(), "at least one character");
        assert!(sets.add_set(&"x".repeat(41), false).is_err(), "at most 40 characters");
        assert_eq!(sets.add_set(&"é".repeat(40), false), Ok(()), "40 characters, not bytes");
        assert_eq!(sets.add_set("Travel", true), Ok(()));
        assert_eq!(sets.sets.last().unwrap().locations, sets.sets[0].locations, "created as a copy of the active set");
        assert!(sets.rename_set("Travel", "Default").is_err(), "a rename keeps names unique");
        assert_eq!(sets.rename_set("Travel", " Camp "), Ok(()));
        assert!(sets.sets.iter().any(|set| set.name == "Camp"));
        assert_eq!(sets.rename_set("Default", "Home"), Ok(()));
        assert_eq!(sets.active, "Home", "renaming the active set keeps it active");
        assert!(sets.rename_set("Nowhere", "Other").is_err());
    }

    #[test]
    fn equipment_sets_last_set_cannot_be_deleted() {
        let mut sets = EquipmentSets::new(set_of(vec![]));
        assert!(sets.delete_set("Default").is_err(), "the last set stays");
        sets.add_set("Combat", false).unwrap();
        sets.add_set("Travel", false).unwrap();
        sets.active = "Travel".to_owned();
        assert_eq!(sets.delete_set("Travel"), Ok(()));
        assert_eq!(sets.active, "Default", "deleting the active set activates Default");
        assert_eq!(sets.delete_set("Default"), Ok(()));
        assert_eq!(sets.active, "Combat", "or the first remaining");
        assert!(sets.delete_set("Combat").is_err());
        assert!(sets.delete_set("Nowhere").is_err());
    }
}
