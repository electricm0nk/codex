//! Spell catalog browser — Tauri command adapter over every ingested PF1
//! spell table: `crb::spell_list` (652 records), `apg::spell_list` (297),
//! `acg::spell_list` (144) and `advanced_race_guide::spell_list` (92),
//! 1185 in total.
//!
//! Pathfinder Unchained is deliberately absent, and that absence is real
//! rather than an oversight: `pu_spells.lst` is 224 lines and every single
//! one is a `#`-commented-out row, so the book defines no spell of its own
//! and there is no `pathfinder_unchained::spell_list` module to chain.
//! Re-verified against the raw corpus rather than taken from the ingest
//! notes: `grep -v '^\s*#' pu_spells.lst | grep -vc '^\s*$'` returns 0.
//! Inventing a PU spell surface here would mean serving records the corpus
//! does not contain.
//!
//! This adapter previously served CRB alone. The APG and ACG tables were
//! fully ingested — with school, level and real corpus spell text — but
//! reached no user-facing surface at all: not this catalog browser, not
//! the Character Sheet's Add Spell picker (which calls `list_spells`), and
//! so not a character's own spell list either. 441 real spells, 40% of
//! everything ingested, were invisible.
//!
//! Distinct from the Character Sheet's Spells tab: this is a standalone
//! catalog view of every real spell record the engine knows about, not
//! what one character has selected. Mirrors `equipment_catalog.rs`.
//!
//! **Optional fields are absences, never placeholders.** CRB's and ACG's
//! tables carry a school, level and description on every record. APG's
//! table types those three as `Option`, and as ingested, 16 of its records
//! carry no school, 41 no level and 12 no description. Those arrive here
//! as `null` rather than an invented value, and the UI must render the
//! absence rather than a plausible-looking default.
//!
//! Those three counts are properties of `apg::spell_list` as ingested, and
//! are asserted below as such — they are deliberately NOT presented as
//! counts of gaps in `apg_spells.lst` itself, because they do not match
//! it. Re-derived from the raw file, the APG rows genuinely lacking a
//! `SCHOOL:` token number ~4-6 (plain records) or 21 (counting the 17
//! `.COPY=` delta records, which carry no fields of their own and inherit
//! from their base); neither is 16. The same mismatch holds for the level
//! and description counts. So the ingest is doing something in between —
//! partially resolving `.COPY=` inheritance, and/or folding `.MOD` rows
//! into the tally — and until that is traced, the honest statement is what
//! this adapter can actually verify: how many records it serves without
//! each field. Tracking as an open ingest-fidelity question; it does not
//! affect this adapter's correctness, which is to pass absences through
//! rather than invent values.

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::{acg, advanced_race_guide, apg, crb};

/// Which ingested book a catalog entry came from. Short codes are the wire
/// form; the frontend maps them to display labels.
const BOOK_CRB: &str = "CRB";
const BOOK_APG: &str = "APG";
const BOOK_ACG: &str = "ACG";
const BOOK_ARG: &str = "ARG";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogEntryDto {
    /// The record's corpus identity — its `KEY:` token when the row
    /// carries one, else its display name. Unique across all four books;
    /// see `tests/spell_cross_book_identity.rs`.
    pub key: String,
    /// `"CRB"`, `"APG"`, `"ACG"` or `"ARG"`.
    pub book: String,
    /// The `Pf1SchoolId` variant name verbatim (e.g. "Abjuration"), or
    /// `None` for an APG record whose corpus row has no `SCHOOL:` token.
    pub school: Option<String>,
    /// `None` for an APG record whose corpus row has no `CLASSES:` token,
    /// so no spell level can be derived without inventing one.
    pub level: Option<u8>,
    /// `None` for an APG record the corpus supplies no `DESC:` text for.
    pub description: Option<String>,
}

fn map_crb_entry(entry: &crb::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_CRB.to_string(),
        school: Some(format!("{:?}", entry.school)),
        level: Some(entry.level),
        description: Some(entry.description.to_string()),
    }
}

fn map_apg_entry(entry: &apg::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_APG.to_string(),
        school: entry.school.map(|school| format!("{school:?}")),
        level: entry.level,
        description: entry.description.map(|text| text.to_string()),
    }
}

