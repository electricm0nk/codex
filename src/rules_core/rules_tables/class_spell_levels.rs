//! Per-class spell levels, keyed by the hub's own `class:<id>` vocabulary.
//!
//! **The defect this module closes.** A spell record's `level` field in
//! `crb::spell_list` (and its APG/ACG siblings) is the MINIMUM spell level
//! across every class named in that record's corpus `CLASSES:` tag — not
//! the level for any one class. `Hideous Laughter` is
//! `CLASSES:Bard=1|Sorcerer,Wizard=2`, so its record level is 1, and every
//! surface reading that field showed a Wizard "Level 1" for a spell a
//! Wizard learns at 2. Re-derived from the shipped tables: **67 of the 580
//! spells on the Wizard list carry a record level that is a wrong number
//! for a Wizard, every one of them biased low** (see
//! `crb::wizard_spell_list`'s own regression test, which recomputes that
//! figure rather than asserting it from memory). The per-class tables that
//! hold the true answer have existed in this crate for some time; nothing
//! routed a class id to them.
//!
//! **What is and is not answerable.** Twelve classes have a real ingested
//! per-class spell list and are answered from it. Four more are answered
//! by a corpus-*stated* redirect: PCGen's class records carry a
//! `SPELLLIST:` token naming the list a class borrows wholesale —
//! `CLASS:Arcanist ... SPELLLIST:1|Wizard`,
//! `CLASS:Investigator ... SPELLLIST:1|Alchemist`,
//! `CLASS:Skald ... SPELLLIST:1|Bard`,
//! `CLASS:Warpriest ... SPELLLIST:1|Cleric` (all in `acg_classes.lst`).
//! Those are relationships the corpus asserts, not overlaps this module
//! inferred from the data.
//!
//! Everything else answers **unknown**, deliberately. `Magus`, `Summoner`
//! and `Oracle` all name themselves in real `CLASSES:` tags in the
//! ingested spell files, so their per-class levels are knowable — but no
//! table for them has been ingested, so this module reports that it does
//! not know rather than falling back to the record's minimum level. That
//! fallback is precisely the wrong number this module exists to remove;
//! reintroducing it as a default would be worse than a labelled gap (see
//! `docs/governance/no-stub-mvp-doctrine.md`). Callers must distinguish
//! "this class has no list here" ([`class_has_spell_list`]) from "this
//! spell is not on that class's list" ([`class_spell_level`] returning
//! `None` for a class that does have one) — the two are different facts
//! and read differently to a player.
//!
//! Wizard is *not* derived from Sorcerer despite the two lists being
//! nearly identical; see `crb::wizard_spell_list`'s doc comment for why.

use super::{acg, apg, crb};

/// A class id paired with the ingested table that answers for it.
///
/// The four `SPELLLIST:`-redirect classes appear here pointing at the
/// table they borrow — the redirect is corpus-stated, so serving the
/// borrowed table is reporting the corpus, not guessing.
const STATIC_CLASS_SPELL_LISTS: &[(&str, &[(&str, u8)])] = &[
    // Classes that name themselves in the corpus's own `CLASSES:` tags.
    ("class:alchemist", apg::alchemist_spell_list::ALCHEMIST_SPELL_LIST),
    ("class:bard", crb::bard_spell_list::BARD_SPELL_LIST),
    ("class:bloodrager", acg::bloodrager_spell_list::BLOODRAGER_SPELL_LIST),
    ("class:cleric", crb::cleric_spell_list::CLERIC_SPELL_LIST),
    ("class:druid", crb::druid_spell_list::DRUID_SPELL_LIST),
    ("class:inquisitor", apg::inquisitor_spell_list::INQUISITOR_SPELL_LIST),
    ("class:paladin", crb::paladin_spell_list::PALADIN_SPELL_LIST),
    ("class:ranger", crb::ranger_spell_list::RANGER_SPELL_LIST),
    ("class:shaman", acg::shaman_spell_list::SHAMAN_SPELL_LIST),
    ("class:sorcerer", crb::sorcerer_spell_list::SORCERER_SPELL_LIST),
    ("class:witch", apg::witch_spell_list::WITCH_SPELL_LIST),
    ("class:wizard", crb::wizard_spell_list::WIZARD_SPELL_LIST),
    // Corpus-stated `SPELLLIST:` redirects — see this module's doc comment.
    ("class:arcanist", crb::wizard_spell_list::WIZARD_SPELL_LIST),
    ("class:investigator", apg::alchemist_spell_list::ALCHEMIST_SPELL_LIST),
    ("class:skald", crb::bard_spell_list::BARD_SPELL_LIST),
    ("class:warpriest", crb::cleric_spell_list::CLERIC_SPELL_LIST),
];

