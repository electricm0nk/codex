//! AT-34-E1-008 (`docs/release/SD-34-book-completion/decisions.md §13`) --
//! additive restamp pass over EXISTING on-disk `data/corpus/<book>/**/*.json`
//! records, never a second generator, following the exact same posture
//! `enrich_class_raw_tokens.rs`/`enrich_equipment_raw_tokens.rs`/
//! `enrich_spell_raw_tokens.rs`/`enrich_companion_raw_tokens.rs` already
//! establish for this repo's "additive enrichment, not a hand-edit" pattern
//! (module docs there, reused here verbatim rather than a fifth divergent
//! copy, per `decisions.md §17`'s "search for an existing path first").
//!
//! **What went stale.** `src/pcgen_import/corpus_traps.rs::audit_ingested_cache`
//! (wired as `scripts/verify.sh --only corpus-trap-audit`, AT-34-E1-007) reads
//! every cache record's stored `wiring_class` and recomputes it FRESH from the
//! record's own cited `.lst` token closure via
//! `codex::rules_core::cache_gen::WiringClassIndex`. Some per-book kinds
//! (`companion`, `class`, `spell`, `equipment` for the books this cycle
//! touches) are produced by `gen_book_cache.rs`/`gen_core_rulebook_cache.rs`/
//! `gen_cache_apg.rs`, which already call that SAME index at generation time,
//! so re-running them re-agrees the stamp. Other kinds (`ability`, `domain`,
//! `skill`, `template`, `race_trait_generic`, `feat_generic`, `trait_generic`)
//! were ingested by one-off Python scripts (`ingest_ability.py`,
//! `ingest_generic_kind.py`, `ingest_simple_filename_kinds.py`,
//! `ingest_race_trait_generic.py`) predating GE-01's real closure
//! determinator and carrying their OWN much simpler, incompatible
//! `static`/`display`-only heuristic -- re-running those scripts can never
//! agree with the audit, because their vocabulary does not even include
//! `derived`/`computed`/`ambiguous`. This tool closes that gap for exactly
//! the field the audit checks, using the audit's own determination code,
//! never re-deriving a second classifier.
//!
//! **Byte-for-byte elsewhere, by construction.** This tool parses each
//! record as a generic `serde_json::Value`, and inserts or overwrites
//! EXACTLY two top-level keys (`wiring_class`, `wiring_class_signals`) when
//! (and only when) the freshly-computed value disagrees with what is
//! stored. Every other field -- `data` (including `raw_tokens`), `source`,
//! `license`, `pi_field`, `pi_marker`, `codex_generated_name`, `rename`,
//! `population`, `completeness`, `ingested_at` -- is left exactly as parsed,
//! so PI/license/provenance survive by construction, not by a
//! post-hoc diff. A record whose `source.kind` is not `lst_token` (no
//! citation to recompute from) is left untouched, matching
//! `audit_ingested_cache`'s own `None`-is-skipped rule.
//!
//! Usage: `cargo run --bin restamp_wiring_class -- <book> [<book> ...]`.
//! `PCGEN_CORPUS_ROOT` (default `$HOME/workspace/repos/pcgen/data`) must
//! point at the pinned oracle checkout (`scripts/pcgen-oracle-pin.env`).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::cache_gen::WiringClassIndex;
use serde_json::Value;

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// `(book_dir, file_basename)` for one citation, replicating
/// `corpus_traps.rs::audit_ingested_cache`'s own derivation EXACTLY (the
/// `core_essentials/races/` special case included, even though neither book
/// this cycle touches needs it, so this tool can never silently disagree
/// with the audit it exists to satisfy over a citation shape it did not
/// anticipate).
fn book_dir_and_basename(data_root: &Path, book: &str, rel: &str) -> (PathBuf, String) {
    const RACES_MARKER: &str = "core_essentials/races/";
    if let Some(at) = rel.find(RACES_MARKER) {
        let book_dir = data_root.join(&rel[..at + RACES_MARKER.len()]);
        let file_basename = rel[at + RACES_MARKER.len()..].to_string();
        return (book_dir, file_basename);
    }
    let book_marker = format!("/{book}/");
    if let Some(at) = rel.find(&book_marker) {
        let book_dir = data_root.join(&rel[..at + book_marker.len() - 1]);
        let file_basename = rel[at + book_marker.len()..].to_string();
        return (book_dir, file_basename);
    }
    let book_dir = Path::new(rel).parent().map(|p| data_root.join(p)).unwrap_or_else(|| data_root.to_path_buf());
    let file_basename = Path::new(rel).file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
    (book_dir, file_basename)
}

