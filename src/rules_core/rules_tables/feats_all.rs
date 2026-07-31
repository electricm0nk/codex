//! The feat catalog across every ingested rule book.
//!
//! Each book owns its own catalog module (`crb::feats`, `apg::feats`,
//! `acg::feats`, `advanced_race_guide::feats`,
//! `pathfinder_unchained::feat_tables`), exactly as each book owns its
//! own spell list. This module is the one place that joins them, tagging
//! each book's slice with the `RuleSetId` it came from so a consumer can
//! tell a player which book a feat is from without re-deriving it from
//! the key.
//!
//! Provenance lives on the *table*, not on every record, because a book
//! is a property of the whole slice -- putting a `rule_set` field on all
//! 690 records would repeat one fact 690 times and let it drift per row.
//!
//! # Why this module owns a join record instead of reusing `crb::feats::FeatTableEntry`
//!
//! CRB, APG and ACG share one type: `apg::feats` and `acg::feats` both
//! declare `use super::super::crb::feats::FeatTableEntry;`, so those
//! three books' records are literally the same struct. ARG and PU do
//! not, and widening them to reuse it was considered and rejected on
//! evidence, not on taste:
//!
//! * **ARG cannot honestly fill `prerequisites`.** `crb::feats::FeatTableEntry`
//!   documents `prerequisites: None` as "the corpus record carries no
//!   `PRE`-family token". Every single one of `arg_feats.lst`'s 187
//!   `CATEGORY:FEAT` records carries at least one (counted directly off
//!   the corpus file). ARG's ingest never gathered them, so reusing the
//!   shared type would force `None` onto 187 records for which that
//!   statement is false -- one fabricated absence per record, which is
//!   precisely what `effect`'s and `description`'s own
//!   "`Some(&[])` never occurs" rules exist to prevent.
//! * **PU's categories are not `TYPE:`-facet categories at all.**
//!   `crb::feats::FeatCategory`'s six variants are documented as derived
//!   from the corpus `TYPE:` facet. `pu_feats.lst` has no usable one: 9
//!   of its 17 real records (the "Champion of ..." alignment feats)
//!   carry no `TYPE:` token whatsoever, so `pathfinder_unchained` derives
//!   its categories from the file's own `###Block:` markers instead
//!   (`Alignment`, `CombatStamina`, `WoundThreshold`, plus `General`).
//!   None of `Alignment`/`CombatStamina`/`WoundThreshold` exists in the
//!   shared enum, and mapping them onto a variant that does exist would
//!   invent a classification the corpus never made.
//! * **PU would also have to fabricate `effect` and `prerequisites`.**
//!   Its ingest carries neither, yet its records genuinely have both in
//!   the corpus (e.g. `Combat Stamina` carries
//!   `BONUS:VAR|StaminaPool|BAB+CON` and `PRETOTALAB:1`), and it carries
//!   a `source_page` the shared type has no field for.
//!
//! So this module follows the precedent `spell_catalog.rs` already set
//! for exactly this situation -- differing per-book record types joined
//! by per-book map functions into one shared record carrying a book tag
//! (`map_crb_entry` / `map_apg_entry` / `map_acg_entry` there). The join
//! record here is [`FeatCatalogRecord`].
//!
//! # What the join record carries, and what it deliberately does not
//!
//! [`FeatCatalogRecord`] carries `key`, `category`, `name` and
//! `description` -- the four facets *every* ingested book's feat table
//! actually holds, and the exact four every consumer of
//! [`all_feat_tables`] reads today (the desktop Feat picker's DTO, the
//! zero-magnitude description check, the feat-identity collision proof,
//! and the two inventory binaries).
//!
//! It does **not** carry `effect`, `prerequisites` or `source_page`.
//! Those are per-book facets only some books ingested, and hoisting them
//! here would have to represent "this book's ingest does not carry this
//! field" and "this corpus record has no such token" with the same
//! `None` -- two different facts collapsed into one value, which is the
//! same ambiguity the per-book types each refuse. A consumer that needs
//! CRB's `prerequisites`, ARG's `effect` or PU's `source_page` reads that
//! book's own table directly, exactly as `feat_prereqs::*` already reads
//! `crb::feats` directly.
//!
//! `category` is the book's own `FeatCategory` variant name verbatim, as
//! a `&'static str`. The three enums are genuinely different closed sets
//! over different corpus signals, so there is no single enum to project
//! onto; and the one consumer that renders a category
//! (`feat_catalog.rs`) already stringified it with `format!("{:?}", ..)`
//! at its DTO boundary, so nothing is lost by doing it here instead.
//! Each `*_category_name` function below is an exhaustive `match`
//! returning a literal -- a new variant in any book is a compile error,
//! not a silently mislabelled row -- and
//! `category_names_match_the_debug_form_of_every_variant` pins each
//! literal to that variant's `Debug` form so the wire strings the
//! frontend filters on cannot drift.
//!
//! # Key collisions
//!
//! Feat keys were globally unique across CRB/APG/ACG. **They are not
//! unique once PU is included:** `Endurance` is in both `cr_feats.lst`
//! and `pu_feats.lst`. Checked against both corpus rows rather than
//! assumed: PU's is the Core Rulebook feat *re-listed* under Pathfinder
//! Unchained's Wound Threshold rules, not a second feat that happens to
//! share a name. The two rows carry the same name and the same `DESC:`
//! text; they differ in `TYPE:` (`General` vs `Wound Threshold`),
//! `SOURCEPAGE:` (`p.112` vs `P.137`) and `BENEFIT:` (PU's adds the
//! wound-penalty reduction) -- and `BENEFIT:` is a token neither book's
//! table ingests.
//!
//! Both rows are kept, in the same not-deduplicated posture `crb::feats`
//! takes with its two real `Combat Expertise` records: dropping PU's
//! would make this table disagree with `pu_feats::feat_tables()` about
//! how many records that book has. The collision set is pinned by
//! `cross_book_key_collisions_are_exactly_the_known_set` below so a
//! genuinely different second feat arriving under an existing key is a
//! test failure rather than a silent shadowing.

