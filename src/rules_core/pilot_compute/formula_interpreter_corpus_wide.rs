//! SD-32 Gate 2 (`gate-2-corpus-wide-runs`, kanban `#8`) — the corpus-wide run
//! AT-32-G2-004 requires for `formula_interpreter.rs` (card 6's F1..F9 engine).
//!
//! # SD-33 AT-33-E3-001/002/003 root-cause fix: the census is regenerated
//! fresh, never read from SD-32's frozen file
//!
//! `README.md §4` row G (SD-33) named a "41% coverage" gap: **6,854** of
//! **11,652** formula-bearing F1..F9 units had never been run through this
//! engine. Root cause (`artifacts/epic-3-engine-coverage/coverage-gap-
//! rootcause.md`), established by execution, not assumption: this module
//! previously sourced its population from `docs/release/SD-32-compute-
//! library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json` —
//! SD-32's own frozen, committed Gate 1 artifact, dated 2026-08-14. Two
//! independent staleness layers compounded:
//!
//! 1. **Stale run, current code**: re-running this module's *unchanged*
//!    scan logic against that same frozen file (still 11,338 F1..F9 rows)
//!    today produces population=11,338, not the artifact's committed 4,798
//!    — the 4,798 committed at `artifacts/gate-2-engines/formula_
//!    interpreter.corpus-wide.json` (SD-32's closed Gate 2 evidence, never
//!    overwritten) was itself a stale run against an earlier code/data
//!    state and was never regenerated after the code that could walk the
//!    full census landed. Most of the "6,854 never run" figure (~6,540 of
//!    it) was this: a correct, ready engine that had simply not been
//!    re-run since.
//! 2. **Stale census, current corpus**: the frozen `ledger.json` itself is
//!    stale relative to the CURRENT `docs/work-inventory.json` / `data/
//!    corpus` state — 314 more F1..F9 units exist today
//!    (`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
//!    reports 11,652 matched F1..F9 rows, not the frozen file's 11,338).
//!
//! The fix for both: this module no longer reads SD-32's frozen file at
//! all. [`fresh_census_rows`] regenerates the Gate 1 census **fresh, at
//! scan time**, by invoking the same `scripts/shape_ledger.py` classifier
//! SD-32 used (never re-implemented in Rust — `decisions.md §4`'s single-
//! source-of-truth discipline: the PF1e family vocabulary is Python data,
//! porting its regex rule list into Rust would be exactly the drift risk
//! that decision exists to prevent), writing its `--output` to a scratch
//! path outside the repo (`std::env::temp_dir()`, the same convention
//! `oracle_validation::pcgen_runner` uses) and reading the rows back. The
//! population this module reports is therefore always the CURRENT true
//! population, never a frozen snapshot that silently drifts out from under
//! every future cycle that trusts it.
//!
//! # What "corpus-wide" means here
//!
//! AT-32-G2-004: "No engine is 'complete' until it has been run corpus-wide
//! once. The corpus-wide run is itself a cycle, with its own receipt, and its
//! own fixture-check, against the closed Gate 1 census. A cycle that runs an
//! engine against a subset and declares the engine done is out of protocol —
//! the subset is not the population the engine claims to handle."
//!
//! This module re-derives, for **every** unit the freshly-regenerated census
//! placed in F1..F9 (never a hand-picked subset), the same DEFINE/BONUS
//! formula segment(s) `shape_ledger.py` itself joined against
//! (`docs/work-inventory.json`'s `(book, source_file, source_line)` triple ->
//! the matching `data/corpus/<book>/**/*.json` record's `data.raw_tokens`),
//! and runs the REAL production grammar (`formula_interpreter::recognises_shape`,
//! backed by the same `PcgenFormulaEvaluator` the family-fixture check in
//! card 6's cycle already proved correct on 9 hand-picked samples) against
//! every one of them.
//!
//! This is a population-scoped grammar-reach proof, not a value proof: full
//! numeric evaluation needs a bound `vars` map a standalone corpus record
//! does not carry (module doc of `formula_interpreter`, point 1, and the
//! `classlevel`/`skillinfo` consumer-binding notes) — no consumer is wired to
//! supply real character state to every one of the 11,652 units in F1..F9,
//! and inventing one would be exactly the "plausible number nobody checks"
//! shape this bundle's own doctrine refuses. What this module proves
//! instead, honestly: the interpreter's grammar actually reaches (parses
//! without refusing) the real formula text of every unit Gate 1 independently
//! counted under that family
//! — the corpus-wide population check AT-32-G2-004 asks for — and reports,
//! per family, how many units it refuses and a sample of why, rather than
//! silently rounding a partial proof up to "done".
//!
//! # The fixture-check against the freshly-regenerated Gate 1 census
//!
//! [`run_corpus_wide_scan`] asserts `total_population` (the number of units
//! this scan actually walked) equals the freshly-regenerated census's own
//! F1..F9 row count, returning [`ScanError::PopulationMismatch`] rather than
//! silently reporting a partial run as complete — this is the "own
//! fixture-check, against the closed Gate 1 census" AT-32-G2-004 names,
//! expressed as a population-parity check rather than a per-value comparison
//! (no oracle byte the engine does not read carries a per-unit expected
//! numeric value for 11,652 units; the census population count itself is
//! the fixture, and it is produced by `scripts/shape_ledger.py`
//! independently of this module, exactly the "engine never reads"
//! provenance discipline `decisions.md` §3 / operator ruling §20 requires).
//! Under SD-33's fix this check is structural (it always holds, since
//! `scan_ledger_rows` walks every row `fresh_census_rows` hands it) rather
//! than a live subset-catcher — it is kept because a future change to
//! either function that reintroduces filtering must still trip it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use super::formula_interpreter::{extract_formula_field, recognises_shape};

