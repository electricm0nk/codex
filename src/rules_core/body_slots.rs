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
//! item's principal row. [`slot_of`] reads only the `TwoHanded` tag for a weapon's hands;
//! [`worn_slot`] is the answer for the hand check: the settled
//! [`crate::rules_core::equipment_record`] wield category, then the kind of weapon for a record that
//! states none (no bow or crossbow but the hand crossbows states one).
//!
//! [`slot_conflict`] is the check itself: what one more worn item would run into, from what is
//! already worn. Nothing here reads or changes a saved character.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::rules_core::damage_total::WieldCategory;
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

/// Tags marking a record as already magic: `Magic`, the price tiers, `Specific`, `Artifact`,
/// `Cursed`, `Legendary`, `Psionic`.
const MAGIC_MARKER_TAGS: &[&str] = &[
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
];

/// Tags marking a mundane record that cannot be a base for a `+N` either: ammunition and barding
/// (`epic-breakdown.md` E1.0), and what is not a lasting item at all.
const NOT_ENHANCEABLE_TAGS: &[&str] = &[
    "Ammunition",
    "Ammo",
    "Barding",
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

/// The slot a record fills when worn or wielded, with its hands settled.
///
/// `types` are the catalog's `TYPE:` tags for the row. A record derived from another (`Dagger
/// (Adamantine)`, `Mithral Shirt`) can hold few or no tags of its own, and one printing of a name
/// can lack what another states (`Madu`), so the slot is read over both.
///
/// A weapon's stated `wield_category` wins over [`slot_of`]'s tag, and a record with no other sign
/// of a slot that states one is wielded (`Dagger (Bloodthirst)`). A weapon stating none is two-handed
/// if it is a bow, or a crossbow that fires from a stock (`Projectile`; the repeating hand crossbow's
/// row has no such tag). Any other weapon stating none stays as [`slot_of`] read it, one hand.
pub fn worn_slot(record: &SheetRule, types: &[String], wield: Option<WieldCategory>) -> Result<BodySlot, String> {
    let mut joined = record.clone();
    joined.tags.extend(types.iter().cloned());
    let slot = match slot_of(&joined)? {
        BodySlot::None if wield.is_some() && slot_line(record).is_none() => BodySlot::WieldMain,
        slot => slot,
    };
    if !matches!(slot, BodySlot::WieldMain | BodySlot::WieldTwoHanded) {
        return Ok(slot);
    }
    Ok(match wield {
        Some(WieldCategory::TwoHanded) => BodySlot::WieldTwoHanded,
        Some(WieldCategory::Light | WieldCategory::OneHanded) => BodySlot::WieldMain,
        None if has_tag(&joined, "Bow") || (has_tag(&joined, "Crossbow") && has_tag(&joined, "Projectile")) => {
            BodySlot::WieldTwoHanded
        }
        None => slot,
    })
}

/// How the sheet says a slot's name in a refusal.
fn slot_words(slot: BodySlot) -> &'static str {
    match slot {
        BodySlot::Armor => "armor",
        BodySlot::Shield => "shield",
        BodySlot::Belt => "belt",
        BodySlot::Body => "body",
        BodySlot::Chest => "chest",
        BodySlot::Eyes => "eyes",
        BodySlot::Feet => "feet",
        BodySlot::Hands => "hands",
        BodySlot::Head => "head",
        BodySlot::Headband => "headband",
        BodySlot::Neck => "neck",
        BodySlot::Ring => "ring",
        BodySlot::Shoulders => "shoulders",
        BodySlot::Wrist => "wrist",
        BodySlot::WieldMain => "main hand",
        BodySlot::WieldOff => "off hand",
        BodySlot::WieldTwoHanded => "two-handed",
        BodySlot::None => "no",
    }
}

/// Puts one worn item on the body: a shield fills its slot and the off hand, a two-handed weapon
/// both hands, a one-handed weapon the main hand and then the off hand.
fn occupy<'a>(held: &mut BTreeMap<BodySlot, Vec<&'a str>>, slot: BodySlot, name: &'a str) {
    let mut fill = |slot: BodySlot| held.entry(slot).or_default().push(name);
    match slot {
        BodySlot::None => {}
        BodySlot::Shield => {
            fill(BodySlot::Shield);
            fill(BodySlot::WieldOff);
        }
        BodySlot::WieldTwoHanded => {
            fill(BodySlot::WieldMain);
            fill(BodySlot::WieldOff);
        }
        BodySlot::WieldMain | BodySlot::WieldOff => {
            let main_held = held.get(&BodySlot::WieldMain).is_some_and(|names| !names.is_empty());
            held.entry(one_hand(main_held)).or_default().push(name);
        }
        other => fill(other),
    }
}

