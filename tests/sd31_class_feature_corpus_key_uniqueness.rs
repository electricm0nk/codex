//! SD-31 wave 23 integration fix — corpus key-uniqueness ratchet for
//! `class_feature`.
//!
//! # Why this exists
//!
//! Wave 23's `class-field-fix` lane (`src/rules_core/cache_gen/class_feature.rs`
//! `true_class_by_key`, `OPEN-ISSUES.md` row 334) regenerated all 12,481
//! `class_feature` corpus records and, while every headline claim about the
//! `class` field correction reproduced exactly, the wave-23 integration
//! review found the regen had also **silently destroyed two records**:
//! `core_rulebook`'s "Draconic Bloodline" (no `~` in its key) and
//! `adventurers_guide`'s "Enlightened Bloodrager ~ Bloodline Feat" (no `~ AG`
//! suffix) were each overwritten with a second copy of an unrelated,
//! similarly-named sibling record, because both members of each pair share a
//! directory (`class_dir_slug`) and a display `name`, and whichever unit was
//! missing from that run's input list left the collision-avoidance `used`
//! slug-set never seeded — so the survivor claimed the FIRST slot instead of
//! the second, silently overwriting the first record's file rather than
//! erroring. `monster_chassis.rs` and `companion_chassis.rs` both carry a
//! `keys_are_unique_within_every_book` gate for exactly this failure shape;
//! `class_feature` had none. This is that gate, generalized to the whole
//! shipped `class_feature` corpus tree rather than one in-memory table,
//! because `class_feature` (unlike monster/companion) has no in-memory
//! roster to check -- the corpus JSON tree on disk *is* the table.
//!
//! # What this does NOT cover
//!
//! This only catches the two SHAPES the wave-23 defect actually took: (1) a
//! `(book, data.key)` pair silently colliding (one record's file overwritten
//! by another's content), and (2) the corpus-wide `raw_tokens` token count
//! dropping below a pinned floor (content loss even when no key collides,
//! e.g. a record truncated in place). It does not re-derive `true_class`
//! correctness itself -- that is `class_feature.rs`'s own
//! `true_class_by_key_*` unit tests' job -- and it does not detect a record
//! that is dropped *and* whose slug slot is never reused by anything else
//! (a pure count regression with no collision), which the `raw_tokens` floor
//! below is a coarse, not exact, proxy for.

use std::path::PathBuf;

fn cache_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus")
}

fn class_feature_files() -> Vec<PathBuf> {
    let mut out = Vec::new();
    let root = cache_dir();
    let Ok(books) = std::fs::read_dir(&root) else {
        panic!("data/corpus is not readable at {root:?}; this test ships with the repo and must always find it");
    };
    for book_entry in books.flatten() {
        let cf_dir = book_entry.path().join("class_feature");
        if !cf_dir.is_dir() {
            continue;
        }
        let mut stack = vec![cf_dir];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    out.push(path);
                }
            }
        }
    }
    out
}

/// A `(book, data.key)` pair must name exactly one shipped record. Two
/// records sharing a key means one silently overwrote the other's file on
/// disk during generation -- the exact wave-23 `class-field-fix` regen
/// defect (two occurrences, both fixed in the same commit as this test).
#[test]
fn class_feature_book_key_pairs_are_unique_across_the_whole_corpus() {
    let files = class_feature_files();
    assert!(files.len() > 10_000, "found only {} class_feature files -- corpus_root resolution is probably wrong", files.len());

    let mut seen: std::collections::BTreeMap<(String, String), PathBuf> = std::collections::BTreeMap::new();
    for path in &files {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("unreadable {path:?}: {e}"));
        let json: serde_json::Value =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("invalid JSON in {path:?}: {e}"));
        // book = the path segment right after `data/corpus/`.
        let root = cache_dir();
        let rel = path.strip_prefix(&root).expect("file was found under cache_dir()");
        let book = rel
            .components()
            .next()
            .and_then(|c| c.as_os_str().to_str())
            .expect("class_feature file has a book path segment")
            .to_string();
        let key = json["data"]["key"]
            .as_str()
            .unwrap_or_else(|| panic!("{path:?} has no data.key"))
            .to_string();
        if let Some(prior) = seen.insert((book.clone(), key.clone()), path.clone()) {
            panic!(
                "duplicate class_feature key within one book -- one of these two files silently overwrote the other during generation:\n  book={book:?} key={key:?}\n  first:  {prior:?}\n  second: {path:?}"
            );
        }
    }
}

/// A coarse content-loss ratchet: the corpus-wide `raw_tokens` count must
/// never drop. This is deliberately a floor, not an exact pin (legitimate
/// future book ingests only ever add tokens), so it will not block real
/// growth -- it exists only to catch a regen that silently empties or
/// truncates records without also colliding a key (finding 2 of the wave-23
/// class-field-fix review: 103,332 -> 103,329 from exactly the two destroyed
/// records above; this floor is set at that pinned value).
#[test]
fn class_feature_raw_tokens_total_never_drops_below_the_wave23_floor() {
    const FLOOR: usize = 103_332;
    let files = class_feature_files();
    let mut total = 0usize;
    for path in &files {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("unreadable {path:?}: {e}"));
        let json: serde_json::Value =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("invalid JSON in {path:?}: {e}"));
        total += json["data"]["raw_tokens"].as_array().map(|a| a.len()).unwrap_or(0);
    }
    assert!(
        total >= FLOOR,
        "class_feature raw_tokens_total dropped to {total}, below the wave-23 floor of {FLOOR} -- a regen likely destroyed or truncated records; re-run the guarded regen from a clean corpus and diff by (book,key) before committing"
    );
}
