//! F10 binding-layer engine CLI: corpus-wide run and fixture-check for
//! `codex::rules_core::pilot_compute::bonus_stack_reader` (kanban card 8,
//! `gate-2-corpus-wide-runs`, AT-32-G2-004).
//!
//! `card 7`'s cycle (`docs/release/SD-32-.../artifacts/gate-2-engines/007_cycle_receipt.md`)
//! generalised the library module to `resolve_producer_chain_corpus_wide` / a data-driven,
//! multi-record producer-chain resolver, but explicitly left "no `--bin bonus_stack_reader`
//! target exists yet" as open scope for this card. This binary is that target.
//!
//! Two modes:
//!
//! - `--corpus-wide --output <path>`: walks every `data/corpus/**/*.json` record (excluding
//!   `LICENSE.json`), reads each one's `data.raw_tokens`, and runs
//!   [`codex::rules_core::pilot_compute::bonus_stack_reader::resolve_all_producer_chains_corpus_wide`]
//!   over the FULL record population — the corpus-wide run AT-32-G2-004 requires ("no engine is
//!   complete until it has been run corpus-wide once... a cycle that runs an engine against a
//!   subset... is out of protocol"). Writes a deterministic JSON report to `<path>`.
//! - `--fixture-check --input <corpus-wide.json> --expected-from <expected.json>`: the
//!   corpus-wide run's own fixture-check. `<expected.json>` is a curated set of target
//!   variables' expected outcomes, hand-transcribed from real corpus bytes (never regenerated
//!   from this binary's own output — see `artifacts/gate-2-engines/bonus_stack_reader.expected.json`'s
//!   own header comment for provenance). Every variable named in `<expected.json>` must match
//!   `<corpus-wide.json>`'s outcome for it exactly, or the check fails.
//!
//! Exit codes: `0` the requested mode ran clean; `1` `--fixture-check` found a mismatch;
//! `2` an I/O, parse, or empty-population failure (an empty population or an empty expected
//! fixture asserts nothing, `corpus_literal_sweep`'s own posture, deliberately mirrored here).
//!
//! Usage: `bonus_stack_reader --corpus-wide --output <path> [--repo-root <path>] [--corpus-root <path>]`
//! or `bonus_stack_reader --fixture-check --input <path> --expected-from <path>`

use codex::rules_core::pilot_compute::bonus_stack_reader::{
    resolve_all_producer_chains_corpus_wide, CorpusWideOutcome, CorpusWideReport,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const LABEL: &str = "bonus-stack-reader";
const CORPUS_RECORDS_REL: &str = "data/corpus";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1).peekable();
    let mut mode: Option<&str> = None;
    let mut repo_root: Option<PathBuf> = None;
    let mut corpus_root: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut input: Option<PathBuf> = None;
    let mut expected_from: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--corpus-wide" => mode = Some("corpus-wide"),
            "--fixture-check" => mode = Some("fixture-check"),
            "--repo-root" => repo_root = args.next().map(PathBuf::from),
            "--corpus-root" => corpus_root = args.next().map(PathBuf::from),
            "--output" => output = args.next().map(PathBuf::from),
            "--input" => input = args.next().map(PathBuf::from),
            "--expected-from" => expected_from = args.next().map(PathBuf::from),
            "--help" => {
                println!(
                    "bonus_stack_reader --corpus-wide --output <path> [--repo-root <path>] [--corpus-root <path>]\nbonus_stack_reader --fixture-check --input <path> --expected-from <path>"
                );
                return ExitCode::SUCCESS;
            }
            other => return fatal(&format!("unknown argument: {other}")),
        }
    }

    match mode {
        Some("corpus-wide") => {
            let Some(output) = output else { return fatal("--corpus-wide needs --output <path>") };
            let repo_root = repo_root.unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
            let corpus_root = corpus_root.unwrap_or_else(|| repo_root.join(CORPUS_RECORDS_REL));
            run_corpus_wide(&repo_root, &corpus_root, &output)
        }
        Some("fixture-check") => {
            let (Some(input), Some(expected_from)) = (input, expected_from) else {
                return fatal("--fixture-check needs --input <path> and --expected-from <path>");
            };
            run_fixture_check(&input, &expected_from)
        }
        _ => fatal("pass exactly one of --corpus-wide or --fixture-check (see --help)"),
    }
}

