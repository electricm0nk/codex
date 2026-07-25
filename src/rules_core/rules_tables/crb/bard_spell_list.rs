//! PF1 CRB Bard spell list — per-class spell-level overrides.
//!
//! Mirrors `sorcerer_spell_list.rs`'s own doc comment exactly, substituting
//! Bard: `crb::spell_list::SPELL_LIST`'s `level` field is the MINIMUM
//! spell level across every class named in the corpus's `CLASSES:` tag
//! for that record, not necessarily the Bard-specific level. This table
//! re-parses the same corpus record's raw `CLASSES:` tag directly
//! (`core_rulebook/cr_spells.lst`), isolating only the Bard-specific level
//! for each of the 164 real records that name Bard at all: 16 cantrips
//! (0th level), 26 first-level, 35 second-level, 29 third-level, 22
//! fourth-level, 16 fifth-level, 20 sixth-level -- matching the real PF1
//! Bard spell-level ceiling of 6th (verified via a direct parse of the raw
//! `CLASSES:` token, and against the real Bard Spells Known/Spells Per
//! Day tables, both of which top out at a 6th-level column). Every `key`
//! here is spot-checked against a real, exact `crb::spell_list::SPELL_LIST`
//! key -- this is a strict subset of the 652 CRB spell records, never an
//! invented name.
//!
//! Regenerate by re-parsing `cr_spells.lst`'s `CLASSES:` tag for any
//! `|`-separated group whose name list contains "Bard", taking that
//! group's own level (not the record's collapsed minimum), if the corpus
//! changes.

