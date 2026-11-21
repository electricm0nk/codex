//! The campaign folder's `Characters/` backups (SD-42 `decisions.md §16`): listing them, importing
//! one onto this device as a stand-in or as the moved master, and the file diff between a local
//! copy and its backup (shown, never merged). The backups are written by
//! `saved_character::campaign_mirror`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::local_store::CAMPAIGN_JSON;
use super::{CampaignFolderSnapshot, CampaignStoreError};
use crate::rules_core::cpr::store::CprCharacterStore;
use crate::saved_character::campaign_mirror::{self, Membership, BACKUP_DIR, MASTER_FILE, MEMBERSHIPS_FILE};
use crate::saved_character::local_store::SavedCharacterStore;

/// One `Characters/<folder>/` backup as the Characters list shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub folder: String,
    pub character_id: String,
    pub label: String,
    pub game_system: String,
    /// `class_id:level` pairs, comma-separated (the hub list's `classSummary` shape).
    pub classes: String,
    pub race: String,
    /// The master device's name; `None` for a backup with no `master.json` (SD-q).
    pub master_device: Option<String>,
    pub counter: Option<u64>,
    /// The master's last write, else the envelope's `saved_at`.
    pub saved_at: String,
    /// Why the folder could not be read; it is listed anyway.
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ImportMode {
    /// Play in the master's place; this copy never writes the backup.
    StandIn,
    /// This device becomes the master: its next save writes `master.json` naming it.
    MoveMaster,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedCharacter {
    pub character_id: String,
    pub root: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Newer {
    Local,
    Backup,
    Same,
}

/// One file that differs between a local copy and its backup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDiff {
    pub path: String,
    pub local_bytes: Option<u64>,
    pub backup_bytes: Option<u64>,
    /// The side modified last (a file on one side only is that side's).
    pub newer: Newer,
    /// For `envelope.txt`: the `key = value` fields whose values differ.
    pub changed_fields: Vec<String>,
}

fn error(message: String) -> CampaignStoreError {
    CampaignStoreError { message }
}

/// The backups folder to read: `Characters/`, else an SD-41 `characters/` not renamed yet.
fn backups_to_read(campaign_dir: &Path) -> Option<PathBuf> {
    [BACKUP_DIR, "characters"].iter().map(|name| campaign_dir.join(name)).find(|dir| dir.is_dir())
}

/// Every backup folder under `<campaign_dir>/Characters/`, sorted by folder name; a folder that
/// does not read is listed with its error.
pub fn list(campaign_dir: &Path) -> Result<Vec<Backup>, CampaignStoreError> {
    let Some(dir) = backups_to_read(campaign_dir) else { return Ok(Vec::new()) };
    let mut folders: Vec<String> = fs::read_dir(&dir)
        .map_err(|err| error(format!("could not read {}: {err}", dir.display())))?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| !name.contains(".tmp-"))
        .collect();
    folders.sort();
    Ok(folders.into_iter().map(|folder| summarize(&dir.join(&folder), folder)).collect())
}

fn summarize(backup: &Path, folder: String) -> Backup {
    let master = campaign_mirror::read_master(backup);
    let mut row = Backup {
        folder,
        character_id: String::new(),
        label: String::new(),
        game_system: String::new(),
        classes: String::new(),
        race: String::new(),
        master_device: None,
        counter: None,
        saved_at: String::new(),
        error: None,
    };
    match SavedCharacterStore::load(backup) {
        Ok(envelope) => {
            row.classes = envelope.character_input.chosen.class_levels.iter().map(|held| format!("{}:{}", held.class_id, held.level)).collect::<Vec<_>>().join(",");
            row.race = envelope.character_input.chosen.race_id;
            (row.character_id, row.label, row.game_system, row.saved_at) = (envelope.character_id, envelope.display_label, envelope.game_system, envelope.saved_at);
        }
        Err(err) => match CprCharacterStore::load(backup) {
            Ok(cpr) => (row.character_id, row.label, row.game_system, row.saved_at) = (cpr.character_id, cpr.display_label, crate::rules_core::game_system::GameSystem::CyberpunkRed.id().to_owned(), cpr.saved_at),
            Err(_) => row.error = Some(err.message),
        },
    }
    match master {
        Ok(Some(master)) => {
            row.master_device = Some(master.device_name);
            row.counter = Some(master.save_counter);
            row.saved_at = master.saved_at;
        }
        Ok(None) => {}
        Err(message) => row.error = Some(message),
    }
    row
}

/// The character id a backup holds (a Pathfinder/Starfinder envelope, else a Cyberpunk RED one).
pub fn backup_character_id(backup: &Path) -> Result<String, CampaignStoreError> {
    match SavedCharacterStore::load(backup) {
        Ok(envelope) => Ok(envelope.character_id),
        Err(err) => CprCharacterStore::load(backup).map(|cpr| cpr.character_id).map_err(|_| error(err.message)),
    }
}

