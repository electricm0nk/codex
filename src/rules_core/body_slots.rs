//! Which body slot an equipment record occupies, and whether it can take a `+N` enhancement.
//!
//! The record already holds what is needed: its `tags` and, on most magic items, the book's
//! `Slot:` stat-block line. The tag-to-slot table is in `data/equipment_types.json` under `slots`,
//! written by `gen_equipment_types` from the pinned oracle's slot list, so a table change is one
//! data row.
//!
//! Resolution order, first hit wins:
//! 1. The book's `Slot:` line. "none" and "neck (does not take up slot)" mean no slot.
//! 2. Barding and ammunition: no slot. Barding is worn by the mount, and ammunition is not wielded.
//! 3. A tag naming a slot outright (`SLOT_Neck`, `Slot_Eyes`), the first one on the record.
//! 4. The table over the record's tags. A shield outranks armor, armor outranks a weapon, and a
//!    weapon outranks a worn tag (a gauntlet is wielded); otherwise the record's first tag decides.
//!
//! A table row with no Codex slot (`Torso`, the `Tattoo` rows) maps to [`BodySlot::None`]
//! (`decisions.md` SD-g): an outfit or a tattoo occupies nothing the slot check enforces.
//!
//! A `#bonusN` row carries its item's tags but no `Slot:` line, so a caller asks about the
//! item's principal row. Tags cannot give a weapon's handedness for most ranged weapons (a longbow
//! has no `TwoHanded` tag): only the tag is read here, and the settled
//! [`crate::rules_core::equipment_record`] wield category is the answer for the hand check.

use serde::{Deserialize, Serialize};

use crate::rules_core::equipment_types::EQUIPMENT_TYPES_PATH;
use crate::rules_core::sheet_rule::{ProsePiece, SheetRule};
use crate::support::paths::repo_root;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodySlot {
    Armor,
    Shield,
    Belt,
    Body,
    Chest,
    Eyes,
    Feet,
    Hands,
    Head,
    Headband,
    Neck,
    Ring,
    Shoulders,
    Wrist,
    WieldMain,
    WieldOff,
    WieldTwoHanded,
    None,
}

