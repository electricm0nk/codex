//! SD-19 class progression catalog browser — Tauri command adapter over the
//! full CRB class table store (`rules_tables::crb::class_tables`, every
//! class's level-1 through its `max_supported_level` ceiling: BAB and the
//! three base saves).
//!
//! Distinct from the Character Sheet: this is a standalone catalog view of
//! every real class-progression row the engine knows about, not what one
//! character has selected. Built to satisfy the operator's full
//! "UI-surfacing" bar for the SD-19 `class.*` matrix rows — literal display
//! of every level of every class, not just a per-character sample. Mirrors
//! `sd19_equipment_catalog.rs` / `sd19_spell_catalog.rs` exactly.

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::crb::class_tables::class_tables;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassCatalogEntryDto {
    /// The `ClassId` variant name verbatim (e.g. "Fighter").
    pub class_id: String,
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassCatalogResponse {
    pub entries: Vec<ClassCatalogEntryDto>,
}

fn map_catalog_entry(
    row: &codex::rules_core::rules_tables::crb::class_tables::ClassTableRow,
) -> ClassCatalogEntryDto {
    ClassCatalogEntryDto {
        class_id: format!("{:?}", row.class_id),
        level: row.level,
        base_attack_bonus: row.base_attack_bonus,
        fort_save: row.fort_save,
        ref_save: row.ref_save,
        will_save: row.will_save,
    }
}

/// Build the full catalog response. A thin, testable wrapper behind the
/// Tauri command below (mirroring `sd19_equipment_catalog`'s and
/// `sd19_spell_catalog`'s own command/pure-fn split).
pub fn build_class_catalog() -> ClassCatalogResponse {
    ClassCatalogResponse {
        entries: class_tables().iter().map(map_catalog_entry).collect(),
    }
}

#[tauri::command]
pub fn list_class_catalog() -> ClassCatalogResponse {
    build_class_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_eleven_classes_and_expected_counts() {
        let response = build_class_catalog();
        assert_eq!(response.entries.len(), 207);

        let counts = |class_id: &str| {
            response
                .entries
                .iter()
                .filter(|e| e.class_id == class_id)
                .count()
        };
        assert_eq!(counts("Barbarian"), 20);
        assert_eq!(counts("Bard"), 20);
        assert_eq!(counts("Cleric"), 20);
        assert_eq!(counts("Druid"), 15);
        assert_eq!(counts("Fighter"), 20);
        assert_eq!(counts("Monk"), 12);
        assert_eq!(counts("Paladin"), 20);
        assert_eq!(counts("Ranger"), 20);
        assert_eq!(counts("Rogue"), 20);
        assert_eq!(counts("Sorcerer"), 20);
        assert_eq!(counts("Wizard"), 20);
    }

    #[test]
    fn every_entry_has_a_non_empty_class_id_and_positive_level() {
        let response = build_class_catalog();
        for entry in &response.entries {
            assert!(!entry.class_id.is_empty());
            assert!(entry.level >= 1);
        }
    }
}
