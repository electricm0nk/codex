//! SD-19 equipment catalog browser — Tauri command adapter over the full
//! equipment table store of **every ingested PF1 book**: `crb`, `apg`,
//! `acg`, `beastiary1`, `advanced_race_guide` (ARG) and
//! `pathfinder_unchained` (PU).
//!
//! **This adapter served CRB alone until now.** The other five books'
//! equipment tables were fully ingested — with category, cost and (for
//! most) weight and description — but reached no user-facing surface at
//! all: not this catalog browser, not the Character Sheet's Add Equipment
//! picker (which calls `list_equipment`), and so not a character's own
//! gear list either. `reach_gate.rs`'s `OPEN_FINDINGS` recorded that gap
//! for `apg`/`acg`/`beastiary1`/`advanced_race_guide` equipment, and
//! named the remedy verbatim: "widen `build_equipment_catalog` across all
//! books and tag each DTO with its book, exactly the way
//! `spell_catalog.rs` and `feat_catalog.rs` were already widened for this
//! same defect." That is what this module now does.
//!
//! **Per-book types, one shared DTO.** Each book defines its own
//! structurally-similar-but-distinct `EquipmentTableEntry` (and its own
//! `EquipmentCategory`), so this module follows `spell_catalog.rs`'s
//! established precedent: one `map_<book>_entry` function per book,
//! feeding a single DTO that carries a `book` tag.
//!
//! **Two per-book shapes are worth stating plainly, because they are real
//! corpus facts rather than adapter shortcuts:**
//!
//! - `beastiary1` and `apg` expose their tables as a `EQUIPMENT_TABLE`
//!   const rather than an `equipment_tables()` accessor; `acg`, `arg`,
//!   `pu` and `crb` expose the accessor. Both are read here as-is.
//! - `pathfinder_unchained` has **no `EquipmentCategory` at all**. Its
//!   only ingested equipment content is `pu_equipmods.lst` — 42 Automatic
//!   Bonus Progression equipment *modifiers* — so every PU row is mapped
//!   to the `"Equipmods"` category. That is the literal truth of the
//!   source file (see `pathfinder_unchained::equipment_tables`'s own doc
//!   comment), not a placeholder.
//!
//! **Key uniqueness, derived rather than assumed.** An earlier draft of
//! this module asserted that equipment keys collide across books the way
//! item names might be expected to. Derived from the live tables, that is
//! false: there are **zero** cross-book key collisions across all six
//! books. What does exist is 316 keys duplicated *within CRB alone* (e.g.
//! `Holy Symbol (Silver)` twice, and a long run of
//! `Intelligent Item ~ Ability Score / …` rows), a pre-existing property
//! of `crb::equipment_tables` that this widening neither creates nor
//! fixes. Both figures are pinned by tests below so neither can drift
//! silently, and `key` is left untouched rather than book-prefixed
//! because existing callers (`equipment_resolver`, the sheet's Add
//! Equipment flow) resolve by the bare corpus key.
//!
//! Distinct from `character_hub`'s per-character Gear tab: this is a
//! standalone catalog view of every real equipment record the engine
//! knows about, not what one character happens to have equipped.

use serde::{Deserialize, Serialize};

use codex::rules_core::equipment_resolver::equipment_catalog_rows;
use codex::rules_core::rules_tables::{
    acg, advanced_race_guide as arg, apg, beastiary1, crb, pathfinder_unchained as pu,
    ultimate_combat as uc, ultimate_equipment as ue, ultimate_intrigue as ui,
    equipment_gap_tables, ultimate_magic as um, ultimate_psionics as upsi,
};

/// Which ingested book a catalog entry came from. Short codes are the wire
/// form; the frontend maps them to display labels, exactly as
/// `spell_catalog.rs`'s own `BOOK_*` codes do.
const BOOK_CRB: &str = "CRB";
const BOOK_APG: &str = "APG";
const BOOK_ACG: &str = "ACG";
const BOOK_B1: &str = "B1";
const BOOK_ARG: &str = "ARG";
const BOOK_PU: &str = "PU";
const BOOK_UI: &str = "UI";
const BOOK_UE: &str = "UE";
const BOOK_UM: &str = "UM";
const BOOK_UPSI: &str = "UPSI";
const BOOK_UC: &str = "UC";

/// Every book code this catalog can emit, in the order
/// `build_equipment_catalog` emits them.
///
/// **Derived, not restated.** This used to be a hand-maintained literal
/// array — the exact shape that let UE (then UM, then UPsi) go on
/// serving real, priced rows through `equipment_resolver`'s
/// `equipment_catalog_rows()` while remaining invisible to the picker,
/// three separate times, because nobody remembered to append the new
/// code to this second, independent list. Deriving it from the
/// resolver's own row set — the identical structural fix `646aea2b`
/// applied to `v06_work_inventory.rs`'s `equipment_keys` — makes a
/// fourth divergence impossible rather than merely caught by a test:
/// a book landing in the resolver chain appears here automatically, with
/// no second edit to remember. Order is first-appearance in the
/// resolver's own row order, which is the same book order
/// `build_equipment_catalog` below chains in.
pub fn equipment_catalog_books() -> Vec<&'static str> {
    let mut seen = std::collections::BTreeSet::new();
    let mut books = Vec::new();
    for row in equipment_catalog_rows() {
        if seen.insert(row.book) {
            books.push(row.book);
        }
    }
    books
}

/// The category name used for every `pathfinder_unchained` record — see
/// this module's doc comment.
const PU_CATEGORY: &str = "Equipmods";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentCatalogEntryDto {
    /// The record's corpus identity — its `KEY:` token when the row
    /// carries one, else its display name. **Not unique across books**;
    /// see this module's doc comment.
    pub key: String,
    /// The `EquipmentCategory` variant name verbatim (e.g. "ArmsArmor").
    /// Always `"Equipmods"` for `PU`, which has no category enum of its
    /// own.
    pub category: String,
    pub name: String,
    pub cost_gp: Option<f64>,
    /// Which ingested book this record came from: one of
    /// [`equipment_catalog_books`]. Additive field — a consumer that does
    /// not read it is unaffected, and one that does can label or filter
    /// by book the way the Spell Catalog screen already does.
    pub book: String,
    /// The record's own words, from [`row_description`]. `None` where the record
    /// genuinely states none — a real and documented gap for template and bookkeeping
    /// rows, never a fabricated stand-in.
    pub description: Option<String>,
}

/// The description one catalog row serves.
///
/// # What changed, and why
///
/// SD-35 `decisions.md §11` — nothing on the live side reads the ingest format. Until
/// `AT-35-E6-003` cycle 7 this module took the compiled table's stored description string and
/// re-parsed it at run time, because a handful of those strings still carried the source
/// format's own literal-percent escape and positional markers. That run-time parse is the
/// ingest-format reader the ruling removes.
///
/// So: **the converted record's own words first** — the substitution that parse performed
/// already happens at ingest (`src/pcgen_import/sheet_rule/`), and
/// `sheet_rule_catalog::catalog_description` renders the result with no character in hand (a
/// final number where the term is settled, the rule's words where it is not,
/// `decisions.md §1`). A row the converted package holds under no rule at all keeps the words
/// the compiled table states, **but only when that string is already the record's plain
/// words**: a stored string still carrying a `%` marker is refused rather than shown, because
/// the rewriter that used to clean it up is precisely what left the live side. Refusing is the
/// same disposition the rest of this crate takes for a description it cannot serve whole —
/// never a partial or half-rendered sentence.
fn row_description(book: &str, key: &str, table_text: Option<&str>) -> Option<String> {
    if let Some(text) = converted_description(book, key) {
        return Some(text);
    }
    table_text.filter(|text| !text.contains('%')).map(str::to_owned)
}

/// The `data/corpus/<dir>/` directory each equipment book code names.
///
/// Panics on an unmapped code for the same reason `spell_catalog::corpus_book_dir` does: a
/// silently-unmapped book would serve no prose at all for every one of its records, and a
/// screen that shows nothing looks exactly like a book with nothing to show.
fn corpus_book_dir(short_code: &str) -> &'static str {
    corpus_book_dir_opt(short_code).unwrap_or_else(|| {
        panic!(
            "equipment_catalog carries an unmapped book code {short_code:?} -- add it to \
             corpus_book_dir so the catalog does not silently serve no prose"
        )
    })
}

/// [`corpus_book_dir`] without the panic, so the census test below can *report* an unmapped
/// code rather than abort on the first one.
fn corpus_book_dir_opt(short_code: &str) -> Option<&'static str> {
    Some(match short_code {
        BOOK_CRB => "core_rulebook",
        BOOK_APG => "advanced_players_guide",
        BOOK_ACG => "advanced_class_guide",
        BOOK_B1 => "beastiary",
        BOOK_ARG => "advanced_race_guide",
        BOOK_PU => "pathfinder_unchained",
        BOOK_UI => "ultimate_intrigue",
        BOOK_UE => "ultimate_equipment",
        BOOK_UM => "ultimate_magic",
        BOOK_UPSI => "ultimate_psionics",
        BOOK_UC => "ultimate_combat",
        // The corpus-gap lane's rows carry books the compiled tables never did.
        "AG" => "adventurers_guide",
        "B2" => "bestiary_2",
        "B3" => "bestiary_3",
        "B4" => "bestiary_4",
        "BB" => "beginner_box",
        "BOTD2" => "book_of_the_damned_volume_2",
        "HA" => "horror_adventures",
        "ISC" => "inner_sea_combat",
        "ISG" => "inner_sea_gods",
        "ISI" => "inner_sea_intrigue",
        "ISM" => "inner_sea_magic",
        "ISR" => "inner_sea_races",
        "ISTEM" => "inner_sea_temples",
        "ISWG" => "inner_sea_world_guide",
        "MC" => "monster_codex",
        "MYTHIC" => "mythic_adventures",
        "OA" => "occult_adventures",
        "UW" => "ultimate_wilderness",
        _ => return None,
    })
}

/// The two converted kinds an equipment catalog row can be: a piece of equipment, or an
/// equipment modifier. `build_equipment_catalog` chains `equipment_tables()` and
/// `equipmod_tables()` through the same mapper for most books, so a row's kind is not known
/// from the mapper it came through — both are asked, equipment first.
const EQUIPMENT_KINDS: [&str; 2] = ["equipment", "equipment_modifier"];