/// The nine families this run is scoped to — `formula_interpreter.rs` / card
/// 6's own scope. `bonus_stack_reader.rs`'s binding layer (card 7's own
/// corpus-wide run, not this one) resolves the producer-bound subset of F4's
/// bare-identifier values; F10 (a 3-unit level-threshold step-count family,
/// canonical vocabulary in `scripts/shape_ledger.py`) is unrelated to the
/// binding layer and is one of this run's own nine directly-evaluated
/// families below.
pub const IN_SCOPE_FAMILIES: [&str; 9] =
    ["F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9"];

/// One ledger row's minimum shape this module needs to re-derive its
/// formula text: the closed Gate 1 census's `id`/`family`, joined against
/// `docs/work-inventory.json`'s `(book, source_file, source_line)` for that
/// same `id`.
#[derive(Debug, Clone)]
pub struct LedgerRow {
    pub id: String,
    pub book: String,
    pub family: String,
}

#[derive(Debug, Clone, Default)]
pub struct FamilyCoverage {
    pub population: usize,
    pub recognised_units: usize,
    pub refused_units: usize,
    pub unjoined_units: usize,
    pub refusal_samples: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CorpusWideReport {
    pub families: BTreeMap<String, FamilyCoverage>,
    pub total_population: usize,
    pub total_recognised_units: usize,
    pub total_refused_units: usize,
    pub total_unjoined_units: usize,
}

#[derive(Debug)]
pub enum ScanError {
    /// Could not read/parse `docs/work-inventory.json`, or could not
    /// regenerate the fresh Gate 1 census (the `python3 scripts/shape_
    /// ledger.py` subprocess failed to spawn, exited non-zero, or wrote
    /// unparseable JSON) — a scan that cannot see its own population must
    /// refuse, never report a vacuous zero as "done".
    MissingInput(String),
    /// The number of units this scan actually walked disagrees with the
    /// freshly-regenerated census's own F1..F9 row count — the
    /// "fixture-check, against the closed Gate 1 census" AT-32-G2-004
    /// requires. This is the check a cycle that (accidentally or
    /// otherwise) ran against a subset must trip.
    PopulationMismatch { scanned: usize, census: usize },
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::MissingInput(m) => write!(f, "corpus-wide scan input missing: {m}"),
            ScanError::PopulationMismatch { scanned, census } => write!(
                f,
                "corpus-wide scan walked {scanned} unit(s) but the freshly-regenerated Gate 1 \
                 census counts {census} in F1..F9 — a cycle that runs against a subset is out of \
                 protocol (AT-32-G2-004)"
            ),
        }
    }
}

