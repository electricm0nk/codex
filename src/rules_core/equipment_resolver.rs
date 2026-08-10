//! SD-19 equipment-id resolver.
//!
//! Resolves a `CharacterInput.equipment_selections[].item_id` to its real
//! PCGen corpus record and (when available) the foundation slice's
//! canonical Paizo-table-cell reference.
//!
//! Lookup rule: (1) exact match against the record's verbatim `KEY:`
//! token, (2) exact match against the record's own unnormalized `name`
//! (needed for KEY-less records whose distinguishing content lives
//! inside parentheses, e.g. "Improvised Weapon (1d2)" vs "(1d3)" —
//! normalizing those away would collapse genuinely distinct items into
//! one), (3) a normalized match on the record's `name` (lowercase,
//! spaces -> underscores, parenthesized qualifiers like `"(Base)"`
//! stripped) as the last-resort fallback for the legacy
//! `"item:longsword"`-style fixture namespace, which predates
//! corpus-linkage and was never the corpus's own exact name.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::pcgen_import::source_content_payload::SourceContentPayload;
use crate::rules_core::pilot_compute_corpus::TableCellRef;
use crate::rules_core::rules_tables::crb::equipment_tables::equipment_tables;
use crate::rules_core::rules_tables::{
    acg, advanced_race_guide as arg, apg, beastiary1, crb, pathfinder_unchained as pu,
    ultimate_combat as uc, ultimate_equipment as ue, ultimate_intrigue as ui,
    ultimate_magic as um, ultimate_psionics as upsi, RuleSetId,
};
use crate::rules_core::source_content::{SourceContentKind, SourcePackageContent};

/// The record's `KEY:` token, if the corpus line carried one. PCGen
/// convention: absent means the record's `name` field is its own key.
pub fn equipment_key_token(record: &EquipmentRecord) -> Option<&str> {
    record
        .tokens
        .iter()
        .find(|token| token.key == "KEY")
        .map(|token| token.value.as_str())
}

fn normalize_equipment_name(name: &str) -> String {
    let stripped = match name.find('(') {
        Some(idx) => name[..idx].trim(),
        None => name.trim(),
    };
    stripped.to_lowercase().replace(' ', "_")
}

fn table_cell_for(rule_set: RuleSetId, key: &str) -> Option<TableCellRef> {
    equipment_tables()
        .iter()
        .find(|entry| entry.key == key)
        .map(|_| TableCellRef {
            rule_set,
            table: "equipment_tables".to_string(),
            row_key: key.to_string(),
            column_key: String::new(),
        })
}

pub fn equipment_id_resolve<'a>(
    item_id: &str,
    rule_set: RuleSetId,
    corpus: &SourcePackageContent<'a>,
) -> Option<(&'a EquipmentRecord, Option<TableCellRef>)> {
    let needle = item_id.strip_prefix("item:").unwrap_or(item_id);
    let normalized_needle = normalize_equipment_name(needle);

    let records: Vec<&'a EquipmentRecord> = corpus
        .records_by_kind(SourceContentKind::Equipment)
        .into_iter()
        .filter_map(|record| match record.payload {
            SourceContentPayload::Equipment(equip) => Some(equip),
            _ => None,
        })
        .collect();

    for equip in &records {
        if let Some(key) = equipment_key_token(equip)
            && (key == needle || key == item_id)
        {
            return Some((equip, table_cell_for(rule_set, key)));
        }
    }

    for equip in &records {
        if equip.name == needle || equip.name == item_id {
            let key = equipment_key_token(equip).unwrap_or(&equip.name);
            return Some((equip, table_cell_for(rule_set, key)));
        }
    }

    for equip in &records {
        if normalize_equipment_name(&equip.name) == normalized_needle {
            let key = equipment_key_token(equip).unwrap_or(&equip.name);
            return Some((equip, table_cell_for(rule_set, key)));
        }
    }

    None
}

