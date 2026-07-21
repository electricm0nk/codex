//! APG shared equipment tables — bootstrap/representative sample.
//!
//! Source: real PCGen corpus records, copied verbatim (name, `COST:`,
//! `WT:`) from `apg_equip_general.lst`, `apg_equip_arms_armor.lst`, and
//! `apg_equip_magic_items.lst` (Advanced Player's Guide). Bootstrap
//! coverage: one representative item per category, mirroring
//! `rules_tables::crb::equipment_tables`'s own "one representative item
//! per category" bootstrap philosophy. Exhaustive per-category coverage
//! is later loop work, one category per cycle. SD-22 Epic 3 criterion 9.
//!
//! - `Iron Spike` — `apg_equip_general.lst`, `COST:0.05 WT:1`.
//! - `Arrow (Blunt)` — `apg_equip_arms_armor.lst`, `COST:0.1 WT:0.15`.
//! - `Knucklebone of Fickle Fortune` — `apg_equip_magic_items.lst`, `COST:0 WT:0.01`.
//!
//! **Alchemist bombs are not equipment-table records.** Bombs are a
//! `Su` class feature computed by formula from class level and Int
//! modifier, not a purchasable item — there is no `Bomb`/`Acid Bomb`
//! record in any `apg_equip_*.lst` file (confirmed by direct inspection
//! of all three files). A prior planning doc
//! (`corpus-source-inventory.md §1.3`'s illustrative
//! `apg:alchemist:bomb:acid` example) is not authoritative per that
//! file's own corrective banner; do not add a fabricated bomb-item row
//! here to satisfy it.

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
    /// `rules_tables::crb::equipment_tables` documents for its own
    /// `key` field.
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. `f64` because
    /// real corpus costs are frequently fractional. `Some(0.0)` for
    /// artifact-priced items (`COST:0`, meaning priceless/not for sale)
    /// per the real `Knucklebone of Fickle Fortune` record.
    pub cost_gp: Option<f64>,
}

pub const EQUIPMENT_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry {
        key: "Iron Spike",
        category: EquipmentCategory::General,
        name: "Iron Spike",
        cost_gp: Some(0.05),
    },
    EquipmentTableEntry {
        key: "Arrow (Blunt)",
        category: EquipmentCategory::ArmsArmor,
        name: "Arrow, Blunt",
        cost_gp: Some(0.1),
    },
    EquipmentTableEntry {
        key: "Knucklebone of Fickle Fortune",
        category: EquipmentCategory::MagicItems,
        name: "Knucklebone of Fickle Fortune",
        cost_gp: Some(0.0),
    },
];

/// SD-24 Epic 6 criterion 6.1 — equipment field-coverage audit row.
/// Mirrors `rules_tables::crb::equipment_tables::EquipmentFieldCoverage`'s
/// shape so the audit can iterate a book's whole equipment table the same
/// way regardless of book. Every field is computed from `EQUIPMENT_TABLE`'s
/// real content or a documented corpus record count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    /// Records currently in `EQUIPMENT_TABLE`.
    pub total_records: u32,
    /// Real, active (non-`.MOD`) record count across
    /// `apg_equip_general.lst` (94) + `apg_equip_arms_armor.lst` (76) +
    /// `apg_equip_magic_items.lst` (171).
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
        records_expected: 94 + 76 + 171,
        has_cost: EQUIPMENT_TABLE.iter().filter(|entry| entry.cost_gp.is_some()).count() as u32,
        has_weight: 0,
        has_description: 0,
    }
}

/// Resolves an APG equipment item by name, scoped to `RuleSetId::Apg`.
/// Returns `None` for any other rule set (cross-book invariant, mirrors
/// `apg::class_chassis_resolve`), and `None` when the key isn't in the
/// bootstrap sample above.
pub fn equipment_resolve(key: &str, rule_set: RuleSetId) -> Option<&'static EquipmentTableEntry> {
    if rule_set != RuleSetId::Apg {
        return None;
    }
    EQUIPMENT_TABLE.iter().find(|entry| entry.key == key)
}