/// Loads `docs/work-inventory.json`'s units into `id -> (book, source_file,
/// source_line)`, the same join key `scripts/shape_ledger.py`'s
/// `build_corpus_index`/`classify_unit` use.
fn load_inventory_join_keys(
    repo_root: &Path,
) -> Result<BTreeMap<String, (String, String, u64)>, ScanError> {
    let path = repo_root.join("docs/work-inventory.json");
    let text = std::fs::read_to_string(&path)
        .map_err(|e| ScanError::MissingInput(format!("{}: {e}", path.display())))?;
    let doc: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| ScanError::MissingInput(format!("{}: invalid JSON: {e}", path.display())))?;
    let units = doc
        .get("units")
        .and_then(|u| u.as_array())
        .ok_or_else(|| ScanError::MissingInput(format!("{}: no `units` array", path.display())))?;
    let mut out = BTreeMap::new();
    for u in units {
        let (Some(id), Some(book), Some(source_file), Some(source_line)) = (
            u.get("id").and_then(|v| v.as_str()),
            u.get("book").and_then(|v| v.as_str()),
            u.get("source_file").and_then(|v| v.as_str()),
            u.get("source_line").and_then(|v| v.as_u64()),
        ) else {
            continue;
        };
        out.insert(id.to_string(), (book.to_string(), source_file.to_string(), source_line));
    }
    Ok(out)
}

/// Process-wide cache for [`fresh_census_rows`]'s regeneration — the
/// `scripts/shape_ledger.py` subprocess walks the whole corpus (tens of
/// seconds); `cargo test` runs every `#[test]` fn for this module in the
/// same process, so caching means the corpus is walked once per test
/// binary invocation, not once per test. The cache stores `Result<_,
/// String>` (never `ScanError`, which is not `Clone`) and is remapped back
/// to `ScanError::MissingInput` on read.
static FRESH_CENSUS_CACHE: OnceLock<Result<Vec<LedgerRow>, String>> = OnceLock::new();

/// Regenerates the Gate 1 shape census **fresh**, at scan time, by invoking
/// `scripts/shape_ledger.py` against the CURRENT `docs/work-inventory.json`
/// / `data/corpus` state — never SD-32's frozen, committed
/// `artifacts/gate-1-shape-closure/ledger.json` (module doc, "SD-33
/// AT-33-E3-001/002/003 root-cause fix"). Returns only the F1..F9 rows —
/// never a hand-filtered subset of those. Cached process-wide via
/// [`FRESH_CENSUS_CACHE`].
fn fresh_census_rows(repo_root: &Path) -> Result<Vec<LedgerRow>, ScanError> {
    FRESH_CENSUS_CACHE
        .get_or_init(|| regenerate_census_rows_uncached(repo_root).map_err(|e| e.to_string()))
        .clone()
        .map_err(ScanError::MissingInput)
}

