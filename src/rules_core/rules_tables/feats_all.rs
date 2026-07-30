//! The feat catalog across every ingested rule book.
//!
//! Each book owns its own catalog module (`crb::feats`, `apg::feats`,
//! `acg::feats`), exactly as each book owns its own spell list. This
//! module is the one place that joins them, tagging each book's slice
//! with the `RuleSetId` it came from so a consumer can tell a player
//! which book a feat is from without re-deriving it from the key.
//!
//! Provenance lives on the *table*, not on every `FeatTableEntry`,
//! because a book is a property of the whole slice -- putting a
//! `rule_set` field on all 486 records would repeat one fact 486 times
//! and let it drift per row.
//!
//! Feat keys are globally unique across the three books today (no CRB /
//! APG / ACG key or display name collides), so a consumer that flattens
//! this into one list loses nothing -- but that is a fact about the
//! current corpus, pinned by
//! `tests/v06_apg_acg_feat_catalog.rs::feat_keys_never_collide_across_books`,
//! not a guarantee. The one within-book duplicate key is CRB's two real
//! `Combat Expertise` corpus records, preserved verbatim rather than
//! deduplicated.

use super::crb::feats::FeatTableEntry;
use super::RuleSetId;

/// One book's feat catalog, tagged with the book it came from.
#[derive(Debug, Clone, Copy)]
pub struct BookFeatTable {
    pub rule_set: RuleSetId,
    pub entries: &'static [FeatTableEntry],
}

/// Every ingested book's feat catalog, in book order (CRB, APG, ACG).
///
/// 486 records total: 185 CRB + 172 APG + 129 ACG. Built once and cached
/// for the process lifetime, over the three per-book `feat_tables()`
/// functions -- this never re-derives or re-filters their contents.
pub fn all_feat_tables() -> &'static [BookFeatTable] {
    static TABLES: std::sync::OnceLock<Vec<BookFeatTable>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        vec![
            BookFeatTable {
                rule_set: RuleSetId::Crb,
                entries: super::crb::feats::feat_tables(),
            },
            BookFeatTable {
                rule_set: RuleSetId::Apg,
                entries: super::apg::feats::feat_tables(),
            },
            BookFeatTable {
                rule_set: RuleSetId::Acg,
                entries: super::acg::feats::feat_tables(),
            },
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spans_the_three_ingested_books_with_their_real_counts() {
        let books = all_feat_tables();
        assert_eq!(books.len(), 3);
        assert_eq!(books[0].rule_set, RuleSetId::Crb);
        assert_eq!(books[0].entries.len(), 185);
        assert_eq!(books[1].rule_set, RuleSetId::Apg);
        assert_eq!(books[1].entries.len(), 172);
        assert_eq!(books[2].rule_set, RuleSetId::Acg);
        assert_eq!(books[2].entries.len(), 129);
    }

    #[test]
    fn every_record_carries_a_real_key_and_name() {
        for book in all_feat_tables() {
            for entry in book.entries {
                assert!(!entry.key.is_empty(), "{:?} entry with empty key", book.rule_set);
                assert!(
                    !entry.name.is_empty(),
                    "{:?} entry '{}' has an empty name",
                    book.rule_set,
                    entry.key
                );
            }
        }
    }
}
