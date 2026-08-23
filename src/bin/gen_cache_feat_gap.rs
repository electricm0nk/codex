//! One-off generation entry point for the feat "gap" JSON cache
//! (SD-32 `decisions.md §20`). Run via
//! `cargo run --locked --bin gen_cache_feat_gap` with `PCGEN_CORPUS_ROOT`
//! pointing at a local PCGen `data/` checkout (defaults to
//! `$HOME/workspace/repos/pcgen/data`). Writes `data/corpus/<book>/feat/
//! *.json` for every book `rules_tables::feat_gap_tables` covers.
//!
//! Mirrors `gen_cache_equipment_gap.rs`'s own posture exactly: an
//! unresolved citation does NOT fail this binary (19 books, one file-set
//! each; a single miss should not block landing the hundreds of rows that
//! DID resolve). Every unresolved row is still reported by name and the
//! binary exits non-zero only if NOTHING resolved at all.

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::feat_gap;

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

    match feat_gap::generate(&corpus_root, &out_root, &ingested_at) {
        Ok(report) => {
            println!(
                "Feat gap cache generated: {} feat records; ingested_at={ingested_at}",
                report.feats_written
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
                    "NOTE: {} record(s) skipped -- a different record already claims that slug: {:?}",
                    report.skipped_pre_existing.len(),
                    report.skipped_pre_existing
                );
            }
            if report.feats_written == 0 {
                eprintln!("FATAL: zero records written -- corpus unreachable or entirely PI-blocked");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("feat gap cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