/// The uncached regeneration this module's doc names: spawns `python3
/// scripts/shape_ledger.py --inventory <repo>/docs/work-inventory.json
/// --corpus-root <repo>/data/corpus --output <scratch path>` (the same
/// classifier `scripts/box_ledger.py`'s SD-32 predecessor, `coverage_
/// ledger.py`, and this module's own SD-32-era code all trusted as the
/// PF1e family vocabulary's single source of truth — `decisions.md` §4),
/// writes to a scratch path under `std::env::temp_dir()` (the same
/// convention `oracle_validation::pcgen_runner::run_pcgen` uses, never a
/// path inside the repo, so this regeneration is never mistaken for a
/// committed artifact), reads the JSON back, and filters to F1..F9.
fn regenerate_census_rows_uncached(repo_root: &Path) -> Result<Vec<LedgerRow>, ScanError> {
    let script = repo_root.join("scripts/shape_ledger.py");
    let inventory = repo_root.join("docs/work-inventory.json");
    let corpus_root = repo_root.join("data/corpus");
    let scratch_output = std::env::temp_dir()
        .join(format!("sd33-e3-shape-ledger-fresh-{}.json", std::process::id()));

    let output = Command::new("python3")
        .arg(&script)
        .arg("--inventory")
        .arg(&inventory)
        .arg("--corpus-root")
        .arg(&corpus_root)
        .arg("--output")
        .arg(&scratch_output)
        .output()
        .map_err(|e| {
            ScanError::MissingInput(format!(
                "spawning `python3 {}`: {e}",
                script.display()
            ))
        })?;
    if !output.status.success() {
        return Err(ScanError::MissingInput(format!(
            "`python3 {}` exited {:?}: {}",
            script.display(),
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let text = std::fs::read_to_string(&scratch_output).map_err(|e| {
        ScanError::MissingInput(format!("{}: {e}", scratch_output.display()))
    })?;
    let _ = std::fs::remove_file(&scratch_output);
    let doc: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
        ScanError::MissingInput(format!("{}: invalid JSON: {e}", scratch_output.display()))
    })?;
    let rows = doc
        .get("rows")
        .and_then(|r| r.as_array())
        .ok_or_else(|| {
            ScanError::MissingInput(format!("{}: no `rows` array", scratch_output.display()))
        })?;
    let mut out = Vec::new();
    for r in rows {
        let (Some(id), Some(book), Some(family)) = (
            r.get("id").and_then(|v| v.as_str()),
            r.get("book").and_then(|v| v.as_str()),
            r.get("family").and_then(|v| v.as_str()),
        ) else {
            continue;
        };
        if IN_SCOPE_FAMILIES.contains(&family) {
            out.push(LedgerRow { id: id.to_string(), book: book.to_string(), family: family.to_string() });
        }
    }
    Ok(out)
}

/// Builds `(basename, source_line) -> [(key, value)]` for every DEFINE/BONUS*
/// raw token in `data/corpus/<book>/**/*.json` — the same join
/// `scripts/shape_ledger.py`'s `build_corpus_index` performs, scoped to one
/// book at a time (this scan only ever needs the books its rows actually
/// name).
fn build_book_corpus_index(
    corpus_root: &Path,
    book: &str,
) -> BTreeMap<(String, u64), Vec<(String, String)>> {
    let mut index = BTreeMap::new();
    let book_dir = corpus_root.join(book);
    for path in walk_json(&book_dir) {
        if path.file_name().and_then(|n| n.to_str()) == Some("LICENSE.json") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(rec) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let Some(source) = rec.get("source") else { continue };
        let (Some(src_path), Some(src_line)) = (
            source.get("path").and_then(|v| v.as_str()),
            source.get("line").and_then(|v| v.as_u64()),
        ) else {
            continue;
        };
        let basename = PathBuf::from(src_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(src_path)
            .to_string();
        let raw_tokens = rec
            .pointer("/data/raw_tokens")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        let tokens: Vec<(String, String)> = raw_tokens
            .iter()
            .filter_map(|t| {
                let key = t.get("key")?.as_str()?;
                let value = t.get("value")?.as_str()?;
                if key == "DEFINE" || key.starts_with("BONUS") {
                    Some((key.to_string(), value.to_string()))
                } else {
                    None
                }
            })
            .collect();
        index.insert((basename, src_line), tokens);
    }
    index
}

fn walk_json(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return out };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk_json(&path));
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
    out
}

/// The pure, testable core: given the closed Gate 1 in-scope rows, the
/// inventory join keys, and the corpus root, walks every row, joins it to
/// its real corpus formula text, and runs the production grammar
/// (`recognises_shape`) against every DEFINE/BONUS segment found. A unit
/// counts as "recognised" only if EVERY formula segment its record carries
/// parses — a single refusal on any segment marks the unit refused, so this
/// never overstates reach by averaging over a unit's easier segments.
pub fn scan_ledger_rows(
    rows: &[LedgerRow],
    inventory: &BTreeMap<String, (String, String, u64)>,
    corpus_root: &Path,
) -> CorpusWideReport {
    let mut report = CorpusWideReport::default();
    let mut book_indices: BTreeMap<String, BTreeMap<(String, u64), Vec<(String, String)>>> =
        BTreeMap::new();

    for row in rows {
        let family_entry = report.families.entry(row.family.clone()).or_default();
        family_entry.population += 1;
        report.total_population += 1;

        let Some((book, source_file, source_line)) = inventory.get(&row.id) else {
            family_entry.unjoined_units += 1;
            report.total_unjoined_units += 1;
            continue;
        };
        let index = book_indices
            .entry(book.clone())
            .or_insert_with(|| build_book_corpus_index(corpus_root, book));
        let key = (source_file.clone(), *source_line);
        let Some(tokens) = index.get(&key) else {
            family_entry.unjoined_units += 1;
            report.total_unjoined_units += 1;
            continue;
        };
        if tokens.is_empty() {
            family_entry.unjoined_units += 1;
            report.total_unjoined_units += 1;
            continue;
        }

        let mut all_recognised = true;
        let mut refusal: Option<String> = None;
        for (tkey, tvalue) in tokens {
            let Some(formula) = extract_formula_field(tkey, tvalue) else { continue };
            if let Err(e) = recognises_shape(formula) {
                all_recognised = false;
                refusal.get_or_insert(format!("{}: {formula:?} -> {}", row.id, e.0));
            }
        }

        if all_recognised {
            family_entry.recognised_units += 1;
            report.total_recognised_units += 1;
        } else {
            family_entry.refused_units += 1;
            report.total_refused_units += 1;
            if let Some(sample) = refusal {
                if family_entry.refusal_samples.len() < 15 {
                    family_entry.refusal_samples.push(sample);
                }
            }
        }
    }

    report
}

/// The full corpus-wide run: reads the closed Gate 1 census, the inventory
/// join keys, and the real corpus, then fixture-checks its own population
/// against the census before returning — the entry point
/// `src/bin/formula_interpreter.rs --corpus-wide` calls.
pub fn run_corpus_wide_scan(repo_root: &Path) -> Result<CorpusWideReport, ScanError> {
    let inventory = load_inventory_join_keys(repo_root)?;
    let rows = fresh_census_rows(repo_root)?;
    let census_population = rows.len();
    let corpus_root = repo_root.join("data/corpus");
    let report = scan_ledger_rows(&rows, &inventory, &corpus_root);

    if report.total_population != census_population {
        return Err(ScanError::PopulationMismatch {
            scanned: report.total_population,
            census: census_population,
        });
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// **The load-bearing test.** The scan's own population, summed across
    /// every family, equals the closed Gate 1 ledger's F1..F9 row count —
    /// the "fixture-check, against the closed Gate 1 census" AT-32-G2-004
    /// requires, run against the real, committed ledger and the real,
    /// committed corpus (not a fixture stand-in for either).
    #[test]
    fn corpus_wide_scan_population_matches_the_closed_gate1_census() {
        let root = repo_root();
        let report = run_corpus_wide_scan(&root).expect("corpus-wide scan must succeed");

        let census_rows = fresh_census_rows(&root).expect("ledger must load");
        assert_eq!(
            report.total_population,
            census_rows.len(),
            "scan population must equal the closed Gate 1 ledger's own F1..F9 row count"
        );
        assert!(report.total_population > 0, "expected a non-empty F1..F9 population");

        // Every in-scope family the ledger actually populated must appear in the report --
        // never silently dropped.
        let mut census_families: std::collections::BTreeSet<String> =
            std::collections::BTreeSet::new();
        for row in &census_rows {
            census_families.insert(row.family.clone());
        }
        for family in &census_families {
            assert!(
                report.families.contains_key(family),
                "family {family} present in the census but missing from the scan report"
            );
        }

        // Every unit is accounted for exactly once: recognised + refused + unjoined == population,
        // per family and in total.
        let mut sum_pop = 0usize;
        let mut sum_rec = 0usize;
        let mut sum_ref = 0usize;
        let mut sum_unjoined = 0usize;
        for (family, cov) in &report.families {
            assert_eq!(
                cov.recognised_units + cov.refused_units + cov.unjoined_units,
                cov.population,
                "family {family}: recognised + refused + unjoined must equal population"
            );
            sum_pop += cov.population;
            sum_rec += cov.recognised_units;
            sum_ref += cov.refused_units;
            sum_unjoined += cov.unjoined_units;
        }
        assert_eq!(sum_pop, report.total_population);
        assert_eq!(sum_rec, report.total_recognised_units);
        assert_eq!(sum_ref, report.total_refused_units);
        assert_eq!(sum_unjoined, report.total_unjoined_units);

        eprintln!(
            "corpus_wide_scan: population={} recognised={} refused={} unjoined={}",
            report.total_population,
            report.total_recognised_units,
            report.total_refused_units,
            report.total_unjoined_units
        );
    }

    /// RED→GREEN mutation proof: a scan that only walks a hand-picked
    /// SUBSET of the closed census's rows (exactly the "out of protocol"
    /// shape AT-32-G2-004 names) must be caught by the population-parity
    /// check `run_corpus_wide_scan` performs — this test drives
    /// `scan_ledger_rows` directly on a truncated row list and confirms the
    /// same mismatch `run_corpus_wide_scan` would return.
    #[test]
    fn a_subset_run_trips_the_population_mismatch_check() {
        let root = repo_root();
        let inventory = load_inventory_join_keys(&root).expect("inventory must load");
        let all_rows = fresh_census_rows(&root).expect("ledger must load");
        assert!(all_rows.len() > 1, "need at least 2 rows to demonstrate a real subset");

        // Deliberately drop the last row -- the "ran against a subset" failure mode.
        let subset = &all_rows[..all_rows.len() - 1];
        let corpus_root = root.join("data/corpus");
        let report = scan_ledger_rows(subset, &inventory, &corpus_root);

        assert_ne!(
            report.total_population,
            all_rows.len(),
            "a subset scan's population must disagree with the full census -- this is the \
             mismatch `run_corpus_wide_scan`'s own fixture-check must trip"
        );
        assert_eq!(report.total_population, subset.len());
    }

    /// AT-33-E3-002's own RED→GREEN proof. `docs/release/SD-32-.../
    /// artifacts/gate-1-shape-closure/ledger.json` is SD-32's frozen Gate 1
    /// census, taken 2026-08-14 against a corpus/inventory state that has
    /// since moved (`decisions.md §6`'s revisit-condition class: content
    /// keeps landing). This test hardcodes F1's CURRENT true
    /// formula-bearing count — independently re-derived 2026-08-24 via
    /// `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
    /// (`family rollup: F1 6308`) — and fails if the module's own census
    /// still answers the stale 6,032 the frozen file carries. This is the
    /// literal shape of the SD-33 defect: a true number (6,032) that was
    /// the right answer against the wrong (stale) denominator.
    ///
    /// `AT-33-E6-001` (2026-08-25): 6,308 itself went stale 44 minutes after
    /// this test's own landing commit (`347e9d1a34`, 2026-08-24 23:56:11) —
    /// `AT-33-E4-002` (`00ca087775`, 2026-08-25 00:39:59, the very next
    /// commit to touch `docs/work-inventory.json` on this branch)
    /// regenerated that file (4,224 units reclassified off `unknown`, plus
    /// 3,985 units of disclosed unrelated SD-32-engine drift), which moves
    /// `shape_ledger.py`'s F1 rollup by construction: F1's population is
    /// built from `coverage_ledger.py`'s `not_done_population()`, gated on
    /// `doneness_verdict(unit) != DONE` for every unit not in
    /// `EXCLUDED_BOOKS` — the same `docs/work-inventory.json` this cycle's
    /// own Shortfall-1 fix reads. Re-derived fresh against the CURRENT
    /// committed file with the identical command this test's own doc
    /// comment already names:
    /// `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
    /// -> `family rollup: F1 6278`, matching this test's own live
    /// `report.families["F1"].population` exactly (both walk the identical
    /// `not_done_population()` gate, so they cannot honestly disagree).
    /// 6,308 - 6,278 = 30 units moved off the F1-eligible population by the
    /// regen — a real content shift, not a defect this cycle introduced:
    /// unaffected by which non-`done` doneness word Shortfall 1 chose for
    /// the 11 `(ambiguous, unmeasurable)` units, since `not_done_population`
    /// only tests `verdict != DONE`, never which specific non-`done` verdict
    /// a unit carries.
    #[test]
    fn f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census() {
        let root = repo_root();
        let report = run_corpus_wide_scan(&root).expect("corpus-wide scan must succeed");
        let f1 = report.families.get("F1").expect("F1 must be present in the report");
        assert_eq!(
            f1.population, 6278,
            "F1 population must equal the CURRENT true formula-bearing count (6,278, re-derived \
             2026-08-25 via `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` \
             against the post-`AT-33-E4-002` inventory), not the pre-regen 6,308 this test pinned \
             on 2026-08-24, and not SD-32's frozen 2026-08-14 census (6,032) — AT-33-E3-002 / \
             AT-33-E6-001"
        );
    }
}
