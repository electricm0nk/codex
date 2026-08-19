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
//! written against.** `src/bin/gen_book_cache.rs` writes each book's
//! `LICENSE.json` and sets `records_processed` to *what that binary itself
//! wrote* — for Pathfinder Unchained, `feat_written + equipment_written`. A
//! later cycle ingested the same book's classes and class features through a
//! *different* binary (`src/bin/ingest_pu_classes.rs`), and ARG's alternate
//! racial traits through `src/bin/ingest_race_traits.rs`. Those binaries
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
//! Both figures are the ones measured when this file was written and are kept
//! as the record of that measurement, not as current truth: ARG is 649 as of
//! SD-29 Epic 7 round 9, which added the book's 14 companion records. Nothing
//! in this file reads either number — every count it asserts is derived from
//! the files on disk at test time, which is the property that makes the
//! narrative above safe to leave standing.
//!
//! # Why the guard did not catch core_rulebook and beastiary
//!
//! It was asked not to. The previous revision of this file carried
//! `const BOOKS: &[&str] = &["pathfinder_unchained", "advanced_race_guide"]`
//! and a scope note recording — accurately, in full, and with both real
//! numbers — that `core_rulebook` said 3326 against 3400 records on disk and
//! `beastiary` said 45 against 164, from the same cause
//! (`src/bin/ingest_races.rs` added race + race_trait records to both books on
//! 2026-07-31, four days after `bb497db0` last wrote either `LICENSE.json`,
//! and that binary does not touch the compliance artifact). Those two files
//! were outside that cycle's write scope, so the defect was reported instead
//! of silently edited. That was the right call. What it left behind was a
//! guard whose coverage was a **hand-maintained constant** — so the drift it
//! existed to stop was invisible to it *by construction*, on exactly the two
//! books that were drifting.
//!
//! **The fix is to stop hand-maintaining the list.** `books_on_disk()` below
//! derives the covered set from the filesystem: every `data/corpus/<book>/`
//! that ships a `LICENSE.json` is asserted. A seventh book cannot be added
//! without this guard covering it, and no future cycle has to remember to
//! extend anything.
//!
//! The same flaw had a second instance in this file, found while fixing the
//! first and fixed the same way. Both remaining hardcoded literals were the
//! string `"PI_REDACTED"` — a value the schema **cannot emit**.
//! `shape_b_v1::License::PiRedacted` serialises as `"PI-REDACTED"` (hyphen;
//! `shape_b_v1.rs:122`), and `advanced_class_guide/spell/discern_next_of_kin.json`
//! is the one record on disk in that state. Under the old `BOOKS` that book
//! was never read, so the mismatch was inert; the moment coverage widened,
//! `every_counted_record_carries_a_real_license_classification` would have
//! failed a correctly-classified record. Both literals are now derived by
//! serialising the enum itself, so the test cannot disagree with the schema.
//!
//! Two books — `advanced_class_guide` and `advanced_players_guide` — shipped a
//! `LICENSE.json` that stated no `records_processed` at all. That was a real
//! third gap, and it is **pinned rather than papered over** by
//! [`exactly_the_one_known_book_omits_a_stated_record_count`]: a new book
//! omitting the field fails, and a listed book gaining it also fails, forcing
//! the exemption list to shrink.
//!
//! **It shrank.** SD-29 Epic 7 round 9 ingested the Advanced Player's Guide's
//! `companion` family, and `gen_book_cache`'s companion generator writes a
//! derived `records_processed` into the book's `LICENSE.json` as a matter of
//! course. So APG now states its count — **646**, the real on-disk file count
//! — and it has been moved OUT of the exemption list into this file's ordinary
//! count coverage, which is exactly the direction the pin exists to force.
//! `advanced_class_guide` is the last book still exempt.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::shape_b_v1::License;

/// The `license` wire values a record may legitimately carry, taken from the
/// schema enum rather than retyped. `License::Pi` is deliberately included:
/// it is a state `shape_b_v1` defines and `validate_license` accepts, so a
/// record in it is classified, not unclassified — whether it *should* ship
/// that way is `validate_license`'s question, not this artifact's.
fn classified_license_values() -> BTreeSet<String> {
    [License::Ogl, License::Pi, License::PiRedacted]
        .into_iter()
        .map(wire_value)
        .collect()
}

/// One `License` variant's exact serialised form. Derived, so a rename of the
/// wire string in `shape_b_v1.rs` moves this test with it instead of silently
/// making it assert a string nothing produces.
fn wire_value(license: License) -> String {
    serde_json::to_value(license)
        .expect("License serialises")
        .as_str()
        .expect("License serialises to a string")
        .to_owned()
}

