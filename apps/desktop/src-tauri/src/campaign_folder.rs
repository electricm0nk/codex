//! Tauri commands over a campaign folder (SD-42 `decisions.md §11`): the known-folders list kept
//! in `<app data>/campaign-folders.json`, creating and opening a campaign folder, and reading,
//! writing, statting and listing its files. Every path a caller passes inside a campaign goes
//! through `codex::campaign::folder::relative`, so no command reads or writes outside the folder.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use tauri::Manager;

use codex::campaign::folder;
use codex::campaign::local_store::{CampaignStore, CAMPAIGN_JSON};
use codex::campaign::CampaignFolderSnapshot;

const KNOWN_FOLDERS_FILE: &str = "campaign-folders.json";
/// Where dragged-in images go (one of `folder::LAYOUT_DIRS`).
const ATTACHMENTS_DIR: &str = "Attachments";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnownFolderEntry {
    pub path: String,
    pub last_opened_ms: u64,
}

/// One known folder as the Manager lists it: the campaign's name when it opens, else why not.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KnownCampaignDto {
    pub path: String,
    pub last_opened_ms: u64,
    pub name: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenedCampaignDto {
    pub campaign_dir: String,
    pub campaign: CampaignFolderSnapshot,
    /// Pass back as `expectedNonce` when saving `campaign.json`.
    pub nonce: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignFolderRequest {
    /// The folder the user picked; the campaign is created in `<parentDir>/<campaign name>/`.
    pub parent_dir: String,
    pub campaign: CampaignFolderSnapshot,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCampaignJsonRequest {
    pub campaign_dir: String,
    pub campaign: CampaignFolderSnapshot,
    /// The nonce the screen read when it opened the campaign.
    pub expected_nonce: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedCampaignJsonDto {
    pub nonce: u64,
    /// Set when another writer saved since `expected_nonce`: their `campaign.json` was kept here.
    pub conflict_dir: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFileRequest {
    pub campaign_dir: String,
    /// `/`-separated, relative to the campaign folder.
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteCampaignFileRequest {
    pub campaign_dir: String,
    pub path: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveAttachmentRequest {
    pub campaign_dir: String,
    /// The dropped file's name; only its last path component is used.
    pub name: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFileDto {
    pub bytes: Vec<u8>,
    pub modified_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFileStatDto {
    pub exists: bool,
    pub is_dir: bool,
    pub size: u64,
    pub modified_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignTreeEntryDto {
    pub name: String,
    /// `/`-separated, relative to the campaign folder.
    pub path: String,
    pub is_dir: bool,
    pub children: Vec<CampaignTreeEntryDto>,
}

// --- known folders ---

fn known_folders_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(KNOWN_FOLDERS_FILE)
}

fn read_known(app_data_dir: &Path) -> Result<Vec<KnownFolderEntry>, String> {
    let path = known_folders_path(app_data_dir);
    match fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).map_err(|err| format!("{}: {err}", path.display())),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(err) => Err(format!("{}: {err}", path.display())),
    }
}

fn write_known(app_data_dir: &Path, entries: &[KnownFolderEntry]) -> Result<(), String> {
    fs::create_dir_all(app_data_dir).map_err(|err| format!("{}: {err}", app_data_dir.display()))?;
    let json = serde_json::to_string_pretty(entries).map_err(|err| err.to_string())?;
    folder::write_atomic(&known_folders_path(app_data_dir), json.as_bytes()).map_err(|err| err.message)
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn require_absolute(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Ok(path)
    } else {
        Err(format!("{} is not an absolute folder path", path.display()))
    }
}

/// Adds `campaign_dir` to the known list, or moves it to the front with a fresh time.
fn remember(app_data_dir: &Path, campaign_dir: &Path) -> Result<(), String> {
    let path = campaign_dir.to_string_lossy().into_owned();
    let mut entries = read_known(app_data_dir)?;
    entries.retain(|entry| entry.path != path);
    entries.insert(0, KnownFolderEntry { path, last_opened_ms: now_ms() });
    write_known(app_data_dir, &entries)
}

pub fn list_known_campaigns_at(app_data_dir: &Path) -> Result<Vec<KnownCampaignDto>, String> {
    let mut entries = read_known(app_data_dir)?;
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.last_opened_ms));
    Ok(entries
        .into_iter()
        .map(|entry| {
            let opened = folder::open(Path::new(&entry.path));
            KnownCampaignDto {
                name: opened.as_ref().ok().map(|loaded| loaded.snapshot.name.clone()),
                error: opened.err().map(|err| err.message),
                path: entry.path,
                last_opened_ms: entry.last_opened_ms,
            }
        })
        .collect())
}

/// "Open folder…": the folder must open as a campaign before it is remembered.
pub fn add_known_campaign_folder_at(app_data_dir: &Path, campaign_dir: &str) -> Result<OpenedCampaignDto, String> {
    open_campaign_folder_at(app_data_dir, campaign_dir)
}

pub fn forget_known_campaign_folder_at(app_data_dir: &Path, campaign_dir: &str) -> Result<(), String> {
    let mut entries = read_known(app_data_dir)?;
    entries.retain(|entry| entry.path != campaign_dir);
    write_known(app_data_dir, &entries)
}

// --- create / open ---

pub fn create_campaign_folder_at(
    app_data_dir: &Path,
    request: &CreateCampaignFolderRequest,
) -> Result<OpenedCampaignDto, String> {
    let parent = require_absolute(&request.parent_dir)?;
    if request.campaign.name.trim().is_empty() {
        return Err("a campaign needs a name".to_owned());
    }
    let campaign_dir = parent.join(codex::campaign::local_store::sanitize_filename(&request.campaign.name));
    let outcome = folder::create_layout(&campaign_dir, &request.campaign).map_err(|err| err.message)?;
    remember(app_data_dir, &campaign_dir)?;
    let loaded = folder::open(&campaign_dir).map_err(|err| err.message)?;
    Ok(OpenedCampaignDto {
        campaign_dir: outcome.campaign_dir.to_string_lossy().into_owned(),
        campaign: loaded.snapshot,
        nonce: loaded.nonce,
    })
}

pub fn open_campaign_folder_at(app_data_dir: &Path, campaign_dir: &str) -> Result<OpenedCampaignDto, String> {
    let dir = require_absolute(campaign_dir)?;
    let loaded = folder::open(&dir).map_err(|err| err.message)?;
    remember(app_data_dir, &dir)?;
    Ok(OpenedCampaignDto {
        campaign_dir: dir.to_string_lossy().into_owned(),
        campaign: loaded.snapshot,
        nonce: loaded.nonce,
    })
}

/// Saves an edited `campaign.json` through the nonce path (`decisions.md §11`, §13.1).
pub fn save_campaign_json_impl(request: &SaveCampaignJsonRequest) -> Result<SavedCampaignJsonDto, String> {
    let dir = require_absolute(&request.campaign_dir)?;
    if request.campaign.members.first().is_none_or(|m| m.name.trim().is_empty()) {
        return Err("a campaign keeps its creator as member #1".to_owned());
    }
    let outcome = CampaignStore::save_campaign_json_with_conflict_detection(&request.campaign, &dir, request.expected_nonce)
        .map_err(|err| err.message)?;
    Ok(SavedCampaignJsonDto {
        nonce: outcome.nonce,
        conflict_dir: outcome.conflict_dir.map(|path| path.to_string_lossy().into_owned()),
    })
}

// --- migration (decisions.md §14) ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrateCampaignRequest {
    /// One old localStorage campaign record, as the UI read it…
    pub record_json: Option<String>,
    /// …and its `codex.campaign.assets.<id>` value, if it had one.
    pub assets_json: Option<String>,
    /// Or: an old folder holding `.config/<name>.json`.
    pub config_dir: Option<String>,
    /// The new campaign is created in `<parentDir>/<campaign name>/`, as "New campaign…" does.
    pub parent_dir: String,
    /// Member #1.
    pub your_name: String,
    /// ISO timestamp from the UI's clock: member `addedAt`, the backup's date.
    pub now: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MigratedCampaignDto {
    pub campaign_dir: String,
    pub backup_file: String,
    /// Markdown files written besides `Party Resources.md`.
    pub files: usize,
}

/// Migrates one old campaign into a new folder, writes the backup under app data, and makes the
/// new folder a known campaign (an old `.config/` folder is forgotten: it no longer opens).
pub fn migrate_campaign_at(app_data_dir: &Path, request: &MigrateCampaignRequest) -> Result<MigratedCampaignDto, String> {
    use codex::campaign::migrate;
    let parent = require_absolute(&request.parent_dir)?;
    let old = match (&request.record_json, &request.config_dir) {
        (Some(record), None) => migrate::from_local_storage(record, request.assets_json.as_deref()).map_err(|err| err.message)?,
        (None, Some(dir)) => {
            let dir = require_absolute(dir)?;
            migrate::from_config_folder(&dir)
                .map_err(|err| err.message)?
                .ok_or_else(|| format!("{} has nothing to migrate (no .config/<name>.json)", dir.display()))?
        }
        _ => return Err("migrate one campaign: an old record or an old folder, not both or neither".to_owned()),
    };
    let target = parent.join(codex::campaign::local_store::sanitize_filename(&old.record.name));
    let out = migrate::migrate(&old, &target, &request.your_name, app_data_dir, &request.now).map_err(|err| err.message)?;
    if let Some(dir) = &request.config_dir {
        forget_known_campaign_folder_at(app_data_dir, dir)?;
    }
    remember(app_data_dir, &out.campaign_dir)?;
    Ok(MigratedCampaignDto {
        campaign_dir: out.campaign_dir.to_string_lossy().into_owned(),
        backup_file: out.backup_file.to_string_lossy().into_owned(),
        files: out.files.len(),
    })
}

// --- files ---

fn resolve(campaign_dir: &str, rel: &str) -> Result<PathBuf, String> {
    let dir = require_absolute(campaign_dir)?;
    folder::relative(&dir, rel).map_err(|err| err.message)
}

fn modified_ms(meta: &fs::Metadata) -> u64 {
    meta.modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn read_campaign_file_impl(request: &CampaignFileRequest) -> Result<CampaignFileDto, String> {
    let path = resolve(&request.campaign_dir, &request.path)?;
    let bytes = fs::read(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    let meta = fs::metadata(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    Ok(CampaignFileDto { bytes, modified_ms: modified_ms(&meta) })
}

/// Writes one file inside the campaign (parents created). `campaign.json` and `.codex/` are
/// refused here: they change only through the conflict-detecting save.
pub fn write_campaign_file_impl(request: &WriteCampaignFileRequest) -> Result<CampaignFileStatDto, String> {
    let path = resolve(&request.campaign_dir, &request.path)?;
    let first = Path::new(&request.path).components().find(|c| !matches!(c, std::path::Component::CurDir));
    if let Some(first) = first {
        let first = first.as_os_str();
        if first == CAMPAIGN_JSON || first == ".codex" {
            return Err(format!("{} is kept by Codex and is not written as a file", request.path));
        }
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("{}: {err}", parent.display()))?;
    }
    folder::write_atomic(&path, &request.bytes).map_err(|err| err.message)?;
    stat_campaign_file_impl(&CampaignFileRequest {
        campaign_dir: request.campaign_dir.clone(),
        path: request.path.clone(),
    })
}

pub fn stat_campaign_file_impl(request: &CampaignFileRequest) -> Result<CampaignFileStatDto, String> {
    let path = resolve(&request.campaign_dir, &request.path)?;
    match fs::metadata(&path) {
        Ok(meta) => Ok(CampaignFileStatDto {
            exists: true,
            is_dir: meta.is_dir(),
            size: meta.len(),
            modified_ms: modified_ms(&meta),
        }),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(CampaignFileStatDto {
            exists: false,
            is_dir: false,
            size: 0,
            modified_ms: 0,
        }),
        Err(err) => Err(format!("{}: {err}", path.display())),
    }
}

/// Characters that would break the `![](Attachments/<name>)` link become `-`.
fn attachment_stem(raw: &str) -> String {
    raw.chars()
        .map(|c| if c.is_whitespace() || "()[]<>#%?\\".contains(c) { '-' } else { c })
        .collect()
}

/// Copies a dragged-in image to `Attachments/<name>` (decisions.md §18, Q-F). A taken name gets
/// `-2`, `-3`…; the file is created with `create_new`, so an existing file is never overwritten.
/// Returns the path inside the campaign, e.g. `Attachments/map-2.png`.
pub fn save_attachment_impl(request: &SaveAttachmentRequest) -> Result<String, String> {
    let base = request.name.rsplit(['/', '\\']).next().unwrap_or_default();
    let (stem, ext) = match base.rfind('.') {
        Some(dot) if dot > 0 => (&base[..dot], &base[dot..]),
        _ => (base, ""),
    };
    let stem = attachment_stem(stem);
    let ext = attachment_stem(ext);
    if stem.trim_matches('-').is_empty() || stem == "." || stem == ".." {
        return Err(format!("{:?} is not a file name", request.name));
    }
    let dir = resolve(&request.campaign_dir, ATTACHMENTS_DIR)?;
    fs::create_dir_all(&dir).map_err(|err| format!("{}: {err}", dir.display()))?;
    for n in 1u32.. {
        let name = if n == 1 { format!("{stem}{ext}") } else { format!("{stem}-{n}{ext}") };
        let path = dir.join(&name);
        match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                use std::io::Write;
                if let Err(err) = file.write_all(&request.bytes) {
                    let _ = fs::remove_file(&path);
                    return Err(format!("{}: {err}", path.display()));
                }
                return Ok(format!("{}/{name}", ATTACHMENTS_DIR));
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(format!("{}: {err}", path.display())),
        }
    }
    unreachable!("u32 names run out before a folder does")
}

fn tree_of(dir: &Path, prefix: &str) -> Result<Vec<CampaignTreeEntryDto>, String> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .map_err(|err| format!("{}: {err}", dir.display()))?
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|entry| entry.file_name());
    let mut out = Vec::new();
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = if prefix.is_empty() { name.clone() } else { format!("{prefix}/{name}") };
        // file_type() does not follow symlinks, so a link out of the folder is listed, never walked.
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let children = if is_dir { tree_of(&entry.path(), &path)? } else { Vec::new() };
        out.push(CampaignTreeEntryDto { name, path, is_dir, children });
    }
    Ok(out)
}

pub fn list_campaign_tree_impl(campaign_dir: &str) -> Result<Vec<CampaignTreeEntryDto>, String> {
    let dir = require_absolute(campaign_dir)?;
    tree_of(&dir, "")
}

// --- commands ---

fn app_data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|err| format!("could not resolve app data directory: {err}"))
}

