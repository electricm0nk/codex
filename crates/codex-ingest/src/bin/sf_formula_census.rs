//! Write the Starfinder formula-system census (SD-37 E3.2) that
//! `scripts/token_coverage.py --sf-formula` sums.
//!
//! ```text
//! cargo run --locked -p codex-ingest --bin sf_formula_census [-- --check] [-- --out <path>]
//! ```
//!
//! Reads the pinned oracle through `closure::corpus_root()` (`$PCGEN_CORPUS_ROOT`). Without
//! `--check` it writes the census and exits 0. With `--check` it also exits 1 when the file on
//! disk differed from the fresh census (and rewrites it, so the fix is one commit). A read
//! failure exits 2. The last line is `tokens=<n> mapped=<n> refused=<n> … verdict=<…>`.

use std::path::PathBuf;
use std::process::ExitCode;

use codex::rules_core::game_system::GameSystem;
use codex_ingest::pcgen_import::sheet_rule::closure;
use codex_ingest::pcgen_import::sheet_rule::formula_system::{census_json, read_formula_system, render_census};

const DEFAULT_OUT: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_3/formula-system/sf-formula-census.json";

fn main() -> ExitCode {
    let mut check = false;
    let mut out = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(DEFAULT_OUT);
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--check" => check = true,
            "--out" => match args.next() {
                Some(p) => out = PathBuf::from(p),
                None => {
                    eprintln!("--out needs a path");
                    return ExitCode::from(2);
                }
            },
            other => {
                eprintln!("unknown argument {other:?}");
                return ExitCode::from(2);
            }
        }
    }
    let root = closure::corpus_root();
    let reading = match read_formula_system(GameSystem::Starfinder1e, &root) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("READ_ERROR: {e}");
            println!("verdict=READ_ERROR");
            return ExitCode::from(2);
        }
    };
    let census = census_json(&reading);
    let fresh = render_census(&census);
    let stale = std::fs::read_to_string(&out).map(|s| s != fresh).unwrap_or(true);
    if let Some(dir) = out.parent()
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        eprintln!("{}: {e}", dir.display());
        return ExitCode::from(2);
    }
    if let Err(e) = std::fs::write(&out, &fresh) {
        eprintln!("{}: {e}", out.display());
        return ExitCode::from(2);
    }
    let t = &census["totals"];
    for (reason, n) in t["refused_by_reason"].as_object().into_iter().flatten() {
        println!("refused {n}: {reason}");
    }
    let verdict = if check && stale { "FAIL_STALE_CENSUS" } else { "PASS" };
    println!(
        "files_read={} tokens={} mapped={} refused={} variables={} channels={} functions={} datatables={} out={} verdict={verdict}",
        reading.files_read.len(),
        t["tokens"],
        t["mapped"],
        t["refused"],
        reading.declarations.variables.len(),
        reading.declarations.channels.len(),
        reading.declarations.functions.len(),
        reading.declarations.datatables.len(),
        out.display(),
    );
    if verdict == "PASS" { ExitCode::SUCCESS } else { ExitCode::from(1) }
}