/// A backup folder inside `<campaign_dir>/Characters/`: one plain path segment.
pub fn backup_path(campaign_dir: &Path, folder: &str) -> Result<PathBuf, CampaignStoreError> {
    let plain = !folder.is_empty() && folder != "." && folder != ".." && !folder.contains(['/', '\\']);
    let dir = backups_to_read(campaign_dir).filter(|_| plain).map(|dir| dir.join(folder));
    dir.filter(|path| path.is_dir()).ok_or_else(|| error(format!("{folder} is not a backup in {}", campaign_dir.display())))
}

/// Copies a backup into `<characters_root>/<character id>/` (never over a local copy) and writes its
/// membership: a stand-in, or the master moved here (its next save writes `master.json`).
pub fn import(campaign_dir: &Path, folder: &str, characters_root: &Path, mode: ImportMode) -> Result<ImportedCharacter, CampaignStoreError> {
    let backup = backup_path(campaign_dir, folder)?;
    let character_id = backup_character_id(&backup)?;
    if character_id.is_empty() || !character_id.chars().all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '.')) || character_id.starts_with('.') {
        return Err(error(format!("the backup's character id {character_id:?} is not a folder name")));
    }
    let root = characters_root.join(&character_id);
    if root.exists() {
        return Err(error(format!("this character is already on this device ({}); open it from the character list", root.display())));
    }
    let master = campaign_mirror::read_master(&backup).map_err(error)?;
    let campaign_name = fs::read_to_string(campaign_dir.join(CAMPAIGN_JSON))
        .ok()
        .and_then(|text| serde_json::from_str::<CampaignFolderSnapshot>(&text).ok())
        .map(|snapshot| snapshot.name)
        .unwrap_or_else(|| campaign_dir.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default());
    let membership = Membership {
        campaign_id: campaign_dir.display().to_string(),
        campaign_name,
        campaign_dir: campaign_dir.to_path_buf(),
        active: true,
        stand_in: mode == ImportMode::StandIn,
        last_backup_counter: master.map(|master| master.save_counter),
    };
    let tmp = characters_root.join(format!("{character_id}.tmp-{}", std::process::id()));
    let copied = fs::create_dir_all(characters_root)
        .and_then(|()| campaign_mirror::copy_tree(&backup, &tmp, true))
        .map_err(|err| error(format!("could not copy {}: {err}", backup.display())))
        .and_then(|()| campaign_mirror::save_memberships(&tmp, &[membership]).map_err(|err| error(err.message)))
        .and_then(|()| fs::rename(&tmp, &root).map_err(|err| error(format!("could not write {}: {err}", root.display()))));
    if copied.is_err() {
        let _ = fs::remove_dir_all(&tmp);
    }
    copied.map(|()| ImportedCharacter { character_id, root })
}

fn files(dir: &Path, prefix: &str, out: &mut BTreeMap<String, (u64, Option<SystemTime>)>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy().into_owned();
        let Ok(meta) = entry.metadata() else { continue };
        if meta.is_dir() {
            files(&entry.path(), &format!("{prefix}{name}/"), out);
        } else if !(prefix.is_empty() && (name == MEMBERSHIPS_FILE || name == MASTER_FILE)) {
            out.insert(format!("{prefix}{name}"), (meta.len(), meta.modified().ok()));
        }
    }
}

fn envelope_fields(path: &Path) -> BTreeMap<String, String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.split_once(" = ").map(|(key, value)| (key.trim().to_owned(), value.to_owned())))
        .collect()
}