/// Which ingested book a [`EquipmentCatalogRow`] came from. These codes are
/// the same wire codes the desktop equipment-catalog adapter emits
/// (`equipment_catalog::EQUIPMENT_CATALOG_BOOKS`); the desktop side pins the
/// two lists against each other in a test, so a book added here and not
/// there (or the reverse) is a caught failure, not a silent one.
pub const EQUIPMENT_BOOK_CRB: &str = "CRB";
pub const EQUIPMENT_BOOK_APG: &str = "APG";
pub const EQUIPMENT_BOOK_ACG: &str = "ACG";
pub const EQUIPMENT_BOOK_B1: &str = "B1";
pub const EQUIPMENT_BOOK_ARG: &str = "ARG";
pub const EQUIPMENT_BOOK_PU: &str = "PU";
pub const EQUIPMENT_BOOK_UI: &str = "UI";
pub const EQUIPMENT_BOOK_UE: &str = "UE";
pub const EQUIPMENT_BOOK_UM: &str = "UM";
pub const EQUIPMENT_BOOK_UPSI: &str = "UPSI";
pub const EQUIPMENT_BOOK_UC: &str = "UC";

/// One book's equipment row, projected onto the three fields every
/// *headless* (no-corpus) caller needs: which book it came from, its corpus
/// identity, and its flat gp price.
///
/// **Why this type exists.** Each ingested book defines its own
/// structurally-similar-but-distinct `EquipmentTableEntry` (different field
/// sets — `apg` has `weight`, `arg` has `weight_lbs`/`description`, `pu` has
/// `equip_type`/`plus` and no cost field at all), so there is no single Rust
/// type spanning them. Before this existed, every headless caller reached
/// for `crb::equipment_tables()` alone, because that was the only table
/// reachable through one name. That is exactly the defect this closes: the
/// desktop Attach Modifier picker offers **763** `Equipmods` rows across
/// CRB/ACG/ARG/PU, and the attach command recognized only CRB's 658 —
/// 105 rows were offered and then refused as "not a recognized equipment
/// catalog item" (a dead affordance,
/// `docs/governance/no-stub-mvp-doctrine.md`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentCatalogRow {
    /// One of the `EQUIPMENT_BOOK_*` codes above.
    pub book: &'static str,
    /// The record's corpus identity — its `KEY:` token when the row carries
    /// one, else its display name. **Not unique**: CRB alone carries 316
    /// keys twice (see [`equipment_catalog_row_by_key`]).
    pub key: &'static str,
    pub name: &'static str,
    /// The book's own `cost_gp`, verbatim — sourced from that record's
    /// corpus `COST:` token by the per-book table generator, never
    /// re-derived here. `None` when the corpus record has no `COST:` token
    /// at all (every one of PU's 42 ABP equipmods) or carries a PCGen
    /// formula the per-book table deliberately does not evaluate (e.g.
    /// ARG's `COST:WT*375` on `Material ~ Darkleaf Cloth ~ Item`, and CRB's
    /// own `+1`..`+10` bonus-squared enhancement pricing). Never a
    /// fabricated flat number.
    pub cost_gp: Option<f64>,
}

