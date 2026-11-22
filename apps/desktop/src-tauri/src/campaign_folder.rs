//! Tauri commands over a campaign folder (SD-42 `decisions.md §11`): the known-folders list kept
//! in `<app data>/campaign-folders.json`, creating and opening a campaign folder, and reading,
//! writing, statting and listing its files. Every path a caller passes inside a campaign goes
//! through `codex::campaign::folder::relative`, so no command reads or writes outside the folder.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use tauri::Manager;

use codex::campaign::{adventure_log, characters, folder, maps, wiki};
use codex::campaign::local_store::{CampaignStore, CAMPAIGN_JSON};
use codex::campaign::CampaignFolderSnapshot;
use codex::saved_character::campaign_mirror;

const KNOWN_FOLDERS_FILE: &str = "campaign-folders.json";
/// Where dragged-in images go (one of `folder::LAYOUT_DIRS`).
const ATTACHMENTS_DIR: &str = "Attachments";
/// Where custom item exchange files go (SD-43 `decisions.md §21`; one of `folder::LAYOUT_DIRS`).
const CUSTOM_ITEMS_DIR: &str = "Custom Items";

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

// --- relocation (SD-43 decisions.md §7) ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelocateCampaignFolderRequest {
    pub campaign_dir: String,
    /// The folder the campaign moves to: absent, empty, or already a copy of this campaign.
    pub target_dir: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelocatedCampaignDto {
    pub campaign_dir: String,
    pub campaign: CampaignFolderSnapshot,
    pub nonce: u64,
    /// The old folder, left exactly as it was (Q-1.3).
    pub previous_dir: String,
    /// Files copied; 0 when the target already held this campaign.
    pub copied_files: usize,
}

/// Schema 2 has no campaign id, so creation time plus name stand in for one; a copy renamed
/// elsewhere is refused rather than opened, which loses nothing.
fn same_campaign(a: &CampaignFolderSnapshot, b: &CampaignFolderSnapshot) -> bool {
    a.created_at == b.created_at && a.name == b.name
}

/// Every folder and file under `dir`, relative, parents before children. A symlink is refused:
/// copying it could reach outside the campaign.
fn campaign_entries(dir: &Path, rel: &Path, out: &mut Vec<(PathBuf, bool)>) -> Result<(), String> {
    let at = dir.join(rel);
    let mut entries: Vec<_> = fs::read_dir(&at)
        .map_err(|err| format!("{}: {err}", at.display()))?
        .collect::<Result<_, _>>()
        .map_err(|err| format!("{}: {err}", at.display()))?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let child = rel.join(entry.file_name());
        let kind = entry.file_type().map_err(|err| format!("{}: {err}", entry.path().display()))?;
        if kind.is_dir() {
            out.push((child.clone(), true));
            campaign_entries(dir, &child, out)?;
        } else if kind.is_file() {
            out.push((child, false));
        } else {
            return Err(format!("{} is a link; Codex moves only files and folders", entry.path().display()));
        }
    }
    Ok(())
}

/// Writes the skeleton with the create path's `create_layout`, copies every entry over it (the
/// old `campaign.json`, nonce and files win), then compares every file's bytes.
fn copy_campaign(from: &Path, to: &Path, snapshot: &CampaignFolderSnapshot) -> Result<usize, String> {
    let mut entries = Vec::new();
    campaign_entries(from, Path::new(""), &mut entries)?;
    folder::create_layout(to, snapshot).map_err(|err| err.message)?;
    for (rel, is_dir) in &entries {
        let (src, dst) = (from.join(rel), to.join(rel));
        let copied = if *is_dir { fs::create_dir_all(&dst) } else { fs::copy(&src, &dst).map(|_| ()) };
        copied.map_err(|err| format!("{} -> {}: {err}", src.display(), dst.display()))?;
    }
    let files: Vec<_> = entries.iter().filter(|(_, is_dir)| !is_dir).map(|(rel, _)| rel).collect();
    for rel in &files {
        let read = |base: &Path| fs::read(base.join(rel)).map_err(|err| format!("{}: {err}", base.join(rel).display()));
        if read(from)? != read(to)? {
            return Err(format!("{} differs after the copy", to.join(rel).display()));
        }
    }
    Ok(files.len())
}

/// The target's path with its nearest existing ancestor resolved, so `..` or a link cannot hide
/// that it lies inside the campaign.
fn resolved(path: &Path) -> PathBuf {
    match (path.canonicalize(), path.parent(), path.file_name()) {
        (Ok(real), _, _) => real,
        (Err(_), Some(parent), Some(name)) => resolved(parent).join(name),
        _ => path.to_path_buf(),
    }
}

