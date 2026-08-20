//! One-off generation entry point for the `class_feature` JSON cache
//! (SD-31 `epic-5-chassis-sweep` F1, `SD31-E5-F1-001`). Run via
//! `cargo run --locked --bin gen_cache_class_feature` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `$HOME/workspace/repos/pcgen/data`) and
//! `CLASS_FEATURE_INVENTORY_JSON` pointing at a `docs/work-inventory.json`-
//! shaped document to source `(book, source_file, source_line, key, name)`
//! citations from (defaults to `docs/work-inventory.json`).
//!
//! Writes `data/corpus/<book>/class_feature/<class>/<feature>.json` for
//! every one of [`class_feature::BOOK_PRIMARY_FILES`]' 22 books.

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::class_feature;

fn real_now_iso8601() -> String {
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("system `date` must be available to stamp ingested_at");
    String::from_utf8(output.stdout).expect("date output is valid UTF-8").trim().to_string()
}

fn main() {
    let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let inventory_path = std::env::var("CLASS_FEATURE_INVENTORY_JSON")
        .unwrap_or_else(|_| format!("{manifest_dir}/docs/work-inventory.json"));
    let inventory_json = std::fs::read_to_string(&inventory_path)
        .unwrap_or_else(|e| panic!("could not read {inventory_path}: {e}"));
    let units = class_feature::units_from_inventory_json(&inventory_json)
        .unwrap_or_else(|e| panic!("could not parse {inventory_path}: {e}"));

    let out_dir = PathBuf::from(manifest_dir).join("data/corpus");
    let grants_root = PathBuf::from(manifest_dir).join("data/class_feature_grants");
    let ingested_at = real_now_iso8601();

    match class_feature::generate(&corpus_root, &grants_root, &out_dir, &ingested_at, &units) {
        Ok(report) => {
            println!(
                "class_feature cache generated: {} records across {} books; {} skipped (NAMEISPI:YES); ingested_at={ingested_at}",
                report.written,
                report.books_written.len(),
                report.name_pi_skipped,
            );
            if !report.unresolved_citations.is_empty() {
                eprintln!(
                    "WARNING: {} citation(s) did not resolve: {:?}",
                    report.unresolved_citations.len(),
                    report.unresolved_citations
                );
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("class_feature cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
