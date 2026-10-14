//! Cyberpunk RED armour (core rulebook p. 184-185 "Armor", p. 350 Master Armor List): the
//! Stopping Power of the head and body locations and the Armor Penalty to REF, DEX and MOVE.
//!
//! Computed here, one `cpr.armor.<field>` row each: head SP and body SP (SP does not stack: the
//! highest source in a location is that location's SP, p. 184), the single most punishing
//! Armor Penalty of everything worn (taken once, even with armour on both locations, p. 184),
//! and REF, DEX and MOVE after that penalty (Minimum 0, p. 350 column header; "MOVE 0
//! completely immobile", p. 184). A Bodyweight Suit is one piece worn on both locations, SP 11
//! each (p. 350). Ablation is a play-time write-in the sheet prints, not a value computed here.
//!
//! Every armour's SP and penalty come from the corpus (`armor.json`).

use std::path::Path;

use serde::Deserialize;

use crate::rules_core::pilot_compute::ComputationExplanation;

const SOURCE_RULES: &str = "Cyberpunk RED core rulebook p. 184";
const SOURCE_TABLE: &str = "Cyberpunk RED core rulebook p. 350";
/// p. 350: "Unlike other armor, a Bodyweight Suit isn't bought in two pieces, and must always
/// be worn on both your body and head location. Each location has its own SP11."
pub const BODYWEIGHT_SUIT: &str = "Bodyweight Suit";

/// The two armour locations (p. 184: "It can be worn on either your body or your head").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprArmorLocation {
    Head,
    Body,
}

impl CprArmorLocation {
    fn label(self) -> &'static str {
        match self {
            CprArmorLocation::Head => "head",
            CprArmorLocation::Body => "body",
        }
    }
}

/// One `armor` corpus record, the fields the engine reads. `sp` is `null` for the Bulletproof
/// Shield, which has HP instead and is held in one arm, not worn at a location.
#[derive(Debug, Clone, Deserialize)]
struct CprArmorRecord {
    name: String,
    sp: Option<u8>,
    armor_penalty_ref_dex_move: i8,
}

/// One wearable armour: its Stopping Power and its Armor Penalty to REF, DEX and MOVE (0 or
/// negative).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprArmor {
    pub name: String,
    pub sp: u8,
    pub penalty: i8,
}

/// One armour piece the character wears, by its Master Armor List name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprWornArmor {
    pub name: String,
    pub location: CprArmorLocation,
}

/// A Stopping Power source that is not worn armour (Skin Weave, Subdermal Armor, p. 365). It
/// competes with worn armour in its location; the highest SP wins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprSpSource {
    pub label: String,
    pub location: CprArmorLocation,
    pub sp: u8,
}

/// The wearable armours of a CPR corpus directory.
#[derive(Debug, Clone)]
pub struct CprArmorCatalog {
    armours: Vec<CprArmor>,
    held: Vec<String>,
}