/// Every ingested book's equipment rows, in the same book order the desktop
/// catalog adapter chains them (CRB, APG, ACG, B1, ARG, PU) and, within a
/// book, in that book's own table order.
///
/// **The order is load-bearing, not incidental.** Both this module's
/// key-lookup and its cost resolution return the *first* matching row, and
/// CRB-first-then-source-order is precisely the order
/// `equipment_cost_gp_headless_resolve` already scanned before it was
/// widened — so every `item_id` CRB could already answer still resolves to
/// the identical CRB row and the identical price. Pinned by
/// `widening_leaves_every_crb_identity_resolving_to_its_original_cost`.
pub fn equipment_catalog_rows() -> &'static [EquipmentCatalogRow] {
    static ROWS: std::sync::OnceLock<Vec<EquipmentCatalogRow>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| {
        let crb_rows = crb::equipment_tables::equipment_tables().iter().map(|entry| {
            EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_CRB,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            }
        });
        let apg_rows = apg::equipment_tables::EQUIPMENT_TABLE.iter().map(|entry| {
            EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_APG,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            }
        });
        let acg_rows = acg::equipment_tables::equipment_tables().iter().map(|entry| {
            EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_ACG,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            }
        });
        let b1_rows = beastiary1::equipment_tables::EQUIPMENT_TABLE.iter().map(|entry| {
            EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_B1,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            }
        });
        let arg_rows = arg::equipment_tables::equipment_tables().iter().map(|entry| {
            EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_ARG,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            }
        });
        // PU's `EquipmentTableEntry` carries no cost field whatsoever:
        // `pu_equipmods.lst` has zero `COST:` tokens (its real cost signal
        // is an `ITEMCOST` formula `BONUS:`), so `None` here is the corpus
        // truth, not a lossy projection.
        let pu_rows = pu::equipment_tables::equipment_tables().iter().map(|entry| {
            EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_PU,
                key: entry.key,
                name: entry.name,
                cost_gp: None,
            }
        });

        // UI's `EquipmentTableEntry` carries a real `cost_gp` on most
        // records (see `ultimate_intrigue::equipment_tables`'s own doc
        // comment) -- passed through, not zeroed like PU's. Both
        // `equipment_tables()` and `equipmod_tables()` are chained under
        // the same `UI` book code, mirroring `equipment_catalog.rs`'s own
        // choice to serve them under one code.
        let ui_rows = ui::equipment_tables::equipment_tables()
            .iter()
            .chain(ui::equipment_tables::equipmod_tables())
            .map(|entry| EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_UI,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            });

        let ue_rows = ue::equipment_tables::equipment_tables()
            .iter()
            .chain(ue::equipment_tables::equipmod_tables())
            .map(|entry| EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_UE,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            });

        // UM (SD28-E15): 26 records -- 24 General (pregenerated spellbooks)
        // + 2 ArmsArmor (Scrollmaster Gear). No equipment-modifier file
        // exists for this book (`um::equipment_tables::equipmod_tables()`
        // is a real, permanently-empty slice, not an omission -- see that
        // module's own doc comment).
        let um_rows = um::equipment_tables::equipment_tables()
            .iter()
            .chain(um::equipment_tables::equipmod_tables())
            .map(|entry| EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_UM,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            });

        // UPsi (SD28-E15): 552 records -- 326 equipment + 226 equipmods
        // (the one `.MOD`-injected row already excluded at the table's own
        // source -- see `ultimate_psionics::equipment_tables`'s own doc
        // comment).
        let upsi_rows = upsi::equipment_tables::equipment_tables()
            .iter()
            .chain(upsi::equipment_tables::equipmod_tables())
            .map(|entry| EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_UPSI,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            });

        // UC (SD28-C4.9): 204 records -- 185 equipment + 19 equipmods (39
        // raw equipmods lines minus 20 `VISIBLE:NO` `.COPY=` legacy-alias
        // rows, the same exclusion shape UPsi's own table established --
        // see `ultimate_combat::equipment_tables`'s own doc comment).
        let uc_rows = uc::equipment_tables::equipment_tables()
            .iter()
            .chain(uc::equipment_tables::equipmod_tables())
            .map(|entry| EquipmentCatalogRow {
                book: EQUIPMENT_BOOK_UC,
                key: entry.key,
                name: entry.name,
                cost_gp: entry.cost_gp,
            });

        crb_rows
            .chain(apg_rows)
            .chain(acg_rows)
            .chain(b1_rows)
            .chain(arg_rows)
            .chain(pu_rows)
            .chain(ui_rows)
            .chain(ue_rows)
            .chain(um_rows)
            .chain(upsi_rows)
            .chain(uc_rows)
            .collect()
    })
}

