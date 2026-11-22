//! Settle the class-feature pool's three ingest-token gates at authoring time.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 18 (`decisions.md` §11, §19; the sheet rule, §1).
//!
//! Until this cycle `rules_core::class_feature_pool_catalog`'s shipping walk asked
//! [`crate::pcgen_import::pool_member_tokens`]'s four predicates about every corpus row it
//! considered, on every process start, on the way to a character sheet. Each of those
//! predicates is a question about the **ingest format** -- which token keys the row carries,
//! how many `DESC:` fields it has, whether a `PREABILITY` names `CATEGORY=Archetype` -- and
//! `§11` says a question like that is answered once, at ingest, into our own schema. So the walk
//! happens here, and the live side reads
//! [`codex::rules_core::record_vars::RecordVarPackage::pool_gates`].
//!
//! **Nothing a reader sees changes.** The three gates below are applied in the order the catalog
//! applied them, to the same `data` object, through the same four predicate functions -- they
//! are not restated here. The proof is [`tests`]'s whole-corpus comparison, which re-reads every
//! `class_feature` record in every ingested book the way the catalog read it and asserts the
//! settled table reproduces the verdict record for record, refusals and reasons included. A
//! settling that quietly admitted one refused record would put a mechanic-bearing option on a
//! sheet as prose-only, which is precisely what these gates exist to prevent.
//!
//! The table is fail-closed by construction: it carries only the records it actually walked, and
//! [`SettledPoolGates::admits`] is false for everything else.

use std::path::{Path, PathBuf};

use crate::pcgen_import::pool_member_tokens;
use codex::rules_core::class_feature_pool_catalog::{settled_pool_gate_key, SettledPoolGates};

/// The catalog's own bucket name for a record refused by the engine-effect-token gate.
pub const REFUSAL_ENGINE_EFFECT_TOKEN: &str = "engine_effect_token_present";
/// The catalog's own bucket name for a record refused by the archetype-lock gate.
pub const REFUSAL_ARCHETYPE_LOCKED: &str = "archetype_locked";
/// The catalog's own bucket name for a record refused by the multi-`DESC:` gate.
pub const REFUSAL_MULTI_DESC_SEGMENT: &str = "multi_desc_segment_not_regenerated";

/// The verdict the three gates reach for one corpus record.
///
/// `None` means every gate admitted it; `Some(reason)` names the first that did not, in the
/// order the catalog asks them.
pub fn verdict(data: &serde_json::Value, raw_desc: &str) -> Option<&'static str> {
    if !pool_member_tokens::has_no_engine_effect_token(data) {
        return Some(REFUSAL_ENGINE_EFFECT_TOKEN);
    }
    if pool_member_tokens::is_archetype_locked(data) {
        return Some(REFUSAL_ARCHETYPE_LOCKED);
    }
    if pool_member_tokens::carries_more_than_one_desc_segment(data)
        && !pool_member_tokens::shipped_description_is_the_already_regenerated_safe_multi_desc_join(
            data, raw_desc,
        )
    {
        return Some(REFUSAL_MULTI_DESC_SEGMENT);
    }
    None
}

/// Every `.json` file under `dir`, recursively. Books nest their class-feature records by
/// group and some do not, so the descent cannot assume a fixed depth.
fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(read_dir) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = read_dir.flatten().collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            json_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
}

/// Every ingested book directory that has a `class_feature` kind, in sorted order.
fn class_feature_dirs(repo_root: &Path) -> Vec<(String, PathBuf)> {
    let corpus_root = repo_root.join("data").join("corpus");
    let Ok(books) = std::fs::read_dir(&corpus_root) else { return Vec::new() };
    let mut book_dirs: Vec<_> = books.flatten().collect();
    book_dirs.sort_by_key(std::fs::DirEntry::file_name);
    let mut out = Vec::new();
    for book_entry in book_dirs {
        let book_dir = book_entry.path();
        if !book_dir.is_dir() {
            continue;
        }
        let cf_dir = book_dir.join("class_feature");
        if !cf_dir.is_dir() {
            continue;
        }
        out.push((book_entry.file_name().to_string_lossy().to_string(), cf_dir));
    }
    out
}

