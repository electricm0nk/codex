//! OGL compliance-artifact drift guard for the two SD-27 books.
//!
//! `data/corpus/<book>/LICENSE.json` is a redistribution-compliance record:
//! it states how many of the book's licensed content records this repo
//! processed through the Product-Identity screen, and how many of them came
//! back redacted. Both numbers are consumed by a human deciding whether the
//! repo may be redistributed, so an undercount is a real defect — it claims
//! coverage of a smaller corpus than the one actually on disk.
//!
//! **The defect this guard exists to stop, verbatim from the history it was
//! written against.** `src/bin/sd27_gen_book_cache.rs` writes each book's
//! `LICENSE.json` and sets `records_processed` to *what that binary itself
//! wrote* — for Pathfinder Unchained, `feat_written + equipment_written`. A
//! later cycle ingested the same book's classes and class features through a
//! *different* binary (`src/bin/ingest_pu_classes.rs`), and ARG's alternate
//! racial traits through `src/bin/ingest_race_traits_arg.rs`. Those binaries
//! write real, licence-classified records into the same book directory and do
//! not touch `LICENSE.json`, so the stated count silently fell behind the
//! corpus: PU read 59 against 127 real records, ARG 479 against 635.
//!
//! So this test derives the count from the filesystem — never from a constant
//! here, and never from the generator — and requires the artifact to match. A
//! future book-widening cycle that adds records through any binary, or by
//! hand, fails here until it restates the compliance number.
//!
//! Derivation command this test mirrors exactly:
//!
//! ```text
//! find data/corpus/pathfinder_unchained -name '*.json' \
//!      ! -name LICENSE.json -not -path '*/_parity/*' | wc -l   # 127
//! find data/corpus/advanced_race_guide -name '*.json' \
//!      ! -name LICENSE.json -not -path '*/_parity/*' | wc -l   # 635
//! ```
//!
//! **Scope note, stated rather than left to be discovered.** Only the two
//! books this cycle owns are asserted here. `data/corpus/core_rulebook` and
//! `data/corpus/beastiary` carry the *same* staleness from the *same* cause
//! (`src/bin/ingest_races.rs` added race + race_trait records to both without
//! restating their `LICENSE.json`): core_rulebook says 3326 against 3400 real
//! records on disk, beastiary says 45 against 164. Those two artifacts are
//! outside this cycle's granted write scope, so correcting them — and
//! extending `BOOKS` below to cover them, which is a one-line change — is a
//! reported finding rather than a silent edit. `BOOKS` is deliberately the
//! only thing that needs to change when that happens.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// The books whose `LICENSE.json` this cycle owns and therefore asserts.
const BOOKS: &[&str] = &["pathfinder_unchained", "advanced_race_guide"];

/// `_parity` holds PCGen round-trip comparison fixtures (a `.pcg` character
/// file and its expected JSON), not licensed content records extracted from
/// the book. Counting them would inflate the compliance number with material
/// the PI screen never ran over.
const NON_RECORD_DIRS: &[&str] = &["_parity"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn book_dir(book: &str) -> PathBuf {
    repo_root().join("data/corpus").join(book)
}

/// Every licensed content record file on disk for one book, grouped by its
/// content-kind directory (`feat`, `class_feature`, `race_trait`, …).
///
/// Reads the directory tree; holds no list of expected kinds, so a book that
/// grows an entirely new content kind is counted without this file changing.
fn record_files_by_kind(book: &str) -> BTreeMap<String, Vec<PathBuf>> {
    let root = book_dir(book);
    let mut by_kind: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();

    let entries = fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("book corpus dir {} must be readable: {e}", root.display()));
    for entry in entries {
        let entry = entry.expect("readable dir entry");
        if !entry.path().is_dir() {
            continue;
        }
        let kind = entry.file_name().to_string_lossy().into_owned();
        if NON_RECORD_DIRS.contains(&kind.as_str()) {
            continue;
        }
        let mut files = Vec::new();
        collect_json(&entry.path(), &mut files);
        files.sort();
        by_kind.insert(kind, files);
    }

    by_kind
}

fn collect_json(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("readable dir {}: {e}", dir.display())) {
        let path = entry.expect("readable dir entry").path();
        if path.is_dir() {
            collect_json(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "json") {
            out.push(path);
        }
    }
}

fn license_json(book: &str) -> serde_json::Value {
    let path = book_dir(book).join("LICENSE.json");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} must exist and be readable: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{} must be valid JSON: {e}", path.display()))
}

fn stated_u64(license: &serde_json::Value, field: &str, book: &str) -> u64 {
    license
        .get(field)
        .and_then(serde_json::Value::as_u64)
        .unwrap_or_else(|| panic!("{book}/LICENSE.json must state an integer `{field}`"))
}