/// Moves a campaign to `target_dir` (Q-1.1–Q-1.3): copy, verify bytes, then switch the
/// known-folders entry; the old folder is kept. A non-empty target is refused unless it is this
/// same campaign (a synced copy), which is opened with nothing copied.
pub fn relocate_campaign_folder_at(
    app_data_dir: &Path,
    request: &RelocateCampaignFolderRequest,
) -> Result<RelocatedCampaignDto, String> {
    let from = require_absolute(&request.campaign_dir)?;
    let to = require_absolute(&request.target_dir)?;
    let current = folder::open(&from).map_err(|err| err.message)?;
    if resolved(&to).starts_with(resolved(&from)) {
        return Err(format!("{} is inside the campaign folder; pick a folder outside it", to.display()));
    }
    if to.exists() && !to.is_dir() {
        return Err(format!("{} is a file, not a folder", to.display()));
    }
    // An unreadable target is refused, never assumed empty: the cleanup below removes the target.
    let non_empty = match fs::read_dir(&to) {
        Ok(mut entries) => entries.next().is_some(),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => false,
        Err(err) => return Err(format!("{}: {err}", to.display())),
    };
    let copied_files = if non_empty {
        match folder::open(&to) {
            Ok(there) if same_campaign(&there.snapshot, &current.snapshot) => 0,
            _ => {
                return Err(format!(
                    "{} is not empty and is not a copy of {}; pick an empty folder",
                    to.display(),
                    current.snapshot.name
                ))
            }
        }
    } else {
        let existed = to.is_dir();
        copy_campaign(&from, &to, &current.snapshot).inspect_err(|_| {
            // The target was absent or empty: removing the partial copy loses nothing.
            let _ = fs::remove_dir_all(&to);
            if existed {
                let _ = fs::create_dir(&to);
            }
        })?
    };
    let loaded = folder::open(&to).map_err(|err| err.message)?;
    forget_known_campaign_folder_at(app_data_dir, &request.campaign_dir)?;
    remember(app_data_dir, &to)?;
    Ok(RelocatedCampaignDto {
        campaign_dir: to.to_string_lossy().into_owned(),
        campaign: loaded.snapshot,
        nonce: loaded.nonce,
        previous_dir: request.campaign_dir.clone(),
        copied_files,
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

/// "Create starter file" on the Party Resources tab: writes the `§15.1` starter into a campaign
/// folder that has none; an existing file is refused, never overwritten.
pub fn create_party_resources_starter_impl(campaign_dir: &str) -> Result<CampaignFileStatDto, String> {
    let dir = require_absolute(campaign_dir)?;
    folder::create_party_resources_starter(&dir).map_err(|err| err.message)?;
    stat_campaign_file_impl(&CampaignFileRequest {
        campaign_dir: campaign_dir.to_owned(),
        path: folder::PARTY_RESOURCES_FILE.to_owned(),
    })
}

/// The campaign's `Custom Items/` folder, made when a campaign from before the skeleton had it
/// lacks it. A folder that is not a campaign is refused.
pub fn custom_items_folder_impl(campaign_dir: &str) -> Result<String, String> {
    let dir = require_absolute(campaign_dir)?;
    folder::open(&dir).map_err(|err| err.message)?;
    let items = dir.join(CUSTOM_ITEMS_DIR);
    fs::create_dir_all(&items).map_err(|err| format!("{}: {err}", items.display()))?;
    Ok(items.to_string_lossy().into_owned())
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

// --- Adventure Log (decisions.md §15.2) ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAdventureLogEntryRequest {
    pub campaign_dir: String,
    pub author: String,
    pub title: String,
    /// The UI's local date, `YYYY-MM-DD`: the file's `posted` and `session`.
    pub today: String,
}

pub fn list_adventure_log_impl(campaign_dir: &str) -> Result<Vec<adventure_log::Entry>, String> {
    let dir = require_absolute(campaign_dir)?;
    adventure_log::list(&dir).map_err(|err| err.message)
}

/// "+": the new entry's path inside the campaign. The campaign's name comes from its `campaign.json`.
pub fn new_adventure_log_entry_impl(request: &NewAdventureLogEntryRequest) -> Result<String, String> {
    let dir = require_absolute(&request.campaign_dir)?;
    let name = folder::open(&dir).map_err(|err| err.message)?.snapshot.name;
    adventure_log::new_entry(&dir, &name, &request.author, &request.title, &request.today).map_err(|err| err.message)
}

// --- Maps (decisions.md §15.3) ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMapPinsRequest {
    pub campaign_dir: String,
    /// The map's notes file, `Maps/<name>.md`.
    pub notes: String,
    pub pins: Vec<maps::Pin>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMapRequest {
    pub campaign_dir: String,
    /// The picked image, an absolute path anywhere on disk.
    pub source: String,
}

pub fn list_maps_impl(campaign_dir: &str) -> Result<Vec<maps::MapEntry>, String> {
    let dir = require_absolute(campaign_dir)?;
    maps::list(&dir).map_err(|err| err.message)
}

/// Writes a map's pins into its notes file's frontmatter; only a `.md` directly under `Maps/`.
pub fn set_map_pins_impl(request: &SetMapPinsRequest) -> Result<(), String> {
    let rel = Path::new(&request.notes);
    let in_maps = rel.parent() == Some(Path::new(maps::MAPS_DIR)) && rel.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
    if !in_maps {
        return Err(format!("{:?} is not a map's notes file (Maps/<name>.md)", request.notes));
    }
    let path = resolve(&request.campaign_dir, &request.notes)?;
    maps::set_pins(&path, &request.pins).map_err(|err| err.message)
}

/// "Add map…": the copied image's path inside the campaign.
pub fn add_map_impl(request: &AddMapRequest) -> Result<String, String> {
    let dir = require_absolute(&request.campaign_dir)?;
    let source = require_absolute(&request.source)?;
    maps::add_map(&dir, &source).map_err(|err| err.message)
}

// --- commands ---

fn app_data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|err| format!("could not resolve app data directory: {err}"))
}

#[tauri::command]
pub fn get_device_identity(app: tauri::AppHandle) -> Result<campaign_mirror::DeviceIdentity, String> {
    campaign_mirror::device_identity(&app_data_dir(&app)?).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set_device_name(app: tauri::AppHandle, name: String) -> Result<campaign_mirror::DeviceIdentity, String> {
    campaign_mirror::set_device_name(&app_data_dir(&app)?, &name).map_err(|err| err.to_string())
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
pub fn relocate_campaign_folder(
    app: tauri::AppHandle,
    request: RelocateCampaignFolderRequest,
) -> Result<RelocatedCampaignDto, String> {
    relocate_campaign_folder_at(&app_data_dir(&app)?, &request)
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
pub fn create_party_resources_starter(campaign_dir: String) -> Result<CampaignFileStatDto, String> {
    create_party_resources_starter_impl(&campaign_dir)
}

#[tauri::command]
pub fn custom_items_folder(campaign_dir: String) -> Result<String, String> {
    custom_items_folder_impl(&campaign_dir)
}

#[tauri::command]
pub fn save_attachment(request: SaveAttachmentRequest) -> Result<String, String> {
    save_attachment_impl(&request)
}

#[tauri::command]
pub fn list_campaign_tree(campaign_dir: String) -> Result<Vec<CampaignTreeEntryDto>, String> {
    list_campaign_tree_impl(&campaign_dir)
}

#[tauri::command]
pub fn list_adventure_log(campaign_dir: String) -> Result<Vec<adventure_log::Entry>, String> {
    list_adventure_log_impl(&campaign_dir)
}

#[tauri::command]
pub fn new_adventure_log_entry(request: NewAdventureLogEntryRequest) -> Result<String, String> {
    new_adventure_log_entry_impl(&request)
}

#[tauri::command]
pub fn list_maps(campaign_dir: String) -> Result<Vec<maps::MapEntry>, String> {
    list_maps_impl(&campaign_dir)
}

#[tauri::command]
pub fn set_map_pins(request: SetMapPinsRequest) -> Result<(), String> {
    set_map_pins_impl(&request)
}

#[tauri::command]
pub fn add_map(request: AddMapRequest) -> Result<String, String> {
    add_map_impl(&request)
}

// --- Wiki (decisions.md §15.4) ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiPathRequest {
    pub campaign_dir: String,
    /// A folder or page inside the campaign, or (for `resolve_wiki_link`) the link text.
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiNameRequest {
    pub campaign_dir: String,
    /// The parent folder (`add_wiki_folder`), the folder (`new_wiki_page`, `rename_wiki_folder`).
    pub path: String,
    /// The new folder's or page's name.
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCampaignRequest {
    pub campaign_dir: String,
    pub query: String,
}

fn wiki_result<T>(campaign_dir: &str, run: impl FnOnce(&Path) -> Result<T, codex::campaign::CampaignStoreError>) -> Result<T, String> {
    run(&require_absolute(campaign_dir)?).map_err(|err| err.message)
}

#[tauri::command]
pub fn list_wiki_folders(campaign_dir: String) -> Result<Vec<String>, String> {
    wiki_result(&campaign_dir, wiki::folders)
}

#[tauri::command]
pub fn list_wiki_pages(campaign_dir: String) -> Result<Vec<wiki::Page>, String> {
    wiki_result(&campaign_dir, wiki::pages)
}

#[tauri::command]
pub fn add_wiki_folder(request: WikiNameRequest) -> Result<String, String> {
    wiki_result(&request.campaign_dir, |dir| wiki::add_folder(dir, &request.path, &request.name))
}

#[tauri::command]
pub fn rename_wiki_folder(request: WikiNameRequest) -> Result<String, String> {
    wiki_result(&request.campaign_dir, |dir| wiki::rename_folder(dir, &request.path, &request.name))
}

/// Empty when the folder was deleted; else its files, none deleted.
#[tauri::command]
pub fn delete_wiki_folder(request: WikiPathRequest) -> Result<Vec<String>, String> {
    wiki_result(&request.campaign_dir, |dir| wiki::delete_folder(dir, &request.path))
}

#[tauri::command]
pub fn new_wiki_page(request: WikiNameRequest) -> Result<String, String> {
    wiki_result(&request.campaign_dir, |dir| wiki::new_page(dir, &request.path, &request.name))
}

#[tauri::command]
pub fn resolve_wiki_link(request: WikiPathRequest) -> Result<Option<String>, String> {
    wiki_result(&request.campaign_dir, |dir| wiki::resolve_link(dir, &request.path))
}

#[tauri::command]
pub fn wiki_backlinks(request: WikiPathRequest) -> Result<Vec<String>, String> {
    wiki_result(&request.campaign_dir, |dir| wiki::backlinks(dir, &request.path))
}

#[tauri::command]
pub fn search_campaign(request: SearchCampaignRequest) -> Result<Vec<wiki::Hit>, String> {
    wiki_result(&request.campaign_dir, |dir| wiki::search(dir, &request.query))
}

// --- Characters/ backups (decisions.md §16) ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignCharacterRequest {
    pub campaign_dir: String,
    /// The backup's folder name in `Characters/`.
    pub folder: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportCampaignCharacterRequest {
    pub campaign_dir: String,
    pub folder: String,
    pub mode: characters::ImportMode,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedCampaignCharacterDto {
    pub character_id: String,
}

fn characters_root(app_data_dir: &Path) -> PathBuf {
    crate::character_hub::characters_root_from_app_data_dir(app_data_dir)
}

/// The character's campaign memberships (`campaigns.json`; none when absent), for the print
/// dialog's Campaign cell.
pub fn load_character_campaigns_at(app_data_dir: &Path, character_id: &str) -> Result<Vec<campaign_mirror::Membership>, String> {
    crate::character_hub::validate_character_id(character_id)?;
    campaign_mirror::load_memberships(&characters_root(app_data_dir).join(character_id)).map_err(|err| err.message)
}

pub fn list_campaign_characters_impl(campaign_dir: &str) -> Result<Vec<characters::Backup>, String> {
    characters::list(&require_absolute(campaign_dir)?).map_err(|err| err.message)
}

pub fn import_campaign_character_at(app_data_dir: &Path, request: &ImportCampaignCharacterRequest) -> Result<ImportedCampaignCharacterDto, String> {
    let campaign_dir = require_absolute(&request.campaign_dir)?;
    characters::import(&campaign_dir, &request.folder, &characters_root(app_data_dir), request.mode)
        .map(|imported| ImportedCampaignCharacterDto { character_id: imported.character_id })
        .map_err(|err| err.message)
}

/// The files that differ between this device's copy of the character and its backup.
pub fn diff_campaign_character_at(app_data_dir: &Path, request: &CampaignCharacterRequest) -> Result<Vec<characters::FileDiff>, String> {
    let backup = characters::backup_path(&require_absolute(&request.campaign_dir)?, &request.folder).map_err(|err| err.message)?;
    let character_id = characters::backup_character_id(&backup).map_err(|err| err.message)?;
    let local = characters_root(app_data_dir).join(&character_id);
    if !local.is_dir() {
        return Err(format!("{} is not on this device", request.folder));
    }
    characters::diff(&local, &backup).map_err(|err| err.message)
}

/// The sheet of a backup, read-only: the load path writes first-load files into the root it reads,
/// so it reads a fresh copy under `<app data>/campaign-character-view/`, never the shared backup.
pub fn load_campaign_character_at(app_data_dir: &Path, request: &CampaignCharacterRequest) -> Result<crate::character_hub::LoadSavedCharacterResponse, String> {
    let backup = characters::backup_path(&require_absolute(&request.campaign_dir)?, &request.folder).map_err(|err| err.message)?;
    let view = app_data_dir.join("campaign-character-view").join(&request.folder);
    if view.exists() {
        fs::remove_dir_all(&view).map_err(|err| format!("could not clear {}: {err}", view.display()))?;
    }
    copy_backup(&backup, &view).map_err(|err| format!("could not copy {}: {err}", backup.display()))?;
    crate::character_hub::load_saved_character_at_root(&view)
}

fn copy_backup(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_backup(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn load_character_campaigns(app: tauri::AppHandle, character_id: String) -> Result<Vec<campaign_mirror::Membership>, String> {
    load_character_campaigns_at(&app_data_dir(&app)?, &character_id)
}

#[tauri::command]
pub fn list_campaign_characters(campaign_dir: String) -> Result<Vec<characters::Backup>, String> {
    list_campaign_characters_impl(&campaign_dir)
}

#[tauri::command]
pub fn import_campaign_character(app: tauri::AppHandle, request: ImportCampaignCharacterRequest) -> Result<ImportedCampaignCharacterDto, String> {
    import_campaign_character_at(&app_data_dir(&app)?, &request)
}

#[tauri::command]
pub fn diff_campaign_character(app: tauri::AppHandle, request: CampaignCharacterRequest) -> Result<Vec<characters::FileDiff>, String> {
    diff_campaign_character_at(&app_data_dir(&app)?, &request)
}

#[tauri::command]
pub fn load_campaign_character(app: tauri::AppHandle, request: CampaignCharacterRequest) -> Result<crate::character_hub::LoadSavedCharacterResponse, String> {
    load_campaign_character_at(&app_data_dir(&app)?, &request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::campaign::{FolderMember, CAMPAIGN_FOLDER_SCHEMA_VERSION};
    use codex::saved_character::campaign_mirror::{self, Membership};

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

    /// Every file under `dir` as (`/`-path, bytes), sorted; written apart from the production walk.
    fn files_with_bytes(dir: &Path) -> Vec<(String, Vec<u8>)> {
        let mut out = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(at) = stack.pop() {
            for entry in fs::read_dir(&at).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    let rel = path.strip_prefix(dir).unwrap().to_string_lossy().replace('\\', "/");
                    out.push((rel, fs::read(&path).unwrap()));
                }
            }
        }
        out.sort();
        out
    }

    fn known_paths(app_data: &Path) -> Vec<String> {
        list_known_campaigns_at(app_data).unwrap().into_iter().map(|c| c.path).collect()
    }

    fn relocate_request(from: &str, to: &Path) -> RelocateCampaignFolderRequest {
        RelocateCampaignFolderRequest { campaign_dir: from.to_owned(), target_dir: to.to_string_lossy().into_owned() }
    }

    #[test]
    fn relocate_campaign_copies_every_byte_completes_the_skeleton_and_keeps_the_old_folder() {
        let root = temp_dir("relocate-copy");
        let app_data = root.join("app-data");
        let old = create_campaign_folder_at(&app_data, &create_request(&root.join("campaigns"), "Moving")).unwrap();
        let old_dir = PathBuf::from(&old.campaign_dir);
        fs::write(old_dir.join("Wiki/NPCs/Bob.md"), "# Bob\n").unwrap();
        fs::write(old_dir.join("Attachments/map.png"), (0u8..=255).collect::<Vec<u8>>()).unwrap();
        fs::create_dir_all(old_dir.join("Wiki/Custom/Empty")).unwrap();
        // A folder made by hand or synced in part: the skeleton is incomplete before the move.
        fs::remove_dir_all(old_dir.join("Maps")).unwrap();
        fs::remove_file(old_dir.join(folder::PARTY_RESOURCES_FILE)).unwrap();
        let saved = save_campaign_json_impl(&SaveCampaignJsonRequest {
            campaign_dir: old.campaign_dir.clone(),
            campaign: old.campaign.clone(),
            expected_nonce: Some(old.nonce),
        })
        .unwrap();
        let before = files_with_bytes(&old_dir);

        let target = root.join("synced").join("Moving");
        let moved = relocate_campaign_folder_at(&app_data, &relocate_request(&old.campaign_dir, &target)).unwrap();

        assert_eq!(moved.campaign_dir, target.to_string_lossy());
        assert_eq!(moved.previous_dir, old.campaign_dir);
        assert_eq!(moved.copied_files, before.len());
        assert_eq!(moved.nonce, saved.nonce, "the nonce file travels with the campaign");
        assert_eq!(moved.campaign, old.campaign);
        // Every byte of every old file is at the target, and the old folder is untouched.
        let after: Vec<_> = files_with_bytes(&target).into_iter().filter(|(rel, _)| rel != folder::PARTY_RESOURCES_FILE).collect();
        assert_eq!(after, before);
        assert_eq!(files_with_bytes(&old_dir), before);
        assert!(target.join("Wiki/Custom/Empty").is_dir(), "empty folders are copied too");
        // The skeleton is complete at the target.
        for sub in folder::LAYOUT_DIRS {
            assert!(target.join(sub).is_dir(), "{sub} missing at the target");
        }
        assert_eq!(fs::read_to_string(target.join(folder::PARTY_RESOURCES_FILE)).unwrap(), folder::PARTY_RESOURCES_STARTER);
        // The known-folders entry now points at the target, and only there.
        assert_eq!(known_paths(&app_data), vec![moved.campaign_dir.clone()]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn relocate_campaign_refuses_a_foreign_non_empty_target() {
        let root = temp_dir("relocate-foreign");
        let app_data = root.join("app-data");
        let campaigns = root.join("campaigns");
        let mine = create_campaign_folder_at(&app_data, &create_request(&campaigns, "Mine")).unwrap();
        let other = create_campaign_folder_at(&app_data, &create_request(&campaigns, "Other")).unwrap();
        let known_before = known_paths(&app_data);

        let stray = root.join("stray");
        fs::create_dir_all(&stray).unwrap();
        fs::write(stray.join("notes.txt"), "keep me").unwrap();
        for target in [stray.clone(), PathBuf::from(&other.campaign_dir)] {
            let before = files_with_bytes(&target);
            let err = relocate_campaign_folder_at(&app_data, &relocate_request(&mine.campaign_dir, &target)).unwrap_err();
            assert!(err.contains("not empty"), "{err}");
            assert_eq!(files_with_bytes(&target), before, "a refused target is not written");
        }
        // A target inside the campaign itself is refused too.
        let inside = PathBuf::from(&mine.campaign_dir).join("Wiki").join("Moved");
        assert!(relocate_campaign_folder_at(&app_data, &relocate_request(&mine.campaign_dir, &inside)).is_err());
        assert!(!inside.exists());
        assert!(relocate_campaign_folder_at(&app_data, &relocate_request(&mine.campaign_dir, Path::new("relative"))).is_err());
        // A file, or a folder that cannot be listed, is refused and left in place.
        let file_target = root.join("a-file");
        fs::write(&file_target, "x").unwrap();
        assert!(relocate_campaign_folder_at(&app_data, &relocate_request(&mine.campaign_dir, &file_target)).is_err());
        assert!(file_target.is_file());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let locked = root.join("locked");
            fs::create_dir_all(locked.join("inside")).unwrap();
            fs::set_permissions(&locked, fs::Permissions::from_mode(0o300)).unwrap();
            let refused = relocate_campaign_folder_at(&app_data, &relocate_request(&mine.campaign_dir, &locked));
            fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).unwrap();
            assert!(refused.is_err());
            assert!(locked.join("inside").is_dir(), "an unlistable target is never cleaned up");
        }
        assert_eq!(known_paths(&app_data), known_before);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn relocate_campaign_opens_a_same_campaign_target_and_copies_nothing() {
        let root = temp_dir("relocate-same");
        let app_data = root.join("app-data");
        let old = create_campaign_folder_at(&app_data, &create_request(&root.join("campaigns"), "Synced")).unwrap();
        let old_dir = PathBuf::from(&old.campaign_dir);
        // The synced copy: the same campaign.json and nonce, one file the old folder lacks.
        let target = root.join("cloud").join("Synced");
        for (rel, bytes) in files_with_bytes(&old_dir) {
            let path = target.join(&rel);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
        }
        fs::create_dir_all(target.join("Wiki")).unwrap();
        fs::write(target.join("Wiki/Synced only.md"), "from the cloud").unwrap();
        fs::write(old_dir.join("Wiki/Local only.md"), "stays local").unwrap();
        let target_before = files_with_bytes(&target);

        let moved = relocate_campaign_folder_at(&app_data, &relocate_request(&old.campaign_dir, &target)).unwrap();
        assert_eq!(moved.copied_files, 0);
        assert_eq!(moved.campaign_dir, target.to_string_lossy());
        assert_eq!(moved.previous_dir, old.campaign_dir);
        assert_eq!(files_with_bytes(&target), target_before, "an opened copy is not written");
        assert!(old_dir.join("Wiki/Local only.md").is_file());
        assert_eq!(known_paths(&app_data), vec![moved.campaign_dir.clone()]);
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
    fn party_resources_starter_is_written_only_when_missing() {
        let root = temp_dir("starter");
        let opened = create_campaign_folder_at(&root.join("app-data"), &create_request(&root, "Starter")).unwrap();
        let dir = opened.campaign_dir.clone();
        let path = Path::new(&dir).join(folder::PARTY_RESOURCES_FILE);
        assert!(create_party_resources_starter_impl(&dir).is_err(), "the starter E2.1 wrote is kept");
        fs::remove_file(&path).unwrap();
        let stat = create_party_resources_starter_impl(&dir).unwrap();
        assert!(stat.exists && !stat.is_dir);
        assert_eq!(fs::read_to_string(&path).unwrap(), folder::PARTY_RESOURCES_STARTER);
        assert!(create_party_resources_starter_impl("relative/dir").is_err());
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

    #[test]
    fn adventure_log_commands_create_and_list() {
        let root = temp_dir("adventure-log");
        let opened = create_campaign_folder_at(&root.join("app-data"), &create_request(&root, "Log")).unwrap();
        let request = |title: &str| NewAdventureLogEntryRequest {
            campaign_dir: opened.campaign_dir.clone(),
            author: "Dana".to_owned(),
            title: title.to_owned(),
            today: "2026-11-17".to_owned(),
        };
        let rel = new_adventure_log_entry_impl(&request("Session 1")).unwrap();
        assert_eq!(rel, "Adventure Log/2026-11-17 Dana - Session 1.md");
        let text = String::from_utf8(read_campaign_file_impl(&CampaignFileRequest { campaign_dir: opened.campaign_dir.clone(), path: rel.clone() }).unwrap().bytes).unwrap();
        assert!(text.contains(&format!("campaign: \"{}\"", opened.campaign.name)), "{text}");
        let listed = list_adventure_log_impl(&opened.campaign_dir).unwrap();
        assert_eq!(listed.iter().map(|e| e.path.as_str()).collect::<Vec<_>>(), [rel.as_str()]);
        assert!(new_adventure_log_entry_impl(&request("Session 1")).is_err(), "a taken name is refused");
        assert!(list_adventure_log_impl("relative").is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn maps_commands_add_pin_and_list() {
        let root = temp_dir("maps");
        let opened = create_campaign_folder_at(&root.join("app-data"), &create_request(&root, "Maps")).unwrap();
        let picked = root.join("Sandpoint.png");
        fs::write(&picked, b"png bytes").unwrap();
        let add = |source: &Path| add_map_impl(&AddMapRequest { campaign_dir: opened.campaign_dir.clone(), source: source.to_string_lossy().into_owned() });
        assert_eq!(add(&picked).unwrap(), "Maps/Sandpoint.png");
        assert!(add(&picked).is_err(), "a taken name is refused");
        assert!(add(Path::new("relative.png")).is_err(), "the picked image must be an absolute path");
        let set = |notes: &str, pins: Vec<maps::Pin>| {
            set_map_pins_impl(&SetMapPinsRequest { campaign_dir: opened.campaign_dir.clone(), notes: notes.to_owned(), pins })
        };
        let two = vec![
            maps::Pin { x: 0.25, y: 0.5, label: "Ferry".to_owned(), page: "Riverside Ferry".to_owned() },
            maps::Pin { x: 0.75, y: 0.1, label: "Tower".to_owned(), page: "Old Light".to_owned() },
        ];
        set("Maps/Sandpoint.md", two.clone()).unwrap();
        let listed = list_maps_impl(&opened.campaign_dir).unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!((listed[0].image.as_deref(), listed[0].pins.clone()), (Some("Maps/Sandpoint.png"), two));
        // Pins are written only into a notes file under Maps/.
        assert!(set("Party Resources.md", vec![]).is_err());
        assert!(set("Maps/../campaign.json", vec![]).is_err());
        assert!(set("Maps/Sandpoint.png", vec![]).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn wiki_commands_reach_the_folder() {
        let root = temp_dir("wiki");
        let opened = create_campaign_folder_at(&root.join("app-data"), &create_request(&root.join("campaigns"), "Wiki")).unwrap();
        let campaign_dir = opened.campaign_dir.clone();
        let named = |path: &str, name: &str| WikiNameRequest { campaign_dir: campaign_dir.clone(), path: path.to_owned(), name: name.to_owned() };
        let at = |path: &str| WikiPathRequest { campaign_dir: campaign_dir.clone(), path: path.to_owned() };
        assert_eq!(list_wiki_folders(campaign_dir.clone()).unwrap(), ["Wiki/Factions", "Wiki/Items", "Wiki/NPCs", "Wiki/Places"]);
        assert_eq!(new_wiki_page(named("Wiki/NPCs", "Ameiko")).unwrap(), "Wiki/NPCs/Ameiko.md");
        assert_eq!(new_wiki_page(named("Wiki/Places", "Inn")).unwrap(), "Wiki/Places/Inn.md");
        fs::write(Path::new(&campaign_dir).join("Wiki/Places/Inn.md"), "Run by [[Ameiko]].\n").unwrap();
        assert_eq!(resolve_wiki_link(at("[[ameiko|her]]")).unwrap().as_deref(), Some("Wiki/NPCs/Ameiko.md"));
        assert_eq!(wiki_backlinks(at("Wiki/NPCs/Ameiko.md")).unwrap(), ["Wiki/Places/Inn.md"]);
        assert_eq!(search_campaign(SearchCampaignRequest { campaign_dir: campaign_dir.clone(), query: "run by".to_owned() }).unwrap()[0].path, "Wiki/Places/Inn.md");
        assert_eq!(list_wiki_pages(campaign_dir.clone()).unwrap().len(), 2);
        assert_eq!(add_wiki_folder(named("Wiki", "Gods")).unwrap(), "Wiki/Gods");
        assert_eq!(rename_wiki_folder(named("Wiki/Gods", "Deities")).unwrap(), "Wiki/Deities");
        assert!(delete_wiki_folder(at("Wiki/Deities")).unwrap().is_empty());
        assert_eq!(delete_wiki_folder(at("Wiki/NPCs")).unwrap(), ["Wiki/NPCs/Ameiko.md"]);
        assert!(list_wiki_folders("relative/path".to_owned()).is_err(), "an absolute campaign folder only");
        let _ = fs::remove_dir_all(&root);
    }

    // --- Characters/ backups (decisions.md §16) ---

    /// Device A's fighter saved into a new campaign folder; returns (campaign dir, A's app data).
    fn campaign_with_a_backup(label: &str) -> (PathBuf, PathBuf) {
        let root = temp_dir(label);
        let campaign = create_campaign_folder_at(&root.join("app-a"), &create_request(&root, "Runelords")).unwrap();
        let campaign_dir = PathBuf::from(&campaign.campaign_dir);
        let app_a = root.join("app-a");
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/sd14/pf1_human_fighter_level1_saved_character");
        let mut envelope = codex::saved_character::local_store::SavedCharacterStore::load(&fixture).expect("fixture loads");
        envelope.character_id = "char-1".to_owned();
        envelope.display_label = "Aldric".to_owned();
        let root_a = characters_root(&app_a).join("char-1");
        fs::create_dir_all(&root_a).unwrap();
        let membership = Membership { campaign_id: campaign.campaign_dir.clone(), campaign_name: "Runelords".to_owned(), campaign_dir: campaign_dir.clone(), active: true, stand_in: false, last_backup_counter: None };
        campaign_mirror::save_memberships(&root_a, &[membership]).unwrap();
        assert!(crate::character_hub::save_and_mirror(&envelope, &root_a).unwrap().is_empty());
        (campaign_dir, root)
    }

    fn import_request(campaign_dir: &Path, mode: characters::ImportMode) -> ImportCampaignCharacterRequest {
        ImportCampaignCharacterRequest { campaign_dir: campaign_dir.to_string_lossy().into_owned(), folder: "aldric-char-1".to_owned(), mode }
    }

    #[test]
    fn import_campaign_character_as_stand_in_lands_in_the_list_and_diffs() {
        let (campaign_dir, root) = campaign_with_a_backup("import-stand-in");
        let listed = list_campaign_characters_impl(&campaign_dir.to_string_lossy()).unwrap();
        assert_eq!(listed.iter().map(|b| (b.folder.as_str(), b.label.as_str(), b.counter)).collect::<Vec<_>>(), [("aldric-char-1", "Aldric", Some(1))]);
        let app_b = root.join("app-b");

        let imported = import_campaign_character_at(&app_b, &import_request(&campaign_dir, characters::ImportMode::StandIn)).unwrap();

        assert_eq!(imported.character_id, "char-1");
        let listing = codex::saved_character::local_store::SavedCharacterStore::list_all(&characters_root(&app_b)).unwrap();
        assert_eq!(listing.characters.iter().map(|c| c.character_id.as_str()).collect::<Vec<_>>(), ["char-1"], "the copy is in this device's character list");
        let root_b = characters_root(&app_b).join("char-1");
        assert!(campaign_mirror::load_memberships(&root_b).unwrap()[0].stand_in);
        let request = CampaignCharacterRequest { campaign_dir: campaign_dir.to_string_lossy().into_owned(), folder: "aldric-char-1".to_owned() };
        assert!(diff_campaign_character_at(&app_b, &request).unwrap().is_empty(), "a fresh stand-in equals its backup");
        fs::write(root_b.join("bio.json"), "{}").unwrap();
        let diffs = diff_campaign_character_at(&app_b, &request).unwrap();
        assert_eq!(diffs.iter().map(|d| d.path.as_str()).collect::<Vec<_>>(), ["bio.json"]);
        assert!(diff_campaign_character_at(&root.join("app-c"), &request).is_err(), "no local copy, no diff");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn import_campaign_character_move_master_next_save_names_this_device() {
        let (campaign_dir, root) = campaign_with_a_backup("import-move");
        let app_b = root.join("app-b");

        import_campaign_character_at(&app_b, &import_request(&campaign_dir, characters::ImportMode::MoveMaster)).unwrap();
        let root_b = characters_root(&app_b).join("char-1");
        let envelope = codex::saved_character::local_store::SavedCharacterStore::load(&root_b).unwrap();
        assert!(crate::character_hub::save_and_mirror(&envelope, &root_b).unwrap().is_empty());

        let master = campaign_mirror::read_master(&campaign_dir.join("Characters").join("aldric-char-1")).unwrap().unwrap();
        assert_eq!(master.device_id, campaign_mirror::device_identity(&app_b).unwrap().id);
        assert_eq!(master.save_counter, 2);
        assert!(import_campaign_character_at(&app_b, &import_request(&campaign_dir, characters::ImportMode::StandIn)).is_err(), "never over a local copy");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn load_campaign_character_reads_a_copy_never_the_backup() {
        let (campaign_dir, root) = campaign_with_a_backup("load-backup");
        let backup = campaign_dir.join("Characters").join("aldric-char-1");
        let before: Vec<_> = fs::read_dir(&backup).unwrap().map(|e| e.unwrap().file_name()).collect();
        let request = CampaignCharacterRequest { campaign_dir: campaign_dir.to_string_lossy().into_owned(), folder: "aldric-char-1".to_owned() };

        let loaded = load_campaign_character_at(&root.join("app-b"), &request).expect("loads");

        assert_eq!((loaded.summary.character_id.as_str(), loaded.summary.display_label.as_str()), ("char-1", "Aldric"), "the backup's sheet loads");
        let after: Vec<_> = fs::read_dir(&backup).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(after, before, "the shared backup gains no file");
        assert!(load_campaign_character_at(&root.join("app-b"), &CampaignCharacterRequest { folder: "../x".to_owned(), ..request }).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn load_character_campaigns_reads_the_memberships_file() {
        let app_data = temp_dir("character-campaigns");
        assert!(load_character_campaigns_at(&app_data, "char-none").unwrap().is_empty(), "no campaigns.json → no campaigns");
        let root = characters_root(&app_data).join("char-aldric");
        fs::create_dir_all(&root).unwrap();
        let membership = Membership { campaign_id: "/c/Runelords".to_owned(), campaign_name: "Runelords".to_owned(), campaign_dir: PathBuf::from("/c/Runelords"), active: true, stand_in: false, last_backup_counter: None };
        campaign_mirror::save_memberships(&root, std::slice::from_ref(&membership)).unwrap();
        assert_eq!(load_character_campaigns_at(&app_data, "char-aldric").unwrap(), vec![membership]);
        assert!(load_character_campaigns_at(&app_data, "../char-aldric").is_err(), "a traversal id is refused");
        let _ = fs::remove_dir_all(&app_data);
    }

    /// Item 4 (Q-4.2): a new campaign has `Custom Items/` at its root, and the folder command makes
    /// it in a campaign created before the skeleton had it.
    #[test]
    fn custom_items_exchange_campaign_folder_has_custom_items() {
        let root = temp_dir("custom-items");
        let campaigns = root.join("campaigns");
        let made = create_campaign_folder_at(&root.join("app-data"), &create_request(&campaigns, "Items")).unwrap();
        assert!(Path::new(&made.campaign_dir).join(CUSTOM_ITEMS_DIR).is_dir(), "the skeleton has Custom Items/");
        fs::remove_dir(Path::new(&made.campaign_dir).join(CUSTOM_ITEMS_DIR)).unwrap();
        let dir = custom_items_folder_impl(&made.campaign_dir).unwrap();
        assert_eq!(Path::new(&dir), Path::new(&made.campaign_dir).join(CUSTOM_ITEMS_DIR));
        assert!(Path::new(&dir).is_dir(), "an older campaign gains it on first use");
        assert!(custom_items_folder_impl(&root.to_string_lossy()).is_err(), "a folder that is not a campaign is refused");
        let _ = fs::remove_dir_all(&root);
    }
}
