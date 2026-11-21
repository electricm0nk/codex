//! Maps with pins (SD-42 `decisions.md §15.3`, Q17): `Maps/<name>.png|jpg|jpeg` beside
//! `Maps/<name>.md`, whose frontmatter holds the pins as fractions of the image's width and
//! height (`§21` Q-E): `pins: [{ x: 0.42, y: 0.17, label: "Ferry", page: "[[Riverside Ferry]]" }]`.

use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::frontmatter::{self, Parsed, Value};
use super::{folder, CampaignStoreError};

pub const MAPS_DIR: &str = "Maps";
const IMAGE_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

/// One pin: where on the image (fractions, `0..=1`), its label, and the wiki page it links to
/// (the name inside `[[…]]`, as Obsidian writes it).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pin {
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub page: String,
}

/// One map: an image and its notes file, or a notes file with no image yet (a migrated map note).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapEntry {
    /// The notes file's `title`, else the filename stem.
    pub title: String,
    /// `Maps/<name>.png|jpg|jpeg`, `/`-separated, relative to the campaign folder.
    pub image: Option<String>,
    /// `Maps/<stem>.md`, whether or not it exists yet.
    pub notes: String,
    pub notes_exists: bool,
    pub pins: Vec<Pin>,
    /// False when the notes file's frontmatter or `pins` cannot be read: shown, never rewritten.
    pub pins_ok: bool,
}

fn err(message: String) -> CampaignStoreError {
    CampaignStoreError { message }
}

fn io_error(path: &Path, cause: std::io::Error) -> CampaignStoreError {
    err(format!("{}: {cause}", path.display()))
}

/// Reads YAML flow collections of scalars: `[{ k: v, … }, …]` and `{ k: v, … }`.
struct Flow<'a> {
    text: &'a str,
    at: usize,
}

impl<'a> Flow<'a> {
    fn rest(&self) -> &'a str {
        &self.text[self.at..]
    }

    fn eat(&mut self, c: char) -> bool {
        self.at = self.text.len() - self.rest().trim_start().len();
        let hit = self.rest().starts_with(c);
        if hit {
            self.at += 1;
        }
        hit
    }

    /// A double-quoted (JSON escapes), single-quoted or plain scalar; plain stops at `stops`.
    fn scalar(&mut self, stops: &[char]) -> Option<String> {
        self.eat(' ');
        let rest = self.rest();
        if rest.starts_with('"') {
            let mut escaped = false;
            let end = rest.char_indices().skip(1).find(|&(_, c)| {
                let close = c == '"' && !escaped;
                escaped = c == '\\' && !escaped;
                close
            })?.0;
            let text = serde_json::from_str(&rest[..=end]).ok()?;
            self.at += end + 1;
            return Some(text);
        }
        if rest.starts_with('\'') {
            let mut from = 1;
            loop {
                let quote = rest[from..].find('\'')? + from;
                if rest[quote + 1..].starts_with('\'') {
                    from = quote + 2;
                    continue;
                }
                self.at += quote + 1;
                return Some(rest[1..quote].replace("''", "'"));
            }
        }
        let end = rest.find(stops).unwrap_or(rest.len());
        let text = rest[..end].trim().to_owned();
        self.at += end;
        Some(text)
    }

    fn map(&mut self) -> Option<Vec<(String, String)>> {
        if !self.eat('{') {
            return None;
        }
        let mut out = Vec::new();
        if self.eat('}') {
            return Some(out);
        }
        loop {
            let key = self.scalar(&[':', ',', '}'])?;
            if !self.eat(':') {
                return None;
            }
            out.push((key, self.scalar(&[',', '}'])?));
            if self.eat('}') {
                return Some(out);
            }
            if !self.eat(',') {
                return None;
            }
        }
    }

    fn seq_of_maps(&mut self) -> Option<Vec<Vec<(String, String)>>> {
        if !self.eat('[') {
            return None;
        }
        let mut out = Vec::new();
        if self.eat(']') {
            return Some(out);
        }
        loop {
            out.push(self.map()?);
            if self.eat(']') {
                return Some(out);
            }
            if !self.eat(',') {
                return None;
            }
        }
    }

    fn done(&self) -> bool {
        self.rest().trim().is_empty()
    }
}

