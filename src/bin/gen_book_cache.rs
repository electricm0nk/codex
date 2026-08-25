//! SD-27 Cycle E2.1/E2.2 -- the shared per-book codegen tool
//! `docs/release/SD-27-future-state-book-content-ingestion/
//! technical-design.md §2.2`/§3 names: it reads an already-completed
//! `rules_tables::<book>/` module's compiled state and serializes it to
//! `data/corpus/<book>/{content_kind}/<id>.json` as Shape B v1
//! (`src/rules_core/shape_b_v1.rs`) records, mirroring
//! `src/bin/gen_core_rulebook_cache.rs`'s established discipline:
//! it never re-derives a `data` field's *value* from raw LST at
//! generation time (values come from the compiled Rust module's own
//! accessors) -- it only reads the live corpus file to compute a real,
//! checkable per-record citation (path + SHA-256 + line number).
//!
//! **Single shared binary name, per book-agnostic design.** The SD-27
//! partition audit (`loop-instruction.md §6`) allow-lists exactly
//! `src/bin/gen_book_cache.rs` -- one file, not a per-book file --
//! for every per-book pre-build cycle (E2.1 Advanced Race Guide, E2.2
//! Pathfinder Unchained, and future SD-28+ cycles). This lands the first
//! book (`pathfinder_unchained`) actually wired; a later cycle authoring
//! ARG or another future-state book extends the `match` in `main()`
//! rather than replacing this file.
//!
//! **Why this binary does NOT `use codex::rules_core::rules_tables::
//! pathfinder_unchained` via the library crate.** SD-27's file-touch
//! partition (`decisions.md §8`, enforced by the literal regex in
//! `loop-instruction.md §6`) allow-lists `src/rules_core/rules_tables/
//! ${BOOK}/` (the new per-book subdirectory) but does NOT allow-list
//! `src/rules_core/rules_tables/mod.rs` (the shared parent module file
//! that would need a `pub mod pathfinder_unchained;` line to expose it
//! through the `codex` library crate). Rather than touch a file outside
//! this cycle's granted write scope, `pathfinder_unchained::mod` is
//! included directly into *this* binary crate via `#[path]`, matching
//! ordinary Rust module resolution (submodules declared inside a
//! `#[path]`-attributed `mod.rs` resolve relative to that file's own
//! directory, exactly as if it were reached via the normal `mod`
//! tree) -- the new rules-table module is real, compiles, and is
//! directly exercised by this tool; it is simply not (yet) exposed
//! through the library's public surface, which SD-28+'s eventual
//! `pilot_compute` integration cycle is free to do properly when it
//! also touches that file.
//!
//! **E2.1 (Advanced Race Guide) extension, this cycle.** Per this file's
//! own guidance above, `gen_advanced_race_guide()` (and its own
//! `#[path]`-included `advanced_race_guide` submodule) is added alongside
//! the already-landed `gen_pathfinder_unchained()` rather than replacing
//! it -- both books' generation logic now share this one file, matching
//! the partition audit's expectation that the two per-book cycles are
//! file-disjoint everywhere except this shared binary and may run in
//! parallel (`loop-instruction.md §3.3.3`: "File-disjoint with E2.1, so
//! it may run in parallel"). `main()`'s `match` picks the requested book
//! by its first CLI argument (`cargo run --bin gen_book_cache --
//! advanced_race_guide`); PU's own no-argument default is left
//! unchanged for backward compatibility with any existing invocation.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use codex::rules_core::cache_gen::WiringClassIndex;
use codex::rules_core::pi_screening;
use codex::rules_core::shape_b_v1::{Completeness, CorpusRecordV1, CorpusSource, License, Population};

/// The `(path, line, record_key)` a `CorpusSource` cites, when it cites a
/// real corpus row at all -- same rationale as
/// `gen_core_rulebook_cache.rs`'s own `wiring_citation`.
/// `WebSecondSource`/`SameBookFallback` carry no citation to read a token
/// closure from, so `wiring_class` for those lands on
/// `ambiguous:no_corpus_line` rather than guessing one.
fn wiring_citation(source: &CorpusSource) -> Option<(&str, u32, &str)> {
    match source {
        CorpusSource::LstToken { path, line, record_key, .. }
        | CorpusSource::LstInheritedCopy { path, line, record_key, .. }
        | CorpusSource::LstCorrectedIngest { path, line, record_key, .. } => {
            Some((path.as_str(), *line, record_key.as_str()))
        }
        CorpusSource::WebSecondSource { .. } | CorpusSource::SameBookFallback { .. } => None,
    }
}

/// `path` relative to `index`'s own book_dir, matching `CorpusLines::line`'s
/// single-level join (D0): the citation's own path tail past the
/// `/{book_id}/` marker when the marker is present (every real corpus path
/// carries a directory segment before the book name, e.g.
/// `campaign_setting/inner_sea_gods/support/isg_races_b4.lst`), or the bare
/// basename otherwise.
///
/// Before this fix, every caller passed a bare `path.rsplit('/').next()`
/// basename regardless of how deep the citation actually sits under the
/// book's root -- correct by coincidence for the first nine books this lane
/// onboarded (this file's own `load_corpus_file_rel` doc comment already
/// names the reason: their citations all happen to sit directly at the
/// book's top level), wrong for `inner_sea_gods`, which keeps 3 monster
/// rows under a `support/` subdirectory. `CorpusLines::line`'s
/// `dir.join(file)` then silently resolved to a nonexistent top-level path,
/// `unwrap_or_default()` swallowed the read failure, and the three affected
/// records shipped stamped `wiring_class: "ambiguous"` (`no_corpus_line`)
/// instead of the `derived` their real BONUS/DR tokens call for --
/// `v06_corpus_trap_report -- --audit` catches the same class of citation
/// resolution bug this fixes; `resolve_book_file`'s own doc comment already
/// flagged the underlying limitation for the citation/sha256 path, this
/// closes it for the wiring-class path too (code review finding
/// SD30-E8-F3-002/003, `decisions.md §51` scope note).
fn wiring_class_file_arg(book_id: &str, path: &str) -> String {
    let marker = format!("/{book_id}/");
    match path.find(&marker) {
        Some(at) => path[at + marker.len()..].to_string(),
        None => path.rsplit('/').next().unwrap_or(path).to_string(),
    }
}

fn wiring_class_for_source(
    index: &WiringClassIndex,
    lines: &mut codex::rules_core::wiring_class::CorpusLines,
    source: &CorpusSource,
) -> (String, Vec<String>) {
    match wiring_citation(source) {
        Some((path, line, record_key)) => {
            // `SD31-E6-F9-005`: a citation whose own `source.path` names
            // `core_essentials`'s directory was NOT read out of `index`'s
            // primary book directory (`load_corpus_file_rel_with_fallback`
            // already fell back there to build the citation itself) -- route
            // the wiring-class lookup to the SAME fallback directory
            // (`build_with_extra`/`wiring_class_for_book`), keyed
            // `"core_essentials"`, rather than always the primary `book_id`,
            // which does not contain this file and would silently stamp
            // `ambiguous`/`no_corpus_line` regardless of the row's real shape.
            let ce_prefix = format!("{CORE_ESSENTIALS_RELATIVE}/");
            if let Some(file) = path.strip_prefix(&ce_prefix) {
                index.wiring_class_for_book(lines, "core_essentials", file, line, record_key, record_key)
            } else {
                let file = wiring_class_file_arg(index.book_id(), path);
                index.wiring_class_for(lines, &file, line, record_key, record_key)
            }
        }
        None => ("ambiguous".to_string(), vec!["no_corpus_line".to_string()]),
    }
}

// SD-27 task "wire PU's 4 Unchained classes" (2026-07-31): the `#[path]`
// include this line used to carry is gone. It existed only because the
// authoring cycle could not touch `rules_tables/mod.rs` to add
// `pub mod pathfinder_unchained;` (see this file's own doc comment above,
// which explicitly invited a later cycle to undo it). That line has since
// landed, so the module is on the library's public surface and is imported
// here like any other. Nothing about the generated cache changes -- it is
// the same source file, reached by the ordinary module path instead of a
// second, duplicate compilation of it into this binary crate.
use codex::rules_core::rules_tables::pathfinder_unchained;

// SD28-E30 (`epic-32-archetype-swap`): `advanced_race_guide::archetype_tables`
// now depends on `rules_tables::archetype_swap`'s shared
// `ArchetypeGrant`/`ArchetypeSwapEntry` struct. Since `advanced_race_guide`
// is duplicated into this binary crate via `#[path]` rather than reached
// through the library crate (see this file's own doc comment above), its
// new dependency has to be duplicated the same way, or the `super::super::`
// path inside `archetype_tables.rs` resolves against this binary's own
// crate root (where `advanced_race_guide` sits directly, not under a
// `rules_tables` parent) and fails to find it.
// SD-29 Epic 7 round 9: the two `#[path]` duplicates above are RETIRED and this
// binary now reaches Advanced Race Guide through the library crate, exactly as
// line 103 already reached `pathfinder_unchained`.
//
// The duplicates were a write-scope workaround from an era when
// `rules_tables/mod.rs` was outside the cycle's granted surface (see this
// file's doc comment). That premise expired long ago — `rules_tables/mod.rs`
// declares `pub mod advanced_race_guide;` and `pub mod archetype_swap;` today —
// and the duplication became load-bearing in the wrong direction the moment ARG
// gained a `companion` family: a `#[path]`-included `mod.rs` resolves its
// `super::` against THIS binary's crate root, where there is no
// `companion_chassis`, so `mod companion_data;` inside ARG failed to compile
// here while compiling fine in the library.
//
// Retiring it is smaller than the alternative, which was duplicating
// `companion_chassis` and `monster_chassis` into this binary as well and
// carrying two copies of the companion tables in one build.
use codex::rules_core::rules_tables::advanced_race_guide;

const BOOK_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/pathfinder_unchained";

fn corpus_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/pathfinder_unchained")
}

const ARG_BOOK_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/advanced_race_guide";

fn arg_corpus_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT_ARG") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_race_guide")
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn ingested_at_now() -> String {
    let output = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("date -u must be available to stamp ingested_at");
    String::from_utf8(output.stdout).expect("date output is valid UTF-8").trim().to_string()
}

fn slugify(raw: &str) -> String {
    let mut out = String::new();
    let mut last_was_sep = false;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep {
            out.push('_');
            last_was_sep = true;
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.is_empty() { "record".to_string() } else { trimmed }
}

struct CorpusFile {
    relative_path: String,
    sha256: String,
    lines: Vec<String>,
}

fn load_corpus_file(root: &Path, file_name: &str) -> CorpusFile {
    load_corpus_file_rel(root, BOOK_RELATIVE, file_name)
}

/// Same as `load_corpus_file`, parametrized by the book's own
/// `BOOK_RELATIVE`-shaped prefix -- lets a second (or Nth) book's
/// generator function share this loader without hardcoding PU's own
/// `BOOK_RELATIVE` const.
/// The real location of `file_name` inside a book directory, which is not
/// always its root, expressed as a path relative to `root` -- or `None` on
/// zero matches, so a caller can try a fallback root
/// (`load_corpus_file_rel_with_fallback` is that caller) before deciding the
/// file is genuinely absent.
///
/// A unit's `source_file` is a BARE BASENAME -- that is what
/// `v06_work_inventory` records, and it is what both `MonsterStatBlock` and
/// `MonsterAbilityRecord` carry. For the first nine books in this lane the
/// basename was also the file's location, so joining it onto the book root was
/// correct by coincidence rather than by rule. `inner_sea_gods` keeps 3 monster
/// rows and 16 ability rows under `support/`, and `occult_adventures` its one
/// monster row; joining onto the root there fails outright.
///
/// Two failure modes, matching `transcribe_monster_tables.py::resolve_book_file`
/// term for term so that the transcriber and the generator cannot disagree
/// about which file a citation names:
///
/// * **Not found under THIS root** -- returns `None`, not a panic: the
///   `core_essentials` fallback (`SD31-E6-F9-005`) means "absent here" is not
///   yet "absent everywhere," matching the Python sibling's own two-directory
///   search.
/// * **Found more than once under THIS root** -- panics regardless of what a
///   fallback root might also hold: a bare basename matching two real files
///   does not identify a row, and picking either is a coin flip on which
///   rules text ships.
fn try_resolve_book_file(root: &Path, file_name: &str) -> Option<String> {
    fn walk(dir: &Path, file_name: &str, prefix: &str, found: &mut Vec<String>) {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                walk(&path, file_name, &format!("{prefix}{name}/"), found);
            } else if name == file_name {
                found.push(format!("{prefix}{name}"));
            }
        }
    }
    let mut found = Vec::new();
    walk(root, file_name, "", &mut found);
    found.sort();
    match found.len() {
        0 => None,
        1 => Some(found.remove(0)),
        n => panic!(
            "{file_name} resolves to {n} files under {root:?} ({}) -- a bare \
             basename that names two real files does not identify a row",
            found.join(", ")
        ),
    }
}

/// `pathfinder/paizo/roleplaying_game/core_essentials`, the one other place a
/// `decisions.md §9`-re-attributed unit's physical file can live: re-attribution
/// moves the reporting `book` field, never the file. Mirrors
/// `transcribe_monster_tables.py`'s `_CORE_ESSENTIALS_DIR` term for term, so the
/// Python transcriber and this Rust generator can never disagree about which
/// file a citation names -- confirmed live (`SD31-E6-F9-005`): before this
/// fallback existed, re-running the Python transcriber for `bestiary`/
/// `bestiary_2` newly reached 168 real, owned `ce_abilities_race.lst`-origin
/// ability rows that this generator then could not cite at all, panicking with
/// "not in this book's MonsterBookSpec::abilities_lsts" the instant it tried.
const CORE_ESSENTIALS_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/core_essentials";

fn load_corpus_file_rel(root: &Path, book_relative: &str, file_name: &str) -> CorpusFile {
    load_corpus_file_rel_with_fallback(root, book_relative, None, file_name)
}

/// Same as [`load_corpus_file_rel`], but when `file_name` is absent from
/// `root` entirely, also tries `core_essentials`'s directory -- computed from
/// `corpus_data_root` (the directory ABOVE `pathfinder/...`, i.e. the same
/// root `root` itself was joined onto `book_relative` under), unless `root`
/// already IS that directory (no self-fallback, matching the Python sibling's
/// own "core essentials does not fall back to itself" rule). The citation's
/// `relative_path` reflects whichever directory the file was ACTUALLY found
/// under -- re-attribution changes the reporting `book` field, never where the
/// bytes the sha256 was taken over actually live.
fn load_corpus_file_rel_with_fallback(
    root: &Path,
    book_relative: &str,
    corpus_data_root: Option<&Path>,
    file_name: &str,
) -> CorpusFile {
    let (used_root, used_book_relative, resolved) = match try_resolve_book_file(root, file_name) {
        Some(found) => (root.to_path_buf(), book_relative.to_string(), found),
        None => {
            let data_root = corpus_data_root.unwrap_or_else(|| {
                panic!(
                    "{file_name} is not present anywhere under {root:?} (no core_essentials \
                     fallback root was supplied for this call)"
                )
            });
            let ce_root = data_root.join(CORE_ESSENTIALS_RELATIVE);
            if ce_root == root {
                panic!("{file_name} is not present anywhere under {root:?}");
            }
            match try_resolve_book_file(&ce_root, file_name) {
                Some(found) => (ce_root, CORE_ESSENTIALS_RELATIVE.to_string(), found),
                None => panic!(
                    "{file_name} is not present anywhere under {root:?} or {ce_root:?}"
                ),
            }
        }
    };
    let full = used_root.join(&resolved);
    let bytes = fs::read(&full).unwrap_or_else(|e| panic!("failed to read corpus file {full:?}: {e}"));
    let sha256 = sha256_hex(&bytes);
    let text = String::from_utf8_lossy(&bytes).to_string();
    CorpusFile {
        // The RESOLVED sub-path under whichever root ACTUALLY held the file,
        // not the bare basename and not necessarily the originally-requested
        // book: a record's `path` citation must lead a reader to the file the
        // sha256 was taken over.
        relative_path: format!("{used_book_relative}/{resolved}"),
        sha256,
        lines: text.lines().map(|s| s.to_string()).collect(),
    }
}

/// Finds the real 1-indexed line whose first tab-delimited column exactly
/// equals `identity` (skipping comment/blank lines) -- the same
/// first-column-identity convention every `pu_*.lst` row in this cycle's
/// 2 in-scope files uses.
fn find_line_by_identity(file: &CorpusFile, identity: &str) -> Option<u32> {
    for (idx, line) in file.lines.iter().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let first_col = line.split('\t').next().unwrap_or_default().trim();
        if first_col == identity {
            return Some((idx + 1) as u32);
        }
    }
    None
}

/// A KEY:-token-first, identity-fallback line index over one corpus file
/// -- mirrors `gen_core_rulebook_cache.rs`'s `LineIndex`/`build_line_index`
/// lookup order (prefer an exact `KEY:` token match; several ARG records,
/// e.g. every `arg_equipmods.lst` row and the one `Drow ~ Spider Step`
/// feat, have a `key` that differs from their corpus identity/display
/// name).
struct ArgLineIndex<'a> {
    by_key: HashMap<&'a str, u32>,
    by_identity: HashMap<&'a str, u32>,
}