fn map_acg_entry(entry: &acg::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ACG.to_string(),
        school: Some(format!("{:?}", entry.school)),
        level: Some(entry.level),
        description: Some(entry.description.to_string()),
    }
}

/// ARG's table types `school`, `level` and `description` non-optionally,
/// exactly as CRB's and ACG's do, so like those two this map invents
/// nothing by wrapping in `Some` — every ARG record genuinely carries all
/// three (pinned by `crb_acg_and_arg_records_are_always_fully_populated`).
fn map_arg_entry(entry: &advanced_race_guide::spell_list::SpellListEntry) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        book: BOOK_ARG.to_string(),
        school: Some(format!("{:?}", entry.school)),
        level: Some(entry.level),
        description: Some(entry.description.to_string()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogResponse {
    pub entries: Vec<SpellCatalogEntryDto>,
}

/// Build the full catalog response across every ingested book. A thin,
/// testable wrapper behind the Tauri command below (mirroring
/// `equipment_catalog`'s own command/pure-fn split).
pub fn build_spell_catalog() -> SpellCatalogResponse {
    let entries = crb::spell_list::SPELL_LIST
        .iter()
        .map(map_crb_entry)
        .chain(apg::spell_list::SPELL_LIST.iter().map(map_apg_entry))
        .chain(acg::spell_list::SPELL_LIST.iter().map(map_acg_entry))
        .chain(
            advanced_race_guide::spell_list::SPELL_LIST
                .iter()
                .map(map_arg_entry),
        )
        .collect();
    SpellCatalogResponse { entries }
}

#[tauri::command]
pub fn list_spell_catalog() -> SpellCatalogResponse {
    build_spell_catalog()
}

/// Filter criteria for `list_spells`. Every field is optional and
/// `None`/empty matches everything — an all-`None` filter is equivalent to
/// the unfiltered `list_spell_catalog` response. Kept deliberately narrow
/// (substring name match, exact school/book match) rather than an
/// exhaustive query DSL; widen only if a real caller needs more.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogFilter {
    /// Case-insensitive substring match against `key` (the spell's corpus
    /// identity/name — see `SpellCatalogEntryDto::key`'s doc comment).
    pub name_contains: Option<String>,
    /// Exact match against the `Pf1SchoolId` variant name verbatim (e.g.
    /// "Evocation"). A record whose corpus row has no `SCHOOL:` token
    /// matches no school filter — it is genuinely unknown, so it is not
    /// swept into any school.
    pub school: Option<String>,
    /// Exact match against `"CRB"`, `"APG"`, `"ACG"` or `"ARG"`.
    pub book: Option<String>,
}

/// Narrows the full catalog to the entries matching `filter`. A thin,
/// testable wrapper behind the `list_spells` Tauri command below, mirroring
/// `build_spell_catalog`'s own command/pure-fn split.
pub fn filter_spell_catalog(filter: &SpellCatalogFilter) -> SpellCatalogResponse {
    let name_needle = filter
        .name_contains
        .as_ref()
        .filter(|needle| !needle.is_empty())
        .map(|needle| needle.to_lowercase());

    let entries = build_spell_catalog()
        .entries
        .into_iter()
        .filter(|entry| match &name_needle {
            Some(needle) => entry.key.to_lowercase().contains(needle.as_str()),
            None => true,
        })
        .filter(|entry| match &filter.school {
            Some(school) => entry.school.as_deref() == Some(school.as_str()),
            None => true,
        })
        .filter(|entry| match &filter.book {
            Some(book) => &entry.book == book,
            None => true,
        })
        .collect();

    SpellCatalogResponse { entries }
}

