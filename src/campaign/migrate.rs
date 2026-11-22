//! Migration of a pre-SD-42 campaign into a campaign folder (`decisions.md §14`, OPERATOR Q13).
//!
//! Two sources: the old localStorage record (`codex.campaigns` + `codex.campaign.assets.<id>`, passed
//! in by the UI as the raw JSON it read) and a folder holding `.config/<name>.json` (schema 1, the
//! SD-21 write-through `local_store` still reads). Either becomes a new folder with E2.1's layout:
//! `campaign.json` (schema 2), `Party Resources.md`, one Markdown file per old entry. The old data is
//! written to `<app data>/campaign-migration-backup-<date>.json` before anything else; nothing is
//! deleted (a `.config/<name>.json` is renamed `.json.bak`, so a second run finds nothing).

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::folder::{self, PARTY_RESOURCES_FILE};
use super::local_store::{sanitize_filename, CampaignStore};
use super::{
    CampaignFolderSnapshot, CampaignSnapshot, CampaignStoreError, FolderMember, CAMPAIGN_FOLDER_SCHEMA_VERSION,
};

const CONFIG_DIR: &str = ".config";

/// One old Markdown asset. localStorage entries carry `updatedAt`; `.config/` folders do not.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LegacyEntry {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// `codex.campaign.assets.<id>`: the four old lists.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LegacyAssets {
    #[serde(default)]
    pub resources: Vec<LegacyEntry>,
    #[serde(default)]
    pub adventure_log: Vec<LegacyEntry>,
    #[serde(default)]
    pub maps: Vec<LegacyEntry>,
    #[serde(default)]
    pub wiki: Vec<LegacyEntry>,
}

/// What one migration reads: the old record, its entries, and the exact old data for the backup.
#[derive(Debug, Clone)]
pub struct LegacyCampaign {
    pub record: CampaignSnapshot,
    pub assets: LegacyAssets,
    /// Written verbatim to the backup file.
    pub backup: serde_json::Value,
    /// Set for a `.config/` source: renamed `.json.bak` once the new folder is written.
    pub config_file: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationOutcome {
    pub campaign_dir: PathBuf,
    pub backup_file: PathBuf,
    /// Markdown files written besides `Party Resources.md`, `/`-separated inside the campaign.
    pub files: Vec<String>,
}

fn error(message: String) -> CampaignStoreError {
    CampaignStoreError { message }
}

/// Reads the old localStorage shape: one `codex.campaigns` record and its assets value (absent →
/// no entries). Both are kept as given for the backup.
pub fn from_local_storage(record_json: &str, assets_json: Option<&str>) -> Result<LegacyCampaign, CampaignStoreError> {
    let record_value: serde_json::Value =
        serde_json::from_str(record_json).map_err(|err| error(format!("the old campaign record is not JSON: {err}")))?;
    let record: CampaignSnapshot = serde_json::from_value(record_value.clone())
        .map_err(|err| error(format!("the old campaign record is not a campaign: {err}")))?;
    let assets_value: serde_json::Value = match assets_json {
        Some(text) => serde_json::from_str(text).map_err(|err| error(format!("the old campaign entries are not JSON: {err}")))?,
        None => serde_json::Value::Null,
    };
    let assets: LegacyAssets = if assets_value.is_null() {
        LegacyAssets::default()
    } else {
        serde_json::from_value(assets_value.clone()).map_err(|err| error(format!("the old campaign entries: {err}")))?
    };
    let backup = serde_json::json!({
        "source": "localStorage",
        "codex.campaigns": [record_value],
        format!("codex.campaign.assets.{}", record.id): assets_value,
    });
    Ok(LegacyCampaign { record, assets, backup, config_file: None })
}

/// The `.config/<name>.json` of an old campaign folder, if it still has one.
pub fn legacy_config_file(dir: &Path) -> Option<PathBuf> {
    let mut found: Vec<PathBuf> = fs::read_dir(dir.join(CONFIG_DIR))
        .ok()?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    found.sort();
    found.into_iter().next()
}

/// Reads an old `.config/` campaign folder (record + its `resources/`, `adventure-log/`, `maps/`,
/// `wiki/` Markdown). `Ok(None)`: nothing to migrate there (already migrated, or never a campaign).
pub fn from_config_folder(dir: &Path) -> Result<Option<LegacyCampaign>, CampaignStoreError> {
    let Some(config_file) = legacy_config_file(dir) else {
        return Ok(None);
    };
    let text = fs::read_to_string(&config_file).map_err(|err| error(format!("{}: {err}", config_file.display())))?;
    let record = CampaignStore::load(dir)?;
    let entries = |list: &[super::CampaignAsset]| {
        list.iter()
            .map(|asset| LegacyEntry { title: asset.title.clone(), body: asset.body.clone(), updated_at: None })
            .collect()
    };
    let assets = LegacyAssets {
        resources: entries(&record.assets.resources),
        adventure_log: entries(&record.assets.adventure_log),
        maps: entries(&record.assets.maps),
        wiki: entries(&record.assets.wiki),
    };
    let config_value: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::String(text));
    let backup = serde_json::json!({
        "source": config_file.to_string_lossy(),
        "config": config_value,
        "entries": serde_json::to_value(&record.assets).unwrap_or_default(),
    });
    Ok(Some(LegacyCampaign { record, assets, backup, config_file: Some(config_file) }))
}