/// The books this guard covers: every `data/corpus/<book>/` that ships a
/// `LICENSE.json`.
///
/// Derived from the filesystem, never listed. This is the whole repair — the
/// previous hand-maintained constant is exactly why core_rulebook's 3326 and
/// beastiary's 45 could sit stale under a green suite.
fn books_on_disk() -> Vec<String> {
    let root = repo_root().join("data/corpus");
    let mut books: Vec<String> = fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("{} must be readable: {e}", root.display()))
        .map(|entry| entry.expect("readable dir entry").path())
        .filter(|path| path.join("LICENSE.json").is_file())
        .map(|path| path.file_name().expect("book dir has a name").to_string_lossy().into_owned())
        .collect();
    books.sort();
    assert!(
        books.len() >= 6,
        "derived only {} books with a LICENSE.json; the derivation, not the corpus, is broken",
        books.len()
    );
    books
}

/// Books whose `LICENSE.json` states no `records_processed`.
///
/// Classified before the field existed and not revisited since. Pinned as an
/// exact set by [`exactly_the_one_known_book_omits_a_stated_record_count`] so
/// the exemption cannot quietly grow, and so closing it forces this list down.
///
/// **Down to one.** `advanced_players_guide` left this list in SD-29 Epic 7
/// round 9: ingesting its `companion` family made `gen_book_cache` rewrite the
/// book's `LICENSE.json` with a derived `records_processed` of 646, and the
/// count guard now covers that book like any other. The pin is what surfaced
/// it — the round's ingest turned this test red, which is the whole point of
/// asserting an exact set rather than a floor.
const BOOKS_WITHOUT_A_STATED_RECORD_COUNT: &[&str] = &["advanced_class_guide"];

