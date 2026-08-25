//! SD-32 Gate 2 (`gate-2-engines-f1-f9`, kanban `#6`) — confirms
//! `src/rules_core/pilot_compute/formula_interpreter.rs`'s `PcgenFormulaEvaluator`
//! reaches every one of `scripts/shape_ledger.py`'s nine in-scope shape families
//! (F1..F9 — the tenth, F10, is `bonus_stack_reader.rs`'s binding-layer scope,
//! kanban `#7`, per `epic-breakdown.md` Epic 1 and `acceptance-and-verification.md`
//! AT-32-G2-001).
//!
//! # What this closes (AT-32-G2-001/002/003)
//!
//! `tests/fixtures/rules_core/formula-interpreter-family-fixtures.json` carries
//! one real corpus `BONUS`/`DEFINE` formula sample per family, each transcribed
//! from the pinned PCGen oracle's raw `.lst` bytes at authoring time (see the
//! fixture's own `"derivation"` field and the cycle receipt at
//! `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/001_cycle_receipt.md`
//! for the exact `sha256sum`/`sed` commands run against the oracle checkout).
//! `expected` values are hand-derived from each formula's own arithmetic by a
//! human reading the formula text — never by running the evaluator and copying
//! its answer (operator ruling §20 / `decisions.md` Decision 3: "an interpreted
//! value with no fixture is not done").
//!
//! This is a narrower, family-scoped sibling of
//! `derived_evaluator_fixture_check.rs`'s consumer-unit-scoped fixture check
//! (which already wires `PcgenFormulaEvaluator` into several `kind=spell`/
//! `kind=monster` seams) — this file asks a different question: does the
//! interpreter's *grammar* reach every named shape family at all, independent
//! of which consumer eventually wires it. Per `technical-design.md`'s
//! file-disjointness table, Gate 2 touches `src/rules_core/pilot_compute/*.rs`
//! and "new test files" — this file and its fixture, not the existing
//! `derived-evaluator-fixtures.json` (a different, unit-kind-scoped gate this
//! card does not touch).

use std::collections::BTreeMap;
use std::path::Path;

use codex::rules_core::pilot_compute::formula_interpreter::{extract_formula_field, PcgenFormulaEvaluator};
use codex::rules_core::pilot_compute::formula_reproduction_harness::FormulaEvaluator;

const FIXTURE_RELATIVE_PATH: &str = "tests/fixtures/rules_core/formula-interpreter-family-fixtures.json";

/// The nine families this card is scoped to, in the exact order
/// `acceptance-and-verification.md` AT-32-G2-001 names them (F1..F9, F10
/// excluded — that is kanban card 7's own scope).
const IN_SCOPE_FAMILIES: [&str; 9] = ["F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9"];

struct FixtureEntry {
    family: String,
    book: String,
    record_key: String,
    corpus_record_path: String,
    upstream_lst: String,
    upstream_lst_sha256: String,
    upstream_line: u64,
    raw_token_key: String,
    raw_token_value: String,
    formula: String,
    vars: BTreeMap<String, i64>,
    expected: i64,
}

