//! Wiki (SD-42 `decisions.md §15.4`, Q18): free folders under `Wiki/` (starters `NPCs/`, `Places/`,
//! `Factions/`, `Items/`, made by `folder::create_layout`), a page per `.md` file, `[[links]]`
//! resolved by `title` then filename stem across the whole campaign folder, backlinks from a scan
//! of every `.md` for `[[…]]`, and search by title then text. Plain file IO; the desktop's
//! `campaign_folder.rs` commands are the callers.

use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;

use super::frontmatter::{self, Parsed, Value};
use super::{folder, CampaignStoreError};

pub const WIKI_DIR: &str = "Wiki";

/// One wiki page as the Wiki tab lists it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    /// `Wiki/…/<name>.md`, `/`-separated, relative to the campaign folder.
    pub path: String,
    /// The frontmatter's `title`, else the filename stem.
    pub title: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub tags: Vec<String>,
    /// SD-o: a `---` block Codex cannot read; the page is listed by its stem and never rewritten.
    pub frontmatter_unread: bool,
}

/// One search result: a title hit (`title_match`) or a text hit, with the text around the match.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub path: String,
    pub title: String,
    pub title_match: bool,
    /// The line(s) around the first text match, `…` where cut; empty for a title hit whose text has no match.
    pub snippet: String,
}

fn err(message: String) -> CampaignStoreError {
    CampaignStoreError { message }
}

fn io_error(path: &Path, cause: std::io::Error) -> CampaignStoreError {
    err(format!("{}: {cause}", path.display()))
}

/// A folder or page name typed by a player: one path segment a file browser and a sync client accept.
fn check_name(name: &str) -> Result<&str, CampaignStoreError> {
    let name = name.trim();
    let bad = name.is_empty() || name.starts_with('.') || name.chars().any(|c| c.is_control() || "/\\:*?\"<>|".contains(c));
    if bad {
        return Err(err(format!("{name:?} is not a usable name (no / \\ : * ? \" < > |, not empty, not starting with a dot)")));
    }
    Ok(name)
}

/// `rel` inside the campaign and inside `Wiki/` (`Wiki` itself only when `allow_root`).
fn wiki_path(dir: &Path, rel: &str, allow_root: bool) -> Result<PathBuf, CampaignStoreError> {
    let path = folder::relative(dir, rel)?;
    let mut parts = Path::new(rel).components().filter(|c| !matches!(c, std::path::Component::CurDir));
    let first = parts.next().map(|c| c.as_os_str().to_string_lossy().into_owned());
    let deeper = parts.next().is_some();
    if first.as_deref() != Some(WIKI_DIR) || !(deeper || allow_root) {
        return Err(err(format!("{rel:?} is not a folder inside {WIKI_DIR}/")));
    }
    Ok(path)
}

fn join(prefix: &str, name: &str) -> String {
    if prefix.is_empty() { name.to_owned() } else { format!("{prefix}/{name}") }
}

/// Every entry under `dir` (relative `prefix`), depth first, sorted case-insensitively; hidden
/// entries (`.codex/`, `.obsidian/`) skipped; symlinks listed as files, never walked.
fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, bool)>) -> Result<(), CampaignStoreError> {
    let read = match fs::read_dir(dir) {
        Ok(read) => read,
        Err(cause) if cause.kind() == ErrorKind::NotFound => return Ok(()),
        Err(cause) => return Err(io_error(dir, cause)),
    };
    let mut entries: Vec<_> = read.filter_map(Result::ok).filter(|e| !e.file_name().to_string_lossy().starts_with('.')).collect();
    entries.sort_by_key(|e| e.file_name().to_string_lossy().to_lowercase());
    for entry in entries {
        let rel = join(prefix, &entry.file_name().to_string_lossy());
        let is_dir = entry.file_type().is_ok_and(|t| t.is_dir());
        out.push((rel.clone(), is_dir));
        if is_dir {
            walk(&entry.path(), &rel, out)?;
        }
    }
    Ok(())
}

