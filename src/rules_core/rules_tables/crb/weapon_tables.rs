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
    /// The weapon's `PROFICIENCY:WEAPON|` token — **a separate namespace
    /// from `key`, and the only correct thing to match a class's
    /// `AUTO:WEAPONPROF|` list against.**
    ///
    /// 58 of the 106 differ from the display key, so matching on `key`
    /// would be wrong for more than half the table: the corpus writes
    /// `Heavy Crossbow` as a display name but `Crossbow (Heavy)` as a
    /// proficiency, `Short Sword` as `Sword (Short)`, `Bastard Sword` as
    /// `Sword (Bastard)`. Wizard's own list names `Crossbow (Heavy)`, so a
    /// key-based join would have reported a Wizard as NOT proficient with
    /// the crossbow it is explicitly granted — the exact inverse of the
    /// bug this field exists to fix.
    ///
    /// It also collapses variants that share one proficiency: both
    /// Composite Longbow and Longbow are `Longbow`, and every Improvised
    /// Weapon size is `Improvised Weapon`.
    ///
    /// `None` for the four shields, which carry no `PROFICIENCY:WEAPON`
    /// token at all — shield bash proficiency comes from shield
    /// proficiency, which is a different mechanic this does not model.
    pub proficiency_name: Option<&'static str>,
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
    WeaponTableEntry { key: "Armor Spikes", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Spiked Armor"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Natural"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Bastard Sword", damage_die: "1d10", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Sword (Bastard)"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Battleaxe", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Battleaxe"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Blowgun", damage_die: "1d2", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Blowgun"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Bolas", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Bolas"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Club", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Club"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Hammers"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Composite Longbow", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Longbow"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Bows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Composite Shortbow", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Shortbow"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Bows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Dagger", damage_die: "1d4", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Dagger"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Dart", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Dart"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Demon Armor Claw Attack", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Unarmed Strike"), proficiency: Some(WeaponProficiency::Simple), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Dire Flail", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Flail (Dire)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Double"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Dwarven Urgrosh", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Urgrosh (Dwarven)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Double"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Dwarven Waraxe", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Waraxe (Dwarven)"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Elven Curve Blade", damage_die: "1d10", critical_threat_range_width: 3, critical_multiplier: 2, proficiency_name: Some("Curve Blade (Elven)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Falchion", damage_die: "2d4", critical_threat_range_width: 3, critical_multiplier: 2, proficiency_name: Some("Falchion"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Flurry of Blows", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Unarmed Strike"), proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Gauntlet", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Gauntlet"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Glaive", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Glaive"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Polearms"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Gnome Hooked Hammer", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Hammer (Gnome Hooked)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Double"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Greataxe", damage_die: "1d12", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Greataxe"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Greatclub", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Greatclub"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Hammers"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Greatsword", damage_die: "2d6", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Greatsword"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Guisarme", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Guisarme"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Polearms"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Halberd", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Halberd"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Polearms"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Halfling Sling Staff", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Sling Staff (Halfling)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Hand Crossbow", damage_die: "1d4", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Crossbow (Hand)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Handaxe", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Handaxe"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Crossbow", damage_die: "1d10", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Crossbow (Heavy)"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Heavy Flail", damage_die: "1d10", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Flail (Heavy)"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Mace", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Mace (Heavy)"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Hammers"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Pick", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 4, proficiency_name: Some("Pick (Heavy)"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Steel Shield", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: None, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Heavy Wooden Shield", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: None, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d10)", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d12)", damage_die: "1d12", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d2)", damage_die: "1d2", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d3)", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d4)", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d6)", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (1d8)", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (2d10)", damage_die: "2d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (2d4)", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (2d6)", damage_die: "2d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (2d8)", damage_die: "2d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon"), proficiency: None, weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d10)", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d12)", damage_die: "1d12", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d2)", damage_die: "1d2", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d3)", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d4)", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d6)", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (1d8)", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (2d10)", damage_die: "2d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (2d4)", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (2d6)", damage_die: "2d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Improvised Weapon (Thrown) (2d8)", damage_die: "2d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Improvised Weapon (Thrown)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Javelin", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Javelin"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Spears"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Kama", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Kama"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Kukri", damage_die: "1d4", critical_threat_range_width: 3, critical_multiplier: 2, proficiency_name: Some("Kukri"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Lance", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Lance"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Spears"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Crossbow", damage_die: "1d8", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Crossbow (Light)"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Light Flail", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Flail"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Hammer", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Hammer (Light)"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Hammers"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Light Mace", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Mace (Light)"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Hammers"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Pick", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 4, proficiency_name: Some("Pick (Light)"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Steel Shield", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: None, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Light Wooden Shield", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: None, proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Longbow", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Longbow"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Bows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Longspear", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Longspear"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Spears"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Longsword", damage_die: "1d8", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Longsword"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Mattock of the Titans", damage_die: "4d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Warhammer"), proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Maul of the Titans", damage_die: "1d10", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Greatclub"), proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Morningstar", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Morningstar"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Nunchaku", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Nunchaku"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Orc Double Axe", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Axe (Orc Double)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Axes"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Punching Dagger", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Dagger (Punching)"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Quarterstaff", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Quarterstaff"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Double"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Ranseur", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Ranseur"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Polearms"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Rapier", damage_die: "1d6", critical_threat_range_width: 3, critical_multiplier: 2, proficiency_name: Some("Rapier"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Repeating Heavy Crossbow", damage_die: "1d10", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Crossbow (Repeating Heavy)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Repeating Light Crossbow", damage_die: "1d8", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Crossbow (Repeating Light)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Crossbows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Sai", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Sai"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Monk"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Sap", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Sap"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Scimitar", damage_die: "1d6", critical_threat_range_width: 3, critical_multiplier: 2, proficiency_name: Some("Scimitar"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Scythe", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 4, proficiency_name: Some("Scythe"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Shieldbash (Heavy Shield)", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Shieldbash"), proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Shieldbash (Light Shield)", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Shieldbash"), proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Short Sword", damage_die: "1d6", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Sword (Short)"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Shortbow", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Shortbow"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Bows"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Shortspear", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Shortspear"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Spears"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Shuriken", damage_die: "1d2", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Shuriken"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Monk"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Siangham", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Siangham"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Sickle", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Sickle"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Sling", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Sling"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Thrown"), is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Spear", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Spear"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Spears"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Spiked Armor", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Spiked Armor"), proficiency: Some(WeaponProficiency::Martial), weapon_group: None, is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Spiked Chain", damage_die: "2d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Chain (Spiked)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Spiked Gauntlet", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Gauntlet (Spiked)"), proficiency: Some(WeaponProficiency::Simple), weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Starknife", damage_die: "1d4", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Starknife"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Blades Light"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Throwing Axe", damage_die: "1d6", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Axe (Throwing)"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Axes"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Touch Attack (Ray Spell)", damage_die: "0", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Spells (Ray)"), proficiency: None, weapon_group: None, is_melee: false, is_ranged: true },
    WeaponTableEntry { key: "Trident", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Trident"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Spears"), is_melee: true, is_ranged: true },
    WeaponTableEntry { key: "Two-Bladed Sword", damage_die: "1d8", critical_threat_range_width: 2, critical_multiplier: 2, proficiency_name: Some("Sword (Two-Bladed)"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Blades Heavy"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Unarmed Strike", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Unarmed Strike"), proficiency: None, weapon_group: Some("Close"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Warhammer", damage_die: "1d8", critical_threat_range_width: 1, critical_multiplier: 3, proficiency_name: Some("Warhammer"), proficiency: Some(WeaponProficiency::Martial), weapon_group: Some("Hammers"), is_melee: true, is_ranged: false },
    WeaponTableEntry { key: "Whip", damage_die: "1d3", critical_threat_range_width: 1, critical_multiplier: 2, proficiency_name: Some("Whip"), proficiency: Some(WeaponProficiency::Exotic), weapon_group: Some("Flails"), is_melee: true, is_ranged: false },
];

