//! PF1 Core Rulebook weapon stat blocks — the ingestion half of the
//! weapon pillar (task #72, stage 1).
//!
//! Source: every record in `core_rulebook/cr_equip_arms_armor.lst` whose
//! `TYPE:` facet contains `Weapon` and which carries both `DAMAGE:` and
//! `CRITMULT:` — **106 records**, transcribed verbatim from the corpus's
//! own tokens. Not hand-authored; regenerate if the corpus changes.
//!
//! **Why this module exists.** `equipment_data::arms_armor` already
//! carried weapon *identity* (keys and names — `Longsword`, `Rapier`,
//! and so on), but `EquipmentTableEntry` has no damage, crit, or group
//! fields, so nothing downstream could compute from a weapon. That is
//! the gap this table closes: identity was present, stat blocks were
//! not.
//!
//! **`critical_threat_range_width` is a WIDTH, not a low bound.** The
//! corpus's `CRITRANGE:` counts how many natural rolls threaten, so the
//! real threat range is `(21 - width)..=20`. Verified against six
//! published PF1 weapons before anything was built on it: Longsword and
//! Dagger 19-20 (width 2), Scimitar and Rapier 18-20 (width 3),
//! Battleaxe and Greataxe 20 only (width 1). Reading the field as a low
//! bound instead would silently produce nonsense for every weapon.
//!
//! **Proficiency is `Option`, deliberately.** 29 of the 106 carry no
//! Simple/Martial/Exotic facet: the four shields (shield bash), Monk's
//! own Unarmed Strike and Flurry of Blows, the five Improvised Weapon
//! entries, and Touch Attack (Ray Spell). Those are real corpus records
//! with real damage — excluding them would have been a fabricated
//! narrowing, and Unarmed Strike in particular is load-bearing for the
//! already-built Monk chassis.

/// Simple/Martial/Exotic, the PF1 proficiency tiers a weapon's `TYPE:`
/// facet can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponProficiency {
    Simple,
    Martial,
    Exotic,
}

/// One weapon's real corpus stat block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaponTableEntry {
    /// The corpus record's own name, used as the lookup key.
    pub key: &'static str,
    /// Verbatim `DAMAGE:` token for a Medium weapon, e.g. `"1d8"`.
    pub damage_die: &'static str,
    /// `CRITRANGE:` — how many natural rolls threaten a critical. The
    /// threat range is `(21 - width)..=20`; see this module's own doc
    /// comment for why this is a width rather than a low bound.
    pub critical_threat_range_width: u8,
    /// `CRITMULT:` with the leading `x` stripped, e.g. `x3` -> `3`.
    pub critical_multiplier: u8,
    /// `None` for the 29 records with no proficiency facet.
    pub proficiency: Option<WeaponProficiency>,
    /// The `Weapon Group <name>` facet, if present — the grouping
    /// Fighter's Weapon Training keys on.
    pub weapon_group: Option<&'static str>,
    pub is_melee: bool,
    pub is_ranged: bool,
}

/// The lowest natural roll that threatens a critical for this weapon.
/// `21 - width`, so width 1 -> 20, width 2 -> 19, width 3 -> 18.
pub fn weapon_critical_threat_low(entry: &WeaponTableEntry) -> u8 {
    21 - entry.critical_threat_range_width
}

/// Look a weapon up by its exact corpus key.
pub fn weapon_by_key(key: &str) -> Option<&'static WeaponTableEntry> {
    WEAPON_TABLE.iter().find(|entry| entry.key == key)
}