/// The prose this catalog serves for one record: **the converted record's own words**, never
/// the compiled table's stored description string.
///
/// SD-35 `decisions.md §11` — nothing on the live side reads the ingest format. The
/// substitution this call site used to perform at run time already happens at ingest
/// (`src/pcgen_import/sheet_rule/`), and `sheet_rule_catalog::catalog_description` renders the
/// converted record with no character in hand: a final number where the term is settled, the
/// rule's own words where it is not (`decisions.md §1`'s three printed forms).
fn converted_description(book: &str, key: &str) -> Option<String> {
    let dir = corpus_book_dir(book);
    EQUIPMENT_KINDS.iter().find_map(|kind| crate::converted_prose::description_for(dir, kind, key))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentCatalogResponse {
    pub entries: Vec<EquipmentCatalogEntryDto>,
}

fn map_crb_entry(entry: &crb::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_CRB.to_string(),
        description: row_description(BOOK_CRB, entry.key, entry.description),
    }
}

fn map_apg_entry(entry: &apg::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_APG.to_string(),
        description: row_description(BOOK_APG, entry.key, entry.description),
    }
}

fn map_acg_entry(entry: &acg::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_ACG.to_string(),
        description: row_description(BOOK_ACG, entry.key, entry.description),
    }
}

fn map_beastiary1_entry(
    entry: &beastiary1::equipment_tables::EquipmentTableEntry,
) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_B1.to_string(),
        description: row_description(BOOK_B1, entry.key, entry.description),
    }
}

fn map_arg_entry(entry: &arg::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_ARG.to_string(),
        description: row_description(BOOK_ARG, entry.key, entry.description),
    }
}

/// PU's entry type carries no `EquipmentCategory` and no `cost_gp` field
/// at all: `pu_equipmods.lst` has zero `COST:` tokens anywhere (its real
/// cost signal is a crafting-cost formula rather than a flat gp number),
/// so `cost_gp` is honestly `None` for all 42 records rather than a
/// fabricated `Some(0.0)`.
fn map_pu_entry(entry: &pu::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: PU_CATEGORY.to_string(),
        name: entry.name.to_string(),
        cost_gp: None,
        book: BOOK_PU.to_string(),
        description: row_description(BOOK_PU, entry.key, entry.description),
    }
}

/// UI's entry type reuses ARG's own shape exactly (own `EquipmentCategory`
/// enum, `description` sourced from `SPROP:` -- see
/// `ultimate_intrigue::equipment_tables`'s own doc comment). Both
/// `equipment_tables()` (91 records) and `equipmod_tables()` (7 records,
/// the honest count after excluding `ui_equipmods.lst`'s `VISIBLE:NO`
/// alias rows -- see that function's own doc comment) are served under
/// the same `BOOK_UI` code, mirroring how CRB/APG/ACG/ARG/PU each serve
/// their own equipment-modifier records alongside their regular equipment
/// under one book code rather than a separate one.
fn map_ui_entry(entry: &ui::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_UI.to_string(),
        description: row_description(BOOK_UI, entry.key, entry.description),
    }
}

/// UE's entry type reuses UI's own shape exactly (own `EquipmentCategory`
/// enum, description joining the record's descriptive and special-property text -- see
/// `ultimate_equipment::equipment_tables`'s own doc comment). Both
/// `equipment_tables()` (1,380 records) and `equipmod_tables()` (180
/// records) are served under the same `BOOK_UE` code.
fn map_ue_entry(entry: &ue::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_UE.to_string(),
        description: row_description(BOOK_UE, entry.key, entry.description),
    }
}

/// UM's entry type reuses UE/UI's own shape exactly (own `EquipmentCategory`
/// enum, `description` sourced the same way). `equipment_tables()` (24
/// General pregenerated spellbooks + 2 ArmsArmor Scrollmaster Gear rows =
/// 26 records) and `equipmod_tables()` (a real, permanently-empty slice --
/// no equipment-modifier file exists for this book, see that module's own
/// doc comment) are both chained under `BOOK_UM`, mirroring UI/UE's own
/// choice to serve equipment and equipmods under one book code. Structural
/// gap this closes: `equipment_resolver.rs`'s headless pricing/recognition
/// chain (`§55`, extended in the UM/UPsi landing decisions) already carried
/// this book; the picker's own independent, hand-maintained book chain had
/// not, so a genuinely purchasable-by-price item was still absent from the
/// Add Equipment / Equipment Catalog screens.
fn map_um_entry(entry: &um::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_UM.to_string(),
        description: row_description(BOOK_UM, entry.key, entry.description),
    }
}

/// UPsi's entry type reuses UE/UI's own shape exactly. `equipment_tables()`
/// (326 records) and `equipmod_tables()` (113 records -- the correct,
/// `VISIBLE:NO` `.COPY=` legacy-alias-excluded count; see
/// `ultimate_psionics::equipment_tables`'s own doc comment for the
/// reconciliation) are both chained under `BOOK_UPSI`, same reasoning as
/// UM above.
fn map_upsi_entry(entry: &upsi::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_UPSI.to_string(),
        description: row_description(BOOK_UPSI, entry.key, entry.description),
    }
}

/// UC's entry type reuses UE/UI/UM/UPsi's own shape exactly.
/// `equipment_tables()` (185 records: General + MagicItems + ArmsArmor)
/// and `equipmod_tables()` (19 records -- the correct, `VISIBLE:NO`
/// `.COPY=` legacy-alias-excluded count, the same reconciliation UPsi's
/// own table required; see `ultimate_combat::equipment_tables`'s own doc
/// comment) are both chained under `BOOK_UC`.
fn map_uc_entry(entry: &uc::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_UC.to_string(),
        description: row_description(BOOK_UC, entry.key, entry.description),
    }
}

/// One corpus-recovered gap row. Unlike the per-book maps above, this row
/// already carries its own `book` code and category string — the generated
/// table is one flat shape across every book, so there is one mapper rather
/// than nine.
fn map_gap_entry(
    row: &equipment_gap_tables::EquipmentGapRow,
) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: row.key.to_string(),
        category: row.category.to_string(),
        name: row.name.to_string(),
        cost_gp: row.cost_gp,
        book: row.book.to_string(),
        description: row_description(row.book, row.key, row.description),
    }
}

/// Build the full catalog response across every ingested book. A thin,
/// testable wrapper behind the Tauri command below (mirroring this
/// codebase's other command/pure-fn split, e.g.
/// `authoring_workbench::build_authoring_workbench_snapshot`).
pub fn build_equipment_catalog() -> EquipmentCatalogResponse {
    let entries = crb::equipment_tables::equipment_tables()
        .iter()
        .map(map_crb_entry)
        .chain(apg::equipment_tables::EQUIPMENT_TABLE.iter().map(map_apg_entry))
        .chain(acg::equipment_tables::equipment_tables().iter().map(map_acg_entry))
        .chain(
            beastiary1::equipment_tables::EQUIPMENT_TABLE
                .iter()
                .map(map_beastiary1_entry),
        )
        .chain(arg::equipment_tables::equipment_tables().iter().map(map_arg_entry))
        .chain(pu::equipment_tables::equipment_tables().iter().map(map_pu_entry))
        .chain(ui::equipment_tables::equipment_tables().iter().map(map_ui_entry))
        .chain(ui::equipment_tables::equipmod_tables().iter().map(map_ui_entry))
        .chain(ue::equipment_tables::equipment_tables().iter().map(map_ue_entry))
        .chain(ue::equipment_tables::equipmod_tables().iter().map(map_ue_entry))
        .chain(um::equipment_tables::equipment_tables().iter().map(map_um_entry))
        .chain(um::equipment_tables::equipmod_tables().iter().map(map_um_entry))
        .chain(upsi::equipment_tables::equipment_tables().iter().map(map_upsi_entry))
        .chain(upsi::equipment_tables::equipmod_tables().iter().map(map_upsi_entry))
        .chain(uc::equipment_tables::equipment_tables().iter().map(map_uc_entry))
        .chain(uc::equipment_tables::equipmod_tables().iter().map(map_uc_entry))
        // The corpus gap lane (`epic-4-proven-equip-mod`): every equipment /
        // equipment-modifier record that belongs to one of these already-
        // compiled books and that no hand-authored table holds. These reach
        // the player through exactly this chain — the picker, the catalog
        // screen and `list_equipment` all read this one response — so the
        // rows are surfaced, not merely resolvable.
        .chain(equipment_gap_tables::equipment_gap_rows().map(map_gap_entry))
        .collect();

    EquipmentCatalogResponse { entries }
}

#[tauri::command]
pub fn list_equipment_catalog() -> EquipmentCatalogResponse {
    build_equipment_catalog()
}

/// Filter criteria for `list_equipment`. Every field is optional and
/// `None`/empty matches everything — an all-`None` filter is equivalent to
/// the unfiltered `list_equipment_catalog` response. Kept deliberately
/// narrow (substring name match, exact category match, exact book match)
/// rather than an exhaustive query DSL; widen only if a real caller needs
/// more.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentCatalogFilter {
    /// Case-insensitive substring match against `name`.
    pub name_contains: Option<String>,
    /// Exact match against the `EquipmentCategory` variant name verbatim
    /// (e.g. "ArmsArmor"), as projected onto `EquipmentCatalogEntryDto::category`.
    pub category: Option<String>,
    /// Exact match against a book code in [`equipment_catalog_books`]
    /// (e.g. "APG"). Omitted/`None` spans every book, so an existing
    /// caller that never sends this field is unaffected — the same
    /// additive shape `SpellCatalogFilter::book` already uses.
    pub book: Option<String>,
}

/// Narrows the full catalog to the entries matching `filter`. A thin,
/// testable wrapper behind the `list_equipment` Tauri command below,
/// mirroring `build_equipment_catalog`'s own command/pure-fn split.
pub fn filter_equipment_catalog(filter: &EquipmentCatalogFilter) -> EquipmentCatalogResponse {
    let name_needle = filter
        .name_contains
        .as_ref()
        .filter(|needle| !needle.is_empty())
        .map(|needle| needle.to_lowercase());

    let entries = build_equipment_catalog()
        .entries
        .into_iter()
        .filter(|entry| match &name_needle {
            Some(needle) => entry.name.to_lowercase().contains(needle.as_str()),
            None => true,
        })
        .filter(|entry| match &filter.category {
            Some(category) => &entry.category == category,
            None => true,
        })
        .filter(|entry| match &filter.book {
            Some(book) => &entry.book == book,
            None => true,
        })
        .collect();

    EquipmentCatalogResponse { entries }
}