/// What kind of `+N` a record can take; the kind sets the price table and the eqmod family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Enhanceable {
    Armor,
    Shield,
    Weapon,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotRow {
    /// The source list's own slot name (`Wrist`, `Torso`, `Tattoo Head`).
    pub name: String,
    pub body_slot: BodySlot,
    /// How many items the slot holds at once.
    pub capacity: u32,
    /// The record tags that place an item in this slot.
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotTable {
    rows: Vec<SlotRow>,
}

/// Tags marking a record that cannot be bought as a mundane base for a `+N`: ammunition and barding
/// (`epic-breakdown.md` E1.0), the already-magic markers (`Magic`, the price tiers, `Specific`,
/// `Artifact`, `Cursed`, `Legendary`, `Psionic`), and what is not a lasting item at all.
const NOT_ENHANCEABLE_TAGS: &[&str] = &[
    "Ammunition",
    "Ammo",
    "Barding",
    "Magic",
    "LeastMinor",
    "LesserMinor",
    "GreaterMinor",
    "Minor",
    "LesserMedium",
    "GreaterMedium",
    "LesserMajor",
    "GreaterMajor",
    "Major",
    "Specific",
    "Artifact",
    "Cursed",
    "Legendary",
    "Psionic",
    "Consumable",
    "Improvised",
    "Natural",
    "Siege",
    "SiegeWeapon",
];

fn has_tag(record: &SheetRule, tag: &str) -> bool {
    record.tags.iter().any(|t| t.as_str() == tag)
}

/// The slot a book or a slot-naming tag spells out. `None` when the words name no slot we hold.
fn slot_from_words(words: &str) -> Option<BodySlot> {
    let words = words.trim().to_ascii_lowercase();
    if words.contains("does not take up slot") {
        return Some(BodySlot::None);
    }
    Some(match words.as_str() {
        "none" => BodySlot::None,
        "armor" => BodySlot::Armor,
        "shield" => BodySlot::Shield,
        "belt" => BodySlot::Belt,
        "body" => BodySlot::Body,
        "chest" => BodySlot::Chest,
        "eyes" => BodySlot::Eyes,
        "feet" => BodySlot::Feet,
        "hands" => BodySlot::Hands,
        "head" => BodySlot::Head,
        "headband" => BodySlot::Headband,
        "neck" => BodySlot::Neck,
        "ring" => BodySlot::Ring,
        "shoulders" => BodySlot::Shoulders,
        "wrist" | "wrists" => BodySlot::Wrist,
        _ => return None,
    })
}

/// The words after `Slot:` on the record's stat block, if it has that line.
fn slot_line(record: &SheetRule) -> Option<&str> {
    record.prose.iter().flat_map(|segment| &segment.pieces).find_map(|piece| match piece {
        ProsePiece::Text(text) => text.strip_prefix("Slot:"),
        _ => None,
    })
}

impl SlotTable {
    /// The `slots` array of the sidecar. Missing or malformed input is an error naming the problem,
    /// never an empty table standing in for "no item has a slot".
    pub fn parse(json: &str) -> Result<Self, String> {
        let document: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("{EQUIPMENT_TYPES_PATH} is not valid JSON: {e}"))?;
        let slots = document
            .get("slots")
            .ok_or_else(|| format!("{EQUIPMENT_TYPES_PATH} has no `slots` array"))?;
        let rows: Vec<SlotRow> = serde_json::from_value(slots.clone())
            .map_err(|e| format!("{EQUIPMENT_TYPES_PATH}: `slots` is malformed: {e}"))?;
        if rows.is_empty() {
            return Err(format!("{EQUIPMENT_TYPES_PATH}: `slots` is empty"));
        }
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[SlotRow] {
        &self.rows
    }

    /// The slot the table alone gives the record's tags.
    fn slot_from_tags(&self, record: &SheetRule) -> BodySlot {
        let mut best: Option<(u8, BodySlot)> = None;
        for tag in &record.tags {
            let hit = self.rows.iter().find(|row| row.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)));
            let Some(row) = hit else { continue };
            let rank = match row.body_slot {
                BodySlot::None => continue,
                BodySlot::Shield => 0,
                BodySlot::Armor => 1,
                BodySlot::WieldMain => 2,
                _ => 3,
            };
            if best.is_none_or(|(held, _)| rank < held) {
                best = Some((rank, row.body_slot));
            }
        }
        match best {
            Some((_, BodySlot::WieldMain)) if has_tag(record, "TwoHanded") => BodySlot::WieldTwoHanded,
            Some((_, slot)) => slot,
            None => BodySlot::None,
        }
    }

    pub fn slot_of(&self, record: &SheetRule) -> BodySlot {
        if let Some(words) = slot_line(record) {
            return slot_from_words(words).unwrap_or(BodySlot::None);
        }
        if has_tag(record, "Barding") || has_tag(record, "Ammunition") || has_tag(record, "Ammo") {
            return BodySlot::None;
        }
        let named = record.tags.iter().find_map(|tag| {
            let lowered = tag.to_ascii_lowercase();
            lowered.strip_prefix("slot_").and_then(slot_from_words)
        });
        named.unwrap_or_else(|| self.slot_from_tags(record))
    }

    /// How many items a slot holds at once: 2 for rings. The hands are each one item; no slot holds nothing.
    pub fn capacity(&self, slot: BodySlot) -> u32 {
        match slot {
            BodySlot::None => 0,
            BodySlot::WieldOff | BodySlot::WieldTwoHanded => 1,
            _ => self.rows.iter().filter(|row| row.body_slot == slot).map(|row| row.capacity).max().unwrap_or(0),
        }
    }
}

/// The table, loaded once per process from [`repo_root`] (the packaged resource root in an
/// installed app).
pub fn slot_table() -> &'static Result<SlotTable, String> {
    static LOADED: std::sync::OnceLock<Result<SlotTable, String>> = std::sync::OnceLock::new();
    LOADED.get_or_init(|| {
        let path = repo_root().join(EQUIPMENT_TYPES_PATH);
        let text = std::fs::read_to_string(&path).map_err(|e| format!("could not read {}: {e}", path.display()))?;
        SlotTable::parse(&text)
    })
}

/// The body slot an equipment record occupies. `Err` only when the table itself cannot be read: a
/// missing table must not read as "this item has no slot".
pub fn slot_of(record: &SheetRule) -> Result<BodySlot, String> {
    slot_table().as_ref().map(|table| table.slot_of(record)).map_err(Clone::clone)
}