fn entries(dir: &Path, rel: &str) -> Result<Vec<(String, bool)>, CampaignStoreError> {
    let mut out = Vec::new();
    walk(&if rel.is_empty() { dir.to_path_buf() } else { dir.join(rel) }, rel, &mut out)?;
    Ok(out)
}

/// Every folder under `Wiki/` (not `Wiki` itself), sorted by path, case-insensitive.
pub fn folders(dir: &Path) -> Result<Vec<String>, CampaignStoreError> {
    let mut out: Vec<String> = entries(dir, WIKI_DIR)?.into_iter().filter(|(_, d)| *d).map(|(p, _)| p).collect();
    out.sort_by_key(|p| p.to_lowercase());
    Ok(out)
}

/// Makes `<parent>/<name>` (parent `Wiki` or a folder under it); returns its path. A taken name is refused (SD-f).
pub fn add_folder(dir: &Path, parent: &str, name: &str) -> Result<String, CampaignStoreError> {
    folder::open(dir)?;
    let name = check_name(name)?;
    let path = wiki_path(dir, parent, true)?.join(name);
    let rel = format!("{}/{name}", parent.trim_end_matches('/'));
    fs::create_dir_all(path.parent().unwrap_or(dir)).map_err(|cause| io_error(dir, cause))?;
    match fs::create_dir(&path) {
        Ok(()) => Ok(rel),
        Err(cause) if cause.kind() == ErrorKind::AlreadyExists => Err(err(format!("{rel} already exists"))),
        Err(cause) => Err(io_error(&path, cause)),
    }
}

/// Renames a folder under `Wiki/` in place (its pages move with it; links by title or stem still
/// resolve). A taken name is refused (SD-f); a case-only rename is allowed.
pub fn rename_folder(dir: &Path, from: &str, to_name: &str) -> Result<String, CampaignStoreError> {
    folder::open(dir)?;
    let name = check_name(to_name)?;
    let source = wiki_path(dir, from, false)?;
    if !source.is_dir() {
        return Err(err(format!("{from} is not a folder")));
    }
    let parent = from.trim_end_matches('/').rsplit_once('/').map_or(WIKI_DIR, |(p, _)| p);
    let rel = format!("{parent}/{name}");
    let target = source.with_file_name(name);
    if target.exists() && rel.to_lowercase() != from.trim_end_matches('/').to_lowercase() {
        return Err(err(format!("{rel} already exists")));
    }
    fs::rename(&source, &target).map_err(|cause| io_error(&source, cause))?;
    Ok(rel)
}

/// Deletes a folder under `Wiki/` only when it holds no file (empty subfolders go with it). A folder
/// holding files is not touched: its files are returned for the screen to list. Empty = deleted.
pub fn delete_folder(dir: &Path, rel: &str) -> Result<Vec<String>, CampaignStoreError> {
    folder::open(dir)?;
    let path = wiki_path(dir, rel, false)?;
    if !path.is_dir() {
        return Err(err(format!("{rel} is not a folder")));
    }
    let rel = rel.trim_end_matches('/');
    let mut all = Vec::new();
    walk(&path, rel, &mut all)?;
    let files: Vec<String> = all.iter().filter(|(_, is_dir)| !is_dir).map(|(p, _)| p.clone()).collect();
    if !files.is_empty() {
        return Ok(files);
    }
    // Deepest first with remove_dir, which fails rather than delete a file it was not shown (a
    // hidden one, or one that appeared meanwhile).
    let mut dirs: Vec<&String> = all.iter().map(|(p, _)| p).collect();
    dirs.sort_by_key(|p| std::cmp::Reverse(p.len()));
    for sub in dirs.into_iter().map(|p| dir.join(p)).chain([path.clone()]) {
        fs::remove_dir(&sub).map_err(|cause| io_error(&sub, cause))?;
    }
    Ok(Vec::new())
}

