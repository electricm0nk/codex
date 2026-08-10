//! ACG shared equipment tables — full corpus coverage.
//!
//! Record coverage: every real, active (non-`.MOD`) record across
//! `acg_equip.lst` (General + Arms/Armor + Magic Items, disambiguated by
//! the `TYPE:` token — `Goods.*`, `Weapon.*`/`Armor.*`/`Shield.*`,
//! `Magic.*` — since ACG carries all three in one file, unlike APG's
//! three separate files) plus `acg_equipmods.lst` (Equipmods), 269 total;
//! see `EquipmentFieldCoverage` below. Copied verbatim (`KEY:`/name,
//! `COST:`) from the real PCGen corpus. SD-24 Epic 6 criteria 6.2-6.4
//! (ACG scope) additionally: (a) completed record coverage from the
//! original 3-item bootstrap sample to the full corpus, (b) added
//! `weight_lbs` (`WT:`) and `description` per record, to the honest
//! ceiling the corpus itself supports (never fabricated). ACG's LST
//! corpus carries no `DESC:` token anywhere in `acg_equip.lst`/
//! `acg_equipmods.lst` (confirmed: zero hits) — `description` is instead
//! sourced from the corpus's `SPROP:` ("Special Property") token, the
//! closest real per-item prose this book's corpus provides. See
//! `equipment_data/*.rs`'s own doc comments for the full sourcing
//! methodology and per-category ceiling. This module-level comment
//! previously claimed only "bootstrap... one representative item per
//! category" coverage (SD-22 Epic 4 criterion 13); that is corrected
//! here (SD-24 criterion 6.1's own finding, remediated by 6.2-6.4).

use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::acg::equipment_data;

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
    /// Equipment records carry their `name` (or, for `acg_equipmods.lst`,
    /// the explicit `KEY:` token) as the corpus identity. `acg_equip.lst`
    /// rows have no distinct `KEY:` token, so `key == name` for General/
    /// ArmsArmor/MagicItems (same fallback `rules_tables::apg::equipment_tables`
    /// documents for its own `key` field); Equipmods rows use the real
    /// `KEY:` token, which can differ from the display name (e.g.
    /// `Special Ability ~ Amorphous ~ Armor` vs. display name `Amorphous`).
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. `f64` because
    /// real corpus costs are frequently fractional. `None` when the token
    /// is absent — genuine for Equipmods rows priced via `PLUS:` (an
    /// enhancement-bonus slot cost, not a flat gp number) rather than a
    /// fixed `COST:`.
    pub cost_gp: Option<f64>,
    /// Weight in pounds from the corpus `WT:` token (SD-24 criterion 6.3,
    /// ACG scope). `None` when the corpus genuinely carries no `WT:`
    /// token for this record — true for every `acg_equipmods.lst` record
    /// (equipment *modifiers* have no independent physical weight of
    /// their own, matching the same finding CRB's own `Equipmods`
    /// category already established) and for a smaller number of
    /// General/Magic Items rows. Never a fabricated value.
    pub weight_lbs: Option<f64>,
    /// Descriptive text for this record (SD-24 criterion 6.4, ACG scope).
    /// Sourced from the corpus `SPROP:` ("Special Property") token —
    /// `acg_equip.lst`/`acg_equipmods.lst` carry no `DESC:` token
    /// anywhere, unlike CRB's equipment files, so `SPROP:` is the closest
    /// real per-item prose ACG's corpus provides. When a record has more
    /// than one `SPROP:` entry they are joined with `"; "`. A trailing
    /// `|<conditional-tag>` qualifier (e.g. `|PRECLASS:1,Slayer=1`) is
    /// stripped before storage — verified by inspection that every real
    /// `|`-suffixed `SPROP:` in this corpus follows the
    /// `<prose>|<directive>` shape, never real item text after the pipe.
    /// `None` only when the corpus record has no `SPROP:` token at all
    /// (rare — see `EquipmentFieldCoverage`'s per-category ceiling). Never
    /// fabricated.
    pub description: Option<&'static str>,
}

