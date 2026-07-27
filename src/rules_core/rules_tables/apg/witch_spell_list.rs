//! APG Witch spell list — one `(spell name, Witch spell level)` entry per
//! real corpus record.
//!
//! Source: every record whose `CLASSES:` token names `Witch=N` across the
//! books this repo actually ingests — `advanced_players_guide` (235) and
//! `advanced_class_guide` (14). Independently re-derived for task #23
//! (2026-07-27): **249 records, spell levels 0-9**, split
//! **15 / 26 / 36 / 31 / 31 / 27 / 24 / 24 / 21 / 14**. No duplicate
//! spell names.
//!
//! **Corpus reachability: 249 of 249.** Every entry resolves against this
//! repo's own ingested `data/corpus/` spell records — unlike Bloodrager's
//! list, where 73 of 183 had no base record here. Nothing needs routing
//! through the unresolved-selection idiom.
//!
//! **The task's inherited "324-spell" figure does not reproduce.** A
//! tree-wide count returns 799 (sweeping in Ultimate Magic/Combat/
//! Intrigue/Wilderness, Occult Adventures, Monster Codex, a third-party
//! book, and a PFS scenario file — none ingested here); the
//! ingested-book count is 249. 324 matches neither, so it is recorded as
//! superseded rather than reconciled. This is the fourth stale
//! cross-book count found this segment, after Bloodrager's 201→183,
//! Oracle's "4 Curses"→5, and Cavalier's "26 Orders"→6 — inherited
//! corpus counts are worth treating as unverified until re-derived.
//!
//! Unlike Bloodrager, Witch genuinely HAS cantrips: 15 records at
//! `Witch=0`, which is a real 0-level spell list rather than the
//! always-zero sentinel column Bloodrager's `CAST:0,1` carries.
//!
//! Scoped to the ingested books deliberately, matching the
//! single-source discipline every other spell list here already uses.

