//! Narrows a shipped equipment record's `web_second_source` citation to the
//! real, pinned PCGen corpus row its identity and magnitudes actually come
//! from (SD-31 `epic-6-ingest-lanes` F5).
//!
//! Run with `PCGEN_CORPUS_ROOT` pointing at the pinned PCGen `data/`
//! checkout (`scripts/pcgen-oracle-pin.env`, bootstrapped by
//! `scripts/fetch-pcgen-oracle.sh`; defaults to
//! `$HOME/workspace/repos/pcgen/data`). The full rationale, and the bar a
//! record has to clear before its citation is narrowed, is in
//! `rules_core::cache_gen::lst_provenance_repair`'s module doc comment.
//!
//! `--check` reports what would change and exits non-zero if anything would,
//! without writing; the default run writes.
//!
//! After a real run, `enrich_equipment_raw_tokens` populates the newly
//! citable records' `raw_tokens`, and `corpus_literal_sweep` then byte-
//! compares them — this binary deliberately does neither, so that the
//! verification is done by the gate's own tools rather than by the tool that
//! made the claim.

use std::collections::BTreeMap;
use std::path::PathBuf;

use codex::rules_core::cache_gen::lst_provenance_repair::{repair_book, RepairReport};

/// The books whose equipment records are known to carry `web_second_source`
/// citations for corpus-derived values, each with the corpus directory its
/// rows physically live in. Derived, not assumed — the census command is in
/// this cycle's receipt:
///
/// ```text
/// python3 -c 'import json,glob,os,collections; ...'   # see progress.md
/// #   advanced_players_guide  331
/// #   core_rulebook            82
/// #   beastiary                 1
/// ```
const BOOKS: &[(&str, &str)] = &[
    (
        "advanced_players_guide",
        "pathfinder/paizo/roleplaying_game/advanced_players_guide",
    ),
    ("core_rulebook", "pathfinder/paizo/roleplaying_game/core_rulebook"),
    // `beastiary` (the misspelled shipped book id) is `bonus_bestiary`'s
    // sibling dump; its rows live in the shared core_essentials library, the
    // same file-vs-attribution split `equipment_gap::book_routing` documents
    // for `B1`.
    ("beastiary", "pathfinder/paizo/roleplaying_game/core_essentials"),
];

fn main() {
    let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            let home = std::env::var("HOME")
                .expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };
    if !corpus_root.is_dir() {
        eprintln!("FATAL: PCGEN_CORPUS_ROOT {corpus_root:?} is not a directory -- run scripts/fetch-pcgen-oracle.sh");
        std::process::exit(2);
    }
    let check_only = std::env::args().any(|a| a == "--check");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let corpus_out = PathBuf::from(manifest_dir).join("data/corpus");

    let mut totals: BTreeMap<&str, RepairReport> = BTreeMap::new();
    for (book_id, book_rel_dir) in BOOKS {
        let records_dir = corpus_out.join(book_id).join("equipment");
        if !records_dir.is_dir() {
            eprintln!("NOTE: {book_id}: no equipment records directory at {records_dir:?}; skipped");
            continue;
        }
        let report = repair_book(&corpus_root, book_rel_dir, &records_dir, !check_only)
            .unwrap_or_else(|e| panic!("{book_id}: repair failed: {e}"));
        totals.insert(book_id, report);
    }

    let mut upgraded = 0usize;
    let mut refused = 0usize;
    for (book_id, r) in &totals {
        println!(
            "{book_id}: {} record(s) narrowed to an lst_token citation, {} already cited, {} refused, of {} read",
            r.upgraded.len(),
            r.already_cited,
            r.refused.len(),
            r.records_seen
        );
        for refusal in &r.refused {
            println!("  REFUSED  {}", refusal.describe());
        }
        upgraded += r.upgraded.len();
        refused += r.refused.len();
    }
    println!("repair-lst-provenance: {upgraded} narrowed, {refused} refused");

    if check_only && upgraded > 0 {
        eprintln!("--check: {upgraded} record(s) would change; run without --check to write them");
        std::process::exit(1);
    }
}