fn arg_build_line_index(file: &CorpusFile) -> ArgLineIndex<'_> {
    let mut by_key = HashMap::new();
    let mut by_identity = HashMap::new();
    for (idx, line) in file.lines.iter().enumerate() {
        let line_no = (idx + 1) as u32;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if let Some(k) = line.split('\t').find_map(|f| f.strip_prefix("KEY:")) {
            by_key.entry(k).or_insert(line_no);
        }
        if let Some(id) = line.split('\t').find(|f| !f.is_empty()) {
            by_identity.entry(id).or_insert(line_no);
        }
    }
    ArgLineIndex { by_key, by_identity }
}

fn arg_find_citation_line(index: &ArgLineIndex<'_>, wanted_key: &str) -> Option<u32> {
    index.by_key.get(wanted_key).copied().or_else(|| index.by_identity.get(wanted_key).copied())
}

/// Like `arg_find_citation_line`, but also returns the literal corpus
/// identity string the matched row actually declares -- for a plain match
/// that is just `wanted_key`, but for a `.COPY=<wanted_key>` racial
/// spell-like-ability variant (decisions.md §15, 2026-08-17) it is the
/// full `<parent>.COPY=<wanted_key>` string, so the record's `source`
/// cites the row that actually declares it rather than a byte-match that
/// does not exist on this row.
fn arg_find_citation_line_with_identity(index: &ArgLineIndex<'_>, wanted_key: &str) -> Option<(u32, String)> {
    if let Some(line) = arg_find_citation_line(index, wanted_key) {
        return Some((line, wanted_key.to_string()));
    }
    let copy_suffix = format!(".COPY={wanted_key}");
    index
        .by_identity
        .iter()
        .find(|(identity, _)| identity.ends_with(&copy_suffix))
        .map(|(identity, line)| (*line, identity.to_string()))
}

/// Every real `*.json` record under `book_dir`, counted directly from disk
/// rather than from this run's own in-memory write count.
///
/// **Why this exists.** `LICENSE.json`'s `records_processed` used to be
/// set from `feat_written + equipment_written` (this generator's own
/// count) -- correct only when this generator is the sole writer into the
/// book directory. `ingest_pu_classes.rs` and `ingest_race_traits.rs`
/// also write real, licence-classified records into the same directories
/// and never touch `LICENSE.json` (`tests/sd27_book_license_record_counts.rs`'s
/// own module doc comment documents this as a known, previously-reported
/// gap). The combined counts that used to be correct on disk (PU 127, ARG
/// 635) were a ONE-OFF MANUAL edit (`b4504c49`), not a mechanism -- so the
/// very next time this generator ran standalone (GE-01's 2026-08-03
/// regeneration cycle), it silently reverted both back to its own
/// narrower count (59, 479), undocumenting 68 and 156 already-screened
/// records as screened. Deriving the count from disk at write time, the
/// same way the test itself verifies it, is correct regardless of write
/// order or which other binaries have already run.
fn count_on_disk_records(book_dir: &Path) -> usize {
    fn walk(dir: &Path, count: &mut usize) {
        let Ok(entries) = fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            // `_parity/` (and any similarly `_`-prefixed directory, matching
            // this corpus's existing `_pfs`-style convention for
            // non-content storage) holds test fixtures, not licensed
            // content records -- `tests/sd27_book_license_record_counts.rs`
            // counts only equipment/feat/race_trait/spell.
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

/// How many of this book's on-disk records carry a redaction marker.
///
/// The companion to [`count_on_disk_records`], and it exists for the same
/// reason (`decisions.md §63.2`): every `LICENSE.json` this binary writes states
/// `records_processed` as the **book-wide on-disk count across every lane that
/// has ingested it**, and then stated `records_redacted: 0` as a literal. For a
/// book only this binary has ever written, the literal was true and read as
/// though it were general. It is not: `core_essentials` was ingested first by
/// the race-trait lane, which redacted **9** of its 64 heritage-trait records,
/// and running any generator here over that directory silently rewrote the 9 to
/// a 0 while all nine `[redacted PI]` markers stayed on disk — a book-wide claim
/// that no record was redacted, published over the evidence that nine were.
///
/// Derived from the same walk as the numerator it must agree with, so the two
/// can never disagree about which files are in scope. Verified against the
/// declaration the race-trait lane wrote by hand:
/// `grep -rl 'redacted PI' data/corpus/core_essentials/race_trait/ | wc -l` → 9.
fn count_on_disk_redactions(book_dir: &Path) -> usize {
    fn walk(dir: &Path, count: &mut usize) {
        let Ok(entries) = fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let is_internal = path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .is_some_and(|n| n.starts_with('_'));
                if !is_internal {
                    walk(&path, count);
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|f| f.to_str()) != Some("LICENSE.json")
                && fs::read_to_string(&path)
                    .is_ok_and(|raw| raw.contains(PI_REDACTION_MARKER))
            {
                *count += 1;
            }
        }
    }
    let mut count = 0;
    walk(book_dir, &mut count);
    count
}

/// The marker every lane writes in place of redacted Product Identity prose.
/// Named once so the counter above and the `redaction_policy.marker` field
/// every `LICENSE.json` publishes cannot drift apart.
const PI_REDACTION_MARKER: &str = "[redacted PI]";

fn write_record<T: serde::Serialize>(path: &Path, record: &CorpusRecordV1<T>) {
    fs::create_dir_all(path.parent().expect("record path must have a parent dir")).expect("failed to create output dir");
    let json = serde_json::to_string_pretty(record).expect("record must serialize");
    fs::write(path, json).unwrap_or_else(|e| panic!("failed to write {path:?}: {e}"));
}

/// Heuristic OGL/PI screen (`docs/governance/ogl-pi-blacklist.md`), the
/// same bounded, documented substring scan `scripts/apg_license_retrofit.py`
/// already applied to the 4 in-scope books: the 20 canonical core
/// Golarion deities plus a sampled set of known setting proper nouns.
/// Shared across both of this binary's per-book generators
/// (`gen_pathfinder_unchained()` and `gen_advanced_race_guide()`). No hit
/// anywhere in either book's real record text (independently re-verified
/// per book) -- every PU and ARG record in these caches classifies
/// `"OGL"`.
const PI_BLACKLIST_TERMS: &[&str] = &[
    "Iomedae", "Sarenrae", "Asmodeus", "Cayden Cailean", "Abadar", "Calistria", "Desna", "Erastil", "Gorum", "Gozreh",
    "Irori", "Lamashtu", "Nethys", "Norgorber", "Pharasma", "Rovagug", "Shelyn", "Torag", "Urgathoa", "Zon-Kuthon",
    "Golarion", "Absalom", "Cheliax", "Varisia", "Andoran", "Taldor", "Osirion", "Katapesh", "Ustalav", "Numeria",
    "Mwangi", "Tian Xia", "Avistan", "Garund", "Sarkoris", "Worldwound", "Vudra", "Kyonin", "Molthune", "Nidal",
    "Nirmathas", "Qadira", "Razmiran", "Rahadoum", "Galt", "Isger", "Lastwall", "Brevoy", "Druma", "Irrisen",
    "Jalmeray", "Thuvia", "Geb", "Nex",
];

/// Returns `(license, pi_field, pi_marker, stored_value)` for a text
/// field per the PI-blacklist screen.
fn classify_field(field_name: &str, value: &str) -> (License, Option<String>, Option<String>, String) {
    for term in PI_BLACKLIST_TERMS {
        if value.contains(term) {
            return (
                License::PiRedacted,
                Some(field_name.to_string()),
                Some(codex::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string()),
                codex::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string(),
            );
        }
    }
    (License::Ogl, None, None, value.to_string())
}

/// Renders one spell's raw `DESC:` token into the prose a corpus record may
/// carry, and **fails the run** if PCGen syntax survives.
///
/// The `spell_list` tables store each description exactly as the corpus
/// writes it — prose plus, where the book states a caster-level formula, a
/// `%N` reference and its `|`-delimited argument tail. Writing that straight
/// into `data/corpus/advanced_race_guide/spell/*.json` is what shipped
/// `Absorbing Inhalation` reading *"for up to %1 rounds"* and ending
/// *"…the cloud's effects|CASTERLEVEL"*. 13 of ARG's 92 spell records carried
/// a leak of this class; `advanced_players_guide` carried 3 more.
///
/// `render_pcgen_desc` owns the treatment (`%%` de-escapes, an integer-literal
/// `%N` substitutes, a formula `%N` is dropped and reported — never guessed,
/// because `decisions.md §24` rules out a formula interpreter). The
/// `leaked_pcgen_syntax` panic is the port of the production guard
/// `src/bin/ingest_races.rs` and `src/bin/ingest_race_traits.rs` already
/// carry: a future leak stops this generator instead of reaching a screen.
fn render_player_facing_description(record_key: &str, raw: &str) -> String {
    let rendered = codex::rules_core::pcgen_desc::render_pcgen_desc(raw);
    if let Some(leak) = codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text) {
        panic!(
            "record {record_key:?}: rendered description still carries {leak}. Raw token: {raw:?}"
        );
    }
    rendered.text
}

#[derive(serde::Serialize)]
struct FeatCacheData {
    key: String,
    category: String,
    name: String,
    description: Option<String>,
    source_page: Option<String>,
}

#[derive(serde::Serialize)]
struct EquipmentCacheData {
    key: String,
    category: String,
    name: String,
    equip_type: String,
    plus: Option<u8>,
    cost_gp: Option<f64>,
    weight_lbs: Option<f64>,
    description: Option<String>,
}

fn gen_pathfinder_unchained() {
    let root = corpus_root();
    let out_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/corpus/pathfinder_unchained");
    let ingested_at = ingested_at_now();
    let wiring_index = WiringClassIndex::build("pathfinder_unchained", &root);
    let mut wiring_lines = wiring_index.lines();

    // **SD-32 Epic 5 protective sweep (`epic-breakdown.md` Epic 5, T3
    // residual / `defects.md` D9)**: this used to unconditionally
    // `remove_dir_all` both `feat` and `equipment` on every run, then
    // rewrite every entry from scratch -- the S6/D9 self-erasure shape
    // `gen_monster_book`, in this SAME FILE, was already fixed for
    // (`SD31-E6-F9-005`), never extended here. All 42 of 42 on-disk
    // `pathfinder_unchained` equipment records carry a `raw_tokens` field
    // `enrich_equipment_raw_tokens.rs` writes AFTER this generator runs,
    // which this generator's own `EquipmentCacheData` cannot reconstruct.
    // Same fix as `gen_monster_book`: a file is removed ONLY when its key
    // is ABSENT from the set this run just computed (per write loop,
    // below), never wiped wholesale up front.
    let mut current_feat_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut current_equipment_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

    // ---- Feats ----
    let feats_file = load_corpus_file(&root, "pu_feats.lst");
    let mut feat_written = 0u32;
    let mut feat_unattributed: Vec<String> = Vec::new();
    for entry in pathfinder_unchained::feat_tables::feat_tables() {
        match find_line_by_identity(&feats_file, entry.key) {
            Some(line) => {
                let source = CorpusSource::LstToken {
                    path: feats_file.relative_path.clone(),
                    sha256: feats_file.sha256.clone(),
                    line,
                    record_key: entry.key.to_string(),
                };
                let (license, pi_field, pi_marker, stored_desc) = match entry.description {
                    Some(desc) => {
                        let (license, pi_field, pi_marker, stored) = classify_field("description", desc);
                        (license, pi_field, pi_marker, Some(stored))
                    }
                    None => (License::Ogl, None, None, None),
                };
                let data = FeatCacheData {
                    key: entry.key.to_string(),
                    category: format!("{:?}", entry.category),
                    name: entry.name.to_string(),
                    description: stored_desc,
                    source_page: entry.source_page.map(|s| s.to_string()),
                };
                let (wiring_class, wiring_class_signals) =
                    wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly },
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
                    wiring_class,
                    wiring_class_signals,
                    description_source: None,
                };
                current_feat_keys.insert(entry.key.to_string());
                let path = out_root.join("feat").join(format!("{}.json", slugify(entry.key)));
                // `SD31-E6-F9-005`-shaped guard (see this fn's own doc
                // comment above): a file already on disk is left completely
                // untouched, not re-derived.
                if !path.exists() {
                    write_record(&path, &record);
                }
                feat_written += 1;
            }
            None => feat_unattributed.push(entry.key.to_string()),
        }
    }
    if out_root.join("feat").exists() {
        // Single writer of `pathfinder_unchained/feat/` (verified: neither
        // `cache_gen::feat_gap` nor `cache_gen::hand_authored_feat_dump`
        // registers `pathfinder_unchained` -- SD-32 cross-generator sweep,
        // 2026-08-23), so an unscoped citation predicate is safe here.
        codex::rules_core::cache_gen::ultimate_equipment::remove_stale_owned_files(
            &out_root.join("feat"),
            &current_feat_keys,
            &|_path, _line| true,
        );
    }

    // ---- Equipment (equipmods) ----
    let equipmods_file = load_corpus_file(&root, "pu_equipmods.lst");
    let mut equipment_written = 0u32;
    let mut equipment_unattributed: Vec<String> = Vec::new();
    let mut used_slugs: HashMap<String, u32> = HashMap::new();
    for entry in pathfinder_unchained::equipment_tables::equipment_tables() {
        match find_line_by_identity(&equipmods_file, entry.name) {
            Some(line) => {
                let source = CorpusSource::LstToken {
                    path: equipmods_file.relative_path.clone(),
                    sha256: equipmods_file.sha256.clone(),
                    line,
                    record_key: entry.key.to_string(),
                };
                let (license, pi_field, pi_marker, stored_desc) = match entry.description {
                    Some(desc) => {
                        let (license, pi_field, pi_marker, stored) = classify_field("description", desc);
                        (license, pi_field, pi_marker, Some(stored))
                    }
                    None => (License::Ogl, None, None, None),
                };
                let data = EquipmentCacheData {
                    key: entry.key.to_string(),
                    category: "equipmods".to_string(),
                    name: entry.name.to_string(),
                    equip_type: entry.equip_type.to_string(),
                    plus: entry.plus,
                    cost_gp: None,
                    weight_lbs: None,
                    description: stored_desc,
                };
                let (wiring_class, wiring_class_signals) =
                    wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly },
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
                    wiring_class,
                    wiring_class_signals,
                    description_source: None,
                };
                current_equipment_keys.insert(entry.key.to_string());
                let base_slug = slugify(entry.name);
                let count = used_slugs.entry(base_slug.clone()).or_insert(0);
                *count += 1;
                let slug = if *count == 1 { base_slug } else { format!("{base_slug}_{count}") };
                let path = out_root.join("equipment").join(format!("{slug}.json"));
                // `SD31-E6-F9-005`-shaped guard: a file already on disk
                // (including one `enrich_equipment_raw_tokens.rs` has since
                // written `raw_tokens` into) is left completely untouched.
                if !path.exists() {
                    write_record(&path, &record);
                }
                equipment_written += 1;
            }
            None => equipment_unattributed.push(entry.name.to_string()),
        }
    }
    if out_root.join("equipment").exists() {
        // Single writer of `pathfinder_unchained/equipment/` (verified: no
        // other `cache_gen` module registers `pathfinder_unchained` for the
        // `equipment` kind -- SD-32 cross-generator sweep, 2026-08-23; this
        // comment's book name corrected 2026-08-24 -- it named
        // `advanced_race_guide`, copy-pasted from a sibling call site, but
        // this call site is inside `gen_pathfinder_unchained()`. The
        // underlying verification (no other writer of THIS function's own
        // `out_root.join("equipment")`) was already correct; only the
        // comment's book name was wrong).
        codex::rules_core::cache_gen::ultimate_equipment::remove_stale_owned_files(
            &out_root.join("equipment"),
            &current_equipment_keys,
            &|_path, _line| true,
        );
    }

    println!("SD-27 E2.2 pathfinder_unchained cache generation report");
    println!(
        "  feats written: {feat_written} / {}",
        pathfinder_unchained::feat_tables::feat_tables().len()
    );
    if !feat_unattributed.is_empty() {
        println!("  feats UNATTRIBUTED (skipped): {feat_unattributed:?}");
    }
    println!(
        "  equipment written: {equipment_written} / {}",
        pathfinder_unchained::equipment_tables::equipment_tables().len()
    );
    if !equipment_unattributed.is_empty() {
        println!("  equipment UNATTRIBUTED (skipped): {equipment_unattributed:?}");
    }

    // ---- LICENSE.json ----
    // Computed once, referenced by both `records_processed` and the note's
    // own prose, so the two can never state two different numbers (exactly
    // the self-contradiction `tests/sd27_book_license_record_counts.rs`'s
    // `the_screening_note_quotes_the_same_count_the_field_states` checks for).
    let records_processed = count_on_disk_records(&out_root);
    let records_redacted = count_on_disk_redactions(&out_root);
    let license_json = serde_json::json!({
        "book": "pathfinder_unchained",
        "license_declaration": {
            "open_game_content": "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2",
            "product_identity_source": "Paizo Pathfinder Roleplaying Game: Pathfinder Unchained, OGL §15 Product Identity section",
            "product_identity_note": "Named deities, NPCs, and unique places are Product Identity per the book's own OGL Section 15 declaration; this book's own feat and equipment-modifier MECHANICS are Open Game Content."
        },
        "redaction_policy": {
            "marker": PI_REDACTION_MARKER,
            "schema_preserving": true,
            "pi_field_recorded": true,
            "blacklist_source": "docs/governance/ogl-pi-blacklist.md",
            "blacklist_version_reviewed": "2026-07-27"
        },
        "screening_method_note": format!(
            "This pass is a heuristic first-pass screen of every `description` value against a bounded, documented term list (the 20 canonical core Golarion deities plus a sampled set of known setting place names, the same list docs/governance/ogl-pi-blacklist.md's operating cycle used for the 4 in-scope books' retro-fit). This generator's own run screened {} records ({feat_written} feats + {equipment_written} equipment modifiers), zero PI hits -- consistent with this book's own subject matter (alignment/stamina/wound-threshold feats and Automatic Bonus Progression equipment modifiers) being entirely rules-mechanical, setting-neutral text. `records_processed` below is {records_processed}, the full on-disk count derived at write time rather than from this run alone, since `ingest_pu_classes.rs` also writes real, separately-screened records (its own `pi_hits()` term scan) into this book's directory. This is NOT an exhaustive human legal review; it is a bounded substring/regex scan against ~54 known names and does not prove the absence of PI beyond what that scan can see.",
            feat_written + equipment_written
        ),
        "redistribution_posture": "ogl-notice-attached",
        "classified_at": ingested_at,
        "classified_by_cycle": "E2.2",
        "records_processed": records_processed,
        "records_redacted": records_redacted,
        "operator_sign_off": {
            "signed_off": false,
            "signed_off_at": null,
            "note": "Set true only after an operator has reviewed this book's classification pass, per docs/governance/ogl-pi-blacklist.md's DRAFT header."
        }
    });
    let license_path = out_root.join("LICENSE.json");
    fs::write(&license_path, serde_json::to_string_pretty(&license_json).unwrap() + "\n")
        .unwrap_or_else(|e| panic!("failed to write {license_path:?}: {e}"));
    println!("  LICENSE.json written to {}", license_path.display());

    // `decisions.md §20` no_record-to-zero, round 4: this book's own
    // `monster`/`monster_ability` family, previously unwired because its CLI
    // dispatch special-cases this function (`main()`'s own `match`, above the
    // generic `monster_book_spec` arm) rather than reaching the
    // `MonsterBookSpec`-driven `gen_monster_book` path every other book here
    // uses. `pathfinder_unchained` is now a registered `MonsterBookSpec`
    // (zero monster rows, 72 owner-less ability rows) -- reuse the SAME
    // mechanism, unmodified, rather than duplicating its write/PI-screen/
    // stale-file-clear logic here.
    gen_monster_book(
        monster_book_spec("pathfinder_unchained")
            .expect("pathfinder_unchained must be registered in MONSTER_BOOK_SPECS"),
    );
}