/// `YYYY-MM-DD` from an ISO timestamp, if it starts with one.
fn iso_date(text: &str) -> Option<&str> {
    let date = text.get(..10)?;
    let ok = date.bytes().enumerate().all(|(i, b)| if i == 4 || i == 7 { b == b'-' } else { b.is_ascii_digit() });
    ok.then_some(date)
}

/// `stem.md` in `taken`'s directory, or `stem 2.md`, `stem 3.md`… — two old entries never share a file.
fn unique_name(taken: &mut Vec<String>, dir: &str, stem: &str) -> String {
    let stem = sanitize_filename(stem);
    let mut n = 1;
    loop {
        let name = if n == 1 { format!("{dir}/{stem}.md") } else { format!("{dir}/{stem} {n}.md") };
        if !taken.iter().any(|t| t.eq_ignore_ascii_case(&name)) {
            taken.push(name.clone());
            return name;
        }
        n += 1;
    }
}

fn yaml_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

fn title_or_untitled(title: &str) -> &str {
    if title.trim().is_empty() {
        "Untitled"
    } else {
        title.trim()
    }
}

/// The backup file: `campaign-migration-backup-<date>.json`, `-2`, `-3`… when a migration that day
/// already wrote one (a backup is never overwritten).
fn write_backup(backup_dir: &Path, date: &str, backup: &serde_json::Value) -> Result<PathBuf, CampaignStoreError> {
    fs::create_dir_all(backup_dir).map_err(|err| error(format!("{}: {err}", backup_dir.display())))?;
    let json = serde_json::to_string_pretty(backup).map_err(|err| error(err.to_string()))?;
    for n in 1u32.. {
        let name = if n == 1 {
            format!("campaign-migration-backup-{date}.json")
        } else {
            format!("campaign-migration-backup-{date}-{n}.json")
        };
        let path = backup_dir.join(name);
        match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                use std::io::Write;
                file.write_all(json.as_bytes()).map_err(|err| error(format!("{}: {err}", path.display())))?;
                file.sync_all().map_err(|err| error(format!("{}: {err}", path.display())))?;
                return Ok(path);
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(error(format!("{}: {err}", path.display()))),
        }
    }
    unreachable!("u32 names run out before the disk does")
}

