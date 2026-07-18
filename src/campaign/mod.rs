//! Campaign snapshot types and local-folder persistence boundary.
//!
//! Campaigns aren't rules-computation, so this module lives as a sibling of
//! `saved_character` rather than under `rules_core` (SD-21 Epic 2
//! engine-shape addendum, 2026-07-18). `CampaignSnapshot`'s fields mirror
//! `apps/desktop/src/campaign/campaignModel.ts`'s `Campaign` +
//! `CampaignAssets` types 1:1, plus a `schema_version` added from day one
//! (per the `saved_character` precedent).

pub mod local_store;

use serde::{Deserialize, Serialize};

/// Persisted-artifact schema version for `CampaignSnapshot`. There is no
/// prior on-disk version to be back-compatible with — `campaign_config_json`
/// has never carried a schema version before this module existed — so this
/// starts at 1, matching the `saved_character` precedent of versioning from
/// day one rather than retrofitting later.
pub const CURRENT_CAMPAIGN_SCHEMA_VERSION: u16 = 1;

fn default_schema_version() -> u16 {
    CURRENT_CAMPAIGN_SCHEMA_VERSION
}

/// One invited campaign member. Mirrors `campaignModel.ts`'s `CampaignMember`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignMember {
    pub email: String,
    pub invited: bool,
}

/// One markdown-backed asset (a resource, adventure-log entry, map, or wiki
/// page). Mirrors `campaignModel.ts`'s `MarkdownAsset`, dropping the
/// UI-local `id`/`updatedAt` bookkeeping fields that only matter to the
/// browser-side list rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignAsset {
    pub title: String,
    pub body: String,
}

/// The four markdown-backed asset lists. Mirrors `campaignModel.ts`'s
/// `CampaignAssets`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CampaignAssets {
    #[serde(default)]
    pub resources: Vec<CampaignAsset>,
    #[serde(default)]
    pub adventure_log: Vec<CampaignAsset>,
    #[serde(default)]
    pub maps: Vec<CampaignAsset>,
    #[serde(default)]
    pub wiki: Vec<CampaignAsset>,
}

/// The full campaign snapshot: `campaignModel.ts`'s `Campaign` record plus
/// its `CampaignAssets`, as one persistable unit.
///
/// `party_character_ids` are keyed on the existing character-hub id space
/// (`SavedCharacterSummary`/`CharacterSummaryDto.character_id`, as already
/// used by `list_saved_characters`) — this module does not invent a new id
/// type for party membership.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignSnapshot {
    /// Not present in `campaign_config_json` payloads written before this
    /// module existed, so a missing value on load defaults to the current
    /// version rather than failing to parse.
    #[serde(default = "default_schema_version")]
    pub schema_version: u16,
    pub id: String,
    pub name: String,
    pub rule_set_id: String,
    pub rule_set_label: String,
    pub description: String,
    #[serde(default)]
    pub members: Vec<CampaignMember>,
    #[serde(default)]
    pub party_character_ids: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub assets: CampaignAssets,
}

/// Flat error type for the campaign store, mirroring
/// `saved_character`'s `SavedCharacterStoreError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CampaignStoreError {
    pub message: String,
}

/// Summary of one campaign, as returned by
/// `local_store::CampaignStore::list_all` without loading every asset body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CampaignSummary {
    pub id: String,
    pub name: String,
    pub rule_set_label: String,
    pub updated_at: String,
    pub party_size: usize,
    /// Directory name under the campaigns root this campaign lives in —
    /// callers need this to address a specific campaign for load/delete.
    pub folder_name: String,
}

/// One campaign folder that could not be loaded as part of a listing, named
/// so a caller can report which entry is corrupt. Mirrors
/// `SavedCharacterListingError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CampaignListingError {
    pub entry_name: String,
    pub message: String,
}

/// The result of listing every campaign under a campaigns-root directory.
/// One unreadable entry never fails the whole listing (mirrors
/// `SavedCharacterListing`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CampaignListing {
    pub campaigns: Vec<CampaignSummary>,
    pub unreadable_entries: Vec<CampaignListingError>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> CampaignSnapshot {
        CampaignSnapshot {
            schema_version: CURRENT_CAMPAIGN_SCHEMA_VERSION,
            id: "campaign-1".to_owned(),
            name: "The Void Between".to_owned(),
            rule_set_id: "crb".to_owned(),
            rule_set_label: "Core Rulebook".to_owned(),
            description: "A test campaign.".to_owned(),
            members: vec![CampaignMember {
                email: "dm@example.com".to_owned(),
                invited: true,
            }],
            party_character_ids: vec!["character-1".to_owned(), "character-2".to_owned()],
            created_at: "2026-07-18T00:00:00Z".to_owned(),
            updated_at: "2026-07-18T00:00:00Z".to_owned(),
            assets: CampaignAssets {
                resources: vec![CampaignAsset {
                    title: "Primer".to_owned(),
                    body: "# Primer".to_owned(),
                }],
                adventure_log: vec![],
                maps: vec![],
                wiki: vec![],
            },
        }
    }

    #[test]
    fn serializes_camel_case_field_names_matching_campaign_model_ts() {
        let json = serde_json::to_value(sample_snapshot()).expect("serializes");
        assert_eq!(json["ruleSetId"], "crb");
        assert_eq!(json["ruleSetLabel"], "Core Rulebook");
        assert_eq!(json["partyCharacterIds"][0], "character-1");
        assert_eq!(json["createdAt"], "2026-07-18T00:00:00Z");
        assert_eq!(json["members"][0]["invited"], true);
        assert_eq!(json["assets"]["resources"][0]["title"], "Primer");
        assert_eq!(json["schemaVersion"], 1);
    }

    #[test]
    fn round_trips_through_json() {
        let snapshot = sample_snapshot();
        let json = serde_json::to_string(&snapshot).expect("serializes");
        let parsed: CampaignSnapshot = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(parsed, snapshot);
    }

    #[test]
    fn deserializing_a_campaign_config_json_with_no_schema_version_or_assets_defaults_them() {
        // This is exactly the shape `campaignModel.ts` sends today via
        // `campaignConfigJson: JSON.stringify(campaign, null, 2)` — no
        // `schemaVersion` field (didn't exist yet) and no `assets` field
        // (assets travel separately in the request's `assets` field).
        let legacy_json = r#"{
            "id": "campaign-1",
            "name": "The Void Between",
            "ruleSetId": "crb",
            "ruleSetLabel": "Core Rulebook",
            "description": "A test campaign.",
            "members": [],
            "partyCharacterIds": [],
            "createdAt": "2026-07-18T00:00:00Z",
            "updatedAt": "2026-07-18T00:00:00Z"
        }"#;
        let parsed: CampaignSnapshot = serde_json::from_str(legacy_json).expect("deserializes");
        assert_eq!(parsed.schema_version, CURRENT_CAMPAIGN_SCHEMA_VERSION);
        assert_eq!(parsed.assets, CampaignAssets::default());
    }
}
