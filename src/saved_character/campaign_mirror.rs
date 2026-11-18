//! The write-through of a saved character's folder into each campaign folder it belongs to:
//! `<campaign>/Characters/<slug>-<id>/` (SD-41 `decisions.md §29`, SD-42 `§16`). The local copy is the
//! one master; the backup carries `master.json` (which device wrote it, its save counter) and is
//! written only when its counter is the one this install last wrote, so a second master is never
//! silently overwritten. A stand-in copy never writes its backup.

use std::fs;
use std::hash::{BuildHasher, Hasher};
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::campaign::local_store::sanitize_filename;
use crate::saved_character::SavedCharacterStoreError;

/// The character's campaign memberships, kept in its own root; never copied to a campaign.
pub const MEMBERSHIPS_FILE: &str = "campaigns.json";
/// The backups' folder in a campaign (SD-41 wrote `characters/`; the first write renames it, SD-q).
pub const BACKUP_DIR: &str = "Characters";
const OLD_BACKUP_DIR: &str = "characters";
/// In a backup: which device is the master and how many times it has written the backup.
pub const MASTER_FILE: &str = "master.json";
/// In the app's data root: this install's identity, minted once.
pub const DEVICE_FILE: &str = "device.json";

/// One campaign the character belongs to. Inactive = removed: the copy stays, saves stop going there.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub campaign_id: String,
    pub campaign_name: String,
    pub campaign_dir: PathBuf,
    pub active: bool,
    /// Imported from another device's backup to play in its place: never writes the backup.
    #[serde(default)]
    pub stand_in: bool,
    /// The `saveCounter` this install last wrote to (or imported from) the backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_backup_counter: Option<u64>,
}

/// A campaign copy that could not be written. The local save has already succeeded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorWarning {
    pub campaign_dir: PathBuf,
    pub reason: String,
}

/// This install: a random id and a readable name (the host name).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceIdentity {
    pub id: String,
    pub name: String,
}

/// A backup's `master.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasterRecord {
    pub device_id: String,
    pub device_name: String,
    pub save_counter: u64,
    pub saved_at: String,
}

/// `<root>/campaigns.json`; absent → no memberships.
pub fn load_memberships(root: &Path) -> Result<Vec<Membership>, SavedCharacterStoreError> {
    let path = root.join(MEMBERSHIPS_FILE);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(store_error(format!("could not read {}: {err}", path.display()))),
    };
    serde_json::from_str(&text).map_err(|err| store_error(format!("{} is not a campaigns file: {err}", path.display())))
}

/// Write `<root>/campaigns.json` (temp file + rename).
pub fn save_memberships(root: &Path, memberships: &[Membership]) -> Result<(), SavedCharacterStoreError> {
    let path = root.join(MEMBERSHIPS_FILE);
    let text = serde_json::to_string_pretty(memberships).map_err(|err| store_error(format!("could not write the campaigns: {err}")))?;
    write_atomic(&path, &text).map_err(|err| store_error(format!("could not write {}: {err}", path.display())))
}

/// The folder-name part from the display label: `sanitize_filename`, lower-cased, spaces → `-`.
pub fn character_slug(label: &str) -> String {
    sanitize_filename(label).to_lowercase().replace(' ', "-")
}

/// The backup's folder name in `Characters/`: `<slug>-<character id>`.
pub fn backup_folder(label: &str, character_id: &str) -> String {
    format!("{}-{}", character_slug(label), sanitize_filename(character_id))
}

/// `<app data>/device.json`, minted on first use: a random id and the host name.
pub fn device_identity(app_data: &Path) -> Result<DeviceIdentity, SavedCharacterStoreError> {
    let path = app_data.join(DEVICE_FILE);
    match fs::read_to_string(&path) {
        Ok(text) => return serde_json::from_str(&text).map_err(|err| store_error(format!("{} is not a device file: {err}", path.display()))),
        Err(err) if err.kind() != io::ErrorKind::NotFound => return Err(store_error(format!("could not read {}: {err}", path.display()))),
        Err(_) => {}
    }
    let device = DeviceIdentity { id: random_id(), name: host_name() };
    write_device(app_data, &device)?;
    Ok(device)
}