/// The hand a one-handed weapon goes in: the main hand, then the off hand.
fn one_hand(main_held: bool) -> BodySlot {
    if main_held {
        BodySlot::WieldOff
    } else {
        BodySlot::WieldMain
    }
}

/// Where each worn item sits, for slots as [`worn_slot`] reads them in saved order, so a list of
/// what is worn says which hand as [`slot_conflict`] counts it. Only a one-handed weapon differs
/// from its own slot.
pub fn worn_places(worn: &[BodySlot]) -> Vec<BodySlot> {
    let mut main_held = false;
    worn.iter()
        .map(|slot| match slot {
            BodySlot::WieldMain | BodySlot::WieldOff => {
                let hand = one_hand(main_held);
                main_held = true;
                hand
            }
            BodySlot::WieldTwoHanded => {
                main_held = true;
                BodySlot::WieldTwoHanded
            }
            other => *other,
        })
        .collect()
}

/// What stops one more item of `slot` going on, given what is `worn` as `(name, slot)` in saved
/// order, or `None` when it fits. Worded `"<slot> slot is taken by <item>"`. A save already holding
/// more than a slot allows stays as it is: this answers only for the item being added.
pub fn slot_conflict(worn: &[(&str, BodySlot)], slot: BodySlot) -> Result<Option<String>, String> {
    let table = slot_table().as_ref().map_err(Clone::clone)?;
    let mut held: BTreeMap<BodySlot, Vec<&str>> = BTreeMap::new();
    for (name, slot) in worn {
        occupy(&mut held, *slot, name);
    }
    let holders = |slot: BodySlot| held.get(&slot).map_or(&[][..], Vec::as_slice);
    let needs: &[BodySlot] = match slot {
        BodySlot::None => &[],
        BodySlot::Shield => &[BodySlot::Shield, BodySlot::WieldOff],
        BodySlot::WieldTwoHanded => &[BodySlot::WieldMain, BodySlot::WieldOff],
        BodySlot::WieldMain | BodySlot::WieldOff if holders(BodySlot::WieldOff).is_empty() => &[],
        BodySlot::WieldMain | BodySlot::WieldOff => &[BodySlot::WieldMain],
        BodySlot::Armor => &[BodySlot::Armor],
        BodySlot::Belt => &[BodySlot::Belt],
        BodySlot::Body => &[BodySlot::Body],
        BodySlot::Chest => &[BodySlot::Chest],
        BodySlot::Eyes => &[BodySlot::Eyes],
        BodySlot::Feet => &[BodySlot::Feet],
        BodySlot::Hands => &[BodySlot::Hands],
        BodySlot::Head => &[BodySlot::Head],
        BodySlot::Headband => &[BodySlot::Headband],
        BodySlot::Neck => &[BodySlot::Neck],
        BodySlot::Ring => &[BodySlot::Ring],
        BodySlot::Shoulders => &[BodySlot::Shoulders],
        BodySlot::Wrist => &[BodySlot::Wrist],
    };
    Ok(needs.iter().find(|need| holders(**need).len() >= table.capacity(**need) as usize).map(|need| {
        format!("{} slot is taken by {}", slot_words(*need), holders(*need).join(" and "))
    }))
}

/// Whether a record's type tags mark it as already magic.
pub fn is_magic_marked(tags: &[&str]) -> bool {
    tags.iter().any(|tag| MAGIC_MARKER_TAGS.contains(tag))
}

/// Whether a record can be offered as `+1`…`+5`, and of which kind.
pub fn enhanceable(record: &SheetRule) -> Option<Enhanceable> {
    let tags: Vec<&str> = record.tags.iter().map(|tag| tag.as_str()).collect();
    enhanceable_tags(&tags)
}

