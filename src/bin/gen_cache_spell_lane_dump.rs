//! One-off generation entry point for the spell-lane supplemental JSON
//! cache (SD-31 `epic-6-ingest-lanes` F2, `SD31-E6-F2-005`). Run via
//! `cargo run --locked --bin gen_cache_spell_lane_dump` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `$HOME/workspace/repos/pcgen/data`).
//! Writes `data/corpus/{occult_adventures,ultimate_magic,ultimate_combat,
//! ultimate_intrigue}/spell/*.json`.
//!
//! This binary is the generator itself, not a standing production
//! surface -- it is re-run only when one of the four books' compiled
//! `spell_list::SPELL_LIST` table changes (mirrors
//! `gen_cache_ultimate_equipment.rs`'s own framing).

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::spell_lane_dump;

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
            let home =
                std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let data_corpus_root = PathBuf::from(manifest_dir).join("data/corpus");

    let ingested_at = real_now_iso8601();

    match spell_lane_dump::generate(&corpus_root, &data_corpus_root, &ingested_at) {
        Ok(report) => {
            println!(
                "spell-lane supplemental cache generated: {} spell records; ingested_at={ingested_at}",
                report.spells_written
            );
            println!(
                "  dropped, NAMEISPI:YES or blacklisted (name is Product Identity, row cannot be published): {}",
                report.name_pi_dropped.len()
            );
            for line in &report.name_pi_dropped {
                println!("    {line}");
            }
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
            eprintln!("spell-lane supplemental cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
