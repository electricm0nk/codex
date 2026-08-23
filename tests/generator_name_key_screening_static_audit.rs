//! `t9-onboarding-pi-last-leak-and-generators` cycle — the structural test
//! the prior cycle (`t9-onboarding-pi-final-leaks-and-generators`) was
//! asked for but did not deliver: **"a test that fails when a generator
//! writes a field it does not screen."**
//!
//! `declared_pi_shipping_audit`'s CHECK C
//! (`audit_blacklist_term_hits`) is generator-agnostic and catches a leak
//! **once it is already shipped in `data/corpus/`**. It cannot, by
//! construction, catch a generator that stops calling the screen for a
//! field it merely happens not to be leaking *yet* — a shipped corpus
//! with zero blacklist matches today is silent evidence of nothing about
//! whether a future `PI_BLACKLIST_TERMS` amendment (this bundle has
//! amended it at least 4 times, `decisions.md §19`) would ever get
//! re-screened. That is exactly the shape of the eighth-and-ninth
//! instances this cycle found and fixed
//! (`cache_gen::ultimate_equipment`, `src/bin/gen_core_rulebook_cache.rs`).
//!
//! **This test inspects generator SOURCE CODE, not shipped corpus bytes.**
//! It walks every file that DEFINES a Shape-B-style record payload struct
//! carrying its own `pub name: String` or `pub key: String` field (an
//! identity field a record cannot omit -- discovered dynamically via
//! `std::fs::read_dir`, never a hand-maintained file list, so a brand-new
//! generator added later is covered automatically the moment it defines
//! such a struct) and asserts the file's own source references at least
//! one of the sanctioned PI-screening symbols. A file that defines the
//! field but never calls a screen anywhere in it is a real, provable
//! defect of the exact shape this bundle has now found nine times.
//!
//! **What this test does NOT prove** (`AGENTS.md` non-negotiable rule 7):
//! it is a textual co-occurrence check, not a data-flow proof -- a file
//! that references a screening symbol ANYWHERE (even on an unrelated
//! field, or in a comment) passes. It cannot tell a real call site from a
//! stale doc-comment mention. This is the "closest enforceable
//! equivalent" the dispatch brief invited if a stronger one could not be
//! built in this cycle's budget: full type-level enforcement (a
//! `ScreenedString` newtype `CacheRecord`'s `name`/`key` fields require,
//! constructible only through the scan) would close this exact gap, but
//! is a schema-wide refactor across ~10 files' public types -- sized as
//! its own follow-on, not a single cycle's remaining scope
//! (`decisions.md §27b`/`docs/governance/blocker-closure-doctrine.md`:
//! named, not silently narrowed).
//!
//! Two files (`rules_tables::crb::json_cache`,
//! `rules_tables::advanced_race_guide::json_cache`) define their record
//! payload shape SEPARATELY from the generator that actually screens it
//! (`src/bin/gen_core_rulebook_cache.rs`, `src/bin/gen_book_cache.rs`
//! respectively) -- `SCHEMA_ONLY_FILES` below maps each to its real
//! generator, checked in its place. Every other discovered file screens
//! (or is expected to screen) within itself.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Symbols this bundle has established as the sanctioned way to screen a
/// `name`/`key` identity field against Product Identity -- the union of
/// the shared blacklist term scan (weak `classify_field`, strong
/// `blacklist_term_hit_including_concatenated`) and the corpus's own
/// declared `NAMEISPI:` reader. A file referencing ANY of these is judged
/// to be screening the field somewhere within it.
const SANCTIONED_SCREEN_SYMBOLS: &[&str] = &[
    "classify_field(\"name\"",
    "classify_field(\"key\"",
    "blacklist_term_hit_including_concatenated",
    "name_or_key_is_pi",
    "resolve_name_or_rename",
    "declared.name",
    "NAMEISPI",
];

/// `(schema-only file, its real generator file)` -- a schema file whose
/// own source can never contain a screen call, because the generator that
/// populates it lives elsewhere.
const SCHEMA_ONLY_FILES: &[(&str, &str)] = &[
    ("src/rules_core/rules_tables/crb/json_cache.rs", "src/bin/gen_core_rulebook_cache.rs"),
    (
        "src/rules_core/rules_tables/advanced_race_guide/json_cache.rs",
        "src/bin/gen_book_cache.rs",
    ),
];

