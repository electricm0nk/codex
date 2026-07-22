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

/// Filter criteria for `list_equipment`. Every field is optional and
/// `None`/empty matches everything — an all-`None` filter is equivalent to
/// the unfiltered `list_equipment_catalog` response. Kept deliberately
/// narrow (substring name match, exact category match) rather than an
/// exhaustive query DSL; widen only if a real caller needs more.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentCatalogFilter {
    /// Case-insensitive substring match against `name`.
    pub name_contains: Option<String>,
    /// Exact match against the `EquipmentCategory` variant name verbatim
    /// (e.g. "ArmsArmor"), as projected onto `EquipmentCatalogEntryDto::category`.
    pub category: Option<String>,
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
        .collect();

    EquipmentCatalogResponse { entries }
}

/// Returns the CRB equipment catalog narrowed by `filter` — see
/// `EquipmentCatalogFilter`'s own doc comment for the supported fields.
/// Distinct from `list_equipment_catalog` (kept unfiltered so the existing
/// `loadEquipmentCatalog` desktop boundary caller is untouched this cycle);
/// this command is the new, additive filtered surface Criterion 19 asks for.
#[tauri::command]
pub fn list_equipment(filter: EquipmentCatalogFilter) -> EquipmentCatalogResponse {
    filter_equipment_catalog(&filter)
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
    fn filter_equipment_catalog_matches_category_exactly() {
        let response = filter_equipment_catalog(&EquipmentCatalogFilter {
            name_contains: None,
            category: Some("ArmsArmor".to_owned()),
        });

        assert_eq!(response.entries.len(), 310);
        for entry in &response.entries {
            assert_eq!(entry.category, "ArmsArmor");
        }
    }

    #[test]
    fn filter_equipment_catalog_combines_name_and_category_filters() {
        let response = filter_equipment_catalog(&EquipmentCatalogFilter {
            name_contains: Some("shield".to_owned()),
            category: Some("MagicItems".to_owned()),
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has known Shield-named magic items (e.g. Ring of Force Shield)"
        );
        for entry in &response.entries {
            assert_eq!(entry.category, "MagicItems");
            assert!(entry.name.to_lowercase().contains("shield"));
        }
    }
}
