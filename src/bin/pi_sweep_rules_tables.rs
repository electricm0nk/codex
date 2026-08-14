//! Provenance gate CLI: sweep `src/rules_core/rules_tables/**/*.rs` for
//! Product-Identity blacklist terms and reconcile against
//! `docs/governance/pi-sweep-baseline.tsv`.
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
use std::path::PathBuf;
use std::process::ExitCode;

const BASELINE_REL: &str = "docs/governance/pi-sweep-baseline.tsv";
const TABLES_REL: &str = "src/rules_core/rules_tables";

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

    let hits = match sweep_dir(&repo_root.join(TABLES_REL)) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("pi_sweep_rules_tables: sweep failed: {e}");
            return ExitCode::from(2);
        }
    };
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
            "pi-sweep: {} hits over {}, {} baseline rows",
            hits.len(),
            TABLES_REL,
            baseline.len()
        );
    }

    if verdict.unbaselined.is_empty() && verdict.stale.is_empty() {
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
        "pi-sweep: FAIL — {} unbaselined hit(s), {} stale row(s). A hit is a hard stop for that record.",
        verdict.unbaselined.len(),
        verdict.stale.len()
    );
    ExitCode::from(1)
}