fn run_corpus_wide(repo_root: &Path, corpus_root: &Path, output: &Path) -> ExitCode {
    if !corpus_root.is_dir() {
        return fatal(&format!("corpus root {corpus_root:?} is not a directory"));
    }
    let files = find_json_files(corpus_root);
    if files.is_empty() {
        return fatal(&format!(
            "{corpus_root:?} contains no JSON records — an empty population asserts nothing"
        ));
    }

    let mut records: Vec<Vec<(String, String)>> = Vec::with_capacity(files.len());
    let mut unreadable: Vec<String> = Vec::new();
    for path in &files {
        match read_raw_tokens(path) {
            Ok(tokens) => records.push(tokens),
            Err(reason) => {
                unreadable.push(format!("{}: {reason}", display_rel(path, repo_root)));
            }
        }
    }

    let borrowed: Vec<Vec<(&str, &str)>> = records
        .iter()
        .map(|r| r.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect())
        .collect();
    let report = resolve_all_producer_chains_corpus_wide(&borrowed);

    if report.population == 0 {
        return fatal(
            "the corpus-wide sweep found zero BONUS:VAR target variables across the whole \
             population — an empty population asserts nothing",
        );
    }

    let body = render_report_json(&report, files.len(), &unreadable);
    if let Err(e) = std::fs::write(output, body) {
        return fatal(&format!("cannot write {output:?}: {e}"));
    }

    let resolved = report
        .outcomes
        .values()
        .filter(|o| matches!(o, CorpusWideOutcome::Resolved(_)))
        .count();
    let refused = report.population - resolved;
    println!(
        "{LABEL}: {} record(s) scanned, {} unreadable, {} distinct F10 target variable(s) \
         found ({} resolved, {} refused) -> {}",
        files.len(),
        unreadable.len(),
        report.population,
        resolved,
        refused,
        output.display()
    );
    ExitCode::SUCCESS
}

fn run_fixture_check(input: &Path, expected_from: &Path) -> ExitCode {
    let input_text = match std::fs::read_to_string(input) {
        Ok(t) => t,
        Err(e) => return fatal(&format!("cannot read {input:?}: {e}")),
    };
    let expected_text = match std::fs::read_to_string(expected_from) {
        Ok(t) => t,
        Err(e) => return fatal(&format!("cannot read {expected_from:?}: {e}")),
    };
    let input_json: serde_json::Value = match serde_json::from_str(&input_text) {
        Ok(v) => v,
        Err(e) => return fatal(&format!("{input:?} is not valid JSON: {e}")),
    };
    let expected_json: serde_json::Value = match serde_json::from_str(&expected_text) {
        Ok(v) => v,
        Err(e) => return fatal(&format!("{expected_from:?} is not valid JSON: {e}")),
    };

    let Some(expected_outcomes) = expected_json.get("outcomes").and_then(|v| v.as_object()) else {
        return fatal(&format!("{expected_from:?} has no top-level \"outcomes\" object"));
    };
    if expected_outcomes.is_empty() {
        return fatal(&format!(
            "{expected_from:?}'s \"outcomes\" is empty — a fixture that checks nothing asserts \
             nothing"
        ));
    }
    let Some(input_outcomes) = input_json.get("outcomes").and_then(|v| v.as_object()) else {
        return fatal(&format!("{input:?} has no top-level \"outcomes\" object"));
    };

    let mut mismatches: Vec<String> = Vec::new();
    for (var, expected_outcome) in expected_outcomes {
        let Some(actual_outcome) = input_outcomes.get(var) else {
            mismatches.push(format!("{var}: absent from the corpus-wide run"));
            continue;
        };
        match expected_outcome.get("status").and_then(|s| s.as_str()) {
            Some("resolved") => {
                // Resolved outcomes are structural facts read directly off the corpus JSON's own
                // `data.raw_tokens` (DEFINE base + BONUS:VAR addend formulas/gates) — never
                // regenerated from this binary's own output — so an exact match on `status`,
                // `base`, and `addends` is meaningful: `expected.json`'s entry and the
                // corpus-wide run's entry are two independent transcriptions of the same real
                // bytes. Compared field-by-field (not whole-object) so `expected.json` may carry
                // its own `_sources`/`_provenance` documentation keys the CLI's own output does
                // not emit.
                let fields_match = actual_outcome.get("status") == expected_outcome.get("status")
                    && actual_outcome.get("base") == expected_outcome.get("base")
                    && actual_outcome.get("addends") == expected_outcome.get("addends");
                if !fields_match {
                    mismatches.push(format!(
                        "{var}: expected status/base/addends {expected_outcome}, got {actual_outcome}"
                    ));
                }
            }
            Some("refused") => {
                // A refusal's exact wording is generated prose, not a fact transcribable from
                // corpus bytes independently of the engine that emits it — pinning it byte-for-byte
                // would make this fixture assert engine-internals-as-written rather than the real
                // behaviour ("this variable's chain is NOT safely resolvable, because <the real
                // corpus reason>"). Check status plus a caller-named substring instead.
                let status_ok = actual_outcome.get("status").and_then(|s| s.as_str()) == Some("refused");
                let contains = expected_outcome
                    .get("reason_contains")
                    .and_then(|s| s.as_str())
                    .unwrap_or("");
                let reason_ok = actual_outcome
                    .get("reason")
                    .and_then(|s| s.as_str())
                    .is_some_and(|r| r.contains(contains));
                if !status_ok || !reason_ok {
                    mismatches.push(format!(
                        "{var}: expected refused (reason containing {contains:?}), got {actual_outcome}"
                    ));
                }
            }
            other => mismatches.push(format!(
                "{var}: {expected_from:?} entry has no recognised \"status\" (got {other:?})"
            )),
        }
    }

    if mismatches.is_empty() {
        println!(
            "{LABEL}: fixture-check OK — {} variable(s) matched their expected outcome exactly",
            expected_outcomes.len()
        );
        ExitCode::SUCCESS
    } else {
        for m in &mismatches {
            println!("{LABEL}: FAIL {m}");
        }
        println!(
            "{LABEL}: fixture-check found {} mismatch(es) of {} checked",
            mismatches.len(),
            expected_outcomes.len()
        );
        ExitCode::from(1)
    }
}