fn write_device(app_data: &Path, device: &DeviceIdentity) -> Result<(), SavedCharacterStoreError> {
    let path = app_data.join(DEVICE_FILE);
    let text = serde_json::to_string_pretty(device).map_err(|err| store_error(err.to_string()))?;
    fs::create_dir_all(app_data)
        .and_then(|()| write_atomic(&path, &text))
        .map_err(|err| store_error(format!("could not write {}: {err}", path.display())))
}

/// Renames this device (the name a backup's `master.json` shows); trimmed, never blank, keeps the id.
pub fn set_device_name(app_data: &Path, name: &str) -> Result<DeviceIdentity, SavedCharacterStoreError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(store_error("the device name cannot be blank".to_string()));
    }
    let device = DeviceIdentity { name: name.to_string(), ..device_identity(app_data)? };
    write_device(app_data, &device)?;
    Ok(device)
}

/// The app's data root for a character root: the app keeps characters at `<app data>/characters/<id>`.
/// A root kept anywhere else (a test's temp root) keeps its identity in the folder that holds it.
pub fn app_data_for_root(root: &Path) -> PathBuf {
    let parent = root.parent().unwrap_or(root);
    match (parent.file_name(), parent.parent()) {
        (Some(name), Some(app_data)) if name == "characters" => app_data.to_path_buf(),
        _ => parent.to_path_buf(),
    }
}

/// A backup's `master.json`; `None` when it has none (a backup SD-41 wrote, or no backup yet).
pub fn read_master(backup: &Path) -> Result<Option<MasterRecord>, String> {
    let path = backup.join(MASTER_FILE);
    match fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).map(Some).map_err(|err| format!("{} is not readable: {err}", path.display())),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(format!("could not read {}: {err}", path.display())),
    }
}

pub fn write_master(backup: &Path, master: &MasterRecord) -> io::Result<()> {
    let text = serde_json::to_string_pretty(master).map_err(io::Error::other)?;
    write_atomic(&backup.join(MASTER_FILE), &text)
}

/// `<campaign>/Characters/`, renaming SD-41's `characters/` on the first write.
pub fn backups_dir(campaign_dir: &Path) -> io::Result<PathBuf> {
    let target = campaign_dir.join(BACKUP_DIR);
    let names: Vec<std::ffi::OsString> = fs::read_dir(campaign_dir)?.filter_map(|entry| entry.ok().map(|e| e.file_name())).collect();
    let old = campaign_dir.join(OLD_BACKUP_DIR);
    if names.iter().any(|name| name == OLD_BACKUP_DIR) && !names.iter().any(|name| name == BACKUP_DIR) && old.is_dir() {
        // Through a temp name, so a case-insensitive disk renames it too.
        let step = campaign_dir.join(format!("{OLD_BACKUP_DIR}.renaming-{}", std::process::id()));
        fs::rename(&old, &step)?;
        fs::rename(&step, &target)?;
    }
    Ok(target)
}

/// `mirror_as` with this install's identity (`app_data_for_root`).
pub fn mirror(root: &Path, label: &str, character_id: &str, memberships: &[Membership]) -> Vec<MirrorWarning> {
    if !memberships.iter().any(|membership| membership.active && !membership.stand_in) {
        return Vec::new();
    }
    match device_identity(&app_data_for_root(root)) {
        Ok(device) => mirror_as(&device, root, label, character_id, memberships),
        Err(err) => vec![MirrorWarning { campaign_dir: root.to_path_buf(), reason: format!("no backup written: {}", err.message) }],
    }
}

