//! APG/ACG Witch spell list — one `(spell name, Witch spell level)`
//! entry per real corpus record.
//!
//! Source: every record whose `CLASSES:` token names `Witch` in any of
//! its comma-separated class groups, across the books this repo ingests:
//! `apg_spells.lst` (250) and `acg_spells.lst` (74). **324 unique
//! spells**, levels 0-9, split
//! **16 / 37 / 58 / 47 / 43 / 31 / 29 / 25 / 23 / 15**.
//!
//! **Corpus reachability: 324 of 324.** Every entry resolves against
//! this repo's ingested `data/corpus/` spell records.
//!
//! # Parsing `CLASSES:` correctly — the bug this module shipped once
//!
//! A `CLASSES:` token is pipe-separated groups, each
//! `Name1,Name2,...=Level`. The level belongs to the WHOLE comma group,
//! so a class named anywhere but last is **not** followed by `=`:
//!
//! ```text
//! CLASSES:Alchemist,Bloodrager,Sorcerer,Witch,Wizard=2
//! ```
//!
//! Here Witch is a 2nd-level spell, but the substring `Witch=` does not
//! occur anywhere in the line. The first version of this module was
//! generated with a `CLASSES:.*Witch=` grep and therefore captured only
//! spells where Witch happened to be the LAST class in its group —
//! silently dropping 60 genuine ACG spells and 15 APG ones, while
//! reporting 100% reachability on the incomplete set it did find.
//!
//! Always split the token into groups, split each group's names on
//! commas, and MEMBERSHIP-TEST the class name. Never substring-match
//! `<Class>=`. This same bug shape has now been hit three times on this
//! one class, so it is worth treating as the default hazard when reading
//! `CLASSES:` for any class that commonly shares spells.
//!
//! The inherited "324-spell" figure in this task was therefore CORRECT
//! all along. An earlier revision of this file claimed it "does not
//! reproduce" and recorded it as a fourth stale cross-book count; that
//! claim was wrong and was produced by the parsing bug above, not by the
//! source. Scout and the team lead had both independently derived 324
//! before this module was first written.
//!
//! Unlike Bloodrager, Witch genuinely HAS cantrips: 16 records at level
//! 0, a real 0-level spell list rather than the always-zero sentinel
//! column Bloodrager's `CAST:0,1` carries.