/// `true` when `text` defines a struct field shaped `pub name: String,` or
/// `pub key: String,` (allowing for `Option<String>` too, since a
/// required-vs-optional identity field is still an identity field) --
/// i.e. this file OWNS a Shape-B-style record payload with an identity
/// field a generator must have screened before writing it.
fn defines_identity_field(text: &str) -> bool {
    text.contains("pub name: String")
        || text.contains("pub key: String")
        || text.contains("pub name: Option<String>")
        || text.contains("pub key: Option<String>")
}

fn references_a_sanctioned_screen(text: &str) -> bool {
    SANCTIONED_SCREEN_SYMBOLS.iter().any(|sym| text.contains(sym))
}

/// Every `.rs` file directly under `dir` (non-recursive, matching this
/// repo's own flat `cache_gen`/`bin` layout) whose source is readable.
fn rs_files_in(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("rs"))
        .collect();
    out.sort();
    out
}

/// Returns `(file_relative_to_repo_root, effective_text_checked)` for
/// every discovered identity-field-owning generator source file, resolving
/// `SCHEMA_ONLY_FILES` to their real generator's text.
fn discover_identity_bearing_generators(root: &Path) -> Vec<(String, String)> {
    let schema_map: std::collections::HashMap<&str, &str> = SCHEMA_ONLY_FILES.iter().copied().collect();
    let dirs = [root.join("src/rules_core/cache_gen"), root.join("src/bin")];
    // `rules_tables/*/json_cache.rs` schema files -- checked via their
    // SCHEMA_ONLY_FILES-mapped generator, never their own (empty) text.
    // Deliberately narrower than the two dirs above: every OTHER file
    // under `rules_tables/*/` (e.g. `mod.rs`) is a compiled RAW DATA
    // table -- `decisions.md §11.3`, this program's own convention that a
    // generator "never re-parses raw LST to derive a field's *value*;
    // every value written [to `data/corpus/`] is read straight from the
    // compiled Rust module" -- so a raw table's own `name`/`key` fields
    // are pre-screening compiled input, not a shipped record; screening
    // happens downstream in the `cache_gen`/`bin` generator that reads
    // the table, which is already covered by the two dirs above.
    let mut json_cache_files: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(root.join("src/rules_core/rules_tables")) {
        for e in entries.flatten() {
            let candidate = e.path().join("json_cache.rs");
            if candidate.is_file() {
                json_cache_files.push(candidate);
            }
        }
    }

    let mut found: Vec<(String, String)> = Vec::new();
    let mut seen_rel: BTreeSet<String> = BTreeSet::new();
    let mut all_candidates: Vec<PathBuf> = Vec::new();
    for dir in &dirs {
        all_candidates.extend(rs_files_in(dir));
    }
    all_candidates.extend(json_cache_files);
    for path in all_candidates {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        if !defines_identity_field(&text) {
            continue;
        }
        let rel = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
        if !seen_rel.insert(rel.clone()) {
            continue;
        }
        let effective_text = match schema_map.get(rel.as_str()) {
            Some(generator_rel) => fs::read_to_string(root.join(generator_rel)).unwrap_or_else(|e| {
                panic!("SCHEMA_ONLY_FILES maps {rel} to {generator_rel}, which failed to read: {e}")
            }),
            None => text,
        };
        found.push((rel, effective_text));
    }
    found
}

/// The real, positive proof: every generator this repo ships today that
/// defines a `name`/`key` identity field screens it somewhere. This is
/// the test that must be GREEN on an honest tree and RED the moment a
/// generator regresses -- proven RED by direct mutation below.
#[test]
fn every_identity_bearing_generator_references_a_pi_screen() {
    let root = repo_root();
    let found = discover_identity_bearing_generators(&root);
    assert!(
        found.len() >= 10,
        "sanity: expected at least the 10 known cache_gen identity-bearing files, found {} -- \
         discovery itself may be broken: {:?}",
        found.len(),
        found.iter().map(|(f, _)| f).collect::<Vec<_>>()
    );

    let mut unscreened: Vec<String> = Vec::new();
    for (rel, text) in &found {
        if !references_a_sanctioned_screen(text) {
            unscreened.push(rel.clone());
        }
    }
    assert!(
        unscreened.is_empty(),
        "{} generator(s) define a name/key identity field but reference NO sanctioned PI screen \
         anywhere in their (or their mapped generator's) source -- this is the exact defect class \
         this bundle has found repeatedly (decisions.md §19's blacklist amendments, `t9-onboarding-\
         pi-final-leaks-and-generators`'s discovery forwards): {unscreened:?}",
        unscreened.len()
    );
}

