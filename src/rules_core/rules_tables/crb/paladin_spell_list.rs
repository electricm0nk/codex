//! PF1 Paladin spell list — per-class spell-level overrides.
//!
//! Mirrors `ranger_spell_list.rs`'s own doc comment exactly, substituting
//! Paladin for Ranger: `crb::spell_list::SPELL_LIST`'s `level` field is the
//! MINIMUM spell level across every class named in the corpus's `CLASSES:`
//! tag for that record, not necessarily the Paladin-specific level. This
//! table re-parses the same corpus record's raw `CLASSES:` tag directly,
//! isolating only the Paladin-specific level for each of the **95** real
//! records that name Paladin at all: 28 first-level, 25 second-level, 23
//! third-level, 19 fourth-level -- matching the real PF1 Paladin
//! spell-level ceiling of 4th (no paladin entry exists above 4th level
//! anywhere in the corpus, consistent with the Paladin spells-per-day
//! table topping out at 4th-level spells).
//!
//! **Widened 2026-07-27 (task #27) from CRB-only to all ingested books.**
//! Per-file: **45 from `cr_spells.lst` + 38 from `apg_spells.lst` + 13
//! from `acg_spells.lst`**, which is 96 raw matches collapsing to **95**
//! distinct spells -- see the `Resounding Blow` note below. Per-file
//! ceiling check: `grep -c Paladin` returns 45 / 41 / 13; the APG ceiling
//! exceeds the parse by 3 because two lines name only `Antipaladin`'s own
//! records and one is a `#`-commented duplicate, none of which are
//! Paladin spells.
//!
//! **This module did NOT have the `CLASSES:` substring bug.** Its original
//! 45 CRB entries are byte-identical to a correct token-split re-parse of
//! `cr_spells.lst`, including all 18 records where Paladin sits mid-group.
//! Two separate defects were real here: book scope, and one record lost to
//! a bracketed level (below).
//!
//! **`Heroic Fortune` is included despite its optional-rule gate.** Its
//! token is `CLASSES:Alchemist,Bard,Cleric=2|Paladin=3[PREVAREQ:Heroic,1]`
//! -- the level carries a trailing `[PREVAREQ:...]` marking the APG Hero
//! Points optional rule. Ruling that such spells belong on the list (the
//! gate is a condition on casting, not on list membership, and this engine
//! enforces no spell-selection prerequisites today): team lead, 2026-07-27
//! (`risks-and-open-questions.md` item 54).
//!
//! **`Resounding Blow` appears once, not twice.** Three APG records carry
//! that name; two of them name Paladin (`CLASSES:Paladin=4|Inquisitor=5`
//! and `CLASSES:Antipaladin,Paladin=4|Inquisitor=5`), both at level 4. A
//! parse keyed on the raw record name rather than the real spell name
//! yields two entries for one spell.
//!
//! **Corpus reachability: all 95 resolve** against the union of the three
//! ingested books' own `SPELL_LIST` tables (1,075 keys) -- never an
//! invented name. The cross-check test asserts exactly that union; a
//! CRB-only check would now reject every APG/ACG entry as fictional.
//!
//! Regenerate by parsing the `CLASSES:` token in all three spell files --
//! split the body on `|`, `rpartition` each group on `=`, strip any
//! trailing `[...]` gate from the level, then membership-test the
//! comma-separated name list. Never substring-match `Paladin=`, and never
//! let an `int()` on the level throw a record away silently.

