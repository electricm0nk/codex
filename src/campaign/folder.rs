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

/// Writes the `§15.1` starter into a campaign folder that has no `Party Resources.md` (a folder
/// made by hand). An existing file is never overwritten.
pub fn create_party_resources_starter(dir: &Path) -> Result<(), CampaignStoreError> {
    open(dir)?;
    let path = dir.join(PARTY_RESOURCES_FILE);
    if path.exists() {
        return Err(CampaignStoreError {
            message: format!("{} already exists; Codex never overwrites it", path.display()),
        });
    }
    write_atomic(&path, PARTY_RESOURCES_STARTER.as_bytes())
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

    #[test]
    fn party_resources_starter_has_the_two_tables() {
        // decisions.md §15.1: a heading, one sentence, the items table with one example row, then
        // a `## Party gold` table with its header only.
        let lines: Vec<&str> = PARTY_RESOURCES_STARTER.lines().collect();
        assert_eq!(lines[0], "# Party Resources");
        let items = lines.iter().position(|l| *l == "| Item | Qty | Value (gp) | Held by | Notes |").expect("items table");
        let gold = lines.iter().position(|l| *l == "## Party gold").expect("party gold heading");
        let gold_header = lines.iter().position(|l| *l == "| Date | Change | Balance | Note |").expect("gold table");
        assert!(items < gold && gold < gold_header);
        let sentences = lines[1..items].iter().filter(|l| !l.trim().is_empty()).count();
        assert_eq!(sentences, 1, "one sentence between the heading and the table");
        let item_rows = lines[items + 1..gold].iter().filter(|l| l.starts_with('|')).count();
        assert_eq!(item_rows, 2, "separator + one example row");
        let gold_rows = lines[gold_header + 1..].iter().filter(|l| l.starts_with('|')).count();
        assert_eq!(gold_rows, 1, "separator only: the party fills the gold ledger");
    }

    #[test]
    fn party_resources_is_never_parsed_by_any_reader() {
        // Q14: nothing parses the file. Every non-test line in src/campaign that names it must be a
        // write, never a read.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/campaign");
        let mut scanned = 0;
        for entry in fs::read_dir(&root).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|ext| ext != "rs") {
                continue;
            }
            let source = fs::read_to_string(&path).unwrap();
            let live = source.split("#[cfg(test)]").next().unwrap();
            for (n, line) in live.lines().enumerate() {
                if line.contains("PARTY_RESOURCES_FILE") || line.contains("Party Resources.md") {
                    let lower = line.to_lowercase();
                    assert!(
                        !lower.contains("read") && !lower.contains("open(") && !lower.contains("parse"),
                        "{}:{} reads Party Resources.md: {line}",
                        path.display(),
                        n + 1
                    );
                }
            }
            scanned += 1;
        }
        assert!(scanned >= 4, "scanned {scanned} files under {}", root.display());
    }

    #[test]
    fn campaign_folder_starter_file_is_created_only_when_missing() {
        // A folder made by hand (no Party Resources.md) gets the starter; an existing file is never overwritten.
        let dir = temp_dir("starter");
        create_layout(&dir, &sample()).expect("layout created");
        let path = dir.join(PARTY_RESOURCES_FILE);
        fs::remove_file(&path).unwrap();
        create_party_resources_starter(&dir).expect("starter created");
        assert_eq!(fs::read_to_string(&path).unwrap(), PARTY_RESOURCES_STARTER);
        fs::write(&path, "# Ours\n").unwrap();
        let err = create_party_resources_starter(&dir).expect_err("existing file kept");
        assert!(err.message.contains("already exists"), "{}", err.message);
        assert_eq!(fs::read_to_string(&path).unwrap(), "# Ours\n");
        let not_campaign = temp_dir("starter-not-campaign");
        fs::create_dir_all(&not_campaign).unwrap();
        assert!(create_party_resources_starter(&not_campaign).is_err(), "not a campaign folder");
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_dir_all(&not_campaign);
    }

    /// NR-18: every file a Codex writer puts in a campaign folder is plain UTF-8 text a Markdown
    /// editor can open — a `.md` whose frontmatter the tolerant parser reads (or that has none) and a
    /// `campaign.json` that is JSON. Images and `.codex/` are the only other files. With
    /// `CODEX_NR18_DIR` set, the folder the app wrote (E2.MC's run) is walked too.
    fn assert_plain_campaign_files(dir: &Path) -> usize {
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            for entry in fs::read_dir(dir).expect("readable folder") {
                let path = entry.expect("entry").path();
                if path.is_dir() {
                    walk(&path, out);
                } else {
                    out.push(path);
                }
            }
        }
        let mut files = Vec::new();
        walk(dir, &mut files);
        let mut checked = 0;
        for file in files {
            let rel = file.strip_prefix(dir).expect("inside").to_string_lossy().replace('\\', "/");
            if rel.starts_with(".codex/") || rel.starts_with("Characters/") {
                continue;
            }
            let ext = file.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
            if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif" | "webp") {
                continue;
            }
            let bytes = fs::read(&file).expect("readable file");
            let text = String::from_utf8(bytes).unwrap_or_else(|_| panic!("{rel} is not UTF-8 text"));
            assert!(!text.contains('\0'), "{rel} holds a NUL byte");
            match ext.as_str() {
                "md" => assert_ne!(crate::campaign::frontmatter::parse(&text), crate::campaign::frontmatter::Parsed::Unreadable, "{rel}: the tolerant parser cannot read its frontmatter"),
                "json" => {
                    serde_json::from_str::<serde_json::Value>(&text).unwrap_or_else(|err| panic!("{rel} is not JSON: {err}"));
                }
                other => panic!("{rel}: a file kind ({other:?}) no Codex writer should leave in a campaign folder"),
            }
            checked += 1;
        }
        checked
    }

    #[test]
    fn campaign_folder_every_written_file_is_plain_text() {
        let dir = temp_dir("nr18").join("The Void Between");
        create_layout(&dir, &sample()).expect("layout created");
        crate::campaign::adventure_log::new_entry(&dir, "The Void Between", "Dana", "Session 1", "2026-11-18").expect("log entry");
        crate::campaign::wiki::new_page(&dir, "Wiki/NPCs", "Ameiko Kaijitsu").expect("wiki page");
        let image = temp_dir("nr18-src").join("Sandpoint.png");
        fs::create_dir_all(image.parent().unwrap()).unwrap();
        fs::write(&image, [0x89u8, b'P', b'N', b'G', 0, 1, 2]).unwrap();
        assert_eq!(crate::campaign::maps::add_map(&dir, &image).expect("map added"), "Maps/Sandpoint.png");
        let pins = [crate::campaign::maps::Pin { x: 0.42, y: 0.17, label: "Ferry # 1".to_owned(), page: "[[Ameiko Kaijitsu]]".to_owned() }];
        crate::campaign::maps::set_pins(&dir.join("Maps/Sandpoint.md"), &pins).expect("pins set");
        // campaign.json, Party Resources.md, the log entry, the page, the map's notes.
        assert_eq!(assert_plain_campaign_files(&dir), 5);
        if let Ok(app_dir) = std::env::var("CODEX_NR18_DIR") {
            let checked = assert_plain_campaign_files(Path::new(&app_dir));
            println!("CODEX_NR18_DIR {app_dir}: {checked} files read as plain text");
        }
        fs::remove_dir_all(dir.parent().unwrap()).ok();
        fs::remove_dir_all(image.parent().unwrap()).ok();
    }

    /// The help screen's template tree (`campaignHelpModel.ts` `TEMPLATE_TREE`, E2.5) is a copy of
    /// what `create_layout` makes; this pins the copy to `LAYOUT_DIRS` and the two root files, so a
    /// layout change cannot leave the help screen describing a different folder.
    #[test]
    fn campaign_folder_help_template_matches_the_layout() {
        let model = Path::new(env!("CARGO_MANIFEST_DIR")).join("apps/desktop/src/campaign/campaignHelpModel.ts");
        let text = fs::read_to_string(&model).expect("the help model is readable");
        let start = text.find("export const TEMPLATE_TREE").expect("TEMPLATE_TREE is declared");
        let tree = &text[start..start + text[start..].find("];").expect("TEMPLATE_TREE ends")];
        let mut dirs: Vec<&str> = tree.split("dir('").skip(1).map(|rest| &rest[..rest.find('\'').unwrap()]).collect();
        dirs.sort_unstable();
        let mut layout = LAYOUT_DIRS.to_vec();
        layout.sort_unstable();
        assert_eq!(dirs, layout, "TEMPLATE_TREE's folders are LAYOUT_DIRS");
        let mut files: Vec<&str> = tree.split("file('").skip(1).map(|rest| &rest[..rest.find('\'').unwrap()]).collect();
        files.sort_unstable();
        assert_eq!(files, [".codex/nonce", "Party Resources.md", CAMPAIGN_JSON], "and its files are the ones create_layout writes");
    }
}