enum Outcome {
    Restamped { from: String, to: String },
    AlreadyAgrees,
    NoLstCitation,
    MissingPathOrLine,
}

fn restamp_one(
    path: &Path,
    data_root: &Path,
    book: &str,
    indexes: &mut std::collections::BTreeMap<String, WiringClassIndex>,
) -> Outcome {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut root: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?} as JSON: {e}"));

    let source = root["source"].clone();
    if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
        return Outcome::NoLstCitation;
    }
    let (Some(rel), Some(line)) = (source["path"].as_str(), source["line"].as_u64()) else {
        return Outcome::MissingPathOrLine;
    };
    let record_key = source.get("record_key").and_then(Value::as_str).unwrap_or_default().to_string();

    let (book_dir, file_basename) = book_dir_and_basename(data_root, book, rel);
    let index_key = book_dir.display().to_string();
    let index = indexes.entry(index_key).or_insert_with(|| WiringClassIndex::build(book, &book_dir));
    let mut lines = index.lines();
    let (computed_class, computed_signals) =
        index.wiring_class_for(&mut lines, &file_basename, line as u32, &record_key, &record_key);

    let stored_class = root["wiring_class"].as_str().unwrap_or_default().to_string();
    let stored_signals: Vec<String> = root["wiring_class_signals"]
        .as_array()
        .map(|a| a.iter().filter_map(|v| v.as_str()).map(str::to_string).collect())
        .unwrap_or_default();

    if stored_class == computed_class && stored_signals == computed_signals {
        return Outcome::AlreadyAgrees;
    }

    let obj = root.as_object_mut().expect("cache record must be a JSON object");
    obj.insert("wiring_class".to_string(), Value::String(computed_class.clone()));
    obj.insert(
        "wiring_class_signals".to_string(),
        Value::Array(computed_signals.iter().cloned().map(Value::String).collect()),
    );

    let new_json = serde_json::to_string_pretty(&root).expect("serialize restamped record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));
    Outcome::Restamped { from: stored_class, to: computed_class }
}

