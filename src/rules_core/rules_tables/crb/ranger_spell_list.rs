//! PF1 Ranger spell list — per-class spell-level overrides.
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
//! tag directly, isolating only the Ranger-specific level for each of the
//! **114** real records that name Ranger at all: 40 first-level, 36
//! second-level, 28 third-level, 10 fourth-level -- matching the real PF1
//! Ranger spell-level ceiling of 4th (no ranger entry exists above 4th
//! level anywhere in the corpus, consistent with the Ranger
//! spells-per-day table topping out at 4th-level spells).
//!
//! **Widened 2026-07-27 (task #26) from CRB-only to all ingested books.**
//! Per-file: **51 from `cr_spells.lst` + 46 from `apg_spells.lst` + 17
//! from `acg_spells.lst` = 114**, all names distinct (no `.MOD` records
//! name Ranger at all, so there is nothing to graft or dedupe).
//! Per-file ceiling check: `grep -c Ranger` returns exactly 51 / 46 / 17,
//! matching the parse line-for-line in every file.
//!
//! **This module did NOT have the `CLASSES:` substring bug.** Its
//! original 51 CRB entries are byte-identical to a correct token-split
//! re-parse of `cr_spells.lst`, including all 7 records where Ranger sits
//! mid-group (e.g. `CLASSES:Bard,Ranger,Sorcerer,Wizard=1`). The only
//! defect was book scope: PF1 does not scope a class's spell list by
//! sourcebook, so the APG and ACG ranger spells belonged here all along.
//! Ruling that scope is CRB+APG+ACG for every list: team lead,
//! 2026-07-27 (`risks-and-open-questions.md` item 53).
//!
//! **Corpus reachability: all 114 resolve** against the union of the
//! three ingested books' own `SPELL_LIST` tables (1,075 keys) -- never an
//! invented name. The cross-check test asserts exactly that union; a
//! CRB-only check would now reject every APG/ACG entry as fictional.
//!
//! Regenerate by parsing the `CLASSES:` token in all three spell files --
//! split the body on `|`, `rpartition` each group on `=`, then
//! membership-test the comma-separated name list, taking that group's own
//! level (not the record's collapsed minimum). Never substring-match
//! `Ranger=`.

