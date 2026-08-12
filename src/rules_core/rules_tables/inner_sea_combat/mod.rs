//! Inner Sea Combat (`SOURCESHORT:ISC`) — `companion`.
//!
//! # Why this book is the companion lane's pilot
//!
//! The SD-29 package pinned it before the lane had a chassis, and the pin was
//! **re-confirmed rather than trusted**: `epic-7-companion-lane-pilot` was
//! written before the race-trait lane's classifier fix moved 13 units into
//! `companion` corpus-wide, and the immediately preceding lane discovered its
//! own pinned pilot book carried none of the kind it was pinned for. Re-derived
//! for this round:
//!
//! ```text
//! python3 scripts/classify_companion_rows.py inner_sea_combat
//! book                              crea  abil  clas  named  prerace  prefix  ORPHAN
//! inner_sea_combat                     4     6     0      6        4       1       0
//! ```
//!
//! 4 creature rows + 6 ability rows = the 10 units `docs/work-inventory.json`
//! counts for this book, and **zero orphans** — every ability row is claimed by
//! a creature row of the same book, which is the predicate
//! `companion_chassis::COMPANION_BOOKS` registers on.
//!
//! # The ownership shape this book found
//!
//! `Mastery (7th Level)` carries `KEY:Worg ~ Mastery`, and this book's creature
//! is keyed `Companion (Worg)`. `monster_chassis`'s bare-prefix rule
//! (`<Owner> ~ <Leaf>` where `<Owner>` is a monster key) does not match, so the
//! row would have read as an orphan and the book as unregisterable. The
//! companion chassis resolves the prefix through the
//! `Companion (<Species>)` / `Familiar (<Species>)` wrapper, which is a naming
//! convention the corpus applies to every companion row in every book.
//!
//! # `Unable to carry a rider while flying` is shared between two creatures
//!
//! It is the one registered ability row with more than one owner (Griffon and
//! Hippogriff both name it), and it carries **no `KEY:` at all** — PCGen falls
//! back to the display name, and so does this ingest. It is also the row that
//! proves the row-named ownership shape is load-bearing: it has neither a
//! `PRERACE:` gate nor a namespaced key, so shapes 2 and 3 would both miss it.

mod companion_data;

pub use super::companion_chassis::{
    CompanionAbilityDelivery, CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord,
    NaturalAttack, Speed, StatAdjustment,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Both counts come from `docs/work-inventory.json`'s units for this book,
    /// never a line count over the `.lst`:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if u['book']=='inner_sea_combat' and
    /// u['kind']=='companion'))"` -> 10, split 4 creature / 6 ability by
    /// `scripts/classify_companion_rows.py`.
    #[test]
    fn the_book_defines_four_companions_and_six_abilities() {
        assert_eq!(companions().len(), 4);
        assert_eq!(companion_abilities().len(), 6);
    }

    /// Verbatim spot-check against `isc_races_companion.lst:5`.
    #[test]
    fn the_griffon_matches_its_corpus_row() {
        let griffon = companions()
            .iter()
            .find(|c| c.key == "Companion (Griffon)")
            .expect("Companion (Griffon) is in this book");
        assert_eq!(griffon.source_line, 5);
        assert_eq!(griffon.size, Some("L"));
        assert_eq!(griffon.race_type, Some("Magical Beast"));
        assert_eq!(griffon.monster_class, Some("Companion:2"));
        assert_eq!(griffon.natural_armor, Some(4));
        assert_eq!(
            griffon.speeds,
            &[
                Speed { mode: "Walk", feet: 30 },
                Speed { mode: "Fly", feet: 40 },
            ]
        );
        // The row's own `BONUS:STAT` tokens, in row order. Adjustments, not
        // scores: the row states `BONUS:STAT|STR|6`, and a Griffon's Strength
        // is not 6.
        assert_eq!(
            griffon.stat_adjustments,
            &[
                StatAdjustment { ability: "STR", amount: 6 },
                StatAdjustment { ability: "DEX", amount: 4 },
                StatAdjustment { ability: "CON", amount: 6 },
                StatAdjustment { ability: "INT", amount: -6 },
                StatAdjustment { ability: "WIS", amount: 2 },
                StatAdjustment { ability: "CHA", amount: -2 },
            ]
        );
        // `Scent` is named by the row and defined in Core Rulebook, not here.
        assert_eq!(griffon.external_ability_refs, &["Scent"]);
    }

    /// The shared, key-less ability row — the one that needs shape 1
    /// (row-named) ownership, because it has neither a `PRERACE:` gate nor a
    /// namespaced key.
    #[test]
    fn the_flight_restriction_is_owned_by_both_flying_companions() {
        let book = super::super::companion_chassis::companion_book("inner_sea_combat")
            .expect("Inner Sea Combat is registered");
        let restriction = book
            .companion_ability_resolve("Unable to carry a rider while flying")
            .expect("the row is in this book");
        assert_eq!(restriction.source_line, 9);
        assert_eq!(
            restriction.owners,
            &["Companion (Griffon)", "Companion (Hippogriff)"]
        );
        assert_eq!(restriction.facet, Some(CompanionAbilityFacet::SpecialQuality));
    }

    /// Every advancement row of this book states its `PRERACE:` owner, and the
    /// stat adjustments it applies are transcribed rather than folded into the
    /// creature's own.
    #[test]
    fn the_worg_advancement_carries_its_own_stat_adjustments() {
        let advancement = companion_abilities()
            .iter()
            .find(|a| a.key == "Companion Advancement ~ Worg")
            .expect("the Worg advancement is in this book");
        assert_eq!(advancement.facet, Some(CompanionAbilityFacet::CompanionAdvancement));
        assert_eq!(advancement.owners, &["Companion (Worg)"]);
        assert_eq!(
            advancement.stat_adjustments,
            &[
                StatAdjustment { ability: "STR", amount: 2 },
                StatAdjustment { ability: "CON", amount: 2 },
            ]
        );
    }

    /// The `DESC:`-carrying row of this book, pinned by content: a description
    /// that reached the screen empty would be indistinguishable from a row the
    /// corpus never described.
    #[test]
    fn the_worg_mastery_row_carries_its_rules_text() {
        let mastery = companion_abilities()
            .iter()
            .find(|a| a.key == "Worg ~ Mastery")
            .expect("Worg ~ Mastery is in this book");
        let description = mastery.description.expect("the row carries a DESC: token");
        assert!(
            description.contains("fearful howl"),
            "unexpected rules text: {description}"
        );
    }
}
