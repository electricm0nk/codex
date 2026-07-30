//! PF1 Sorcerer spell list — per-class spell-level overrides.
//!
//! Mirrors `ranger_spell_list.rs`/`paladin_spell_list.rs`'s own doc comment
//! exactly, substituting Sorcerer: `crb::spell_list::SPELL_LIST`'s `level`
//! field is the MINIMUM spell level across every class named in the
//! corpus's `CLASSES:` tag for that record, not necessarily the
//! Sorcerer-specific level. This table re-parses the same corpus record's
//! raw `CLASSES:` tag directly, isolating only the Sorcerer-specific level
//! for each of the **578** real records that name Sorcerer at all: 21
//! cantrips (0th level), then 82 / 86 / 82 / 68 / 60 / 57 / 49 / 42 / 31
//! across levels 1-9 -- the real PF1 Sorcerer/Wizard shared arcane list,
//! and by a wide margin the largest spell list in this codebase.
//!
//! Unlike Ranger/Paladin (partial casters, prepared, no 0th-level spells),
//! Sorcerer is a full 9th-level caster with 0th-level cantrips included:
//! spell level 0 entries are real (cantrips are "spells known" only, per
//! PF1 rules, outside the spells-per-day ladder but still capped by the
//! Sorcerer Spells Known table's own 0th-level column).
//!
//! **Widened 2026-07-27 (task #30) from CRB-only to all ingested books**
//! (`risks-and-open-questions.md` item 53). Per-file: **394 from
//! `cr_spells.lst` + 95 from `apg_spells.lst` + 89 from `acg_spells.lst`
//! = 578**, all names distinct, no `.MOD` record assigns Sorcerer.
//!
//! **Sorcerer is the extreme case of the mid-group shape.** It is almost
//! always listed immediately before Wizard, so **all 394** CRB records
//! name it mid-group -- a `Sorcerer=` substring grep would have found
//! ZERO of them. That this module shipped all 394 correctly is itself the
//! proof it never had the substring bug; book scope was the only defect.
//!
//! **Two records carry an optional-rule gate on the level**
//! (`Malediction`, `Unravel Destiny`, both `=3[PREVAREQ:Heroic,1]` -- the
//! APG Hero Points rule). Both are included per the lead's ruling
//! (item 54); they are exactly the records a naive `int(level)` discards
//! silently, and they have their own regression test.
//!
//! Per-file ceiling check: `grep -c Sorcerer` returns 395 / 95 / 89 --
//! exact in two files. The single CRB excess is a `###Block:` section
//! comment ("Spell variants for Sorcerer Bloodlines"), not a record.
//!
//! **Corpus reachability: all 578 resolve** against the union of the
//! three ingested books' own `SPELL_LIST` tables (1,075 keys) -- never an
//! invented name. The cross-check test asserts exactly that union; a
//! CRB-only check would now reject every APG/ACG entry as fictional.
//!
//! Regenerate by parsing the `CLASSES:` token in all three spell files --
//! split the body on `|`, `rpartition` each group on `=`, strip any
//! trailing `[...]` gate from the level, then membership-test the
//! comma-separated name list. Never substring-match `Sorcerer=`, and
//! never let an `int()` on the level throw a record away silently.

/// (spell key, Sorcerer-specific spell level 0-9). A real Sorcerer may
/// only know a spell that appears in this table, subject to the
/// character's own spell-level access ceiling (for 1st+ level spells) and
/// the Sorcerer Spells Known table's per-level cap -- see
/// `sorcerer_spell_level` for the lookup helper.
pub const SORCERER_SPELL_LIST: &[(&str, u8)] = &[
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

/// Looks up a spell's Sorcerer-specific spell level (0-9). `None` means the
/// named spell is not on the real Sorcerer spell list at all -- either
/// it's not a real spell, or it's a real spell no Sorcerer can ever know.
pub fn sorcerer_spell_level(spell_key: &str) -> Option<u8> {
    SORCERER_SPELL_LIST
        .iter()
        .find(|(key, _)| *key == spell_key)
        .map(|(_, level)| *level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::acg::spell_list as acg_spell_list;
    use crate::rules_core::rules_tables::apg::spell_list as apg_spell_list;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn sorcerer_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(SORCERER_SPELL_LIST.len(), 578);
    }

    /// Guards the book-scope widening (task #30). One anchor per ingested
    /// book. Sorcerer is the extreme case of the mid-group shape: it is
    /// almost always listed before Wizard, so **all 394** CRB records name
    /// it mid-group and a `Sorcerer=` substring grep would find ZERO of
    /// them. Raw lines:
    /// `Acid Arrow` is `CLASSES:Sorcerer,Wizard=2` (CRB),
    /// `Alter Winds` is `CLASSES:Druid,Sorcerer,Wizard=1` (APG),
    /// `Adhesive Blood` is
    /// `CLASSES:Alchemist,Bloodrager,Sorcerer,Witch,Wizard=2` (ACG).
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        assert_eq!(sorcerer_spell_level("Acid Arrow"), Some(2));
        assert_eq!(sorcerer_spell_level("Alter Winds"), Some(1));
        assert_eq!(sorcerer_spell_level("Adhesive Blood"), Some(2));
    }

    /// The two records whose level carries a trailing optional-rule gate
    /// (`CLASSES:Sorcerer,Witch,Wizard=3[PREVAREQ:Heroic,1]` and
    /// `CLASSES:Cleric,Sorcerer,Witch,Wizard=3[PREVAREQ:Heroic,1]`).
    /// These are the exact records a naive `int(level)` throws away
    /// silently; included per item 54.
    #[test]
    fn optional_rule_gated_spells_are_on_the_list() {
        assert_eq!(sorcerer_spell_level("Malediction"), Some(3));
        assert_eq!(sorcerer_spell_level("Unravel Destiny"), Some(3));
    }

    #[test]
    fn every_sorcerer_spell_level_is_within_the_real_sorcerer_ceiling() {
        for (key, level) in SORCERER_SPELL_LIST {
            assert!(
                (0..=9).contains(level),
                "{key} has out-of-range Sorcerer spell level {level}"
            );
        }
    }

    /// Sorcerer's list spans all three ingested books, so a CRB-only
    /// cross-check would reject every APG/ACG entry as fictional. Checks
    /// the union instead -- still a real "never an invented name"
    /// guarantee, just scoped to everything this repo actually ingests.
    #[test]
    fn every_sorcerer_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in SORCERER_SPELL_LIST {
            let known = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            assert!(known, "{key} is not a real spell key in any ingested book");
        }
    }

    #[test]
    fn sorcerer_spell_level_looks_up_known_values() {
        assert_eq!(sorcerer_spell_level("Acid Splash"), Some(0));
        assert_eq!(sorcerer_spell_level("Wish"), Some(9));
        assert_eq!(sorcerer_spell_level("Magic Missile"), Some(1));
        assert_eq!(sorcerer_spell_level("Nonexistent Spell"), None);
    }

    #[test]
    fn level_distribution_matches_the_real_corpus_parse() {
        let count_at =
            |level: u8| SORCERER_SPELL_LIST.iter().filter(|(_, l)| *l == level).count();
        assert_eq!(count_at(0), 21);
        assert_eq!(count_at(1), 82);
        assert_eq!(count_at(2), 86);
        assert_eq!(count_at(3), 82);
        assert_eq!(count_at(4), 68);
        assert_eq!(count_at(5), 60);
        assert_eq!(count_at(6), 57);
        assert_eq!(count_at(7), 49);
        assert_eq!(count_at(8), 42);
        assert_eq!(count_at(9), 31);
    }
}