/// A Markdown file of the campaign, read once for resolution, backlinks and search.
struct Doc {
    path: String,
    stem: String,
    /// The frontmatter's `title`, if it has one.
    title: Option<String>,
    kind: Option<String>,
    tags: Vec<String>,
    frontmatter_unread: bool,
    text: String,
}

impl Doc {
    fn shown_title(&self) -> &str {
        self.title.as_deref().unwrap_or(&self.stem)
    }
}

fn is_md(path: &str) -> bool {
    path.to_lowercase().ends_with(".md")
}

/// Every `.md` under `rel` (the whole campaign when empty), read.
fn docs(dir: &Path, rel: &str) -> Result<Vec<Doc>, CampaignStoreError> {
    let mut out = Vec::new();
    for (path, is_dir) in entries(dir, rel)? {
        if is_dir || !is_md(&path) {
            continue;
        }
        let file = dir.join(&path);
        let text = String::from_utf8_lossy(&fs::read(&file).map_err(|cause| io_error(&file, cause))?).into_owned();
        let (mut title, mut kind, mut tags) = (None, None, Vec::new());
        let parsed = frontmatter::parse(&text);
        let frontmatter_unread = matches!(parsed, Parsed::Unreadable);
        if let Parsed::Read(fm) = parsed {
            title = fm.str("title").map(str::to_owned);
            kind = fm.str("type").map(str::to_owned);
            tags = match fm.get("tags") {
                Some(Value::List(items)) => items.iter().filter(|t| !t.is_empty()).cloned().collect(),
                Some(Value::Scalar(tag)) if !tag.is_empty() => vec![tag.clone()],
                _ => Vec::new(),
            };
        }
        let name = path.rsplit('/').next().unwrap_or(&path);
        let stem = name[..name.len() - 3].to_owned();
        out.push(Doc { path, stem, title, kind, tags, frontmatter_unread, text });
    }
    Ok(out)
}

/// Every page under `Wiki/`, sorted by path, case-insensitive.
pub fn pages(dir: &Path) -> Result<Vec<Page>, CampaignStoreError> {
    let mut out: Vec<Page> = docs(dir, WIKI_DIR)?
        .into_iter()
        .map(|d| Page {
            title: d.shown_title().to_owned(),
            path: d.path,
            kind: d.kind,
            tags: d.tags,
            frontmatter_unread: d.frontmatter_unread,
        })
        .collect();
    out.sort_by_key(|p| p.path.to_lowercase());
    Ok(out)
}

/// `Places` → `Place`, `Cities` → `City`, `NPCs` → `NPC`: the `type` "+" writes for a folder's page.
fn singular(folder: &str) -> String {
    if let Some(stem) = folder.strip_suffix("ies").filter(|s| !s.is_empty()) {
        return format!("{stem}y");
    }
    match folder.strip_suffix('s') {
        Some(stem) if !stem.is_empty() && !stem.ends_with('s') => stem.to_owned(),
        _ => folder.to_owned(),
    }
}

/// "+" in a folder: writes `<folder>/<name>.md` with `§15.4`'s frontmatter (title, type = the
/// folder's name made singular, empty tags) and a heading; returns its path. Never overwrites (SD-f).
pub fn new_page(dir: &Path, folder_rel: &str, name: &str) -> Result<String, CampaignStoreError> {
    folder::open(dir)?;
    let name = check_name(name)?;
    let parent = wiki_path(dir, folder_rel, true)?;
    let folder_rel = folder_rel.trim_end_matches('/');
    let rel = format!("{folder_rel}/{name}.md");
    let path = parent.join(format!("{name}.md"));
    fs::create_dir_all(&parent).map_err(|cause| io_error(&parent, cause))?;
    // JSON strings are valid YAML double-quoted scalars (the Adventure Log writes the same way).
    let quote = |value: &str| serde_json::to_string(value).unwrap_or_default();
    let kind = singular(folder_rel.rsplit('/').next().unwrap_or(folder_rel));
    let text = format!("---\ntitle: {}\ntype: {}\ntags: []\n---\n# {name}\n", quote(name), quote(&kind));
    let mut file = match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(file) => file,
        Err(cause) if cause.kind() == ErrorKind::AlreadyExists => return Err(err(format!("{rel} already exists"))),
        Err(cause) => return Err(io_error(&path, cause)),
    };
    file.write_all(text.as_bytes()).map_err(|cause| io_error(&path, cause))?;
    Ok(rel)
}