/// SD-27 Cycle E2.1 -- Advanced Race Guide. Extends this shared binary's
/// `match` per this file's own module doc comment ("a later cycle
/// authoring ARG ... extends the `match` in `main()` rather than
/// replacing this file"). Real, independently re-verified record counts
/// (differ from the cycle's scoping-brief rough estimates -- see
/// `advanced_race_guide::spell_list`/`equipment_tables`/`feats`'s own doc
/// comments for the full accounting): 92 spells, 200 equipment (28
/// ArmsArmor + 79 General + 78 MagicItems + 15 Equipmods), 187 feats (132
/// General + 52 Combat + 3 Teamwork).
fn gen_advanced_race_guide() {
    let root = arg_corpus_root();
    let out_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/corpus/advanced_race_guide");
    let ingested_at = ingested_at_now();
    let wiring_index = WiringClassIndex::build("advanced_race_guide", &root);
    let mut wiring_lines = wiring_index.lines();

    // **SD-32 Epic 5 protective sweep (`defects.md` D9), live-reproduced**:
    // this used to unconditionally `remove_dir_all` all three of `spell`,
    // `equipment`, `feat` on every run, then rewrite every entry from
    // scratch. Live-reproduced against this repo's real committed corpus
    // in an isolated worktree (git status clean before, `git checkout --`
    // after): one run wiped `raw_tokens` from all 93 `advanced_race_guide`
    // spell records (100% of the book's spell population) and permanently
    // deleted 15 real, populated `equipment` records belonging to
    // `gen_equipment_gap_tables.rs`/`cache_gen::equipment_gap`'s own write
    // into the SAME `equipment` directory. Same fix as `gen_monster_book`
    // (`SD31-E6-F9-005`) for `spell`/`feat` -- a file is removed ONLY when
    // its key is ABSENT from the set this run just computed. `equipment`
    // gets the exists-guard WITHOUT a stale-key sweep (unlike `spell`/
    // `feat`): a stale-key sweep there would delete
    // `cache_gen::equipment_gap`'s own 15 records the instant this
    // generator ran, since their keys are never in this generator's own
    // `equipment_tables()` -- the exact collision this fix must not
    // reintroduce.
    let mut current_spell_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut current_feat_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

    // ---- Spells ----
    let spells_file = load_corpus_file_rel(&root, ARG_BOOK_RELATIVE, "arg_spells.lst");
    let spells_index = arg_build_line_index(&spells_file);
    let mut spell_written = 0u32;
    let mut spell_unattributed: Vec<String> = Vec::new();
    let mut spell_slugs_used: std::collections::HashSet<String> = std::collections::HashSet::new();
    for entry in advanced_race_guide::spell_list::SPELL_LIST {
        match arg_find_citation_line_with_identity(&spells_index, entry.key) {
            Some((line_no, record_key)) => {
                let rendered = render_player_facing_description(entry.key, entry.description);
                let (license, pi_field, pi_marker, stored_desc) = classify_field("description", &rendered);
                let data = advanced_race_guide::json_cache::SpellCacheData {
                    key: entry.key.to_string(),
                    school: format!("{:?}", entry.school),
                    level: entry.level,
                    description: stored_desc,
                };
                let source = CorpusSource::LstToken {
                    path: spells_file.relative_path.clone(),
                    sha256: spells_file.sha256.clone(),
                    line: line_no,
                    record_key,
                };
                let (wiring_class, wiring_class_signals) =
                    wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: Completeness::Full,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
                    wiring_class,
                    wiring_class_signals,
                    description_source: None,
                };
                let base = slugify(entry.key);
                let slug = if spell_slugs_used.insert(base.clone()) {
                    base
                } else {
                    let mut n = 2;
                    loop {
                        let candidate = format!("{base}_{n}");
                        if spell_slugs_used.insert(candidate.clone()) {
                            break candidate;
                        }
                        n += 1;
                    }
                };
                current_spell_keys.insert(entry.key.to_string());
                let path = out_root.join("spell").join(format!("{slug}.json"));
                if !path.exists() {
                    write_record(&path, &record);
                }
                spell_written += 1;
            }
            None => spell_unattributed.push(entry.key.to_string()),
        }
    }
    if out_root.join("spell").exists() {
        // Single writer of `advanced_race_guide/spell/` (verified:
        // `advanced_race_guide` is in neither `cache_gen::spell_lane_dump`'s
        // nor `cache_gen::spell_mod_access`'s book lists -- SD-32
        // cross-generator sweep, 2026-08-23).
        codex::rules_core::cache_gen::ultimate_equipment::remove_stale_owned_files(
            &out_root.join("spell"),
            &current_spell_keys,
            &|_path, _line| true,
        );
    }

    // ---- Equipment ----
    let equip_file_names = [
        "arg_equip_arms_armor.lst",
        "arg_equip_general.lst",
        "arg_equip_magic_items.lst",
        "arg_equipmods.lst",
    ];
    let mut equip_files: HashMap<&str, CorpusFile> = HashMap::new();
    for f in equip_file_names {
        equip_files.insert(f, load_corpus_file_rel(&root, ARG_BOOK_RELATIVE, f));
    }
    let mut equip_indexes: HashMap<&str, ArgLineIndex<'_>> = HashMap::new();
    for (name, file) in &equip_files {
        equip_indexes.insert(name, arg_build_line_index(file));
    }
    let equip_corpus_file_name = |category: advanced_race_guide::equipment_tables::EquipmentCategory| -> &'static str {
        use advanced_race_guide::equipment_tables::EquipmentCategory::*;
        match category {
            ArmsArmor => "arg_equip_arms_armor.lst",
            General => "arg_equip_general.lst",
            MagicItems => "arg_equip_magic_items.lst",
            Equipmods => "arg_equipmods.lst",
        }
    };
    let equip_category_slug = |category: advanced_race_guide::equipment_tables::EquipmentCategory| -> &'static str {
        use advanced_race_guide::equipment_tables::EquipmentCategory::*;
        match category {
            ArmsArmor => "arms_armor",
            General => "general",
            MagicItems => "magic_items",
            Equipmods => "equipmods",
        }
    };

    let mut equipment_written = 0u32;
    let mut equipment_unattributed: Vec<String> = Vec::new();
    let mut equipment_slugs_used: HashMap<&'static str, std::collections::HashSet<String>> = HashMap::new();
    for entry in advanced_race_guide::equipment_tables::equipment_tables() {
        let file_name = equip_corpus_file_name(entry.category);
        let file = &equip_files[file_name];
        let index = &equip_indexes[file_name];
        match arg_find_citation_line(index, entry.key) {
            Some(line_no) => {
                let (license, pi_field, pi_marker, stored_desc) = match entry.description {
                    Some(desc) => {
                        let (l, f, m, s) = classify_field("description", desc);
                        (l, f, m, Some(s))
                    }
                    None => (License::Ogl, None, None, None),
                };
                let completeness = if entry.description.is_some() {
                    Completeness::Full
                } else if entry.cost_gp.is_some() || entry.weight_lbs.is_some() {
                    Completeness::ChassisPlusExtract
                } else {
                    Completeness::ChassisOnly
                };
                let category_slug = equip_category_slug(entry.category);
                let data = advanced_race_guide::json_cache::EquipmentCacheData {
                    key: entry.key.to_string(),
                    category: category_slug.to_string(),
                    name: entry.name.to_string(),
                    cost_gp: entry.cost_gp,
                    weight_lbs: entry.weight_lbs,
                    description: stored_desc,
                };
                let source = CorpusSource::LstToken {
                    path: file.relative_path.clone(),
                    sha256: file.sha256.clone(),
                    line: line_no,
                    record_key: entry.key.to_string(),
                };
                let (wiring_class, wiring_class_signals) =
                    wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
                    wiring_class,
                    wiring_class_signals,
                    description_source: None,
                };
                let used = equipment_slugs_used.entry(category_slug).or_default();
                let base = slugify(entry.key);
                let slug = if used.insert(base.clone()) {
                    base
                } else {
                    let mut n = 2;
                    loop {
                        let candidate = format!("{base}_{n}");
                        if used.insert(candidate.clone()) {
                            break candidate;
                        }
                        n += 1;
                    }
                };
                // Exists-guard only, deliberately NO stale-key sweep here --
                // see this fn's own doc comment for why a sweep would
                // wrongly delete `cache_gen::equipment_gap`'s own 15
                // records sharing this same directory.
                let path = out_root.join("equipment").join(category_slug).join(format!("{slug}.json"));
                if !path.exists() {
                    write_record(&path, &record);
                }
                equipment_written += 1;
            }
            None => equipment_unattributed.push(format!("{:?}:{}", entry.category, entry.key)),
        }
    }

    // ---- Feats ----
    let feats_file = load_corpus_file_rel(&root, ARG_BOOK_RELATIVE, "arg_feats.lst");
    let feats_index = arg_build_line_index(&feats_file);
    let feat_category_slug = |category: advanced_race_guide::feats::FeatCategory| -> &'static str {
        use advanced_race_guide::feats::FeatCategory::*;
        match category {
            General => "general",
            Combat => "combat",
            Teamwork => "teamwork",
        }
    };
    let mut feat_written = 0u32;
    let mut feat_unattributed: Vec<String> = Vec::new();
    let mut feat_slugs_used: HashMap<&'static str, std::collections::HashSet<String>> = HashMap::new();
    for entry in advanced_race_guide::feats::feat_tables() {
        match arg_find_citation_line(&feats_index, entry.key) {
            Some(line_no) => {
                let (license, pi_field, pi_marker, stored_desc) = match entry.description {
                    Some(desc) => {
                        let (l, f, m, s) = classify_field("description", desc);
                        (l, f, m, Some(s))
                    }
                    None => (License::Ogl, None, None, None),
                };
                let effect_vec: Vec<Vec<String>> = entry
                    .effect
                    .map(|bonuses| bonuses.iter().map(|b| b.qualifiers.iter().map(|q| q.to_string()).collect()).collect())
                    .unwrap_or_default();
                let category_slug = feat_category_slug(entry.category);
                let data = advanced_race_guide::json_cache::FeatCacheData {
                    key: entry.key.to_string(),
                    category: category_slug.to_string(),
                    name: entry.name.to_string(),
                    description: stored_desc,
                    effect: effect_vec,
                };
                let source = CorpusSource::LstToken {
                    path: feats_file.relative_path.clone(),
                    sha256: feats_file.sha256.clone(),
                    line: line_no,
                    record_key: entry.key.to_string(),
                };
                let (wiring_class, wiring_class_signals) =
                    wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: Completeness::Full,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
                    wiring_class,
                    wiring_class_signals,
                    description_source: None,
                };
                let used = feat_slugs_used.entry(category_slug).or_default();
                let base = slugify(entry.key);
                let slug = if used.insert(base.clone()) {
                    base
                } else {
                    let mut n = 2;
                    loop {
                        let candidate = format!("{base}_{n}");
                        if used.insert(candidate.clone()) {
                            break candidate;
                        }
                        n += 1;
                    }
                };
                current_feat_keys.insert(entry.key.to_string());
                let path = out_root.join("feat").join(category_slug).join(format!("{slug}.json"));
                if !path.exists() {
                    write_record(&path, &record);
                }
                feat_written += 1;
            }
            None => feat_unattributed.push(format!("{:?}:{}", entry.category, entry.key)),
        }
    }
    // **SD-32 Epic 5 protective sweep correction (2026-08-24), live-reproduced
    // against the real pinned corpus**: the previous stale-key sweep here
    // (`remove_stale_owned_files` with an unscoped `|_p,_l| true` predicate)
    // carried a comment claiming "Single writer of `pathfinder_unchained/feat/`"
    // -- the WRONG book, copy-pasted from `gen_pathfinder_unchained`'s own
    // identical call above and never re-verified for `advanced_race_guide`.
    // `cache_gen::feat_gap::FEAT_GAP_BOOKS` DOES register `advanced_race_guide`
    // (it writes gap-filled feat records this generator's own curated
    // `feat_tables()` list does not model, into this SAME `feat/` directory,
    // parsed from the SAME `arg_feats.lst`). Reproduced live: one run deleted
    // 48 real `cache_gen::feat_gap` records (e.g.
    // `data/corpus/advanced_race_guide/feat/angelic_flesh_brazen.json`,
    // `source.path` = `arg_feats.lst`, a flat top-level file -- never under
    // this generator's own `feat/<category_slug>/` nesting) -- reverted
    // immediately (`git checkout -- data/corpus`), never committed. No
    // stale-key sweep here now, same carve-out already used for `equipment`
    // just above (`cache_gen::equipment_gap`'s own 15 records) and for the
    // identical reason: this generator can never regenerate a sibling
    // generator's own gap-filled records, so it must never delete one either.
    // A record already on disk is left alone via the `exists()` guard above;
    // nothing here removes what this run does not itself own.

    let total = spell_written + equipment_written + feat_written;

    // ---- LICENSE.json ----
    // See gen_pathfinder_unchained()'s own comment: computed once so
    // `records_processed` and the note's prose can never disagree.
    let records_processed = count_on_disk_records(&out_root);
    let records_redacted = count_on_disk_redactions(&out_root);
    let license_json = serde_json::json!({
        "book": "advanced_race_guide",
        "license_declaration": {
            "open_game_content": "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2",
            "product_identity_source": "Paizo Pathfinder Roleplaying Game Advanced Race Guide, OGL §15 Product Identity section",
            "product_identity_note": "Named deities, NPCs, and unique places are Product Identity per the book's own OGL Section 15 declaration; core spell/equipment/feat and racial-trait MECHANICS are Open Game Content."
        },
        "redaction_policy": {
            "marker": PI_REDACTION_MARKER,
            "schema_preserving": true,
            "pi_field_recorded": true,
            "blacklist_source": "docs/governance/ogl-pi-blacklist.md",
            "blacklist_version_reviewed": "2026-07-27"
        },
        "screening_method_note": format!(
            "This pass is a heuristic first-pass screen of every `description` value against a bounded, documented term list (the same PI_BLACKLIST_TERMS this binary's gen_pathfinder_unchained() also uses -- the 20 canonical core Golarion deities plus a sampled set of known setting place names, matching docs/governance/ogl-pi-blacklist.md's operating cycle for the 4 in-scope books' retro-fit). This generator's own run screened {total} records (spells + equipment + feats), zero PI hits. `records_processed` below is {records_processed}, the full on-disk count derived at write time rather than from this run alone, since `ingest_race_traits.rs` also writes real, separately-screened alternate-racial-trait records into this book's directory. This is NOT an exhaustive human legal review; it is a bounded substring/regex scan against ~54 known names and does not prove the absence of PI beyond what that scan can see."
        ),
        "redistribution_posture": "ogl-notice-attached",
        "classified_at": ingested_at,
        "classified_by_cycle": "E2.1",
        "records_processed": records_processed,
        "records_redacted": records_redacted,
        "operator_sign_off": {
            "signed_off": false,
            "signed_off_at": null,
            "note": "Set true only after an operator has reviewed this book's classification pass, per docs/governance/ogl-pi-blacklist.md's DRAFT header."
        }
    });
    let license_path = out_root.join("LICENSE.json");
    fs::write(&license_path, serde_json::to_string_pretty(&license_json).unwrap() + "\n")
        .unwrap_or_else(|e| panic!("failed to write {license_path:?}: {e}"));

    println!("SD-27 E2.1 advanced_race_guide cache generation report");
    println!("  spells written: {spell_written} / {}", advanced_race_guide::spell_list::SPELL_LIST.len());
    if !spell_unattributed.is_empty() {
        println!("  spells UNATTRIBUTED: {spell_unattributed:?}");
    }
    println!(
        "  equipment written: {equipment_written} / {}",
        advanced_race_guide::equipment_tables::equipment_tables().len()
    );
    if !equipment_unattributed.is_empty() {
        println!("  equipment UNATTRIBUTED: {equipment_unattributed:?}");
    }
    println!("  feats written: {feat_written} / {}", advanced_race_guide::feats::feat_tables().len());
    if !feat_unattributed.is_empty() {
        println!("  feats UNATTRIBUTED: {feat_unattributed:?}");
    }
    println!("  LICENSE.json written to {}", license_path.display());

    // `decisions.md §20` no_record-to-zero, round 4: this book's own
    // `monster`/`monster_ability` family, previously unwired for the same
    // reason `gen_pathfinder_unchained` above names -- reuse the SAME
    // `MonsterBookSpec`-driven mechanism rather than duplicating it.
    gen_monster_book(
        monster_book_spec("advanced_race_guide")
            .expect("advanced_race_guide must be registered in MONSTER_BOOK_SPECS"),
    );
}

