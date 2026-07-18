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
    CampaignStoreError, CampaignSummary, LoadedCampaign, SaveOutcome,
};

const CONFIG_DIR: &str = ".config";
const RESOURCES_DIR: &str = "resources";
const ADVENTURE_LOG_DIR: &str = "adventure-log";
const MAPS_DIR: &str = "maps";
const WIKI_DIR: &str = "wiki";
const CONFLICTS_DIR: &str = "conflicts";
/// Revision nonce sidecar file, deliberately not part of the JSON config
/// (and not a `CampaignSnapshot` field — the addendum's field list mirrors
/// `campaignModel.ts` 1:1 and a revision nonce has no frontend counterpart).
const NONCE_FILE: &str = "nonce";

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

    /// Loads a campaign snapshot from `campaign_dir` along with the revision
    /// nonce it was saved at (0 if this campaign predates nonce-tracking).
    /// Pass the returned `nonce` back into `save_with_conflict_detection` on
    /// the next save so a stale-read save can be detected as a conflict.
    pub fn load_with_nonce(campaign_dir: &Path) -> Result<LoadedCampaign, CampaignStoreError> {
        let snapshot = Self::load(campaign_dir)?;
        let nonce = read_nonce(campaign_dir).unwrap_or(0);
        Ok(LoadedCampaign { snapshot, nonce })
    }

    /// Saves a campaign snapshot with conflict detection: if `expected_nonce`
    /// (the nonce this save's in-memory state was loaded at) doesn't match
    /// the nonce currently on disk, another writer has saved since — the
    /// existing on-disk contents are preserved under
    /// `<campaign_dir>/conflicts/<timestamp>/` before the new snapshot is
    /// written as the active state (local wins, per `decisions.md` §7; the
    /// DM resolves the conflict copy manually). `expected_nonce: None` means
    /// "no prior read to compare against" (a brand-new campaign) and never
    /// triggers a conflict.
    pub fn save_with_conflict_detection(
        snapshot: &CampaignSnapshot,
        campaign_dir: &Path,
        expected_nonce: Option<u64>,
    ) -> Result<SaveOutcome, CampaignStoreError> {
        let current_nonce = read_nonce(campaign_dir);

        let conflict_dir = match (expected_nonce, current_nonce) {
            (Some(expected), Some(current)) if expected != current => {
                Some(move_existing_state_to_conflicts(campaign_dir)?)
            }
            _ => None,
        };

        Self::save(snapshot, campaign_dir)?;
        let new_nonce = current_nonce.unwrap_or(0) + 1;
        write_nonce(campaign_dir, new_nonce)?;

        Ok(SaveOutcome {
            campaign_dir: campaign_dir.to_path_buf(),
            nonce: new_nonce,
            conflict_dir,
        })
    }

    /// `save_under_root` + `save_with_conflict_detection` combined, for
    /// callers (namely `campaign_drive.rs`'s Tauri commands) that only know
    /// the campaigns root and a snapshot, not a specific campaign directory.
    pub fn save_under_root_with_conflict_detection(
        snapshot: &CampaignSnapshot,
        campaigns_root: &Path,
        expected_nonce: Option<u64>,
    ) -> Result<SaveOutcome, CampaignStoreError> {
        let campaign_dir = campaigns_root.join(sanitize_filename(&snapshot.name));
        Self::save_with_conflict_detection(snapshot, &campaign_dir, expected_nonce)
    }
}

fn read_nonce(campaign_dir: &Path) -> Option<u64> {
    let path = campaign_dir.join(CONFIG_DIR).join(NONCE_FILE);
    fs::read_to_string(path)
        .ok()
        .and_then(|text| text.trim().parse::<u64>().ok())
}

fn write_nonce(campaign_dir: &Path, nonce: u64) -> Result<(), CampaignStoreError> {
    let config_dir = campaign_dir.join(CONFIG_DIR);
    fs::create_dir_all(&config_dir).map_err(|err| io_error(&config_dir, err))?;
    let path = config_dir.join(NONCE_FILE);
    fs::write(&path, nonce.to_string()).map_err(|err| io_error(&path, err))
}