/// The compliance artifact's stated record count must equal the number of
/// licensed content records actually on disk for that book.
///
/// This is the guard proper. It fails in *both* directions: a cycle that adds
/// records without restating the number, and a number raised past a corpus
/// that shrank.
#[test]
fn every_owned_books_stated_record_count_equals_the_records_on_disk() {
    for book in BOOKS {
        let by_kind = record_files_by_kind(book);
        let on_disk: usize = by_kind.values().map(Vec::len).sum();
        let stated = stated_u64(&license_json(book), "records_processed", book) as usize;

        let breakdown: Vec<String> = by_kind
            .iter()
            .map(|(kind, files)| format!("{kind}: {}", files.len()))
            .collect();

        assert_eq!(
            stated,
            on_disk,
            "data/corpus/{book}/LICENSE.json states records_processed = {stated}, but {on_disk} \
             licensed content records are on disk ({}). This artifact is an OGL redistribution \
             record: restate the number (and the screening note that quotes it) to match the \
             corpus, rather than adjusting this test.",
            breakdown.join(", ")
        );

        assert!(
            on_disk > 0,
            "derived zero records for {book}; the derivation, not the artifact, is broken"
        );
    }
}

/// `records_redacted` must equal the records actually carrying a
/// Product-Identity redaction on disk.
///
/// The count of *redacted* records is the half of the artifact a redistributor
/// relies on most directly, and it has the same staleness exposure as
/// `records_processed`: it is written once by one generator and never revisited
/// when another binary adds records. Derived here from the records themselves.
#[test]
fn every_owned_books_stated_redaction_count_equals_the_redactions_on_disk() {
    for book in BOOKS {
        let mut redacted_paths = Vec::new();
        for files in record_files_by_kind(book).values() {
            for path in files {
                let record: serde_json::Value =
                    serde_json::from_str(&fs::read_to_string(path).expect("readable record"))
                        .unwrap_or_else(|e| panic!("{} must be valid JSON: {e}", path.display()));
                let license = record.get("license").and_then(serde_json::Value::as_str);
                if license == Some("PI_REDACTED") || record.get("pi_marker").is_some_and(|m| !m.is_null()) {
                    redacted_paths.push(path.clone());
                }
            }
        }
        let stated = stated_u64(&license_json(book), "records_redacted", book) as usize;
        assert_eq!(
            stated,
            redacted_paths.len(),
            "data/corpus/{book}/LICENSE.json states records_redacted = {stated}, but {} records \
             on disk carry a PI redaction: {:?}",
            redacted_paths.len(),
            redacted_paths.iter().take(4).collect::<Vec<_>>()
        );
    }
}

/// Every record the artifact counts must actually carry a licence
/// classification.
///
/// Without this, `records_processed` could match a directory full of records
/// the PI screen never ran over — the count would be right and the claim it
/// stands for ("this many records were processed through the screen") would
/// still be false.
#[test]
fn every_counted_record_carries_a_real_license_classification() {
    for book in BOOKS {
        let mut unclassified = Vec::new();
        for files in record_files_by_kind(book).values() {
            for path in files {
                let record: serde_json::Value =
                    serde_json::from_str(&fs::read_to_string(path).expect("readable record"))
                        .unwrap_or_else(|e| panic!("{} must be valid JSON: {e}", path.display()));
                match record.get("license").and_then(serde_json::Value::as_str) {
                    Some("OGL") | Some("PI_REDACTED") => {}
                    _ => unclassified.push(path.clone()),
                }
            }
        }
        assert!(
            unclassified.is_empty(),
            "{} record(s) counted in {book}'s LICENSE.json carry no OGL/PI_REDACTED \
             classification: {:?}",
            unclassified.len(),
            unclassified.iter().take(4).collect::<Vec<_>>()
        );
    }
}

/// The prose `screening_method_note` quotes the record count in words. A
/// number corrected in the field but not in the note leaves the artifact
/// self-contradicting, which is how the original 59 survived: the note said
/// "this book's 59 records (17 feats + 42 equipment modifiers)" and read as
/// deliberate.
#[test]
fn the_screening_note_quotes_the_same_count_the_field_states() {
    for book in BOOKS {
        let license = license_json(book);
        let stated = stated_u64(&license, "records_processed", book);
        let note = license
            .get("screening_method_note")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_else(|| panic!("{book}/LICENSE.json must carry a screening_method_note"));

        assert!(
            note.contains(&stated.to_string()),
            "{book}/LICENSE.json states records_processed = {stated}, but its \
             screening_method_note never mentions that number — the artifact contradicts \
             itself. Note: {note}"
        );
    }
}
