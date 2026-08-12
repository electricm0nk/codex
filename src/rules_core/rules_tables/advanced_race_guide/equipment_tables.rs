//! ARG shared equipment tables -- full in-scope corpus coverage.
//!
//! Record coverage: every real, active (non-`.MOD`) record across
//! `arg_equip_arms_armor.lst`, `arg_equip_general.lst`,
//! `arg_equip_magic_items.lst`, and `arg_equipmods.lst` -- disambiguated by
//! source file (unlike ACG's single combined `acg_equip.lst`, ARG carries
//! ArmsArmor/General/MagicItems in 3 separate files, the same shape CRB's
//! own 4-file split already establishes).
//!
//! Real, verified counts (SD-27 Cycle E2.1/E2.2, re-measured directly from
//! the live corpus, not taken from the scoping brief's rough estimate):
//! ArmsArmor 28, General 79, MagicItems
//! 78, Equipmods 15 -- 200 total. A small
//! number of genuine corpus defects were resolved during extraction (never
//! fabricated around): one byte-for-byte duplicate `Bonebreaker Gauntlets`
//! row in `arg_equip_magic_items.lst` (kept the first occurrence), one
//! redundant `Claw Blades (Catfolk).COPY=Rending Claw Blades` row in
//! `arg_equip_arms_armor.lst` that duplicates an already-fully-specified
//! `Claw Blades (Catfolk)` row under the same display name (kept the richer
//! row), and `arg_equipmods.lst`'s own trailing `# Old KEYs`
//! `VISIBLE:NO`-alias block (14 rows, excluded -- see
//! `equipment_data/equipmods.rs`'s own doc comment).
//!
//! `description` is sourced from the corpus `SPROP:` ("Special Property")
//! token -- like ACG, none of ARG's equipment/equipmods corpus files carry
//! a `DESC:` token anywhere (confirmed by direct grep: zero hits across all
//! 4 files). Multiple `SPROP:` entries on one record are joined with `"; "`;
//! a trailing `|<conditional-tag>` qualifier is stripped before storage.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    General,
    ArmsArmor,
    MagicItems,
    Equipmods,
}

impl EquipmentCategory {
    pub const ALL: &'static [EquipmentCategory] = &[
        EquipmentCategory::General,
        EquipmentCategory::ArmsArmor,
        EquipmentCategory::MagicItems,
        EquipmentCategory::Equipmods,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    /// Equipment records carry their `name` (or, for `arg_equipmods.lst`,
    /// the explicit `KEY:` token when present) as the corpus identity.
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. `None` when the
    /// token is absent, or when it is a non-numeric PCGen formula (e.g.
    /// `COST:WT*375` on `Material ~ Darkleaf Cloth ~ Item`) this table does
    /// not evaluate -- never a fabricated flat number for a formula cost.
    pub cost_gp: Option<f64>,
    /// Weight in pounds from the corpus `WT:` token. `None` when the corpus
    /// genuinely carries no `WT:` token for this record (true for every
    /// `arg_equipmods.lst` record, matching CRB's/ACG's own established
    /// finding that equipment *modifiers* carry no independent weight).
    pub weight_lbs: Option<f64>,
    /// Descriptive text, sourced from the corpus `SPROP:` token -- see this
    /// module's own doc comment. `None` only when the corpus record has no
    /// `SPROP:` token at all. Never fabricated.
    pub description: Option<&'static str>,
}

/// SD-27 Cycle E2.1/E2.2 equipment field-coverage audit row. Mirrors
/// `rules_tables::acg::equipment_tables::EquipmentFieldCoverage`'s shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    pub total_records: u32,
    /// Real, active (non-`.MOD`), de-duplicated record count (this module's
    /// own doc comment: 200 total).
    pub records_expected: u32,
    pub has_cost: u32,
    pub has_weight: u32,
    pub has_description: u32,
}

/// Computes this book's equipment field-coverage audit row.
pub fn field_coverage_report() -> EquipmentFieldCoverage {
    let table = equipment_tables();
    EquipmentFieldCoverage {
        total_records: table.len() as u32,
        records_expected: 200,
        has_cost: table.iter().filter(|entry| entry.cost_gp.is_some()).count() as u32,
        has_weight: table.iter().filter(|entry| entry.weight_lbs.is_some()).count() as u32,
        has_description: table.iter().filter(|entry| entry.description.is_some()).count() as u32,
    }
}

/// Full ARG equipment table store: every real corpus record across all 4
/// source files. Built once and cached for the process lifetime.
pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<EquipmentTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(
            super::equipment_data::general::GENERAL_TABLE.len()
                + super::equipment_data::arms_armor::ARMS_ARMOR_TABLE.len()
                + super::equipment_data::magic_items::MAGIC_ITEMS_TABLE.len()
                + super::equipment_data::equipmods::EQUIPMODS_TABLE.len(),
        );
        all.extend_from_slice(super::equipment_data::general::GENERAL_TABLE);
        all.extend_from_slice(super::equipment_data::arms_armor::ARMS_ARMOR_TABLE);
        all.extend_from_slice(super::equipment_data::magic_items::MAGIC_ITEMS_TABLE);
        all.extend_from_slice(super::equipment_data::equipmods::EQUIPMODS_TABLE);
        all
    })
}

/// Resolves an ARG equipment item by key. Unlike
/// `rules_tables::acg::equipment_tables::equipment_resolve`, this is not
/// `RuleSetId`-scoped — see `spell_list::spell_resolve`'s own doc comment
/// for why (this module is not wired into the shared `RuleSetId` enum,
/// per SD-27's per-cycle file-touch partition).
pub fn equipment_resolve(key: &str) -> Option<&'static EquipmentTableEntry> {
    equipment_tables().iter().find(|entry| entry.key == key)
}