use super::advanced_race_guide::feats as arg_feats;
use super::crb::feats::{FeatCategory as SharedFeatCategory, FeatTableEntry as SharedFeatTableEntry};
use super::pathfinder_unchained::feat_tables as pu_feats;
use super::RuleSetId;

/// One feat record, projected out of whichever per-book table it came
/// from. See this module's own doc comment for why the four fields are
/// these four and why `category` is a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatCatalogRecord {
    /// The record's corpus identity -- its `KEY:` token when its row
    /// carries one, else its display name. Each book's own table already
    /// applies that fallback; this is that value, unchanged.
    ///
    /// Not unique across books -- see this module's "Key collisions"
    /// section.
    pub key: &'static str,
    /// The source book's own `FeatCategory` variant name, verbatim (e.g.
    /// `"Combat"`, `"Panache"`, `"WoundThreshold"`).
    pub category: &'static str,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim; `None` when the record has
    /// none. Passed through from the book's own table, which already
    /// applies that rule -- never substituted or borrowed from a sibling
    /// record.
    pub description: Option<&'static str>,
}

/// One book's feat catalog, tagged with the book it came from.
#[derive(Debug, Clone, Copy)]
pub struct BookFeatTable {
    pub rule_set: RuleSetId,
    pub entries: &'static [FeatCatalogRecord],
}

// ---------------------------------------------------------------------------
// Per-book category projections
// ---------------------------------------------------------------------------

/// The shared CRB/APG/ACG category enum's variant names.
fn shared_category_name(category: SharedFeatCategory) -> &'static str {
    match category {
        SharedFeatCategory::General => "General",
        SharedFeatCategory::Combat => "Combat",
        SharedFeatCategory::ItemCreation => "ItemCreation",
        SharedFeatCategory::Metamagic => "Metamagic",
        SharedFeatCategory::Teamwork => "Teamwork",
        SharedFeatCategory::Panache => "Panache",
    }
}

/// ARG's own three-variant enum. Its names coincide with three of the
/// shared enum's, so ARG's records merge into the picker's existing
/// General/Combat/Teamwork filters rather than opening new ones -- that
/// is a real property of the two corpora, not a mapping decision made
/// here.
fn arg_category_name(category: arg_feats::FeatCategory) -> &'static str {
    match category {
        arg_feats::FeatCategory::General => "General",
        arg_feats::FeatCategory::Combat => "Combat",
        arg_feats::FeatCategory::Teamwork => "Teamwork",
    }
}

/// PU's own `###Block:`-derived enum. Three of its four names are new to
/// the catalog; that is what this book's corpus says, and inventing a
/// projection onto an existing category would be a classification the
/// corpus never made.
fn pu_category_name(category: pu_feats::FeatCategory) -> &'static str {
    match category {
        pu_feats::FeatCategory::Alignment => "Alignment",
        pu_feats::FeatCategory::CombatStamina => "CombatStamina",
        pu_feats::FeatCategory::WoundThreshold => "WoundThreshold",
        pu_feats::FeatCategory::General => "General",
    }
}

// ---------------------------------------------------------------------------
// Per-book map functions
// ---------------------------------------------------------------------------

/// CRB, APG and ACG all hold `crb::feats::FeatTableEntry`, so one map
/// function serves all three.
fn map_shared_entry(entry: &SharedFeatTableEntry) -> FeatCatalogRecord {
    FeatCatalogRecord {
        key: entry.key,
        category: shared_category_name(entry.category),
        name: entry.name,
        description: entry.description,
    }
}