/// Moves every existing top-level content directory (config + the four
/// asset groups — never the `conflicts/` directory itself) into a fresh
/// `conflicts/<timestamp>/` directory, so the caller can write a new active
/// state into `campaign_dir` afterward without losing what was there.
fn move_existing_state_to_conflicts(campaign_dir: &Path) -> Result<PathBuf, CampaignStoreError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos().to_string())
        .unwrap_or_else(|_| "0".to_owned());
    let conflict_dir = campaign_dir.join(CONFLICTS_DIR).join(&timestamp);
    fs::create_dir_all(&conflict_dir).map_err(|err| io_error(&conflict_dir, err))?;

    for dir_name in [CONFIG_DIR, RESOURCES_DIR, ADVENTURE_LOG_DIR, MAPS_DIR, WIKI_DIR] {
        let source = campaign_dir.join(dir_name);
        if source.exists() {
            let dest = conflict_dir.join(dir_name);
            fs::rename(&source, &dest).map_err(|err| io_error(&source, err))?;
        }
    }

    Ok(conflict_dir)
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
    fn load_honors_an_external_obsidian_style_edit_to_an_asset_markdown_file() {
        // E2.9: any of the per-asset `.md` files is a plain markdown file on
        // disk — editing one directly in an external editor (e.g. Obsidian)
        // and then re-loading the campaign must surface that edit, not the
        // stale in-memory body from the original save.
        let root = temp_root("obsidian-edit");
        let _ = fs::remove_dir_all(&root);
        CampaignStore::save(&sample_snapshot(), &root).expect("save should succeed");

        let resource_path = root.join(RESOURCES_DIR).join("Primer.md");
        fs::write(&resource_path, "# Primer\n\nEdited outside the app.").expect("external edit");

        let reloaded = CampaignStore::load(&root).expect("load should succeed");
        assert_eq!(reloaded.assets.resources.len(), 1);
        assert_eq!(reloaded.assets.resources[0].body, "# Primer\n\nEdited outside the app.");

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn load_picks_up_a_markdown_file_created_entirely_outside_the_app() {
        // Same principle, for a brand-new file Obsidian (or any editor)
        // created directly in the asset directory rather than through
        // `CampaignStore::save` — the filename (minus `.md`) becomes the
        // asset's title.
        let root = temp_root("obsidian-new-file");
        let _ = fs::remove_dir_all(&root);
        CampaignStore::save(&sample_snapshot(), &root).expect("save should succeed");

        // `sample_snapshot()` has no wiki entries, so `save` never created
        // this directory — an external editor creating it (plus a page in
        // it) for the first time must still be picked up on load.
        fs::create_dir_all(root.join(WIKI_DIR)).expect("create wiki dir");
        fs::write(root.join(WIKI_DIR).join("New Page.md"), "Written directly on disk.")
            .expect("external file write");

        let reloaded = CampaignStore::load(&root).expect("load should succeed");
        assert_eq!(reloaded.assets.wiki.len(), 1);
        assert_eq!(reloaded.assets.wiki[0].title, "New Page");
        assert_eq!(reloaded.assets.wiki[0].body, "Written directly on disk.");

        let _ = fs::remove_dir_all(&root);
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
    fn first_save_with_conflict_detection_bumps_nonce_to_one_and_reports_no_conflict() {
        let root = temp_root("conflict-first-save");
        let _ = fs::remove_dir_all(&root);

        let outcome = CampaignStore::save_with_conflict_detection(&sample_snapshot(), &root, None)
            .expect("save should succeed");

        assert_eq!(outcome.nonce, 1);
        assert!(outcome.conflict_dir.is_none());

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn save_with_a_matching_expected_nonce_never_conflicts() {
        let root = temp_root("conflict-matching-nonce");
        let _ = fs::remove_dir_all(&root);

        let first = CampaignStore::save_with_conflict_detection(&sample_snapshot(), &root, None)
            .expect("first save should succeed");

        let mut updated = sample_snapshot();
        updated.description = "Updated after an honest read-then-write.".to_owned();
        let second =
            CampaignStore::save_with_conflict_detection(&updated, &root, Some(first.nonce))
                .expect("second save should succeed");

        assert!(second.conflict_dir.is_none());
        assert_eq!(second.nonce, first.nonce + 1);
        assert_eq!(
            CampaignStore::load(&root).expect("load should succeed").description,
            "Updated after an honest read-then-write."
        );

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn save_with_a_stale_expected_nonce_moves_the_prior_state_into_conflicts() {
        let root = temp_root("conflict-stale-nonce");
        let _ = fs::remove_dir_all(&root);

        // "Device A" saves first.
        let device_a_first = CampaignStore::save_with_conflict_detection(&sample_snapshot(), &root, None)
            .expect("device A's first save should succeed");

        // "Device B" also saved since, bumping the on-disk nonce past what
        // device A still thinks is current.
        let mut device_b_snapshot = sample_snapshot();
        device_b_snapshot.description = "Device B's edit.".to_owned();
        CampaignStore::save_with_conflict_detection(&device_b_snapshot, &root, Some(device_a_first.nonce))
            .expect("device B's save should succeed");

        // Device A now saves its own (stale-based) edit using the nonce it
        // originally loaded at — a real conflict.
        let mut device_a_snapshot = sample_snapshot();
        device_a_snapshot.description = "Device A's edit.".to_owned();
        let conflicting_outcome = CampaignStore::save_with_conflict_detection(
            &device_a_snapshot,
            &root,
            Some(device_a_first.nonce),
        )
        .expect("conflicting save should still succeed");

        let conflict_dir = conflicting_outcome
            .conflict_dir
            .expect("a stale expected_nonce must produce a conflict directory");
        assert!(conflict_dir.starts_with(root.join("conflicts")));
        // Device B's contents are preserved in the conflict directory...
        assert!(conflict_dir.join(".config").join("The Void Between.json").exists());
        let preserved_config =
            fs::read_to_string(conflict_dir.join(".config").join("The Void Between.json")).unwrap();
        assert!(preserved_config.contains("Device B's edit."));
        // ...while device A's write becomes the active state.
        assert_eq!(
            CampaignStore::load(&root).expect("load should succeed").description,
            "Device A's edit."
        );

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn load_with_nonce_returns_zero_for_a_campaign_saved_without_conflict_detection() {
        let root = temp_root("load-with-nonce-legacy");
        let _ = fs::remove_dir_all(&root);
        CampaignStore::save(&sample_snapshot(), &root).expect("save should succeed");

        let loaded = CampaignStore::load_with_nonce(&root).expect("load should succeed");
        assert_eq!(loaded.nonce, 0);
        assert_eq!(loaded.snapshot, sample_snapshot());

        let _ = fs::remove_dir_all(&root);
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
