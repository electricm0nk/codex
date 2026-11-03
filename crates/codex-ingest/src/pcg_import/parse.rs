//! The PCGen `.pcg` line format: `KEY:value|KEY:value`, where a `[...]` value nests the same shape
//! and `&pipe;`-style entities hide the separators inside a value. Reads text into tags and knows
//! nothing about what any key means; that is `map.rs`.

/// One `KEY:value` of a line. A bracketed value keeps its brackets; read it with [`Tag::nested`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    /// The tags inside a `[A:x|B:y]` value. A value with no brackets holds none.
    pub fn nested(&self) -> Vec<Tag> {
        let trimmed = self.value.trim();
        match trimmed.strip_prefix('[').and_then(|rest| rest.strip_suffix(']')) {
            Some(inner) => split_tags(inner),
            None => Vec::new(),
        }
    }
}

/// One line that holds data, with the first tag's key as its key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub tags: Vec<Tag>,
}

impl Line {
    pub fn key(&self) -> &str {
        self.tags.first().map_or("", |tag| tag.key.as_str())
    }

    pub fn value(&self) -> &str {
        self.tags.first().map_or("", |tag| tag.value.as_str())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.tags.iter().find(|tag| tag.key == key).map(|tag| tag.value.as_str())
    }
}

/// Every data line of a file. Blank lines and `#` section comments hold no data.
pub fn parse(text: &str) -> Vec<Line> {
    text.trim_start_matches('\u{feff}')
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| Line { tags: split_tags(line) })
        .filter(|line| !line.tags.is_empty())
        .collect()
}

/// Whether the text is a PCGen character at all, so a wrong file gets a named refusal.
pub fn looks_like_a_character(lines: &[Line]) -> bool {
    lines.iter().any(|line| matches!(line.key(), "PCGVERSION" | "CHARACTERNAME"))
}

/// Splits on `|` outside brackets. Entities decode only in a leaf value, so a bracketed value can
/// still be split after its own `&pipe;` has been kept.
fn split_tags(text: &str) -> Vec<Tag> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    for ch in text.chars() {
        match ch {
            '[' => {
                depth += 1;
                current.push(ch);
            }
            ']' => {
                depth = depth.saturating_sub(1);
                current.push(ch);
            }
            '|' if depth == 0 => parts.push(std::mem::take(&mut current)),
            _ => current.push(ch),
        }
    }
    parts.push(current);
    parts
        .into_iter()
        .filter(|part| !part.trim().is_empty())
        .map(|part| match part.split_once(':') {
            Some((key, value)) => Tag { key: key.trim().to_owned(), value: leaf_or_nested(value) },
            None => Tag { key: part.trim().to_owned(), value: String::new() },
        })
        .collect()
}

fn leaf_or_nested(value: &str) -> String {
    if value.trim_start().starts_with('[') {
        value.to_owned()
    } else {
        decode(value)
    }
}

/// PCGen's `EntityEncoder` table, with `&amp;` last so a literal `&amp;pipe;` stays `&pipe;`.
fn decode(value: &str) -> String {
    value
        .replace("&nl;", "\n")
        .replace("&cr;", "")
        .replace("&lf;", "\n")
        .replace("&colon;", ":")
        .replace("&pipe;", "|")
        .replace("&lbracket;", "[")
        .replace("&rbracket;", "]")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bracketed_value_stays_whole_and_nests() {
        let lines = parse("SKILL:Ride|OUTPUTORDER:23|CLASSBOUGHT:[CLASS:Paladin|RANKS:1.0|COST:1]\n");
        assert_eq!(lines[0].key(), "SKILL");
        assert_eq!(lines[0].value(), "Ride");
        let bought = lines[0].tags.iter().find(|tag| tag.key == "CLASSBOUGHT").unwrap();
        assert_eq!(bought.nested()[1], Tag { key: "RANKS".into(), value: "1.0".into() });
    }

    #[test]
    fn entities_decode_in_a_value_and_comments_are_no_data() {
        let lines = parse("# Section\n\nCHARACTERBIO:a&colon;b&pipe;c&cr;&nl;d\n");
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].value(), "a:b|c\nd");
    }

    #[test]
    fn a_key_with_a_colon_in_its_value_splits_at_the_first_colon() {
        let lines = parse("PORTRAIT:D:\\x.jpg\n");
        assert_eq!(lines[0].value(), "D:\\x.jpg");
    }
}
