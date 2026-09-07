//! PF1 Wizard spell list — per-class spell-level overrides.
//!
//! Mirrors `sorcerer_spell_list.rs`'s own doc comment exactly, substituting
//! Wizard: `crb::spell_list::SPELL_LIST`'s `level` field is the MINIMUM
//! spell level across every class named in the corpus's `CLASSES:` tag for
//! that record, not necessarily the Wizard-specific level. This table
//! re-parses the same corpus record's raw `CLASSES:` tag directly,
//! isolating only the Wizard-specific level for each of the **580** real
//! records that name Wizard at all: 21 cantrips (0th level), then
//! 82 / 86 / 82 / 69 / 60 / 58 / 49 / 42 / 31 across levels 1-9 -- the real
//! PF1 Sorcerer/Wizard shared arcane list, and the largest spell list in
//! this codebase.
//!
//! **Why this table exists at all, given `sorcerer_spell_list`.** The two
//! lists are very nearly the same data -- 578 keys in common at *identical*
//! levels, no key on Sorcerer's list absent from Wizard's, and exactly two
//! Wizard-only records (`Mage's Lucubration` 6th and `Mnemonic Enhancer`
//! 4th, both `cr_spells.lst`). It is therefore tempting to derive Wizard
//! from Sorcerer the way `acg::hunter_spell_list` derives Hunter from Druid
//! and Ranger. That was deliberately NOT done: Hunter's derivation is
//! grounded in a corpus token that *states* the relationship
//! (`acg_classes.lst`'s `CLASS:Hunter ... SPELLLIST:2|Druid|Ranger`),
//! whereas the corpus states nothing of the kind about Wizard -- `CLASS:
//! Wizard` carries no `SPELLLIST:` token, and every one of these 580
//! records names `Wizard` in its own `CLASSES:` tag independently. The
//! Sorcerer/Wizard overlap is an observed property of the data, not a rule
//! the data asserts, so it is pinned as a regression test
//! (`the_sorcerer_overlap_is_an_observation_not_a_derivation`) rather than
//! used as a generation shortcut. If a future corpus revision splits the
//! two lists, this table stays correct and that test reports the change.
//!
//! Per-file: **396 from `cr_spells.lst` + 95 from `apg_spells.lst` + 89
//! from `acg_spells.lst` = 580**, all names distinct, no `.MOD` and no
//! `.COPY=` record assigns Wizard.
//!
//! **Wizard is always the LAST name in its `CLASSES:` comma group** (all
//! 580 records), and is mid-group -- i.e. not first -- in 578 of them.
//! That trailing position means a naive `Wizard=` substring grep would in
//! fact have found these records, unlike Sorcerer's; the token-split parse
//! is used anyway because the position is a corpus accident, not a
//! guarantee. There is no `Paladin`/`Antipaladin`-shaped collision here:
//! across all three ingested spell files the `CLASSES:` tags name only 18
//! distinct classes and none of them contains "Wizard" as a substring.
//!
//! **Two records carry an optional-rule gate on the level**
//! (`Malediction` and `Unravel Destiny`, both
//! `...,Wizard=3[PREVAREQ:Heroic,1]` -- the APG Hero Points rule). Both
//! are included per the lead's ruling (`risks-and-open-questions.md` item
//! 54); they are exactly the records a naive `int(level)` discards
//! silently, and they have their own regression test.
//!
//! Per-file ceiling check: `grep -c Wizard` returns 397 / 96 / 89 -- exact
//! in one file. The CRB excess is the `Fox's Cunning.MOD` record, which
//! matches only because its `DESC:` prose mentions a wizard (its base
//! record does name Wizard and IS counted); the APG excess is a
//! `###Block: Elemtalist Wizard Spells` section comment, not a record.
//!
//! **Corpus reachability: all 580 resolve** against the union of the three
//! ingested books' own `SPELL_LIST` tables -- never an invented name.
//!
//! Regenerate by parsing the `CLASSES:` token in all three spell files --
//! split the body on `|`, `rpartition` each group on `=`, strip any
//! trailing `[...]` gate from the level, then membership-test the
//! comma-separated name list. Never substring-match `Wizard=`, and never
//! let an `int()` on the level throw a record away silently.

