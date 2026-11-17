//! A campaign is a folder (SD-42 `decisions.md §11`): `campaign.json` (schema 2) at the root, the
//! Markdown files and directories a player can open in any editor, and `.codex/nonce` for the
//! conflict counter. Everything here is plain file IO; the desktop's `campaign_folder.rs` commands
//! are the only callers that touch a campaign folder.
//!
//! A schema-1 campaign (`.config/<name>.json`, the SD-21 layout `local_store` still reads) is never
//! opened here: migration upgrades it into this layout first.

use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use super::local_store::{CampaignStore, CAMPAIGN_JSON};
use super::{CampaignFolderSnapshot, CampaignStoreError, LoadedCampaignFolder, SaveOutcome};

pub const PARTY_RESOURCES_FILE: &str = "Party Resources.md";

/// Every directory `create_layout` makes, parents before children.
pub const LAYOUT_DIRS: [&str; 10] = [
    "Adventure Log",
    "Maps",
    "Wiki",
    "Wiki/NPCs",
    "Wiki/Places",
    "Wiki/Factions",
    "Wiki/Items",
    "Characters",
    "Attachments",
    ".codex",
];

/// The `§15.1` starter: a heading, one sentence, the shared-items table with one example row and
/// the party-gold table. Nothing in Codex parses it.
pub const PARTY_RESOURCES_STARTER: &str = "# Party Resources\n\
\n\
What the party holds in common. Edit this file freely; Codex never moves anything from it to a character.\n\
\n\
| Item | Qty | Value (gp) | Held by | Notes |\n\
|---|---|---|---|---|\n\
| Potion of cure light wounds | 2 | 100 | Party chest | example row; replace it |\n\
\n\
## Party gold\n\
\n\
| Date | Change | Balance | Note |\n\
|---|---|---|---|\n";

/// SD-r: how long the writer waits before its one retry when a sync client holds the file.
pub const WRITE_RETRY_DELAY: Duration = Duration::from_millis(200);

/// Creates a new campaign folder at `dir` with the `§11` layout and writes `campaign.json`
/// through the conflict-detecting save. `dir` must not exist or be empty (SD-f: a folder-name
/// collision is refused, never merged or auto-suffixed).
pub fn create_layout(dir: &Path, snapshot: &CampaignFolderSnapshot) -> Result<SaveOutcome, CampaignStoreError> {
    if fs::read_dir(dir).is_ok_and(|mut entries| entries.next().is_some()) {
        return Err(CampaignStoreError {
            message: format!("{} is not empty; pick a new folder for a new campaign", dir.display()),
        });
    }
    for sub in LAYOUT_DIRS {
        let path = dir.join(sub);
        fs::create_dir_all(&path).map_err(|err| io_error(&path, err))?;
    }
    write_atomic(&dir.join(PARTY_RESOURCES_FILE), PARTY_RESOURCES_STARTER.as_bytes())?;
    let mut snapshot = snapshot.clone();
    snapshot.schema_version = super::CAMPAIGN_FOLDER_SCHEMA_VERSION;
    CampaignStore::save_campaign_json_with_conflict_detection(&snapshot, dir, None)
}

/// Opens a schema-2 campaign folder. A folder with no `campaign.json` (a schema-1 `.config/`
/// campaign, or not a campaign at all) or a `campaign.json` of another schema is refused.
pub fn open(dir: &Path) -> Result<LoadedCampaignFolder, CampaignStoreError> {
    if !dir.join(CAMPAIGN_JSON).is_file() {
        let hint = if dir.join(".config").is_dir() {
            "it is an older campaign (.config/); migrate it first"
        } else {
            "it is not a campaign folder"
        };
        return Err(CampaignStoreError {
            message: format!("{} has no {CAMPAIGN_JSON}: {hint}", dir.display()),
        });
    }
    CampaignStore::load_campaign_json_with_nonce(dir)
}

/// Resolves `rel` (a `/`-separated path inside the campaign) against `dir`. Refuses an empty
/// path, an absolute path, any `..`, and a path that leaves `dir` through a symlink.
pub fn relative(dir: &Path, rel: &str) -> Result<PathBuf, CampaignStoreError> {
    let refuse = |why: &str| CampaignStoreError {
        message: format!("{rel:?} is not a path inside the campaign folder ({why})"),
    };
    if rel.trim().is_empty() {
        return Err(refuse("empty"));
    }
    let rel_path = Path::new(rel);
    for component in rel_path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {}
            Component::ParentDir => return Err(refuse("`..` is not allowed")),
            Component::RootDir | Component::Prefix(_) => return Err(refuse("absolute")),
        }
    }
    let joined = dir.join(rel_path);

    // The deepest part of the path that exists must still be inside the folder once symlinks
    // are followed.
    let root = dir.canonicalize().map_err(|err| io_error(dir, err))?;
    let mut probe = joined.as_path();
    loop {
        match probe.canonicalize() {
            Ok(real) if real.starts_with(&root) => break,
            Ok(_) => return Err(refuse("it leaves the folder through a link")),
            Err(_) => match probe.parent() {
                Some(parent) => probe = parent,
                None => break,
            },
        }
    }
    Ok(joined)
}

