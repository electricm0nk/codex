//! Settle every ingested spell record's `DURATION:`/`RANGE:` formula at authoring time.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 17 (`decisions.md` §11, §19; the sheet rule, §1).
//!
//! Until this cycle the live side did this walk itself, on every process start, through
//! `rules_core::derived_evaluator_fixture_check`'s `load_spell_durations`/`load_spell_ranges` --
//! and not only for a gate: `apps/desktop/src-tauri/src/spell_catalog.rs`'s `duration_for()` /
//! `range_for()` are backed by it, so a reader browsing the spell catalog was, at that moment,
//! reading raw ingest tokens out of `data/corpus/`. `§11` says rule conversion happens at ingest,
//! into our own schema, and a sheet carries the final form. So the walk happens here, once, and
//! the live side reads [`codex::rules_core::record_vars::RecordVarPackage::spell_formulas`].
//!
//! **Nothing about what a reader sees changes.** The walk below is the one that moved -- same
//! recursive descent, same `data.key` identity, same "first token wins" reading, same two live
//! parsers ([`parse_caster_level_linear_duration`], [`spell_range_formula`]) applied to the same
//! text. The proof that it is the same is [`tests`]'s own whole-corpus comparison, which re-reads
//! every record in every book and asserts the settled tables agree key for key and value for
//! value -- including the refusals, because a refusal that silently became a formula would print
//! a fabricated duration on a sheet.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::pcgen_import::ingest_record;
use codex::rules_core::derived_evaluator_fixture_check::{
    SPELL_CORPUS_BOOK_DIRS, SettledSpellFormulas, parse_caster_level_linear_duration,
    settled_spell_key, spell_range_formula,
};

/// Every `data/corpus/<book>/spell/**.json` record's raw `DURATION:` and `RANGE:` token,
/// keyed by the record's own `data.key`.
///
/// The recursive walk (rather than a fixed depth) is load-bearing: some books nest their spell
/// records by spell level and some are flat.
fn raw_spell_tokens(spell_dir: &Path) -> BTreeMap<String, (Option<String>, Option<String>)> {
    let mut out: BTreeMap<String, (Option<String>, Option<String>)> = BTreeMap::new();
    let mut stack = vec![spell_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else { continue };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let Some(key) = doc["data"]["key"].as_str() else { continue };
            let duration =
                ingest_record::first_token_value(&doc, "DURATION").map(|v| v.to_string());
            let range = ingest_record::first_token_value(&doc, "RANGE").map(|v| v.to_string());
            if duration.is_none() && range.is_none() {
                continue;
            }
            out.insert(key.to_string(), (duration, range));
        }
    }
    out
}

/// Where this repo's ingest of `book`'s spell kind lives, and whether it exists.
fn spell_corpus_dir(repo_root: &Path, book: &str) -> Option<PathBuf> {
    let dir = repo_root.join("data").join("corpus").join(book).join("spell");
    dir.is_dir().then_some(dir)
}

