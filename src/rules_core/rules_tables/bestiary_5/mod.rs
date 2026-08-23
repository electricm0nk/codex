//! Bestiary 5 (`SOURCESHORT:B5`) — `companion`.
//!
//! # A "bestiary" with no monsters, whose companion rows are the largest
//! zero-orphan block left in the lane
//!
//! `loop-instruction.md`'s corpus-shape notes record that B5 carries **zero**
//! monsters — its pcc's `CAMPAIGN` line says "Only Player Options Implemented"
//! outright. So this book reaches the engine through the companion lane rather
//! than the monster lane, and `RuleSetId::B5` is its first rule set of any kind.
//!
//! ```text
//! python3 scripts/classify_companion_rows.py bestiary_5
//! book                              crea  abil  clas  named  prerace  prefix  ORPHAN
//! bestiary_5                          33    22     0     18       18       4       0
//!     GATED  Familiar (Brain Mole) — b5_races_companion_oa.lst loaded under PRECAMPAIGN:1,Occult Adventures
//!     GATED  Familiar (Chuspiki) — b5_races_companion_oa.lst loaded under PRECAMPAIGN:1,Occult Adventures
//! ```
//!
//! # Two of the book's 57 companion units are deliberately not here
//!
//! `docs/work-inventory.json` counts 57 `companion` units for this book. Two of
//! them — `Familiar (Brain Mole)` and `Familiar (Chuspiki)` — live in
//! `support/b5_races_companion_oa.lst`, which `_bestiary_5.pcc:69` loads as
//! `RACE:support/b5_races_companion_oa.lst|PRECAMPAIGN:1,Occult Adventures`.
//! Occult Adventures is not an ingested book, so those two rows are out of this
//! rule set's scope **by construction, not by omission** — the ruling
//! `decisions.md §47.2` made for Horror Adventures' Occult-Adventures-gated
//! race-trait file, applied to the same gate on a different kind.
//!
//! The exclusion is derived, never hardcoded: `classify_companion_rows`'s
//! `precampaign_gates` reads the pcc load line, because the gate is on the pcc
//! line and a `grep PRECAMPAIGN` over the `.lst` itself returns nothing.

mod companion_data;
mod monster_data;

pub use super::companion_chassis::{
    CompanionAbilityDelivery, CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord,
    NaturalAttack, Speed, StatAdjustment,
};

// `decisions.md §20` no_record-to-zero, round 3: this book's own
// `monster_ability` orphans (`monster_data.rs`'s own header derives the
// count, including the one row `parse_desc` refused). Zero monster rows of
// its own, so every transcribed row ships owner-less by construction -- see
// `monster_data.rs`'s header for the exact keys and `reach_gate.rs::
// UNREACHED_RECORD_FINDINGS` for the pinned non-reach.
// `NaturalAttack`/`Speed`/`StatAdjustment` are deliberately NOT re-imported
// from `monster_chassis` here -- this module already imports the companion
// chassis' own same-named types above (`bestiary_2`'s module doc names the
// ambiguity hazard of doing both).
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

    /// From `docs/work-inventory.json`'s own units for this book, minus the two
    /// the pcc gates on Occult Adventures: 57 units, 55 in this rule set's
    /// scope, 33 creature rows and 22 ability rows.
    #[test]
    fn the_book_defines_thirty_three_companions_and_twenty_two_abilities() {
        assert_eq!(companions().len(), 33);
        assert_eq!(companion_abilities().len(), 22);
    }

    /// The Occult-Adventures-gated rows are absent BY NAME, not by a count that
    /// any two missing records would satisfy. A future transcriber change that
    /// started following `support/` unconditionally fails here.
    #[test]
    fn the_occult_adventures_gated_familiars_are_not_in_this_rule_set() {
        for key in ["Familiar (Brain Mole)", "Familiar (Chuspiki)"] {
            assert!(
                companions().iter().all(|c| c.key != key),
                "{key} is loaded only under PRECAMPAIGN:1,Occult Adventures and is out of \
                 this rule set's scope (decisions.md §47.2)"
            );
        }
    }

    /// Verbatim spot-check against `b5_races_companion.lst`, on the row that
    /// exercises the most reader paths at once: three speed modes including one
    /// (`Jet`) no other registered book carries, and a `RACESUBTYPE` beside a
    /// `RACETYPE`.
    #[test]
    fn the_cameroceras_matches_its_corpus_row() {
        let companion = companions()
            .iter()
            .find(|c| c.key == "Companion (Cameroceras)")
            .expect("Cameroceras is in this book");
        assert_eq!(companion.size, Some("M"));
        assert_eq!(companion.race_type, Some("Companion"));
        assert_eq!(companion.race_subtype, Some("Aquatic"));
        assert_eq!(
            companion.speeds,
            &[
                Speed { mode: "Walk", feet: 5 },
                Speed { mode: "Swim", feet: 20 },
                Speed { mode: "Jet", feet: 90 },
            ]
        );
    }

    /// Every ability row in this book is owned by a creature row of this book,
    /// and every owner names it back. The chassis holds the same invariant for
    /// every registered book; this pins it for the book with the most rows.
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
