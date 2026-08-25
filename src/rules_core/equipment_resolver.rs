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
    equipment_gap_tables, ultimate_magic as um, ultimate_psionics as upsi, RuleSetId,
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

    // A record's corpus IDENTITY is its `KEY:` token when it carries one and
    // its NAME when it does not -- the same rule `equipment_catalog_rows()`
    // uses to mint the very keys this function is asked to resolve. Matching on
    // identity first is what makes the answer a rule rather than a coincidence
    // of corpus scan order.
    //
    // This pass used to test the `KEY:` token alone, so a needle naming a
    // KEYLESS record fell through to the bare-name pass below -- where a record
    // that merely DISPLAYS that name, while being identified as something else
    // entirely, could answer first purely because the filesystem handed it over
    // first. `corpus_loader::find_json_files` walks with `read_dir`, whose
    // order is stable for one directory on one machine and not stable across
    // two checkouts of the same corpus, so the winner was a property of the
    // disk rather than of the data.
    //
    // CRB's `Shoes` is the live instance and it moved a doneness number:
    // `equipment/general/shoes.json` is the item (no `KEY:`, so its identity is
    // `Shoes`; `TYPE`/`COST`/`WT`/`SLOTS`/`MODS`/`QUALITY` and not one
    // mechanical token), while `equipment/equipmods/artisan_s_tools_shoes.json`
    // is an equipment MODIFIER whose identity is `Artisan's Tools (Shoes)` and
    // whose display name is also `Shoes`. Resolving `Shoes` to the modifier
    // makes `equipment_key_is_wired` report a mechanical effect for an item
    // that has none, promoting `core_rulebook:equipment:shoes` to `grounded` on
    // a different record's tokens. That is the name-coincidence over-claim
    // `modelled_race_of_race_trait` and this probe's own book-scoping already
    // exist to prevent, here appearing INSIDE one book.
    //
    // 38 equipment names across CRB/ACG/ARG are decided this way. `Potion` is
    // the widest: `general/potion.json` (the empty flask) against fifty-odd
    // `Potion of ...`/`Oil of ...` magic items whose display name is likewise
    // `Potion`. Identity resolves every one of them to the right record.
    for equip in &records {
        let identity = equipment_key_token(equip).unwrap_or(&equip.name);
        if identity == needle || identity == item_id {
            return Some((equip, table_cell_for(rule_set, identity)));
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
/// Ultimate Wilderness. This book has no hand-authored `equipment_tables`
/// module at all — every one of its catalog rows comes from
/// [`equipment_gap_tables`], which is why the code is declared here rather
/// than beside a per-book table import.
pub const EQUIPMENT_BOOK_UW: &str = "UW";

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
        // Hand-authored per-book tables first, in their established order, then
        // the corpus-recovered gap rows. Order is load-bearing (see this
        // function's own doc comment): putting the gap rows LAST means every
        // key any hand table already answered still resolves to the identical
        // hand row at the identical price.
        hand_authored_equipment_rows()
            .iter()
            .copied()
            .chain(equipment_gap_tables::equipment_gap_rows().map(|row| EquipmentCatalogRow {
                book: row.book,
                key: row.key,
                name: row.name,
                cost_gp: row.cost_gp,
            }))
            .collect()
    })
}

