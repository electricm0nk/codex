//! SD-32 Gate 2 corpus-wide run CLI (`gate-2-corpus-wide-runs`, kanban `#8`)
//! for `src/rules_core/pilot_compute/formula_interpreter.rs` (F1..F9,
//! card 6's engine). Implements AT-32-G2-004's own verification command:
//!
//! ```text
//! cargo run --locked --bin formula_interpreter -- --corpus-wide \
//!   --output artifacts/gate-2-engines/formula_interpreter.corpus-wide.json
//! ```
//!
//! Thin CLI wrapper, same shape as `derived_evaluator_fixture_check.rs`: the
//! real logic lives in
//! [`codex::rules_core::pilot_compute::formula_interpreter_corpus_wide::run_corpus_wide_scan`],
//! this binary only parses arguments, runs it, and reports.
//!
//! Exit codes: `0` the corpus-wide run completed and its population matched
//! the closed Gate 1 census (AT-32-G2-004's own fixture-check); `1` the scan
//! could not read its inputs, or its population disagreed with the closed
//! census -- a cycle that ran against a subset must fail loudly here, not
//! report a partial run as complete.

use codex::rules_core::pilot_compute::formula_interpreter_corpus_wide::{
    run_corpus_wide_scan, ScanError,
};
use std::path::PathBuf;
use std::process::ExitCode;

const LABEL: &str = "formula-interpreter-corpus-wide";

fn main() -> ExitCode {
    let mut corpus_wide = false;
    let mut output: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--corpus-wide" => corpus_wide = true,
            "--output" => match args.next() {
                Some(v) => output = Some(PathBuf::from(v)),
                None => return fatal("--output needs a path"),
            },
            "--help" => {
                println!(
                    "{LABEL}: usage: formula_interpreter --corpus-wide --output <path>\n\
                     Runs the production PcgenFormulaEvaluator grammar (recognises_shape) \
                     against every unit the closed Gate 1 census (artifacts/gate-1-shape-closure/\
                     ledger.json) placed in family F1..F9, fixture-checking the run's own \
                     population against that census before reporting. See the module doc of \
                     codex::rules_core::pilot_compute::formula_interpreter_corpus_wide."
                );
                return ExitCode::SUCCESS;
            }
            other => return fatal(&format!("unknown argument: {other}")),
        }
    }

    if !corpus_wide {
        return fatal("--corpus-wide is required (this CLI has no other mode yet)");
    }

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let report = match run_corpus_wide_scan(&repo_root) {
        Ok(r) => r,
        Err(ScanError::MissingInput(m)) => return fatal(&format!("input missing: {m}")),
        Err(e @ ScanError::PopulationMismatch { .. }) => return fatal(&e.to_string()),
    };

    let mut families_json = String::new();
    for (family, cov) in &report.families {
        if !families_json.is_empty() {
            families_json.push(',');
        }
        let samples: Vec<String> =
            cov.refusal_samples.iter().map(|s| format!("{s:?}")).collect();
        families_json.push_str(&format!(
            "{family:?}:{{\"population\":{},\"recognised_units\":{},\"refused_units\":{},\
             \"unjoined_units\":{},\"refusal_samples\":[{}]}}",
            cov.population,
            cov.recognised_units,
            cov.refused_units,
            cov.unjoined_units,
            samples.join(",")
        ));
    }
    let body = format!(
        "{{\"engine\":\"formula_interpreter\",\"scope\":\"F1..F9\",\"total_population\":{},\
         \"total_recognised_units\":{},\"total_refused_units\":{},\"total_unjoined_units\":{},\
         \"families\":{{{families_json}}}}}\n",
        report.total_population,
        report.total_recognised_units,
        report.total_refused_units,
        report.total_unjoined_units,
    );

    if let Some(path) = &output {
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                return fatal(&format!("--output: cannot create {}: {e}", parent.display()));
            }
        }
        if let Err(e) = std::fs::write(path, &body) {
            return fatal(&format!("--output: cannot write {}: {e}", path.display()));
        }
    }

    println!(
        "{LABEL}: population={} (matches closed Gate 1 census) recognised={} refused={} \
         unjoined={}",
        report.total_population,
        report.total_recognised_units,
        report.total_refused_units,
        report.total_unjoined_units
    );
    for (family, cov) in &report.families {
        println!(
            "{LABEL}: {family} population={} recognised={} refused={}",
            cov.population, cov.recognised_units, cov.refused_units
        );
    }

    ExitCode::SUCCESS
}

fn fatal(msg: &str) -> ExitCode {
    eprintln!("{LABEL}: FATAL: {msg}");
    ExitCode::FAILURE
}