fn map_arg_entry(entry: &arg_feats::FeatTableEntry) -> FeatCatalogRecord {
    FeatCatalogRecord {
        key: entry.key,
        category: arg_category_name(entry.category),
        name: entry.name,
        description: entry.description,
    }
}

fn map_pu_entry(entry: &pu_feats::FeatTableEntry) -> FeatCatalogRecord {
    FeatCatalogRecord {
        key: entry.key,
        category: pu_category_name(entry.category),
        name: entry.name,
        description: entry.description,
    }
}

/// Project one book's table once and hand out a `'static` slice of it.
///
/// The projection is a real allocation, so it is cached per book for the
/// process lifetime exactly as each book's own `feat_tables()` caches its
/// table -- `all_feat_tables()` is called per request by the desktop
/// catalog command and must not re-map 690 records each time.
fn projected(
    cell: &'static std::sync::OnceLock<Vec<FeatCatalogRecord>>,
    build: impl FnOnce() -> Vec<FeatCatalogRecord>,
) -> &'static [FeatCatalogRecord] {
    cell.get_or_init(build).as_slice()
}

fn crb_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || {
        super::crb::feats::feat_tables().iter().map(map_shared_entry).collect()
    })
}

fn apg_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || {
        super::apg::feats::feat_tables().iter().map(map_shared_entry).collect()
    })
}

fn acg_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || {
        super::acg::feats::feat_tables().iter().map(map_shared_entry).collect()
    })
}

fn arg_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || arg_feats::feat_tables().iter().map(map_arg_entry).collect())
}

fn pu_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || pu_feats::feat_tables().iter().map(map_pu_entry).collect())
}