/// Returns the full cross-book equipment catalog narrowed by `filter` —
/// see `EquipmentCatalogFilter`'s own doc comment for the supported
/// fields. Distinct from `list_equipment_catalog` (kept unfiltered so the
/// existing `loadEquipmentCatalog` desktop boundary caller is untouched);
/// this command is the filtered surface Criterion 19 asks for.
#[tauri::command]
pub fn list_equipment(filter: EquipmentCatalogFilter) -> EquipmentCatalogResponse {
    filter_equipment_catalog(&filter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    /// **The corpus-wide floor under the equipment catalog's prose.**
    ///
    /// SD-35 `AT-35-E6-003` cycle 7 swapped this catalog's description source from the
    /// compiled tables' stored strings (re-parsed at run time) to the converted package
    /// (`row_description`). A ratchet over the whole live corpus, never a fixture with a
    /// hand-derived value (`decisions.md §4`): if a converter change or a package
    /// regeneration takes prose away from this screen, this fails.
    ///
    /// Counts stated here were derived by running this test, not assumed.
    #[test]
    fn converted_equipment_prose_population() {
        let entries = build_equipment_catalog().entries;
        let described = entries.iter().filter(|e| e.description.is_some()).count();
        println!("equipment rows served={} with a description={described}", entries.len());
        let mut by_book: BTreeMap<String, usize> = BTreeMap::new();
        for entry in entries.iter().filter(|e| e.description.is_some()) {
            *by_book.entry(entry.book.clone()).or_default() += 1;
        }
        println!("described by book: {by_book:?}");
        assert_eq!(
            entries.len(),
            8119,
            "the equipment catalog's served-row count moved; re-derive the floor below before \
             changing it"
        );
        assert!(
            described >= 5389,
            "the equipment catalog serves {described} descriptions, below the floor of 5389"
        );
        for entry in &entries {
            assert!(
                corpus_book_dir_opt(&entry.book).is_some(),
                "{}: book code {:?} is not mapped to a corpus directory, so every one of its \
                 rows silently serves no converted prose",
                entry.key,
                entry.book
            );
        }
    }

    /// **One guard over every catalog, rather than one guard per catalog.**
    ///
    /// `spell_catalog.rs` already carried a per-catalog version of this test,
    /// and it caught nothing outside spells — which is exactly how equipment
    /// descriptions kept 54 raw `%%` escapes and the Add Feat picker kept 16
    /// leaking feat descriptions. Five per-catalog guards leave the sixth
    /// catalog unguarded; this one fails the moment *any* description-bearing
    /// surface serves PCGen syntax, so a new catalog is covered by existing
    /// code rather than by someone remembering to add a test.
    ///
    /// **Every description-bearing catalog surface in the desktop app is
    /// enumerated here.** Derived by reading each `*_catalog.rs`/picker DTO,
    /// not assumed:
    ///
    /// | module | prose field(s) | covered |
    /// |---|---|---|
    /// | `spell_catalog` | `description` | yes |
    /// | `feat_catalog` | `description` | yes |
    /// | `equipment_catalog` | `description` | yes |
    /// | `race_catalog` | `detail` (the trait's own words), `trait_name` | yes |
    /// | `race_trait_picker` | alternates' + standard traits' `description` | yes |
    /// | `class_catalog` | none — the DTO is `classId` plus five integers | n/a |
    /// | `monster_catalog` | none — no description field on the DTO | n/a |
    ///
    /// **Two monster-catalog strings are deliberately out of this guard's
    /// scope, and neither is a description:**
    ///
    /// * `NaturalAttackDto::grounding_note` quotes real corpus tokens inside
    ///   backticks on purpose (``ABILITY:Internal|AUTOMATIC|Bite``) — it is
    ///   provenance for a reader re-checking a transcription, and rendering
    ///   the quoted token would destroy the thing it exists to show. 14 notes.
    /// * `MonsterCatalogEntryDto::race_subtype` serves the `RACESUBTYPE:`
    ///   token verbatim, and 2 rows are multi-valued: Vargouille
    ///   `"Evil|Extraplanar"` and Hell Hound `"Evil|Extraplanar|Fire|Lawful"`.
    ///   That **is** a raw PCGen separator reaching a player, but it is a
    ///   structured field rather than a rendered description, its fix is to join the
    ///   values for display, and `monster_catalog.rs` is outside this change's
    ///   write scope. Recorded here so it is a known open finding rather than
    ///   a silent omission.
    #[test]
    fn no_catalog_serves_a_description_carrying_raw_pcgen_syntax() {
        use codex::pcgen_import::pcgen_desc::leaked_pcgen_syntax;

        let mut checked = 0usize;
        let mut leaks: Vec<String> = Vec::new();
        let mut check = |surface: &str, identity: &str, text: &str| {
            checked += 1;
            if let Some(leak) = leaked_pcgen_syntax(text) {
                leaks.push(format!("{surface} {identity}: {leak} in {text:?}"));
            }
        };

        for entry in &crate::spell_catalog::build_spell_catalog().entries {
            if let Some(description) = entry.description.as_deref() {
                check("spell", &entry.key, description);
            }
        }
        for entry in &crate::feat_catalog::build_feat_catalog().entries {
            if let Some(description) = entry.description.as_deref() {
                check("feat", &entry.key, description);
            }
        }
        for entry in &build_equipment_catalog().entries {
            if let Some(description) = entry.description.as_deref() {
                check("equipment", &entry.key, description);
            }
        }

        // The two corpus-backed surfaces report their own read failures. A
        // diagnostic here would mean the guard is inspecting a shrunken
        // catalog, so it fails rather than passing on less than it claims.
        let races = crate::race_catalog::build_race_catalog();
        assert!(races.diagnostics.is_empty(), "race catalog diagnostics: {:?}", races.diagnostics);
        for entry in &races.entries {
            let identity = format!("{}/{}", entry.race_id, entry.trait_name);
            check("race.detail", &identity, &entry.detail);
            check("race.traitName", &identity, &entry.trait_name);
        }

        let picker = crate::race_trait_picker::build_alternate_racial_traits();
        assert!(
            picker.diagnostics.is_empty(),
            "race trait picker diagnostics: {:?}",
            picker.diagnostics
        );
        for race in &picker.races {
            for trait_dto in &race.alternates {
                check("raceTrait.alternate", &trait_dto.key, &trait_dto.description);
            }
            for trait_dto in &race.standard_traits {
                check("raceTrait.standard", &trait_dto.key, &trait_dto.description);
            }
        }

        assert!(leaks.is_empty(), "catalogs serving raw PCGen syntax ({}): {leaks:#?}", leaks.len());
        // A guard that silently stopped reading anything would also report
        // zero leaks, so the reach is pinned as a floor. 5394 strings,
        // derived by printing each surface's own tally rather than assumed:
        // 1185 spell + 681 feat + 2856 equipment descriptions, 2x173 race
        // (detail and traitName), and 326 race-trait descriptions (153
        // alternates + 173 standard). The catalogs may grow; they cannot
        // quietly empty out beneath the guard.
        assert!(
            checked >= 5394,
            "the guard inspected only {checked} descriptions; it is no longer covering the \
             catalogs it claims to"
        );
    }

    /// The 58-record raw-syntax leak (54 literal-percent escapes + 4 unresolved choice
    /// markers), pinned on both
    /// sides of the render so the fix cannot be mistaken for the corpus
    /// having changed.
    ///
    /// **Widened `SD31-W6-INTEGRATE-001`**: `leaked_pcgen_syntax` originally
    /// only flagged `%%`/`%<digit>`; it was widened to also flag
    /// an unresolved choice marker after this test's own sibling
    /// (`no_catalog_serves_a_description_carrying_raw_pcgen_syntax`) caught
    /// the equipment catalog serving that marker verbatim to a player. That
    /// widening makes THIS test's raw-side scan see 4 more real occurrences
    /// it always contained but never counted (ACG's `Equipmods` category:
    /// Blood-Hunting/Spirit-Hunting Weapon + Amulet of Mighty Fists, all
    /// four `+2 enhancement... against <an unmade choice> bloodline/mystery` shaped) --
    /// the run-time renderer of the day was widened in the SAME cycle to drop an
    /// unresolved `%<KEYWORD>` the same no-fabrication way it already drops
    /// an unresolved `%N` (there is no `PcgenDisplayValues` slot for a
    /// chargen-time player selection like a bloodline choice), so the
    /// SERVED side stays 0 either way.
    ///
    /// The compiled tables still hold the raw escape — that is correct, they
    /// are a transcription of the corpus — and the catalog is what must not
    /// pass it on. Per book and category, derived by running
    /// `leaked_pcgen_syntax` over both sides:
    ///
    /// | book | category | raw table | served |
    /// |---|---|---:|---:|
    /// | CRB | MagicItems | 41 | 0 |
    /// | CRB | General | 6 | 0 |
    /// | CRB | ArmsArmor | 6 | 0 |
    /// | ARG | ArmsArmor | 1 | 0 |
    /// | ACG | Equipmods | 4 | 0 |
    /// | APG / B1 / PU | all | 0 | 0 |
    #[test]
    fn the_raw_percent_escape_stops_at_the_catalog_boundary() {
        use codex::pcgen_import::pcgen_desc::leaked_pcgen_syntax;

        let mut raw_leaks: BTreeMap<(&str, String), usize> = BTreeMap::new();
        let mut count_raw = |book: &'static str, category: String, description: Option<&str>| {
            if let Some(text) = description {
                if leaked_pcgen_syntax(text).is_some() {
                    *raw_leaks.entry((book, category)).or_default() += 1;
                }
            }
        };
        for entry in crb::equipment_tables::equipment_tables() {
            count_raw("CRB", format!("{:?}", entry.category), entry.description);
        }
        for entry in apg::equipment_tables::EQUIPMENT_TABLE {
            count_raw("APG", format!("{:?}", entry.category), entry.description);
        }
        for entry in acg::equipment_tables::equipment_tables() {
            count_raw("ACG", format!("{:?}", entry.category), entry.description);
        }
        for entry in beastiary1::equipment_tables::EQUIPMENT_TABLE {
            count_raw("B1", format!("{:?}", entry.category), entry.description);
        }
        for entry in arg::equipment_tables::equipment_tables() {
            count_raw("ARG", format!("{:?}", entry.category), entry.description);
        }
        for entry in pu::equipment_tables::equipment_tables() {
            count_raw("PU", PU_CATEGORY.to_owned(), entry.description);
        }

        let expected: BTreeMap<(&str, String), usize> = [
            (("CRB", "MagicItems".to_owned()), 41),
            (("CRB", "General".to_owned()), 6),
            (("CRB", "ArmsArmor".to_owned()), 6),
            (("ARG", "ArmsArmor".to_owned()), 1),
            // `("ACG", "Equipmods") => 4` is GONE, and its removal is a fix
            // landing, not a loss. SD-35 `AT-35-E6-003-SWEEP` cycle 5
            // (`6fe6131922`) moved `gen_equipment_gap_tables.rs::
            // safe_description` from *rendering a description only to decide
            // whether to keep it, then storing the raw one* to storing the
            // rendered text. ACG's four Equipmods rows were exactly that case:
            // their stored strings leaked a raw `%` hole onto the sheet and so
            // were also refused a description downstream. They now carry
            // rendered prose, which is why this bucket empties and
            // `with_description("ACG")` rises by the same 4 below -- the two
            // assertions move together, which is what says the movement is one
            // real change rather than two unrelated drifts.
            //
            // Attributed by bisection over the desktop crate, not assumed:
            // green at `6fe6131922~1` (`b069f01962`), red at `6fe6131922`, red
            // at every tree since, including this cycle's start `94b4db5306`.
            // It sat red from cycle 5 to cycle 15 because `apps/` is on epic
            // cadence and no cycle between them touched it, so no cycle ran
            // this crate. Re-derive:
            //   git worktree add /tmp/wt <sha> && cd /tmp/wt/apps/desktop/src-tauri
            //   cargo test --locked -j 4 equipment_catalog
            // SD31-W8-INTEGRATE-001: `leaked_pcgen_syntax` widened to catch
            // a bare '%' hole neither a digit nor an uppercase keyword
            // follows (wave-8 adversarial review). This surfaced ONE
            // pre-existing, previously-invisible hand-authored leak:
            // ACG's "Gloves of Marking" (`equipment_data/magic_items.rs`)
            // reads "...must save (Will DC %) or be shaken..." -- a
            // literal unfilled DC placeholder that predates every wave-8
            // lane and is untouched by any of them. Pinned here, not
            // fabricated a value for: this repo's own no-stub doctrine
            // forbids inventing the missing DC, and the source book text
            // is not available to re-derive it from.
            (("ACG", "MagicItems".to_owned()), 1),
        ]
        .into_iter()
        .collect();
        assert_eq!(raw_leaks, expected, "the raw tables' own leak profile");
        // 59 -> 55: the four ACG Equipmods rows named above. Re-derived from
        // the assertion's own `left`, not adjusted by delta.
        assert_eq!(raw_leaks.values().sum::<usize>(), 55);

        let served_leaks: Vec<&str> = build_equipment_catalog()
            .entries
            .iter()
            .filter(|entry| {
                entry
                    .description
                    .as_deref()
                    .is_some_and(|d| leaked_pcgen_syntax(d).is_some())
            })
            .map(|_| "leak")
            .collect();
        assert!(served_leaks.is_empty(), "{} served equipment descriptions still leak", served_leaks.len());
    }

    /// The record the defect was reported against, asserted as the whole
    /// sentence a player reads rather than as "does not contain `%%`" — a
    /// containment check would also pass if the renderer had eaten the
    /// number along with the escape.
    #[test]
    fn helmet_dwarven_boulder_reads_as_prose() {
        let helmet = build_equipment_catalog()
            .entries
            .into_iter()
            .find(|entry| entry.key == "Helmet (Dwarven Boulder)")
            .expect("ARG's Dwarven Boulder Helmet is in the catalog");
        assert_eq!(helmet.book, "ARG");

        let description = helmet.description.expect("the ARG record states its own words");
        assert!(
            description.contains(
                "A dwarven boulder helmet adds 20% to the wearer's arcane spell failure chance."
            ),
            "the arcane-spell-failure sentence must read as prose: {description}"
        );
        assert_eq!(
            description,
            "This heavy, reinforced helmet can be used to make melee attacks. The wearer may \
             also use the helmet when attempting bull rush maneuvers, granting a +2 circumstance \
             bonus on the check, but after completing the maneuver (whether successful or not), \
             the wearer is staggered until the end of his next turn. In addition, the helmet \
             grants a +2 circumstance bonus to the wearer's AC against critical hit confirmation \
             rolls. A dwarven boulder helmet adds 20% to the wearer's arcane spell failure \
             chance. It occupies the head slot and is made of metal, not stone, meaning that it \
             can be crafted from unusual materials as a metal weapon. A dwarven boulder helmet \
             can be enchanted as a weapon (not as armor, despite providing some protection)."
        );
    }

    /// How many records actually carry description text, per book. Pinned
    /// because `description` is `Option` and a mapper silently handing out
    /// `None` everywhere would pass every leak assertion above.
    #[test]
    fn description_coverage_is_pinned_per_book() {
        let response = build_equipment_catalog();
        let with_description = |book: &str| {
            response
                .entries
                .iter()
                .filter(|e| e.book == book && e.description.is_some())
                .count()
        };

        // Real corpus coverage, not a target: CRB and ARG carry template and
        // bookkeeping rows that state no description at all, and that gap is
        // documented on `crb::equipment_tables::EquipmentTableEntry`.
        //
        // RAISED `SD31-E6-F6-001`, 2026-08-16: `gen_equipment_gap_tables.rs`
        // gained copy-row inheritance (a copy row with no descriptive or
        // `SPROP:` of its own now inherits its base record's real one) --
        // every book whose gap-lane rows include `.COPY=` variants gained
        // real, corpus-true descriptions that were previously `None` purely
        // because the parser never looked at the base row. One newly-
        // recovered description (`CRB IntItemBase`) was refused rather than
        // shipped: its base's `SPROP:` states 4 bare (unnumbered) `%`
        // placeholders with a 4-argument tail the run-time renderer's
        // numbered-reference detection does not resolve, so `gen_equipment_
        // gap_tables.rs`'s own `safe_description` gate (reusing this exact
        // module's `leaked_pcgen_syntax` check) ships `None` instead of
        // broken syntax -- see `no_catalog_serves_a_description_carrying_
        // raw_pcgen_syntax` immediately below, which is what caught it.
        // Books untouched by the fix (`B1`, `PU`, `UE`, `UM`) are unpinned-
        // changed; `UW`'s own 2 recovered fields were `weight_lbs`, not
        // `description`, so its count is unchanged too. Every figure below
        // re-derived fresh from the catalog itself, not adjusted by delta.
        // `SD31-E6-F10-002`: `Poison (Violet Venom)` (has a description)
        // moved CRB -> B1 (`decisions.md §9`); `Rock (Small)`/`Rock
        // (Medium)` (no description) moved the same way. 2219 - 1 = 2218;
        // 4 + 1 = 5.
        // 2647 -> 2648, SD-35 `AT-35-E6-003-SWEEP` cycle 17, and the +1 is one record, named:
        // cycle 16's `description` fallback gave `core_rulebook:equipment:staff_of_the_magi` the
        // book's own sentence, and cycle 17 collapsed the `%%` literal-percent escape that
        // sentence carried (`text_stat`, `printable_description`), so it stops being refused at
        // this catalog's own `safe_description` gate. Re-derive with
        // `cargo test --locked --bin codex-desktop description_coverage_is_pinned_per_book`.
        // Cycle 17 note: these pins are COLLECTED and reported together, not asserted one at
        // a time. A per-book `assert_eq!` stops at the first stale pin and hides every one
        // behind it — cycle 17 fixed `CRB` and only then learned `APG` had been stale too,
        // at the cost of a twenty-minute crate run per book. `AGENTS.md`'s concurrency
        // section asks for exactly this: attribute every failure and name each, rather than
        // discover them one run at a time.
        let mut pinned: Vec<(&str, usize)> = Vec::new();
        pinned.push(("CRB", 2648));
        // 368 -> 374, and `UE` 573 -> 586: cycle 16's `description` fallback
        // (`dca9c80fe3`, 239 equipment + 44 equipment_modifier rule files) gave these rows the
        // book's own sentence, and `catalog_description` now has words to return for them.
        // Not cycle 17's escape fix, which touched four `core_rulebook` files only. Both pins
        // had been stale since cycle 16, which did not run this crate; both surfaced in one
        // run because the pins are collected rather than asserted one at a time.
        pinned.push(("APG", 374));
        // 307 -> 311, SD-35 `AT-35-E6-003-SWEEP` cycle 5 (`6fe6131922`),
        // pinned here by cycle 15, the first run of this crate since. The four
        // are ACG's Equipmods rows whose stored string used to leak a raw `%`
        // and was therefore refused; `safe_description` now stores the rendered
        // text and they serve real prose. Same four rows as the
        // `("ACG", "Equipmods")` bucket that disappears from
        // `the_raw_percent_escape_stops_at_the_catalog_boundary` above.
        // Re-derived from the built catalog itself (the assertion's own
        // `left`), not adjusted by delta.
        pinned.push(("ACG", 311));
        pinned.push(("B1", 5));
        pinned.push(("ARG", 205));
        pinned.push(("PU", 42));
        pinned.push(("UI", 48));
        pinned.push(("UE", 586));
        // 24 of UM's 26 (both Scrollmaster Gear ArmsArmor rows carry no
        // description; all 24 General spellbooks do).
        pinned.push(("UM", 24));
        // 403 -> 404, SD-35 `AT-35-E6-003` cycle 10. Re-derived from the
        // built catalog itself (the assertion's own `left`), not adjusted by
        // delta. This is the first run of the desktop crate since cycle 7
        // swapped this catalog's description source from the compiled
        // table's stored string to the converted package -- the crate is a
        // separate cargo workspace, so the root `cargo test` never builds it
        // and `workflow-instruction.md` §6 runs it only for a cycle that
        // touches `apps/`. Cycles 8 and 9 regenerated `data/sheet_rules/`
        // and did not. One further Ultimate Psionics row therefore reaches a
        // player with its own words; verified NOT to be this cycle's own
        // converter change, which altered exactly one field (`print`, 527
        // times) and no rule's prose or id -- `catalog_description` does not
        // read `print`.
        pinned.push(("UPSI", 404));
        // Most ArmsArmor rows (ammunition, armor, plain weapons) carry no
        // `SPROP:` token at all, matching every other book's own
        // weapon-heavy shortfall.
        pinned.push(("UC", 105));
        // UW reaches this catalog only through the corpus gap lane; 57 of its
        // 127 rows state real descriptive or special-property text.
        pinned.push(("UW", 56));
        // `SD31-E6-F10-003`: 8 further already-compiled books (`OA`, `HA`,
        // `ISR`, `ISWG`, `MC`, `B2`, `B3`, `B4`) extended into the corpus
        // gap lane -- same "no hand-authored table, every row from the gap
        // lane" shape as `UW` above. Re-derived fresh from the built
        // catalog, not adjusted by delta: 4235 + 195 = 4430 (`declared_pi_at`'s
        // own fix in `gen_equipment_gap_tables.rs` redacts/excludes 4 fewer
        // than the earlier, pre-fix intermediate count -- re-derived fresh,
        // not hand-adjusted, after that fix landed).
        // `SD31-E6-F10-004`: 5 further already-compiled books extended into
        // the corpus gap lane -- same shape. Per-book, re-derived fresh from
        // the built catalog: `ISG` 72/125, `MYTHIC` 97/252 (most mythic
        // items are `.MOD`/`NAMEISPI` rows or bare stat-boost items with no
        // descriptive or special-property text), `ISC` 7/65, `ISI` 9/34, `BOTD2` 3/5.
        // SD-32 `decisions.md §24` re-derivation (`t9-onboarding-unowned-
        // reds`): `ISG`'s 25 newly-included neutral-named rows contribute
        // 25 more real descriptions (72 -> 97); `ISI`'s 8 newly-included
        // rows contribute 3 more (9 -> 12); `BOTD2`'s 1 newly-included row
        // contributes 1 more (3 -> 4). `MYTHIC`'s and `ISC`'s newly-
        // included rows state no descriptive or special-property text of their own, so
        // their description counts are unchanged.
        pinned.push(("ISG", 139));
        pinned.push(("MYTHIC", 116));
        pinned.push(("ISC", 7));
        pinned.push(("ISI", 12));
        pinned.push(("BOTD2", 4));
        // 4430 + 188 (72 + 97 + 7 + 9 + 3) = 4618.
        // SD-32 T9 onboarding (card 11): `ISTEM` 33/43, `ISM` 4/6 --
        // re-derived directly against the generated `equipment_gap_tables.rs`
        // (counting non-`None` `description` fields, not hand-adjusted).
        pinned.push(("ISTEM", 33));
        // SD-32 T9 residual (`decisions.md §20`): `ISM` 4 -> 54.
        // `cache_gen::equipment_gap::book_routing` had no arm for `"ISM"`
        // at all (fixed) and `ism_equipmods.lst` regained its citations on
        // a stale exclusion (fixed) -- ISM's row count itself grew 6 -> 68,
        // and 54 of those 68 state real descriptive or special-property text,
        // re-derived directly against the regenerated
        // `equipment_gap_tables.rs`.
        pinned.push(("ISM", 54));
        // SD-32 T9 residual: the new `AG` book (`adventurers_guide`, no
        // corpus gap config at all before this cycle) -- 14 of its 97 rows
        // carry a real description, re-derived directly against the
        // generated table.
        // SD-32 `decisions.md §24`/T9 residual re-derivation: `AG`'s
        // newly-included rows (97 -> 116 total, see `catalog_spans_every_
        // ingested_book_with_their_real_counts`) contribute 4 more real
        // descriptions (14 -> 18).
        pinned.push(("AG", 19));
        // SD-32 desktop count re-sweep: `BB` (`beginner_box`) -- 13 of its
        // 19 rows state real descriptive or special-property text (6 `description:
        // None`), re-derived directly against the regenerated
        // `equipment_gap_tables.rs`.
        pinned.push(("BB", 15));
        // Re-derived fresh this cycle (`sd32-desktop-count-resweep`) as the
        // real, measured total -- not the old 4719 plus a hand-adjusted
        // delta, because `OA`/`HA`/`ISR`/`ISWG`/`MC`/`B2`/`B3`/`B4` are not
        // individually pinned above and their own description counts moved
        // too (their ROW counts drifted in `catalog_spans_every_ingested_
        // book_with_their_real_counts` above, and some of that growth
        // states real descriptive or special-property text). 4756 -> 4769 (+13, `BB`
        // above). Command: `cd apps/desktop/src-tauri && cargo test
        // --locked --bin codex-desktop equipment_catalog -- --nocapture`
        // with a temporary per-book description-count dump.
        // 5389 -> 5390, SD-35 `AT-35-E6-003` cycle 10: the SAME single row
        // as the `UPSI` pin above, and the only one -- every other book's
        // per-book count is unchanged, which is what makes the total's +1 a
        // confirmation of that story rather than a second, unattributed
        // move. Re-derived from the built catalog (the assertion's own
        // `left`).
        // 5390 -> 5394, SD-35 `AT-35-E6-003-SWEEP` cycle 5 (`6fe6131922`),
        // pinned by cycle 15. The whole +4 is ACG's four Equipmods rows named
        // in the per-book pin above and in the leak profile; no other book's
        // per-book count moved, which is what makes the total's +4 a
        // confirmation of that one story rather than a second, unattributed
        // move. Re-derived from the built catalog (the assertion's own `left`).

        let mut stale: Vec<String> = Vec::new();
        for (book, expected) in &pinned {
            let actual = with_description(book);
            if actual != *expected {
                stale.push(format!("{book}: pinned {expected}, catalog says {actual}"));
            }
        }
        assert!(stale.is_empty(), "per-book description pins are stale:\n  {}", stale.join("\n  "));
        // 5394 -> 5414, SD-35 `AT-35-E6-003-SWEEP` cycle 17, and the +20 is fully attributed
        // by the per-book pins above rather than left as a bare total: `CRB` +1 (this cycle's
        // `%%` escape collapse freeing `staff_of_the_magi`), `APG` +6 and `UE` +13 (cycle 16's
        // `description` fallback). 1 + 6 + 13 = 20, and no other book's per-book count moved,
        // which is what makes the total a confirmation of those two stories rather than a
        // third, unattributed one. Re-derived from the built catalog (the assertion's own
        // `left`).
        assert_eq!(
            response.entries.iter().filter(|e| e.description.is_some()).count(),
            5414
        );
    }

    /// Every count in this module's tests was derived, never assumed, by
    /// running the catalog itself and printing the tallies:
    ///
    /// ```text
    /// cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/codex-target-equipment \
    ///   cargo test --locked --bin codex-desktop equipment_catalog -- --nocapture
    /// ```
    ///
    /// The per-book totals independently agree with `reach_gate.rs`'s own
    /// `OPEN_FINDINGS` prose (APG 338, ACG 269, Bestiary 1 4, ARG 200) and
    /// with `pathfinder_unchained::equipment_tables`'s documented 42.
    fn count_by_book(response: &EquipmentCatalogResponse, book: &str) -> usize {
        response.entries.iter().filter(|e| e.book == book).count()
    }

    fn count_by_book_category(
        response: &EquipmentCatalogResponse,
        book: &str,
        category: &str,
    ) -> usize {
        response
            .entries
            .iter()
            .filter(|e| e.book == book && e.category == category)
            .count()
    }

    #[test]
    fn catalog_spans_every_ingested_book_with_their_real_counts() {
        let response = build_equipment_catalog();

        // `SD31-E6-F10-002`: 3 corpus-gap rows re-attributed CRB -> B1
        // (`decisions.md §9`, `tests/equipment_gap_tables.rs` has the full
        // story). 3312 - 3 = 3309; 4 + 3 = 7. Catalog total is unchanged.
        assert_eq!(count_by_book(&response, "CRB"), 3309);
        assert_eq!(count_by_book(&response, "APG"), 375);
        assert_eq!(count_by_book(&response, "ACG"), 319);
        assert_eq!(count_by_book(&response, "B1"), 7);
        assert_eq!(count_by_book(&response, "ARG"), 215);
        assert_eq!(count_by_book(&response, "PU"), 42);
        // 91 equipment + 7 equipmods -- see `ultimate_intrigue::equipment_tables`'s
        // own doc comment for why 7, not the 14 `work-inventory.json` reports.
        assert_eq!(count_by_book(&response, "UI"), 105);
        // 1,369 equipment + 180 equipmods -- see
        // `ultimate_equipment::equipment_tables`'s own doc comment for the
        // full raw/dupe/collision reconciliation (1,425 raw - 1 same-book
        // dupe - 55 cross-book collisions = 1,369; 190 raw - 10 collisions
        // = 180).
        // `SD31-E6-F10-003`: 1614 -> 1613, `declared_pi_at`'s extension over
        // this whole compiled table catching a pre-existing PI leak
        // (`ultimate_equipment:"Elysian Shield"`, `NAMEISPI:YES`) that
        // predates this cycle -- see `equipment_resolver.rs`'s own comment
        // on the same number for the full citation.
        //
        // SD-32 T9 onboarding (card 11): re-derived pre-existing red,
        // unrelated to this cycle's own changes -- see
        // `equipment_resolver.rs`'s identical correction (same underlying
        // `ue::equipment_tables()` static table, byte-identical to this
        // branch's pinned base) for the full citation. 1613 was already
        // stale before this cycle touched anything; 1614 is the real count.
        assert_eq!(count_by_book(&response, "UE"), 1614);
        // 24 General (pregenerated spellbooks) + 2 ArmsArmor (Scrollmaster
        // Gear); no `um_equipmods.lst` file exists for this book. Matches
        // `equipment_resolver::EQUIPMENT_BOOK_UM`'s own pinned 26.
        assert_eq!(count_by_book(&response, "UM"), 26);
        // 326 equipment + 113 equipmods (the `VISIBLE:NO` `.COPY=`
        // legacy-alias-excluded count). Matches
        // `equipment_resolver::EQUIPMENT_BOOK_UPSI`'s own pinned 439.
        assert_eq!(count_by_book(&response, "UPSI"), 552);
        // 185 equipment (26 General + 10 MagicItems + 149 ArmsArmor) + 19
        // equipmods (39 raw lines minus 20 VISIBLE:NO .COPY= legacy
        // aliases). Matches `equipment_resolver::EQUIPMENT_BOOK_UC`'s own
        // pinned 204.
        assert_eq!(count_by_book(&response, "UC"), 224);

        // UW: 127 rows, every one of them from the corpus gap lane -- this
        // book has no hand-authored equipment table at all, so before that
        // lane landed it served ZERO rows here despite already being a
        // compiled rule set whose feats and archetypes reach the player.
        assert_eq!(count_by_book(&response, "UW"), 127);

        // `SD31-E6-F10-003`: 8 further already-compiled books extended into
        // the corpus gap lane, same "no hand-authored table" shape as `UW`
        // above -- each book's count here is exactly its
        // `gen_cache_equipment_gap` file count (`find data/corpus/<book>/
        // equipment -name '*.json' | wc -l`), one catalog row per shipped
        // JSON file, one to one.
        assert_eq!(count_by_book(&response, "OA"), 119);
        assert_eq!(count_by_book(&response, "HA"), 117);
        // SD-32 `decisions.md §24` (re-derived this cycle, `t9-onboarding-
        // unowned-reds`, against `tests/equipment_gap_tables.rs`'s own
        // already-correct `EXPECTED_PER_BOOK`, which independently agrees):
        // a declared-PI/blacklisted-name row is no longer excluded from
        // this book whole -- it is INCLUDED under a Codex-generated
        // neutral name. ISR 71 -> 72 (+1), ISWG 46 -> 53 (+7).
        assert_eq!(count_by_book(&response, "ISR"), 72);
        assert_eq!(count_by_book(&response, "ISWG"), 53);
        assert_eq!(count_by_book(&response, "MC"), 49);
        // `B2`/`B3`: 7/8, not 8/9 -- each net of 1 bare PFS organized-play
        // legality OVERLAY row (`is_non_record_line`'s `PFSNotLegal`
        // extension) that was shipping as a spurious second catalog entry;
        // see `tests/equipment_gap_tables.rs`'s own `EXPECTED_PER_BOOK` for
        // the full citation.
        assert_eq!(count_by_book(&response, "B2"), 7);
        assert_eq!(count_by_book(&response, "B3"), 8);
        // SD-32 `decisions.md §24`: 5 -> 8 (+3), same neutral-name-inclusion
        // shape as ISR/ISWG above.
        assert_eq!(count_by_book(&response, "B4"), 8);

        // `SD31-E6-F10-004`: 5 further already-compiled books, the ones
        // `SD31-E6-F10-003` deliberately left out (`OPEN-ISSUES.md` row 186)
        // because their real corpus text hit `screen_generated_table`'s
        // whole-file blacklist hard stop. Reachable now that a per-record
        // `blacklist_hit` pre-filter excludes/redacts only the individual
        // offending rows; see `tests/equipment_gap_tables.rs`'s own
        // `EXPECTED_PER_BOOK` for the full citation.
        // SD-32 `decisions.md §24` (re-derived this cycle against
        // `tests/equipment_gap_tables.rs`'s own already-correct
        // `EXPECTED_PER_BOOK`, which independently agrees): each book's
        // declared-PI/blacklist name exclusions are no longer excluded
        // whole -- they are INCLUDED under a Codex-generated neutral name.
        // 125 -> 150 (`inner_sea_gods`, +25), 252 -> 255 (`mythic_
        // adventures`, +3), 65 -> 72 (`inner_sea_combat`, +7), 34 -> 42
        // (`inner_sea_intrigue`, +8), 5 -> 6 (`book_of_the_damned_
        // volume_2`, +1).
        assert_eq!(count_by_book(&response, "ISG"), 150);
        assert_eq!(count_by_book(&response, "MYTHIC"), 255);
        assert_eq!(count_by_book(&response, "ISC"), 72);
        assert_eq!(count_by_book(&response, "ISI"), 42);
        assert_eq!(count_by_book(&response, "BOTD2"), 6);
        // SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off --
        // two more already-compiled books; see `tests/equipment_gap_tables.rs`
        // `EXPECTED_PER_BOOK` for the full citation.
        assert_eq!(count_by_book(&response, "ISTEM"), 43);
        // SD-32 T9 residual (`decisions.md §20`): 6 -> 68.
        // `cache_gen::equipment_gap::book_routing` had no arm for `"ISM"`
        // (nor `"ISTEM"` above) at all -- the corpus gap lane's own config
        // table already generated these rows, but the cache writer silently
        // dropped every one before it reached `data/corpus/`. Fixed; see
        // `tests/equipment_gap_tables.rs`'s own `EXPECTED_PER_BOOK` for the
        // full citation, including the separately-recovered
        // `ism_equipmods.lst` (+62) that raises this to 68.
        assert_eq!(count_by_book(&response, "ISM"), 68);
        // SD-32 `decisions.md §24`/T9 residual (`decisions.md §20`):
        // `adventurers_guide` (`AG`) had no corpus gap config at all before
        // T9 residual landed -- the single largest un-covered `equipment`
        // population. 97 -> 115 (+18, its declared-PI exclusions are no
        // longer excluded whole under `§24`) -> 116 (+1, `ag_equipmods.lst`
        // was absent from `AG`'s `BOOK_INPUTS`, recovering one real
        // `equipment_modifier` object). `tests/equipment_gap_tables.rs`'s
        // own already-correct `EXPECTED_PER_BOOK` independently agrees.
        assert_eq!(count_by_book(&response, "AG"), 116);
        // SD-32 desktop count re-sweep: `beginner_box` (`BB`) was ingested
        // (`decisions.md §27b`, the `EXCLUDED_BOOKS` carve-out removed) --
        // 19 new corpus files under `data/corpus/beginner_box/equipment/`,
        // every one routed through the same corpus gap lane as `UW`/`ISG`/
        // etc above, one catalog row per shipped JSON file. Re-derived
        // directly against the regenerated `equipment_gap_tables.rs`
        // (`grep -c 'book: "BB"'`).
        assert_eq!(count_by_book(&response, "BB"), 19);

        // Pinned as a total as well as per book so that a book silently
        // dropping out of the chain cannot be masked by another book
        // growing. Re-derived fresh this cycle (`sd32-desktop-count-
        // resweep`) by summing every `count_by_book` assertion above (equal
        // to `response.entries.len()` itself, run via `cd apps/desktop/
        // src-tauri && cargo test --locked --bin codex-desktop
        // equipment_catalog -- --nocapture` with a temporary per-book
        // dump, matching `tests/equipment_gap_tables.rs`'s independently-
        // derived `EXPECTED_PER_BOOK` sum): 8025 -> 8100 (+75 = the 9
        // `decisions.md §24` neutral-name-inclusion deltas above -- ISR +1,
        // ISWG +7, B4 +3, ISG +25, MYTHIC +3, ISC +7, ISI +8, BOTD2 +1,
        // AG +19 (97->116) -- plus the pre-existing 1-unit UE-total
        // inconsistency this same pass also closed: the per-book `UE`
        // assertion above was already 1614, but the OLD total assertion
        // (8025) had never been updated off the superseded 1613, so 8025
        // was already 1 short of its own per-book sum before this cycle
        // touched anything) -> 8119 (+19, `BB` above; `beginner_box`
        // ingestion, `decisions.md §27b` -- a pinned-count staleness
        // regression, not a logic regression: this workspace is a
        // SEPARATE cargo build from the root-level sweep that ingested
        // `beginner_box`, so nothing here re-ran until now).
        assert_eq!(response.entries.len(), 8119);
    }

    #[test]
    fn every_book_code_is_a_declared_one_and_every_declared_code_is_present() {
        let response = build_equipment_catalog();
        let declared: BTreeSet<&str> = equipment_catalog_books().into_iter().collect();
        let seen: BTreeSet<&str> = response.entries.iter().map(|e| e.book.as_str()).collect();
        assert_eq!(
            seen,
            declared,
            "every emitted book code must be declared, and every declared code must \
             actually reach the response — an unreachable code is the same defect this \
             widening exists to fix"
        );
    }

    #[test]
    fn per_book_category_counts_are_pinned() {
        let response = build_equipment_catalog();

        // CRB — `SD31-E6-F10-002` moved 2 ArmsArmor rows (`Rock (Small)`,
        // `Rock (Medium)`) and 1 General row (`Poison (Violet Venom)`) to
        // B1 below (`decisions.md §9` re-attribution).
        assert_eq!(count_by_book_category(&response, "CRB", "ArmsArmor"), 310);
        assert_eq!(count_by_book_category(&response, "CRB", "General"), 453);
        assert_eq!(count_by_book_category(&response, "CRB", "MagicItems"), 1556);
        assert_eq!(count_by_book_category(&response, "CRB", "Equipmods"), 990);

        // APG — no `apg_equipmods.lst` in the corpus, so no Equipmods rows.
        assert_eq!(count_by_book_category(&response, "APG", "ArmsArmor"), 75);
        assert_eq!(count_by_book_category(&response, "APG", "General"), 93);
        assert_eq!(count_by_book_category(&response, "APG", "MagicItems"), 170);
        assert_eq!(count_by_book_category(&response, "APG", "Equipmods"), 37);

        assert_eq!(count_by_book_category(&response, "ACG", "ArmsArmor"), 20);
        assert_eq!(count_by_book_category(&response, "ACG", "General"), 62);
        assert_eq!(count_by_book_category(&response, "ACG", "MagicItems"), 141);
        assert_eq!(count_by_book_category(&response, "ACG", "Equipmods"), 96);

        // Bestiary 1 — 4 hand-authored monster-intrinsic items plus, as of
        // `SD31-E6-F10-002`, 3 corpus-gap rows re-attributed from CRB
        // (`Rock (Small)`/`Rock (Medium)` ArmsArmor, `Poison (Violet
        // Venom)` General; `decisions.md §9`) — genuinely no
        // `b1_equipmods.lst` file at all, so Equipmods stays 0.
        assert_eq!(count_by_book_category(&response, "B1", "ArmsArmor"), 4);
        assert_eq!(count_by_book_category(&response, "B1", "General"), 2);
        assert_eq!(count_by_book_category(&response, "B1", "MagicItems"), 1);
        assert_eq!(count_by_book_category(&response, "B1", "Equipmods"), 0);

        assert_eq!(count_by_book_category(&response, "ARG", "ArmsArmor"), 29);
        assert_eq!(count_by_book_category(&response, "ARG", "General"), 79);
        assert_eq!(count_by_book_category(&response, "ARG", "MagicItems"), 78);
        assert_eq!(count_by_book_category(&response, "ARG", "Equipmods"), 29);

        // PU has no category enum: every row is an `pu_equipmods.lst`
        // equipment modifier, so all 42 land in Equipmods and nowhere else.
        assert_eq!(count_by_book_category(&response, "PU", "Equipmods"), 42);
    }

    #[test]
    fn pu_rows_are_all_equipmods_with_an_honest_absent_cost() {
        let response = build_equipment_catalog();
        let pu: Vec<_> = response.entries.iter().filter(|e| e.book == "PU").collect();
        assert_eq!(pu.len(), 42);
        for entry in pu {
            assert_eq!(entry.category, PU_CATEGORY);
            assert!(
                entry.cost_gp.is_none(),
                "`pu_equipmods.lst` carries no COST: token on any row, so {:?} must arrive \
                 as a null cost rather than a fabricated 0",
                entry.key
            );
        }
    }

    #[test]
    fn every_entry_has_a_non_empty_key_name_and_book() {
        let response = build_equipment_catalog();
        for entry in &response.entries {
            assert!(!entry.key.is_empty());
            assert!(!entry.name.is_empty());
            assert!(!entry.book.is_empty());
            assert!(!entry.category.is_empty());
        }
    }

    #[test]
    fn keys_do_not_collide_across_books_and_crbs_own_duplicates_are_pinned() {
        let response = build_equipment_catalog();

        let mut books_per_key: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        let mut rows_per_book_key: BTreeMap<(&str, &str), usize> = BTreeMap::new();
        for entry in &response.entries {
            books_per_key
                .entry(entry.key.as_str())
                .or_default()
                .insert(entry.book.as_str());
            *rows_per_book_key
                .entry((entry.book.as_str(), entry.key.as_str()))
                .or_default() += 1;
        }

        let cross_book: BTreeSet<&str> = books_per_key
            .iter()
            .filter(|(_, books)| books.len() > 1)
            .map(|(key, _)| *key)
            .collect();
        // SD28-C4.9: UC joining the catalog introduced 136 real cross-book
        // key collisions with UE, none of it a defect. Ultimate Equipment
        // is a consolidation reprint of earlier books' weapon/armor
        // catalogs, and UC is one of the books it consolidates -- spot
        // checked directly against both source files: `Bo Staff`
        // (`uc_equip_arms_armor.lst:63`, cost 1gp/weight 3lb) and UE's own
        // copy (`ue_equip_arms_armor.lst:365`, identical cost/weight);
        // `Gladius` the same shape (15gp/3lb, both books). Every one of the
        // 136 is UC<->UE only -- confirmed no third book is ever involved
        // in any of them. Pinned by exact set, not by count, so a new,
        // unrelated collision still fails here rather than silently
        // hiding behind this one's growth.
        let expected_cross_book: BTreeSet<&str> = [
            "Alchemical Cartridge (Dragon's Breath)",
            "Alchemical Cartridge (Entangling Shot)",
            "Alchemical Cartridge (Flare)",
            "Alchemical Cartridge (Paper/Bullet)",
            "Alchemical Cartridge (Paper/Pellet)",
            "Alchemical Cartridge (Salt Shot)",
            "Amulet of Bullet Protection +1",
            "Amulet of Bullet Protection +2",
            "Amulet of Bullet Protection +3",
            "Amulet of Bullet Protection +4",
            "Amulet of Bullet Protection +5",
            "Atlatl",
            "Atlatl Dart",
            "Black Powder (Dose)",
            "Blunderbuss",
            "Bo Staff",
            "Broadsword (Nine Ring)",
            "Buckler Gun",
            "Bullet (Firearm)",
            "Bullet (Firearm/30)",
            "Bullet (Firearm/Pitted)",
            "Butterfly Sword",
            "Culverin",
            "Dan Bong",
            "Do-maru",
            "Double Chicken Saber",
            "Double Hackbut",
            "Dry Load Powder Horn",
            "Emei Piercer",
            "Far-Reaching Sight",
            "Fighting Fan",
            "Figurine of Wondrous Power (Slate Spider)",
            "Fire Lance",
            "Flying Blade",
            "Four-mirror Armor",
            "Gladius",
            "Gunsmith's Kit",
            "Haramaki",
            "Harpoon",
            "Hooked Axe",
            "Hooked Lance",
            "Iron Brush",
            "Jutte",
            "Kama (Double-Chained)",
            "Katana",
            "Katana (Double Walking Stick)",
            "Kerambit",
            "Kestros",
            "Kestros Dart (10)",
            "Kikko Armor",
            "Knuckle Axe",
            "Kusari Gusoku",
            "Kusarigama (Sickle and Chain)",
            "Kyoketsu Shoge",
            "Lamellar (Horn)",
            "Lamellar (Iron)",
            "Lamellar (Leather)",
            "Lamellar (Steel)",
            "Lamellar (Stone)",
            "Lamellar Cuirass",
            "Lungchuan Tamo",
            "Madu (Leather)",
            "Madu (Steel)",
            "Material ~ Bone",
            "Material ~ Bronze",
            "Material ~ Gold",
            "Material ~ Obsidian",
            "Material ~ Stone",
            "Mattock",
            "Mere Club",
            "Metal Cartridge",
            "Meteor Hammer",
            "Monk's Spade",
            "Mountain Pattern Armor",
            "Musket",
            "Musket (Axe)",
            "Musket (Double-Barreled)",
            "Musket (Warhammer)",
            "Naginata",
            "Nine-Section Whip",
            "Nodachi",
            "O-yoroi",
            "Oil (Of Silence)",
            "Pata",
            "Pellets (Handful)",
            "Pellets (Handful/30)",
            "Pepperbox",
            "Pistol",
            "Pistol (Coat)",
            "Pistol (Dagger)",
            "Pistol (Double-Barreled)",
            "Pistol (Dragon)",
            "Pistol (Sword Cane)",
            "Poisoned Sand Tube",
            "Powder Horn",
            "Powder Keg",
            "Quadrens",
            "Revolver",
            "Rhomphaia",
            "Rifle",
            "Rifle (Pepperbox)",
            "Rope Dart",
            "Sansetsukon",
            "Scizore",
            "Scorpion Whip",
            "Shang Gou",
            "Shotel",
            "Shotgun",
            "Shotgun (Double-Barreled)",
            "Sibat",
            "Sica",
            "Silken Ceremonial Armor",
            "Special Ability ~ Dry Load ~ Firearm / Ammunition",
            "Special Ability ~ Lucky / Greater ~ Firearm",
            "Special Ability ~ Lucky ~ Firearm",
            "Special Ability ~ Reliable / Greater ~ Firearm",
            "Special Ability ~ Reliable ~ Firearm",
            "Special Quality ~ Fragile",
            "Special Quality ~ Performance",
            "Special Quality ~ Scatter ~ Firearm",
            "Sword (Seven-Branched)",
            "Sword (Tri-Point Double-Edged)",
            "Taiaha",
            "Tatami-do",
            "Tekko-Kagi (Iron Claw)",
            "Tepoztopilli",
            "Terbutje",
            "Terbutje (Great)",
            "Tetsubo",
            "Throwing Shield",
            "Tiger Fork",
            "Tonfa",
            "Tube Arrow Shooter",
            "Urumi",
            "Wahaika",
            "Wakizashi",
        ]
        .into_iter()
        .collect();
        // The pinned set above is the HAND-AUTHORED baseline, so it is
        // asserted against the hand-authored rows alone. Recomputed here by
        // removing every `(book, key)` pair the corpus gap lane contributes,
        // which keeps the original UC/UE review intact instead of dissolving
        // it into a larger, unreviewed set.
        let gap_pairs: BTreeSet<(&str, &str)> =
            equipment_gap_tables::equipment_gap_rows().map(|r| (r.book, r.key)).collect();
        let mut hand_books_per_key: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for entry in &response.entries {
            let pair = (entry.book.as_str(), entry.key.as_str());
            if gap_pairs.contains(&pair) {
                continue;
            }
            hand_books_per_key.entry(entry.key.as_str()).or_default().insert(entry.book.as_str());
        }
        let hand_cross_book: BTreeSet<&str> = hand_books_per_key
            .iter()
            .filter(|(_, books)| books.len() > 1)
            .map(|(key, _)| *key)
            .collect();
        assert_eq!(
            hand_cross_book, expected_cross_book,
            "cross-book equipment key collisions among the HAND-AUTHORED tables changed -- \
             every UC/UE reprint pair is named above; a key outside that set is a new, \
             unreviewed collision"
        );

        // The corpus gap lane (`epic-4-proven-equip-mod`) raises the
        // cross-book total from 136 to 203, and that is the CORRECT answer
        // rather than a defect to dedupe away: an item Ultimate Equipment
        // reprints out of the Core Rulebook is a record in BOTH books, and
        // this lane's whole predicate is "a record this book's own table does
        // not hold". Each DTO carries its own `book` tag, so the catalog
        // shows the CRB copy as CRB's and the UE copy as UE's. What is
        // asserted here is that every collision the lane introduced actually
        // involves a gap row -- a new collision between two hand tables would
        // still fail, above.
        // `SD31-E6-F10-003`: +12 new cross-book collisions from the 8
        // newly-extended gap-lane books (203 -> 215), every one verified
        // below to involve a gap row.
        // 213 -> 212: `bestiary_2`'s bare `Maul of the Titans` PFS legality
        // overlay row (removed, `is_non_record_line`'s `PFSNotLegal`
        // extension) was one of the 12 new cross-book collisions; the
        // `bestiary_3` `Ranged Cannon` removal did not change this count
        // (it never collided with another book's key).
        // `SD31-E6-F10-004`: 212 -> 213, +1 new cross-book collision from
        // the 5 newly-extended gap-lane books (`inner_sea_gods`/
        // `mythic_adventures`/`inner_sea_combat`/`inner_sea_intrigue`/
        // `book_of_the_damned_volume_2`), verified below (the loop over
        // `cross_book.difference(&expected_cross_book)`) to involve a gap
        // row, same discipline as every prior growth of this count.
        // SD-32 T9 residual (`decisions.md §20`): 213 -> 225, +12 new
        // cross-book collisions from the `ISM` routing/citation fix and the
        // new `AG`/`UM` books, every one verified below (the loop
        // immediately following) to involve a gap row -- not hand-counted.
        // SD-32 desktop count re-sweep (`beginner_box` ingested,
        // `decisions.md §27b`): 225 -> 230, +5 new cross-book collisions
        // from `BB` (`Bandages of Rapid Recovery`, `Campfire Bead`,
        // `Dawnflower Sash`, `Flying Ointment`, `Glowing Glove`, each
        // shared with an already-cataloged book's hand table or gap row),
        // every one verified below (the loop immediately following) to
        // involve a gap row -- not hand-counted. Command: `cd apps/desktop/
        // src-tauri && cargo test --locked --bin codex-desktop
        // keys_do_not_collide -- --nocapture` with a temporary
        // `cross_book.difference(&expected_cross_book)` dump.
        assert_eq!(cross_book.len(), 230);
        for key in cross_book.difference(&expected_cross_book) {
            assert!(
                gap_pairs.iter().any(|(_, gap_key)| gap_key == key),
                "new cross-book collision {key:?} involves no gap row -- unreviewed"
            );
        }

        // 316 keys appear twice within CRB alone (e.g. `Holy Symbol
        // (Silver)`). That is a pre-existing property of
        // `crb::equipment_tables`, not something this widening introduced,
        // and it is pinned here so it cannot grow unnoticed — and so that
        // the cross-book assertion above is not quietly passing because
        // duplicate detection broke.
        //
        // UE adds one more, of a genuinely different shape: `Masterwork
        // Tool` is both a real purchasable item (`ue_equip_general.lst`,
        // `General`, 50 gp) and a real equipment *modifier* (a bonus you
        // apply, the UE equipment-modifier source, `Equipmods`, `<an unmade choice> circumstance
        // Bonus`) -- two distinct corpus records that happen to share a
        // display name, the same "kept, not deduped" treatment CRB's own
        // 316 already get, not a defect this widening introduced.
        let intra_book_dupes = rows_per_book_key.values().filter(|count| **count > 1).count();
        assert_eq!(intra_book_dupes, 317);
        let intra_book_dupes_outside_crb: Vec<&(&str, &str)> = rows_per_book_key
            .iter()
            .filter(|((book, _), count)| **count > 1 && *book != "CRB")
            .map(|(key, _)| key)
            .collect();
        assert_eq!(intra_book_dupes_outside_crb, vec![&("UE", "Masterwork Tool")]);
    }

    #[test]
    fn filter_equipment_catalog_with_no_filter_fields_returns_the_full_catalog() {
        let response = filter_equipment_catalog(&EquipmentCatalogFilter::default());
        assert_eq!(response.entries.len(), build_equipment_catalog().entries.len());
    }

    #[test]
    fn filter_equipment_catalog_matches_name_contains_case_insensitively() {
        let response = filter_equipment_catalog(&EquipmentCatalogFilter {
            name_contains: Some("dagger".to_owned()),
            category: None,
            book: None,
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has known Dagger records"
        );
        assert!(response.entries.len() < build_equipment_catalog().entries.len());
        for entry in &response.entries {
            assert!(
                entry.name.to_lowercase().contains("dagger"),
                "entry {:?} does not contain 'dagger'",
                entry.name
            );
        }
    }

    #[test]
    fn filter_equipment_catalog_matches_category_exactly_across_every_book() {
        let response = filter_equipment_catalog(&EquipmentCatalogFilter {
            name_contains: None,
            category: Some("ArmsArmor".to_owned()),
            book: None,
        });

        // 310 CRB + 75 APG + 20 ACG + 4 B1 + 29 ARG + 0 PU + 14 UI + 281 UE
        // + 2 UM + 52 UPSI + 149 UC + 1 UW. (`SD31-E6-F10-002`: 2 ArmsArmor
        // rows moved CRB -> B1, `decisions.md §9`; the 937 total is
        // unchanged.) `SD31-E6-F10-003`: +86 ArmsArmor rows across the 8
        // newly-extended gap-lane books (937 -> 1023), re-derived fresh from
        // the built catalog, not adjusted by a hand count.
        // 1017 -> 1015: both removed PFS legality overlay rows
        // (`bestiary_2`'s `Maul of the Titans`, `bestiary_3`'s `Ranged
        // Cannon`) were `ArmsArmor` category.
        // `SD31-E6-F10-004`: +35 ArmsArmor rows across the 5 newly-extended
        // gap-lane books (1015 -> 1050), re-derived fresh from the built
        // catalog, not adjusted by a hand count.
        // SD-32 T9 onboarding (card 11): +8 ArmsArmor rows, all from
        // inner_sea_temples (ism contributes 0 -- its 6 rows are all
        // `General`), re-derived directly against the generated
        // `equipment_gap_tables.rs` (1050 -> 1058).
        // SD-32 T9 residual (`decisions.md §20`): +19 ArmsArmor rows, all
        // from the new `AG` book (`ism`'s routing fix and recovered
        // `ism_equipmods.lst` rows are all `Equipmods` category, 0
        // ArmsArmor; `um` contributes 0 rows at all), re-derived directly
        // against the regenerated table (1058 -> 1077).
        // SD-32 `decisions.md §24` re-derivation (`t9-onboarding-unowned-
        // reds`): 1077 -> 1095 (+18), measured directly (not hand-summed
        // from a per-book delta) via a temporary per-book/per-category
        // dump against the current corpus, alongside the neutral-name-
        // inclusion widening documented on `catalog_spans_every_ingested_
        // book_with_their_real_counts` above (whose per-book row-count
        // deltas this total is consistent with: `ISG`/`MYTHIC`/`ISI`/
        // `BOTD2`'s current `ArmsArmor` counts are all 0, so their growth
        // is entirely `General`/`Equipmods`; `ISR`/`ISWG`/`B4`/`ISC` each
        // currently carry a nonzero `ArmsArmor` count, consistent with
        // being the source of this delta). Command: `cd apps/desktop/
        // src-tauri && cargo test --locked --bin codex-desktop
        // equipment_catalog -- --nocapture` with a temporary per-book
        // `ArmsArmor`-count dump.
        // SD-32 desktop count re-sweep: 1095 -> 1097 (+2, `BB`'s 2
        // `ArmsArmor` rows -- `beginner_box` ingestion, `decisions.md
        // §27b`).
        assert_eq!(response.entries.len(), 1097);
        for entry in &response.entries {
            assert_eq!(entry.category, "ArmsArmor");
        }
    }

    #[test]
    fn filter_equipment_catalog_narrows_to_one_book() {
        for (book, expected) in [
            // `SD31-E6-F10-002`: 3 rows moved CRB -> B1 (`decisions.md §9`).
            ("CRB", 3309),
            ("APG", 375),
            ("ACG", 319),
            ("B1", 7),
            ("ARG", 215),
            ("PU", 42),
            // UW is filterable only because the corpus gap lane put it in
            // the catalog at all.
            ("UW", 127),
        ] {
            let response = filter_equipment_catalog(&EquipmentCatalogFilter {
                name_contains: None,
                category: None,
                book: Some(book.to_owned()),
            });
            assert_eq!(response.entries.len(), expected, "book {book}");
            for entry in &response.entries {
                assert_eq!(entry.book, book);
            }
        }
    }

    #[test]
    fn filter_equipment_catalog_with_an_unknown_book_matches_nothing() {
        // "UM" used to be this test's sentinel -- it stopped being unknown
        // the moment UM joined the catalog, so a genuinely unassigned code
        // is needed instead ("ZZ" is not, and never has been, declared by
        // `equipment_catalog_books`).
        let response = filter_equipment_catalog(&EquipmentCatalogFilter {
            name_contains: None,
            category: None,
            book: Some("ZZ".to_owned()),
        });
        assert!(response.entries.is_empty());
    }

    #[test]
    fn filter_equipment_catalog_combines_name_category_and_book_filters() {
        let response = filter_equipment_catalog(&EquipmentCatalogFilter {
            name_contains: Some("shield".to_owned()),
            category: Some("MagicItems".to_owned()),
            book: None,
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has known Shield-named magic items (e.g. Ring of Force Shield)"
        );
        for entry in &response.entries {
            assert_eq!(entry.category, "MagicItems");
            assert!(entry.name.to_lowercase().contains("shield"));
        }

        let apg_only = filter_equipment_catalog(&EquipmentCatalogFilter {
            name_contains: Some("shield".to_owned()),
            category: Some("MagicItems".to_owned()),
            book: Some("APG".to_owned()),
        });
        assert!(apg_only.entries.len() <= response.entries.len());
        for entry in &apg_only.entries {
            assert_eq!(entry.book, "APG");
        }
    }

    /// The `book` field must actually serialize onto the wire under the
    /// camelCase name the TypeScript boundary reads — a Rust-side field
    /// that never crosses the IPC boundary would surface nothing, which is
    /// the exact defect class this change closes.
    #[test]
    fn book_is_serialized_onto_the_wire() {
        let entry = build_equipment_catalog()
            .entries
            .into_iter()
            .find(|entry| entry.book == "ARG")
            .expect("ARG entries are in the catalog");
        let json = serde_json::to_value(&entry).expect("entry serializes");
        assert_eq!(json.get("book").and_then(|v| v.as_str()), Some("ARG"));
        assert!(json.get("costGp").is_some(), "existing camelCase fields are unchanged");
    }
}
