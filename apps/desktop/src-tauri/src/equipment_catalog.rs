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

use codex::rules_core::pcgen_desc::render_pcgen_desc;
use codex::rules_core::rules_tables::{
    acg, advanced_race_guide as arg, apg, beastiary1, crb, pathfinder_unchained as pu,
    ultimate_equipment as ue, ultimate_intrigue as ui,
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

/// Every book code this catalog can emit, in the order
/// `build_equipment_catalog` emits them.
pub const EQUIPMENT_CATALOG_BOOKS: &[&str] = &[
    BOOK_CRB, BOOK_APG, BOOK_ACG, BOOK_B1, BOOK_ARG, BOOK_PU, BOOK_UI, BOOK_UE,
];

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
    /// [`EQUIPMENT_CATALOG_BOOKS`]. Additive field — a consumer that does
    /// not read it is unaffected, and one that does can label or filter
    /// by book the way the Spell Catalog screen already does.
    pub book: String,
    /// The record's corpus `DESC:` prose, rendered by [`serve_description`].
    /// `None` where the corpus row genuinely carries no description — a
    /// real and documented gap for template/bookkeeping rows (see
    /// `crb::equipment_tables::EquipmentTableEntry::description`), never a
    /// fabricated placeholder.
    pub description: Option<String>,
}

/// Renders one table description into the prose this catalog is allowed to
/// serve — the identical treatment `spell_catalog::serve_description`
/// already applies, and the reason this module now has one.
///
/// Equipment descriptions were being read out of the compiled tables and
/// were **never** run through the renderer, so 54 records still carried the
/// raw PCGen `%%` literal-percent escape: ARG's `Helmet (Dwarven Boulder)`
/// ("adds 20%% to the wearer's arcane spell failure chance") plus 53 CRB
/// records (41 MagicItems, 6 General, 6 ArmsArmor). Counts derived by
/// running the catalog through `leaked_pcgen_syntax`, not assumed.
///
/// [`render_pcgen_desc`] owns the treatment and the reasoning about what
/// may and may not be substituted; this module does not re-decide it.
fn serve_description(raw: &str) -> String {
    render_pcgen_desc(raw).text
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
        description: entry.description.map(serve_description),
    }
}

fn map_apg_entry(entry: &apg::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_APG.to_string(),
        description: entry.description.map(serve_description),
    }
}

fn map_acg_entry(entry: &acg::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_ACG.to_string(),
        description: entry.description.map(serve_description),
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
        description: entry.description.map(serve_description),
    }
}

fn map_arg_entry(entry: &arg::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
        book: BOOK_ARG.to_string(),
        description: entry.description.map(serve_description),
    }
}

/// PU's entry type carries no `EquipmentCategory` and no `cost_gp` field
/// at all: `pu_equipmods.lst` has zero `COST:` tokens anywhere (its real
/// cost signal is an `ITEMCOST` formula `BONUS:`, not a flat gp number),
/// so `cost_gp` is honestly `None` for all 42 records rather than a
/// fabricated `Some(0.0)`.
fn map_pu_entry(entry: &pu::equipment_tables::EquipmentTableEntry) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: PU_CATEGORY.to_string(),
        name: entry.name.to_string(),
        cost_gp: None,
        book: BOOK_PU.to_string(),
        description: entry.description.map(serve_description),
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
        description: entry.description.map(serve_description),
    }
}

/// UE's entry type reuses UI's own shape exactly (own `EquipmentCategory`
/// enum, description joining `DESC:`/`SPROP:` -- see
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
        description: entry.description.map(serve_description),
    }
}