#[tauri::command]
pub fn list_known_campaigns(app: tauri::AppHandle) -> Result<Vec<KnownCampaignDto>, String> {
    list_known_campaigns_at(&app_data_dir(&app)?)
}

#[tauri::command]
pub fn add_known_campaign_folder(app: tauri::AppHandle, campaign_dir: String) -> Result<OpenedCampaignDto, String> {
    add_known_campaign_folder_at(&app_data_dir(&app)?, &campaign_dir)
}

#[tauri::command]
pub fn forget_known_campaign_folder(app: tauri::AppHandle, campaign_dir: String) -> Result<(), String> {
    forget_known_campaign_folder_at(&app_data_dir(&app)?, &campaign_dir)
}

#[tauri::command]
pub fn create_campaign_folder(
    app: tauri::AppHandle,
    request: CreateCampaignFolderRequest,
) -> Result<OpenedCampaignDto, String> {
    create_campaign_folder_at(&app_data_dir(&app)?, &request)
}

#[tauri::command]
pub fn open_campaign_folder(app: tauri::AppHandle, campaign_dir: String) -> Result<OpenedCampaignDto, String> {
    open_campaign_folder_at(&app_data_dir(&app)?, &campaign_dir)
}

#[tauri::command]
pub fn migrate_campaign(app: tauri::AppHandle, request: MigrateCampaignRequest) -> Result<MigratedCampaignDto, String> {
    migrate_campaign_at(&app_data_dir(&app)?, &request)
}