/// Copy every file under `root` except `campaigns.json` to `<campaign_dir>/Characters/<slug>-<id>/`
/// for each active membership that is not a stand-in, replacing the old copy whole, with a
/// `master.json` naming `device` and the next save counter. A backup whose counter is not the one
/// this install last wrote is left alone (a warning naming who changed it and when). The counters
/// written are kept in `<root>/campaigns.json`. Failures are warnings per campaign; the loop always continues.
pub fn mirror_as(device: &DeviceIdentity, root: &Path, label: &str, character_id: &str, memberships: &[Membership]) -> Vec<MirrorWarning> {
    let folder = backup_folder(label, character_id);
    let mut updated = memberships.to_vec();
    let mut warnings = Vec::new();
    for membership in updated.iter_mut().filter(|membership| membership.active && !membership.stand_in) {
        match mirror_one(device, root, membership, &folder) {
            Ok(counter) => membership.last_backup_counter = Some(counter),
            Err(reason) => warnings.push(MirrorWarning { campaign_dir: membership.campaign_dir.clone(), reason }),
        }
    }
    if updated != memberships && let Err(err) = save_memberships(root, &updated) {
        warnings.push(MirrorWarning { campaign_dir: root.join(MEMBERSHIPS_FILE), reason: err.message });
    }
    warnings
}

/// Writes one backup; returns the counter written.
fn mirror_one(device: &DeviceIdentity, root: &Path, membership: &Membership, folder: &str) -> Result<u64, String> {
    let campaign_dir = &membership.campaign_dir;
    // Never create a missing campaign dir: it means the shared folder is not there.
    if !campaign_dir.is_dir() {
        return Err("the campaign folder does not exist or is not a folder".to_owned());
    }
    let characters = backups_dir(campaign_dir).map_err(|err| err.to_string())?;
    let target = characters.join(folder);
    let counter = match read_master(&target)? {
        None => 0,
        Some(master) => {
            let ours = membership.last_backup_counter == Some(master.save_counter)
                || (membership.last_backup_counter.is_none() && master.device_id == device.id);
            if !ours {
                return Err(format!(
                    "the campaign copy was changed by {} on {}; your local save is safe; re-import to see their version",
                    master.device_name, master.saved_at
                ));
            }
            master.save_counter
        }
    };
    let master = MasterRecord { device_id: device.id.clone(), device_name: device.name.clone(), save_counter: counter + 1, saved_at: iso_utc(now_secs()) };
    let tmp = characters.join(format!("{folder}.tmp-{}", std::process::id()));
    let result = (|| -> io::Result<()> {
        fs::create_dir_all(&characters)?;
        if tmp.exists() {
            fs::remove_dir_all(&tmp)?;
        }
        copy_tree(root, &tmp, true)?;
        write_master(&tmp, &master)?;
        if target.exists() {
            fs::remove_dir_all(&target)?;
        }
        fs::rename(&tmp, &target)
    })();
    result.map(|()| master.save_counter).map_err(|err| {
        let _ = fs::remove_dir_all(&tmp);
        err.to_string()
    })
}

/// Copies a character folder; at the top level `campaigns.json` (the local memberships) and
/// `master.json` (the backup's own record) are never copied.
pub(crate) fn copy_tree(from: &Path, to: &Path, top: bool) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let name = entry.file_name();
        if top && (name == MEMBERSHIPS_FILE || name == MASTER_FILE) {
            continue;
        }
        if kind.is_dir() {
            copy_tree(&entry.path(), &to.join(&name), false)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), to.join(&name))?;
        }
    }
    Ok(())
}

