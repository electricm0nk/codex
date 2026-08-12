//! Princes of Darkness — Book of the Damned, Volume 1 (`SOURCESHORT:BOTD1`)
//! — `monster` + `monster_ability`.
//!
//! # Why this book, and not a denser one
//!
//! It is one of exactly two remaining books in the lane with **zero orphan
//! ability rows**: every one of its 36 `monster_ability` rows is named by a
//! monster row in this same book, so the whole book reaches the catalog or none
//! of it does. Re-derived rather than transcribed from the round-1 receipt:
//!
//! ```text
//! python3 scripts/classify_monster_ability_rows.py book_of_the_damned_volume_1
//! book                          mon  abil row-named prefix ORPHAN
//! book_of_the_damned_volume_1     5    36        36      0      0
//! ```
//!
//! `bestiary_4` carries 988 units and 152 of them are orphans; a book with
//! orphans cannot land a whole-book reach claim, and this lane's `reach_gate`
//! claims refuse partial credit.
//!
//! # The first campaign-setting book in the chassis
//!
//! Bonus Bestiary and Monster Codex are both `roleplaying_game/` books. Both
//! Book of the Damned volumes live under `campaign_setting/`, and the
//! expectation going in was that the PI screen would bite: Inner Sea Races, the
//! race-trait lane's campaign-setting book, had **12 of its 72** descriptions
//! PI-redacted (`decisions.md §45.2`), because setting-specific nation and
//! ethnicity names occur inside otherwise mechanical prose.
//!
//! (That sentence originally named the setting. `pi_table_sweep` rejected it,
//! correctly: the sweep does not read intent, and a doc comment explaining the
//! PI screen has no need to instantiate a PI term to make its point. Reworded
//! rather than baselined — `decisions.md §47.3`.)
//!
//! **It did not, and the derived reason is worth recording rather than
//! rounding away.** `data/corpus/book_of_the_damned_volume_1/LICENSE.json`
//! reads `records_redacted: 0`, and volume 2's the same. A campaign-setting
//! book's *monster* rows are not its setting prose: these records name devils
//! and demons by their Open Game Content type names, and the geography that
//! carries Product Identity lives in the book's chapters, not in a stat block.
//! "Campaign setting" predicts a PI hit rate for `race_trait` and does not
//! predict one for `monster`.
//!
//! # The link shape is Monster Codex's, at scale
//!
//! All 5 monster rows name their abilities outright with
//! `ABILITY:Special Ability|AUTOMATIC|<key>`, and every ability key is
//! namespaced `<Devil> ~ <Ability>` — the two shapes agree here, where Bonus
//! Bestiary had only the first and Monster Codex only the second.
//!
//! Three names in the monster rows' `ABILITY:` lists are **not** rows of this
//! book (`Flight Maneuverability`, `Regeneration`, `Fast Healing`) and one is a
//! row this book defines with `TYPE:Internal`
//! (`Warmonger Devil ~ Trample`, `botd1_abilities_race.lst:46`), which the work
//! inventory's `internal_namespace` trap correctly excludes from the unit set.
//! All four are recorded in [`MonsterStatBlock::external_ability_refs`] rather
//! than silently dropped, and
//! `monster_chassis::the_chassis_link_resolves_in_both_directions_for_every_book`
//! holds that they stay external.

mod monster_data;

pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
    NaturalAttack, Speed,
};

/// Every monster stat block this book defines, in corpus row order.
pub const fn monsters_static() -> &'static [MonsterStatBlock] {
    monster_data::MONSTERS
}

/// Every monster-ability record this book defines, in corpus row order.
pub const fn monster_abilities_static() -> &'static [MonsterAbilityRecord] {
    monster_data::MONSTER_ABILITIES
}

/// Every monster stat block this book defines, in corpus row order.
pub fn monsters() -> &'static [MonsterStatBlock] {
    monsters_static()
}

/// Every monster-ability record this book defines, in corpus row order.
pub fn monster_abilities() -> &'static [MonsterAbilityRecord] {
    monster_abilities_static()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both counts come from `docs/work-inventory.json`'s units for this book,
    /// never a line count over the `.lst`:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if
    /// u['book']=='book_of_the_damned_volume_1' and u['kind']=='monster'))"`
    /// -> 5, and the same for `monster_ability` -> 36.
    #[test]
    fn the_book_defines_five_monsters_and_thirty_six_abilities() {
        assert_eq!(monsters().len(), 5);
        assert_eq!(monster_abilities().len(), 36);
    }

    /// Verbatim spot-check against `botd1_races.lst:11`, checkable against the
    /// named line rather than merely self-consistent. This row is also the
    /// book's only monster whose corpus row names no natural attack at all —
    /// an honest empty, not a transcription that dropped one.
    #[test]
    fn the_lesser_host_devil_matches_its_corpus_row() {
        let devil = monsters()
            .iter()
            .find(|m| m.key == "Devil (Lesser Host)")
            .expect("Devil (Lesser Host) is in this book");
        assert_eq!(devil.source_line, 11);
        assert_eq!(devil.name, "Devil, Lesser Host (Gaav)");
        assert_eq!(devil.size, Some("S"));
        assert_eq!(devil.challenge_rating, Some("3"));
        assert_eq!(devil.monster_class, Some("Outsider (Fort/Ref):4"));
        assert_eq!(devil.race_type, Some("Outsider"));
        assert_eq!(devil.source_page, Some("p.58"));
        assert_eq!(
            devil.speeds,
            &[
                Speed { mode: "Walk", feet: 5 },
                Speed { mode: "Fly", feet: 60 },
            ]
        );
        assert!(
            devil.natural_attacks.is_empty(),
            "botd1_races.lst:11 carries no NATURALATTACKS: token"
        );
        assert_eq!(devil.external_ability_refs, &["Flight Maneuverability"]);
    }

    /// The Warmonger Devil's row names `Warmonger Devil ~ Trample`, which this
    /// book *does* define upstream — as `TYPE:Internal`
    /// (`botd1_abilities_race.lst:46`), which the inventory's
    /// `internal_namespace` trap excludes from the unit set. It must therefore
    /// be carried as an external reference and must NOT be a record here: a
    /// transcription that ingested it would ship a row the inventory does not
    /// count, and one that dropped the reference would hide a real citation.
    #[test]
    fn an_internal_typed_ability_stays_an_external_reference() {
        let devil = monsters()
            .iter()
            .find(|m| m.key == "Devil (Warmonger)")
            .expect("Devil (Warmonger) is in this book");
        assert!(devil.external_ability_refs.contains(&"Warmonger Devil ~ Trample"));
        assert!(
            !monster_abilities()
                .iter()
                .any(|a| a.key == "Warmonger Devil ~ Trample"),
            "an internal-typed row must not become a served record"
        );
    }

    /// Every ability row of this book is owned by a monster row of this book —
    /// the property that made it the correct next target, held as a test rather
    /// than left in a receipt.
    #[test]
    fn no_ability_row_of_this_book_is_an_orphan() {
        for ability in monster_abilities() {
            assert!(
                !ability.owners.is_empty(),
                "{} reaches no monster and would load without ever being shown",
                ability.key
            );
        }
    }
}
