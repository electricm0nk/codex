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

use std::path::{Path, PathBuf};
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

/// The book's real, on-disk licensed-content record count -- every `.json`
/// file under `book_dir` except `LICENSE.json` itself and anything under a
/// `_`-prefixed directory (`_parity/` etc., test fixtures, not licensed
/// content). Identical logic to `gen_book_cache.rs`'s own
/// `count_on_disk_records`, duplicated here rather than shared across two
/// separate `src/bin/` binaries -- see this book's `LICENSE.json`'s
/// `records_processed` field, which this function now derives instead of
/// leaving hand-maintained (SD28-E16, `decisions.md §36` instance 8: this
/// book's `LICENSE.json` predates any generator owning it -- `cache_gen::
/// beastiary1` only ever wrote `monster/`+`equipment/`, while `race/` and
/// `race_trait/` are written by the separate `ingest_races.rs`, so no
/// single writer's own count was ever the whole book's count, exactly the
/// gap `gen_book_cache.rs`'s own doc comment already named for ARG/PU
/// before this fix closed it for Bestiary 1 too).
fn count_on_disk_records(book_dir: &Path) -> usize {
    fn walk(dir: &Path, count: &mut usize) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let is_internal = path.file_name().and_then(|f| f.to_str()).is_some_and(|n| n.starts_with('_'));
                if !is_internal {
                    walk(&path, count);
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|f| f.to_str()) != Some("LICENSE.json")
            {
                *count += 1;
            }
        }
    }
    let mut count = 0;
    walk(book_dir, &mut count);
    count
}

/// Rewrites `LICENSE.json`'s `records_processed` field from the real
/// on-disk count, operating on raw `serde_json::Value` (never a typed
/// struct -- `enrich_equipment_raw_tokens.rs`'s own doc comment names why:
/// deserializing into a struct that doesn't know every field silently
/// drops the fields it doesn't know about) so every other field --
/// `license_declaration`, `redaction_policy`, `screening_method_note`,
/// everything -- survives untouched.
fn resync_license_record_count(out_dir: &Path, book_dir: &Path) {
    let license_path = out_dir.join("LICENSE.json");
    let text = std::fs::read_to_string(&license_path)
        .unwrap_or_else(|e| panic!("failed to read {license_path:?}: {e}"));
    let mut value: serde_json::Value =
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("{license_path:?} is not valid JSON: {e}"));
    let real_count = count_on_disk_records(book_dir);
    let stated = value["records_processed"].as_u64().map(|n| n as usize);
    if stated == Some(real_count) {
        return;
    }
    println!(
        "LICENSE.json records_processed: stated {stated:?}, real on-disk count {real_count} -- resyncing"
    );
    value["records_processed"] = serde_json::json!(real_count);
    let json = serde_json::to_string_pretty(&value).expect("LICENSE.json value must serialize");
    std::fs::write(&license_path, json + "\n").unwrap_or_else(|e| panic!("failed to write {license_path:?}: {e}"));
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
            resync_license_record_count(&out_dir, &out_dir);
        }
        Err(e) => {
            eprintln!("Bestiary 1 cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
