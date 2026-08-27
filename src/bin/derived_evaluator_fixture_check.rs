//! Derived-fixture evaluator-vs-fixture gate CLI: reports which `derived`
//! wiring-class units the engine's evaluator matches against
//! `tests/fixtures/rules_core/derived-evaluator-fixtures.json`.
//!
//! This is the check a `derived` unit's bar has always named and, until now,
//! nothing outside the test suite could produce. The bar check itself lives
//! in [`codex::rules_core::derived_evaluator_fixture_check::run_bar_check`],
//! the exact function `tests/derived_evaluator_fixture_check.rs` asserts
//! against — this binary is a thin CLI wrapper that reports it as JSON for
//! `v06_work_inventory` to join, and does not decide anyone's doneness
//! verdict or write anything.
//!
//! Unlike `corpus_literal_sweep`, a per-book failure does not poison the
//! whole report: the fixture's coverage is deliberately narrow (94 of 2,879
//! held `derived` units, see the test file's module doc), each entry stands
//! on its own corpus record, and the `UNCLEARED` register in the test file
//! already tracks the one known structural failure. `--json-out` lists only
//! the `unit_id`s that actually cleared -- the same set the test suite's
//! `cleared` count reports.
//!
//! Exit codes: `0` the bar check ran (regardless of how many units cleared),
//! `1` the fixture is empty or unreadable, which would make the report
//! vacuous.
//!
//! Usage: `derived_evaluator_fixture_check [--json-out <path>] [--quiet]`

use codex::rules_core::derived_evaluator_fixture_check::run_bar_check;
use std::path::PathBuf;
use std::process::ExitCode;

const LABEL: &str = "derived-evaluator-fixture-check";

fn main() -> ExitCode {
    let mut json_out: Option<PathBuf> = None;
    let mut quiet = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--json-out" => match args.next() {
                Some(v) => json_out = Some(PathBuf::from(v)),
                None => return fatal("--json-out needs a path"),
            },
            "--quiet" => quiet = true,
            other => return fatal(&format!("unknown argument: {other}")),
        }
    }

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let report = run_bar_check(&repo_root);

    if report.fixtures_total == 0 {
        return fatal("the fixture is empty, which would make every assertion vacuously true");
    }

    if let Some(path) = &json_out {
        let verified: Vec<String> =
            report.cleared.iter().map(|id| format!("{:?}", id)).collect();
        let body = format!(
            "{{\"fixtures_total\":{},\"cleared\":{},\"failed\":{},\"engine_does_not_hold\":{},\"verified\":[{}]}}\n",
            report.fixtures_total,
            report.cleared.len(),
            report.failures.len(),
            report.engine_does_not_hold.len(),
            verified.join(",")
        );
        if let Err(e) = std::fs::write(path, body) {
            return fatal(&format!("--json-out: cannot write {}: {e}", path.display()));
        }
    }

    if !quiet {
        // "units cleared" and "fixture rows" are two different counts and this
        // line used to conflate them, which was true only while every seam
        // emitted exactly one row per unit. The `monster_sla` seam
        // (SD31-W15-MONSTER-SLA-001) emits one row per GRANTED SPELL and
        // clears a unit only when every one of its rows clears, so
        // `fixtures_total` is now strictly ≥ the number of covered units.
        // Reporting them as one number would have read as 102 silently
        // uncleared units.
        println!(
            "{LABEL}: {} unit(s) cleared over {} fixture row(s); {} failed; {} not ingested",
            report.cleared.len(),
            report.fixtures_total,
            report.failures.len(),
            report.engine_does_not_hold.len()
        );
        for (id, reason) in &report.failures {
            println!("{LABEL}: FAIL {id}: {reason}");
        }
    }

    ExitCode::SUCCESS
}

fn fatal(message: &str) -> ExitCode {
    eprintln!("{LABEL}: {message}");
    ExitCode::from(2)
}
