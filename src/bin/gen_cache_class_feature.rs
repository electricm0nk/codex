//! One-off generation entry point for the `class_feature` JSON cache
//! (SD-31 `epic-5-chassis-sweep` F1, `SD31-E5-F1-001`). Run via
//! `cargo run --locked --bin gen_cache_class_feature` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `$HOME/workspace/repos/pcgen/data`) and
//! `CLASS_FEATURE_INVENTORY_JSON` pointing at a `docs/work-inventory.json`-
//! shaped document to source `(book, source_file, source_line, key, name)`
//! citations from (defaults to `docs/work-inventory.json`).
//!
//! Writes `data/corpus/<book>/class_feature/<class>/<feature>.json` for
//! every one of [`class_feature::BOOK_PRIMARY_FILES`]' 22 books.
//!
//! **`--coordinates <file>` mode** (SD-32 card 11, T9-onboarding-class-
//! feature-pi-and-rescreen): re-screen ONLY a named subset of already-
//! shipped records, instead of the full corpus-wide regen above. `file` is
//! a newline-separated list of `<book>:<source_file>:<source_line>`
//! coordinates (the same triple `shape_ledger.py` already joins on). Reads
//! the SAME pinned oracle and inventory as the unconditional path above,
//! but calls [`class_feature::generate`] with `units` FILTERED down to
//! exactly those coordinates -- so a re-screen after a blacklist amendment
//! (`decisions.md §19`) touches only the named records, never the other
//! ~18,000 already-shipped `class_feature` rows this generator's
//! unconditional `std::fs::write` (no no-clobber check, unlike
//! `feat_gap.rs`/`equipment_gap.rs`) would otherwise rewrite wholesale on
//! every run. This is this generator's own version of the `--remediate`
//! shape `scripts/ingest_generic_kind.py` already established for the
//! Python-side generic-kind writers: re-derive from the pinned oracle,
//! re-apply the CURRENT scrub pipeline, touch only what was named.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::cache_gen::class_feature;

/// Parses a `--coordinates <file>` list into the `(book, source_file,
/// source_line)` set `units_from_inventory_json`'s own units are filtered
/// against. Blank lines and `#`-prefixed comment lines are skipped.
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
            let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let inventory_path = std::env::var("CLASS_FEATURE_INVENTORY_JSON")
        .unwrap_or_else(|_| format!("{manifest_dir}/docs/work-inventory.json"));
    let inventory_json = std::fs::read_to_string(&inventory_path)
        .unwrap_or_else(|e| panic!("could not read {inventory_path}: {e}"));
    let mut units = class_feature::units_from_inventory_json(&inventory_json)
        .unwrap_or_else(|e| panic!("could not parse {inventory_path}: {e}"));
    let corpus_class_names = class_feature::corpus_class_names_from_inventory_json(&inventory_json)
        .unwrap_or_else(|e| panic!("could not parse {inventory_path}: {e}"));

    let coordinates_arg = std::env::args().position(|a| a == "--coordinates").and_then(|i| std::env::args().nth(i + 1));
    if let Some(coords_path) = coordinates_arg {
        let wanted = read_coordinates(&coords_path);
        let before = units.len();
        units.retain(|u| wanted.contains(&(u.book.clone(), u.source_file.clone(), u.source_line)));
        eprintln!(
            "--coordinates {coords_path}: {} of {} named coordinates matched a real inventory unit; \
             generating ONLY those {} record(s), not the full {before}-unit corpus",
            units.len(),
            wanted.len(),
            units.len()
        );
        if units.len() != wanted.len() {
            eprintln!(
                "WARNING: {} named coordinate(s) did not match any inventory unit (typo, or the unit \
                 was renamed/removed since the coordinate list was written)",
                wanted.len() - units.len()
            );
        }
    }

    let out_dir = PathBuf::from(manifest_dir).join("data/corpus");
    let grants_root = PathBuf::from(manifest_dir).join("data/class_feature_grants");
    let ingested_at = real_now_iso8601();

    match class_feature::generate(&corpus_root, &grants_root, &out_dir, &ingested_at, &units, &corpus_class_names) {
        Ok(report) => {
            println!(
                "class_feature cache generated: {} records across {} books ({} renamed under a Codex-generated \
                 neutral name, decisions.md §24); ingested_at={ingested_at}",
                report.written,
                report.books_written.len(),
                report.name_pi_skipped,
            );
            // `decisions.md §24b`-4: divergence entries carry coordinates +
            // reason only, never the original PI string.
            if let Ok(report_path) = std::env::var("CLASS_FEATURE_RENAME_REPORT") {
                let doc = serde_json::json!({
                    "population_name_pi_renamed": report.name_pi_skipped,
                    "renamed_records": report.name_pi_renamed_records,
                });
                let text = serde_json::to_string_pretty(&doc).expect("plain-data JSON cannot fail to serialize");
                std::fs::write(&report_path, text)
                    .unwrap_or_else(|e| panic!("could not write CLASS_FEATURE_RENAME_REPORT={report_path}: {e}"));
            }
            if !report.unresolved_citations.is_empty() {
                eprintln!(
                    "WARNING: {} citation(s) did not resolve: {:?}",
                    report.unresolved_citations.len(),
                    report.unresolved_citations
                );
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("class_feature cache generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
