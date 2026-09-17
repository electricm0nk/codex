//! The `kind=class_feature` level-scaling bar check -- oracle-side, never live.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 17 (`decisions.md` §11, §19). This half of
//! [`run_bar_check`](codex::rules_core::derived_evaluator_fixture_check::run_bar_check)
//! walks `data/corpus/<book>/class_feature/` and reads each record's raw
//! `BONUS:VAR|<name>|<formula>` tokens through the converter's own ingest-row
//! accessor. That is a read of the ingest format, and `§11` states the live side
//! (`src/rules_core/**`) may not do one: the converter, the parser and the oracle
//! harness are KEPT, and a gate that drives them belongs beside them.
//!
//! **The check itself is unchanged from
//! `rules_core::derived_evaluator_fixture_check`, where it lived until this
//! cycle; only its address moved** -- the same three functions, the same control
//! flow, the same failure strings, and the same six mutation-proof tests, which
//! drive the real function end to end over a synthetic root. This is the second
//! time this exact move has been made for this exact reason: `AT-35-E6-001` moved
//! the `kind=race_trait` FORMULA half to
//! [`crate::oracle_validation::race_trait_formula_bar_check`], and
//! `run_bar_check` folds that report into the aggregate the same way it folds
//! this one, so the gate's reach is identical.
//!
//! What did NOT move, deliberately: `parse_class_feature_level_scaling` and
//! `load_class_feature_fixtures` stay in `rules_core`. The parser is a live
//! evaluator with live consumers, and the fixture loader reads this repo's own
//! committed fixture file, not a corpus record.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::pcgen_import::ingest_record;
use codex::rules_core::derived_evaluator_fixture_check::{
    BarCheckReport, load_class_feature_fixtures, parse_class_feature_level_scaling,
};

/// Where this repo's own ingest of `book`'s `class_feature` kind lives, and
/// whether it exists -- the `class_feature` sibling of
/// `derived_evaluator_fixture_check`'s `ingested_equipment_dir`/`spell_corpus_dir_exists`.
pub(crate) fn class_feature_corpus_dir_exists(repo_root: &Path, book: &str) -> Option<PathBuf> {
    let dir = repo_root.join("data").join("corpus").join(book);
    dir.join("class_feature").is_dir().then_some(dir)
}

/// Walks `data/corpus/<book>/class_feature/` once (nested by class/ability
/// slug) and returns every record's `BONUS:VAR|<name>|<formula>` tokens,
/// keyed by the record's own `data.key` -- the `class_feature` sibling of
/// the spell seams' own recursive walk, carrying
/// every `VAR` token (not just one field) because a class-feature bar check
/// needs BOTH the headline formula token and, potentially on a DIFFERENT
/// record in the same walk, the level-variable's own alias definition.
fn load_class_feature_bonus_vars(
    class_feature_dir: &Path,
) -> BTreeMap<String, Vec<(String, String)>> {
    let mut out: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    let mut stack = vec![class_feature_dir.to_path_buf()];
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
            let mut vars = Vec::new();
            for v in ingest_record::token_values(&doc, "BONUS") {
                let Some(rest) = v.strip_prefix("VAR|") else { continue };
                let Some((name, formula)) = rest.split_once('|') else { continue };
                vars.push((name.to_string(), formula.to_string()));
            }
            if !vars.is_empty() {
                out.entry(key.to_string()).or_default().extend(vars);
            }
        }
    }
    out
}

/// Searches every record's `BONUS:VAR` tokens `bonus_vars` carries (the
/// WHOLE book, not one record -- see `ClassFeatureFixture`'s doc comment
/// on why the alias may live on a sibling record) for a token whose NAME is
/// `level_var`, and returns its formula text verbatim (the declared alias,
/// e.g. `"BarbarianLVL"` or, one hop short of a base class,
/// `"SlayerStudiedTargetLVL"`) -- `None` if no record in the book defines it.
fn find_level_var_alias(
    bonus_vars: &BTreeMap<String, Vec<(String, String)>>,
    level_var: &str,
) -> Option<String> {
    bonus_vars.values().flatten().find(|(name, _)| name == level_var).map(|(_, v)| v.clone())
}

