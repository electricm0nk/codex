//! Real local-disk write-through for campaign "Drive" artifacts.
//!
//! `campaignModel.ts`'s design doc describes a Drive folder per campaign
//! (`<driveFolderPath>/<name>/`) holding a `.config/<name>.json` metadata
//! file plus markdown files for resources/adventure-log/maps/wiki — but
//! until now that was only ever a simulated string ("Would create Drive
//! folder..."), never real I/O. This module makes it real: given the
//! configured local Drive-folder path (today a local sync-client mirror,
//! since there's no OAuth/Drive API integration yet — see
//! settings/googleDrive.ts), it writes the actual folder structure.

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownAssetDto {
    pub title: String,
    pub body: String,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteCampaignDriveArtifactsRequest {
    pub drive_folder_path: String,
    pub campaign_name: String,
    /// The full `Campaign` record, already JSON-serialized by the caller —
    /// this module just writes bytes, it doesn't know the frontend's shape.
    pub campaign_config_json: String,
    #[serde(default)]
    pub assets: CampaignAssetsDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteCampaignDriveArtifactsResponse {
    pub campaign_folder_path: String,
}

/// Replaces characters that are unsafe/awkward in a filename with `_`.
/// Keeps letters, digits, spaces, and hyphens verbatim.
fn sanitize_filename(input: &str) -> String {
    let cleaned: String = input
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == ' ' { c } else { '_' })
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "untitled".to_owned()
    } else {
        trimmed.to_owned()
    }
}

fn write_asset_group(campaign_dir: &PathBuf, subdir: &str, assets: &[MarkdownAssetDto]) -> Result<(), String> {
    if assets.is_empty() {
        return Ok(());
    }
    let dir = campaign_dir.join(subdir);
    fs::create_dir_all(&dir).map_err(|err| format!("{}: {err}", dir.display()))?;
    for asset in assets {
        let path = dir.join(format!("{}.md", sanitize_filename(&asset.title)));
        fs::write(&path, &asset.body).map_err(|err| format!("{}: {err}", path.display()))?;
    }
    Ok(())
}

/// Pure implementation, unit-testable without a `tauri::AppHandle` — the
/// command below is a thin wrapper, mirroring this codebase's established
/// command/pure-fn split (e.g. `ge08_workbench::build_ge08_workbench_snapshot`).
pub fn write_campaign_drive_artifacts_impl(
    request: &WriteCampaignDriveArtifactsRequest,
) -> Result<WriteCampaignDriveArtifactsResponse, String> {
    if request.drive_folder_path.trim().is_empty() {
        return Err("no Drive folder configured — set one under Settings \u{2192} Google Drive first".to_owned());
    }

    let campaign_dir = PathBuf::from(&request.drive_folder_path).join(sanitize_filename(&request.campaign_name));
    let config_dir = campaign_dir.join(".config");
    fs::create_dir_all(&config_dir).map_err(|err| format!("{}: {err}", config_dir.display()))?;

    let config_path = config_dir.join(format!("{}.json", sanitize_filename(&request.campaign_name)));
    fs::write(&config_path, &request.campaign_config_json)
        .map_err(|err| format!("{}: {err}", config_path.display()))?;

    write_asset_group(&campaign_dir, "resources", &request.assets.resources)?;
    write_asset_group(&campaign_dir, "adventure-log", &request.assets.adventure_log)?;
    write_asset_group(&campaign_dir, "maps", &request.assets.maps)?;
    write_asset_group(&campaign_dir, "wiki", &request.assets.wiki)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_filename_replaces_unsafe_characters() {
        assert_eq!(sanitize_filename("The Void / Between: The Stars?"), "The Void _ Between_ The Stars_");
    }

    #[test]
    fn writes_config_and_asset_files_under_a_temp_root() {
        let temp = std::env::temp_dir().join(format!("codex-campaign-drive-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);

        let request = WriteCampaignDriveArtifactsRequest {
            drive_folder_path: temp.to_string_lossy().into_owned(),
            campaign_name: "Test Campaign".to_owned(),
            campaign_config_json: "{\"name\":\"Test Campaign\"}".to_owned(),
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
            campaign_config_json: "{}".to_owned(),
            assets: CampaignAssetsDto::default(),
        };
        assert!(write_campaign_drive_artifacts_impl(&request).is_err());
    }
}
