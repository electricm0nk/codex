//! The Adventure Log (SD-42 `decisions.md §15.2`, Q15, Q16): one Markdown file per session per
//! author under `Adventure Log/`, its frontmatter read by `frontmatter::parse`. A file whose
//! frontmatter is missing or unreadable is still listed, by filename, and never rewritten (SD-o).

use std::fs;
use std::path::Path;

use serde::Serialize;

use super::frontmatter::{self, Frontmatter, Parsed, Value};
use super::{folder, local_store::sanitize_filename, CampaignStoreError};

pub const ADVENTURE_LOG_DIR: &str = "Adventure Log";

/// `session`: the real play date, one day or a range (`2026-11-14/2026-11-16`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateOrRange {
    pub start: String,
    pub end: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// `/`-separated, relative to the campaign folder.
    pub path: String,
    /// The frontmatter's `title`, else the filename stem.
    pub title: String,
    pub author: Option<String>,
    pub posted: Option<String>,
    pub session: Option<DateOrRange>,
    /// Free text, shown and never parsed (Q16).
    pub in_game_date: Option<String>,
    /// False when the file has no frontmatter or Codex cannot read it (SD-o).
    pub frontmatter_ok: bool,
}

/// `YYYY-MM-DD` at the start of `text` (a timestamp's date part), else `None`.
fn iso_date(text: &str) -> Option<String> {
    let date = text.trim().get(..10)?;
    let ok = date.bytes().enumerate().all(|(i, b)| if i == 4 || i == 7 { b == b'-' } else { b.is_ascii_digit() });
    ok.then(|| date.to_owned())
}

fn session(value: &Value) -> Option<DateOrRange> {
    let (start, end) = match value {
        Value::Scalar(text) => match text.split_once('/') {
            Some((a, b)) => (iso_date(a)?, Some(iso_date(b)?)),
            None => (iso_date(text)?, None),
        },
        Value::List(items) => match items.as_slice() {
            [one] => (iso_date(one)?, None),
            [a, b] => (iso_date(a)?, Some(iso_date(b)?)),
            _ => return None,
        },
    };
    Some(DateOrRange { start, end })
}

fn entry(rel: String, stem: &str, text: &str) -> Entry {
    let fm = match frontmatter::parse(text) {
        Parsed::Read(fm) => Some(fm),
        Parsed::Missing | Parsed::Unreadable => None,
    };
    let get = |key: &str| fm.as_ref().and_then(|fm: &Frontmatter| fm.str(key)).map(str::to_owned);
    Entry {
        path: rel,
        title: get("title").unwrap_or_else(|| stem.to_owned()),
        author: get("author"),
        posted: get("posted"),
        session: fm.as_ref().and_then(|fm| fm.get("session")).and_then(session),
        in_game_date: get("in_game_date"),
        frontmatter_ok: fm.is_some(),
    }
}

/// Every `.md` file directly under `<dir>/Adventure Log/`, newest session first (a range sorts by
/// its first day), then by path; a file with no readable `session` sorts after the dated ones.
// shortcut: top-level files only; read subfolders if players start nesting the log.
pub fn list(dir: &Path) -> Result<Vec<Entry>, CampaignStoreError> {
    let log = dir.join(ADVENTURE_LOG_DIR);
    let read_dir = match fs::read_dir(&log) {
        Ok(read_dir) => read_dir,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(io_error(&log, err)),
    };
    let mut entries = Vec::new();
    for item in read_dir.filter_map(Result::ok) {
        let path = item.path();
        let name = item.file_name().to_string_lossy().into_owned();
        let Some(stem) = name.strip_suffix(".md").or_else(|| name.strip_suffix(".MD")) else { continue };
        if !item.file_type().is_ok_and(|t| t.is_file()) {
            continue;
        }
        let bytes = fs::read(&path).map_err(|err| io_error(&path, err))?;
        entries.push(entry(format!("{ADVENTURE_LOG_DIR}/{name}"), stem, &String::from_utf8_lossy(&bytes)));
    }
    entries.sort_by(|a, b| {
        let start = |e: &Entry| e.session.as_ref().map(|s| s.start.clone());
        start(b).cmp(&start(a)).then_with(|| a.path.cmp(&b.path))
    });
    Ok(entries)
}