/// The `kind=class_feature` half of `run_bar_check`.
pub fn run_class_feature_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_class_feature_fixtures(repo_root);
    let fixtures_total = fixtures.len();
    let books: BTreeSet<String> = fixtures.iter().map(|f| f.book.clone()).collect();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for book in &books {
        let Some(dir) = class_feature_corpus_dir_exists(repo_root, book) else {
            for f in fixtures.iter().filter(|f| &f.book == book) {
                engine_does_not_hold.insert(f.unit_id.clone(), book.clone());
            }
            continue;
        };
        let bonus_vars = load_class_feature_bonus_vars(&dir.join("class_feature"));

        for fixture in fixtures.iter().filter(|f| &f.book == book) {
            let Some(record_vars) = bonus_vars.get(&fixture.record_key) else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "{:?} does not resolve against {book}'s ingested class_feature cache",
                        fixture.record_key
                    ),
                );
                continue;
            };
            let Some((_, raw_formula)) =
                record_vars.iter().find(|(name, _)| name == &fixture.bonus_var_name)
            else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} but carries no bonus-variable magnitude named {} at all",
                        fixture.corpus_field, fixture.bonus_var_name
                    ),
                );
                continue;
            };
            let Some((level_var, formula)) = parse_class_feature_level_scaling(raw_formula)
            else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} but the evaluator could not parse a level-scaling \
                         formula from {raw_formula:?}",
                        fixture.corpus_field
                    ),
                );
                continue;
            };
            if level_var != fixture.expected_level_var
                || formula.offset_pre != fixture.expected_offset_pre
                || formula.divisor != fixture.expected_divisor
                || formula.offset_post != fixture.expected_offset_post
            {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row {:?} states level_var {:?} offset_pre {} divisor {} \
                         offset_post {}, evaluator produced level_var {:?} offset_pre {} \
                         divisor {} offset_post {}",
                        fixture.corpus_field,
                        fixture.expected_level_var,
                        fixture.expected_offset_pre,
                        fixture.expected_divisor,
                        fixture.expected_offset_post,
                        level_var,
                        formula.offset_pre,
                        formula.divisor,
                        formula.offset_post
                    ),
                );
                continue;
            }
            match find_level_var_alias(&bonus_vars, &level_var) {
                Some(alias) if alias == fixture.expected_class_level_alias => {
                    cleared.insert(fixture.unit_id.clone());
                }
                Some(alias) => {
                    failures.insert(
                        fixture.unit_id.clone(),
                        format!(
                            "level_var {level_var:?} aliases {alias:?} in {book}'s own class_feature \
                             corpus, fixture expected {:?}",
                            fixture.expected_class_level_alias
                        ),
                    );
                }
                None => {
                    failures.insert(
                        fixture.unit_id.clone(),
                        format!(
                            "no record in {book}'s class_feature corpus defines a bonus-variable \
                             magnitude named {level_var}, \
                             so the fixture's expected alias {:?} cannot be confirmed",
                            fixture.expected_class_level_alias
                        ),
                    );
                }
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

#[cfg(test)]
mod class_feature_scaling_bar_check_tests {
    use super::*;

    #[test]
    fn run_class_feature_bar_check_clears_every_committed_class_feature_fixture() {
        let repo_root = crate::repo_root();
        let report = run_class_feature_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one class_feature_entries row"
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed class_feature fixture's book must be ingested, got: {:?}",
            report.engine_does_not_hold
        );
        assert!(
            report.failures.is_empty(),
            "every committed class_feature fixture must clear the bar, got {} failures, first \
             few: {:?}",
            report.failures.len(),
            report.failures.iter().take(5).collect::<Vec<_>>()
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// A synthetic `repo_root` carrying one `class_feature` corpus record
    /// (`Rage Power ~ Superstition`-shaped: `2+ProbeLVL/4`, plus a sibling
    /// record defining `ProbeLVL`'s own alias) plus one fixture the caller
    /// corrupts -- same `ScratchRangeRoot`/`ScratchDurationRoot` pattern the
    /// spell seams above use, so a test can drive the REAL
    /// `run_class_feature_bar_check(&root)` end to end without touching the
    /// committed fixture.
    struct ScratchClassFeatureRoot {
        root: PathBuf,
    }

    impl ScratchClassFeatureRoot {
        /// The real corpus formula is fixed (`2+ProbeLVL/4`, offset_pre=0
        /// under `parse_class_feature_level_scaling`'s own N+VAR/D shape);
        /// every parameter here is what the FIXTURE claims via `expected`,
        /// so a caller can independently mutate any one of the four
        /// compared fields away from truth and prove `run_class_feature_
        /// bar_check` catches that specific mismatch (SD31-W13-INTEGRATE-001:
        /// `offset_pre` and `level_var` were previously never mutated at
        /// all -- `offset_pre` had in fact been dropped from this
        /// constructor's own parameter list, `let _ = expected_offset_pre;`
        /// dead code, one commit prior).
        fn new_full(
            name: &str,
            expected_offset_pre: i32,
            expected_divisor: i32,
            expected_offset_post: i32,
            expected_level_var: &str,
        ) -> Self {
            let root = std::env::temp_dir().join(format!(
                "codex_class_feature_mutation_proof_{name}_{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            let cf_dir = root.join("data/corpus/core_rulebook/class_feature");
            std::fs::create_dir_all(&cf_dir).unwrap();
            std::fs::write(
                cf_dir.join("scratch_power.json"),
                crate::pcgen_import::ingest_payload::ingest_record_json(
                    "Probe ~ Scratch Power",
                    &[("BONUS", "VAR|ScratchPowerBonus|2+ProbeLVL/4")],
                ),
            )
            .unwrap();
            std::fs::write(
                cf_dir.join("scratch_pool_header.json"),
                crate::pcgen_import::ingest_payload::ingest_record_json("Probe ~ Scratch Powers", &[("BONUS", "VAR|ProbeLVL|ProbeClassLVL")]),
            )
            .unwrap();
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"class_feature_entries":[{{
                        "unit_id":"scratch:class_feature:scratch_power",
                        "book":"core_rulebook",
                        "record_key":"Probe ~ Scratch Power",
                        "bonus_var_name":"ScratchPowerBonus",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"BONUS:VAR|ScratchPowerBonus|2+ProbeLVL/4",
                        "alias_upstream_line":1,
                        "alias_corpus_field":"BONUS:VAR|ProbeLVL|ProbeClassLVL",
                        "expected":{{
                            "offset_pre":{expected_offset_pre},
                            "divisor":{expected_divisor},
                            "offset_post":{expected_offset_post},
                            "level_var":"{expected_level_var}",
                            "class_level_alias":"ProbeClassLVL"
                        }}
                    }}]}}"#
                ),
            )
            .unwrap();
            ScratchClassFeatureRoot { root }
        }

        fn new(name: &str, expected_divisor: i32, expected_offset_post: i32) -> Self {
            Self::new_full(name, 0, expected_divisor, expected_offset_post, "ProbeLVL")
        }
    }

    impl Drop for ScratchClassFeatureRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    // MUTATION PROOF: a fixture whose `expected.divisor` is deliberately
    // wrong must make the REAL `run_class_feature_bar_check` report a
    // failure, not silently pass.
    #[test]
    fn a_wrong_expected_divisor_makes_run_class_feature_bar_check_report_a_failure() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        let wrong_divisor = real.divisor + 1;
        let scratch = ScratchClassFeatureRoot::new("wrong_divisor", wrong_divisor, real.offset_post);
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(
            report.cleared.is_empty(),
            "a fixture asserting a wrong expected divisor must never clear the bar, got {:?}",
            report.cleared
        );
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:class_feature:scratch_power"));
    }

    // MUTATION PROOF (SD31-W13-INTEGRATE-001, was missing entirely): a
    // fixture whose `expected.offset_pre` is deliberately wrong must also
    // make the real check fail. The real formula (`2+ProbeLVL/4`) has
    // offset_pre=0; asserting 1 must not clear the bar.
    #[test]
    fn a_wrong_expected_offset_pre_makes_run_class_feature_bar_check_report_a_failure() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        assert_eq!(real.offset_pre, 0, "test assumption: real offset_pre is 0");
        let scratch =
            ScratchClassFeatureRoot::new_full("wrong_offset_pre", 1, real.divisor, real.offset_post, "ProbeLVL");
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:class_feature:scratch_power"));
    }

    // MUTATION PROOF (SD31-W13-INTEGRATE-001, was missing entirely): a
    // fixture whose `expected.offset_post` is deliberately wrong must also
    // make the real check fail.
    #[test]
    fn a_wrong_expected_offset_post_makes_run_class_feature_bar_check_report_a_failure() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        let wrong_offset_post = real.offset_post + 1;
        let scratch =
            ScratchClassFeatureRoot::new_full("wrong_offset_post", 0, real.divisor, wrong_offset_post, "ProbeLVL");
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:class_feature:scratch_power"));
    }

    // MUTATION PROOF (SD31-W13-INTEGRATE-001, was missing entirely): a
    // fixture whose `expected.level_var` names the WRONG variable must also
    // make the real check fail -- distinct from the class_level_alias proof
    // below, which mutates the alias the level_var resolves TO, not the
    // level_var name itself.
    #[test]
    fn a_wrong_expected_level_var_makes_run_class_feature_bar_check_report_a_failure() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        let scratch = ScratchClassFeatureRoot::new_full(
            "wrong_level_var",
            0,
            real.divisor,
            real.offset_post,
            "TotallyTheWrongLevelVar",
        );
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:class_feature:scratch_power"));
    }

    // The same proof for `expected.class_level_alias`: a fixture claiming
    // the WRONG owning class for the level variable must also fail, not just
    // a wrong numeric coefficient -- this is the check that would have
    // caught a level-scaling formula silently pointing at the wrong class.
    #[test]
    fn a_wrong_expected_class_level_alias_makes_run_class_feature_bar_check_report_a_failure() {
        let root = std::env::temp_dir()
            .join(format!("codex_class_feature_mutation_proof_wrong_alias_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let cf_dir = root.join("data/corpus/core_rulebook/class_feature");
        std::fs::create_dir_all(&cf_dir).unwrap();
        std::fs::write(
            cf_dir.join("scratch_power.json"),
            crate::pcgen_import::ingest_payload::ingest_record_json("Probe ~ Scratch Power", &[("BONUS", "VAR|ScratchPowerBonus|2+ProbeLVL/4")]),
        )
        .unwrap();
        std::fs::write(
            cf_dir.join("scratch_pool_header.json"),
            crate::pcgen_import::ingest_payload::ingest_record_json("Probe ~ Scratch Powers", &[("BONUS", "VAR|ProbeLVL|ProbeClassLVL")]),
        )
        .unwrap();
        let fixture_dir = root.join("tests/fixtures/rules_core");
        std::fs::create_dir_all(&fixture_dir).unwrap();
        std::fs::write(
            fixture_dir.join("derived-evaluator-fixtures.json"),
            r#"{"class_feature_entries":[{
                "unit_id":"scratch:class_feature:scratch_power",
                "book":"core_rulebook",
                "record_key":"Probe ~ Scratch Power",
                "bonus_var_name":"ScratchPowerBonus",
                "upstream_lst":"scratch.lst",
                "upstream_lst_sha256":"0",
                "upstream_line":1,
                "corpus_field":"BONUS:VAR|ScratchPowerBonus|2+ProbeLVL/4",
                "alias_upstream_line":1,
                "alias_corpus_field":"BONUS:VAR|ProbeLVL|ProbeClassLVL",
                "expected":{
                    "offset_pre":0,
                    "divisor":4,
                    "offset_post":2,
                    "level_var":"ProbeLVL",
                    "class_level_alias":"TotallyTheWrongClassLVL"
                }
            }]}"#,
        )
        .unwrap();

        let report = run_class_feature_bar_check(&root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        let _ = std::fs::remove_dir_all(&root);
    }

    // The positive control for both mutation-proof tests above: a fixture
    // whose `expected` matches the real corpus row EXACTLY (divisor and
    // alias both correct) must clear the bar -- proving the two tests above
    // fail because the asserted value is wrong, not because the synthetic
    // harness always reports a failure.
    #[test]
    fn a_correct_expected_class_feature_formula_clears_run_class_feature_bar_check() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        let scratch = ScratchClassFeatureRoot::new("correct", real.divisor, real.offset_post);
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:class_feature:scratch_power"));
    }
}