/// Every CRB weapon whose corpus `TYPE:` facet contains `Finesseable` --
/// the exact set Weapon Finesse applies to, and the only correct way to
/// answer "may this weapon use Dexterity on attack rolls".
///
/// **Read from the corpus facet, never re-derived from the PF1 prose.**
/// The rulebook says "a light weapon, rapier, whip, or spiked chain",
/// which is both incomplete and misleading against the data: the facet
/// also carries Elven Curve Blade (an Exotic two-handed weapon), Unarmed
/// Strike, Flurry of Blows and Shieldbash (Light Shield), and it carries
/// them individually rather than through any `Light` grouping this table
/// records. Deriving the list from "weapons this table calls light" is not
/// even possible here -- `WeaponTableEntry` has no light facet at all.
///
/// **Two collisions in this list are real and were checked.**
///  1. `Spiked Armor` and `Armor Spikes` are two separate corpus records
///     that share one `PROFICIENCY:WEAPON|Spiked Armor` name. Only
///     `Spiked Armor` carries `Finesseable`; `Armor Spikes` does not, so
///     matching on the shared proficiency name instead of the display key
///     would wrongly finesse it.
///  2. The corpus's 27th `Finesseable` record is
///     `Bastard Sword (Base).COPY=Sun Blade (Bastard Sword)` -- a `.COPY`
///     record minting a specific MAGIC ITEM (the Sun Blade), not the base
///     Bastard Sword, which is not finesseable and is not in this list.
///     `Sun Blade` is not in `WEAPON_TABLE`, so the 27 corpus records fold
///     to these 26 keys, each verified present above.
///
/// Source: `core_rulebook/cr_equip_arms_armor.lst`, every record whose
/// `TYPE:` facet contains `Finesseable`.
pub const FINESSEABLE_WEAPON_KEYS: &[&str] = &[
    "Dagger",
    "Elven Curve Blade",
    "Flurry of Blows",
    "Gauntlet",
    "Handaxe",
    "Kama",
    "Kukri",
    "Light Hammer",
    "Light Mace",
    "Light Pick",
    "Nunchaku",
    "Punching Dagger",
    "Rapier",
    "Sai",
    "Sap",
    "Shieldbash (Light Shield)",
    "Short Sword",
    "Siangham",
    "Sickle",
    "Spiked Armor",
    "Spiked Chain",
    "Spiked Gauntlet",
    "Starknife",
    "Throwing Axe",
    "Unarmed Strike",
    "Whip",
];

/// Whether Weapon Finesse can apply to this weapon. Matched on the display
/// `key`, not `proficiency_name` -- see [`FINESSEABLE_WEAPON_KEYS`] for the
/// Spiked Armor / Armor Spikes collision that makes the distinction
/// load-bearing.
pub fn weapon_is_finesseable(entry: &WeaponTableEntry) -> bool {
    FINESSEABLE_WEAPON_KEYS.contains(&entry.key)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every finesseable key must resolve to a real row -- a typo here
    /// would silently mean "this weapon is never finesseable".
    #[test]
    fn every_finesseable_key_resolves_to_a_real_weapon() {
        assert_eq!(FINESSEABLE_WEAPON_KEYS.len(), 26);
        for key in FINESSEABLE_WEAPON_KEYS {
            let entry = weapon_by_key(key)
                .unwrap_or_else(|| panic!("{key} is not a real WEAPON_TABLE key"));
            assert!(entry.is_melee, "{key} must be usable in melee to be finessed");
            assert!(weapon_is_finesseable(entry), "{key} must answer true");
        }
    }

    /// The collision the list's doc comment exists to prevent: two records
    /// share one proficiency name, and only one of them is finesseable.
    #[test]
    fn spiked_armor_is_finesseable_but_armor_spikes_is_not() {
        let spiked_armor = weapon_by_key("Spiked Armor").expect("present");
        let armor_spikes = weapon_by_key("Armor Spikes").expect("present");
        assert_eq!(
            spiked_armor.proficiency_name, armor_spikes.proficiency_name,
            "the two share one proficiency name, which is why key-matching matters"
        );
        assert!(weapon_is_finesseable(spiked_armor));
        assert!(
            !weapon_is_finesseable(armor_spikes),
            "Armor Spikes carries no Finesseable facet in the corpus"
        );
    }

    /// The base Bastard Sword is not finesseable -- only the Sun Blade
    /// `.COPY` record derived from it is, and that is a magic item this
    /// table does not carry.
    #[test]
    fn heavy_weapons_are_not_finesseable() {
        for key in ["Bastard Sword", "Longsword", "Greatsword", "Battleaxe"] {
            assert!(
                !weapon_is_finesseable(weapon_by_key(key).expect("present")),
                "{key} must not be finesseable"
            );
        }
    }

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
            longsword.critical_threat_range_width,
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

/// One class's weapon proficiency: the blanket tiers it is granted, plus
/// the individually named weapons on top.
///
/// **Both halves are load-bearing — a tier-only model is wrong.** Druid,
/// Monk and Wizard receive NO blanket Simple grant: their records carry
/// only `Weapon Prof ~ Auto` plus an explicit `AUTO:WEAPONPROF|` list.
/// Modelling them as "simple-weapon classes" would hand a Wizard every
/// simple weapon in the book when the corpus grants it exactly five.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassWeaponProficiency {
    /// The engine's class id, e.g. `"class:wizard"`.
    pub class_id: &'static str,
    /// Blanket tiers, from `ABILITY:Internal|AUTOMATIC|Weapon Prof ~ X`
    /// (which resolves to `AUTO:WEAPONPROF|TYPE=X`) and the `.MOD` grants.
    pub tiers: &'static [WeaponProficiency],
    /// Individually named proficiencies, verbatim from the class's own
    /// `AUTO:WEAPONPROF|` list. These are `PROFICIENCY:WEAPON` names, NOT
    /// weapon display keys — see [`WeaponTableEntry::proficiency_name`].
    pub named: &'static [&'static str],
    /// Whole `Weapon Group <name>` grants. Only Brawler uses this among the
    /// 27 base classes, but omitting it would have silently understated
    /// Brawler's proficiency for every Close-group weapon.
    pub weapon_groups: &'static [&'static str],
}