/// The Bonus Bestiary corpus cache -- SD-29 Epic 5's pilot, and the first
/// `monster_ability` records this repo has ever written to disk.
///
/// Two kinds, one book directory, per `docs/release/corpus-work-channels.md`
/// §9.2: `monster/` is the chassis and `monster_ability/` is the features
/// attached to it, the same shape `race`/`race_trait` already have. Both are
/// dumped from the compiled `rules_tables::bonus_bestiary` module -- this
/// generator never re-derives a value from raw LST, exactly as
/// `gen_pathfinder_unchained`/`gen_advanced_race_guide` above do not. It reads
/// the live `.lst` only to attach a real `path`/`sha256`/`line` citation, and
/// the line it cites is the one the table itself recorded at transcription
/// time, verified against the file rather than trusted.
/// Compose a `LICENSE.json` `screening_method_note` append-only.
///
/// `decisions.md §54.4`: a book can be ingested by several lanes, and the note
/// is the only field carrying which passes put records on disk. Overwriting it
/// leaves a file whose `records_processed` counts every lane and whose note
/// accounts for one. Written once here rather than a third time inline: this
/// function exists because the companion generator's copy was the fix and the
/// monster generator's absence of it was the defect, and a second copy is how
/// that happens again.
///
/// `marker` is this pass's own leading text. Finding it means this cycle already
/// appended, so its previous entry is replaced rather than duplicated — a
/// re-run of the same cycle is a no-op on the note, the way a second run of
/// `v06_work_inventory` is a no-op on the inventory.
fn compose_screening_note(prior: Option<String>, marker: &str, this_pass: String) -> String {
    let Some(previous) = prior else {
        return this_pass;
    };
    let head = match previous.find(marker) {
        Some(at) => previous[..at].trim_end().to_string(),
        None => previous.trim_end().to_string(),
    };
    if head.is_empty() {
        this_pass
    } else {
        format!("{head} {this_pass}")
    }
}

fn gen_monster_book(spec: &MonsterBookSpec) {
    use codex::rules_core::rules_tables::monster_chassis;

    let book_id = spec.corpus_book;
    let table = monster_chassis::monster_book(book_id)
        .unwrap_or_else(|| panic!("{book_id} is not registered in monster_chassis::MONSTER_BOOKS"));
    let root = monster_book_corpus_root(spec);
    let out_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/corpus").join(book_id);
    let ingested_at = ingested_at_now();
    // The wiring class is COMPUTED from each cited row's own token closure,
    // never asserted here. A first draft hard-coded `static` for every record
    // on the reasoning that every field is a verbatim token; `v06_corpus_trap_report
    // -- --audit` rejected 17 of the 31 records for exactly that
    // (`wiring-class-mismatch`), because the class describes what the ROW does
    // -- `Water Naga ~ Poison` carries a `BONUS:VAR` and is `derived`, most
    // ability rows carry no magnitude token at all and are `display`.
    // `SD31-E6-F9-005`: also index `core_essentials`'s own directory under
    // its own book key, `"core_essentials"` -- a `decisions.md §9`-re-attributed
    // ability's wiring-class citation needs it (`wiring_class_for_source`
    // below routes to it when the citation's own `source.path` names that
    // directory), and without it every such record silently stamps
    // `wiring_class: "ambiguous"` regardless of its real corpus shape.
    let wiring_data_root = monster_book_corpus_data_root(spec);
    let wiring_index = match &wiring_data_root {
        Some(data_root) => {
            let ce_dir = data_root.join(CORE_ESSENTIALS_RELATIVE);
            if ce_dir == root {
                WiringClassIndex::build(book_id, &root)
            } else {
                WiringClassIndex::build_with_extra(book_id, &root, "core_essentials", &ce_dir)
            }
        }
        None => WiringClassIndex::build(book_id, &root),
    };
    let mut wiring_lines = wiring_index.lines();

    // Clear only what is GENUINELY STALE: a file this generator owns (its
    // `data.key` is namespaced to this book and kind) whose key is no longer
    // in the table's CURRENT membership.
    //
    // **CORRECTED (`SD31-E6-F9-005`): this used to remove EVERY owned file
    // unconditionally, regardless of whether its key was still in the table**
    // -- confirmed live against this repo's own checked-in corpus: every one
    // of `beastiary`/`bestiary_2`'s already-shipped `monster`/`monster_ability`
    // records carries a `data.raw_tokens` array a LATER, SEPARATE enrichment
    // pass wrote (`enrich_monster_raw_tokens.rs`/
    // `enrich_monster_ability_raw_tokens.rs` -- neither field exists on
    // `MonsterStatBlock`/`MonsterAbilityRecord` at all, so this generator can
    // never reconstruct it). The unconditional clear-then-rewrite silently
    // regenerated every one of those 724 already-enriched records in the
    // narrower un-enriched base shape the instant this generator ran again --
    // "Do NOT regenerate `data/corpus/` wholesale" the hard way, and exactly
    // the "generated artifacts mutated post-hoc" hazard this program has
    // already named once (`docs/retro/tranche-*`).
    //
    // `data/corpus/beastiary/monster/` ALSO holds 46 records SD-22 wrote, in
    // the pre-`key` Shape B v1 shape (`data.id`, no `data.key`), beside the
    // 280+ this chassis writes (`decisions.md §58.3`) -- those are excluded by
    // the SAME `data.key`-prefix ownership test as before, unchanged.
    //
    // The new rule: a file this generator owns is removed ONLY when its key is
    // ABSENT from the table this run just computed -- a real deletion (a
    // record dropped from the source `.lst`, or newly PI-excluded), never a
    // record that is merely about to be rewritten with the same key. See the
    // write loops below for the matching half: a file whose key IS present is
    // left COMPLETELY untouched (not rewritten in the base shape either),
    // preserving whatever a later enrichment pass added.
    let current_monster_keys: std::collections::HashSet<String> = table
        .monsters
        .iter()
        .map(|m| format!("{book_id}:monster:{}", slugify(m.key)))
        .collect();
    let current_ability_keys: std::collections::HashSet<String> = table
        .monster_abilities
        .iter()
        .map(|a| format!("{book_id}:monster_ability:{}", slugify(a.key)))
        .collect();
    for (sub, current_keys) in [
        ("monster", &current_monster_keys),
        ("monster_ability", &current_ability_keys),
    ] {
        let dir = out_root.join(sub);
        if !dir.exists() {
            continue;
        }
        let prefix = format!("{book_id}:{sub}:");
        for entry in fs::read_dir(&dir).expect("read generated subdir") {
            let path = entry.expect("read generated subdir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let key = fs::read_to_string(&path)
                .ok()
                .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
                .and_then(|v| v.get("data")?.get("key")?.as_str().map(str::to_string));
            let Some(key) = key else { continue };
            if key.starts_with(&prefix) && !current_keys.contains(&key) {
                fs::remove_file(&path).expect("clear stale generated record");
            }
        }
    }

    // `SD31-E6-F9-005`: a `decisions.md §9`-re-attributed unit's `source_file`
    // can name a file that physically lives under `core_essentials`'s own
    // directory rather than this book's -- see `load_corpus_file_rel_with_fallback`.
    // Same root the wiring-class index above was built from -- one call, one
    // answer, never re-derived a second way.
    let corpus_data_root = wiring_data_root;
    // Keyed by file name, because a record's `source_line` is only meaningful
    // together with its `source_file` -- see `MonsterBookSpec::races_lsts`.
    let races_files: HashMap<&'static str, CorpusFile> = spec
        .races_lsts
        .iter()
        .map(|name| {
            (
                *name,
                load_corpus_file_rel_with_fallback(
                    &root,
                    spec.book_relative,
                    corpus_data_root.as_deref(),
                    name,
                ),
            )
        })
        .collect();
    // Keyed by file name for `races_files`' reason: an ability's `source_line`
    // is only meaningful together with its `source_file`.
    let abilities_files: HashMap<&'static str, CorpusFile> = spec
        .abilities_lsts
        .iter()
        .map(|name| {
            (
                *name,
                load_corpus_file_rel_with_fallback(
                    &root,
                    spec.book_relative,
                    corpus_data_root.as_deref(),
                    name,
                ),
            )
        })
        .collect();

    // ---- monsters ----
    let mut monster_written = 0u32;
    let mut monster_kept = 0u32;
    let mut pi_hits: Vec<String> = Vec::new();
    for block in table.monsters {
        // **`SD31-E6-F9-005`: a record whose file already exists is left
        // COMPLETELY untouched** -- not rewritten in the base shape, not
        // re-citation-verified. See the clear loop above for why: a later
        // enrichment pass (`raw_tokens`) writes fields this generator's own
        // `MonsterStatBlock`-shaped `data` object cannot reconstruct, and
        // rewriting would silently discard them. This means an EXISTING
        // record's underlying `.lst` citation is not re-verified by this run
        // -- `v06_corpus_trap_report --audit`'s own citation check covers
        // that ground independently, on the already-written file, every gate.
        let out_path = out_root.join("monster").join(format!("{}.json", slugify(block.key)));
        if out_path.exists() {
            monster_kept += 1;
            continue;
        }
        // The display name, not the key: the first column of a monster row is
        // the display name, and Monster Codex is the first book where they
        // differ (`Sootwing Bat` in column 1, `KEY:Bat (Sootwing)`).
        let races_file = races_files.get(block.source_file).unwrap_or_else(|| {
            panic!(
                "{book_id}:{} cites {}, which is not in this book's MonsterBookSpec::races_lsts \
                 ({:?}) -- a citation this generator cannot verify is not a citation",
                block.key, block.source_file, spec.races_lsts
            )
        });
        let line = verified_citation_line(races_file, block.source_line, block.name, false);
        let data = serde_json::json!({
            "key": format!("{book_id}:monster:{}", slugify(block.key)),
            "corpus_key": block.key,
            "name": block.name,
            "size": block.size,
            "speeds": block.speeds.iter().map(|s| serde_json::json!({ "mode": s.mode, "feet": s.feet })).collect::<Vec<_>>(),
            "race_type": block.race_type,
            "race_subtype": block.race_subtype,
            "challenge_rating": block.challenge_rating,
            "monster_class": block.monster_class,
            "source_page": block.source_page,
            "natural_attacks": block.natural_attacks.iter().map(|a| serde_json::json!({ "name": a.name, "damage_dice": a.damage_dice })).collect::<Vec<_>>(),
            "ability_keys": block.ability_keys.iter().map(|k| format!("{book_id}:monster_ability:{}", slugify(k))).collect::<Vec<_>>(),
            "external_ability_refs": block.external_ability_refs,
            // SD31-E6-F1-002: the same `{ability, amount}` shape the
            // companion generator already emits for `stat_adjustments`
            // below (search this file for `"stat_adjustments"` on a
            // `CompanionRecord` for the precedent) -- a delta against a base
            // this ingest does not carry, never a final ability score.
            "stat_adjustments": block.stat_adjustments.iter().map(|a| serde_json::json!({ "ability": a.ability, "amount": a.amount })).collect::<Vec<_>>(),
            "has_spell_like_abilities": block.has_spell_like_abilities,
        });
        pi_hits.extend(monster_record_pi_hits(block.key, &data.to_string()));
        let source = CorpusSource::LstToken {
            path: races_file.relative_path.clone(),
            sha256: races_file.sha256.clone(),
            line,
            record_key: block.key.to_string(),
        };
        let (wiring_class, wiring_class_signals) =
            wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::ChassisOnly,
            ingested_at: ingested_at.clone(),
            data,
            source,
            license: Some(License::Ogl),
            pi_field: None,
            pi_marker: None,
            wiring_class,
            wiring_class_signals,
            description_source: None,
        };
        write_record(&out_path, &record);
        monster_written += 1;
    }

    // ---- monster abilities ----
    let mut ability_written = 0u32;
    let mut ability_kept = 0u32;
    for ability in table.monster_abilities {
        // Same "leave existing files completely alone" rule as the monster
        // loop above, and the same reason (`raw_tokens`).
        let out_path = out_root
            .join("monster_ability")
            .join(format!("{}.json", slugify(ability.key)));
        if out_path.exists() {
            ability_kept += 1;
            continue;
        }
        let abilities_file = abilities_files.get(ability.source_file).unwrap_or_else(|| {
            panic!(
                "{book_id}:{} cites {}, which is not in this book's \
                 MonsterBookSpec::abilities_lsts ({:?}) -- a citation this \
                 generator cannot verify is not a citation",
                ability.key, ability.source_file, spec.abilities_lsts
            )
        });
        let line = verified_citation_line(
            abilities_file,
            ability.source_line,
            ability.name,
            ability.codex_generated_name,
        );
        // SD-32 declared-pi-shipping-65-followups: this loop used to
        // hardcode `license: Ogl`/`pi_field: None` unconditionally, even
        // when `ability.description` was already the redaction marker (a
        // static-table literal a prior pass had blanked by hand). The
        // `monster_record_pi_hits` hard gate below still catches any LIVE,
        // un-redacted blacklist-term text in `description` and aborts the
        // whole run before anything is written -- so a description that
        // reaches this point is always either ordinary prose or already
        // the marker, and `classify_optional_field_declared` correctly
        // stamps both cases (this is the same shared classifier
        // `cache_gen::{equipment_gap, feat_gap, class_feature}` already
        // use for their own optional text fields).
        let (ability_license, ability_pi_field, ability_pi_marker, _) =
            pi_screening::classify_optional_field_declared("description", ability.description, false);
        let data = serde_json::json!({
            "key": format!("{book_id}:monster_ability:{}", slugify(ability.key)),
            "corpus_key": ability.key,
            "name": ability.name,
            "facet": ability.facet.corpus_token(),
            "delivery": ability.delivery.map(|d| d.corpus_token()),
            "traits": ability.traits,
            "description": ability.description,
            "description_variables": ability.description_variables,
            "source_page": ability.source_page,
            "owners": ability.owners.iter().map(|o| format!("{book_id}:monster:{}", slugify(o))).collect::<Vec<_>>(),
            // `decisions.md §24b`-3: "a field marks it as carrying a
            // Codex-generated name". `§24b`-4: the divergence stops at the
            // coordinate -- never the original string, which is why there
            // is no field here that could carry it.
            "codex_generated_name": ability.codex_generated_name,
            "rename": if ability.codex_generated_name {
                serde_json::json!({
                    "reason": ability.rename_reason,
                    "coordinate": ability.rename_coordinate,
                })
            } else {
                serde_json::Value::Null
            },
        });
        pi_hits.extend(monster_record_pi_hits(ability.key, &data.to_string()));
        let source = CorpusSource::LstToken {
            path: abilities_file.relative_path.clone(),
            sha256: abilities_file.sha256.clone(),
            line,
            record_key: ability.key.to_string(),
        };
        let (wiring_class, wiring_class_signals) =
            wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: ingested_at.clone(),
            data,
            source,
            license: Some(ability_license),
            pi_field: ability_pi_field,
            pi_marker: ability_pi_marker,
            wiring_class,
            wiring_class_signals,
            description_source: None,
        };
        write_record(&out_path, &record);
        ability_written += 1;
    }

    // Epic 3's provenance gate, applied to this lane's extraction step: a hit
    // is a hard stop, not a warning. Nothing is written past this point, and
    // what was already written is left for inspection rather than silently
    // shipped -- the operator has to see the hit.
    if !pi_hits.is_empty() {
        eprintln!("PI screen FAILED for {book_id}: {pi_hits:?}");
        std::process::exit(1);
    }

    let records_processed = count_on_disk_records(&out_root);
    let records_redacted = count_on_disk_redactions(&out_root);
    let license_path = out_root.join("LICENSE.json");
    // A book can be ingested by more than one lane. Monster Codex's
    // `race_trait/` records were written by `ingest_race_traits.rs` first, and
    // that binary derived a sharper OGL citation than this spec carries (it
    // cites the `.pcc`'s ISOGL line and COPYRIGHT block by line number).
    // Clobbering it would replace a real derivation with a weaker one and would
    // leave the note describing 5 race-trait records in a directory that now
    // holds 10 records across three kinds -- which
    // `the_screening_note_quotes_the_same_count_the_field_states` would catch,
    // but only after the wrong artifact had been written. The prior
    // declaration is preserved and the note is rewritten to cover every lane.
    let prior: Option<serde_json::Value> = fs::read_to_string(&license_path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok());
    let prior_declaration = prior
        .as_ref()
        .and_then(|v| v.get("license_declaration"))
        .cloned();
    // `decisions.md §54.4`'s fix, which was applied to the companion generator
    // and NOT to this one. The declaration was preserved here from the first
    // run; the SCREENING NOTE was not, and that half is the one carrying
    // history. `data/corpus/beastiary/LICENSE.json` states four earlier passes
    // by cycle, date and record count -- E2.0.9's 45, `ingest_races`' 119,
    // SD28-E16's 5 and the companion lane's 59 -- and this generator would have
    // replaced all of it with a sentence about its own 607 rows the first time
    // round 8 ran it, leaving a file whose `records_processed` and whose method
    // note account for different things.
    //
    // The note is append-only from here, and idempotent: re-running the same
    // cycle replaces its own trailing pass rather than stacking a copy.
    let prior_note = prior
        .as_ref()
        .and_then(|v| v.get("screening_method_note"))
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let this_pass = format!(
        "PASS -- {} (monster lane), {ingested_at}: every field of the {} records this run wrote ({monster_written} monsters + {ability_written} monster abilities) was screened against the bounded, documented term list in docs/governance/ogl-pi-blacklist.md, zero hits. A hit is a hard stop in this generator, not a warning. records_processed is {records_processed}: the real on-disk count for this book across every kind any lane has ingested, which for a book ingested by more than one lane is larger than this run's own output. This is NOT an exhaustive human legal review; it is a bounded substring scan and does not prove the absence of PI beyond what that scan can see.",
        spec.classified_by_cycle,
        monster_written + ability_written
    );
    let screening_method_note = compose_screening_note(
        prior_note,
        &format!("PASS -- {} (monster lane)", spec.classified_by_cycle),
        this_pass,
    );
    let license_json = serde_json::json!({
        "book": book_id,
        "license_declaration": prior_declaration.unwrap_or_else(|| serde_json::json!({
            "open_game_content": spec.open_game_content,
            "product_identity_source": spec.product_identity_source,
            "product_identity_note": "Named deities, NPCs and unique places are Product Identity; monster stat blocks and their special-ability rules text are Open Game Content."
        })),
        "redaction_policy": {
            "marker": PI_REDACTION_MARKER,
            "schema_preserving": true,
            "pi_field_recorded": true,
            "blacklist_source": "docs/governance/ogl-pi-blacklist.md",
            "blacklist_version_reviewed": "2026-07-27"
        },
        "screening_method_note": screening_method_note,
        "redistribution_posture": "ogl-notice-attached",
        "classified_at": ingested_at,
        "classified_by_cycle": spec.classified_by_cycle,
        "records_processed": records_processed,
        "records_redacted": records_redacted,
        "operator_sign_off": {
            "signed_off": false,
            "signed_off_at": null,
            "note": "Set true only after an operator has reviewed this book's classification pass, per docs/governance/ogl-pi-blacklist.md's DRAFT header."
        }
    });
    let license_path = out_root.join("LICENSE.json");
    fs::write(&license_path, serde_json::to_string_pretty(&license_json).unwrap() + "\n")
        .unwrap_or_else(|e| panic!("failed to write {license_path:?}: {e}"));
    println!(
        "{book_id} cache generated: {monster_written} new monsters ({monster_kept} already on disk, \
         left untouched), {ability_written} new monster abilities ({ability_kept} already on disk, \
         left untouched); LICENSE.json records_processed={records_processed}"
    );
}