/// Hunter's list is a computed union of Druid's and Ranger's rather than a
/// static array (corpus-stated: `CLASS:Hunter ... SPELLLIST:2|Druid|Ranger`),
/// so it is dispatched separately from [`STATIC_CLASS_SPELL_LISTS`].
const HUNTER_CLASS_ID: &str = "class:hunter";

/// Whether this crate has a real per-class spell list for `class_id`.
///
/// `false` does **not** mean the class casts no spells — see this module's
/// doc comment on Magus/Summoner/Oracle. It means only that no per-class
/// level can be reported here without inventing one, so a caller must
/// render the absence rather than substitute a record-level default.
pub fn class_has_spell_list(class_id: &str) -> bool {
    class_id == HUNTER_CLASS_ID
        || STATIC_CLASS_SPELL_LISTS
            .iter()
            .any(|(id, _)| *id == class_id)
}

/// Every class id [`class_spell_level`] can answer for, sorted. Callers
/// that need to describe the boundary (a UI note, a diagnostic) should
/// read it from here rather than restating the list.
pub fn classes_with_spell_lists() -> Vec<&'static str> {
    let mut ids: Vec<&'static str> = STATIC_CLASS_SPELL_LISTS.iter().map(|(id, _)| *id).collect();
    ids.push(HUNTER_CLASS_ID);
    ids.sort_unstable();
    ids
}

/// The spell level `spell_key` sits at **for `class_id` specifically**.
///
/// `None` covers two distinct situations, which [`class_has_spell_list`]
/// separates: either this crate has no list for `class_id` at all, or it
/// has one and the spell is not on it (no member of that class can ever
/// cast it). Never falls back to the record's own minimum-across-classes
/// level — that value is the bug, not a default.
pub fn class_spell_level(class_id: &str, spell_key: &str) -> Option<u8> {
    if class_id == HUNTER_CLASS_ID {
        return acg::hunter_spell_list::hunter_spell_level(spell_key);
    }
    STATIC_CLASS_SPELL_LISTS
        .iter()
        .find(|(id, _)| *id == class_id)
        .and_then(|(_, list)| {
            list.iter()
                .find(|(key, _)| *key == spell_key)
                .map(|(_, level)| *level)
        })
}

