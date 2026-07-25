//! PF1 CRB Ranger spell list — per-class spell-level overrides.
//!
//! `crb::spell_list::SPELL_LIST`'s `level` field is the MINIMUM spell level
//! across every class named in the corpus's `CLASSES:` tag for that record
//! (see that module's own doc comment) -- not necessarily the
//! Ranger-specific level. For example `Animal Growth` carries
//! `CLASSES:Ranger=4|Druid,Sorcerer,Wizard=5`, which collapses to a minimum
//! of 4 in `SPELL_LIST` (coincidentally matching Ranger here), but nothing
//! guarantees that agreement in general -- a record could just as easily
//! sit lower for another class than for Ranger.
//!
//! This table instead re-parses the same corpus record's raw `CLASSES:`
//! tag directly (`core_rulebook/cr_spells.lst`), isolating only the
//! Ranger-specific level for each of the 51 real records that name Ranger
//! at all: 19 first-level, 12 second-level, 14 third-level, 6 fourth-level
//! -- matching the real PF1 Ranger spell-level ceiling of 4th (verified via
//! a direct parse of the raw `CLASSES:` token; no ranger entry exists above
//! 4th level in the corpus, consistent with the PF1 Core Rulebook Ranger
//! spells-per-day table topping out at 4th-level spells). Every `key` here
//! is spot-checked against a real, exact `crb::spell_list::SPELL_LIST` key
//! -- this is a strict subset of the 652 CRB spell records, never an
//! invented name.
//!
//! Regenerate by re-parsing `cr_spells.lst`'s `CLASSES:` tag for any
//! `|`-separated group whose name list contains "Ranger", taking that
//! group's own level (not the record's collapsed minimum), if the corpus
//! changes.

/// (spell key, Ranger-specific spell level 1-4). A real CRB Ranger may only
/// prepare a spell that appears in this table, subject to the character's
/// own spell-level access ceiling for their ranger level -- see
/// `ranger_spell_level` for the lookup helper.
pub const RANGER_SPELL_LIST: &[(&str, u8)] = &[
    ("Alarm", 1),
    ("Animal Growth", 4),
    ("Animal Messenger", 1),
    ("Barkskin", 2),
    ("Bear's Endurance", 2),
    ("Calm Animals", 1),
    ("Cat's Grace", 2),
    ("Charm Animal", 1),
    ("Command Plants", 3),
    ("Commune with Nature", 4),
    ("Cure Light Wounds", 2),
    ("Cure Moderate Wounds", 3),
    ("Cure Serious Wounds", 4),
    ("Darkvision", 3),
    ("Delay Poison", 1),
    ("Detect Animals or Plants", 1),
    ("Detect Poison", 1),
    ("Detect Snares and Pits", 1),
    ("Diminish Plants", 3),
    ("Endure Elements", 1),
    ("Entangle", 1),
    ("Freedom of Movement", 4),
    ("Hide from Animals", 1),
    ("Hold Animal", 2),
    ("Jump", 1),
    ("Longstrider", 1),
    ("Magic Fang", 1),
    ("Magic Fang (Greater)", 3),
    ("Neutralize Poison", 3),
    ("Nondetection", 3),
    ("Owl's Wisdom", 2),
    ("Pass without Trace", 1),
    ("Plant Growth", 3),
    ("Protection from Energy", 2),
    ("Read Magic", 1),
    ("Reduce Animal", 3),
    ("Remove Disease", 3),
    ("Repel Vermin", 3),
    ("Resist Energy", 1),
    ("Snare", 2),
    ("Speak with Animals", 1),
    ("Speak with Plants", 2),
    ("Spike Growth", 2),
    ("Summon Nature's Ally I", 1),
    ("Summon Nature's Ally II", 2),
    ("Summon Nature's Ally III", 3),
    ("Summon Nature's Ally IV", 4),
    ("Tree Shape", 3),
    ("Tree Stride", 4),
    ("Water Walk", 3),
    ("Wind Wall", 2),
];

/// Looks up a spell's Ranger-specific spell level (1-4). `None` means the
/// named spell is not on the real CRB Ranger spell list at all -- either
/// it's not a real spell, or it's a real spell no Ranger can ever prepare.
pub fn ranger_spell_level(spell_key: &str) -> Option<u8> {
    RANGER_SPELL_LIST
        .iter()
        .find(|(key, _)| *key == spell_key)
        .map(|(_, level)| *level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn ranger_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(RANGER_SPELL_LIST.len(), 51);
    }

    #[test]
    fn every_ranger_spell_level_is_within_the_real_ranger_ceiling() {
        for (key, level) in RANGER_SPELL_LIST {
            assert!(
                (1..=4).contains(level),
                "{key} has out-of-range Ranger spell level {level}"
            );
        }
    }

    #[test]
    fn every_ranger_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in RANGER_SPELL_LIST {
            assert!(
                SPELL_LIST.iter().any(|entry| entry.key == *key),
                "{key} is not a real SPELL_LIST key"
            );
        }
    }

    #[test]
    fn ranger_spell_level_looks_up_known_values() {
        assert_eq!(ranger_spell_level("Alarm"), Some(1));
        assert_eq!(ranger_spell_level("Animal Growth"), Some(4));
        assert_eq!(ranger_spell_level("Cure Light Wounds"), Some(2));
        assert_eq!(ranger_spell_level("Magic Missile"), None);
    }

    #[test]
    fn level_distribution_matches_the_real_corpus_parse() {
        let count_at =
            |level: u8| RANGER_SPELL_LIST.iter().filter(|(_, l)| *l == level).count();
        assert_eq!(count_at(1), 19);
        assert_eq!(count_at(2), 12);
        assert_eq!(count_at(3), 14);
        assert_eq!(count_at(4), 6);
    }
}
