//! One-off generation entry point for the equipment/equipment_modifier
//! "gap" JSON cache (SD-31 `epic-6-ingest-lanes` F5/F6, `SD31-E6-F5-002`).
//! Run via `cargo run --locked --bin gen_cache_equipment_gap` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `$HOME/workspace/repos/pcgen/data`). Writes
//! `data/corpus/<book>/equipment/*.json` for every book
//! `rules_tables::equipment_gap_tables` covers except `ultimate_equipment`
//! (`cache_gen::ultimate_equipment` owns that book's directory).
//!
//! Unlike `gen_cache_ultimate_equipment`, an unresolved citation does NOT
//! fail this binary: with 8 books and up to 3 fallback file/field
//! strategies each, a single miss should not block landing the hundreds
//! of rows that DID resolve. Every unresolved row is still reported by
//! name and the binary exits non-zero only if NOTHING resolved at all
//! (a real corpus-unreachable condition, not a partial-coverage one).

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::equipment_gap;

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
            let home = std::env::var("HOME")
                .expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let out_root = PathBuf::from(manifest_dir).join("data/corpus");

    let ingested_at = real_now_iso8601();

    match equipment_gap::generate(&corpus_root, &out_root, &ingested_at) {
        Ok(report) => {
            println!(
                "Equipment gap cache generated: {} equipment, {} equipment_modifier records; ingested_at={ingested_at}",
                report.equipment_written, report.equipment_modifier_written
            );
            if !report.unresolved_citations.is_empty() {
                eprintln!(
                    "WARNING: {} record(s) had no resolvable LST citation (skipped, not written): {:?}",
                    report.unresolved_citations.len(),
                    report.unresolved_citations
                );
            }
            if !report.name_pi_excluded.is_empty() {
                eprintln!(
                    "NOTE: {} record(s) excluded whole (not redacted) for name-field PI: {:?}",
                    report.name_pi_excluded.len(),
                    report.name_pi_excluded
                );
            }
            if !report.skipped_pre_existing.is_empty() {
                eprintln!(
                    "NOTE: {} record(s) skipped -- an already-shipped file already claims that slug: {:?}",
                    report.skipped_pre_existing.len(),
                    report.skipped_pre_existing
                );
            }
            if report.equipment_written == 0 && report.equipment_modifier_written == 0 {
                eprintln!("FATAL: zero records written -- corpus likely unreachable.");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Equipment gap cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
