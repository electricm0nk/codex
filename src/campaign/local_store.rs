//! `CampaignStore` — a concrete, zero-field struct with associated save/load
//! functions, mirroring `saved_character::local_store::SavedCharacterStore`
//! (per the SD-21 Epic 2 engine-shape addendum, 2026-07-18: no `*Backend`
//! trait pattern exists anywhere in this codebase, and OAuth/Drive-API is
//! already descoped to local-disk-only, so there's no second backend to
//! justify trait-object indirection).
//!
//! This first cut proves the struct's save/load shape against a single JSON
//! snapshot file. The real local-folder layout (a `.config/<name>.json`
//! metadata file plus per-asset-group markdown files, matching what
//! `campaign_drive.rs` already writes) lands in a later cycle (E2.7), which
//! is a pure change to how `save`/`load` read and write bytes — the struct's
//! public shape here is what backs it.

use std::fs;
use std::path::Path;

use super::{CampaignSnapshot, CampaignStoreError};

const SNAPSHOT_FILE: &str = "campaign.json";

pub struct CampaignStore;

impl CampaignStore {
    /// Saves a campaign snapshot under `root` (the campaign's own directory,
    /// not the campaigns root — same per-entity-root convention as
    /// `SavedCharacterStore::save`). Creates `root` if it does not exist.
    pub fn save(snapshot: &CampaignSnapshot, root: &Path) -> Result<(), CampaignStoreError> {
        fs::create_dir_all(root).map_err(|err| io_error(root, err))?;

        let json = serde_json::to_string_pretty(snapshot).map_err(|err| CampaignStoreError {
            message: format!("failed to serialize campaign snapshot: {err}"),
        })?;

        let path = root.join(SNAPSHOT_FILE);
        fs::write(&path, json).map_err(|err| io_error(&path, err))
    }

    /// Loads a campaign snapshot from `root`. Returns `Err` if the snapshot
    /// file is missing, unreadable, or fails to parse as a `CampaignSnapshot`.
    pub fn load(root: &Path) -> Result<CampaignSnapshot, CampaignStoreError> {
        let path = root.join(SNAPSHOT_FILE);
        let text = fs::read_to_string(&path).map_err(|err| CampaignStoreError {
            message: format!("{} missing or unreadable: {err}", path.display()),
        })?;

        serde_json::from_str(&text).map_err(|err| CampaignStoreError {
            message: format!("{} failed to parse as a campaign snapshot: {err}", path.display()),
        })
    }
}

fn io_error(path: &Path, err: std::io::Error) -> CampaignStoreError {
    CampaignStoreError {
        message: format!("{}: {err}", path.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::{CampaignAsset, CampaignAssets, CampaignMember, CURRENT_CAMPAIGN_SCHEMA_VERSION};

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
    fn save_creates_the_root_directory_if_missing() {
        let root = temp_root("creates-root");
        let _ = fs::remove_dir_all(&root);
        assert!(!root.exists());

        CampaignStore::save(&sample_snapshot(), &root).expect("save should succeed");
        assert!(root.join(SNAPSHOT_FILE).exists());

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn load_reports_an_error_for_a_missing_snapshot_file() {
        let root = temp_root("missing");
        let _ = fs::remove_dir_all(&root);

        let result = CampaignStore::load(&root);
        assert!(result.is_err());

        let _ = fs::remove_dir_all(&root);
    }
}
