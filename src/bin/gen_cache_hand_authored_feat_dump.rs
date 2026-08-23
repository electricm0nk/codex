//! One-off generation entry point for the hand-authored feat table JSON
//! cache (SD-32 `decisions.md §20`). Run via
//! `cargo run --locked --bin gen_cache_hand_authored_feat_dump` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `$HOME/workspace/repos/pcgen/data`). Writes
//! `data/corpus/<book>/feat/*.json` for `core_rulebook`,
//! `ultimate_psionics`, `advanced_class_guide`, `ultimate_campaign`.
//!
//! Mirrors `gen_cache_feat_gap.rs`'s posture: an unresolved citation does
//! NOT fail this binary; it exits non-zero only if nothing at all
//! resolved.

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::hand_authored_feat_dump;

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

    match hand_authored_feat_dump::generate(&corpus_root, &out_root, &ingested_at) {
        Ok(report) => {
            println!(
                "Hand-authored feat dump generated: {} records; ingested_at={ingested_at}",
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
            eprintln!("hand-authored feat dump generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