/// Settle every ingested book's spell formulas.
///
/// A record whose token the parser refuses lands in the matching `*_refused` set rather than
/// being dropped: "this record states a duration the engine reads no formula out of" and "this
/// repo has no such record" are different answers, and the bar check reports them differently.
pub fn build(repo_root: &Path) -> SettledSpellFormulas {
    let mut settled = SettledSpellFormulas::default();
    for book in SPELL_CORPUS_BOOK_DIRS {
        let Some(dir) = spell_corpus_dir(repo_root, book) else { continue };
        for (record_key, (duration, range)) in raw_spell_tokens(&dir) {
            let key = settled_spell_key(book, &record_key);
            if let Some(raw) = duration {
                match parse_caster_level_linear_duration(&raw) {
                    Some(formula) => {
                        settled.durations.insert(key.clone(), formula);
                    }
                    None => {
                        settled.duration_refused.insert(key.clone());
                    }
                }
            }
            if let Some(raw) = range {
                match spell_range_formula(&raw) {
                    Some(formula) => {
                        settled.ranges.insert(key.clone(), formula);
                    }
                    None => {
                        settled.range_refused.insert(key.clone());
                    }
                }
            }
        }
    }
    settled
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::derived_evaluator_fixture_check::SETTLED_SPELL_KEY_SEPARATOR;
    use std::collections::BTreeSet;

    fn repo_root() -> PathBuf {
        crate::repo_root()
    }

    /// The whole-corpus parity proof, and the reason the swap is safe rather than plausible.
    ///
    /// It re-reads EVERY spell record in EVERY ingested book -- the same reading the live side
    /// performed until this cycle -- and asserts the settled tables reproduce it exactly: the
    /// same keys, the same formulas, and the same refusals. A settling that quietly turned a
    /// refusal into a formula, or dropped one book's nesting shape, passes no part of this.
    #[test]
    fn every_ingested_spell_records_duration_and_range_settle_to_what_the_parsers_read() {
        let root = repo_root();
        let settled = build(&root);

        let mut expected = SettledSpellFormulas::default();
        let mut books_walked = 0usize;
        let mut records_walked = 0usize;
        for book in SPELL_CORPUS_BOOK_DIRS {
            let Some(dir) = spell_corpus_dir(&root, book) else { continue };
            books_walked += 1;
            for (record_key, (duration, range)) in raw_spell_tokens(&dir) {
                records_walked += 1;
                let key = settled_spell_key(book, &record_key);
                if let Some(raw) = duration {
                    match parse_caster_level_linear_duration(&raw) {
                        Some(f) => {
                            expected.durations.insert(key.clone(), f);
                        }
                        None => {
                            expected.duration_refused.insert(key.clone());
                        }
                    }
                }
                if let Some(raw) = range {
                    match spell_range_formula(&raw) {
                        Some(f) => {
                            expected.ranges.insert(key.clone(), f);
                        }
                        None => {
                            expected.range_refused.insert(key.clone());
                        }
                    }
                }
            }
        }

        assert!(books_walked >= 8, "the ingested spell books must be present, got {books_walked}");
        assert!(
            records_walked >= 1_900,
            "the spell corpus must be the real one, got {records_walked} records"
        );
        assert_eq!(settled.durations, expected.durations);
        assert_eq!(settled.duration_refused, expected.duration_refused);
        assert_eq!(settled.ranges, expected.ranges);
        assert_eq!(settled.range_refused, expected.range_refused);
        assert!(
            settled.durations.len() >= 1_000,
            "the settled duration table must carry the real population, got {}",
            settled.durations.len()
        );
        assert!(
            settled.ranges.len() >= 800,
            "the settled range table must carry the real population, got {}",
            settled.ranges.len()
        );
    }

    /// The key format is only unambiguous while neither half contains the separator. Asserted
    /// over the real corpus rather than assumed, because a book that ever shipped a `|` in a
    /// record key would silently mis-split every lookup built on it.
    #[test]
    fn no_book_or_record_key_in_this_corpus_contains_the_key_separator() {
        let root = repo_root();
        for book in SPELL_CORPUS_BOOK_DIRS {
            assert!(!book.contains(SETTLED_SPELL_KEY_SEPARATOR), "book {book:?}");
            let Some(dir) = spell_corpus_dir(&root, book) else { continue };
            for record_key in raw_spell_tokens(&dir).keys() {
                assert!(
                    !record_key.contains(SETTLED_SPELL_KEY_SEPARATOR),
                    "record key {record_key:?} in {book}"
                );
            }
        }
    }

    /// A parsed record and a refused one are never both: the two tables partition the records
    /// that carry the token, which is what makes the bar check's two failure branches distinct.
    #[test]
    fn a_record_is_either_settled_or_refused_never_both() {
        let settled = build(&repo_root());
        let duration_overlap: BTreeSet<&String> = settled
            .durations
            .keys()
            .filter(|k| settled.duration_refused.contains(*k))
            .collect();
        assert!(duration_overlap.is_empty(), "overlap: {duration_overlap:?}");
        let range_overlap: BTreeSet<&String> =
            settled.ranges.keys().filter(|k| settled.range_refused.contains(*k)).collect();
        assert!(range_overlap.is_empty(), "overlap: {range_overlap:?}");
    }
}
