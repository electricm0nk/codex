//! Bestiary 1 equipment tables. SD-25 criterion 7.N item 4
//! ("Bestiary 1 equipment + spells") — the register's own framing of this
//! item is "plain scope gap, NOT a corpus ceiling": no `beastiary1`
//! equipment module existed at all prior to this cycle (see
//! `tests/sd24_equipment_coverage_audit.rs`'s pre-existing "Bestiary 1 is
//! not covered by this test file... no live code to call" note, closed
//! by this cycle).
//!
//! Mirrors `rules_tables::{crb,apg,acg}::equipment_tables`'s shape
//! (`EquipmentTableEntry`/`EquipmentFieldCoverage`/`field_coverage_report`/
//! `equipment_resolve`) so the cross-book coverage audit can iterate this
//! book the same way. See `equipment_data.rs`'s module doc comment for
//! full sourcing methodology and the register A8/A10/A11 method notes.
//!
//! **Register A8 (codegen-path decision):** the shared
//! `pcgen_import`-backed codegen path (`lst_parser::equipment` ->
//! semantic map -> Rust literals) that CRB/APG/ACG's much larger
//! record counts justified building tooling for was deliberately **not**
//! used here. Bestiary 1's real equipment corpus is 4 records total
//! across all three files (see `equipment_data.rs`) — small enough to
//! transcribe and identity-verify by hand in the time it would take to
//! wire up and validate a one-off generator for a single-digit record
//! count, matching the same hand-authored precedent
//! `beastiary1::monster_subset_01`..`08` already established for this
//! book's monster chassis data. If a future Bestiary 1 ingest widens
//! scope (e.g. `.MOD` companion records from later splatbooks), that is
//! the point to revisit building the shared path.
//!
//! **Register A13 (spell-list existence check):** confirmed before
//! writing any code that no spell-list concept exists for Bestiary 1 in
//! the real PCGen corpus at
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary/`
//! — there is no `b1_spells.lst` (or any `*spell*` file) in that
//! directory at all, unlike CRB/APG which each carry a dedicated
//! `<book>_spells.lst`. `SPELLS:`/`CLASSSPELL`-shaped tokens do appear,
//! but only inline on `b1_abilities_race.lst`/`b1_races.lst`/
//! `b1_templates.lst`/`b1_kits_race.lst` rows as innate spell-like
//! ability grants on individual monster stat blocks (e.g. a monster's
//! "Spell-Like Abilities" special quality) — the same kind of
//! monster-intrinsic ability data `beastiary1::MonsterStatBlock`'s own
//! doc comment already scopes out of this book's chassis-data surface
//! (AC/HP/saves/spells are derived, not literal chassis tokens). There is
//! no standalone, player-facing spell-list table to ingest here, so
//! "spells" is correctly N/A for this item, not an unclaimed gap.

use crate::rules_core::rules_tables::RuleSetId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    General,
    ArmsArmor,
    MagicItems,
}

impl EquipmentCategory {
    pub const ALL: &'static [EquipmentCategory] = &[
        EquipmentCategory::General,
        EquipmentCategory::ArmsArmor,
        EquipmentCategory::MagicItems,
    ];

    /// Which `bestiary` corpus file this category's records live in.
    /// (The on-disk directory/filename prefix is `b1_`, matching the
    /// corpus's own naming; there is no `b1_equipmods.lst` file at all —
    /// Bestiary 1 introduces no equipment *modifiers*, only base items.)
    pub fn corpus_file_name(self) -> &'static str {
        match self {
            EquipmentCategory::General => "b1_equip_general.lst",
            EquipmentCategory::ArmsArmor => "b1_equip_arms_armor.lst",
            EquipmentCategory::MagicItems => "b1_equip_magic_items.lst",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    /// The corpus's raw first-column name (its `SORTKEY:`-equivalent
    /// identity when no explicit `KEY:` token is present, which is every
    /// record in this book), falling back to `name` otherwise — same
    /// convention `rules_tables::{crb,apg,acg}::equipment_tables`
    /// document for their own `key` field.
    pub key: &'static str,
    pub category: EquipmentCategory,
    /// Display name from the corpus's `OUTPUTNAME:` token when present
    /// (`[NAME]` means "echo the raw name unchanged"), else the raw name.
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. Every one of
    /// this book's 4 records carries `COST:0` — these are
    /// monster-intrinsic items (a poison, a racial rag-armor quality, a
    /// thrown weapon, a periapt gemstone) rather than PC shop-priced
    /// goods, so `Some(0.0)` is the honest, literal corpus value, not a
    /// missing-data placeholder.
    pub cost_gp: Option<f64>,
    /// Weight in pounds from the corpus `WT:` token. Present on all 4
    /// records (including `Some(0.0)` where the corpus literally states
    /// `WT:0`).
    pub weight_lbs: Option<f64>,
    /// Descriptive text. Sourced from the corpus `SPROP:` ("Special
    /// Property") token where present — Bestiary 1's equipment corpus
    /// carries no `DESC:` token at all (checked directly), but 3 of the
    /// 4 records do carry `SPROP:` (register A10: same convention
    /// `rules_tables::acg::equipment_data` already established, "ACG hit
    /// 98.1% via `SPROP:` alone"). The 1 record with neither `DESC:` nor
    /// `SPROP:` (`Rag Armor (Dark Creeper)`) is filled from a web
    /// second-source per this cycle's receipt — never fabricated, and
    /// only after an identity-match confirmation (name + monster +
    /// source page) against the same Dark Creeper stat block the LST
    /// record itself is keyed to.
    pub description: Option<&'static str>,
}

pub use super::equipment_data::EQUIPMENT_RECORDS;

pub const EQUIPMENT_TABLE: &[EquipmentTableEntry] = EQUIPMENT_RECORDS;

/// SD-25 criterion 7.N equipment field-coverage audit row. Mirrors
/// `rules_tables::{crb,apg,acg}::equipment_tables::EquipmentFieldCoverage`'s
/// shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    /// Records currently in `EQUIPMENT_TABLE`.
    pub total_records: u32,
    /// Real, active (non-`.MOD`, non-`.COPY=`, non-`SOURCELONG`-header,
    /// non-comment, non-blank) record count across `b1_equip_general.lst`
    /// (1) + `b1_equip_arms_armor.lst` (2) + `b1_equip_magic_items.lst`
    /// (1) = 4. Verified directly (no `.MOD`/`.COPY=` rows exist in any
    /// of the 3 files; each carries exactly one `SOURCELONG:` header line
    /// excluded from this count per the same off-by-one-per-file
    /// methodology correction `rules_tables::apg::equipment_data`
    /// documents (register A11)).
    pub records_expected: u32,
    /// Records with `cost_gp.is_some()`.
    pub has_cost: u32,
    /// Records with `weight_lbs.is_some()`.
    pub has_weight: u32,
    /// Records with `description.is_some()` — 4/4 after this cycle's web
    /// second-source pass closed the one `Rag Armor (Dark Creeper)` gap
    /// neither `DESC:` nor `SPROP:` covered.
    pub has_description: u32,
}

/// Computes this book's equipment field-coverage audit row. See
/// `EquipmentFieldCoverage`'s own field doc comments for methodology.
pub fn field_coverage_report() -> EquipmentFieldCoverage {
    EquipmentFieldCoverage {
        total_records: EQUIPMENT_TABLE.len() as u32,
        records_expected: 1 + 2 + 1,
        has_cost: EQUIPMENT_TABLE.iter().filter(|entry| entry.cost_gp.is_some()).count() as u32,
        has_weight: EQUIPMENT_TABLE.iter().filter(|entry| entry.weight_lbs.is_some()).count() as u32,
        has_description: EQUIPMENT_TABLE
            .iter()
            .filter(|entry| entry.description.is_some())
            .count() as u32,
    }
}

/// Resolves a Bestiary 1 equipment item by key, scoped to
/// `RuleSetId::Bestiary1` (cross-book invariant, mirrors
/// `beastiary1::monster_resolve` and `apg::equipment_tables::equipment_resolve`).
/// Returns `None` for any other rule set, and `None` when the key isn't in
/// `EQUIPMENT_TABLE`.
pub fn equipment_resolve(key: &str, rule_set: RuleSetId) -> Option<&'static EquipmentTableEntry> {
    if rule_set != RuleSetId::Bestiary1 {
        return None;
    }
    EQUIPMENT_TABLE.iter().find(|entry| entry.key == key)
}