/// Every CRB weapon carrying a real stat block, sorted by key.
pub const WEAPON_TABLE: &[WeaponTableEntry] = &[
    WeaponTableEntry { key: "Armor Spikes", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Natural"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Bastard Sword", damage_die: "1d10", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Battleaxe", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Blowgun", damage_die: "1d2", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Bolas", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Club", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Hammers"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Composite Longbow", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Bows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Composite Shortbow", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Bows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Dagger", damage_die: "1d4", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Dart", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Demon Armor Claw Attack", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Dire Flail", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Double"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Dwarven Urgrosh", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Double"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Dwarven Waraxe", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Elven Curve Blade", damage_die: "1d10", critical_threat_range_width: 3, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Falchion", damage_die: "2d4", critical_threat_range_width: 3, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Flurry of Blows", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Gauntlet", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Glaive", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Polearms"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Gnome Hooked Hammer", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Double"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Greataxe", damage_die: "1d12", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Greatclub", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Hammers"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Greatsword", damage_die: "2d6", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Guisarme", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Polearms"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Halberd", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Polearms"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Halfling Sling Staff", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Hand Crossbow", damage_die: "1d4", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Handaxe", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Crossbow", damage_die: "1d10", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Heavy Flail", damage_die: "1d10", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Mace", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Hammers"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Pick", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 4, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Steel Shield", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Wooden Shield", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d10)", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d12)", damage_die: "1d12", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d2)", damage_die: "1d2", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d3)", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d4)", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d6)", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d8)", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (2d10)", damage_die: "2d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (2d4)", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (2d6)", damage_die: "2d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (2d8)", damage_die: "2d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d10)", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d12)", damage_die: "1d12", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d2)", damage_die: "1d2", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d3)", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d4)", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d6)", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d8)", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (2d10)", damage_die: "2d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (2d4)", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (2d6)", damage_die: "2d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (2d8)", damage_die: "2d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Javelin", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Spears"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Kama", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Kukri", damage_die: "1d4", critical_threat_range_width: 3, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Lance", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Spears"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Crossbow", damage_die: "1d8", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Light Flail", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Hammer", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Hammers"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Light Mace", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Hammers"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Pick", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 4, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Steel Shield", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Wooden Shield", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Longbow", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Bows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Longspear", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Spears"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Longsword", damage_die: "1d8", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Mattock of the Titans", damage_die: "4d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Maul of the Titans", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Morningstar", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Nunchaku", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Orc Double Axe", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Punching Dagger", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Quarterstaff", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Double"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Ranseur", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Polearms"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Rapier", damage_die: "1d6", critical_threat_range_width: 3, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Repeating Heavy Crossbow", damage_die: "1d10", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Repeating Light Crossbow", damage_die: "1d8", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Sai", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Monk"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Sap", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Scimitar", damage_die: "1d6", critical_threat_range_width: 3, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Scythe", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 4, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Shieldbash (Heavy Shield)", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Shieldbash (Light Shield)", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Short Sword", damage_die: "1d6", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Shortbow", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Bows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Shortspear", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Spears"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Shuriken", damage_die: "1d2", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Monk"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Siangham", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Sickle", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Sling", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Spear", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Spears"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Spiked Armor", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Spiked Chain", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Spiked Gauntlet", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Starknife", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Throwing Axe", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Touch Attack (Ray Spell)", damage_die: "0", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Trident", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Spears"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Two-Bladed Sword", damage_die: "1d8", critical_threat_range_width: 2, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Unarmed Strike", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Warhammer", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Hammers"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Whip", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_matches_the_verified_corpus_extraction() {
        assert_eq!(WEAPON_TABLE.len(), 106, "106 CRB weapon records carry DAMAGE + CRITMULT");
        let melee = WEAPON_TABLE.iter().filter(|w| w.is_melee).count();
        let ranged = WEAPON_TABLE.iter().filter(|w| w.is_ranged).count();
        assert_eq!((melee, ranged), (78, 36), "some records are both (thrown)");
    }

    #[test]
    fn proficiency_tiers_match_the_corpus_facets() {
        let count = |p: Option<WeaponProficiency>| {
            WEAPON_TABLE.iter().filter(|w| w.proficiency == p).count()
        };
        assert_eq!(count(Some(WeaponProficiency::Simple)), 20);
        assert_eq!(count(Some(WeaponProficiency::Martial)), 39);
        assert_eq!(count(Some(WeaponProficiency::Exotic)), 18);
        assert_eq!(count(None), 29, "shields, unarmed/flurry, improvised, ray touch");
    }

    /// The six weapons whose published PF1 stats were checked directly
    /// against the corpus before this table was built. These pin both
    /// the damage die and -- critically -- that CRITRANGE is a WIDTH.
    #[test]
    fn spot_checked_weapons_match_their_published_pf1_stats() {
        for (key, die, low, mult) in [
            ("Longsword", "1d8", 19u8, 2u8),
            ("Dagger", "1d4", 19, 2),
            ("Scimitar", "1d6", 18, 2),
            ("Rapier", "1d6", 18, 2),
            ("Battleaxe", "1d8", 20, 3),
            ("Greataxe", "1d12", 20, 3),
        ] {
            let w = weapon_by_key(key).unwrap_or_else(|| panic!("{key} must be in the table"));
            assert_eq!(w.damage_die, die, "{key} damage");
            assert_eq!(weapon_critical_threat_low(w), low, "{key} threat range low");
            assert_eq!(w.critical_multiplier, mult, "{key} crit multiplier");
        }
    }

    /// Guards the width-vs-low-bound reading specifically. If the field
    /// were ever reinterpreted as a low bound, a Longsword would report
    /// a threat range starting at 2 and this fails loudly.
    #[test]
    fn the_threat_range_is_derived_from_a_width_not_a_low_bound() {
        let longsword = weapon_by_key("Longsword").expect("Longsword");
        assert_eq!(longsword.critical_threat_range_width, 2, "raw corpus CRITRANGE");
        assert_eq!(weapon_critical_threat_low(longsword), 19, "21 - 2");
        assert_ne!(
            u8::from(longsword.critical_threat_range_width),
            weapon_critical_threat_low(longsword),
            "width and low bound must not be conflated"
        );
    }

    /// Monk's own weapons are present and untiered -- they are why the
    /// 29 untiered records were kept rather than filtered out.
    #[test]
    fn monk_unarmed_weapons_are_present_and_untiered() {
        for key in ["Unarmed Strike", "Flurry of Blows"] {
            let w = weapon_by_key(key).unwrap_or_else(|| panic!("{key} must be present"));
            assert_eq!(w.proficiency, None, "{key} carries no proficiency facet");
            assert!(w.is_melee, "{key} is melee");
        }
    }

    #[test]
    fn every_entry_has_a_sane_stat_block() {
        for w in WEAPON_TABLE {
            assert!(!w.damage_die.is_empty(), "{} has no damage die", w.key);
            assert!(
                (1..=3).contains(&w.critical_threat_range_width),
                "{} threat width {} out of the real 1..=3 range",
                w.key,
                w.critical_threat_range_width
            );
            assert!(
                (2..=4).contains(&w.critical_multiplier),
                "{} crit multiplier {} out of the real x2..x4 range",
                w.key,
                w.critical_multiplier
            );
            assert!(w.is_melee || w.is_ranged, "{} is neither melee nor ranged", w.key);
        }
    }

    #[test]
    fn keys_are_unique_and_lookup_rejects_unknown_weapons() {
        let mut keys: Vec<&str> = WEAPON_TABLE.iter().map(|w| w.key).collect();
        let n = keys.len();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), n, "duplicate weapon keys");
        assert!(weapon_by_key("Lightsaber").is_none());
    }
}
