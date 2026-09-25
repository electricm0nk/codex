//! SD-36 Epic F3b2b (2): the standing supersession ruling, applied to the converter's reference
//! resolver (`decisions.md` §12, "Converter resolver scope").
//!
//! Operator ruling 2026-08-16 (SD-31): "if a duplicate is found, the most recent publishing takes
//! precedence and the older one is flagged as superseded". Its two guards are part of the rule:
//! publication order comes from the `.pcc` `SOURCEDATE:` headers
//! ([`PinnedTree::source_dates`]), never from memory; and a later VARIANT (a mythic version, an
//! Unchained version) is a new object, not a reprint, so two records must be proved to be the
//! SAME object field by field before the newer one is chosen.
//!
//! # The rule (one mechanism, no per-record case)
//!
//! A reference whose `(category, key-or-name)` pair names more than one record (an ambiguous
//! pair, [`super::ctx::CorpusIndex::ambiguous_cat_key`]) resolves to the newest printing when
//! every candidate is a printing of one object:
//!
//! - the candidates share their kind, and none sits in a variant-line book
//!   ([`VARIANT_LINE_BOOKS`]; the ruling's amendment: default answer "variant");
//! - each candidate's base row in the pinned tree is a plain record row, and the rows agree on
//!   the name field, the `KEY:` (else the name) and the `CATEGORY:`, compared case-insensitively;
//! - and EITHER every row states a `DESC:` and the descriptions, lower-cased with whitespace
//!   collapsed, are prefix-ordered (each shorter one is the start of the next longer one: a
//!   reprint repeats the text and may extend it; a variant restates it), OR the rows are
//!   identical token for token apart from the `SOURCE*` bookkeeping tokens (a row with no text
//!   to compare, `ag_abilities_class.lst:419` / `iswg_abilities_class.lst:151`
//!   `RMA Weapon Proficiencies`);
//! - every candidate's book has a `SOURCEDATE:`, and exactly one candidate carries the latest.
//!
//! Anything else stays ambiguous and keeps its named defect. Under the `DESC:` test, filing tokens
//! (`TYPE:`, `SOURCEPAGE:`) are not identity: a reprint re-files the object (`ism_abilities_class.lst:8`
//! `TYPE:CyphermageClassFeatures...` vs `ag_abilities_class.lst:103`
//! `TYPE:ClassFeatures.Cyphermage Class Feature...`).

use std::collections::BTreeMap;

use codex::rules_core::sheet_rule::RuleId;

use super::closure::{row_identity, tokenize_row, PinnedTree, RowRef, RowShape};
use super::ctx::RecordRef;

/// Books whose records are variants of an earlier object by default (operator amendment
/// 2026-08-16: Mythic Adventures publishes mythic versions; Pathfinder Unchained publishes
/// Unchained versions; "one does not replace the other").
pub const VARIANT_LINE_BOOKS: [&str; 2] = ["mythic_adventures", "pathfinder_unchained"];

/// The identity fields of a record's base row.
struct Identity {
    name: String,
    key: String,
    category: String,
    /// The `DESC:`, lower-cased, whitespace collapsed; `None` when absent or empty.
    desc: Option<String>,
    /// Every token but the `SOURCE*` bookkeeping ones, sorted.
    tokens: Vec<(String, String)>,
}

fn identity(tree: &PinnedTree, r: &RecordRef) -> Option<Identity> {
    let file = tree.file_index(&r.rel_path)?;
    if r.line == 0 || r.line > tree.files[file].lines.len() {
        return None;
    }
    let raw = tree.row_text(RowRef { file, line: r.line });
    let id = row_identity(raw);
    if !matches!(id.shape, RowShape::Plain) {
        return None;
    }
    let (name, tokens) = tokenize_row(raw);
    let token = |k: &str| tokens.iter().find(|(t, _)| t == k).map(|(_, v)| v.trim().to_string());
    let desc = token("DESC").map(|d| d.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ")).filter(|d| !d.is_empty());
    let name = name.trim().to_ascii_uppercase();
    let key = token("KEY").map(|k| k.to_ascii_uppercase()).unwrap_or_else(|| name.clone());
    let category = token("CATEGORY").unwrap_or_default().to_ascii_uppercase();
    let mut rest: Vec<(String, String)> = tokens.iter().filter(|(k, _)| !k.starts_with("SOURCE")).map(|(k, v)| (k.clone(), v.trim().to_string())).collect();
    rest.sort();
    Some(Identity { name, key, category, desc, tokens: rest })
}

/// The newest printing among `candidates`, when they are printings of one object (module doc);
/// `None` otherwise.
pub fn newest_printing(tree: &PinnedTree, by_id: &BTreeMap<&str, &RecordRef>, candidates: &[RuleId]) -> Option<RuleId> {
    if candidates.len() < 2 {
        return None;
    }
    let records: Vec<&RecordRef> = candidates.iter().map(|id| by_id.get(id.as_str()).copied()).collect::<Option<_>>()?;
    let kind = &records[0].kind;
    if records.iter().any(|r| &r.kind != kind || VARIANT_LINE_BOOKS.contains(&r.book.as_str())) {
        return None;
    }
    let mut printed: Vec<(&RecordRef, Identity, &String)> = Vec::new();
    for r in &records {
        let ident = identity(tree, r)?;
        let date = tree.source_dates.get(&r.book)?;
        printed.push((r, ident, date));
    }
    let first = &printed[0].1;
    if printed.iter().any(|(_, i, _)| i.name != first.name || i.key != first.key || i.category != first.category) {
        return None;
    }
    let identical = printed.iter().all(|(_, i, _)| i.tokens == first.tokens);
    let descs: Option<Vec<&String>> = printed.iter().map(|(_, i, _)| i.desc.as_ref()).collect();
    let prefix_ordered = descs.is_some_and(|mut d| {
        d.sort_by_key(|x| x.len());
        d.windows(2).all(|w| w[1].starts_with(w[0].as_str()))
    });
    if !identical && !prefix_ordered {
        return None;
    }
    let latest = printed.iter().map(|(_, _, d)| *d).max()?;
    let mut newest = printed.iter().filter(|(_, _, d)| *d == latest);
    let (winner, _, _) = newest.next()?;
    if newest.next().is_some() {
        return None;
    }
    Some(winner.id.clone())
}

/// [`newest_printing`] over every ambiguous pair; pairs with no newest printing are left out.
pub fn newest_printings(
    tree: &PinnedTree,
    by_id: &BTreeMap<&str, &RecordRef>,
    candidates: &BTreeMap<(String, String), Vec<RuleId>>,
) -> BTreeMap<(String, String), RuleId> {
    candidates.iter().filter_map(|(pair, ids)| newest_printing(tree, by_id, ids).map(|w| (pair.clone(), w))).collect()
}