/// Migrates `old` into the new folder `target` (must not exist or be empty, SD-f). `your_name` is
/// member #1; `now` is an ISO timestamp (the UI's clock). Order: backup, folder, entries, then the
/// `.config/<name>.json` rename. A refused target leaves the backup and touches nothing else.
pub fn migrate(
    old: &LegacyCampaign,
    target: &Path,
    your_name: &str,
    backup_dir: &Path,
    now: &str,
) -> Result<MigrationOutcome, CampaignStoreError> {
    let your_name = your_name.trim();
    if your_name.is_empty() {
        return Err(error("Your name is required: the migrating user is member #1.".to_owned()));
    }
    let today = iso_date(now).ok_or_else(|| error(format!("{now:?} is not an ISO timestamp")))?;
    let backup_file = write_backup(backup_dir, today, &old.backup)?;

    let record = &old.record;
    let mut members = vec![FolderMember { name: your_name.to_owned(), added_at: now.to_owned() }];
    for member in &record.members {
        let name = member.email.trim();
        if !name.is_empty() && !members.iter().any(|m| m.name.eq_ignore_ascii_case(name)) {
            members.push(FolderMember { name: name.to_owned(), added_at: now.to_owned() });
        }
    }
    let snapshot = CampaignFolderSnapshot {
        schema_version: CAMPAIGN_FOLDER_SCHEMA_VERSION,
        name: record.name.clone(),
        rule_set_id: record.rule_set_id.clone(),
        rule_set_label: record.rule_set_label.clone(),
        description: record.description.clone(),
        members,
        created_at: record.created_at.clone(),
        updated_at: now.to_owned(),
    };
    folder::create_layout(target, &snapshot)?;

    let write = |rel: &str, text: &str| folder::write_atomic(&target.join(rel), text.as_bytes());
    let resources = &old.assets.resources;
    if !resources.is_empty() {
        let joined: Vec<String> = resources
            .iter()
            .map(|entry| format!("## {}\n\n{}\n", title_or_untitled(&entry.title), entry.body.trim_end()))
            .collect();
        write(PARTY_RESOURCES_FILE, &format!("# Party Resources\n\n{}", joined.join("\n")))?;
    }

    let mut files = Vec::new();
    let fallback_date = iso_date(&record.updated_at).unwrap_or(today);
    for entry in &old.assets.adventure_log {
        let title = title_or_untitled(&entry.title);
        let date = entry.updated_at.as_deref().and_then(iso_date).unwrap_or(fallback_date);
        let rel = unique_name(&mut files, "Adventure Log", &format!("{date} {your_name} - {title}"));
        let text = format!(
            "---\ncampaign: {}\nauthor: {}\nposted: {date}\nsession: {date}\nin_game_date:\ntitle: {}\n---\n{}",
            yaml_string(&record.name),
            yaml_string(your_name),
            yaml_string(title),
            entry.body
        );
        write(&rel, &text)?;
    }
    for (dir, list) in [("Maps", &old.assets.maps), ("Wiki", &old.assets.wiki)] {
        for entry in list {
            let title = title_or_untitled(&entry.title);
            let rel = unique_name(&mut files, dir, title);
            let text = if dir == "Wiki" {
                format!("---\ntitle: {}\ntype: page\ntags: []\n---\n{}", yaml_string(title), entry.body)
            } else {
                entry.body.clone()
            };
            write(&rel, &text)?;
        }
    }

    if let Some(config_file) = &old.config_file {
        let mut bak = config_file.clone().into_os_string();
        bak.push(".bak");
        fs::rename(config_file, &bak).map_err(|err| error(format!("{}: {err}", config_file.display())))?;
    }
    Ok(MigrationOutcome { campaign_dir: target.to_path_buf(), backup_file, files })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::folder::PARTY_RESOURCES_STARTER;
    use crate::campaign::{CampaignAsset, CampaignAssets, CampaignMember, CURRENT_CAMPAIGN_SCHEMA_VERSION};

    const NOW: &str = "2026-11-17T10:00:00.000Z";

    fn temp_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("codex-campaign-migrate-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    const RECORD: &str = r#"{"id":"c1","name":"Rise of the Runelords","ruleSetId":"crb","ruleSetLabel":"Core",
        "description":"Sandpoint.","members":[{"email":"ana@example.com"},{"email":"Gwen"}],
        "partyCharacterIds":["x"],"createdAt":"2025-01-02T00:00:00Z","updatedAt":"2025-03-04T00:00:00Z"}"#;
    const ASSETS: &str = r#"{"resources":[{"id":"r","title":"Loot","body":"| a | b |","updatedAt":"2025-02-01T00:00:00Z"}],
        "adventureLog":[{"id":"a1","title":"Session 1","body":"We met.","updatedAt":"2025-02-03T09:00:00Z"},
                        {"id":"a2","title":"Session 1","body":"Again.","updatedAt":"2025-02-03T21:00:00Z"}],
        "maps":[{"id":"m","title":"Sandpoint","body":"Map notes.","updatedAt":"x"}],
        "wiki":[{"id":"w","title":"Ameiko","body":"Innkeeper.","updatedAt":"x"},{"id":"w2","title":"","body":"No title.","updatedAt":"x"}]}"#;

    fn read(dir: &Path, rel: &str) -> String {
        fs::read_to_string(dir.join(rel)).unwrap_or_else(|err| panic!("{rel}: {err}"))
    }

    #[test]
    fn campaign_migrate_localstorage_shape_writes_every_entry_as_a_file() {
        let root = temp_dir("ls");
        let old = from_local_storage(RECORD, Some(ASSETS)).unwrap();
        let target = root.join("Rise of the Runelords");
        let out = migrate(&old, &target, " Gwen ", &root.join("appdata"), NOW).unwrap();

        let opened = folder::open(&target).expect("a schema-2 campaign folder");
        let names: Vec<&str> = opened.snapshot.members.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, ["Gwen", "ana@example.com"], "migrating user first; old emails as names; no duplicate");
        assert_eq!(opened.snapshot.description, "Sandpoint.");
        assert_eq!(opened.snapshot.created_at, "2025-01-02T00:00:00Z");

        let resources = read(&target, PARTY_RESOURCES_FILE);
        assert!(resources.contains("## Loot\n\n| a | b |"), "{resources}");
        // 2 log entries + 1 map + 2 wiki pages = 5 files, one per old entry.
        assert_eq!(out.files.len(), 5, "{:?}", out.files);
        let first = read(&target, "Adventure Log/2025-02-03 Gwen - Session 1.md");
        assert!(first.starts_with("---\ncampaign: \"Rise of the Runelords\"\nauthor: \"Gwen\"\nposted: 2025-02-03\nsession: 2025-02-03\n"), "{first}");
        assert!(first.ends_with("---\nWe met."), "{first}");
        assert!(read(&target, "Adventure Log/2025-02-03 Gwen - Session 1 2.md").ends_with("Again."));
        assert_eq!(read(&target, "Maps/Sandpoint.md"), "Map notes.");
        assert!(read(&target, "Wiki/Ameiko.md").contains("title: \"Ameiko\"\ntype: page\ntags: []\n---\nInnkeeper."));
        assert!(read(&target, "Wiki/Untitled.md").ends_with("No title."));
        for sub in folder::LAYOUT_DIRS {
            assert!(target.join(sub).is_dir(), "{sub}");
        }
        let _ = fs::remove_dir_all(&root);
    }

    fn schema_1_folder(dir: &Path) {
        let snapshot = CampaignSnapshot {
            schema_version: CURRENT_CAMPAIGN_SCHEMA_VERSION,
            id: "c2".to_owned(),
            name: "Old Folder".to_owned(),
            rule_set_id: "crb".to_owned(),
            rule_set_label: "Core".to_owned(),
            description: "From .config".to_owned(),
            members: vec![CampaignMember { email: "bo@example.com".to_owned(), invited: true }],
            party_character_ids: vec![],
            created_at: "2025-05-05T00:00:00Z".to_owned(),
            updated_at: "2025-06-06T00:00:00Z".to_owned(),
            assets: CampaignAssets {
                resources: vec![],
                adventure_log: vec![CampaignAsset { title: "Night one".to_owned(), body: "Rain.".to_owned() }],
                maps: vec![],
                wiki: vec![CampaignAsset { title: "Bo".to_owned(), body: "A bard.".to_owned() }],
            },
        };
        CampaignStore::save(&snapshot, dir).unwrap();
    }

    #[test]
    fn campaign_migrate_config_json_folder_upgrades_and_keeps_a_bak() {
        let root = temp_dir("config");
        let source = root.join("drive/Old Folder");
        schema_1_folder(&source);
        let old = from_config_folder(&source).unwrap().expect("an old campaign");
        let target = root.join("new/Old Folder");
        migrate(&old, &target, "Dana", &root.join("appdata"), NOW).unwrap();

        let opened = folder::open(&target).unwrap();
        assert_eq!(opened.snapshot.schema_version, 2);
        assert_eq!(opened.snapshot.members.iter().map(|m| m.name.as_str()).collect::<Vec<_>>(), ["Dana", "bo@example.com"]);
        // No per-entry date in a .config folder: the old record's updatedAt dates the log entry.
        assert!(read(&target, "Adventure Log/2025-06-06 Dana - Night one.md").ends_with("Rain."));
        assert!(read(&target, "Wiki/Bo.md").ends_with("A bard."));
        // No old resources: the starter.
        assert_eq!(read(&target, PARTY_RESOURCES_FILE), PARTY_RESOURCES_STARTER);
        // Nothing deleted: the config is renamed .bak, the old Markdown stays.
        assert!(!source.join(".config/Old Folder.json").exists());
        assert!(source.join(".config/Old Folder.json.bak").is_file());
        assert!(source.join("wiki/Bo.md").is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn campaign_migrate_writes_a_backup_before_anything_else() {
        let root = temp_dir("backup");
        let appdata = root.join("appdata");
        let old = from_local_storage(RECORD, Some(ASSETS)).unwrap();
        // A target that is not empty is refused (SD-f) — after the backup is already on disk.
        let target = root.join("taken");
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("keep.txt"), "mine").unwrap();
        let err = migrate(&old, &target, "Gwen", &appdata, NOW).unwrap_err();
        assert!(err.message.contains("not empty"), "{}", err.message);
        assert_eq!(fs::read_dir(&target).unwrap().count(), 1, "the refused target is untouched");

        let backup = appdata.join("campaign-migration-backup-2026-11-17.json");
        let saved: serde_json::Value = serde_json::from_str(&fs::read_to_string(&backup).unwrap()).unwrap();
        let record: serde_json::Value = serde_json::from_str(RECORD).unwrap();
        let assets: serde_json::Value = serde_json::from_str(ASSETS).unwrap();
        assert_eq!(saved["codex.campaigns"][0], record, "the old record, verbatim");
        assert_eq!(saved["codex.campaign.assets.c1"], assets, "the old entries, verbatim (ids and dates too)");

        // A second backup the same day never overwrites the first.
        let out = migrate(&old, &root.join("ok"), "Gwen", &appdata, NOW).unwrap();
        assert_eq!(out.backup_file, appdata.join("campaign-migration-backup-2026-11-17-2.json"));
        assert!(backup.is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn campaign_migrate_is_idempotent() {
        let root = temp_dir("idem");
        let source = root.join("Old Folder");
        schema_1_folder(&source);
        let old = from_config_folder(&source).unwrap().unwrap();
        migrate(&old, &root.join("new"), "Dana", &root.join("appdata"), NOW).unwrap();
        // The .config folder now holds only a .bak: nothing left to migrate.
        assert!(from_config_folder(&source).unwrap().is_none());
        assert!(legacy_config_file(&source).is_none());

        // localStorage: the same record into the folder it already made is refused, the files unchanged.
        let ls = from_local_storage(RECORD, None).unwrap();
        let target = root.join("ls");
        migrate(&ls, &target, "Gwen", &root.join("appdata"), NOW).unwrap();
        let before = read(&target, "campaign.json");
        assert!(migrate(&ls, &target, "Gwen", &root.join("appdata"), NOW).is_err());
        assert_eq!(read(&target, "campaign.json"), before);
        let _ = fs::remove_dir_all(&root);
    }
}