/// (spell key, Paladin-specific spell level 1-4). A real Paladin may
/// only prepare a spell that appears in this table, subject to the
/// character's own spell-level access ceiling for their paladin level --
/// see `paladin_spell_level` for the lookup helper.
pub const PALADIN_SPELL_LIST: &[(&str, u8)] = &[
    ("Animal Purpose Training", 1),
    ("Aura of Greater Courage", 2),
    ("Bestow Auras", 3),
    ("Bestow Grace", 2),
    ("Blaze of Glory", 4),
    ("Bless", 1),
    ("Bless Water", 1),
    ("Bless Weapon", 1),
    ("Blessed Fist", 1),
    ("Blessing of Courage and Life", 2),
    ("Break Enchantment", 4),
    ("Bull's Strength", 2),
    ("Bullet Ward", 2),
    ("Challenge Evil", 1),
    ("Corruption Resistance", 2),
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
    ("Divine Transfer", 3),
    ("Eagle's Splendor", 2),
    ("Endure Elements", 1),
    ("Fire of Entanglement", 2),
    ("Fire of Judgment", 3),
    ("Fire of Vengeance", 4),
    ("Forced Repentance", 4),
    ("Ghostbane Dirge", 1),
    ("Ghostbane Dirge (Mass)", 3),
    ("Grace", 1),
    ("Guardian of Faith", 4),
    ("Heal Mount", 3),
    ("Hero's Defiance", 1),
    ("Heroic Fortune", 3),
    ("Holy Sword", 4),
    ("Holy Whisper", 3),
    ("Honeyed Tongue", 1),
    ("Instant Armor", 2),
    ("King's Castle", 4),
    ("Knight's Calling", 1),
    ("Light Lance", 2),
    ("Magic Circle against Chaos", 3),
    ("Magic Circle against Evil", 3),
    ("Magic Weapon", 1),
    ("Magic Weapon (Greater)", 3),
    ("Mantle of Calm", 3),
    ("Mark of Justice", 4),
    ("Marks of Forbiddance", 3),
    ("Neutralize Poison", 4),
    ("Oath of Peace", 4),
    ("Owl's Wisdom", 2),
    ("Paladin's Sacrifice", 2),
    ("Planeslayer's Call", 4),
    ("Prayer", 3),
    ("Protection from Chaos", 1),
    ("Protection from Evil", 1),
    ("Rally Point", 1),
    ("Read Magic", 1),
    ("Remove Blindness/Deafness", 3),
    ("Remove Curse", 3),
    ("Remove Paralysis", 2),
    ("Resist Energy", 2),
    ("Resistance", 1),
    ("Resounding Blow", 4),
    ("Restoration", 4),
    ("Restoration (Lesser)", 1),
    ("Righteous Vigor", 2),
    ("Sacred Bond", 2),
    ("Sacrificial Oath", 4),
    ("Saddle Surge", 2),
    ("Sanctify Armor", 3),
    ("Shield Companion", 2),
    ("Shield Other", 2),
    ("Shield of Fortification", 1),
    ("Shield of Fortification (Greater)", 3),
    ("Stay the Hand", 4),
    ("Stunning Barrier", 1),
    ("Stunning Barrier (Greater)", 3),
    ("Undetectable Alignment", 2),
    ("Veil of Positive Energy", 1),
    ("Virtue", 1),
    ("Wake of Light", 2),
    ("Weapon of Awe", 2),
    ("Widen Auras", 2),
    ("Wrathful Mantle", 3),
    ("Zone of Truth", 2),
];

/// Looks up a spell's Paladin-specific spell level (1-4). `None` means the
/// named spell is not on the real Paladin spell list at all -- either
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
    use crate::rules_core::rules_tables::acg::spell_list as acg_spell_list;
    use crate::rules_core::rules_tables::apg::spell_list as apg_spell_list;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn paladin_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(PALADIN_SPELL_LIST.len(), 95);
    }

    /// Guards the book-scope widening (task #27), one anchor per ingested
    /// book. `Bull's Strength` and `Animal Purpose Training` both name
    /// Paladin mid-group, so their raw lines carry no `Paladin=`
    /// substring at all:
    /// `CLASSES:Cleric,Druid,Paladin,Sorcerer,Wizard=2` (CRB) and
    /// `CLASSES:Antipaladin,Inquisitor,Paladin,Ranger,Witch=1|Bard,Druid,Shaman=2` (ACG).
    /// APG has no mid-group Paladin record at all, so its anchor is the
    /// other shape that was silently dropped -- a bracketed optional-rule
    /// gate on the level: `CLASSES:Alchemist,Bard,Cleric=2|Paladin=3[PREVAREQ:Heroic,1]`.
    #[test]
    fn spells_tagged_mid_list_or_gated_in_their_classes_group_are_present() {
        assert_eq!(paladin_spell_level("Bull's Strength"), Some(2));
        assert_eq!(paladin_spell_level("Heroic Fortune"), Some(3));
        assert_eq!(paladin_spell_level("Animal Purpose Training"), Some(1));
    }

    /// `Resounding Blow` has three APG records; two of them name Paladin
    /// (`CLASSES:Paladin=4|Inquisitor=5` and
    /// `CLASSES:Antipaladin,Paladin=4|Inquisitor=5`), both at level 4.
    /// It must collapse to exactly ONE entry -- a parse that dedupes by
    /// raw record name rather than real spell name yields two.
    #[test]
    fn a_spell_with_two_paladin_bearing_records_appears_exactly_once() {
        let hits = PALADIN_SPELL_LIST
            .iter()
            .filter(|(key, _)| *key == "Resounding Blow")
            .count();
        assert_eq!(hits, 1, "Resounding Blow must appear exactly once");
        assert_eq!(paladin_spell_level("Resounding Blow"), Some(4));
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

    /// Paladin's list spans all three ingested books, so a CRB-only
    /// cross-check would reject every APG/ACG entry as fictional. Checks
    /// the union instead -- still a real "never an invented name"
    /// guarantee, just scoped to everything this repo actually ingests.
    #[test]
    fn every_paladin_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in PALADIN_SPELL_LIST {
            let known = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            assert!(known, "{key} is not a real spell key in any ingested book");
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
        assert_eq!(count_at(1), 28);
        assert_eq!(count_at(2), 25);
        assert_eq!(count_at(3), 23);
        assert_eq!(count_at(4), 19);
    }
}
