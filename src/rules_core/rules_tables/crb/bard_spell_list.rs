//! PF1 Bard spell list — per-class spell-level overrides.
//!
//! Mirrors `sorcerer_spell_list.rs`'s own doc comment exactly, substituting
//! Bard: `crb::spell_list::SPELL_LIST`'s `level` field is the MINIMUM
//! spell level across every class named in the corpus's `CLASSES:` tag
//! for that record, not necessarily the Bard-specific level. This table
//! re-parses the same corpus record's raw `CLASSES:` tag directly,
//! isolating only the Bard-specific level for each of the **264** real
//! records that name Bard at all: 19 cantrips (0th level), 49
//! first-level, 61 second-level, 46 third-level, 34 fourth-level, 26
//! fifth-level, 29 sixth-level -- matching the real PF1 Bard spell-level
//! ceiling of 6th (the Bard Spells Known and Spells Per Day tables both
//! top out at a 6th-level column).
//!
//! **Widened 2026-07-27 (task #28) from CRB-only to all ingested books**
//! (`risks-and-open-questions.md` item 53). Per-file: **164 from
//! `cr_spells.lst` + 62 from `apg_spells.lst` + 38 from `acg_spells.lst`
//! = 264**, all names distinct, no `.MOD` record assigns Bard.
//!
//! Per-file ceiling check: `grep -c Bard` returns 166 / 63 / 38. The two
//! CRB and one APG lines above the parse are all `.MOD` records carrying
//! **no `CLASSES:` token at all** (`Eagle's Splendor.MOD`,
//! `Geas/Quest.MOD`, `Bard's Escape.MOD` -- the last matching only
//! because "Bard" is in its own name). Each one's base record does name
//! Bard and IS counted, so nothing is missing.
//!
//! **This module did NOT have the `CLASSES:` substring bug.** Its
//! original 164 CRB entries are byte-identical to a correct token-split
//! re-parse of `cr_spells.lst`, including all 125 records where Bard sits
//! mid-group. The only defect was book scope.
//!
//! **Corpus reachability: all 264 resolve** against the union of the
//! three ingested books' own `SPELL_LIST` tables (1,075 keys) -- never an
//! invented name. The cross-check test asserts exactly that union; a
//! CRB-only check would now reject every APG/ACG entry as fictional.
//!
//! Regenerate by parsing the `CLASSES:` token in all three spell files --
//! split the body on `|`, `rpartition` each group on `=`, strip any
//! trailing `[...]` gate from the level, then membership-test the
//! comma-separated name list. Never substring-match `Bard=`.