/// "+": writes `Adventure Log/<today> <author> - <title>.md` with `§15.2`'s frontmatter (session =
/// posted = `today`, the UI's local date) and returns its path inside the campaign. A taken name
/// is refused, never overwritten or suffixed (SD-f).
pub fn new_entry(dir: &Path, campaign: &str, author: &str, title: &str, today: &str) -> Result<String, CampaignStoreError> {
    let refuse = |message: String| CampaignStoreError { message };
    folder::open(dir)?;
    let (author, title) = (author.trim(), title.trim());
    if author.is_empty() || title.is_empty() {
        return Err(refuse("a log entry needs an author and a title".to_owned()));
    }
    let today = iso_date(today).filter(|d| d.len() == today.trim().len()).ok_or_else(|| refuse(format!("{today:?} is not a YYYY-MM-DD date")))?;
    let rel = format!("{ADVENTURE_LOG_DIR}/{}.md", sanitize_filename(&format!("{today} {author} - {title}")));
    let path = dir.join(&rel);
    fs::create_dir_all(dir.join(ADVENTURE_LOG_DIR)).map_err(|err| io_error(dir, err))?;
    // JSON strings are valid YAML double-quoted scalars (the migration writes the same way).
    let quote = |value: &str| serde_json::to_string(value).unwrap_or_default();
    let text = format!(
        "---\ncampaign: {}\nauthor: {}\nposted: {today}\nsession: {today}\nin_game_date:\ntitle: {}\n---\n",
        quote(campaign),
        quote(author),
        quote(title)
    );
    let mut file = match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(file) => file,
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(refuse(format!("{rel} already exists; pick another title")));
        }
        Err(err) => return Err(io_error(&path, err)),
    };
    use std::io::Write;
    file.write_all(text.as_bytes()).map_err(|err| io_error(&path, err))?;
    Ok(rel)
}

