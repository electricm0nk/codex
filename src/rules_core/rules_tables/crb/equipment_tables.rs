//! PF1 CRB equipment tables — one representative item per category.
//!
//! Bootstrap coverage: one item per `core_rulebook/cr_equip_*.lst` /
//! `cr_equipmods.lst` category, copied verbatim (`KEY:`/name, `COST:`,
//! `WT:`) from the real PCGen corpus. Exhaustive per-category coverage
//! is the loop's job, one category per cycle, per `scope-draft.md` §2.5
//! ("a representative sample of items per round").

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    ArmsArmor,
    General,
    MagicItems,
    Equipmods,
}

impl EquipmentCategory {
    pub const ALL: &'static [EquipmentCategory] = &[
        EquipmentCategory::ArmsArmor,
        EquipmentCategory::General,
        EquipmentCategory::MagicItems,
        EquipmentCategory::Equipmods,
    ];

    /// Which `core_rulebook` corpus file this category's records live in.
    pub fn corpus_file_name(self) -> &'static str {
        match self {
            EquipmentCategory::ArmsArmor => "cr_equip_arms_armor.lst",
            EquipmentCategory::General => "cr_equip_general.lst",
            EquipmentCategory::MagicItems => "cr_equip_magic_items.lst",
            EquipmentCategory::Equipmods => "cr_equipmods.lst",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    /// The corpus `KEY:` token (equipment records carry an explicit key,
    /// unlike spells — see `spell_list.rs`), falling back to the record's
    /// `name` when no `KEY:` token is present (matching
    /// `equipment_resolver::equipment_key_token`'s own fallback).
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. `f64` because
    /// real corpus costs are frequently fractional (e.g. `0.05` for an
    /// arrow); `None` when the token is absent or non-numeric.
    pub cost_gp: Option<f64>,
}

/// Full CRB equipment table store: every real corpus record across all 4
/// `core_rulebook` equipment files, generated from the live corpus (see
/// `equipment_data/`'s own doc comment for the generation method — not
/// hand-authored, so there is no fabrication/transcription risk at this
/// scale). Built once and cached for the process lifetime.
pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<EquipmentTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(
            super::equipment_data::arms_armor::ARMS_ARMOR_TABLE.len()
                + super::equipment_data::general::GENERAL_TABLE.len()
                + super::equipment_data::magic_items::MAGIC_ITEMS_TABLE.len()
                + super::equipment_data::equipmods::EQUIPMODS_TABLE.len(),
        );
        all.extend_from_slice(super::equipment_data::arms_armor::ARMS_ARMOR_TABLE);
        all.extend_from_slice(super::equipment_data::general::GENERAL_TABLE);
        all.extend_from_slice(super::equipment_data::magic_items::MAGIC_ITEMS_TABLE);
        all.extend_from_slice(super::equipment_data::equipmods::EQUIPMODS_TABLE);
        all
    })
}
