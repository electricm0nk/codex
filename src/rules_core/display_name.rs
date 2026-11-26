//! The one place a record's display name is derived (decisions.md §22). A generated record's
//! `name` can be an unfilled source template (`[NAME] Giant-hide`, `Metamagic Rod of [NAME]`)
//! while its variant, `data.key` (`Giant Hide (Hill)`), says which item it is. The name a screen
//! shows comes from [`display_name`]: the operator-approved name for that variant when the
//! approved list holds one, else the record's own name. Every producer of a player-facing name
//! (sheet-rule labels through [`crate::rules_core::sheet_rule::display_label`], the equipment
//! loader, the Market and the feat catalog) asks here, so shipping a name is one table row.

use crate::rules_core::sheet_rule::slug;

/// Approved names by variant (`data.key`). Names are proposed from the book text in
/// `artifacts/epic_4/proposed-names.md` and enter this table only once the operator approves
/// them (decisions.md §22, 20-a): until then it is empty and every record shows its own name.
const APPROVED: &[(&str, &str)] = &[];

/// The name a screen shows for the record whose variant is `key` and whose own name is `name`.
pub fn display_name(key: &str, name: &str) -> String {
    APPROVED.iter().find(|(k, _)| *k == key).map_or_else(|| name.to_owned(), |(_, n)| (*n).to_owned())
}

/// [`display_name`] for a sheet rule, which carries its variant only as the slug of its id
/// (`slug(data.key)`, the converter's own file name).
pub fn display_name_of_slug(rule_slug: &str, label: &str) -> String {
    APPROVED.iter().find(|(k, _)| slug(k) == rule_slug).map_or_else(|| label.to_owned(), |(_, n)| (*n).to_owned())
}

/// Whether a name still holds an unfilled source template: a `[`, a `]`, or the word `NAME` (§22, 20-b).
pub fn holds_placeholder(name: &str) -> bool {
    name.contains(['[', ']']) || name.split(|c: char| !c.is_ascii_alphanumeric()).any(|w| w == "NAME")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::corpus_loader::load_sheet_rules;
    use crate::rules_core::sheet_rule::{display_label, split_rule_id};
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    const PROPOSAL: &str = "docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_4/proposed-names.md";

    fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                json_files(&path, out);
            } else if path.extension().is_some_and(|e| e == "json") {
                out.push(path);
            }
        }
    }

    /// Every `{key, name}` object anywhere in a rules-table file (the Market's rows).
    fn keyed_names(v: &serde_json::Value, out: &mut Vec<(String, String)>) {
        match v {
            serde_json::Value::Object(map) => {
                if let Some(name) = map.get("name").and_then(|n| n.as_str()) {
                    let key = map.get("key").and_then(|k| k.as_str()).unwrap_or("");
                    out.push((key.to_owned(), name.to_owned()));
                }
                map.values().for_each(|v| keyed_names(v, out));
            }
            serde_json::Value::Array(items) => items.iter().for_each(|v| keyed_names(v, out)),
            _ => {}
        }
    }

    #[test]
    fn holds_placeholder_reads_brackets_and_the_word_name() {
        assert!(holds_placeholder("[NAME] Giant-hide"));
        assert!(holds_placeholder("Diplomacy [+1]"));
        assert!(holds_placeholder("Rod of NAME"));
        assert!(!holds_placeholder("Named Bullet"));
        assert!(!holds_placeholder("Hill Giant-hide"));
    }

    /// 20-b: one test over every item kind. A display name holding `[`, `]` or `NAME` fails it
    /// unless `proposed-names.md` proposes a name for that record. Three surfaces: every corpus
    /// record's `data.name` (the 201 of 51,474 E0.3 counted), every sheet rule's
    /// [`display_label`], and every rules-table `{key, name}` row (the Market's source).
    #[test]
    fn no_display_name_of_any_kind_holds_a_placeholder_the_proposal_does_not_cover() {
        let root = crate::support::paths::repo_root();
        let proposal = std::fs::read_to_string(root.join(PROPOSAL)).unwrap_or_default();
        let proposed: BTreeSet<&str> = proposal
            .lines()
            .filter_map(|l| l.strip_prefix("| `data/corpus/"))
            .filter_map(|l| l.split('`').next())
            .collect();

        let mut files = Vec::new();
        json_files(&root.join("data/corpus"), &mut files);
        let (mut scanned, mut offenders) = (0usize, Vec::new());
        let (mut covered_keys, mut covered_slugs) = (BTreeSet::new(), BTreeSet::new());
        for path in &files {
            let rel = path.strip_prefix(root.join("data/corpus")).unwrap().to_string_lossy().replace('\\', "/");
            let parts: Vec<&str> = rel.split('/').collect();
            if parts.len() < 3 || parts.contains(&"_settled") || parts.contains(&"_parity") {
                continue;
            }
            scanned += 1;
            let record: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
            let Some(name) = record["data"]["name"].as_str() else { continue };
            let key = record["data"]["key"].as_str().unwrap_or("");
            if proposed.contains(rel.as_str()) {
                covered_keys.insert(key.to_owned());
                let stem = path.file_stem().unwrap().to_string_lossy().to_string();
                covered_slugs.insert((parts[0].to_owned(), parts[1].to_owned(), slug(key)));
                covered_slugs.insert((parts[0].to_owned(), parts[1].to_owned(), stem));
            } else if holds_placeholder(&display_name(key, name)) {
                offenders.push(format!("corpus {rel}: {name:?}"));
            }
        }
        assert!(scanned > 50_000, "the corpus walk must reach every kind (scanned {scanned})");

        let load = load_sheet_rules(&root.join("data/sheet_rules"));
        for rule in load.package.rules.values() {
            let (book, kind, rule_slug) = split_rule_id(&rule.id);
            let shown = display_label(rule);
            if holds_placeholder(&shown) && !covered_slugs.contains(&(book.to_owned(), kind.to_owned(), rule_slug.to_owned())) {
                offenders.push(format!("sheet rule {}: {shown:?}", rule.id));
            }
        }

        let mut tables = Vec::new();
        json_files(&root.join("data/rules_tables"), &mut tables);
        for path in &tables {
            let value: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
            let mut rows = Vec::new();
            keyed_names(&value, &mut rows);
            for (key, name) in rows {
                if holds_placeholder(&display_name(&key, &name)) && !covered_keys.contains(&key) {
                    offenders.push(format!("rules table {} {key:?}: {name:?}", path.strip_prefix(&root).unwrap().display()));
                }
            }
        }

        assert!(
            offenders.is_empty(),
            "{} display names hold an unfilled template no proposed name covers (first 5): {:#?}",
            offenders.len(),
            &offenders[..offenders.len().min(5)]
        );
    }
}
