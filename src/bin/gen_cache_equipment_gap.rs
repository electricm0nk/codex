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
//!
//! **`--coordinates <file>` mode** (`t9-onboarding-pi-last-leak-and-
//! generators`, `decisions.md §17`): re-screen ONLY a named subset of
//! already-shipped records, instead of the full corpus-wide walk above.
//! `file` is a newline-separated list of `<book>:<source_file>:
//! <source_line>` coordinates -- the SAME shape `gen_cache_class_
//! feature.rs`'s own `--coordinates` mode already established; reused
//! verbatim rather than inventing a third scoped-regen shape.
//! `equipment_gap::generate`'s own `write_json` stays no-clobber
//! (unchanged) -- to actually replace an already-shipped leaking record,
//! remove that one file first (a guarded, coordinate-named `rm`), then
//! run this mode so ONLY the freed slot gets rewritten.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::equipment_gap;

/// Parses a `--coordinates <file>` list into the `(book_id, source_file,
/// source_line)` set `equipment_gap::generate`'s own `coordinates` filter
/// is checked against. Blank lines and `#`-prefixed comment lines are
/// skipped -- byte-for-byte the same parser
/// `gen_cache_class_feature.rs::read_coordinates` already established
/// (own local copy per this repo's no-shared-binary-helpers convention;
/// behaviour must stay identical, so keep any future fix mirrored there).
fn read_coordinates(path: &str) -> BTreeSet<(String, String, u32)> {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("could not read --coordinates file {path}: {e}"));
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            let mut parts = l.splitn(3, ':');
            let book = parts.next().unwrap_or_default().to_string();
            let source_file = parts.next().unwrap_or_default().to_string();
            let source_line: u32 = parts
                .next()
                .unwrap_or_default()
                .parse()
                .unwrap_or_else(|e| panic!("--coordinates line {l:?}: bad source_line: {e}"));
            (book, source_file, source_line)
        })
        .collect()
}

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

    let coordinates_arg =
        std::env::args().position(|a| a == "--coordinates").and_then(|i| std::env::args().nth(i + 1));
    let wanted = coordinates_arg.as_deref().map(read_coordinates);
    if let Some(w) = &wanted {
        eprintln!(
            "--coordinates {}: generating ONLY {} named coordinate(s), not the full corpus-wide walk",
            coordinates_arg.as_deref().unwrap_or_default(),
            w.len()
        );
    }

    match equipment_gap::generate(&corpus_root, &out_root, &ingested_at, wanted.as_ref()) {
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
            if !report.disambiguated_collision.is_empty() {
                eprintln!(
                    "NOTE: {} record(s) written under a disambiguated slug -- a DIFFERENT real citation line than the file already occupying that slug: {:?}",
                    report.disambiguated_collision.len(),
                    report.disambiguated_collision
                );
            }
            if !report.excluded_non_content_directive.is_empty() {
                eprintln!(
                    "NOTE: {} record(s) excluded as non-content (.FORGET directive or a disabled #-prefixed row): {:?}",
                    report.excluded_non_content_directive.len(),
                    report.excluded_non_content_directive
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