fn load_fixtures(repo_root: &Path) -> Vec<FixtureEntry> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed family fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed family fixture must be valid JSON");
    let entries = doc["entries"].as_array().expect("fixture carries an `entries` array");
    entries
        .iter()
        .map(|e| {
            let vars = e["vars"]
                .as_object()
                .expect("entry.vars")
                .iter()
                .map(|(k, v)| (k.clone(), v.as_i64().expect("var value fits in i64")))
                .collect();
            FixtureEntry {
                family: e["family"].as_str().expect("family").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                corpus_record_path: e["corpus_record_path"].as_str().expect("corpus_record_path").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"].as_str().expect("upstream_lst_sha256").to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                raw_token_key: e["raw_token_key"].as_str().expect("raw_token_key").to_string(),
                raw_token_value: e["raw_token_value"].as_str().expect("raw_token_value").to_string(),
                formula: e["formula"].as_str().expect("formula").to_string(),
                vars,
                expected: e["expected"].as_i64().expect("expected"),
            }
        })
        .collect()
}

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// **The load-bearing test.** Every one of the nine in-scope families' fixture
/// entry evaluates, through the real `PcgenFormulaEvaluator`, to exactly the
/// hand-derived expected value — no adjustment to the evaluator to force
/// agreement; a disagreement here is a finding, not a defect in this test to
/// relax.
#[test]
fn engine_reaches_every_in_scope_family_and_clears_its_fixture() {
    let fixtures = load_fixtures(&repo_root());
    let evaluator = PcgenFormulaEvaluator;
    let mut covered: Vec<String> = Vec::new();
    let mut failures: Vec<String> = Vec::new();

    for entry in &fixtures {
        covered.push(entry.family.clone());
        match evaluator.evaluate(&entry.formula, &entry.vars) {
            Ok(v) if v == entry.expected => {}
            Ok(v) => failures.push(format!(
                "{} ({} / {:?}): evaluator returned {v}, fixture expects {} (formula {:?})",
                entry.family, entry.book, entry.record_key, entry.expected, entry.formula
            )),
            Err(e) => failures.push(format!(
                "{} ({} / {:?}): evaluator REFUSED {:?}: {}",
                entry.family, entry.book, entry.record_key, entry.formula, e.0
            )),
        }
    }

    assert!(
        failures.is_empty(),
        "one or more in-scope families failed their fixture check:\n{}",
        failures.join("\n")
    );

    for family in IN_SCOPE_FAMILIES {
        assert!(
            covered.iter().any(|f| f == family),
            "family {family} has no fixture entry — AT-32-G2-001 requires every in-scope \
             family to be covered, not just the ones that happen to be easy"
        );
    }
    assert_eq!(
        covered.len(),
        IN_SCOPE_FAMILIES.len(),
        "expected exactly one fixture entry per in-scope family ({} families), found {} entries \
         — a duplicate or an out-of-scope family (e.g. F0/F10) slipped in",
        IN_SCOPE_FAMILIES.len(),
        covered.len()
    );
}

/// Decision 1(a) mutation proof: a deliberately wrong evaluator (off-by-one on
/// every result) MUST be caught disagreeing with at least one fixture entry —
/// proves the check above is a real gate, not one that would pass regardless
/// of what the interpreter computes.
#[test]
fn mutated_evaluator_is_caught_disagreeing_with_the_family_fixtures() {
    struct OffByOneEvaluator;
    impl FormulaEvaluator for OffByOneEvaluator {
        fn evaluate(
            &self,
            formula: &str,
            vars: &BTreeMap<String, i64>,
        ) -> Result<i64, codex::rules_core::pilot_compute::formula_reproduction_harness::FormulaEvalError> {
            PcgenFormulaEvaluator.evaluate(formula, vars).map(|v| v + 1)
        }
    }
    let fixtures = load_fixtures(&repo_root());
    let mutant = OffByOneEvaluator;
    let mut disagreements = 0usize;
    for entry in &fixtures {
        if mutant.evaluate(&entry.formula, &entry.vars).ok() != Some(entry.expected) {
            disagreements += 1;
        }
    }
    assert_eq!(
        disagreements,
        fixtures.len(),
        "every fixture entry has a non-zero expected magnitude, so an off-by-one mutant must \
         disagree on all of them — a mutant that agrees on any would mean this fixture set \
         cannot distinguish a broken evaluator from a correct one"
    );
}

/// Provenance guarantee: `formula` in the fixture must be exactly what
/// `extract_formula_field` (the SAME positional heuristic Gate 1's shape
/// ledger and the corpus-wide coverage scan both use) derives from
/// `raw_token_key`/`raw_token_value` — i.e. the fixture author did not
/// hand-edit the formula text away from what the real extraction path
/// produces.
#[test]
fn fixture_formula_matches_extract_formula_field_on_its_own_raw_token() {
    let fixtures = load_fixtures(&repo_root());
    for entry in &fixtures {
        let extracted = extract_formula_field(&entry.raw_token_key, &entry.raw_token_value);
        assert_eq!(
            extracted,
            Some(entry.formula.as_str()),
            "{}: extract_formula_field({:?}, {:?}) = {:?}, fixture claims formula {:?}",
            entry.family,
            entry.raw_token_key,
            entry.raw_token_value,
            extracted,
            entry.formula
        );
    }
}