fn write_atomic(path: &Path, text: &str) -> io::Result<()> {
    let tmp = path.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(&tmp, text).and_then(|()| fs::rename(&tmp, path))
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// `YYYY-MM-DDTHH:MM:SSZ` for seconds since the Unix epoch (civil-from-days, proleptic Gregorian).
pub fn iso_utc(secs: u64) -> String {
    let (days, rem) = ((secs / 86_400) as i64, secs % 86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z", rem / 3_600, rem % 3_600 / 60, rem % 60)
}

/// 128 random bits as a UUID-shaped string (std's per-process random hasher keys, time and pid).
fn random_id() -> String {
    let mut words = [0u64; 2];
    for (index, word) in words.iter_mut().enumerate() {
        let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
        hasher.write_u128(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0));
        hasher.write_u32(std::process::id());
        hasher.write_usize(index);
        *word = hasher.finish();
    }
    let hex = format!("{:016x}{:016x}", words[0], words[1]);
    format!("{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32])
}

/// The host name, for people to read ("Bea's laptop"); "This device" when none is found.
fn host_name() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .ok()
        .or_else(|| fs::read_to_string("/etc/hostname").ok())
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "This device".to_owned())
}

fn store_error(message: String) -> SavedCharacterStoreError {
    SavedCharacterStoreError { message }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::saved_character::local_store::SavedCharacterStore;
    use sha2::{Digest, Sha256};
    use std::collections::BTreeMap;

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codex-campaign-mirror-{label}-{}-{unique}", std::process::id()));
        fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    fn membership(campaign_dir: &Path, active: bool) -> Membership {
        Membership {
            campaign_id: "camp-1".to_owned(),
            campaign_name: "Rise of the Runelords".to_owned(),
            campaign_dir: campaign_dir.to_path_buf(),
            active,
            stand_in: false,
            last_backup_counter: None,
        }
    }

    /// `<app data>` of a root made by `character_root`.
    fn app_data(root: &Path) -> PathBuf {
        root.parent().and_then(Path::parent).expect("root is <app data>/characters/<id>").to_path_buf()
    }

    /// A character root as the app keeps it (`<app data>/characters/<id>`): the two store files plus
    /// sidecars and a nested file.
    fn character_root(label: &str) -> PathBuf {
        let root = tempdir(label).join("characters").join("char-1");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("envelope.txt"), "character_id = x\n").unwrap();
        fs::write(root.join("authoritative_character_input.txt"), "race = race:human\n").unwrap();
        fs::write(root.join("bio.json"), r#"{"deity":"Iomedae","backstory":"Born in Sandpoint."}"#).unwrap();
        fs::write(root.join("equipment_sets.json"), r#"{"sets":[]}"#).unwrap();
        fs::write(root.join("portrait.png"), [0x89u8, b'P', b'N', b'G', 0, 1, 2, 255]).unwrap();
        fs::create_dir_all(root.join("notes")).unwrap();
        fs::write(root.join("notes").join("session1.txt"), "met the mayor").unwrap();
        root
    }

    fn sha256_tree(dir: &Path) -> BTreeMap<PathBuf, String> {
        fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<PathBuf, String>) {
            for entry in fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    walk(base, &path, out);
                } else {
                    let digest = Sha256::digest(fs::read(&path).unwrap());
                    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
                    out.insert(path.strip_prefix(base).unwrap().to_path_buf(), hex);
                }
            }
        }
        let mut out = BTreeMap::new();
        walk(dir, dir, &mut out);
        out
    }

    #[test]
    fn campaign_mirror_two_alike_names_two_folders() {
        let campaign = tempdir("alike-campaign");
        let first = character_root("alike-a");
        let second = character_root("alike-b");
        fs::write(second.join("bio.json"), r#"{"deity":"Gorum"}"#).unwrap();
        let memberships = [membership(&campaign, true)];

        assert!(mirror(&first, "Aldric", "char-aaa", &memberships).is_empty());
        assert!(mirror(&second, "Aldric", "char-bbb", &memberships).is_empty());

        let a = campaign.join(BACKUP_DIR).join("aldric-char-aaa");
        let b = campaign.join(BACKUP_DIR).join("aldric-char-bbb");
        assert!(a.is_dir() && b.is_dir(), "two characters named alike get two folders");
        assert_ne!(a, b);
        assert_eq!(fs::read_to_string(a.join("bio.json")).unwrap(), r#"{"deity":"Iomedae","backstory":"Born in Sandpoint."}"#);
        assert_eq!(fs::read_to_string(b.join("bio.json")).unwrap(), r#"{"deity":"Gorum"}"#);
        assert_eq!(character_slug("Sir Aldric of Sandpoint"), "sir-aldric-of-sandpoint");
        assert_eq!(character_slug("Ald/ric"), "ald_ric");
    }

    #[test]
    fn campaign_mirror_copies_every_file_byte_identical() {
        let campaign = tempdir("bytes-campaign");
        let root = character_root("bytes");
        save_memberships(&root, &[membership(&campaign, true)]).unwrap();
        assert_eq!(load_memberships(&root).unwrap(), vec![membership(&campaign, true)]);

        let warnings = mirror(&root, "Aldric", "char-1", &load_memberships(&root).unwrap());
        assert!(warnings.is_empty(), "{warnings:?}");

        let mut expected = sha256_tree(&root);
        assert!(expected.remove(Path::new(MEMBERSHIPS_FILE)).is_some(), "campaigns.json was written locally");
        let copy = campaign.join(BACKUP_DIR).join("aldric-char-1");
        let mut copied = sha256_tree(&copy);
        assert!(copied.remove(Path::new(MASTER_FILE)).is_some(), "the copy carries master.json");
        assert_eq!(copied, expected, "every file but campaigns.json, byte-identical");
        assert_eq!(expected.len(), 6);
        let leftovers: Vec<_> = fs::read_dir(campaign.join(BACKUP_DIR)).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(leftovers, vec![std::ffi::OsString::from("aldric-char-1")], "no temp folder left behind");
    }

    #[test]
    fn campaign_mirror_inactive_membership_writes_nothing() {
        let campaign = tempdir("inactive-campaign");
        let root = character_root("inactive");
        assert!(load_memberships(&root).unwrap().is_empty(), "absent campaigns.json → no memberships");

        let warnings = mirror(&root, "Aldric", "char-1", &[membership(&campaign, false)]);
        assert!(warnings.is_empty());
        assert_eq!(fs::read_dir(&campaign).unwrap().count(), 0, "an inactive membership writes nothing");
    }

    #[test]
    fn campaign_mirror_missing_dir_is_a_warning_local_intact() {
        let parent = tempdir("missing-parent");
        let missing = parent.join("not-there");
        let reachable = tempdir("missing-reachable");
        let root = character_root("missing");
        let before = sha256_tree(&root);

        let warnings = mirror(&root, "Aldric", "char-1", &[membership(&missing, true), membership(&reachable, true)]);

        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert_eq!(warnings[0].campaign_dir, missing);
        assert!(!warnings[0].reason.is_empty());
        assert!(!missing.exists(), "a missing campaign dir is never created");
        let mut after = sha256_tree(&root);
        assert!(after.remove(Path::new(MEMBERSHIPS_FILE)).is_some(), "the reachable copy's counter is kept in campaigns.json");
        assert_eq!(after, before, "the character's files untouched");
        let counters: Vec<Option<u64>> = load_memberships(&root).unwrap().iter().map(|m| m.last_backup_counter).collect();
        assert_eq!(counters, vec![None, Some(1)]);
        assert!(reachable.join(BACKUP_DIR).join("aldric-char-1").join("bio.json").is_file(), "the loop continues past the warning");
    }

    #[test]
    fn campaign_mirror_second_save_replaces_the_folder() {
        let campaign = tempdir("replace-campaign");
        let root = tempdir("replace-root").join("characters").join("char-replace");
        fs::create_dir_all(&root).unwrap();
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sd14/pf1_human_fighter_level1_saved_character");
        let mut envelope = SavedCharacterStore::load(&fixture).expect("fixture loads");
        envelope.character_id = "char-replace".to_owned();
        envelope.display_label = "Aldric".to_owned();
        save_memberships(&root, &[membership(&campaign, true)]).unwrap();
        fs::write(root.join("custom.json"), "[]").unwrap();

        let warnings = SavedCharacterStore::save_with_warnings(&envelope, &root).expect("first save");
        assert!(warnings.is_empty(), "{warnings:?}");
        let copy = campaign.join(BACKUP_DIR).join("aldric-char-replace");
        assert!(copy.join("custom.json").is_file());
        assert_eq!(fs::read(copy.join("envelope.txt")).unwrap(), fs::read(root.join("envelope.txt")).unwrap());

        fs::remove_file(root.join("custom.json")).unwrap();
        envelope.revision_id = "char-replace.rev.2".to_owned();
        SavedCharacterStore::save(&envelope, &root).expect("second save");

        assert!(!copy.join("custom.json").exists(), "a file deleted locally is gone from the copy");
        assert!(fs::read_to_string(copy.join("envelope.txt")).unwrap().contains("char-replace.rev.2"));
        assert!(!copy.join(MEMBERSHIPS_FILE).exists());
    }

    fn backup(campaign: &Path) -> PathBuf {
        campaign.join(BACKUP_DIR).join("aldric-char-1")
    }

    #[test]
    fn characters_backup_target_is_capital_c_and_renames_old() {
        let campaign = tempdir("rename-campaign");
        let old = campaign.join("characters").join("aldric-char-old");
        fs::create_dir_all(&old).unwrap();
        fs::write(old.join("envelope.txt"), "an SD-41 copy").unwrap();
        let root = character_root("rename");

        let warnings = mirror(&root, "Aldric", "char-1", &[membership(&campaign, true)]);

        assert!(warnings.is_empty(), "{warnings:?}");
        let names: Vec<String> = fs::read_dir(&campaign).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
        assert_eq!(names, vec![BACKUP_DIR.to_owned()], "characters/ is renamed, not kept beside Characters/");
        assert_eq!(fs::read_to_string(campaign.join(BACKUP_DIR).join("aldric-char-old").join("envelope.txt")).unwrap(), "an SD-41 copy", "the old copy moves with it");
        assert!(backup(&campaign).join("bio.json").is_file());
    }

    #[test]
    fn characters_backup_device_name_is_saved_and_keeps_the_id() {
        let app = tempdir("device-name");
        let before = device_identity(&app).unwrap();
        let after = set_device_name(&app, "  Table laptop ").unwrap();
        assert_eq!((after.id.as_str(), after.name.as_str()), (before.id.as_str(), "Table laptop"));
        assert_eq!(device_identity(&app).unwrap(), after, "the next read, and so the next backup, carries the new name");
    }

    #[test]
    fn characters_backup_device_name_refuses_blank() {
        let app = tempdir("device-name-blank");
        let before = device_identity(&app).unwrap();
        assert!(set_device_name(&app, "   ").is_err());
        assert_eq!(device_identity(&app).unwrap(), before, "a refused name changes nothing");
    }

    #[test]
    fn characters_backup_writes_master_json() {
        let campaign = tempdir("master-campaign");
        let root = character_root("master");
        let app = app_data(&root);
        save_memberships(&root, &[membership(&campaign, true)]).unwrap();

        assert!(mirror(&root, "Aldric", "char-1", &load_memberships(&root).unwrap()).is_empty());

        let device = device_identity(&app).expect("device.json was minted by the first backup");
        assert!(!device.id.is_empty() && !device.name.is_empty(), "{device:?}");
        let master = read_master(&backup(&campaign)).unwrap().expect("master.json is written with the copy");
        assert_eq!((master.device_id.as_str(), master.device_name.as_str(), master.save_counter), (device.id.as_str(), device.name.as_str(), 1));
        assert_eq!(master.saved_at.len(), "2026-11-18T00:00:00Z".len(), "{}", master.saved_at);
        assert!(master.saved_at.starts_with("20") && master.saved_at.ends_with('Z'), "{}", master.saved_at);
        assert_eq!(load_memberships(&root).unwrap()[0].last_backup_counter, Some(1), "the counter this install wrote is kept");
        assert!(!root.join(MASTER_FILE).exists(), "master.json lives only in the backup");

        assert!(mirror(&root, "Aldric", "char-1", &load_memberships(&root).unwrap()).is_empty());
        assert_eq!(read_master(&backup(&campaign)).unwrap().unwrap().save_counter, 2, "each backup write bumps the counter");
        assert_eq!(load_memberships(&root).unwrap()[0].last_backup_counter, Some(2));
        assert_eq!(device_identity(&app).unwrap(), device, "minted once, then read");
        assert_eq!(iso_utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_utc(1_763_424_000), "2025-11-18T00:00:00Z");
        assert_eq!(iso_utc(951_782_400), "2000-02-29T00:00:00Z");
    }

    #[test]
    fn characters_backup_counter_mismatch_does_not_overwrite_and_warns() {
        let campaign = tempdir("mismatch-campaign");
        let root = character_root("mismatch");
        save_memberships(&root, &[membership(&campaign, true)]).unwrap();
        assert!(mirror(&root, "Aldric", "char-1", &load_memberships(&root).unwrap()).is_empty());
        // Another device took the master over and saved since.
        let theirs = MasterRecord { device_id: "device-b".to_owned(), device_name: "Bea's laptop".to_owned(), save_counter: 5, saved_at: "2026-11-17T20:00:00Z".to_owned() };
        write_master(&backup(&campaign), &theirs).unwrap();
        fs::write(backup(&campaign).join("bio.json"), "their bio").unwrap();
        fs::write(root.join("bio.json"), "my bio").unwrap();

        let warnings = mirror(&root, "Aldric", "char-1", &load_memberships(&root).unwrap());

        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert_eq!(warnings[0].campaign_dir, campaign);
        assert!(warnings[0].reason.contains("Bea's laptop") && warnings[0].reason.contains("2026-11-17T20:00:00Z"), "names who and when: {}", warnings[0].reason);
        assert_eq!(fs::read_to_string(backup(&campaign).join("bio.json")).unwrap(), "their bio", "the backup is not overwritten");
        assert_eq!(read_master(&backup(&campaign)).unwrap(), Some(theirs));
        assert_eq!(fs::read_to_string(root.join("bio.json")).unwrap(), "my bio", "the local copy is untouched");
        assert_eq!(load_memberships(&root).unwrap()[0].last_backup_counter, Some(1));
        // A membership that lost its counter (re-saved without it) still never overwrites another device's backup.
        let mut reset = load_memberships(&root).unwrap();
        reset[0].last_backup_counter = None;
        assert_eq!(mirror(&root, "Aldric", "char-1", &reset).len(), 1);
    }

    #[test]
    fn characters_backup_stand_in_never_writes() {
        let campaign = tempdir("stand-in-campaign");
        let root = character_root("stand-in");
        let mut stand_in = membership(&campaign, true);
        stand_in.stand_in = true;
        save_memberships(&root, std::slice::from_ref(&stand_in)).unwrap();

        let warnings = mirror(&root, "Aldric", "char-1", &load_memberships(&root).unwrap());

        assert!(warnings.is_empty(), "{warnings:?}");
        assert_eq!(fs::read_dir(&campaign).unwrap().count(), 0, "a stand-in writes no backup");
        assert_eq!(load_memberships(&root).unwrap(), vec![stand_in], "and its membership is unchanged");
        let old: Membership = serde_json::from_str(r#"{"campaignId":"c","campaignName":"N","campaignDir":"/x","active":true}"#).unwrap();
        assert!(!old.stand_in && old.last_backup_counter.is_none(), "an SD-41 campaigns.json still reads");
    }
}
