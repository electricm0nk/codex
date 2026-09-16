//! The `kind=race_trait` FORMULA-shape bar check -- oracle-side, never live.
//!
//! SD-35 `AT-35-E6-001` (`decisions.md` §11): this gate drives the PCGen
//! formula interpreter over a committed fixture roster to prove the SHIPPED
//! `UNDINE_RACE_TRAIT_FORMULAS` table transcribes each formula correctly. That
//! is oracle work -- the interpreter is a converter input and a test oracle,
//! and the live side (`src/rules_core/**`) may not name it. The check itself
//! is unchanged from `rules_core::derived_evaluator_fixture_check`, where it
//! lived until this cycle; only its address moved.
//!
//! `rules_core::derived_evaluator_fixture_check::run_bar_check` still folds
//! this report into the aggregate, so the gate's reach is identical.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::pcgen_import::formula_interpreter::PcgenFormulaEvaluator;
use crate::pcgen_import::formula_reproduction_harness::FormulaEvaluator;
use crate::rules_core::derived_evaluator_fixture_check::{BarCheckReport, FIXTURE_RELATIVE_PATH};
use crate::rules_core::pilot_compute::UNDINE_RACE_TRAIT_FORMULAS;

// -------------------------------------------------------------------------------------------
// Folded into SD-33 from `worktree-wf_be4660f2-72a-3` (2026-08-26) per
// `docs/release/SD-31-corpus-closure-grind/artifacts/OPEN-ISSUES.md` row 365's remediation
// path (a). The seam + fixtures below are unchanged from the branch (reviewer-confirmed sound);
// the branch's race-level `FORMULA_RACE_TRAIT_RACES` doneness-credit const was NOT folded — see
// `src/rules_core/pilot_compute/mod.rs`'s own fold-note next to `explain_undine_formula_race_trait`.
// -------------------------------------------------------------------------------------------

// ---------------------------------------------------------------------------------------------
// `kind=race_trait`, FORMULA shape (SD31-W26-RACETRAIT-001).
// ---------------------------------------------------------------------------------------------
//
// The first consumer of `formula_interpreter::PcgenFormulaEvaluator` anywhere in this codebase
// (`grep -rn PcgenFormulaEvaluator src/` before this addition returns only the evaluator's own
// module) — wave 25b built and proved the interpreter but shipped no consumer of it, per
// `OPERATOR-RULINGS-2026-08-21.md` §20's own condition: "every interpreted value must clear
// `derived_evaluator_fixture_check` ... An interpreted value with no fixture is not done." This
// is that gate for `src/rules_core/pilot_compute/mod.rs`'s
// `explain_undine_formula_race_trait`/`UNDINE_RACE_TRAIT_FORMULAS`.
//
// Runs against the SHIPPED table (`UNDINE_RACE_TRAIT_FORMULAS`), exactly as
// `run_companion_skill_bar_check` runs against `record.skill_ability_diff_bonuses` and for the
// same reason: a transcription that corrupted the formula text in `pilot_compute::mod.rs` must
// fail HERE, not pass silently against a corpus file no player-facing code reads.

/// One `kind=race_trait` formula fixture row — a sibling top-level
/// `race_trait_formula_entries` array in the same committed fixture JSON.
#[derive(Debug, Clone)]
pub struct RaceTraitFormulaFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    /// field name -> raw formula text, as the generator re-verified against
    /// the pinned oracle. Compared against `UNDINE_RACE_TRAIT_FORMULAS`
    /// below so a transcription regression in EITHER the shipped table or
    /// the fixture turns this check red, never just the arithmetic.
    pub formulas: BTreeMap<String, String>,
    /// `(TL, CON, CHA, {field: expected_value})` at each of the ten sample
    /// points `scripts/derive_race_trait_formula_fixtures.py` computed with
    /// its own from-scratch Python function per formula shape — never read
    /// back from this repo's evaluator.
    pub expected_at: Vec<(i64, i64, i64, BTreeMap<String, i64>)>,
}