/// One companion book's corpus cache -- SD-29 Epic 7 (companion lane).
///
/// One kind, not two, unlike `gen_monster_book`: `v06_work_inventory::file_kind`
/// types both a book's `*_races_companion.lst` creature rows and its
/// `*_abilities_companion.lst` ability rows as `Kind::Companion`, so the corpus
/// writes both under `data/corpus/<book>/companion/` and each record states its
/// own `record_type`. Splitting them into two directories here would create a
/// corpus family the inventory has no kind for, and the two would then be
/// counted against a denominator that does not exist.
///
/// Everything else is `gen_monster_book`'s shape and for its reasons: the values
/// are dumped from the compiled `rules_tables` module, the `.lst` is re-read
/// only to attach a real `path`/`sha256`/`line` citation, that line is verified
/// against the file rather than trusted, and a PI hit is a hard stop.
fn gen_companion_book(spec: &CompanionBookSpec) {
    use codex::rules_core::rules_tables::companion_chassis;

    let book_id = spec.corpus_book;
    let table = companion_chassis::companion_book(book_id).unwrap_or_else(|| {
        panic!("{book_id} is not registered in companion_chassis::COMPANION_BOOKS")
    });
    let root = companion_book_corpus_root(spec);
    let out_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/corpus").join(book_id);
    let ingested_at = ingested_at_now();
    // Index `core_essentials`'s own directory under its own book key too, for
    // the reason `gen_monster_book` states: a `decisions.md §9`-re-attributed
    // row's citation `path` names that directory, and without the extra index
    // `wiring_class_for_source` cannot find the row and stamps `ambiguous`
    // regardless of the row's real corpus shape.
    let wiring_data_root = companion_book_corpus_data_root(spec);
    let wiring_index = match &wiring_data_root {
        Some(data_root) => {
            let ce_dir = data_root.join(CORE_ESSENTIALS_RELATIVE);
            if ce_dir == root {
                WiringClassIndex::build(book_id, &root)
            } else {
                WiringClassIndex::build_with_extra(book_id, &root, "core_essentials", &ce_dir)
            }
        }
        None => WiringClassIndex::build(book_id, &root),
    };
    let mut wiring_lines = wiring_index.lines();

    // **SD-32 Epic 5 protective sweep (`defects.md` D9)**: this used to
    // unconditionally `remove_dir_all` the whole `companion` directory on
    // every run, then rewrite every entry from scratch -- confirmed
    // vulnerable by code-read the same way `gen_advanced_race_guide` was
    // live-reproduced (`grep -n "out_path.exists()" src/bin/gen_book_cache.rs`
    // returns zero hits for this function). 927 of 927 companion records
    // (100% of the kind, across 16 `CompanionBookSpec` books) carry a
    // `raw_tokens` field `enrich_companion_raw_tokens.rs` writes AFTER this
    // generator runs, which this generator's own inline `serde_json::json!`
    // shape cannot reconstruct. Same fix as `gen_monster_book`
    // (`SD31-E6-F9-005`): a file is removed ONLY when its key is ABSENT
    // from the set this run just computed.
    let mut current_companion_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Keyed by file name, because a record's `source_line` is only meaningful
    // together with its `source_file` -- see `CompanionRecord::source_file`.
    //
    // Loaded through the `core_essentials` fallback: a `decisions.md §9`
    // re-attributed row reports its real book (`ce_races_familiar_cr.lst` says
    // `SOURCELONG:Bestiary`, so its rows report `bestiary`) while its physical
    // file never leaves `core_essentials`' PCGen directory. Same rule, same
    // term, as `transcribe_companion_tables`'s own resolver.
    let races_files: HashMap<&'static str, CorpusFile> = spec
        .races_lsts
        .iter()
        .map(|name| {
            (
                *name,
                load_corpus_file_rel_with_fallback(
                    &root,
                    spec.book_relative,
                    wiring_data_root.as_deref(),
                    name,
                ),
            )
        })
        .collect();
    let abilities_files: HashMap<&'static str, CorpusFile> = spec
        .abilities_lsts
        .iter()
        .map(|name| {
            (
                *name,
                load_corpus_file_rel_with_fallback(
                    &root,
                    spec.book_relative,
                    wiring_data_root.as_deref(),
                    name,
                ),
            )
        })
        .collect();

    let mut pi_hits: Vec<String> = Vec::new();
    let mut creature_written = 0u32;
    for companion in table.companions {
        let races_file = races_files.get(companion.source_file).unwrap_or_else(|| {
            panic!(
                "{book_id}:{} cites {}, which is not in this book's \
                 CompanionBookSpec::races_lsts ({:?}) -- a citation this generator \
                 cannot verify is not a citation",
                companion.key, companion.source_file, spec.races_lsts
            )
        });
        let line = verified_citation_line(races_file, companion.source_line, companion.name, false);
        let data = serde_json::json!({
            "key": format!("{book_id}:companion:{}", slugify(companion.key)),
            "corpus_key": companion.key,
            "name": companion.name,
            "record_type": "creature",
            "size": companion.size,
            "speeds": companion.speeds.iter().map(|s| serde_json::json!({ "mode": s.mode, "feet": s.feet })).collect::<Vec<_>>(),
            "reach_feet": companion.reach_feet,
            "race_type": companion.race_type,
            "race_subtype": companion.race_subtype,
            "monster_class": companion.monster_class,
            "type_segments": companion.type_segments,
            "natural_attacks": companion.natural_attacks.iter().map(|a| serde_json::json!({ "name": a.name, "damage_dice": a.damage_dice })).collect::<Vec<_>>(),
            "stat_adjustments": companion.stat_adjustments.iter().map(|a| serde_json::json!({ "ability": a.ability, "amount": a.amount })).collect::<Vec<_>>(),
            "natural_armor": companion.natural_armor,
            "source_page": companion.source_page,
            "ability_keys": companion.ability_keys.iter().map(|k| format!("{book_id}:companion:{}", slugify(k))).collect::<Vec<_>>(),
            "external_ability_refs": companion.external_ability_refs,
        });
        pi_hits.extend(monster_record_pi_hits(companion.key, &data.to_string()));
        let source = CorpusSource::LstToken {
            path: races_file.relative_path.clone(),
            sha256: races_file.sha256.clone(),
            line,
            record_key: companion.key.to_string(),
        };
        let (wiring_class, wiring_class_signals) =
            wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
        let record = CorpusRecordV1 {
            population: Population::InScope,
            // The creature's AC, hit points and saves are PCGen-computed from
            // the `MONSTERCLASS:` token this ingest carries verbatim and does
            // not expand -- the same corpus fact `MonsterStatBlock` records.
            completeness: Completeness::ChassisOnly,
            ingested_at: ingested_at.clone(),
            data,
            source,
            license: Some(License::Ogl),
            pi_field: None,
            pi_marker: None,
            wiring_class,
            wiring_class_signals,
            description_source: None,
        };
        let key = format!("{book_id}:companion:{}", slugify(companion.key));
        current_companion_keys.insert(key);
        let path = out_root.join("companion").join(format!("{}.json", slugify(companion.key)));
        if !path.exists() {
            write_record(&path, &record);
        }
        creature_written += 1;
    }

    let mut ability_written = 0u32;
    for ability in table.companion_abilities {
        let abilities_file = abilities_files.get(ability.source_file).unwrap_or_else(|| {
            panic!(
                "{book_id}:{} cites {}, which is not in this book's \
                 CompanionBookSpec::abilities_lsts ({:?}) -- a citation this generator \
                 cannot verify is not a citation",
                ability.key, ability.source_file, spec.abilities_lsts
            )
        });
        let line = verified_citation_line(abilities_file, ability.source_line, ability.name, false);
        let data = serde_json::json!({
            "key": format!("{book_id}:companion:{}", slugify(ability.key)),
            "corpus_key": ability.key,
            "name": ability.name,
            "record_type": "ability",
            "facet": ability.facet.map(|f| f.corpus_token()),
            "delivery": ability.delivery.map(|d| d.corpus_token()),
            "type_segments": ability.type_segments,
            "description": ability.description,
            "description_variables": ability.description_variables,
            // Conditional `DESC:` variants, carried on disk for the same reason
            // they are carried in the table: a row that states its rules text
            // once per gate has no single description, and writing only the
            // ungated one (or, worse, the first) would put the wrong text in
            // the corpus cache for every character on the other side of the
            // gate. Empty for the ordinary single-`DESC:` row, which is why
            // adding it re-generated every previously ingested book's records
            // byte-identical (`decisions.md §61.1`).
            "description_variants": ability.description_variants.iter().map(|v| serde_json::json!({
                "text": v.text,
                "variables": v.variables,
                "conditions": v.conditions,
            })).collect::<Vec<_>>(),
            "stat_adjustments": ability.stat_adjustments.iter().map(|a| serde_json::json!({ "ability": a.ability, "amount": a.amount })).collect::<Vec<_>>(),
            "source_page": ability.source_page,
            "owners": ability.owners.iter().map(|o| format!("{book_id}:companion:{}", slugify(o))).collect::<Vec<_>>(),
        });
        pi_hits.extend(monster_record_pi_hits(ability.key, &data.to_string()));
        let source = CorpusSource::LstToken {
            path: abilities_file.relative_path.clone(),
            sha256: abilities_file.sha256.clone(),
            line,
            record_key: ability.key.to_string(),
        };
        let (wiring_class, wiring_class_signals) =
            wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: ingested_at.clone(),
            data,
            source,
            license: Some(License::Ogl),
            pi_field: None,
            pi_marker: None,
            wiring_class,
            wiring_class_signals,
            description_source: None,
        };
        let key = format!("{book_id}:companion:{}", slugify(ability.key));
        current_companion_keys.insert(key);
        let path = out_root.join("companion").join(format!("{}.json", slugify(ability.key)));
        if !path.exists() {
            write_record(&path, &record);
        }
        ability_written += 1;
    }
    if out_root.join("companion").exists() {
        // Single writer of `<book>/companion/` (verified: `gen_companion_book`
        // is the only generator that writes a `companion` kind directory --
        // SD-32 cross-generator sweep, 2026-08-23).
        codex::rules_core::cache_gen::ultimate_equipment::remove_stale_owned_files(
            &out_root.join("companion"),
            &current_companion_keys,
            &|_path, _line| true,
        );
    }

    if !pi_hits.is_empty() {
        eprintln!("PI screen FAILED for {book_id}: {pi_hits:?}");
        std::process::exit(1);
    }

    let records_processed = count_on_disk_records(&out_root);
    let records_redacted = count_on_disk_redactions(&out_root);
    let license_path = out_root.join("LICENSE.json");
    // Same preservation rule `gen_monster_book` states: Monster Codex and
    // Horror Adventures were both ingested by earlier lanes, whose LICENSE.json
    // carries a sharper OGL citation than this spec does. Clobbering it would
    // replace a real derivation with a weaker one.
    let prior: Option<serde_json::Value> = fs::read_to_string(&license_path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok());
    let prior_declaration = prior
        .as_ref()
        .and_then(|v| v.get("license_declaration"))
        .cloned();
    // The declaration was preserved from the first companion book that landed
    // in an already-ingested directory; the SCREENING NOTE was not, and that
    // half is the one that carries history. `data/corpus/beastiary`'s note
    // stated three earlier passes by name, date and record count (E2.0.9's 45,
    // `ingest_races`' 119, SD28-E16's 5) and this generator replaced all of it
    // with a sentence about its own 59 rows — leaving a file whose
    // `records_processed` said 228 and whose method note accounted for 59.
    //
    // It had already happened twice unnoticed, on `monster_codex` and
    // `horror_adventures`, in this lane's round 1 (`decisions.md §54.4`). The
    // note is append-only from here: every pass that put records on disk stays
    // named, and re-running this generator over a book it already wrote
    // re-composes onto the same prior text rather than stacking a copy.
    let prior_note = prior
        .as_ref()
        .and_then(|v| v.get("screening_method_note"))
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let this_pass = format!(
        "PASS -- {} (companion lane), {ingested_at}: every field of the {} records this run wrote ({creature_written} companion creatures + {ability_written} companion abilities) was screened against the bounded, documented term list in docs/governance/ogl-pi-blacklist.md, zero hits. A hit is a hard stop in this generator, not a warning. records_processed is {records_processed}: the real on-disk count for this book across every kind any lane has ingested, which for a book ingested by more than one lane is larger than this run's own output. This is NOT an exhaustive human legal review; it is a bounded substring scan and does not prove the absence of PI beyond what that scan can see.",
        spec.classified_by_cycle,
        creature_written + ability_written
    );
    // Idempotent and append-only; see `compose_screening_note`, which this lane
    // wrote inline first and round 8 lifted out when the monster generator
    // needed the identical logic.
    let screening_method_note = compose_screening_note(
        prior_note,
        &format!("PASS -- {} (companion lane)", spec.classified_by_cycle),
        this_pass,
    );
    let license_json = serde_json::json!({
        "book": book_id,
        "license_declaration": prior_declaration.unwrap_or_else(|| serde_json::json!({
            "open_game_content": spec.open_game_content,
            "product_identity_source": spec.product_identity_source,
            "product_identity_note": "Named deities, NPCs and unique places are Product Identity; companion and familiar stat blocks and their special-ability rules text are Open Game Content."
        })),
        "redaction_policy": {
            "marker": PI_REDACTION_MARKER,
            "schema_preserving": true,
            "pi_field_recorded": true,
            "blacklist_source": "docs/governance/ogl-pi-blacklist.md",
            "blacklist_version_reviewed": "2026-07-27"
        },
        "screening_method_note": screening_method_note,
        "redistribution_posture": "ogl-notice-attached",
        "classified_at": ingested_at,
        "classified_by_cycle": spec.classified_by_cycle,
        "records_processed": records_processed,
        "records_redacted": records_redacted,
        "operator_sign_off": {
            "signed_off": false,
            "signed_off_at": null,
            "note": "Set true only after an operator has reviewed this book's classification pass, per docs/governance/ogl-pi-blacklist.md's DRAFT header."
        }
    });
    fs::write(&license_path, serde_json::to_string_pretty(&license_json).unwrap() + "\n")
        .unwrap_or_else(|e| panic!("failed to write {license_path:?}: {e}"));
    println!(
        "{book_id} companion cache generated: {creature_written} creatures, {ability_written} \
         abilities; LICENSE.json records_processed={records_processed}"
    );
}

