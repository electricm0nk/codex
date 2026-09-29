//! SD-36 F1c-4, defect D7: the always-held global abilities.
//!
//! # The oracle rule
//!
//! PCGen gives every character every stat and every check the loaded data declares:
//! `PlayerCharacter()` runs `statFacet.addAll(id, ...PCStat...)` and
//! `checkFacet.addAll(id, ...PCCheck...)` (`code/src/java/pcgen/core/PlayerCharacter.java:572-573`)
//! -- the rows of every `STAT:` and `SAVE:` file a `.pcc` names (`core_rulebook.pcc:52-53`,
//! `SAVE:cr__saves.lst` / `STAT:cr__stats.lst`). A token on such a row therefore applies to
//! every character, and an unconditional `ABILITY:<category>|AUTOMATIC|<key>` on one grants that
//! ability to every character. Pathfinder states exactly one: the Strength row's
//! `ABILITY:Internal|AUTOMATIC|Default` (`cr__stats.lst:4`, and the Core Essentials twin) -- the
//! global `Default` ability every book `.MOD`s its bookkeeping variables onto
//! (`apg_abilities_class.lst:715,717`: `DEFINE:StandardSummonerAllowed|0`,
//! `BONUS:VAR|StandardSummonerAllowed|1`).
//!
//! # What the converter writes
//!
//! Every converted record whose `(CATEGORY, KEY)` is such a global grant's target -- the base
//! row and each book's `.MOD` record alike, since PCGen folds them all into the one object --
//! gets `always_held: true` on its principal ([`SheetRule::always_held`]). One mechanism, read
//! from the oracle's own rows: no record is named here, and a grant carrying any `PRE` item is
//! not unconditional and is not followed.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use codex::rules_core::sheet_rule::SheetRule;

use super::closure::{row_identity, tokenize_row, PinnedTree, RowRef, RowShape};
use super::ctx::CorpusIndex;
use super::rule_file_rel;

/// One unconditional global ability grant: `(CATEGORY upper, KEY upper)` and the row citing it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalGrant {
    pub category: String,
    pub key: String,
    pub cite: String,
}

/// The `(CATEGORY, KEY)` targets of an `ABILITY:` token value when it is an unconditional
/// automatic grant (`<category>|AUTOMATIC|<key>[|<key>...]`, no `PRE`/`!PRE` item); else none.
pub fn unconditional_automatic_targets(value: &str) -> Vec<(String, String)> {
    let parts: Vec<&str> = value.split('|').map(str::trim).collect();
    if parts.len() < 3 || !parts[1].eq_ignore_ascii_case("AUTOMATIC") {
        return Vec::new();
    }
    let keys = &parts[2..];
    if keys.iter().any(|k| k.starts_with("PRE") || k.starts_with("!PRE") || k.starts_with('[')) {
        return Vec::new();
    }
    keys.iter()
        .filter(|k| !k.is_empty() && !k.contains('='))
        .map(|k| (parts[0].to_ascii_uppercase(), k.to_ascii_uppercase()))
        .collect()
}

/// The `.lst` files every character holds every row of: each `STAT:` / `SAVE:` file a `.pcc`
/// under a book directory names, as corpus-relative paths.
fn global_object_files(tree: &PinnedTree) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for dir in tree.book_paths.values() {
        let mut pccs = Vec::new();
        walk_pcc(dir, &mut pccs);
        for pcc in pccs {
            let Ok(text) = std::fs::read_to_string(&pcc) else { continue };
            let pcc_dir = pcc.parent().unwrap_or(dir);
            for line in text.lines() {
                let line = line.trim();
                let Some(target) = line.strip_prefix("STAT:").or_else(|| line.strip_prefix("SAVE:")) else { continue };
                let target = target.split('|').next().unwrap_or("").trim();
                let path = match target.strip_prefix("@/") {
                    Some(rest) => tree.root.join(rest),
                    None => pcc_dir.join(target),
                };
                let rel = path.strip_prefix(&tree.root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
                out.insert(rel);
            }
        }
    }
    out
}

fn walk_pcc(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("pcc") {
                out.push(path);
            }
        }
    }
    out.sort();
}

/// Every unconditional `ABILITY:...|AUTOMATIC|...` grant on a row of a global object file.
pub fn global_grants(tree: &PinnedTree) -> Vec<GlobalGrant> {
    let files = global_object_files(tree);
    let mut out = BTreeSet::new();
    for (fi, file) in tree.files.iter().enumerate() {
        if file.is_pfs || !files.contains(&file.rel_path) {
            continue;
        }
        for (li, raw) in file.lines.iter().enumerate() {
            if raw.trim_start().starts_with('#') {
                continue;
            }
            let (_, tokens) = tokenize_row(raw);
            for (k, v) in tokens {
                if k != "ABILITY" {
                    continue;
                }
                for (category, key) in unconditional_automatic_targets(&v) {
                    out.insert(GlobalGrant { category, key, cite: tree.cite(RowRef { file: fi, line: li + 1 }) });
                }
            }
        }
    }
    out.into_iter().collect()
}

/// Set `always_held` on the principal of every converted record a [`GlobalGrant`] targets.
/// Returns `(marked record ids, grants that named no converted record)`.
pub fn mark_always_held(
    tree: &PinnedTree,
    files: &mut BTreeMap<String, Vec<SheetRule>>,
    index: &CorpusIndex,
    grants: &[GlobalGrant],
) -> (BTreeSet<String>, Vec<String>) {
    let mut marked = BTreeSet::new();
    let mut unresolved = Vec::new();
    for g in grants {
        let mut hit = false;
        for r in &index.records {
            // The record's identity is its own source row's (`row_identity`): a `.MOD` row states
            // its category in its first field (`CATEGORY=Internal|Default.MOD`), never as a token.
            // A `_pfs/` overlay row is not part of the default data set (the B9 rule every closure
            // index already applies), so it is not an always-held record.
            let Some(fi) = (r.line > 0).then(|| tree.file_index(&r.rel_path)).flatten().filter(|&fi| !tree.files[fi].is_pfs) else { continue };
            let id = row_identity(tree.row_text(RowRef { file: fi, line: r.line }));
            if matches!(id.shape, RowShape::NotARecord | RowShape::LevelLine(_)) || id.category != g.category || id.key != g.key {
                continue;
            }
            if let Some(first) = files.get_mut(&rule_file_rel(&r.book, &r.kind, &r.id)).and_then(|rules| rules.first_mut()) {
                first.always_held = true;
                marked.insert(first.id.clone());
                hit = true;
            }
        }
        if !hit {
            unresolved.push(format!("{}|{} ({}): an always-held global grant names no converted record", g.category, g.key, g.cite));
        }
    }
    (marked, unresolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_an_unconditional_automatic_grant_is_global() {
        assert_eq!(unconditional_automatic_targets("Internal|AUTOMATIC|Default"), vec![("INTERNAL".into(), "DEFAULT".into())]);
        assert!(unconditional_automatic_targets("Internal|AUTOMATIC|Default|PRERACE:1,Elf").is_empty());
        assert!(unconditional_automatic_targets("Internal|AUTOMATIC|Default|!PREVARGT:X,0").is_empty());
        assert!(unconditional_automatic_targets("Special Ability|VIRTUAL|Darkvision").is_empty());
        assert!(unconditional_automatic_targets("1,CATEGORY=Special Ability,TYPE.Racial Vision").is_empty());
        assert!(unconditional_automatic_targets("Internal|AUTOMATIC|TYPE=Something").is_empty());
    }
}