/// Whether a record can be offered as `+1`…`+5`, and of which kind.
pub fn enhanceable(record: &SheetRule) -> Option<Enhanceable> {
    if NOT_ENHANCEABLE_TAGS.iter().any(|tag| has_tag(record, tag)) {
        return None;
    }
    if has_tag(record, "Shield") {
        Some(Enhanceable::Shield)
    } else if has_tag(record, "Armor") {
        Some(Enhanceable::Armor)
    } else if has_tag(record, "Weapon") {
        Some(Enhanceable::Weapon)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::{Path, PathBuf};

    use crate::support::paths::pcgen_corpus_root;

    fn table() -> &'static SlotTable {
        slot_table().as_ref().expect("data/equipment_types.json has a slot table")
    }

    fn rules_in(kind: &str) -> Vec<SheetRule> {
        let root = repo_root().join("data/sheet_rules");
        let mut files = Vec::new();
        for book in std::fs::read_dir(&root).unwrap() {
            let dir = book.unwrap().path().join(kind);
            if dir.is_dir() {
                for file in std::fs::read_dir(&dir).unwrap() {
                    files.push(file.unwrap().path());
                }
            }
        }
        files.sort();
        let mut rules = Vec::new();
        for file in files {
            let text = std::fs::read_to_string(&file).unwrap();
            let parsed: Vec<SheetRule> =
                serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
            rules.extend(parsed);
        }
        rules
    }

    fn id_of(record: &SheetRule) -> String {
        record.id.as_str().to_string()
    }

    fn by_id<'a>(rules: &'a [SheetRule], id: &str) -> &'a SheetRule {
        rules.iter().find(|r| r.id.as_str() == id).unwrap_or_else(|| panic!("no record {id}"))
    }

    #[test]
    fn slot_of_places_the_named_records() {
        let rules = rules_in("equipment");
        for (id, want) in [
            ("core_rulebook:equipment:ring_of_protection_1", BodySlot::Ring),
            ("core_rulebook:equipment:headband_of_mental_prowess_int_cha_2", BodySlot::Headband),
            ("core_rulebook:equipment:hat_of_disguise", BodySlot::Head),
            ("core_rulebook:equipment:amulet_of_natural_armor_1", BodySlot::Neck),
            ("core_rulebook:equipment:leather", BodySlot::Armor),
            ("core_rulebook:equipment:leather_armor_base", BodySlot::Armor),
            ("core_rulebook:equipment:rope_hemp_50_ft", BodySlot::None),
        ] {
            assert_eq!(slot_of(by_id(&rules, id)).unwrap(), want, "{id}");
        }
    }

    #[test]
    fn slot_of_rings_take_two_and_everything_else_one() {
        assert_eq!(table().capacity(BodySlot::Ring), 2);
        for slot in [BodySlot::Head, BodySlot::Armor, BodySlot::Shield, BodySlot::Neck, BodySlot::WieldMain] {
            assert_eq!(table().capacity(slot), 1, "{slot:?}");
        }
        assert_eq!(table().capacity(BodySlot::None), 0);
    }

    /// Every record where the book's line and the tag table differ. F-12's "19" is 12 spellings
    /// (`wrists`) + 7 "does not take up slot", and it leaves out the 8 weapons whose line says
    /// "none". All 27 resolve to what the book's line says.
    #[test]
    fn slot_of_lists_and_resolves_every_tag_and_book_disagreement() {
        let rules = rules_in("equipment");
        let mut spelling = Vec::new();
        let mut no_slot_taken = Vec::new();
        let mut none_on_weapon = Vec::new();
        for record in &rules {
            let Some(words) = slot_line(record) else { continue };
            let words = words.trim();
            let tag_slot = table().slot_from_tags(record);
            if tag_slot == BodySlot::None {
                continue;
            }
            let tag_word = serde_json::to_value(tag_slot).unwrap().as_str().unwrap().to_string();
            if words.eq_ignore_ascii_case(&tag_word) {
                continue;
            }
            let bucket = match words {
                "wrists" => &mut spelling,
                "neck (does not take up slot)" => &mut no_slot_taken,
                "none" => &mut none_on_weapon,
                other => panic!("{}: unclassified disagreement: book {other:?}, tags {tag_slot:?}", id_of(record)),
            };
            bucket.push(id_of(record));
            assert_eq!(table().slot_of(record), slot_from_words(words).unwrap(), "{}", id_of(record));
        }
        println!(
            "tag/book disagreements: spelling={} no_slot_taken={} none_on_weapon={} total={}",
            spelling.len(),
            no_slot_taken.len(),
            none_on_weapon.len(),
            spelling.len() + no_slot_taken.len() + none_on_weapon.len()
        );
        println!("spelling (wrists -> wrist): {spelling:#?}");
        println!("no slot taken: {no_slot_taken:#?}");
        println!("Slot: none on a weapon-tagged record: {none_on_weapon:#?}");
        assert_eq!(spelling.len(), 12);
        assert_eq!(no_slot_taken.len(), 7);
        assert_eq!(none_on_weapon.len(), 8);
        assert!(no_slot_taken.iter().all(|id| id.contains("necklace_of_fireballs")), "{no_slot_taken:?}");
        for id in &spelling {
            assert_eq!(table().slot_of(by_id(&rules, id)), BodySlot::Wrist, "{id}");
        }
        for id in no_slot_taken.iter().chain(&none_on_weapon) {
            assert_eq!(table().slot_of(by_id(&rules, id)), BodySlot::None, "{id}");
        }
    }

    /// F-12 and F-13, and every shape the rules above decide. A moved count fails here.
    #[test]
    fn slot_of_counts_over_the_equipment_corpus() {
        let rules = rules_in("equipment");
        let principals: Vec<&SheetRule> = rules.iter().filter(|r| !r.id.as_str().contains('#')).collect();
        let with_line = rules.iter().filter(|r| slot_line(r).is_some()).count();
        let with_tag = rules
            .iter()
            .filter(|r| {
                r.tags.iter().any(|tag| {
                    table().rows().iter().filter(|row| !row.name.starts_with("Tattoo")).any(|row| {
                        row.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
                    })
                })
            })
            .count();
        let resolved = |set: &[&SheetRule]| set.iter().filter(|r| table().slot_of(r) != BodySlot::None).count();
        let all: Vec<&SheetRule> = rules.iter().collect();

        let mut by_slot: BTreeMap<BodySlot, usize> = BTreeMap::new();
        for record in &principals {
            *by_slot.entry(table().slot_of(record)).or_default() += 1;
        }
        let line_only = rules
            .iter()
            .filter(|r| {
                slot_line(r).is_some_and(|w| slot_from_words(w) != Some(BodySlot::None))
                    && table().slot_from_tags(r) == BodySlot::None
            })
            .count();
        let unknown_words: Vec<String> = rules
            .iter()
            .filter_map(slot_line)
            .filter(|w| slot_from_words(w).is_none())
            .map(str::to_string)
            .collect();
        let no_codex_slot: Vec<&str> = table()
            .rows()
            .iter()
            .filter(|row| row.body_slot == BodySlot::None)
            .map(|row| row.name.as_str())
            .collect();
        let clothing_only = principals
            .iter()
            .filter(|r| has_tag(r, "Clothing") && table().slot_of(r) == BodySlot::None && slot_line(r).is_none())
            .count();
        let tattoo = principals.iter().filter(|r| r.tags.iter().any(|t| t.starts_with("Tattoo "))).count();
        let barding_or_ammo_tagged_worn = principals
            .iter()
            .filter(|r| {
                (has_tag(r, "Barding") || has_tag(r, "Ammunition") || has_tag(r, "Ammo"))
                    && table().slot_from_tags(r) != BodySlot::None
            })
            .count();
        let weapons_no_hand_tag = principals
            .iter()
            .filter(|r| {
                table().slot_of(r) == BodySlot::WieldMain
                    && !["Light", "OneHanded", "TwoHanded"].iter().any(|t| has_tag(r, t))
            })
            .count();
        let bonus_rows_differ = rules
            .iter()
            .filter(|r| {
                r.id.as_str().split_once('#').is_some_and(|(base, _)| {
                    let principal = rules.iter().find(|p| p.id.as_str() == base);
                    principal.is_some_and(|p| table().slot_of(p) != table().slot_of(r))
                })
            })
            .count();

        println!("F-12 records={} with_table_tag={} with_slot_line={}", rules.len(), with_tag, with_line);
        println!(
            "F-13 slot_of resolves a body slot: rows {} of {}; principals {} of {}",
            resolved(&all),
            rules.len(),
            resolved(&principals),
            principals.len()
        );
        println!("principals by slot: {by_slot:?}");
        println!("slot from the book's line alone, no table tag: {line_only}");
        println!("SD-g table rows with no Codex slot: {no_codex_slot:?}");
        println!("SD-g records: Clothing with no other slot={clothing_only}, Tattoo <slot> tag={tattoo}");
        println!("barding or ammunition whose tags alone would seat them: {barding_or_ammo_tagged_worn}");
        println!("WieldMain with no hand tag (handedness unknown from tags): {weapons_no_hand_tag}");
        println!("#bonus rows whose slot_of differs from their principal: {bonus_rows_differ}");

        assert_eq!(rules.len(), 7386);
        assert_eq!(with_tag, 2906);
        assert_eq!(with_line, 1617);
        assert_eq!(resolved(&all), 2838);
        assert_eq!(resolved(&principals), 2130);
        assert!(unknown_words.is_empty(), "a Slot: line we cannot read: {unknown_words:?}");
        assert_eq!(no_codex_slot.len(), 11, "{no_codex_slot:?}");
    }

    /// The committed table agrees with the pinned source list, read here independently of the
    /// generator. Nothing but this test region reads the oracle.
    #[test]
    fn slot_of_table_matches_the_pinned_source_list() {
        let list = pcgen_corpus_root().join("../system/gameModes/Pathfinder/equipmentslots.lst");
        let text = std::fs::read_to_string(&list).unwrap_or_else(|e| panic!("{}: {e}", list.display()));
        let mut source: BTreeMap<String, (u32, BTreeSet<String>)> = BTreeMap::new();
        for line in text.lines() {
            let mut fields = line.split('\t').map(str::trim).filter(|f| !f.is_empty());
            let Some(name) = fields.next().and_then(|f| f.strip_prefix("EQSLOT:")) else { continue };
            let contains = fields.find_map(|f| f.strip_prefix("CONTAINS:")).expect("a CONTAINS field");
            let (tags, capacity) = contains.rsplit_once('=').expect("CONTAINS ends in =N");
            let tags = tags.split(',').map(str::to_string).collect();
            source.insert(name.to_string(), (capacity.parse().unwrap(), tags));
        }
        let committed: BTreeMap<String, (u32, BTreeSet<String>)> = table()
            .rows()
            .iter()
            .map(|row| (row.name.clone(), (row.capacity, row.tags.iter().cloned().collect())))
            .collect();
        assert_eq!(committed, source);
        assert_eq!(source.len(), 26);
    }

    #[test]
    fn enhanceable_comes_from_the_tags_and_refuses_ammunition_barding_and_magic() {
        let rules = rules_in("equipment");
        for (id, want) in [
            ("core_rulebook:equipment:longsword", Some(Enhanceable::Weapon)),
            ("core_rulebook:equipment:leather", Some(Enhanceable::Armor)),
            ("core_rulebook:equipment:heavy_steel_shield_base", Some(Enhanceable::Shield)),
            ("core_rulebook:equipment:ring_of_protection_1", None),
            ("core_rulebook:equipment:celestial_armor", None),
            ("core_rulebook:equipment:rope_hemp_50_ft", None),
        ] {
            assert_eq!(enhanceable(by_id(&rules, id)), want, "{id}");
        }
        let principals: Vec<&SheetRule> = rules.iter().filter(|r| !r.id.as_str().contains('#')).collect();
        let tagged = principals
            .iter()
            .filter(|r| ["Armor", "Shield", "Weapon"].iter().any(|t| has_tag(r, t)))
            .count();
        let offered: Vec<&&SheetRule> = principals.iter().filter(|r| enhanceable(r).is_some()).collect();
        let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
        for record in &offered {
            *kinds.entry(format!("{:?}", enhanceable(record).unwrap())).or_default() += 1;
        }
        let with_aura = offered
            .iter()
            .filter(|r| {
                r.prose.iter().flat_map(|s| &s.pieces).any(|p| matches!(p, ProsePiece::Text(t) if t.starts_with("Aura:")))
            })
            .map(|r| id_of(r))
            .collect::<Vec<_>>();
        println!("enhanceable {} of {} Armor/Shield/Weapon principals: {kinds:?}", offered.len(), tagged);
        println!("enhanceable but carrying an Aura: line: {with_aura:?}");
        assert!(with_aura.is_empty(), "an already-magic record reads as mundane: {with_aura:?}");
        for record in &offered {
            for tag in ["Ammunition", "Ammo", "Barding", "Magic"] {
                assert!(!has_tag(record, tag), "{} has {tag}", id_of(record));
            }
        }
    }

    fn walk_lst(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk_lst(&path, out);
            } else if path.extension().is_some_and(|e| e == "lst") {
                out.push(path);
            }
        }
    }

    fn fields(line: &str) -> Vec<&str> {
        line.split('\t').map(str::trim).collect()
    }

    fn lst_text(path: &Path) -> String {
        String::from_utf8_lossy(&std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))).into_owned()
    }

    /// `repeatable` against the oracle two ways. First, per record: the last `MULT:` on the source rows
    /// it names must equal its flag. Second, per row: a `CATEGORY:FEAT` + `MULT:YES` row anywhere in a
    /// book we ship must belong to a repeatable record, so a row the first check never saw shows up.
    #[test]
    fn repeatable_equals_the_pinned_mult_yes_on_feats() {
        let oracle = pcgen_corpus_root();
        let feats = rules_in("feat");
        let mut cache: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let (mut agree, mut disagree, mut repeatable) = (0, Vec::new(), 0);
        let mut row_owner: BTreeMap<String, bool> = BTreeMap::new();
        let mut books = BTreeSet::new();
        for rule in &feats {
            let provenance = &rule.provenance;
            books.insert(provenance.book.clone());
            let mut last_mult: Option<bool> = None;
            for row in &provenance.closure_rows {
                let (rel, number) = row.rsplit_once(':').unwrap();
                let lines = cache
                    .entry(rel.to_string())
                    .or_insert_with(|| lst_text(&oracle.join(rel)).lines().map(str::to_string).collect());
                let line = &lines[number.parse::<usize>().unwrap() - 1];
                if let Some(value) = fields(line).iter().rev().find_map(|f| f.strip_prefix("MULT:")) {
                    last_mult = Some(value.eq_ignore_ascii_case("YES"));
                }
                row_owner.entry(row.clone()).and_modify(|held| *held |= rule.repeatable).or_insert(rule.repeatable);
            }
            repeatable += usize::from(rule.repeatable);
            if last_mult.unwrap_or(false) == rule.repeatable {
                agree += 1;
            } else {
                disagree.push(id_of(rule));
            }
        }

        let mut files = Vec::new();
        walk_lst(&oracle.join("pathfinder"), &mut files);
        files.sort();
        let (mut rows_yes, mut owned_repeatable, mut owned_not, mut unowned) = (0, 0, Vec::new(), Vec::new());
        for file in &files {
            let rel = file.strip_prefix(&oracle).unwrap().to_string_lossy().into_owned();
            if !rel.split('/').any(|part| books.contains(part)) {
                continue;
            }
            for (index, line) in lst_text(file).lines().enumerate() {
                let row = fields(line);
                if row[0].starts_with('#') || !row.contains(&"MULT:YES") || !row.contains(&"CATEGORY:FEAT") {
                    continue;
                }
                rows_yes += 1;
                let at = format!("{rel}:{}", index + 1);
                match row_owner.get(&at) {
                    Some(true) => owned_repeatable += 1,
                    Some(false) => owned_not.push(at),
                    None => unowned.push(format!("{at} {}", row[0])),
                }
            }
        }

        println!(
            "repeatable vs MULT: agree {agree} / disagree {} of {}; repeatable={repeatable}",
            disagree.len(),
            feats.len()
        );
        println!(
            "MULT:YES feat rows in shipped books: {rows_yes}; on a repeatable record {owned_repeatable}; on a non-repeatable record {}; on no feat record {}",
            owned_not.len(),
            unowned.len()
        );
        println!("rows on no feat record: {unowned:#?}");
        assert!(disagree.is_empty(), "{disagree:?}");
        assert!(owned_not.is_empty(), "{owned_not:?}");
        assert_eq!(unowned.len(), 2, "{unowned:?}");
    }
}