/// The eleven hand-authored per-book equipment tables, chained — everything
/// [`equipment_catalog_rows`] served before the corpus gap lane landed.
///
/// **Why this is a separate public function.** `gen_equipment_gap_tables`
/// needs to know what the hand tables already hold in order to emit only the
/// records they do not; asking [`equipment_catalog_rows`] would be circular
/// once the generated rows are chained into it. Splitting the two makes the
/// generator's filter provably the complement of these tables rather than a
/// hand-maintained exclusion list that can drift — the same
/// derive-don't-restate fix `equipment_catalog_books()` already applies on the
/// desktop side.
pub fn hand_authored_equipment_rows() -> &'static [EquipmentCatalogRow] {
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
    // Hand-authored tables are searched to exhaustion FIRST, and only then
    // the full catalog (which adds the corpus gap rows).
    //
    // **This two-pass shape is a fix, not a flourish.** Chaining the gap rows
    // last is not by itself enough to leave shipped pricing alone, because
    // this resolver's precedence is *stage-major, not row-major*: stage 1
    // matches any CRB row by KEY before stage 2 matches any CRB row by NAME.
    // `Cold Iron` is exactly that collision — CRB's hand table holds a row
    // whose display *name* is `Cold Iron` (0 gp), while `cr_equipmods.lst`
    // holds a distinct record whose `KEY:` is `Cold Iron` and which carries no
    // `COST:` token at all. With one pass over the combined rows the gap row
    // won stage 1 and repriced a shipped CRB identity `Some(0.0)` -> `None`.
    // Caught by `widening_leaves_every_crb_identity_resolving_to_its_original_cost`,
    // which exists for precisely this class and is the reason it is exhaustive
    // rather than sampled.
    if let Some(row) = resolve_catalog_row(hand_authored_equipment_rows(), item_id) {
        return row.cost_gp;
    }
    resolve_catalog_row(equipment_catalog_rows(), item_id).and_then(|row| row.cost_gp)
}