/// SD-24 Epic 6 criterion 6.1 (originating audit) / 6.2-6.4 (this cycle's
/// remediation) — equipment field-coverage audit row. Mirrors
/// `rules_tables::crb::equipment_tables::EquipmentFieldCoverage`'s shape.
/// Every field is computed from `equipment_tables()`'s real content or a
/// documented corpus record count (never a hand-guessed or invented
/// number).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    /// Records currently in `equipment_tables()`.
    pub total_records: u32,
    /// Real, active (non-`.MOD`) record count: `acg_equip.lst` (221:
    /// 60 General + 20 Arms/Armor + 141 Magic Items, `TYPE:`-disambiguated)
    /// \+ `acg_equipmods.lst` (48 `KEY:`-bearing modifier records,
    /// excluding the file's own trailing "Old KEYs" `.COPY=`-only block).
    /// SD-24 criterion 6.1 originally cited 221 for equipment (not
    /// counting `acg_equipmods.lst` at all, unlike CRB's four-category
    /// scope) — this cycle widens the scope to match CRB's own four-file
    /// treatment; see `progress.md`'s `## DISCOVERED` for the correction.
    pub records_expected: u32,
    /// Records with `cost_gp.is_some()`.
    pub has_cost: u32,
    /// Records with `weight_lbs.is_some()` (SD-24 criterion 6.3, landed
    /// this cycle for ACG).
    pub has_weight: u32,
    /// Records with `description.is_some()` (SD-24 criterion 6.4, landed
    /// this cycle for ACG, sourced from `SPROP:` — see `description`'s own
    /// doc comment).
    pub has_description: u32,
}

/// Computes this book's equipment field-coverage audit row. See
/// `EquipmentFieldCoverage`'s own field doc comments for methodology.
pub fn field_coverage_report() -> EquipmentFieldCoverage {
    let table = equipment_tables();
    EquipmentFieldCoverage {
        total_records: table.len() as u32,
        records_expected: 221 + 48,
        has_cost: table.iter().filter(|entry| entry.cost_gp.is_some()).count() as u32,
        has_weight: table.iter().filter(|entry| entry.weight_lbs.is_some()).count() as u32,
        has_description: table.iter().filter(|entry| entry.description.is_some()).count() as u32,
    }
}

/// Full ACG equipment table store: every real corpus record across
/// `acg_equip.lst`'s three `TYPE:`-disambiguated categories plus
/// `acg_equipmods.lst`, generated from the live corpus (see
/// `equipment_data/`'s own doc comment for the generation method — not
/// hand-authored, so there is no fabrication/transcription risk at this
/// scale). Built once and cached for the process lifetime.
pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<EquipmentTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(
            equipment_data::general::GENERAL_TABLE.len()
                + equipment_data::arms_armor::ARMS_ARMOR_TABLE.len()
                + equipment_data::magic_items::MAGIC_ITEMS_TABLE.len()
                + equipment_data::equipmods::EQUIPMODS_TABLE.len(),
        );
        all.extend_from_slice(equipment_data::general::GENERAL_TABLE);
        all.extend_from_slice(equipment_data::arms_armor::ARMS_ARMOR_TABLE);
        all.extend_from_slice(equipment_data::magic_items::MAGIC_ITEMS_TABLE);
        all.extend_from_slice(equipment_data::equipmods::EQUIPMODS_TABLE);
        all
    })
}

/// Resolves an ACG equipment item by name, scoped to `RuleSetId::Acg`.
/// Returns `None` for any other rule set (cross-book invariant, mirrors
/// `acg::class_chassis_resolve`), and `None` when the key isn't in the
/// full equipment table.
pub fn equipment_resolve(key: &str, rule_set: RuleSetId) -> Option<&'static EquipmentTableEntry> {
    if rule_set != RuleSetId::Acg {
        return None;
    }
    equipment_tables().iter().find(|entry| entry.key == key)
}