impl CprArmorCatalog {
    /// Load `armor.json` from `corpus_dir` (`GameSystem::CyberpunkRed.package_roots(..).corpus`).
    /// A record without SP (the Bulletproof Shield) is held, not worn; a positive penalty is
    /// refused (the book prints only "None", "-2" and "-4").
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        let path = corpus_dir.join("armor.json");
        let text = std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let records: Vec<CprArmorRecord> =
            serde_json::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
        let mut catalog = CprArmorCatalog { armours: Vec::new(), held: Vec::new() };
        for record in records {
            if record.armor_penalty_ref_dex_move > 0 {
                return Err(format!("{}: an Armor Penalty is never positive", record.name));
            }
            match record.sp {
                Some(sp) => catalog.armours.push(CprArmor { name: record.name, sp, penalty: record.armor_penalty_ref_dex_move }),
                None => catalog.held.push(record.name),
            }
        }
        if !catalog.armours.iter().any(|a| a.name == BODYWEIGHT_SUIT) {
            return Err(format!("armor.json has no {BODYWEIGHT_SUIT}"));
        }
        Ok(catalog)
    }

    /// The number of wearable (SP-bearing) armours.
    pub fn len(&self) -> usize {
        self.armours.len()
    }

    pub fn is_empty(&self) -> bool {
        self.armours.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<&CprArmor> {
        self.armours.iter().find(|a| a.name == name)
    }

    /// The armour a character wears, with any other SP sources. Refused: an unknown armour, a
    /// held item (the Bulletproof Shield), and more than one Bodyweight Suit (p. 350: "You
    /// can't wear more than one Bodyweight Suit").
    pub fn wear(&self, worn: &[CprWornArmor], other_sources: &[CprSpSource]) -> Result<CprArmorLoadout, String> {
        let mut loadout = CprArmorLoadout { sp_head: 0, sp_body: 0, penalty: 0, sources: Vec::new(), bodyweight_suit: false };
        for piece in worn {
            if self.held.iter().any(|h| *h == piece.name) {
                return Err(format!("{} is held in one arm, not worn at a location (p. 350)", piece.name));
            }
            let armour = self.get(&piece.name).ok_or_else(|| format!("{} is not on the Master Armor List (p. 350)", piece.name))?;
            let locations: &[CprArmorLocation] = if armour.name == BODYWEIGHT_SUIT {
                if loadout.bodyweight_suit {
                    return Err(format!("you can't wear more than one {BODYWEIGHT_SUIT} (p. 350)"));
                }
                loadout.bodyweight_suit = true;
                &[CprArmorLocation::Head, CprArmorLocation::Body]
            } else {
                std::slice::from_ref(&piece.location)
            };
            for location in locations {
                loadout.add(*location, armour.sp, &armour.name);
            }
            loadout.penalty = loadout.penalty.min(armour.penalty);
        }
        for source in other_sources {
            loadout.add(source.location, source.sp, &source.label);
        }
        Ok(loadout)
    }
}

/// Worn armour and the numbers it puts on the sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprArmorLoadout {
    sp_head: u8,
    sp_body: u8,
    penalty: i8,
    /// `(location, sp, name)` of every source, in the order given, for the row text.
    sources: Vec<(CprArmorLocation, u8, String)>,
    bodyweight_suit: bool,
}

impl CprArmorLoadout {
    fn add(&mut self, location: CprArmorLocation, sp: u8, name: &str) {
        let slot = match location {
            CprArmorLocation::Head => &mut self.sp_head,
            CprArmorLocation::Body => &mut self.sp_body,
        };
        *slot = (*slot).max(sp);
        self.sources.push((location, sp, name.to_string()));
    }

    /// The location's SP: the highest single source, never a sum.
    pub fn sp(&self, location: CprArmorLocation) -> u8 {
        match location {
            CprArmorLocation::Head => self.sp_head,
            CprArmorLocation::Body => self.sp_body,
        }
    }

    /// The most punishing Armor Penalty of everything worn (0 or negative), taken once.
    pub fn penalty(&self) -> i8 {
        self.penalty
    }

    pub fn bodyweight_suit_worn(&self) -> bool {
        self.bodyweight_suit
    }

    /// A REF, DEX or MOVE value after the Armor Penalty, Minimum 0.
    pub fn adjusted(&self, stat: u8) -> u8 {
        (i16::from(stat) + i16::from(self.penalty)).max(0) as u8
    }

    fn sources_text(&self, location: CprArmorLocation) -> String {
        let names: Vec<String> = self
            .sources
            .iter()
            .filter(|(l, _, _)| *l == location)
            .map(|(_, sp, name)| format!("{name} {sp}"))
            .collect();
        if names.is_empty() { "nothing worn".to_string() } else { names.join(", ") }
    }