fn read_raw_tokens(path: &Path) -> Result<Vec<(String, String)>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("read failed: {e}"))?;
    let v: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("invalid JSON: {e}"))?;
    let Some(tokens) = v.pointer("/data/raw_tokens").and_then(|t| t.as_array()) else {
        // Records with no data.raw_tokens (not yet enriched, or a kind that carries none) are
        // silently absent from the F10 population -- not an error, mirroring corpus_loader.rs's
        // "an unenriched record still resolves, just thinly" posture; nothing here CLAIMS such a
        // record contributes a BONUS:VAR/DEFINE token, so skipping it costs no false coverage.
        return Ok(Vec::new());
    };
    Ok(tokens
        .iter()
        .filter_map(|t| {
            let key = t.get("key").and_then(|k| k.as_str())?;
            let value = t.get("value").and_then(|v| v.as_str())?;
            Some((key.to_string(), value.to_string()))
        })
        .collect())
}

fn render_report_json(report: &CorpusWideReport, records_scanned: usize, unreadable: &[String]) -> String {
    let mut outcomes = String::new();
    for (i, (var, outcome)) in report.outcomes.iter().enumerate() {
        if i > 0 {
            outcomes.push(',');
        }
        outcomes.push_str(&json_string(var));
        outcomes.push(':');
        outcomes.push_str(&render_outcome_json(outcome));
    }
    let resolved =
        report.outcomes.values().filter(|o| matches!(o, CorpusWideOutcome::Resolved(_))).count();
    let refused = report.population - resolved;
    let unreadable_json: Vec<String> = unreadable.iter().map(|s| json_string(s)).collect();
    format!(
        "{{\"records_scanned\":{},\"unreadable\":[{}],\"population\":{},\"resolved\":{},\"refused\":{},\"outcomes\":{{{}}}}}\n",
        records_scanned,
        unreadable_json.join(","),
        report.population,
        resolved,
        refused,
        outcomes
    )
}

fn render_outcome_json(outcome: &CorpusWideOutcome) -> String {
    match outcome {
        CorpusWideOutcome::Resolved(chain) => {
            let base = match &chain.base {
                Some(b) => json_string(b),
                None => "null".to_string(),
            };
            let addends: Vec<String> = chain
                .addends
                .iter()
                .map(|a| {
                    let gate = match &a.gate {
                        Some(g) => format!(
                            "{{\"variable\":{},\"threshold\":{}}}",
                            json_string(&g.variable),
                            g.threshold
                        ),
                        None => "null".to_string(),
                    };
                    format!("{{\"formula\":{},\"gate\":{}}}", json_string(&a.formula), gate)
                })
                .collect();
            format!(
                "{{\"status\":\"resolved\",\"base\":{},\"addends\":[{}]}}",
                base,
                addends.join(",")
            )
        }
        CorpusWideOutcome::Refused(reason) => {
            format!("{{\"status\":\"refused\",\"reason\":{}}}", json_string(reason))
        }
    }
}

fn find_json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().and_then(|f| f.to_str()) == Some("_parity") {
                    continue;
                }
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|f| f.to_str()) != Some("LICENSE.json")
            {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn display_rel(path: &Path, repo_root: &Path) -> String {
    path.strip_prefix(repo_root).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn fatal(message: &str) -> ExitCode {
    eprintln!("{LABEL}: {message}");
    ExitCode::from(2)
}
