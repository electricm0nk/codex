//! Ultimate Combat equipment tables -- full corpus coverage.
//!
//! Record coverage: every real, active (non-`.MOD`) record across four
//! source files -- `uc_equip_general.lst` (General: firearm consumables,
//! siege-engine hardware), `uc_equip_magic_items.lst` (MagicItems:
//! Wondrous Items), `uc_equip_arms_armor.lst` (ArmsArmor: ammunition,
//! armor, weapons, firearms), and `uc_equipmods.lst` (Equipmods) -- 185
//! equipment + 19 equipmods = 204 total.
//!
//! **`.MOD` rows excluded from the equipment count, same ruling this
//! program has applied to every other book's `.MOD` rows.** 61 `.MOD`
//! rows exist across the two files that carry any (`uc_equip_general.lst`
//! 1, `uc_equip_arms_armor.lst` 60); 4 additional would-be `.MOD` lines in
//! `uc_equip_arms_armor.lst` are themselves comment-prefixed (disabled)
//! and were never live content in the first place. Every live `.MOD`
//! checked resolves to either a same-file record this table already
//! counts once (e.g. `Hooked Axe.MOD` re-tags the `Hooked Axe` this table
//! already lists, adding a weapon-group `TYPE:` after the fact) or a
//! cross-book base item this book does not own (`Carriage.MOD`,
//! `Pilum.MOD`) -- neither shape is a second, distinct catalog record.
//! `uc_equip_general.lst` (27 raw lines) minus its 1 `.MOD` = 26;
//! `uc_equip_magic_items.lst` (10 raw lines, no `.MOD`) = 10;
//! `uc_equip_arms_armor.lst` (209 raw lines) minus its 60 live `.MOD` =
//! 149. 26 + 10 + 149 = 185, matching the declared work-inventory figure
//! for this book's `equipment` kind exactly.
//!
//! **`uc_equipmods.lst`'s declared figure does NOT survive at face
//! value -- the same `.COPY=` legacy-alias hazard `§58` found on UPsi,
//! found again here.** 39 raw content lines, but 20 of them are a
//! `#Old KEYs` block: every one `VISIBLE:NO`, every one a
//! `<RealKey>.COPY=<SHORT_CODE>` legacy alias mirroring a real,
//! already-counted record above it (e.g. `Special Quality ~
//! Thrown.COPY=THROWN` aliases the real `Thrown` equipmod), not a second
//! population. One alias (`Special Quality ~ Reach.COPY=REACH`) mirrors a
//! record that is itself commented out in the corpus (`#Reach`) --
//! correctly excluded either way, since `VISIBLE:NO` `.COPY=` rows are
//! never counted regardless of what they alias. Real count: 39 - 20 = 19,
//! not the 39 a raw line count would suggest.
//!
//! `description` is sourced from the corpus `SPROP:` token(s), joined
//! with `"; "` when a record carries more than one -- the same convention
//! every other book in this program already establishes. `key` is the
//! corpus `KEY:` token when the record carries one (every `Equipmods` row
//! does; no `General`/`MagicItems`/`ArmsArmor` row does), else the
//! record's own display name. `name` is the `OUTPUTNAME:` token when
//! present, else the same field used for `key`. One record
//! (`Oil (Of Silence)`) carries a literal, unresolved `OUTPUTNAME:Oil
//! [NAME]` template -- preserved verbatim rather than fabricating a
//! resolution, the identical treatment `ultimate_psionics::equipment_tables`
//! already established for its own `[NAME]`-templated Psicrown records.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    General,
    MagicItems,
    ArmsArmor,
    Equipmods,
}

impl EquipmentCategory {
    pub const ALL: &'static [EquipmentCategory] = &[
        EquipmentCategory::General,
        EquipmentCategory::MagicItems,
        EquipmentCategory::ArmsArmor,
        EquipmentCategory::Equipmods,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    /// The record's corpus `KEY:` token when present (every `Equipmods`
    /// row), else its own display name.
    pub key: &'static str,
    pub category: EquipmentCategory,
    /// `OUTPUTNAME:` when the corpus record carries one, else the same as
    /// `key`'s source field.
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. `None` when the
    /// token is absent (several `Equipmods` rows price via `PLUS:`, an
    /// enhancement-bonus slot cost, not a flat gp number) or carries a
    /// PCGen formula this table does not evaluate (`Material ~ Gold`'s
    /// `COST:(BASECOST)*9`).
    pub cost_gp: Option<f64>,
    /// Weight in pounds from the corpus `WT:` token. `None` for every
    /// `Equipmods` row (modifiers carry no independent weight, matching
    /// every other book's own established finding) and for any
    /// `General`/`MagicItems`/`ArmsArmor` row whose corpus record
    /// genuinely carries no `WT:` token.
    pub weight_lbs: Option<f64>,
    /// Descriptive text, sourced from the corpus `SPROP:` token(s) --
    /// joined with `"; "` when more than one. `None` only when the corpus
    /// record has no `SPROP:` token at all.
    pub description: Option<&'static str>,
}

/// SD-28-E15-style equipment field-coverage audit row, mirroring the
/// shape every other book's equipment table already establishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    pub total_records: u32,
    pub records_expected: u32,
    pub has_cost: u32,
    pub has_weight: u32,
    pub has_description: u32,
}

/// Computes this book's equipment field-coverage audit row (equipment
/// only -- General + MagicItems + ArmsArmor, matching every other book's
/// own `field_coverage_report` scope, which excludes Equipmods).
pub fn field_coverage_report() -> EquipmentFieldCoverage {
    let table = equipment_tables();
    EquipmentFieldCoverage {
        total_records: table.len() as u32,
        records_expected: 185,
        has_cost: table.iter().filter(|entry| entry.cost_gp.is_some()).count() as u32,
        has_weight: table.iter().filter(|entry| entry.weight_lbs.is_some()).count() as u32,
        has_description: table.iter().filter(|entry| entry.description.is_some()).count() as u32,
    }
}

