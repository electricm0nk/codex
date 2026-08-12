//! Bestiary 2 (`SOURCESHORT:B2`) — `companion`.
//!
//! # The lane's first FAMILIAR book, and the first book in it that another lane
//! also wants
//!
//! Every companion book registered before this one contributes
//! `*_races_companion.lst` rows. B2's 16 companion units are
//! `*_races_familiar.lst` and `*_abilities_familiar_race.lst` — the same kind by
//! `v06_work_inventory::file_kind`, and the same two structural shapes, but the
//! creature rows are `TYPE:Companion.Familiar.Animal` wizard/witch familiars
//! rather than druid animal companions.
//!
//! ```text
//! python3 scripts/classify_companion_rows.py bestiary_2
//! book                              crea  abil  clas  named  prerace  prefix  ORPHAN
//! bestiary_2                          15     1     0      1        0       1       0
//! ```
//!
//! **`RuleSetId::B2` compiles this book's `companion` family and nothing else.**
//! B2 also holds 782 `monster` / `monster_ability` units, which are the
//! monster lane's (`decisions.md §46`, round-3 target list). Registering the
//! rule set moves those units from `not-started` to `not-ingested` — a status
//! relabel that states the truth more precisely, since the engine now compiles
//! part of this book — and does not claim them.
//!
//! # One ability row, and both ownership shapes disagree about how it is owned
//!
//! `Snapping Turtle ~ Shell` is claimed by `prefix` (its namespaced `KEY:`
//! resolves through the `Familiar (Snapping Turtle)` wrapper) and by `named`
//! (the creature row's `ABILITY:Special Ability|AUTOMATIC|` names it) but NOT by
//! `prerace` — the row carries no `PRERACE:` at all. It is recorded once; the
//! chassis dedupes on the key.

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

    /// From `docs/work-inventory.json`'s own units for this book: 16 companion
    /// units, 15 creature rows and 1 ability row.
    #[test]
    fn the_book_defines_fifteen_familiars_and_one_ability() {
        assert_eq!(companions().len(), 15);
        assert_eq!(companion_abilities().len(), 1);
    }

    /// Every creature row in this book is a FAMILIAR, not an animal companion —
    /// the property that makes this the first book of its shape in the lane, and
    /// the one a `*_races_companion.lst`-shaped reader would have quietly got
    /// wrong.
    #[test]
    fn every_creature_row_is_a_familiar() {
        for companion in companions() {
            assert!(
                companion.key.starts_with("Familiar ("),
                "{} is not a familiar",
                companion.key
            );
            assert!(
                companion.type_segments.contains(&"Familiar"),
                "{} does not state Familiar in its TYPE:",
                companion.key
            );
        }
    }

    /// Verbatim spot-check against `b2_abilities_familiar_race.lst:6` and the
    /// creature row that names it. The link closes in both directions on the
    /// book's only ability.
    #[test]
    fn the_snapping_turtles_shell_matches_its_corpus_row_and_its_owner() {
        let shell = &companion_abilities()[0];
        assert_eq!(shell.key, "Snapping Turtle ~ Shell");
        assert_eq!(shell.name, "Shell");
        assert_eq!(shell.facet, Some(CompanionAbilityFacet::SpecialQuality));
        assert_eq!(shell.delivery, Some(CompanionAbilityDelivery::Extraordinary));
        assert_eq!(shell.source_page, Some("p.273"));
        assert_eq!(shell.owners, &["Familiar (Snapping Turtle)"]);

        let turtle = companions()
            .iter()
            .find(|c| c.key == "Familiar (Snapping Turtle)")
            .expect("the Snapping Turtle is in this book");
        assert!(turtle.ability_keys.contains(&"Snapping Turtle ~ Shell"));
    }

    /// The other 14 familiars carry no ability of this book, and their
    /// `external_ability_refs` say so rather than the table silently holding an
    /// empty list on both sides.
    #[test]
    fn the_other_fourteen_familiars_name_only_abilities_this_book_does_not_define() {
        let with_ability: Vec<_> = companions()
            .iter()
            .filter(|c| !c.ability_keys.is_empty())
            .map(|c| c.key)
            .collect();
        assert_eq!(with_ability, vec!["Familiar (Snapping Turtle)"]);
    }
}