#[tauri::command]
pub fn save_campaign_json(request: SaveCampaignJsonRequest) -> Result<SavedCampaignJsonDto, String> {
    save_campaign_json_impl(&request)
}

#[tauri::command]
pub fn read_campaign_file(request: CampaignFileRequest) -> Result<CampaignFileDto, String> {
    read_campaign_file_impl(&request)
}

#[tauri::command]
pub fn write_campaign_file(request: WriteCampaignFileRequest) -> Result<CampaignFileStatDto, String> {
    write_campaign_file_impl(&request)
}

#[tauri::command]
pub fn stat_campaign_file(request: CampaignFileRequest) -> Result<CampaignFileStatDto, String> {
    stat_campaign_file_impl(&request)
}

#[tauri::command]
pub fn save_attachment(request: SaveAttachmentRequest) -> Result<String, String> {
    save_attachment_impl(&request)
}

#[tauri::command]
pub fn list_campaign_tree(campaign_dir: String) -> Result<Vec<CampaignTreeEntryDto>, String> {
    list_campaign_tree_impl(&campaign_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::campaign::{FolderMember, CAMPAIGN_FOLDER_SCHEMA_VERSION};

    fn temp_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("codex-campaign-folder-cmd-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn create_request(parent: &Path, name: &str) -> CreateCampaignFolderRequest {
        CreateCampaignFolderRequest {
            parent_dir: parent.to_string_lossy().into_owned(),
            campaign: CampaignFolderSnapshot {
                schema_version: CAMPAIGN_FOLDER_SCHEMA_VERSION,
                name: name.to_owned(),
                rule_set_id: "crb".to_owned(),
                rule_set_label: "Core Rulebook".to_owned(),
                description: String::new(),
                members: vec![FolderMember { name: "Dana".to_owned(), added_at: "2026-11-17T00:00:00Z".to_owned() }],
                created_at: "2026-11-17T00:00:00Z".to_owned(),
                updated_at: "2026-11-17T00:00:00Z".to_owned(),
            },
        }
    }

    #[test]
    fn known_folders_round_trip() {
        let root = temp_dir("known");
        let app_data = root.join("app-data");
        let campaigns = root.join("campaigns");
        let first = create_campaign_folder_at(&app_data, &create_request(&campaigns, "First")).unwrap();
        let second = create_campaign_folder_at(&app_data, &create_request(&campaigns, "Second")).unwrap();
        assert_eq!(first.campaign_dir, campaigns.join("First").to_string_lossy());
        assert_eq!(second.nonce, 1);

        let listed = list_known_campaigns_at(&app_data).unwrap();
        let names: Vec<_> = listed.iter().map(|c| c.name.clone().unwrap_or_default()).collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"First".to_owned()) && names.contains(&"Second".to_owned()));

        // Re-opening by path does not duplicate the entry; "Open folder…" refuses a non-campaign.
        add_known_campaign_folder_at(&app_data, &first.campaign_dir).unwrap();
        assert_eq!(list_known_campaigns_at(&app_data).unwrap().len(), 2);
        assert!(add_known_campaign_folder_at(&app_data, &root.to_string_lossy()).is_err());
        assert!(add_known_campaign_folder_at(&app_data, "relative/path").is_err());
        assert_eq!(list_known_campaigns_at(&app_data).unwrap().len(), 2);

        // A folder deleted outside Codex is still listed, with the reason it does not open.
        fs::remove_dir_all(&second.campaign_dir).unwrap();
        let gone = list_known_campaigns_at(&app_data)
            .unwrap()
            .into_iter()
            .find(|c| c.path == second.campaign_dir)
            .unwrap();
        assert!(gone.name.is_none() && gone.error.is_some());

        forget_known_campaign_folder_at(&app_data, &second.campaign_dir).unwrap();
        let left: Vec<_> = list_known_campaigns_at(&app_data).unwrap().into_iter().map(|c| c.path).collect();
        assert_eq!(left, vec![first.campaign_dir.clone()]);

        // A second create with the same name is refused (SD-f), never suffixed.
        assert!(create_campaign_folder_at(&app_data, &create_request(&campaigns, "First")).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn migrate_campaign_writes_a_folder_and_remembers_it() {
        let root = temp_dir("migrate");
        let app_data = root.join("app-data");
        let record = r#"{"id":"c1","name":"Old One","ruleSetId":"crb","ruleSetLabel":"Core","description":"",
            "members":[{"email":"ana@example.com"}],"partyCharacterIds":[],"createdAt":"2025-01-01T00:00:00Z",
            "updatedAt":"2025-01-02T00:00:00Z"}"#;
        let request = MigrateCampaignRequest {
            record_json: Some(record.to_owned()),
            assets_json: Some(r#"{"wiki":[{"id":"w","title":"Ameiko","body":"Innkeeper.","updatedAt":"x"}]}"#.to_owned()),
            config_dir: None,
            parent_dir: root.join("campaigns").to_string_lossy().into_owned(),
            your_name: "Gwen".to_owned(),
            now: "2026-11-17T10:00:00.000Z".to_owned(),
        };
        let out = migrate_campaign_at(&app_data, &request).unwrap();
        assert_eq!(out.campaign_dir, root.join("campaigns/Old One").to_string_lossy());
        assert_eq!(out.files, 1);
        assert!(app_data.join("campaign-migration-backup-2026-11-17.json").is_file());
        let known = list_known_campaigns_at(&app_data).unwrap();
        assert_eq!((known.len(), known[0].name.as_deref()), (1, Some("Old One")));

        // A .config/ folder: "Open folder…" refuses it (the Manager then offers migration), and it migrates.
        let old_dir = root.join("drive/Old Two");
        let snapshot: codex::campaign::CampaignSnapshot =
            serde_json::from_str(&record.replace("Old One", "Old Two")).unwrap();
        CampaignStore::save(&snapshot, &old_dir).unwrap();
        assert!(add_known_campaign_folder_at(&app_data, &old_dir.to_string_lossy()).is_err());
        assert!(codex::campaign::migrate::legacy_config_file(&old_dir).is_some());
        let from_folder = MigrateCampaignRequest {
            record_json: None,
            assets_json: None,
            config_dir: Some(old_dir.to_string_lossy().into_owned()),
            ..request
        };
        let out = migrate_campaign_at(&app_data, &from_folder).unwrap();
        assert_eq!(out.campaign_dir, root.join("campaigns/Old Two").to_string_lossy());
        assert!(codex::campaign::migrate::legacy_config_file(&old_dir).is_none());
        // Nothing left there: a second run is refused, not repeated.
        assert!(migrate_campaign_at(&app_data, &from_folder).unwrap_err().contains("nothing to migrate"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn tree_lists_the_layout() {
        let root = temp_dir("tree");
        let opened = create_campaign_folder_at(&root.join("app-data"), &create_request(&root, "Tree")).unwrap();
        let tree = list_campaign_tree_impl(&opened.campaign_dir).unwrap();
        let top: Vec<_> = tree.iter().map(|e| (e.name.as_str(), e.is_dir)).collect();
        for expected in [
            (".codex", true),
            ("Adventure Log", true),
            ("Attachments", true),
            ("Characters", true),
            ("Maps", true),
            ("Party Resources.md", false),
            ("Wiki", true),
            ("campaign.json", false),
        ] {
            assert!(top.contains(&expected), "{expected:?} missing from {top:?}");
        }
        let wiki = tree.iter().find(|e| e.name == "Wiki").unwrap();
        let wiki_paths: Vec<_> = wiki.children.iter().map(|e| e.path.as_str()).collect();
        assert_eq!(wiki_paths, vec!["Wiki/Factions", "Wiki/Items", "Wiki/NPCs", "Wiki/Places"]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn read_write_stat_a_file() {
        let root = temp_dir("files");
        let opened = create_campaign_folder_at(&root.join("app-data"), &create_request(&root, "Files")).unwrap();
        let dir = opened.campaign_dir.clone();
        let file = |path: &str| CampaignFileRequest { campaign_dir: dir.clone(), path: path.to_owned() };
        let write = |path: &str, bytes: &[u8]| WriteCampaignFileRequest {
            campaign_dir: dir.clone(),
            path: path.to_owned(),
            bytes: bytes.to_vec(),
        };

        assert!(!stat_campaign_file_impl(&file("Wiki/NPCs/Bob.md")).unwrap().exists);
        // CRLF and a BOM come back byte for byte.
        let bytes = b"\xEF\xBB\xBF# Bob\r\nA smith.\r\n";
        let stat = write_campaign_file_impl(&write("Wiki/NPCs/Bob.md", bytes)).unwrap();
        assert!(stat.exists && !stat.is_dir);
        assert_eq!(stat.size, bytes.len() as u64);
        let read = read_campaign_file_impl(&file("Wiki/NPCs/Bob.md")).unwrap();
        assert_eq!(read.bytes, bytes.to_vec());
        assert_eq!(read.modified_ms, stat.modified_ms);
        // A new subfolder is created on write.
        write_campaign_file_impl(&write("Wiki/Gods/Iomedae.md", b"# Iomedae\n")).unwrap();
        assert!(stat_campaign_file_impl(&file("Wiki/Gods")).unwrap().is_dir);

        // Outside the folder, and the files Codex keeps, are refused.
        assert!(read_campaign_file_impl(&file("../outside.md")).is_err());
        assert!(write_campaign_file_impl(&write("/tmp/x.md", b"x")).is_err());
        assert!(write_campaign_file_impl(&write("campaign.json", b"{}")).is_err());
        assert!(write_campaign_file_impl(&write(".codex/nonce", b"9")).is_err());
        assert!(stat_campaign_file_impl(&file("Wiki/../../x")).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn save_campaign_json_bumps_the_nonce_and_reports_a_conflict() {
        let root = temp_dir("save");
        let opened = create_campaign_folder_at(&root.join("app-data"), &create_request(&root, "Save")).unwrap();
        let mut edited = opened.campaign.clone();
        edited.members.push(FolderMember { name: "Eli".to_owned(), added_at: "2026-11-17T01:00:00Z".to_owned() });
        edited.description = "Edited".to_owned();
        let save = |campaign: &CampaignFolderSnapshot, nonce: u64| {
            save_campaign_json_impl(&SaveCampaignJsonRequest {
                campaign_dir: opened.campaign_dir.clone(),
                campaign: campaign.clone(),
                expected_nonce: Some(nonce),
            })
        };

        let first = save(&edited, opened.nonce).unwrap();
        assert_eq!(first.nonce, opened.nonce + 1);
        assert!(first.conflict_dir.is_none());
        let reopened = folder::open(Path::new(&opened.campaign_dir)).unwrap();
        let names: Vec<_> = reopened.snapshot.members.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["Dana", "Eli"]);
        assert_eq!(reopened.snapshot.description, "Edited");

        // A save from a stale nonce still writes (local wins) and names where the other copy went.
        let stale = save(&opened.campaign, opened.nonce).unwrap();
        let kept = stale.conflict_dir.expect("a stale nonce reports the kept copy");
        assert!(Path::new(&kept).join(CAMPAIGN_JSON).is_file());

        // A save with no member, or outside an absolute folder, is refused.
        let mut empty = opened.campaign.clone();
        empty.members.clear();
        assert!(save(&empty, stale.nonce).is_err());
        assert!(save_campaign_json_impl(&SaveCampaignJsonRequest {
            campaign_dir: "relative".to_owned(),
            campaign: opened.campaign.clone(),
            expected_nonce: None,
        })
        .is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn save_attachment_copies_and_suffixes() {
        let root = temp_dir("attach");
        let opened = create_campaign_folder_at(&root.join("app-data"), &create_request(&root, "Attach")).unwrap();
        let dir = opened.campaign_dir.clone();
        let save = |name: &str, bytes: &[u8]| {
            save_attachment_impl(&SaveAttachmentRequest {
                campaign_dir: dir.clone(),
                name: name.to_owned(),
                bytes: bytes.to_vec(),
            })
        };

        assert_eq!(save("map.png", b"one").unwrap(), "Attachments/map.png");
        // A collision adds -2, then -3; the first file is never overwritten.
        assert_eq!(save("map.png", b"two").unwrap(), "Attachments/map-2.png");
        assert_eq!(save("map.png", b"three").unwrap(), "Attachments/map-3.png");
        let read = |rel: &str| read_campaign_file_impl(&CampaignFileRequest { campaign_dir: dir.clone(), path: rel.to_owned() });
        assert_eq!(read("Attachments/map.png").unwrap().bytes, b"one".to_vec());
        assert_eq!(read("Attachments/map-2.png").unwrap().bytes, b"two".to_vec());
        // Characters that break a Markdown link become `-`; a path in the name never leaves Attachments/.
        assert_eq!(save("Old Tam (sketch).jpg", b"x").unwrap(), "Attachments/Old-Tam--sketch-.jpg");
        assert_eq!(save("../../escape.png", b"x").unwrap(), "Attachments/escape.png");
        assert_eq!(save("noext", b"x").unwrap(), "Attachments/noext");
        assert!(save("", b"x").is_err());
        let _ = fs::remove_dir_all(&root);
    }
}