/// (spell key, Wizard-specific spell level 0-9). A real Wizard may only
/// scribe or prepare a spell that appears in this table, subject to the
/// character's own spell-level access ceiling -- see `wizard_spell_level`
/// for the lookup helper.
pub const WIZARD_SPELL_LIST: &[(&str, u8)] = &[
    ("Accelerate Poison", 2),
    ("Acid Arrow", 2),
    ("Acid Fog", 6),
    ("Acid Pit", 4),
    ("Acid Splash", 0),
    ("Adhesive Blood", 2),
    ("Adhesive Spittle", 1),
    ("Adjustable Disguise", 3),
    ("Adjustable Polymorph", 4),
    ("Aggressive Thundercloud", 2),
    ("Aggressive Thundercloud (Greater)", 4),
    ("Air Geyser", 3),
    ("Air Step", 2),
    ("Alarm", 1),
    ("Alter Musical Instrument", 1),
    ("Alter Self", 2),
    ("Alter Winds", 1),
    ("Analyze Dweomer", 6),
    ("Anchored Step", 3),
    ("Animal Growth", 5),
    ("Animate Dead", 4),
    ("Animate Rope", 1),
    ("Ant Haul", 1),
    ("Antimagic Field", 6),
    ("Antipathy", 8),
    ("Aqueous Orb", 3),
    ("Arcane Eye", 4),
    ("Arcane Lock", 2),
    ("Arcane Mark", 0),
    ("Arcane Sight", 3),
    ("Arcane Sight (Greater)", 7),
    ("Arrow Eruption", 2),
    ("Astral Projection", 9),
    ("Aura Sight", 3),
    ("Baleful Polymorph", 5),
    ("Ball Lightning", 4),
    ("Banishment", 7),
    ("Banshee Blast", 6),
    ("Barrow Haze", 3),
    ("Bear's Endurance", 2),
    ("Bear's Endurance (Mass)", 6),
    ("Beast Shape I", 3),
    ("Beast Shape II", 4),
    ("Beast Shape III", 5),
    ("Beast Shape IV", 6),
    ("Bestow Curse", 4),
    ("Binding", 8),
    ("Black Tentacles", 4),
    ("Bleed", 0),
    ("Blight", 5),
    ("Blindness/Deafness", 2),
    ("Blink", 3),
    ("Bloatbomb", 4),
    ("Blood Armor", 2),
    ("Blood Biography", 3),
    ("Blood Sentinel", 3),
    ("Blur", 2),
    ("Blurred Movement", 1),
    ("Body Capacitance", 1),
    ("Break", 1),
    ("Break Enchantment", 5),
    ("Bull's Strength", 2),
    ("Bull's Strength (Mass)", 6),
    ("Bullet Ward", 2),
    ("Buoyancy", 2),
    ("Burning Gaze", 2),
    ("Burning Hands", 1),
    ("Calcific Touch", 4),
    ("Campfire Wall", 3),
    ("Cat's Grace", 2),
    ("Cat's Grace (Mass)", 6),
    ("Cause Fear", 1),
    ("Chain Lightning", 6),
    ("Charm Monster", 4),
    ("Charm Monster (Mass)", 8),
    ("Charm Person", 1),
    ("Chill Touch", 1),
    ("Circle of Death", 6),
    ("Clairaudience/Clairvoyance", 3),
    ("Clashing Rocks", 9),
    ("Clenched Fist", 8),
    ("Cloak of Dreams", 6),
    ("Cloak of Winds", 3),
    ("Clone", 8),
    ("Cloudkill", 5),
    ("Color Spray", 1),
    ("Command Undead", 2),
    ("Companion Life Link", 2),
    ("Comprehend Languages", 1),
    ("Cone of Cold", 5),
    ("Confusion", 4),
    ("Contact Other Plane", 5),
    ("Contagion", 4),
    ("Contagious Flame", 6),
    ("Contingency", 6),
    ("Contingent Action", 3),
    ("Contingent Scroll", 4),
    ("Continual Flame", 2),
    ("Control Undead", 7),
    ("Control Water", 6),
    ("Control Weather", 7),
    ("Crafter's Curse", 1),
    ("Crafter's Fortune", 1),
    ("Create Greater Undead", 8),
    ("Create Pit", 2),
    ("Create Treasure Map", 2),
    ("Create Undead", 6),
    ("Creeping Ice", 4),
    ("Crimson Confession", 2),
    ("Crushing Despair", 4),
    ("Crushing Hand", 9),
    ("Curse of Burning Sleep", 4),
    ("Dancing Lantern", 1),
    ("Dancing Lights", 0),
    ("Darkness", 2),
    ("Darkvision", 2),
    ("Daylight", 3),
    ("Daze", 0),
    ("Daze Monster", 2),
    ("Deep Slumber", 3),
    ("Deflection", 7),
    ("Delayed Blast Fireball", 7),
    ("Demand", 8),
    ("Detect Magic", 0),
    ("Detect Poison", 0),
    ("Detect Scrying", 4),
    ("Detect Secret Doors", 1),
    ("Detect Thoughts", 2),
    ("Detect Undead", 1),
    ("Detonate", 4),
    ("Devolution", 3),
    ("Dimension Door", 4),
    ("Dimensional Anchor", 4),
    ("Dimensional Bounce", 7),
    ("Dimensional Lock", 8),
    ("Disable Construct", 3),
    ("Discern Location", 8),
    ("Discern Next of Kin", 1),
    ("Disguise Self", 1),
    ("Disguise Weapon", 1),
    ("Disintegrate", 6),
    ("Dismissal", 5),
    ("Dispel Magic", 3),
    ("Dispel Magic (Greater)", 6),
    ("Displacement", 3),
    ("Disrupt Undead", 0),
    ("Dominate Monster", 9),
    ("Dominate Person", 5),
    ("Draconic Reservoir", 3),
    ("Dragon's Breath", 4),
    ("Dream", 5),
    ("Dust of Twilight", 2),
    ("Eagle's Splendor", 2),
    ("Eagle's Splendor (Mass)", 6),
    ("Elemental Aura", 3),
    ("Elemental Body I", 4),
    ("Elemental Body II", 5),
    ("Elemental Body III", 6),
    ("Elemental Body IV", 7),
    ("Elemental Speech", 2),
    ("Elemental Touch", 2),
    ("Enchantment Foil", 4),
    ("Endure Elements", 1),
    ("Enemy Hammer", 6),
    ("Energy Drain", 9),
    ("Enervation", 4),
    ("Enlarge Person", 1),
    ("Enlarge Person (Mass)", 4),
    ("Enter Image", 3),
    ("Erase", 1),
    ("Ethereal Jaunt", 7),
    ("Etherealness", 9),
    ("Euphoric Cloud", 2),
    ("Euphoric Tranquility", 8),
    ("Expeditious Excavation", 1),
    ("Expeditious Retreat", 1),
    ("Expend", 7),
    ("Explosive Runes", 3),
    ("Extreme Flexibility", 2),
    ("Eyebite", 6),
    ("Eyes of the Void", 4),
    ("Fabricate", 5),
    ("False Life", 2),
    ("False Vision", 5),
    ("Fear", 4),
    ("Feast on Fear", 5),
    ("Feather Fall", 1),
    ("Feeblemind", 5),
    ("Fiery Body", 9),
    ("Finger of Death", 7),
    ("Fire Breath", 2),
    ("Fire Shield", 4),
    ("Fire Snake", 5),
    ("Fire Trap", 4),
    ("Fireball", 3),
    ("Firebrand", 7),
    ("Firefall", 4),
    ("Flame Arrow", 3),
    ("Flaming Sphere", 2),
    ("Flaming Sphere (Greater)", 4),
    ("Flare", 0),
    ("Flare Burst", 1),
    ("Flesh to Stone", 6),
    ("Floating Disk", 1),
    ("Fluid Form", 6),
    ("Fly", 3),
    ("Fly (Mass)", 7),
    ("Fog Cloud", 2),
    ("Forcecage", 7),
    ("Forceful Hand", 6),
    ("Foresight", 9),
    ("Form of the Dragon I", 6),
    ("Form of the Dragon II", 7),
    ("Form of the Dragon III", 8),
    ("Fox's Cunning", 2),
    ("Fox's Cunning (Mass)", 6),
    ("Freedom", 9),
    ("Freezing Sphere", 6),
    ("Gaseous Form", 3),
    ("Gate", 9),
    ("Geas (Lesser)", 4),
    ("Geas/Quest", 6),
    ("Gentle Breeze", 1),
    ("Gentle Repose", 3),
    ("Getaway", 6),
    ("Geyser", 5),
    ("Ghost Sound", 0),
    ("Ghoul Touch", 2),
    ("Giant Form I", 7),
    ("Giant Form II", 8),
    ("Glide", 2),
    ("Glitterdust", 2),
    ("Globe of Invulnerability", 6),
    ("Globe of Invulnerability (Lesser)", 4),
    ("Glue Seal", 1),
    ("Grasping Hand", 7),
    ("Gravity Bow", 1),
    ("Grease", 1),
    ("Guards and Wards", 6),
    ("Gust of Wind", 2),
    ("Hallucinatory Terrain", 4),
    ("Halt Undead", 3),
    ("Haste", 3),
    ("Heart of the Metal", 3),
    ("Heightened Awareness", 1),
    ("Heroism", 3),
    ("Heroism (Greater)", 6),
    ("Hideous Laughter", 2),
    ("Hold Monster", 5),
    ("Hold Monster (Mass)", 9),
    ("Hold Person", 3),
    ("Hold Person (Mass)", 7),
    ("Hold Portal", 1),
    ("Horrid Wilting", 8),
    ("Hungry Pit", 5),
    ("Hydraulic Push", 1),
    ("Hydraulic Torrent", 3),
    ("Hypnotic Pattern", 2),
    ("Hypnotism", 1),
    ("Ice Storm", 4),
    ("Identify", 1),
    ("Illusory Script", 3),
    ("Illusory Wall", 4),
    ("Imprisonment", 9),
    ("Incendiary Cloud", 8),
    ("Insanity", 7),
    ("Instant Summons", 7),
    ("Interposing Hand", 5),
    ("Investigative Mind", 2),
    ("Invisibility", 2),
    ("Invisibility (Greater)", 4),
    ("Invisibility (Mass)", 7),
    ("Invisibility Alarm", 1),
    ("Invisibility Sphere", 3),
    ("Iron Body", 8),
    ("Irresistible Dance", 8),
    ("Jump", 1),
    ("Keen Edge", 3),
    ("Knock", 2),
    ("Legend Lore", 6),
    ("Levitate", 2),
    ("Life Bubble", 5),
    ("Life Pact", 2),
    ("Light", 0),
    ("Lightning Bolt", 3),
    ("Limited Wish", 7),
    ("Line in the Sand", 1),
    ("Locate Creature", 4),
    ("Locate Object", 2),
    ("Long Arm", 1),
    ("Mage Armor", 1),
    ("Mage Hand", 0),
    ("Mage's Disjunction", 9),
    ("Mage's Faithful Hound", 5),
    ("Mage's Lucubration", 6),
    ("Mage's Magnificent Mansion", 7),
    ("Mage's Private Sanctum", 5),
    ("Mage's Sword", 7),
    ("Magic Aura", 1),
    ("Magic Circle against Chaos", 3),
    ("Magic Circle against Evil", 3),
    ("Magic Circle against Good", 3),
    ("Magic Circle against Law", 3),
    ("Magic Jar", 5),
    ("Magic Missile", 1),
    ("Magic Mouth", 2),
    ("Magic Weapon", 1),
    ("Magic Weapon (Greater)", 3),
    ("Major Creation", 5),
    ("Major Image", 3),
    ("Make Whole", 2),
    ("Malediction", 3),
    ("Maze", 8),
    ("Memorize Page", 1),
    ("Memory Lapse", 1),
    ("Mending", 0),
    ("Message", 0),
    ("Meteor Swarm", 9),
    ("Mind Blank", 8),
    ("Mind Fog", 5),
    ("Mindlocked Messenger", 3),
    ("Minor Creation", 4),
    ("Minor Image", 2),
    ("Mirage Arcana", 5),
    ("Mirror Hideaway", 2),
    ("Mirror Image", 2),
    ("Mirror Polish", 1),
    ("Mirror Transport", 4),
    ("Misdirection", 2),
    ("Mislead", 6),
    ("Mnemonic Enhancer", 4),
    ("Molten Orb", 2),
    ("Moment of Prescience", 8),
    ("Monkey Fish", 1),
    ("Moonstruck", 4),
    ("Mount", 1),
    ("Move Earth", 6),
    ("Nauseating Trail", 3),
    ("Nightmare", 5),
    ("Nondetection", 3),
    ("Obscure Object", 2),
    ("Obscuring Mist", 1),
    ("Open/Close", 0),
    ("Overland Flight", 5),
    ("Owl's Wisdom", 2),
    ("Owl's Wisdom (Mass)", 6),
    ("Pain Strike", 3),
    ("Pain Strike (Mass)", 5),
    ("Passwall", 5),
    ("Permanency", 5),
    ("Permanent Image", 6),
    ("Persistent Image", 5),
    ("Phantasmal Killer", 4),
    ("Phantasmal Revenge", 7),
    ("Phantasmal Web", 5),
    ("Phantom Blood", 1),
    ("Phantom Steed", 3),
    ("Phantom Trap", 2),
    ("Phase Door", 7),
    ("Pierce Disguise", 3),
    ("Planar Adaptation", 5),
    ("Planar Adaptation (Mass)", 7),
    ("Planar Binding", 6),
    ("Planar Binding (Greater)", 8),
    ("Planar Binding (Lesser)", 5),
    ("Plane Shift", 7),
    ("Plant Shape I", 5),
    ("Plant Shape II", 6),
    ("Plant Shape III", 7),
    ("Polar Ray", 8),
    ("Polymorph", 5),
    ("Polymorph (Greater)", 7),
    ("Polymorph Any Object", 8),
    ("Polymorph Familiar", 3),
    ("Power Word Blind", 7),
    ("Power Word Kill", 9),
    ("Power Word Stun", 8),
    ("Prestidigitation", 0),
    ("Prismatic Sphere", 9),
    ("Prismatic Spray", 7),
    ("Prismatic Wall", 8),
    ("Programmed Image", 6),
    ("Project Image", 7),
    ("Protection from Arrows", 2),
    ("Protection from Chaos", 1),
    ("Protection from Energy", 3),
    ("Protection from Evil", 1),
    ("Protection from Good", 1),
    ("Protection from Law", 1),
    ("Protection from Spells", 8),
    ("Prying Eyes", 5),
    ("Prying Eyes (Greater)", 8),
    ("Pyrotechnics", 2),
    ("Rage", 3),
    ("Rainbow Pattern", 4),
    ("Rampart", 7),
    ("Ray of Enfeeblement", 1),
    ("Ray of Exhaustion", 3),
    ("Ray of Frost", 0),
    ("Read Magic", 0),
    ("Reduce Person", 1),
    ("Reduce Person (Mass)", 4),
    ("Refine Improvised Weapon", 1),
    ("Refuge", 9),
    ("Remove Curse", 4),
    ("Repair Undead", 1),
    ("Repair Undead (Mass)", 5),
    ("Repulsion", 6),
    ("Resilient Sphere", 4),
    ("Resist Energy", 2),
    ("Resistance", 0),
    ("Reverse Gravity", 7),
    ("River Whip", 2),
    ("River of Wind", 4),
    ("Rope Trick", 2),
    ("Scare", 2),
    ("Scintillating Pattern", 8),
    ("Scorching Ray", 2),
    ("Screen", 8),
    ("Scrying", 4),
    ("Scrying (Greater)", 7),
    ("Sculpt Corpse", 1),
    ("Seamantle", 8),
    ("Secret Chest", 5),
    ("Secret Page", 3),
    ("Secure Shelter", 4),
    ("See Invisibility", 2),
    ("Seek Thoughts", 3),
    ("Seeming", 5),
    ("Sending", 5),
    ("Sepia Snake Sigil", 3),
    ("Sequester", 7),
    ("Shades", 9),
    ("Shadow Conjuration", 4),
    ("Shadow Conjuration (Greater)", 7),
    ("Shadow Evocation", 5),
    ("Shadow Evocation (Greater)", 8),
    ("Shadow Projection", 4),
    ("Shadow Walk", 6),
    ("Shapechange", 9),
    ("Share Language", 2),
    ("Share Senses", 4),
    ("Shatter", 2),
    ("Shield", 1),
    ("Shield Companion", 3),
    ("Shifting Sand", 3),
    ("Shocking Grasp", 1),
    ("Shout", 4),
    ("Shout (Greater)", 8),
    ("Shrink Item", 3),
    ("Silent Image", 1),
    ("Silent Table", 2),
    ("Silver Darts", 3),
    ("Simulacrum", 7),
    ("Sirocco", 6),
    ("Sleep", 1),
    ("Sleet Storm", 3),
    ("Slipstream", 2),
    ("Slow", 3),
    ("Solid Fog", 4),
    ("Sonic Form", 6),
    ("Sonic Scream", 2),
    ("Soul Bind", 9),
    ("Spark", 0),
    ("Spectral Hand", 2),
    ("Spell Turning", 7),
    ("Spellcrash", 6),
    ("Spellcrash (Greater)", 8),
    ("Spellcrash (Lesser)", 4),
    ("Spider Climb", 2),
    ("Spiked Pit", 3),
    ("Statue", 7),
    ("Stinking Cloud", 3),
    ("Stone Call", 2),
    ("Stone Discus", 2),
    ("Stone Fist", 1),
    ("Stone Shape", 4),
    ("Stone to Flesh", 6),
    ("Stoneskin", 4),
    ("Stormbolts", 8),
    ("Stricken Heart", 2),
    ("Stumble Gap", 1),
    ("Stunning Barrier", 1),
    ("Stunning Barrier (Greater)", 3),
    ("Suffocation", 5),
    ("Suffocation (Mass)", 9),
    ("Suggestion", 3),
    ("Suggestion (Mass)", 6),
    ("Summon Monster I", 1),
    ("Summon Monster II", 2),
    ("Summon Monster III", 3),
    ("Summon Monster IV", 4),
    ("Summon Monster IX", 9),
    ("Summon Monster V", 5),
    ("Summon Monster VI", 6),
    ("Summon Monster VII", 7),
    ("Summon Monster VIII", 8),
    ("Summon Swarm", 2),
    ("Sunburst", 8),
    ("Sunder Breaker", 1),
    ("Sundering Shards", 1),
    ("Symbol of Death", 8),
    ("Symbol of Fear", 6),
    ("Symbol of Insanity", 8),
    ("Symbol of Laughter", 4),
    ("Symbol of Pain", 5),
    ("Symbol of Persuasion", 6),
    ("Symbol of Sleep", 5),
    ("Symbol of Stunning", 7),
    ("Symbol of Weakness", 7),
    ("Sympathy", 8),
    ("Telekinesis", 5),
    ("Telekinetic Sphere", 8),
    ("Telepathic Bond", 5),
    ("Teleport", 5),
    ("Teleport (Greater)", 7),
    ("Teleport Object", 7),
    ("Teleportation Circle", 9),
    ("Temporal Stasis", 8),
    ("Thunderstomp", 1),
    ("Thunderstomp (Greater)", 3),
    ("Time Shudder", 2),
    ("Time Stop", 9),
    ("Tiny Hut", 3),
    ("Tongues", 3),
    ("Touch of Fatigue", 0),
    ("Touch of Gracelessness", 1),
    ("Touch of Idiocy", 2),
    ("Touch of the Sea", 1),
    ("Transformation", 6),
    ("Transmute Mud to Rock", 5),
    ("Transmute Rock to Mud", 5),
    ("Trap the Soul", 8),
    ("Treasure Stitching", 5),
    ("Triggered Suggestion", 4),
    ("True Form", 4),
    ("True Seeing", 6),
    ("True Strike", 1),
    ("Tsunami", 9),
    ("Twilight Haze", 2),
    ("Twilight Knife", 3),
    ("Unbearable Brightness", 4),
    ("Undeath to Death", 6),
    ("Unliving Rage", 3),
    ("Unravel Destiny", 3),
    ("Unseen Servant", 1),
    ("Unwilling Shield", 6),
    ("Vampiric Shadow Shield", 5),
    ("Vampiric Touch", 3),
    ("Vanish", 1),
    ("Veil", 6),
    ("Ventriloquism", 1),
    ("Versatile Weapon", 3),
    ("Vision", 7),
    ("Vortex", 7),
    ("Wail of the Banshee", 9),
    ("Wall of Blindness/Deafness", 4),
    ("Wall of Fire", 4),
    ("Wall of Force", 5),
    ("Wall of Ice", 4),
    ("Wall of Iron", 6),
    ("Wall of Lava", 8),
    ("Wall of Nausea", 3),
    ("Wall of Stone", 5),
    ("Wall of Suppression", 9),
    ("Wandering Star Motes", 4),
    ("Water Breathing", 3),
    ("Wave Shield", 1),
    ("Waves of Exhaustion", 7),
    ("Waves of Fatigue", 5),
    ("Web", 2),
    ("Weird", 9),
    ("Whip of Ants", 6),
    ("Whip of Centipedes", 5),
    ("Whip of Spiders", 2),
    ("Whispering Wind", 2),
    ("Wind Wall", 3),
    ("Winds of Vengeance", 9),
    ("Wish", 9),
    ("World Wave", 9),
];