/// The files that differ between a local copy and its backup (by name, size and content), with the
/// side changed last. Nothing is merged (Q22).
pub fn diff(local_root: &Path, backup_root: &Path) -> Result<Vec<FileDiff>, CampaignStoreError> {
    for root in [local_root, backup_root] {
        if !root.is_dir() {
            return Err(error(format!("{} is not a folder", root.display())));
        }
    }
    let (mut local, mut backup) = (BTreeMap::new(), BTreeMap::new());
    files(local_root, "", &mut local);
    files(backup_root, "", &mut backup);
    let paths: std::collections::BTreeSet<&String> = local.keys().chain(backup.keys()).collect();
    let mut out = Vec::new();
    for path in paths {
        let (mine, theirs) = (local.get(path), backup.get(path));
        if mine.is_some() && theirs.is_some() && fs::read(local_root.join(path)).ok() == fs::read(backup_root.join(path)).ok() {
            continue;
        }
        let newer = match (mine.and_then(|m| m.1), theirs.and_then(|t| t.1), mine, theirs) {
            (_, _, Some(_), None) => Newer::Local,
            (_, _, None, Some(_)) => Newer::Backup,
            (Some(a), Some(b), _, _) if a > b => Newer::Local,
            (Some(a), Some(b), _, _) if b > a => Newer::Backup,
            _ => Newer::Same,
        };
        let changed_fields = if path == "envelope.txt" {
            let (a, b) = (envelope_fields(&local_root.join(path)), envelope_fields(&backup_root.join(path)));
            let keys: std::collections::BTreeSet<&String> = a.keys().chain(b.keys()).collect();
            keys.into_iter().filter(|key| a.get(*key) != b.get(*key)).cloned().collect()
        } else {
            Vec::new()
        };
        out.push(FileDiff { path: path.clone(), local_bytes: mine.map(|m| m.0), backup_bytes: theirs.map(|t| t.0), newer, changed_fields });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::saved_character::campaign_mirror::{self, device_identity, load_memberships, read_master, save_memberships, Membership, BACKUP_DIR};
    use crate::saved_character::local_store::SavedCharacterStore;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time").as_nanos();
        let path = std::env::temp_dir().join(format!("codex-campaign-characters-{label}-{}-{unique}", std::process::id()));
        fs::create_dir_all(&path).expect("temp dir");
        path
    }

    fn membership(campaign_dir: &Path) -> Membership {
        Membership {
            campaign_id: campaign_dir.display().to_string(),
            campaign_name: "Rise of the Runelords".to_owned(),
            campaign_dir: campaign_dir.to_path_buf(),
            active: true,
            stand_in: false,
            last_backup_counter: None,
        }
    }

    /// A campaign folder with a `campaign.json`, and device A's saved fighter in it (`<app a>/characters/<id>`).
    fn campaign_with_a_backup(label: &str) -> (PathBuf, PathBuf) {
        let campaign = tempdir(&format!("{label}-campaign"));
        fs::write(campaign.join("campaign.json"), r#"{"schemaVersion":2,"name":"Rise of the Runelords","ruleSetId":"pf1","ruleSetLabel":"Pathfinder","createdAt":"t","updatedAt":"t"}"#).unwrap();
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sd14/pf1_human_fighter_level1_saved_character");
        let mut envelope = SavedCharacterStore::load(&fixture).expect("fixture loads");
        envelope.character_id = "char-1".to_owned();
        envelope.display_label = "Aldric".to_owned();
        let root = tempdir(&format!("{label}-device-a")).join("characters").join("char-1");
        save_memberships(&root_ready(&root), &[membership(&campaign)]).unwrap();
        let warnings = SavedCharacterStore::save_with_warnings(&envelope, &root).expect("device A saves");
        assert!(warnings.is_empty(), "{warnings:?}");
        (campaign, root)
    }

    fn root_ready(root: &Path) -> PathBuf {
        fs::create_dir_all(root).unwrap();
        root.to_path_buf()
    }

    /// Device B's app data, with a device name the test can look for.
    fn device_b(label: &str) -> PathBuf {
        let app = tempdir(&format!("{label}-device-b"));
        fs::write(app.join(campaign_mirror::DEVICE_FILE), r#"{"id":"device-b","name":"Bea's laptop"}"#).unwrap();
        app
    }

    #[test]
    fn characters_backup_list_reads_summaries() {
        let (campaign, _root) = campaign_with_a_backup("list");
        fs::create_dir_all(campaign.join(BACKUP_DIR).join("broken-x")).unwrap();

        let backups = list(&campaign).expect("lists");

        assert_eq!(backups.len(), 2, "{backups:?}");
        let aldric = &backups[0];
        assert_eq!((aldric.folder.as_str(), aldric.character_id.as_str(), aldric.label.as_str()), ("aldric-char-1", "char-1", "Aldric"));
        assert_eq!(aldric.classes, "class:fighter:1");
        assert_eq!(aldric.race, "race:human");
        assert_eq!(aldric.counter, Some(1));
        assert!(aldric.master_device.as_deref().is_some_and(|name| !name.is_empty()), "{aldric:?}");
        assert!(aldric.saved_at.ends_with('Z') && aldric.error.is_none(), "{aldric:?}");
        assert_eq!(backups[1].folder, "broken-x");
        assert!(backups[1].error.is_some(), "an unreadable backup is listed with why, never hidden");
        assert!(list(&tempdir("list-empty")).unwrap().is_empty(), "no Characters/ → no backups");
    }

    #[test]
    fn characters_backup_move_master_takes_over() {
        let (campaign, root_a) = campaign_with_a_backup("move");
        let app_b = device_b("move");
        let backup = campaign.join(BACKUP_DIR).join("aldric-char-1");

        let imported = import(&campaign, "aldric-char-1", &app_b.join("characters"), ImportMode::MoveMaster).expect("imports");

        assert_eq!(imported.character_id, "char-1", "the same character, so its backup is the same folder");
        let root_b = app_b.join("characters").join("char-1");
        assert_eq!(imported.root, root_b);
        assert!(!root_b.join(campaign_mirror::MASTER_FILE).exists(), "master.json stays in the backup");
        let membership_b = &load_memberships(&root_b).unwrap()[0];
        assert!(!membership_b.stand_in && membership_b.active);
        assert_eq!((membership_b.campaign_name.as_str(), membership_b.last_backup_counter), ("Rise of the Runelords", Some(1)));

        // B's next save writes master.json naming B.
        let envelope = SavedCharacterStore::load(&root_b).unwrap();
        assert!(SavedCharacterStore::save_with_warnings(&envelope, &root_b).unwrap().is_empty());
        let master = read_master(&backup).unwrap().unwrap();
        assert_eq!((master.device_id.as_str(), master.device_name.as_str(), master.save_counter), ("device-b", "Bea's laptop", 2));

        // A's next save hits check 2: warned, backup untouched, A's local save done.
        let mut envelope_a = SavedCharacterStore::load(&root_a).unwrap();
        envelope_a.display_label = "Aldric".to_owned();
        envelope_a.saved_at = "2026-11-18T09:00:00Z".to_owned();
        let warnings = SavedCharacterStore::save_with_warnings(&envelope_a, &root_a).unwrap();
        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert!(warnings[0].reason.contains("Bea's laptop"), "{}", warnings[0].reason);
        assert_eq!(read_master(&backup).unwrap().unwrap().device_id, "device-b");
        assert_eq!(SavedCharacterStore::load(&root_a).unwrap().saved_at, "2026-11-18T09:00:00Z");
        assert_ne!(device_identity(root_a.parent().unwrap().parent().unwrap()).unwrap().id, "device-b");

        let again = import(&campaign, "aldric-char-1", &app_b.join("characters"), ImportMode::StandIn);
        assert!(again.is_err_and(|err| err.message.contains("already")), "a second import never overwrites a local copy");
    }

    #[test]
    fn characters_backup_import_as_stand_in_never_writes_back() {
        let (campaign, _root_a) = campaign_with_a_backup("stand-in");
        let app_b = device_b("stand-in");
        let backup = campaign.join(BACKUP_DIR).join("aldric-char-1");
        let before = fs::read(backup.join("envelope.txt")).unwrap();

        let imported = import(&campaign, "aldric-char-1", &app_b.join("characters"), ImportMode::StandIn).expect("imports");
        let membership_b = &load_memberships(&imported.root).unwrap()[0];
        assert!(membership_b.stand_in, "{membership_b:?}");
        let mut envelope = SavedCharacterStore::load(&imported.root).unwrap();
        envelope.display_label = "Aldric (played by Bea)".to_owned();
        assert!(SavedCharacterStore::save_with_warnings(&envelope, &imported.root).unwrap().is_empty());

        assert_eq!(fs::read(backup.join("envelope.txt")).unwrap(), before, "a stand-in save never reaches the backup");
        assert_eq!(read_master(&backup).unwrap().unwrap().save_counter, 1);
        assert!(import(&campaign, "../x", &app_b.join("characters"), ImportMode::StandIn).is_err(), "a folder name is one path segment");
    }

    #[test]
    fn characters_backup_diff_lists_changed_files() {
        let (campaign, root) = campaign_with_a_backup("diff");
        let backup = campaign.join(BACKUP_DIR).join("aldric-char-1");
        fs::write(root.join("bio.json"), r#"{"deity":"Iomedae"}"#).unwrap();
        fs::write(backup.join("notes.md"), "their notes").unwrap();
        let envelope = fs::read_to_string(backup.join("envelope.txt")).unwrap().replace("display_label = Aldric", "display_label = Aldric the Bold");
        fs::write(backup.join("envelope.txt"), envelope).unwrap();

        let diffs = diff(&root, &backup).expect("diffs");

        let paths: Vec<&str> = diffs.iter().map(|d| d.path.as_str()).collect();
        assert_eq!(paths, vec!["bio.json", "envelope.txt", "notes.md"], "campaigns.json and master.json are not compared; equal files are not listed");
        assert_eq!((diffs[0].local_bytes, diffs[0].backup_bytes), (Some(19), None));
        assert_eq!(diffs[0].newer, Newer::Local);
        assert_eq!(diffs[1].changed_fields, vec!["display_label".to_owned()]);
        assert_eq!((diffs[2].local_bytes, diffs[2].backup_bytes, diffs[2].newer), (None, Some(11), Newer::Backup));
    }
}
