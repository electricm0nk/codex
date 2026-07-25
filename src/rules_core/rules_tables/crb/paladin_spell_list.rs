//! PF1 CRB Paladin spell list — per-class spell-level overrides.
//!
//! Mirrors `ranger_spell_list.rs`'s own doc comment exactly, substituting
//! Paladin for Ranger: `crb::spell_list::SPELL_LIST`'s `level` field is the
//! MINIMUM spell level across every class named in the corpus's `CLASSES:`
//! tag for that record, not necessarily the Paladin-specific level. This
//! table re-parses the same corpus record's raw `CLASSES:` tag directly
//! (`core_rulebook/cr_spells.lst`), isolating only the Paladin-specific
//! level for each of the 45 real records that name Paladin at all: 16
//! first-level, 9 second-level, 11 third-level, 9 fourth-level -- matching
//! the real PF1 Paladin spell-level ceiling of 4th (verified via a direct
//! parse of the raw `CLASSES:` token; no paladin entry exists above 4th
//! level in the corpus, consistent with the PF1 Core Rulebook Paladin
//! spells-per-day table topping out at 4th-level spells). Every `key` here
//! is spot-checked against a real, exact `crb::spell_list::SPELL_LIST` key
//! -- this is a strict subset of the 652 CRB spell records, never an
//! invented name.
//!
//! Regenerate by re-parsing `cr_spells.lst`'s `CLASSES:` tag for any
//! `|`-separated group whose name list contains "Paladin", taking that
//! group's own level (not the record's collapsed minimum), if the corpus
//! changes.

/// (spell key, Paladin-specific spell level 1-4). A real CRB Paladin may
/// only prepare a spell that appears in this table, subject to the
/// character's own spell-level access ceiling for their paladin level --
/// see `paladin_spell_level` for the lookup helper.
pub const PALADIN_SPELL_LIST: &[(&str, u8)] = &[
    ("Bless", 1),
    ("Bless Water", 1),
    ("Bless Weapon", 1),
    ("Break Enchantment", 4),
    ("Bull's Strength", 2),
    ("Create Water", 1),
    ("Cure Light Wounds", 1),
    ("Cure Moderate Wounds", 3),
    ("Cure Serious Wounds", 4),
    ("Daylight", 3),
    ("Death Ward", 4),
    ("Delay Poison", 2),
    ("Detect Poison", 1),
    ("Detect Undead", 1),
    ("Discern Lies", 3),
    ("Dispel Chaos", 4),
    ("Dispel Evil", 4),
    ("Dispel Magic", 3),
    ("Divine Favor", 1),
    ("Eagle's Splendor", 2),
    ("Endure Elements", 1),
    ("Heal Mount", 3),
    ("Holy Sword", 4),
    ("Magic Circle against Chaos", 3),
    ("Magic Circle against Evil", 3),
    ("Magic Weapon", 1),
    ("Magic Weapon (Greater)", 3),
    ("Mark of Justice", 4),
    ("Neutralize Poison", 4),
    ("Owl's Wisdom", 2),
    ("Prayer", 3),
    ("Protection from Chaos", 1),
    ("Protection from Evil", 1),
    ("Read Magic", 1),
    ("Remove Blindness/Deafness", 3),
    ("Remove Curse", 3),
    ("Remove Paralysis", 2),
    ("Resist Energy", 2),
    ("Resistance", 1),
    ("Restoration", 4),
    ("Restoration (Lesser)", 1),
    ("Shield Other", 2),
    ("Undetectable Alignment", 2),
    ("Virtue", 1),
    ("Zone of Truth", 2),
];

/// Looks up a spell's Paladin-specific spell level (1-4). `None` means the
/// named spell is not on the real CRB Paladin spell list at all -- either
/// it's not a real spell, or it's a real spell no Paladin can ever prepare.
pub fn paladin_spell_level(spell_key: &str) -> Option<u8> {
    PALADIN_SPELL_LIST
        .iter()
        .find(|(key, _)| *key == spell_key)
        .map(|(_, level)| *level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn paladin_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(PALADIN_SPELL_LIST.len(), 45);
    }

    #[test]
    fn every_paladin_spell_level_is_within_the_real_paladin_ceiling() {
        for (key, level) in PALADIN_SPELL_LIST {
            assert!(
                (1..=4).contains(level),
                "{key} has out-of-range Paladin spell level {level}"
            );
        }
    }

    #[test]
    fn every_paladin_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in PALADIN_SPELL_LIST {
            assert!(
                SPELL_LIST.iter().any(|entry| entry.key == *key),
                "{key} is not a real SPELL_LIST key"
            );
        }
    }

    #[test]
    fn paladin_spell_level_looks_up_known_values() {
        assert_eq!(paladin_spell_level("Bless"), Some(1));
        assert_eq!(paladin_spell_level("Break Enchantment"), Some(4));
        assert_eq!(paladin_spell_level("Cure Light Wounds"), Some(1));
        assert_eq!(paladin_spell_level("Magic Missile"), None);
    }

    #[test]
    fn level_distribution_matches_the_real_corpus_parse() {
        let count_at =
            |level: u8| PALADIN_SPELL_LIST.iter().filter(|(_, l)| *l == level).count();
        assert_eq!(count_at(1), 16);
        assert_eq!(count_at(2), 9);
        assert_eq!(count_at(3), 11);
        assert_eq!(count_at(4), 9);
    }
}
