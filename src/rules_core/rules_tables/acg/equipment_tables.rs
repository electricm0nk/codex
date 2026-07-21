//! ACG shared equipment tables — bootstrap/representative sample.
//!
//! Source: real PCGen corpus records, copied verbatim (name, `COST:`)
//! from the single `acg_equip.lst` file (Advanced Class Guide). Unlike
//! APG, which splits equipment across three separate files
//! (`apg_equip_general.lst`/`apg_equip_arms_armor.lst`/`apg_equip_magic_items.lst`),
//! ACG carries general goods, weapons/armor, and magic items together in
//! one `acg_equip.lst` file, disambiguated by the `TYPE:` token
//! (`Goods.*`, `Weapon.*`/`Armor.*`, `Magic.*`). Bootstrap coverage: one
//! representative item per category, mirroring
//! `rules_tables::apg::equipment_tables`'s own "one representative item
//! per category" bootstrap philosophy. Exhaustive per-category coverage
//! is later loop work, one category per cycle. SD-22 Epic 4 criterion 13.
//!
//! - `Marlinspike` — `acg_equip.lst:179`, `TYPE:Goods.Tools`, `COST:0.8`.
//! - `Headsman's Blade` — `acg_equip.lst:262`, `TYPE:Weapon...`, `COST:50`.
//! - `Ring of Eloquence` — `acg_equip.lst:271`, `TYPE:Magic.Ring`, `COST:3500`.

use crate::rules_core::rules_tables::RuleSetId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    General,
    ArmsArmor,
    MagicItems,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    /// Equipment records carry their `name` as the corpus identity (no
    /// distinct `KEY:` token on these three rows), same fallback
    /// `rules_tables::apg::equipment_tables` documents for its own
    /// `key` field.
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. `f64` because
    /// real corpus costs are frequently fractional.
    pub cost_gp: Option<f64>,
}

pub const EQUIPMENT_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry {
        key: "Marlinspike",
        category: EquipmentCategory::General,
        name: "Marlinspike",
        cost_gp: Some(0.8),
    },
    EquipmentTableEntry {
        key: "Headsman's Blade",
        category: EquipmentCategory::ArmsArmor,
        name: "Headsman's Blade",
        cost_gp: Some(50.0),
    },
    EquipmentTableEntry {
        key: "Ring of Eloquence",
        category: EquipmentCategory::MagicItems,
        name: "Ring of Eloquence",
        cost_gp: Some(3500.0),
    },
];

/// SD-24 Epic 6 criterion 6.1 — equipment field-coverage audit row.
/// Mirrors `rules_tables::crb::equipment_tables::EquipmentFieldCoverage`'s
/// shape. Every field is computed from `EQUIPMENT_TABLE`'s real content
/// or a documented corpus record count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    /// Records currently in `EQUIPMENT_TABLE`.
    pub total_records: u32,
    /// Real, active (non-`.MOD`) record count in the single `acg_equip.lst`.
    pub records_expected: u32,
    /// Records with `cost_gp.is_some()`.
    pub has_cost: u32,
    /// Records with a `weight` field populated. Always 0: `EquipmentTableEntry`
    /// has no `weight` field at all today -- see criterion 6.3.
    pub has_weight: u32,
    /// Records with a `description` field populated. Always 0:
    /// `EquipmentTableEntry` has no `description` field at all today --
    /// see criterion 6.4.
    pub has_description: u32,
}

/// Computes this book's equipment field-coverage audit row. See
/// `EquipmentFieldCoverage`'s own field doc comments for methodology.
pub fn field_coverage_report() -> EquipmentFieldCoverage {
    EquipmentFieldCoverage {
        total_records: EQUIPMENT_TABLE.len() as u32,
        records_expected: 221,
        has_cost: EQUIPMENT_TABLE.iter().filter(|entry| entry.cost_gp.is_some()).count() as u32,
        has_weight: 0,
        has_description: 0,
    }
}

/// Resolves an ACG equipment item by name, scoped to `RuleSetId::Acg`.
/// Returns `None` for any other rule set (cross-book invariant, mirrors
/// `acg::class_chassis_resolve`), and `None` when the key isn't in the
/// bootstrap sample above.
pub fn equipment_resolve(key: &str, rule_set: RuleSetId) -> Option<&'static EquipmentTableEntry> {
    if rule_set != RuleSetId::Acg {
        return None;
    }
    EQUIPMENT_TABLE.iter().find(|entry| entry.key == key)
}
