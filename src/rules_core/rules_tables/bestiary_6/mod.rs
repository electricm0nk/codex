//! Bestiary 6 (`SOURCESHORT:B6`) — `companion` and `spell` (SD-31 wave 24:
//! see `spell_list` for the book's 2 Scalykind-subdomain spell rows).
//!
//! # The second monster-less bestiary, and the cleanest one-to-one book in the lane
//!
//! Like Bestiary 5, B6 carries **zero** monsters (`loop-instruction.md`'s
//! corpus-shape notes; it is a player-options dataset). Its companion rows pair
//! exactly: 14 creatures, 12 advancement abilities, every ability owned by the
//! creature whose species names it.
//!
//! ```text
//! python3 scripts/classify_companion_rows.py bestiary_6
//! book                              crea  abil  clas  named  prerace  prefix  ORPHAN
//! bestiary_6                          14    12     0     12       12       0       0
//! ```
//!
//! `named` and `prerace` both fire on all 12 — the creature row names the
//! advancement outright *and* the advancement's own `PRERACE:` names the
//! creature back — which is the both-shapes-agree confirmation Horror Adventures
//! provided on a single pair, here on twelve.
//!
//! # Two creature rows carry no ability at all, and that is the corpus's claim
//!
//! 14 creatures against 12 abilities is not a transcription shortfall: two of
//! this book's companions have no `CompanionAdvancement` row in the corpus. A
//! reader that "balanced" the two counts would be inventing records.

mod companion_data;
mod monster_data;
pub mod spell_list;

pub use super::companion_chassis::{
    CompanionAbilityDelivery, CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord,
    NaturalAttack, Speed, StatAdjustment,
};

// `decisions.md §20` no_record-to-zero, round 3: this book's own
// `monster_ability` orphans (`monster_data.rs`'s own header derives the
// count). Zero monster rows of its own, so every one ships owner-less by
// construction -- see `monster_data.rs`'s header for the exact keys and
// `reach_gate.rs::UNREACHED_RECORD_FINDINGS` for the pinned non-reach.
// `NaturalAttack`/`Speed`/`StatAdjustment` are deliberately NOT re-imported
// from `monster_chassis` here -- this module already imports the companion
// chassis' own same-named types above, and re-exporting both under one bare
// name is exactly how a later reader builds a monster's speed out of a
// companion's struct (`bestiary_2`'s module doc names the same hazard).
pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
};

/// Every companion creature this book defines, in corpus row order.
pub const fn companions_static() -> &'static [CompanionRecord] {
    companion_data::COMPANIONS
}

/// Every companion ability record this book defines, in corpus row order.
pub const fn companion_abilities_static() -> &'static [CompanionAbilityRecord] {
    companion_data::COMPANION_ABILITIES
}

/// Every companion creature this book defines, in corpus row order.
pub fn companions() -> &'static [CompanionRecord] {
    companions_static()
}

/// Every companion ability record this book defines, in corpus row order.
pub fn companion_abilities() -> &'static [CompanionAbilityRecord] {
    companion_abilities_static()
}

/// Every monster stat block this book defines (0 rows -- see `monster_data.rs`).
pub const fn monsters_static() -> &'static [MonsterStatBlock] {
    monster_data::MONSTERS
}

/// Every monster-ability record this book defines, in corpus row order.
pub const fn monster_abilities_static() -> &'static [MonsterAbilityRecord] {
    monster_data::MONSTER_ABILITIES
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From `docs/work-inventory.json`'s own units for this book: 26 companion
    /// units, 14 creature rows and 12 ability rows.
    #[test]
    fn the_book_defines_fourteen_companions_and_twelve_abilities() {
        assert_eq!(companions().len(), 14);
        assert_eq!(companion_abilities().len(), 12);
    }

    /// The two creatures with no advancement row are named, not rounded away.
    /// This is the assertion that separates "the corpus has no row" from "the
    /// transcriber dropped one".
    #[test]
    fn exactly_two_creatures_carry_no_ability_and_they_are_named() {
        let bare: Vec<_> = companions()
            .iter()
            .filter(|c| c.ability_keys.is_empty())
            .map(|c| c.key)
            .collect();
        assert_eq!(bare.len(), 2, "found {bare:?}");
    }

    /// Verbatim spot-check against `b6_races_companion.lst`. The Amargasaurus is
    /// the row that proves `type_segments` is transcribed as the corpus states
    /// it and never defaulted: this row carries no `TYPE:` token at all, so the
    /// honest value is an EMPTY list, not the `["Companion", …]` every other
    /// book's rows carry.
    #[test]
    fn the_amargasaurus_matches_its_corpus_row_including_an_absent_type_token() {
        let companion = companions()
            .iter()
            .find(|c| c.key == "Companion (Amargasaurus)")
            .expect("Amargasaurus is in this book");
        assert_eq!(companion.size, Some("M"));
        assert_eq!(companion.race_subtype, Some("AnimalCompanionDinosaur"));
        assert_eq!(companion.monster_class, Some("Companion:2"));
        assert!(companion.type_segments.is_empty());
        assert_eq!(companion.ability_keys, &["Companion Advancement ~ Amargasaurus"]);
    }

    /// Every ability row in this book is owned by a creature row of this book.
    #[test]
    fn every_ability_row_names_at_least_one_owner_in_this_book() {
        let keys: Vec<_> = companions().iter().map(|c| c.key).collect();
        for ability in companion_abilities() {
            assert!(!ability.owners.is_empty(), "{} is an orphan", ability.key);
            for owner in ability.owners {
                assert!(
                    keys.contains(owner),
                    "{}: owner {owner} is not a creature in this book",
                    ability.key
                );
            }
        }
    }
}
