//! `CampaignStore` — a concrete, zero-field struct with associated save/load
//! functions, mirroring `saved_character::local_store::SavedCharacterStore`
//! (per the SD-21 Epic 2 engine-shape addendum, 2026-07-18: no `*Backend`
//! trait pattern exists anywhere in this codebase, and OAuth/Drive-API is
//! already descoped to local-disk-only, so there's no second backend to
//! justify trait-object indirection).
//!
//! The on-disk layout matches what `campaign_drive.rs`'s already-shipped
//! `write_campaign_drive_artifacts_impl` writes (PR #320), so this module
//! can become that command's real backing store rather than a
//! parallel-named duplicate:
//!
//! ```text
//! <campaign_dir>/
//!   .config/<sanitized name>.json   # CampaignSnapshot minus `assets`
//!   resources/<sanitized title>.md
//!   adventure-log/<sanitized title>.md
//!   maps/<sanitized title>.md
//!   wiki/<sanitized title>.md
//! ```
//!
//! Markdown asset bodies are the file contents verbatim, so any of them is
//! directly editable in an external markdown editor (e.g. Obsidian) —
//! `load` reconstitutes `CampaignAsset { title, body }` entries straight
//! from whatever `.md` files it finds, honoring external edits.

use std::fs;
use std::path::{Path, PathBuf};

use super::{
    CampaignAsset, CampaignAssets, CampaignListing, CampaignListingError, CampaignSnapshot,
    CampaignStoreError, CampaignSummary,
};

const CONFIG_DIR: &str = ".config";
const RESOURCES_DIR: &str = "resources";
const ADVENTURE_LOG_DIR: &str = "adventure-log";
const MAPS_DIR: &str = "maps";
const WIKI_DIR: &str = "wiki";

pub struct CampaignStore;

impl CampaignStore {
    /// Saves a campaign snapshot under `campaign_dir` (the campaign's own
    /// directory, not the campaigns root — same per-entity-root convention
    /// as `SavedCharacterStore::save`). Creates `campaign_dir` and every
    /// asset subdirectory it needs.
    pub fn save(snapshot: &CampaignSnapshot, campaign_dir: &Path) -> Result<(), CampaignStoreError> {
        fs::create_dir_all(campaign_dir).map_err(|err| io_error(campaign_dir, err))?;

        let config_dir = campaign_dir.join(CONFIG_DIR);
        fs::create_dir_all(&config_dir).map_err(|err| io_error(&config_dir, err))?;

        // Assets travel as markdown files, not JSON — the config file only
        // ever carries the non-asset fields, matching what
        // `campaign_config_json` already looked like before this module
        // existed.
        let mut config_only = snapshot.clone();
        config_only.assets = CampaignAssets::default();
        let json = serde_json::to_string_pretty(&config_only).map_err(|err| CampaignStoreError {
            message: format!("failed to serialize campaign snapshot: {err}"),
        })?;
        let config_path = config_dir.join(format!("{}.json", sanitize_filename(&snapshot.name)));
        fs::write(&config_path, json).map_err(|err| io_error(&config_path, err))?;

        write_asset_group(&campaign_dir.join(RESOURCES_DIR), &snapshot.assets.resources)?;
        write_asset_group(&campaign_dir.join(ADVENTURE_LOG_DIR), &snapshot.assets.adventure_log)?;
        write_asset_group(&campaign_dir.join(MAPS_DIR), &snapshot.assets.maps)?;
        write_asset_group(&campaign_dir.join(WIKI_DIR), &snapshot.assets.wiki)?;

        Ok(())
    }

    /// Loads a campaign snapshot from `campaign_dir`, re-reading the
    /// markdown asset files fresh every time — so an edit made outside the
    /// app (e.g. in Obsidian) between save and load is honored.
    pub fn load(campaign_dir: &Path) -> Result<CampaignSnapshot, CampaignStoreError> {
        let config_dir = campaign_dir.join(CONFIG_DIR);
        let config_entry = first_sorted_entry_with_extension(&config_dir, "json")?.ok_or_else(|| {
            CampaignStoreError {
                message: format!("no campaign config .json file found under {}", config_dir.display()),
            }
        })?;

        let text = fs::read_to_string(config_entry.path())
            .map_err(|err| io_error(&config_entry.path(), err))?;
        let mut snapshot: CampaignSnapshot = serde_json::from_str(&text).map_err(|err| CampaignStoreError {
            message: format!(
                "{} failed to parse as a campaign snapshot: {err}",
                config_entry.path().display()
            ),
        })?;

        snapshot.assets = CampaignAssets {
            resources: read_asset_group(&campaign_dir.join(RESOURCES_DIR))?,
            adventure_log: read_asset_group(&campaign_dir.join(ADVENTURE_LOG_DIR))?,
            maps: read_asset_group(&campaign_dir.join(MAPS_DIR))?,
            wiki: read_asset_group(&campaign_dir.join(WIKI_DIR))?,
        };

        Ok(snapshot)
    }

