//! Ultimate Intrigue (UI) shared equipment tables -- full in-scope corpus
//! coverage. SD28-E24 slice 2, mirroring
//! `advanced_race_guide::equipment_tables`'s own established shape exactly
//! (own `EquipmentCategory` enum, `description` sourced from the corpus
//! `SPROP:` token -- `ui_equip_*.lst`/`ui_equipmods.lst` carry no `DESC:`
//! token anywhere, confirmed by direct grep: zero hits across all 4
//! files).
//!
//! Record coverage: every real, active (non-`.MOD`) record across
//! `ui_equip_general.lst` (32),
//! `ui_equip_arms_armor.lst` (14),
//! `ui_equip_magic_items.lst` (45) --
//! 91 equipment records total -- and `ui_equipmods.lst`
//! (7 equipment-modifier records, its own `work-inventory.json`
//! `equipment_modifier` kind, not counted in the 91 above).
//!
//! `key` is the corpus `KEY:` token when the row carries one, else its
//! display `name` -- the same fallback `advanced_race_guide::equipment_tables`
//! documents. `cost_gp`/`weight_lbs` are `None` when the corpus token is
//! absent or a non-numeric PCGen formula (e.g. `BASEQTY:`-scaled costs on
//! ammunition bundles) this table does not evaluate -- never a fabricated
//! flat number for a formula cost.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    General,
    ArmsArmor,
    MagicItems,
    Equipmods,
}

impl EquipmentCategory {
    pub const ALL: &'static [EquipmentCategory] = &[
        EquipmentCategory::General,
        EquipmentCategory::ArmsArmor,
        EquipmentCategory::MagicItems,
        EquipmentCategory::Equipmods,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. `None` when the
    /// token is absent or a non-numeric PCGen formula this table does not
    /// evaluate.
    pub cost_gp: Option<f64>,
    /// Weight in pounds from the corpus `WT:` token. `None` when the
    /// corpus record carries no `WT:` token (true for every
    /// `ui_equipmods.lst` record -- equipment *modifiers* carry no
    /// independent weight, matching CRB's/ACG's/ARG's own established
    /// finding).
    pub weight_lbs: Option<f64>,
    /// Descriptive text, sourced from the corpus `SPROP:` token. `None`
    /// only when the corpus record has no `SPROP:` token at all.
    pub description: Option<&'static str>,
}