/// Writes `bytes` to a `<name>.tmp` sibling and renames it over `path`, so a crash never leaves
/// a half-written file. SD-r: a failed rename is retried once after 200 ms; a second failure is
/// reported and the new bytes stay in the temp file (the save is never lost).
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), CampaignStoreError> {
    write_atomic_with(path, bytes, |from, to| fs::rename(from, to), WRITE_RETRY_DELAY)
}

pub(crate) fn write_atomic_with(
    path: &Path,
    bytes: &[u8],
    mut rename: impl FnMut(&Path, &Path) -> std::io::Result<()>,
    retry_delay: Duration,
) -> Result<(), CampaignStoreError> {
    let mut tmp_name = path.file_name().map(|n| n.to_os_string()).unwrap_or_default();
    tmp_name.push(".tmp");
    let tmp_path = path.with_file_name(tmp_name);
    fs::write(&tmp_path, bytes).map_err(|err| io_error(&tmp_path, err))?;
    if rename(&tmp_path, path).is_ok() {
        return Ok(());
    }
    std::thread::sleep(retry_delay);
    rename(&tmp_path, path).map_err(|err| CampaignStoreError {
        message: format!(
            "{} could not be replaced after one retry ({err}); your text is kept in {}",
            path.display(),
            tmp_path.display()
        ),
    })
}

