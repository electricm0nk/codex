//! One-way write-through of a saved character's folder into each campaign folder it belongs to
//! (SD-41 `decisions.md §29`). The app is master; the campaign copy is never read back.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::campaign::local_store::sanitize_filename;
use crate::saved_character::SavedCharacterStoreError;

/// The character's campaign memberships, kept in its own root; never copied to a campaign.
pub const MEMBERSHIPS_FILE: &str = "campaigns.json";

/// One campaign the character belongs to. Inactive = removed: the copy stays, saves stop going there.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub campaign_id: String,
    pub campaign_name: String,
    pub campaign_dir: PathBuf,
    pub active: bool,
}

/// A campaign copy that could not be written. The local save has already succeeded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorWarning {
    pub campaign_dir: PathBuf,
    pub reason: String,
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
    let tmp = root.join(format!("{MEMBERSHIPS_FILE}.tmp"));
    fs::write(&tmp, text)
        .and_then(|()| fs::rename(&tmp, &path))
        .map_err(|err| store_error(format!("could not write {}: {err}", path.display())))
}

/// The folder-name part from the display label: `sanitize_filename`, lower-cased, spaces → `-`.
pub fn character_slug(label: &str) -> String {
    sanitize_filename(label).to_lowercase().replace(' ', "-")
}

/// Copy every file under `root` except `campaigns.json` to
/// `<campaign_dir>/characters/<slug>-<character_id>/` for each active membership, replacing the old
/// copy whole. Failures are warnings per campaign; the loop always continues.
pub fn mirror(root: &Path, label: &str, character_id: &str, memberships: &[Membership]) -> Vec<MirrorWarning> {
    let folder = format!("{}-{}", character_slug(label), sanitize_filename(character_id));
    memberships
        .iter()
        .filter(|membership| membership.active)
        .filter_map(|membership| {
            mirror_one(root, &membership.campaign_dir, &folder)
                .err()
                .map(|reason| MirrorWarning { campaign_dir: membership.campaign_dir.clone(), reason })
        })
        .collect()
}

fn mirror_one(root: &Path, campaign_dir: &Path, folder: &str) -> Result<(), String> {
    // Never create a missing campaign dir: it means the Drive folder is not there.
    if !campaign_dir.is_dir() {
        return Err("the campaign folder does not exist or is not a folder".to_owned());
    }
    let characters = campaign_dir.join("characters");
    let target = characters.join(folder);
    let tmp = characters.join(format!("{folder}.tmp-{}", std::process::id()));
    let result = (|| -> io::Result<()> {
        fs::create_dir_all(&characters)?;
        if tmp.exists() {
            fs::remove_dir_all(&tmp)?;
        }
        copy_tree(root, &tmp, true)?;
        if target.exists() {
            fs::remove_dir_all(&target)?;
        }
        fs::rename(&tmp, &target)
    })();
    result.map_err(|err| {
        let _ = fs::remove_dir_all(&tmp);
        err.to_string()
    })
}

fn copy_tree(from: &Path, to: &Path, top: bool) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let name = entry.file_name();
        if top && name == MEMBERSHIPS_FILE {
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
        }
    }

    /// A character root as the app keeps it: the two store files plus sidecars and a nested file.
    fn character_root(label: &str) -> PathBuf {
        let root = tempdir(label);
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

        let a = campaign.join("characters").join("aldric-char-aaa");
        let b = campaign.join("characters").join("aldric-char-bbb");
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
        let copy = campaign.join("characters").join("aldric-char-1");
        assert_eq!(sha256_tree(&copy), expected, "every file but campaigns.json, byte-identical");
        assert_eq!(expected.len(), 6);
        let leftovers: Vec<_> = fs::read_dir(campaign.join("characters")).unwrap().map(|e| e.unwrap().file_name()).collect();
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
        assert_eq!(sha256_tree(&root), before, "local files untouched");
        assert!(reachable.join("characters").join("aldric-char-1").join("bio.json").is_file(), "the loop continues past the warning");
    }

    #[test]
    fn campaign_mirror_second_save_replaces_the_folder() {
        let campaign = tempdir("replace-campaign");
        let root = tempdir("replace-root");
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sd14/pf1_human_fighter_level1_saved_character");
        let mut envelope = SavedCharacterStore::load(&fixture).expect("fixture loads");
        envelope.character_id = "char-replace".to_owned();
        envelope.display_label = "Aldric".to_owned();
        save_memberships(&root, &[membership(&campaign, true)]).unwrap();
        fs::write(root.join("custom.json"), "[]").unwrap();

        let warnings = SavedCharacterStore::save_with_warnings(&envelope, &root).expect("first save");
        assert!(warnings.is_empty(), "{warnings:?}");
        let copy = campaign.join("characters").join("aldric-char-replace");
        assert!(copy.join("custom.json").is_file());
        assert_eq!(fs::read(copy.join("envelope.txt")).unwrap(), fs::read(root.join("envelope.txt")).unwrap());

        fs::remove_file(root.join("custom.json")).unwrap();
        envelope.revision_id = "char-replace.rev.2".to_owned();
        SavedCharacterStore::save(&envelope, &root).expect("second save");

        assert!(!copy.join("custom.json").exists(), "a file deleted locally is gone from the copy");
        assert!(fs::read_to_string(copy.join("envelope.txt")).unwrap().contains("char-replace.rev.2"));
        assert!(!copy.join(MEMBERSHIPS_FILE).exists());
    }
}