/// Every ingested book's feat catalog, in book order (CRB, APG, ACG,
/// ARG, PU).
///
/// 690 records total: 185 CRB + 172 APG + 129 ACG + 187 ARG + 17 PU.
/// Built once and cached for the process lifetime, over the five
/// per-book `feat_tables()` functions -- this never re-derives or
/// re-filters their contents, only projects each record onto
/// [`FeatCatalogRecord`].
pub fn all_feat_tables() -> &'static [BookFeatTable] {
    static TABLES: std::sync::OnceLock<Vec<BookFeatTable>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        vec![
            BookFeatTable { rule_set: RuleSetId::Crb, entries: crb_records() },
            BookFeatTable { rule_set: RuleSetId::Apg, entries: apg_records() },
            BookFeatTable { rule_set: RuleSetId::Acg, entries: acg_records() },
            BookFeatTable { rule_set: RuleSetId::Arg, entries: arg_records() },
            BookFeatTable { rule_set: RuleSetId::Pu, entries: pu_records() },
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn spans_every_ingested_book_with_their_real_counts() {
        let books = all_feat_tables();
        assert_eq!(books.len(), 5);
        assert_eq!(books[0].rule_set, RuleSetId::Crb);
        assert_eq!(books[0].entries.len(), 185);
        assert_eq!(books[1].rule_set, RuleSetId::Apg);
        assert_eq!(books[1].entries.len(), 172);
        assert_eq!(books[2].rule_set, RuleSetId::Acg);
        assert_eq!(books[2].entries.len(), 129);
        assert_eq!(books[3].rule_set, RuleSetId::Arg);
        assert_eq!(books[3].entries.len(), 187);
        assert_eq!(books[4].rule_set, RuleSetId::Pu);
        assert_eq!(books[4].entries.len(), 17);

        let total: usize = books.iter().map(|book| book.entries.len()).sum();
        assert_eq!(total, 690, "185 CRB + 172 APG + 129 ACG + 187 ARG + 17 PU");
    }

    /// The projection must not lose or invent a record: each book's slice
    /// is exactly as long as the book's own table, checked against the
    /// per-book functions rather than against the numbers above.
    #[test]
    fn each_books_slice_is_exactly_its_own_table() {
        let books = all_feat_tables();
        assert_eq!(books[0].entries.len(), super::super::crb::feats::feat_tables().len());
        assert_eq!(books[1].entries.len(), super::super::apg::feats::feat_tables().len());
        assert_eq!(books[2].entries.len(), super::super::acg::feats::feat_tables().len());
        assert_eq!(books[3].entries.len(), arg_feats::feat_tables().len());
        assert_eq!(books[4].entries.len(), pu_feats::feat_tables().len());
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
                assert!(
                    !entry.category.is_empty(),
                    "{:?} entry '{}' has an empty category",
                    book.rule_set,
                    entry.key
                );
            }
        }
    }

    /// The category strings are the wire form the desktop picker filters
    /// on, so each literal must be its variant's `Debug` form. Written
    /// over each enum's own `ALL` roster, so a variant added without a
    /// `match` arm fails to compile and a variant added *with* a
    /// mismatched literal fails here.
    #[test]
    fn category_names_match_the_debug_form_of_every_variant() {
        for category in SharedFeatCategory::ALL {
            assert_eq!(shared_category_name(*category), format!("{category:?}"));
        }
        for category in arg_feats::FeatCategory::ALL {
            assert_eq!(arg_category_name(*category), format!("{category:?}"));
        }
        for category in pu_feats::FeatCategory::ALL {
            assert_eq!(pu_category_name(*category), format!("{category:?}"));
        }
    }

    /// The real per-book category breakdown, derived from the live
    /// tables. Pins ARG's and PU's own corpus-documented splits (ARG:
    /// 132 General / 52 Combat / 3 Teamwork, of which one "General" is
    /// the corpus's own `TYPE:Genaral` typo, classified rather than
    /// dropped; PU: 9 Alignment / 3 CombatStamina / 3 WoundThreshold /
    /// 2 General) at the join boundary, so a regression in either book's
    /// table surfaces here and not only in that book's own module.
    #[test]
    fn the_per_book_category_split_is_the_real_one() {
        let split = |rule_set: RuleSetId| -> BTreeMap<&'static str, usize> {
            let mut counts = BTreeMap::new();
            for book in all_feat_tables().iter().filter(|book| book.rule_set == rule_set) {
                for entry in book.entries {
                    *counts.entry(entry.category).or_insert(0) += 1;
                }
            }
            counts
        };

        assert_eq!(
            split(RuleSetId::Arg),
            BTreeMap::from([("Combat", 52), ("General", 132), ("Teamwork", 3)])
        );
        assert_eq!(
            split(RuleSetId::Pu),
            BTreeMap::from([
                ("Alignment", 9),
                ("CombatStamina", 3),
                ("General", 2),
                ("WoundThreshold", 3),
            ])
        );
    }

    /// The point of widening the aggregate: real ARG and PU feats are in
    /// it, with their real corpus description text.
    #[test]
    fn real_arg_and_pu_records_are_in_the_aggregate_with_their_descriptions() {
        let find = |key: &str| {
            all_feat_tables()
                .iter()
                .flat_map(|book| book.entries.iter().map(move |entry| (book.rule_set, entry)))
                .find(|(_, entry)| entry.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be in the aggregate catalog"))
        };

        let (book, wings) = find("Angel Wings");
        assert_eq!(book, RuleSetId::Arg);
        assert_eq!(wings.category, "General");
        assert_eq!(wings.description, Some("Feathered wings sprout from your back."));

        let (book, champion) = find("Champion of Tyranny");
        assert_eq!(book, RuleSetId::Pu);
        assert_eq!(champion.category, "Alignment");
        assert_eq!(
            champion.description,
            Some("You must beat down the masses to have true order.")
        );

        let (book, stamina) = find("Combat Stamina");
        assert_eq!(book, RuleSetId::Pu);
        assert_eq!(stamina.category, "CombatStamina");
    }

    /// Feat keys were globally unique across CRB/APG/ACG and are not
    /// once PU is in. `Endurance` is the only one, and it is a re-listing
    /// rather than two different feats -- see this module's own "Key
    /// collisions" section for the corpus evidence.
    ///
    /// What that costs today: a consumer that flattens the catalog and
    /// looks up by key resolves CRB's row, because CRB is first in book
    /// order. For this collision that is harmless -- the two rows'
    /// ingested fields other than `category` are identical, so
    /// `description_completion` returns the same text either way -- and
    /// the desktop picker shows two rows whose `source` and `category`
    /// tell them apart. The assertion is exact so that a *different*
    /// second feat arriving under an existing key fails here instead of
    /// silently shadowing one book's record with another's.
    #[test]
    fn cross_book_key_collisions_are_exactly_the_known_set() {
        let mut seen: BTreeMap<&'static str, RuleSetId> = BTreeMap::new();
        let mut collisions: Vec<(&'static str, RuleSetId, RuleSetId)> = Vec::new();
        for book in all_feat_tables() {
            for entry in book.entries {
                match seen.insert(entry.key, book.rule_set) {
                    Some(previous) if previous != book.rule_set => {
                        collisions.push((entry.key, previous, book.rule_set));
                    }
                    _ => {}
                }
            }
        }

        assert_eq!(collisions, vec![("Endurance", RuleSetId::Crb, RuleSetId::Pu)]);

        // ... and it really is the same feat re-listed, not a name clash
        // between two different ones: both rows carry the corpus's own
        // identical `DESC:` text, and only the block-derived category
        // differs.
        let rows: Vec<&FeatCatalogRecord> = all_feat_tables()
            .iter()
            .flat_map(|book| book.entries.iter())
            .filter(|entry| entry.key == "Endurance")
            .collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].description, rows[1].description);
        assert_eq!(rows[0].category, "General");
        assert_eq!(rows[1].category, "WoundThreshold");
    }
}