/// Returns the spell catalog narrowed by `filter` — see
/// `SpellCatalogFilter`'s own doc comment for the supported fields.
#[tauri::command]
pub fn list_spells(filter: SpellCatalogFilter) -> SpellCatalogResponse {
    filter_spell_catalog(&filter)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn book_entries(book: &str) -> Vec<SpellCatalogEntryDto> {
        build_spell_catalog()
            .entries
            .into_iter()
            .filter(|entry| entry.book == book)
            .collect()
    }

    #[test]
    fn the_catalog_serves_every_ingested_book_not_only_crb() {
        let response = build_spell_catalog();
        assert_eq!(response.entries.len(), 1185);
        assert_eq!(book_entries(BOOK_CRB).len(), 652);
        assert_eq!(book_entries(BOOK_APG).len(), 297);
        assert_eq!(book_entries(BOOK_ACG).len(), 144);
        assert_eq!(book_entries(BOOK_ARG).len(), 92);
    }

    #[test]
    fn crb_school_counts_match_the_real_corpus() {
        let crb = book_entries(BOOK_CRB);
        let counts = |school: &str| {
            crb.iter()
                .filter(|e| e.school.as_deref() == Some(school))
                .count()
        };
        assert_eq!(counts("Abjuration"), 73);
        assert_eq!(counts("Conjuration"), 116);
        assert_eq!(counts("Divination"), 50);
        assert_eq!(counts("Enchantment"), 60);
        assert_eq!(counts("Evocation"), 87);
        assert_eq!(counts("Illusion"), 47);
        assert_eq!(counts("Necromancy"), 62);
        assert_eq!(counts("Transmutation"), 152);
        assert_eq!(counts("Universal"), 5);
    }

    #[test]
    fn every_entry_has_a_non_empty_key_and_a_known_book() {
        for entry in &build_spell_catalog().entries {
            assert!(!entry.key.is_empty());
            assert!([BOOK_CRB, BOOK_APG, BOOK_ACG, BOOK_ARG].contains(&entry.book.as_str()));
        }
    }

    #[test]
    fn no_key_is_served_twice_so_a_selection_resolves_unambiguously() {
        let entries = build_spell_catalog().entries;
        let mut keys: Vec<String> = entries.iter().map(|entry| entry.key.clone()).collect();
        keys.sort();
        let total = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), total, "the catalog serves a duplicate spell key");
    }

    #[test]
    fn crb_acg_and_arg_records_are_always_fully_populated() {
        for entry in book_entries(BOOK_CRB)
            .iter()
            .chain(book_entries(BOOK_ACG).iter())
            .chain(book_entries(BOOK_ARG).iter())
        {
            assert!(entry.school.is_some(), "{} has no school", entry.key);
            assert!(entry.level.is_some(), "{} has no level", entry.key);
            assert!(
                entry.description.as_deref().is_some_and(|d| !d.is_empty()),
                "{} has no description",
                entry.key
            );
        }
    }

    #[test]
    fn apg_records_missing_a_field_are_served_with_that_field_null() {
        // Transcribed from `apg::spell_list` as ingested — a pin on what
        // this adapter serves, NOT a claim about how many rows
        // `apg_spells.lst` omits each field on (see this module's doc
        // comment: the raw-file figures differ and the gap is untraced).
        // If the ingest changes, re-derive these; do not relax them.
        let apg = book_entries(BOOK_APG);
        assert_eq!(apg.iter().filter(|e| e.school.is_none()).count(), 16);
        assert_eq!(apg.iter().filter(|e| e.level.is_none()).count(), 41);
        assert_eq!(apg.iter().filter(|e| e.description.is_none()).count(), 12);
    }

    #[test]
    fn the_archetype_summon_records_are_served_under_their_corpus_key() {
        let entries = build_spell_catalog().entries;
        let has = |key: &str| entries.iter().any(|entry| entry.key == key);
        // The CRB spell and the two archetype variants are three distinct
        // records and all reach the catalog under distinct names.
        assert!(has("Summon Monster I"));
        assert!(has("Summoner Summon Monster I"));
        assert!(has("Summon Nature's Ally I"));
        assert!(has("Naturalist Summon Nature's Ally I"));
    }

    #[test]
    fn filter_spell_catalog_with_no_filter_fields_returns_the_full_catalog() {
        let response = filter_spell_catalog(&SpellCatalogFilter::default());
        assert_eq!(response.entries.len(), build_spell_catalog().entries.len());
    }

    #[test]
    fn filter_spell_catalog_matches_name_contains_case_insensitively() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: Some("fireball".to_owned()),
            school: None,
            book: None,
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has a Fireball record"
        );
        assert!(response.entries.len() < build_spell_catalog().entries.len());
        for entry in &response.entries {
            assert!(
                entry.key.to_lowercase().contains("fireball"),
                "entry {:?} does not contain 'fireball'",
                entry.key
            );
        }
    }

    #[test]
    fn filter_spell_catalog_matches_school_exactly_across_books() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: None,
            school: Some("Evocation".to_owned()),
            book: None,
        });

        // Now spans all four books, so strictly more than CRB's own 87.
        assert!(response.entries.len() > 87);
        for entry in &response.entries {
            assert_eq!(entry.school.as_deref(), Some("Evocation"));
        }
    }

    #[test]
    fn filter_spell_catalog_narrows_to_one_book() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: None,
            school: None,
            book: Some(BOOK_APG.to_owned()),
        });

        assert_eq!(response.entries.len(), 297);
        for entry in &response.entries {
            assert_eq!(entry.book, BOOK_APG);
        }
    }

    #[test]
    fn arg_school_counts_match_the_real_ingested_table() {
        // Derived from `advanced_race_guide::spell_list::SPELL_LIST` as
        // served by this adapter, not from any planning figure. The nine
        // school variants sum to ARG's whole 92; `Universal` is absent
        // from `arg_spells.lst` (the variant exists only for cross-book
        // schema parity), so it is pinned at 0 rather than omitted.
        let arg = book_entries(BOOK_ARG);
        let counts = |school: &str| {
            arg.iter()
                .filter(|e| e.school.as_deref() == Some(school))
                .count()
        };
        assert_eq!(counts("Abjuration"), 9);
        assert_eq!(counts("Conjuration"), 10);
        assert_eq!(counts("Divination"), 4);
        assert_eq!(counts("Enchantment"), 8);
        assert_eq!(counts("Evocation"), 8);
        assert_eq!(counts("Illusion"), 9);
        assert_eq!(counts("Necromancy"), 7);
        assert_eq!(counts("Transmutation"), 37);
        assert_eq!(counts("Universal"), 0);
        assert_eq!(
            counts("Abjuration")
                + counts("Conjuration")
                + counts("Divination")
                + counts("Enchantment")
                + counts("Evocation")
                + counts("Illusion")
                + counts("Necromancy")
                + counts("Transmutation")
                + counts("Universal"),
            arg.len(),
            "an ARG record landed outside the nine PF1 schools"
        );
    }

    #[test]
    fn filter_spell_catalog_narrows_to_arg() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: None,
            school: None,
            book: Some(BOOK_ARG.to_owned()),
        });

        assert_eq!(response.entries.len(), 92);
        for entry in &response.entries {
            assert_eq!(entry.book, BOOK_ARG);
        }
    }

    #[test]
    fn a_real_arg_spell_reaches_the_catalog_with_its_corpus_text() {
        let entries = build_spell_catalog().entries;
        let entry = entries
            .iter()
            .find(|entry| entry.key == "Aboleth's Lung")
            .expect("ARG's `Aboleth's Lung` record must reach the catalog");
        assert_eq!(entry.book, BOOK_ARG);
        assert_eq!(entry.school.as_deref(), Some("Transmutation"));
        assert_eq!(entry.level, Some(2));
        assert!(
            entry
                .description
                .as_deref()
                .is_some_and(|text| text.contains("breathe water")),
            "the ARG record must carry its real corpus text, not a placeholder"
        );
    }

    #[test]
    fn filter_spell_catalog_combines_name_and_school_filters() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: Some("flame".to_owned()),
            school: Some("Evocation".to_owned()),
            book: None,
        });

        assert!(
            !response.entries.is_empty(),
            "the real corpus has Evocation spells with 'flame' in the name (e.g. Flame \
             Blade, Flame Strike)"
        );
        for entry in &response.entries {
            assert_eq!(entry.school.as_deref(), Some("Evocation"));
            assert!(entry.key.to_lowercase().contains("flame"));
        }
    }
}
