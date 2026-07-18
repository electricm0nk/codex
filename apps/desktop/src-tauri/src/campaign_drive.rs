//! Real local-disk write-through for campaign "Drive" artifacts.
//!
//! `campaignModel.ts`'s design doc describes a Drive folder per campaign
//! (`<driveFolderPath>/<name>/`) holding a `.config/<name>.json` metadata
//! file plus markdown files for resources/adventure-log/maps/wiki. This
//! module is the thin Tauri-command adapter over the headless
//! `codex::campaign` crate (SD-21 Epic 2 engine-shape addendum, 2026-07-18):
//! it deserializes the frontend's already-JSON campaign payloads into a
//! typed `CampaignSnapshot` and delegates all real file I/O to
//! `codex::campaign::local_store::CampaignStore` — mirroring how
//! `character_hub.rs` wraps the headless `codex::saved_character` crate.
//!
//! Google OAuth / Drive API integration does not exist (see
//! `settings/googleDrive.ts`) and is out of scope for SD-21 — the
//! "Drive folder" is really just a local path (typically a
//! Drive-for-Desktop-synced directory) the user points the app at.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use codex::campaign::local_store::CampaignStore;
use codex::campaign::{CampaignAsset, CampaignAssets, CampaignSnapshot};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownAssetDto {
    pub title: String,
    pub body: String,
}