/// Build the full catalog response across every ingested book. A thin,
/// testable wrapper behind the Tauri command below (mirroring this
/// codebase's other command/pure-fn split, e.g.
/// `ge08_workbench::build_ge08_workbench_snapshot`).
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
    /// Exact match against a book code in [`EQUIPMENT_CATALOG_BOOKS`]
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
    /// | `race_catalog` | `detail` (the trait's `DESC:`), `trait_name` | yes |
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
    ///   token field rather than a `DESC:` rendering, its fix is to join the
    ///   values for display, and `monster_catalog.rs` is outside this change's
    ///   write scope. Recorded here so it is a known open finding rather than
    ///   a silent omission.
    #[test]
    fn no_catalog_serves_a_description_carrying_raw_pcgen_syntax() {
        use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;

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

    /// The 54-record `%%` leak, pinned on both sides of the render so the
    /// fix cannot be mistaken for the corpus having changed.
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
    /// | APG / ACG / B1 / PU | all | 0 | 0 |
    #[test]
    fn the_raw_percent_escape_stops_at_the_catalog_boundary() {
        use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;

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
        ]
        .into_iter()
        .collect();
        assert_eq!(raw_leaks, expected, "the raw tables' own leak profile");
        assert_eq!(raw_leaks.values().sum::<usize>(), 54);

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

        let description = helmet.description.expect("the ARG row carries a DESC: token");
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
        // bookkeeping rows with no `DESC:` token at all, and that gap is
        // documented on `crb::equipment_tables::EquipmentTableEntry`.
        assert_eq!(with_description("CRB"), 2021);
        assert_eq!(with_description("APG"), 331);
        assert_eq!(with_description("ACG"), 264);
        assert_eq!(with_description("B1"), 4);
        assert_eq!(with_description("ARG"), 194);
        assert_eq!(with_description("PU"), 42);
        assert_eq!(with_description("UI"), 41);
        assert_eq!(with_description("UE"), 435);
        assert_eq!(
            response.entries.iter().filter(|e| e.description.is_some()).count(),
            3332
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

        assert_eq!(count_by_book(&response, "CRB"), 2977);
        assert_eq!(count_by_book(&response, "APG"), 338);
        assert_eq!(count_by_book(&response, "ACG"), 269);
        assert_eq!(count_by_book(&response, "B1"), 4);
        assert_eq!(count_by_book(&response, "ARG"), 200);
        assert_eq!(count_by_book(&response, "PU"), 42);
        // 91 equipment + 7 equipmods -- see `ultimate_intrigue::equipment_tables`'s
        // own doc comment for why 7, not the 14 `work-inventory.json` reports.
        assert_eq!(count_by_book(&response, "UI"), 98);
        // 1,369 equipment + 180 equipmods -- see
        // `ultimate_equipment::equipment_tables`'s own doc comment for the
        // full raw/dupe/collision reconciliation (1,425 raw - 1 same-book
        // dupe - 55 cross-book collisions = 1,369; 190 raw - 10 collisions
        // = 180).
        assert_eq!(count_by_book(&response, "UE"), 1549);

        // 2977 + 338 + 269 + 4 + 200 + 42 + 98 + 1549. Pinned as a total as
        // well as per book so that a book silently dropping out of the
        // chain cannot be masked by another book growing.
        assert_eq!(response.entries.len(), 5477);
    }

    #[test]
    fn every_book_code_is_a_declared_one_and_every_declared_code_is_present() {
        let response = build_equipment_catalog();
        let declared: BTreeSet<&str> = EQUIPMENT_CATALOG_BOOKS.iter().copied().collect();
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

        // CRB — unchanged by the widening; the pre-existing pins.
        assert_eq!(count_by_book_category(&response, "CRB", "ArmsArmor"), 310);
        assert_eq!(count_by_book_category(&response, "CRB", "General"), 453);
        assert_eq!(count_by_book_category(&response, "CRB", "MagicItems"), 1556);
        assert_eq!(count_by_book_category(&response, "CRB", "Equipmods"), 658);

        // APG — no `apg_equipmods.lst` in the corpus, so no Equipmods rows.
        assert_eq!(count_by_book_category(&response, "APG", "ArmsArmor"), 75);
        assert_eq!(count_by_book_category(&response, "APG", "General"), 93);
        assert_eq!(count_by_book_category(&response, "APG", "MagicItems"), 170);
        assert_eq!(count_by_book_category(&response, "APG", "Equipmods"), 0);

        assert_eq!(count_by_book_category(&response, "ACG", "ArmsArmor"), 20);
        assert_eq!(count_by_book_category(&response, "ACG", "General"), 60);
        assert_eq!(count_by_book_category(&response, "ACG", "MagicItems"), 141);
        assert_eq!(count_by_book_category(&response, "ACG", "Equipmods"), 48);

        // Bestiary 1 — 4 monster-intrinsic items, and genuinely no
        // `b1_equipmods.lst` file at all.
        assert_eq!(count_by_book_category(&response, "B1", "ArmsArmor"), 2);
        assert_eq!(count_by_book_category(&response, "B1", "General"), 1);
        assert_eq!(count_by_book_category(&response, "B1", "MagicItems"), 1);
        assert_eq!(count_by_book_category(&response, "B1", "Equipmods"), 0);

        assert_eq!(count_by_book_category(&response, "ARG", "ArmsArmor"), 28);
        assert_eq!(count_by_book_category(&response, "ARG", "General"), 79);
        assert_eq!(count_by_book_category(&response, "ARG", "MagicItems"), 78);
        assert_eq!(count_by_book_category(&response, "ARG", "Equipmods"), 15);

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

        let cross_book: Vec<&&str> = books_per_key
            .iter()
            .filter(|(_, books)| books.len() > 1)
            .map(|(key, _)| key)
            .collect();
        assert!(
            cross_book.is_empty(),
            "no equipment key is shared between two books today; first offenders: {:?}",
            &cross_book.iter().take(5).collect::<Vec<_>>()
        );

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
        // apply, `ue_equipmods.lst`, `Equipmods`, `%CHOICE circumstance
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

        // 310 CRB + 75 APG + 20 ACG + 2 B1 + 28 ARG + 0 PU + 14 UI + 269 UE.
        assert_eq!(response.entries.len(), 718);
        for entry in &response.entries {
            assert_eq!(entry.category, "ArmsArmor");
        }
    }

    #[test]
    fn filter_equipment_catalog_narrows_to_one_book() {
        for (book, expected) in [
            ("CRB", 2977),
            ("APG", 338),
            ("ACG", 269),
            ("B1", 4),
            ("ARG", 200),
            ("PU", 42),
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
        let response = filter_equipment_catalog(&EquipmentCatalogFilter {
            name_contains: None,
            category: None,
            book: Some("UM".to_owned()),
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