/// Weapon proficiency for the eleven CRB base classes.
///
/// **Every class here has a real corpus record.** An earlier survey
/// concluded seven of them had none; that was a grep artifact. The corpus
/// uses two key conventions for the same feature —
/// `KEY:Barbarian ~ Weapon and Armor Proficiency` for some and
/// `KEY:Weapon and Armor Proficiency ~ Fighter` for others — so a search
/// for one form silently misses the other.
///
/// The grants themselves sit in three places, all of which had to be read
/// to get Fighter right:
///  1. `AUTO:WEAPONPROF|<list>` directly on the record (Wizard, Monk,
///     Druid, Rogue, Bard).
///  2. `ABILITY:Internal|AUTOMATIC|Weapon Prof ~ Simple`, an indirection
///     whose target carries `AUTO:WEAPONPROF|TYPE=Simple`.
///  3. `.MOD` records — `CATEGORY=Class|Fighter.MOD` grants Martial,
///     gated on not having an archetype that replaces it.
///
/// Archetype replacement is NOT modelled: every grant here is the
/// unarchetyped base class, which is all this engine composes.
pub const CLASS_WEAPON_PROFICIENCIES: &[ClassWeaponProficiency] = &[
    ClassWeaponProficiency { class_id: "class:alchemist", tiers: &[WeaponProficiency::Simple], named: &["Bomb"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:arcanist", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:barbarian", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:bard", tiers: &[WeaponProficiency::Simple], named: &["Longsword", "Rapier", "Sap", "Sword (Short)", "Shortbow", "Whip"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:bloodrager", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    // Brawler is the only base class granted a whole weapon GROUP
    // ("Close"), which is why `weapon_groups` exists at all.
    ClassWeaponProficiency { class_id: "class:brawler", tiers: &[WeaponProficiency::Simple], named: &["Handaxe", "Sword (Short)"], weapon_groups: &["Close"] },
    ClassWeaponProficiency { class_id: "class:cavalier", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:cleric", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:druid", tiers: &[], named: &["Club", "Dagger", "Dart", "Quarterstaff", "Scimitar", "Scythe", "Sickle", "Shortspear", "Sling", "Spear"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:fighter", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:hunter", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:inquisitor", tiers: &[WeaponProficiency::Simple], named: &["Crossbow (Hand)", "Longbow", "Crossbow (Repeating Heavy)", "Crossbow (Repeating Light)", "Shortbow"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:investigator", tiers: &[WeaponProficiency::Simple], named: &["Crossbow (Hand)", "Rapier", "Sap", "Shortbow", "Sword (Short)", "Sword Cane"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:monk", tiers: &[], named: &["Club", "Crossbow (Light)", "Crossbow (Heavy)", "Dagger", "Handaxe", "Javelin", "Kama", "Nunchaku", "Quarterstaff", "Sai", "Shortspear", "Sword (Short)", "Shuriken", "Siangham", "Sling", "Spear", "Unarmed Strike"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:oracle", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:paladin", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:ranger", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:rogue", tiers: &[WeaponProficiency::Simple], named: &["Crossbow (Hand)", "Rapier", "Sap", "Shortbow", "Sword (Short)"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:shaman", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:skald", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:slayer", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:sorcerer", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:summoner", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:swashbuckler", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:warpriest", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:witch", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:wizard", tiers: &[], named: &["Club", "Dagger", "Crossbow (Heavy)", "Crossbow (Light)", "Quarterstaff"], weapon_groups: &[] },
    // SD-27 (2026-07-31): Pathfinder Unchained's four classes. Each row is
    // transcribed from that class's OWN proficiency record under
    // `data/corpus/pathfinder_unchained/class_feature/<class>/`, never
    // copied across from the class it replaces -- three of the four happen
    // to come out identical to their namesake, and the fourth does not,
    // which is exactly why they were read separately.
    //
    //  - Unchained Barbarian (`~ Weapon and Armor Proficiency`, p.8) grants
    //    `ABILITY:Internal|AUTOMATIC|Weapon Prof ~ Simple` and
    //    `... ~ Martial` -- convention 2 above. Same as CRB Barbarian.
    //  - Unchained Rogue (`~ Weapon Proficiency`) carries both
    //    `AUTO:WEAPONPROF|Crossbow (Hand)|Rapier|Sap|Shortbow|Sword (Short)`
    //    and `ABILITY:Internal|AUTOMATIC|Weapon Prof ~ Simple`. Same as CRB
    //    Rogue.
    //  - Unchained Summoner (`~ Weapon and Armor Proficiency`, p.25) grants
    //    `ABILITY:Internal|AUTOMATIC|TYPE=WeaponProfSimple`. Same as APG
    //    Summoner.
    //  - Unchained Monk (`~ Weapon and Armor Proficiency`, p.14) is the one
    //    that differs, and the difference is transcribed rather than
    //    smoothed over. Its token is
    //    `AUTO:WEAPONPROF|Club|Crossbow (Light)|Crossbow (Heavy)|Dagger|Handaxe|Javelin|Kama|Nunchaku|Quarterstaff|Sai|Sword (Short)|Shortspear|Shuriken|Siangham|Sling|Spear|TYPE=Monk`
    //    -- SIXTEEN named weapons where the CRB Monk row above carries
    //    seventeen. PU's token does NOT name `Unarmed Strike`; CRB's does.
    //    Two consequences, both stated rather than papered over:
    //      (a) `TYPE=Monk` is a PCGen weapon *type* selector. This table
    //          models tiers, named weapons and `Weapon Group <name>`, and a
    //          weapon type is none of those, so the clause is NOT modelled.
    //          Its practical cost today is nil: every weapon in the ingested
    //          CRB table that carries the monk quality (Kama, Nunchaku, Sai,
    //          Shuriken, Siangham) is already on the named list above.
    //          Conflating it with the `Monk` weapon *group* Sai and Shuriken
    //          carry would be inventing a mapping, so it is not done.
    //      (b) unarmed-strike proficiency for this class does not come from
    //          the weapon-proficiency lane at all: `Unchained Monk ~ Unarmed
    //          Strike` grants `ABILITY:FEAT|VIRTUAL|Improved Unarmed Strike`.
    //          Virtual feat grants from class features are not modelled in
    //          this engine, so an Unchained Monk reads as non-proficient with
    //          an Unarmed Strike here. That is a real gap, recorded in the
    //          class's own `class_feature.pu.unchained_monk.
    //          other_features_deferred` diagnostic.
    ClassWeaponProficiency { class_id: "class:unchained_barbarian", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:unchained_monk", tiers: &[], named: &["Club", "Crossbow (Light)", "Crossbow (Heavy)", "Dagger", "Handaxe", "Javelin", "Kama", "Nunchaku", "Quarterstaff", "Sai", "Sword (Short)", "Shortspear", "Shuriken", "Siangham", "Sling", "Spear"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:unchained_rogue", tiers: &[WeaponProficiency::Simple], named: &["Crossbow (Hand)", "Rapier", "Sap", "Shortbow", "Sword (Short)"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:unchained_summoner", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    // SD-31 wave 20 (chassis-coverage lane): Ultimate Combat's Gunslinger
    // (`uc_abilities_class.lst`, `KEY:Gunslinger ~ Proficiencies`) grants
    // `ABILITY:Internal|AUTOMATIC|TYPE=WeaponProfMartial|TYPE=ArmorProfLight`
    // plus a second `ABILITY:Internal|AUTOMATIC|Weapon Prof ~ Auto|Weapon
    // Prof ~ Simple|...` indirection -- the same convention-2 shape Fighter
    // and Barbarian's own Simple+Martial rows already use. That same record
    // also carries `AUTO:WEAPONPROF|TYPE=Firearm`: a weapon TYPE selector,
    // not a tier/named-weapon/weapon-group this table models, exactly the
    // documented boundary the Unchained Monk's `TYPE=Monk` selector already
    // states above -- deliberately NOT claimed here. Ninja and Samurai
    // (Ultimate Combat's other two classes) are NOT added by this same
    // cycle: Ninja's own proficiency record grants a named-weapon list with
    // no visible Simple-tier token on the same row (its DESC prose says
    // "proficient with all simple weapons" but the ingested corpus token
    // does not carry a matching `AUTO:WEAPONPROF|TYPE=Simple`/indirection,
    // and this table's own discipline is to transcribe the token, not the
    // prose), and Samurai's record is `AUTO:WEAPONPROF|TYPE=Samurai`, a
    // weapon TYPE selector this table has no representation for at all
    // (unlike Gunslinger's Firearm gap, dropping it would leave Samurai with
    // ZERO named/tier coverage, which is not the same known-boundary shape).
    // Both are real, open, honestly-reported gaps for a future cycle.
    ClassWeaponProficiency { class_id: "class:gunslinger", tiers: &[WeaponProficiency::Simple, WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    // SD-34 wave 33 lane C (`class_modelled_but_no_observed_delta_on_the_
    // rendered_snapshot`): nine classes `untabled_base_class_chassis`/
    // `crb_untabled_class_chassis` already compute a real BAB/save chassis
    // for, but which `has_supported_class_chassis` did not yet recognize
    // (fixed the same cycle, `pilot_compute/mod.rs`) -- and which this
    // table had never carried a row for at all, so `class_weapon_
    // proficiency` returned `None` and `combat.baseline_weapon_
    // proficiency_unknown` claim-blocked them regardless of the chassis
    // gate. Each row transcribes the class's OWN `data/corpus/<book>/
    // class_feature/<class>/weapon_and_armor_proficiency*.json` (or, for
    // Psion, `.../psion_weapon_proficiencies/psion_weapon_proficiencies.json`)
    // token -- never the DESC prose alone, same discipline as every row
    // above.
    //
    //  - Kineticist (`occult_adventures`): `TYPE=WeaponProfSimple` -> Simple
    //    tier only, no named weapons.
    //  - Medium (`occult_adventures`): `Weapon Prof ~ Auto|Weapon Prof ~
    //    Simple` indirection -> Simple tier only, the same convention-2
    //    shape Fighter/Barbarian use.
    //  - Mesmerist (`occult_adventures`): Simple tier plus
    //    `AUTO:WEAPONPROF|Crossbow (Hand)|Sap|Sword Cane|Whip`.
    //  - Occultist (`occult_adventures`) and Vigilante (`ultimate_intrigue`):
    //    the token carries ONLY `TYPE=WeaponProfMartial`, even though both
    //    classes' own DESC says "simple and martial" -- the Ninja/Samurai
    //    boundary above applies identically here: transcribe the token, not
    //    the prose, so Simple is deliberately NOT added for either. This
    //    does not affect the Longsword question below (Longsword is a
    //    Martial-tier weapon either way).
    //  - Psychic (`occult_adventures`): `TYPE=WeaponProfSimple` -> Simple
    //    tier only, no named weapons or armor/shield token at all (its own
    //    DESC states "not with any type of armor or shield").
    //  - Spiritualist (`occult_adventures`): Simple tier plus
    //    `AUTO:WEAPONPROF|Kukri|Sap|Scythe`.
    //  - Psion (`ultimate_psionics`): no tier token at all, only
    //    `AUTO:WEAPONPROF|Club|Dagger|Crossbow (Heavy)|Crossbow (Light)|
    //    Quarterstaff|Shortspear` -- the same named-only shape as Wizard's
    //    row above, one weapon (Shortspear) wider.
    //  - Shifter (`ultimate_wilderness`): no tier token, only
    //    `AUTO:WEAPONPROF|Club|Dagger|Dart|Quarterstaff|Scimitar|Scythe|
    //    Sickle|Shortspear|Sling|Spear` (its DESC's natural-attack
    //    proficiency clause is not a weapon-table entry and is not modelled
    //    here).
    //
    // Aegis, Antipaladin, Cryptic, Dread, Magus, Marksman, Soulknife,
    // Tactician (`ultimate_psionics`'s base class, distinct from Ultimate
    // Combat's Tactician fighter archetype -- a corpus identifier scope
    // collision checked and avoided), Vitalist, Wilder (`untabled_base_
    // class_chassis`'s remaining ten), and the seven CRB NPC/Ex-* classes
    // (`crb_untabled_class_chassis`) carry no matching corpus proficiency
    // record found this cycle and are NOT added -- a real, open,
    // honestly-reported gap for a future cycle, not silently assumed
    // Simple-only.
    ClassWeaponProficiency { class_id: "class:kineticist", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:medium", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:mesmerist", tiers: &[WeaponProficiency::Simple], named: &["Crossbow (Hand)", "Sap", "Sword Cane", "Whip"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:occultist", tiers: &[WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:vigilante", tiers: &[WeaponProficiency::Martial], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:psychic", tiers: &[WeaponProficiency::Simple], named: &[], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:spiritualist", tiers: &[WeaponProficiency::Simple], named: &["Kukri", "Sap", "Scythe"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:psion", tiers: &[], named: &["Club", "Dagger", "Crossbow (Heavy)", "Crossbow (Light)", "Quarterstaff", "Shortspear"], weapon_groups: &[] },
    ClassWeaponProficiency { class_id: "class:shifter", tiers: &[], named: &["Club", "Dagger", "Dart", "Quarterstaff", "Scimitar", "Scythe", "Sickle", "Shortspear", "Sling", "Spear"], weapon_groups: &[] },
    // SD-34 wave 34 lane C: this cycle's own re-derive of wave 33 lane C's
    // named 19-unit remainder (`docs/release/SD-34-book-completion/
    // artifacts/bucket-d-mining/wave33_laneC_class_snapshot_delta_cycle_
    // receipt.md`'s Next-cycle plan item 1), checked per-class against
    // `data/corpus/**/class_feature/<class>/` individually, not batch-
    // assumed:
    //
    //  - **17 of 19 re-confirmed genuinely absent, no row added.** The 10
    //    untabled base classes (Aegis, Antipaladin, Cryptic, Dread, Magus,
    //    Marksman, Soulknife, Tactician [`ultimate_psionics`'s base class,
    //    not Ultimate Combat's Tactician fighter archetype], Vitalist,
    //    Wilder) and the 7 CRB NPC/`Ex-*` classes (Adept, Aristocrat,
    //    Commoner, Expert, Warrior, Ex-Barbarian, Ex-Paladin) still carry
    //    no `AUTO:WEAPONPROF` token anywhere under their own corpus
    //    directories this cycle's own search found (Magus carries a real
    //    `armor_proficiency.json` but no weapon-proficiency record at all;
    //    Ex-Barbarian and Ex-Paladin carry no `class_feature/` directory of
    //    their own whatsoever). Real, open, honestly-reported, unchanged
    //    from wave 33 lane C's own count.
    //  - **Samurai: a real record exists but carries nothing this table
    //    can represent, so no row is added either.**
    //    `ultimate_combat/class_feature/samurai_proficiencies/
    //    samurai_proficiencies.json` carries exactly `AUTO:WEAPONPROF|
    //    TYPE=Samurai` -- a weapon TYPE selector, not a Simple/Martial/
    //    Exotic tier, named weapon, or weapon group this table's schema
    //    models -- plus a virtual `Exotic Weapon Proficiency (Katana)` feat
    //    grant, which virtual-feat grants from class features are not
    //    modelled anywhere in this engine either (the same boundary
    //    Unchained Monk's unarmed-strike virtual feat already documents
    //    above). An all-empty row would be indistinguishable from a real
    //    "proficient with nothing" claim and is deliberately not added --
    //    the same reasoning wave 33 lane C's own comment already gave for
    //    leaving this one open.
    //  - **Ninja: closed.** `ultimate_combat/class_feature/ninja/
    //    ninja_weapon_proficiencies.json` carries a real `AUTO:WEAPONPROF|
    //    Shortbow|Sword (Short)|Kama|Kusarigama (Sickle and Chain)|
    //    Nunchaku|Sai|Shuriken|Siangham|Wakizashi` token -- transcribed
    //    below in full. Its DESC additionally claims blanket Simple-weapon
    //    proficiency ("proficient with all simple weapons"), but no
    //    matching `TYPE=WeaponProfSimple`/indirection token exists on this
    //    record, so Simple is deliberately NOT added -- the identical
    //    boundary this file already applies to Occultist/Vigilante's own
    //    "simple and martial" DESC vs. Martial-only token (wave 33 lane
    //    C's own comment above named this as the exact boundary a future
    //    Ninja row would need). The virtual `Exotic Weapon Proficiency
    //    (Katana)` feat grant is likewise not modelled, the same boundary
    //    as Samurai's above. This is a real, honest, partial
    //    transcription, not a complete proficiency list -- but it resolves
    //    correctly for THIS table's one live consumer
    //    (`character_is_proficient_with`, always checked against the
    //    Longsword): Longsword is Martial-tier and not on Ninja's named
    //    list either way, so the Longsword nonproficiency verdict below is
    //    correct regardless of the missing Simple tier. Kusarigama (Sickle
    //    and Chain) and Wakizashi are real Ultimate Combat weapons this
    //    CRB-only table has no stat block for -- the same scope boundary
    //    `OUTSIDE_THE_CRB_WEAPON_TABLE` already carries for Mesmerist's
    //    Sword Cane, extended below rather than silently dropping either
    //    name.
    ClassWeaponProficiency { class_id: "class:ninja", tiers: &[], named: &["Shortbow", "Sword (Short)", "Kama", "Kusarigama (Sickle and Chain)", "Nunchaku", "Sai", "Shuriken", "Siangham", "Wakizashi"], weapon_groups: &[] },
];

/// This class's weapon proficiency, or `None` for a class this table does
/// not cover.
///
/// `None` means "not ingested", NOT "proficient with nothing". Callers
/// must treat it as unknown and refuse to assert a proficiency-dependent
/// number, rather than defaulting either way.
pub fn class_weapon_proficiency(class_id: &str) -> Option<&'static ClassWeaponProficiency> {
    CLASS_WEAPON_PROFICIENCIES.iter().find(|entry| entry.class_id == class_id)
}

/// Whether a class is proficient with a weapon.
///
/// Matches the weapon's `proficiency_name` against the class's named list
/// first, then its tier against the class's blanket tiers. A weapon with
/// no `proficiency_name` (the four shields) can only ever match on tier,
/// and no CRB class grants a tier those shields carry, so they come back
/// non-proficient — correct for weapon proficiency, since shield bash
/// proficiency is a separate mechanic.
pub fn class_is_proficient_with(
    proficiency: &ClassWeaponProficiency,
    weapon: &WeaponTableEntry,
) -> bool {
    if let Some(name) = weapon.proficiency_name
        && proficiency.named.contains(&name)
    {
        return true;
    }
    if let Some(group) = weapon.weapon_group
        && proficiency.weapon_groups.contains(&group)
    {
        return true;
    }
    match weapon.proficiency {
        Some(tier) => proficiency.tiers.contains(&tier),
        None => false,
    }
}

/// `AT-34-E3-001` (`class_feature_option_pool_record_not_held_by_engine`
/// mechanism, cycle 6, armor/shield-flavored slice of the proficiency/
/// mechanical-grant possession-tracking sub-cause cycle 5's own next-cycle
/// plan named). One class's armor/shield proficiency, read the same way
/// [`ClassWeaponProficiency`] is: each field transcribed from that class's
/// own `cr_abilities_class.lst` `"Weapon and Armor Proficiency ~ <Class>"`
/// record's literal `ABILITY:Internal|AUTOMATIC|Armor Prof ~ <Tier>` /
/// `Shield Prof` / `Shield Prof ~ Tower` indirection targets -- never a
/// shape guess. PF1 armor proficiency has no per-item exotic-armor
/// analogue to a weapon's named list, so a class's whole grant is exactly
/// these five booleans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassArmorProficiency {
    /// The engine's class id, e.g. `"class:fighter"`.
    pub class_id: &'static str,
    pub light: bool,
    pub medium: bool,
    pub heavy: bool,
    /// Buckler/Light Shield/Heavy Shield (`Shield Prof`'s own grant, NOT
    /// the tower shield).
    pub shield: bool,
    pub tower_shield: bool,
}

/// Armor/shield proficiency for the five CRB base classes whose own
/// `"Weapon and Armor Proficiency ~ <Class>"` record's weapon-side content
/// was ALSO independently verified (`weapon_and_armor_proficiency_grant_
/// class_table_matches_are_exact` in `class_feature_pool_catalog.rs`) as
/// an exact match against [`CLASS_WEAPON_PROFICIENCIES`] -- Druid and Monk
/// were investigated and are deliberately absent (see that test's own doc
/// comment): Druid's combined record's `AUTO:WEAPONPROF` list is missing
/// `Scythe` against its own dedicated `"Weapon Proficiencies ~ Druid"`
/// record and the table row (a real corpus-internal discrepancy, not this
/// table's concern to paper over), and Monk repeats the established
/// `Flurry of Blows`/`Unarmed Strike` mismatch. Widening this table to any
/// other class requires the same per-record verification, not a name
/// pattern.
pub const CLASS_ARMOR_PROFICIENCIES: &[ClassArmorProficiency] = &[
    // Bard: `ABILITY:Internal|AUTOMATIC|Armor Prof ~ Light|Shield Prof`.
    ClassArmorProficiency {
        class_id: "class:bard",
        light: true,
        medium: false,
        heavy: false,
        shield: true,
        tower_shield: false,
    },
    // Fighter: Heavy + Medium + Light + Shield Prof + Shield Prof ~ Tower,
    // each its own separate `ABILITY:Internal|AUTOMATIC|` indirection.
    ClassArmorProficiency {
        class_id: "class:fighter",
        light: true,
        medium: true,
        heavy: true,
        shield: true,
        tower_shield: true,
    },
    // Paladin: Heavy + Medium + Light + Shield Prof (no tower).
    ClassArmorProficiency {
        class_id: "class:paladin",
        light: true,
        medium: true,
        heavy: true,
        shield: true,
        tower_shield: false,
    },
    // Ranger: Light + Medium + Shield Prof (no heavy, no tower).
    ClassArmorProficiency {
        class_id: "class:ranger",
        light: true,
        medium: true,
        heavy: false,
        shield: true,
        tower_shield: false,
    },
    // Rogue: Light only, no shield of any kind.
    ClassArmorProficiency {
        class_id: "class:rogue",
        light: true,
        medium: false,
        heavy: false,
        shield: false,
        tower_shield: false,
    },
];

/// This class's armor/shield proficiency, or `None` for a class this table
/// does not cover. `None` means "not ingested", NOT "proficient with
/// nothing" -- same discipline as [`class_weapon_proficiency`].
pub fn class_armor_proficiency(class_id: &str) -> Option<&'static ClassArmorProficiency> {
    CLASS_ARMOR_PROFICIENCIES.iter().find(|entry| entry.class_id == class_id)
}

#[cfg(test)]
mod class_armor_proficiency_tests {
    use super::*;

    /// Every row's own claim, re-derived from the LIVE corpus record's own
    /// `ABILITY` tokens -- not merely asserted in the table above. RED if
    /// the corpus record ever changes which armor/shield tiers it grants.
    #[test]
    fn class_armor_proficiencies_match_their_own_corpus_records() {
        use std::path::PathBuf;
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/core_rulebook/class_feature/weapon_and_armor_proficiency");
        let expectations: &[(&str, &str)] = &[
            ("class:bard", "Bard"),
            ("class:fighter", "Fighter"),
            ("class:paladin", "Paladin"),
            ("class:ranger", "Ranger"),
            ("class:rogue", "Rogue"),
        ];
        for (class_id, class_name) in expectations {
            let row = class_armor_proficiency(class_id)
                .unwrap_or_else(|| panic!("{class_id} must be a real row in CLASS_ARMOR_PROFICIENCIES"));
            let mut found_file = false;
            for entry in std::fs::read_dir(&dir).expect("weapon_and_armor_proficiency dir exists") {
                let entry = entry.expect("readable dir entry");
                let text = std::fs::read_to_string(entry.path()).expect("readable corpus json");
                let json: serde_json::Value =
                    serde_json::from_str(&text).expect("valid corpus json");
                let key = json["data"]["key"].as_str().unwrap_or_default();
                if key != format!("Weapon and Armor Proficiency ~ {class_name}") {
                    continue;
                }
                found_file = true;
                let ability_tokens: Vec<String> = json["data"]["raw_tokens"]
                    .as_array()
                    .expect("raw_tokens is an array")
                    .iter()
                    .filter(|t| t["key"].as_str() == Some("ABILITY"))
                    .map(|t| t["value"].as_str().unwrap_or_default().to_string())
                    .collect();
                let has = |needle: &str| ability_tokens.iter().any(|v| v.contains(needle));
                assert_eq!(has("Armor Prof ~ Light"), row.light, "{class_name} light armor");
                assert_eq!(has("Armor Prof ~ Medium"), row.medium, "{class_name} medium armor");
                assert_eq!(has("Armor Prof ~ Heavy"), row.heavy, "{class_name} heavy armor");
                assert_eq!(has("Shield Prof ~ Tower"), row.tower_shield, "{class_name} tower shield");
                // "Shield Prof" alone (not "Shield Prof ~ Tower") is the
                // Buckler/Light/Heavy-shield grant -- must be checked
                // without matching the Tower variant's own substring.
                let has_plain_shield_prof = ability_tokens
                    .iter()
                    .any(|v| v.split('|').any(|part| part == "Shield Prof"));
                assert_eq!(has_plain_shield_prof, row.shield, "{class_name} shield (non-tower)");
            }
            assert!(found_file, "no corpus record found for {class_name}");
        }
    }

    #[test]
    fn druid_and_monk_are_deliberately_absent() {
        assert!(class_armor_proficiency("class:druid").is_none());
        assert!(class_armor_proficiency("class:monk").is_none());
    }
}

#[cfg(test)]
mod class_weapon_proficiency_tests {
    use super::*;

    fn prof(class_id: &str) -> &'static ClassWeaponProficiency {
        class_weapon_proficiency(class_id).expect("class must be covered")
    }

    fn weapon(key: &str) -> &'static WeaponTableEntry {
        weapon_by_key(key).expect("weapon must be in the table")
    }

    /// The bug this whole table exists to fix: a Wizard is not proficient
    /// with a Longsword, so the melee total owes a nonproficiency penalty.
    #[test]
    fn a_wizard_is_not_proficient_with_a_longsword() {
        assert!(!class_is_proficient_with(prof("class:wizard"), weapon("Longsword")));
    }

    /// The join that would have silently broken. Wizard's corpus list
    /// names `Crossbow (Heavy)`; the weapon's display key is `Heavy
    /// Crossbow`. Matching on the display key would report a Wizard as NOT
    /// proficient with a weapon it is explicitly granted.
    #[test]
    fn the_proficiency_namespace_join_survives_the_renamed_weapons() {
        let wizard = prof("class:wizard");
        assert!(class_is_proficient_with(wizard, weapon("Heavy Crossbow")));
        assert!(class_is_proficient_with(wizard, weapon("Light Crossbow")));
        // And the display key really does differ, so this test is not
        // passing for a trivial reason.
        assert_eq!(weapon("Heavy Crossbow").proficiency_name, Some("Crossbow (Heavy)"));
        assert_ne!(weapon("Heavy Crossbow").proficiency_name, Some("Heavy Crossbow"));
    }

    #[test]
    fn a_wizard_is_proficient_with_exactly_its_five_granted_weapons() {
        let wizard = prof("class:wizard");
        for granted in ["Club", "Dagger", "Heavy Crossbow", "Light Crossbow", "Quarterstaff"] {
            assert!(
                class_is_proficient_with(wizard, weapon(granted)),
                "{granted} is on Wizard's own corpus list"
            );
        }
        // No blanket Simple grant: a Wizard is NOT proficient with every
        // simple weapon, only those five.
        for denied in ["Spear", "Sling", "Light Mace", "Sickle"] {
            assert!(
                !class_is_proficient_with(wizard, weapon(denied)),
                "{denied} is Simple but NOT on Wizard's list -- a tier-only model would wrongly allow it"
            );
        }
    }

    /// Fighter's Martial grant lives in a `.MOD` record and an internal
    /// ability, not on the feature record's own `AUTO:WEAPONPROF`. A
    /// partial read of that record says Fighter has no Martial at all.
    #[test]
    fn a_fighter_is_proficient_with_simple_and_martial_weapons() {
        let fighter = prof("class:fighter");
        assert!(class_is_proficient_with(fighter, weapon("Longsword")), "Martial");
        assert!(class_is_proficient_with(fighter, weapon("Club")), "Simple");
        assert!(
            !class_is_proficient_with(fighter, weapon("Whip")),
            "an Exotic weapon is not granted to a Fighter by proficiency alone"
        );
    }

    /// Bastard Sword and Dwarven Waraxe carry BOTH `Exotic` and `Martial`
    /// in one `TYPE:` facet -- PF1's real rule that they are Martial used
    /// two-handed and Exotic used one-handed. This table stores one tier
    /// (Martial), so a Fighter reads as proficient: correct two-handed,
    /// over-permissive one-handed.
    ///
    /// Recorded as a test rather than only a comment so the limit is
    /// visible if anyone later relies on it. Resolving it properly needs
    /// wield state, which this engine does not record anywhere -- the same
    /// missing input that keeps the per-weapon damage record a feat-bonus
    /// rather than a damage total. Inventing it here would be fabrication.
    #[test]
    fn the_two_dual_tier_weapons_resolve_to_their_martial_tier() {
        for key in ["Bastard Sword", "Dwarven Waraxe"] {
            assert_eq!(
                weapon(key).proficiency,
                Some(WeaponProficiency::Martial),
                "{key} is Exotic one-handed and Martial two-handed; this table keeps Martial"
            );
            assert!(class_is_proficient_with(prof("class:fighter"), weapon(key)));
        }
    }

    #[test]
    fn a_bard_gets_simple_plus_its_six_named_martial_weapons() {
        let bard = prof("class:bard");
        assert!(class_is_proficient_with(bard, weapon("Longsword")), "named");
        assert!(class_is_proficient_with(bard, weapon("Short Sword")), "named as Sword (Short)");
        assert!(class_is_proficient_with(bard, weapon("Club")), "blanket Simple");
        assert!(
            !class_is_proficient_with(bard, weapon("Greataxe")),
            "an unnamed Martial weapon stays out of reach"
        );
    }

    /// Every named proficiency in the class table must be a real
    /// `PROFICIENCY:WEAPON` value carried by some weapon. A typo here
    /// would silently narrow a class's proficiency with no test failing
    /// anywhere else.
    #[test]
    fn every_named_class_proficiency_matches_a_real_weapon() {
        // Names legitimately absent, documented rather than skipped so the
        // guard still catches real typos: `Bomb` is the Alchemist's class
        // feature (no stat block); `Sword Cane` is a real weapon but an APG
        // one; `Kusarigama (Sickle and Chain)` and `Wakizashi` (SD-34 wave
        // 34 lane C, Ninja's own corpus token) are real Ultimate Combat
        // weapons -- all three are outside this CRB-only table's own scope,
        // a genuine scope mismatch, recorded here as a known limit.
        const OUTSIDE_THE_CRB_WEAPON_TABLE: &[&str] =
            &["Bomb", "Sword Cane", "Kusarigama (Sickle and Chain)", "Wakizashi"];
        for class in CLASS_WEAPON_PROFICIENCIES {
            for named in class.named {
                if OUTSIDE_THE_CRB_WEAPON_TABLE.contains(named) {
                    continue;
                }
                assert!(
                    WEAPON_TABLE.iter().any(|w| w.proficiency_name == Some(*named)),
                    "{}'s named proficiency {named:?} matches no weapon's PROFICIENCY:WEAPON token",
                    class.class_id
                );
            }
        }
    }

    /// SD-31 wave 20 (chassis-coverage lane): Ultimate Combat's Gunslinger
    /// carries a real corpus proficiency record
    /// (`uc_abilities_class.lst`, `KEY:Gunslinger ~ Proficiencies`) --
    /// `ABILITY:Internal|AUTOMATIC|TYPE=WeaponProfMartial|TYPE=ArmorProfLight`
    /// plus a second indirection through `Weapon Prof ~ Auto`/`Weapon Prof
    /// ~ Simple` -- the same convention-2 indirection shape Fighter and
    /// Barbarian already use for their own Simple+Martial grant. `TYPE=Firearm`
    /// is also on that record; it is a weapon TYPE selector, not a tier this
    /// table models (the same documented, deliberate boundary the Unchained
    /// Monk's `TYPE=Monk` selector already carries above), so it is NOT
    /// claimed here. Was previously absent entirely -- `class:gunslinger`
    /// read `None` ("not ingested"), which claim-blocked the whole melee
    /// baseline for every Gunslinger regardless of what weapon it held.
    #[test]
    fn gunslinger_has_simple_and_martial_tiers_from_its_real_corpus_record() {
        let gunslinger = prof("class:gunslinger");
        assert_eq!(gunslinger.tiers, &[WeaponProficiency::Simple, WeaponProficiency::Martial]);
        assert!(gunslinger.named.is_empty());
        assert!(gunslinger.weapon_groups.is_empty());
        // The actual bug this closes: a Gunslinger IS proficient with the
        // Longsword (Martial tier), so `combat.baseline_weapon_proficiency_unknown`
        // must resolve rather than claim-block.
        assert!(class_is_proficient_with(gunslinger, weapon("Longsword")));
    }

    /// SD-34 wave 34 lane C: Ninja's own real corpus token (`ultimate_
    /// combat/class_feature/ninja/ninja_weapon_proficiencies.json`) --
    /// nine named weapons, no tier, transcribed verbatim. Deliberately does
    /// NOT assert Ninja is non-proficient with every simple weapon: real
    /// PF1 Ninjas ARE proficient with all simple weapons per the class's
    /// own DESC, this table just has no matching token to ground that
    /// claim on -- a documented partial transcription, not a claim that
    /// this row is complete.
    #[test]
    fn ninja_has_its_real_named_weapon_list_and_no_blanket_simple_tier() {
        let ninja = prof("class:ninja");
        assert!(ninja.tiers.is_empty(), "no Simple/Martial/Exotic facet on Ninja's own token");
        assert!(ninja.weapon_groups.is_empty());
        for granted in ["Shortbow", "Short Sword", "Kama", "Nunchaku", "Sai", "Shuriken", "Siangham"] {
            assert!(
                class_is_proficient_with(ninja, weapon(granted)),
                "{granted} is on Ninja's own corpus AUTO:WEAPONPROF list"
            );
        }
        // The bug this closes: Ninja now resolves a real (correct)
        // Longsword verdict instead of leaving
        // `combat.baseline_weapon_proficiency_unknown` claim-blocking the
        // whole melee baseline. Longsword is Martial-tier and not on
        // Ninja's own named list, so the correct verdict is non-proficient.
        assert!(!class_is_proficient_with(ninja, weapon("Longsword")));
    }

    #[test]
    fn an_unknown_class_reports_unknown_rather_than_non_proficient() {
        assert!(class_weapon_proficiency("class:not_a_class").is_none());
        assert!(class_weapon_proficiency("class:eldritch_knight").is_none());
    }

    /// The whole 31-class roster, not just the CRB set.
    ///
    /// Shipping this CRB-only was a real near-miss: it returned "unknown"
    /// for 16 classes, and any caller flagging non-proficiency by omission
    /// would have penalised seven martial ones (Bloodrager, Skald, Slayer,
    /// Swashbuckler, Cavalier, Hunter, Warpriest).
    ///
    /// SD-27 (2026-07-31) added Pathfinder Unchained's four. The same
    /// near-miss applied to them in a sharper form: an Unchained class with
    /// no row here reads as "unknown", which claim-blocks the whole combat
    /// baseline, so the class would have been selectable and uncomputable.
    #[test]
    fn every_class_in_the_roster_is_covered() {
        for class_id in [
            "class:alchemist", "class:arcanist", "class:barbarian", "class:bard",
            "class:bloodrager", "class:brawler", "class:cavalier", "class:cleric",
            "class:druid", "class:fighter", "class:hunter", "class:inquisitor",
            "class:investigator", "class:monk", "class:oracle", "class:paladin",
            "class:ranger", "class:rogue", "class:shaman", "class:skald",
            "class:slayer", "class:sorcerer", "class:summoner", "class:swashbuckler",
            "class:warpriest", "class:witch", "class:wizard",
            "class:unchained_barbarian", "class:unchained_monk",
            "class:unchained_rogue", "class:unchained_summoner",
            "class:gunslinger",
            "class:kineticist", "class:medium", "class:mesmerist", "class:occultist",
            "class:vigilante", "class:psychic", "class:spiritualist", "class:psion",
            "class:shifter",
            // SD-34 wave 34 lane C.
            "class:ninja",
        ] {
            assert!(
                class_weapon_proficiency(class_id).is_some(),
                "{class_id} has a real corpus proficiency record and must be covered"
            );
        }
        assert_eq!(CLASS_WEAPON_PROFICIENCIES.len(), 42);
    }

    /// Each Unchained class's grants against the class it replaces. Three
    /// match exactly; the Unchained Monk does not, and the difference is
    /// pinned rather than tolerated -- PU's `AUTO:WEAPONPROF` names 16
    /// weapons where CRB's Monk row names 17, omitting `Unarmed Strike`.
    #[test]
    fn unchained_weapon_grants_match_their_base_class_except_the_monks_unarmed_strike() {
        for (unchained, base) in [
            ("class:unchained_barbarian", "class:barbarian"),
            ("class:unchained_rogue", "class:rogue"),
            ("class:unchained_summoner", "class:summoner"),
        ] {
            let u = prof(unchained);
            let b = prof(base);
            assert_eq!(u.tiers, b.tiers, "{unchained} tiers");
            assert_eq!(u.named, b.named, "{unchained} named weapons");
            assert_eq!(u.weapon_groups, b.weapon_groups, "{unchained} weapon groups");
        }

        let pu_monk = prof("class:unchained_monk");
        let crb_monk = prof("class:monk");
        assert_eq!(pu_monk.named.len(), 16);
        assert_eq!(crb_monk.named.len(), 17);
        assert!(crb_monk.named.contains(&"Unarmed Strike"));
        assert!(
            !pu_monk.named.contains(&"Unarmed Strike"),
            "PU's own token does not name it -- transcribed, not corrected"
        );
        // Everything else on the CRB list is on PU's list too, so the
        // difference really is exactly the one entry.
        for weapon_name in crb_monk.named {
            if *weapon_name == "Unarmed Strike" {
                continue;
            }
            assert!(pu_monk.named.contains(weapon_name), "{weapon_name}");
        }
    }

    /// The Longsword question decided for the whole roster in one place --
    /// this is what the melee baseline actually turns on. 13 proficient,
    /// 18 not.
    ///
    /// SD-27 added `class:unchained_barbarian` to the proficient side and
    /// the other three Unchained classes to the non-proficient side, each
    /// on its own corpus record: the Unchained Barbarian grants the Martial
    /// tier, and the Unchained Monk / Rogue / Summoner do not, exactly like
    /// the classes they replace.
    #[test]
    fn longsword_proficiency_is_correct_for_every_class() {
        let longsword = weapon("Longsword");
        let expected_proficient = [
            "class:barbarian", "class:bard", "class:bloodrager", "class:cavalier",
            "class:fighter", "class:hunter", "class:paladin", "class:ranger",
            "class:skald", "class:slayer", "class:swashbuckler", "class:warpriest",
            "class:unchained_barbarian", "class:gunslinger",
            // SD-34 wave 33 lane C: both carry ONLY `TYPE=WeaponProfMartial`
            // in their real corpus token (see the roster comment above),
            // and Longsword is a Martial-tier weapon.
            "class:occultist", "class:vigilante",
        ];
        let mut proficient = 0;
        for class in CLASS_WEAPON_PROFICIENCIES {
            let actual = class_is_proficient_with(class, longsword);
            let expected = expected_proficient.contains(&class.class_id);
            assert_eq!(
                actual, expected,
                "{} Longsword proficiency: expected {expected}, got {actual}",
                class.class_id
            );
            proficient += usize::from(actual);
        }
        // SD-34 wave 34 lane C: Ninja added to the roster, non-proficient
        // (Longsword is Martial-tier and not on Ninja's own named list),
        // so the proficient count is unchanged, only the denominator moves.
        assert_eq!(proficient, 16, "16 of 42 classes are Longsword-proficient");
    }

    /// Bard reaches Longsword through its explicit list, NOT a martial
    /// tier. A tier-only model would wrongly deny it.
    #[test]
    fn bard_reaches_longsword_by_name_not_by_tier() {
        let bard = prof("class:bard");
        assert!(!bard.tiers.contains(&WeaponProficiency::Martial), "Bard has no martial tier");
        assert!(class_is_proficient_with(bard, weapon("Longsword")), "but names it explicitly");
    }

    /// Inquisitor is Simple + bows/crossbows + deity weapons -- NOT
    /// martial, contrary to a common RAW recollection that would have
    /// excluded it from the affected set.
    #[test]
    fn inquisitor_is_not_martial_despite_the_common_assumption() {
        let inquisitor = prof("class:inquisitor");
        assert!(!class_is_proficient_with(inquisitor, weapon("Longsword")));
        assert!(class_is_proficient_with(inquisitor, weapon("Shortbow")), "named");
        assert!(class_is_proficient_with(inquisitor, weapon("Club")), "blanket Simple");
    }

    /// Brawler is the only class granted a whole weapon GROUP. Longsword
    /// is Blades Heavy, not Close, so it stays non-proficient -- but a
    /// Close-group weapon must resolve through the group grant.
    #[test]
    fn brawler_gets_its_close_weapon_group() {
        let brawler = prof("class:brawler");
        assert!(!class_is_proficient_with(brawler, weapon("Longsword")), "Blades Heavy");
        let close = WEAPON_TABLE
            .iter()
            .find(|w| {
                w.weapon_group == Some("Close")
                    && w.proficiency == Some(WeaponProficiency::Martial)
            })
            .expect("the table has a martial Close-group weapon");
        assert!(
            class_is_proficient_with(brawler, close),
            "{} is Close group and must resolve via the group grant",
            close.key
        );
    }
}