/// The five-stage identity match this resolver has always used, over one row
/// set. Returns the ROW rather than its cost, so "matched a row whose price is
/// honestly `None`" stays distinguishable from "matched nothing" — the
/// distinction the two-pass caller above depends on.
fn resolve_catalog_row<'a>(
    rows: &'a [EquipmentCatalogRow],
    item_id: &str,
) -> Option<&'a EquipmentCatalogRow> {
    let needle = item_id.strip_prefix("item:").unwrap_or(item_id);
    let normalized_needle = normalize_equipment_name(needle);
    let is_crb = |row: &&EquipmentCatalogRow| row.book == EQUIPMENT_BOOK_CRB;
    let is_not_crb = |row: &&EquipmentCatalogRow| row.book != EQUIPMENT_BOOK_CRB;
    let key_hit = |row: &&EquipmentCatalogRow| row.key == needle || row.key == item_id;
    let name_hit = |row: &&EquipmentCatalogRow| row.name == needle || row.name == item_id;

    rows.iter()
        .filter(is_crb)
        .find(key_hit)
        .or_else(|| rows.iter().filter(is_crb).find(name_hit))
        .or_else(|| rows.iter().filter(is_not_crb).find(key_hit))
        .or_else(|| rows.iter().filter(is_not_crb).find(name_hit))
        .or_else(|| {
            rows.iter()
                .filter(is_crb)
                .find(|row| normalize_equipment_name(row.name) == normalized_needle)
        })
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

    /// The two real CRB records behind the `Shoes` defect, as their `.lst`
    /// rows: the item (KEY-less, so its corpus identity is its name, and it
    /// carries no mechanical token) and the equipment MODIFIER that merely
    /// displays the same name while being identified as
    /// `Artisan's Tools (Shoes)`.
    const SHOES_ITEM: &str = "Shoes\tTYPE:Feet.Shoes\tCOST:0\tWT:0\tSLOTS:2\tMODS:REQUIRED\n";
    const SHOES_MODIFIER: &str =
        "Shoes\tKEY:Artisan's Tools (Shoes)\tTYPE:EQMODARTISAN\tCOST:0\tVISIBLE:QUALITY\n";

    /// A needle must resolve to the record whose corpus IDENTITY it is, never
    /// to a record that merely DISPLAYS that name while being identified as
    /// something else.
    ///
    /// Both orders are asserted, and that is the whole point: before the
    /// identity pass existed the answer was first-match-wins over the corpus in
    /// `read_dir` order, so which record answered was a property of the
    /// filesystem rather than of the data — stable on one machine, different in
    /// another checkout of the same corpus. Sorting that scan (which this cycle
    /// also did) flipped `core_rulebook:equipment:shoes` from
    /// `ingested-magnitude` to a FALSE `grounded`, because
    /// `equipment_key_is_wired` then read the modifier's tokens and reported a
    /// mechanical effect for an item that has none. Determinism alone would
    /// have frozen the wrong answer; this rule makes it the right one either
    /// way.
    #[test]
    fn a_needle_resolves_to_the_record_whose_identity_it_is_not_to_a_name_twin() {
        for (label, text) in [
            ("item first", format!("{SHOES_ITEM}{SHOES_MODIFIER}")),
            ("modifier first", format!("{SHOES_MODIFIER}{SHOES_ITEM}")),
        ] {
            let corpus = corpus_from(&text);
            let (record, _) = equipment_id_resolve("Shoes", RuleSetId::Crb, &corpus)
                .expect("expected 'Shoes' to resolve");
            assert_eq!(
                equipment_key_token(record),
                None,
                "[{label}] 'Shoes' must resolve to the KEY-less item whose identity is 'Shoes', \
                 not to the modifier identified as \"Artisan's Tools (Shoes)\""
            );
            assert!(
                record.tokens.iter().any(|t| t.key == "SLOTS"),
                "[{label}] resolved the wrong record: the item carries SLOTS, the modifier does not"
            );
        }
    }

    /// The other half of the same rule: the name-twin is still reachable, by
    /// its own identity. Fixing the collision must not make a real record
    /// unresolvable.
    #[test]
    fn the_name_twin_is_still_reachable_by_its_own_corpus_key() {
        for (label, text) in [
            ("item first", format!("{SHOES_ITEM}{SHOES_MODIFIER}")),
            ("modifier first", format!("{SHOES_MODIFIER}{SHOES_ITEM}")),
        ] {
            let corpus = corpus_from(&text);
            let (record, _) =
                equipment_id_resolve("Artisan's Tools (Shoes)", RuleSetId::Crb, &corpus)
                    .expect("expected the modifier to resolve by its own KEY");
            assert_eq!(
                equipment_key_token(record),
                Some("Artisan's Tools (Shoes)"),
                "[{label}] the modifier must still be reachable by its identity"
            );
        }
    }

    /// The widest instance of the same shape in the real corpus: CRB's
    /// `general/potion.json` (the empty flask, KEY-less) against fifty-odd
    /// `Potion of ...` / `Oil of ...` magic items that all display as `Potion`.
    /// Asking for `Potion` must yield the flask, not whichever potion the disk
    /// offered first.
    #[test]
    fn a_keyless_generic_wins_over_its_many_specific_name_twins() {
        let text = "\
Potion\tKEY:Potion of Fly\tTYPE:Magic.Potion\tCOST:750
Potion\tTYPE:Item.Potion\tCOST:0\tWT:0
Potion\tKEY:Potion of Blur\tTYPE:Magic.Potion\tCOST:300
";
        let corpus = corpus_from(text);
        let (record, _) = equipment_id_resolve("Potion", RuleSetId::Crb, &corpus)
            .expect("expected 'Potion' to resolve");
        assert_eq!(
            equipment_key_token(record),
            None,
            "'Potion' must resolve to the KEY-less flask whose identity is 'Potion'"
        );
        // ... and each specific potion stays reachable by its own identity.
        let (fly, _) = equipment_id_resolve("Potion of Fly", RuleSetId::Crb, &corpus)
            .expect("expected 'Potion of Fly' to resolve");
        assert_eq!(equipment_key_token(fly), Some("Potion of Fly"));
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
        // `SD31-E6-F10-002`: 3 rows moved CRB -> B1 (`decisions.md §9`
        // re-attribution; `tests/equipment_gap_tables.rs`'s own doc comment
        // has the full story). 3312 - 3 = 3309; 4 + 3 = 7.
        assert_eq!(count(EQUIPMENT_BOOK_CRB), 3309);
        assert_eq!(count(EQUIPMENT_BOOK_APG), 375);
        assert_eq!(count(EQUIPMENT_BOOK_ACG), 319);
        assert_eq!(count(EQUIPMENT_BOOK_B1), 7);
        assert_eq!(count(EQUIPMENT_BOOK_ARG), 215);
        assert_eq!(count(EQUIPMENT_BOOK_PU), 42);
        assert_eq!(count(EQUIPMENT_BOOK_UI), 105);
        // `SD31-E6-F10-003`: 1614 -> 1613. Extending this generator's own
        // `declared_pi_at` check (built for the 8 new books this cycle)
        // over the FULL compiled table caught a genuine, PRE-EXISTING PI
        // leak this cycle did not introduce: `ultimate_equipment:"Elysian
        // Shield"` declares `NAMEISPI:YES` in the real corpus
        // (`ue_equip_arms_armor.lst`) and was shipping unscreened in this
        // compiled table (and therefore live in the desktop catalog, which
        // reads `equipment_catalog_rows()` -- chaining this table directly,
        // never through `data/corpus/`'s own, separately-screened JSON) --
        // `gen_cache_equipment_gap`'s JSON-write path already excluded it
        // correctly; this generator's OWN output did not, until now.
        //
        // SD-32 T9 onboarding (card 11): re-derived pre-existing red,
        // unrelated to this cycle's own group-E rename fix -- this
        // catalog (`hand_authored_equipment_rows`) chains
        // `ue::equipment_tables::{equipment_tables,equipmod_tables}()`
        // verbatim with NO `declared_pi_at`/NAMEISPI screening of its own
        // (confirmed: no such call anywhere in this function), so its row
        // count is exactly that static table's length and does not move
        // with `cache_gen::ultimate_equipment`'s corpus-JSON generator.
        // `src/rules_core/rules_tables/ultimate_equipment/equipment_tables.rs`
        // is byte-identical to this branch's own pinned base (`git diff`
        // against `origin/tranche/12` is empty), so 1614 was already the
        // real count before this cycle touched anything -- 1613 was stale.
        assert_eq!(count(EQUIPMENT_BOOK_UE), 1614);
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
        assert_eq!(count(EQUIPMENT_BOOK_UPSI), 552);
        // SD28-C4.9: UC's 204-record equipment table (185 equipment + 19
        // equipmods). The declared work-inventory equipment figure (185)
        // matches this table's own derivation exactly; the equipmods
        // figure (39) does not -- it is the raw content-line count
        // including 20 VISIBLE:NO .COPY= legacy aliases, the same hazard
        // UPsi's own table found. Real count: 19. See
        // `ultimate_combat::equipment_tables`'s own doc comment.
        assert_eq!(count(EQUIPMENT_BOOK_UC), 224);
        // SD-29 `epic-4-proven-equip-mod`: UW reaches this chain only through
        // `equipment_gap_tables`; it has no hand-authored table at all.
        assert_eq!(count(EQUIPMENT_BOOK_UW), 127);
        // 6,146 hand-authored + 769 corpus gap rows. The +769 is exactly
        // `docs/work-inventory.json`'s `not-ingested` equipment /
        // equipment_modifier population across the nine already-compiled
        // books; `tests/equipment_gap_tables.rs` pins the per-book split.
        // `SD31-E6-F10-003`: the gap lane's own row count grew by 421
        // (769 -> 1,190; 8 further already-compiled books, net of 12
        // declared-PI exclusions AND 2 bare PFS organized-play legality
        // OVERLAY rows this cycle's own fixes caught -- see
        // `gen_equipment_gap_tables.rs`'s `declared_pi_at` and its
        // `is_non_record_line` `PFSNotLegal` extension), so the total grows
        // by the same 421 (6,915 -> 7,336). Hand-authored count is
        // unchanged; this card's file grant never touches a hand-authored
        // table.
        // `SD31-E6-F10-004`: the gap lane's own row count grew by another
        // 481 (1,190 -> 1,671; the 5 books `SD31-E6-F10-003` deliberately
        // left out, `OPEN-ISSUES.md` row 186, now reachable via a
        // per-record blacklist pre-filter rather than the whole-file hard
        // stop -- see `gen_equipment_gap_tables.rs`'s `blacklist_hit`), so
        // the total grows by the same 481 (7,336 -> 7,817). Hand-authored
        // count is unchanged; this card's file grant never touches a
        // hand-authored table.
        assert_eq!(hand_authored_equipment_rows().len(), 6_146);
        // SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off: the
        // gap lane's own row count grew by 49 (1,671 -> 1,720; two more
        // already-compiled books, inner_sea_temples 43 + inner_sea_magic 6),
        // so the total grows by the same 49 (7,817 -> 7,866). Hand-authored
        // count is unchanged.
        // SD-32 T9 residual (`decisions.md §20`): the gap lane's own row
        // count grew by another 159 (1,720 -> 1,879; `cache_gen::equipment_
        // gap::book_routing` had no arm for `ISTEM`/`ISM` at all, silently
        // dropping rows the table already generated -- fixed, +0 new rows,
        // just unblocked; `ISM` also regained its `ism_equipmods.lst`
        // citations on a stale exclusion, +62; the new `adventurers_guide`
        // book adds +97; the new `ultimate_magic` book adds +0, see
        // `tests/equipment_gap_tables.rs`), so the total grows by the same
        // 159 (7,866 -> 8,025). Hand-authored count is unchanged.
        //
        // SD-32 T9 onboarding (card 11): re-derived pre-existing red. At
        // this cycle's PIN, `equipment_gap_tables.rs`'s own header already
        // stated "Total: 1953 rows" -- 74 more than the 1,879 this pinned
        // lineage's arithmetic assumes (6,146 + 1,879 = 8,025) -- an
        // untraced drift from an earlier cycle's regen that updated the
        // generated table without updating this pinned total. This cycle's
        // own group-C fix (`decisions.md §20` residual: `ag_equipmods.lst`
        // was absent from `adventurers_guide`'s `BOOK_INPUTS`) adds the
        // real, evidenced +1 (1,953 -> 1,954, one `equipment_modifier` row,
        // `cargo run --locked --bin gen_equipment_gap_tables`'s own diff:
        // 3 insertions, 2 deletions, all in the `adventurers_guide` block).
        // Retargeted to the proven total: 6,146 + 1,954 = 8,100.
        //
        // `AT-33-E6-001` (2026-08-25): 8,100 was already stale AT THE
        // `tranche/13` CUT (`f652db7ac7`) -- inherited, not caused by any
        // SD-33 wave. `equipment_gap_tables.rs`'s own header already stated
        // "Total: 1973 rows" at that exact commit (`git show
        // f652db7ac7:src/rules_core/rules_tables/equipment_gap_tables.rs |
        // grep '^//! Total:'` -> `1973 rows`; `git log
        // f652db7ac7..HEAD -- src/rules_core/rules_tables/
        // equipment_gap_tables.rs` is EMPTY -- no SD-33 commit, including
        // wave 6's `data/corpus/**` regeneration, ever touched this
        // generated file) -- another untraced drift from whatever cycle
        // last ran `gen_equipment_gap_tables` before the cut, same shape as
        // the 1953->1954 one two paragraphs up: the generated table moved,
        // this pinned arithmetic did not follow it. Hand-authored count is
        // unchanged (still 6,146, asserted above, still passing) --
        // 6,146 + 1,973 = 8,119, matching `rows.len()` live. Retargeted to
        // the proven total.
        assert_eq!(rows.len(), 8_119);

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
        // SD-29 `epic-4-proven-equip-mod` grew this list from 1 to 28, and
        // every addition is one shape: a corpus gap row whose `KEY:` equals
        // some hand-authored row's display NAME. `Cold Iron` is the worked
        // example — CRB's hand table has a row *named* `Cold Iron` (0 gp);
        // `cr_equipmods.lst` has a different record *keyed* `Cold Iron` with
        // no `COST:` token. Both records are real and both belong in the
        // catalog; what is ambiguous is only the free-form string.
        //
        // The remedy is the one this test's own doc comment already
        // prescribes and that `attach_equipment_modifier_at_root` /
        // `purchase_equipment_at_root` already follow: a caller holding a
        // catalog key from a picker prices via `equipment_catalog_row_by_key`,
        // never via the free-form resolver. Pinned by name, not by count, so
        // a 29th — which would be a genuinely new ambiguity — still fails.
        //
        // CORRECTED `SD31-W4-INTEGRATE-001`, 2026-08-16 (found already red
        // at this integration cycle's own inherited tip, `40771d3bf`,
        // predating every wave-4 branch): a prior pass added 8 ACG
        // equipment_modifier names (Amorphous, Burdenless, Exclusionary,
        // Prehensile, Restful, Sneaky, Spiteful, Trackless) to this pinned
        // list, apparently anticipating a KEY/NAME collision by the same
        // shape as `Cold Iron`. Traced one deep: `acg/equipment_data/
        // equipmods.rs`'s hand-authored `Amorphous` row and
        // `equipment_gap_tables.rs`'s gap row both cost 4500 gp -- the two
        // lookups AGREE (both resolve to 4500), so there is no real pricing
        // ambiguity for any of these 8, only a coincidental KEY/NAME name
        // reuse with an identical price. Removed the 8 phantom entries;
        // `disagreements` reproduces this 28-item list exactly, verified by
        // an isolated `cargo test --lib
        // equipment_resolver::tests::the_two_lookups_agree` run.
        //
        // SHRUNK 28 -> 14, `SD31-E6-F6-001`, 2026-08-16 (this fix's own
        // `.COPY=` inheritance in `gen_equipment_gap_tables.rs`, generalized
        // from `OPEN-ISSUES.md` rows 70/103's description recovery to
        // `cost_gp`/`weight_lbs`). The SAME shape the wave-4 correction
        // above already established for its own 8 entries: "Amorphous"
        // etc. agreed because both lookups happened to already carry the
        // real price. These 14 gap rows previously shipped `cost_gp: None`
        // (a `.COPY=` row with no `COST:` token of its own, no inheritance
        // mechanism to recover its base's real value) — `by_key` returned
        // `None` while `equipment_cost_gp_headless_resolve` found the SAME
        // real price via a hand-authored row's NAME match, so the two
        // lookups genuinely disagreed. Now the gap row inherits its base's
        // real `COST:` token (verified one record deep for every removed
        // entry, e.g. `Cold Iron` inherits `COST:0` from
        // `cr_equipmods.lst:109`'s `KEY:Material ~ Cold Iron` row, the
        // SAME row `equipment_cost_gp_headless_resolve`'s CRB-name-match
        // tier already finds) — both lookups now return the IDENTICAL real
        // value, so there is no more pricing ambiguity to disambiguate.
        // Removed: `Adamantine (Ammo)`, `Alchemical Silver`, `BRACE`,
        // `CLOTH`, `Cold Iron`, `DISARM`, `LEATHER`, `MONK`, `Mithral
        // (Light Armor)`, `Mithral (Shield)`, `NONLETHAL`, `STEEL`, `TRIP`,
        // `WOOD` — each individually re-verified via a scratch print
        // (`by_key.cost_gp == headless`, both `Some(<same value>)`) before
        // removal, not assumed from the count alone. The remaining 14 are
        // untouched by this cycle's file territory (no `equipment_gap`
        // record shares their key) and still genuinely disagree.
        //
        // GREW 14 -> 16, `SD31-E6-F10-003` (8 further already-compiled
        // books extended into the gap lane): `"Bullet (Sling/Alchemical)"`
        // (Monster Codex's `KEY:` for its alchemical sling-bullet row,
        // `mc_equip_arms_armor.lst`) and `"Incense (10 sticks)"` (Occult
        // Adventures' `oa_equip.lst`) are the same shape as every entry
        // above -- a corpus gap row's `KEY:` coincidentally matches a
        // string `equipment_cost_gp_headless_resolve`'s free-form,
        // CRB-precedence matching resolves to a DIFFERENT real row. Both
        // records are real, both belong in the catalog, and neither this
        // cycle's file grant nor its own new rows are the wrong side of the
        // disagreement -- verified by direct construction: re-derived this
        // exact list from a fresh `cargo test --lib equipment_resolver::
        // tests::the_two_lookups_agree` run against this cycle's own final
        // state (not copied from an earlier, mid-edit gate run that also
        // transiently showed a THIRD, now-gone name -- `"Bolas (Shoanti)"`,
        // `inner_sea_world_guide`'s own declared-PI-excluded row, correctly
        // absent from the compiled table once `declared_pi_at` landed, so
        // it was never a real disagreement to pin).
        //
        // GREW 16 -> 19, `SD31-E6-F10-004` (5 further already-compiled
        // books extended into the gap lane): `"Feather Token (Catapult)"`,
        // `"Feather Token (Ram)"`, `"Feather Token (Siege Tower)"` --
        // `inner_sea_combat`'s `isc_equip_magic.lst:9-11`, three real, keyed-
        // by-name, priced rows (400/500/1000 gp) whose display name is the
        // same shape as every entry above: a parenthetical-qualified
        // variant name the free-form resolver's CRB-precedence tiers
        // resolve to a DIFFERENT real row (CRB's own base `Feather Token`
        // family) than the catalog's own by-key lookup returns for the
        // SAME string. Both records are real and both belong in the
        // catalog; the ambiguity is only in the free-form string, exactly
        // the shape this test's own doc comment already prescribes the
        // remedy for (resolve by catalog key, never the free-form
        // resolver, when a caller holds one) -- not this cycle's file
        // grant to fix (the free-form matcher lives in this file's own
        // `equipment_cost_gp_headless_resolve`, `equipment_resolver.rs`,
        // outside `cache_gen`/equipment-ingest territory). Re-derived this
        // exact list from a fresh, isolated `cargo test --lib
        // equipment_resolver::tests::the_two_lookups_agree` run against
        // this cycle's own final state.
        //
        // GREW 19 -> 20, SD-32 T9 residual (`decisions.md §20`): the new
        // `adventurers_guide` gap book (see the `rows.len()` assertion
        // above) adds `"Rod (Storm Kindler's)"` (`ag_equip_magic_items.lst`,
        // `AG`) -- same shape as every entry above, a corpus gap row's
        // `KEY:` coincidentally matching a string
        // `equipment_cost_gp_headless_resolve`'s free-form, CRB-precedence
        // matching resolves to a different real `Rod` row. Both records are
        // real; not this cycle's file grant to fix.
        assert_eq!(
            disagreements,
            vec![
                "Adamantine (Heavy Armor)",
                "Adamantine (Light Armor)",
                "Adamantine (Medium Armor)",
                "Adamantine (Weapon)",
                "Backpack (Carrier)",
                "Backpack (Hydration)",
                "Backpack (Weaponrack)",
                "Bullet (Sling/Alchemical)",
                "Feather Token (Catapult)",
                "Feather Token (Ram)",
                "Feather Token (Siege Tower)",
                "Incense (10 sticks)",
                "Mithral (Heavy Armor)",
                "Mithral (Item)",
                "Mithral (Medium Armor)",
                "OBSIDIAN",
                "REACH",
                "ROPE",
                "Rod (Storm Kindler's)",
                "Wooden",
            ],
            "a cross-book identity collision outside this pinned set means a newly ambiguous id \
             that callers must be told how to disambiguate"
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