    /// The `cpr.armor.` rows: head SP, body SP, the penalty, then REF, DEX and MOVE after it.
    pub fn rows(&self, reflexes: u8, dexterity: u8, movement: u8) -> Vec<ComputationExplanation> {
        let row = |id: &str, value: i16, detail: String| ComputationExplanation { id: format!("cpr.armor.{id}"), value, detail };
        let sp_row = |location: CprArmorLocation| {
            let sp = self.sp(location);
            let label = location.label();
            row(
                &format!("sp_{label}"),
                i16::from(sp),
                format!(
                    "{} SP {sp}: the highest source in the {label} location ({}); SP does not stack ({SOURCE_RULES})",
                    if location == CprArmorLocation::Head { "Head" } else { "Body" },
                    self.sources_text(location)
                ),
            )
        };
        let stat_row = |id: &str, label: &str, value: u8| {
            let after = self.adjusted(value);
            row(
                id,
                i16::from(after),
                format!("{label} {after} = max(0, {label} {value} + Armor Penalty {}) ({SOURCE_RULES}; Minimum 0, {SOURCE_TABLE})", self.penalty),
            )
        };
        vec![
            sp_row(CprArmorLocation::Head),
            sp_row(CprArmorLocation::Body),
            row(
                "penalty",
                i16::from(self.penalty),
                format!(
                    "Armor Penalty {}: the most punishing penalty of the armour worn, taken once for both locations ({SOURCE_RULES}; {SOURCE_TABLE})",
                    self.penalty
                ),
            ),
            stat_row("ref", "REF", reflexes),
            stat_row("dex", "DEX", dexterity),
            stat_row("move", "MOVE", movement),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// The Master Armor List, transcribed from the page (p351, printed p. 350; the same SP and
    /// penalties print on p098 and p186), not from the corpus the engine reads:
    /// (armour, SP, penalty to REF, DEX and MOVE).
    const MASTER_ARMOR_LIST_P351: [(&str, u8, i8); 8] = [
        ("Leathers", 4, 0),
        ("Kevlar", 7, 0),
        ("Light Armorjack", 11, 0),
        ("Bodyweight Suit", 11, 0),
        ("Medium Armorjack", 12, -2),
        ("Heavy Armorjack", 13, -2),
        ("Flak", 15, -4),
        ("Metalgear", 18, -4),
    ];

    fn catalog() -> CprArmorCatalog {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/cyberpunk-red/corpus");
        CprArmorCatalog::load(&dir).expect("the CPR armour corpus loads")
    }

    fn wear(name: &str, location: CprArmorLocation) -> CprWornArmor {
        CprWornArmor { name: name.to_string(), location }
    }

    fn row(rows: &[ComputationExplanation], id: &str) -> i16 {
        rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id}")).value
    }

    #[test]
    fn the_catalog_equals_the_p351_master_armor_list() {
        let catalog = catalog();
        assert_eq!(catalog.len(), 8, "8 SP-bearing armours (the Bulletproof Shield is held, not worn)");
        for (name, sp, penalty) in MASTER_ARMOR_LIST_P351 {
            let armour = catalog.get(name).unwrap_or_else(|| panic!("{name} missing"));
            assert_eq!((armour.sp, armour.penalty), (sp, penalty), "{name}");
        }
    }

    #[test]
    fn light_armorjack_head_and_flak_body_give_body_sp_15_head_sp_11_and_minus_4_once() {
        let worn = [wear("Light Armorjack", CprArmorLocation::Head), wear("Flak", CprArmorLocation::Body)];
        let armour = catalog().wear(&worn, &[]).unwrap();
        assert_eq!(armour.sp(CprArmorLocation::Body), 15);
        assert_eq!(armour.sp(CprArmorLocation::Head), 11);
        assert_eq!(armour.penalty(), -4);
        let rows = armour.rows(7, 6, 5);
        assert_eq!(row(&rows, "cpr.armor.sp_body"), 15);
        assert_eq!(row(&rows, "cpr.armor.sp_head"), 11);
        assert_eq!(row(&rows, "cpr.armor.penalty"), -4);
        assert_eq!(row(&rows, "cpr.armor.ref"), 3, "REF 7 - 4, applied once");
        assert_eq!(row(&rows, "cpr.armor.dex"), 2, "DEX 6 - 4, applied once");
        assert_eq!(row(&rows, "cpr.armor.move"), 1, "MOVE 5 - 4, applied once");
    }

    #[test]
    fn move_3_with_minus_4_floors_at_0() {
        let worn = [wear("Light Armorjack", CprArmorLocation::Head), wear("Flak", CprArmorLocation::Body)];
        let armour = catalog().wear(&worn, &[]).unwrap();
        assert_eq!(armour.adjusted(3), 0);
        assert_eq!(row(&armour.rows(3, 3, 3), "cpr.armor.move"), 0);
        assert_eq!(armour.adjusted(2), 0, "REF/DEX share the same Minimum 0 (p351 column header)");
    }

    #[test]
    fn sp_does_not_stack_the_highest_source_in_a_location_wins() {
        let worn = [
            wear("Kevlar", CprArmorLocation::Body),
            wear("Light Armorjack", CprArmorLocation::Body),
            wear("Leathers", CprArmorLocation::Body),
        ];
        let armour = catalog().wear(&worn, &[]).unwrap();
        assert_eq!(armour.sp(CprArmorLocation::Body), 11, "max(7, 11, 4), never 7 + 11 + 4");
        assert_eq!(armour.sp(CprArmorLocation::Head), 0, "nothing on the head");
    }

    #[test]
    fn the_single_worst_penalty_applies_once_across_both_locations() {
        let worn = [wear("Medium Armorjack", CprArmorLocation::Head), wear("Heavy Armorjack", CprArmorLocation::Body)];
        let armour = catalog().wear(&worn, &[]).unwrap();
        assert_eq!(armour.penalty(), -2, "two -2 pieces are -2, not -4");
        let worn = [wear("Medium Armorjack", CprArmorLocation::Head), wear("Metalgear", CprArmorLocation::Body)];
        assert_eq!(catalog().wear(&worn, &[]).unwrap().penalty(), -4, "the most punishing of -2 and -4");
        let worn = [wear("Kevlar", CprArmorLocation::Head), wear("Light Armorjack", CprArmorLocation::Body)];
        let armour = catalog().wear(&worn, &[]).unwrap();
        assert_eq!(armour.penalty(), 0);
        assert_eq!(row(&armour.rows(8, 7, 6), "cpr.armor.ref"), 8, "no penalty leaves REF unchanged");
    }

    #[test]
    fn the_bodyweight_suit_covers_both_locations_at_sp_11() {
        let worn = [wear("Bodyweight Suit", CprArmorLocation::Body)];
        let armour = catalog().wear(&worn, &[]).unwrap();
        assert_eq!(armour.sp(CprArmorLocation::Body), 11);
        assert_eq!(armour.sp(CprArmorLocation::Head), 11, "p351: worn on both body and head, each SP11");
        assert!(armour.bodyweight_suit_worn());

        let worn = [wear("Bodyweight Suit", CprArmorLocation::Head), wear("Flak", CprArmorLocation::Body)];
        let armour = catalog().wear(&worn, &[]).unwrap();
        assert_eq!(armour.sp(CprArmorLocation::Body), 15);
        assert_eq!(armour.sp(CprArmorLocation::Head), 11);
        assert_eq!(armour.penalty(), -4);
    }

    #[test]
    fn other_sp_sources_compete_with_worn_armour_highest_wins() {
        let worn = [wear("Kevlar", CprArmorLocation::Body)];
        let subdermal = CprSpSource { label: "Subdermal Armor".to_string(), location: CprArmorLocation::Body, sp: 11 };
        let armour = catalog().wear(&worn, std::slice::from_ref(&subdermal)).unwrap();
        assert_eq!(armour.sp(CprArmorLocation::Body), 11);
        let worn = [wear("Flak", CprArmorLocation::Body)];
        assert_eq!(catalog().wear(&worn, &[subdermal]).unwrap().sp(CprArmorLocation::Body), 15);
    }

    #[test]
    fn refusals_name_their_reason() {
        let catalog = catalog();
        let two_suits = [wear("Bodyweight Suit", CprArmorLocation::Body), wear("Bodyweight Suit", CprArmorLocation::Head)];
        assert!(catalog.wear(&two_suits, &[]).unwrap_err().contains("more than one Bodyweight Suit"));
        let shield = [wear("Bulletproof Shield", CprArmorLocation::Body)];
        assert!(catalog.wear(&shield, &[]).unwrap_err().contains("Bulletproof Shield"));
        let unknown = [wear("Power Armor", CprArmorLocation::Body)];
        assert!(catalog.wear(&unknown, &[]).unwrap_err().contains("Power Armor"));
    }

    #[test]
    fn rows_cite_the_page_and_the_stat_order_is_ref_dex_move() {
        let worn = [wear("Flak", CprArmorLocation::Body)];
        let rows = catalog().wear(&worn, &[]).unwrap().rows(6, 5, 4);
        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(
            ids,
            ["cpr.armor.sp_head", "cpr.armor.sp_body", "cpr.armor.penalty", "cpr.armor.ref", "cpr.armor.dex", "cpr.armor.move"]
        );
        assert!(rows.iter().all(|r| r.detail.contains("p. ")), "every row cites a page");
    }
}
