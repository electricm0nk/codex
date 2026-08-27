//! UPsi equipment table -- full in-scope corpus coverage.
//!
//! Record coverage: every real, active, non-legacy-alias record across
//! `up_equipment.lst` (326 records: weapons, armor, wondrous/psionic
//! items) and `up_equipmods.lst` (113 real records, Equipmods category)
//! -- 439 total, `equipment_tables()` + `equipmod_tables()` combined.
//! `docs/work-inventory.json` declares 552 units for this book's
//! equipment/equipment_modifier kinds combined (326 + 226); the 113-unit
//! gap between 226 declared and 113 landed is real, named, and explained
//! below -- not a silent shortfall.
//!
//! **`up_equipmods.lst` carries two non-standalone-content hazards,
//! both excluded from this table, both found and corrected across two
//! passes (the first pass wrongly fabricated near-empty entries for the
//! second hazard rather than excluding it -- corrected before landing,
//! not left as shipped):**
//!
//! 1. **One `.MOD` row** (`Special Ability ~ Keen ~ Weapon.MOD`, line 13)
//!    -- a psionic-specific prerequisite restriction injected onto an
//!    existing cross-book `Keen` weapon special ability, not a new
//!    standalone UPsi record. Same exclusion `§51`'s tier-1 archetype
//!    tables already establish for `.MOD`-injected grants.
//! 2. **113 `.COPY=<SHORTCODE>` rows, every one `VISIBLE:NO` with no
//!    `COST:`/`SPROP:` of its own** (e.g. `Special Ability ~ Aporter ~
//!    Armor.COPY=APORT`) -- legacy/internal short-code aliases of the 113
//!    real base modifiers this table already carries under their own
//!    descriptive names. Confirmed genuinely non-standalone, not merely
//!    assumed: none of the 113 short codes (`APORT`, `AGILE`, `COLLI_A`,
//!    etc.) appears anywhere else in the UPsi corpus as an `EQMOD:` lookup
//!    target or any other real reference -- they exist only as their own
//!    declaration line. **Same exclusion shape `ultimate_intrigue
//!    ::equipment_tables` and `advanced_race_guide::equipment_data
//!    ::equipmods` already establish for their own `VISIBLE:NO` `.COPY=`
//!    "Old KEYs" blocks** (UI's own doc comment: *"the classifier does not
//!    know about VISIBLE:NO and counts both rows as distinct declared
//!    units -- the classifier's notion of a unit being broader than
//!    reality"*). This module's own first extraction pass got this wrong
//!    in the opposite direction from ARG's/UI's -- it fabricated 113
//!    near-empty table rows (no cost, no description, keyed only by the
//!    short code) purely to make `docs/work-inventory.json`'s declared
//!    count resolve, rather than recognizing the same legacy-alias shape
//!    UI/ARG already named and excluding it. Corrected here before commit:
//!    the 113-unit gap is real, is classifier over-counting (not missing
//!    content), and is named rather than closed by manufacturing entries.
//!
//! Unlike `ultimate_magic::equipment_tables` (24/26 with a real SPROP
//! description) or `advanced_race_guide::equipment_tables` (zero `DESC:`
//! tokens anywhere), UPsi's field coverage is genuinely mixed: 216/326
//! equipment records and 95 of the 113 real equipmod records carry a real
//! `SPROP:`-sourced description; the rest (mostly the psionic base items --
//! `Dorje`/`Power Stone`/`Psicrown`/etc. -- and the Astral Suit family)
//! carry none. Neither file carries a `DESC:` token anywhere (confirmed by
//! direct grep, zero hits across both files) -- `SPROP:` is this book's
//! own description source, same convention ACG/ARG/UM already established.
//!
//! **Categorization, coarser than a per-block split:** every record's
//! corpus `TYPE:` first segment of `Weapon`/`Armor`/`Shield`/`Enhancement`
//! maps to `ArmsArmor` (52 records); everything else (`Psionic`,
//! `PsionicTattoo`, `Magic`, `Skin`, `Psicrown`, `PowerStone`, `MindStone`,
//! `GreaterMindStone`, `Dorje`, and any record with no `TYPE:` field at
//! all) maps to `MagicItems` (274 records) -- this book's equipment is
//! overwhelmingly wondrous/psionic items, not mundane gear, unlike every
//! previously-landed book's own General-heavy split.
//!
//! **One deliberately-verbatim corpus oddity, not a parsing defect:** the
//! Astral Suit family (`Astral Armor`, `Astral Juggernaut`) carries a real
//! `COST:-150` token in the raw corpus (`up_equipment.lst:12-13`) -- a
//! negative cost for Aegis's own body-integrated armor form, not a purchase
//! price. Extracted verbatim, not clamped or treated as an extraction
//! error.
//!
//! Where `OUTPUTNAME:` differs from the record's own leading display
//! field (a handful of records, e.g. `Mind Blade (Light Bludgeoning)` ->
//! `Mind Blade, Light Bludgeoning`), `name` is sourced from `OUTPUTNAME:`
//! -- the corpus's own preferred display text -- while `key` stays the
//! record's real corpus identity (its `KEY:` token when present, else the
//! leading field), matching every other table's own key/name split.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    ArmsArmor,
    MagicItems,
    Equipmods,
}

impl EquipmentCategory {
    pub const ALL: &'static [EquipmentCategory] = &[
        EquipmentCategory::ArmsArmor,
        EquipmentCategory::MagicItems,
        EquipmentCategory::Equipmods,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    /// The record's `KEY:` token when present, else its own leading
    /// display field -- the corpus identity, distinct from `name` when
    /// `OUTPUTNAME:` supplies a different display string.
    pub key: &'static str,
    pub category: EquipmentCategory,
    /// Display name -- `OUTPUTNAME:` when the corpus record carries one,
    /// else the same as `key`'s source field.
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. Some records
    /// (the Astral Suit family) carry a real, verbatim negative value --
    /// see this module's own doc comment.
    pub cost_gp: Option<f64>,
    /// Weight in pounds from the corpus `WT:` token. `None` for every
    /// equipmod record (modifiers carry no independent weight, matching
    /// every other book's own established finding).
    pub weight_lbs: Option<f64>,
    /// Descriptive text, sourced from the corpus `SPROP:` token(s), joined
    /// with `"; "` when a record carries more than one. `None` when the
    /// corpus record has no `SPROP:` token at all -- see this module's own
    /// doc comment for the real field-coverage split.
    pub description: Option<&'static str>,
}

/// SD-28-E15 equipment field-coverage audit row, mirroring the shape every
/// sibling book's own `EquipmentFieldCoverage` already establishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    pub total_records: u32,
    pub records_expected: u32,
    pub has_cost: u32,
    pub has_weight: u32,
    pub has_description: u32,
}

/// Computes this book's equipment field-coverage audit row.
pub fn field_coverage_report() -> EquipmentFieldCoverage {
    let table = equipment_tables();
    let mods = equipmod_tables();
    EquipmentFieldCoverage {
        total_records: (table.len() + mods.len()) as u32,
        records_expected: 439,
        has_cost: table.iter().chain(mods.iter()).filter(|e| e.cost_gp.is_some()).count() as u32,
        has_weight: table.iter().chain(mods.iter()).filter(|e| e.weight_lbs.is_some()).count() as u32,
        has_description: table
            .iter()
            .chain(mods.iter())
            .filter(|e| e.description.is_some())
            .count() as u32,
    }
}

