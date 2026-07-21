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

/// Filter criteria for `list_spells`. Every field is optional and
/// `None`/empty matches everything — an all-`None` filter is equivalent to
/// the unfiltered `list_spell_catalog` response. Kept deliberately narrow
/// (substring name match, exact school match) rather than an exhaustive
/// query DSL; widen only if a real caller needs more.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellCatalogFilter {
    /// Case-insensitive substring match against `key` (the spell's corpus
    /// identity/name — see `SpellCatalogEntryDto::key`'s doc comment).
    pub name_contains: Option<String>,
    /// Exact match against the `Pf1SchoolId` variant name verbatim (e.g.
    /// "Evocation"), as projected onto `SpellCatalogEntryDto::school`.
    pub school: Option<String>,
}

/// Narrows the full catalog to the entries matching `filter`. A thin,
/// testable wrapper behind the `list_spells` Tauri command below, mirroring
/// `build_spell_catalog`'s own command/pure-fn split.
pub fn filter_spell_catalog(filter: &SpellCatalogFilter) -> SpellCatalogResponse {
    let name_needle = filter
        .name_contains
        .as_ref()
        .filter(|needle| !needle.is_empty())
        .map(|needle| needle.to_lowercase());

    let entries = build_spell_catalog()
        .entries
        .into_iter()
        .filter(|entry| match &name_needle {
            Some(needle) => entry.key.to_lowercase().contains(needle.as_str()),
            None => true,
        })
        .filter(|entry| match &filter.school {
            Some(school) => &entry.school == school,
            None => true,
        })
        .collect();

    SpellCatalogResponse { entries }
}

/// Returns the CRB spell catalog narrowed by `filter` — see
/// `SpellCatalogFilter`'s own doc comment for the supported fields.
/// Distinct from `list_spell_catalog` (kept unfiltered so the existing
/// `loadSpellCatalog` desktop boundary caller is untouched this cycle); this
/// command is the new, additive filtered surface Criterion 19 asks for.
#[tauri::command]
pub fn list_spells(filter: SpellCatalogFilter) -> SpellCatalogResponse {
    filter_spell_catalog(&filter)
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

    #[test]
    fn filter_spell_catalog_with_no_filter_fields_returns_the_full_catalog() {
        let response = filter_spell_catalog(&SpellCatalogFilter::default());
        assert_eq!(response.entries.len(), build_spell_catalog().entries.len());
    }

    #[test]
    fn filter_spell_catalog_matches_name_contains_case_insensitively() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: Some("fireball".to_owned()),
            school: None,
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has a Fireball record"
        );
        assert!(response.entries.len() < build_spell_catalog().entries.len());
        for entry in &response.entries {
            assert!(
                entry.key.to_lowercase().contains("fireball"),
                "entry {:?} does not contain 'fireball'",
                entry.key
            );
        }
    }

    #[test]
    fn filter_spell_catalog_matches_school_exactly() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: None,
            school: Some("Evocation".to_owned()),
        });

        assert_eq!(response.entries.len(), 87);
        for entry in &response.entries {
            assert_eq!(entry.school, "Evocation");
        }
    }

    #[test]
    fn filter_spell_catalog_combines_name_and_school_filters() {
        let response = filter_spell_catalog(&SpellCatalogFilter {
            name_contains: Some("flame".to_owned()),
            school: Some("Evocation".to_owned()),
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has Evocation spells with 'flame' in the name (e.g. Flame \
             Blade, Flame Strike)"
        );
        for entry in &response.entries {
            assert_eq!(entry.school, "Evocation");
            assert!(entry.key.to_lowercase().contains("flame"));
        }
    }
}
