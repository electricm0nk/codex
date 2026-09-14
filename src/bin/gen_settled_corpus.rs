//! Authoring-time producer of `data/corpus/<book>/_settled/<kind>.json` — the
//! settled-record bundles the live corpus loaders read with serde.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 15, `decisions.md` §11 ("not one line of
//! PCGen in our live code") and §19 (operator ruling B16: naming `pcgen_import`
//! in shipping code under a live root is a hit).
//!
//! ## What this closes
//!
//! The `corpus_json_boundary` group — the last two `pcgen_import` names under
//! `src/rules_core/` that were **calls**, not payload imports. Cycle 13 moved
//! the equipment kind's ingest-format reading to the converter side and cycle 14
//! did the same for the two race kinds; each cycle booked the boundary *call*
//! that remained on the live side honestly, and each named the clearing
//! condition in the same words: *"it clears when `data/corpus/` carries the
//! settled fields itself."* This binary is that producer.
//!
//! Every value it writes comes from the exact call the live loader made at run
//! time before this cycle — `corpus_equipment_json::corpus_equipment_source_record`,
//! `corpus_race_json::corpus_race_source_record` and
//! `::corpus_race_trait_source_record`. Nothing is re-derived and no new reading
//! rule is spelled: the conversion simply happens once, at authoring time,
//! instead of on every load.
//!
//! ## What it writes
//!
//! ```text
//! data/corpus/<book>/_settled/equipment.json    <- keyed by path under equipment/
//! data/corpus/<book>/_settled/race.json         <- keyed by path under race/
//! data/corpus/<book>/_settled/race_trait.json   <- keyed by path under race_trait/
//! apps/desktop/src-tauri/resources/corpus_fixtures/_settled/equipment.json
//!   ^ built from apps/desktop/src-tauri/fixtures_src/equipment/ (ruling B17:
//!     the ingest-format records are a converter input and do not ship)
//! ```
//!
//! `_settled/` sits **beside** the kind directories, never inside one, so no
//! corpus walk can mistake a bundle for a record and no existing traversal
//! needed a new skip rule. **No corpus record file is read-modify-written**:
//! the ingested records, their license blocks and their `pi_*` redaction stamps
//! are not touched at all.
//!
//! ## Determinism
//!
//! Bundles are `BTreeMap`s and `serde_json`'s object type is a `BTreeMap` in
//! this build (no `preserve_order`), so output is key-sorted and byte-stable
//! across checkouts. `--check` regenerates every bundle in memory and compares
//! bytes, the same contract `sheet_rule_convert --check` and
//! `gen_desktop_fixture_corpus --check` carry.
//!
//! ## Usage
//!
//! ```text
//! cargo run --locked --bin gen_settled_corpus            # write
//! cargo run --locked --bin gen_settled_corpus -- --check # verify, exit 1 on drift
//! ```

use std::path::{Path, PathBuf};

use codex::pcgen_import::corpus_settled_bundle::{bundles_for_book, BundleArtifact};
use codex::rules_core::settled_corpus::{corpus_book_dirs, CORPUS_ROOT};

/// The desktop's bundled fixture corpus is a `BookCorpusRoot` like any other —
/// `corpus_loader::load_equipment_corpus` reads it with the same function — so
/// it gets its bundle from the same producer.
///
/// SD-35 `AT-35-E6-005-SHIPPED-DATA`, ruling B17: its INGEST-format records
/// live outside `bundle.resources`, at `fixtures_src/`, because a converter
/// input is kept (`decisions.md` §11) but is not a shipped file. So this
/// producer READS one root and WRITES the bundle into the other — the only
/// place in this binary where those two differ.
const DESKTOP_FIXTURE_SRC_ROOT: &str = "apps/desktop/src-tauri/fixtures_src";
const DESKTOP_FIXTURE_SHIP_ROOT: &str = "apps/desktop/src-tauri/resources/corpus_fixtures";

fn roots() -> Vec<PathBuf> {
    let mut roots = corpus_book_dirs(Path::new(CORPUS_ROOT));
    let fixtures = PathBuf::from(DESKTOP_FIXTURE_SRC_ROOT);
    if fixtures.is_dir() {
        roots.push(fixtures);
    }
    roots
}

/// Where a bundle built from `root` is written.
///
/// Identity for every real book. For the desktop fixture root it is the shipped
/// resource directory: the live loader reads `_settled/equipment.json` beside
/// the records it enumerates, and those records ship.
fn bundle_destination(artifact_path: &Path) -> PathBuf {
    match artifact_path.strip_prefix(DESKTOP_FIXTURE_SRC_ROOT) {
        Ok(rest) => Path::new(DESKTOP_FIXTURE_SHIP_ROOT).join(rest),
        Err(_) => artifact_path.to_path_buf(),
    }
}

fn main() {
    let check = std::env::args().any(|arg| arg == "--check");
    let roots = roots();
    if roots.is_empty() {
        eprintln!("gen_settled_corpus: no corpus roots under {CORPUS_ROOT}; run from the repo root");
        std::process::exit(2);
    }

    let mut bundles = 0usize;
    let mut records = 0usize;
    let mut skipped = 0usize;
    let mut drifted: Vec<PathBuf> = Vec::new();

    for root in &roots {
        for mut artifact in bundles_for_book(root) {
            artifact.path = bundle_destination(&artifact.path);
            bundles += 1;
            records += artifact.records;
            skipped += artifact.skipped.len();
            if check {
                if std::fs::read_to_string(&artifact.path).ok().as_deref()
                    != Some(artifact.text.as_str())
                {
                    eprintln!("DRIFT {}", artifact.path.display());
                    drifted.push(artifact.path.clone());
                }
            } else if let Err(err) = write(&artifact) {
                eprintln!("gen_settled_corpus: could not write {}: {err}", artifact.path.display());
                std::process::exit(2);
            }
        }
    }

    let verdict = if drifted.is_empty() { "PASS" } else { "DRIFT" };
    println!(
        "roots={} bundles={bundles} records={records} skipped={skipped} drifted={} verdict={verdict}",
        roots.len(),
        drifted.len()
    );
    if !drifted.is_empty() {
        std::process::exit(1);
    }
}

fn write(artifact: &BundleArtifact) -> std::io::Result<()> {
    if let Some(parent) = artifact.path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&artifact.path, &artifact.text)
}
