//! SD-19 spell-id resolver.
//!
//! Resolves a `CharacterInput.spells_selected[].spell_id` to its real
//! PCGen corpus record and (when available) the foundation slice's
//! canonical Paizo-table-cell reference.
//!
//! Note on identity: unlike equipment records, spell records in
//! `cr_spells.lst` carry no `KEY:` token — a spell's identity is its
//! `name` field (confirmed against the real corpus; see
//! `rules_tables::crb::spell_list`'s doc comment). So "spell_id" here
//! means the spell's corpus `name`, matched exactly; no normalization is
//! needed since PF1 spell names are unique across the strict-school
//! partition.

use crate::pcgen_import::lst_parser::spell::LstSpellRecord;
use crate::pcgen_import::source_content_payload::SourceContentPayload;
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::{
    acg, advanced_race_guide, apg, crb, ultimate_intrigue, ultimate_magic,
};
use crate::rules_core::source_content::{SourceContentKind, SourcePackageContent};

/// Wire-form book codes for [`spell_catalog_rows`]. These are the same
/// short codes the desktop Spell Catalog already put on the wire, kept
/// verbatim so this consolidation changes no payload the frontend reads.
pub const SPELL_BOOK_CRB: &str = "CRB";
pub const SPELL_BOOK_APG: &str = "APG";
pub const SPELL_BOOK_ACG: &str = "ACG";
pub const SPELL_BOOK_ARG: &str = "ARG";
pub const SPELL_BOOK_UI: &str = "UI";
/// SD31-E6-F2-002: Ultimate Magic, the sixth book -- the first widening of
/// this catalog beyond the five books `spell_resolver.rs`'s own module doc
/// comment named as the reachable set before this cycle.
pub const SPELL_BOOK_UM: &str = "UM";

/// One ingested spell record, normalized across every book's own
/// `spell_list` table.
///
/// **Why this type exists.** Each ingested book declares its *own*
/// `SpellListEntry` and its *own* `Pf1SchoolId` enum (`crb`, `apg`, `acg`,
/// `advanced_race_guide`, `ultimate_intrigue` and `ultimate_magic` each
/// define both), so there is no single Rust type spanning them and every
/// consumer that wanted "all ingested spells" had to chain them by hand. Two
/// consumers did exactly that and drifted apart: the desktop
/// `spell_catalog::build_spell_catalog` chained **five** books, while
/// `v06_work_inventory::gather_engine_facts` inserted **three**
/// (`core_rulebook`, `advanced_players_guide`, `advanced_class_guide`) —
/// so every ARG and UI spell already shipping in the catalog was reported
/// `not-ingested` by the work inventory. That is precisely the
/// SD-28-E15 defect `equipment_resolver::equipment_catalog_rows` was built
/// to close for equipment, reproduced on the spell family; this type
/// closes it the same way, by leaving no second list to diverge.
///
/// **SD31-E6-F2-002 widened the chain to six books**, adding Ultimate Magic
/// -- the structural finding wave 3's spell lane made (the catalog chains
/// only the books this doc comment names, so every OTHER spell-bearing
/// book's units are structurally `not-ingested` no matter how much ingest
/// work runs against them) closed for one real book rather than only
/// analyzed. See `src/bin/ingest_ultimate_magic_spells.rs` for the ingest
/// path and `docs/release/SD-31-corpus-closure-grind/progress.md`'s
/// `SD31-E6-F2-002` receipt for the remaining books this cycle did not
/// reach.
#[derive(Debug, Clone, PartialEq)]
pub struct SpellCatalogRow {
    /// One of the `SPELL_BOOK_*` codes above.
    pub book: &'static str,
    /// The record's corpus identity — its `KEY:` token when the row
    /// carries one, else its display name.
    pub key: &'static str,
    /// The book table's `Pf1SchoolId` variant name verbatim (e.g.
    /// `"Abjuration"`). `None` only where the book's own table types the
    /// field optionally *and* the corpus row carries no `SCHOOL:` token
    /// (APG, UM). Never fabricated.
    pub school: Option<String>,
    /// Minimum spell level across the corpus record's `CLASSES:`/`DOMAINS:`
    /// tag(s). `None` only where the book's own table types it optionally
    /// *and* the corpus row carries neither token (APG, UM).
    pub level: Option<u8>,
    /// The record's `DESC:` text exactly as the book's table stores it —
    /// still carrying PCGen `%N`/`|` syntax. Rendering for a player is the
    /// caller's job (the desktop catalog runs `render_pcgen_desc`); this
    /// registry deliberately does not pre-render, so a non-player consumer
    /// (the work inventory) sees the corpus text unaltered.
    pub description: Option<&'static str>,
}