fn find_record_files(book_dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(kinds) = fs::read_dir(book_dir) else { return out };
    for kind_entry in kinds.flatten() {
        let kind_dir = kind_entry.path();
        if !kind_dir.is_dir() {
            continue;
        }
        let Ok(entries) = fs::read_dir(&kind_dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn main() {
    let books: Vec<String> = env::args().skip(1).collect();
    if books.is_empty() {
        eprintln!("usage: restamp_wiring_class <book> [<book> ...]");
        std::process::exit(1);
    }

    let data_root = pcgen_data_root();
    let corpus_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus");

    for book in &books {
        let book_dir = corpus_root.join(book);
        if !book_dir.is_dir() {
            eprintln!("restamp_wiring_class: {book_dir:?} does not exist, skipping {book}");
            continue;
        }
        let files = find_record_files(&book_dir);
        let mut indexes: std::collections::BTreeMap<String, WiringClassIndex> = std::collections::BTreeMap::new();

        let mut restamped = 0u32;
        let mut agreed = 0u32;
        let mut no_citation = 0u32;
        let mut missing = 0u32;
        let mut moves: std::collections::BTreeMap<(String, String), u32> = std::collections::BTreeMap::new();

        for file in &files {
            match restamp_one(file, &data_root, book, &mut indexes) {
                Outcome::Restamped { from, to } => {
                    restamped += 1;
                    *moves.entry((from, to)).or_insert(0) += 1;
                }
                Outcome::AlreadyAgrees => agreed += 1,
                Outcome::NoLstCitation => no_citation += 1,
                Outcome::MissingPathOrLine => missing += 1,
            }
        }

        println!(
            "restamp_wiring_class {book}: {} records scanned, {restamped} restamped, {agreed} \
             already agreed, {no_citation} no-lst-citation (untouched), {missing} missing \
             path/line (untouched)",
            files.len()
        );
        for ((from, to), n) in &moves {
            println!("  {n} x {from} -> {to}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    struct Scratch {
        dir: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let n = COUNTER.fetch_add(1, Ordering::SeqCst);
            let dir = std::env::temp_dir()
                .join(format!("codex_restamp_wiring_class_{name}_{}_{n}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            Scratch { dir }
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    /// A record whose stored `wiring_class` is `display` but whose cited
    /// row carries a real `BONUS:` token (so the fresh determinator
    /// disagrees) gets restamped to the freshly-computed class, and every
    /// OTHER field on the record survives unchanged.
    #[test]
    fn a_stale_display_record_with_a_real_bonus_token_restamps_to_static() {
        let scratch = Scratch::new("stale_display");
        let book_dir = scratch.dir.join("book");
        fs::create_dir_all(&book_dir).unwrap();
        fs::write(book_dir.join("test_abilities.lst"), "Test Ability\tBONUS:SAVE|Fort|1|TYPE=Luck\n").unwrap();

        let kind_dir = scratch.dir.join("cache").join("test_book").join("ability");
        fs::create_dir_all(&kind_dir).unwrap();
        let record_path = kind_dir.join("test_ability.json");
        let record = serde_json::json!({
            "population": "in_scope",
            "completeness": "chassis_only",
            "ingested_at": "2026-01-01T00:00:00Z",
            "data": {
                "key": "Test Ability",
                "name": "Test Ability",
                "description": null,
                "raw_tokens": [{"key": "BONUS", "value": "SAVE|Fort|1|TYPE=Luck"}]
            },
            "source": {
                "kind": "lst_token",
                "path": "book/test_abilities.lst",
                "sha256": "deadbeef",
                "line": 1,
                "record_key": "Test Ability"
            },
            "wiring_class": "display",
            "wiring_class_signals": ["display:no_magnitude_token"],
            "license": "OGL",
            "pi_field": null,
            "pi_marker": null,
            "codex_generated_name": false,
            "rename": null
        });
        fs::write(&record_path, serde_json::to_string_pretty(&record).unwrap()).unwrap();

        let mut indexes = std::collections::BTreeMap::new();
        let outcome = restamp_one(&record_path, &scratch.dir, "test_book", &mut indexes);
        match outcome {
            Outcome::Restamped { from, to } => {
                assert_eq!(from, "display");
                assert_eq!(to, "static");
            }
            _ => panic!("expected a restamp"),
        }

        let after: Value = serde_json::from_str(&fs::read_to_string(&record_path).unwrap()).unwrap();
        assert_eq!(after["wiring_class"], "static");
        // Every other field is untouched.
        assert_eq!(after["data"]["raw_tokens"], record["data"]["raw_tokens"]);
        assert_eq!(after["license"], "OGL");
        assert_eq!(after["source"], record["source"]);
        assert_eq!(after["data"]["key"], "Test Ability");
    }

    /// A record already agreeing with the fresh computation is left
    /// byte-for-byte untouched -- re-running this tool must be idempotent.
    #[test]
    fn a_record_that_already_agrees_is_not_rewritten() {
        let scratch = Scratch::new("already_agrees");
        let book_dir = scratch.dir.join("book");
        fs::create_dir_all(&book_dir).unwrap();
        fs::write(book_dir.join("test_abilities.lst"), "Test Ability\tDESC:Just flavor text\n").unwrap();

        let kind_dir = scratch.dir.join("cache").join("test_book").join("ability");
        fs::create_dir_all(&kind_dir).unwrap();
        let record_path = kind_dir.join("test_ability.json");
        let record = serde_json::json!({
            "data": {"key": "Test Ability", "name": "Test Ability"},
            "source": {
                "kind": "lst_token",
                "path": "book/test_abilities.lst",
                "line": 1,
                "record_key": "Test Ability"
            },
            "wiring_class": "display",
            "wiring_class_signals": ["display:no_magnitude_token"]
        });
        let before_text = serde_json::to_string_pretty(&record).unwrap() + "\n";
        fs::write(&record_path, &before_text).unwrap();

        let mut indexes = std::collections::BTreeMap::new();
        let outcome = restamp_one(&record_path, &scratch.dir, "test_book", &mut indexes);
        assert!(matches!(outcome, Outcome::AlreadyAgrees));
        let after_text = fs::read_to_string(&record_path).unwrap();
        assert_eq!(after_text, before_text, "file must not be rewritten when already agreeing");
    }

    /// A record with no `lst_token` source (e.g. a web second-source) is
    /// left alone, matching `audit_ingested_cache`'s own skip rule.
    #[test]
    fn a_non_lst_token_source_is_skipped() {
        let scratch = Scratch::new("non_lst");
        let kind_dir = scratch.dir.join("cache").join("test_book").join("spell");
        fs::create_dir_all(&kind_dir).unwrap();
        let record_path = kind_dir.join("web_spell.json");
        let record = serde_json::json!({
            "data": {"key": "Web Spell"},
            "source": {"kind": "web_second_source"},
            "wiring_class": "ambiguous",
            "wiring_class_signals": []
        });
        fs::write(&record_path, serde_json::to_string_pretty(&record).unwrap()).unwrap();

        let mut indexes = std::collections::BTreeMap::new();
        let outcome = restamp_one(&record_path, &scratch.dir, "test_book", &mut indexes);
        assert!(matches!(outcome, Outcome::NoLstCitation));
    }
}
