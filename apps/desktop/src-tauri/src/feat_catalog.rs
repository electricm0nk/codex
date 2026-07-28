//! v0.6 alpha swarm feat catalog browser — Tauri command adapter over the
//! full CRB feat table store (`rules_tables::crb::feats`, all 185 real
//! corpus records across all 4 CRB feat categories).
//!
//! Mirrors `equipment_catalog.rs`'s own command/pure-fn split and
//! unfiltered/filtered command pair exactly — this is a standalone catalog
//! view of every real feat record the engine knows about, for the
//! frontend's Feat picker (which does not exist yet; this is the backend
//! half of "expose + consume", the existing 185-record catalog just had
//! zero Tauri exposure before this).

use serde::{Deserialize, Serialize};

use codex::rules_core::feat_effects;
use codex::rules_core::rules_tables::crb::feats::feat_tables;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatCatalogEntryDto {
    pub key: String,
    /// The `FeatCategory` variant name verbatim (e.g. "Combat").
    pub category: String,
    pub name: String,
    pub description: Option<String>,
    /// `"Weapon"`, `"Skill"` or `"SpellSchool"` for a feat whose target the
    /// engine consumes; `None` for every other feat.
    ///
    /// This is deliberately narrower than the corpus: many more feats carry a
    /// `CHOOSE:` token, but a target recorded against a feat no producer
    /// reads would render in the picker and change nothing computed. Only
    /// the feats in `feat_effects::CHOOSER_FEAT_CONTRACTS` are marked, so a
    /// prompt shown to a player always leads to real arithmetic.
    pub chooser_target_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatCatalogResponse {
    pub entries: Vec<FeatCatalogEntryDto>,
}

fn map_catalog_entry(
    entry: &codex::rules_core::rules_tables::crb::feats::FeatTableEntry,
) -> FeatCatalogEntryDto {
    FeatCatalogEntryDto {
        key: entry.key.to_string(),
        category: format!("{:?}", entry.category),
        name: entry.name.to_string(),
        description: entry.description.map(|d| d.to_string()),
        chooser_target_kind: feat_effects::chooser_contract_for_feat(entry.key)
            .map(|contract| format!("{:?}", contract.target_kind)),
    }
}

/// Build the full catalog response. A thin, testable wrapper behind the
/// Tauri command below — mirrors `equipment_catalog::build_equipment_catalog`.
pub fn build_feat_catalog() -> FeatCatalogResponse {
    FeatCatalogResponse {
        entries: feat_tables().iter().map(map_catalog_entry).collect(),
    }
}

#[tauri::command]
pub fn list_feat_catalog() -> FeatCatalogResponse {
    build_feat_catalog()
}

/// Filter criteria for `list_feats`. Every field is optional and
/// `None`/empty matches everything — mirrors
/// `equipment_catalog::EquipmentCatalogFilter` exactly.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatCatalogFilter {
    /// Case-insensitive substring match against `name`.
    pub name_contains: Option<String>,
    /// Exact match against the `FeatCategory` variant name verbatim (e.g.
    /// "Combat"), as projected onto `FeatCatalogEntryDto::category`.
    pub category: Option<String>,
}

/// Narrows the full catalog to the entries matching `filter`. A thin,
/// testable wrapper behind the `list_feats` Tauri command below — mirrors
/// `equipment_catalog::filter_equipment_catalog`.
pub fn filter_feat_catalog(filter: &FeatCatalogFilter) -> FeatCatalogResponse {
    let name_needle = filter
        .name_contains
        .as_ref()
        .filter(|needle| !needle.is_empty())
        .map(|needle| needle.to_lowercase());

    let entries = build_feat_catalog()
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

    FeatCatalogResponse { entries }
}

