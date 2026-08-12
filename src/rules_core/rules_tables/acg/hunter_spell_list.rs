//! ACG Hunter spell list — the union of the Druid and Ranger general spell
//! lists, filtered to Hunter's own level-6 spells-known ceiling.
//!
//! Source: PCGen `advanced_class_guide/acg_classes.lst`'s `CLASS:Hunter`
//! record carries `SPELLLIST:2|Druid|Ranger` — Hunter's spell list is not a
//! freshly-ingested list of its own, but the union of the already-ingested,
//! already-verified Druid (`druid_spell_list::DRUID_SPELL_LIST`, 271
//! records) and Ranger (`ranger_spell_list::RANGER_SPELL_LIST`, 114
//! records) general lists. This module derives that union directly from
//! the two source tables via [`hunter_spell_level`] rather than
//! hand-transcribing a third, independent 255-entry array — a duplicate
//! transcription would both risk copy error at that scale and be able to
//! silently drift out of sync if either source list is ever corrected.
//!
//! **Union size: 300 distinct spell keys** (271 + 114 - 85 names that
//! appear on both lists). **85 of the 300 keys are on both the Druid and
//! Ranger lists; 27 of those 85 carry a different spell level on the two
//! lists** (e.g. `Cure Light Wounds` is Druid 1st / Ranger 2nd; `Animal
//! Growth` is Druid 5th / Ranger 4th) — independently re-derived from the
//! live contents of both source modules, not merely re-asserting a prior
//! count (see `tests` below for the reproduction).
//!
//! **Modeling choice (not corpus-stated): a spell present on both lists at
//! two different levels resolves to the LOWER of the two levels.** PCGen's
//! `SPELLLIST:2|Druid|Ranger` token itself states only that the two lists
//! are unioned, not how to arbitrate a level conflict between them, and no
//! PF1 primary source names this specific Hunter edge case either. Taking
//! the lower level is the more permissive, player-favorable reading (a
//! Hunter never pays more for a spell than the cheaper of her two source
//! lists would charge) and is the simplest rule that is still internally
//! consistent (a single deterministic level per spell, independent of
//! which source list "wins"). This is the same kind of explicit,
//! documented modeling decision as the Witch familiar-store choice
//! elsewhere in this codebase — named here rather than silently assumed.
//!
//! **Hunter's own spells-known ceiling is 6th level** (the Hunter Spells
//! Known table's highest column, `pilot_compute::hunter_spells_known_table`).
//! Filtering the 300-key union to that ceiling drops 45 keys — every one
//! of them a Druid-only 7th-9th-level spell (Ranger's own list never goes
//! above 4th level, so Ranger contributes no out-of-range keys), leaving
//! **255 castable spells**, distributed 14 / 56 / 65 / 45 / 32 / 21 / 22
//! across levels 0-6. [`hunter_spell_level`] enforces this ceiling
//! directly (returns `None` above 6th), so no separate range check is
//! needed at any call site.
//!
//! The six Summon Nature's Ally spells (I-VI) Hunter automatically knows
//! per the corpus's own `KNOWNSPELLS:Summon Nature's Ally I|II|III|IV|V|VI`
//! token are already members of this union at their expected levels
//! (I=1 ... VI=6, identical on both source lists) — no separate handling
//! is needed here for list membership; the automatic-known-on-top-of-the-
//! table grant itself is grounded in `pilot_compute.rs`, not this module.

use super::super::crb::druid_spell_list;
use super::super::crb::ranger_spell_list;

/// Hunter's own spells-known ceiling (the Hunter Spells Known table's
/// highest spell-level column). See this module's own doc comment.
pub const HUNTER_SPELL_CEILING: u8 = 6;

