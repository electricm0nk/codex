//! The YAML subset Obsidian writes at the top of a Markdown file (SD-42 `decisions.md §15.2`):
//! `key: value`, lists of scalars (`- item` lines or `[a, b]`), and quoted strings. Anything else
//! inside the block (a nested map, a block scalar) is kept as the raw text of its lines, never an
//! error. A block that does not close, or a top-level line that is not `key: value`, is unreadable:
//! the caller lists the file with its filename (SD-o) and never rewrites it.

use std::path::Path;

use super::CampaignStoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Scalar(String),
    List(Vec<String>),
}

impl Value {
    /// The scalar's text; `None` for a list or an empty value.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Scalar(text) if !text.is_empty() => Some(text),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frontmatter {
    /// In file order; a repeated key keeps its first value (`get`).
    pub entries: Vec<(String, Value)>,
}

impl Frontmatter {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn str(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(Value::as_str)
    }
}

/// What `parse` found at the top of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Parsed {
    /// No `---` first line: the file has no frontmatter.
    Missing,
    /// A `---` block Codex cannot read (no closing line, or a line that is not `key: value`).
    Unreadable,
    Read(Frontmatter),
}

/// One line of the text with its byte range, the line ending excluded.
struct Line<'a> {
    start: usize,
    /// End of the line's content, before `\r\n` / `\n`.
    end: usize,
    /// Start of the next line.
    next: usize,
    text: &'a str,
}

fn lines(text: &str) -> Vec<Line<'_>> {
    let mut out = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let next = text[start..].find('\n').map_or(text.len(), |i| start + i + 1);
        let mut end = next;
        if text[start..end].ends_with('\n') {
            end -= 1;
        }
        if text[start..end].ends_with('\r') {
            end -= 1;
        }
        out.push(Line { start, end, next, text: &text[start..end] });
        start = next;
    }
    out
}

const BOM: &str = "\u{feff}";

