//! Repairs shipped `equipment`/`equipment_modifier` records' stale
//! `.COPY=` citations (SD-32 T9 residual, `decisions.md §20`). Full
//! rationale and the safety bar each record must clear are in
//! `rules_core::cache_gen::equipment_copy_citation_repair`'s module doc
//! comment.
//!
//! Run with `PCGEN_CORPUS_ROOT` pointing at the pinned PCGen `data/`
//! checkout. `--check` reports what would change and exits non-zero if
//! anything would, without writing; the default run writes.
//!
//! After a real run, `enrich_equipment_raw_tokens` repopulates the moved
//! records' `raw_tokens`/`raw_bonus_chains` from the corrected line -- this
//! binary deliberately removes rather than recomputes them, so that field
//! is always produced by its one established mechanism.

use std::path::PathBuf;

use codex::rules_core::cache_gen::equipment_copy_citation_repair::repair_book;

/// Every book this binary considers -- `(book_id, book_rel_dir)`, the same
/// pair `equipment_gap::book_routing` returns for each code. Scoped to the
/// books this cycle's `no_record` re-derivation actually traced a
/// coincidental-`.COPY=`-citation shape in
/// (`scripts/shape_ledger.py --inventory docs/work-inventory.json`,
/// `equipment_modifier` `no_record` rows); `repair_book`'s own safety gate
/// (independent old-line coverage, no new-line collision) means running it
/// against every book with equipment records would still refuse everywhere
/// else rather than mis-fire, but scoping the list keeps this run's report
/// legible and matches what was actually re-derived.
const BOOKS: &[(&str, &str)] = &[
    ("advanced_class_guide", "pathfinder/paizo/roleplaying_game/advanced_class_guide"),
    ("core_rulebook", "pathfinder/paizo/roleplaying_game/core_rulebook"),
    ("mythic_adventures", "pathfinder/paizo/roleplaying_game/mythic_adventures"),
];

fn main() {
    let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            eprintln!("FATAL: PCGEN_CORPUS_ROOT must be set");
            std::process::exit(2);
        }
    };
    if !corpus_root.is_dir() {
        eprintln!("FATAL: PCGEN_CORPUS_ROOT {corpus_root:?} is not a directory -- run scripts/fetch-pcgen-oracle.sh");
        std::process::exit(2);
    }
    let check_only = std::env::args().any(|a| a == "--check");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let corpus_out = PathBuf::from(manifest_dir).join("data/corpus");

    let mut total_repaired = 0usize;
    let mut total_refused = 0usize;
    for (book_id, book_rel_dir) in BOOKS {
        let records_dir = corpus_out.join(book_id).join("equipment");
        if !records_dir.is_dir() {
            eprintln!("NOTE: {book_id}: no equipment records directory at {records_dir:?}; skipped");
            continue;
        }
        let report = repair_book(&corpus_root, book_rel_dir, &records_dir, !check_only)
            .unwrap_or_else(|e| panic!("{book_id}: repair failed: {e}"));
        println!(
            "{book_id}: {} record(s) repaired, {} refused, of {} read",
            report.repaired.len(),
            report.refused.len(),
            report.records_seen
        );
        for r in &report.repaired {
            println!("  REPAIRED  {}: line {} -> {}", r.file.display(), r.old_line, r.new_line);
        }
        total_repaired += report.repaired.len();
        total_refused += report.refused.len();
    }
    println!("repair-equipment-copy-citations: {total_repaired} repaired, {total_refused} refused");

    if check_only && total_repaired > 0 {
        eprintln!("--check: {total_repaired} record(s) would change; run without --check to write them");
        std::process::exit(1);
    }
}