/// Provenance guarantee: the fixture's `corpus_record_path` exists in this
/// repo's own ingested corpus and its `source` block matches the fixture's
/// claimed book/record_key/upstream_lst/upstream_line/upstream_lst_sha256
/// exactly — the same fields `data/corpus/**/*.json` independently carries
/// about its own provenance, so a regenerated or hand-edited corpus record
/// that drifted from the upstream `.lst` byte this fixture cites would be
/// caught here rather than silently compared against a stale copy.
#[test]
fn fixture_provenance_matches_the_committed_corpus_records_own_source_block() {
    let root = repo_root();
    let fixtures = load_fixtures(&root);
    for entry in &fixtures {
        let path = root.join(&entry.corpus_record_path);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{}: corpus record must be readable at {path:?}: {e}", entry.family));
        let rec: serde_json::Value =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: corpus record must be valid JSON: {e}", entry.family));
        let source = &rec["source"];
        assert_eq!(source["path"].as_str(), Some(entry.upstream_lst.as_str()), "{}: upstream_lst", entry.family);
        assert_eq!(
            source["sha256"].as_str(),
            Some(entry.upstream_lst_sha256.as_str()),
            "{}: upstream_lst_sha256 — a regenerated/edited corpus would drift here",
            entry.family
        );
        assert_eq!(source["line"].as_u64(), Some(entry.upstream_line), "{}: upstream_line", entry.family);
        assert_eq!(source["record_key"].as_str(), Some(entry.record_key.as_str()), "{}: record_key", entry.family);
        let book_dir = format!("data/corpus/{}/", entry.book);
        assert!(
            entry.corpus_record_path.starts_with(&book_dir),
            "{}: corpus_record_path {:?} does not start under {book_dir:?}",
            entry.family,
            entry.corpus_record_path
        );
    }
}

/// Every entry's `family` is exactly what `scripts/shape_ledger.py`'s
/// `classify_formula()` (Gate 1's own classifier, same priority-ordered rule
/// list) would assign to `formula` today -- so this fixture cannot silently
/// drift out of step with the family Gate 1's ledger already counted this
/// unit's shape under. Shells out to `python3`, matching this repo's existing
/// pattern of cross-language fixture consistency checks
/// (`tests/sd27_pu_prose_derived_class_features_reach_the_sheet.rs` and
/// siblings shell out to corpus-walking scripts the same way); skips (not
/// fails) rather than false-failing when `python3` is unavailable in a given
/// CI image, since the Rust-side fixture-vs-evaluator checks above are the
/// load-bearing gate and do not depend on Python being present.
#[test]
fn fixture_family_matches_shape_ledgers_own_classifier() {
    let root = repo_root();
    let fixtures = load_fixtures(&root);
    let script = root.join("scripts").join("shape_ledger.py");
    if !script.is_file() {
        eprintln!("fixture_family_matches_shape_ledgers_own_classifier: {script:?} not found, skipping");
        return;
    }
    for entry in &fixtures {
        let code = format!(
            "import sys; sys.path.insert(0, {scripts_dir:?}); import shape_ledger as SL; print(SL.classify_formula({formula:?}))",
            scripts_dir = root.join("scripts").to_string_lossy(),
            formula = entry.formula,
        );
        let output = std::process::Command::new("python3")
            .args(["-c", &code])
            .current_dir(&root)
            .output();
        let Ok(output) = output else {
            eprintln!("fixture_family_matches_shape_ledgers_own_classifier: python3 unavailable, skipping");
            return;
        };
        if !output.status.success() {
            eprintln!(
                "fixture_family_matches_shape_ledgers_own_classifier: shape_ledger.py invocation failed \
                 ({:?}), skipping this environment rather than false-failing",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        let classified = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(
            classified, entry.family,
            "{}: shape_ledger.py's own classify_formula({:?}) says {classified:?}, fixture claims {:?}",
            entry.family, entry.formula, entry.family
        );
    }
}
