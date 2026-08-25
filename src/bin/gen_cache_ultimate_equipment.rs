//! One-off generation entry point for the Ultimate Equipment (UE) JSON
//! cache (SD-31 `epic-6-ingest-lanes` F5/F6, `SD31-E6-F5-001`). Run via
//! `cargo run --locked --bin gen_cache_ultimate_equipment` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `$HOME/workspace/repos/pcgen/data`).
//! Writes `data/corpus/ultimate_equipment/equipment/*.json`.
//!
//! This binary is the generator itself, not a standing production
//! surface -- it is re-run only when
//! `codex::rules_core::rules_tables::ultimate_equipment` changes (mirrors
//! `gen_cache_apg.rs`'s own framing).

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::ultimate_equipment;

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
    let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            let home = std::env::var("HOME")
                .expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let out_dir = PathBuf::from(manifest_dir).join("data/corpus/ultimate_equipment");

    let ingested_at = real_now_iso8601();

    match ultimate_equipment::generate(&corpus_root, &out_dir, &ingested_at) {
        Ok(report) => {
            println!(
                "Ultimate Equipment cache generated: {} equipment, {} equipment_modifier records; ingested_at={ingested_at}",
                report.equipment_written, report.equipment_modifier_written
            );
            println!(
                "  renamed under a Codex-generated neutral identity, NAMEISPI:YES (`decisions.md §24`): {}",
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
            eprintln!("Ultimate Equipment cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