/// The first cross-book catalog row whose `KEY:` identity is exactly `key`.
///
/// This is the single function a caller should use when it needs to both
/// **recognize** an item id and **price** it, because it guarantees the two
/// answers come from the same row. Resolving recognition and cost through
/// two independent lookups is what produced the defect this closes: the
/// desktop attach command recognized against CRB's table and priced against
/// CRB's table, so an ARG row was neither recognized nor priced — and
/// widening only recognition would have attached a real 500 gp
/// `Material ~ Whipwood` for free.
///
/// **First match, deliberately.** 316 keys appear twice within CRB alone,
/// 134 of those pairs with differing `cost_gp` (e.g.
/// `Intelligent Item ~ Ability Score / Charisma 11` is `Some(200.0)` then
/// `None`). Returning the first is exactly what `equipment_cost_gp_headless_resolve`
/// has always done, so this preserves shipped behavior rather than
/// silently repricing 134 CRB rows. That duplication is a pre-existing
/// property of `crb::equipment_tables` (pinned by
/// `equipment_catalog.rs`'s own `keys_do_not_collide_across_books_and_crbs_own_duplicates_are_pinned`),
/// not something this function creates or is able to fix from here.
pub fn equipment_catalog_row_by_key(key: &str) -> Option<&'static EquipmentCatalogRow> {
    equipment_catalog_rows().iter().find(|row| row.key == key)
}

