//! Generate `data/converted/record_vars.json` -- every corpus record's variable chain, converted
//! at ingest into our own `Expr` (SD-35 `AT-35-E6-001`, `decisions.md` §11).
//!
//! ```text
//! cargo run --locked --bin gen_record_vars            # write the artifact
//! cargo run --locked --bin gen_record_vars -- --check # fail if the shipped artifact is stale
//! ```
//!
//! The live side (`rules_core::record_vars`) reads only what this writes; it never parses or
//! evaluates a source formula. `--check` is the freshness gate: it re-runs the conversion and
//! compares byte for byte, so a corpus change that moves a chain cannot ship silently.

use std::path::PathBuf;

use codex::pcgen_import::class_feature_vars;
use codex::rules_core::record_vars::RECORD_VARS_PATH;

fn main() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_path = repo.join(RECORD_VARS_PATH);
    let check = std::env::args().any(|a| a == "--check");
    let started = std::time::Instant::now();

    let package = class_feature_vars::build(&repo);
    // Compact, not pretty: 4,445 records of converted expression trees are diffed by
    // regenerating and comparing, never by reading, and pretty-printing quadruples the size.
    let mut text = serde_json::to_string(&package).expect("serialise record vars");
    text.push('\n');

    let described = package.class_feature_described.len();
    let any = package.class_feature_any.len();
    let classes = package.class_records.len();
    let domains = package.domain_records.len();
    let defaults = package.var_defaults.len();
    let converted: usize = package
        .class_feature_any
        .values()
        .chain(package.class_records.values())
        .chain(package.domain_records.values())
        .map(|c| c.len())
        .sum();
    let summary = format!(
        "class_feature_described={described} class_feature_any={any} class_records={classes} \
         domain_records={domains} converted_vars={converted} var_defaults={defaults} ({:.1}s)",
        started.elapsed().as_secs_f64()
    );

    if check {
        match std::fs::read_to_string(&out_path) {
            Ok(on_disk) if on_disk == text => {
                println!("{summary} verdict=PASS");
            }
            Ok(_) => {
                eprintln!(
                    "{} is stale -- re-run `cargo run --locked --bin gen_record_vars`",
                    out_path.display()
                );
                println!("{summary} verdict=FAIL");
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("{}: {e}", out_path.display());
                println!("{summary} verdict=FAIL");
                std::process::exit(1);
            }
        }
        return;
    }

    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).expect("create data/converted");
    }
    std::fs::write(&out_path, text).expect("write record vars");
    println!("{summary} -> {}", out_path.display());
}
