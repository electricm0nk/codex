//! SD-19 spell catalog browser — Tauri command adapter over the full CRB
//! spell list (`rules_tables::crb::spell_list::SPELL_LIST`, all 652 real
//! corpus records across all 9 PF1 strict schools).
//!
//! Distinct from the Character Sheet's Spells tab: this is a standalone
//! catalog view of every real spell record the engine knows about, not
//! what one character has selected. Built to satisfy the operator's full
//! "UI-surfacing" bar for the SD-19 remaining-school matrix rows —
//! literal display of every spell in every school, not just a
//! per-character sample. Mirrors `sd19_equipment_catalog.rs` exactly.

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogEntryDto {
    pub key: String,
    /// The `Pf1SchoolId` variant name verbatim (e.g. "Abjuration").
    pub school: String,
    pub level: u8,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogResponse {
    pub entries: Vec<SpellCatalogEntryDto>,
}

fn map_catalog_entry(
    entry: &codex::rules_core::rules_tables::crb::spell_list::SpellListEntry,
) -> SpellCatalogEntryDto {
    SpellCatalogEntryDto {
        key: entry.key.to_string(),
        school: format!("{:?}", entry.school),
        level: entry.level,
        description: entry.description.to_string(),
    }
}

/// Build the full catalog response. A thin, testable wrapper behind the
/// Tauri command below (mirroring `sd19_equipment_catalog`'s own
/// command/pure-fn split).
pub fn build_spell_catalog() -> SpellCatalogResponse {
    SpellCatalogResponse {
        entries: SPELL_LIST.iter().map(map_catalog_entry).collect(),
    }
}

#[tauri::command]
pub fn list_spell_catalog() -> SpellCatalogResponse {
    build_spell_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_nine_schools_and_expected_counts() {
        let response = build_spell_catalog();
        assert_eq!(response.entries.len(), 652);

        let counts = |school: &str| {
            response
                .entries
                .iter()
                .filter(|e| e.school == school)
                .count()
        };
        assert_eq!(counts("Abjuration"), 73);
        assert_eq!(counts("Conjuration"), 116);
        assert_eq!(counts("Divination"), 50);
        assert_eq!(counts("Enchantment"), 60);
        assert_eq!(counts("Evocation"), 87);
        assert_eq!(counts("Illusion"), 47);
        assert_eq!(counts("Necromancy"), 62);
        assert_eq!(counts("Transmutation"), 152);
        assert_eq!(counts("Universal"), 5);
    }

    #[test]
    fn every_entry_has_a_non_empty_key_and_description() {
        let response = build_spell_catalog();
        for entry in &response.entries {
            assert!(!entry.key.is_empty());
            assert!(!entry.description.is_empty());
        }
    }
}