/// (spell key, Bard-specific spell level 0-6). A real Bard may only
/// know a spell that appears in this table, subject to the character's own
/// spell-level access ceiling (for 1st+ level spells) and the Bard Spells
/// Known table's per-level cap -- see `bard_spell_level` for the lookup
/// helper.
pub const BARD_SPELL_LIST: &[(&str, u8)] = &[
    ("Adjustable Disguise", 3),
    ("Adjustable Polymorph", 4),
    ("Air Step", 2),
    ("Alarm", 1),
    ("Alter Musical Instrument", 1),
    ("Alter Self", 2),
    ("Analyze Dweomer", 6),
    ("Animal Messenger", 2),
    ("Animal Purpose Training", 2),
    ("Animal Trance", 2),
    ("Animate Objects", 6),
    ("Animate Rope", 1),
    ("Anonymous Interaction", 2),
    ("Arcane Concordance", 3),
    ("Bard's Escape", 5),
    ("Beguiling Gift", 1),
    ("Blindness/Deafness", 2),
    ("Blink", 3),
    ("Blood Biography", 2),
    ("Blur", 2),
    ("Blurred Movement", 1),
    ("Borrow Skill", 1),
    ("Break Enchantment", 4),
    ("Brilliant Inspiration", 6),
    ("Bullet Ward", 2),
    ("Buoyancy", 2),
    ("Cacophonous Call", 2),
    ("Cacophonous Call (Mass)", 5),
    ("Calm Emotions", 2),
    ("Campfire Wall", 3),
    ("Cat's Grace", 2),
    ("Cat's Grace (Mass)", 6),
    ("Cause Fear", 1),
    ("Charm Monster", 3),
    ("Charm Monster (Mass)", 6),
    ("Charm Person", 1),
    ("Clairaudience/Clairvoyance", 3),
    ("Cloak of Dreams", 5),
    ("Comprehend Languages", 1),
    ("Confusion", 3),
    ("Confusion (Lesser)", 1),
    ("Contingent Action", 3),
    ("Contingent Scroll", 4),
    ("Coordinated Effort", 3),
    ("Create Treasure Map", 2),
    ("Crushing Despair", 3),
    ("Cure Critical Wounds", 4),
    ("Cure Light Wounds", 1),
    ("Cure Light Wounds (Mass)", 5),
    ("Cure Moderate Wounds", 2),
    ("Cure Moderate Wounds (Mass)", 6),
    ("Cure Serious Wounds", 3),
    ("Dancing Lantern", 1),
    ("Dancing Lights", 0),
    ("Darkness", 2),
    ("Daylight", 3),
    ("Daze", 0),
    ("Daze Monster", 2),
    ("Deadly Finale", 6),
    ("Deafening Song Bolt", 5),
    ("Deep Slumber", 3),
    ("Delay Poison", 2),
    ("Denounce", 4),
    ("Detect Magic", 0),
    ("Detect Scrying", 4),
    ("Detect Secret Doors", 1),
    ("Detect Thoughts", 2),
    ("Dimension Door", 4),
    ("Discern Next of Kin", 1),
    ("Discordant Blast", 4),
    ("Disguise Self", 1),
    ("Disguise Weapon", 1),
    ("Dispel Magic", 3),
    ("Dispel Magic (Greater)", 5),
    ("Displacement", 3),
    ("Dominate Person", 4),
    ("Dream", 5),
    ("Dust of Twilight", 2),
    ("Eagle's Splendor", 2),
    ("Eagle's Splendor (Mass)", 6),
    ("Elemental Speech", 3),
    ("Enter Image", 2),
    ("Enthrall", 2),
    ("Erase", 1),
    ("Euphoric Tranquility", 6),
    ("Expeditious Retreat", 1),
    ("Eyebite", 6),
    ("False Vision", 5),
    ("Fear", 3),
    ("Feast on Fear", 4),
    ("Feather Fall", 1),
    ("Feather Step", 1),
    ("Feather Step (Mass)", 3),
    ("Find the Path", 6),
    ("Flare", 0),
    ("Flare Burst", 1),
    ("Flexible Fury", 3),
    ("Focused Scrutiny", 2),
    ("Foe to Friend", 5),
    ("Fool's Forbiddance", 6),
    ("Fox's Cunning", 2),
    ("Fox's Cunning (Mass)", 6),
    ("Freedom of Movement", 4),
    ("Frozen Note", 5),
    ("Gallant Inspiration", 2),
    ("Gaseous Form", 3),
    ("Geas (Lesser)", 4),
    ("Geas/Quest", 6),
    ("Getaway", 6),
    ("Ghost Sound", 0),
    ("Ghostbane Dirge", 2),
    ("Ghostbane Dirge (Mass)", 4),
    ("Glibness", 3),
    ("Glitterdust", 2),
    ("Glue Seal", 1),
    ("Good Hope", 3),
    ("Grease", 1),
    ("Hallucinatory Terrain", 4),
    ("Haste", 3),
    ("Heightened Awareness", 1),
    ("Heightened Reflexes", 2),
    ("Heroes' Feast", 6),
    ("Heroic Finale", 4),
    ("Heroic Fortune", 2),
    ("Heroic Fortune (Mass)", 4),
    ("Heroism", 2),
    ("Heroism (Greater)", 5),
    ("Hidden Speech", 2),
    ("Hideous Laughter", 1),
    ("Hold Monster", 4),
    ("Hold Person", 2),
    ("Honeyed Tongue", 2),
    ("Hypnotic Pattern", 2),
    ("Hypnotism", 1),
    ("Identify", 1),
    ("Illusory Script", 3),
    ("Innocence", 1),
    ("Investigative Mind", 2),
    ("Invigorate", 1),
    ("Invigorate (Mass)", 3),
    ("Invisibility", 2),
    ("Invisibility (Greater)", 4),
    ("Invisibility Alarm", 1),
    ("Invisibility Sphere", 3),
    ("Irresistible Dance", 6),
    ("Jester's Jaunt", 3),
    ("Know Direction", 0),
    ("Legend Lore", 4),
    ("Light", 0),
    ("Locate Creature", 4),
    ("Locate Object", 2),
    ("Lullaby", 0),
    ("Mage Hand", 0),
    ("Magic Aura", 1),
    ("Magic Mouth", 1),
    ("Magnifying Chime", 6),
    ("Major Image", 3),
    ("Marching Chant", 2),
    ("Memorize Page", 1),
    ("Memory Lapse", 1),
    ("Mending", 0),
    ("Message", 0),
    ("Mind Fog", 5),
    ("Mindlocked Messenger", 2),
    ("Minor Image", 2),
    ("Mirage Arcana", 5),
    ("Mirror Image", 2),
    ("Misdirection", 2),
    ("Mislead", 5),
    ("Modify Memory", 4),
    ("Muffle Sound", 2),
    ("Neutralize Poison", 4),
    ("Nightmare", 5),
    ("Obscure Object", 1),
    ("Open/Close", 0),
    ("Path of Glory", 2),
    ("Path of Glory (Greater)", 4),
    ("Permanent Image", 6),
    ("Persistent Image", 5),
    ("Phantasmal Web", 5),
    ("Phantom Steed", 3),
    ("Pied Piping", 6),
    ("Pierce Disguise", 3),
    ("Prestidigitation", 0),
    ("Programmed Image", 6),
    ("Project Image", 6),
    ("Purging Finale", 3),
    ("Pyrotechnics", 2),
    ("Rage", 2),
    ("Rainbow Pattern", 4),
    ("Read Magic", 0),
    ("Remove Curse", 3),
    ("Remove Fear", 1),
    ("Repel Vermin", 4),
    ("Resistance", 0),
    ("Restful Sleep", 1),
    ("Reviving Finale", 3),
    ("Saving Finale", 1),
    ("Scare", 2),
    ("Scrying", 3),
    ("Scrying (Greater)", 6),
    ("Sculpt Sound", 3),
    ("Secret Page", 3),
    ("Secure Shelter", 4),
    ("See Invisibility", 3),
    ("Seek Thoughts", 3),
    ("Seeming", 5),
    ("Sepia Snake Sigil", 3),
    ("Shadow Conjuration", 4),
    ("Shadow Evocation", 5),
    ("Shadow Walk", 5),
    ("Share Language", 1),
    ("Shatter", 2),
    ("Shout", 4),
    ("Shout (Greater)", 6),
    ("Sift", 0),
    ("Silence", 2),
    ("Silent Image", 1),
    ("Silent Table", 2),
    ("Sleep", 1),
    ("Slow", 3),
    ("Solid Note", 1),
    ("Song of Discord", 5),
    ("Sonic Form", 6),
    ("Sonic Scream", 2),
    ("Sound Burst", 2),
    ("Spark", 0),
    ("Speak with Animals", 3),
    ("Speak with Plants", 4),
    ("Stunning Finale", 5),
    ("Suggestion", 2),
    ("Suggestion (Mass)", 5),
    ("Summon Instrument", 0),
    ("Summon Monster I", 1),
    ("Summon Monster II", 2),
    ("Summon Monster III", 3),
    ("Summon Monster IV", 4),
    ("Summon Monster V", 5),
    ("Summon Monster VI", 6),
    ("Summon Swarm", 2),
    ("Sympathetic Vibration", 6),
    ("Thundering Drums", 3),
    ("Timely Inspiration", 1),
    ("Tiny Hut", 3),
    ("Tongues", 2),
    ("Touch of Gracelessness", 1),
    ("Treasure Stitching", 4),
    ("Triggered Suggestion", 3),
    ("Undetectable Alignment", 1),
    ("Unseen Servant", 1),
    ("Unwilling Shield", 5),
    ("Unwitting Ally", 0),
    ("Vanish", 1),
    ("Veil", 6),
    ("Ventriloquism", 1),
    ("Versatile Weapon", 2),
    ("Wall of Blindness/Deafness", 4),
    ("Wall of Nausea", 3),
    ("Wandering Star Motes", 4),
    ("Whip of Ants", 6),
    ("Whip of Centipedes", 5),
    ("Whip of Spiders", 2),
    ("Whispering Wind", 2),
    ("Zone of Silence", 4),
];

