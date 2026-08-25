//! Bestiary 6 (B6) shared spell list.
//!
//! Transcribed directly from the pinned oracle's `b6_spells.lst` (SD-31 wave
//! 24, `bestiary_6` book-auditor lane). Record coverage: both of the book's
//! two base spell declarations -- `Animal Growth (Reptiles Only)` and
//! `Animal Shapes (Reptiles Only)`, the reptile-only Scalykind subdomain
//! spells granted at domain levels 5 and 8 respectively
//! (`DOMAINS:Scalykind=5` / `DOMAINS:Scalykind=8`). Neither carries a
//! `CLASSES:` token; each carries exactly one `DOMAINS:` token, so `level` is
//! that token's own number, never fabricated.
//!
//! Both rows are also reprinted verbatim (same `DESC:`, same
//! `SOURCEPAGE:p.240`/`p.241` citing THIS book's own pages) inside Ultimate
//! Wilderness's `uw_spells.lst`, already registered as
//! `rules_tables::ultimate_wilderness::spell_list::SPELL_LIST` -- Ultimate
//! Wilderness's own Scalykind-subdomain content needs the spell text on hand
//! without requiring Bestiary 6 to be loaded too. That is a second book
//! legitimately citing the same real-world spell, not a corpus error: the
//! `(book, key)` scoping discipline `v06_work_inventory::classify`'s
//! `Kind::Spell` arm already applies (documented there as the "Celestial
//! Shield" discipline) means this book's own units ground only through this
//! book's own table, exactly as UW's neither borrow from nor conflict with
//! this one. `SpellCatalogEntryDto`'s `book` field carries the two rows'
//! separate provenance through to the desktop Spell Catalog, same as any
//! other cross-book reprint (`spell_resolver`'s own `Share Language
//! (Communal)` precedent, which is deduped only when a later ingest is
//! genuinely a *thinner* duplicate of the same book/edition -- not this
//! book's case, and this file does not attempt that call for Decision 10's
//! Supersession Register, which is proposed and not yet applied).
//!
//! `school`/`description` are `Option` only to match every sibling
//! `spell_list.rs`'s own shape (`rules_tables::ultimate_wilderness::
//! spell_list`'s doc comment); both are populated for both of this book's
//! records.

/// The full 9-school PF1 spell-school enum, mirroring every other book's own
/// copy exactly (`rules_tables::advanced_race_guide::spell_list::Pf1SchoolId`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pf1SchoolId {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    Universal,
}

impl Pf1SchoolId {
    pub fn from_corpus_str(raw: &str) -> Option<Self> {
        match raw {
            "Abjuration" => Some(Pf1SchoolId::Abjuration),
            "Conjuration" => Some(Pf1SchoolId::Conjuration),
            "Divination" => Some(Pf1SchoolId::Divination),
            "Enchantment" => Some(Pf1SchoolId::Enchantment),
            "Evocation" => Some(Pf1SchoolId::Evocation),
            "Illusion" => Some(Pf1SchoolId::Illusion),
            "Necromancy" => Some(Pf1SchoolId::Necromancy),
            "Transmutation" => Some(Pf1SchoolId::Transmutation),
            "Universal" => Some(Pf1SchoolId::Universal),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellListEntry {
    pub key: &'static str,
    pub name_pi_line: Option<u32>,
    pub school: Option<Pf1SchoolId>,
    pub level: Option<u8>,
    pub description: Option<&'static str>,
}

pub const SPELL_LIST: &[SpellListEntry] = &[
    SpellListEntry {
        key: "Animal Growth (Reptiles Only)",
        name_pi_line: None,
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(5),
        description: Some(
            "The target reptile grows to twice its normal size and eight times its normal \
             weight.",
        ),
    },
    SpellListEntry {
        key: "Animal Shapes (Reptiles Only)",
        name_pi_line: None,
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(8),
        description: Some(
            "You change the form of up to (CASTERLEVEL) willing reptiles into an animal of \
             your choice.",
        ),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// From `docs/work-inventory.json`'s own units for this book: exactly 2
    /// spell units, `bestiary_6:spell:animal_growth_reptiles_only` and
    /// `bestiary_6:spell:animal_shapes_reptiles_only`.
    #[test]
    fn the_book_defines_exactly_two_spells() {
        assert_eq!(SPELL_LIST.len(), 2);
    }

    /// Verbatim spot-check against `b6_spells.lst`: neither row carries a
    /// `CLASSES:` token, so `level` must come from the record's own
    /// `DOMAINS:Scalykind=N` token, never a guessed or copied value.
    #[test]
    fn animal_growth_matches_its_corpus_domains_level() {
        let entry = SPELL_LIST
            .iter()
            .find(|e| e.key == "Animal Growth (Reptiles Only)")
            .expect("Animal Growth (Reptiles Only) is in this book");
        assert_eq!(entry.level, Some(5));
        assert_eq!(entry.school, Some(Pf1SchoolId::Transmutation));
        assert_eq!(
            entry.description,
            Some(
                "The target reptile grows to twice its normal size and eight times its normal \
                 weight."
            )
        );
    }

    #[test]
    fn animal_shapes_matches_its_corpus_domains_level() {
        let entry = SPELL_LIST
            .iter()
            .find(|e| e.key == "Animal Shapes (Reptiles Only)")
            .expect("Animal Shapes (Reptiles Only) is in this book");
        assert_eq!(entry.level, Some(8));
    }
}
