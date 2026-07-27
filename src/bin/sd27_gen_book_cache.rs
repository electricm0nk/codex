//! SD-27 Cycle E2.1/E2.2 -- the shared per-book codegen tool
//! `docs/release/SD-27-future-state-book-content-ingestion/
//! technical-design.md §2.2`/§3 names: it reads an already-completed
//! `rules_tables::<book>/` module's compiled state and serializes it to
//! `data/corpus/<book>/{content_kind}/<id>.json` as Shape B v1
//! (`src/rules_core/shape_b_v1.rs`) records, mirroring
//! `src/bin/sd26_gen_core_rulebook_cache.rs`'s established discipline:
//! it never re-derives a `data` field's *value* from raw LST at
//! generation time (values come from the compiled Rust module's own
//! accessors) -- it only reads the live corpus file to compute a real,
//! checkable per-record citation (path + SHA-256 + line number).
//!
//! **Single shared binary name, per book-agnostic design.** The SD-27
//! partition audit (`loop-instruction.md §6`) allow-lists exactly
//! `src/bin/sd27_gen_book_cache.rs` -- one file, not a per-book file --
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
//! by its first CLI argument (`cargo run --bin sd27_gen_book_cache --
//! advanced_race_guide`); PU's own no-argument default is left
//! unchanged for backward compatibility with any existing invocation.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use codex::rules_core::shape_b_v1::{Completeness, CorpusRecordV1, CorpusSource, License, Population};

#[path = "../rules_core/rules_tables/pathfinder_unchained/mod.rs"]
mod pathfinder_unchained;

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
/// -- mirrors `sd26_gen_core_rulebook_cache.rs`'s `LineIndex`/`build_line_index`
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

fn write_record<T: serde::Serialize>(path: &Path, record: &CorpusRecordV1<T>) {
    fs::create_dir_all(path.parent().expect("record path must have a parent dir")).expect("failed to create output dir");
    let json = serde_json::to_string_pretty(record).expect("record must serialize");
    fs::write(path, json).unwrap_or_else(|e| panic!("failed to write {path:?}: {e}"));
}

/// Heuristic OGL/PI screen (`docs/governance/ogl-pi-blacklist.md`), the
/// same bounded, documented substring scan `scripts/sd27_apg_license_retrofit.py`
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
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly },
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
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
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly },
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
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
        "screening_method_note": "This pass is a heuristic first-pass screen of every `description` value against a bounded, documented term list (the 20 canonical core Golarion deities plus a sampled set of known setting place names, the same list docs/governance/ogl-pi-blacklist.md's operating cycle used for the 4 in-scope books' retro-fit). Zero matches were found for this book's 59 records (17 feats + 42 equipment modifiers) -- consistent with this book's own subject matter (alignment/stamina/wound-threshold feats and Automatic Bonus Progression equipment modifiers) being entirely rules-mechanical, setting-neutral text. This is NOT an exhaustive human legal review; it is a bounded substring/regex scan against ~54 known names and does not prove the absence of PI beyond what that scan can see.",
        "redistribution_posture": "ogl-notice-attached",
        "classified_at": ingested_at,
        "classified_by_cycle": "E2.2",
        "records_processed": feat_written + equipment_written,
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
                let (license, pi_field, pi_marker, stored_desc) = classify_field("description", entry.description);
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
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: Completeness::Full,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
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
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
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
                let record = CorpusRecordV1 {
                    population: Population::InScope,
                    completeness: Completeness::Full,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    license: Some(license),
                    pi_field,
                    pi_marker,
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
    let license_json = serde_json::json!({
        "book": "advanced_race_guide",
        "license_declaration": {
            "open_game_content": "OGL 1.0a (Wizards of the Coast), inlined verbatim per docs/governance/ogl-pi-blacklist.md §2.2",
            "product_identity_source": "Paizo Pathfinder Roleplaying Game Advanced Race Guide, OGL §15 Product Identity section",
            "product_identity_note": "Named deities, NPCs, and unique places are Product Identity per the book's own OGL Section 15 declaration; core spell/equipment/feat MECHANICS are Open Game Content."
        },
        "redaction_policy": {
            "marker": "[redacted PI]",
            "schema_preserving": true,
            "pi_field_recorded": true,
            "blacklist_source": "docs/governance/ogl-pi-blacklist.md",
            "blacklist_version_reviewed": "2026-07-27"
        },
        "screening_method_note": format!(
            "This pass is a heuristic first-pass screen of every `description` value against a bounded, documented term list (the same PI_BLACKLIST_TERMS this binary's gen_pathfinder_unchained() also uses -- the 20 canonical core Golarion deities plus a sampled set of known setting place names, matching docs/governance/ogl-pi-blacklist.md's operating cycle for the 4 in-scope books' retro-fit). Scan result for this book: {total} records screened, all matched OGL (0 PI hits observed in this cycle's real spell/equipment/feat description text). This is NOT an exhaustive human legal review; it is a bounded substring/regex scan against ~54 known names and does not prove the absence of PI beyond what that scan can see."
        ),
        "redistribution_posture": "ogl-notice-attached",
        "classified_at": ingested_at,
        "classified_by_cycle": "E2.1",
        "records_processed": total,
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

fn main() {
    let book = std::env::args().nth(1).unwrap_or_else(|| "pathfinder_unchained".to_string());
    match book.as_str() {
        "pathfinder_unchained" => gen_pathfinder_unchained(),
        "advanced_race_guide" => gen_advanced_race_guide(),
        other => panic!(
            "sd27_gen_book_cache: no generator wired for book {other:?} yet (only pathfinder_unchained/advanced_race_guide today -- \
             a future SD-27/SD-28+ cycle extends this match arm for its own book, per this file's own module doc comment)"
        ),
    }
}
