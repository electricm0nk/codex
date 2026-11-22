//! Provenance gate CLI: sweep the rules-table code for Product-Identity
//! blacklist terms and reconcile against `docs/governance/pi-sweep-baseline.tsv`.
//! The code swept is `src/rules_core/rules_catalog/**/*.rs` (SD-37 E4a.4 removed
//! the compiled `src/rules_core/rules_tables/**/*.rs` this first swept; its rows
//! are the data package below, its types and formulas the catalog).
//!
//! SD-37 E4a.1: the same gate covers the `rules_tables` data package
//! (`data/rules_tables/**/*.json`). Every package file's rows are re-screened
//! (`rules_data_package::sweep_package`); each unredacted hit is reconciled
//! against the same baseline (keyed `data/rules_tables/<table id>.json`), and a
//! file whose licence/PI stamp disagrees with the re-screen fails the gate.
//!
//! Run by `scripts/verify.sh --only pi-sweep` and by every kind lane before
//! its first content commit; the lane pastes this output into its cycle
//! receipt per `docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md
//! §37.3` / acceptance criterion AT-29-003a. A lane generating a table in
//! process calls `pi_table_sweep::screen_generated_table` instead, before the
//! write — this binary is the standing check that nothing landed by any other
//! path.
//!
//! Exit codes: `0` clean (every hit accounted for by the baseline, no stale
//! baseline row), `1` a hit the baseline does not account for or a stale row,
//! `2` an I/O or parse failure.
//!
//! Usage: `pi_sweep_rules_tables [--repo-root <path>] [--quiet]`

use codex::rules_core::pi_table_sweep::{parse_baseline, reconcile, sweep_dir};
use codex::rules_core::rules_data_package::{package_root, sweep_package, PACKAGE_RELATIVE};
use std::path::PathBuf;
use std::process::ExitCode;

const BASELINE_REL: &str = "docs/governance/pi-sweep-baseline.tsv";
const TABLES_REL: &str = "src/rules_core/rules_catalog";

fn main() -> ExitCode {
    let mut repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut quiet = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--repo-root" => match args.next() {
                Some(v) => repo_root = PathBuf::from(v),
                None => {
                    eprintln!("pi_sweep_rules_tables: --repo-root needs a path");
                    return ExitCode::from(2);
                }
            },
            "--quiet" => quiet = true,
            other => {
                eprintln!("pi_sweep_rules_tables: unknown argument: {other}");
                return ExitCode::from(2);
            }
        }
    }

    let mut hits = match sweep_dir(&repo_root.join(TABLES_REL)) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("pi_sweep_rules_tables: sweep failed: {e}");
            return ExitCode::from(2);
        }
    };
    let rust_hits = hits.len();
    let (package_hits, bad_stamps) = match sweep_package(&package_root(&repo_root)) {
        Ok(found) => found,
        Err(e) => {
            eprintln!("pi_sweep_rules_tables: package sweep failed: {e}");
            return ExitCode::from(2);
        }
    };
    let package_hit_count = package_hits.len();
    hits.extend(package_hits);
    let baseline_text = match std::fs::read_to_string(repo_root.join(BASELINE_REL)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("pi_sweep_rules_tables: cannot read {BASELINE_REL}: {e}");
            return ExitCode::from(2);
        }
    };
    let baseline = match parse_baseline(&baseline_text) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("pi_sweep_rules_tables: {e}");
            return ExitCode::from(2);
        }
    };

    let verdict = reconcile(&hits, &baseline);

    if !quiet {
        println!(
            "pi-sweep: {} hits over {} ({rust_hits}) + {} ({package_hit_count}), {} baseline rows, {} package stamps disagree",
            hits.len(),
            TABLES_REL,
            PACKAGE_RELATIVE,
            baseline.len(),
            bad_stamps.len()
        );
    }

    for stamp in &bad_stamps {
        println!("pi-sweep: BAD PACKAGE STAMP {stamp}");
    }
    if verdict.unbaselined.is_empty() && verdict.stale.is_empty() && bad_stamps.is_empty() {
        if !quiet {
            println!("pi-sweep: CLEAN — no unbaselined Product-Identity hits");
        }
        return ExitCode::SUCCESS;
    }

    for hit in &verdict.unbaselined {
        println!("pi-sweep: UNBASELINED {}:{} [{}] {}", hit.file, hit.line, hit.term, hit.context);
    }
    for entry in &verdict.stale {
        println!(
            "pi-sweep: STALE BASELINE ROW {}\t{}\t{} (tree no longer carries it — remove the row)",
            entry.file, entry.term, entry.count
        );
    }
    println!(
        "pi-sweep: FAIL — {} unbaselined hit(s), {} stale row(s), {} bad package stamp(s). A hit is a hard stop for that record.",
        verdict.unbaselined.len(),
        verdict.stale.len(),
        bad_stamps.len()
    );
    ExitCode::from(1)
}