/// Returns the CRB feat catalog narrowed by `filter` — see
/// `FeatCatalogFilter`'s own doc comment for the supported fields.
#[tauri::command]
pub fn list_feats(filter: FeatCatalogFilter) -> FeatCatalogResponse {
    filter_feat_catalog(&filter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_four_categories_and_expected_counts() {
        let response = build_feat_catalog();
        assert_eq!(response.entries.len(), 185);

        let counts = |category: &str| {
            response.entries.iter().filter(|e| e.category == category).count()
        };
        assert_eq!(counts("General"), 50);
        assert_eq!(counts("Combat"), 110);
        assert_eq!(counts("ItemCreation"), 8);
        assert_eq!(counts("Metamagic"), 17);
    }

    #[test]
    fn every_entry_has_a_non_empty_key_and_name() {
        let response = build_feat_catalog();
        for entry in &response.entries {
            assert!(!entry.key.is_empty());
            assert!(!entry.name.is_empty());
        }
    }

    #[test]
    fn filter_feat_catalog_with_no_filter_fields_returns_the_full_catalog() {
        let response = filter_feat_catalog(&FeatCatalogFilter::default());
        assert_eq!(response.entries.len(), build_feat_catalog().entries.len());
    }

    #[test]
    fn filter_feat_catalog_matches_name_contains_case_insensitively() {
        let response = filter_feat_catalog(&FeatCatalogFilter {
            name_contains: Some("dodge".to_owned()),
            category: None,
        });

        assert!(!response.entries.is_empty(), "the real CRB corpus has a Dodge feat");
        assert!(response.entries.len() < build_feat_catalog().entries.len());
        for entry in &response.entries {
            assert!(entry.name.to_lowercase().contains("dodge"), "{:?}", entry.name);
        }
    }

    #[test]
    fn filter_feat_catalog_matches_category_exactly() {
        let response = filter_feat_catalog(&FeatCatalogFilter {
            name_contains: None,
            category: Some("Metamagic".to_owned()),
        });

        assert_eq!(response.entries.len(), 17);
        for entry in &response.entries {
            assert_eq!(entry.category, "Metamagic");
        }
    }

    #[test]
    fn filter_feat_catalog_combines_name_and_category_filters() {
        let response = filter_feat_catalog(&FeatCatalogFilter {
            name_contains: Some("spell".to_owned()),
            category: Some("Metamagic".to_owned()),
        });

        assert!(
            !response.entries.is_empty(),
            "the real CRB corpus has known metamagic feats with 'spell' in the name (e.g. Still Spell)"
        );
        for entry in &response.entries {
            assert_eq!(entry.category, "Metamagic");
            assert!(entry.name.to_lowercase().contains("spell"));
        }
    }
}

/// The real corpus weapon list, for the "which weapon?" step of adding a
/// chooser feat.
///
/// Sourced from `rules_tables::crb::weapon_tables::WEAPON_TABLE` -- the same
/// 106 ingested records the per-weapon attack/damage/threat-range totals are
/// computed from. Deliberately NOT the arms-and-armor equipment catalog:
/// that mixes armor and shields in, and offering "Chain Shirt" as a Weapon
/// Focus target would let a player record a choice no producer can honour.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponTargetDto {
    /// The weapon's corpus key, which is what a chooser feat's target names.
    pub key: String,
    /// e.g. `"1d8 · threat 19-20/x2"` -- enough to tell similar weapons apart
    /// in the picker without opening anything.
    pub detail: String,
}

pub fn build_weapon_target_list() -> Vec<WeaponTargetDto> {
    use codex::rules_core::rules_tables::crb::weapon_tables::{
        weapon_critical_threat_low, WEAPON_TABLE,
    };

    WEAPON_TABLE
        .iter()
        .map(|entry| WeaponTargetDto {
            key: entry.key.to_string(),
            detail: format!(
                "{} · threat {}-20/x{}",
                entry.damage_die,
                weapon_critical_threat_low(entry),
                entry.critical_multiplier
            ),
        })
        .collect()
}

#[tauri::command]
pub fn list_weapon_targets() -> Vec<WeaponTargetDto> {
    build_weapon_target_list()
}

#[cfg(test)]
mod weapon_target_tests {
    use super::*;

    #[test]
    fn the_weapon_target_list_is_the_real_ingested_table() {
        let targets = build_weapon_target_list();
        assert!(targets.len() > 100, "expected the full ingested table, got {}", targets.len());
        let longsword = targets
            .iter()
            .find(|t| t.key == "Longsword")
            .expect("Longsword must be offerable as a target");
        assert_eq!(longsword.detail, "1d8 · threat 19-20/x2");
    }

    /// Body armor must never appear -- a Weapon Focus target naming a Chain
    /// Shirt could be recorded and would then ground nothing.
    ///
    /// **Shields deliberately DO appear, and that is correct.** A shield
    /// bash is a real PF1 attack: the corpus gives `Shieldbash (Heavy
    /// Shield)` its own `1d4`/x2 record and Martial proficiency, so Weapon
    /// Focus (Heavy Steel Shield) is a legitimate build. An earlier version
    /// of this test asserted no key could contain "Shield" and failed --
    /// the data was right and the assertion was wrong.
    #[test]
    fn body_armor_is_not_offered_as_a_weapon_target_but_shields_are() {
        let targets = build_weapon_target_list();
        for armor in ["Chain Shirt", "Breastplate", "Full Plate", "Leather Armor"] {
            assert!(
                !targets.iter().any(|t| t.key.contains(armor)),
                "{armor} is not a weapon and must not be offerable as a target"
            );
        }
        assert!(
            targets.iter().any(|t| t.key.contains("Shieldbash")),
            "shield bash is a real weapon and must remain offerable"
        );
    }
}