/// Reads the `race_trait_formula_entries` array of the same committed
/// fixture file [`load_fixtures`] reads `entries` from.
pub fn load_race_trait_formula_fixtures(repo_root: &Path) -> Vec<RaceTraitFormulaFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("race_trait_formula_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let formulas: BTreeMap<String, String> = e["formulas"]
                .as_object()
                .expect("formulas")
                .iter()
                .map(|(k, v)| (k.clone(), v.as_str().expect("formula value").to_string()))
                .collect();
            let expected_at = e["expected_at_sample_points"]
                .as_array()
                .expect("expected_at_sample_points")
                .iter()
                .map(|p| {
                    let expected: BTreeMap<String, i64> = p["expected"]
                        .as_object()
                        .expect("expected")
                        .iter()
                        .map(|(k, v)| (k.clone(), v.as_i64().expect("expected value fits in i64")))
                        .collect();
                    (
                        p["TL"].as_i64().expect("TL"),
                        p["CON"].as_i64().expect("CON"),
                        p["CHA"].as_i64().expect("CHA"),
                        expected,
                    )
                })
                .collect();
            RaceTraitFormulaFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                formulas,
                expected_at,
            }
        })
        .collect()
}

/// The `kind=race_trait` formula half of
/// [`crate::rules_core::derived_evaluator_fixture_check::run_bar_check`].
pub fn run_race_trait_formula_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_race_trait_formula_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    let evaluator = PcgenFormulaEvaluator;

    for fixture in &fixtures {
        let mut mismatch: Option<String> = None;

        for (field, expected_formula) in &fixture.formulas {
            let Some((_, _, shipped_formula)) =
                UNDINE_RACE_TRAIT_FORMULAS.iter().find(|(_, f, _)| f == field)
            else {
                mismatch = Some(format!(
                    "fixture names field {field:?} but UNDINE_RACE_TRAIT_FORMULAS carries no \
                     entry for it at all"
                ));
                break;
            };
            // The independence check: confirm the SHIPPED table states the
            // SAME formula text the fixture (independently re-derived from
            // the oracle) expects, not merely SOME formula for this field --
            // the same posture `run_companion_skill_bar_check` takes for
            // `parsed.plus`/`parsed.minus` against `fixture.plus_ability`.
            if shipped_formula != expected_formula {
                mismatch = Some(format!(
                    "fixture expects {field}={expected_formula:?} but UNDINE_RACE_TRAIT_FORMULAS \
                     states {field}={shipped_formula:?}"
                ));
                break;
            }
        }
        if let Some(message) = mismatch {
            failures.insert(fixture.unit_id.clone(), message);
            continue;
        }

        if fixture.expected_at.is_empty() {
            // A fixture that pins no sample point asserts nothing about the
            // evaluator. Refused rather than counted -- a gate that cannot
            // fail is worse than no gate (`decisions.md` Decision 1(a)).
            failures.insert(
                fixture.unit_id.clone(),
                format!("fixture for {:?} pins no sample point at all, so it asserts nothing", fixture.record_key),
            );
            continue;
        }

        let mut all_matched = true;
        for (tl, con, cha, expected) in &fixture.expected_at {
            let mut vars: BTreeMap<String, i64> = BTreeMap::new();
            vars.insert("TL".to_owned(), *tl);
            vars.insert("CON".to_owned(), *con);
            vars.insert("CHA".to_owned(), *cha);

            for (field, expected_value) in expected {
                let Some((_, _, formula)) = UNDINE_RACE_TRAIT_FORMULAS.iter().find(|(_, f, _)| f == field)
                else {
                    // Already reported as a mismatch above; unreachable here.
                    all_matched = false;
                    continue;
                };
                match evaluator.evaluate(formula, &vars) {
                    Ok(actual) if actual == *expected_value => {}
                    Ok(actual) => {
                        failures.insert(
                            fixture.unit_id.clone(),
                            format!(
                                "at TL={tl} CON={con} CHA={cha}, {field} expected \
                                 {expected_value} but PcgenFormulaEvaluator produced {actual} \
                                 for formula {formula:?}"
                            ),
                        );
                        all_matched = false;
                    }
                    Err(e) => {
                        failures.insert(
                            fixture.unit_id.clone(),
                            format!(
                                "at TL={tl} CON={con} CHA={cha}, {field}'s formula {formula:?} \
                                 refused to evaluate: {e}"
                            ),
                        );
                        all_matched = false;
                    }
                }
                if !all_matched {
                    break;
                }
            }
            if !all_matched {
                break;
            }
        }
        if all_matched {
            cleared.insert(fixture.unit_id.clone());
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

#[cfg(test)]
mod race_trait_formula_bar_check_tests {
    use super::*;
    use crate::pcgen_import::formula_reproduction_harness::FormulaEvalError;

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(
            std::env::var("CODEX_REPO_ROOT").unwrap_or_else(|_| ".".to_string()),
        )
    }

    /// The real gate, run against the real committed fixture and the real
    /// shipped `UNDINE_RACE_TRAIT_FORMULAS` table: every entry must clear.
    #[test]
    fn run_race_trait_formula_bar_check_clears_every_committed_fixture() {
        let report = run_race_trait_formula_bar_check(&repo_root());
        assert!(
            report.failures.is_empty(),
            "every committed race_trait_formula fixture must clear: {:?}",
            report.failures
        );
        assert!(report.engine_does_not_hold.is_empty());
        assert_eq!(report.fixtures_total, 3, "3 Undine alternate-trait records are fixture-pinned");
        assert_eq!(report.cleared.len(), 3);
    }

    /// Anti-gaming mutation proof (Decision 1(a)): a wrong-but-plausible
    /// evaluator must be caught. Mirrors `harness_detects_a_deliberately_
    /// wrong_evaluator` in `formula_reproduction_harness.rs` and every other
    /// bar check's own mutation test in this file -- a gate that cannot
    /// fail is worse than no gate.
    struct OffByOneEvaluator;
    impl FormulaEvaluator for OffByOneEvaluator {
        fn evaluate(
            &self,
            formula: &str,
            vars: &BTreeMap<String, i64>,
        ) -> Result<i64, FormulaEvalError> {
            PcgenFormulaEvaluator.evaluate(formula, vars).map(|v| v + 1)
        }
    }

    #[test]
    fn a_mutated_evaluator_is_caught_by_the_race_trait_formula_gate() {
        let fixtures = load_race_trait_formula_fixtures(&repo_root());
        assert!(!fixtures.is_empty(), "the committed fixture must carry at least one entry");
        let evaluator = OffByOneEvaluator;
        let mut any_mismatch = false;
        for fixture in &fixtures {
            for (tl, con, cha, expected) in &fixture.expected_at {
                let mut vars: BTreeMap<String, i64> = BTreeMap::new();
                vars.insert("TL".to_owned(), *tl);
                vars.insert("CON".to_owned(), *con);
                vars.insert("CHA".to_owned(), *cha);
                for (field, expected_value) in expected {
                    let (_, _, formula) =
                        UNDINE_RACE_TRAIT_FORMULAS.iter().find(|(_, f, _)| f == field).unwrap();
                    let actual = evaluator.evaluate(formula, &vars).unwrap();
                    if actual != *expected_value {
                        any_mismatch = true;
                    }
                }
            }
        }
        assert!(
            any_mismatch,
            "an evaluator that is off by one on every result must disagree with at least one \
             pinned expected value -- if this fails, the fixture itself cannot detect a wrong \
             evaluator"
        );
    }

    /// The shipped table and the committed fixture must state the IDENTICAL
    /// formula text for every field -- proves the independence check inside
    /// `run_race_trait_formula_bar_check` itself is reachable and correct,
    /// not merely present in the source.
    #[test]
    fn a_transcription_regression_in_the_shipped_table_is_caught() {
        let fixtures = load_race_trait_formula_fixtures(&repo_root());
        for fixture in &fixtures {
            for (field, expected_formula) in &fixture.formulas {
                let (_, _, shipped_formula) =
                    UNDINE_RACE_TRAIT_FORMULAS.iter().find(|(_, f, _)| f == field).unwrap_or_else(|| {
                        panic!("UNDINE_RACE_TRAIT_FORMULAS carries no entry for {field:?}")
                    });
                assert_eq!(
                    shipped_formula, expected_formula,
                    "shipped formula for {field} must match the independently-derived fixture"
                );
            }
        }
    }
}