/// **`decisions.md §1a` mutation proof, run inline (not a separate manual
/// step) so it can never silently stop being exercised**: the detection
/// logic itself, applied to a SYNTHETIC file text that defines an identity
/// field but has had its screen call stripped -- the exact mutation shape
/// a real regression in `ultimate_equipment.rs` or `gen_core_rulebook_
/// cache.rs` would produce. Fails for the intended reason (the synthetic
/// text really does define the field and really does lack every
/// sanctioned symbol), proving `references_a_sanctioned_screen` cannot
/// trivially always return `true`.
#[test]
fn the_detector_itself_goes_red_on_a_synthetic_unscreened_generator() {
    let mutated_source = r#"
        pub struct SyntheticData {
            pub key: String,
            pub name: String,
            pub description: Option<String>,
        }
        fn generate() {
            let data = SyntheticData {
                key: entry.key.to_string(),
                name: entry.name.to_string(),
                description: entry.description.map(str::to_string),
            };
        }
    "#;
    assert!(
        defines_identity_field(mutated_source),
        "sanity: the synthetic source must define an identity field"
    );
    assert!(
        !references_a_sanctioned_screen(mutated_source),
        "the mutation-proof text must NOT reference any sanctioned screen -- \
         if it does, the symbol list is over-broad and this proof is worthless"
    );
}

/// The positive mirror: the SAME synthetic shape, with a real screen call
/// added, passes -- proves the detector is not simply inverted or
/// unconditionally red.
#[test]
fn the_detector_passes_the_same_synthetic_generator_once_a_screen_is_added() {
    let screened_source = r#"
        pub struct SyntheticData {
            pub key: String,
            pub name: String,
        }
        fn generate() {
            let name_is_pi = name_or_key_is_pi(declared.name, entry.name);
        }
    "#;
    assert!(defines_identity_field(screened_source));
    assert!(references_a_sanctioned_screen(screened_source));
}

/// End-to-end mutation proof against a REAL repo file (`§1a`: prove it
/// goes red against the actual thing, not only a synthetic string) --
/// reads `cache_gen::ultimate_equipment`'s real source and strips EVERY
/// occurrence of EVERY sanctioned symbol (a doc comment mentioning the
/// PCGen `NAMEISPI:` token by name, e.g., is expected to survive a real
/// code-only removal, so stripping all of them is the honest simulation
/// of "no trace of a screen remains," not an artifact of picking one
/// arbitrary symbol). Confirms the detector flags the fully-mutated copy
/// while the real on-disk file still passes. Never writes the mutated
/// copy back to the real file -- this test only ever operates on an
/// in-memory string.
#[test]
fn mutating_a_real_generators_screen_call_away_makes_the_detector_fail_for_it() {
    let root = repo_root();
    let real_path = root.join("src/rules_core/cache_gen/ultimate_equipment.rs");
    let real_text = fs::read_to_string(&real_path).expect("ultimate_equipment.rs must exist and be readable");
    assert!(defines_identity_field(&real_text), "sanity: this file must own an identity field");
    assert!(
        references_a_sanctioned_screen(&real_text),
        "sanity: the REAL, unmutated file must currently pass (this cycle's own fix)"
    );

    let mut mutated_text = real_text.clone();
    for sym in SANCTIONED_SCREEN_SYMBOLS {
        mutated_text = mutated_text.replace(sym, "IGNORED_LEGACY_CHECK");
    }
    assert_ne!(mutated_text, real_text, "sanity: the replacement must actually change the text");
    assert!(
        !references_a_sanctioned_screen(&mutated_text),
        "stripping every sanctioned screen-symbol reference from ultimate_equipment.rs must make the \
         detector fail for it"
    );
}