/// Every `(spell name, Witch spell level)` pair on the real APG/ACG
/// Witch spell list, sorted by name.
pub const WITCH_SPELL_LIST: &[(&str, u8)] = &[
    ("Adhesive Blood", 2),
    ("Adhesive Spittle", 1),
    ("Adjustable Disguise", 3),
    ("Adjustable Polymorph", 4),
    ("Aggressive Thundercloud", 2),
    ("Aggressive Thundercloud (Greater)", 4),
    ("Air Geyser", 3),
    ("Air Step", 2),
    ("Alter Musical Instrument", 1),
    ("Alter Self", 2),
    ("Analyze Dweomer", 6),
    ("Anchored Step", 3),
    ("Animal Purpose Training", 1),
    ("Animate Objects", 6),
    ("Anonymous Interaction", 2),
    ("Anti-Incorporeal Shell", 4),
    ("Antipathy", 8),
    ("Arcane Eye", 4),
    ("Arcane Mark", 0),
    ("Arcane Sight", 3),
    ("Arcane Sight (Greater)", 7),
    ("Astral Projection", 9),
    ("Augury", 2),
    ("Aura Sight", 3),
    ("Baleful Polymorph", 5),
    ("Banish Seeming", 5),
    ("Banshee Blast", 6),
    ("Barrow Haze", 3),
    ("Beastspeak", 2),
    ("Beguiling Gift", 1),
    ("Bestow Curse", 3),
    ("Black Tentacles", 4),
    ("Bleed", 0),
    ("Blight", 5),
    ("Blindness/Deafness", 2),
    ("Blood Armor", 2),
    ("Break Enchantment", 5),
    ("Bullet Ward", 2),
    ("Buoyancy", 2),
    ("Burning Gaze", 2),
    ("Burning Hands", 1),
    ("Cause Fear", 1),
    ("Chain Lightning", 7),
    ("Charm Monster", 4),
    ("Charm Monster (Mass)", 8),
    ("Charm Person", 1),
    ("Chill Touch", 1),
    ("Clairaudience/Clairvoyance", 3),
    ("Climbing Beanstalk", 2),
    ("Cloak of Dreams", 6),
    ("Clone", 8),
    ("Cloudkill", 5),
    ("Command", 1),
    ("Companion Life Link", 2),
    ("Comprehend Languages", 1),
    ("Cone of Cold", 6),
    ("Confusion", 4),
    ("Contact Other Plane", 5),
    ("Control Weather", 7),
    ("Crimson Confession", 2),
    ("Crushing Despair", 4),
    ("Cup of Dust", 3),
    ("Cure Critical Wounds", 5),
    ("Cure Critical Wounds (Mass)", 9),
    ("Cure Light Wounds", 1),
    ("Cure Light Wounds (Mass)", 6),
    ("Cure Moderate Wounds", 2),
    ("Cure Moderate Wounds (Mass)", 7),
    ("Cure Serious Wounds", 4),
    ("Cure Serious Wounds (Mass)", 8),
    ("Curse of Burning Sleep", 4),
    ("Dancing Lantern", 1),
    ("Dancing Lights", 0),
    ("Daze", 0),
    ("Daze Monster", 2),
    ("Death Knell", 2),
    ("Death Ward", 4),
    ("Deep Slumber", 3),
    ("Delay Poison", 2),
    ("Demand", 8),
    ("Destruction", 8),
    ("Detect Magic", 0),
    ("Detect Poison", 0),
    ("Detect Scrying", 4),
    ("Detect Secret Doors", 1),
    ("Detect Thoughts", 2),
    ("Dimension Door", 4),
    ("Dimensional Bounce", 7),
    ("Disable Construct", 3),
    ("Discern Lies", 4),
    ("Discern Location", 8),
    ("Discern Next of Kin", 1),
    ("Disguise Weapon", 1),
    ("Dispel Magic", 3),
    ("Dispel Magic (Greater)", 6),
    ("Divination", 4),
    ("Dominate Monster", 9),
    ("Dominate Person", 5),
    ("Elemental Swarm", 9),
    ("Enchantment Foil", 4),
    ("Enervation", 4),
    ("Enlarge Person", 1),
    ("Enthrall", 2),
    ("Euphoric Cloud", 2),
    ("Extreme Flexibility", 2),
    ("Eyebite", 6),
    ("Fairy Ring Retreat", 7),
    ("False Life", 2),
    ("Familiar Double", 7),
    ("Fear", 4),
    ("Feast of Ashes", 2),
    ("Feast on Fear", 5),
    ("Feeblemind", 5),
    ("Fester", 2),
    ("Fester (Mass)", 6),
    ("Find Traps", 2),
    ("Find the Path", 6),
    ("Flesh to Stone", 6),
    ("Fly", 3),
    ("Fog Cloud", 2),
    ("Foresight", 9),
    ("Geas (Lesser)", 4),
    ("Geas/Quest", 6),
    ("Gentle Breeze", 1),
    ("Gentle Repose", 2),
    ("Glide", 2),
    ("Glitterdust", 2),
    ("Glyph of Warding", 3),
    ("Guards and Wards", 6),
    ("Guidance", 0),
    ("Guiding Star", 3),
    ("Harm", 7),
    ("Heal", 7),
    ("Heart of the Metal", 3),
    ("Heroism", 3),
    ("Heroism (Greater)", 6),
    ("Hex Glyph", 3),
    ("Hex Glyph (Greater)", 5),
    ("Hex Vulnerability", 1),
    ("Hidden Speech", 2),
    ("Hold Monster", 5),
    ("Hold Monster (Mass)", 9),
    ("Hold Person", 2),
    ("Hold Person (Mass)", 7),
    ("Horrid Wilting", 8),
    ("Hypnotism", 1),
    ("Ice Storm", 4),
    ("Identify", 1),
    ("Ill Omen", 1),
    ("Inflict Critical Wounds", 5),
    ("Inflict Critical Wounds (Mass)", 9),
    ("Inflict Light Wounds", 1),
    ("Inflict Light Wounds (Mass)", 6),
    ("Inflict Moderate Wounds", 2),
    ("Inflict Moderate Wounds (Mass)", 7),
    ("Inflict Serious Wounds", 4),
    ("Inflict Serious Wounds (Mass)", 8),
    ("Insanity", 7),
    ("Instant Summons", 7),
    ("Investigative Mind", 2),
    ("Irresistible Dance", 8),
    ("Legend Lore", 6),
    ("Levitate", 2),
    ("Life Pact", 2),
    ("Light", 0),
    ("Lightning Bolt", 3),
    ("Locate Creature", 4),
    ("Locate Object", 3),
    ("Long Arm", 1),
    ("Mage Armor", 1),
    ("Magic Jar", 5),
    ("Major Creation", 5),
    ("Mark of Justice", 5),
    ("Mask Dweomer", 1),
    ("Maze", 8),
    ("Memorize Page", 1),
    ("Mending", 0),
    ("Message", 0),
    ("Mind Blank", 8),
    ("Mind Fog", 5),
    ("Mindlocked Messenger", 3),
    ("Minor Creation", 4),
    ("Mirror Hideaway", 2),
    ("Mirror Polish", 1),
    ("Mirror Transport", 4),
    ("Molten Orb", 2),
    ("Moment of Prescience", 8),
    ("Moonstruck", 4),
    ("Mount", 1),
    ("Nature's Exile", 3),
    ("Nauseating Dart", 1),
    ("Nauseating Trail", 3),
    ("Neutralize Poison", 4),
    ("Obscuring Mist", 1),
    ("Overland Flight", 5),
    ("Pain Strike", 3),
    ("Pain Strike (Mass)", 5),
    ("Perceive Cues", 2),
    ("Persistent Vigor", 4),
    ("Phantasmal Killer", 4),
    ("Phase Door", 7),
    ("Pierce Disguise", 3),
    ("Plane Shift", 7),
    ("Poison", 4),
    ("Polymorph Familiar", 3),
    ("Power Word Blind", 7),
    ("Power Word Kill", 9),
    ("Power Word Stun", 8),
    ("Pox Pustules", 2),
    ("Prying Eyes", 5),
    ("Prying Eyes (Greater)", 8),
    ("Putrefy Food and Drink", 0),
    ("Rage", 3),
    ("Raise Dead", 6),
    ("Ray of Enfeeblement", 1),
    ("Ray of Exhaustion", 3),
    ("Read Magic", 0),
    ("Reduce Person", 1),
    ("Refuge", 9),
    ("Regenerate", 7),
    ("Reincarnate", 5),
    ("Remove Blindness/Deafness", 3),
    ("Remove Curse", 3),
    ("Remove Disease", 3),
    ("Resistance", 0),
    ("Rest Eternal", 5),
    ("Resurrection", 8),
    ("River Whip", 2),
    ("Scare", 2),
    ("Screech", 3),
    ("Scrying", 4),
    ("Scrying (Greater)", 7),
    ("Secret Chest", 5),
    ("Secure Shelter", 4),
    ("See Invisibility", 2),
    ("Seek Thoughts", 3),
    ("Sepia Snake Sigil", 3),
    ("Severed Fate", 2),
    ("Share Senses", 3),
    ("Silent Table", 2),
    ("Silver Darts", 3),
    ("Slay Living", 6),
    ("Sleep", 1),
    ("Sleepwalk", 4),
    ("Sleet Storm", 3),
    ("Solid Fog", 4),
    ("Soul Bind", 9),
    ("Spark", 0),
    ("Speak with Dead", 3),
    ("Speak with Haunt", 4),
    ("Spectral Hand", 2),
    ("Spellcrash", 6),
    ("Spellcrash (Greater)", 8),
    ("Spellcrash (Lesser)", 4),
    ("Spite", 4),
    ("Stabilize", 0),
    ("Status", 2),
    ("Stinking Cloud", 3),
    ("Stone Discus", 2),
    ("Stone to Flesh", 6),
    ("Storm of Vengeance", 9),
    ("Stormbolts", 8),
    ("Stricken Heart", 2),
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
    ("Sundering Shards", 1),
    ("Swarm Skin", 6),
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
    ("Telepathic Bond", 5),
    ("Teleport", 5),
    ("Teleport (Greater)", 7),
    ("Teleport Object", 7),
    ("Teleportation Circle", 9),
    ("Thorny Entanglement", 3),
    ("Threefold Aspect", 4),
    ("Tongues", 3),
    ("Touch of Fatigue", 0),
    ("Touch of Idiocy", 2),
    ("Transformation", 6),
    ("Trap the Soul", 8),
    ("Triggered Suggestion", 4),
    ("True Seeing", 6),
    ("Twilight Haze", 2),
    ("Twilight Knife", 3),
    ("Unbearable Brightness", 4),
    ("Unliving Rage", 3),
    ("Unseen Servant", 1),
    ("Unwilling Shield", 6),
    ("Vampiric Touch", 3),
    ("Vision", 7),
    ("Vomit Swarm", 2),
    ("Wail of the Banshee", 9),
    ("Wall of Blindness/Deafness", 4),
    ("Wandering Star Motes", 4),
    ("Water Walk", 3),
    ("Wave Shield", 1),
    ("Waves of Exhaustion", 7),
    ("Waves of Fatigue", 5),
    ("Web", 2),
    ("Whip of Ants", 6),
    ("Whip of Centipedes", 5),
    ("Whip of Spiders", 2),
    ("Zone of Truth", 2),
];