/// (spell key, Bard-specific spell level 0-6). A real CRB Bard may only
/// know a spell that appears in this table, subject to the character's own
/// spell-level access ceiling (for 1st+ level spells) and the Bard Spells
/// Known table's per-level cap -- see `bard_spell_level` for the lookup
/// helper.
pub const BARD_SPELL_LIST: &[(&str, u8)] = &[
    ("Alarm", 1),
    ("Alter Self", 2),
    ("Analyze Dweomer", 6),
    ("Animal Messenger", 2),
    ("Animal Trance", 2),
    ("Animate Objects", 6),
    ("Animate Rope", 1),
    ("Blindness/Deafness", 2),
    ("Blink", 3),
    ("Blur", 2),
    ("Break Enchantment", 4),
    ("Calm Emotions", 2),
    ("Cat's Grace", 2),
    ("Cat's Grace (Mass)", 6),
    ("Cause Fear", 1),
    ("Charm Monster", 3),
    ("Charm Monster (Mass)", 6),
    ("Charm Person", 1),
    ("Clairaudience/Clairvoyance", 3),
    ("Comprehend Languages", 1),
    ("Confusion", 3),
    ("Confusion (Lesser)", 1),
    ("Crushing Despair", 3),
    ("Cure Critical Wounds", 4),
    ("Cure Light Wounds", 1),
    ("Cure Light Wounds (Mass)", 5),
    ("Cure Moderate Wounds", 2),
    ("Cure Moderate Wounds (Mass)", 6),
    ("Cure Serious Wounds", 3),
    ("Dancing Lights", 0),
    ("Darkness", 2),
    ("Daylight", 3),
    ("Daze", 0),
    ("Daze Monster", 2),
    ("Deep Slumber", 3),
    ("Delay Poison", 2),
    ("Detect Magic", 0),
    ("Detect Scrying", 4),
    ("Detect Secret Doors", 1),
    ("Detect Thoughts", 2),
    ("Dimension Door", 4),
    ("Disguise Self", 1),
    ("Dispel Magic", 3),
    ("Dispel Magic (Greater)", 5),
    ("Displacement", 3),
    ("Dominate Person", 4),
    ("Dream", 5),
    ("Eagle's Splendor", 2),
    ("Eagle's Splendor (Mass)", 6),
    ("Enthrall", 2),
    ("Erase", 1),
    ("Expeditious Retreat", 1),
    ("Eyebite", 6),
    ("False Vision", 5),
    ("Fear", 3),
    ("Feather Fall", 1),
    ("Find the Path", 6),
    ("Flare", 0),
    ("Fox's Cunning", 2),
    ("Fox's Cunning (Mass)", 6),
    ("Freedom of Movement", 4),
    ("Gaseous Form", 3),
    ("Geas (Lesser)", 4),
    ("Geas/Quest", 6),
    ("Ghost Sound", 0),
    ("Glibness", 3),
    ("Glitterdust", 2),
    ("Good Hope", 3),
    ("Grease", 1),
    ("Hallucinatory Terrain", 4),
    ("Haste", 3),
    ("Heroes' Feast", 6),
    ("Heroism", 2),
    ("Heroism (Greater)", 5),
    ("Hideous Laughter", 1),
    ("Hold Monster", 4),
    ("Hold Person", 2),
    ("Hypnotic Pattern", 2),
    ("Hypnotism", 1),
    ("Identify", 1),
    ("Illusory Script", 3),
    ("Invisibility", 2),
    ("Invisibility (Greater)", 4),
    ("Invisibility Sphere", 3),
    ("Irresistible Dance", 6),
    ("Know Direction", 0),
    ("Legend Lore", 4),
    ("Light", 0),
    ("Locate Creature", 4),
    ("Locate Object", 2),
    ("Lullaby", 0),
    ("Mage Hand", 0),
    ("Magic Aura", 1),
    ("Magic Mouth", 1),
    ("Major Image", 3),
    ("Mending", 0),
    ("Message", 0),
    ("Mind Fog", 5),
    ("Minor Image", 2),
    ("Mirage Arcana", 5),
    ("Mirror Image", 2),
    ("Misdirection", 2),
    ("Mislead", 5),
    ("Modify Memory", 4),
    ("Neutralize Poison", 4),
    ("Nightmare", 5),
    ("Obscure Object", 1),
    ("Open/Close", 0),
    ("Permanent Image", 6),
    ("Persistent Image", 5),
    ("Phantom Steed", 3),
    ("Prestidigitation", 0),
    ("Programmed Image", 6),
    ("Project Image", 6),
    ("Pyrotechnics", 2),
    ("Rage", 2),
    ("Rainbow Pattern", 4),
    ("Read Magic", 0),
    ("Remove Curse", 3),
    ("Remove Fear", 1),
    ("Repel Vermin", 4),
    ("Resistance", 0),
    ("Scare", 2),
    ("Scrying", 3),
    ("Scrying (Greater)", 6),
    ("Sculpt Sound", 3),
    ("Secret Page", 3),
    ("Secure Shelter", 4),
    ("See Invisibility", 3),
    ("Seeming", 5),
    ("Sepia Snake Sigil", 3),
    ("Shadow Conjuration", 4),
    ("Shadow Evocation", 5),
    ("Shadow Walk", 5),
    ("Shatter", 2),
    ("Shout", 4),
    ("Shout (Greater)", 6),
    ("Silence", 2),
    ("Silent Image", 1),
    ("Sleep", 1),
    ("Slow", 3),
    ("Song of Discord", 5),
    ("Sound Burst", 2),
    ("Speak with Animals", 3),
    ("Speak with Plants", 4),
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
    ("Tiny Hut", 3),
    ("Tongues", 2),
    ("Undetectable Alignment", 1),
    ("Unseen Servant", 1),
    ("Veil", 6),
    ("Ventriloquism", 1),
    ("Whispering Wind", 2),
    ("Zone of Silence", 4),
];

/// Looks up a spell's Bard-specific spell level (0-6). `None` means the
/// named spell is not on the real CRB Bard spell list at all -- either
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
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn bard_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(BARD_SPELL_LIST.len(), 164);
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

    #[test]
    fn every_bard_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in BARD_SPELL_LIST {
            assert!(
                SPELL_LIST.iter().any(|entry| entry.key == *key),
                "{key} is not a real SPELL_LIST key"
            );
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
        assert_eq!(count_at(0), 16);
        assert_eq!(count_at(1), 26);
        assert_eq!(count_at(2), 35);
        assert_eq!(count_at(3), 29);
        assert_eq!(count_at(4), 22);
        assert_eq!(count_at(5), 16);
        assert_eq!(count_at(6), 20);
    }
}
