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

fn wiring_class_for_source(
    index: &WiringClassIndex,
    lines: &mut codex::rules_core::wiring_class::CorpusLines,
    source: &CorpusSource,
) -> (String, Vec<String>) {
    match wiring_citation(source) {
        Some((path, line, record_key)) => {
            let basename = path.rsplit('/').next().unwrap_or(path);
            index.wiring_class_for(lines, basename, line, record_key, record_key)
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
#[path = "../rules_core/rules_tables/archetype_swap.rs"]
mod archetype_swap;
#[path = "../rules_core/rules_tables/advanced_race_guide/mod.rs"]
mod advanced_race_guide;

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
fn load_corpus_file_rel(root: &Path, book_relative: &str, file_name: &str) -> CorpusFile {
    let full = root.join(file_name);
    let bytes = fs::read(&full).unwrap_or_else(|e| panic!("failed to read corpus file {full:?}: {e}"));
    let sha256 = sha256_hex(&bytes);
    let text = String::from_utf8_lossy(&bytes).to_string();
    CorpusFile {
        relative_path: format!("{book_relative}/{file_name}"),
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

    for sub in ["feat", "equipment"] {
        let dir = out_root.join(sub);
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("clear stale generated subdir");
        }
    }

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
                };
                let path = out_root.join("feat").join(format!("{}.json", slugify(entry.key)));
                write_record(&path, &record);
                feat_written += 1;
            }
            None => feat_unattributed.push(entry.key.to_string()),
        }
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
                };
                let base_slug = slugify(entry.name);
                let count = used_slugs.entry(base_slug.clone()).or_insert(0);
                *count += 1;
                let slug = if *count == 1 { base_slug } else { format!("{base_slug}_{count}") };
                let path = out_root.join("equipment").join(format!("{slug}.json"));
                write_record(&path, &record);
                equipment_written += 1;
            }
            None => equipment_unattributed.push(entry.name.to_string()),
        }
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
    let license_json = serde_json::json!({
        "book": "pathfinder_unchained",
        "license_declaration": {
            "open_game_content": "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2",
            "product_identity_source": "Paizo Pathfinder Roleplaying Game: Pathfinder Unchained, OGL §15 Product Identity section",
            "product_identity_note": "Named deities, NPCs, and unique places are Product Identity per the book's own OGL Section 15 declaration; this book's own feat and equipment-modifier MECHANICS are Open Game Content."
        },
        "redaction_policy": {
            "marker": "[redacted PI]",
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
        "records_redacted": 0,
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

    for sub in ["spell", "equipment", "feat"] {
        let dir = out_root.join(sub);
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("clear stale generated subdir");
        }
    }

    // ---- Spells ----
    let spells_file = load_corpus_file_rel(&root, ARG_BOOK_RELATIVE, "arg_spells.lst");
    let spells_index = arg_build_line_index(&spells_file);
    let mut spell_written = 0u32;
    let mut spell_unattributed: Vec<String> = Vec::new();
    let mut spell_slugs_used: std::collections::HashSet<String> = std::collections::HashSet::new();
    for entry in advanced_race_guide::spell_list::SPELL_LIST {
        match arg_find_citation_line(&spells_index, entry.key) {
            Some(line_no) => {
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
                write_record(&out_root.join("spell").join(format!("{slug}.json")), &record);
                spell_written += 1;
            }
            None => spell_unattributed.push(entry.key.to_string()),
        }
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
                write_record(&out_root.join("equipment").join(category_slug).join(format!("{slug}.json")), &record);
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
                write_record(&out_root.join("feat").join(category_slug).join(format!("{slug}.json")), &record);
                feat_written += 1;
            }
            None => feat_unattributed.push(format!("{:?}:{}", entry.category, entry.key)),
        }
    }

    let total = spell_written + equipment_written + feat_written;

    // ---- LICENSE.json ----
    // See gen_pathfinder_unchained()'s own comment: computed once so
    // `records_processed` and the note's prose can never disagree.
    let records_processed = count_on_disk_records(&out_root);
    let license_json = serde_json::json!({
        "book": "advanced_race_guide",
        "license_declaration": {
            "open_game_content": "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2",
            "product_identity_source": "Paizo Pathfinder Roleplaying Game Advanced Race Guide, OGL §15 Product Identity section",
            "product_identity_note": "Named deities, NPCs, and unique places are Product Identity per the book's own OGL Section 15 declaration; core spell/equipment/feat and racial-trait MECHANICS are Open Game Content."
        },
        "redaction_policy": {
            "marker": "[redacted PI]",
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
        "records_redacted": 0,
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
    let wiring_index = WiringClassIndex::build(book_id, &root);
    let mut wiring_lines = wiring_index.lines();

    for sub in ["monster", "monster_ability"] {
        let dir = out_root.join(sub);
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("clear stale generated subdir");
        }
    }

    // Keyed by file name, because a record's `source_line` is only meaningful
    // together with its `source_file` -- see `MonsterBookSpec::races_lsts`.
    let races_files: HashMap<&'static str, CorpusFile> = spec
        .races_lsts
        .iter()
        .map(|name| (*name, load_corpus_file_rel(&root, spec.book_relative, name)))
        .collect();
    let abilities_file = load_corpus_file_rel(&root, spec.book_relative, spec.abilities_lst);

    // ---- monsters ----
    let mut monster_written = 0u32;
    let mut pi_hits: Vec<String> = Vec::new();
    for block in table.monsters {
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
        let line = verified_citation_line(races_file, block.source_line, block.name);
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
        };
        write_record(
            &out_root.join("monster").join(format!("{}.json", slugify(block.key))),
            &record,
        );
        monster_written += 1;
    }

    // ---- monster abilities ----
    let mut ability_written = 0u32;
    for ability in table.monster_abilities {
        let line = verified_citation_line(&abilities_file, ability.source_line, ability.name);
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
        };
        write_record(
            &out_root
                .join("monster_ability")
                .join(format!("{}.json", slugify(ability.key))),
            &record,
        );
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
    let license_json = serde_json::json!({
        "book": book_id,
        "license_declaration": prior_declaration.unwrap_or_else(|| serde_json::json!({
            "open_game_content": spec.open_game_content,
            "product_identity_source": spec.product_identity_source,
            "product_identity_note": "Named deities, NPCs and unique places are Product Identity; monster stat blocks and their special-ability rules text are Open Game Content."
        })),
        "redaction_policy": {
            "marker": "[redacted PI]",
            "schema_preserving": true,
            "pi_field_recorded": true,
            "blacklist_source": "docs/governance/ogl-pi-blacklist.md",
            "blacklist_version_reviewed": "2026-07-27"
        },
        "screening_method_note": format!(
            "Every field of the {} records this run wrote ({monster_written} monsters + {ability_written} monster abilities) was screened against the bounded, documented term list in docs/governance/ogl-pi-blacklist.md, zero hits. A hit is a hard stop in this generator, not a warning. records_processed is {records_processed}: the real on-disk count for this book across every kind any lane has ingested, which for a book ingested by more than one lane is larger than this run's own output. This is NOT an exhaustive human legal review; it is a bounded substring scan and does not prove the absence of PI beyond what that scan can see.",
            monster_written + ability_written
        ),
        "redistribution_posture": "ogl-notice-attached",
        "classified_at": ingested_at,
        "classified_by_cycle": spec.classified_by_cycle,
        "records_processed": records_processed,
        "records_redacted": 0,
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
        "{book_id} cache generated: {monster_written} monsters, {ability_written} monster abilities; \
         LICENSE.json records_processed={records_processed}"
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
    let wiring_index = WiringClassIndex::build(book_id, &root);
    let mut wiring_lines = wiring_index.lines();

    let dir = out_root.join("companion");
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("clear stale generated subdir");
    }

    let races_file = load_corpus_file_rel(&root, spec.book_relative, spec.races_lst);
    let abilities_file = load_corpus_file_rel(&root, spec.book_relative, spec.abilities_lst);

    let mut pi_hits: Vec<String> = Vec::new();
    let mut creature_written = 0u32;
    for companion in table.companions {
        let line = verified_citation_line(&races_file, companion.source_line, companion.name);
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
        };
        write_record(
            &out_root.join("companion").join(format!("{}.json", slugify(companion.key))),
            &record,
        );
        creature_written += 1;
    }

    let mut ability_written = 0u32;
    for ability in table.companion_abilities {
        let line = verified_citation_line(&abilities_file, ability.source_line, ability.name);
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
        };
        write_record(
            &out_root.join("companion").join(format!("{}.json", slugify(ability.key))),
            &record,
        );
        ability_written += 1;
    }

    if !pi_hits.is_empty() {
        eprintln!("PI screen FAILED for {book_id}: {pi_hits:?}");
        std::process::exit(1);
    }

    let records_processed = count_on_disk_records(&out_root);
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
    let license_json = serde_json::json!({
        "book": book_id,
        "license_declaration": prior_declaration.unwrap_or_else(|| serde_json::json!({
            "open_game_content": spec.open_game_content,
            "product_identity_source": spec.product_identity_source,
            "product_identity_note": "Named deities, NPCs and unique places are Product Identity; companion and familiar stat blocks and their special-ability rules text are Open Game Content."
        })),
        "redaction_policy": {
            "marker": "[redacted PI]",
            "schema_preserving": true,
            "pi_field_recorded": true,
            "blacklist_source": "docs/governance/ogl-pi-blacklist.md",
            "blacklist_version_reviewed": "2026-07-27"
        },
        "screening_method_note": format!(
            "Every field of the {} records this run wrote ({creature_written} companion creatures + {ability_written} companion abilities) was screened against the bounded, documented term list in docs/governance/ogl-pi-blacklist.md, zero hits. A hit is a hard stop in this generator, not a warning. records_processed is {records_processed}: the real on-disk count for this book across every kind any lane has ingested, which for a book ingested by more than one lane is larger than this run's own output. This is NOT an exhaustive human legal review; it is a bounded substring scan and does not prove the absence of PI beyond what that scan can see.",
            creature_written + ability_written
        ),
        "redistribution_posture": "ogl-notice-attached",
        "classified_at": ingested_at,
        "classified_by_cycle": spec.classified_by_cycle,
        "records_processed": records_processed,
        "records_redacted": 0,
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

/// Where one companion book's two `.lst` files live and what its OGL notice
/// says. Same shape and same discipline as [`MonsterBookSpec`]: locations and
/// citations only, never behaviour.
struct CompanionBookSpec {
    corpus_book: &'static str,
    book_relative: &'static str,
    races_lst: &'static str,
    abilities_lst: &'static str,
    open_game_content: &'static str,
    product_identity_source: &'static str,
    classified_by_cycle: &'static str,
}

const COMPANION_BOOK_SPECS: &[CompanionBookSpec] = &[
    CompanionBookSpec {
        corpus_book: "inner_sea_combat",
        book_relative: "pathfinder/paizo/campaign_setting/inner_sea_combat",
        races_lst: "isc_races_companion.lst",
        abilities_lst: "isc_abilities_companion.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own inner_sea_combat.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Inner Sea Combat, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F1-002",
    },
    CompanionBookSpec {
        corpus_book: "monster_codex",
        book_relative: "pathfinder/paizo/roleplaying_game/monster_codex",
        races_lst: "mc_races_companion.lst",
        abilities_lst: "mc_abilities_companion.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _monster_codex.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Monster Codex, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F1-002",
    },
    CompanionBookSpec {
        corpus_book: "inner_sea_intrigue",
        book_relative: "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
        races_lst: "isi_races_companion.lst",
        abilities_lst: "isi_abilities_race_companion.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own inner_sea_intrigue.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Inner Sea Intrigue, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F1-002",
    },
    CompanionBookSpec {
        corpus_book: "horror_adventures",
        book_relative: "pathfinder/paizo/roleplaying_game/horror_adventures",
        races_lst: "ha_races_companion.lst",
        abilities_lst: "ha_abilities_companion.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own horror_adventures.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Horror Adventures, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F1-002",
    },
    // SD-29 Epic 7 round 2 (`SD29-E7-F2-003`). Bestiary 5 and Bestiary 6 carry
    // ZERO monsters -- B5's pcc `CAMPAIGN` line says "Only Player Options
    // Implemented" -- so this generator, not the monster one, is the whole of
    // what those books contribute.
    //
    // **B5's `support/b5_races_companion_oa.lst` is deliberately NOT named
    // here.** `_bestiary_5.pcc:69` loads it under
    // `PRECAMPAIGN:1,Occult Adventures`, a book this repo has not ingested;
    // `decisions.md §47.2`. The transcriber excludes its two rows from the
    // table by reading that pcc gate, so this spec has nothing to point at.
    CompanionBookSpec {
        corpus_book: "bestiary_5",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_5",
        races_lst: "b5_races_companion.lst",
        abilities_lst: "b5_abilities_companion.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _bestiary_5.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 5, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-003",
    },
    CompanionBookSpec {
        corpus_book: "bestiary_6",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_6",
        races_lst: "b6_races_companion.lst",
        abilities_lst: "b6_abilities_companion.lst",
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
        races_lst: "b2_races_familiar.lst",
        abilities_lst: "b2_abilities_familiar_race.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bestiary_2.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 2, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E7-F2-003",
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
    abilities_lst: &'static str,
    open_game_content: &'static str,
    product_identity_source: &'static str,
    classified_by_cycle: &'static str,
}

const MONSTER_BOOK_SPECS: &[MonsterBookSpec] = &[
    MonsterBookSpec {
        corpus_book: "bonus_bestiary",
        book_relative: "pathfinder/paizo/roleplaying_game/bonus_bestiary",
        races_lsts: &["bb_races.lst"],
        abilities_lst: "bb_abilities_race.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bonus_bestiary.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bonus Bestiary, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F1-001",
    },
    MonsterBookSpec {
        corpus_book: "monster_codex",
        book_relative: "pathfinder/paizo/roleplaying_game/monster_codex",
        races_lsts: &["mc_races.lst"],
        abilities_lst: "mc_abilities_race.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _monster_codex.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Monster Codex, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-002",
    },
    MonsterBookSpec {
        corpus_book: "book_of_the_damned_volume_1",
        book_relative: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
        races_lsts: &["botd1_races.lst"],
        abilities_lst: "botd1_abilities_race.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own book_of_the_damned_volume_1.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Princes of Darkness, Book of the Damned Volume 1, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-003",
    },
    MonsterBookSpec {
        corpus_book: "book_of_the_damned_volume_2",
        book_relative: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
        races_lsts: &["botd2_races.lst"],
        abilities_lst: "botd2_abilities_race.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own _book_of_the_damned_volume_2.pcc declares ISOGL:YES and carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Lords of Chaos, Book of the Damned Volume 2, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-003",
    },
    MonsterBookSpec {
        corpus_book: "inner_sea_world_guide",
        book_relative: "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
        races_lsts: &["iswg_races.lst", "iswg_races_bestiary.lst"],
        abilities_lst: "iswg_abilities_race.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own inner_sea_world_guide.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Campaign Setting: Inner Sea World Guide, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-004",
    },
    MonsterBookSpec {
        corpus_book: "bestiary_2",
        book_relative: "pathfinder/paizo/roleplaying_game/bestiary_2",
        races_lsts: &["b2_races.lst"],
        abilities_lst: "b2_abilities_race.lst",
        open_game_content: "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2; the book's own bestiary_2.pcc carries a live COPYRIGHT block plus a real OGL.txt",
        product_identity_source: "Paizo Pathfinder Roleplaying Game: Bestiary 2, OGL §15 Product Identity section",
        classified_by_cycle: "SD29-E5-F2-005",
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
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data").join(spec.book_relative)
}

/// The table records the line it was transcribed from; this re-reads that line
/// out of the live file and requires its first column to still be the record's
/// display name before citing it.
///
/// A citation nobody checked is the failure mode `v06_corpus_trap_report
/// --audit` exists to catch after the fact; checking it here means the cache is
/// never written with a stale line number in the first place.
fn verified_citation_line(file: &CorpusFile, recorded: u32, display_name: &str) -> u32 {
    let idx = recorded as usize - 1;
    let line = file
        .lines
        .get(idx)
        .unwrap_or_else(|| panic!("{} has no line {recorded}", file.relative_path));
    let first_col = line.split('\t').next().unwrap_or_default().trim();
    assert_eq!(
        first_col, display_name,
        "{}:{recorded} names {first_col:?}, not {display_name:?} -- the table's recorded line is \
         stale and must be re-transcribed, not papered over here",
        file.relative_path
    );
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