fn fraction(raw: Option<&String>, name: &str) -> Result<f64, String> {
    let raw = raw.ok_or(format!("a pin has no {name}"))?;
    raw.parse::<f64>()
        .ok()
        .filter(|v| (0.0..=1.0).contains(v))
        .ok_or(format!("a pin's {name} {raw:?} is not a fraction of the image (0 to 1)"))
}

fn pin_of(fields: Vec<(String, String)>) -> Result<Pin, String> {
    let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v);
    let page = get("page").map(String::as_str).unwrap_or_default();
    let page = page.strip_prefix("[[").and_then(|p| p.strip_suffix("]]")).unwrap_or(page);
    Ok(Pin { x: fraction(get("x"), "x")?, y: fraction(get("y"), "y")?, label: get("label").cloned().unwrap_or_default(), page: page.to_owned() })
}

/// The pins in a notes file's frontmatter; none when the file has no frontmatter or no `pins`.
/// An unreadable block or `pins` value is an error (the caller shows it and never rewrites it).
pub fn pins(text: &str) -> Result<Vec<Pin>, CampaignStoreError> {
    let fm = match frontmatter::parse(text) {
        Parsed::Missing => return Ok(Vec::new()),
        Parsed::Unreadable => return Err(err("the notes file's frontmatter is not readable".to_owned())),
        Parsed::Read(fm) => fm,
    };
    let unread = || err("the notes file's `pins` are not in a form Codex reads ([{ x: …, y: …, label: …, page: … }])".to_owned());
    let maps = match fm.get("pins") {
        None => return Ok(Vec::new()),
        Some(Value::Scalar(raw)) if raw.is_empty() => return Ok(Vec::new()),
        Some(Value::Scalar(raw)) => {
            let mut flow = Flow { text: raw, at: 0 };
            flow.seq_of_maps().filter(|_| flow.done()).ok_or_else(unread)?
        }
        Some(Value::List(items)) => items
            .iter()
            .map(|item| {
                let mut flow = Flow { text: item, at: 0 };
                flow.map().filter(|_| flow.done())
            })
            .collect::<Option<_>>()
            .ok_or_else(unread)?,
    };
    maps.into_iter().map(pin_of).collect::<Result<_, _>>().map_err(err)
}

/// The `pins` value as one flow line; strings JSON-quoted (valid YAML double-quoted scalars).
fn pins_value(pins: &[Pin]) -> Result<String, CampaignStoreError> {
    let quote = |text: &str| serde_json::to_string(text).unwrap_or_default();
    let items = pins
        .iter()
        .map(|pin| {
            fraction(Some(&pin.x.to_string()), "x")?;
            fraction(Some(&pin.y.to_string()), "y")?;
            let page = if pin.page.is_empty() { String::new() } else { format!("[[{}]]", pin.page) };
            Ok(format!("{{ x: {}, y: {}, label: {}, page: {} }}", pin.x, pin.y, quote(&pin.label), quote(&page)))
        })
        .collect::<Result<Vec<_>, String>>()
        .map_err(err)?;
    Ok(format!("[{}]", items.join(", ")))
}

/// Writes `pins` into the notes file's frontmatter through `frontmatter::set_key`, so every other
/// byte stays. A missing notes file is created with a `title` (its stem). A `pins` value Codex
/// cannot read (a hand edit) is refused, never overwritten.
pub fn set_pins(md: &Path, pins: &[Pin]) -> Result<(), CampaignStoreError> {
    let value = pins_value(pins)?;
    match fs::read(md) {
        Ok(bytes) => {
            let text = String::from_utf8(bytes).map_err(|_| err(format!("{} is not UTF-8 text", md.display())))?;
            self::pins(&text).map_err(|cause| err(format!("pins not written: {}", cause.message)))?;
        }
        Err(cause) if cause.kind() == ErrorKind::NotFound => {
            let stem = md.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
            if let Some(parent) = md.parent() {
                fs::create_dir_all(parent).map_err(|cause| io_error(parent, cause))?;
            }
            let title = format!("---\ntitle: {}\n---\n", serde_json::to_string(&stem).unwrap_or_default());
            folder::write_atomic(md, title.as_bytes())?;
        }
        Err(cause) => return Err(io_error(md, cause)),
    }
    frontmatter::write_key(md, "pins", &value)
}