/// Every ingested book's spell rows, in the same book order the desktop
/// catalog adapter chains them (CRB, APG, ACG, ARG, UI) and, within a book,
/// in that book's own table order.
///
/// Adding a sixth book here widens the desktop catalog **and** the work
/// inventory's `spell_levels` map in the same edit — there is no second
/// place to remember.
pub fn spell_catalog_rows() -> &'static [SpellCatalogRow] {
    static ROWS: std::sync::OnceLock<Vec<SpellCatalogRow>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| {
        let crb_rows = crb::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_CRB,
            key: entry.key,
            school: Some(format!("{:?}", entry.school)),
            level: Some(entry.level),
            description: Some(entry.description),
        });
        let apg_rows = apg::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_APG,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        let acg_rows = acg::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_ACG,
            key: entry.key,
            school: Some(format!("{:?}", entry.school)),
            level: Some(entry.level),
            description: Some(entry.description),
        });
        let arg_rows =
            advanced_race_guide::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_ARG,
                key: entry.key,
                school: Some(format!("{:?}", entry.school)),
                level: Some(entry.level),
                description: Some(entry.description),
            });
        let ui_rows =
            ultimate_intrigue::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
                book: SPELL_BOOK_UI,
                key: entry.key,
                school: Some(format!("{:?}", entry.school)),
                level: Some(entry.level),
                description: Some(entry.description),
            });
        let um_rows = ultimate_magic::spell_list::SPELL_LIST.iter().map(|entry| SpellCatalogRow {
            book: SPELL_BOOK_UM,
            key: entry.key,
            school: entry.school.map(|school| format!("{school:?}")),
            level: entry.level,
            description: entry.description,
        });
        crb_rows
            .chain(apg_rows)
            .chain(acg_rows)
            .chain(arg_rows)
            .chain(ui_rows)
            .chain(um_rows)
            .collect()
    })
}

pub fn spell_id_resolve<'a>(
    spell_id: &str,
    rule_set: RuleSetId,
    corpus: &SourcePackageContent<'a>,
) -> Option<(&'a LstSpellRecord, Option<TableCellRef>)> {
    for record in corpus.records_by_kind(SourceContentKind::Spell) {
        if let SourceContentPayload::Spell(spell) = record.payload
            && spell.name == spell_id
        {
            let table_cell = SPELL_LIST
                .iter()
                .find(|entry| entry.key == spell_id)
                .map(|_| TableCellRef {
                    rule_set,
                    table: "spell_list".to_string(),
                    row_key: spell_id.to_string(),
                    column_key: String::new(),
                });
            return Some((spell, table_cell));
        }
    }
    None
}

#[cfg(test)]
mod spell_catalog_rows_tests {
    use super::*;

    /// SD31-E6-F2-002: Ultimate Magic is the sixth book chained into
    /// `spell_catalog_rows()`, the widening this cycle's own dispatch names
    /// as its primary deliverable. A real, non-empty row set proves the
    /// chain actually wired the new book in, not merely that the constant
    /// compiles.
    #[test]
    fn ultimate_magic_is_chained_into_the_catalog() {
        let um_rows: Vec<&SpellCatalogRow> =
            spell_catalog_rows().iter().filter(|row| row.book == SPELL_BOOK_UM).collect();
        assert!(!um_rows.is_empty(), "expected at least one Ultimate Magic spell row");
        assert!(
            um_rows.iter().any(|row| row.key == "Acidic Spray"),
            "expected the real corpus record 'Acidic Spray' among Ultimate Magic's rows"
        );
    }

    /// A record this cycle's own ingest confirmed carries neither `CLASSES:`
    /// nor `DOMAINS:` (`Restore Eidolon`) must ship with `level: None`, never
    /// a fabricated level -- the no-stub-mvp doctrine applied to this
    /// specific, named corpus gap.
    #[test]
    fn a_um_record_with_no_classes_or_domains_token_carries_no_level() {
        let row = spell_catalog_rows()
            .iter()
            .find(|row| row.book == SPELL_BOOK_UM && row.key == "Restore Eidolon")
            .expect("Restore Eidolon must be present in the UM catalog");
        assert_eq!(row.level, None);
    }
}
