//! SD-19 equipment catalog browser — Tauri command adapter over the full
//! CRB equipment table store (`rules_tables::crb::equipment_tables`, all
//! ~2,977 real corpus records across all 4 core-rulebook categories).
//!
//! Distinct from `character_hub`'s per-character Gear tab: this is a
//! standalone catalog view of every real equipment record the engine
//! knows about, not what one character happens to have equipped. Built
//! to satisfy the operator's full "UI-surfacing" bar for the SD-19
//! equipment matrix rows — literal display of every item in every
//! category, not just a per-character sample.

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::crb::equipment_tables::equipment_tables;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentCatalogEntryDto {
    pub key: String,
    /// The `EquipmentCategory` variant name verbatim (e.g. "ArmsArmor").
    pub category: String,
    pub name: String,
    pub cost_gp: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentCatalogResponse {
    pub entries: Vec<EquipmentCatalogEntryDto>,
}

fn map_catalog_entry(
    entry: &codex::rules_core::rules_tables::crb::equipment_tables::EquipmentTableEntry,
) -> EquipmentCatalogEntryDto {
    EquipmentCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        cost_gp: entry.cost_gp,
    }
}

/// Build the full catalog response. A thin, testable wrapper behind the
/// Tauri command below (mirroring this codebase's other command/pure-fn
/// split, e.g. `ge08_workbench::build_ge08_workbench_snapshot`).
pub fn build_equipment_catalog() -> EquipmentCatalogResponse {
    EquipmentCatalogResponse {
        entries: equipment_tables().iter().map(map_catalog_entry).collect(),
    }
}

#[tauri::command]
pub fn list_equipment_catalog() -> EquipmentCatalogResponse {
    build_equipment_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_four_categories_and_expected_counts() {
        let response = build_equipment_catalog();
        assert_eq!(response.entries.len(), 2977);

        let counts = |category: &str| {
            response
                .entries
                .iter()
                .filter(|e| e.category == category)
                .count()
        };
        assert_eq!(counts("ArmsArmor"), 310);
        assert_eq!(counts("General"), 453);
        assert_eq!(counts("MagicItems"), 1556);
        assert_eq!(counts("Equipmods"), 658);
    }

    #[test]
    fn every_entry_has_a_non_empty_key_and_name() {
        let response = build_equipment_catalog();
        for entry in &response.entries {
            assert!(!entry.key.is_empty());
            assert!(!entry.name.is_empty());
        }
    }
}