/// Looks up a spell's Bard-specific spell level (0-6). `None` means the
/// named spell is not on the real Bard spell list at all -- either
/// it's not a real spell, or it's a real spell no Bard can ever know.
pub fn bard_spell_level(spell_key: &str) -> Option<u8> {
    BARD_SPELL_LIST
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
    fn bard_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(BARD_SPELL_LIST.len(), 264);
    }

    /// Guards the book-scope widening (task #28). One anchor per ingested
    /// book, each naming Bard mid-group so its raw line carries no
    /// `Bard=` substring at all:
    /// `Alarm` is `CLASSES:Bard,Ranger,Sorcerer,Wizard=1` (CRB),
    /// `Beguiling Gift` is `CLASSES:Bard,Witch=1` (APG),
    /// `Adjustable Disguise` is
    /// `CLASSES:Alchemist,Antipaladin,Bard,Inquisitor,Sorcerer,Witch,Wizard=3` (ACG).
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        assert_eq!(bard_spell_level("Alarm"), Some(1));
        assert_eq!(bard_spell_level("Beguiling Gift"), Some(1));
        assert_eq!(bard_spell_level("Adjustable Disguise"), Some(3));
    }

    #[test]
    fn every_bard_spell_level_is_within_the_real_bard_ceiling() {
        for (key, level) in BARD_SPELL_LIST {
            assert!(
                (0..=6).contains(level),
                "{key} has out-of-range Bard spell level {level}"
            );
        }
    }

    /// Bard's list spans all three ingested books, so a CRB-only
    /// cross-check would reject every APG/ACG entry as fictional. Checks
    /// the union instead -- still a real "never an invented name"
    /// guarantee, just scoped to everything this repo actually ingests.
    #[test]
    fn every_bard_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in BARD_SPELL_LIST {
            let known = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            assert!(known, "{key} is not a real spell key in any ingested book");
        }
    }

    #[test]
    fn bard_spell_level_looks_up_known_values() {
        assert_eq!(bard_spell_level("Dancing Lights"), Some(0));
        assert_eq!(bard_spell_level("Cure Light Wounds"), Some(1));
        assert_eq!(bard_spell_level("Grease"), Some(1));
        assert_eq!(bard_spell_level("Nonexistent Spell"), None);
    }

    #[test]
    fn level_distribution_matches_the_real_corpus_parse() {
        let count_at = |level: u8| BARD_SPELL_LIST.iter().filter(|(_, l)| *l == level).count();
        assert_eq!(count_at(0), 19);
        assert_eq!(count_at(1), 49);
        assert_eq!(count_at(2), 61);
        assert_eq!(count_at(3), 46);
        assert_eq!(count_at(4), 34);
        assert_eq!(count_at(5), 26);
        assert_eq!(count_at(6), 29);
    }
}