    /// Lists every campaign under `campaigns_root`.
    ///
    /// A nonexistent root returns an empty listing rather than an error — a
    /// campaign manager with no campaigns yet is not a failure. Each
    /// subdirectory is loaded independently: one corrupt/unreadable
    /// campaign is reported in `unreadable_entries` without failing the
    /// rest of the listing (mirrors `SavedCharacterStore::list_all`).
    pub fn list_all(campaigns_root: &Path) -> Result<CampaignListing, CampaignStoreError> {
        let entries = match fs::read_dir(campaigns_root) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(CampaignListing::default());
            }
            Err(err) => return Err(io_error(campaigns_root, err)),
        };

        let mut subdirectories: Vec<_> = entries
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .collect();
        subdirectories.sort_by_key(|entry| entry.file_name());

        let mut campaigns = Vec::new();
        let mut unreadable_entries = Vec::new();
        for entry in subdirectories {
            let folder_name = entry.file_name().to_string_lossy().into_owned();
            match Self::load(&entry.path()) {
                Ok(snapshot) => campaigns.push(summarize(&snapshot, folder_name)),
                Err(err) => unreadable_entries.push(CampaignListingError {
                    entry_name: folder_name,
                    message: err.message,
                }),
            }
        }

        Ok(CampaignListing {
            campaigns,
            unreadable_entries,
        })
    }

    /// Deletes a campaign directory outright. Deleting a campaign that does
    /// not exist is a no-op rather than an error — the caller's desired end
    /// state (the campaign is gone) already holds.
    pub fn delete(campaign_dir: &Path) -> Result<(), CampaignStoreError> {
        if !campaign_dir.exists() {
            return Ok(());
        }
        fs::remove_dir_all(campaign_dir).map_err(|err| io_error(campaign_dir, err))
    }

    /// Convenience for callers that only know the campaigns root (not a
    /// specific campaign directory) — sanitizes `snapshot.name` into a
    /// directory name under `campaigns_root`, saves there, and returns the
    /// campaign directory that was created. This is the shape
    /// `campaign_drive.rs`'s Tauri commands consume: they only ever see a
    /// user-configured Drive folder path plus a campaign name/snapshot.
    pub fn save_under_root(
        snapshot: &CampaignSnapshot,
        campaigns_root: &Path,
    ) -> Result<PathBuf, CampaignStoreError> {
        let campaign_dir = campaigns_root.join(sanitize_filename(&snapshot.name));
        Self::save(snapshot, &campaign_dir)?;
        Ok(campaign_dir)
    }
}

fn summarize(snapshot: &CampaignSnapshot, folder_name: String) -> CampaignSummary {
    CampaignSummary {
        id: snapshot.id.clone(),
        name: snapshot.name.clone(),
        rule_set_label: snapshot.rule_set_label.clone(),
        updated_at: snapshot.updated_at.clone(),
        party_size: snapshot.party_character_ids.len(),
        folder_name,
    }
}

/// Replaces characters that are unsafe/awkward in a filename with `_`. Keeps
/// letters, digits, spaces, and hyphens verbatim. Moved here (rather than
/// duplicated) from `campaign_drive.rs`, which becomes a thin adapter over
/// this module in a later cycle.
pub(crate) fn sanitize_filename(input: &str) -> String {
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

fn write_asset_group(dir: &Path, assets: &[CampaignAsset]) -> Result<(), CampaignStoreError> {
    if assets.is_empty() {
        return Ok(());
    }
    fs::create_dir_all(dir).map_err(|err| io_error(dir, err))?;
    for asset in assets {
        let path = dir.join(format!("{}.md", sanitize_filename(&asset.title)));
        fs::write(&path, &asset.body).map_err(|err| io_error(&path, err))?;
    }
    Ok(())
}

/// Reads back every `.md` file in `dir` as a `CampaignAsset`, using the
/// filename (minus extension) as the title. A missing directory means no
/// assets of that kind exist yet — not an error.
fn read_asset_group(dir: &Path) -> Result<Vec<CampaignAsset>, CampaignStoreError> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(io_error(dir, err)),
    };

    let mut files: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
        .collect();
    files.sort_by_key(|entry| entry.file_name());

    let mut assets = Vec::new();
    for entry in files {
        let path = entry.path();
        let body = fs::read_to_string(&path).map_err(|err| io_error(&path, err))?;
        let title = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_default();
        assets.push(CampaignAsset { title, body });
    }
    Ok(assets)
}