/// Settle the three gates over every ingested book's `class_feature` records.
///
/// A record with no `data.key`, or with no `description` string, is not walked: the catalog
/// refuses it earlier, before the gates are asked, so the table has nothing true to say about
/// it and says nothing. `admits` is false for it either way.
pub fn build(repo_root: &Path) -> SettledPoolGates {
    let mut settled = SettledPoolGates::default();
    for (book, cf_dir) in class_feature_dirs(repo_root) {
        let mut files = Vec::new();
        json_files(&cf_dir, &mut files);
        for file in files {
            let Ok(text) = std::fs::read_to_string(&file) else { continue };
            let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let data = &doc["data"];
            let Some(record_key) = data["key"].as_str() else { continue };
            let Some(raw_desc) = data["description"].as_str() else { continue };
            let key = settled_pool_gate_key(&book, record_key);
            match verdict(data, raw_desc) {
                Some(reason) => {
                    settled.refused.insert(key, reason.to_string());
                }
                None => {
                    settled.admitted.insert(key);
                }
            }
        }
    }
    settled
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        crate::repo_root()
    }

    /// The whole-corpus parity proof, and the reason the swap is safe rather than plausible.
    ///
    /// It re-reads EVERY `class_feature` record in EVERY ingested book -- the same reading the
    /// shipping walk performed until this cycle -- and asserts the settled table reproduces the
    /// verdict exactly: the same admitted keys, the same refused keys, and the same reason on
    /// each refusal. A settling that admitted one record these gates refuse would put a
    /// mechanic-bearing or archetype-locked option on a sheet as prose-only.
    #[test]
    fn every_ingested_class_feature_records_gate_verdict_settles_to_what_the_predicates_read() {
        let root = repo_root();
        let settled = build(&root);

        let mut expected = SettledPoolGates::default();
        let mut books_walked = 0usize;
        let mut records_walked = 0usize;
        for (book, cf_dir) in class_feature_dirs(&root) {
            books_walked += 1;
            let mut files = Vec::new();
            json_files(&cf_dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                let data = &doc["data"];
                let Some(record_key) = data["key"].as_str() else { continue };
                let Some(raw_desc) = data["description"].as_str() else { continue };
                records_walked += 1;
                let key = settled_pool_gate_key(&book, record_key);
                // Asked through the converter's predicates directly, in the order the catalog
                // asked them, rather than through `verdict` -- so this is a check on the
                // settling, not a restatement of it.
                if !pool_member_tokens::has_no_engine_effect_token(data) {
                    expected.refused.insert(key, REFUSAL_ENGINE_EFFECT_TOKEN.to_string());
                } else if pool_member_tokens::is_archetype_locked(data) {
                    expected.refused.insert(key, REFUSAL_ARCHETYPE_LOCKED.to_string());
                } else if pool_member_tokens::carries_more_than_one_desc_segment(data)
                    && !pool_member_tokens::shipped_description_is_the_already_regenerated_safe_multi_desc_join(
                        data, raw_desc,
                    )
                {
                    expected.refused.insert(key, REFUSAL_MULTI_DESC_SEGMENT.to_string());
                } else {
                    expected.admitted.insert(key);
                }
            }
        }

        // Floors below the real population, so the proof cannot pass vacuously on a tree that
        // lost its corpus. Re-derive with:
        //   ls data/corpus/*/class_feature -d | wc -l
        assert!(books_walked >= 8, "books_walked={books_walked}");
        assert!(records_walked >= 10_000, "records_walked={records_walked}");

        assert_eq!(
            settled.admitted, expected.admitted,
            "settled admissions must reproduce the predicates' own verdict"
        );
        assert_eq!(
            settled.refused, expected.refused,
            "settled refusals, and their reasons, must reproduce the predicates' own verdict"
        );
    }

    /// A record is admitted or refused, never both, which is what keeps `admits` and `refusal`
    /// from disagreeing about the same key.
    #[test]
    fn a_record_is_either_admitted_or_refused_never_both() {
        let settled = build(&repo_root());
        for key in settled.refused.keys() {
            assert!(
                !settled.admitted.contains(key),
                "{key} is both admitted and refused"
            );
        }
        assert!(!settled.admitted.is_empty(), "the admitted set must not be empty");
        assert!(!settled.refused.is_empty(), "the refused set must not be empty");
    }

    /// The key format's own precondition, asserted over the real corpus rather than assumed:
    /// no book directory and no record key contains the separator, so
    /// `settled_pool_gate_key` is injective over this corpus.
    #[test]
    fn no_book_or_record_key_in_this_corpus_contains_the_key_separator() {
        use codex::rules_core::class_feature_pool_catalog::SETTLED_POOL_GATE_KEY_SEPARATOR;
        let root = repo_root();
        for (book, cf_dir) in class_feature_dirs(&root) {
            assert!(
                !book.contains(SETTLED_POOL_GATE_KEY_SEPARATOR),
                "book {book} contains the key separator"
            );
            let mut files = Vec::new();
            json_files(&cf_dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                let Some(record_key) = doc["data"]["key"].as_str() else { continue };
                assert!(
                    !record_key.contains(SETTLED_POOL_GATE_KEY_SEPARATOR),
                    "record key {record_key} in {book} contains the key separator"
                );
            }
        }
    }

    /// The fail-closed direction, stated as a test: a key the table never walked is not
    /// admitted, so a missing or stale artifact serves fewer options rather than unvetted ones.
    #[test]
    fn an_unknown_key_is_not_admitted() {
        let settled = build(&repo_root());
        assert!(!settled.admits("core_rulebook", "No Such Record ~ At All"));
        assert!(!settled.knows("core_rulebook", "No Such Record ~ At All"));
        assert_eq!(settled.refusal("core_rulebook", "No Such Record ~ At All"), None);
    }
}