fn io_error(path: &Path, err: std::io::Error) -> CampaignStoreError {
    CampaignStoreError { message: format!("{}: {err}", path.display()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::{CampaignFolderSnapshot, FolderMember, CAMPAIGN_FOLDER_SCHEMA_VERSION};
    use std::path::PathBuf;

    fn campaign(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("codex-adventure-log-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let snapshot = CampaignFolderSnapshot {
            schema_version: CAMPAIGN_FOLDER_SCHEMA_VERSION,
            name: "Rise: of the Runelords".to_owned(),
            rule_set_id: "crb".to_owned(),
            rule_set_label: "Core".to_owned(),
            description: String::new(),
            members: vec![FolderMember { name: "Dana".to_owned(), added_at: "2026-11-17T00:00:00Z".to_owned() }],
            created_at: "2026-11-17T00:00:00Z".to_owned(),
            updated_at: "2026-11-17T00:00:00Z".to_owned(),
        };
        folder::create_layout(&dir, &snapshot).expect("layout");
        dir
    }

    fn put(dir: &Path, name: &str, text: &str) {
        fs::write(dir.join(ADVENTURE_LOG_DIR).join(name), text).unwrap();
    }

    #[test]
    fn adventure_log_new_entry_frontmatter() {
        let dir = campaign("new");
        let rel = new_entry(&dir, "Rise: of the Runelords", "Dana", "Session 1", "2026-11-17").unwrap();
        assert_eq!(rel, "Adventure Log/2026-11-17 Dana - Session 1.md");
        assert_eq!(
            fs::read_to_string(dir.join(&rel)).unwrap(),
            "---\ncampaign: \"Rise: of the Runelords\"\nauthor: \"Dana\"\nposted: 2026-11-17\nsession: 2026-11-17\nin_game_date:\ntitle: \"Session 1\"\n---\n"
        );
        let listed = list(&dir).unwrap();
        assert_eq!(listed.len(), 1);
        let e = &listed[0];
        assert_eq!((e.title.as_str(), e.author.as_deref(), e.posted.as_deref()), ("Session 1", Some("Dana"), Some("2026-11-17")));
        assert_eq!(e.session, Some(DateOrRange { start: "2026-11-17".into(), end: None }));
        assert_eq!((e.in_game_date.as_deref(), e.frontmatter_ok), (None, true));
        // SD-f: the same name again is refused and the first file is untouched.
        let err = new_entry(&dir, "x", "Dana", "Session 1", "2026-11-17").unwrap_err();
        assert!(err.message.contains("already exists"), "{}", err.message);
        assert!(new_entry(&dir, "x", " ", "t", "2026-11-17").is_err(), "author required");
        assert!(new_entry(&dir, "x", "Dana", "t", "17/11/2026").is_err(), "date checked");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn adventure_log_sorts_by_session_start_with_ranges() {
        let dir = campaign("sort");
        put(&dir, "a.md", "---\ntitle: Old\nsession: 2026-10-01\n---\n");
        put(&dir, "b.md", "---\ntitle: Weekend\nsession: 2026-11-14/2026-11-16\nin_game_date: 3 Rova 4707\n---\n");
        put(&dir, "c.md", "---\ntitle: Single\nsession: 2026-11-15\n---\n");
        put(&dir, "d.md", "---\ntitle: Listed range\nsession: [2026-11-20, '2026-11-21']\n---\n");
        put(&dir, "e.md", "---\ntitle: Undated\nsession: someday\n---\n");
        put(&dir, "notes.txt", "not a log entry");
        let listed = list(&dir).unwrap();
        let titles: Vec<&str> = listed.iter().map(|e| e.title.as_str()).collect();
        assert_eq!(titles, ["Listed range", "Single", "Weekend", "Old", "Undated"]);
        let weekend = &listed[2];
        assert_eq!(weekend.session, Some(DateOrRange { start: "2026-11-14".into(), end: Some("2026-11-16".into()) }));
        assert_eq!(weekend.in_game_date.as_deref(), Some("3 Rova 4707"));
        assert_eq!(listed[0].session.as_ref().and_then(|s| s.end.as_deref()), Some("2026-11-21"));
        assert_eq!((listed[4].session.as_ref(), listed[4].frontmatter_ok), (None, true));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn adventure_log_missing_frontmatter_is_listed_not_hidden() {
        let dir = campaign("missing");
        put(&dir, "2026-11-01 Gwen - Hand written.md", "Just notes, no frontmatter.\n");
        put(&dir, "broken.md", "---\ntitle: never closed\n");
        let before = fs::read(dir.join(ADVENTURE_LOG_DIR).join("broken.md")).unwrap();
        let listed = list(&dir).unwrap();
        let shown: Vec<(&str, bool)> = listed.iter().map(|e| (e.title.as_str(), e.frontmatter_ok)).collect();
        assert_eq!(shown, [("2026-11-01 Gwen - Hand written", false), ("broken", false)]);
        assert_eq!(fs::read(dir.join(ADVENTURE_LOG_DIR).join("broken.md")).unwrap(), before, "never rewritten");
        // A campaign with no Adventure Log folder lists nothing.
        fs::remove_dir_all(dir.join(ADVENTURE_LOG_DIR)).unwrap();
        assert!(list(&dir).unwrap().is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn adventure_log_hand_edited_frontmatter_survives() {
        // Obsidian-style edits: reordered keys, single quotes, comments, an extra list, CRLF, a
        // migrated file's JSON-quoted values — every field still reads.
        let dir = campaign("hand");
        put(
            &dir,
            "x.md",
            "---\r\n# edited in Obsidian\r\ntags:\r\n  - session\r\ntitle: 'The Ferry'\r\nsession: \"2026-11-02\"\r\nauthor: Gwen # me\r\nin_game_date: \"Night of 3 Rova\"\r\nextra: kept\r\n---\r\nBody\r\n",
        );
        let e = &list(&dir).unwrap()[0];
        assert_eq!(e.title, "The Ferry");
        assert_eq!(e.author.as_deref(), Some("Gwen"));
        assert_eq!(e.session.as_ref().map(|s| s.start.as_str()), Some("2026-11-02"));
        assert_eq!(e.in_game_date.as_deref(), Some("Night of 3 Rova"));
        assert!(e.frontmatter_ok);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn adventure_log_write_key_changes_only_that_key() {
        let dir = campaign("writekey");
        let rel = new_entry(&dir, "C", "Dana", "Session 2", "2026-11-17").unwrap();
        let path = dir.join(&rel);
        let mut text = fs::read_to_string(&path).unwrap();
        text.push_str("\n# Session 2\n\nWe crossed the river.\n");
        fs::write(&path, &text).unwrap();
        frontmatter::write_key(&path, "session", "2026-11-14/2026-11-16").unwrap();
        let after = fs::read_to_string(&path).unwrap();
        assert_eq!(after, text.replace("session: 2026-11-17", "session: 2026-11-14/2026-11-16"));
        let e = &list(&dir).unwrap()[0];
        assert_eq!(e.session, Some(DateOrRange { start: "2026-11-14".into(), end: Some("2026-11-16".into()) }));
        let _ = fs::remove_dir_all(&dir);
    }
}