/// 91 equipment records: `General` + `ArmsArmor` + `MagicItems`.
pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
    static TABLE: std::sync::OnceLock<Vec<EquipmentTableEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
    EquipmentTableEntry { key: "Code Rod", category: EquipmentCategory::General, name: "Code Rod", cost_gp: Some(0.1), weight_lbs: Some(1.0), description: None }, // ui_equip_general.lst:12
    EquipmentTableEntry { key: "Concealment Coin", category: EquipmentCategory::General, name: "Concealment Coin", cost_gp: Some(12.0), weight_lbs: Some(0.1), description: None }, // ui_equip_general.lst:13
    EquipmentTableEntry { key: "Esquire Attache Case", category: EquipmentCategory::General, name: "Esquire Attache Case", cost_gp: Some(100.0), weight_lbs: Some(8.0), description: None }, // ui_equip_general.lst:14
    EquipmentTableEntry { key: "Hollow Book", category: EquipmentCategory::General, name: "Hollow Book", cost_gp: Some(15.0), weight_lbs: Some(3.0), description: None }, // ui_equip_general.lst:15
    EquipmentTableEntry { key: "Perfume Kit", category: EquipmentCategory::General, name: "Perfume Kit", cost_gp: Some(40.0), weight_lbs: Some(10.0), description: None }, // ui_equip_general.lst:16
    EquipmentTableEntry { key: "Poison Lip Paint", category: EquipmentCategory::General, name: "Poison Lip Paint", cost_gp: Some(5.0), weight_lbs: None, description: None }, // ui_equip_general.lst:17
    EquipmentTableEntry { key: "Puzzle Heel (3 steps)", category: EquipmentCategory::General, name: "Puzzle Heel (3 steps)", cost_gp: Some(20.0), weight_lbs: None, description: Some("+4 on Sleight of Hand checks to conceal items, DC 20 Disable Device to open.") }, // ui_equip_general.lst:18
    EquipmentTableEntry { key: "Puzzle Heel (4 steps)", category: EquipmentCategory::General, name: "Puzzle Heel (4 steps)", cost_gp: Some(40.0), weight_lbs: None, description: Some("+6 on Sleight of Hand checks to conceal items, DC 25 Disable Device to open.") }, // ui_equip_general.lst:19
    EquipmentTableEntry { key: "Puzzle Heel (5 steps)", category: EquipmentCategory::General, name: "Puzzle Heel (5 steps)", cost_gp: Some(80.0), weight_lbs: None, description: Some("+8 on Sleight of Hand checks to conceal items, DC 30 Disable Device to open.") }, // ui_equip_general.lst:20
    EquipmentTableEntry { key: "Quick-Change Outfit", category: EquipmentCategory::General, name: "Quick-Change Outfit", cost_gp: Some(0.0), weight_lbs: Some(5.0), description: Some("Actual price is double the cost of the more expensive of the two outfits mimicked.") }, // ui_equip_general.lst:21
    EquipmentTableEntry { key: "Scroll Belt", category: EquipmentCategory::General, name: "Scroll Belt", cost_gp: Some(0.3), weight_lbs: None, description: None }, // ui_equip_general.lst:22
    EquipmentTableEntry { key: "Sentry Seeds", category: EquipmentCategory::General, name: "Sentry Seeds", cost_gp: Some(7.0), weight_lbs: Some(0.5), description: None }, // ui_equip_general.lst:23
    EquipmentTableEntry { key: "Subversive Vest", category: EquipmentCategory::General, name: "Subversive Vest", cost_gp: Some(45.0), weight_lbs: Some(1.0), description: Some("+2 on Sleight of Hand checks to conceal thieves' tools.") }, // ui_equip_general.lst:24
    EquipmentTableEntry { key: "Subversive Vestment", category: EquipmentCategory::General, name: "Subversive Vestment", cost_gp: Some(90.0), weight_lbs: Some(2.0), description: Some("+2 on Sleight of Hand checks to conceal thieves' tools and two 1-pint flasks.") }, // ui_equip_general.lst:25
    EquipmentTableEntry { key: "Vigilante's Kit", category: EquipmentCategory::General, name: "Vigilante's Kit", cost_gp: Some(8.0), weight_lbs: Some(22.0), description: Some("This kit includes a backpack, a belt pouch, a blanket, torches (10), trail rations (5 days), and a waterskin.") }, // ui_equip_general.lst:33
    EquipmentTableEntry { key: "Accuracy Lozenge", category: EquipmentCategory::General, name: "Accuracy Lozenge", cost_gp: Some(330.0), weight_lbs: None, description: Some("Accept alchemical burn to make second attack in a round at full attack bonus.") }, // ui_equip_general.lst:39
    EquipmentTableEntry { key: "Age Ointment", category: EquipmentCategory::General, name: "Age Ointment", cost_gp: Some(80.0), weight_lbs: None, description: Some("Reduces penalty to Disguise checks for looking younger or older by 2.") }, // ui_equip_general.lst:40
    EquipmentTableEntry { key: "Alchemical Dye Kit", category: EquipmentCategory::General, name: "Alchemical Dye Kit", cost_gp: Some(160.0), weight_lbs: Some(6.0), description: Some("Reduces time to create a disguise, may reduce penalty to appear as different race.") }, // ui_equip_general.lst:41
    EquipmentTableEntry { key: "Boar's Bellow", category: EquipmentCategory::General, name: "Boar's Bellow", cost_gp: Some(190.0), weight_lbs: None, description: Some("Allows alchemical inspiration on 1d4 Intimidate checks, makes Diplomacy checks more difficult.") }, // ui_equip_general.lst:42
    EquipmentTableEntry { key: "Efreeti Cord", category: EquipmentCategory::General, name: "Efreeti Cord", cost_gp: Some(50.0), weight_lbs: None, description: None }, // ui_equip_general.lst:43
    EquipmentTableEntry { key: "Efreeti Switch", category: EquipmentCategory::General, name: "Efreeti Switch", cost_gp: Some(25.0), weight_lbs: None, description: None }, // ui_equip_general.lst:44
    EquipmentTableEntry { key: "Falsehood Fizz", category: EquipmentCategory::General, name: "Falsehood Fizz", cost_gp: Some(190.0), weight_lbs: None, description: Some("Allows alchemical inspiration on 1d4 Bluff checks, makes Sense Motive checks more difficult.") }, // ui_equip_general.lst:45
    EquipmentTableEntry { key: "Fellowship Film", category: EquipmentCategory::General, name: "Fellowship Film", cost_gp: Some(190.0), weight_lbs: None, description: Some("Allows alchemical inspiration on 1d4 Diplomacy checks, makes Intimidate checks more difficult.") }, // ui_equip_general.lst:46
    EquipmentTableEntry { key: "Flash Seeds", category: EquipmentCategory::General, name: "Flash Seeds", cost_gp: Some(35.0), weight_lbs: Some(0.5), description: None }, // ui_equip_general.lst:47
    EquipmentTableEntry { key: "Intuition Serum", category: EquipmentCategory::General, name: "Intuition Serum", cost_gp: Some(190.0), weight_lbs: None, description: Some("Allows alchemical inspiration on 1d4 Sense Motive checks, makes Bluff checks more difficult.") }, // ui_equip_general.lst:48
    EquipmentTableEntry { key: "Night Stalker's Tonic", category: EquipmentCategory::General, name: "Night Stalker's Tonic", cost_gp: Some(200.0), weight_lbs: None, description: Some("Accept alchemical burn to deal more sneak attack or hidden strike damage for one round.") }, // ui_equip_general.lst:49
    EquipmentTableEntry { key: "Rake's Friend", category: EquipmentCategory::General, name: "Rake's Friend", cost_gp: Some(120.0), weight_lbs: None, description: Some("Accept alchemical burn to gain 1 grit or panache point for up to an hour.") }, // ui_equip_general.lst:50
    EquipmentTableEntry { key: "Singer's Solution", category: EquipmentCategory::General, name: "Singer's Solution", cost_gp: Some(120.0), weight_lbs: None, description: Some("Accept alchemical burn to gain an additional 4 rounds of bardic performance or raging song for up to an hour.") }, // ui_equip_general.lst:51
    EquipmentTableEntry { key: "Speech Resin", category: EquipmentCategory::General, name: "Speech Resin", cost_gp: Some(410.0), weight_lbs: None, description: Some("Allows alchemical inspiration on 1d4 Charisma-based skill checks, makes Wisdom-based skill checks more difficult.") }, // ui_equip_general.lst:52
    EquipmentTableEntry { key: "Swift Hands Tonic", category: EquipmentCategory::General, name: "Swift Hands Tonic", cost_gp: Some(250.0), weight_lbs: None, description: Some("Allows alchemical inspiration on 1d4 Disable Device and Slight of Hand checks, makes Acrobatics and Escape Artist checks more difficult.") }, // ui_equip_general.lst:53
    EquipmentTableEntry { key: "Brilliant Plan Encumberance", category: EquipmentCategory::General, name: "Brilliant Plan Encumberance", cost_gp: Some(0.0), weight_lbs: Some(20.0), description: None }, // ui_equip_general.lst:59
    EquipmentTableEntry { key: "Darts (Featherweight/10)", category: EquipmentCategory::ArmsArmor, name: "Darts, Featherweight (10)", cost_gp: Some(1.0), weight_lbs: None, description: Some("Used in blowguns and wrist launchers to deliver poison. Any DR, hardness, etc., negates poison delivery.") }, // ui_equip_arms_armor.lst:13
    EquipmentTableEntry { key: "Spring Blade", category: EquipmentCategory::ArmsArmor, name: "Spring Blade", cost_gp: Some(70.0), weight_lbs: Some(1.0), description: Some("Free action to release the blade, move action to reset. DC 20 Perception check to spot when closed, +4 Sleight of Hand to conceal.") }, // ui_equip_arms_armor.lst:17
    EquipmentTableEntry { key: "Wrist Launcher", category: EquipmentCategory::ArmsArmor, name: "Wrist Launcher", cost_gp: Some(200.0), weight_lbs: Some(1.0), description: Some("Full-round action to reloaded, Sleight of Hand check to keep concealed after firing.") }, // ui_equip_arms_armor.lst:21
    EquipmentTableEntry { key: "Wrist Launcher (Heavy)", category: EquipmentCategory::ArmsArmor, name: "Wrist Launcher, Heavy", cost_gp: Some(250.0), weight_lbs: Some(2.0), description: Some("Full-round action to reloaded, Sleight of Hand check to keep concealed after firing.") }, // ui_equip_arms_armor.lst:22
    EquipmentTableEntry { key: "Diviner's Blight", category: EquipmentCategory::ArmsArmor, name: "Diviner's Blight", cost_gp: Some(10.0), weight_lbs: Some(15.0), description: None }, // ui_equip_arms_armor.lst:28
    EquipmentTableEntry { key: "Lockpick Shield", category: EquipmentCategory::ArmsArmor, name: "Lockpick Shield", cost_gp: Some(9.0), weight_lbs: Some(6.0), description: Some("Shield can attempt Disable Device check with +10 total bonus.") }, // ui_equip_arms_armor.lst:29
    EquipmentTableEntry { key: "Courtesan's Ire", category: EquipmentCategory::ArmsArmor, name: "Courtesan's Ire", cost_gp: Some(5.0), weight_lbs: Some(0.1), description: Some("Can fire blades in a 15-ft. cone 3/day, deals 3d4 damage, Reflex DC 14 for half.") }, // ui_equip_arms_armor.lst:35
    EquipmentTableEntry { key: "Dart of Recovery", category: EquipmentCategory::ArmsArmor, name: "Dart of Recovery", cost_gp: Some(206.0), weight_lbs: None, description: Some("Featherweight dart that user can recall as a swift action after use.") }, // ui_equip_arms_armor.lst:36
    EquipmentTableEntry { key: "Launcher of Distraction", category: EquipmentCategory::ArmsArmor, name: "Launcher of Distraction", cost_gp: Some(250.0), weight_lbs: Some(2.0), description: Some("Twice a day, the sound of this heavy wrist launcher can be made to appear to come from elsewhere.") }, // ui_equip_arms_armor.lst:37
    EquipmentTableEntry { key: "Mind's Eye Blade", category: EquipmentCategory::ArmsArmor, name: "Mind's Eye Blade", cost_gp: Some(2.0), weight_lbs: Some(1.0), description: Some("Wielder gains concealment from damaged targets for 1 round, 3/day may attempt to steal information during sneak attack or coup de grace.") }, // ui_equip_arms_armor.lst:38
    EquipmentTableEntry { key: "Prying Star", category: EquipmentCategory::ArmsArmor, name: "Prying Star", cost_gp: Some(646.0), weight_lbs: Some(0.1), description: Some("Plants a magical sensor on the target that is active for 9 minutes.") }, // ui_equip_arms_armor.lst:39
    EquipmentTableEntry { key: "Serpent's Fang", category: EquipmentCategory::ArmsArmor, name: "Serpent's Fang", cost_gp: Some(2.0), weight_lbs: Some(1.0), description: Some("Dagger holds up to 5 doses of poison in an extradimentional space, wielder can release one onto the blade as a swift action.") }, // ui_equip_arms_armor.lst:40
    EquipmentTableEntry { key: "Silent Sentry Crossbow", category: EquipmentCategory::ArmsArmor, name: "Silent Sentry Crossbow", cost_gp: Some(35.0), weight_lbs: Some(4.0), description: Some("Reduces sniping penalty by 5, 3/day can set an invisible alarm line up to 180 ft long.") }, // ui_equip_arms_armor.lst:41
    EquipmentTableEntry { key: "Mystic Bolts", category: EquipmentCategory::ArmsArmor, name: "Mystic Bolts", cost_gp: Some(0.0), weight_lbs: Some(0.0), description: Some("Damage by level") }, // ui_equip_arms_armor.lst:47
    EquipmentTableEntry { key: "Accent Pill", category: EquipmentCategory::MagicItems, name: "Accent Pill", cost_gp: Some(300.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:12
    EquipmentTableEntry { key: "Best Friend Pendant", category: EquipmentCategory::MagicItems, name: "Best Friend Pendant", cost_gp: Some(5000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:13
    EquipmentTableEntry { key: "Best Friend Pendant (Greater)", category: EquipmentCategory::MagicItems, name: "Best Friend Pendant (Greater)", cost_gp: Some(9000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:14
    EquipmentTableEntry { key: "Black Marketeer's Bag", category: EquipmentCategory::MagicItems, name: "Black Marketeer's Bag", cost_gp: Some(6200.0), weight_lbs: Some(3.0), description: None }, // ui_equip_magic_items.lst:15
    EquipmentTableEntry { key: "Candle of Comity", category: EquipmentCategory::MagicItems, name: "Candle of Comity", cost_gp: Some(2000.0), weight_lbs: Some(1.0), description: None }, // ui_equip_magic_items.lst:16
    EquipmentTableEntry { key: "Candle of Drowsiness", category: EquipmentCategory::MagicItems, name: "Candle of Drowsiness", cost_gp: Some(2500.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:17
    EquipmentTableEntry { key: "Coat of Pockets", category: EquipmentCategory::MagicItems, name: "Coat of Pockets", cost_gp: Some(2500.0), weight_lbs: Some(7.0), description: None }, // ui_equip_magic_items.lst:18
    EquipmentTableEntry { key: "Coat of the Undercity", category: EquipmentCategory::MagicItems, name: "Coat of the Undercity", cost_gp: Some(7500.0), weight_lbs: Some(5.0), description: None }, // ui_equip_magic_items.lst:19
    EquipmentTableEntry { key: "Coat of the Undercity (Greater)", category: EquipmentCategory::MagicItems, name: "Coat of the Undercity (Greater)", cost_gp: Some(37500.0), weight_lbs: Some(5.0), description: None }, // ui_equip_magic_items.lst:20
    EquipmentTableEntry { key: "Codex of Conversations", category: EquipmentCategory::MagicItems, name: "Codex of Conversations", cost_gp: Some(10000.0), weight_lbs: Some(3.0), description: None }, // ui_equip_magic_items.lst:21
    EquipmentTableEntry { key: "Costume Bureau", category: EquipmentCategory::MagicItems, name: "Costume Bureau", cost_gp: Some(7000.0), weight_lbs: Some(175.0), description: None }, // ui_equip_magic_items.lst:22
    EquipmentTableEntry { key: "Courier's Secure Pouch", category: EquipmentCategory::MagicItems, name: "Courier's Secure Pouch", cost_gp: Some(1600.0), weight_lbs: Some(2.0), description: None }, // ui_equip_magic_items.lst:23
    EquipmentTableEntry { key: "Communique Ring", category: EquipmentCategory::MagicItems, name: "Communique Ring", cost_gp: Some(6000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:24
    EquipmentTableEntry { key: "Deadened Shadows Cloak", category: EquipmentCategory::MagicItems, name: "Deadened Shadows Cloak", cost_gp: Some(63250.0), weight_lbs: Some(2.0), description: None }, // ui_equip_magic_items.lst:25
    EquipmentTableEntry { key: "Deck of Doors", category: EquipmentCategory::MagicItems, name: "Deck of Doors", cost_gp: Some(12150.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:26
    EquipmentTableEntry { key: "Fan of Flirting", category: EquipmentCategory::MagicItems, name: "Fan of Flirting", cost_gp: Some(1700.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:27
    EquipmentTableEntry { key: "Ghost Needle", category: EquipmentCategory::MagicItems, name: "Ghost Needle", cost_gp: Some(3600.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:28
    EquipmentTableEntry { key: "Glass of Veils", category: EquipmentCategory::MagicItems, name: "Glass of Veils", cost_gp: Some(53000.0), weight_lbs: Some(40.0), description: None }, // ui_equip_magic_items.lst:29
    EquipmentTableEntry { key: "Glittering Trinket (Belt Buckle)", category: EquipmentCategory::MagicItems, name: "Glittering Trinket (Belt Buckle)", cost_gp: Some(120.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:30
    EquipmentTableEntry { key: "Glittering Trinket (Bracelet)", category: EquipmentCategory::MagicItems, name: "Glittering Trinket (Bracelet)", cost_gp: Some(120.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:31
    EquipmentTableEntry { key: "Glittering Trinket (Necklace)", category: EquipmentCategory::MagicItems, name: "Glittering Trinket (Necklace)", cost_gp: Some(120.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:32
    EquipmentTableEntry { key: "Glittering Trinket (Ring)", category: EquipmentCategory::MagicItems, name: "Glittering Trinket (Ring)", cost_gp: Some(120.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:33
    EquipmentTableEntry { key: "Gloves of Unexpected Violence", category: EquipmentCategory::MagicItems, name: "Gloves of Unexpected Violence", cost_gp: Some(10000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:34
    EquipmentTableEntry { key: "Gloves of Unexpected Violence (Expert)", category: EquipmentCategory::MagicItems, name: "Gloves of Unexpected Violence (Expert)", cost_gp: Some(30000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:35
    EquipmentTableEntry { key: "Ink of Mimicry", category: EquipmentCategory::MagicItems, name: "Ink of Mimicry", cost_gp: Some(1950.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:36
    EquipmentTableEntry { key: "Magnificent Map", category: EquipmentCategory::MagicItems, name: "Magnificent Map", cost_gp: Some(7800.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:37
    EquipmentTableEntry { key: "Mask of Stolen Mien", category: EquipmentCategory::MagicItems, name: "Mask of Stolen Mien", cost_gp: Some(17200.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:38
    EquipmentTableEntry { key: "Memory Box", category: EquipmentCategory::MagicItems, name: "Memory Box", cost_gp: Some(28000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:39
    EquipmentTableEntry { key: "Monocle of Flawlessness", category: EquipmentCategory::MagicItems, name: "Monocle of Flawlessness", cost_gp: Some(8750.0), weight_lbs: None, description: Some("+2 on saves to disbelieve illusions.") }, // ui_equip_magic_items.lst:40
    EquipmentTableEntry { key: "Murderer's Silence", category: EquipmentCategory::MagicItems, name: "Murderer's Silence", cost_gp: Some(900.0), weight_lbs: Some(1.0), description: None }, // ui_equip_magic_items.lst:41
    EquipmentTableEntry { key: "Parley Ward", category: EquipmentCategory::MagicItems, name: "Parley Ward", cost_gp: Some(52000.0), weight_lbs: Some(2.0), description: None }, // ui_equip_magic_items.lst:42
    EquipmentTableEntry { key: "Pipe of Revealing Mists", category: EquipmentCategory::MagicItems, name: "Pipe of Revealing Mists", cost_gp: Some(2592.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:43
    EquipmentTableEntry { key: "Planar Parchment", category: EquipmentCategory::MagicItems, name: "Planar Parchment", cost_gp: Some(18000.0), weight_lbs: Some(1.0), description: None }, // ui_equip_magic_items.lst:44
    EquipmentTableEntry { key: "Polish of Inconspicuous Armor", category: EquipmentCategory::MagicItems, name: "Polish of Inconspicuous Armor", cost_gp: Some(750.0), weight_lbs: Some(0.5), description: None }, // ui_equip_magic_items.lst:45
    EquipmentTableEntry { key: "Private Palanquin", category: EquipmentCategory::MagicItems, name: "Private Palanquin", cost_gp: Some(63000.0), weight_lbs: Some(500.0), description: None }, // ui_equip_magic_items.lst:46
    EquipmentTableEntry { key: "Quick-Change Mask", category: EquipmentCategory::MagicItems, name: "Quick-Change Mask", cost_gp: Some(650.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:47
    EquipmentTableEntry { key: "Raucous Canard", category: EquipmentCategory::MagicItems, name: "Raucous Canard", cost_gp: Some(100.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:48
    EquipmentTableEntry { key: "Ring of the Shadow Victim", category: EquipmentCategory::MagicItems, name: "Ring of the Shadow Victim", cost_gp: Some(101000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:49
    EquipmentTableEntry { key: "Rings of Bondage", category: EquipmentCategory::MagicItems, name: "Rings of Bondage", cost_gp: Some(18200.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:50
    EquipmentTableEntry { key: "Rings of Bondage (Greater)", category: EquipmentCategory::MagicItems, name: "Rings of Bondage (Greater)", cost_gp: Some(42000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:51
    EquipmentTableEntry { key: "Shadow Hand Smoke Pellet", category: EquipmentCategory::MagicItems, name: "Shadow Hand Smoke Pellet", cost_gp: Some(800.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:52
    EquipmentTableEntry { key: "Swarmwalker's Ring", category: EquipmentCategory::MagicItems, name: "Swarmwalker's Ring", cost_gp: Some(26000.0), weight_lbs: None, description: None }, // ui_equip_magic_items.lst:53
    EquipmentTableEntry { key: "Time Bomb", category: EquipmentCategory::MagicItems, name: "Time Bomb", cost_gp: Some(1000.0), weight_lbs: Some(1.0), description: None }, // ui_equip_magic_items.lst:54
    EquipmentTableEntry { key: "Time Bomb (Greater)", category: EquipmentCategory::MagicItems, name: "Time Bomb (Greater)", cost_gp: Some(6000.0), weight_lbs: Some(1.0), description: None }, // ui_equip_magic_items.lst:55
    EquipmentTableEntry { key: "Vestments of False Faith", category: EquipmentCategory::MagicItems, name: "Vestments of False Faith", cost_gp: Some(27000.0), weight_lbs: Some(5.0), description: None }, // ui_equip_magic_items.lst:56
    EquipmentTableEntry { key: "Thieves' Tools (Concealable)", category: EquipmentCategory::General, name: "Thieves' Tools (Concealable)", cost_gp: Some(190.0), weight_lbs: Some(0.5), description: Some("+4 on Sleight of Hand checks to conceal these on your body.") }, // ui_equip_general.lst:26
        ]
    })
}

/// 7 equipment-modifier records from `ui_equipmods.lst`, this book's own
/// `work-inventory.json` `equipment_modifier` kind (which reports 14, not
/// 7 -- see below), not mixed into [`equipment_tables`].
///
/// **`work-inventory.json`'s 14 is an over-count, not a gap this table
/// should close.** `ui_equipmods.lst` declares each of these 7 real
/// modifiers twice: once under its real name (`Liberating`, `Peaceful`,
/// ...) with `KEY:Special Ability ~ <Name> ~ <Type>`, and once more as a
/// `VISIBLE:NO` `.COPY=` alias row (`Special Ability ~ Liberating ~
/// Melee.COPY=LIBERATING`, ...) -- `advanced_race_guide::equipment_tables`
/// already establishes the same exclusion for its own corpus's "Old KEYs"
/// `VISIBLE:NO`-alias block (see that module's own doc comment). The
/// classifier that produces `work-inventory.json` does not know about
/// `VISIBLE:NO` and counts both rows as distinct declared units -- the
/// classifier's notion of "a unit" being broader than reality
/// (`decisions.md §36`'s catalogued pattern), not a real 14-record gap.
pub fn equipmod_tables() -> &'static [EquipmentTableEntry] {
    static TABLE: std::sync::OnceLock<Vec<EquipmentTableEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
    EquipmentTableEntry { key: "Special Ability ~ Liberating ~ Melee", category: EquipmentCategory::Equipmods, name: "Liberating", cost_gp: None, weight_lbs: None, description: Some("Aids wielder in detecting when a creature is under an enchantment. Striking a mentally controlled deals nonlethal damage, grants new save with a bonus.") }, // ui_equipmods.lst:12
    EquipmentTableEntry { key: "Special Ability ~ Peaceful ~ Weapon", category: EquipmentCategory::Equipmods, name: "Peaceful", cost_gp: None, weight_lbs: None, description: Some("A creature that has taken nonlethal damage from a peaceful weapon becomes shaken for 1 round each time it deals lethal damage to another living creature.") }, // ui_equipmods.lst:13
    EquipmentTableEntry { key: "Special Ability ~ Silencing ~ Weapon", category: EquipmentCategory::Equipmods, name: "Silencing", cost_gp: None, weight_lbs: None, description: Some("A silencing weapon makes no noise when drawn or when used to attack. Targets struck can be muffled, somewhat. Critical hits silence target.") }, // ui_equipmods.lst:14
    EquipmentTableEntry { key: "Special Ability ~ Slithering ~ Melee", category: EquipmentCategory::Equipmods, name: "Slithering", cost_gp: None, weight_lbs: None, description: Some("Weapon ignores half of AC bonus from cover, half of penalty from squeezing, and is harder to disarm or sunder.") }, // ui_equipmods.lst:15
    EquipmentTableEntry { key: "Special Ability ~ Truthful ~ Melee", category: EquipmentCategory::Equipmods, name: "Truthful", cost_gp: None, weight_lbs: None, description: Some("Weapon ignores miss chance provided by illusions, such as blur, and acts as a dispelling weapon against figments and shadow conjuration effects.") }, // ui_equipmods.lst:16
    EquipmentTableEntry { key: "Special Ability ~ Umbral ~ Melee", category: EquipmentCategory::Equipmods, name: "Umbral", cost_gp: None, weight_lbs: None, description: Some("Weapon is considered to have concealment for the purposes of attacks or effects directed at the weapon, grants wielder darkvision, 30 ft, while held, radiates 20-ft radius of darkness on command.") }, // ui_equipmods.lst:17
    EquipmentTableEntry { key: "Special Ability ~ Unseen ~ Melee", category: EquipmentCategory::Equipmods, name: "Unseen", cost_gp: None, weight_lbs: None, description: Some("Weapon, and scabbard, become invisible on command, may deny unaware target Dex bonus to AC on first strike, has total concealment against attacks or effects directed at the weapon.") }, // ui_equipmods.lst:18
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_91_equipment_records() {
        assert_eq!(equipment_tables().len(), 91);
    }

    #[test]
    fn catalog_has_7_equipmod_records() {
        assert_eq!(equipmod_tables().len(), 7);
    }

    #[test]
    fn every_record_carries_a_real_key_and_name() {
        for e in equipment_tables().iter().chain(equipmod_tables()) {
            assert!(!e.key.is_empty());
            assert!(!e.name.is_empty());
        }
    }

    #[test]
    fn keys_are_unique_within_each_table() {
        let eq_keys: std::collections::BTreeSet<&str> = equipment_tables().iter().map(|e| e.key).collect();
        assert_eq!(eq_keys.len(), equipment_tables().len());
        let mod_keys: std::collections::BTreeSet<&str> = equipmod_tables().iter().map(|e| e.key).collect();
        assert_eq!(mod_keys.len(), equipmod_tables().len());
    }
}