/// Looks up a spell's Wizard-specific spell level (0-9). `None` means the
/// named spell is not on the real Wizard spell list at all -- either it's
/// not a real spell, or it's a real spell no Wizard can ever prepare.
pub fn wizard_spell_level(spell_key: &str) -> Option<u8> {
    WIZARD_SPELL_LIST
        .iter()
        .find(|(key, _)| *key == spell_key)
        .map(|(_, level)| *level)
}

/// `AT-34-E3-001` (`class_feature_option_pool_record_not_held_by_engine`
/// mechanism), wizard-opposition-school-spell-tracking sub-cause. Joins
/// this table's own Wizard-specific spell levels against
/// [`crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST`]'s
/// `school` field: every 0th-level spell BOTH tables agree Wizard can
/// prepare, filtered to one school.
///
/// This is a pure join of two already-shipped, already-tested tables --
/// no new raw data is introduced, so there is no second place a Wizard
/// spell-school fact could drift out of sync with either source table.
/// Sorted for a deterministic, easily-diffed return value.
///
/// **Why this exists.** `cr_abilities_class.lst`'s own `"<School> Wizard
/// Spells"` internal chassis records (`CATEGORY:Internal`,
/// `SPELLKNOWN:CLASS|Wizard=0|<spells>`) partition every 0th-level Wizard
/// spell by school -- the corpus's own encoding of which cantrips belong
/// to which arcane school for a Wizard specifically (as opposed to any
/// other class that spell might also appear on). Verified byte-for-byte
/// against all 9 of those corpus records
/// (`wizard_school_zero_level_spells_matches_the_real_corpus_records`,
/// `class_feature_pool_catalog`'s own
/// `wizard_school_spell_list_key_owner_matches_are_exact` test) --
/// `WIZARD_SPELL_LIST`'s own level field (already isolated to
/// Wizard specifically, unlike `SPELL_LIST`'s minimum-across-classes
/// `level`, see this file's own module doc comment) is what makes this
/// join exact rather than the near-miss a naive `SPELL_LIST`-only
/// level-0 read would produce (that naive read includes Cleric/Druid-only
/// 0-level spells like `Create Water`/`Guidance` that Wizard cannot
/// prepare at all -- confirmed absent from every one of the 9 corpus
/// records).
pub fn wizard_school_zero_level_spells(
    school: crate::rules_core::rules_tables::crb::spell_list::Pf1SchoolId,
) -> Vec<&'static str> {
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;
    let mut spells: Vec<&'static str> = WIZARD_SPELL_LIST
        .iter()
        .filter(|(_, level)| *level == 0)
        .filter_map(|(key, _)| {
            SPELL_LIST
                .iter()
                .find(|entry| entry.key == *key && entry.school == school)
                .map(|entry| entry.key)
        })
        .collect();
    spells.sort_unstable();
    spells
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::acg::spell_list as acg_spell_list;
    use crate::rules_core::rules_tables::apg::spell_list as apg_spell_list;
    use crate::rules_core::rules_tables::crb::sorcerer_spell_list::SORCERER_SPELL_LIST;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;
    use std::path::PathBuf;

    #[test]
    fn wizard_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(WIZARD_SPELL_LIST.len(), 580);
    }

    /// `AT-34-E3-001` wizard-opposition-school-spell-tracking sub-cause:
    /// proves [`wizard_school_zero_level_spells`] byte-for-byte against
    /// the 9 real, committed `"<School> Wizard Spells"` corpus records
    /// under `data/corpus/core_rulebook/class_feature/` -- not merely
    /// asserted in a doc comment. RED if either source table (or the
    /// corpus itself) ever drifts, which is exactly when this join must
    /// be revisited.
    #[test]
    fn wizard_school_zero_level_spells_matches_the_real_corpus_records() {
        use crate::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let cases: &[(&str, Pf1SchoolId)] = &[
            ("abjuration_wizard_spells", Pf1SchoolId::Abjuration),
            ("conjuration_wizard_spells", Pf1SchoolId::Conjuration),
            ("divination_wizard_spells", Pf1SchoolId::Divination),
            ("enchantment_wizard_spells", Pf1SchoolId::Enchantment),
            ("evocation_wizard_spells", Pf1SchoolId::Evocation),
            ("illusion_wizard_spells", Pf1SchoolId::Illusion),
            ("necromancy_wizard_spells", Pf1SchoolId::Necromancy),
            ("transmutation_wizard_spells", Pf1SchoolId::Transmutation),
            ("universal_wizard_spells", Pf1SchoolId::Universal),
        ];
        for (dir, school) in cases {
            let path = repo_root
                .join("data/corpus/core_rulebook/class_feature")
                .join(dir)
                .join(format!("{dir}.json"));
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("readable corpus json at {path:?}: {e}"));
            let json: serde_json::Value =
                serde_json::from_str(&text).expect("valid corpus json");
            let raw_tokens = json["data"]["raw_tokens"].as_array().expect("raw_tokens array");
            let spellknown = raw_tokens
                .iter()
                .find(|t| t["key"].as_str() == Some("SPELLKNOWN"))
                .and_then(|t| t["value"].as_str())
                .unwrap_or_else(|| panic!("{dir} carries a SPELLKNOWN token"));
            // `CLASS|Wizard=0|Spell One,Spell Two`
            let corpus_spells: Vec<&str> = spellknown
                .split('|')
                .nth(2)
                .unwrap_or_else(|| panic!("{dir}'s SPELLKNOWN token has a spell-list segment"))
                .split(',')
                .map(str::trim)
                .collect();
            let mut expected = corpus_spells.clone();
            expected.sort_unstable();
            let actual = wizard_school_zero_level_spells(*school);
            assert_eq!(
                actual, expected,
                "{dir}: engine join disagrees with the real corpus SPELLKNOWN token"
            );
        }
    }

    /// One anchor per ingested book, each a record where Wizard is NOT the
    /// first name in its `CLASSES:` comma group. Raw lines:
    /// `Acid Arrow` is `CLASSES:Sorcerer,Wizard=2` (CRB),
    /// `Alter Winds` is `CLASSES:Druid,Sorcerer,Wizard=1` (APG),
    /// `Adhesive Blood` is
    /// `CLASSES:Alchemist,Bloodrager,Sorcerer,Witch,Wizard=2` (ACG).
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        assert_eq!(wizard_spell_level("Acid Arrow"), Some(2));
        assert_eq!(wizard_spell_level("Alter Winds"), Some(1));
        assert_eq!(wizard_spell_level("Adhesive Blood"), Some(2));
    }

    /// The two records whose level carries a trailing optional-rule gate
    /// (`CLASSES:Sorcerer,Witch,Wizard=3[PREVAREQ:Heroic,1]` and
    /// `CLASSES:Cleric,Sorcerer,Witch,Wizard=3[PREVAREQ:Heroic,1]`).
    /// These are the exact records a naive `int(level)` throws away
    /// silently; included per item 54.
    #[test]
    fn optional_rule_gated_spells_are_on_the_list() {
        assert_eq!(wizard_spell_level("Malediction"), Some(3));
        assert_eq!(wizard_spell_level("Unravel Destiny"), Some(3));
    }

    #[test]
    fn every_wizard_spell_level_is_within_the_real_wizard_ceiling() {
        for (key, level) in WIZARD_SPELL_LIST {
            assert!(
                (0..=9).contains(level),
                "{key} has out-of-range Wizard spell level {level}"
            );
        }
    }

    #[test]
    fn every_wizard_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in WIZARD_SPELL_LIST {
            let known = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            assert!(known, "{key} is not a real spell key in any ingested book");
        }
    }

    #[test]
    fn no_spell_key_appears_twice() {
        let mut keys: Vec<&str> = WIZARD_SPELL_LIST.iter().map(|(key, _)| *key).collect();
        keys.sort_unstable();
        let total = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), total, "a spell key is listed twice");
    }

    #[test]
    fn wizard_spell_level_looks_up_known_values() {
        assert_eq!(wizard_spell_level("Acid Splash"), Some(0));
        assert_eq!(wizard_spell_level("Wish"), Some(9));
        assert_eq!(wizard_spell_level("Magic Missile"), Some(1));
        assert_eq!(wizard_spell_level("Nonexistent Spell"), None);
    }

    /// The bug this table exists to fix, at its canonical example.
    /// `Hideous Laughter` is `CLASSES:Bard=1|Sorcerer,Wizard=2`, so the
    /// record's own minimum-across-classes level is 1 -- which is what
    /// `crb::spell_list` carries and what the Character Sheet used to show
    /// a Wizard. A Wizard learns it at 2.
    #[test]
    fn hideous_laughter_is_second_level_for_a_wizard_not_first() {
        let record_level = SPELL_LIST
            .iter()
            .find(|entry| entry.key == "Hideous Laughter")
            .expect("Hideous Laughter is a real CRB record")
            .level;
        assert_eq!(record_level, 1, "the record's own level is the Bard level");
        assert_eq!(wizard_spell_level("Hideous Laughter"), Some(2));
    }

    #[test]
    fn level_distribution_matches_the_real_corpus_parse() {
        let count_at = |level: u8| WIZARD_SPELL_LIST.iter().filter(|(_, l)| *l == level).count();
        assert_eq!(count_at(0), 21);
        assert_eq!(count_at(1), 82);
        assert_eq!(count_at(2), 86);
        assert_eq!(count_at(3), 82);
        assert_eq!(count_at(4), 69);
        assert_eq!(count_at(5), 60);
        assert_eq!(count_at(6), 58);
        assert_eq!(count_at(7), 49);
        assert_eq!(count_at(8), 42);
        assert_eq!(count_at(9), 31);
    }

    /// Pins the Sorcerer relationship as an OBSERVATION -- see this
    /// module's doc comment for why it is not used as a derivation. If a
    /// corpus revision ever splits the two arcane lists, this test is the
    /// one that reports it; it is not a correctness constraint on either
    /// table.
    #[test]
    fn the_sorcerer_overlap_is_an_observation_not_a_derivation() {
        let wizard_only: Vec<&str> = WIZARD_SPELL_LIST
            .iter()
            .filter(|(key, _)| !SORCERER_SPELL_LIST.iter().any(|(other, _)| other == key))
            .map(|(key, _)| *key)
            .collect();
        assert_eq!(wizard_only, vec!["Mage's Lucubration", "Mnemonic Enhancer"]);
        assert_eq!(wizard_spell_level("Mage's Lucubration"), Some(6));
        assert_eq!(wizard_spell_level("Mnemonic Enhancer"), Some(4));

        let sorcerer_only: Vec<&str> = SORCERER_SPELL_LIST
            .iter()
            .filter(|(key, _)| !WIZARD_SPELL_LIST.iter().any(|(other, _)| other == key))
            .map(|(key, _)| *key)
            .collect();
        assert!(sorcerer_only.is_empty(), "sorcerer-only keys: {sorcerer_only:?}");

        let disagreements: Vec<&str> = SORCERER_SPELL_LIST
            .iter()
            .filter(|(key, level)| wizard_spell_level(key).is_some_and(|w| w != *level))
            .map(|(key, _)| *key)
            .collect();
        assert!(
            disagreements.is_empty(),
            "the two arcane lists disagree on: {disagreements:?}"
        );
    }

    /// The scale of the defect this table fixes, re-derived here rather
    /// than asserted from memory: how many of the 580 Wizard spells the
    /// record's own `level` field gets wrong. Every one of them is biased
    /// LOW (the record's level is a minimum), never high.
    #[test]
    fn the_record_level_understates_the_wizard_level_for_67_of_580_spells() {
        let record_level = |key: &str| -> Option<u8> {
            SPELL_LIST
                .iter()
                .find(|entry| entry.key == key)
                .map(|entry| entry.level)
                .or_else(|| {
                    apg_spell_list::SPELL_LIST
                        .iter()
                        .find(|entry| entry.key == key)
                        .and_then(|entry| entry.level)
                })
                .or_else(|| {
                    acg_spell_list::SPELL_LIST
                        .iter()
                        .find(|entry| entry.key == key)
                        .map(|entry| entry.level)
                })
        };

        let mut wrong = 0;
        let mut biased_high = 0;
        let mut no_record_level = 0;
        for (key, wizard_level) in WIZARD_SPELL_LIST {
            match record_level(key) {
                None => no_record_level += 1,
                Some(shown) if shown == *wizard_level => {}
                Some(shown) => {
                    wrong += 1;
                    if shown > *wizard_level {
                        biased_high += 1;
                    }
                }
            }
        }
        assert_eq!(wrong, 67, "spells whose record level is a wrong number");
        assert_eq!(biased_high, 0, "the bias is always low, never high");
        // The remaining two are `Malediction` and `Unravel Destiny`: their
        // APG records carry no usable `level`, so the sheet showed no
        // number at all rather than a wrong one. This table supplies 3
        // for both.
        assert_eq!(no_record_level, 2);
    }
}
