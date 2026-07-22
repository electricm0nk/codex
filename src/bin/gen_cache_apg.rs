//! One-off generation entry point for the APG JSON cache (SD-26 Epic 3,
//! Criterion 3.2). Run via `cargo run --bin gen_cache_apg` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `/home/ubuntu/workspace/repos/pcgen/data`, the same
//! default `tests/sd17_b5_equipment.rs` and siblings already use).
//! Writes `data/corpus/advanced_players_guide/{class,spell,equipment}/*.json`.
//!
//! This binary is the generator itself, not a standing production
//! surface -- it is re-run only when `codex::rules_core::rules_tables::apg`
//! changes (per `decisions.md §11.3`, the cache dumps that module's
//! current state; it does not run at app runtime).

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::apg;

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
    let out_dir = PathBuf::from(manifest_dir).join("data/corpus/advanced_players_guide");

    let ingested_at = real_now_iso8601();
    // Real commit date of the SD-25 `corpus-intake-apg-description` +
    // `corpus-intake-apg-spell-text` cycles (the actual web second-source
    // research this cache dumps) -- see `apg.rs`'s doc comment for why
    // this proxy is used instead of a per-fetch timestamp.
    let fetched_at_web = "2026-07-22T08:37:22-04:00";

    match apg::generate(&corpus_root, &out_dir, &ingested_at, fetched_at_web) {
        Ok(report) => {
            println!(
                "APG cache generated: {} classes, {} spells, {} equipment records; ingested_at={ingested_at}",
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
            eprintln!("APG cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