/// The books this file's count assertions actually run over: everything on
/// disk that claims a number, which is the only thing a number can be checked
/// against.
fn books_stating_a_record_count() -> Vec<String> {
    books_on_disk()
        .into_iter()
        .filter(|book| !BOOKS_WITHOUT_A_STATED_RECORD_COUNT.contains(&book.as_str()))
        .collect()
}

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
/// EVERY book is reported, not only the first one that disagrees.
///
/// CONFIRMED finding, `SD31-W14-INTEGRATE-001` (adversarial review of
/// `SD31-CE-COMPANION-001`): the prior form ran `assert_eq!` inside the loop,
/// so it panicked on the first mismatching book and every later book went
/// unchecked. `advanced_players_guide` sorts before `advanced_race_guide`, and
/// its own 2735-vs-2743 drift HID ARG's 1514-vs-1578 drift for an entire wave.
/// A guard that stops at the first failure is a guard that can only ever tell
/// you about one book. Accumulate, then assert once.
#[test]
fn every_owned_books_stated_record_count_equals_the_records_on_disk() {
    let mut mismatches: Vec<String> = Vec::new();
    let mut empty: Vec<String> = Vec::new();
    for book in books_stating_a_record_count() {
        let book = book.as_str();
        let by_kind = record_files_by_kind(book);
        let on_disk: usize = by_kind.values().map(Vec::len).sum();
        let stated = stated_u64(&license_json(book), "records_processed", book) as usize;

        let breakdown: Vec<String> = by_kind
            .iter()
            .map(|(kind, files)| format!("{kind}: {}", files.len()))
            .collect();

        if stated != on_disk {
            mismatches.push(format!(
                "{book}: LICENSE.json states records_processed = {stated}, {on_disk} on disk ({})",
                breakdown.join(", ")
            ));
        }
        if on_disk == 0 {
            empty.push(book.to_string());
        }
    }

    assert!(
        mismatches.is_empty(),
        "{} book(s) state a records_processed that does not match the records on disk. This \
         artifact is an OGL redistribution record: restate the number (and the screening note \
         that quotes it) to match the corpus, rather than adjusting this test.\n  {}",
        mismatches.len(),
        mismatches.join("\n  ")
    );
    assert!(
        empty.is_empty(),
        "derived zero records for {empty:?}; the derivation, not the artifact, is broken"
    );
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
    let redacted = wire_value(License::PiRedacted);
    // Accumulated and asserted once, for the same reason
    // `every_owned_books_stated_record_count_equals_the_records_on_disk`
    // accumulates: a per-book `assert_eq!` masks every later book.
    let mut mismatches: Vec<String> = Vec::new();
    for book in books_stating_a_record_count() {
        let book = book.as_str();
        let mut redacted_paths = Vec::new();
        for files in record_files_by_kind(book).values() {
            for path in files {
                let record: serde_json::Value =
                    serde_json::from_str(&fs::read_to_string(path).expect("readable record"))
                        .unwrap_or_else(|e| panic!("{} must be valid JSON: {e}", path.display()));
                let license = record.get("license").and_then(serde_json::Value::as_str);
                if license == Some(redacted.as_str())
                    || record.get("pi_marker").is_some_and(|m| !m.is_null())
                {
                    redacted_paths.push(path.clone());
                }
            }
        }
        let stated = stated_u64(&license_json(book), "records_redacted", book) as usize;
        if stated != redacted_paths.len() {
            mismatches.push(format!(
                "{book}: LICENSE.json states records_redacted = {stated}, {} records on disk \
                 carry a PI redaction: {:?}",
                redacted_paths.len(),
                redacted_paths.iter().take(4).collect::<Vec<_>>()
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "{} book(s) state a records_redacted that does not match the redactions on disk:\n  {}",
        mismatches.len(),
        mismatches.join("\n  ")
    );
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
    let classified = classified_license_values();
    // Every book on disk, not only the ones stating a count: a record's
    // classification is checkable without any number in the artifact, so the
    // two books missing `records_processed` are still covered here.
    for book in books_on_disk() {
        let book = book.as_str();
        let mut unclassified = Vec::new();
        for files in record_files_by_kind(book).values() {
            for path in files {
                let record: serde_json::Value =
                    serde_json::from_str(&fs::read_to_string(path).expect("readable record"))
                        .unwrap_or_else(|e| panic!("{} must be valid JSON: {e}", path.display()));
                match record.get("license").and_then(serde_json::Value::as_str) {
                    Some(value) if classified.contains(value) => {}
                    _ => unclassified.push(path.clone()),
                }
            }
        }
        assert!(
            unclassified.is_empty(),
            "{} record(s) counted in {book}'s LICENSE.json carry no {classified:?} \
             classification: {:?}",
            unclassified.len(),
            unclassified.iter().take(4).collect::<Vec<_>>()
        );
    }
}

/// The exemption list is exactly the two books known to omit the field.
///
/// Two-directional on purpose. A newly-added book whose `LICENSE.json` states
/// no `records_processed` fails here rather than slipping past the count guard
/// unnoticed — which is the same failure mode, one level up, as the
/// hand-maintained `BOOKS` constant this file used to carry. And the day
/// `advanced_class_guide` or `advanced_players_guide` gains the field, this
/// fails too, forcing `BOOKS_WITHOUT_A_STATED_RECORD_COUNT` to shrink and the
/// count guard to widen.
#[test]
fn exactly_the_one_known_book_omits_a_stated_record_count() {
    let omitting: BTreeSet<String> = books_on_disk()
        .into_iter()
        .filter(|book| license_json(book).get("records_processed").is_none())
        .collect();
    let expected: BTreeSet<String> = BOOKS_WITHOUT_A_STATED_RECORD_COUNT
        .iter()
        .map(|book| (*book).to_owned())
        .collect();

    assert_eq!(
        omitting, expected,
        "the set of books whose LICENSE.json states no `records_processed` changed. Either a new \
         book shipped a compliance artifact without the count (state it), or a known omission was \
         filled in (remove it from BOOKS_WITHOUT_A_STATED_RECORD_COUNT so the count guard covers \
         that book)."
    );
}

/// Every book on disk is covered by this file, one way or the other.
///
/// The guard against the guard. `books_on_disk()` derives coverage, and this
/// asserts the derivation actually reaches the whole corpus — so "which books
/// does this test check?" has an answer that cannot drift away from "all of
/// them" the way the old constant did.
#[test]
fn no_book_on_disk_escapes_this_files_coverage() {
    let all: BTreeSet<String> = books_on_disk().into_iter().collect();
    let counted: BTreeSet<String> = books_stating_a_record_count().into_iter().collect();
    let exempt: BTreeSet<String> = BOOKS_WITHOUT_A_STATED_RECORD_COUNT
        .iter()
        .map(|book| (*book).to_owned())
        .collect();

    assert_eq!(
        all,
        counted.union(&exempt).cloned().collect::<BTreeSet<String>>(),
        "a book on disk is in neither the count-guarded set nor the recorded-exemption set"
    );
    assert!(
        exempt.is_subset(&all),
        "BOOKS_WITHOUT_A_STATED_RECORD_COUNT names a book that is not on disk: {:?}",
        exempt.difference(&all).collect::<Vec<_>>()
    );
}

/// The prose `screening_method_note` quotes the record count in words. A
/// number corrected in the field but not in the note leaves the artifact
/// self-contradicting, which is how the original 59 survived: the note said
/// "this book's 59 records (17 feats + 42 equipment modifiers)" and read as
/// deliberate.
#[test]
fn the_screening_note_quotes_the_same_count_the_field_states() {
    for book in books_stating_a_record_count() {
        let book = book.as_str();
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
