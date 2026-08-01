//! One-off generation entry point for the Bestiary 1 JSON cache (SD-26
//! Epic 3, Criterion 3.4). Run via `cargo run --bin gen_cache_beastiary`
//! with `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `$HOME/workspace/repos/pcgen/data`, the same
//! default `gen_cache_apg`/`gen_cache_acg` and siblings already use).
//! Writes `data/corpus/beastiary/{monster,equipment}/*.json`.
//!
//! This binary is the generator itself, not a standing production
//! surface -- it is re-run only when
//! `codex::rules_core::rules_tables::beastiary1` changes (per
//! `decisions.md §11.3`, the cache dumps that module's current state; it
//! does not run at app runtime).

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::beastiary1;

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
    // HOME-relative default: the operator keeps `workspace/` in the home
    // directory and syncs it between machines, so this resolves correctly on
    // any box. `PCGEN_CORPUS_ROOT` still wins when set.
    let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            let home = std::env::var("HOME")
                .expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let out_dir = PathBuf::from(manifest_dir).join("data/corpus/beastiary");

    let ingested_at = real_now_iso8601();
    // Real commit date of the SD-25 `corpus-intake-bestiary1` cycle (the
    // actual web second-source research for `Rag Armor (Dark Creeper)`
    // this cache dumps) -- see `beastiary1.rs`'s doc comment for why this
    // proxy is used instead of a per-fetch timestamp, mirrors
    // `gen_cache_apg.rs`'s own `fetched_at_web` precedent.
    let fetched_at_web = "2026-07-22T09:14:38-04:00";

    match beastiary1::generate(&corpus_root, &out_dir, &ingested_at, fetched_at_web) {
        Ok(report) => {
            println!(
                "Bestiary 1 cache generated: {} monsters, {} equipment records; ingested_at={ingested_at}",
                report.monsters_written, report.equipment_written
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
            eprintln!("Bestiary 1 cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