impl From<&MarkdownAssetDto> for CampaignAsset {
    fn from(dto: &MarkdownAssetDto) -> Self {
        CampaignAsset {
            title: dto.title.clone(),
            body: dto.body.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CampaignAssetsDto {
    #[serde(default)]
    pub resources: Vec<MarkdownAssetDto>,
    #[serde(default)]
    pub adventure_log: Vec<MarkdownAssetDto>,
    #[serde(default)]
    pub maps: Vec<MarkdownAssetDto>,
    #[serde(default)]
    pub wiki: Vec<MarkdownAssetDto>,
}

impl From<&CampaignAssetsDto> for CampaignAssets {
    fn from(dto: &CampaignAssetsDto) -> Self {
        CampaignAssets {
            resources: dto.resources.iter().map(CampaignAsset::from).collect(),
            adventure_log: dto.adventure_log.iter().map(CampaignAsset::from).collect(),
            maps: dto.maps.iter().map(CampaignAsset::from).collect(),
            wiki: dto.wiki.iter().map(CampaignAsset::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteCampaignDriveArtifactsRequest {
    pub drive_folder_path: String,
    /// Preserved verbatim for wire-shape compatibility with
    /// `writeCampaignDriveArtifacts.ts` (PR #320's shipped boundary
    /// contract) — the impl below now derives the folder name from
    /// `campaign_config_json`'s own `name` field instead, since that's the
    /// authoritative typed value once deserialized.
    #[allow(dead_code)]
    pub campaign_name: String,
    /// The full `Campaign` record, already JSON-serialized by the caller —
    /// deserialized below into a typed `CampaignSnapshot`.
    pub campaign_config_json: String,
    #[serde(default)]
    pub assets: CampaignAssetsDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteCampaignDriveArtifactsResponse {
    pub campaign_folder_path: String,
}

fn require_drive_folder_path(drive_folder_path: &str) -> Result<(), String> {
    if drive_folder_path.trim().is_empty() {
        return Err("no Drive folder configured — set one under Settings \u{2192} Google Drive first".to_owned());
    }
    Ok(())
}

/// Pure implementation, unit-testable without a `tauri::AppHandle` — the
/// command below is a thin wrapper, mirroring this codebase's established
/// command/pure-fn split (e.g. `ge08_workbench::build_ge08_workbench_snapshot`).
///
/// A thin adapter over `CampaignStore`: `request.campaign_config_json` is
/// the already-JSON `Campaign` record (no `assets` field — those travel
/// separately in `request.assets`), deserialized here into a
/// `CampaignSnapshot` before delegating to the real store.
pub fn write_campaign_drive_artifacts_impl(
    request: &WriteCampaignDriveArtifactsRequest,
) -> Result<WriteCampaignDriveArtifactsResponse, String> {
    require_drive_folder_path(&request.drive_folder_path)?;

    let mut snapshot: CampaignSnapshot = serde_json::from_str(&request.campaign_config_json)
        .map_err(|err| format!("invalid campaign_config_json: {err}"))?;
    snapshot.assets = CampaignAssets::from(&request.assets);

    let campaign_dir = CampaignStore::save_under_root(&snapshot, Path::new(&request.drive_folder_path))
        .map_err(|err| err.message)?;

    Ok(WriteCampaignDriveArtifactsResponse {
        campaign_folder_path: campaign_dir.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
pub fn write_campaign_drive_artifacts(
    request: WriteCampaignDriveArtifactsRequest,
) -> Result<WriteCampaignDriveArtifactsResponse, String> {
    write_campaign_drive_artifacts_impl(&request)
}

// --- E2.8: drive_list_campaigns / drive_load_campaign / drive_save_campaign
// / drive_delete_campaign, local-folder backed (OAuth/Drive-API descoped
// per the 2026-07-18 operator directive). ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveListCampaignsRequest {
    pub drive_folder_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignSummaryDto {
    pub id: String,
    pub name: String,
    pub rule_set_label: String,
    pub updated_at: String,
    pub party_size: usize,
    /// The directory name under `driveFolderPath` this campaign lives in —
    /// pass this back as `folderName` to `drive_load_campaign`/
    /// `drive_delete_campaign` to address this exact campaign.
    pub folder_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveListCampaignsResponse {
    pub campaigns: Vec<CampaignSummaryDto>,
    pub unreadable_count: usize,
}

pub fn drive_list_campaigns_impl(
    request: &DriveListCampaignsRequest,
) -> Result<DriveListCampaignsResponse, String> {
    let root = PathBuf::from(&request.drive_folder_path);
    let listing = CampaignStore::list_all(&root).map_err(|err| err.message)?;
    Ok(DriveListCampaignsResponse {
        campaigns: listing
            .campaigns
            .into_iter()
            .map(|summary| CampaignSummaryDto {
                id: summary.id,
                name: summary.name,
                rule_set_label: summary.rule_set_label,
                updated_at: summary.updated_at,
                party_size: summary.party_size,
                folder_name: summary.folder_name,
            })
            .collect(),
        unreadable_count: listing.unreadable_entries.len(),
    })
}

#[tauri::command]
pub fn drive_list_campaigns(
    request: DriveListCampaignsRequest,
) -> Result<DriveListCampaignsResponse, String> {
    drive_list_campaigns_impl(&request)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveCampaignFolderRequest {
    pub drive_folder_path: String,
    pub folder_name: String,
}

fn campaign_dir_for(request: &DriveCampaignFolderRequest) -> PathBuf {
    PathBuf::from(&request.drive_folder_path).join(&request.folder_name)
}

pub fn drive_load_campaign_impl(
    request: &DriveCampaignFolderRequest,
) -> Result<CampaignSnapshot, String> {
    CampaignStore::load(&campaign_dir_for(request)).map_err(|err| err.message)
}

#[tauri::command]
pub fn drive_load_campaign(request: DriveCampaignFolderRequest) -> Result<CampaignSnapshot, String> {
    drive_load_campaign_impl(&request)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveSaveCampaignRequest {
    pub drive_folder_path: String,
    pub snapshot: CampaignSnapshot,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveSaveCampaignResponse {
    pub campaign_folder_path: String,
}

pub fn drive_save_campaign_impl(
    request: &DriveSaveCampaignRequest,
) -> Result<DriveSaveCampaignResponse, String> {
    require_drive_folder_path(&request.drive_folder_path)?;
    let campaign_dir =
        CampaignStore::save_under_root(&request.snapshot, Path::new(&request.drive_folder_path))
            .map_err(|err| err.message)?;
    Ok(DriveSaveCampaignResponse {
        campaign_folder_path: campaign_dir.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
pub fn drive_save_campaign(
    request: DriveSaveCampaignRequest,
) -> Result<DriveSaveCampaignResponse, String> {
    drive_save_campaign_impl(&request)
}

pub fn drive_delete_campaign_impl(request: &DriveCampaignFolderRequest) -> Result<(), String> {
    CampaignStore::delete(&campaign_dir_for(request)).map_err(|err| err.message)
}

#[tauri::command]
pub fn drive_delete_campaign(request: DriveCampaignFolderRequest) -> Result<(), String> {
    drive_delete_campaign_impl(&request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!("codex-campaign-drive-test-{label}-{}", std::process::id()))
    }

    fn sample_campaign_config_json(name: &str) -> String {
        format!(
            r#"{{"id":"campaign-1","name":"{name}","ruleSetId":"crb","ruleSetLabel":"Core Rulebook","description":"","members":[],"partyCharacterIds":[],"createdAt":"2026-07-18T00:00:00Z","updatedAt":"2026-07-18T00:00:00Z"}}"#
        )
    }

    #[test]
    fn writes_config_and_asset_files_under_a_temp_root() {
        let temp = temp_dir("write");
        let _ = fs::remove_dir_all(&temp);

        let request = WriteCampaignDriveArtifactsRequest {
            drive_folder_path: temp.to_string_lossy().into_owned(),
            campaign_name: "Test Campaign".to_owned(),
            campaign_config_json: sample_campaign_config_json("Test Campaign"),
            assets: CampaignAssetsDto {
                resources: vec![MarkdownAssetDto { title: "Primer".to_owned(), body: "# Primer".to_owned() }],
                adventure_log: vec![],
                maps: vec![],
                wiki: vec![],
            },
        };

        let response = write_campaign_drive_artifacts_impl(&request).expect("write should succeed");
        let campaign_dir = PathBuf::from(&response.campaign_folder_path);
        assert!(campaign_dir.join(".config").join("Test Campaign.json").exists());
        assert!(campaign_dir.join("resources").join("Primer.md").exists());

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn rejects_an_empty_drive_folder_path() {
        let request = WriteCampaignDriveArtifactsRequest {
            drive_folder_path: String::new(),
            campaign_name: "X".to_owned(),
            campaign_config_json: sample_campaign_config_json("X"),
            assets: CampaignAssetsDto::default(),
        };
        assert!(write_campaign_drive_artifacts_impl(&request).is_err());
    }

    #[test]
    fn rejects_invalid_campaign_config_json() {
        let request = WriteCampaignDriveArtifactsRequest {
            drive_folder_path: temp_dir("invalid-json").to_string_lossy().into_owned(),
            campaign_name: "X".to_owned(),
            campaign_config_json: "{\"name\":\"missing required fields\"}".to_owned(),
            assets: CampaignAssetsDto::default(),
        };
        assert!(write_campaign_drive_artifacts_impl(&request).is_err());
    }

    #[test]
    fn round_trips_a_campaign_through_save_list_load_delete() {
        let temp = temp_dir("round-trip-commands");
        let _ = fs::remove_dir_all(&temp);

        let write_request = WriteCampaignDriveArtifactsRequest {
            drive_folder_path: temp.to_string_lossy().into_owned(),
            campaign_name: "Round Trip".to_owned(),
            campaign_config_json: sample_campaign_config_json("Round Trip"),
            assets: CampaignAssetsDto::default(),
        };
        write_campaign_drive_artifacts_impl(&write_request).expect("write should succeed");

        let listing = drive_list_campaigns_impl(&DriveListCampaignsRequest {
            drive_folder_path: temp.to_string_lossy().into_owned(),
        })
        .expect("list should succeed");
        assert_eq!(listing.campaigns.len(), 1);
        assert_eq!(listing.campaigns[0].name, "Round Trip");
        let folder_name = listing.campaigns[0].folder_name.clone();

        let load_request = DriveCampaignFolderRequest {
            drive_folder_path: temp.to_string_lossy().into_owned(),
            folder_name: folder_name.clone(),
        };
        let loaded = drive_load_campaign_impl(&load_request).expect("load should succeed");
        assert_eq!(loaded.name, "Round Trip");

        drive_delete_campaign_impl(&load_request).expect("delete should succeed");
        let listing_after_delete = drive_list_campaigns_impl(&DriveListCampaignsRequest {
            drive_folder_path: temp.to_string_lossy().into_owned(),
        })
        .expect("list should succeed");
        assert!(listing_after_delete.campaigns.is_empty());

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn drive_save_campaign_rejects_an_empty_drive_folder_path() {
        let snapshot: CampaignSnapshot =
            serde_json::from_str(&sample_campaign_config_json("X")).expect("fixture parses");
        let request = DriveSaveCampaignRequest {
            drive_folder_path: String::new(),
            snapshot,
        };
        assert!(drive_save_campaign_impl(&request).is_err());
    }
}