/// Every `(spell key, level)` pair on `class_id`'s list, sorted by key.
///
/// `None` when this crate has no list for that class — distinct from
/// `Some(vec![])`, which would claim the class casts nothing at all.
/// Hunter's union is materialised here from its two source tables.
pub fn class_spell_list_entries(class_id: &str) -> Option<Vec<(&'static str, u8)>> {
    if class_id == HUNTER_CLASS_ID {
        let mut keys: Vec<&'static str> = crb::druid_spell_list::DRUID_SPELL_LIST
            .iter()
            .map(|(key, _)| *key)
            .chain(
                crb::ranger_spell_list::RANGER_SPELL_LIST
                    .iter()
                    .map(|(key, _)| *key),
            )
            .collect();
        keys.sort_unstable();
        keys.dedup();
        let mut entries: Vec<(&'static str, u8)> = keys
            .into_iter()
            .filter_map(|key| {
                acg::hunter_spell_list::hunter_spell_level(key).map(|level| (key, level))
            })
            .collect();
        entries.sort_unstable();
        return Some(entries);
    }

    STATIC_CLASS_SPELL_LISTS
        .iter()
        .find(|(id, _)| *id == class_id)
        .map(|(_, list)| {
            let mut entries: Vec<(&'static str, u8)> = list.to_vec();
            entries.sort_unstable();
            entries
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn a_wizard_learns_hideous_laughter_at_second_level_not_first() {
        assert_eq!(class_spell_level("class:wizard", "Hideous Laughter"), Some(2));
        assert_eq!(class_spell_level("class:bard", "Hideous Laughter"), Some(1));
        assert_eq!(
            class_spell_level("class:sorcerer", "Hideous Laughter"),
            Some(2)
        );
    }

    #[test]
    fn a_class_with_no_ingested_spell_list_answers_unknown_rather_than_guessing() {
        assert!(!class_has_spell_list("class:magus"));
        assert_eq!(class_spell_level("class:magus", "Hideous Laughter"), None);
        // Magus really does name itself in the corpus, so this is a gap in
        // what has been ingested, not a claim that a Magus casts nothing.
        assert!(!class_has_spell_list("class:summoner"));
        assert!(!class_has_spell_list("class:oracle"));
        // A non-caster, and an id that is not a class at all.
        assert!(!class_has_spell_list("class:fighter"));
        assert!(!class_has_spell_list("class:demo"));
        assert!(!class_has_spell_list(""));
    }

    /// More spells that sit at genuinely different levels for different
    /// classes — each one a row the old record-level rendering got wrong
    /// for at least one class. Raw `CLASSES:` tags, read off
    /// `cr_spells.lst` (not recalled):
    /// `Cure Light Wounds` -> `Bard,Cleric,Druid,Paladin=1|Ranger=2`,
    /// `Animal Growth` -> `Ranger=4|Druid,Sorcerer,Wizard=5`,
    /// `Cure Moderate Wounds` -> `Bard,Cleric=2|Druid,Paladin,Ranger=3`,
    /// `Neutralize Poison` -> `Druid,Ranger=3|Bard,Cleric,Paladin=4`,
    /// `Bestow Curse` -> `Cleric=3|Sorcerer,Wizard=4`.
    #[test]
    fn spells_that_differ_across_classes_resolve_per_class() {
        assert_eq!(class_spell_level("class:cleric", "Cure Light Wounds"), Some(1));
        assert_eq!(class_spell_level("class:druid", "Cure Light Wounds"), Some(1));
        assert_eq!(class_spell_level("class:paladin", "Cure Light Wounds"), Some(1));
        assert_eq!(class_spell_level("class:ranger", "Cure Light Wounds"), Some(2));

        // Ranger is the only class that gets Animal Growth at 4; the
        // record level is therefore 4 and reads wrong for everyone else.
        assert_eq!(class_spell_level("class:ranger", "Animal Growth"), Some(4));
        assert_eq!(class_spell_level("class:druid", "Animal Growth"), Some(5));
        assert_eq!(class_spell_level("class:wizard", "Animal Growth"), Some(5));
        assert_eq!(class_spell_level("class:sorcerer", "Animal Growth"), Some(5));

        assert_eq!(class_spell_level("class:bard", "Cure Moderate Wounds"), Some(2));
        assert_eq!(class_spell_level("class:cleric", "Cure Moderate Wounds"), Some(2));
        assert_eq!(class_spell_level("class:druid", "Cure Moderate Wounds"), Some(3));
        assert_eq!(class_spell_level("class:paladin", "Cure Moderate Wounds"), Some(3));
        assert_eq!(class_spell_level("class:ranger", "Cure Moderate Wounds"), Some(3));

        assert_eq!(class_spell_level("class:druid", "Neutralize Poison"), Some(3));
        assert_eq!(class_spell_level("class:ranger", "Neutralize Poison"), Some(3));
        assert_eq!(class_spell_level("class:bard", "Neutralize Poison"), Some(4));
        assert_eq!(class_spell_level("class:cleric", "Neutralize Poison"), Some(4));
        assert_eq!(class_spell_level("class:paladin", "Neutralize Poison"), Some(4));

        assert_eq!(class_spell_level("class:cleric", "Bestow Curse"), Some(3));
        assert_eq!(class_spell_level("class:wizard", "Bestow Curse"), Some(4));
    }

    /// Each spell above carries a record `level` that is the minimum
    /// across its classes — the single number the sheet used to show every
    /// class, and the reason at least one class read wrong.
    #[test]
    fn the_record_level_is_the_minimum_and_so_wrong_for_the_higher_classes() {
        let record_level = |key: &str| {
            SPELL_LIST
                .iter()
                .find(|entry| entry.key == key)
                .map(|entry| entry.level)
        };
        assert_eq!(record_level("Hideous Laughter"), Some(1));
        assert_eq!(record_level("Cure Light Wounds"), Some(1));
        assert_eq!(record_level("Animal Growth"), Some(4));
        assert_eq!(record_level("Cure Moderate Wounds"), Some(2));
        assert_eq!(record_level("Neutralize Poison"), Some(3));
        assert_eq!(record_level("Bestow Curse"), Some(3));
    }

    #[test]
    fn a_spell_absent_from_a_classs_list_is_unknown_not_the_record_level() {
        // Real CRB spell, record level 3, but no Paladin ever casts it.
        assert!(SPELL_LIST.iter().any(|entry| entry.key == "Fireball"));
        assert!(class_has_spell_list("class:paladin"));
        assert_eq!(class_spell_level("class:paladin", "Fireball"), None);
    }

    #[test]
    fn every_dispatched_class_has_a_non_empty_list() {
        for class_id in classes_with_spell_lists() {
            assert!(class_has_spell_list(class_id), "{class_id}");
            let entries =
                class_spell_list_entries(class_id).expect("a dispatched class must have entries");
            assert!(!entries.is_empty(), "{class_id} dispatches an empty list");
        }
    }

    #[test]
    fn the_dispatch_roster_is_the_sixteen_static_classes_plus_hunter() {
        let ids = classes_with_spell_lists();
        assert_eq!(ids.len(), 17);
        assert!(ids.contains(&"class:hunter"));
        assert!(ids.contains(&"class:wizard"));
        // Sorted, so a caller can binary-search or diff it stably.
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
    }

    /// The corpus-stated `SPELLLIST:` redirects serve exactly the borrowed
    /// list, not an approximation of it.
    #[test]
    fn spelllist_redirect_classes_serve_the_borrowed_list_verbatim() {
        assert_eq!(
            class_spell_list_entries("class:arcanist"),
            class_spell_list_entries("class:wizard")
        );
        assert_eq!(
            class_spell_list_entries("class:investigator"),
            class_spell_list_entries("class:alchemist")
        );
        assert_eq!(
            class_spell_list_entries("class:skald"),
            class_spell_list_entries("class:bard")
        );
        assert_eq!(
            class_spell_list_entries("class:warpriest"),
            class_spell_list_entries("class:cleric")
        );
        // And the borrowed answer is the per-class one, not the record's.
        assert_eq!(
            class_spell_level("class:arcanist", "Hideous Laughter"),
            Some(2)
        );
    }

    #[test]
    fn class_spell_list_entries_is_none_not_empty_for_an_unknown_class() {
        assert!(class_spell_list_entries("class:magus").is_none());
        assert!(class_spell_list_entries("class:wizard").is_some());
    }

    #[test]
    fn hunters_union_is_materialised_and_agrees_with_its_lookup() {
        let entries = class_spell_list_entries("class:hunter").expect("hunter has a list");
        assert_eq!(entries.len(), 255);
        for (key, level) in &entries {
            assert_eq!(class_spell_level("class:hunter", key), Some(*level));
            assert!(*level <= 6, "{key} exceeds Hunter's 6th-level ceiling");
        }
    }

    /// **Not every dispatched key has an ingested spell record, and that
    /// is deliberate.** `bloodrager_spell_list`'s own doc comment records
    /// the ruling (team lead, 2026-07-27): 73 of its 200 entries are
    /// `.MOD` grafts whose base records live in Ultimate Magic, Ultimate
    /// Combat and the Advanced Race Guide — books this repo does not
    /// ingest. They stay on the list because they genuinely are on
    /// Bloodrager's PF1 spell list; what is missing is the *record*, not
    /// the membership. `shaman_spell_list` carries the same shape.
    ///
    /// This is inert for the display path, which joins in the other
    /// direction: the Spells tab resolves a selection against the catalog
    /// first and only then asks this module for that record's per-class
    /// level, so a class key with no catalog record is never reached.
    /// Pinned here so the gap stays visible and any change to it is a
    /// deliberate edit rather than a silent drift.
    #[test]
    fn the_dispatched_keys_with_no_ingested_record_are_the_known_documented_gap() {
        use crate::rules_core::rules_tables::acg::spell_list as acg_spell_list;
        use crate::rules_core::rules_tables::apg::spell_list as apg_spell_list;

        let mut gaps: Vec<(&str, usize)> = Vec::new();
        for class_id in classes_with_spell_lists() {
            let missing = class_spell_list_entries(class_id)
                .expect("dispatched")
                .into_iter()
                .filter(|(key, _)| {
                    !(SPELL_LIST.iter().any(|entry| entry.key == *key)
                        || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                        || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key))
                })
                .count();
            if missing > 0 {
                gaps.push((class_id, missing));
            }
        }
        assert_eq!(gaps, vec![("class:bloodrager", 73), ("class:shaman", 21)]);
    }

    #[test]
    fn every_served_level_is_a_real_pf1_spell_level() {
        for class_id in classes_with_spell_lists() {
            for (key, level) in class_spell_list_entries(class_id).expect("dispatched") {
                assert!((0..=9).contains(&level), "{class_id} {key} level {level}");
            }
        }
    }
}