/// Full UPsi equipment table: `up_equipment.lst`'s 326 real records
/// (52 ArmsArmor + 274 MagicItems).
const EQUIPMENT_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry { key: "Astral Skin", category: EquipmentCategory::MagicItems, name: "Astral Skin", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:11
    EquipmentTableEntry { key: "Astral Armor", category: EquipmentCategory::ArmsArmor, name: "Astral Armor", cost_gp: Some(-150.0_f64), weight_lbs: Some(40.0_f64), description: None }, // up_equipment.lst:12
    EquipmentTableEntry { key: "Astral Juggernaut", category: EquipmentCategory::ArmsArmor, name: "Astral Juggernaut", cost_gp: Some(-150.0_f64), weight_lbs: Some(50.0_f64), description: None }, // up_equipment.lst:13
    EquipmentTableEntry { key: "Astral Suit Ram 1", category: EquipmentCategory::ArmsArmor, name: "Ram", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:17
    EquipmentTableEntry { key: "Astral Suit Ram 2", category: EquipmentCategory::ArmsArmor, name: "Ram", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:18
    EquipmentTableEntry { key: "Astral Suit Ram 3", category: EquipmentCategory::ArmsArmor, name: "Ram", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:19
    EquipmentTableEntry { key: "Astral Suit Ram 4", category: EquipmentCategory::ArmsArmor, name: "Ram", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:20
    EquipmentTableEntry { key: "Mind Blade (Light Bludgeoning)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, Light Bludgeoning", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:25
    EquipmentTableEntry { key: "Mind Blade (Light Piercing)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, Light Piercing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:26
    EquipmentTableEntry { key: "Mind Blade (Light Slashing)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, Light Slashing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:27
    EquipmentTableEntry { key: "Mind Blade (One-Handed Bludgeoning)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, One-Handed Bludgeoning", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:28
    EquipmentTableEntry { key: "Mind Blade (One-Handed Piercing)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, One-Handed Piercing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:29
    EquipmentTableEntry { key: "Mind Blade (One-Handed Slashing)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, One-Handed Slashing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:30
    EquipmentTableEntry { key: "Mind Blade (Two-Handed Bludgeoning)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, Two-Handed Bludgeoning", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:31
    EquipmentTableEntry { key: "Mind Blade (Two-Handed Piercing)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, Two-Handed Piercing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:32
    EquipmentTableEntry { key: "Mind Blade (Two-Handed Slashing)", category: EquipmentCategory::ArmsArmor, name: "Mind Blade, Two-Handed Slashing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:33
    EquipmentTableEntry { key: "Mind Dagger", category: EquipmentCategory::ArmsArmor, name: "Mind Dagger", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:37
    EquipmentTableEntry { key: "Deadly Fist", category: EquipmentCategory::ArmsArmor, name: "Deadly Fist", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:38
    EquipmentTableEntry { key: "Mind Arrow", category: EquipmentCategory::ArmsArmor, name: "Mind Arrow", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:40
    EquipmentTableEntry { key: "Mind Xephyr Knife", category: EquipmentCategory::ArmsArmor, name: "Mind Xephyr Knife", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:41
    EquipmentTableEntry { key: "Mind Bolt (Long Range Bludgeoning)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Long Range Bludgeoning", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:45
    EquipmentTableEntry { key: "Mind Bolt (Long Range Piercing)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Long Range Piercing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:46
    EquipmentTableEntry { key: "Mind Bolt (Long Range Slashing)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Long Range Slashing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:47
    EquipmentTableEntry { key: "Mind Bolt (Medium Range Bludgeoning)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Medium Range Bludgeoning", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:48
    EquipmentTableEntry { key: "Mind Bolt (Medium Range Piercing)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Medium Range Piercing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:49
    EquipmentTableEntry { key: "Mind Bolt (Medium Range Slashing)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Medium Range Slashing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:50
    EquipmentTableEntry { key: "Mind Bolt (Short Range Bludgeoning)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Short Range Bludgeoning", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:51
    EquipmentTableEntry { key: "Mind Bolt (Short Range Piercing)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Short Range Piercing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:52
    EquipmentTableEntry { key: "Mind Bolt (Short Range Slashing)", category: EquipmentCategory::ArmsArmor, name: "Mind Bolt, Short Range Slashing", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:53
    EquipmentTableEntry { key: "Mind Armor (Light)", category: EquipmentCategory::ArmsArmor, name: "Mind Armor (Light)", cost_gp: Some(0.0_f64), weight_lbs: Some(25.0_f64), description: None }, // up_equipment.lst:57
    EquipmentTableEntry { key: "Mind Armor (Medium)", category: EquipmentCategory::ArmsArmor, name: "Mind Armor (Medium)", cost_gp: Some(0.0_f64), weight_lbs: Some(40.0_f64), description: None }, // up_equipment.lst:58
    EquipmentTableEntry { key: "Mind Armor (Heavy)", category: EquipmentCategory::ArmsArmor, name: "Mind Armor (Heavy)", cost_gp: Some(0.0_f64), weight_lbs: Some(45.0_f64), description: None }, // up_equipment.lst:59
    EquipmentTableEntry { key: "Improved Mind Armor (Light)", category: EquipmentCategory::ArmsArmor, name: "Improved Mind Armor (Light)", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:61
    EquipmentTableEntry { key: "Improved Mind Armor (Medium)", category: EquipmentCategory::ArmsArmor, name: "Improved Mind Armor (Medium)", cost_gp: Some(0.0_f64), weight_lbs: Some(30.0_f64), description: None }, // up_equipment.lst:62
    EquipmentTableEntry { key: "Improved Mind Armor (Heavy)", category: EquipmentCategory::ArmsArmor, name: "Improved Mind Armor (Heavy)", cost_gp: Some(0.0_f64), weight_lbs: Some(50.0_f64), description: None }, // up_equipment.lst:63
    EquipmentTableEntry { key: "Mind Shield", category: EquipmentCategory::ArmsArmor, name: "Mind Shield", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:67
    EquipmentTableEntry { key: "Mind Shield (Heavy)", category: EquipmentCategory::ArmsArmor, name: "Mind Shield (Heavy)", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:68
    EquipmentTableEntry { key: "Mind Shield (Tower)", category: EquipmentCategory::ArmsArmor, name: "Mind Shield (Tower)", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: Some("can grant full cover") }, // up_equipment.lst:69
    EquipmentTableEntry { key: "Flurry of Fists", category: EquipmentCategory::ArmsArmor, name: "Flurry of Fists", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:73
    EquipmentTableEntry { key: "Flurry of Strikes", category: EquipmentCategory::ArmsArmor, name: "Flurry of Strikes", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:74
    EquipmentTableEntry { key: "Astral Warrior Weapon (Bludgeoning)", category: EquipmentCategory::ArmsArmor, name: "Astral Warrior Weapon (Bludgeoning)", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:79
    EquipmentTableEntry { key: "Astral Warrior Weapon (Piercing)", category: EquipmentCategory::ArmsArmor, name: "Astral Warrior Weapon (Piercing)", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:80
    EquipmentTableEntry { key: "Astral Warrior Weapon (Slashing)", category: EquipmentCategory::ArmsArmor, name: "Astral Warrior Weapon (Slashing)", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:81
    EquipmentTableEntry { key: "Crystal Shard", category: EquipmentCategory::ArmsArmor, name: "Crystal Shard", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:86
    EquipmentTableEntry { key: "Xephyr Knife", category: EquipmentCategory::ArmsArmor, name: "Xephyr Knife", cost_gp: Some(15.0_f64), weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:91
    EquipmentTableEntry { key: "Dorje", category: EquipmentCategory::MagicItems, name: "Dorje", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:96
    EquipmentTableEntry { key: "Power Stone", category: EquipmentCategory::MagicItems, name: "Power Stone", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:97
    EquipmentTableEntry { key: "Psicrown", category: EquipmentCategory::MagicItems, name: "Psicrown", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:98
    EquipmentTableEntry { key: "Psionic Tattoo", category: EquipmentCategory::MagicItems, name: "Psionic Tattoo", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:99
    EquipmentTableEntry { key: "Crawling Tattoo", category: EquipmentCategory::MagicItems, name: "Crawling Tattoo", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:100
    EquipmentTableEntry { key: "Mind Stone", category: EquipmentCategory::MagicItems, name: "Mind Stone", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:101
    EquipmentTableEntry { key: "Mind Stone (Greater)", category: EquipmentCategory::MagicItems, name: "Greater Mind Stone", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:102
    EquipmentTableEntry { key: "Leather of Confined Spaces", category: EquipmentCategory::ArmsArmor, name: "Leather of Confined Spaces", cost_gp: Some(10.0_f64), weight_lbs: Some(15.0_f64), description: Some("1/day reduce size by one category, +5 to Escape Artist and Combat Manuever checks to escape from grapples") }, // up_equipment.lst:107
    EquipmentTableEntry { key: "Shadow Shirt", category: EquipmentCategory::ArmsArmor, name: "Shadow Shirt", cost_gp: Some(100.0_f64), weight_lbs: Some(25.0_f64), description: Some("use shadow body for 5 rounds/day; +5 to Stealth checks") }, // up_equipment.lst:108
    EquipmentTableEntry { key: "Skinwalker's Leather", category: EquipmentCategory::ArmsArmor, name: "Skinwalker's Leather", cost_gp: Some(25.0_f64), weight_lbs: Some(20.0_f64), description: Some("merges with skin; gain 1 customization point as 2nd level aberrant or treat class level as 2 higher for customizations") }, // up_equipment.lst:109
    EquipmentTableEntry { key: "Plate of the Juggernaut", category: EquipmentCategory::ArmsArmor, name: "Plate of the Juggernaut", cost_gp: Some(1500.0_f64), weight_lbs: Some(50.0_f64), description: Some("gain 1 customization point as 2nd level aegis or treat class level as 2 higher for customizations") }, // up_equipment.lst:110
    EquipmentTableEntry { key: "Strengthbleeder", category: EquipmentCategory::ArmsArmor, name: "Strengthbleeder", cost_gp: Some(15.0_f64), weight_lbs: Some(4.0_f64), description: Some("1/day, until end of combat, enemy takes 1 Strength damage when hit and wielder gets a cumulative +1 enhancement bonus to Strength (up to +8)") }, // up_equipment.lst:115
    EquipmentTableEntry { key: "Cognizance Crystal (1)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (1)", cost_gp: Some(1000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 1 Power Point to be used for manifesting powers") }, // up_equipment.lst:120
    EquipmentTableEntry { key: "Cognizance Crystal (3)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (3)", cost_gp: Some(4000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 3 Power Points to be used for manifesting powers") }, // up_equipment.lst:121
    EquipmentTableEntry { key: "Cognizance Crystal (5)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (5)", cost_gp: Some(9000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 5 Power Points to be used for manifesting powers") }, // up_equipment.lst:122
    EquipmentTableEntry { key: "Cognizance Crystal (7)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (7)", cost_gp: Some(16000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 7 Power Points to be used for manifesting powers") }, // up_equipment.lst:123
    EquipmentTableEntry { key: "Cognizance Crystal (9)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (9)", cost_gp: Some(25000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 9 Power Points to be used for manifesting powers") }, // up_equipment.lst:124
    EquipmentTableEntry { key: "Cognizance Crystal (11)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (11)", cost_gp: Some(36000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 11 Power Points to be used for manifesting powers") }, // up_equipment.lst:125
    EquipmentTableEntry { key: "Cognizance Crystal (13)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (13)", cost_gp: Some(49000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 13 Power Points to be used for manifesting powers") }, // up_equipment.lst:126
    EquipmentTableEntry { key: "Cognizance Crystal (15)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (15)", cost_gp: Some(64000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 15 Power Points to be used for manifesting powers") }, // up_equipment.lst:127
    EquipmentTableEntry { key: "Cognizance Crystal (17)", category: EquipmentCategory::MagicItems, name: "Cognizance Crystal (17)", cost_gp: Some(81000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Stores up to 17 Power Points to be used for manifesting powers") }, // up_equipment.lst:128
    EquipmentTableEntry { key: "Crystal Bow +1", category: EquipmentCategory::MagicItems, name: "Crystal Bow +1", cost_gp: Some(11200.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind bolt is increased by +1, up to +5") }, // up_equipment.lst:133
    EquipmentTableEntry { key: "Crystal Bow +2", category: EquipmentCategory::MagicItems, name: "Crystal Bow +2", cost_gp: Some(44800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind bolt is increased by +2, up to +5") }, // up_equipment.lst:134
    EquipmentTableEntry { key: "Crystal Bow +3", category: EquipmentCategory::MagicItems, name: "Crystal Bow +3", cost_gp: Some(100800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind bolt is increased by +3, up to +5") }, // up_equipment.lst:135
    EquipmentTableEntry { key: "Crystal Gauntlets +1", category: EquipmentCategory::MagicItems, name: "Crystal Gauntlets +1", cost_gp: Some(11200.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of empowered unarmed strikes is increased by +1, up to +5") }, // up_equipment.lst:136
    EquipmentTableEntry { key: "Crystal Gauntlets +2", category: EquipmentCategory::MagicItems, name: "Crystal Gauntlets +2", cost_gp: Some(44800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of empowered unarmed strikes is increased by +2, up to +5") }, // up_equipment.lst:137
    EquipmentTableEntry { key: "Crystal Gauntlets +3", category: EquipmentCategory::MagicItems, name: "Crystal Gauntlets +3", cost_gp: Some(100800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of empowered unarmed strikes is increased by +3, up to +5") }, // up_equipment.lst:138
    EquipmentTableEntry { key: "Crystal Grip +1", category: EquipmentCategory::MagicItems, name: "Crystal Grip +1", cost_gp: Some(5600.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind shield is increased by +1, up to +5") }, // up_equipment.lst:139
    EquipmentTableEntry { key: "Crystal Grip +2", category: EquipmentCategory::MagicItems, name: "Crystal Grip +2", cost_gp: Some(22400.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind shield is increased by +2, up to +5") }, // up_equipment.lst:140
    EquipmentTableEntry { key: "Crystal Grip +3", category: EquipmentCategory::MagicItems, name: "Crystal Grip +3", cost_gp: Some(50400.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind shield is increased by +3, up to +5") }, // up_equipment.lst:141
    EquipmentTableEntry { key: "Crystal Hilt (Light) +1", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (Light) +1", cost_gp: Some(11200.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +1, up to +5") }, // up_equipment.lst:142
    EquipmentTableEntry { key: "Crystal Hilt (Light) +2", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (Light) +2", cost_gp: Some(44800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +2, up to +5") }, // up_equipment.lst:143
    EquipmentTableEntry { key: "Crystal Hilt (Light) +3", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (Light) +3", cost_gp: Some(100800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +3, up to +5") }, // up_equipment.lst:144
    EquipmentTableEntry { key: "Crystal Hilt (One-Handed) +1", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (One-Handed) +1", cost_gp: Some(11200.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +1, up to +5") }, // up_equipment.lst:145
    EquipmentTableEntry { key: "Crystal Hilt (One-Handed) +2", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (One-Handed) +2", cost_gp: Some(44800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +2, up to +5") }, // up_equipment.lst:146
    EquipmentTableEntry { key: "Crystal Hilt (One-Handed) +3", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (One-Handed) +3", cost_gp: Some(100800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +3, up to +5") }, // up_equipment.lst:147
    EquipmentTableEntry { key: "Crystal Hilt (Two-Handed) +1", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (Two-Handed) +1", cost_gp: Some(11200.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +1, up to +5") }, // up_equipment.lst:148
    EquipmentTableEntry { key: "Crystal Hilt (Two-Handed) +2", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (Two-Handed) +2", cost_gp: Some(44800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +2, up to +5") }, // up_equipment.lst:149
    EquipmentTableEntry { key: "Crystal Hilt (Two-Handed) +3", category: EquipmentCategory::MagicItems, name: "Crystal Hilt (Two-Handed) +3", cost_gp: Some(100800.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +3, up to +5") }, // up_equipment.lst:150
    EquipmentTableEntry { key: "Crystal Spaulders +1", category: EquipmentCategory::MagicItems, name: "Crystal Spaulders +1", cost_gp: Some(5600.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind armor is increased by +1, up to +5") }, // up_equipment.lst:151
    EquipmentTableEntry { key: "Crystal Spaulders +2", category: EquipmentCategory::MagicItems, name: "Crystal Spaulders +2", cost_gp: Some(22400.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind armor is increased by +2, up to +5") }, // up_equipment.lst:152
    EquipmentTableEntry { key: "Crystal Spaulders +3", category: EquipmentCategory::MagicItems, name: "Crystal Spaulders +3", cost_gp: Some(50400.0_f64), weight_lbs: Some(2.0_f64), description: Some("enhancement of mind armor is increased by +3, up to +5") }, // up_equipment.lst:153
    EquipmentTableEntry { key: "Psicrown (Astral Legion)", category: EquipmentCategory::MagicItems, name: "True Psicrown of the [NAME]", cost_gp: Some(122400.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 170; Manifester Level - 17; Powers - Astral Construct; Recharge Cost 17 pp, Gain 17 pp") }, // up_equipment.lst:158
    EquipmentTableEntry { key: "Psicrown (Beast)", category: EquipmentCategory::MagicItems, name: "Lesser Psicrown of the [NAME]", cost_gp: Some(23400.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 30; Manifester Level - 9; Powers - Claw of Energy, Duodimensional Claw, Prevenom, Truevenom; Recharge Cost 9 pp, Gain 3 pp") }, // up_equipment.lst:159
    EquipmentTableEntry { key: "Psicrown (Cautious Warrior)", category: EquipmentCategory::MagicItems, name: "Greater Psicrown of the [NAME]", cost_gp: Some(61600.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 60; Manifester Level - 11; Powers - Adapt Body, Body Adjustment, Precognition (Defensive), Inertial Barrier; Recharge Cost 11 pp, Gain 6 pp") }, // up_equipment.lst:160
    EquipmentTableEntry { key: "Psicrown (Discerning Watcher)", category: EquipmentCategory::MagicItems, name: "Greater Psicrown of the [NAME]", cost_gp: Some(105600.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 60; Manifester Level - 11; Powers - Aura Sight, Clairtangent Hand, Clairvoyant Sense, Heightened Vision, Pierce the Veils, Remote Viewing; Recharge Cost 11 pp, Gain 6 pp") }, // up_equipment.lst:161
    EquipmentTableEntry { key: "Psicrown (Dominator)", category: EquipmentCategory::MagicItems, name: "Greater Psicrown of the [NAME]", cost_gp: Some(39600.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 60; Manifester Level - 11; Powers - Empathic Connection, Mind Control, Compelling Voice; Recharge Cost 11 pp, Gain 6 pp") }, // up_equipment.lst:162
    EquipmentTableEntry { key: "Psicrown (Lesser Dominator)", category: EquipmentCategory::MagicItems, name: "Lesser Psicrown of the Dominator", cost_gp: Some(14400.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 30; Manifester Level - 9; Powers - Empathic Connection, Mind Control, Compelling Voice; Recharge Cost 9 pp, Gain 3 pp") }, // up_equipment.lst:163
    EquipmentTableEntry { key: "Psicrown (Evader)", category: EquipmentCategory::MagicItems, name: "True Psicrown of the [NAME]", cost_gp: Some(145600.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 140; Manifester Level - 14; Powers - Flight, Slip the Bonds, Wall Walker; Recharge Cost 14 pp, Gain 14 pp") }, // up_equipment.lst:164
    EquipmentTableEntry { key: "Psicrown (Fiery Ruin)", category: EquipmentCategory::MagicItems, name: "Lesser Psicrown of [NAME]", cost_gp: Some(57000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 50; Manifester Level - 15; Powers - Energy Ball, Energy Cone, Energy Missile, Energy Stun, Fiery Discorporation; Recharge Cost 15 pp, Gain 5 pp") }, // up_equipment.lst:165
    EquipmentTableEntry { key: "Psicrown (Temporal Juggler)", category: EquipmentCategory::MagicItems, name: "Greater Psicrown of the [NAME]", cost_gp: Some(122400.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 90; Manifester Level - 17; Powers - Temporal Acceleration, Time Hop, Timeless Body; Recharge Cost 17 pp, Gain 9 pp") }, // up_equipment.lst:166
    EquipmentTableEntry { key: "Psicrown (Traveler)", category: EquipmentCategory::MagicItems, name: "Lesser Psicrown of the [NAME]", cost_gp: Some(54000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 50; Manifester Level - 15; Powers - Astral Traveler, Fold Space, Psychoport (Greater), Psychoport; Recharge Cost 15 pp, Gain 5 pp") }, // up_equipment.lst:167
    EquipmentTableEntry { key: "Psicrown (Kinetic Control)", category: EquipmentCategory::MagicItems, name: "True Psicrown of [NAME]", cost_gp: Some(130400.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 150; Manifester Level - 15; Powers - Psychokinetic Sphere, Telekinetic Force, Telekinetic Maneuver; Recharge Cost 15 pp, Gain 15 pp") }, // up_equipment.lst:168
    EquipmentTableEntry { key: "Psicrown (Lesser Force and Fire)", category: EquipmentCategory::MagicItems, name: "Lesser Psicrown of Force and Fire", cost_gp: Some(22000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 60; Manifester Level - 11; Powers - Deflect Missiles, Energy Burst, Energy Push, Force Screen; Recharge Cost 11 pp, Gain 6 pp") }, // up_equipment.lst:169
    EquipmentTableEntry { key: "Psicrown (Force and Fire)", category: EquipmentCategory::MagicItems, name: "True Psicrown of [NAME]", cost_gp: Some(61600.0_f64), weight_lbs: Some(0.0_f64), description: Some("Power Points - 140; Manifester Level - 14; Powers - Deflect Missiles, Energy Burst, Energy Push, Force Screen; Recharge Cost 14 pp, Gain 14 pp") }, // up_equipment.lst:170
    EquipmentTableEntry { key: "Amulet of Catapsi", category: EquipmentCategory::MagicItems, name: "Amulet of Catapsi", cost_gp: Some(16200.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:176
    EquipmentTableEntry { key: "Aura Monocle (Good/Evil)", category: EquipmentCategory::MagicItems, name: "Aura Monocle (Good/Evil)", cost_gp: Some(23520.0_f64), weight_lbs: Some(0.5_f64), description: Some("Discern good/evil alignment 3/day") }, // up_equipment.lst:177
    EquipmentTableEntry { key: "Aura Monocle (Law/Chaos)", category: EquipmentCategory::MagicItems, name: "Aura Monocle (Law/Chaos)", cost_gp: Some(23520.0_f64), weight_lbs: Some(0.5_f64), description: Some("Discern lawful/chaotic alignment 3/day") }, // up_equipment.lst:178
    EquipmentTableEntry { key: "Aura Monocle (Greater)", category: EquipmentCategory::MagicItems, name: "Aura Monocle, Greater", cost_gp: Some(47040.0_f64), weight_lbs: Some(0.5_f64), description: Some("Discern good/evil and lawful/chaotic alignment 3/day") }, // up_equipment.lst:179
    EquipmentTableEntry { key: "Belt of Ectoplasmic Safety", category: EquipmentCategory::MagicItems, name: "Belt of Ectoplasmic Safety", cost_gp: Some(30000.0_f64), weight_lbs: Some(2.0_f64), description: Some("3/day move out of area of effect on successful Reflex save, leaving ectoplasmic copy") }, // up_equipment.lst:183
    EquipmentTableEntry { key: "Boots of Gravity Binding", category: EquipmentCategory::MagicItems, name: "Boots of Gravity Binding", cost_gp: Some(18000.0_f64), weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:184
    EquipmentTableEntry { key: "Boots of Landing", category: EquipmentCategory::MagicItems, name: "Boots of Landing", cost_gp: Some(1000.0_f64), weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:185
    EquipmentTableEntry { key: "Boots of Skating", category: EquipmentCategory::MagicItems, name: "Boots of Skating", cost_gp: Some(7000.0_f64), weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:186
    EquipmentTableEntry { key: "Boots of Stomping", category: EquipmentCategory::MagicItems, name: "Boots of Stomping", cost_gp: Some(600.0_f64), weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:187
    EquipmentTableEntry { key: "Boots of Temporal Acceleration", category: EquipmentCategory::MagicItems, name: "Boots of Temporal Acceleration", cost_gp: Some(43200.0_f64), weight_lbs: Some(1.0_f64), description: Some("1/day temporal acceleration for 2 rounds") }, // up_equipment.lst:188
    EquipmentTableEntry { key: "Boots of the Nomad", category: EquipmentCategory::MagicItems, name: "Boots of the Nomad", cost_gp: Some(10000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Gain Nomad's Step as 4th-level Nomad, or add 4 to Nomad level for Nomad's Step") }, // up_equipment.lst:189
    EquipmentTableEntry { key: "Bracers of Disruption", category: EquipmentCategory::MagicItems, name: "Bracers of Disruption", cost_gp: Some(20000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:190
    EquipmentTableEntry { key: "Bracers of Martial Surging", category: EquipmentCategory::MagicItems, name: "Bracers of Martial Surging", cost_gp: Some(5000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Charge bracers with wild surge; use charges to give weapon enhancement bonus or special ability") }, // up_equipment.lst:191
    EquipmentTableEntry { key: "Cacophonous Bell", category: EquipmentCategory::MagicItems, name: "Cacophonous Bell", cost_gp: Some(10500.0_f64), weight_lbs: Some(3.0_f64), description: Some("Bardic performance more difficult when rung") }, // up_equipment.lst:195
    EquipmentTableEntry { key: "Charm of Perservation", category: EquipmentCategory::MagicItems, name: "Charm of Perservation", cost_gp: Some(250.0_f64), weight_lbs: Some(0.0_f64), description: Some("Allows psionic revivify to be used up to a week later") }, // up_equipment.lst:196
    EquipmentTableEntry { key: "Charms of Friendly Interception", category: EquipmentCategory::MagicItems, name: "Charms of Friendly Interception", cost_gp: Some(12000.0_f64), weight_lbs: Some(0.0_f64), description: Some("switch places with wearer of mated charm (30 ft. range)") }, // up_equipment.lst:198
    EquipmentTableEntry { key: "Circlet of the Sheltered Mind", category: EquipmentCategory::MagicItems, name: "Circlet of the Sheltered Mind", cost_gp: Some(7500.0_f64), weight_lbs: Some(2.0_f64), description: Some("Reduce enervation chance by 5%") }, // up_equipment.lst:199
    EquipmentTableEntry { key: "Coin of Brotherhood", category: EquipmentCategory::MagicItems, name: "Coin of Brotherhood", cost_gp: Some(5000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Gain +10 bonus on Diplomacy") }, // up_equipment.lst:200
    EquipmentTableEntry { key: "Companion Stone (Diplomacy)", category: EquipmentCategory::MagicItems, name: "Companion Stone of [NAME]", cost_gp: Some(1700.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:201
    EquipmentTableEntry { key: "Companion Stone (Electrical Protection)", category: EquipmentCategory::MagicItems, name: "Companion Stone of [NAME]", cost_gp: Some(27200.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:202
    EquipmentTableEntry { key: "Companion Stone (Far Sight)", category: EquipmentCategory::MagicItems, name: "Companion Stone of [NAME]", cost_gp: Some(7500.0_f64), weight_lbs: Some(0.0_f64), description: Some("+2 save vs. illusions") }, // up_equipment.lst:203
    EquipmentTableEntry { key: "Companion Stone (Fire)", category: EquipmentCategory::MagicItems, name: "Companion Stone of [NAME]", cost_gp: Some(17000.0_f64), weight_lbs: Some(0.0_f64), description: Some("energy cone [fire]; 30 charges") }, // up_equipment.lst:204
    EquipmentTableEntry { key: "Companion Stone (Fortify)", category: EquipmentCategory::MagicItems, name: "Companion Stone of [NAME]", cost_gp: Some(9000.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day +4 resistance bonus to saves for 5 minutes") }, // up_equipment.lst:205
    EquipmentTableEntry { key: "Companion Stone (Truthful Dealings)", category: EquipmentCategory::MagicItems, name: "Companion Stone of [NAME]", cost_gp: Some(10800.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day read thoughts for 3 minutes") }, // up_equipment.lst:206
    EquipmentTableEntry { key: "Cowardly Cryptic's Goggles", category: EquipmentCategory::MagicItems, name: "Cowardly Cryptic's Goggles", cost_gp: Some(10000.0_f64), weight_lbs: Some(1.0_f64), description: Some("gain trap spotter insight if wearer has insights; when passing within 10 ft. of a detected trap, goggles black out everything but the trap") }, // up_equipment.lst:207
    EquipmentTableEntry { key: "Creature of Habit's Medallion", category: EquipmentCategory::MagicItems, name: "Creature of Habit's Medallion", cost_gp: Some(2000.0_f64), weight_lbs: Some(0.0_f64), description: Some("when hit, wearer can cause attacker to make the same action next turn (Will DC 11)") }, // up_equipment.lst:208
    EquipmentTableEntry { key: "Crawling Tattoo (Concussion)", category: EquipmentCategory::MagicItems, name: "Crawling Tattoo of Concussion", cost_gp: Some(50.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:209
    EquipmentTableEntry { key: "Crawling Tattoo (Energy Bolt)", category: EquipmentCategory::MagicItems, name: "Crawling Tattoo of Energy Bolt", cost_gp: Some(750.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:210
    EquipmentTableEntry { key: "Crown of Traded Will", category: EquipmentCategory::MagicItems, name: "Crown of Traded Will", cost_gp: Some(20000.0_f64), weight_lbs: Some(2.0_f64), description: Some("1/day swap minds for one round (Will DC 19)") }, // up_equipment.lst:211
    EquipmentTableEntry { key: "Crystal Anchor (Body)", category: EquipmentCategory::MagicItems, name: "Body Crystal Anchor", cost_gp: Some(24000.0_f64), weight_lbs: Some(3.0_f64), description: Some("Those who manifest psychometabolism powers are affected") }, // up_equipment.lst:212
    EquipmentTableEntry { key: "Crystal Anchor (Comprehension)", category: EquipmentCategory::MagicItems, name: "Comprehension Crystal Anchor", cost_gp: Some(24000.0_f64), weight_lbs: Some(3.0_f64), description: Some("Those who manifest clairsentience powers are affected") }, // up_equipment.lst:213
    EquipmentTableEntry { key: "Crystal Anchor (Creation)", category: EquipmentCategory::MagicItems, name: "Creation Crystal Anchor", cost_gp: Some(24000.0_f64), weight_lbs: Some(3.0_f64), description: Some("Those who manifest metacreativity powers are affected") }, // up_equipment.lst:214
    EquipmentTableEntry { key: "Crystal Anchor (Energy)", category: EquipmentCategory::MagicItems, name: "Energy Crystal Anchor", cost_gp: Some(24000.0_f64), weight_lbs: Some(3.0_f64), description: Some("Those who manifest psychokinesis powers are affected") }, // up_equipment.lst:215
    EquipmentTableEntry { key: "Crystal Anchor (Ghost)", category: EquipmentCategory::MagicItems, name: "Ghost Crystal Anchor", cost_gp: Some(24000.0_f64), weight_lbs: Some(3.0_f64), description: Some("Those who manifest remote viewing are affected") }, // up_equipment.lst:216
    EquipmentTableEntry { key: "Crystal Anchor (Mind)", category: EquipmentCategory::MagicItems, name: "Mind Crystal Anchor", cost_gp: Some(24000.0_f64), weight_lbs: Some(3.0_f64), description: Some("Those who manifest telepathy powers are affected") }, // up_equipment.lst:217
    EquipmentTableEntry { key: "Crystal Anchor (Travel)", category: EquipmentCategory::MagicItems, name: "Travel Crystal Anchor", cost_gp: Some(24000.0_f64), weight_lbs: Some(3.0_f64), description: Some("Those who manifest psychoportation powers are affected") }, // up_equipment.lst:218
    EquipmentTableEntry { key: "Crystal Mask (Detection)", category: EquipmentCategory::MagicItems, name: "Crystal Mask of Detection", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:219
    EquipmentTableEntry { key: "Crystal Mask (Discernment)", category: EquipmentCategory::MagicItems, name: "Crystal Mask of Discernment", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:220
    EquipmentTableEntry { key: "Crystal Mask (Dread)", category: EquipmentCategory::MagicItems, name: "Crystal Mask of Dread", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:221
    EquipmentTableEntry { key: "Crystal Mask (Insightful Detection)", category: EquipmentCategory::MagicItems, name: "Crystal Mask of Insightful Detection", cost_gp: Some(20250.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:222
    EquipmentTableEntry { key: "Crystal Mask (Knowledge)", category: EquipmentCategory::MagicItems, name: "Crystal Mask of Knowledge", cost_gp: Some(2500.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:223
    EquipmentTableEntry { key: "Crystal Mask (Languages)", category: EquipmentCategory::MagicItems, name: "Crystal Mask of Languages", cost_gp: Some(2500.0_f64), weight_lbs: Some(0.5_f64), description: Some("Grants the ability to speak and write five different languages") }, // up_equipment.lst:224
    EquipmentTableEntry { key: "Crystal Mask (Mindarmor)", category: EquipmentCategory::MagicItems, name: "Crystal Mask of Mindarmor", cost_gp: Some(10667.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:225
    EquipmentTableEntry { key: "Crystal Mask (Psionic Craft)", category: EquipmentCategory::MagicItems, name: "Crystal Mask of Psionic Craft", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:226
    EquipmentTableEntry { key: "Dissipating Gloves", category: EquipmentCategory::MagicItems, name: "Dissipating Gloves", cost_gp: Some(6000.0_f64), weight_lbs: Some(0.0_f64), description: Some("3/day, melee touch attack deals 3d6 damage") }, // up_equipment.lst:230
    EquipmentTableEntry { key: "Earring of Resistance +1", category: EquipmentCategory::MagicItems, name: "Earring of Resistance +1", cost_gp: Some(700.0_f64), weight_lbs: Some(0.0_f64), description: Some("+1 competence bonus on save against a single psionic power") }, // up_equipment.lst:234
    EquipmentTableEntry { key: "Earring of Resistance +2", category: EquipmentCategory::MagicItems, name: "Earring of Resistance +2", cost_gp: Some(2800.0_f64), weight_lbs: Some(0.0_f64), description: Some("+2 competence bonus on save against a single psionic power") }, // up_equipment.lst:235
    EquipmentTableEntry { key: "Earring of Resistance +3", category: EquipmentCategory::MagicItems, name: "Earring of Resistance +3", cost_gp: Some(6300.0_f64), weight_lbs: Some(0.0_f64), description: Some("+3 competence bonus on save against a single psionic power") }, // up_equipment.lst:236
    EquipmentTableEntry { key: "Earring of Resistance +4", category: EquipmentCategory::MagicItems, name: "Earring of Resistance +4", cost_gp: Some(12200.0_f64), weight_lbs: Some(0.0_f64), description: Some("+4 competence bonus on save against a single psionic power") }, // up_equipment.lst:237
    EquipmentTableEntry { key: "Earring of Resistance +5", category: EquipmentCategory::MagicItems, name: "Earring of Resistance +5", cost_gp: Some(17500.0_f64), weight_lbs: Some(0.0_f64), description: Some("+5 competence bonus on save against a single psionic power") }, // up_equipment.lst:238
    EquipmentTableEntry { key: "Empathic Monocle", category: EquipmentCategory::MagicItems, name: "Empathic Monocle", cost_gp: Some(1750.0_f64), weight_lbs: Some(0.5_f64), description: Some("Expend focus for bonus to Diplomacy and Sense Motive") }, // up_equipment.lst:239
    EquipmentTableEntry { key: "Empathic Monocle (Greater)", category: EquipmentCategory::MagicItems, name: "Empathic Monocle, Greater", cost_gp: Some(8400.0_f64), weight_lbs: Some(0.5_f64), description: Some("Learn surface thoughts of creature (Will DC 12 negates)") }, // up_equipment.lst:240
    EquipmentTableEntry { key: "Eyes of Disarming Glances", category: EquipmentCategory::MagicItems, name: "Eyes of Disarming Glances", cost_gp: Some(20000.0_f64), weight_lbs: Some(0.0_f64), description: Some("2/day disarm with 30ft. range gaze attack") }, // up_equipment.lst:241
    EquipmentTableEntry { key: "Eyes of Expanded Vision", category: EquipmentCategory::MagicItems, name: "Eyes of Expanded Vision", cost_gp: Some(3000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Flanking opponents gain only +1 bonus instead of +2 and -2 penalty on save against gaze attacks.") }, // up_equipment.lst:242
    EquipmentTableEntry { key: "Eyes of Power Leech", category: EquipmentCategory::MagicItems, name: "Eyes of Power Leech", cost_gp: Some(10080.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day power leech for 7 rounds (1d6 pp lost; 1 pp gained)") }, // up_equipment.lst:243
    EquipmentTableEntry { key: "Eyes of Power Leech (Vampiric)", category: EquipmentCategory::MagicItems, name: "Eyes of Power Leech, Vampiric", cost_gp: Some(20160.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day power leech for 13 rounds (1d6 pp lost; 1 pp gained), can go over pp maximum") }, // up_equipment.lst:244
    EquipmentTableEntry { key: "Fear Drinker's Amulet", category: EquipmentCategory::MagicItems, name: "Fear Drinker's Amulet", cost_gp: Some(2500.0_f64), weight_lbs: Some(0.0_f64), description: Some("using devastating touch gives 2 temporary hit points, stacks with self up to dread level") }, // up_equipment.lst:248
    EquipmentTableEntry { key: "Fear Drinker's Amulet (Greater)", category: EquipmentCategory::MagicItems, name: "Fear Drinker's Amulet (Greater)", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.0_f64), description: Some("using devastating touch gives 2 temporary hit points, stacks with self up to twice dread level; if target is frightened,gain 1 temporary power point") }, // up_equipment.lst:249
    EquipmentTableEntry { key: "Gauntlet of the Thunder Shield", category: EquipmentCategory::MagicItems, name: "Gauntlet of the Thunder Shield", cost_gp: Some(8000.0_f64), weight_lbs: Some(1.0_f64), description: Some("gain +4 shield bonus to AC and sonic resistance 10 for 30 rounds per day") }, // up_equipment.lst:253
    EquipmentTableEntry { key: "Gladiator's Gauze", category: EquipmentCategory::MagicItems, name: "Gladiator's Gauze", cost_gp: Some(35.0_f64), weight_lbs: Some(0.5_f64), description: Some("Stabilizes and stops bleed effects") }, // up_equipment.lst:254
    EquipmentTableEntry { key: "Glove of Calling", category: EquipmentCategory::MagicItems, name: "Glove of Calling", cost_gp: Some(4000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Call a weapon as a swift action.") }, // up_equipment.lst:255
    EquipmentTableEntry { key: "Gloves of Object Reading", category: EquipmentCategory::MagicItems, name: "Gloves of Object Reading", cost_gp: Some(3000.0_f64), weight_lbs: Some(0.0_f64), description: Some("object reading while handling an item") }, // up_equipment.lst:256
    EquipmentTableEntry { key: "Gloves of Titan's Grip", category: EquipmentCategory::MagicItems, name: "Gloves of Titan's Grip", cost_gp: Some(14000.0_f64), weight_lbs: Some(0.0_f64), description: Some("3/day +8 enhancement bonus on grapple checks for 7 rounds") }, // up_equipment.lst:257
    EquipmentTableEntry { key: "Gloves of the Beast (1d3)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (1d3)", cost_gp: Some(625.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:258
    EquipmentTableEntry { key: "Gloves of the Beast (1d4)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (1d4)", cost_gp: Some(1000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:259
    EquipmentTableEntry { key: "Gloves of the Beast (1d6)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (1d6)", cost_gp: Some(3000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:260
    EquipmentTableEntry { key: "Gloves of the Beast (1d8)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (1d8)", cost_gp: Some(5000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:261
    EquipmentTableEntry { key: "Gloves of the Beast (2d6)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (2d6)", cost_gp: Some(7000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:262
    EquipmentTableEntry { key: "Gloves of the Beast (3d6)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (3d6)", cost_gp: Some(11000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:263
    EquipmentTableEntry { key: "Gloves of the Beast (4d6)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (4d6)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:264
    EquipmentTableEntry { key: "Gloves of the Beast (5d6)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (5d6)", cost_gp: Some(19000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:265
    EquipmentTableEntry { key: "Gloves of the Beast (6d6)", category: EquipmentCategory::MagicItems, name: "Gloves of the Beast (6d6)", cost_gp: Some(22000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:266
    EquipmentTableEntry { key: "Boots of the Beast (1d3)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (1d3)", cost_gp: Some(625.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:267
    EquipmentTableEntry { key: "Boots of the Beast (1d4)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (1d4)", cost_gp: Some(1000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:268
    EquipmentTableEntry { key: "Boots of the Beast (1d6)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (1d6)", cost_gp: Some(3000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:269
    EquipmentTableEntry { key: "Boots of the Beast (1d8)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (1d8)", cost_gp: Some(5000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:270
    EquipmentTableEntry { key: "Boots of the Beast (2d6)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (2d6)", cost_gp: Some(7000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:271
    EquipmentTableEntry { key: "Boots of the Beast (3d6)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (3d6)", cost_gp: Some(11000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:272
    EquipmentTableEntry { key: "Boots of the Beast (4d6)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (4d6)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:273
    EquipmentTableEntry { key: "Boots of the Beast (5d6)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (5d6)", cost_gp: Some(19000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:274
    EquipmentTableEntry { key: "Boots of the Beast (6d6)", category: EquipmentCategory::MagicItems, name: "Boots of the Beast (6d6)", cost_gp: Some(22000.0_f64), weight_lbs: Some(0.5_f64), description: None }, // up_equipment.lst:275
    EquipmentTableEntry { key: "Goggles of Far Sight", category: EquipmentCategory::MagicItems, name: "Goggles of Far Sight", cost_gp: Some(4000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Ignore first ranged increment penalty") }, // up_equipment.lst:276
    EquipmentTableEntry { key: "Headband of the Great Village (1 creature)", category: EquipmentCategory::MagicItems, name: "Headband of the Great Village (1 creature)", cost_gp: Some(3000.0_f64), weight_lbs: Some(0.5_f64), description: Some("Mindlink with 1 creature within 30 ft.; Add one to size of collective|PREMULT:1,[PREVARGT:TacticianCollectiveMinds,0],[PREVARGT:CollectiveMinds,0]") }, // up_equipment.lst:281
    EquipmentTableEntry { key: "Headband of the Great Village (2 creatures)", category: EquipmentCategory::MagicItems, name: "Headband of the Great Village (2 creatures)", cost_gp: Some(12000.0_f64), weight_lbs: Some(0.5_f64), description: Some("Mindlink with 2 creatures within 30 ft.; Add two to size of collective|PREMULT:1,[PREVARGT:TacticianCollectiveMinds,0],[PREVARGT:CollectiveMinds,0]") }, // up_equipment.lst:282
    EquipmentTableEntry { key: "Headband of the Great Village (3 creatures)", category: EquipmentCategory::MagicItems, name: "Headband of the Great Village (3 creatures)", cost_gp: Some(27000.0_f64), weight_lbs: Some(0.5_f64), description: Some("Mindlink with 3 creatures within 30 ft.; Add three to size of collective|PREMULT:1,[PREVARGT:TacticianCollectiveMinds,0],[PREVARGT:CollectiveMinds,0]") }, // up_equipment.lst:283
    EquipmentTableEntry { key: "Helm of Attitude Adjustment", category: EquipmentCategory::MagicItems, name: "Helm of Attitude Adjustment", cost_gp: Some(18000.0_f64), weight_lbs: Some(2.0_f64), description: Some("2/day shift attitude by one step (Will DC 14 negates); +4 bonus on Bluff, Diplomacy, Intimidate to change attitude") }, // up_equipment.lst:284
    EquipmentTableEntry { key: "Ioun Stone (Dull Grey)", category: EquipmentCategory::MagicItems, name: "Ioun Stone, [NAME]", cost_gp: Some(25.0_f64), weight_lbs: Some(0.02_f64), description: Some("Provides 1 psionic power point then disintegrates.") }, // up_equipment.lst:289
    EquipmentTableEntry { key: "Ioun Stone (Green and White)", category: EquipmentCategory::MagicItems, name: "Ioun Stone, [NAME]", cost_gp: Some(12000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Grants an additional power known.") }, // up_equipment.lst:291
    EquipmentTableEntry { key: "Ioun Stone (Rainbow)", category: EquipmentCategory::MagicItems, name: "Ioun Stone, [NAME]", cost_gp: Some(16000.0_f64), weight_lbs: Some(0.02_f64), description: Some("Provides 5 psionic power points per day (regenerates)") }, // up_equipment.lst:292
    EquipmentTableEntry { key: "Lava Walker's Boots", category: EquipmentCategory::MagicItems, name: "Lava Walker's Boots", cost_gp: Some(12000.0_f64), weight_lbs: Some(1.0_f64), description: Some("walk on liquid surfaces and fire resistance 10 for 10 rounds/day") }, // up_equipment.lst:296
    EquipmentTableEntry { key: "Mantle of the Void", category: EquipmentCategory::MagicItems, name: "Mantle of the Void", cost_gp: Some(18480.0_f64), weight_lbs: Some(0.0_f64), description: Some("gain +6 insight bonus to one d20 roll in the next minute; take 1d3 Wisdom burn if used more than once per week") }, // up_equipment.lst:300
    EquipmentTableEntry { key: "Map of the Mind", category: EquipmentCategory::MagicItems, name: "Map of the Mind", cost_gp: Some(16000.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/week psychic reformation") }, // up_equipment.lst:301
    EquipmentTableEntry { key: "Mask of Confounded Foes", category: EquipmentCategory::MagicItems, name: "Mask of Confounded Foes", cost_gp: Some(12000.0_f64), weight_lbs: Some(0.5_f64), description: Some("1/day force once creature to redirect attack (Will DC 16 negates)") }, // up_equipment.lst:302
    EquipmentTableEntry { key: "Meld Stone (Alchemist)", category: EquipmentCategory::MagicItems, name: "Meld Stone of the [NAME]", cost_gp: Some(3040.0_f64), weight_lbs: Some(0.0_f64), description: Some("+8 to Craft (Alchemy), +6 to Appraise and Use Magic Device, +4 to Knowledge (Arcana), if set in a Synaptic Mask") }, // up_equipment.lst:303
    EquipmentTableEntry { key: "Meld Stone (Inflitrator)", category: EquipmentCategory::MagicItems, name: "Meld Stone of the [NAME]", cost_gp: Some(3040.0_f64), weight_lbs: Some(0.0_f64), description: Some("+8 to Bluff, +6 to Disguise and Forgery, +4 to Gather Information, if set in a Synaptic Mask") }, // up_equipment.lst:304
    EquipmentTableEntry { key: "Meld Stone (Nimble Trickster)", category: EquipmentCategory::MagicItems, name: "Meld Stone of the [NAME]", cost_gp: Some(3040.0_f64), weight_lbs: Some(0.0_f64), description: Some("+8 to Tumble, +6 to Escape Artist and Stealth") }, // up_equipment.lst:305
    EquipmentTableEntry { key: "Mender's Vestments (extra time)", category: EquipmentCategory::MagicItems, name: "Mender's Vestments (extra time)", cost_gp: Some(3750.0_f64), weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:306
    EquipmentTableEntry { key: "Mender's Vestments (extra 1d6)", category: EquipmentCategory::MagicItems, name: "Mender's Vestments (extra 1d6)", cost_gp: Some(4000.0_f64), weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:307
    EquipmentTableEntry { key: "Mender's Vestments (extra time and 1d6)", category: EquipmentCategory::MagicItems, name: "Mender's Vestments (extra time and 1d6)", cost_gp: Some(10000.0_f64), weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:308
    EquipmentTableEntry { key: "Mind Sharing Circlets", category: EquipmentCategory::MagicItems, name: "Mind Sharing Circlets", cost_gp: Some(42000.0_f64), weight_lbs: Some(1.0_f64), description: Some("manifest powers known by other wearer for 6 extra pp") }, // up_equipment.lst:309
    EquipmentTableEntry { key: "Mirror of Lost Recollections", category: EquipmentCategory::MagicItems, name: "Mirror of Lost Recollections", cost_gp: Some(150000.0_f64), weight_lbs: Some(0.5_f64), description: Some("deal 8d6 damage (Will DC 16 half) to creature within 30 ft.; 1/day kill target (Will DC 22 for 5d6 damage)") }, // up_equipment.lst:310
    EquipmentTableEntry { key: "Mirror of Mind Switch", category: EquipmentCategory::MagicItems, name: "Mirror of Mind Switch", cost_gp: Some(19800.0_f64), weight_lbs: Some(0.5_f64), description: Some("1/day mind switch as gaze attack") }, // up_equipment.lst:311
    EquipmentTableEntry { key: "Mirror of Suggestion", category: EquipmentCategory::MagicItems, name: "Mirror of Suggestion", cost_gp: Some(3600.0_f64), weight_lbs: Some(0.5_f64), description: Some("2/day suggestion as gaze attack") }, // up_equipment.lst:312
    EquipmentTableEntry { key: "Mirror of Time Hop", category: EquipmentCategory::MagicItems, name: "Mirror of Time Hop", cost_gp: Some(9000.0_f64), weight_lbs: Some(0.5_f64), description: Some("2/day time hop as gaze attack") }, // up_equipment.lst:313
    EquipmentTableEntry { key: "Pattern Breaker's Gloves", category: EquipmentCategory::MagicItems, name: "Pattern Breaker's Gloves", cost_gp: Some(2000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Use disrupt pattern with unarmed strikes or natural attacks") }, // up_equipment.lst:317
    EquipmentTableEntry { key: "Pearl (Brain Lock)", category: EquipmentCategory::MagicItems, name: "Pearl, Brain Lock", cost_gp: Some(300.0_f64), weight_lbs: Some(0.0_f64), description: Some("DC 13 Will") }, // up_equipment.lst:318
    EquipmentTableEntry { key: "Pearl (Breath Crisis)", category: EquipmentCategory::MagicItems, name: "Pearl, Breath Crisis", cost_gp: Some(750.0_f64), weight_lbs: Some(0.0_f64), description: Some("DC 14 Will") }, // up_equipment.lst:319
    EquipmentTableEntry { key: "Pearl (Mind Seed)", category: EquipmentCategory::MagicItems, name: "Pearl, Mind Seed", cost_gp: Some(18500.0_f64), weight_lbs: Some(0.0_f64), description: Some("DC 22 Will") }, // up_equipment.lst:320
    EquipmentTableEntry { key: "Pearl (Personality Parasite)", category: EquipmentCategory::MagicItems, name: "Pearl, Personality Parasite", cost_gp: Some(1400.0_f64), weight_lbs: Some(0.0_f64), description: Some("DC 16 Will") }, // up_equipment.lst:321
    EquipmentTableEntry { key: "Psicrystal Crown", category: EquipmentCategory::MagicItems, name: "Psicrystal Crown", cost_gp: Some(2000.0_f64), weight_lbs: Some(0.5_f64), description: Some("as psicrystal staff; expend psionic focus to gain psicrystal's sighted ability for one round") }, // up_equipment.lst:322
    EquipmentTableEntry { key: "Psicrystal Harness", category: EquipmentCategory::MagicItems, name: "Psicrystal Harness", cost_gp: Some(2000.0_f64), weight_lbs: Some(0.5_f64), description: Some("as psicrystal staff but no companion stones; expend psionic focus to gain psicrystal's natural armor adjustment as DR, and sonic vulnerability, until next turn") }, // up_equipment.lst:323
    EquipmentTableEntry { key: "Psicrystal Staff", category: EquipmentCategory::MagicItems, name: "Psicrystal Staff", cost_gp: Some(2000.0_f64), weight_lbs: Some(5.0_f64), description: Some("Can hold psicrystal, 3 setting stones, 10 companion stones") }, // up_equipment.lst:324
    EquipmentTableEntry { key: "Psicrystal Staff (Greater)", category: EquipmentCategory::ArmsArmor, name: "Psicrystal Staff, Greater", cost_gp: Some(10000.0_f64), weight_lbs: Some(5.0_f64), description: Some("Can hold psicrystal, 5 setting stones, 10 companion stones; psicrystal repairs 1 hp/minute; double personality bonuses [not implemented]") }, // up_equipment.lst:325
    EquipmentTableEntry { key: "Psionatrix (Clairsentience)", category: EquipmentCategory::MagicItems, name: "Psionatrix of [NAME]", cost_gp: Some(8000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:326
    EquipmentTableEntry { key: "Psionatrix (Metacreativity)", category: EquipmentCategory::MagicItems, name: "Psionatrix of [NAME]", cost_gp: Some(8000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:327
    EquipmentTableEntry { key: "Psionatrix (Psychokinesis)", category: EquipmentCategory::MagicItems, name: "Psionatrix of [NAME]", cost_gp: Some(8000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:328
    EquipmentTableEntry { key: "Psionatrix (Psychometabolism)", category: EquipmentCategory::MagicItems, name: "Psionatrix of [NAME]", cost_gp: Some(8000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:329
    EquipmentTableEntry { key: "Psionatrix (Psychoportation)", category: EquipmentCategory::MagicItems, name: "Psionatrix of [NAME]", cost_gp: Some(8000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:330
    EquipmentTableEntry { key: "Psionatrix (Telepathy)", category: EquipmentCategory::MagicItems, name: "Psionatrix of [NAME]", cost_gp: Some(8000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:331
    EquipmentTableEntry { key: "Psionic Restraint (Lesser)", category: EquipmentCategory::MagicItems, name: "Psionic Restraint, [NAME]", cost_gp: Some(1000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Wearer can use at most 5PP/round") }, // up_equipment.lst:332
    EquipmentTableEntry { key: "Psionic Restraint (Average)", category: EquipmentCategory::MagicItems, name: "Psionic Restraint, [NAME]", cost_gp: Some(6000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Wearer can use at most 3PP/round") }, // up_equipment.lst:333
    EquipmentTableEntry { key: "Psionic Restraint (Greater)", category: EquipmentCategory::MagicItems, name: "Psionic Restraint, [NAME]", cost_gp: Some(12000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Wearer can use at most 1PP/round") }, // up_equipment.lst:334
    EquipmentTableEntry { key: "Psionic Restraint (Damping)", category: EquipmentCategory::MagicItems, name: "Psionic Restraint, [NAME]", cost_gp: Some(24000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Wearer can use no PP") }, // up_equipment.lst:335
    EquipmentTableEntry { key: "Psychoactive Skin (Chameleon)", category: EquipmentCategory::MagicItems, name: "Skin of the [NAME]", cost_gp: Some(18000.0_f64), weight_lbs: Some(2.0_f64), description: None }, // up_equipment.lst:336
    EquipmentTableEntry { key: "Psychoactive Skin (Claw)", category: EquipmentCategory::MagicItems, name: "Skin of the [NAME]", cost_gp: Some(16000.0_f64), weight_lbs: Some(2.0_f64), description: Some("Claws of the Beast at will, as free action.  Only for Psychic Warrior.") }, // up_equipment.lst:337
    EquipmentTableEntry { key: "Psychoactive Skin (Defender)", category: EquipmentCategory::MagicItems, name: "Skin of the [NAME]", cost_gp: Some(32000.0_f64), weight_lbs: Some(2.0_f64), description: None }, // up_equipment.lst:338
    EquipmentTableEntry { key: "Psychoactive Skin (Fiery Response)", category: EquipmentCategory::MagicItems, name: "Skin of [NAME]", cost_gp: Some(60000.0_f64), weight_lbs: Some(2.0_f64), description: Some("1/round Energy Retort [fire]") }, // up_equipment.lst:339
    EquipmentTableEntry { key: "Psychoactive Skin (Hero)", category: EquipmentCategory::MagicItems, name: "Skin of the [NAME]", cost_gp: Some(77500.0_f64), weight_lbs: Some(2.0_f64), description: None }, // up_equipment.lst:340
    EquipmentTableEntry { key: "Psychoactive Skin (Iron)", category: EquipmentCategory::MagicItems, name: "Skin of [NAME]", cost_gp: Some(129600.0_f64), weight_lbs: Some(2.0_f64), description: Some("3/day Psionic Iron Body for 15 minutes") }, // up_equipment.lst:341
    EquipmentTableEntry { key: "Psychoactive Skin (Nimbleness)", category: EquipmentCategory::MagicItems, name: "Skin of [NAME]", cost_gp: Some(10000.0_f64), weight_lbs: Some(2.0_f64), description: None }, // up_equipment.lst:342
    EquipmentTableEntry { key: "Psychoactive Skin (Proteus)", category: EquipmentCategory::MagicItems, name: "Skin of [NAME]", cost_gp: Some(84000.0_f64), weight_lbs: Some(2.0_f64), description: Some("continuous Metamorphosis") }, // up_equipment.lst:343
    EquipmentTableEntry { key: "Psychoactive Skin (Psion)", category: EquipmentCategory::MagicItems, name: "Skin of the [NAME]", cost_gp: Some(151000.0_f64), weight_lbs: Some(2.0_f64), description: Some("Power Resistance 21") }, // up_equipment.lst:344
    EquipmentTableEntry { key: "Psychoactive Skin (Spider)", category: EquipmentCategory::MagicItems, name: "Skin of the [NAME]", cost_gp: Some(79080.0_f64), weight_lbs: Some(2.0_f64), description: Some("continual Body Equilibrium; Entangling Ectoplasm 3/day, 30 ft.") }, // up_equipment.lst:345
    EquipmentTableEntry { key: "Psychoactive Skin (Troll)", category: EquipmentCategory::MagicItems, name: "Skin of the [NAME]", cost_gp: Some(61200.0_f64), weight_lbs: Some(2.0_f64), description: Some("continual True Metabolism except heal 5 HP/minute.") }, // up_equipment.lst:346
    EquipmentTableEntry { key: "Quiver of Recall", category: EquipmentCategory::MagicItems, name: "Quiver of Recall", cost_gp: Some(2000.0_f64), weight_lbs: Some(2.0_f64), description: Some("holds 60 units of ammunition; misses return to quiver (all if marksman using wind reader; surviving otherwise)") }, // up_equipment.lst:350
    EquipmentTableEntry { key: "Rug of Object Sliding (500 lbs.)", category: EquipmentCategory::MagicItems, name: "Rug of Object Sliding (500 lbs.)", cost_gp: Some(2000.0_f64), weight_lbs: Some(0.5_f64), description: Some("slides under object; only 1/10 of weight for dragging") }, // up_equipment.lst:354
    EquipmentTableEntry { key: "Rug of Object Sliding (2500 lbs.)", category: EquipmentCategory::MagicItems, name: "Rug of Object Sliding (2500 lbs.)", cost_gp: Some(5000.0_f64), weight_lbs: Some(0.5_f64), description: Some("slides under object; only 1/10 of weight for dragging") }, // up_equipment.lst:355
    EquipmentTableEntry { key: "Rug of Object Sliding (5000 lbs.)", category: EquipmentCategory::MagicItems, name: "Rug of Object Sliding (5000 lbs.)", cost_gp: Some(7400.0_f64), weight_lbs: Some(0.5_f64), description: Some("slides under object; only 1/10 of weight for dragging") }, // up_equipment.lst:356
    EquipmentTableEntry { key: "Rug of Object Sliding (10000 lbs.)", category: EquipmentCategory::MagicItems, name: "Rug of Object Sliding (10000 lbs.)", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.5_f64), description: Some("slides under object; only 1/10 of weight for dragging") }, // up_equipment.lst:357
    EquipmentTableEntry { key: "Setting Stone (Invigoration)", category: EquipmentCategory::MagicItems, name: "Setting Stone of [NAME]", cost_gp: Some(10080.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day invigorate self for 10 minutes") }, // up_equipment.lst:361
    EquipmentTableEntry { key: "Setting Stone (Kenosis)", category: EquipmentCategory::MagicItems, name: "Setting Stone of [NAME]", cost_gp: Some(29000.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day ranged touch to suppress psi-like, spell-like, and supernatural abilities for 1d4 rounds") }, // up_equipment.lst:362
    EquipmentTableEntry { key: "Setting Stone (Power Echo (3rd))", category: EquipmentCategory::MagicItems, name: "Setting Stone of [NAME]", cost_gp: Some(9000.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day manifest 3rd or lower level power manifested earlier that day as free manifestation") }, // up_equipment.lst:363
    EquipmentTableEntry { key: "Setting Stone (Power Echo (6th))", category: EquipmentCategory::MagicItems, name: "Setting Stone of [NAME]", cost_gp: Some(36000.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day manifest 6th or lower level power manifested earlier that day as free manifestation") }, // up_equipment.lst:364
    EquipmentTableEntry { key: "Setting Stone (Power Echo (9th))", category: EquipmentCategory::MagicItems, name: "Setting Stone of [NAME]", cost_gp: Some(81000.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day manifest 9th or lower level power manifested earlier that day as free manifestation") }, // up_equipment.lst:365
    EquipmentTableEntry { key: "Setting Stone (Reconstruction)", category: EquipmentCategory::MagicItems, name: "Setting Stone of [NAME]", cost_gp: Some(2500.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day psicrystal heals 5 hit points per round, up to wielder's manifester level") }, // up_equipment.lst:366
    EquipmentTableEntry { key: "Shard +1", category: EquipmentCategory::MagicItems, name: "Shard +1", cost_gp: Some(10.0_f64), weight_lbs: Some(0.0_f64), description: Some("+1 competence bonus on one specific skill roll.") }, // up_equipment.lst:367
    EquipmentTableEntry { key: "Shard +2", category: EquipmentCategory::MagicItems, name: "Shard +2", cost_gp: Some(40.0_f64), weight_lbs: Some(0.0_f64), description: Some("+2 competence bonus on one specific skill roll.") }, // up_equipment.lst:368
    EquipmentTableEntry { key: "Shard +3", category: EquipmentCategory::MagicItems, name: "Shard +3", cost_gp: Some(90.0_f64), weight_lbs: Some(0.0_f64), description: Some("+3 competence bonus on one specific skill roll.") }, // up_equipment.lst:369
    EquipmentTableEntry { key: "Shard +4", category: EquipmentCategory::MagicItems, name: "Shard +4", cost_gp: Some(160.0_f64), weight_lbs: Some(0.0_f64), description: Some("+4 competence bonus on one specific skill roll.") }, // up_equipment.lst:370
    EquipmentTableEntry { key: "Shard +5", category: EquipmentCategory::MagicItems, name: "Shard +5", cost_gp: Some(250.0_f64), weight_lbs: Some(0.0_f64), description: Some("+5 competence bonus on one specific skill roll.") }, // up_equipment.lst:371
    EquipmentTableEntry { key: "Shard +6", category: EquipmentCategory::MagicItems, name: "Shard +6", cost_gp: Some(360.0_f64), weight_lbs: Some(0.0_f64), description: Some("+6 competence bonus on one specific skill roll.") }, // up_equipment.lst:372
    EquipmentTableEntry { key: "Shard +7", category: EquipmentCategory::MagicItems, name: "Shard +7", cost_gp: Some(490.0_f64), weight_lbs: Some(0.0_f64), description: Some("+7 competence bonus on one specific skill roll.") }, // up_equipment.lst:373
    EquipmentTableEntry { key: "Shard +8", category: EquipmentCategory::MagicItems, name: "Shard +8", cost_gp: Some(640.0_f64), weight_lbs: Some(0.0_f64), description: Some("+8 competence bonus on one specific skill roll.") }, // up_equipment.lst:374
    EquipmentTableEntry { key: "Shard +9", category: EquipmentCategory::MagicItems, name: "Shard +9", cost_gp: Some(810.0_f64), weight_lbs: Some(0.0_f64), description: Some("+9 competence bonus on one specific skill roll.") }, // up_equipment.lst:375
    EquipmentTableEntry { key: "Shard +10", category: EquipmentCategory::MagicItems, name: "Shard +10", cost_gp: Some(1000.0_f64), weight_lbs: Some(0.0_f64), description: Some("+10 competence bonus on one specific skill roll.") }, // up_equipment.lst:376
    EquipmentTableEntry { key: "Shimmering Vest", category: EquipmentCategory::MagicItems, name: "Shimmering Vest", cost_gp: Some(12500.0_f64), weight_lbs: Some(0.5_f64), description: Some("Gives or enhances light-bending pattern insight") }, // up_equipment.lst:377
    EquipmentTableEntry { key: "Shimmering Vest (Greater)", category: EquipmentCategory::MagicItems, name: "Shimmering Vest, Greater", cost_gp: Some(25000.0_f64), weight_lbs: Some(0.5_f64), description: Some("Gives or enhances light-bending pattern insight") }, // up_equipment.lst:378
    EquipmentTableEntry { key: "Sliver Prison", category: EquipmentCategory::MagicItems, name: "Sliver Prison", cost_gp: Some(5000.0_f64), weight_lbs: Some(1.0_f64), description: Some("capture psicrystal; gain +2 on DC of Will saves against master") }, // up_equipment.lst:379
    EquipmentTableEntry { key: "Snatching Gloves", category: EquipmentCategory::MagicItems, name: "Snatching Gloves", cost_gp: Some(20000.0_f64), weight_lbs: Some(1.0_f64), description: Some("1/day teleport item to hand") }, // up_equipment.lst:380
    EquipmentTableEntry { key: "Student's Robes", category: EquipmentCategory::MagicItems, name: "Student's Robes", cost_gp: Some(13000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Psion level treated as five higher for discipline abilities.") }, // up_equipment.lst:381
    EquipmentTableEntry { key: "Suffocating Collar", category: EquipmentCategory::MagicItems, name: "Suffocating Collar", cost_gp: Some(30000.0_f64), weight_lbs: Some(1.0_f64), description: Some("1/day use crisis of breath on all creatures within 20 ft. (Will DC 17); chokes wearer for duration of effect") }, // up_equipment.lst:382
    EquipmentTableEntry { key: "Surge Crystal +1", category: EquipmentCategory::MagicItems, name: "Surge Crystal +1", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.5_f64), description: Some("permanent 2 Con drain; gain wild surge as free surge; take bleed damage to use surge blast; bonus to wilder's wild surge") }, // up_equipment.lst:383
    EquipmentTableEntry { key: "Surge Crystal +2", category: EquipmentCategory::MagicItems, name: "Surge Crystal +2", cost_gp: Some(30000.0_f64), weight_lbs: Some(0.5_f64), description: Some("permanent 2 Con drain; gain wild surge as free surge; take bleed damage to use surge blast; bonus to wilder's wild surge") }, // up_equipment.lst:384
    EquipmentTableEntry { key: "Surge Crystal +3", category: EquipmentCategory::MagicItems, name: "Surge Crystal +3", cost_gp: Some(60000.0_f64), weight_lbs: Some(0.5_f64), description: Some("permanent 2 Con drain; gain wild surge as free surge; take bleed damage to use surge blast; bonus to wilder's wild surge") }, // up_equipment.lst:385
    EquipmentTableEntry { key: "Survivor's Sleeping Bag", category: EquipmentCategory::MagicItems, name: "Survivor's Sleeping Bag", cost_gp: Some(1400.0_f64), weight_lbs: Some(3.0_f64), description: Some("No harm from hot or cold environments") }, // up_equipment.lst:386
    EquipmentTableEntry { key: "Sycophant's Ring", category: EquipmentCategory::MagicItems, name: "Sycophant's Ring", cost_gp: Some(1000.0_f64), weight_lbs: Some(0.0_f64), description: Some("wearer does not count against collective limit; must take worse of two rolls vs. charm effects and begin forcibly added to a collective") }, // up_equipment.lst:387
    EquipmentTableEntry { key: "Synaptic Mask", category: EquipmentCategory::MagicItems, name: "Synaptic Mask", cost_gp: Some(9000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Can use embedded shard without losing shard") }, // up_equipment.lst:388
    EquipmentTableEntry { key: "Synaptic Shard (Medic)", category: EquipmentCategory::MagicItems, name: "Synaptic Shard (Medic)", cost_gp: Some(2000.0_f64), weight_lbs: Some(0.0_f64), description: Some("can take 10 on Heal checks even in combat; must be in synaptic mask") }, // up_equipment.lst:389
    EquipmentTableEntry { key: "Synaptic Shard (Precision)", category: EquipmentCategory::MagicItems, name: "Synaptic Shard (Precision)", cost_gp: Some(16000.0_f64), weight_lbs: Some(0.0_f64), description: Some("add 1d6 precision damage when flanking; must be in synaptic mask") }, // up_equipment.lst:390
    EquipmentTableEntry { key: "Synaptic Shard (Shifting Steps)", category: EquipmentCategory::MagicItems, name: "Synaptic Shard (Shifting Steps)", cost_gp: Some(9000.0_f64), weight_lbs: Some(0.0_f64), description: Some("use move action for a 5 foot step; must be in synaptic mask") }, // up_equipment.lst:391
    EquipmentTableEntry { key: "Synaptic Shard (Guarded Flank)", category: EquipmentCategory::MagicItems, name: "Synaptic Shard (Guarded Flank)", cost_gp: Some(16000.0_f64), weight_lbs: Some(0.0_f64), description: Some("+2 AC vs. flankers; must be in synaptic mask") }, // up_equipment.lst:392
    EquipmentTableEntry { key: "Tactician's Chessboard", category: EquipmentCategory::MagicItems, name: "Tactician's Chessboard", cost_gp: Some(8000.0_f64), weight_lbs: Some(10.0_f64), description: Some("shows collective members and what they see; can aid another through the board") }, // up_equipment.lst:396
    EquipmentTableEntry { key: "Third Eye (Aware)", category: EquipmentCategory::MagicItems, name: "Third Eye (Aware)", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:397
    EquipmentTableEntry { key: "Third Eye (Conceal)", category: EquipmentCategory::MagicItems, name: "Third Eye (Conceal)", cost_gp: Some(120000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Continuous Psionic Mind Blank") }, // up_equipment.lst:398
    EquipmentTableEntry { key: "Third Eye (Concentrate)", category: EquipmentCategory::MagicItems, name: "Third Eye (Concentrate)", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.0_f64), description: Some("+10 competence bonus to concentrate checks") }, // up_equipment.lst:399
    EquipmentTableEntry { key: "Third Eye (Dominate)", category: EquipmentCategory::MagicItems, name: "Third Eye (Dominate)", cost_gp: Some(120000.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day Mind Control, DC 18") }, // up_equipment.lst:400
    EquipmentTableEntry { key: "Third Eye (Energy Ray)", category: EquipmentCategory::MagicItems, name: "Third Eye (Energy Ray)", cost_gp: Some(7200.0_f64), weight_lbs: Some(0.0_f64), description: Some("3/day 30-ft. ray of active energy type, 5d6 damage (5d3 if sonic)") }, // up_equipment.lst:401
    EquipmentTableEntry { key: "Third Eye (Expose)", category: EquipmentCategory::MagicItems, name: "Third Eye (Expose)", cost_gp: Some(112000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Know when someone lies directly to you") }, // up_equipment.lst:402
    EquipmentTableEntry { key: "Third Eye (Gather)", category: EquipmentCategory::MagicItems, name: "Third Eye (Gather)", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:403
    EquipmentTableEntry { key: "Third Eye (Penetrate)", category: EquipmentCategory::MagicItems, name: "Third Eye (Penetrate)", cost_gp: Some(8000.0_f64), weight_lbs: Some(0.0_f64), description: Some("+2 to overcome power resistance") }, // up_equipment.lst:404
    EquipmentTableEntry { key: "Third Eye (Powerthieve)", category: EquipmentCategory::MagicItems, name: "Third Eye (Powerthieve)", cost_gp: Some(10080.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day steal power from target within 40 ft., lasts for 70 minutes") }, // up_equipment.lst:405
    EquipmentTableEntry { key: "Third Eye (Repudiate)", category: EquipmentCategory::MagicItems, name: "Third Eye (Repudiate)", cost_gp: Some(43200.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day Dispel Psionics at +20 modifier") }, // up_equipment.lst:406
    EquipmentTableEntry { key: "Third Eye (Sense)", category: EquipmentCategory::MagicItems, name: "Third Eye (Sense)", cost_gp: Some(24000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Clairvoyant Sense at will") }, // up_equipment.lst:407
    EquipmentTableEntry { key: "Third Eye (Talented)", category: EquipmentCategory::MagicItems, name: "Third Eye (Talented)", cost_gp: Some(10180.0_f64), weight_lbs: Some(0.0_f64), description: Some("grants user psionic talent, and Wild Talent or Psionic Talent feat") }, // up_equipment.lst:409
    EquipmentTableEntry { key: "Third Eye (View)", category: EquipmentCategory::MagicItems, name: "Third Eye (View)", cost_gp: Some(10180.0_f64), weight_lbs: Some(0.0_f64), description: Some("1/day Remote Viewing") }, // up_equipment.lst:410
    EquipmentTableEntry { key: "Torc of Free Will", category: EquipmentCategory::MagicItems, name: "Torc of Free Will", cost_gp: Some(6000.0_f64), weight_lbs: Some(2.0_f64), description: Some("not affected by Brain Lock power or items.") }, // up_equipment.lst:411
    EquipmentTableEntry { key: "Torc of Interrogation", category: EquipmentCategory::MagicItems, name: "Torc of Interrogation", cost_gp: Some(0.0_f64), weight_lbs: Some(2.0_f64), description: Some("take 2d6 nonlethal damage when lying (Will DC 14); cumulative -1 penalty per save attempted") }, // up_equipment.lst:413
    EquipmentTableEntry { key: "Headband of Interrogation", category: EquipmentCategory::MagicItems, name: "Headband of Interrogation", cost_gp: Some(20000.0_f64), weight_lbs: Some(0.0_f64), description: Some("get +10 to Intimidate and Sense Motive against wearer of torc") }, // up_equipment.lst:414
    EquipmentTableEntry { key: "Torc of Leech Freedom", category: EquipmentCategory::MagicItems, name: "Torc of Leech Freedom", cost_gp: Some(12000.0_f64), weight_lbs: Some(2.0_f64), description: Some("resist up to two uses of Power Leech per day") }, // up_equipment.lst:415
    EquipmentTableEntry { key: "Torc of Power Preservation", category: EquipmentCategory::MagicItems, name: "Torc of Power Preservation", cost_gp: Some(36000.0_f64), weight_lbs: Some(2.0_f64), description: Some("manifest all powers by paying power points equal to the standard cost minus 1 (minimum of 1)") }, // up_equipment.lst:416
    EquipmentTableEntry { key: "Warrior's Bracer (Archer Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Archer Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:421
    EquipmentTableEntry { key: "Warrior's Bracer (Ascetic Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Ascetic Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:422
    EquipmentTableEntry { key: "Warrior's Bracer (Assassin's Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Assassin's Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:423
    EquipmentTableEntry { key: "Warrior's Bracer (Brawling Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Brawling Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:424
    EquipmentTableEntry { key: "Warrior's Bracer (Dervish Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Dervish Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:425
    EquipmentTableEntry { key: "Warrior's Bracer (Feral Warrior Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Feral Warrior Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:426
    EquipmentTableEntry { key: "Warrior's Bracer (Gladiator Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Gladiator Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:427
    EquipmentTableEntry { key: "Warrior's Bracer (Infiltrator Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Infiltrator Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:428
    EquipmentTableEntry { key: "Warrior's Bracer (Interceptor Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Interceptor Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:429
    EquipmentTableEntry { key: "Warrior's Bracer (Mind Knight Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Mind Knight Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:430
    EquipmentTableEntry { key: "Warrior's Bracer (Survivor Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Survivor Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:431
    EquipmentTableEntry { key: "Warrior's Bracer (Weaponmaster Path)", category: EquipmentCategory::MagicItems, name: "Warrior's Bracer (Weaponmaster Path)", cost_gp: Some(15000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Treat level as 4 higher for trance and maneuver for specific warrior's path, or can use the trance and maneuver 3/day for 1 minute if path is not known.") }, // up_equipment.lst:432
    EquipmentTableEntry { key: "Warrior's Scabbard", category: EquipmentCategory::MagicItems, name: "Warrior's Scabbard", cost_gp: Some(1000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Gain +1 insight bonus on attack rolls for 1 turn") }, // up_equipment.lst:433
    EquipmentTableEntry { key: "Warblade Staff", category: EquipmentCategory::MagicItems, name: "Warblade Staff", cost_gp: Some(16000.0_f64), weight_lbs: Some(5.0_f64), description: Some("as psicrystal staff; can be transformed into crystal blade (masterwork deep crystal bastard sword); can deliver touch attacks with melee attacks") }, // up_equipment.lst:434
    EquipmentTableEntry { key: "Warblade Staff (Greater)", category: EquipmentCategory::MagicItems, name: "Warblade Staff (Greater)", cost_gp: Some(20000.0_f64), weight_lbs: Some(5.0_f64), description: Some("as psicrystal staff; can be transformed into crystal blade (masterwork deep crystal bastard sword); can deliver touch attacks with melee attacks; crystal blade can create energy burst") }, // up_equipment.lst:435
    EquipmentTableEntry { key: "Wooden Shirt", category: EquipmentCategory::MagicItems, name: "Wooden Shirt", cost_gp: Some(32760.0_f64), weight_lbs: Some(3.0_f64), description: Some("1/day oak body for 9 minutes") }, // up_equipment.lst:436
    EquipmentTableEntry { key: "Ring of Altered Perception", category: EquipmentCategory::MagicItems, name: "Ring of Altered Perception", cost_gp: Some(12000.0_f64), weight_lbs: Some(0.0_f64), description: Some("Give false information to clairsentience powers") }, // up_equipment.lst:441
    EquipmentTableEntry { key: "Ring of Missile Protection", category: EquipmentCategory::MagicItems, name: "Ring of Missile Protection", cost_gp: Some(4000.0_f64), weight_lbs: Some(0.0_f64), description: Some("DR 10/magic vs. ranged attacks, inert for 24 hours after 50 damage avoided") }, // up_equipment.lst:442
    EquipmentTableEntry { key: "Ring of Psionics +2", category: EquipmentCategory::MagicItems, name: "Ring of Psionics +2", cost_gp: Some(20000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Treat your key ability score as 2 higher for bonus power points") }, // up_equipment.lst:443
    EquipmentTableEntry { key: "Ring of Psionics +4", category: EquipmentCategory::MagicItems, name: "Ring of Psionics +4", cost_gp: Some(40000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Treat your key ability score as 4 higher for bonus power points") }, // up_equipment.lst:444
    EquipmentTableEntry { key: "Ring of Psionics +6", category: EquipmentCategory::MagicItems, name: "Ring of Psionics +6", cost_gp: Some(70000.0_f64), weight_lbs: Some(1.0_f64), description: Some("Treat your key ability score as 6 higher for bonus power points") }, // up_equipment.lst:445
    EquipmentTableEntry { key: "Ring (Self Sufficiency)", category: EquipmentCategory::MagicItems, name: "Ring of Self-Sufficiency", cost_gp: Some(10000.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:446
    EquipmentTableEntry { key: "Reverse Capacitor", category: EquipmentCategory::MagicItems, name: "Reverse Capacitor", cost_gp: None, weight_lbs: Some(1.0_f64), description: Some("lose 1d6 pp per round for 7 rounds when trying to use as crystal capacitor") }, // up_equipment.lst:451
    EquipmentTableEntry { key: "Sutra of Tranquil Thought", category: EquipmentCategory::MagicItems, name: "Sutra of Tranquil Thought", cost_gp: Some(0.0_f64), weight_lbs: Some(3.0_f64), description: None }, // up_equipment.lst:456
    EquipmentTableEntry { key: "Annulus", category: EquipmentCategory::MagicItems, name: "Annulus", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:458
    EquipmentTableEntry { key: "Psicrown of the Crystal Mind", category: EquipmentCategory::MagicItems, name: "Psicrown of the Crystal Mind", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // up_equipment.lst:459
    EquipmentTableEntry { key: "Crown of Chaos", category: EquipmentCategory::MagicItems, name: "Crown of Chaos", cost_gp: None, weight_lbs: Some(0.0_f64), description: Some("Power Points - 60; Manifester Level - 11; Powers - Deflect Missiles, Energy Burst, Energy Push, Force Screen; Recharge Cost 11 pp, Gain 6 pp; treat powers as if on power list|PREVARLT:CrownOfChaosLVL,4; Power Points - 140; Manifester Level - 14; Powers - Deflect Missiles, Energy Burst, Energy Push, Force Screen; Recharge Cost 14 pp, Gain 14 pp; treat powers as if on power list; 1/day recharge for 14 pp|PREVARGTEQ:CrownOfChaosLVL,4") }, // up_equipment.lst:464
    EquipmentTableEntry { key: "Dancing Robes of Sharatwan", category: EquipmentCategory::MagicItems, name: "Dancing Robes of Sharatwan", cost_gp: None, weight_lbs: Some(10.0_f64), description: None }, // up_equipment.lst:465
    EquipmentTableEntry { key: "Disruptor", category: EquipmentCategory::MagicItems, name: "Disruptor", cost_gp: None, weight_lbs: Some(2.0_f64), description: Some("enhancement of mind shield is increased by +%, up to +5|DisruptorShieldBonus") }, // up_equipment.lst:466
    EquipmentTableEntry { key: "Dissonance", category: EquipmentCategory::ArmsArmor, name: "Dissonance, Wind of the Mind", cost_gp: None, weight_lbs: Some(10.0_f64), description: None }, // up_equipment.lst:467
    EquipmentTableEntry { key: "Groundscorn Boots", category: EquipmentCategory::MagicItems, name: "Groundscorn Boots of the Twice-Loved", cost_gp: None, weight_lbs: Some(1.0_f64), description: None }, // up_equipment.lst:468
    EquipmentTableEntry { key: "Heartstaff", category: EquipmentCategory::MagicItems, name: "The Heartstaff", cost_gp: None, weight_lbs: Some(5.0_f64), description: Some("Can hold psicrystal, 3 setting stones, 10 companion stones|PREVARLT:HeartstaffLVL,5; Can hold psicrystal, 5 setting stones, 10 companion stones; psicrystal repairs 1 hp/minute; double personality bonuses [not implemented]|PREVARGTEQ:HeartstaffLVL,5") }, // up_equipment.lst:469
    EquipmentTableEntry { key: "Helm of the Hydra", category: EquipmentCategory::MagicItems, name: "Helm of the Hydra", cost_gp: None, weight_lbs: None, description: None }, // up_equipment.lst:470
    EquipmentTableEntry { key: "Moldev", category: EquipmentCategory::MagicItems, name: "Moldev, the Secret Strike", cost_gp: None, weight_lbs: None, description: Some("Can create a light piercing weapon with a 19-20/x2 crit range as a mind blade (1st level soulknife).") }, // up_equipment.lst:471
    EquipmentTableEntry { key: "Severis", category: EquipmentCategory::ArmsArmor, name: "Severis, the Scourge Slayer", cost_gp: None, weight_lbs: Some(6.0_f64), description: None }, // up_equipment.lst:472
    EquipmentTableEntry { key: "Tempest's Blade", category: EquipmentCategory::MagicItems, name: "Tempest's Blade", cost_gp: None, weight_lbs: Some(2.0_f64), description: Some("enhancement of mind blade is increased by +%, up to +5|TempestsBladeHiltBonus") }, // up_equipment.lst:473
];

/// Full UPsi equipment-modifier table: `up_equipmods.lst`'s 113 real
/// standalone records, excluding the one `.MOD`-injected grant and the 113
/// `VISIBLE:NO` `.COPY=` legacy-alias rows -- see this module's own doc
/// comment.
const EQUIPMODS_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry { key: "Special Ability ~ Psionic Blade ~ Weapon", category: EquipmentCategory::Equipmods, name: "Psionic Blade", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // up_equipmods.lst:12
    EquipmentTableEntry { key: "Material ~ Crystal / Mundane", category: EquipmentCategory::Equipmods, name: "Crystal (Mundane)", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("25hp/inch and 8 hardness") }, // up_equipmods.lst:17
    EquipmentTableEntry { key: "Material ~ Crystal / Deep", category: EquipmentCategory::Equipmods, name: "Crystal (Deep)", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("30hp/inch and 10 hardness;Weapon may be charged with 2 Psionic Power Points for +2d6 damage") }, // up_equipmods.lst:18
    EquipmentTableEntry { key: "Material ~ Crystal / Mundane ~ Item", category: EquipmentCategory::Equipmods, name: "Crystal (Mundane)", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("25hp/inch and 8 hardness") }, // up_equipmods.lst:20
    EquipmentTableEntry { key: "Material ~ Crystal / Deep ~ Item", category: EquipmentCategory::Equipmods, name: "Crystal (Deep)", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("30hp/inch and 10 hardness") }, // up_equipmods.lst:21
    EquipmentTableEntry { key: "Special Ability ~ Aporter ~ Armor", category: EquipmentCategory::Equipmods, name: "Aporter", cost_gp: Some(40320.0_f64), weight_lbs: None, description: Some("2/day transports wearer and equipment to any spot within 800 feet as Fold Space") }, // up_equipmods.lst:26
    EquipmentTableEntry { key: "Special Ability ~ Averter ~ Shield", category: EquipmentCategory::Equipmods, name: "Averter", cost_gp: Some(12960.0_f64), weight_lbs: None, description: Some("3/day Aversion to shield, DC 14") }, // up_equipmods.lst:27
    EquipmentTableEntry { key: "Special Ability ~ Determination ~ Armor", category: EquipmentCategory::Equipmods, name: "Determination", cost_gp: Some(30000.0_f64), weight_lbs: None, description: Some("1/day automatic breath of life at 0 hp") }, // up_equipmods.lst:28
    EquipmentTableEntry { key: "Special Ability ~ Ectoplasmic ~ Armor", category: EquipmentCategory::Equipmods, name: "Ectoplasmic", cost_gp: Some(10800.0_f64), weight_lbs: None, description: Some("1/day ectoplasmic form for up to 5 minutes") }, // up_equipmods.lst:29
    EquipmentTableEntry { key: "Special Ability ~ Floating ~ Armor", category: EquipmentCategory::Equipmods, name: "Floating", cost_gp: Some(4000.0_f64), weight_lbs: None, description: Some("negates normal penalty for swimming in armor; additionally a +4 circumstance bonus for Swim checks") }, // up_equipmods.lst:30
    EquipmentTableEntry { key: "Special Ability ~ Fusing ~ Armor / Heavy", category: EquipmentCategory::Equipmods, name: "Fusing", cost_gp: None, weight_lbs: None, description: Some("Armor Check reduced, Max Dex increased, one category lighter") }, // up_equipmods.lst:31
    EquipmentTableEntry { key: "Special Ability ~ Fusing ~ Armor / Light", category: EquipmentCategory::Equipmods, name: "Fusing", cost_gp: None, weight_lbs: None, description: Some("Armor Check reduced, Max Dex increased, one category lighter") }, // up_equipmods.lst:32
    EquipmentTableEntry { key: "Special Ability ~ Fusing ~ Armor / Medium", category: EquipmentCategory::Equipmods, name: "Fusing", cost_gp: None, weight_lbs: None, description: Some("Armor Check reduced, Max Dex increased, one category lighter") }, // up_equipmods.lst:33
    EquipmentTableEntry { key: "Special Ability ~ Fusing ~ Shield", category: EquipmentCategory::Equipmods, name: "Fusing", cost_gp: None, weight_lbs: None, description: Some("Armor Check reduced, Max Dex increased, one category lighter") }, // up_equipmods.lst:34
    EquipmentTableEntry { key: "Special Ability ~ Gleaming ~ Armor", category: EquipmentCategory::Equipmods, name: "Gleaming", cost_gp: None, weight_lbs: None, description: Some("flashes and gleams give wearer a 'fuzzy' appearance granting concealment") }, // up_equipmods.lst:35
    EquipmentTableEntry { key: "Special Ability ~ Heartening ~ Shield", category: EquipmentCategory::Equipmods, name: "Heartening", cost_gp: Some(720.0_f64), weight_lbs: None, description: Some("grants 5 temporary hit points a day for up to 4 minutes") }, // up_equipmods.lst:36
    EquipmentTableEntry { key: "Special Ability ~ Landing ~ Armor", category: EquipmentCategory::Equipmods, name: "Landing", cost_gp: Some(4000.0_f64), weight_lbs: None, description: Some("wearer takes no damage from the first 60' of a fall; target lands on feet no matter the distance of a fall") }, // up_equipmods.lst:37
    EquipmentTableEntry { key: "Special Ability ~ Linked ~ Armor", category: EquipmentCategory::Equipmods, name: "Linked", cost_gp: Some(6000.0_f64), weight_lbs: None, description: Some("wearer forms a bond with other wearers of linked armor or shields in 10 miles; as mindlink power") }, // up_equipmods.lst:38
    EquipmentTableEntry { key: "Special Ability ~ Manifester ~ Shield", category: EquipmentCategory::Equipmods, name: "Manifester", cost_gp: Some(10800.0_f64), weight_lbs: None, description: Some("generates 3 power points per day that the wielder can use when manifesting a power; all must be used at once") }, // up_equipmods.lst:39
    EquipmentTableEntry { key: "Special Ability ~ Mindarmor ~ Armor", category: EquipmentCategory::Equipmods, name: "Mindarmor", cost_gp: Some(24000.0_f64), weight_lbs: None, description: Some("wearer gains a +3 insight bonus to Will saves vs. mind affecting and compulsion effects") }, // up_equipmods.lst:40
    EquipmentTableEntry { key: "Special Ability ~ Murmuring ~ Armor", category: EquipmentCategory::Equipmods, name: "Murmuring", cost_gp: None, weight_lbs: None, description: Some("-5 penalty to others' concentration checks within 30 ft.") }, // up_equipmods.lst:41
    EquipmentTableEntry { key: "Special Ability ~ Murmuring / Greater ~ Armor", category: EquipmentCategory::Equipmods, name: "Murmuring (Greater)", cost_gp: None, weight_lbs: None, description: Some("-5 penalty to others' concentration checks within 30 ft.; failed concentration check does 1d6 bleed damage") }, // up_equipmods.lst:42
    EquipmentTableEntry { key: "Special Ability ~ Phasing ~ Armor", category: EquipmentCategory::Equipmods, name: "Phasing", cost_gp: Some(65520.0_f64), weight_lbs: None, description: Some("phase through wood, plaster, or stone for total of 60' per day") }, // up_equipmods.lst:43
    EquipmentTableEntry { key: "Special Ability ~ Power Resistance / 13 ~ Armor", category: EquipmentCategory::Equipmods, name: "Power Resistance (13)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:44
    EquipmentTableEntry { key: "Special Ability ~ Power Resistance / 15 ~ Armor", category: EquipmentCategory::Equipmods, name: "Power Resistance (15)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:45
    EquipmentTableEntry { key: "Special Ability ~ Power Resistance / 17 ~ Armor", category: EquipmentCategory::Equipmods, name: "Power Resistance (17)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:46
    EquipmentTableEntry { key: "Special Ability ~ Power Resistance / 19 ~ Armor", category: EquipmentCategory::Equipmods, name: "Power Resistance (19)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:47
    EquipmentTableEntry { key: "Special Ability ~ Quickness ~ Armor", category: EquipmentCategory::Equipmods, name: "Quickness", cost_gp: None, weight_lbs: None, description: Some("armor adds 5' to wearer's movement after armor reduction") }, // up_equipmods.lst:48
    EquipmentTableEntry { key: "Special Ability ~ Radiant / Psionic ~ Armor", category: EquipmentCategory::Equipmods, name: "Radiant", cost_gp: None, weight_lbs: None, description: Some("resistance 10 to energy attacks; radiates light in 60' radius for rounds equal to points absorbed") }, // up_equipmods.lst:49
    EquipmentTableEntry { key: "Special Ability ~ Ranged ~ Shield", category: EquipmentCategory::Equipmods, name: "Ranged", cost_gp: None, weight_lbs: None, description: Some("can be thrown with a range increment of 30 ft by those proficient in its use; returns just before wielder's next turn") }, // up_equipmods.lst:50
    EquipmentTableEntry { key: "Special Ability ~ Seeing ~ Armor", category: EquipmentCategory::Equipmods, name: "Seeing", cost_gp: Some(6000.0_f64), weight_lbs: None, description: Some("flanking only gains +1 bonus; gains +1 enhancement to Perception checks, but a -2 save vs. gaze attacks") }, // up_equipmods.lst:51
    EquipmentTableEntry { key: "Special Ability ~ Time Buttress ~ Shield", category: EquipmentCategory::Equipmods, name: "Time Buttress", cost_gp: None, weight_lbs: None, description: Some("1/day Timeless Body") }, // up_equipmods.lst:52
    EquipmentTableEntry { key: "Special Ability ~ Vanishing ~ Armor", category: EquipmentCategory::Equipmods, name: "Vanishing", cost_gp: None, weight_lbs: None, description: Some("2/day cloud mind") }, // up_equipmods.lst:53
    EquipmentTableEntry { key: "Special Ability ~ Wall ~ Shield", category: EquipmentCategory::Equipmods, name: "Wall", cost_gp: Some(20160.0_f64), weight_lbs: None, description: Some("1/day drop to form Wall of Ectoplasm for 7 minutes") }, // up_equipmods.lst:54
    EquipmentTableEntry { key: "Special Ability ~ Agile ~ Melee", category: EquipmentCategory::Equipmods, name: "Agile", cost_gp: None, weight_lbs: None, description: Some("Add Dexterity bonus, instead of Strength bonus, to weapon damage") }, // up_equipmods.lst:59
    EquipmentTableEntry { key: "Special Ability ~ Bodyfeeder ~ Melee", category: EquipmentCategory::Equipmods, name: "Bodyfeeder", cost_gp: None, weight_lbs: None, description: Some("adds damage as temporary hit points on a critical hit") }, // up_equipmods.lst:60
    EquipmentTableEntry { key: "Special Ability ~ Challenger ~ Melee", category: EquipmentCategory::Equipmods, name: "Challenger", cost_gp: None, weight_lbs: None, description: Some("attack and use Intimidate to give target penalties to attack others") }, // up_equipmods.lst:61
    EquipmentTableEntry { key: "Special Ability ~ Collision ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Collision", cost_gp: None, weight_lbs: None, description: Some("+5 damage on any hit on top of enhancement bonus") }, // up_equipmods.lst:62
    EquipmentTableEntry { key: "Special Ability ~ Collision ~ Melee", category: EquipmentCategory::Equipmods, name: "Collision", cost_gp: None, weight_lbs: None, description: Some("+5 damage on any hit on top of enhancement bonus") }, // up_equipmods.lst:63
    EquipmentTableEntry { key: "Special Ability ~ Collision ~ Ranged", category: EquipmentCategory::Equipmods, name: "Collision", cost_gp: None, weight_lbs: None, description: Some("+5 damage on any hit on top of enhancement bonus") }, // up_equipmods.lst:64
    EquipmentTableEntry { key: "Special Ability ~ Coup de Grace ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Coup de Grace", cost_gp: None, weight_lbs: None, description: Some("paralyzed for 1 round on crit, DC 27") }, // up_equipmods.lst:65
    EquipmentTableEntry { key: "Special Ability ~ Coup de Grace ~ Melee", category: EquipmentCategory::Equipmods, name: "Coup de Grace", cost_gp: None, weight_lbs: None, description: Some("paralyzed for 1 round on crit, DC 27") }, // up_equipmods.lst:66
    EquipmentTableEntry { key: "Special Ability ~ Coup de Grace ~ Ranged", category: EquipmentCategory::Equipmods, name: "Coup de Grace", cost_gp: None, weight_lbs: None, description: Some("paralyzed for 1 round on crit, DC 27") }, // up_equipmods.lst:67
    EquipmentTableEntry { key: "Special Ability ~ Dislocator ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Dislocator", cost_gp: None, weight_lbs: None, description: Some("teleport target 1-100 miles randomly, DC 17") }, // up_equipmods.lst:68
    EquipmentTableEntry { key: "Special Ability ~ Dislocator ~ Melee", category: EquipmentCategory::Equipmods, name: "Dislocator", cost_gp: None, weight_lbs: None, description: Some("3/day teleport target 1-100 miles randomly, DC 17") }, // up_equipmods.lst:69
    EquipmentTableEntry { key: "Special Ability ~ Dislocator ~ Ranged", category: EquipmentCategory::Equipmods, name: "Dislocator", cost_gp: None, weight_lbs: None, description: Some("3/day teleport target 1-100 miles randomly, DC 17") }, // up_equipmods.lst:70
    EquipmentTableEntry { key: "Special Ability ~ Dissipater ~ Melee", category: EquipmentCategory::Equipmods, name: "Dissipater", cost_gp: None, weight_lbs: None, description: Some("ignores damage reduction and hardness of ectoplasm creatures and items and treats all successful hits as critical hits") }, // up_equipmods.lst:71
    EquipmentTableEntry { key: "Special Ability ~ Dueling / Luck Bonus ~ Melee", category: EquipmentCategory::Equipmods, name: "Dueling", cost_gp: Some(14000.0_f64), weight_lbs: None, description: Some("Gain luck bonus to CMB and CMD double the enchantment bonus with maneuvers when using weapon") }, // up_equipmods.lst:72
    EquipmentTableEntry { key: "Special Ability ~ Energy / Cold ~ Ranged", category: EquipmentCategory::Equipmods, name: "Energy (Cold)", cost_gp: Some(12000.0_f64), weight_lbs: None, description: Some("Creates cold energy ammunition") }, // up_equipmods.lst:73
    EquipmentTableEntry { key: "Special Ability ~ Energy / Electricity ~ Ranged", category: EquipmentCategory::Equipmods, name: "Energy (Electricity)", cost_gp: Some(12000.0_f64), weight_lbs: None, description: Some("Creates electricity energy ammunition") }, // up_equipmods.lst:74
    EquipmentTableEntry { key: "Special Ability ~ Energy / Fire ~ Ranged", category: EquipmentCategory::Equipmods, name: "Energy (Fire)", cost_gp: Some(12000.0_f64), weight_lbs: None, description: Some("Creates fire energy ammunition") }, // up_equipmods.lst:75
    EquipmentTableEntry { key: "Special Ability ~ Energy / Cold / Greater ~ Ranged", category: EquipmentCategory::Equipmods, name: "Energy (Greater Cold)", cost_gp: None, weight_lbs: None, description: Some("Converts ammunition to cold energy") }, // up_equipmods.lst:76
    EquipmentTableEntry { key: "Special Ability ~ Energy / Electricity / Greater ~ Ranged", category: EquipmentCategory::Equipmods, name: "Energy (Greater Electricity)", cost_gp: None, weight_lbs: None, description: Some("Converts ammunition to electricity energy") }, // up_equipmods.lst:77
    EquipmentTableEntry { key: "Special Ability ~ Energy / Fire / Greater ~ Ranged", category: EquipmentCategory::Equipmods, name: "Energy (Greater Fire)", cost_gp: None, weight_lbs: None, description: Some("Converts ammunition to fire energy") }, // up_equipmods.lst:78
    EquipmentTableEntry { key: "Special Ability ~ Fatalist ~ Weapon", category: EquipmentCategory::Equipmods, name: "Fatalist", cost_gp: None, weight_lbs: None, description: Some("for 1 minute, target gets -2 to AC and Will saves; Intimidate checks against target get +3") }, // up_equipmods.lst:79
    EquipmentTableEntry { key: "Special Ability ~ Dislocator / Great ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Great Dislocator", cost_gp: None, weight_lbs: None, description: Some("3/day send target to random plane, DC 20") }, // up_equipmods.lst:80
    EquipmentTableEntry { key: "Special Ability ~ Dislocator / Great ~ Melee", category: EquipmentCategory::Equipmods, name: "Great Dislocator", cost_gp: None, weight_lbs: None, description: Some("3/day send target to random plane, DC 20") }, // up_equipmods.lst:81
    EquipmentTableEntry { key: "Special Ability ~ Dislocator / Great ~ Ranged", category: EquipmentCategory::Equipmods, name: "Great Dislocator", cost_gp: None, weight_lbs: None, description: Some("3/day send target to random plane, DC 20") }, // up_equipmods.lst:82
    EquipmentTableEntry { key: "Special Ability ~ Guardian ~ Weapon", category: EquipmentCategory::Equipmods, name: "Guardian", cost_gp: None, weight_lbs: None, description: Some("Transfer some or all of enhancement bonus to saves") }, // up_equipmods.lst:83
    EquipmentTableEntry { key: "Special Ability ~ Invader ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Invader", cost_gp: None, weight_lbs: None, description: Some("Can forcibly add target to collective") }, // up_equipmods.lst:84
    EquipmentTableEntry { key: "Special Ability ~ Knockout ~ Weapon", category: EquipmentCategory::Equipmods, name: "Knockout", cost_gp: None, weight_lbs: None, description: Some("Target falls asleep for 3 rounds, new save every round (DC 13 Fort)") }, // up_equipmods.lst:85
    EquipmentTableEntry { key: "Special Ability ~ Linked Striking ~ Weapon", category: EquipmentCategory::Equipmods, name: "Linked Striking", cost_gp: None, weight_lbs: None, description: Some("Add 2 to enhancement bonus and +2d6 damage on subsequent hits vs. an enemy") }, // up_equipmods.lst:86
    EquipmentTableEntry { key: "Special Ability ~ Lucky ~ Melee", category: EquipmentCategory::Equipmods, name: "Lucky", cost_gp: None, weight_lbs: None, description: Some("1/day reroll failed attack roll as free action") }, // up_equipmods.lst:87
    EquipmentTableEntry { key: "Special Ability ~ Manifester ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Manifester", cost_gp: Some(16000.0_f64), weight_lbs: None, description: Some("generates 5 power points per day that the wielder can use when manifesting a power; all must be used at once") }, // up_equipmods.lst:88
    EquipmentTableEntry { key: "Special Ability ~ Manifester ~ Melee", category: EquipmentCategory::Equipmods, name: "Manifester", cost_gp: Some(16000.0_f64), weight_lbs: None, description: Some("generates 5 power points per day that the wielder can use when manifesting a power; all must be used at once") }, // up_equipmods.lst:89
    EquipmentTableEntry { key: "Special Ability ~ Manifester ~ Ranged", category: EquipmentCategory::Equipmods, name: "Manifester", cost_gp: Some(16000.0_f64), weight_lbs: None, description: Some("generates 5 power points per day that the wielder can use when manifesting a power; all must be used at once") }, // up_equipmods.lst:90
    EquipmentTableEntry { key: "Special Ability ~ Mindcrusher ~ Melee", category: EquipmentCategory::Equipmods, name: "Mindcrusher", cost_gp: None, weight_lbs: None, description: Some("drains half base damage in power points; if no power points or non-psionic, 1d2 Wisdom Damage, DC 17") }, // up_equipmods.lst:91
    EquipmentTableEntry { key: "Special Ability ~ Mindfeeder ~ Melee", category: EquipmentCategory::Equipmods, name: "Mindfeeder", cost_gp: None, weight_lbs: None, description: Some("on a critical hit gives wielder temporary power points equal to damage dealt; points overlap, don't stack") }, // up_equipmods.lst:92
    EquipmentTableEntry { key: "Special Ability ~ Parrying ~ Melee", category: EquipmentCategory::Equipmods, name: "Parrying", cost_gp: Some(8000.0_f64), weight_lbs: None, description: Some("provides a +1 insight bonus to AC and saves when wielded") }, // up_equipmods.lst:93
    EquipmentTableEntry { key: "Special Ability ~ Power Storing ~ Melee", category: EquipmentCategory::Equipmods, name: "Power Storing", cost_gp: None, weight_lbs: None, description: Some("can store one power up to 5PP and discharge on successful hit as a swift action if desired") }, // up_equipmods.lst:94
    EquipmentTableEntry { key: "Special Ability ~ Psibane ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Psibane", cost_gp: None, weight_lbs: None, description: Some("+2 attack bonus and does +2d6 bonus damage vs. psionic creatures") }, // up_equipmods.lst:95
    EquipmentTableEntry { key: "Special Ability ~ Psibane ~ Melee", category: EquipmentCategory::Equipmods, name: "Psibane", cost_gp: None, weight_lbs: None, description: Some("+2 attack bonus and does +2d6 bonus damage vs. psionic creatures") }, // up_equipmods.lst:96
    EquipmentTableEntry { key: "Special Ability ~ Psibane ~ Ranged", category: EquipmentCategory::Equipmods, name: "Psibane", cost_gp: None, weight_lbs: None, description: Some("+2 attack bonus and does +2d6 bonus damage vs. psionic creatures bestowed on ammunition") }, // up_equipmods.lst:97
    EquipmentTableEntry { key: "Special Ability ~ Psicrystal Setting ~ Weapon", category: EquipmentCategory::Equipmods, name: "Psicrystal Setting", cost_gp: Some(2000.0_f64), weight_lbs: None, description: Some("psicrystal can be docked to weapon like a psicrystal staff; while docked, the weapon can be charged like deep crystal") }, // up_equipmods.lst:98
    EquipmentTableEntry { key: "Special Ability ~ Psychic ~ Melee", category: EquipmentCategory::Equipmods, name: "Psychic", cost_gp: Some(35000.0_f64), weight_lbs: None, description: Some("enhancement bonus varies depending on Power Point reserve of character; see text") }, // up_equipmods.lst:99
    EquipmentTableEntry { key: "Special Ability ~ Psychodisruptive ~ Weapon", category: EquipmentCategory::Equipmods, name: "Psychodisruptive", cost_gp: None, weight_lbs: None, description: Some("disrupts psionic and magical abilities") }, // up_equipmods.lst:100
    EquipmentTableEntry { key: "Special Ability ~ Psychokinetic ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Psychokinetic", cost_gp: None, weight_lbs: None, description: Some("+1d4 ectoplasmic damage (ignores DR)") }, // up_equipmods.lst:101
    EquipmentTableEntry { key: "Special Ability ~ Psychokinetic ~ Melee", category: EquipmentCategory::Equipmods, name: "Psychokinetic", cost_gp: None, weight_lbs: None, description: Some("+1d4 ectoplasmic damage (ignores DR)") }, // up_equipmods.lst:102
    EquipmentTableEntry { key: "Special Ability ~ Psychokinetic ~ Ranged", category: EquipmentCategory::Equipmods, name: "Psychokinetic", cost_gp: None, weight_lbs: None, description: Some("+1d4 ectoplasmic damage (ignores DR)") }, // up_equipmods.lst:103
    EquipmentTableEntry { key: "Special Ability ~ Psychokinetic Burst ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Psychokinetic Burst", cost_gp: None, weight_lbs: None, description: Some("+1d4 ectoplasmic damage (ignores DR); On a critical hit deals +%d6 additional ectoplasmic damage (ignores DR)|CRITMULT-1") }, // up_equipmods.lst:104
    EquipmentTableEntry { key: "Special Ability ~ Psychokinetic Burst ~ Melee", category: EquipmentCategory::Equipmods, name: "Psychokinetic Burst", cost_gp: None, weight_lbs: None, description: Some("+1d4 ectoplasmic damage (ignores DR); On a critical hit deals +%d6 additional ectoplasmic damage (ignores DR)|CRITMULT-1") }, // up_equipmods.lst:105
    EquipmentTableEntry { key: "Special Ability ~ Psychokinetic Burst ~ Ranged", category: EquipmentCategory::Equipmods, name: "Psychokinetic Burst", cost_gp: None, weight_lbs: None, description: Some("+1d4 ectoplasmic damage (ignores DR) bestowed on ammunition; On a critical hit deals +%d6 additional ectoplasmic damage (ignores DR)|CRITMULT-1") }, // up_equipmods.lst:106
    EquipmentTableEntry { key: "Special Ability ~ Rebounding ~ Thrown", category: EquipmentCategory::Equipmods, name: "Rebounding", cost_gp: Some(12000.0_f64), weight_lbs: None, description: Some("Use iterative attacks with single thrown weapon") }, // up_equipmods.lst:107
    EquipmentTableEntry { key: "Special Ability ~ Scourgebane ~ Weapon", category: EquipmentCategory::Equipmods, name: "Scourgebane", cost_gp: None, weight_lbs: None, description: Some("extra +2 enhancement, +2d6 damage against phrenic scourge creatures") }, // up_equipmods.lst:108
    EquipmentTableEntry { key: "Special Ability ~ Soulbreaker ~ Melee", category: EquipmentCategory::Equipmods, name: "Soulbreaker", cost_gp: None, weight_lbs: None, description: Some("on a critical hit bestows one negative level on target; FORT Save (DC:18) day later or permanent") }, // up_equipmods.lst:109
    EquipmentTableEntry { key: "Special Ability ~ Sundering ~ Melee", category: EquipmentCategory::Equipmods, name: "Sundering", cost_gp: None, weight_lbs: None, description: Some("wielder treated as if has Improved Sunder Feat") }, // up_equipmods.lst:110
    EquipmentTableEntry { key: "Special Ability ~ Suppression ~ Ammunition", category: EquipmentCategory::Equipmods, name: "Suppression", cost_gp: None, weight_lbs: None, description: Some("target opponent or object suffers targeted dispel psionics ((1d20 + 5 + wielder's manifester level, max 15) vs DC of 11+manifester level of power") }, // up_equipmods.lst:111
    EquipmentTableEntry { key: "Special Ability ~ Suppression ~ Melee", category: EquipmentCategory::Equipmods, name: "Suppression", cost_gp: None, weight_lbs: None, description: Some("target opponent or object suffers targeted dispel psionics ((1d20 + 5 + wielders manifester level, max 15) vs DC of 11+manifester level of power") }, // up_equipmods.lst:112
    EquipmentTableEntry { key: "Special Ability ~ Suppression ~ Ranged", category: EquipmentCategory::Equipmods, name: "Suppression", cost_gp: None, weight_lbs: None, description: Some("3/day target opponent or object suffers targeted dispel psionics ((1d20 + 5 + wielders manifester level, max 15) vs DC of 11+manifester level of power") }, // up_equipmods.lst:113
    EquipmentTableEntry { key: "Special Ability ~ Teleporting ~ Thrown", category: EquipmentCategory::Equipmods, name: "Teleporting", cost_gp: None, weight_lbs: None, description: Some("when thrown will return, by teleporting through Astral Plane, back to thrower's hand just before thrower's next turn") }, // up_equipmods.lst:114
    EquipmentTableEntry { key: "Special Ability ~ Whistling ~ Weapon", category: EquipmentCategory::Equipmods, name: "Whistling", cost_gp: None, weight_lbs: None, description: Some("target staggered or dazed if already staggered (Will DC 15); target must pay additional 4 power points to manifest a power in the next round") }, // up_equipmods.lst:115
    EquipmentTableEntry { key: "Special Ability ~ Wrenching ~ Ranged", category: EquipmentCategory::Equipmods, name: "Wrenching", cost_gp: None, weight_lbs: None, description: Some("Reposition target towards wielder") }, // up_equipmods.lst:116
    EquipmentTableEntry { key: "Special Ability ~ Power Effect / Power Trigger", category: EquipmentCategory::Equipmods, name: "|Power Effect (50 Charges/Power Trigger)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:121
    EquipmentTableEntry { key: "Special Ability ~ Power Effect / Single Use", category: EquipmentCategory::Equipmods, name: "|Power Effect (Single Use/Use Activated)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:122
    EquipmentTableEntry { key: "Special Ability ~ Power Effect / Single Use / Crawling", category: EquipmentCategory::Equipmods, name: "|Power Effect (Single Use/Use Activated)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:123
    EquipmentTableEntry { key: "Special Ability ~ Power Effect / Completion", category: EquipmentCategory::Equipmods, name: "|Power Effect (Single Use/Completion)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:124
    EquipmentTableEntry { key: "Special Ability ~ Power Effect / Psicrown / Lesser", category: EquipmentCategory::Equipmods, name: "|Power Effect (Psicrown/Lesser)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:125
    EquipmentTableEntry { key: "Special Ability ~ Power Effect / Psicrown / Greater", category: EquipmentCategory::Equipmods, name: "|Power Effect (Psicrown/Greater)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:126
    EquipmentTableEntry { key: "Special Ability ~ Power Effect / Psicrown / True", category: EquipmentCategory::Equipmods, name: "|Power Effect (Psicrown/True)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:127
    EquipmentTableEntry { key: "Special Ability ~ Power Choice / Earring of Resistance", category: EquipmentCategory::Equipmods, name: "|Power Choice (Earring of Resistance)", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // up_equipmods.lst:128
    EquipmentTableEntry { key: "Special Ability ~ Power Choice / Green White Ioun Stone", category: EquipmentCategory::Equipmods, name: "|Power Choice (Green White Ioun Stone)", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // up_equipmods.lst:129
    EquipmentTableEntry { key: "Special Ability ~ Power Choice / Mind Stone", category: EquipmentCategory::Equipmods, name: "|Power Choice (Mind Stone)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:130
    EquipmentTableEntry { key: "Special Ability ~ Power Choice / Mind Stone / Greater", category: EquipmentCategory::Equipmods, name: "|Power Choice (Greater Mind Stone)", cost_gp: None, weight_lbs: None, description: None }, // up_equipmods.lst:131
    EquipmentTableEntry { key: "Special Ability ~ Power Choice / Third Eye", category: EquipmentCategory::Equipmods, name: "|Talent Choice (Third Eye)", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // up_equipmods.lst:132
    EquipmentTableEntry { key: "Special Quality ~ Gloves of Calling", category: EquipmentCategory::Equipmods, name: "Weapon Choice", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // up_equipmods.lst:134
    EquipmentTableEntry { key: "Special Quality ~ Dancing Robes of Sharatwan / Armor Bonus", category: EquipmentCategory::Equipmods, name: "Armor Bonus for Dancing Robes of Sharatwan", cost_gp: None, weight_lbs: None, description: Some("+% enhancement|DancingRobesArmorBonus") }, // up_equipmods.lst:139
    EquipmentTableEntry { key: "Special Quality ~ Dancing Robes of Sharatwan / Determination", category: EquipmentCategory::Equipmods, name: "Determination for Dancing Robes of Sharatwan", cost_gp: None, weight_lbs: None, description: Some("1/day automatic breath of life at 0 hp|PREMULT:2,[PREVARGTEQ:TL,14],[PREABILITY:1,CATEGORY=Special Ability,Dancing Robes of Sharatwan Attunement]") }, // up_equipmods.lst:140
    EquipmentTableEntry { key: "Special Quality ~ Dissonance / Enhancement Bonus / Main", category: EquipmentCategory::Equipmods, name: "Enhancement Bonus for Dissonance Main", cost_gp: None, weight_lbs: None, description: Some("+% enhancement|DissonanceEnhancementBonusMain") }, // up_equipmods.lst:141
    EquipmentTableEntry { key: "Special Quality ~ Dissonance / Enhancement Bonus / Alt", category: EquipmentCategory::Equipmods, name: "Enhancement Bonus for Dissonance Alt", cost_gp: None, weight_lbs: None, description: Some("+% enhancement|DissonanceEnhancementBonusAlt") }, // up_equipmods.lst:142
    EquipmentTableEntry { key: "Special Quality ~ Dissonance / Whistling / Main", category: EquipmentCategory::Equipmods, name: "Whistling for Dissonance Main", cost_gp: None, weight_lbs: None, description: Some("target staggered or dazed if already staggered (Will DC 15); target must pay additional 4 power points to manifest a power in the next round|PREVARGTEQ:TL,6") }, // up_equipmods.lst:143
    EquipmentTableEntry { key: "Special Quality ~ Dissonance / Whistling / Alt", category: EquipmentCategory::Equipmods, name: "Whistling for Dissonance Alt", cost_gp: None, weight_lbs: None, description: Some("target staggered or dazed if already staggered (Will DC 15); target must pay additional 4 power points to manifest a power in the next round|PREVARGTEQ:TL,18") }, // up_equipmods.lst:144
    EquipmentTableEntry { key: "Special Quality ~ Severis / Enhancement Bonus", category: EquipmentCategory::Equipmods, name: "Enhancement Bonus for Severis", cost_gp: None, weight_lbs: None, description: Some("+% enhancement|Severis") }, // up_equipmods.lst:145
    EquipmentTableEntry { key: "Special Quality ~ Severis / Scourgebane", category: EquipmentCategory::Equipmods, name: "Scourgebane for Severis", cost_gp: None, weight_lbs: None, description: Some("extra +2 enhancement, +2d6 damage against phrenic scourge creatures|PREMULT:2,[PREVARGTEQ:TL,6],[PREABILITY:1,CATEGORY=Special Ability,Severis Attunement]") }, // up_equipmods.lst:146
];

/// The 113 `VISIBLE:NO` `.COPY=` legacy-alias short codes this table
/// deliberately does not carry as standalone entries -- named explicitly
/// (not just counted) so the classifier's own resulting `engine-does-not-hold`
/// residue for this book can be attributed to this exclusion by anyone
/// auditing `docs/work-inventory.json` later, the same way ARG's/UI's own
/// modules name their excluded rows rather than leaving an unexplained gap.
pub const EXCLUDED_LEGACY_ALIAS_SHORT_CODES: &[&str] = &[
"PSIBLADE", // line 161
"CRYS_MUN", // line 165
"CRYS_DEEP", // line 166
"CRYS_MUN_ITEM", // line 168
"CRYS_DEEP_ITEM", // line 169
"APORT", // line 174
"AVERT", // line 175
"DETERM", // line 176
"ECTOPLAS", // line 177
"FLOAT", // line 178
"FUSE_ARMR_HVY", // line 179
"FUSE_ARMR_LT", // line 180
"FUSE_ARMR_MED", // line 181
"FUSE_ARMR_SHLD", // line 182
"GLEAM", // line 183
"HEART", // line 184
"LANDI", // line 185
"LNKED", // line 186
"MANIF_SHLD", // line 187
"MNDARMR", // line 188
"MURMR", // line 189
"MURMR_G", // line 190
"PHASING", // line 191
"PWRRST_13", // line 192
"PWRRST_15", // line 193
"PWRRST_17", // line 194
"PWRRST_19", // line 195
"QUICKN", // line 196
"RADIANT", // line 197
"RNGD", // line 198
"SEEING", // line 199
"TIMEBUT", // line 200
"VANISH", // line 201
"WALL", // line 202
"AGILE", // line 207
"BODYFEED", // line 208
"CHALL", // line 209
"COLLI_A", // line 210
"COLLI_M", // line 211
"COLLI_R", // line 212
"COUPGR_A", // line 213
"COUPGR_M", // line 214
"COUPGR_R", // line 215
"DISLOC_A", // line 216
"DISLOC_M", // line 217
"DISLOC_R", // line 218
"DISSIP", // line 219
"DUEL", // line 220
"ENRGY_C", // line 221
"ENRGY_E", // line 222
"ENRGY_F", // line 223
"ENRGY_GC", // line 224
"ENRGY_GE", // line 225
"ENRGY_GF", // line 226
"FTLST", // line 227
"DISLOC_GRT_A", // line 228
"DISLOC_GRT_M", // line 229
"DISLOC_GRT_R", // line 230
"GUARDN", // line 231
"INVDR", // line 232
"KNOUT", // line 233
"LNKST", // line 234
"LUCKY", // line 235
"MANIF_WPN_A", // line 236
"MANIF_WPN_M", // line 237
"MANIF_WPN_R", // line 238
"MINDCRU", // line 239
"MINDFEED", // line 240
"PARRYING", // line 241
"PWRSTOR", // line 242
"PSIBANE_A", // line 243
"PSIBANE_M", // line 244
"PSIBANE_R", // line 245
"PSISET", // line 246
"PSYCHIC", // line 247
"PSYDIS", // line 248
"PKIN_A", // line 249
"PKIN_M", // line 250
"PKIN_R", // line 251
"PKIN_BR_A", // line 252
"PKIN_BR_M", // line 253
"PKIN_BR_R", // line 254
"RBDNG", // line 255
"SCGBN", // line 256
"SOULBREAK", // line 257
"SUNDERING", // line 258
"SUPPRESS_A", // line 259
"SUPPRESS_M", // line 260
"SUPPRESS_R", // line 261
"TPORTING", // line 262
"WHSTL", // line 263
"WRNCH", // line 264
"PWR_DRJ", // line 269
"PWR_PT", // line 270
"PWR_CRWL", // line 271
"PWR_PS", // line 272
"PWR_PCWN_L", // line 273
"PWR_PCWN_G", // line 274
"PWR_PCWN_T", // line 275
"PWR_ERNG", // line 276
"PWR_GWI", // line 277
"PWR_MDST", // line 278
"PWR_MDST_G", // line 279
"PWR_THRDI", // line 280
"WPN_CALL", // line 282
"PLUSN_DRS", // line 287
"DETERM_DRS", // line 288
"PLUSN_DIS_M", // line 289
"PLUSN_DIS_A", // line 290
"WHSTL_DIS_M", // line 291
"WHSTL_DIS_A", // line 292
"PLUSN_SVS", // line 293
"SCGBN_SVS", // line 294
];

pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
    EQUIPMENT_TABLE
}

pub fn equipmod_tables() -> &'static [EquipmentTableEntry] {
    EQUIPMODS_TABLE
}

/// Resolves a UPsi equipment or equipment-modifier item by key.
pub fn equipment_resolve(key: &str) -> Option<&'static EquipmentTableEntry> {
    EQUIPMENT_TABLE
        .iter()
        .chain(EQUIPMODS_TABLE)
        .find(|entry| entry.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_439_records_326_equipment_113_equipmods() {
        assert_eq!(EQUIPMENT_TABLE.len(), 326, "326 real records in up_equipment.lst");
        assert_eq!(EQUIPMODS_TABLE.len(), 113, "113 real standalone records in up_equipmods.lst, excluding the .MOD row and the 113 .COPY= legacy aliases");
        assert_eq!(EQUIPMENT_TABLE.len() + EQUIPMODS_TABLE.len(), 439);
    }

    #[test]
    fn keys_are_unique_within_each_table() {
        for (label, table) in [("equipment", EQUIPMENT_TABLE), ("equipmods", EQUIPMODS_TABLE)] {
            let mut keys: Vec<&str> = table.iter().map(|e| e.key).collect();
            keys.sort_unstable();
            let before = keys.len();
            keys.dedup();
            assert_eq!(keys.len(), before, "every {label} key must be unique within its own table");
        }
    }

    /// Regression guard against the one `.MOD` row (`Special Ability ~
    /// Keen ~ Weapon.MOD`) ever being counted as a new standalone record.
    #[test]
    fn the_keen_weapon_mod_row_is_not_a_standalone_equipmod() {
        assert!(
            equipment_resolve("Special Ability ~ Keen ~ Weapon.MOD").is_none(),
            "the .MOD row injects onto an existing cross-book record, it is not a new UPsi equipmod"
        );
    }

    /// Regression guard, both directions, against the `.COPY=` hazard this
    /// table's own doc comment traced and corrected: the legacy alias
    /// short codes must NOT resolve as their own standalone entries (the
    /// first extraction pass's mistake), and the named exclusion list must
    /// match the real 113-row population exactly.
    #[test]
    fn copy_alias_short_codes_are_named_but_not_shipped_as_entries() {
        assert_eq!(EXCLUDED_LEGACY_ALIAS_SHORT_CODES.len(), 113);
        for code in ["APORT", "AVERT", "AGILE", "CRYS_MUN", "CRYS_DEEP"] {
            assert!(
                EXCLUDED_LEGACY_ALIAS_SHORT_CODES.contains(&code),
                "{code} must be named in the exclusion list"
            );
            assert!(
                equipment_resolve(code).is_none(),
                "{code} must NOT resolve as its own standalone entry -- it is a VISIBLE:NO legacy alias"
            );
        }
    }

    /// Astral Armor/Astral Juggernaut's real, verbatim negative COST:-150
    /// -- pinned so a future "cost must be non-negative" cleanup does not
    /// silently clamp real corpus data.
    #[test]
    fn astral_suit_family_keeps_its_real_negative_cost() {
        let armor = equipment_resolve("Astral Armor").expect("Astral Armor must resolve");
        assert_eq!(armor.cost_gp, Some(-150.0));
        let juggernaut = equipment_resolve("Astral Juggernaut").expect("Astral Juggernaut must resolve");
        assert_eq!(juggernaut.cost_gp, Some(-150.0));
    }

    /// Field-coverage audit row pins the exact counts named in this
    /// module's own doc comment.
    #[test]
    fn field_coverage_matches_documented_counts() {
        let report = field_coverage_report();
        assert_eq!(report.total_records, 439);
        assert_eq!(report.records_expected, 439);
        assert_eq!(report.has_cost, 315 + 33);
        assert_eq!(report.has_weight, 324);
        assert_eq!(report.has_description, 216 + 95);
    }

    #[test]
    fn arms_armor_and_magic_items_split_matches_documented_counts() {
        let arms_armor = EQUIPMENT_TABLE.iter().filter(|e| e.category == EquipmentCategory::ArmsArmor).count();
        let magic_items = EQUIPMENT_TABLE.iter().filter(|e| e.category == EquipmentCategory::MagicItems).count();
        assert_eq!(arms_armor, 52);
        assert_eq!(magic_items, 274);
        assert_eq!(arms_armor + magic_items, 326);
    }
}