const GENERAL_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry { key: "Black Powder (Dose)", category: EquipmentCategory::General, name: "Black Powder (Dose)", cost_gp: Some(10.0_f64), weight_lbs: None, description: None }, // uc_equip_general.lst:8
    EquipmentTableEntry { key: "Black Powder (Keg)", category: EquipmentCategory::General, name: "Black Powder (Keg)", cost_gp: Some(1000.0_f64), weight_lbs: Some(5.0_f64), description: None }, // uc_equip_general.lst:9
    EquipmentTableEntry { key: "Gunsmith's Kit", category: EquipmentCategory::General, name: "Gunsmith's Kit", cost_gp: Some(15.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_general.lst:10
    EquipmentTableEntry { key: "Powder Horn", category: EquipmentCategory::General, name: "Powder Horn", cost_gp: Some(3.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_general.lst:11
    EquipmentTableEntry { key: "Powder Keg", category: EquipmentCategory::General, name: "Powder Keg", cost_gp: Some(0.0_f64), weight_lbs: Some(5.0_f64), description: None }, // uc_equip_general.lst:12
    EquipmentTableEntry { key: "Bridge (Assault/Colossal)", category: EquipmentCategory::General, name: "Bridge (Assault/Colossal)", cost_gp: Some(250.0_f64), weight_lbs: None, description: None }, // uc_equip_general.lst:17
    EquipmentTableEntry { key: "Bridge (Assault/Gargantuan)", category: EquipmentCategory::General, name: "Bridge (Assault/Gargantuan)", cost_gp: Some(50.0_f64), weight_lbs: None, description: None }, // uc_equip_general.lst:18
    EquipmentTableEntry { key: "Bridge (Assault/Huge)", category: EquipmentCategory::General, name: "Bridge (Assault/Huge)", cost_gp: Some(10.0_f64), weight_lbs: None, description: None }, // uc_equip_general.lst:19
    EquipmentTableEntry { key: "Bridge (Assault/Large)", category: EquipmentCategory::General, name: "Bridge (Assault/Large)", cost_gp: Some(1.0_f64), weight_lbs: None, description: None }, // uc_equip_general.lst:20
    EquipmentTableEntry { key: "Corvus", category: EquipmentCategory::General, name: "Corvus", cost_gp: Some(100.0_f64), weight_lbs: None, description: Some("Crew 1, Speed Special") }, // uc_equip_general.lst:21
    EquipmentTableEntry { key: "Gallery (Colossal)", category: EquipmentCategory::General, name: "Gallery (Colossal)", cost_gp: Some(2000.0_f64), weight_lbs: None, description: Some("Crew 18, Speed 15 ft.") }, // uc_equip_general.lst:22
    EquipmentTableEntry { key: "Gallery (Gargantuan)", category: EquipmentCategory::General, name: "Gallery (Gargantuan)", cost_gp: Some(1000.0_f64), weight_lbs: None, description: Some("Crew 9, Speed 15 ft.") }, // uc_equip_general.lst:23
    EquipmentTableEntry { key: "Gallery (Huge)", category: EquipmentCategory::General, name: "Gallery (Huge)", cost_gp: Some(500.0_f64), weight_lbs: None, description: Some("Crew 6, Speed 15 ft.") }, // uc_equip_general.lst:24
    EquipmentTableEntry { key: "Gallery (Large)", category: EquipmentCategory::General, name: "Gallery (Large)", cost_gp: Some(250.0_f64), weight_lbs: None, description: Some("Crew 3, Speed 15 ft.") }, // uc_equip_general.lst:25
    EquipmentTableEntry { key: "Ladder (Escalade/Colossal)", category: EquipmentCategory::General, name: "Ladder (Escalade/Colossal)", cost_gp: Some(250.0_f64), weight_lbs: None, description: Some("Crew 8, Speed as crew speed") }, // uc_equip_general.lst:26
    EquipmentTableEntry { key: "Ladder (Escalade/Gargantuan)", category: EquipmentCategory::General, name: "Ladder (Escalade/Gargantuan)", cost_gp: Some(50.0_f64), weight_lbs: None, description: Some("Crew 6, Speed as crew speed") }, // uc_equip_general.lst:27
    EquipmentTableEntry { key: "Ladder (Escalade/Huge)", category: EquipmentCategory::General, name: "Ladder (Escalade/Huge)", cost_gp: Some(10.0_f64), weight_lbs: None, description: Some("Crew 4, Speed as crew speed") }, // uc_equip_general.lst:28
    EquipmentTableEntry { key: "Ladder (Escalade/Large)", category: EquipmentCategory::General, name: "Ladder (Escalade/Large)", cost_gp: Some(5.0_f64), weight_lbs: None, description: Some("Crew 2, Speed as crew speed") }, // uc_equip_general.lst:29
    EquipmentTableEntry { key: "Ram (Colossal)", category: EquipmentCategory::General, name: "Ram (Colossal)", cost_gp: Some(5000.0_f64), weight_lbs: None, description: Some("Damage 10d6 20/x3 Bludgeoning; Crew 40, Speed 15 ft.") }, // uc_equip_general.lst:30
    EquipmentTableEntry { key: "Ram (Gargantuan)", category: EquipmentCategory::General, name: "Ram (Gargantuan)", cost_gp: Some(2000.0_f64), weight_lbs: None, description: Some("Damage 6d6 20/x3 Bludgeoning; Crew 20, Speed 15 ft.") }, // uc_equip_general.lst:31
    EquipmentTableEntry { key: "Ram (Huge)", category: EquipmentCategory::General, name: "Ram (Huge)", cost_gp: Some(1000.0_f64), weight_lbs: None, description: Some("Damage 3d6 20/x3 Bludgeoning; Crew 10, Speed 15 ft.") }, // uc_equip_general.lst:32
    EquipmentTableEntry { key: "Ram (Large)", category: EquipmentCategory::General, name: "Ram (Large)", cost_gp: Some(500.0_f64), weight_lbs: None, description: Some("Damage 2d6 20/x3 Bludgeoning; Crew 5, Speed 15 ft.") }, // uc_equip_general.lst:33
    EquipmentTableEntry { key: "Siege Tower (Colossal)", category: EquipmentCategory::General, name: "Siege Tower (Colossal)", cost_gp: Some(10000.0_f64), weight_lbs: None, description: Some("Crew 48, Speed 15 ft.") }, // uc_equip_general.lst:34
    EquipmentTableEntry { key: "Siege Tower (Gargantuan)", category: EquipmentCategory::General, name: "Siege Tower (Gargantuan)", cost_gp: Some(5000.0_f64), weight_lbs: None, description: Some("Crew 24, Speed 15 ft.") }, // uc_equip_general.lst:35
    EquipmentTableEntry { key: "Siege Tower (Huge)", category: EquipmentCategory::General, name: "Siege Tower (Huge)", cost_gp: Some(2000.0_f64), weight_lbs: None, description: Some("Crew 12, Speed 15 ft.") }, // uc_equip_general.lst:36
    EquipmentTableEntry { key: "Siege Tower (Large)", category: EquipmentCategory::General, name: "Siege Tower (Large)", cost_gp: Some(1000.0_f64), weight_lbs: None, description: Some("Crew 6, Speed 15 ft.") }, // uc_equip_general.lst:37
];

const MAGIC_ITEMS_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry { key: "Amulet of Bullet Protection +1", category: EquipmentCategory::MagicItems, name: "Amulet of Bullet Protection +1", cost_gp: Some(1500.0_f64), weight_lbs: None, description: Some("This amulet, usually crafted from the splintered remains of spent firearm bullets shaped into a rough holy symbol or clover, grants the wearer a luck bonus to AC against firearm attacks that target touch AC.") }, // uc_equip_magic_items.lst:7
    EquipmentTableEntry { key: "Amulet of Bullet Protection +2", category: EquipmentCategory::MagicItems, name: "Amulet of Bullet Protection +2", cost_gp: Some(6000.0_f64), weight_lbs: None, description: Some("This amulet, usually crafted from the splintered remains of spent firearm bullets shaped into a rough holy symbol or clover, grants the wearer a luck bonus to AC against firearm attacks that target touch AC.") }, // uc_equip_magic_items.lst:8
    EquipmentTableEntry { key: "Amulet of Bullet Protection +3", category: EquipmentCategory::MagicItems, name: "Amulet of Bullet Protection +3", cost_gp: Some(13500.0_f64), weight_lbs: None, description: Some("This amulet, usually crafted from the splintered remains of spent firearm bullets shaped into a rough holy symbol or clover, grants the wearer a luck bonus to AC against firearm attacks that target touch AC.") }, // uc_equip_magic_items.lst:9
    EquipmentTableEntry { key: "Amulet of Bullet Protection +4", category: EquipmentCategory::MagicItems, name: "Amulet of Bullet Protection +4", cost_gp: Some(24000.0_f64), weight_lbs: None, description: Some("This amulet, usually crafted from the splintered remains of spent firearm bullets shaped into a rough holy symbol or clover, grants the wearer a luck bonus to AC against firearm attacks that target touch AC.") }, // uc_equip_magic_items.lst:10
    EquipmentTableEntry { key: "Amulet of Bullet Protection +5", category: EquipmentCategory::MagicItems, name: "Amulet of Bullet Protection +5", cost_gp: Some(37500.0_f64), weight_lbs: None, description: Some("This amulet, usually crafted from the splintered remains of spent firearm bullets shaped into a rough holy symbol or clover, grants the wearer a luck bonus to AC against firearm attacks that target touch AC.") }, // uc_equip_magic_items.lst:11
    EquipmentTableEntry { key: "Dry Load Powder Horn", category: EquipmentCategory::MagicItems, name: "Dry Load Powder Horn", cost_gp: Some(2000.0_f64), weight_lbs: None, description: Some("This magical powder horn holds 20 doses of black powder. Furthermore, when a firearm is loaded with black powder directly from this horn, the horn creates a small pocket of air that envelops the gun and allows that shot to be fired underwater or in an area lacking air, such as a vacuum. Once the gun is loaded with powder from the dry load powder horn, it retains the pocket of air for 10 minutes or until the firearm is fired, whichever comes first. Firing a firearm that has been loaded from this horn underwater still incurs the -2 penalty on attack rolls for every 5 feet of water the bullet passes through, in addition to the normal penalties to range. When a shot loaded from a dry load powder horn results in a firearm explosion while underwater or in an airless environment, that explosion occurs normally.") }, // uc_equip_magic_items.lst:12
    EquipmentTableEntry { key: "Far-Reaching Sight", category: EquipmentCategory::MagicItems, name: "Far-Reaching Sight", cost_gp: Some(4000.0_f64), weight_lbs: Some(1.0_f64), description: Some("This sight can be attached to a single two-handed firearm. When this is done, the sight becomes part of the weapon, but can be removed from that weapon with a full-round action. A firearm wielder can choose to spend a full-round action to make a single shot with a firearm that has this sight. When she does, she can resolve the attack against the touch AC of her target regardless of the range increment.") }, // uc_equip_magic_items.lst:13
    EquipmentTableEntry { key: "Figurine of Wondrous Power (Slate Spider)", category: EquipmentCategory::MagicItems, name: "Figurine of Wondrous Power (Slate Spider)", cost_gp: Some(10000.0_f64), weight_lbs: None, description: Some("This figure of wondrous power uses the same general rules for all magic items of this type (Core Rulebook 513). This statuette of a spider with stubby legs can be used once per day for 1 minute. When activated, the figurine spouts longer, segmented legs, and scampers about the activator, picking lint off robes, chiseling grime from armor, or otherwise grooming its activator. If commanded to do so as a free action, it perches on the muzzle of a one-handed or two-handed firearm barrel and, after the firearm is fired, travels down the barrel and cleans out the firearm. Each time the slate spider cleans a firearm, the next shot the firearm fires has no chance of misfiring. When animated, a slate spider has 1 hit point and is considered an attended object. An animated slate spider will never willingly leave space of its animator.") }, // uc_equip_magic_items.lst:14
    EquipmentTableEntry { key: "Oil (Of Silence)", category: EquipmentCategory::MagicItems, name: "Oil [NAME]", cost_gp: Some(250.0_f64), weight_lbs: None, description: Some("When applied to a one-handed or two-handed firearm, this strange grayish oil renders that firearm silent for 1 hour. Five vials of oil of silence can be used in conjunction to silence a Large firearm siege engine, and 10 can be used to silence a Huge firearm siege engine. The oil does not work on firearm siege engines that are larger than Huge.") }, // uc_equip_magic_items.lst:15
    EquipmentTableEntry { key: "See Invisibility Sight", category: EquipmentCategory::MagicItems, name: "See Invisibility Sight", cost_gp: Some(12000.0_f64), weight_lbs: Some(1.0_f64), description: Some("This sight can be attached to a single two-handed firearm. When this is done, the sight becomes part of the weapon, but can be removed from that weapon with a full-round action. A firearm wielder using a firearm that has this sight can choose to spend a full-round action to either locate an invisible creature within line of sight or make a single shot that ignores the invisibility of a creature that she knows is in the area.") }, // uc_equip_magic_items.lst:16
];

const ARMS_ARMOR_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry { key: "Alchemical Cartridge (Dragon's Breath)", category: EquipmentCategory::ArmsArmor, name: "Alchemical Cartridge (Dragon's Breath)", cost_gp: Some(40.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:8
    EquipmentTableEntry { key: "Alchemical Cartridge (Entangling Shot)", category: EquipmentCategory::ArmsArmor, name: "Alchemical Cartridge (Entangling Shot)", cost_gp: Some(40.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:9
    EquipmentTableEntry { key: "Alchemical Cartridge (Flare)", category: EquipmentCategory::ArmsArmor, name: "Alchemical Cartridge (Flare)", cost_gp: Some(10.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:10
    EquipmentTableEntry { key: "Alchemical Cartridge (Paper/Bullet)", category: EquipmentCategory::ArmsArmor, name: "Alchemical Cartridge (Paper/Bullet)", cost_gp: Some(12.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:11
    EquipmentTableEntry { key: "Alchemical Cartridge (Paper/Pellet)", category: EquipmentCategory::ArmsArmor, name: "Alchemical Cartridge (Paper/Pellet)", cost_gp: Some(12.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:12
    EquipmentTableEntry { key: "Alchemical Cartridge (Salt Shot)", category: EquipmentCategory::ArmsArmor, name: "Alchemical Cartridge (Salt Shot)", cost_gp: Some(12.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:13
    EquipmentTableEntry { key: "Arrow (Iron-tipped Distance/20)", category: EquipmentCategory::ArmsArmor, name: "Arrow (Iron-tipped Distance/20)", cost_gp: Some(1.0_f64), weight_lbs: Some(4.0_f64), description: Some("Iron-tipped distance arrows increase their bow's range increment by 10 feet but take a -1 penalty on damage dealt per range increment.") }, // uc_equip_arms_armor.lst:14
    EquipmentTableEntry { key: "Arrow (Whistling)", category: EquipmentCategory::ArmsArmor, name: "Arrow (Whistling)", cost_gp: Some(0.1_f64), weight_lbs: Some(0.15_f64), description: None }, // uc_equip_arms_armor.lst:15
    EquipmentTableEntry { key: "Atlatl Dart", category: EquipmentCategory::ArmsArmor, name: "Atlatl Dart", cost_gp: Some(1.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:16
    EquipmentTableEntry { key: "Bamboo Shaft (10)", category: EquipmentCategory::ArmsArmor, name: "Bamboo Shaft (10)", cost_gp: Some(1.0_f64), weight_lbs: Some(0.5_f64), description: None }, // uc_equip_arms_armor.lst:17
    EquipmentTableEntry { key: "Bullet (Adamantine)", category: EquipmentCategory::ArmsArmor, name: "Bullet (Adamantine)", cost_gp: Some(61.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:18
    EquipmentTableEntry { key: "Bullet (Firearm)", category: EquipmentCategory::ArmsArmor, name: "Bullet (Firearm)", cost_gp: Some(1.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:19
    EquipmentTableEntry { key: "Bullet (Firearm/30)", category: EquipmentCategory::ArmsArmor, name: "Bullet (Firearm/30)", cost_gp: Some(30.0_f64), weight_lbs: Some(0.5_f64), description: None }, // uc_equip_arms_armor.lst:20
    EquipmentTableEntry { key: "Bullet (Firearm/Pitted)", category: EquipmentCategory::ArmsArmor, name: "Bullet (Firearm/Pitted)", cost_gp: Some(5.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:21
    EquipmentTableEntry { key: "Bullet (Firearm/Silver)", category: EquipmentCategory::ArmsArmor, name: "Bullet (Firearm/Silver)", cost_gp: Some(25.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:22
    EquipmentTableEntry { key: "Burrowing Bullet", category: EquipmentCategory::ArmsArmor, name: "Burrowing Bullet", cost_gp: Some(1722.0_f64), weight_lbs: None, description: Some("This +1 firearm bullet deals normal damage, but when it hits a living creature, it burrows into the creature's flesh, causing wracking pain until removed or until the bullet burrows its way out of the creature. While these bullets burrow, the creature is staggered. This effect lasts for 1d3 rounds or until the bullet is removed with a DC 15 Heal check made as a standard action.") }, // uc_equip_arms_armor.lst:23
    EquipmentTableEntry { key: "Burrowing Bullet (Greater)", category: EquipmentCategory::ArmsArmor, name: "Burrowing Bullet (Greater)", cost_gp: Some(3447.0_f64), weight_lbs: None, description: Some("This +1 firearm bullet deals normal damage, but when it hits a living creature, it burrows into the creature's flesh, causing wracking pain until removed or until the bullet burrows its way out of the creature. While these bullets burrow, the creature is staggered. This effect lasts for 1d3+2 rounds or until the bullet is removed with a DC 20 Heal check made as a standard action.") }, // uc_equip_arms_armor.lst:24
    EquipmentTableEntry { key: "Kestros Dart (10)", category: EquipmentCategory::ArmsArmor, name: "Kestros Dart (10)", cost_gp: Some(5.0_f64), weight_lbs: Some(5.0_f64), description: None }, // uc_equip_arms_armor.lst:25
    EquipmentTableEntry { key: "Metal Cartridge", category: EquipmentCategory::ArmsArmor, name: "Metal Cartridge", cost_gp: Some(15.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:26
    EquipmentTableEntry { key: "Pellets (Handful)", category: EquipmentCategory::ArmsArmor, name: "Pellets (Handful)", cost_gp: Some(1.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:27
    EquipmentTableEntry { key: "Pellets (Handful/30)", category: EquipmentCategory::ArmsArmor, name: "Pellets (Handful/30)", cost_gp: Some(30.0_f64), weight_lbs: Some(0.5_f64), description: None }, // uc_equip_arms_armor.lst:28
    EquipmentTableEntry { key: "Siege Engine Ammunition (Alchemist's Fire)", category: EquipmentCategory::ArmsArmor, name: "Siege Engine Ammunition (Alchemist's Fire)", cost_gp: Some(200.0_f64), weight_lbs: Some(10.0_f64), description: None }, // uc_equip_arms_armor.lst:29
    EquipmentTableEntry { key: "Siege Engine Ammunition (Blast Shot)", category: EquipmentCategory::ArmsArmor, name: "Siege Engine Ammunition (Blast Shot)", cost_gp: Some(30.0_f64), weight_lbs: Some(25.0_f64), description: None }, // uc_equip_arms_armor.lst:30
    EquipmentTableEntry { key: "Siege Engine Ammunition (Bomb)", category: EquipmentCategory::ArmsArmor, name: "Siege Engine Ammunition (Bomb)", cost_gp: Some(600.0_f64), weight_lbs: Some(30.0_f64), description: None }, // uc_equip_arms_armor.lst:31
    EquipmentTableEntry { key: "Siege Engine Ammunition (Chain Shot)", category: EquipmentCategory::ArmsArmor, name: "Siege Engine Ammunition (Chain Shot)", cost_gp: Some(50.0_f64), weight_lbs: Some(30.0_f64), description: None }, // uc_equip_arms_armor.lst:32
    EquipmentTableEntry { key: "Siege Engine Ammunition (Liquid Ice)", category: EquipmentCategory::ArmsArmor, name: "Siege Engine Ammunition (Liquid Ice)", cost_gp: Some(400.0_f64), weight_lbs: Some(20.0_f64), description: None }, // uc_equip_arms_armor.lst:33
    EquipmentTableEntry { key: "Siege Engine Ammunition (Plague Bundle)", category: EquipmentCategory::ArmsArmor, name: "Siege Engine Ammunition (Plague Bundle)", cost_gp: Some(80.0_f64), weight_lbs: Some(20.0_f64), description: None }, // uc_equip_arms_armor.lst:34
    EquipmentTableEntry { key: "Siege Engine Ammunition (Smoke Shot)", category: EquipmentCategory::ArmsArmor, name: "Siege Engine Ammunition (Smoke Shot)", cost_gp: Some(250.0_f64), weight_lbs: Some(20.0_f64), description: None }, // uc_equip_arms_armor.lst:35
    EquipmentTableEntry { key: "Tracer Bullet", category: EquipmentCategory::ArmsArmor, name: "Tracer Bullet", cost_gp: Some(100.0_f64), weight_lbs: None, description: Some("These +1 firearm bullets deal no damage, but instead cause a pale glow to outline the target, granting the effect of a faerie fire spell and causing the target to take a -2 penalty to AC against ranged attacks. These effects last for 1d4 rounds.") }, // uc_equip_arms_armor.lst:36
    EquipmentTableEntry { key: "Do-maru", category: EquipmentCategory::ArmsArmor, name: "Do-maru", cost_gp: Some(200.0_f64), weight_lbs: Some(30.0_f64), description: None }, // uc_equip_arms_armor.lst:41
    EquipmentTableEntry { key: "Four-mirror Armor", category: EquipmentCategory::ArmsArmor, name: "Four-mirror Armor", cost_gp: Some(125.0_f64), weight_lbs: Some(45.0_f64), description: None }, // uc_equip_arms_armor.lst:42
    EquipmentTableEntry { key: "Haramaki", category: EquipmentCategory::ArmsArmor, name: "Haramaki", cost_gp: Some(3.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:43
    EquipmentTableEntry { key: "Kusari Gusoku", category: EquipmentCategory::ArmsArmor, name: "Kusari Gusoku", cost_gp: Some(350.0_f64), weight_lbs: Some(45.0_f64), description: None }, // uc_equip_arms_armor.lst:44
    EquipmentTableEntry { key: "Kikko Armor", category: EquipmentCategory::ArmsArmor, name: "Kikko Armor", cost_gp: Some(250.0_f64), weight_lbs: Some(25.0_f64), description: None }, // uc_equip_arms_armor.lst:45
    EquipmentTableEntry { key: "Lamellar (Horn)", category: EquipmentCategory::ArmsArmor, name: "Lamellar (Horn)", cost_gp: Some(100.0_f64), weight_lbs: Some(30.0_f64), description: None }, // uc_equip_arms_armor.lst:46
    EquipmentTableEntry { key: "Lamellar (Iron)", category: EquipmentCategory::ArmsArmor, name: "Lamellar (Iron)", cost_gp: Some(200.0_f64), weight_lbs: Some(50.0_f64), description: None }, // uc_equip_arms_armor.lst:47
    EquipmentTableEntry { key: "Lamellar (Stone)", category: EquipmentCategory::ArmsArmor, name: "Lamellar (Stone)", cost_gp: Some(500.0_f64), weight_lbs: Some(45.0_f64), description: None }, // uc_equip_arms_armor.lst:48
    EquipmentTableEntry { key: "Lamellar (Leather)", category: EquipmentCategory::ArmsArmor, name: "Lamellar (Leather)", cost_gp: Some(60.0_f64), weight_lbs: Some(25.0_f64), description: None }, // uc_equip_arms_armor.lst:49
    EquipmentTableEntry { key: "Lamellar (Steel)", category: EquipmentCategory::ArmsArmor, name: "Lamellar (Steel)", cost_gp: Some(150.0_f64), weight_lbs: Some(35.0_f64), description: None }, // uc_equip_arms_armor.lst:50
    EquipmentTableEntry { key: "Lamellar Cuirass", category: EquipmentCategory::ArmsArmor, name: "Lamellar Cuirass", cost_gp: Some(15.0_f64), weight_lbs: Some(8.0_f64), description: None }, // uc_equip_arms_armor.lst:51
    EquipmentTableEntry { key: "Mountain Pattern Armor", category: EquipmentCategory::ArmsArmor, name: "Mountain Pattern Armor", cost_gp: Some(250.0_f64), weight_lbs: Some(40.0_f64), description: None }, // uc_equip_arms_armor.lst:52
    EquipmentTableEntry { key: "O-yoroi", category: EquipmentCategory::ArmsArmor, name: "O-yoroi", cost_gp: Some(1700.0_f64), weight_lbs: Some(45.0_f64), description: None }, // uc_equip_arms_armor.lst:53
    EquipmentTableEntry { key: "Silken Ceremonial Armor", category: EquipmentCategory::ArmsArmor, name: "Silken Ceremonial Armor", cost_gp: Some(30.0_f64), weight_lbs: Some(4.0_f64), description: None }, // uc_equip_arms_armor.lst:54
    EquipmentTableEntry { key: "Tatami-do", category: EquipmentCategory::ArmsArmor, name: "Tatami-do", cost_gp: Some(1000.0_f64), weight_lbs: Some(45.0_f64), description: None }, // uc_equip_arms_armor.lst:55
    EquipmentTableEntry { key: "Aklys (Gladiator)", category: EquipmentCategory::ArmsArmor, name: "Aklys (Gladiator)", cost_gp: Some(5.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:60
    EquipmentTableEntry { key: "Amentum (Javelin)", category: EquipmentCategory::ArmsArmor, name: "Amentum (Javelin)", cost_gp: None, weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:61
    EquipmentTableEntry { key: "Atlatl", category: EquipmentCategory::ArmsArmor, name: "Atlatl", cost_gp: Some(2.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:62
    EquipmentTableEntry { key: "Bo Staff", category: EquipmentCategory::ArmsArmor, name: "Bo Staff", cost_gp: Some(1.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:63
    EquipmentTableEntry { key: "Broadsword (Nine Ring)", category: EquipmentCategory::ArmsArmor, name: "Broadsword (Nine Ring)", cost_gp: Some(15.0_f64), weight_lbs: Some(4.0_f64), description: None }, // uc_equip_arms_armor.lst:64
    EquipmentTableEntry { key: "Butterfly Sword", category: EquipmentCategory::ArmsArmor, name: "Butterfly Sword", cost_gp: Some(20.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:65
    EquipmentTableEntry { key: "Dan Bong", category: EquipmentCategory::ArmsArmor, name: "Dan Bong", cost_gp: Some(1.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:66
    EquipmentTableEntry { key: "Double Chicken Saber", category: EquipmentCategory::ArmsArmor, name: "Double Chicken Saber", cost_gp: Some(12.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:67
    EquipmentTableEntry { key: "Emei Piercer", category: EquipmentCategory::ArmsArmor, name: "Emei Piercer", cost_gp: Some(3.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:68
    EquipmentTableEntry { key: "Fighting Fan", category: EquipmentCategory::ArmsArmor, name: "Fighting Fan", cost_gp: Some(5.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:69
    EquipmentTableEntry { key: "Flying Blade", category: EquipmentCategory::ArmsArmor, name: "Flying Blade", cost_gp: Some(40.0_f64), weight_lbs: Some(12.0_f64), description: None }, // uc_equip_arms_armor.lst:70
    EquipmentTableEntry { key: "Gladius", category: EquipmentCategory::ArmsArmor, name: "Gladius", cost_gp: Some(15.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:71
    EquipmentTableEntry { key: "Harpoon", category: EquipmentCategory::ArmsArmor, name: "Harpoon", cost_gp: Some(5.0_f64), weight_lbs: Some(16.0_f64), description: None }, // uc_equip_arms_armor.lst:72
    EquipmentTableEntry { key: "Hooked Axe", category: EquipmentCategory::ArmsArmor, name: "Hooked Axe", cost_gp: Some(20.0_f64), weight_lbs: Some(7.0_f64), description: None }, // uc_equip_arms_armor.lst:73
    EquipmentTableEntry { key: "Hooked Lance", category: EquipmentCategory::ArmsArmor, name: "Hooked Lance", cost_gp: Some(3.0_f64), weight_lbs: Some(10.0_f64), description: None }, // uc_equip_arms_armor.lst:74
    EquipmentTableEntry { key: "Iron Brush", category: EquipmentCategory::ArmsArmor, name: "Iron Brush", cost_gp: Some(2.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:75
    EquipmentTableEntry { key: "Jutte", category: EquipmentCategory::ArmsArmor, name: "Jutte", cost_gp: Some(8.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:76
    EquipmentTableEntry { key: "Kama (Double-Chained)", category: EquipmentCategory::ArmsArmor, name: "Kama (Double-Chained)", cost_gp: Some(8.0_f64), weight_lbs: Some(4.0_f64), description: None }, // uc_equip_arms_armor.lst:77
    EquipmentTableEntry { key: "Katana", category: EquipmentCategory::ArmsArmor, name: "Katana", cost_gp: Some(50.0_f64), weight_lbs: Some(6.0_f64), description: None }, // uc_equip_arms_armor.lst:78
    EquipmentTableEntry { key: "Katana (Double Walking Stick)", category: EquipmentCategory::ArmsArmor, name: "Katana (Double Walking Stick)", cost_gp: Some(50.0_f64), weight_lbs: Some(6.0_f64), description: None }, // uc_equip_arms_armor.lst:79
    EquipmentTableEntry { key: "Kerambit", category: EquipmentCategory::ArmsArmor, name: "Kerambit", cost_gp: Some(2.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:80
    EquipmentTableEntry { key: "Kestros", category: EquipmentCategory::ArmsArmor, name: "Kestros", cost_gp: Some(1.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:81
    EquipmentTableEntry { key: "Knuckle Axe", category: EquipmentCategory::ArmsArmor, name: "Knuckle Axe", cost_gp: Some(9.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:82
    EquipmentTableEntry { key: "Kusarigama (Sickle and Chain)", category: EquipmentCategory::ArmsArmor, name: "Kusarigama (Sickle and Chain)", cost_gp: Some(12.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:83
    EquipmentTableEntry { key: "Kyoketsu Shoge", category: EquipmentCategory::ArmsArmor, name: "Kyoketsu Shoge", cost_gp: Some(6.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:84
    EquipmentTableEntry { key: "Lungchuan Tamo", category: EquipmentCategory::ArmsArmor, name: "Lungchuan Tamo", cost_gp: Some(5.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:85
    EquipmentTableEntry { key: "Madu (Leather)", category: EquipmentCategory::ArmsArmor, name: "Madu (Leather)", cost_gp: Some(40.0_f64), weight_lbs: Some(5.0_f64), description: None }, // uc_equip_arms_armor.lst:86
    EquipmentTableEntry { key: "Madu (Steel)", category: EquipmentCategory::ArmsArmor, name: "Madu (Steel)", cost_gp: Some(40.0_f64), weight_lbs: Some(6.0_f64), description: None }, // uc_equip_arms_armor.lst:87
    EquipmentTableEntry { key: "Mattock", category: EquipmentCategory::ArmsArmor, name: "Mattock", cost_gp: Some(12.0_f64), weight_lbs: Some(12.0_f64), description: None }, // uc_equip_arms_armor.lst:88
    EquipmentTableEntry { key: "Mere Club", category: EquipmentCategory::ArmsArmor, name: "Mere Club", cost_gp: Some(2.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:89
    EquipmentTableEntry { key: "Meteor Hammer", category: EquipmentCategory::ArmsArmor, name: "Meteor Hammer", cost_gp: Some(10.0_f64), weight_lbs: Some(10.0_f64), description: None }, // uc_equip_arms_armor.lst:90
    EquipmentTableEntry { key: "Monk's Spade", category: EquipmentCategory::ArmsArmor, name: "Monk's Spade", cost_gp: Some(20.0_f64), weight_lbs: Some(12.0_f64), description: None }, // uc_equip_arms_armor.lst:91
    EquipmentTableEntry { key: "Naginata", category: EquipmentCategory::ArmsArmor, name: "Naginata", cost_gp: Some(35.0_f64), weight_lbs: Some(9.0_f64), description: None }, // uc_equip_arms_armor.lst:92
    EquipmentTableEntry { key: "Nine-Section Whip", category: EquipmentCategory::ArmsArmor, name: "Nine-Section Whip", cost_gp: Some(8.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:93
    EquipmentTableEntry { key: "Nodachi", category: EquipmentCategory::ArmsArmor, name: "Nodachi", cost_gp: Some(60.0_f64), weight_lbs: Some(8.0_f64), description: None }, // uc_equip_arms_armor.lst:94
    EquipmentTableEntry { key: "Pata", category: EquipmentCategory::ArmsArmor, name: "Pata", cost_gp: Some(14.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:95
    EquipmentTableEntry { key: "Poisoned Sand Tube", category: EquipmentCategory::ArmsArmor, name: "Poisoned Sand Tube", cost_gp: Some(1.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:96
    EquipmentTableEntry { key: "Quadrens", category: EquipmentCategory::ArmsArmor, name: "Quadrens", cost_gp: Some(8.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:97
    EquipmentTableEntry { key: "Rhomphaia", category: EquipmentCategory::ArmsArmor, name: "Rhomphaia", cost_gp: Some(15.0_f64), weight_lbs: Some(10.0_f64), description: None }, // uc_equip_arms_armor.lst:98
    EquipmentTableEntry { key: "Rope Dart", category: EquipmentCategory::ArmsArmor, name: "Rope Dart", cost_gp: Some(1.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:99
    EquipmentTableEntry { key: "Sansetsukon", category: EquipmentCategory::ArmsArmor, name: "Sansetsukon", cost_gp: Some(8.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:100
    EquipmentTableEntry { key: "Scizore", category: EquipmentCategory::ArmsArmor, name: "Scizore", cost_gp: Some(20.0_f64), weight_lbs: Some(3.0_f64), description: Some("The Scizore offers a bonus to AC when not attacking with it, or a -1 to hit if using it to attack.") }, // uc_equip_arms_armor.lst:101
    EquipmentTableEntry { key: "Scorpion Whip", category: EquipmentCategory::ArmsArmor, name: "Scorpion Whip", cost_gp: Some(5.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:102
    EquipmentTableEntry { key: "Shang Gou", category: EquipmentCategory::ArmsArmor, name: "Shang Gou", cost_gp: Some(6.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:103
    EquipmentTableEntry { key: "Shotel", category: EquipmentCategory::ArmsArmor, name: "Shotel", cost_gp: Some(30.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:104
    EquipmentTableEntry { key: "Sibat", category: EquipmentCategory::ArmsArmor, name: "Sibat", cost_gp: Some(2.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:105
    EquipmentTableEntry { key: "Sica", category: EquipmentCategory::ArmsArmor, name: "Sica", cost_gp: Some(10.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:106
    EquipmentTableEntry { key: "Sword (Seven-Branched)", category: EquipmentCategory::ArmsArmor, name: "Sword (Seven-Branched)", cost_gp: Some(50.0_f64), weight_lbs: Some(7.0_f64), description: None }, // uc_equip_arms_armor.lst:107
    EquipmentTableEntry { key: "Sword (Tri-Point Double-Edged)", category: EquipmentCategory::ArmsArmor, name: "Sword (Tri-Point Double-Edged)", cost_gp: Some(12.0_f64), weight_lbs: Some(14.0_f64), description: None }, // uc_equip_arms_armor.lst:108
    EquipmentTableEntry { key: "Taiaha", category: EquipmentCategory::ArmsArmor, name: "Taiaha", cost_gp: Some(10.0_f64), weight_lbs: Some(8.0_f64), description: None }, // uc_equip_arms_armor.lst:109
    EquipmentTableEntry { key: "Tekko-Kagi (Iron Claw)", category: EquipmentCategory::ArmsArmor, name: "Tekko-Kagi (Iron Claw)", cost_gp: Some(3.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:110
    EquipmentTableEntry { key: "Tepoztopilli", category: EquipmentCategory::ArmsArmor, name: "Tepoztopilli", cost_gp: Some(8.0_f64), weight_lbs: Some(8.0_f64), description: None }, // uc_equip_arms_armor.lst:111
    EquipmentTableEntry { key: "Terbutje", category: EquipmentCategory::ArmsArmor, name: "Terbutje", cost_gp: Some(5.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:112
    EquipmentTableEntry { key: "Terbutje (Great)", category: EquipmentCategory::ArmsArmor, name: "Terbutje (Great)", cost_gp: Some(12.0_f64), weight_lbs: Some(4.0_f64), description: None }, // uc_equip_arms_armor.lst:113
    EquipmentTableEntry { key: "Tetsubo", category: EquipmentCategory::ArmsArmor, name: "Tetsubo", cost_gp: Some(20.0_f64), weight_lbs: Some(10.0_f64), description: None }, // uc_equip_arms_armor.lst:114
    EquipmentTableEntry { key: "Throwing Shield", category: EquipmentCategory::ArmsArmor, name: "Throwing Shield", cost_gp: Some(50.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:115
    EquipmentTableEntry { key: "Tiger Fork", category: EquipmentCategory::ArmsArmor, name: "Tiger Fork", cost_gp: Some(5.0_f64), weight_lbs: Some(8.0_f64), description: None }, // uc_equip_arms_armor.lst:116
    EquipmentTableEntry { key: "Tonfa", category: EquipmentCategory::ArmsArmor, name: "Tonfa", cost_gp: Some(1.0_f64), weight_lbs: Some(1.0_f64), description: None }, // uc_equip_arms_armor.lst:117
    EquipmentTableEntry { key: "Tube Arrow Shooter", category: EquipmentCategory::ArmsArmor, name: "Tube Arrow Shooter", cost_gp: Some(3.0_f64), weight_lbs: Some(0.5_f64), description: None }, // uc_equip_arms_armor.lst:118
    EquipmentTableEntry { key: "Urumi", category: EquipmentCategory::ArmsArmor, name: "Urumi", cost_gp: Some(30.0_f64), weight_lbs: Some(6.0_f64), description: None }, // uc_equip_arms_armor.lst:119
    EquipmentTableEntry { key: "Wahaika", category: EquipmentCategory::ArmsArmor, name: "Wahaika", cost_gp: Some(3.0_f64), weight_lbs: Some(3.0_f64), description: None }, // uc_equip_arms_armor.lst:120
    EquipmentTableEntry { key: "Wakizashi", category: EquipmentCategory::ArmsArmor, name: "Wakizashi", cost_gp: Some(35.0_f64), weight_lbs: Some(2.0_f64), description: None }, // uc_equip_arms_armor.lst:121
    EquipmentTableEntry { key: "Wushu Dart (5)", category: EquipmentCategory::ArmsArmor, name: "Wushu Dart (5)", cost_gp: Some(1.0_f64), weight_lbs: None, description: None }, // uc_equip_arms_armor.lst:122
    EquipmentTableEntry { key: "Blunderbuss", category: EquipmentCategory::ArmsArmor, name: "Blunderbuss", cost_gp: Some(2000.0_f64), weight_lbs: Some(8.0_f64), description: Some("Misfire 1-2 (10 ft); Scatter 15' cone") }, // uc_equip_arms_armor.lst:128
    EquipmentTableEntry { key: "Buckler Gun", category: EquipmentCategory::ArmsArmor, name: "Buckler Gun", cost_gp: Some(750.0_f64), weight_lbs: Some(6.0_f64), description: Some("Misfire 1 (5 ft.)") }, // uc_equip_arms_armor.lst:129
    EquipmentTableEntry { key: "Culverin", category: EquipmentCategory::ArmsArmor, name: "Culverin", cost_gp: Some(4000.0_f64), weight_lbs: Some(40.0_f64), description: Some("Misfire 1 (10 ft.); Firing a culverin without support (such as a wall, a window, or a stand) imparts a -4 penalty on the attack rolls, and the wielder is knocked prone. A culverin uses 4 doses of black powder and grapeshot.") }, // uc_equip_arms_armor.lst:130
    EquipmentTableEntry { key: "Double Hackbut", category: EquipmentCategory::ArmsArmor, name: "Double Hackbut", cost_gp: Some(4000.0_f64), weight_lbs: Some(18.0_f64), description: Some("Misfire 1-2 (5 ft.)") }, // uc_equip_arms_armor.lst:131
    EquipmentTableEntry { key: "Fire Lance", category: EquipmentCategory::ArmsArmor, name: "Fire Lance", cost_gp: Some(25.0_f64), weight_lbs: Some(4.0_f64), description: Some("Misfire 1-4 (5 ft.)") }, // uc_equip_arms_armor.lst:132
    EquipmentTableEntry { key: "Musket", category: EquipmentCategory::ArmsArmor, name: "Musket", cost_gp: Some(1500.0_f64), weight_lbs: Some(9.0_f64), description: Some("Misfire 1-2 (5 ft)") }, // uc_equip_arms_armor.lst:133
    EquipmentTableEntry { key: "Musket (Axe)", category: EquipmentCategory::ArmsArmor, name: "Axe Musket", cost_gp: Some(1600.0_f64), weight_lbs: Some(6.0_f64), description: Some("Misfire 1-2 (5 ft.)") }, // uc_equip_arms_armor.lst:134
    EquipmentTableEntry { key: "Musket (Double-Barreled)", category: EquipmentCategory::ArmsArmor, name: "Double-Barreled Musket", cost_gp: Some(2500.0_f64), weight_lbs: Some(11.0_f64), description: Some("Misfire 1-3 (5 ft.)") }, // uc_equip_arms_armor.lst:135
    EquipmentTableEntry { key: "Musket (Warhammer)", category: EquipmentCategory::ArmsArmor, name: "Warhammer Musket", cost_gp: Some(1600.0_f64), weight_lbs: Some(6.0_f64), description: Some("Misfire 1-2 (5 ft.)") }, // uc_equip_arms_armor.lst:136
    EquipmentTableEntry { key: "Pepperbox", category: EquipmentCategory::ArmsArmor, name: "Pepperbox", cost_gp: Some(3000.0_f64), weight_lbs: Some(5.0_f64), description: Some("Misfire 1-2 (5 ft.)") }, // uc_equip_arms_armor.lst:137
    EquipmentTableEntry { key: "Pistol", category: EquipmentCategory::ArmsArmor, name: "Pistol", cost_gp: Some(1000.0_f64), weight_lbs: Some(4.0_f64), description: Some("Misfire 1 (5 ft)") }, // uc_equip_arms_armor.lst:138
    EquipmentTableEntry { key: "Pistol (Coat)", category: EquipmentCategory::ArmsArmor, name: "Coat Pistol", cost_gp: Some(750.0_f64), weight_lbs: Some(1.0_f64), description: Some("Misfire 1 (5 ft.)") }, // uc_equip_arms_armor.lst:139
    EquipmentTableEntry { key: "Pistol (Dagger)", category: EquipmentCategory::ArmsArmor, name: "Dagger Pistol", cost_gp: Some(740.0_f64), weight_lbs: Some(1.0_f64), description: Some("Misfire 1 (5 ft.)") }, // uc_equip_arms_armor.lst:140
    EquipmentTableEntry { key: "Pistol (Double-Barreled)", category: EquipmentCategory::ArmsArmor, name: "Double-Barreled Pistol", cost_gp: Some(1750.0_f64), weight_lbs: Some(5.0_f64), description: Some("Misfire 1-2 (5 ft.)") }, // uc_equip_arms_armor.lst:141
    EquipmentTableEntry { key: "Pistol (Dragon)", category: EquipmentCategory::ArmsArmor, name: "Dragon Pistol", cost_gp: Some(1000.0_f64), weight_lbs: Some(3.0_f64), description: Some("Misfire 1-2 (5 ft.); Scatter 15' cone") }, // uc_equip_arms_armor.lst:142
    EquipmentTableEntry { key: "Pistol (Sword Cane)", category: EquipmentCategory::ArmsArmor, name: "Sword Cane Pistol", cost_gp: Some(775.0_f64), weight_lbs: Some(1.0_f64), description: Some("Misfire 1 (5 ft.)") }, // uc_equip_arms_armor.lst:143
    EquipmentTableEntry { key: "Revolver", category: EquipmentCategory::ArmsArmor, name: "Revolver", cost_gp: Some(4000.0_f64), weight_lbs: Some(4.0_f64), description: Some("Misfire 1") }, // uc_equip_arms_armor.lst:144
    EquipmentTableEntry { key: "Rifle", category: EquipmentCategory::ArmsArmor, name: "Rifle", cost_gp: Some(5000.0_f64), weight_lbs: Some(12.0_f64), description: Some("Misfire 1") }, // uc_equip_arms_armor.lst:145
    EquipmentTableEntry { key: "Rifle (Pepperbox)", category: EquipmentCategory::ArmsArmor, name: "Pepperbox Rifle", cost_gp: Some(7000.0_f64), weight_lbs: Some(15.0_f64), description: Some("Misfire 1-2") }, // uc_equip_arms_armor.lst:146
    EquipmentTableEntry { key: "Shotgun", category: EquipmentCategory::ArmsArmor, name: "Shotgun", cost_gp: Some(5000.0_f64), weight_lbs: Some(12.0_f64), description: Some("Misfire 1-2; Scatter 30' cone") }, // uc_equip_arms_armor.lst:147
    EquipmentTableEntry { key: "Shotgun (Double-Barreled)", category: EquipmentCategory::ArmsArmor, name: "Double-Barreled Shotgun", cost_gp: Some(7000.0_f64), weight_lbs: Some(15.0_f64), description: Some("Misfire 1-2; Scatter 30' cone") }, // uc_equip_arms_armor.lst:148
    EquipmentTableEntry { key: "Battered Blunderbuss", category: EquipmentCategory::ArmsArmor, name: "Battered Blunderbuss", cost_gp: Some(40.0_f64), weight_lbs: Some(8.0_f64), description: Some("Misfire 1-2 (10 ft); Scatter 15' cone") }, // uc_equip_arms_armor.lst:150
    EquipmentTableEntry { key: "Battered Musket", category: EquipmentCategory::ArmsArmor, name: "Battered Musket", cost_gp: Some(40.0_f64), weight_lbs: Some(9.0_f64), description: Some("Misfire 1-2 (5 ft)") }, // uc_equip_arms_armor.lst:151
    EquipmentTableEntry { key: "Battered Pistol", category: EquipmentCategory::ArmsArmor, name: "Battered Pistol", cost_gp: Some(40.0_f64), weight_lbs: Some(4.0_f64), description: Some("Misfire 1 (5 ft)") }, // uc_equip_arms_armor.lst:152
    EquipmentTableEntry { key: "Ballista (Gate Breaker)", category: EquipmentCategory::ArmsArmor, name: "Ballista (Gate Breaker)", cost_gp: Some(1200.0_f64), weight_lbs: None, description: Some("Crew 5, Aim 3, Load 5, Speed 0 ft.") }, // uc_equip_arms_armor.lst:158
    EquipmentTableEntry { key: "Ballista (Heavy)", category: EquipmentCategory::ArmsArmor, name: "Ballista (Heavy)", cost_gp: Some(800.0_f64), weight_lbs: None, description: Some("Crew 3, Aim 2, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:159
    EquipmentTableEntry { key: "Ballista (Light)", category: EquipmentCategory::ArmsArmor, name: "Ballista (Light)", cost_gp: Some(500.0_f64), weight_lbs: None, description: Some("Crew 1, Aim 0, Load 2, Speed 10 ft.") }, // uc_equip_arms_armor.lst:160
    EquipmentTableEntry { key: "Bombard (Heavy)", category: EquipmentCategory::ArmsArmor, name: "Bombard (Heavy)", cost_gp: Some(16000.0_f64), weight_lbs: None, description: Some(" Range (100 ft. min.); Crew 4, Aim 3, Load 5, Speed 0 ft.") }, // uc_equip_arms_armor.lst:161
    EquipmentTableEntry { key: "Bombard (Light)", category: EquipmentCategory::ArmsArmor, name: "Bombard (Light)", cost_gp: Some(6000.0_f64), weight_lbs: None, description: Some(" Range (50 ft. min.); Crew 2, Aim 1, Load 3, Speed 10 ft.") }, // uc_equip_arms_armor.lst:162
    EquipmentTableEntry { key: "Bombard (Standard)", category: EquipmentCategory::ArmsArmor, name: "Bombard (Standard)", cost_gp: Some(8000.0_f64), weight_lbs: None, description: Some(" Range (100 ft. min.); Crew 2, Aim 1, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:163
    EquipmentTableEntry { key: "Cannon", category: EquipmentCategory::ArmsArmor, name: "Cannon", cost_gp: Some(6000.0_f64), weight_lbs: None, description: Some("Crew 2, Aim 1, Load 3, Speed 10 ft.") }, // uc_equip_arms_armor.lst:164
    EquipmentTableEntry { key: "Cannon (Fiend's Mouth)", category: EquipmentCategory::ArmsArmor, name: "Cannon (Fiend's Mouth)", cost_gp: Some(9000.0_f64), weight_lbs: None, description: Some("Crew 3, Aim 1, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:165
    EquipmentTableEntry { key: "Catapult (Heavy)", category: EquipmentCategory::ArmsArmor, name: "Catapult (Heavy)", cost_gp: Some(1000.0_f64), weight_lbs: None, description: Some(" Range (100 ft. min.); Crew 4, Aim 3, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:166
    EquipmentTableEntry { key: "Catapult (Light)", category: EquipmentCategory::ArmsArmor, name: "Catapult (Light)", cost_gp: Some(550.0_f64), weight_lbs: None, description: Some(" Range (50 ft. min.); Crew 2, Aim 2, Load 3, Speed 10 ft.") }, // uc_equip_arms_armor.lst:167
    EquipmentTableEntry { key: "Catapult (Standard)", category: EquipmentCategory::ArmsArmor, name: "Catapult (Standard)", cost_gp: Some(800.0_f64), weight_lbs: None, description: Some(" Range (100 ft. min.); Crew 3, Aim 2, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:168
    EquipmentTableEntry { key: "Firedrake", category: EquipmentCategory::ArmsArmor, name: "Firedrake", cost_gp: Some(4000.0_f64), weight_lbs: None, description: Some("Crew 3, Aim 2, Load 5, Speed 10 ft.") }, // uc_equip_arms_armor.lst:169
    EquipmentTableEntry { key: "Firewyrm", category: EquipmentCategory::ArmsArmor, name: "Firewyrm", cost_gp: Some(6000.0_f64), weight_lbs: None, description: Some("Crew 5, Aim 2, Load 6, Speed 0 ft.") }, // uc_equip_arms_armor.lst:170
    EquipmentTableEntry { key: "Springal (Arrow)", category: EquipmentCategory::ArmsArmor, name: "Springal (Arrow)", cost_gp: Some(1000.0_f64), weight_lbs: None, description: Some(" Range (50 ft. min.); Crew 3, Aim 2, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:171
    EquipmentTableEntry { key: "Springal (Rocket)", category: EquipmentCategory::ArmsArmor, name: "Springal (Rocket)", cost_gp: Some(6000.0_f64), weight_lbs: None, description: Some(" Range (50 ft. min.); Crew 3, Aim 2, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:172
    EquipmentTableEntry { key: "Trebuchet (Heavy)", category: EquipmentCategory::ArmsArmor, name: "Trebuchet (Heavy)", cost_gp: Some(1500.0_f64), weight_lbs: None, description: Some(" Range (200 ft. min.); Crew 4, Aim 3, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:173
    EquipmentTableEntry { key: "Trebuchet (Light)", category: EquipmentCategory::ArmsArmor, name: "Trebuchet (Light)", cost_gp: Some(800.0_f64), weight_lbs: None, description: Some(" Range (100 ft. min.); Crew 3, Aim 2, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:174
    EquipmentTableEntry { key: "Trebuchet (Standard)", category: EquipmentCategory::ArmsArmor, name: "Trebuchet (Standard)", cost_gp: Some(1000.0_f64), weight_lbs: None, description: Some(" Range (150 ft. min.); Crew 4, Aim 2, Load 3, Speed 0 ft.") }, // uc_equip_arms_armor.lst:175
];

const EQUIPMODS_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry { key: "Special Quality ~ Thrown", category: EquipmentCategory::Equipmods, name: "Thrown", cost_gp: None, weight_lbs: None, description: Some("Weapon can be thrown") }, // uc_equipmods.lst:6
    EquipmentTableEntry { key: "Special Quality ~ Scatter ~ Firearm", category: EquipmentCategory::Equipmods, name: "Scatter", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("scatter") }, // uc_equipmods.lst:10
    EquipmentTableEntry { key: "Special Ability ~ Dry Load ~ Firearm / Ammunition", category: EquipmentCategory::Equipmods, name: "Dry Load", cost_gp: Some(30.0_f64), weight_lbs: None, description: Some("This special ability can only be applied to alchemical or metal firearm cartridges. Dry load cartridges can be used to load guns underwater or in other airless environments, such as a vacuum. This ability protects the cartridge's contents as it is being loaded and creates a residual bubble of air that surrounds the firearm, further protecting the ammunition and allowing the firearm with this ammunition to be fired in an airless environment. After the cartridge is loaded, the bubble of air lasts for 3 minutes, or until the firearm is fired, whichever occurs first. A firearm loaded with this ammunition still takes the -2 penalty on attack rolls when fired underwater for every 5 feet of water the bullet passes through, in addition to the normal penalties to range. When firing a dry load cartridge underwater or in an airless environment, a misfire that results in a firearm explosion occurs normally.") }, // uc_equipmods.lst:14
    EquipmentTableEntry { key: "Special Ability ~ Lucky ~ Firearm", category: EquipmentCategory::Equipmods, name: "Lucky", cost_gp: None, weight_lbs: None, description: Some("A lucky firearm has its own magical reservoir of grit (see page 9). Usually this grit is stored within the marks of an engraving or in a trinket that hangs from the firearm. Often these marks take the form of holy symbols or fetishes, but such a reservoir can take just about any form. This reservoir holds 1 grit point, which is refreshed at the start of each day. Whether or not the wielder of a lucky firearm has any deeds (see page 10), she can always spend 1 grit point from the lucky firearm to reroll an attack from it that would result in a misfire. When the wielder does so, she must take the second result, even if that attack roll also results in a misfire.") }, // uc_equipmods.lst:19
    EquipmentTableEntry { key: "Special Ability ~ Lucky / Greater ~ Firearm", category: EquipmentCategory::Equipmods, name: "Lucky (Greater)", cost_gp: None, weight_lbs: None, description: Some("A greater lucky firearm has its own magical reservoir of grit (see page 9). Usually this grit is stored within the marks of an engraving or in a trinket that hangs from the firearm. Often these marks take the form of holy symbols or fetishes, but such a reservoir can take just about any form. This reservoir holds 3 grit points, which is refreshed at the start of each day. Whether or not the wielder of a lucky firearm has any deeds (see page 10), she can always spend 1 grit point from the lucky firearm to reroll an attack from it that would result in a misfire. When the wielder does so, she must take the second result, even if that attack roll also results in a misfire.") }, // uc_equipmods.lst:20
    EquipmentTableEntry { key: "Special Ability ~ Reliable ~ Firearm", category: EquipmentCategory::Equipmods, name: "Reliable", cost_gp: None, weight_lbs: None, description: Some("A reliable firearm is enchanted so that it is less likely to jam than other firearms. This enchantment reduces the misfire value of the affected firearm by 1 (minimum 0). This reduction occurs after any increases are calculated for firing with the broken condition, or for any other effect that might increase the misfire value of a firearm.") }, // uc_equipmods.lst:21
    EquipmentTableEntry { key: "Special Ability ~ Reliable / Greater ~ Firearm", category: EquipmentCategory::Equipmods, name: "Reliable (Greater)", cost_gp: None, weight_lbs: None, description: Some("A greater reliable firearm is enchanted so as to be less likely to jam than other firearms. It reduces the misfire value of the affected firearm by 4 (minimum 0). This reduction occurs after any increases are calculated for firing with the broken condition, or for any other effect that might increase the misfire value of a firearm. A firearm with greater reliable cannot have the reliable special ability.") }, // uc_equipmods.lst:22
    EquipmentTableEntry { key: "Special Quality ~ Blocking", category: EquipmentCategory::Equipmods, name: "Blocking", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("blocking") }, // uc_equipmods.lst:26
    EquipmentTableEntry { key: "Special Quality ~ Deadly", category: EquipmentCategory::Equipmods, name: "Deadly", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("deadly") }, // uc_equipmods.lst:27
    EquipmentTableEntry { key: "Special Quality ~ Distracting", category: EquipmentCategory::Equipmods, name: "Distracting", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("distracting") }, // uc_equipmods.lst:28
    EquipmentTableEntry { key: "Special Quality ~ Grapple", category: EquipmentCategory::Equipmods, name: "Grapple", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("grapple") }, // uc_equipmods.lst:29
    EquipmentTableEntry { key: "Special Quality ~ Fragile", category: EquipmentCategory::Equipmods, name: "Fragile", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("fragile") }, // uc_equipmods.lst:32
    EquipmentTableEntry { key: "Special Quality ~ Performance", category: EquipmentCategory::Equipmods, name: "Performance", cost_gp: Some(0.0_f64), weight_lbs: None, description: Some("performance") }, // uc_equipmods.lst:35
    EquipmentTableEntry { key: "Material ~ Bone", category: EquipmentCategory::Equipmods, name: "Bone", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // uc_equipmods.lst:39
    EquipmentTableEntry { key: "Material ~ Bronze", category: EquipmentCategory::Equipmods, name: "Bronze", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // uc_equipmods.lst:40
    EquipmentTableEntry { key: "Material ~ Gold", category: EquipmentCategory::Equipmods, name: "Gold", cost_gp: None, weight_lbs: None, description: None }, // uc_equipmods.lst:41
    EquipmentTableEntry { key: "Material ~ Stone", category: EquipmentCategory::Equipmods, name: "Stone", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // uc_equipmods.lst:42
    EquipmentTableEntry { key: "Material ~ Obsidian", category: EquipmentCategory::Equipmods, name: "Obsidian", cost_gp: Some(0.0_f64), weight_lbs: None, description: None }, // uc_equipmods.lst:43
    EquipmentTableEntry { key: "Special Quality ~ Throwing Shield ~ Shield", category: EquipmentCategory::Equipmods, name: "Throwing Shield", cost_gp: Some(50.0_f64), weight_lbs: None, description: Some("small 1d4 damage; medium 1d6 damage; range 20 ft., deals bludgeoning damage.") }, // uc_equipmods.lst:47
];

pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<EquipmentTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(
            GENERAL_TABLE.len() + MAGIC_ITEMS_TABLE.len() + ARMS_ARMOR_TABLE.len(),
        );
        all.extend_from_slice(GENERAL_TABLE);
        all.extend_from_slice(MAGIC_ITEMS_TABLE);
        all.extend_from_slice(ARMS_ARMOR_TABLE);
        all
    })
}

pub fn equipmod_tables() -> &'static [EquipmentTableEntry] {
    EQUIPMODS_TABLE
}

/// Resolves a UC equipment or equipment-modifier item by key.
pub fn equipment_resolve(key: &str) -> Option<&'static EquipmentTableEntry> {
    equipment_tables()
        .iter()
        .chain(equipmod_tables())
        .find(|entry| entry.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn record_counts_are_pinned() {
        assert_eq!(GENERAL_TABLE.len(), 26);
        assert_eq!(MAGIC_ITEMS_TABLE.len(), 10);
        assert_eq!(ARMS_ARMOR_TABLE.len(), 149);
        assert_eq!(equipment_tables().len(), 185);
        assert_eq!(EQUIPMODS_TABLE.len(), 19);
        assert_eq!(equipmod_tables().len(), 19);
    }

    #[test]
    fn keys_are_unique_within_this_book() {
        let keys: BTreeSet<&str> = equipment_tables()
            .iter()
            .chain(equipmod_tables())
            .map(|entry| entry.key)
            .collect();
        assert_eq!(
            keys.len(),
            204,
            "204 real records (185 equipment + 19 equipmods) must carry 204 distinct keys              within this book -- a collision here would silently merge two real items"
        );
    }

    #[test]
    fn field_coverage_is_pinned() {
        let report = field_coverage_report();
        assert_eq!(report.total_records, 185);
        assert_eq!(report.records_expected, 185);
        assert!(report.has_cost > 0);
        assert!(report.has_description > 0);
    }

    #[test]
    fn no_copy_alias_or_mod_row_reached_the_tables() {
        for entry in equipment_tables().iter().chain(equipmod_tables()) {
            assert!(
                !entry.key.ends_with(".MOD") && !entry.key.contains(".COPY="),
                "a raw PCGen modifier/alias suffix leaked into a real table entry: {:?}",
                entry
            );
        }
    }
}
