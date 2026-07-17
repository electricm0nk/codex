//! SD-19 race trait catalog browser — Tauri command adapter over the full
//! CRB race trait table store (`rules_tables::crb::race_tables`, every
//! race's grounded ability-modifier/size/speed/senses/special-trait
//! recognition bundle, transcribed from `pilot_compute.rs`'s per-race
//! explanation seams).
//!
//! Distinct from the Character Sheet: this is a standalone catalog view of
//! every real race-trait row the engine knows about, not what one character
//! has selected. Built to satisfy the operator's full "UI-surfacing" bar for
//! the SD-19 `race.*` matrix rows — literal display of every trait of every
//! race, not just a per-character sample. Mirrors `sd19_class_catalog.rs` /
//! `sd19_equipment_catalog.rs` / `sd19_spell_catalog.rs` exactly.

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::crb::race_tables::race_traits;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceCatalogEntryDto {
    /// The `RaceId` variant name verbatim (e.g. "HalfElf").
    pub race_id: String,
    pub trait_name: String,
    pub value: i16,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceCatalogResponse {
    pub entries: Vec<RaceCatalogEntryDto>,
}

fn map_catalog_entry(
    entry: &codex::rules_core::rules_tables::crb::race_tables::RaceTraitEntry,
) -> RaceCatalogEntryDto {
    RaceCatalogEntryDto {
        race_id: format!("{:?}", entry.race_id),
        trait_name: entry.trait_name.to_owned(),
        value: entry.value,
        detail: entry.detail.to_owned(),
    }
}

/// Build the full catalog response. A thin, testable wrapper behind the
/// Tauri command below (mirroring `sd19_class_catalog`'s own
/// command/pure-fn split).
pub fn build_race_catalog() -> RaceCatalogResponse {
    RaceCatalogResponse {
        entries: race_traits().iter().map(map_catalog_entry).collect(),
    }
}

#[tauri::command]
pub fn list_race_catalog() -> RaceCatalogResponse {
    build_race_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_seven_races_and_expected_counts() {
        let response = build_race_catalog();
        assert_eq!(response.entries.len(), 49);

        let counts = |race_id: &str| {
            response
                .entries
                .iter()
                .filter(|e| e.race_id == race_id)
                .count()
        };
        assert_eq!(counts("Human"), 6);
        assert_eq!(counts("Dwarf"), 9);
        assert_eq!(counts("Elf"), 7);
        assert_eq!(counts("Gnome"), 8);
        assert_eq!(counts("HalfElf"), 6);
        assert_eq!(counts("HalfOrc"), 5);
        assert_eq!(counts("Halfling"), 8);
    }

    #[test]
    fn every_entry_has_a_non_empty_race_id_and_trait_name() {
        let response = build_race_catalog();
        for entry in &response.entries {
            assert!(!entry.race_id.is_empty());
            assert!(!entry.trait_name.is_empty());
            assert!(!entry.detail.is_empty());
        }
    }
}