fn first_sorted_entry_with_extension(
    dir: &Path,
    extension: &str,
) -> Result<Option<fs::DirEntry>, CampaignStoreError> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(io_error(dir, err)),
    };

    let mut matches: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some(extension))
        .collect();
    matches.sort_by_key(|entry| entry.file_name());
    Ok(matches.into_iter().next())
}

fn io_error(path: &Path, err: std::io::Error) -> CampaignStoreError {
    CampaignStoreError {
        message: format!("{}: {err}", path.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::{CampaignMember, CURRENT_CAMPAIGN_SCHEMA_VERSION};

    fn temp_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "codex-campaign-store-test-{label}-{}",
            std::process::id()
        ))
    }

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
            party_character_ids: vec!["character-1".to_owned()],
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
    fn saves_then_loads_an_equivalent_snapshot() {
        let root = temp_root("roundtrip");
        let _ = fs::remove_dir_all(&root);

        let snapshot = sample_snapshot();
        CampaignStore::save(&snapshot, &root).expect("save should succeed");
        let loaded = CampaignStore::load(&root).expect("load should succeed");
        assert_eq!(loaded, snapshot);

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn writes_the_real_local_folder_layout_matching_campaign_drive() {
        let root = temp_root("layout");
        let _ = fs::remove_dir_all(&root);

        CampaignStore::save(&sample_snapshot(), &root).expect("save should succeed");

        assert!(root.join(".config").join("The Void Between.json").exists());
        assert!(root.join("resources").join("Primer.md").exists());
        assert_eq!(
            fs::read_to_string(root.join("resources").join("Primer.md")).unwrap(),
            "# Primer"
        );
        // Empty asset groups never get a subdirectory.
        assert!(!root.join("adventure-log").exists());

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn load_reports_an_error_for_a_missing_config_file() {
        let root = temp_root("missing");
        let _ = fs::remove_dir_all(&root);

        let result = CampaignStore::load(&root);
        assert!(result.is_err());

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn list_all_tolerates_a_missing_campaigns_root() {
        let root = temp_root("missing-listing-root");
        let _ = fs::remove_dir_all(&root);

        let listing = CampaignStore::list_all(&root).expect("should not error on a missing root");
        assert!(listing.campaigns.is_empty());
        assert!(listing.unreadable_entries.is_empty());
    }

    #[test]
    fn list_all_isolates_one_unreadable_entry_from_the_rest_of_the_listing() {
        let campaigns_root = temp_root("listing");
        let _ = fs::remove_dir_all(&campaigns_root);
        fs::create_dir_all(&campaigns_root).unwrap();

        CampaignStore::save(&sample_snapshot(), &campaigns_root.join("good-campaign"))
            .expect("save should succeed");
        fs::create_dir_all(campaigns_root.join("corrupt-campaign")).unwrap();
        // No `.config/*.json` written for this one — an unreadable entry.

        let listing = CampaignStore::list_all(&campaigns_root).expect("listing should succeed");
        assert_eq!(listing.campaigns.len(), 1);
        assert_eq!(listing.campaigns[0].name, "The Void Between");
        assert_eq!(listing.unreadable_entries.len(), 1);
        assert_eq!(listing.unreadable_entries[0].entry_name, "corrupt-campaign");

        let _ = fs::remove_dir_all(&campaigns_root);
    }

    #[test]
    fn sanitize_filename_replaces_unsafe_characters() {
        assert_eq!(
            sanitize_filename("The Void / Between: The Stars?"),
            "The Void _ Between_ The Stars_"
        );
    }

    #[test]
    fn save_under_root_sanitizes_the_campaign_name_into_a_directory() {
        let campaigns_root = temp_root("save-under-root");
        let _ = fs::remove_dir_all(&campaigns_root);

        let mut snapshot = sample_snapshot();
        snapshot.name = "Tales From: The Void?".to_owned();
        let campaign_dir =
            CampaignStore::save_under_root(&snapshot, &campaigns_root).expect("save should succeed");

        assert_eq!(campaign_dir, campaigns_root.join("Tales From_ The Void_"));
        assert!(campaign_dir.join(".config").exists());

        let _ = fs::remove_dir_all(&campaigns_root);
    }

    #[test]
    fn delete_removes_the_campaign_directory_and_is_idempotent() {
        let root = temp_root("delete");
        let _ = fs::remove_dir_all(&root);
        CampaignStore::save(&sample_snapshot(), &root).expect("save should succeed");

        CampaignStore::delete(&root).expect("delete should succeed");
        assert!(!root.exists());

        // Deleting again (already gone) is a no-op, not an error.
        CampaignStore::delete(&root).expect("deleting a missing dir is a no-op");
    }
}