/// Where one companion book's `.lst` files live and what its OGL notice says.
/// Same shape and same discipline as [`MonsterBookSpec`]: locations and
/// citations only, never behaviour.
///
/// Both file fields are LISTS. Through round 3 every registered book had
/// exactly one file per shape, so a single-file field was never wrong and read
/// as though it were general; Bestiary 3 carries a `_companion` and a
/// `_familiar` file for each shape (`decisions.md §56.2`). A record names its
/// own file, so the citation check below can never verify a line against the
/// wrong one.
struct CompanionBookSpec {
    corpus_book: &'static str,
    book_relative: &'static str,
    races_lsts: &'static [&'static str],
    abilities_lsts: &'static [&'static str],
    open_game_content: &'static str,
    product_identity_source: &'static str,
    classified_by_cycle: &'static str,
}

const COMPANION_BOOK_SPECS: &[CompanionBookSpec] = &[
    CompanionBookSpec {
        corpus_book: "inner_sea_combat",
        book_relative: "pathfinder/paizo/campaign_setting/inner_sea_combat",
        races_lsts: &["isc_races_companion.lst"],
        abilities_lsts: &["isc_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own inner_sea_combat.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Inner Sea Combat, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F1-002",
    },
    CompanionBookSpec {
        corpus_book: "monster_codex",
        book_relative: "pathfinder/paizo/roleplaying_game/monster_codex",
        races_lsts: &["mc_races_companion.lst"],
        abilities_lsts: &["mc_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _monster_codex.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Monster Codex, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F1-002",
    },
    CompanionBookSpec {
        corpus_book: "inner_sea_intrigue",
        book_relative: "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
        races_lsts: &["isi_races_companion.lst"],
        abilities_lsts: &["isi_abilities_race_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own inner_sea_intrigue.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Inner Sea Intrigue, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F1-002",
    },
    CompanionBookSpec {
        corpus_book: "horror_adventures",
        book_relative: "pathfinder/paizo/roleplaying_game/horror_adventures",
        races_lsts: &["ha_races_companion.lst"],
        abilities_lsts: &["ha_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own horror_adventures.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Horror Adventures, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F1-002",
    },
    // SD-29 Epic 7 round 2 (`SD29-E7-F2-003`). Bestiary 5 and Bestiary 6 carry
    // ZERO monsters -- B5's pcc `CAMPAIGN` line says "Only Player Options
    // Implemented" -- so this generator, not the monster one, is the whole of
    // what those books contribute.
    //
    // **B5's `support/b5_races_companion_oa.lst` IS named here (SD-32 row
    // 19, cycle 2).** It used to be excluded on the premise that
    // `PRECAMPAIGN:1,Occult Adventures` (`_bestiary_5.pcc:69`) gated it
    // behind an uningested book (`decisions.md §47.2`). That premise is
    // false: `occult_adventures` is an ingested book
    // (`reach_gate.rs::CORPUS_BOOK_IDS`), and `decisions.md §27b`
    // ("EVERYTHING") separately overturned this exact reachability-driven
    // exclusion shape. `classify_companion_rows.py`'s
    // `UNINGESTED_CAMPAIGN_GATES` was emptied and the transcriber now
    // includes both of this file's rows in `COMPANIONS`, so this spec must
    // point at the file that owns them or `gen_companion_book` panics on
    // an unresolvable citation.
    CompanionBookSpec {
        corpus_book: "bestiary_5",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_5",
        races_lsts: &["b5_races_companion.lst", "b5_races_companion_oa.lst"],
        abilities_lsts: &["b5_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _bestiary_5.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 5, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-003",
    },
    CompanionBookSpec {
        corpus_book: "bestiary_6",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_6",
        races_lsts: &["b6_races_companion.lst"],
        abilities_lsts: &["b6_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _bestiary_6.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 6, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-003",
    },
    // The lane's first FAMILIAR book: `*_races_familiar.lst` rather than
    // `*_races_companion.lst`, which is why the spec names the files rather
    // than deriving them from a `<prefix>_races_companion.lst` convention.
    CompanionBookSpec {
        corpus_book: "bestiary_2",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_2",
        races_lsts: &["b2_races_familiar.lst"],
        abilities_lsts: &["b2_abilities_familiar_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bestiary_2.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 2, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-003",
    },
    // SD-29 Epic 7 round 3 (`SD29-E7-F2-004`). Bestiary 1. `corpus_book` is
    // `beastiary` because it names the `data/corpus/` directory this generator
    // writes into, and Bestiary 1's has been spelled that way since SD-22;
    // `book_relative` is `bestiary` because that is the PCGen source directory.
    // The two differ for exactly one book in this table and the difference is
    // load-bearing: writing `data/corpus/bestiary/` would split the book's
    // corpus in half, giving it a second LICENSE.json and a monster/equipment
    // half the new companion half could never be judged against.
    //
    // `SD31-CE-COMPANION-001` added the two `ce_*` files below. They physically
    // live under `core_essentials/`, and both declare `SOURCELONG:Bestiary` in
    // their own headers -- so `decisions.md §9` re-attribution reports their 95
    // rows as this book's, and `v06_work_inventory`'s `SOURCELONG_TO_BOOK` has
    // said so since 2026-08-16. The filename suffix `_cr` is NOT the signal and
    // must never be read as one: `ce_races_familiar_cr.lst` means "the Core
    // Rulebook classes' familiar list", and the stat blocks in it are the
    // Bestiary's. `load_corpus_file_rel_with_fallback` is what lets a spec name
    // a file that is not under its own `book_relative`.
    CompanionBookSpec {
        corpus_book: "beastiary",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary",
        races_lsts: &["b1_races_companion.lst", "ce_races_familiar_cr.lst"],
        abilities_lsts: &[
            "b1_abilities_companion.lst",
            "ce_abilities_familiar_race_cr.lst",
        ],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bestiary.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-004",
    },
    // SD-29 Epic 7 round 4 (`SD29-E7-F2-005`). Bestiary 3, and the FIRST book
    // with two files per shape -- which is what widened both fields from a
    // single name to a list (`decisions.md §56.2`). Its companion and familiar
    // files are separate corpus rows of the same kind, not two kinds: the
    // chassis has modelled `Familiar` as a `type_segment` since Bestiary 2.
    //
    // Registering it costs NO scope flip and needs no new `RuleSetId`: the
    // monster lane compiled `RuleSetId::B3` for this book's monsters in
    // `9595bd82`, the same free registration `bestiary` had in round 3.
    CompanionBookSpec {
        corpus_book: "bestiary_3",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_3",
        races_lsts: &["b3_races_companion.lst", "b3_races_familiar.lst"],
        abilities_lsts: &["b3_abilities_companion.lst", "b3_abilities_familiar.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bestiary_3.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 3, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-005",
    },
    // SD-29 Epic 7 round 5 (`SD29-E7-F2-006`). Bestiary 4, and the first book
    // with THREE ability-shape files: `b4_abilities_race_ce_companion.lst` sits
    // beside `b4_abilities_companion.lst`, and its own header comment says it
    // "should probably go into ce_abilities_race.lst". It is named here because
    // `_bestiary_4_for_players.pcc:80` loads it UNGATED alongside the other two,
    // which is what puts its 4 rows in this book's companion unit set.
    //
    // Registering the book costs no scope flip and no new `RuleSetId`: the
    // monster lane compiled `RuleSetId::B4` for its monsters in `52da4bc3`.
    CompanionBookSpec {
        corpus_book: "bestiary_4",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_4",
        races_lsts: &["b4_races_companion.lst"],
        abilities_lsts: &["b4_abilities_companion.lst", "b4_abilities_race_ce_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _bestiary_4.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 4, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-006",
    },
    // SD-29 Epic 7 round 6 (`SD29-E7-F2-007`). Ultimate Wilderness, the largest
    // companion block in the corpus — 169 creature rows, more than every
    // previously registered companion book combined.
    //
    // ONE abilities file, not two. `support/uw_abilities_companion_pu.lst` is
    // deliberately absent: its 17 rows are Pathfinder-Unchained option rows and
    // every one of them is an orphan under `classify_companion_rows`, so the
    // transcriber emits none and a record citing that file cannot exist. Naming
    // it here would assert a citation surface this book's table never uses.
    //
    // Registration cost no scope flip and no new `RuleSetId`: SD-28 Epic 26
    // compiled `RuleSetId::Uw` for this book's 136 feats.
    CompanionBookSpec {
        corpus_book: "ultimate_wilderness",
        book_relative: "pathfinder/paizo/roleplaying_game/ultimate_wilderness",
        races_lsts: &["uw_races_companion.lst"],
        abilities_lsts: &["uw_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _ultimate_wilderness.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Ultimate Wilderness, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-007",
    },
    // SD-29 Epic 7 round 8. Core Rulebook.
    //
    // Back to the standard formula, and CHECKED rather than assumed after
    // round 7 found it false for Core Essentials: `core_rulebook.pcc` carries a
    // live `ISOGL:YES` (line 19) and an uncommented `COPYRIGHT:` block (31
    // onward), and the directory ships a real `OGL.txt`. Both verified this
    // round by reading the file, not by pattern-matching the other eleven rows.
    //
    // As with every registered book, the generator will use NEITHER string:
    // `gen_companion_book` preserves a prior `license_declaration`
    // (`decisions.md §54.4`) and this book has one from cycle `E2.0.6`. They are
    // written correctly anyway, per `§63`'s note that a fallback which is only
    // correct while it stays unreached is how `§59.2`'s `mod_only` half sat
    // wrong for two rounds.
    CompanionBookSpec {
        corpus_book: "core_rulebook",
        book_relative: "pathfinder/paizo/roleplaying_game/core_rulebook",
        races_lsts: &["cr_races_companion.lst"],
        abilities_lsts: &["cr_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own core_rulebook.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game Core Rulebook, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-009",
    },
    // SD-29 Epic 7 round 9 (`SD29-E7-F2-010`) — the lane's final pass, four
    // books at once.
    //
    // Every one of the four was CHECKED rather than pattern-matched, per round
    // 7's finding that the standard formula was false for Core Essentials: each
    // `.pcc` carries a live `ISOGL:YES` and an uncommented `COPYRIGHT:` block,
    // and each directory ships a real `OGL.txt`. Verified by reading the files
    // (`grep -n 'ISOGL|^COPYRIGHT' <book>/*.pcc`, `ls <book>/OGL.txt`), not by
    // copying the row above.
    //
    // As with every registered book the generator will use NEITHER string where
    // a `license_declaration` already exists on disk (`decisions.md §54.4`);
    // they are written correctly anyway, per `§63`'s note that a fallback which
    // is only correct while it stays unreached is how `§59.2`'s `mod_only` half
    // sat wrong for two rounds.
    // The two `ce_*` files below both declare `SOURCELONG:Ultimate Magic` in
    // their own headers (`SD31-CE-COMPANION-001`); see `beastiary`'s row above
    // for why the header, never the filename, decides.
    CompanionBookSpec {
        corpus_book: "ultimate_magic",
        book_relative: "pathfinder/paizo/roleplaying_game/ultimate_magic",
        races_lsts: &["um_races_companion.lst", "ce_races_familiar_um.lst"],
        abilities_lsts: &[
            "um_abilities_companion.lst",
            "ce_abilities_familiar_race_um.lst",
        ],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own ultimate_magic.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game Ultimate Magic, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-010",
    },
    CompanionBookSpec {
        corpus_book: "advanced_race_guide",
        book_relative: "pathfinder/paizo/roleplaying_game/advanced_race_guide",
        races_lsts: &["arg_races_companion.lst"],
        abilities_lsts: &["arg_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own advanced_race_guide.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game Advanced Race Guide, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-010",
    },
    // `ce_races_familiar_apg.lst` declares `SOURCELONG:Advanced Player's Guide`
    // for the block its 8 transcribed rows sit in (`SD31-CE-COMPANION-001`).
    // Adding those 8 familiars also gave five previously-orphan
    // `apg_abilities_companion.lst` rows an owner, which is why this book's
    // shipped table moved 4 -> 17 and not 4 -> 12.
    CompanionBookSpec {
        corpus_book: "advanced_players_guide",
        book_relative: "pathfinder/paizo/roleplaying_game/advanced_players_guide",
        races_lsts: &["apg_races_companion.lst", "ce_races_familiar_apg.lst"],
        abilities_lsts: &["apg_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own advanced_players_guide.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game Advanced Player's Guide, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-010",
    },
    // The only one of the four under `campaign_setting/` rather than
    // `roleplaying_game/`, and the only registered book whose companion and
    // monster families both come from this generator.
    CompanionBookSpec {
        corpus_book: "book_of_the_damned_volume_1",
        book_relative: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
        races_lsts: &["botd1_races_companion.lst"],
        abilities_lsts: &["botd1_abilities_companion.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own book_of_the_damned_volume_1.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Book of the Damned, Volume 1 — Princes of Darkness, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-010",
    },
];

fn companion_book_spec(book: &str) -> Option<&'static CompanionBookSpec> {
    COMPANION_BOOK_SPECS.iter().find(|s| s.corpus_book == book)
}

fn companion_book_corpus_root(spec: &CompanionBookSpec) -> PathBuf {
    let override_var = format!("PCGEN_CORPUS_ROOT_{}", spec.corpus_book.to_uppercase());
    if let Ok(v) = std::env::var(&override_var) {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME")
        .expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data").join(spec.book_relative)
}

/// The directory ABOVE `pathfinder/...` for a companion book, i.e. the base the
/// `core_essentials` fallback in [`load_corpus_file_rel_with_fallback`] is
/// computed from. `None` when a per-book `PCGEN_CORPUS_ROOT_<BOOK>` override is
/// in effect, because an override points at a book directory directly and the
/// data root above it is not derivable -- the same rule and the same shape as
/// [`monster_book_corpus_data_root`], which the monster lane added in
/// `SD31-E6-F9-005` for exactly this reason.
fn companion_book_corpus_data_root(spec: &CompanionBookSpec) -> Option<PathBuf> {
    let override_var = format!("PCGEN_CORPUS_ROOT_{}", spec.corpus_book.to_uppercase());
    if std::env::var(&override_var).is_ok() {
        return None;
    }
    let home = std::env::var("HOME")
        .expect("HOME must be set to locate the default PCGen corpus checkout");
    Some(PathBuf::from(home).join("workspace/repos/pcgen/data"))
}

/// Where one monster book's two `.lst` files live and what its OGL notice
/// says. Every field is a *location* or a *citation*, never a behaviour: the
/// parsing, the citation check, the PI screen and the wiring-class computation
/// are identical across books because they are properties of PCGen's `.lst`
/// format, not of any one book. Adding a book here plus a row in
/// `monster_chassis::MONSTER_BOOKS` is the whole generator cost.
struct MonsterBookSpec {
    corpus_book: &'static str,
    book_relative: &'static str,
    /// Every races-`.lst` file this book's monster rows come from.
    ///
    /// A slice, not a string, because a book is not guaranteed one: Inner Sea
    /// World Guide splits its 14 monsters 7/7 across `iswg_races.lst` and
    /// `iswg_races_bestiary.lst`, and their line numbers COLLIDE
    /// (`iswg_races.lst:10` is the Aluum, `iswg_races_bestiary.lst:10` is the
    /// Firefoot Fennec). Each record names its own file in
    /// `MonsterStatBlock::source_file`; this list is what that name is checked
    /// against, so a transcription that invents a file fails here rather than
    /// citing a line in the wrong one.
    races_lsts: &'static [&'static str],
    /// Every abilities-`.lst` file this book's ability rows come from.
    ///
    /// A slice for `races_lsts`' reason, one book later: Inner Sea Gods splits
    /// its 161 ability rows 145/16 across `isg_abilities_races.lst` and
    /// `support/isg_abilities_races_b4.lst`. Each record names its own file in
    /// `MonsterAbilityRecord::source_file`; this list is what that name is
    /// checked against, so a transcription that invents a file fails here
    /// rather than citing a line in the wrong one.
    abilities_lsts: &'static [&'static str],
    open_game_content: &'static str,
    product_identity_source: &'static str,
    classified_by_cycle: &'static str,
}

const MONSTER_BOOK_SPECS: &[MonsterBookSpec] = &[
    MonsterBookSpec {
        corpus_book: "bonus_bestiary",
        book_relative: "pathfinder/paizo/roleplaying_game/bonus_bestiary",
        races_lsts: &["bb_races.lst"],
        abilities_lsts: &["bb_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bonus_bestiary.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bonus Bestiary, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F1-001",
    },
    MonsterBookSpec {
        corpus_book: "monster_codex",
        book_relative: "pathfinder/paizo/roleplaying_game/monster_codex",
        races_lsts: &["mc_races.lst"],
        abilities_lsts: &["mc_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _monster_codex.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Monster Codex, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-002",
    },
    MonsterBookSpec {
        corpus_book: "book_of_the_damned_volume_1",
        book_relative: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
        races_lsts: &["botd1_races.lst"],
        abilities_lsts: &["botd1_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own book_of_the_damned_volume_1.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Princes of Darkness, Book of the Damned Volume 1, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-003",
    },
    MonsterBookSpec {
        corpus_book: "book_of_the_damned_volume_2",
        book_relative: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
        races_lsts: &["botd2_races.lst"],
        abilities_lsts: &["botd2_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _book_of_the_damned_volume_2.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Lords of Chaos, Book of the Damned Volume 2, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-003",
    },
    MonsterBookSpec {
        corpus_book: "inner_sea_world_guide",
        book_relative: "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
        races_lsts: &["iswg_races.lst", "iswg_races_bestiary.lst"],
        abilities_lsts: &["iswg_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own inner_sea_world_guide.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Inner Sea World Guide, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-004",
    },
    MonsterBookSpec {
        corpus_book: "bestiary_2",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_2",
        races_lsts: &["b2_races.lst"],
        // `ce_abilities_race.lst` added `SD31-E6-F9-005`: 92 `decisions.md §9`
        // re-attributed ability rows physically live under `core_essentials`'s
        // own directory, reached via `load_corpus_file_rel_with_fallback`'s
        // core_essentials fallback -- the citation's own `path` records
        // `core_essentials/ce_abilities_race.lst`, never a `bestiary_2` path
        // the file does not exist under.
        abilities_lsts: &["b2_abilities_race.lst", "ce_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bestiary_2.pcc carries a live COPYRIGHT block plus a real OGL.txt. `ce_abilities_race.lst`'s own provenance is `core_essentials`'s -- see that book's own MonsterBookSpec entry -- verified identical (same PCGen `_core_essentials.pcc` OGL declaration governs every file it loads).",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 2, OGL §15 Product Identity section; the 92 `ce_abilities_race.lst`-origin rows are screened by this generator's own PI_BLACKLIST_TERMS scan on every emitted value exactly like every other row (no NAMEISPI:YES hits found, `SD31-E6-F9-005`)",
        classified_by_cycle: "SD29-E5-F2-005",
    },
    MonsterBookSpec {
        corpus_book: "bestiary_3",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_3",
        races_lsts: &["b3_races.lst"],
        // `vishkanya_abilities_race.lst` added `decisions.md §20`
        // no_record-to-zero wave 2 follow-on: one owner-less
        // `monster_ability` row (`Vishkanya ~ Toxic ~ Vishkanya Venom`,
        // `docs/work-inventory.json`'s own `book: "bestiary_3"` re-
        // attribution) physically lives under
        // `core_essentials/races/vishkanya/vishkanya_abilities_race.lst`
        // (that race's own `_race.pcc` loads it, not any `bestiary_3.pcc`
        // line -- confirmed by `grep -rl vishkanya_abilities_race
        // **/*.pcc`), reached via the SAME core_essentials fallback
        // `ce_abilities_race.lst`/`b4_abilities_races_ce.lst` already use
        // (`load_corpus_file_rel_with_fallback`'s recursive walk finds it
        // under the subdirectory once the basename is registered here).
        // Never registered until this cycle because no prior transcription
        // reached this row -- `transcribe_monster_tables.py::
        // resolve_book_file` already resolved it (Python's fallback walk is
        // also recursive), so only this generator's citation allow-list was
        // stale.
        // `ce_abilities_race.lst` added the same cycle, same cause: another
        // owner-less row (`Asura ~ Save Bonus` and siblings, the `Traits
        // Output ~ <Kind>` rows this module's own header names) physically
        // lives under `core_essentials`'s own directory, the identical
        // shape `bestiary`/`bestiary_2`/`bestiary_4`'s entries already
        // register for the same file.
        abilities_lsts: &["b3_abilities_race.lst", "vishkanya_abilities_race.lst", "ce_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bestiary_3.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt. `vishkanya_abilities_race.lst`'s own provenance is `core_essentials`'s -- verified identical (same PCGen `_race.pcc` OGL declaration governs it).",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 3, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-006",
    },
    // SD-29 Epic 5 extend, round 6. The `.pcc` name carries a LEADING
    // UNDERSCORE (`_bestiary_4.pcc`) where B1/B2/B3's do not -- the naming split
    // `loop-instruction.md`'s corpus shape notes warn about. Provenance verified
    // against the file rather than copied from the row above: `ISOGL:YES` at
    // line 23, 17 `COPYRIGHT` lines, and a real 9,977-byte `OGL.txt`.
    MonsterBookSpec {
        corpus_book: "bestiary_4",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_4",
        races_lsts: &["b4_races.lst"],
        // `b4_abilities_races_ce.lst` added SD-32 card 11 (T9 onboarding,
        // `decisions.md §19` sign-off / `§17` generic-pass discipline): both
        // `_bestiary_4.pcc:59` and `_bestiary_4_for_players.pcc:59` load it
        // UNGATED alongside `b4_abilities_race.lst` (`ABILITY:` line, no
        // `PRECAMPAIGN`/other condition), the same "second abilities file at
        // the book's own root" shape `inner_sea_gods`'s `support/` pair and
        // `beastiary`'s `ce_abilities_race.lst` entry already cover --
        // `transcribe_monster_tables.py`'s re-run against the fresh inventory
        // found 42 newly-reachable rows citing this file, and this generator
        // refuses (rather than silently drops) any citation to a file not
        // named here.
        // `ce_abilities_race.lst`/`wyrwood_abilities_race.lst` added
        // `decisions.md §20` no_record-to-zero wave 2 follow-on: owner-less
        // rows physically living under `core_essentials`'s own directory
        // (the `Traits Output ~ <Kind>` rows and `Wyrwood ~ Construct
        // Traits`), the identical shape `bestiary_3`'s matching entry adds
        // this same cycle, reached via the same core_essentials fallback.
        abilities_lsts: &[
            "b4_abilities_race.lst",
            "b4_abilities_races_ce.lst",
            "ce_abilities_race.lst",
            "wyrwood_abilities_race.lst",
        ],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _bestiary_4.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt. `b4_abilities_races_ce.lst` is loaded by the SAME `_bestiary_4.pcc`/`_bestiary_4_for_players.pcc` (line 59, `ABILITY:` token, no gate) so the same OGL declaration governs it.",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 4, OGL §15 Product Identity section; 14 monster rows additionally declare NAMEISPI:YES per-record and are dropped by the screen",
        classified_by_cycle: "SD29-E5-F2-007",
    },
    // SD-29 Epic 5 extend, round 7. The first `campaign_setting/` bestiary in
    // this registry. Provenance verified against the file rather than copied
    // from the row above.
    MonsterBookSpec {
        corpus_book: "inner_sea_bestiary",
        book_relative: "pathfinder/paizo/campaign_setting/inner_sea_bestiary",
        races_lsts: &["isb_races.lst"],
        abilities_lsts: &["isb_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own inner_sea_bestiary.pcc declares ISOGL:YES at line 23, carries 4 COPYRIGHT lines and a real 6,739-byte OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Inner Sea Bestiary, OGL §15 Product Identity section; 7 ability rows carry a blacklisted proper name in their namespace, and the 2 monster rows that NAME them are dropped with them",
        classified_by_cycle: "SD29-E5-F2-008",
    },
    // SD-29 Epic 5 extend, round 8. Bestiary 1's complement -- `decisions.md
    // §58.3`. `corpus_book` is the `data/corpus/` spelling `beastiary`, which is
    // where SD-22's 46 monster records already live; `book_relative` is the
    // SOURCE spelling `bestiary`, which is what PCGen calls the directory. The
    // two differ for this book alone (`decisions.md §54.3` lists all four
    // spellings), and the generator writing into an already-populated directory
    // is exactly why this round made its record sweep and its LICENSE note
    // preserve what other lanes wrote.
    MonsterBookSpec {
        corpus_book: "beastiary",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary",
        races_lsts: &["b1_races.lst"],
        // `ce_abilities_race.lst` added `SD31-E6-F9-005`: 76 `decisions.md §9`
        // re-attributed ability rows physically live under `core_essentials`'s
        // own directory, reached via `load_corpus_file_rel_with_fallback`'s
        // core_essentials fallback -- see `bestiary_2`'s matching entry for
        // the full mechanism.
        abilities_lsts: &["b1_abilities_race.lst", "ce_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bestiary.pcc carries a live COPYRIGHT block plus a real OGL.txt. `ce_abilities_race.lst`'s own provenance is `core_essentials`'s -- see that book's own MonsterBookSpec entry -- verified identical (same PCGen `_core_essentials.pcc` OGL declaration governs every file it loads).",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary, OGL §15 Product Identity section; zero rows of either .lst declare NAMEISPI:YES, which is what the blacklist's per-record predicate predicts for a roleplaying_game/ bestiary. The 76 `ce_abilities_race.lst`-origin rows are screened by this generator's own PI_BLACKLIST_TERMS scan on every emitted value exactly like every other row (no NAMEISPI:YES hits found, `SD31-E6-F9-005`)",
        classified_by_cycle: "SD29-E5-F2-009",
    },
    // SD-29 Epic 5 extend, round 9. The first book in this registry that needs
    // BOTH list fields to be plural, and the first whose files are not all at
    // the book root -- `support/isg_races_b4.lst` and
    // `support/isg_abilities_races_b4.lst` are loaded by
    // `_inner_sea_gods.pcc:68`/`:70` under `PRECAMPAIGN:1,INCLUDES=Bestiary 4`,
    // a gate this repo satisfies since round 6 registered `bestiary_4`. Both
    // are named here by BARE BASENAME because that is what the inventory (and
    // therefore every record's `source_file`) carries; `resolve_book_file`
    // turns each into its real sub-path and the record's `path` citation gets
    // the resolved form.
    //
    // Provenance verified against the file rather than copied from the row
    // above: `_inner_sea_gods.pcc:17` declares `ISOGL:YES`, the pcc carries 18
    // COPYRIGHT lines, and a real 9,547-byte OGL.txt sits beside it.
    MonsterBookSpec {
        corpus_book: "inner_sea_gods",
        book_relative: "pathfinder/paizo/campaign_setting/inner_sea_gods",
        races_lsts: &["isg_races.lst", "isg_races_b4.lst"],
        abilities_lsts: &["isg_abilities_races.lst", "isg_abilities_races_b4.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _inner_sea_gods.pcc declares ISOGL:YES at line 17, carries 18 COPYRIGHT lines and a real 9,547-byte OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Inner Sea Gods, OGL §15 Product Identity section; zero rows of any of the four .lst files declare NAMEISPI:YES, and the 5 ability rows the screen drops are dropped for a blacklisted deity name in an emitted value",
        classified_by_cycle: "SD29-E5-F2-010",
    },
    // SD-29 Epic 5 extend, round 10. Ultimate Psionics -- the first NON-PAIZO
    // book in this table, and the first whose `RuleSetId` was already compiled
    // for other kinds (`RuleSetId::Upsi`, SD-28 E29). Both `.lst` files sit at
    // the book root, so `resolve_book_file` resolves each in one hop; round 9's
    // widening is not load-bearing here and this row says so rather than
    // letting a later reader infer it.
    //
    // Provenance verified against the file rather than copied from the row
    // above: `ultimate_psionics.pcc:21` declares `ISOGL:YES`, the pcc carries
    // 29 COPYRIGHT lines (the first being the OGL itself and the last the
    // Dreamscarred Press title), and a real 10,418-byte OGL.txt sits beside it.
    // `grep -c NAMEISPI:YES up_races.lst up_abilities_race.lst` -> 0, 0.
    MonsterBookSpec {
        corpus_book: "ultimate_psionics",
        book_relative: "pathfinder/dreamscarred_press/ultimate_psionics",
        races_lsts: &["up_races.lst"],
        abilities_lsts: &["up_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own ultimate_psionics.pcc declares ISOGL:YES at line 21, carries 29 COPYRIGHT lines and a real 10,418-byte OGL.txt",
        product_identity_source: "Dreamscarred Press Ultimate Psionics, OGL §15 Product Identity section; zero rows of either .lst declare NAMEISPI:YES, and the classifier's Product Identity screen returns 0 -- which is what the blacklist's per-record predicate predicts for a book whose creatures are generic psionic species rather than named personae",
        classified_by_cycle: "SD29-E5-F2-011",
    },
    // SD-29 Epic 5 extend, FINAL round. Horror Adventures -- the last book in
    // this lane with any workable unit, at 3 monster rows and 6 owned ability
    // rows.
    //
    // Both `.lst` files sit at the book ROOT and both load UNCONDITIONALLY:
    // `grep -n 'ha_races.lst\|ha_abilities_race.lst' _horror_adventures.pcc`
    // -> `63:ABILITY:ha_abilities_race.lst`, `77:RACE:ha_races.lst`, neither
    // carrying a `PRECAMPAIGN` gate. That check is read from the PCC LOAD LINE,
    // per round 9's finding, and at the time this was written it disqualified
    // this lane's other nominally-workable book: `occult_adventures` loads
    // its one monster row under a NEGATED gate
    // (`!PRECAMPAIGN:1,INCLUDES=Bestiary 3`) this repo's campaign set fails.
    // `decisions.md §27b` overturns that disposition: the gate is a
    // reachability finding, not an ingest exemption, so `occult_adventures`
    // is registered below too (own `MonsterBookSpec` entry) -- reachability
    // for its 5 units is reported honestly as 0.
    //
    // `corpus_book` is `horror_adventures`, matching the `data/corpus/` tree
    // this book's `race_trait` (Epic 6 round 3) and `companion` (Epic 7)
    // families already write into -- a third family under one directory, not a
    // fourth directory.
    //
    // Provenance verified against the file rather than copied from the
    // `CompanionBookSpec` row above: `_horror_adventures.pcc:26` declares
    // `ISOGL:YES`, the pcc carries 19 COPYRIGHT lines, and a real 9,924-byte
    // OGL.txt sits beside it. `grep -c NAMEISPI:YES ha_races.lst
    // ha_abilities_race.lst` -> 0, 0.
    MonsterBookSpec {
        corpus_book: "horror_adventures",
        book_relative: "pathfinder/paizo/roleplaying_game/horror_adventures",
        races_lsts: &["ha_races.lst"],
        abilities_lsts: &["ha_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _horror_adventures.pcc declares ISOGL:YES at line 26, carries 19 COPYRIGHT lines and a real 9,924-byte OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Horror Adventures, OGL §15 Product Identity section; zero rows of either .lst declare NAMEISPI:YES, and the classifier's Product Identity screen returns 0 -- which is what the blacklist's per-record predicate predicts for three generic hive-insect species rather than named personae",
        classified_by_cycle: "SD29-E5-F2-012",
    },
    // `decisions.md §20` no_record-to-zero, round 3. Five zero-monster books
    // `decisions.md §17a`'s re-derive found unregistered
    // (`scripts/classify_monster_ability_rows.py`'s "ZERO-monster books"
    // line). Every one has `races_lsts: &[]` -- no monster row of the book
    // exists to own any ability, so every transcribed row ships owner-less by
    // construction, the same shape the `bestiary_2`/`bestiary_3`/`bestiary_4`
    // core_essentials-fallback entries already ship a subset of.
    //
    // Provenance verified against each file directly (not copied from the row
    // above): `grep -c NAMEISPI:YES <abilities_lst>` -> 0 for all five;
    // `ultimate_wilderness` alone carries 1 `NAMEISPI:YES`/`DESCISPI:YES` hit
    // elsewhere in its 296-line abilities file (screened per-record by this
    // generator's own PI_BLACKLIST_TERMS scan, not assumed clear).
    MonsterBookSpec {
        corpus_book: "ultimate_wilderness",
        book_relative: "pathfinder/paizo/roleplaying_game/ultimate_wilderness",
        races_lsts: &[],
        abilities_lsts: &["uw_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _ultimate_wilderness.pcc declares ISOGL:YES at line 27, carries 16 COPYRIGHT lines and a real 9,214-byte OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Ultimate Wilderness, OGL §15 Product Identity section; the abilities file's own screen runs per-record on every emitted value",
        classified_by_cycle: "SD32-T9-NORECORD-R3",
    },
    MonsterBookSpec {
        corpus_book: "ultimate_intrigue",
        book_relative: "pathfinder/paizo/roleplaying_game/ultimate_intrigue",
        races_lsts: &[],
        abilities_lsts: &["ui_abilities_race_pu.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _ultimate_intrigue.pcc declares ISOGL:YES at line 18, carries 18 COPYRIGHT lines and a real 9,728-byte OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Ultimate Intrigue, OGL §15 Product Identity section; zero rows of the abilities file declare NAMEISPI:YES",
        classified_by_cycle: "SD32-T9-NORECORD-R3",
    },
    MonsterBookSpec {
        corpus_book: "ultimate_magic",
        book_relative: "pathfinder/paizo/roleplaying_game/ultimate_magic",
        races_lsts: &[],
        abilities_lsts: &["um_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _ultimate_magic.pcc declares ISOGL:YES at line 19, carries 29 COPYRIGHT lines and a real 10,158-byte OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Ultimate Magic, OGL §15 Product Identity section; zero rows of the abilities file declare NAMEISPI:YES",
        classified_by_cycle: "SD32-T9-NORECORD-R3",
    },
    // `ce_abilities_race.lst` -- one owner-less row (`Universal Monster Rule ~
    // Ferocity`) physically lives under `core_essentials`'s own directory,
    // the identical shape `bestiary`/`bestiary_2`/`bestiary_3`/`bestiary_4`'s
    // matching entries already register for this file, reached via the same
    // `load_corpus_file_rel_with_fallback` core_essentials fallback. Found by
    // this generator's own citation refusal, not assumed.
    MonsterBookSpec {
        corpus_book: "bestiary_6",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_6",
        races_lsts: &[],
        abilities_lsts: &["b6_abilities_race.lst", "ce_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _bestiary_6.pcc declares ISOGL:YES at line 23, carries 23 COPYRIGHT lines and a real 10,687-byte OGL.txt. `ce_abilities_race.lst`'s own provenance is `core_essentials`'s -- see that book's own MonsterBookSpec entry -- verified identical (same PCGen `_core_essentials.pcc` OGL declaration governs every file it loads).",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 6, OGL §15 Product Identity section; zero rows of the abilities file declare NAMEISPI:YES",
        classified_by_cycle: "SD32-T9-NORECORD-R3",
    },
    // `b5_abilities_race_oa.lst` -- `_bestiary_5.pcc:66` loads
    // `support/b5_abilities_race_oa.lst|PRECAMPAIGN:1,Occult Adventures`: this
    // repo has NOT registered `occult_adventures` as an included book, so
    // PCGen's own chargen would not load this file for a `bestiary_5`-only
    // campaign. Registered here anyway, deliberately: `docs/work-inventory.json`
    // independently attributes these 3 rows to `book: "bestiary_5"` (the
    // census walker reads a book's own `.pcc` ABILITY lines regardless of
    // PRECAMPAIGN, which governs optional chargen inclusion, not which book
    // physically owns the file) -- Gate 0's census already counted them as
    // this book's content, and `decisions.md §20` requires their shape be
    // measured, so this generator ingests what the census already scoped
    // rather than re-litigating it. Found by this generator's own citation
    // refusal, not assumed.
    MonsterBookSpec {
        corpus_book: "bestiary_5",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_5",
        races_lsts: &[],
        abilities_lsts: &["b5_abilities_race.lst", "ce_abilities_race.lst", "b5_abilities_race_oa.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _bestiary_5.pcc declares ISOGL:YES at line 25, carries 8 COPYRIGHT lines and a real 7,806-byte OGL.txt. `ce_abilities_race.lst`'s own provenance is `core_essentials`'s (see that book's own entry); `b5_abilities_race_oa.lst` is loaded by the same `_bestiary_5.pcc` (ABILITY line, no PRECAMPAIGN gate on that token).",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 5, OGL §15 Product Identity section; zero rows of the abilities files declare NAMEISPI:YES. One owned row (`Traits Output ~ Sahkil`, `b5_abilities_race.lst:96`) is a multi-DESC: shape `parse_desc` refuses rather than mistranscribes -- real per-record work, not shipped by this cycle.",
        classified_by_cycle: "SD32-T9-NORECORD-R3",
    },
    // `decisions.md §20` no_record-to-zero, round 4. Reached via
    // `gen_pathfinder_unchained`'s own call to `gen_monster_book` (this
    // book's CLI dispatch special-cases its own generator function in
    // `main()`, above the generic `monster_book_spec` arm, so this spec is
    // never reached through that arm directly -- only through the explicit
    // call the generator function makes). `pu_abilities_race.lst` loads
    // UNGATED at the book's own `.pcc` root (line 43, no `PRECAMPAIGN`).
    MonsterBookSpec {
        corpus_book: "pathfinder_unchained",
        book_relative: "pathfinder/paizo/roleplaying_game/pathfinder_unchained",
        races_lsts: &[],
        abilities_lsts: &["pu_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _pathfinder_unchained.pcc declares ISOGL:YES at line 19 and carries 7 COPYRIGHT lines and a real 7,429-byte OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Pathfinder Unchained, OGL §15 Product Identity section; zero rows of the abilities file declare NAMEISPI:YES",
        classified_by_cycle: "SD32-T9-NORECORD-R4",
    },
    // `decisions.md §20` no_record-to-zero, round 4. Reached via
    // `gen_advanced_race_guide`'s own call to `gen_monster_book`, the same
    // shape as `pathfinder_unchained` above. `arg_abilities_race.lst` loads
    // UNGATED at the book's own `.pcc` root (line 57, no `PRECAMPAIGN`).
    MonsterBookSpec {
        corpus_book: "advanced_race_guide",
        book_relative: "pathfinder/paizo/roleplaying_game/advanced_race_guide",
        races_lsts: &[],
        abilities_lsts: &["arg_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own advanced_race_guide.pcc declares ISOGL:YES at line 20 and carries 11 COPYRIGHT lines and a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Advanced Race Guide, OGL §15 Product Identity section; zero rows of the abilities file declare NAMEISPI:YES",
        classified_by_cycle: "SD32-T9-NORECORD-R4",
    },
    // `decisions.md §20` no_record-to-zero, round 5. This book carries no
    // hand-rolled `gen_book_cache.rs` function (its `spell` family is
    // reached through `src/bin/ingest_spells.rs`'s config-driven path
    // instead), so it is reached entirely through `main`'s generic
    // `monster_book_spec` fallback arm below -- no new generator code, only
    // this registry row. `ma_abilities_race.lst` loads UNGATED at the
    // book's own `.pcc` root (line 40, no `PRECAMPAIGN`).
    MonsterBookSpec {
        corpus_book: "mythic_adventures",
        book_relative: "pathfinder/paizo/roleplaying_game/mythic_adventures",
        races_lsts: &[],
        abilities_lsts: &["ma_abilities_race.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _mythic_adventures.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Mythic Adventures, OGL §15 Product Identity section; zero rows of the abilities file declare NAMEISPI:YES or DESCISPI:YES",
        classified_by_cycle: "SD32-T9-NORECORD-R5",
    },
    // `decisions.md §27b` -- EVERYTHING. Overturns the disposition this
    // file's own comment above (near `horror_adventures`) recorded: "occult_
    // adventures loads its one monster row under a NEGATED gate this repo's
    // campaign set fails" was a REACHABILITY finding, not an ingest
    // exemption -- the book's `.lst` rows exist and are ingested here like
    // every other book; reachability is reported separately and honestly
    // (0 -- all 5 `monster_ability` rows ship owner-less, the identical
    // shape `mythic_adventures` above already ships). `races_lsts: []`
    // deliberately: `oa_races.lst`'s 4 rows are `docs/work-inventory.json`
    // kind `race`, a different kind and a sibling lane's territory, not
    // this cycle's `monster_ability` scope; the Python transcriber's own
    // unit-set (`docs/work-inventory.json`, not a raw `races_lsts` glob)
    // auto-includes the one `kind: monster` row (`oa_races_b3.lst`'s Kami
    // (Shikigami)) regardless, because it is the sole `monster`-kind unit
    // this book has -- see `occult_adventures/monster_data.rs`'s own header.
    MonsterBookSpec {
        corpus_book: "occult_adventures",
        book_relative: "pathfinder/paizo/roleplaying_game/occult_adventures",
        races_lsts: &["oa_races_b3.lst"],
        abilities_lsts: &["oa_abilities_race.lst", "oa_abilities_race_b3.lst"],
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _occult_adventures.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Occult Adventures, OGL §15 Product Identity section; zero rows of either abilities file declare NAMEISPI:YES or DESCISPI:YES",
        classified_by_cycle: "SD32-DECISION-27B-CARVEOUT-CLOSURE",
    },
];

fn monster_book_spec(book: &str) -> Option<&'static MonsterBookSpec> {
    MONSTER_BOOK_SPECS.iter().find(|s| s.corpus_book == book)
}

fn monster_book_corpus_root(spec: &MonsterBookSpec) -> PathBuf {
    let override_var = format!(
        "PCGEN_CORPUS_ROOT_{}",
        spec.corpus_book.to_uppercase()
    );
    if let Ok(v) = std::env::var(&override_var) {
        return PathBuf::from(v);
    }
    // `decisions.md §20` (no_record-to-zero wave 2): every OTHER tool in this
    // bundle (`scripts/census_independent.py`, `scripts/card15_reconcile.py`,
    // `scripts/generic_pass_state_rederive.py`, ...) reads the repo-local
    // pinned oracle via the plain `PCGEN_CORPUS_ROOT` env var
    // (`artifacts/corpus/README.md`, `scripts/fetch-pcgen-oracle.sh`). This
    // binary alone fell back straight to the deprecated
    // `~/workspace/repos/pcgen/data` checkout the program's own standing
    // rule says never to reference, with no way to point it at the pinned
    // oracle short of a per-book override that also (undesirably) disables
    // the `core_essentials` cross-book fallback below. Checked SECOND, after
    // the per-book override, so an existing `PCGEN_CORPUS_ROOT_<BOOK>` still
    // wins for a synthetic/test-only book directory exactly as before.
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v).join(spec.book_relative);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data").join(spec.book_relative)
}

/// The PCGen `data/` root ABOVE `pathfinder/...` -- i.e. what
/// `monster_book_corpus_root` joins `spec.book_relative` onto -- for use as
/// [`load_corpus_file_rel_with_fallback`]'s core_essentials fallback base.
/// `None` when a per-book `PCGEN_CORPUS_ROOT_<BOOK>` override is set: that
/// override names a synthetic/test book directory directly, with no
/// corpus-wide sibling tree above it for a fallback to reach, matching
/// [`load_corpus_file_rel_with_fallback`]'s own "no fallback offered" contract
/// for that case.
fn monster_book_corpus_data_root(spec: &MonsterBookSpec) -> Option<PathBuf> {
    let override_var = format!("PCGEN_CORPUS_ROOT_{}", spec.corpus_book.to_uppercase());
    if std::env::var(&override_var).is_ok() {
        return None;
    }
    // See `monster_book_corpus_root`'s matching comment: the plain
    // `PCGEN_CORPUS_ROOT` env var (the repo-local pinned oracle's `data/`
    // root) is checked before the deprecated `~/workspace/repos/pcgen`
    // fallback.
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return Some(PathBuf::from(v));
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    Some(PathBuf::from(home).join("workspace/repos/pcgen/data"))
}

/// The table records the line it was transcribed from; this re-reads that line
/// out of the live file and requires its first column to still be the record's
/// display name before citing it.
///
/// A citation nobody checked is the failure mode `v06_corpus_trap_report
/// --audit` exists to catch after the fact; checking it here means the cache is
/// never written with a stale line number in the first place.
/// `codex_generated_name`: `true` when `display_name` is a Codex-minted
/// neutral identity (`decisions.md §24`), not the printed name -- the row's
/// own first column is still the ORIGINAL (possibly PI) name in that case,
/// so the exact-match check below would fail by design and must be skipped.
/// The line still has to EXIST (the `.get(idx)` bounds check below still
/// runs unconditionally) -- a citation this generator cannot verify at all
/// is not a citation, renamed or not.
fn verified_citation_line(
    file: &CorpusFile,
    recorded: u32,
    display_name: &str,
    codex_generated_name: bool,
) -> u32 {
    let idx = recorded as usize - 1;
    let line = file
        .lines
        .get(idx)
        .unwrap_or_else(|| panic!("{} has no line {recorded}", file.relative_path));
    let first_col = line.split('\t').next().unwrap_or_default().trim();
    // A `.COPY=` overlay row's own first column is a compound directive
    // (`CATEGORY=Special Ability|Rake.COPY=Rake`), not the record's display
    // name -- the same "row's first column is not the emitted name" shape
    // `codex_generated_name` exempts, but structural rather than a rename:
    // provable from the line's own bytes, never guessed (`decisions.md
    // §27`'s provisional-facet-default cycle, round 8 -- `Aurumvorax ~
    // Rake`/`Carnivorous Blob ~ Split`). A `.COPY=` MONSTER row never
    // reaches this function at all (dropped before emission, see the
    // `.COPY=` screen above), so this only ever exempts an ABILITY row.
    let is_copy_overlay_row = first_col.contains(".COPY=");
    if !codex_generated_name && !is_copy_overlay_row {
        assert_eq!(
            first_col, display_name,
            "{}:{recorded} names {first_col:?}, not {display_name:?} -- the table's recorded \
             line is stale and must be re-transcribed, not papered over here",
            file.relative_path
        );
    }
    recorded
}

/// The same bounded PI screen the two generators above run, applied to a whole
/// serialized record rather than a single `description` field -- a monster row
/// can carry a setting proper noun in its name, its subtype or its rules text,
/// so screening one field would leave the others unscreened.
fn monster_record_pi_hits(record_key: &str, serialized: &str) -> Vec<String> {
    PI_BLACKLIST_TERMS
        .iter()
        .filter(|term| serialized.contains(*term))
        .map(|term| format!("{record_key}: {term}"))
        .collect()
}

fn main() {
    let book = std::env::args().nth(1).unwrap_or_else(|| "pathfinder_unchained".to_string());
    match book.as_str() {
        "pathfinder_unchained" => gen_pathfinder_unchained(),
        "advanced_race_guide" => gen_advanced_race_guide(),
        // `companion:<book>` rather than a bare book name: three of the four
        // companion books are ALSO monster or race-trait books, so a bare name
        // would be ambiguous and would silently run whichever generator the
        // match arm reached first.
        other if other.starts_with("companion:") => {
            let book = &other["companion:".len()..];
            match companion_book_spec(book) {
                Some(spec) => gen_companion_book(spec),
                None => panic!(
                    "gen_book_cache: no companion generator wired for book {book:?} yet -- add a \
                     CompanionBookSpec and a companion_chassis::COMPANION_BOOKS row"
                ),
            }
        }
        other => match monster_book_spec(other) {
            Some(spec) => gen_monster_book(spec),
            None => panic!(
                "gen_book_cache: no generator wired for book {other:?} yet (pathfinder_unchained, \
                 advanced_race_guide, and every book in MONSTER_BOOK_SPECS today -- a future cycle \
                 adds its own book, per this file's own module doc comment)"
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Code review finding SD30-E8-F3-002/003: `wiring_class_file_arg` must
    // preserve every path segment past the book's own `/{book_id}/` marker,
    // not just the bare basename -- `CorpusLines::line`'s single-level
    // `dir.join(file)` join otherwise silently resolves a nested citation
    // to a nonexistent top-level path.

    // `decisions.md §27`'s provisional-facet-default cycle (round 8):
    // `verified_citation_line` used to assert a cited line's own first
    // column equals the emitted `display_name` unconditionally (except for
    // `codex_generated_name`) -- a `.COPY=` overlay ability row's first
    // column is a compound directive (`CATEGORY=Special Ability|Rake.COPY=
    // Rake`), never the emitted name, so this panicked on every real
    // `.COPY=` ability row before the fix.

    #[test]
    fn verified_citation_line_exempts_a_copy_overlay_rows_compound_first_column() {
        let file = CorpusFile {
            relative_path: "test.lst".to_string(),
            sha256: String::new(),
            lines: vec!["CATEGORY=Special Ability|Rake.COPY=Rake\tKEY:Aurumvorax ~ Rake".to_string()],
        };
        // Would panic under the pre-fix code: first column
        // (`CATEGORY=Special Ability|Rake.COPY=Rake`) != display_name (`Rake`).
        assert_eq!(verified_citation_line(&file, 1, "Rake", false), 1);
    }

    #[test]
    fn verified_citation_line_still_catches_a_genuinely_stale_citation() {
        // Mutation-style proof this is not a blanket bypass: a row that is
        // NOT a `.COPY=` overlay and whose first column genuinely diverges
        // from the emitted name must still panic.
        let file = CorpusFile {
            relative_path: "test.lst".to_string(),
            sha256: String::new(),
            lines: vec!["Stale Name\tKEY:Something ~ Else".to_string()],
        };
        let result = std::panic::catch_unwind(|| {
            verified_citation_line(&file, 1, "Different Name", false)
        });
        assert!(result.is_err(), "a genuinely stale citation must still panic");
    }

    #[test]
    fn wiring_class_file_arg_keeps_the_subdirectory_a_bare_basename_would_drop() {
        let path = "pathfinder/paizo/campaign_setting/inner_sea_gods/support/isg_races_b4.lst";
        assert_eq!(wiring_class_file_arg("inner_sea_gods", path), "support/isg_races_b4.lst");
    }

    #[test]
    fn wiring_class_file_arg_is_the_bare_basename_when_the_citation_sits_at_the_book_top_level() {
        let path = "pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst";
        assert_eq!(wiring_class_file_arg("pathfinder_unchained", path), "pu_abilities_class.lst");
    }

    #[test]
    fn wiring_class_file_arg_falls_back_to_the_bare_basename_when_the_book_marker_is_absent() {
        // No `/{book_id}/` segment anywhere in the path -- a shape this
        // lane's real corpus paths never produce (every one carries a
        // subtree prefix before the book name), kept as a defined fallback
        // rather than a panic so a future caller with an unexpected path
        // shape degrades to the pre-fix behavior instead of crashing.
        assert_eq!(wiring_class_file_arg("inner_sea_gods", "isg_races_b4.lst"), "isg_races_b4.lst");
    }
}