/// (spell key, Ranger-specific spell level 1-4). A real Ranger may only
/// prepare a spell that appears in this table, subject to the character's
/// own spell-level access ceiling for their ranger level -- see
/// `ranger_spell_level` for the lookup helper.
pub const RANGER_SPELL_LIST: &[(&str, u8)] = &[
    ("Accelerate Poison", 2),
    ("Air Step", 2),
    ("Alarm", 1),
    ("Allfood", 2),
    ("Animal Growth", 4),
    ("Animal Messenger", 1),
    ("Animal Purpose Training", 1),
    ("Ant Haul", 1),
    ("Arrow Eruption", 2),
    ("Aspect of the Bear", 2),
    ("Aspect of the Falcon", 1),
    ("Aspect of the Stag", 3),
    ("Aspect of the Wolf", 4),
    ("Barkskin", 2),
    ("Bear's Endurance", 2),
    ("Blessing of the Salamander", 4),
    ("Bloodhound", 2),
    ("Bloody Claws", 3),
    ("Bow Spirit", 4),
    ("Bullet Ward", 2),
    ("Call Animal", 1),
    ("Calm Animals", 1),
    ("Campfire Wall", 2),
    ("Cat's Grace", 2),
    ("Chameleon Stride", 2),
    ("Chameleon Stride (Greater)", 3),
    ("Charm Animal", 1),
    ("Cloak of Shade", 1),
    ("Cloak of Winds", 3),
    ("Command Plants", 3),
    ("Commune with Nature", 4),
    ("Companion Life Link", 2),
    ("Create Treasure Map", 2),
    ("Cure Light Wounds", 2),
    ("Cure Moderate Wounds", 3),
    ("Cure Serious Wounds", 4),
    ("Dancing Lantern", 1),
    ("Darkvision", 3),
    ("Delay Poison", 1),
    ("Detect Aberration", 1),
    ("Detect Animals or Plants", 1),
    ("Detect Poison", 1),
    ("Detect Snares and Pits", 1),
    ("Diminish Plants", 3),
    ("Eagle Eye", 2),
    ("Endure Elements", 1),
    ("Enemy Insight", 2),
    ("Entangle", 1),
    ("Feather Step", 1),
    ("Feather Step (Mass)", 3),
    ("Freedom of Movement", 4),
    ("Glide", 1),
    ("Gravity Bow", 1),
    ("Grove of Respite", 4),
    ("Guiding Star", 2),
    ("Heightened Awareness", 1),
    ("Hide Campsite", 2),
    ("Hide from Animals", 1),
    ("Hold Animal", 2),
    ("Hunter's Eye", 2),
    ("Hunter's Howl", 1),
    ("Instant Enemy", 3),
    ("Invisibility Alarm", 1),
    ("Jump", 1),
    ("Keen Senses", 1),
    ("Lead Blades", 1),
    ("Life Bubble", 3),
    ("Lockjaw", 2),
    ("Longstrider", 1),
    ("Longstrider (Greater)", 3),
    ("Magic Fang", 1),
    ("Magic Fang (Greater)", 3),
    ("Negate Aroma", 1),
    ("Neutralize Poison", 3),
    ("Nondetection", 3),
    ("Owl's Wisdom", 2),
    ("Pass without Trace", 1),
    ("Perceive Cues", 2),
    ("Plant Growth", 3),
    ("Protection from Energy", 2),
    ("Protective Spirit", 2),
    ("Read Magic", 1),
    ("Reduce Animal", 3),
    ("Refine Improvised Weapon", 1),
    ("Remove Disease", 3),
    ("Repel Vermin", 3),
    ("Residual Tracking", 1),
    ("Resist Energy", 1),
    ("Shield Companion", 2),
    ("Sickening Entanglement", 2),
    ("Slipstream", 2),
    ("Snare", 2),
    ("Speak with Animals", 1),
    ("Speak with Plants", 2),
    ("Spike Growth", 2),
    ("Stench of Prey", 3),
    ("Stone Call", 2),
    ("Strong Jaw", 3),
    ("Summon Nature's Ally I", 1),
    ("Summon Nature's Ally II", 2),
    ("Summon Nature's Ally III", 3),
    ("Summon Nature's Ally IV", 4),
    ("Thorn Javelin", 1),
    ("Thorny Entanglement", 3),
    ("Thunderstomp", 1),
    ("Thunderstomp (Greater)", 3),
    ("Tireless Pursuers", 3),
    ("Tireless Pursuit", 1),
    ("Tree Shape", 3),
    ("Tree Stride", 4),
    ("Venomous Bolt", 3),
    ("Versatile Weapon", 2),
    ("Water Walk", 3),
    ("Wind Wall", 2),
];

/// Looks up a spell's Ranger-specific spell level (1-4). `None` means the
/// named spell is not on the real Ranger spell list at all -- either
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
    use crate::rules_core::rules_tables::acg::spell_list as acg_spell_list;
    use crate::rules_core::rules_tables::apg::spell_list as apg_spell_list;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn ranger_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(RANGER_SPELL_LIST.len(), 114);
    }

    /// Guards the book-scope widening (task #26). One anchor per ingested
    /// book, each of which names Ranger mid-group so its raw line carries
    /// no `Ranger=` substring at all:
    /// `Alarm` is `CLASSES:Bard,Ranger,Sorcerer,Wizard=1` (CRB),
    /// `Ant Haul` is `CLASSES:Alchemist,Cleric,Druid,Ranger,Sorcerer,Wizard=1` (APG),
    /// `Air Step` is `CLASSES:Alchemist,Bard,Cleric,Druid,Ranger,Sorcerer,Witch,Wizard=2` (ACG).
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        assert_eq!(ranger_spell_level("Alarm"), Some(1));
        assert_eq!(ranger_spell_level("Ant Haul"), Some(1));
        assert_eq!(ranger_spell_level("Air Step"), Some(2));
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

    /// Ranger's list spans all three ingested books, so a CRB-only
    /// cross-check would reject every APG/ACG entry as fictional. Checks
    /// the union instead -- still a real "never an invented name"
    /// guarantee, just scoped to everything this repo actually ingests.
    #[test]
    fn every_ranger_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in RANGER_SPELL_LIST {
            let known = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            assert!(known, "{key} is not a real spell key in any ingested book");
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
        assert_eq!(count_at(1), 40);
        assert_eq!(count_at(2), 36);
        assert_eq!(count_at(3), 28);
        assert_eq!(count_at(4), 10);
    }
}