fn io_error(path: &Path, err: std::io::Error) -> CampaignStoreError {
    CampaignStoreError {
        message: format!("{}: {err}", path.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::{CampaignFolderSnapshot, FolderMember, CAMPAIGN_FOLDER_SCHEMA_VERSION};
    use std::cell::Cell;
    use std::fs;
    use std::path::PathBuf;

    fn temp_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("codex-campaign-folder-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    fn sample() -> CampaignFolderSnapshot {
        CampaignFolderSnapshot {
            schema_version: CAMPAIGN_FOLDER_SCHEMA_VERSION,
            name: "The Void Between".to_owned(),
            rule_set_id: "crb".to_owned(),
            rule_set_label: "Core Rulebook".to_owned(),
            description: "A test campaign.".to_owned(),
            members: vec![FolderMember { name: "Dana".to_owned(), added_at: "2026-11-17T00:00:00Z".to_owned() }],
            created_at: "2026-11-17T00:00:00Z".to_owned(),
            updated_at: "2026-11-17T00:00:00Z".to_owned(),
        }
    }

    #[test]
    fn campaign_folder_layout_is_created() {
        let dir = temp_dir("layout").join("The Void Between");
        let outcome = create_layout(&dir, &sample()).expect("layout created");
        assert_eq!(outcome.nonce, 1);
        assert!(dir.join("campaign.json").is_file());
        assert!(dir.join(".codex/nonce").is_file());
        for sub in LAYOUT_DIRS {
            assert!(dir.join(sub).is_dir(), "{sub} missing");
        }
        let starter = fs::read_to_string(dir.join(PARTY_RESOURCES_FILE)).expect("starter written");
        assert!(starter.contains("| Item | Qty | Value (gp) | Held by | Notes |"));
        assert!(starter.contains("## Party gold"));
        assert!(starter.contains("| Date | Change | Balance | Note |"));
        // SD-f: a second create into a non-empty folder is refused, never merged.
        let err = create_layout(&dir, &sample()).expect_err("non-empty folder refused");
        assert!(err.message.contains("not empty"), "{}", err.message);
        let _ = fs::remove_dir_all(dir.parent().unwrap());
    }

    #[test]
    fn campaign_folder_json_schema_2_round_trips() {
        let dir = temp_dir("roundtrip");
        create_layout(&dir, &sample()).expect("layout created");
        let raw: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(dir.join("campaign.json")).unwrap()).unwrap();
        assert_eq!(raw["schemaVersion"], 2);
        assert_eq!(raw["members"][0]["name"], "Dana");
        assert_eq!(raw["members"][0]["addedAt"], "2026-11-17T00:00:00Z");
        assert!(raw.get("assets").is_none());
        assert!(raw.get("partyCharacterIds").is_none());
        let loaded = open(&dir).expect("opens");
        assert_eq!(loaded.snapshot, sample());
        assert_eq!(loaded.nonce, 1);
        // A save from a stale read keeps the other writer's campaign.json under .codex/conflicts/.
        let mut edited = sample();
        edited.description = "edited".to_owned();
        let saved = CampaignStore::save_campaign_json_with_conflict_detection(&edited, &dir, Some(1)).unwrap();
        assert_eq!((saved.nonce, saved.conflict_dir.is_none()), (2, true));
        let stale = CampaignStore::save_campaign_json_with_conflict_detection(&sample(), &dir, Some(1)).unwrap();
        let kept = stale.conflict_dir.expect("conflict detected");
        assert!(kept.starts_with(dir.join(".codex")));
        assert!(fs::read_to_string(kept.join("campaign.json")).unwrap().contains("edited"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn campaign_folder_refuses_dotdot_and_absolute() {
        let dir = temp_dir("relative");
        create_layout(&dir, &sample()).expect("layout created");
        assert_eq!(relative(&dir, "Wiki/NPCs/Bob.md").unwrap(), dir.join("Wiki/NPCs/Bob.md"));
        for bad in ["../outside.md", "Wiki/../../outside.md", "", "/etc/passwd"] {
            assert!(relative(&dir, bad).is_err(), "{bad:?} accepted");
        }
        #[cfg(unix)]
        {
            let outside = temp_dir("relative-outside");
            fs::create_dir_all(&outside).unwrap();
            std::os::unix::fs::symlink(&outside, dir.join("Wiki/escape")).unwrap();
            assert!(relative(&dir, "Wiki/escape/x.md").is_err(), "symlink escape accepted");
            let _ = fs::remove_dir_all(&outside);
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn campaign_folder_write_is_atomic_and_retries_once() {
        let dir = temp_dir("atomic");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("note.md");
        write_atomic(&path, b"first").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"first");
        assert!(!dir.join("note.md.tmp").exists(), "temp file left behind");

        // SD-r: one failed rename (a sync client holding the file) is retried once.
        let calls = Cell::new(0);
        let flaky = |from: &std::path::Path, to: &std::path::Path| {
            calls.set(calls.get() + 1);
            if calls.get() == 1 {
                Err(std::io::Error::other("busy"))
            } else {
                fs::rename(from, to)
            }
        };
        write_atomic_with(&path, b"second", flaky, std::time::Duration::ZERO).unwrap();
        assert_eq!((calls.get(), fs::read(&path).unwrap()), (2, b"second".to_vec()));

        // Two failures: reported, the old file untouched, the new bytes kept in the temp file.
        let always = |_: &std::path::Path, _: &std::path::Path| Err(std::io::Error::other("busy"));
        let err = write_atomic_with(&path, b"third", always, std::time::Duration::ZERO).unwrap_err();
        assert!(err.message.contains("note.md.tmp"), "{}", err.message);
        assert_eq!(fs::read(&path).unwrap(), b"second");
        assert_eq!(fs::read(dir.join("note.md.tmp")).unwrap(), b"third");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn campaign_folder_open_reads_schema_2_only() {
        // A schema-1 folder (.config/<name>.json, the SD-21 layout) is migration's job, not open's.
        let old = temp_dir("schema1");
        let mut legacy = crate::campaign::CampaignSnapshot {
            schema_version: crate::campaign::CURRENT_CAMPAIGN_SCHEMA_VERSION,
            id: "c".to_owned(),
            name: "Old".to_owned(),
            rule_set_id: "crb".to_owned(),
            rule_set_label: "Core".to_owned(),
            description: String::new(),
            members: vec![],
            party_character_ids: vec![],
            created_at: "t".to_owned(),
            updated_at: "t".to_owned(),
            assets: Default::default(),
        };
        legacy.description = "schema 1".to_owned();
        CampaignStore::save(&legacy, &old).unwrap();
        let err = open(&old).expect_err("schema 1 refused");
        assert!(err.message.contains("migrat"), "{}", err.message);

        // A campaign.json that says schema 1 is refused too.
        let wrong = temp_dir("schema-wrong");
        fs::create_dir_all(&wrong).unwrap();
        let mut one = serde_json::to_value(sample()).unwrap();
        one["schemaVersion"] = serde_json::json!(1);
        fs::write(wrong.join("campaign.json"), one.to_string()).unwrap();
        assert!(open(&wrong).is_err());
        let _ = fs::remove_dir_all(&old);
        let _ = fs::remove_dir_all(&wrong);
    }
}