fn image_stem(name: &str) -> Option<&str> {
    let (stem, ext) = name.rsplit_once('.')?;
    (!stem.is_empty() && IMAGE_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str())).then_some(stem)
}

/// "Add map…": copies the picked image into `Maps/` under its own name and creates its notes file
/// (an existing one, e.g. a migrated map note of the same name, is kept). A taken image name is
/// refused, never overwritten or suffixed (SD-f). Returns the image's path inside the campaign.
pub fn add_map(dir: &Path, src_image: &Path) -> Result<String, CampaignStoreError> {
    folder::open(dir)?;
    let name = src_image.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let stem = image_stem(&name).ok_or_else(|| err(format!("{name:?} is not a .png, .jpg or .jpeg image")))?;
    let maps = dir.join(MAPS_DIR);
    fs::create_dir_all(&maps).map_err(|cause| io_error(&maps, cause))?;
    let mut source = fs::File::open(src_image).map_err(|cause| io_error(src_image, cause))?;
    let target = maps.join(&name);
    let mut copy = match fs::OpenOptions::new().write(true).create_new(true).open(&target) {
        Ok(file) => file,
        Err(cause) if cause.kind() == ErrorKind::AlreadyExists => {
            return Err(err(format!("{MAPS_DIR}/{name} already exists; rename the image and add it again")));
        }
        Err(cause) => return Err(io_error(&target, cause)),
    };
    if let Err(cause) = std::io::copy(&mut source, &mut copy) {
        let _ = fs::remove_file(&target);
        return Err(io_error(&target, cause));
    }
    let notes = maps.join(format!("{stem}.md"));
    if !notes.exists() {
        set_pins(&notes, &[])?;
    }
    Ok(format!("{MAPS_DIR}/{name}"))
}

