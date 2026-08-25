//! One-off generation entry point for the hand-authored-but-never-dumped
//! per-book equipment JSON cache (SD-31 `epic-6-ingest-lanes` F5,
//! `SD31-E6-F5-003`). Run via `cargo run --locked --bin
//! gen_cache_hand_authored_equipment` with `PCGEN_CORPUS_ROOT` pointing at
//! a local PCGen `data/` checkout (defaults to
//! `$HOME/workspace/repos/pcgen/data`). Writes `data/corpus/<book>/
//! equipment/*.json` for Ultimate Psionics, Ultimate Combat, Ultimate
//! Intrigue, and Ultimate Magic's already-compiled `equipment_tables()`
//! rows -- see `cache_gen::hand_authored_equipment`'s module doc comment
//! for why these four books' equipment was never dumped despite already
//! being wired into the player-visible equipment catalog.
//!
//! An unresolved citation does NOT fail this binary, matching
//! `gen_cache_equipment_gap`'s own precedent: a handful of misses should
//! not block landing the hundreds of rows that DID resolve. The binary
//! exits non-zero only if NOTHING resolved at all.

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::hand_authored_equipment;

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

    match hand_authored_equipment::generate(&corpus_root, &out_root, &ingested_at) {
        Ok(report) => {
            println!(
                "Hand-authored equipment cache generated: {} equipment, {} equipment_modifier records; ingested_at={ingested_at}",
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
                    "NOTE: {} record(s) ingested under a Codex-generated neutral name (decisions.md §24, name-field PI), by coordinate: {:?}",
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
            eprintln!("Hand-authored equipment cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