/// The page name a link names: `[[Page|alias]]`, `[[Page#heading]]`, a table's `[[Page\|alias]]`
/// → `Page`; the brackets are optional.
pub fn link_name(link: &str) -> &str {
    let link = link.trim();
    let inner = link.strip_prefix("[[").and_then(|l| l.strip_suffix("]]")).unwrap_or(link);
    inner.split(['|', '#']).next().unwrap_or_default().trim_end_matches('\\').trim()
}

/// `title` first, then the filename stem, then a path ending (`[[Places/Inn]]`); case-insensitive.
/// Ties go to the first path in sorted order.
fn resolve_in<'a>(docs: &'a [Doc], link: &str) -> Option<&'a Doc> {
    let wanted = link_name(link).to_lowercase();
    if wanted.is_empty() {
        return None;
    }
    let path_tail = format!("/{wanted}");
    docs.iter()
        .find(|d| d.title.as_ref().is_some_and(|t| t.to_lowercase() == wanted))
        .or_else(|| docs.iter().find(|d| d.stem.to_lowercase() == wanted))
        .or_else(|| {
            docs.iter().find(|d| {
                let bare = d.path[..d.path.len() - 3].to_lowercase();
                bare == wanted || bare.ends_with(&path_tail)
            })
        })
}

/// The file a link opens, across every `.md` in the campaign (Obsidian's rule with `title` first), or `None`.
pub fn resolve_link(dir: &Path, link: &str) -> Result<Option<String>, CampaignStoreError> {
    Ok(resolve_in(&docs(dir, "")?, link).map(|d| d.path.clone()))
}

/// Every `[[…]]` on one line of `text` (an unclosed `[[` is not a link).
fn links_in(text: &str) -> impl Iterator<Item = &str> {
    let mut rest = text;
    std::iter::from_fn(move || loop {
        let open = rest.find("[[")?;
        let after = &rest[open + 2..];
        let close = after.find("]]");
        let line_end = after.find('\n').unwrap_or(after.len());
        match close {
            Some(close) if close < line_end => {
                rest = &after[close + 2..];
                return Some(&after[..close]);
            }
            _ => rest = &after[line_end.min(after.len())..],
        }
    })
}

/// The link index: for each `.md` of the campaign, the pages its `[[links]]` resolve to.
fn link_index(docs: &[Doc]) -> Vec<(&str, Vec<&str>)> {
    docs.iter()
        .map(|d| (d.path.as_str(), links_in(&d.text).filter_map(|l| resolve_in(docs, l)).map(|t| t.path.as_str()).collect()))
        .collect()
}

/// "Pages that link here": every other `.md` of the campaign with a `[[link]]` that resolves to `page`.
pub fn backlinks(dir: &Path, page: &str) -> Result<Vec<String>, CampaignStoreError> {
    let all = docs(dir, "")?;
    let mut out: Vec<String> = link_index(&all)
        .into_iter()
        .filter(|(from, targets)| *from != page && targets.contains(&page))
        .map(|(from, _)| from.to_owned())
        .collect();
    out.sort_by_key(|p| p.to_lowercase());
    Ok(out)
}