/// Every image directly under `Maps/` with its `<stem>.md`, plus any notes file with no image;
/// sorted by name, case-insensitive.
// shortcut: top-level files only, as the Adventure Log; read subfolders if players nest maps.
pub fn list(dir: &Path) -> Result<Vec<MapEntry>, CampaignStoreError> {
    let maps = dir.join(MAPS_DIR);
    let read_dir = match fs::read_dir(&maps) {
        Ok(read_dir) => read_dir,
        Err(cause) if cause.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(cause) => return Err(io_error(&maps, cause)),
    };
    let mut names: Vec<String> = read_dir
        .filter_map(Result::ok)
        .filter(|item| item.file_type().is_ok_and(|t| t.is_file()))
        .map(|item| item.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort_by_key(|name| name.to_lowercase());
    let md_stem = |name: &str| name.strip_suffix(".md").or_else(|| name.strip_suffix(".MD")).map(str::to_owned);
    let mut paired = Vec::new();
    let mut out = Vec::new();
    let mut push = |stem: &str, image: Option<String>, notes_name: Option<&String>| -> Result<(), CampaignStoreError> {
        let notes_name = notes_name.cloned().unwrap_or_else(|| format!("{stem}.md"));
        let (mut title, mut pins_ok, mut found) = (stem.to_owned(), true, Vec::new());
        let notes_exists = notes_name != format!("{stem}.md") || maps.join(&notes_name).is_file();
        if notes_exists {
            let path = maps.join(&notes_name);
            let text = String::from_utf8_lossy(&fs::read(&path).map_err(|cause| io_error(&path, cause))?).into_owned();
            if let Parsed::Read(fm) = frontmatter::parse(&text) {
                title = fm.str("title").map_or(title, str::to_owned);
            }
            match pins(&text) {
                Ok(read) => found = read,
                Err(_) => pins_ok = false,
            }
        }
        out.push(MapEntry { title, image, notes: format!("{MAPS_DIR}/{notes_name}"), notes_exists, pins: found, pins_ok });
        Ok(())
    };
    for name in &names {
        let Some(stem) = image_stem(name) else { continue };
        let notes = names.iter().find(|n| md_stem(n).as_deref() == Some(stem));
        if let Some(notes) = notes {
            paired.push(notes.clone());
        }
        push(stem, Some(format!("{MAPS_DIR}/{name}")), notes)?;
    }
    for name in &names {
        if let Some(stem) = md_stem(name).filter(|_| !paired.contains(name)) {
            push(&stem, None, Some(name))?;
        }
    }
    let key = |m: &MapEntry| m.notes.to_lowercase();
    out.sort_by(|a, b| key(a).cmp(&key(b)).then_with(|| a.image.cmp(&b.image)));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::{folder, CampaignFolderSnapshot, FolderMember, CAMPAIGN_FOLDER_SCHEMA_VERSION};
    use std::fs;
    use std::path::{Path, PathBuf};

    fn campaign(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("codex-maps-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let snapshot = CampaignFolderSnapshot {
            schema_version: CAMPAIGN_FOLDER_SCHEMA_VERSION,
            name: "Maps test".to_owned(),
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

    fn pin(x: f64, y: f64, label: &str, page: &str) -> Pin {
        Pin { x, y, label: label.to_owned(), page: page.to_owned() }
    }

    fn put(dir: &Path, name: &str, bytes: &[u8]) {
        fs::write(dir.join(MAPS_DIR).join(name), bytes).unwrap();
    }

    #[test]
    fn maps_pins_round_trip_as_fractions() {
        let dir = campaign("roundtrip");
        let md = dir.join(MAPS_DIR).join("Sandpoint.md");
        let want = vec![pin(0.42, 0.17, "Ferry", "Riverside Ferry"), pin(0.0, 1.0, "Room #3, \"the\" vault", "Places/Vault#Door")];
        // No notes file yet: set_pins creates it with a title.
        set_pins(&md, &want).unwrap();
        let text = fs::read_to_string(&md).unwrap();
        assert!(text.starts_with("---\ntitle: \"Sandpoint\"\npins: [{ x: 0.42, y: 0.17, label: \"Ferry\", page: \"[[Riverside Ferry]]\" }, "), "{text}");
        assert_eq!(pins(&text).unwrap(), want);
        // The shapes a hand edit or Obsidian leaves: the spec's own line, single quotes, a block list.
        assert_eq!(
            pins("---\npins: [{ x: 0.42, y: 0.17, label: \"Ferry\", page: \"[[Riverside Ferry]]\" }]\n---\n").unwrap(),
            vec![pin(0.42, 0.17, "Ferry", "Riverside Ferry")]
        );
        assert_eq!(
            pins("---\npins:\n  - { x: 1, y: .5, label: 'It''s', page: \"[[A|alias]]\" }\n  - {x: 0.1, y: 0.2}\n---\n").unwrap(),
            vec![pin(1.0, 0.5, "It's", "A|alias"), pin(0.1, 0.2, "", "")]
        );
        assert_eq!(pins("---\ntitle: x\n---\n").unwrap(), vec![], "no pins key");
        assert_eq!(pins("no frontmatter").unwrap(), vec![]);
        assert!(pins("---\npins: [{ x: 1.5, y: 0.2 }]\n---\n").is_err(), "outside the image");
        assert!(pins("---\npins: [{ y: 0.2 }]\n---\n").is_err(), "x missing");
        assert!(pins("---\npins:\n  - x: 0.1\n    y: 0.2\n---\n").is_err(), "block maps are not read");
        assert!(set_pins(&md, &[pin(-0.1, 0.5, "", "")]).is_err(), "a fraction outside 0..1 is refused");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn maps_set_pins_keeps_the_rest_of_the_file() {
        let dir = campaign("keep");
        let md = dir.join(MAPS_DIR).join("Docks.md");
        let before = "---\r\ntitle: The Docks\r\ntags: [harbour]\r\npins: [{ x: 0.1, y: 0.1 }]\r\n---\r\n# Notes\r\n\r\nSmugglers at night.\r\n";
        fs::write(&md, before).unwrap();
        set_pins(&md, &[pin(0.5, 0.25, "Warehouse", "Warehouse 9")]).unwrap();
        assert_eq!(
            fs::read_to_string(&md).unwrap(),
            before.replace("pins: [{ x: 0.1, y: 0.1 }]", "pins: [{ x: 0.5, y: 0.25, label: \"Warehouse\", page: \"[[Warehouse 9]]\" }]")
        );
        set_pins(&md, &[]).unwrap();
        assert!(fs::read_to_string(&md).unwrap().contains("\r\npins: []\r\n---\r\n# Notes"));
        // A pins value Codex cannot read is a hand edit: refused, the file untouched.
        let hand = "---\npins:\n  - x: 0.1\n    y: 0.2\n---\nbody\n";
        fs::write(&md, hand).unwrap();
        assert!(set_pins(&md, &[pin(0.5, 0.5, "", "")]).is_err());
        assert_eq!(fs::read_to_string(&md).unwrap(), hand);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn maps_add_map_copies_image_and_creates_notes() {
        let dir = campaign("add");
        let src_dir = std::env::temp_dir().join(format!("codex-maps-src-{}", std::process::id()));
        let _ = fs::remove_dir_all(&src_dir);
        fs::create_dir_all(&src_dir).unwrap();
        let src = src_dir.join("Old Town.PNG");
        fs::write(&src, b"\x89PNG fake bytes").unwrap();
        assert_eq!(add_map(&dir, &src).unwrap(), "Maps/Old Town.PNG");
        assert_eq!(fs::read(dir.join("Maps/Old Town.PNG")).unwrap(), b"\x89PNG fake bytes");
        assert_eq!(fs::read_to_string(dir.join("Maps/Old Town.md")).unwrap(), "---\ntitle: \"Old Town\"\npins: []\n---\n");
        // SD-f: a taken name is refused, nothing overwritten.
        fs::write(&src, b"other").unwrap();
        assert!(add_map(&dir, &src).unwrap_err().message.contains("already"));
        assert_eq!(fs::read(dir.join("Maps/Old Town.PNG")).unwrap(), b"\x89PNG fake bytes");
        // Notes that already exist (a migrated map note) are kept and paired.
        put(&dir, "Harbour.md", b"# Harbour notes\n");
        let jpg = src_dir.join("Harbour.jpg");
        fs::write(&jpg, b"jpg").unwrap();
        assert_eq!(add_map(&dir, &jpg).unwrap(), "Maps/Harbour.jpg");
        assert_eq!(fs::read_to_string(dir.join("Maps/Harbour.md")).unwrap(), "# Harbour notes\n");
        // Only png/jpg/jpeg.
        let gif = src_dir.join("x.gif");
        fs::write(&gif, b"gif").unwrap();
        assert!(add_map(&dir, &gif).is_err());
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_dir_all(&src_dir);
    }

    #[test]
    fn maps_list_pairs_image_and_notes() {
        let dir = campaign("list");
        put(&dir, "Sandpoint.png", b"png");
        put(&dir, "Sandpoint.md", b"---\ntitle: Sandpoint and hinterland\npins: [{ x: 0.5, y: 0.5, label: Ferry, page: \"[[Ferry]]\" }]\n---\n");
        put(&dir, "cave.JPEG", b"jpeg");
        put(&dir, "Migrated note.md", b"# from the old campaign\n");
        put(&dir, "broken.jpg", b"jpg");
        put(&dir, "broken.md", b"---\npins:\n  - x: 1\n    y: 1\n---\n");
        put(&dir, "readme.txt", b"not a map");
        let listed = list(&dir).unwrap();
        type Row<'a> = (&'a str, Option<&'a str>, &'a str, bool, bool, usize);
        let shown: Vec<Row> = listed
            .iter()
            .map(|m| (m.title.as_str(), m.image.as_deref(), m.notes.as_str(), m.notes_exists, m.pins_ok, m.pins.len()))
            .collect();
        assert_eq!(
            shown,
            [
                ("broken", Some("Maps/broken.jpg"), "Maps/broken.md", true, false, 0),
                ("cave", Some("Maps/cave.JPEG"), "Maps/cave.md", false, true, 0),
                ("Migrated note", None, "Maps/Migrated note.md", true, true, 0),
                ("Sandpoint and hinterland", Some("Maps/Sandpoint.png"), "Maps/Sandpoint.md", true, true, 1),
            ]
        );
        assert_eq!(listed[3].pins, vec![pin(0.5, 0.5, "Ferry", "Ferry")]);
        fs::remove_dir_all(dir.join(MAPS_DIR)).unwrap();
        assert!(list(&dir).unwrap().is_empty(), "no Maps folder lists nothing");
        let _ = fs::remove_dir_all(&dir);
    }
}