/// The Witch spell level for `spell_key`, or `None` when the spell is
/// not on the Witch list at all.
pub fn witch_spell_level(spell_key: &str) -> Option<u8> {
    WITCH_SPELL_LIST
        .iter()
        .find(|(key, _)| *key == spell_key)
        .map(|(_, level)| *level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_matches_the_verified_corpus_extraction() {
        assert_eq!(WITCH_SPELL_LIST.len(), 324, "324 real Witch spell records");
        let expected = [16, 37, 58, 47, 43, 31, 29, 25, 23, 15];
        for (level, want) in expected.iter().enumerate() {
            let count = WITCH_SPELL_LIST
                .iter()
                .filter(|(_, l)| usize::from(*l) == level)
                .count();
            assert_eq!(count, *want, "spell level {level} count");
        }
    }

    /// Regression guard for the mid-list `CLASSES:` parsing bug that
    /// dropped 60 ACG spells from this module's first revision. Each of
    /// these is tagged for Witch in a comma group where Witch is NOT
    /// last, so the substring `Witch=` never appears on its line -- a
    /// `CLASSES:.*Witch=` grep finds none of them.
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        for (name, level) in [
            ("Adhesive Blood", 2),
            ("Adjustable Polymorph", 4),
            ("Aggressive Thundercloud", 2),
        ] {
            assert_eq!(
                witch_spell_level(name),
                Some(level),
                "{name} is tagged Witch mid-group and must not be dropped"
            );
        }
    }

    /// Witch is a full 0-9 caster WITH real cantrips, unlike Bloodrager
    /// whose leading zero is a sentinel and whose list stops at 4th.
    #[test]
    fn witch_is_a_full_nine_level_caster_with_real_cantrips() {
        for (name, level) in WITCH_SPELL_LIST {
            assert!(*level <= 9, "{name} at level {level}: 9 is the ceiling");
        }
        assert!(WITCH_SPELL_LIST.iter().any(|(_, l)| *l == 0));
        assert!(WITCH_SPELL_LIST.iter().any(|(_, l)| *l == 9));
    }

    #[test]
    fn the_list_has_no_duplicate_entries() {
        let mut names: Vec<&str> = WITCH_SPELL_LIST.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(names.len(), before, "no spell may appear twice");
    }

    #[test]
    fn lookup_resolves_a_real_entry_and_rejects_an_off_list_one() {
        let (first_name, first_level) = WITCH_SPELL_LIST[0];
        assert_eq!(witch_spell_level(first_name), Some(first_level));
        assert_eq!(witch_spell_level("Not A Real Spell"), None);
        assert_eq!(witch_spell_level(""), None);
    }
}