/// The char index of the first case-insensitive match of `wanted` (lowercase chars) in `text`.
fn find_folded(text: &[char], wanted: &[char]) -> Option<usize> {
    (0..text.len()).find(|&i| {
        let mut folded = text[i..].iter().flat_map(|c| c.to_lowercase());
        wanted.iter().all(|w| folded.next() == Some(*w))
    })
}

const BEFORE: usize = 40;
const AFTER: usize = 60;

fn snippet(text: &[char], at: usize, len: usize) -> String {
    let start = at.saturating_sub(BEFORE);
    let end = (at + len + AFTER).min(text.len());
    let body: String = text[start..end].iter().map(|&c| if c == '\n' || c == '\r' { ' ' } else { c }).collect();
    format!("{}{}{}", if start > 0 { "…" } else { "" }, body.trim(), if end < text.len() && !text[end..].iter().all(|c| c.is_whitespace()) { "…" } else { "" })
}

/// Search every `.md` in the campaign: title hits (the `title` or the filename stem) first, then
/// text hits, each group by path; case-insensitive. An empty query finds nothing.
pub fn search(dir: &Path, query: &str) -> Result<Vec<Hit>, CampaignStoreError> {
    let wanted: Vec<char> = query.trim().chars().flat_map(char::to_lowercase).collect();
    if wanted.is_empty() {
        return Ok(Vec::new());
    }
    let mut titles = Vec::new();
    let mut texts = Vec::new();
    for doc in docs(dir, "")? {
        let chars: Vec<char> = doc.text.chars().collect();
        let in_text = find_folded(&chars, &wanted);
        let title_match = [doc.shown_title(), &doc.stem].iter().any(|t| find_folded(&t.chars().collect::<Vec<_>>(), &wanted).is_some());
        if !title_match && in_text.is_none() {
            continue;
        }
        let snippet = in_text.map(|at| snippet(&chars, at, wanted.len())).unwrap_or_default();
        let hit = Hit { title: doc.shown_title().to_owned(), path: doc.path, title_match, snippet };
        if title_match { titles.push(hit) } else { texts.push(hit) }
    }
    titles.sort_by_key(|h| h.path.to_lowercase());
    texts.sort_by_key(|h| h.path.to_lowercase());
    titles.extend(texts);
    Ok(titles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::{folder, CampaignFolderSnapshot, FolderMember, CAMPAIGN_FOLDER_SCHEMA_VERSION};
    use std::fs;
    use std::path::{Path, PathBuf};

    fn campaign(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("codex-wiki-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let snapshot = CampaignFolderSnapshot {
            schema_version: CAMPAIGN_FOLDER_SCHEMA_VERSION,
            name: "Wiki test".to_owned(),
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

    fn put(dir: &Path, rel: &str, text: &str) {
        let path = dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn wiki_starter_folders() {
        let dir = campaign("starters");
        assert_eq!(folders(&dir).unwrap(), ["Wiki/Factions", "Wiki/Items", "Wiki/NPCs", "Wiki/Places"]);
        assert_eq!(add_folder(&dir, "Wiki/Places", "Towns").unwrap(), "Wiki/Places/Towns");
        assert_eq!(add_folder(&dir, "Wiki", "Deities").unwrap(), "Wiki/Deities");
        assert_eq!(folders(&dir).unwrap(), ["Wiki/Deities", "Wiki/Factions", "Wiki/Items", "Wiki/NPCs", "Wiki/Places", "Wiki/Places/Towns"]);
        // SD-f: a taken name is refused; a name with a path in it, and a folder outside Wiki/, too.
        assert!(add_folder(&dir, "Wiki", "NPCs").unwrap_err().message.contains("already"));
        for (parent, name) in [("Wiki", "a/b"), ("Wiki", ".hidden"), ("Wiki", "  "), ("Maps", "x"), ("Wiki/../Maps", "x")] {
            assert!(add_folder(&dir, parent, name).is_err(), "{parent} {name}");
        }
        fs::remove_dir_all(dir.join(WIKI_DIR)).unwrap();
        assert!(folders(&dir).unwrap().is_empty(), "no Wiki folder lists nothing");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn wiki_new_page_frontmatter() {
        let dir = campaign("newpage");
        assert_eq!(new_page(&dir, "Wiki/NPCs", "Ameiko Kaijitsu").unwrap(), "Wiki/NPCs/Ameiko Kaijitsu.md");
        assert_eq!(
            fs::read_to_string(dir.join("Wiki/NPCs/Ameiko Kaijitsu.md")).unwrap(),
            "---\ntitle: \"Ameiko Kaijitsu\"\ntype: \"NPC\"\ntags: []\n---\n# Ameiko Kaijitsu\n"
        );
        new_page(&dir, "Wiki/Factions", "Sczarni").unwrap();
        assert!(fs::read_to_string(dir.join("Wiki/Factions/Sczarni.md")).unwrap().contains("type: \"Faction\"\n"));
        add_folder(&dir, "Wiki", "Cities").unwrap();
        new_page(&dir, "Wiki/Cities", "Magnimar").unwrap();
        assert!(fs::read_to_string(dir.join("Wiki/Cities/Magnimar.md")).unwrap().contains("type: \"City\"\n"));
        // SD-f: never overwritten; a page outside Wiki/ or with a path in its name is refused.
        fs::write(dir.join("Wiki/NPCs/Ameiko Kaijitsu.md"), "edited").unwrap();
        assert!(new_page(&dir, "Wiki/NPCs", "Ameiko Kaijitsu").unwrap_err().message.contains("already"));
        assert_eq!(fs::read_to_string(dir.join("Wiki/NPCs/Ameiko Kaijitsu.md")).unwrap(), "edited");
        assert!(new_page(&dir, "Maps", "x").is_err());
        assert!(new_page(&dir, "Wiki/NPCs", "../x").is_err());
        let listed = pages(&dir).unwrap();
        let shown: Vec<(&str, &str, Option<&str>)> = listed.iter().map(|p| (p.path.as_str(), p.title.as_str(), p.kind.as_deref())).collect();
        assert_eq!(
            shown,
            [
                ("Wiki/Cities/Magnimar.md", "Magnimar", Some("City")),
                ("Wiki/Factions/Sczarni.md", "Sczarni", Some("Faction")),
                ("Wiki/NPCs/Ameiko Kaijitsu.md", "Ameiko Kaijitsu", None),
            ],
            "an edited page with no frontmatter is listed by its stem"
        );
        put(&dir, "Wiki/Items/Sword.md", "---\ntitle: The Sword\ntags: [weapon, cursed]\n---\n");
        let sword = pages(&dir).unwrap().into_iter().find(|p| p.path == "Wiki/Items/Sword.md").unwrap();
        assert_eq!((sword.title.as_str(), sword.tags), ("The Sword", vec!["weapon".to_owned(), "cursed".to_owned()]));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn wiki_unreadable_frontmatter_listed_by_stem_with_mark() {
        // SD-o: a frontmatter Codex cannot read is listed by filename with a mark, never hidden or
        // rewritten; a page with no frontmatter at all (a plain Obsidian note) carries no mark.
        let dir = campaign("unread");
        let broken = "---\ntitle The Inn\n# The Inn\n";
        put(&dir, "Wiki/Places/Inn.md", broken);
        put(&dir, "Wiki/Places/Plain.md", "# Plain\n");
        put(&dir, "Wiki/Places/Read.md", "---\ntitle: Read\n---\n");
        let listed: Vec<(String, String, bool)> =
            pages(&dir).unwrap().into_iter().map(|p| (p.path, p.title, p.frontmatter_unread)).collect();
        assert_eq!(
            listed,
            [
                ("Wiki/Places/Inn.md".to_owned(), "Inn".to_owned(), true),
                ("Wiki/Places/Plain.md".to_owned(), "Plain".to_owned(), false),
                ("Wiki/Places/Read.md".to_owned(), "Read".to_owned(), false),
            ]
        );
        assert_eq!(fs::read_to_string(dir.join("Wiki/Places/Inn.md")).unwrap(), broken, "never rewritten");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn wiki_resolve_by_title_then_stem_with_alias_and_heading() {
        let dir = campaign("resolve");
        put(&dir, "Wiki/NPCs/ameiko.md", "---\ntitle: Ameiko Kaijitsu\n---\n");
        put(&dir, "Wiki/Places/Ameiko Kaijitsu.md", "# a stem that equals another page's title\n");
        put(&dir, "Wiki/Places/Rusty Dragon.md", "no frontmatter\n");
        put(&dir, "Adventure Log/2026-11-17 Dana - Session 1.md", "---\ntitle: Session 1\n---\n");
        let at = |link: &str| resolve_link(&dir, link).unwrap();
        assert_eq!(at("[[Ameiko Kaijitsu]]").as_deref(), Some("Wiki/NPCs/ameiko.md"), "title before stem");
        assert_eq!(at("[[ameiko kaijitsu|the innkeeper]]").as_deref(), Some("Wiki/NPCs/ameiko.md"), "alias, case");
        assert_eq!(at("[[Rusty Dragon#Upstairs]]").as_deref(), Some("Wiki/Places/Rusty Dragon.md"), "heading; stem");
        assert_eq!(at("Rusty Dragon\\|the inn").as_deref(), Some("Wiki/Places/Rusty Dragon.md"), "a table's escaped pipe, no brackets");
        assert_eq!(at("[[Places/Ameiko Kaijitsu]]").as_deref(), Some("Wiki/Places/Ameiko Kaijitsu.md"), "a path link");
        assert_eq!(at("[[Session 1]]").as_deref(), Some("Adventure Log/2026-11-17 Dana - Session 1.md"), "across the whole folder");
        assert_eq!(at("[[Nobody]]"), None);
        assert_eq!(at("[[ |x]]"), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn wiki_backlinks_from_the_index() {
        let dir = campaign("backlinks");
        put(&dir, "Wiki/NPCs/Ameiko.md", "---\ntitle: Ameiko Kaijitsu\n---\nOwns the [[Rusty Dragon]]. See [[Ameiko Kaijitsu]].\n");
        put(&dir, "Wiki/Places/Rusty Dragon.md", "The inn.\n");
        put(&dir, "Adventure Log/2026-11-17 Dana - Arrival.md", "We met [[ameiko kaijitsu|Ameiko]] at the [[Rusty Dragon#Bar]].\n");
        put(&dir, "Maps/Sandpoint.md", "---\npins: [{ x: 0.5, y: 0.5, label: \"Inn\", page: \"[[Rusty Dragon]]\" }]\n---\n");
        put(&dir, "Party Resources.md", "| [[Rusty Dragon\\|inn]] | 1 |\n[[ broken\n");
        put(&dir, ".codex/notes.md", "[[Rusty Dragon]] hidden folders are not read\n");
        assert_eq!(
            backlinks(&dir, "Wiki/Places/Rusty Dragon.md").unwrap(),
            ["Adventure Log/2026-11-17 Dana - Arrival.md", "Maps/Sandpoint.md", "Party Resources.md", "Wiki/NPCs/Ameiko.md"]
        );
        assert_eq!(backlinks(&dir, "Wiki/NPCs/Ameiko.md").unwrap(), ["Adventure Log/2026-11-17 Dana - Arrival.md"], "a self-link is not a backlink");
        assert!(backlinks(&dir, "Wiki/Items/None.md").unwrap().is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn wiki_search_title_before_text() {
        let dir = campaign("search");
        put(&dir, "Wiki/NPCs/Ameiko.md", "---\ntitle: Ameiko Kaijitsu\n---\nThe innkeeper of the Rusty Dragon.\n");
        put(&dir, "Wiki/Places/Rusty Dragon.md", "Ameiko's inn.\n");
        put(&dir, "Adventure Log/2026-11-17 Dana - Arrival.md", "We slept at the RUSTY dragon, Ämeiko's place.\n");
        let hits = search(&dir, "rusty dragon").unwrap();
        let shown: Vec<(&str, bool)> = hits.iter().map(|h| (h.path.as_str(), h.title_match)).collect();
        assert_eq!(
            shown,
            [("Wiki/Places/Rusty Dragon.md", true), ("Adventure Log/2026-11-17 Dana - Arrival.md", false), ("Wiki/NPCs/Ameiko.md", false)],
            "title hits first, then text, case-insensitive, every .md in the campaign"
        );
        assert_eq!(hits[1].snippet, "We slept at the RUSTY dragon, Ämeiko's place.");
        assert_eq!(search(&dir, "kaijitsu").unwrap()[0].title, "Ameiko Kaijitsu", "by the frontmatter title");
        assert_eq!(search(&dir, "ämeiko").unwrap().len(), 1);
        assert!(search(&dir, "  ").unwrap().is_empty());
        let long = format!("{}needle{}\n", "a".repeat(300), "b".repeat(300));
        put(&dir, "Wiki/Items/Long.md", &long);
        let snippet = &search(&dir, "needle").unwrap()[0].snippet;
        assert!(snippet.starts_with('…') && snippet.ends_with('…') && snippet.contains("needle") && snippet.chars().count() < 120, "{snippet}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn wiki_delete_folder_refuses_when_not_empty() {
        let dir = campaign("delete");
        put(&dir, "Wiki/NPCs/Ameiko.md", "x");
        put(&dir, "Wiki/NPCs/Old/portrait.png", "x");
        assert_eq!(delete_folder(&dir, "Wiki/NPCs").unwrap(), ["Wiki/NPCs/Ameiko.md", "Wiki/NPCs/Old/portrait.png"], "nothing deleted; the files listed");
        assert!(dir.join("Wiki/NPCs/Ameiko.md").is_file() && dir.join("Wiki/NPCs/Old/portrait.png").is_file());
        add_folder(&dir, "Wiki/Items", "Empty").unwrap();
        assert!(delete_folder(&dir, "Wiki/Items").unwrap().is_empty(), "a folder holding only empty folders goes");
        assert!(!dir.join("Wiki/Items").exists());
        for bad in ["Wiki", "Maps", "Wiki/Nope", "Wiki/../Maps"] {
            assert!(delete_folder(&dir, bad).is_err(), "{bad}");
        }
        assert!(dir.join("Maps").is_dir());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn wiki_rename_folder_keeps_pages() {
        let dir = campaign("rename");
        put(&dir, "Wiki/NPCs/Ameiko.md", "---\ntitle: Ameiko\n---\nbody\n");
        assert_eq!(rename_folder(&dir, "Wiki/NPCs", "People").unwrap(), "Wiki/People");
        assert_eq!(fs::read_to_string(dir.join("Wiki/People/Ameiko.md")).unwrap(), "---\ntitle: Ameiko\n---\nbody\n");
        assert!(!dir.join("Wiki/NPCs").exists());
        assert_eq!(resolve_link(&dir, "[[Ameiko]]").unwrap().as_deref(), Some("Wiki/People/Ameiko.md"), "links by title still resolve");
        assert!(rename_folder(&dir, "Wiki/People", "Places").unwrap_err().message.contains("already"), "SD-f");
        assert_eq!(rename_folder(&dir, "Wiki/People", "people").unwrap(), "Wiki/people", "a case-only rename");
        for (from, to) in [("Wiki", "Lore"), ("Maps", "X"), ("Wiki/Nope", "X"), ("Wiki/people", "a/b")] {
            assert!(rename_folder(&dir, from, to).is_err(), "{from} -> {to}");
        }
        let _ = fs::remove_dir_all(&dir);
    }
}
