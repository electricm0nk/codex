//! Shared logic for the `derived` wiring class's evaluator-vs-fixture check.
//!
//! Extracted from `tests/derived_evaluator_fixture_check.rs` (operator
//! directive 2026-08-13, "add the done rung for static and derived") so
//! `v06_work_inventory` can emit the SAME evidence the test suite proves,
//! rather than inventing a second implementation of the bar. The test file
//! keeps its own guarantee-3/guarantee-4 provenance checks (which do not
//! belong in a report the generator consumes) and now calls
//! [`run_bar_check`] for the bar itself, so the two can never drift apart.
//!
//! The bar, precisely: the engine's evaluator, run over the real corpus
//! record through the shipping `compute_equipment_effects` seam, must
//! produce exactly the ability list and magnitude the fixture's pinned
//! corpus field states. See `tests/derived_evaluator_fixture_check.rs`'s
//! module doc for the four independent guarantees this rests on.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
use crate::rules_core::corpus_loader::{BookCorpusRoot, load_equipment_corpus};
use crate::rules_core::equipment_effects::compute_equipment_effects;

pub const FIXTURE_RELATIVE_PATH: &str = "tests/fixtures/rules_core/derived-evaluator-fixtures.json";

/// One fixture row, in the shape the committed JSON carries.
#[derive(Debug, Clone)]
pub struct Fixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub corpus_field: String,
    pub expected_abilities: Vec<String>,
    pub expected_bonus: i16,
}

/// Reads the committed fixture. Panics on a missing/malformed file -- the
/// same "a hand-invented input must never look like evidence" posture the
/// test suite takes, and correct for a report a `--json-out` consumer trusts.
pub fn load_fixtures(repo_root: &Path) -> Vec<Fixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let entries = doc["entries"].as_array().expect("fixture carries an `entries` array");
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            Fixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                expected_abilities: expected["abilities"]
                    .as_array()
                    .expect("expected.abilities")
                    .iter()
                    .map(|a| a.as_str().expect("ability name").to_string())
                    .collect(),
                expected_bonus: i16::try_from(
                    expected["bonus"].as_i64().expect("expected.bonus"),
                )
                .expect("an ability bonus fits in i16"),
            }
        })
        .collect()
}

/// Where this repo's own ingest of `book` lives, and whether it exists.
/// Absence is a real, structural fact -- a book nobody has ingested has no
/// records for any evaluator to run over.
fn ingested_equipment_dir(repo_root: &Path, book: &str) -> Option<PathBuf> {
    let dir = repo_root.join("data").join("corpus").join(book);
    dir.join("equipment").is_dir().then_some(dir)
}

/// The result of running the `derived` bar check over every fixture entry.
pub struct BarCheckReport {
    /// `unit_id`s whose evaluator output matched the fixture's expected
    /// value exactly -- the ONLY units this instrument licenses moving to
    /// `fixture-verified`.
    pub cleared: BTreeSet<String>,
    /// `unit_id` -> reason, for every covered entry that did NOT clear
    /// (evaluator mismatch, no resolution, or no ability bonus produced).
    pub failures: BTreeMap<String, String>,
    /// `unit_id` -> book, for entries whose book has no ingest in this repo
    /// at all -- distinct from a failure because the evaluator was never
    /// reached.
    pub not_ingested: BTreeMap<String, String>,
    pub fixtures_total: usize,
}

/// Runs the `derived` bar over every fixture entry, exactly as
/// `tests/derived_evaluator_fixture_check.rs::engine_evaluator_output_equals_the_corpus_derived_expected_value`
/// does, factored out so both the test and `v06_work_inventory` call the
/// same code.
pub fn run_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_fixtures(repo_root);
    let fixtures_total = fixtures.len();
    let books: BTreeSet<String> = fixtures.iter().map(|f| f.book.clone()).collect();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut not_ingested: BTreeMap<String, String> = BTreeMap::new();

    for book in &books {
        let Some(dir) = ingested_equipment_dir(repo_root, book) else {
            for f in fixtures.iter().filter(|f| &f.book == book) {
                not_ingested.insert(f.unit_id.clone(), book.clone());
            }
            continue;
        };
        let roots = [BookCorpusRoot { book_id: book.as_str(), dir: Path::new(&dir) }];
        let corpus = load_equipment_corpus(&roots);

        for fixture in fixtures.iter().filter(|f| &f.book == book) {
            let selection = vec![EquipmentSelection {
                item_id: fixture.record_key.clone(),
                equipped_or_active: true,
                active_state: ActiveState::EquippedActive,
                applied_modifiers: Vec::new(),
            }];
            let effects = compute_equipment_effects(&selection, &corpus);
            let Some(item) = effects.per_item.first() else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!("{:?} does not resolve against its own ingested book", fixture.record_key),
                );
                continue;
            };
            let Some(bonus) = &item.ability_bonus else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} but the evaluator produced no ability bonus at all",
                        fixture.corpus_field
                    ),
                );
                continue;
            };
            let abilities: Vec<String> =
                bonus.ability.split(',').map(str::trim).map(str::to_string).collect();
            if abilities != fixture.expected_abilities || bonus.bonus != fixture.expected_bonus {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row {:?} states {:?} {:+}, evaluator produced {:?} {:+}",
                        fixture.corpus_field,
                        fixture.expected_abilities,
                        fixture.expected_bonus,
                        abilities,
                        bonus.bonus
                    ),
                );
                continue;
            }
            cleared.insert(fixture.unit_id.clone());
        }
    }

    BarCheckReport { cleared, failures, not_ingested, fixtures_total }
}
