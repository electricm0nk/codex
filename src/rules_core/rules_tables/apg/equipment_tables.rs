//! APG shared equipment tables — full corpus ingest (SD-24 Epic 6
//! criteria 6.2/6.3/6.4).
//!
//! Source: every real, active PCGen corpus record from
//! `apg_equip_general.lst`, `apg_equip_arms_armor.lst`, and
//! `apg_equip_magic_items.lst` (Advanced Player's Guide) — 338 total, in
//! `equipment_data::EQUIPMENT_RECORDS` (see that module's doc comment for
//! full sourcing methodology and the 341 -> 338 audit correction).
//! Supersedes the SD-22 Epic 3 bootstrap (one representative item per
//! category); this is exhaustive per-category coverage, per
//! `scope-draft.md §2.5` and
//! `docs/release/SD-24-beta-readiness-and-multiclass/epic-breakdown.md`
//! criterion 6.2.
//!
//! - `Iron Spike` — `apg_equip_general.lst`, `COST:0.05 WT:1`.
//! - `Arrow (Blunt)` — `apg_equip_arms_armor.lst`, `COST:0.1 WT:0.15`.
//! - `Knucklebone of Fickle Fortune` — `apg_equip_magic_items.lst`, `COST:0 WT:0.01`.
//!
//! (the three records above are the original SD-22 bootstrap sample,
//! still present and unchanged among the full 338.)
//!
//! **Correction to the prior "no Bomb record" claim.** An earlier
//! version of this doc comment asserted no `Bomb`/`Acid Bomb` record
//! exists in any `apg_equip_*.lst` file. The full-corpus ingest finds
//! this to be only half true: `apg_equip_arms_armor.lst` does carry one
//! real `Bomb` record (`PROFICIENCY:WEAPON|Bomb`,
//! `TYPE:Weapon.Exotic.Ranged.Standard.Thrown.Splash.Fire...`) — not a
//! purchasable consumable, but a `BONUS:WEAPON|TOHIT|4|...` to-hit
//! template the Alchemist's bomb attack roll references (`cost_gp`/
//! `weight`/`description` are all `None` for it, honestly, since the
//! corpus line carries none of those tokens — it isn't priced or
//! weighed because it isn't sold). There is still no per-damage-type
//! `Acid Bomb`/`Fire Bomb`/... record; bomb damage itself remains a
//! class-feature formula, not equipment-table data. A prior planning doc
//! (`corpus-source-inventory.md §1.3`'s illustrative
//! `apg:alchemist:bomb:acid` example) is not authoritative per that
//! file's own corrective banner; do not add a fabricated per-damage-type
//! bomb-item row here to satisfy it.

use crate::rules_core::rules_tables::RuleSetId;

pub use super::equipment_data::EQUIPMENT_RECORDS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    General,
    ArmsArmor,
    MagicItems,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    /// Equipment records carry their `name` as the corpus identity by
    /// default (no distinct `KEY:` token on most rows), same fallback
    /// `rules_tables::crb::equipment_tables` documents for its own `key`
    /// field. A handful of `arms_armor` rows do carry a real `KEY:`
    /// token, which wins when present.
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. `f64` because
    /// real corpus costs are frequently fractional. `Some(0.0)` for
    /// artifact-priced items (`COST:0`, meaning priceless/not for sale)
    /// per the real `Knucklebone of Fickle Fortune` record.
    pub cost_gp: Option<f64>,
    /// Weight in pounds from the corpus `WT:` token. `None` for the 19 of
    /// 338 records whose corpus line carries no `WT:` token at all (a
    /// real, honest gap — see `equipment_data`'s doc comment) — never a
    /// fabricated `Some(0.0)`.
    pub weight: Option<f64>,
    /// Prose description. `None` for every one of the 338 records today:
    /// direct inspection of all three APG equipment corpus files finds
    /// zero `DESC:` tokens on any equipment row (a genuine corpus
    /// limitation, not a parsing gap — see `equipment_data`'s doc
    /// comment and this cycle's receipt).
    pub description: Option<&'static str>,
}

pub const EQUIPMENT_TABLE: &[EquipmentTableEntry] = EQUIPMENT_RECORDS;

/// SD-24 Epic 6 criterion 6.1 — equipment field-coverage audit row.
/// Mirrors `rules_tables::crb::equipment_tables::EquipmentFieldCoverage`'s
/// shape so the audit can iterate a book's whole equipment table the same
/// way regardless of book. Every field is computed from `EQUIPMENT_TABLE`'s
/// real content or a documented corpus record count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    /// Records currently in `EQUIPMENT_TABLE`.
    pub total_records: u32,
    /// Real, active, deduplicated-by-name record count across
    /// `apg_equip_general.lst` (93) + `apg_equip_arms_armor.lst` (75) +
    /// `apg_equip_magic_items.lst` (170) = 338. Corrected from the
    /// criterion 6.1 audit's originally-documented 341 (94+76+171) — see
    /// `equipment_data`'s module doc comment for the off-by-one-per-file
    /// `SOURCELONG:` header-line miscount this corrects.
    pub records_expected: u32,
    /// Records with `cost_gp.is_some()`.
    pub has_cost: u32,
    /// Records with `weight.is_some()` -- a real per-row count (319/338;
    /// the remaining 19 have no `WT:` token in the corpus).
    pub has_weight: u32,
    /// Records with `description.is_some()` -- always 0 today: the real
    /// APG equipment corpus carries no `DESC:` token on any row (see
    /// `equipment_data`'s doc comment).
    pub has_description: u32,
}

/// Computes this book's equipment field-coverage audit row. See
/// `EquipmentFieldCoverage`'s own field doc comments for methodology.
pub fn field_coverage_report() -> EquipmentFieldCoverage {
    EquipmentFieldCoverage {
        total_records: EQUIPMENT_TABLE.len() as u32,
        records_expected: 93 + 75 + 170,
        has_cost: EQUIPMENT_TABLE.iter().filter(|entry| entry.cost_gp.is_some()).count() as u32,
        has_weight: EQUIPMENT_TABLE.iter().filter(|entry| entry.weight.is_some()).count() as u32,
        has_description: EQUIPMENT_TABLE.iter().filter(|entry| entry.description.is_some()).count() as u32,
    }
}

/// Resolves an APG equipment item by name, scoped to `RuleSetId::Apg`.
/// Returns `None` for any other rule set (cross-book invariant, mirrors
/// `apg::class_chassis_resolve`), and `None` when the key isn't in
/// `EQUIPMENT_TABLE`.
pub fn equipment_resolve(key: &str, rule_set: RuleSetId) -> Option<&'static EquipmentTableEntry> {
    if rule_set != RuleSetId::Apg {
        return None;
    }
    EQUIPMENT_TABLE.iter().find(|entry| entry.key == key)
}