/// The block's lines between the two `---` lines: `Some((open, close))` line indices, or `None`
/// with `Missing`/`Unreadable` said by the caller.
fn block(all: &[Line<'_>]) -> Result<(usize, usize), Parsed> {
    let first = all.first().map(|l| l.text.strip_prefix(BOM).unwrap_or(l.text));
    if first.map(str::trim_end) != Some("---") {
        return Err(Parsed::Missing);
    }
    all.iter()
        .enumerate()
        .skip(1)
        .find(|(_, l)| matches!(l.text.trim_end(), "---" | "..."))
        .map(|(close, _)| (0, close))
        .ok_or(Parsed::Unreadable)
}

/// `key: rest` for a top-level line, or `None`.
fn key_line(text: &str) -> Option<(&str, &str)> {
    if text.starts_with([' ', '\t', '-', '#']) {
        return None;
    }
    let colon = text.find(':')?;
    let key = text[..colon].trim_end();
    let rest = &text[colon + 1..];
    (!key.is_empty() && (rest.is_empty() || rest.starts_with([' ', '\t']))).then_some((key, rest.trim()))
}

/// A scalar as written: `"json-style"`, `'single ''quoted'''`, or plain (a ` #` comment dropped).
fn scalar(raw: &str) -> String {
    let raw = raw.trim();
    if let Some(text) = raw.starts_with('"').then(|| serde_json::from_str::<String>(raw).ok()).flatten() {
        return text;
    }
    if raw.len() >= 2 && raw.starts_with('\'') && raw.ends_with('\'') {
        return raw[1..raw.len() - 1].replace("''", "'");
    }
    match raw.find(" #") {
        Some(i) => raw[..i].trim_end().to_owned(),
        None if raw.starts_with('#') => String::new(),
        None => raw.to_owned(),
    }
}

/// `[a, "b", c]` with no nested brackets or braces → its items; else `None` (kept raw).
fn flow_list(raw: &str) -> Option<Vec<String>> {
    let inner = raw.strip_prefix('[')?.strip_suffix(']')?;
    if inner.contains(['[', ']', '{', '}']) {
        return None;
    }
    if inner.trim().is_empty() {
        return Some(Vec::new());
    }
    Some(inner.split(',').map(scalar).collect())
}

pub fn parse(text: &str) -> Parsed {
    let all = lines(text);
    let (open, close) = match block(&all) {
        Ok(range) => range,
        Err(parsed) => return parsed,
    };
    let mut entries: Vec<(String, Value)> = Vec::new();
    // The indented lines under the current key.
    let mut under: Vec<&str> = Vec::new();
    let finish = |entries: &mut Vec<(String, Value)>, under: &mut Vec<&str>| {
        if under.is_empty() {
            return;
        }
        let items: Option<Vec<String>> = under
            .iter()
            .map(|l| l.trim_start().strip_prefix('-').filter(|r| r.is_empty() || r.starts_with(' ')).map(scalar))
            .collect();
        if let Some((_, value)) = entries.last_mut() {
            // `key:` then indented lines, or a block scalar (`|` keeps line breaks, `>` folds them).
            let joiner = match value {
                Value::Scalar(s) if s.is_empty() || s.starts_with('>') => Some(" "),
                Value::Scalar(s) if s.starts_with('|') => Some("\n"),
                _ => None,
            };
            if let Some(joiner) = joiner {
                let block = matches!(value, Value::Scalar(s) if !s.is_empty());
                *value = match items {
                    Some(items) if !block => Value::List(items),
                    _ => Value::Scalar(under.iter().map(|l| l.trim()).collect::<Vec<_>>().join(joiner)),
                };
            }
        }
        under.clear();
    };
    for line in &all[open + 1..close] {
        let text = line.text;
        if text.trim().is_empty() || text.trim_start().starts_with('#') {
            continue;
        }
        if let Some((key, rest)) = key_line(text) {
            finish(&mut entries, &mut under);
            let value = match flow_list(rest) {
                Some(items) => Value::List(items),
                None => Value::Scalar(scalar(rest)),
            };
            entries.push((key.to_owned(), value));
        } else if text.starts_with([' ', '\t', '-']) && !entries.is_empty() {
            under.push(text);
        } else {
            return Parsed::Unreadable;
        }
    }
    finish(&mut entries, &mut under);
    Parsed::Read(Frontmatter { entries })
}

/// Sets `key` to the raw YAML `value` (written as given: quote it first if it needs quoting) and
/// returns the new text. Only that key's line(s) change; every other byte, the line endings
/// included, is kept. A missing key is added before the closing `---`; a file with no
/// frontmatter gets a block at the top. An unreadable block is refused, never rewritten.
pub fn set_key(text: &str, key: &str, value: &str) -> Result<String, CampaignStoreError> {
    let refuse = |why: &str| CampaignStoreError { message: format!("frontmatter key {key:?} not written: {why}") };
    if key_line(&format!("{key}: x")).map(|(k, _)| k) != Some(key) || value.contains('\n') {
        return Err(refuse("not a plain key and one-line value"));
    }
    let all = lines(text);
    let (_, close) = match block(&all) {
        Ok(range) => range,
        Err(Parsed::Missing) => {
            let (bom, rest) = text.strip_prefix(BOM).map_or(("", text), |rest| (BOM, rest));
            let eol = if rest.contains("\r\n") { "\r\n" } else { "\n" };
            return Ok(format!("{bom}---{eol}{key}: {value}{eol}---{eol}{rest}"));
        }
        Err(_) => return Err(refuse("the file's frontmatter is not readable")),
    };
    if matches!(parse(text), Parsed::Unreadable) {
        return Err(refuse("the file's frontmatter is not readable"));
    }
    let eol = if text[all[0].end..all[0].next].contains('\r') { "\r\n" } else { "\n" };
    let new_line = format!("{key}: {value}");
    let found = (1..close).find(|&i| key_line(all[i].text).is_some_and(|(k, _)| k == key));
    let Some(at) = found else {
        let insert = all[close].start;
        return Ok(format!("{}{new_line}{eol}{}", &text[..insert], &text[insert..]));
    };
    // The key's own line plus the indented lines under it.
    let mut last = at;
    while last + 1 < close && all[last + 1].text.starts_with([' ', '\t', '-']) {
        last += 1;
    }
    Ok(format!("{}{new_line}{}", &text[..all[at].start], &text[all[last].end..]))
}

/// `set_key` on a file, written atomically. The file is left untouched when the key cannot be set.
pub fn write_key(file: &Path, key: &str, value: &str) -> Result<(), CampaignStoreError> {
    let text = std::fs::read_to_string(file).map_err(|err| CampaignStoreError { message: format!("{}: {err}", file.display()) })?;
    super::folder::write_atomic(file, set_key(&text, key, value)?.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(text: &str) -> Frontmatter {
        match parse(text) {
            Parsed::Read(fm) => fm,
            other => panic!("not read: {other:?}"),
        }
    }

    #[test]
    fn frontmatter_reads_scalars_lists_and_quoted_strings() {
        let fm = read(
            "---\ncampaign: \"Rise: of the \\\"Runelords\\\"\"\nauthor: 'Dana ''the'' Bold'\nposted: 2026-11-17 # today\n\
             tags: [npc, \"ally\"]\naliases:\n  - Ameiko\n  - 'Kaijitsu'\nin_game_date:\nempty: []\n---\n# Body\n",
        );
        assert_eq!(fm.str("campaign"), Some("Rise: of the \"Runelords\""));
        assert_eq!(fm.str("author"), Some("Dana 'the' Bold"));
        assert_eq!(fm.str("posted"), Some("2026-11-17"));
        assert_eq!(fm.get("tags"), Some(&Value::List(vec!["npc".into(), "ally".into()])));
        assert_eq!(fm.get("aliases"), Some(&Value::List(vec!["Ameiko".into(), "Kaijitsu".into()])));
        assert_eq!(fm.get("in_game_date"), Some(&Value::Scalar(String::new())));
        assert_eq!(fm.str("in_game_date"), None);
        assert_eq!(fm.get("empty"), Some(&Value::List(vec![])));
    }

    #[test]
    fn frontmatter_missing_and_unreadable_are_told_apart() {
        assert_eq!(parse("# Just a note\n"), Parsed::Missing);
        assert_eq!(parse(""), Parsed::Missing);
        assert_eq!(parse("---\ntitle: never closed\n"), Parsed::Unreadable);
        assert_eq!(parse("---\nthis line has no colon\n---\n"), Parsed::Unreadable);
        // CRLF, a BOM and `...` as the closing line all read.
        let fm = read("\u{feff}---\r\ntitle: Windows\r\n...\r\nbody\r\n");
        assert_eq!(fm.str("title"), Some("Windows"));
    }

    #[test]
    fn frontmatter_keeps_nested_values_raw_instead_of_failing() {
        // A nested map or block scalar is not the subset, but the rest of the block still reads.
        let fm = read("---\npins: [{ x: 0.4, y: 0.1 }]\nsummary: |\n  line one\n  line two\ntitle: Kept\n---\n");
        assert_eq!(fm.str("pins"), Some("[{ x: 0.4, y: 0.1 }]"));
        assert_eq!(fm.str("summary"), Some("line one\nline two"));
        assert_eq!(fm.str("title"), Some("Kept"));
    }

    #[test]
    fn frontmatter_set_key_replaces_adds_and_refuses() {
        let text = "---\r\ntitle: Old\r\ntags:\r\n  - a\r\n  - b\r\nauthor: Dana\r\n---\r\nBody stays.\r\n";
        assert_eq!(
            set_key(text, "tags", "[c]").unwrap(),
            "---\r\ntitle: Old\r\ntags: [c]\r\nauthor: Dana\r\n---\r\nBody stays.\r\n"
        );
        assert_eq!(
            set_key(text, "in_game_date", "Rova 3").unwrap(),
            "---\r\ntitle: Old\r\ntags:\r\n  - a\r\n  - b\r\nauthor: Dana\r\nin_game_date: Rova 3\r\n---\r\nBody stays.\r\n"
        );
        assert_eq!(set_key("Body\n", "title", "New").unwrap(), "---\ntitle: New\n---\nBody\n");
        assert!(set_key("---\nno colon\n---\n", "title", "x").is_err(), "unreadable block rewritten");
        assert!(set_key("---\ntitle: x\n---\n", "bad key:", "x").is_err());
        assert!(set_key("---\ntitle: x\n---\n", "title", "two\nlines").is_err());
    }

    #[test]
    fn frontmatter_write_key_leaves_the_file_alone_on_refusal() {
        let dir = std::env::temp_dir().join(format!("codex-frontmatter-write-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("note.md");
        std::fs::write(&file, "---\ntitle: A\n---\nbody").unwrap();
        write_key(&file, "title", "B").unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "---\ntitle: B\n---\nbody");
        std::fs::write(&file, "---\nbroken\n---\nbody").unwrap();
        assert!(write_key(&file, "title", "B").is_err());
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "---\nbroken\n---\nbody");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