/// [`enhanceable`] over a record's type tags alone, for a caller holding a catalog row.
pub fn enhanceable_tags(tags: &[&str]) -> Option<Enhanceable> {
    if is_magic_marked(tags) || tags.iter().any(|tag| NOT_ENHANCEABLE_TAGS.contains(tag)) {
        return None;
    }
    if tags.contains(&"Shield") {
        Some(Enhanceable::Shield)
    } else if tags.contains(&"Armor") {
        Some(Enhanceable::Armor)
    } else if tags.contains(&"Weapon") {
        Some(Enhanceable::Weapon)
    } else {
        None
    }
}

impl Enhanceable {
    /// What a `+N` of this kind costs over its base item: masterwork, then `N²` times the kind's
    /// rate (armor and shield 150 gp + N² × 1,000 gp; weapon 300 gp + N² × 2,000 gp).
    pub fn price_gp(self, base_gp: f64, bonus: u8) -> f64 {
        let (masterwork, rate) = match self {
            Self::Armor | Self::Shield => (150.0, 1_000.0),
            Self::Weapon => (300.0, 2_000.0),
        };
        base_gp + masterwork + f64::from(bonus) * f64::from(bonus) * rate
    }

    /// The catalog key of the `equipment_modifier` record that carries a `+N` of this kind. These
    /// hold the bonus. `PLUS1W`, `PLUS1A` and `PLUS1S` are hidden copies whose converted records
    /// hold none, so attaching one would charge for a bonus that never arrives.
    pub fn modifier_key(self, bonus: u8) -> String {
        let family = match self {
            Self::Armor => "Armor",
            Self::Shield => "Shield",
            Self::Weapon => "Weapon",
        };
        format!("Special Ability ~ +{bonus} ~ {family}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
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
    fn slot_conflict_worn_slot_reads_the_stated_wield_category_before_the_tags() {
        let rules = rules_in("equipment");
        let record = |id: &str| by_id(&rules, id);
        let sword = record("core_rulebook:equipment:longsword_base");
        assert_eq!(worn_slot(sword, &[], Some(WieldCategory::OneHanded)).unwrap(), BodySlot::WieldMain);
        assert_eq!(worn_slot(sword, &[], Some(WieldCategory::TwoHanded)).unwrap(), BodySlot::WieldTwoHanded);
        // The corpus states no category for these: a bow and a stocked crossbow are two hands,
        // a thrown weapon and a sling stay one.
        for (id, want) in [
            ("core_rulebook:equipment:longbow_base", BodySlot::WieldTwoHanded),
            ("core_rulebook:equipment:heavy_crossbow_base", BodySlot::WieldTwoHanded),
            ("core_rulebook:equipment:javelin_base", BodySlot::WieldMain),
            ("core_rulebook:equipment:sling_base", BodySlot::WieldMain),
        ] {
            assert_eq!(worn_slot(record(id), &[], None).unwrap(), want, "{id}");
        }
        // Not a weapon: the category has nothing to say.
        let ring = record("core_rulebook:equipment:ring_of_protection_1");
        assert_eq!(worn_slot(ring, &[], Some(WieldCategory::TwoHanded)).unwrap(), BodySlot::Ring);
    }

    /// `worn_places` agrees with what `slot_conflict` counts: a third one-handed weapon is refused
    /// because the two before it fill the two hands the list names.
    #[test]
    fn equipment_panel_worn_places_name_the_hand() {
        use BodySlot::*;
        assert_eq!(worn_places(&[WieldMain, Armor, WieldMain]), vec![WieldMain, Armor, WieldOff]);
        assert_eq!(worn_places(&[Shield, WieldMain]), vec![Shield, WieldMain], "a shield fills the off hand, not the main");
        assert_eq!(worn_places(&[WieldTwoHanded, Ring, Ring]), vec![WieldTwoHanded, Ring, Ring]);
        assert_eq!(worn_places(&[None]), vec![None]);
        let two = [("Longsword", WieldMain), ("Dagger", WieldMain)];
        assert_eq!(slot_conflict(&two, WieldMain).unwrap().as_deref(), Some("main hand slot is taken by Longsword"));
    }

    #[test]
    fn slot_conflict_the_check_over_slots_alone() {
        use BodySlot::{Head, None as NoSlot, Ring, Shield, WieldMain, WieldTwoHanded};
        let check = |worn: &[(&str, BodySlot)], slot| slot_conflict(worn, slot).unwrap();
        assert_eq!(check(&[], Head), None);
        assert_eq!(check(&[("Hat", Head)], Head).as_deref(), Some("head slot is taken by Hat"));
        assert_eq!(check(&[("A", Ring)], Ring), None);
        assert_eq!(check(&[("A", Ring), ("B", Ring)], Ring).as_deref(), Some("ring slot is taken by A and B"));
        // No slot is never refused, on any body.
        assert_eq!(check(&[("A", WieldTwoHanded), ("B", Head)], NoSlot), None);
        // Hands: main, off, then full, whichever order they went on.
        assert_eq!(check(&[("Sword", WieldMain)], WieldMain), None);
        assert_eq!(
            check(&[("Sword", WieldMain), ("Dagger", WieldMain)], WieldMain).as_deref(),
            Some("main hand slot is taken by Sword"),
        );
        assert_eq!(check(&[("Sword", WieldMain)], Shield), None);
        assert_eq!(
            check(&[("Sword", WieldMain), ("Dagger", WieldMain)], Shield).as_deref(),
            Some("off hand slot is taken by Dagger"),
        );
        assert_eq!(check(&[("Tower", Shield)], WieldMain), None);
        assert_eq!(check(&[("Tower", Shield)], WieldTwoHanded).as_deref(), Some("off hand slot is taken by Tower"));
        assert_eq!(check(&[("Sword", WieldMain)], WieldTwoHanded).as_deref(), Some("main hand slot is taken by Sword"));
        assert_eq!(check(&[("Bow", WieldTwoHanded)], WieldMain).as_deref(), Some("main hand slot is taken by Bow"));
        assert_eq!(check(&[("Bow", WieldTwoHanded)], Shield).as_deref(), Some("off hand slot is taken by Bow"));
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

    /// The keys are real records holding the bonus; the `PLUS…` twins hold none (see
    /// [`Enhanceable::modifier_key`]). Read from the converted package, so a converter change that
    /// moved the bonus would fail here.
    #[test]
    fn enhancement_price_table_modifier_keys_name_the_records_that_hold_the_bonus() {
        let rules = rules_in("equipment_modifier");
        let number_on = |id: &str| {
            rules
                .iter()
                .filter(|r| r.id.as_str() == id || r.id.as_str().starts_with(&format!("{id}#")))
                .any(|r| serde_json::to_value(&r.value).unwrap().get("Number").is_some())
        };
        for (kind, family) in
            [(Enhanceable::Weapon, "weapon"), (Enhanceable::Armor, "armor"), (Enhanceable::Shield, "shield")]
        {
            for bonus in 1..=5u8 {
                let family_title = kind.modifier_key(bonus);
                assert!(family_title.ends_with(&format!("~ +{bonus} ~ {}", family[..1].to_uppercase() + &family[1..])));
                let holder = format!("core_rulebook:equipment_modifier:special_ability_{bonus}_{family}");
                assert!(number_on(&holder), "{holder} holds no number");
                let letter = &family[..1];
                let copy = format!("core_rulebook:equipment_modifier:plus{bonus}{letter}");
                assert!(rules.iter().any(|r| r.id.as_str() == copy), "control: {copy} exists");
                assert!(!number_on(&copy), "{copy} now holds a number: revisit modifier_key");
            }
        }
    }

    /// `decisions.md §8`. The base prices are CRB's: a +1 longsword is 2,315 gp, a +1 chain shirt 1,250 gp.
    #[test]
    fn enhancement_price_table_is_base_plus_masterwork_plus_the_square() {
        for (bonus, weapon, armor) in
            [(1, 2_315.0, 1_250.0), (2, 8_315.0, 4_250.0), (3, 18_315.0, 9_250.0), (4, 32_315.0, 16_250.0), (5, 50_315.0, 25_250.0)]
        {
            assert_eq!(Enhanceable::Weapon.price_gp(15.0, bonus), weapon);
            assert_eq!(Enhanceable::Armor.price_gp(100.0, bonus), armor);
            assert_eq!(Enhanceable::Shield.price_gp(100.0, bonus), armor);
        }
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
