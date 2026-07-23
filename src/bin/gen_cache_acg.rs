//! One-off generation entry point for the ACG JSON cache (SD-26 Epic 3,
//! Criterion 3.3). Run via `cargo run --bin gen_cache_acg` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `/home/ubuntu/workspace/repos/pcgen/data`, the same
//! default `gen_cache_apg`/`tests/sd17_b5_equipment.rs` and siblings
//! already use). Writes
//! `data/corpus/advanced_class_guide/{class,spell,equipment}/*.json`.
//!
//! This binary is the generator itself, not a standing production
//! surface -- it is re-run only when
//! `codex::rules_core::rules_tables::acg` changes (per `decisions.md
//! §11.3`, the cache dumps that module's current state; it does not run
//! at app runtime).

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::acg;

fn real_now_iso8601() -> String {
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("system `date` must be available to stamp ingested_at");
    String::from_utf8(output.stdout)
        .expect("date output is valid UTF-8")
        .trim()
        .to_string()
}

fn main() {
    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .unwrap_or_else(|_| "/home/ubuntu/workspace/repos/pcgen/data".to_string());
    let corpus_root = PathBuf::from(corpus_root);

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let out_dir = PathBuf::from(manifest_dir).join("data/corpus/advanced_class_guide");

    let ingested_at = real_now_iso8601();

    match acg::generate(&corpus_root, &out_dir, &ingested_at) {
        Ok(report) => {
            println!(
                "ACG cache generated: {} classes, {} spells, {} equipment records; ingested_at={ingested_at}",
                report.classes_written, report.spells_written, report.equipment_written
            );
            if !report.unresolved_citations.is_empty() {
                eprintln!(
                    "WARNING: {} record(s) had no resolvable LST citation: {:?}",
                    report.unresolved_citations.len(),
                    report.unresolved_citations
                );
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("ACG cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