/// Every `(spell name, Witch spell level)` pair on the real APG/ACG
/// Witch spell list, sorted by name.
pub const WITCH_SPELL_LIST: &[(&str, u8)] = &[
    ("Alter Self", 2),
    ("Analyze Dweomer", 6),
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
    ("Baleful Polymorph", 5),
    ("Banish Seeming", 5),
    ("Beastspeak", 2),
    ("Beguiling Gift", 1),
    ("Bestow Curse", 3),
    ("Black Tentacles", 4),
    ("Bleed", 0),
    ("Blight", 5),
    ("Blindness/Deafness", 2),
    ("Break Enchantment", 5),
    ("Burning Hands", 1),
    ("Cause Fear", 1),
    ("Chain Lightning", 7),
    ("Charm Monster", 4),
    ("Charm Monster (Mass)", 8),
    ("Charm Person", 1),
    ("Chill Touch", 1),
    ("Clairaudience/Clairvoyance", 3),
    ("Climbing Beanstalk", 2),
    ("Clone", 8),
    ("Cloudkill", 5),
    ("Command", 1),
    ("Comprehend Languages", 1),
    ("Cone of Cold", 6),
    ("Confusion", 4),
    ("Contact Other Plane", 5),
    ("Control Weather", 7),
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
    ("Discern Lies", 4),
    ("Discern Location", 8),
    ("Dispel Magic", 3),
    ("Dispel Magic (Greater)", 6),
    ("Divination", 4),
    ("Dominate Monster", 9),
    ("Dominate Person", 5),
    ("Elemental Swarm", 9),
    ("Enervation", 4),
    ("Enlarge Person", 1),
    ("Enthrall", 2),
    ("Eyebite", 6),
    ("Fairy Ring Retreat", 7),
    ("False Life", 2),
    ("Familiar Double", 7),
    ("Fear", 4),
    ("Feast of Ashes", 2),
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
    ("Gentle Repose", 2),
    ("Glitterdust", 2),
    ("Glyph of Warding", 3),
    ("Guards and Wards", 6),
    ("Guidance", 0),
    ("Guiding Star", 3),
    ("Harm", 7),
    ("Heal", 7),
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
    ("Irresistible Dance", 8),
    ("Legend Lore", 6),
    ("Levitate", 2),
    ("Light", 0),
    ("Lightning Bolt", 3),
    ("Locate Creature", 4),
    ("Locate Object", 3),
    ("Mage Armor", 1),
    ("Magic Jar", 5),
    ("Major Creation", 5),
    ("Mark of Justice", 5),
    ("Mask Dweomer", 1),
    ("Maze", 8),
    ("Mending", 0),
    ("Message", 0),
    ("Mind Blank", 8),
    ("Mind Fog", 5),
    ("Minor Creation", 4),
    ("Moment of Prescience", 8),
    ("Mount", 1),
    ("Nature's Exile", 3),
    ("Nauseating Dart", 1),
    ("Neutralize Poison", 4),
    ("Obscuring Mist", 1),
    ("Overland Flight", 5),
    ("Perceive Cues", 2),
    ("Persistent Vigor", 4),
    ("Phantasmal Killer", 4),
    ("Phase Door", 7),
    ("Plane Shift", 7),
    ("Poison", 4),
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
    ("Scare", 2),
    ("Screech", 3),
    ("Scrying", 4),
    ("Scrying (Greater)", 7),
    ("Secret Chest", 5),
    ("Secure Shelter", 4),
    ("See Invisibility", 2),
    ("Sepia Snake Sigil", 3),
    ("Severed Fate", 2),
    ("Share Senses", 3),
    ("Slay Living", 6),
    ("Sleep", 1),
    ("Sleepwalk", 4),
    ("Sleet Storm", 3),
    ("Solid Fog", 4),
    ("Soul Bind", 9),
    ("Speak with Dead", 3),
    ("Speak with Haunt", 4),
    ("Spectral Hand", 2),
    ("Spite", 4),
    ("Stabilize", 0),
    ("Status", 2),
    ("Stinking Cloud", 3),
    ("Stone to Flesh", 6),
    ("Storm of Vengeance", 9),
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
    ("Swarm Skin", 6),
    ("Symbol of Death", 8),
    ("Symbol of Fear", 6),
    ("Symbol of Insanity", 8),
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
    ("True Seeing", 6),
    ("Unseen Servant", 1),
    ("Vampiric Touch", 3),
    ("Vision", 7),
    ("Vomit Swarm", 2),
    ("Wail of the Banshee", 9),
    ("Water Walk", 3),
    ("Waves of Exhaustion", 7),
    ("Waves of Fatigue", 5),
    ("Web", 2),
    ("Zone of Truth", 2),
];

/// The Witch spell level for `spell_key`, or `None` when the spell is
/// not on the Witch list at all. Mirrors
/// `alchemist_spell_list::alchemist_spell_level`'s own shape.
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
        assert_eq!(WITCH_SPELL_LIST.len(), 249, "249 real CLASSES:Witch= records");
        let expected = [15, 26, 36, 31, 31, 27, 24, 24, 21, 14];
        for (level, want) in expected.iter().enumerate() {
            let count = WITCH_SPELL_LIST
                .iter()
                .filter(|(_, l)| usize::from(*l) == level)
                .count();
            assert_eq!(count, *want, "spell level {level} count");
        }
    }

    /// Witch is a full 0-9 caster WITH real cantrips -- 15 of them --
    /// unlike Bloodrager, whose leading zero column is a sentinel and
    /// whose list stops at 4th level.
    #[test]
    fn witch_is_a_full_nine_level_caster_with_real_cantrips() {
        for (name, level) in WITCH_SPELL_LIST {
            assert!(*level <= 9, "{name} at level {level}: 9 is the ceiling");
        }
        assert!(
            WITCH_SPELL_LIST.iter().any(|(_, l)| *l == 0),
            "Witch genuinely has 0-level spells"
        );
        assert!(WITCH_SPELL_LIST.iter().any(|(_, l)| *l == 9), "and 9th-level spells");
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