/// v0.6 alpha swarm (money-purchase coupling, risks-and-open-questions.md
/// item 9): resolves an `item_id` to its `cost_gp`, with NO corpus access
/// at all -- unlike `equipment_id_resolve` above (which needs a real
/// `SourcePackageContent`), this only needs the flat cost figure, and
/// `equipment_tables()` (`rules_tables::crb::equipment_tables`) already
/// carries `cost_gp` on a `pub const`/`OnceLock`-cached table compiled
/// directly into the binary -- generated from the corpus at build time,
/// verified to mirror the same `KEY:`/`name` identity `equipment_id_resolve`
/// discovers (e.g. `"item:longsword"` -> `key: "Longsword (Base)"`,
/// `name: "Longsword"`, `cost_gp: Some(15.0)` on both paths). This is NOT
/// the same headless-vs-corpus-aware architecture wall that blocked
/// AC-widening earlier this swarm (real per-item AC deltas only exist via
/// a corpus-resolved `EquipmentRecord`; cost does not have that problem) --
/// checked, not assumed, before writing this function.
///
/// Mirrors `equipment_id_resolve`'s exact three-tier match (key, then
/// unnormalized name, then normalized name) against the static table
/// instead of corpus records, so behavior is identical for any item_id
/// both resolvers can find. Returns `None` when no entry matches OR the
/// matched entry's `cost_gp` is itself `None` (a genuine corpus absence --
/// a `(Base)` template record or a formula-priced equipment modifier, per
/// `EquipmentTableEntry.cost_gp`'s own doc comment) -- callers must treat
/// both cases identically (an unaffordable-to-verify purchase, not a free
/// one).
///
/// **SD-27: widened from CRB alone to [`equipment_catalog_rows`] (all six
/// ingested books).** It scanned `crb::equipment_tables()` only, while
/// every user-facing picker that feeds it (`list_equipment`, backed by
/// `build_equipment_catalog`) has served all six books since the catalog
/// widening. The mismatch made 105 of the 763 offered `Equipmods` rows
/// unattachable and 279 of the 435 offered `ArmsArmor` rows unpurchasable.
///
/// **Precedence, and why it is not a plain three-tier scan over the union.**
/// Naively widening each tier across all six books changes real CRB
/// answers, and naively resolving CRB entirely first mis-prices the newly
/// reachable books. Both were measured, not assumed:
///
/// - `"Wooden"` is the *name* of a CRB row (1 gp) and the *key* of an APG
///   row (20 gp). A key-tier-first scan over the union silently reprices
///   this shipped CRB identity from 1 gp to 20 gp.
/// - 90 non-CRB rows -- `Chest (Small)`, `Arrow (Flight)`, `Tent (Large)`,
///   ... -- have keys whose *normalized* form collides with a CRB row's
///   name (`Chest (Small)` -> `chest`). Resolving all of CRB first would
///   answer APG's 2 gp `Chest (Small)` with CRB's Chest price.
///
/// The ordering below is the one that is correct in both directions. It
/// treats an exact `KEY:`/`name` match as a strong identity and the
/// normalized tier as what its own module doc says it is -- a last-resort
/// compatibility shim for the legacy `"item:longsword"` fixture namespace,
/// which predates every non-CRB book and so is CRB-only by construction:
///
/// 1. CRB exact `KEY:`
/// 2. CRB exact `name`
/// 3. every other book's exact `KEY:`, in chain order
/// 4. every other book's exact `name`, in chain order
/// 5. CRB normalized `name` (the legacy shim, now genuinely last)
///
/// There is deliberately no normalized tier for the other five books:
/// they never had a legacy id namespace to be compatible with, so a lossy
/// match there would buy nothing and risk exactly the `chest` mispricing
/// above.
///
/// Every `item_id` CRB could already answer resolves to the identical row
/// and identical price -- proven exhaustively over all 2,977 CRB rows'
/// keys, names and legacy-prefixed names by
/// `widening_leaves_every_crb_identity_resolving_to_its_original_cost`.
pub fn equipment_cost_gp_headless_resolve(item_id: &str) -> Option<f64> {
    let needle = item_id.strip_prefix("item:").unwrap_or(item_id);
    let normalized_needle = normalize_equipment_name(needle);
    let rows = equipment_catalog_rows();
    let is_crb = |row: &&EquipmentCatalogRow| row.book == EQUIPMENT_BOOK_CRB;
    let is_not_crb = |row: &&EquipmentCatalogRow| row.book != EQUIPMENT_BOOK_CRB;
    let key_hit = |row: &&EquipmentCatalogRow| row.key == needle || row.key == item_id;
    let name_hit = |row: &&EquipmentCatalogRow| row.name == needle || row.name == item_id;

    if let Some(row) = rows.iter().filter(is_crb).find(key_hit) {
        return row.cost_gp;
    }
    if let Some(row) = rows.iter().filter(is_crb).find(name_hit) {
        return row.cost_gp;
    }
    if let Some(row) = rows.iter().filter(is_not_crb).find(key_hit) {
        return row.cost_gp;
    }
    if let Some(row) = rows.iter().filter(is_not_crb).find(name_hit) {
        return row.cost_gp;
    }
    if let Some(row) = rows
        .iter()
        .filter(is_crb)
        .find(|row| normalize_equipment_name(row.name) == normalized_needle)
    {
        return row.cost_gp;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use crate::rules_core::source_content::SourceRef;

    fn corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("test.lst", text);
        let source_ref = SourceRef {
            lst_file: "test.lst".to_string(),
            line: 1,
        };
        let mut corpus = SourcePackageContent::empty("test", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    /// Regression test: KEY-less records whose only distinguishing
    /// content is inside parentheses (e.g. the real corpus's
    /// "Improvised Weapon (1d2)" through "(2d10)" damage-die variants)
    /// must resolve to themselves exactly, not to whichever sibling the
    /// lossy normalized-name fallback happens to hit first.
    #[test]
    fn key_less_records_distinguished_only_by_parenthesized_content_resolve_exactly() {
        let text = "\
Improvised Weapon (1d2)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:1
Improvised Weapon (1d3)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:1
Improvised Weapon (1d4)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:2
";
        let corpus = corpus_from(text);

        let (record, _) = equipment_id_resolve("Improvised Weapon (1d3)", RuleSetId::Crb, &corpus)
            .expect("expected 'Improvised Weapon (1d3)' to resolve");
        assert_eq!(record.name, "Improvised Weapon (1d3)");

        let (record, _) = equipment_id_resolve("Improvised Weapon (1d2)", RuleSetId::Crb, &corpus)
            .expect("expected 'Improvised Weapon (1d2)' to resolve");
        assert_eq!(record.name, "Improvised Weapon (1d2)");
    }

    /// Control: the legacy `"item:longsword"`-style fixture namespace
    /// must still resolve via the normalized-name fallback, since it
    /// predates corpus-linkage and never matches the corpus's exact name.
    #[test]
    fn legacy_item_prefix_fixture_namespace_still_resolves_via_normalized_fallback() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\n";
        let corpus = corpus_from(text);

        let (record, _) = equipment_id_resolve("item:longsword", RuleSetId::Crb, &corpus)
            .expect("expected 'item:longsword' to resolve via the normalized fallback");
        assert_eq!(record.name, "Longsword");
    }

    #[test]
    fn equipment_cost_gp_headless_resolve_finds_a_real_item_by_the_legacy_item_prefix() {
        assert_eq!(equipment_cost_gp_headless_resolve("item:longsword"), Some(15.0));
    }

    #[test]
    fn equipment_cost_gp_headless_resolve_finds_a_real_item_by_its_exact_corpus_key() {
        assert_eq!(
            equipment_cost_gp_headless_resolve("Longsword (Base)"),
            Some(15.0)
        );
    }

    #[test]
    fn equipment_cost_gp_headless_resolve_returns_none_for_an_unknown_item() {
        assert_eq!(
            equipment_cost_gp_headless_resolve("item:not_a_real_item_at_all"),
            None
        );
    }

    // ----- SD-27: cross-book widening (dead-affordance closure) -----

    /// The exact pre-widening implementation, kept verbatim as a test-only
    /// oracle. Not a paraphrase of the old code -- a copy of it, so the
    /// regression check below compares against what actually shipped rather
    /// than against a re-derivation that could drift the same way the
    /// production path did.
    fn crb_only_cost_oracle(item_id: &str) -> Option<f64> {
        let needle = item_id.strip_prefix("item:").unwrap_or(item_id);
        let normalized_needle = normalize_equipment_name(needle);
        let table = equipment_tables();
        for entry in table {
            if entry.key == needle || entry.key == item_id {
                return entry.cost_gp;
            }
        }
        for entry in table {
            if entry.name == needle || entry.name == item_id {
                return entry.cost_gp;
            }
        }
        for entry in table {
            if normalize_equipment_name(entry.name) == normalized_needle {
                return entry.cost_gp;
            }
        }
        None
    }

    /// **The "CRB behaviour is unchanged" proof.** Every identity the
    /// shipped CRB-only resolver could answer -- all 2,977 rows' `key`s,
    /// all their `name`s, and each name under the legacy `item:` prefix --
    /// resolves through the widened resolver to the byte-identical price.
    /// Exhaustive, not sampled: a five-book widening that silently
    /// repriced even one CRB item would be a strictly worse defect than
    /// the dead affordance it was fixing.
    #[test]
    fn widening_leaves_every_crb_identity_resolving_to_its_original_cost() {
        let mut checked = 0usize;
        for entry in equipment_tables() {
            for probe in [
                entry.key.to_string(),
                entry.name.to_string(),
                format!("item:{}", normalize_equipment_name(entry.name)),
            ] {
                assert_eq!(
                    equipment_cost_gp_headless_resolve(&probe),
                    crb_only_cost_oracle(&probe),
                    "widening changed the resolved cost for CRB identity {probe:?}"
                );
                checked += 1;
            }
        }
        assert_eq!(
            checked,
            equipment_tables().len() * 3,
            "every CRB row must contribute all three probes"
        );
        assert_eq!(equipment_tables().len(), 2_977, "CRB's own real row count");
    }

    /// The rows really do span all six ingested books, in the documented
    /// order, with the real per-book counts. Pinned so a book silently
    /// dropping out of the chain -- which is precisely how the original
    /// defect looked from the outside -- fails loudly here.
    #[test]
    fn catalog_rows_span_every_ingested_book_with_their_real_counts() {
        let rows = equipment_catalog_rows();
        let count = |book: &str| rows.iter().filter(|row| row.book == book).count();
        assert_eq!(count(EQUIPMENT_BOOK_CRB), 2_977);
        assert_eq!(count(EQUIPMENT_BOOK_APG), 338);
        assert_eq!(count(EQUIPMENT_BOOK_ACG), 269);
        assert_eq!(count(EQUIPMENT_BOOK_B1), 4);
        assert_eq!(count(EQUIPMENT_BOOK_ARG), 200);
        assert_eq!(count(EQUIPMENT_BOOK_PU), 42);
        assert_eq!(count(EQUIPMENT_BOOK_UI), 98);
        assert_eq!(count(EQUIPMENT_BOOK_UE), 1_549);
        // SD28-E15: UM's 26-record equipment table (24 General + 2
        // ArmsArmor). Re-derived from the catalog itself, not by hand-adding
        // 26 to the old 5,477 total -- also independently confirmed the 26
        // UM keys are unique within the book (no internal duplicate).
        assert_eq!(count(EQUIPMENT_BOOK_UM), 26);
        // SD28-E15: UPsi's 439-record equipment table (326 equipment + 113
        // equipmods). Corrected after landing: the first extraction pass
        // wrongly fabricated 113 near-empty entries for VISIBLE:NO .COPY=
        // legacy aliases; the real, correct table excludes them (same
        // exclusion shape ultimate_intrigue/advanced_race_guide already
        // establish for their own VISIBLE:NO .COPY= "Old KEYs" blocks) --
        // see `ultimate_psionics::equipment_tables`'s own doc comment.
        // Re-derived from the catalog itself, not by hand-adding 439 to the
        // old 5,503 total -- also independently confirmed the 439 UPsi keys
        // are unique within the book.
        assert_eq!(count(EQUIPMENT_BOOK_UPSI), 439);
        // SD28-C4.9: UC's 204-record equipment table (185 equipment + 19
        // equipmods). The declared work-inventory equipment figure (185)
        // matches this table's own derivation exactly; the equipmods
        // figure (39) does not -- it is the raw content-line count
        // including 20 VISIBLE:NO .COPY= legacy aliases, the same hazard
        // UPsi's own table found. Real count: 19. See
        // `ultimate_combat::equipment_tables`'s own doc comment.
        assert_eq!(count(EQUIPMENT_BOOK_UC), 204);
        assert_eq!(rows.len(), 6_146);

        // CRB first, then the documented chain order -- the property the
        // "CRB behaviour unchanged" guarantee rests on.
        let first_index = |book: &str| rows.iter().position(|row| row.book == book).unwrap();
        assert_eq!(first_index(EQUIPMENT_BOOK_CRB), 0);
        assert!(first_index(EQUIPMENT_BOOK_APG) < first_index(EQUIPMENT_BOOK_ACG));
        assert!(first_index(EQUIPMENT_BOOK_ACG) < first_index(EQUIPMENT_BOOK_B1));
        assert!(first_index(EQUIPMENT_BOOK_B1) < first_index(EQUIPMENT_BOOK_ARG));
        assert!(first_index(EQUIPMENT_BOOK_ARG) < first_index(EQUIPMENT_BOOK_PU));
    }

    /// Recognition and price for a non-CRB row now come from that row's own
    /// book, at its own real corpus price -- the half-fix this change
    /// specifically had to avoid was recognizing `Material ~ Whipwood` and
    /// then attaching a genuinely 500 gp item for free.
    #[test]
    fn a_real_arg_equipmod_resolves_to_its_own_books_real_cost() {
        let row = equipment_catalog_row_by_key("Material ~ Whipwood")
            .expect("ARG's Whipwood must be recognized");
        assert_eq!(row.book, EQUIPMENT_BOOK_ARG);
        assert_eq!(row.name, "Whipwood");
        assert_eq!(row.cost_gp, Some(500.0), "arg_equipmods.lst carries COST:500");
        assert_eq!(equipment_cost_gp_headless_resolve("Material ~ Whipwood"), Some(500.0));
    }

    /// ACG's own priced equipmods were refused by the same defect (the
    /// 105 refused rows are ACG 48 + ARG 15 + PU 42, not ARG+PU alone).
    #[test]
    fn a_real_acg_equipmod_resolves_to_its_own_books_real_cost() {
        let row = equipment_catalog_row_by_key("Special Ability ~ Amorphous ~ Armor")
            .expect("ACG's Amorphous must be recognized");
        assert_eq!(row.book, EQUIPMENT_BOOK_ACG);
        assert_eq!(row.cost_gp, Some(4500.0));
    }

    /// PU's 42 ABP equipmods carry no `COST:` token anywhere in
    /// `pu_equipmods.lst`, so they must resolve as *recognized with no
    /// known price* -- never as a fabricated `Some(0.0)`, and never as
    /// unrecognized.
    #[test]
    fn every_pu_equipmod_is_recognized_and_honestly_priceless() {
        let pu_rows: Vec<_> = equipment_catalog_rows()
            .iter()
            .filter(|row| row.book == EQUIPMENT_BOOK_PU)
            .collect();
        assert_eq!(pu_rows.len(), 42);
        for row in pu_rows {
            assert!(
                equipment_catalog_row_by_key(row.key).is_some(),
                "{:?} must be recognized",
                row.key
            );
            assert_eq!(row.cost_gp, None, "{:?} must not carry a fabricated price", row.key);
        }
        // The specific key the desktop app refused on screen.
        assert!(equipment_catalog_row_by_key("ABP ~ +3 Attunement ~ Armor").is_some());
    }

    /// The two lookups answer two different questions, and for all but one
    /// of the 3,830 catalog keys they agree:
    ///
    /// - [`equipment_catalog_row_by_key`] answers *"the user picked this
    ///   exact catalog row; what is it?"*
    /// - [`equipment_cost_gp_headless_resolve`] answers *"resolve this
    ///   free-form `item_id`"*, and keeps CRB's shipped precedence for
    ///   ambiguous strings.
    ///
    /// The single divergence is real and is pinned by name so it can never
    /// grow silently: `"Wooden"` is APG's `KEY:` (20 gp) and also a CRB
    /// row's `name` (1 gp). Callers handed a catalog key by a picker must
    /// therefore price via `equipment_catalog_row_by_key`, not via the
    /// free-form resolver -- which is exactly what
    /// `attach_equipment_modifier_at_root` and `purchase_equipment_at_root`
    /// now do. Resolving recognition from one lookup and price from the
    /// other is the defect shape this whole change exists to remove.
    #[test]
    fn the_two_lookups_agree_on_every_catalog_key_but_the_one_pinned_collision() {
        let mut disagreements: Vec<&str> = Vec::new();
        for row in equipment_catalog_rows() {
            let by_key = equipment_catalog_row_by_key(row.key)
                .expect("every catalog row's own key must resolve");
            if by_key.cost_gp != equipment_cost_gp_headless_resolve(row.key) {
                disagreements.push(row.key);
            }
        }
        disagreements.sort_unstable();
        disagreements.dedup();
        assert_eq!(
            disagreements,
            vec!["Wooden"],
            "exactly one cross-book identity collision exists today; a new one means a newly \
             ambiguous id that callers must be told how to disambiguate"
        );

        // The collision itself, stated explicitly rather than left implicit
        // in the list above.
        assert_eq!(
            equipment_catalog_row_by_key("Wooden").map(|row| (row.book, row.cost_gp)),
            Some((EQUIPMENT_BOOK_APG, Some(20.0)))
        );
        assert_eq!(
            equipment_cost_gp_headless_resolve("Wooden"),
            Some(1.0),
            "the free-form resolver keeps CRB's shipped answer for this ambiguous string"
        );
    }
}
