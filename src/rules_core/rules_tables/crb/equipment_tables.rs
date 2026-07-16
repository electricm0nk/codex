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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentTableEntry {
    /// The corpus `KEY:` token (equipment records carry an explicit key,
    /// unlike spells — see `spell_list.rs`).
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces, `None` when the corpus record has no `COST:`
    /// token (e.g. base-material equipmods).
    pub cost_gp: Option<u32>,
}

/// Sources: the four `core_rulebook` equipment corpus files.
pub const EQUIPMENT_TABLES: &[EquipmentTableEntry] = &[
    EquipmentTableEntry {
        key: "Longsword (Base)",
        category: EquipmentCategory::ArmsArmor,
        name: "Longsword",
        cost_gp: Some(15),
    },
    EquipmentTableEntry {
        key: "Backpack",
        category: EquipmentCategory::General,
        name: "Backpack",
        cost_gp: Some(2),
    },
    EquipmentTableEntry {
        key: "Potion of Aid",
        category: EquipmentCategory::MagicItems,
        name: "Potion of Aid",
        cost_gp: None,
    },
    EquipmentTableEntry {
        key: "Material ~ Cloth",
        category: EquipmentCategory::Equipmods,
        name: "Cloth",
        cost_gp: Some(0),
    },
];