/// Looks up a spell's Hunter-specific spell level (0-6), unioning the real
/// Druid and Ranger general spell lists and taking the lower level when a
/// spell sits at two different levels on those two lists (see this
/// module's own doc comment for the modeling choice). `None` means the
/// named spell is not on either source list at all, or its union level
/// exceeds Hunter's own 6th-level ceiling.
pub fn hunter_spell_level(spell_key: &str) -> Option<u8> {
    let druid_level = druid_spell_list::druid_spell_level(spell_key);
    let ranger_level = ranger_spell_list::ranger_spell_level(spell_key);
    let union_level = match (druid_level, ranger_level) {
        (Some(druid_level), Some(ranger_level)) => druid_level.min(ranger_level),
        (Some(druid_level), None) => druid_level,
        (None, Some(ranger_level)) => ranger_level,
        (None, None) => return None,
    };
    if union_level <= HUNTER_SPELL_CEILING {
        Some(union_level)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::crb::druid_spell_list::DRUID_SPELL_LIST;
    use crate::rules_core::rules_tables::crb::ranger_spell_list::RANGER_SPELL_LIST;
    use std::collections::BTreeSet;

    /// Every key on either source list, deduplicated -- the raw union
    /// before Hunter's own level-6 ceiling is applied.
    fn union_keys() -> BTreeSet<&'static str> {
        DRUID_SPELL_LIST
            .iter()
            .map(|(key, _)| *key)
            .chain(RANGER_SPELL_LIST.iter().map(|(key, _)| *key))
            .collect()
    }

    #[test]
    fn union_of_the_two_source_lists_has_300_distinct_keys() {
        assert_eq!(union_keys().len(), 300);
    }

    #[test]
    fn exactly_85_keys_are_on_both_source_lists() {
        let druid_keys: BTreeSet<&str> = DRUID_SPELL_LIST.iter().map(|(k, _)| *k).collect();
        let ranger_keys: BTreeSet<&str> = RANGER_SPELL_LIST.iter().map(|(k, _)| *k).collect();
        let both = druid_keys.intersection(&ranger_keys).count();
        assert_eq!(both, 85);
    }

    #[test]
    fn exactly_27_of_the_shared_keys_conflict_in_level() {
        let druid_keys: std::collections::BTreeMap<&str, u8> =
            DRUID_SPELL_LIST.iter().cloned().collect();
        let ranger_keys: std::collections::BTreeMap<&str, u8> =
            RANGER_SPELL_LIST.iter().cloned().collect();
        let conflicts = druid_keys
            .iter()
            .filter(|(key, druid_level)| {
                ranger_keys.get(*key).is_some_and(|ranger_level| ranger_level != *druid_level)
            })
            .count();
        assert_eq!(conflicts, 27);
    }

    /// `Cure Light Wounds` is Druid 1st / Ranger 2nd -- a real conflicting
    /// entry. The take-the-lower ruling must resolve it to 1st, not 2nd.
    #[test]
    fn a_conflicting_spell_resolves_to_the_lower_of_the_two_levels() {
        assert_eq!(druid_spell_list::druid_spell_level("Cure Light Wounds"), Some(1));
        assert_eq!(ranger_spell_list::ranger_spell_level("Cure Light Wounds"), Some(2));
        assert_eq!(hunter_spell_level("Cure Light Wounds"), Some(1));
    }

    /// `Animal Growth` is Druid 5th / Ranger 4th -- take-the-lower must
    /// resolve it to 4th (the Ranger side is lower here, proving the rule
    /// isn't "always prefer Druid").
    #[test]
    fn a_conflicting_spell_prefers_ranger_when_ranger_is_lower() {
        assert_eq!(druid_spell_list::druid_spell_level("Animal Growth"), Some(5));
        assert_eq!(ranger_spell_list::ranger_spell_level("Animal Growth"), Some(4));
        assert_eq!(hunter_spell_level("Animal Growth"), Some(4));
    }

    /// The six Summon Nature's Ally spells sit at the same level on both
    /// source lists (I=1 ... VI=6), so they resolve unambiguously and are
    /// all within Hunter's own 6th-level ceiling.
    #[test]
    fn all_six_summon_natures_ally_spells_resolve_within_the_hunter_ceiling() {
        for (name, level) in [
            ("Summon Nature's Ally I", 1),
            ("Summon Nature's Ally II", 2),
            ("Summon Nature's Ally III", 3),
            ("Summon Nature's Ally IV", 4),
            ("Summon Nature's Ally V", 5),
            ("Summon Nature's Ally VI", 6),
        ] {
            assert_eq!(hunter_spell_level(name), Some(level), "{name}");
        }
    }

    /// Summon Nature's Ally VII-IX are real Druid 7th-9th-level spells
    /// (Ranger's list never carries them at all -- Ranger tops out at
    /// 4th), and they must be excluded by Hunter's own 6th-level ceiling.
    #[test]
    fn summon_natures_ally_vii_through_ix_are_excluded_above_the_ceiling() {
        for name in
            ["Summon Nature's Ally VII", "Summon Nature's Ally VIII", "Summon Nature's Ally IX"]
        {
            assert_eq!(
                ranger_spell_list::ranger_spell_level(name),
                None,
                "{name} must not be on the Ranger list"
            );
            assert_eq!(hunter_spell_level(name), None, "{name} must be excluded above the ceiling");
        }
    }

    /// Any real Druid- or Ranger-only 7th-9th-level spell is excluded the
    /// same way, not merely the six named Summon Nature's Ally examples.
    #[test]
    fn every_out_of_range_druid_spell_is_excluded_by_the_hunter_ceiling() {
        let excluded = DRUID_SPELL_LIST
            .iter()
            .filter(|(key, level)| *level > HUNTER_SPELL_CEILING && hunter_spell_level(key).is_none())
            .count();
        let total_out_of_range =
            DRUID_SPELL_LIST.iter().filter(|(_, level)| *level > HUNTER_SPELL_CEILING).count();
        assert_eq!(excluded, total_out_of_range);
        assert!(total_out_of_range > 0);
    }

    /// A spell name from neither list is not on Hunter's list either.
    #[test]
    fn a_spell_on_neither_source_list_is_not_castable() {
        assert_eq!(hunter_spell_level("Definitely Not A Real Spell"), None);
    }

    /// The full castable union (post-ceiling-filter) is exactly 255
    /// spells, distributed 14/56/65/45/32/21/22 across levels 0-6 --
    /// independently re-derived from the live source lists, matching the
    /// scoping session's own re-derivation.
    #[test]
    fn castable_union_has_255_spells_with_the_expected_level_distribution() {
        let mut counts = [0u32; 7];
        let mut total = 0u32;
        for key in union_keys() {
            if let Some(level) = hunter_spell_level(key) {
                counts[usize::from(level)] += 1;
                total += 1;
            }
        }
        assert_eq!(total, 255);
        assert_eq!(counts, [14, 56, 65, 45, 32, 21, 22]);
    }
}
