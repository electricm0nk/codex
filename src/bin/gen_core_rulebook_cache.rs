//! SD-26 Epic 3 Criterion 3.1 -- one-off codegen tool that dumps the
//! *already-completed* `rules_tables::crb` module state into the
//! `data/corpus/core_rulebook/` JSON cache (Shape B, per
//! `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md`
//! §7/§11).
//!
//! **This binary is a generation-time tool, not shipped runtime code.** It
//! reads the real, local PCGen corpus checkout (`PCGEN_CORPUS_ROOT`, default
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook`)
//! *only* to compute real, checkable per-record citations (file path +
//! SHA-256 + line number) for the discriminated `source` union -- per
//! `decisions.md §11.3`, it never re-derives any `data` field's *value*
//! from the raw LST; every value comes from the compiled Rust
//! `rules_tables::crb` module's own public accessors
//! (`class_tables()`, `equipment_tables()`, `SPELL_LIST`,
//! `ClassId::ALL`, `good_saves_for()`), matching `corpus_ingest_diagnostic.rs`'s
//! existing accessor surface.
//!
//! Run once: `cargo run --bin gen_core_rulebook_cache`. Regenerate if
//! the corpus or `rules_tables::crb` changes (same "regenerate, don't
//! hand-edit" convention `equipment_data/`'s own modules already document).

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use codex::rules_core::cache_gen::WiringClassIndex;
use codex::rules_core::cache_gen::equipment_gap::resolve_name_or_rename;
use codex::rules_core::pi_screening;
use codex::rules_core::rules_tables::crb::class_tables::{self, ClassId, ClassTableRow};
use codex::rules_core::rules_tables::crb::equipment_tables::{self, EquipmentCategory, EquipmentTableEntry};
use codex::rules_core::rules_tables::crb::json_cache::{
    ClassCacheData, Completeness, CorpusRecord, CorpusSource, EquipmentCacheData, Population,
    SpellCacheData,
};
use codex::rules_core::rules_tables::crb::spell_list;

/// `wiring_class`'s corpus-wide book id for CRB.
const WIRING_CLASS_BOOK_ID: &str = "core_rulebook";

/// The `(path, line, record_key)` a `CorpusSource` cites, when it cites a
/// real corpus row at all. `WebSecondSource`/`SameBookFallback` carry no
/// citation to read a token closure from -- `wiring_class` for those
/// lands on `ambiguous:no_corpus_line` rather than guessing one.
fn wiring_citation(source: &CorpusSource) -> Option<(&str, u32, &str)> {
    match source {
        CorpusSource::LstToken { path, line, record_key, .. }
        | CorpusSource::LstInheritedCopy { path, line, record_key, .. }
        | CorpusSource::LstCorrectedIngest { path, line, record_key, .. } => {
            Some((path.as_str(), *line, record_key.as_str()))
        }
        CorpusSource::WebSecondSource { .. } => None,
        CorpusSource::SameBookFallback { .. } => None,
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

const BOOK_RELATIVE: &str = "pathfinder/paizo/roleplaying_game/core_rulebook";

fn corpus_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook")
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Real wall-clock UTC timestamp at generation time (`decisions.md §11.1`
/// -- stamped at write time, never derived from git log).
fn ingested_at_now() -> String {
    let output = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("date -u must be available to stamp ingested_at");
    String::from_utf8(output.stdout)
        .expect("date output is valid UTF-8")
        .trim()
        .to_string()
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

/// Guarantees a unique filename slug within one output directory.
fn unique_slug(used: &mut HashSet<String>, base: &str) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }
    let mut n = 2;
    loop {
        let candidate = format!("{base}_{n}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        n += 1;
    }
}

struct CorpusFile {
    relative_path: String,
    sha256: String,
    lines: Vec<String>,
}

fn load_corpus_file(root: &Path, file_name: &str) -> CorpusFile {
    let full = root.join(file_name);
    let bytes = fs::read(&full).unwrap_or_else(|e| panic!("failed to read corpus file {full:?}: {e}"));
    let sha256 = sha256_hex(&bytes);
    let text = String::from_utf8_lossy(&bytes).to_string();
    CorpusFile {
        relative_path: format!("{BOOK_RELATIVE}/{file_name}"),
        sha256,
        lines: text.lines().map(|s| s.to_string()).collect(),
    }
}

fn identity_field(line: &str) -> Option<&str> {
    line.split('\t').find(|f| !f.is_empty())
}

fn key_token(line: &str) -> Option<&str> {
    line.split('\t').find_map(|f| f.strip_prefix("KEY:"))
}

fn desc_tokens(line: &str) -> Vec<&str> {
    line.split('\t').filter(|f| f.starts_with("DESC:")).collect()
}

struct LineIndex<'a> {
    by_key: HashMap<&'a str, (u32, &'a str)>,
    by_identity: HashMap<&'a str, (u32, &'a str)>,
}

fn build_line_index(file: &CorpusFile) -> LineIndex<'_> {
    let mut by_key = HashMap::new();
    let mut by_identity = HashMap::new();
    for (idx, line) in file.lines.iter().enumerate() {
        let line_no = (idx + 1) as u32;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if let Some(k) = key_token(line) {
            by_key.entry(k).or_insert((line_no, line.as_str()));
        }
        if let Some(id) = identity_field(line) {
            by_identity.entry(id).or_insert((line_no, line.as_str()));
        }
    }
    LineIndex { by_key, by_identity }
}

/// Known d20pfsrd-sourced equipmods concept names (`decisions.md §11.2`/
/// `§11.5`; carried forward verbatim from
/// `../SD-25-ui-evaluation-defect-closure/artifacts/epic_7/corpus-intake-crb-description_cycle_receipt.md`'s
/// own citation list) -> (url, identity_match_basis). `Ghost Touch` is
/// disambiguated by the caller (weapon vs. armor variant share a name).
fn web_second_source_for(name: &str, key: &str) -> Option<(String, String)> {
    const MATERIALS: &[&str] = &["Adamantine", "Mithral", "Darkwood", "Dragonhide", "Cold Iron", "Alchemical Silver"];
    const WEAPON_ABILITIES: &[&str] = &[
        "Flaming", "Frost", "Shock", "Keen", "Vorpal", "Holy", "Unholy", "Bane", "Speed",
        "Wounding", "Anarchic", "Axiomatic", "Brilliant Energy", "Defending", "Disruption",
        "Flaming Burst", "Icy Burst", "Shocking Burst", "Thundering", "Vicious", "Merciful",
        "Mighty Cleaving", "Spell Storing", "Dancing", "Distance", "Returning", "Seeking", "Throwing",
    ];
    const ARMOR_ABILITIES: &[&str] = &["Bashing", "Wild", "Glamered", "Etherealness", "Arrow Catching", "Arrow Deflection"];

    if MATERIALS.contains(&name) {
        return Some((
            "https://www.d20pfsrd.com/equipment---final/special-materials/".to_string(),
            "name".to_string(),
        ));
    }
    if name == "Light Fortification" || name == "Moderate Fortification" || name == "Heavy Fortification" {
        return Some((
            "https://www.d20pfsrd.com/magic-items/magic-armor/magic-armor-and-shield-special-abilities/fortification/".to_string(),
            "name".to_string(),
        ));
    }
    if name.starts_with("Energy Resistance") {
        return Some((
            "https://www.d20pfsrd.com/magic-items/magic-armor/magic-armor-and-shield-special-abilities/energy-resistance/".to_string(),
            "name".to_string(),
        ));
    }
    if name == "Ghost Touch" {
        let url = if key.contains("Armor") {
            "https://www.d20pfsrd.com/magic-items/magic-armor/magic-armor-and-shield-special-abilities/ghost-touch/"
        } else {
            "https://www.d20pfsrd.com/magic-items/magic-weapons/magic-weapon-special-abilities/ghost-touch/"
        };
        return Some((url.to_string(), "name+key-category".to_string()));
    }
    if WEAPON_ABILITIES.contains(&name) {
        let slug = slugify(name).replace('_', "-");
        return Some((
            format!("https://www.d20pfsrd.com/magic-items/magic-weapons/magic-weapon-special-abilities/{slug}/"),
            "name".to_string(),
        ));
    }
    if ARMOR_ABILITIES.contains(&name) {
        let slug = slugify(name).replace('_', "-");
        return Some((
            format!("https://www.d20pfsrd.com/magic-items/magic-armor/magic-armor-and-shield-special-abilities/{slug}/"),
            "name".to_string(),
        ));
    }
    None
}

/// `(source_file, source_line)` for `resolve_name_or_rename`'s neutral-name
/// derivation (`decisions.md §24b`-1: coordinates only, never the PI
/// string) -- extracted generically across `equipment_source`'s four
/// possible `CorpusSource` variants. `WebSecondSource` carries no LST line
/// at all; its `url` (unique per record) stands in for `source_file` with
/// `source_line == 0` so the derivation stays deterministic even for that
/// variant.
fn source_coordinate(source: &CorpusSource) -> (String, u32) {
    match source {
        CorpusSource::LstToken { path, line, .. }
        | CorpusSource::LstInheritedCopy { path, line, .. }
        | CorpusSource::LstCorrectedIngest { path, line, .. } => (path.clone(), *line),
        CorpusSource::WebSecondSource { url, .. } => (url.clone(), 0),
        CorpusSource::SameBookFallback { fallback_basis } => (fallback_basis.clone(), 0),
    }
}

fn equipment_source(file: &CorpusFile, index: &LineIndex<'_>, entry: &EquipmentTableEntry) -> Option<CorpusSource> {
    let (line_no, line_text) = index
        .by_key
        .get(entry.key)
        .copied()
        .or_else(|| index.by_identity.get(entry.key).copied())?;

    let desc_toks = desc_tokens(line_text);
    let has_clear = desc_toks.contains(&"DESC:.CLEAR");
    let real_desc = desc_toks.iter().find(|t| **t != "DESC:.CLEAR");

    if has_clear && real_desc.is_some() {
        return Some(CorpusSource::LstCorrectedIngest {
            path: file.relative_path.clone(),
            sha256: file.sha256.clone(),
            line: line_no,
            record_key: entry.key.to_string(),
            original_ingest_defect:
                "same-line 'DESC:.CLEAR' sentinel followed by a real DESC: token on the same corpus row; \
                 an earlier codegen pass captured only the .CLEAR sentinel (SD-25 criterion 7.N fix)"
                    .to_string(),
        });
    }

    if real_desc.is_some() {
        return Some(CorpusSource::LstToken {
            path: file.relative_path.clone(),
            sha256: file.sha256.clone(),
            line: line_no,
            record_key: entry.key.to_string(),
        });
    }

    if entry.description.is_none() {
        // No DESC: token in the corpus, and this record's own description
        // is genuinely None -- still a real citation for the record's
        // identity/cost/weight fields.
        return Some(CorpusSource::LstToken {
            path: file.relative_path.clone(),
            sha256: file.sha256.clone(),
            line: line_no,
            record_key: entry.key.to_string(),
        });
    }

    // Rust carries a real description but the LST line has no DESC: token.
    let identity = identity_field(line_text).unwrap_or_default();
    if let Some(copy_pos) = identity.find(".COPY=") {
        let base = &identity[..copy_pos];
        return Some(CorpusSource::LstInheritedCopy {
            path: file.relative_path.clone(),
            sha256: file.sha256.clone(),
            line: line_no,
            record_key: entry.key.to_string(),
            inherited_from_record_key: base.to_string(),
        });
    }

    if let Some((url, basis)) = web_second_source_for(entry.name, entry.key) {
        return Some(CorpusSource::WebSecondSource {
            url,
            fetched_at: "2026-07-22".to_string(),
            identity_match_basis: basis,
        });
    }

    None
}

fn class_source(classes_file: &CorpusFile, class_name: &str) -> Option<CorpusSource> {
    let wanted = format!("CLASS:{class_name}");
    for (idx, line) in classes_file.lines.iter().enumerate() {
        if line.starts_with('#') {
            continue;
        }
        if identity_field(line) == Some(wanted.as_str()) && line.contains("HD:") && line.contains("MAXLEVEL:") {
            return Some(CorpusSource::LstToken {
                path: classes_file.relative_path.clone(),
                sha256: classes_file.sha256.clone(),
                line: (idx + 1) as u32,
                record_key: wanted,
            });
        }
    }
    None
}

fn class_cache_data(rows: &[ClassTableRow], class_id: ClassId) -> ClassCacheData {
    let class_rows: Vec<&ClassTableRow> = rows.iter().filter(|r| r.class_id == class_id).collect();
    let maxlevel = class_rows
        .iter()
        .map(|r| r.level)
        .max()
        .unwrap_or_else(|| panic!("{class_id:?} must have at least one class_tables() row"));
    let level4 = class_rows
        .iter()
        .find(|r| r.level == 4)
        .unwrap_or_else(|| panic!("{class_id:?} must have a level-4 row to derive its real BAB formula"));
    let bab = match level4.base_attack_bonus {
        4 => "level",
        3 => "level*3/4",
        2 => "level/2",
        other => panic!("unexpected level-4 BAB {other} for {class_id:?}; refusing to fabricate a formula"),
    };
    let (fort_good, ref_good, will_good) = class_tables::good_saves_for(class_id)
        .unwrap_or_else(|| panic!("good_saves_for({class_id:?}) must have real data"));
    let fmt_save = |good: bool| if good { "level/2+2" } else { "level/3" };
    ClassCacheData {
        class_id: format!("{class_id:?}"),
        maxlevel,
        bab: bab.to_string(),
        save_fort: fmt_save(fort_good).to_string(),
        save_ref: fmt_save(ref_good).to_string(),
        save_will: fmt_save(will_good).to_string(),
    }
}

fn write_record<T: serde::Serialize>(path: &Path, record: &CorpusRecord<T>) {
    fs::create_dir_all(path.parent().expect("record path must have a parent dir"))
        .expect("failed to create output dir");
    let json = serde_json::to_string_pretty(record).expect("record must serialize");
    fs::write(path, json).unwrap_or_else(|e| panic!("failed to write {path:?}: {e}"));
}

fn equipment_category_slug(category: EquipmentCategory) -> &'static str {
    match category {
        EquipmentCategory::ArmsArmor => "arms_armor",
        EquipmentCategory::General => "general",
        EquipmentCategory::MagicItems => "magic_items",
        EquipmentCategory::Equipmods => "equipmods",
    }
}

fn main() {
    let root = corpus_root();
    let out_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/corpus/core_rulebook");

    // **SD-32 Epic 5 protective sweep (`epic-breakdown.md` Epic 5, T3
    // residual / `defects.md` D9), live-reproduced**: this used to wipe
    // EVERY subdirectory of `out_root` on every run -- not just the three
    // kinds (`class`/`spell`/`equipment`) this binary itself owns.
    // Live-reproduced against this repo's real committed corpus in an
    // isolated worktree (git status clean before, `git checkout -- ` +
    // `git clean -fd` after): one run deleted 959 `class_feature`, 84
    // `companion`, 330 `equipment`, 7 `race` and 67 `race_trait` records --
    // every one of them owned by a DIFFERENT generator
    // (`cache_gen::class_feature`, companion books that cite
    // `core_rulebook`, `ingest_races.rs`/`ingest_race_traits.rs`,
    // `cache_gen::equipment_gap`) that also writes into this same book's
    // `out_root`, none of which this binary's own class/spell/equipment
    // loops below ever touch -- and stripped `raw_tokens` from all 664 of
    // this book's own spell records (`enrich_spell_raw_tokens.rs` writes
    // that field in a later pass this generator's own `SpellCacheData`
    // cannot reconstruct). Fixed at the root: this binary now touches
    // ONLY the three kind directories it actually writes, each with its
    // own guard (see the write loops below and their doc comments).
    let ingested_at = ingested_at_now();
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &root);
    let mut wiring_lines = wiring_index.lines();

    // ---- Classes ----
    let classes_file = load_corpus_file(&root, "cr_classes.lst");
    let class_rows = class_tables::class_tables();
    let mut class_written = 0u32;
    let mut class_unattributed: Vec<String> = Vec::new();
    for &class_id in ClassId::ALL {
        let class_name = format!("{class_id:?}");
        match class_source(&classes_file, &class_name) {
            Some(source) => {
                let data = class_cache_data(&class_rows, class_id);
                let (wiring_class, wiring_class_signals) =
                    wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
                let (license, pi_field, pi_marker) = pi_screening::blanket_ogl();
                let record = CorpusRecord {
                    population: Population::InScope,
                    completeness: Completeness::ChassisOnly,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    wiring_class,
                    wiring_class_signals,
                    license: Some(license),
                    pi_field,
                    pi_marker,
                    codex_generated_name: false,
                    rename: None,
                };
                // Exists-guard only -- `ClassId::ALL` has no `key` field to
                // key a stale-sweep on (see this fn's own doc comment), and
                // the 11-class population is effectively fixed.
                let path = out_root.join("class").join(format!("{}.json", slugify(&class_name)));
                if !path.exists() {
                    write_record(&path, &record);
                }
                class_written += 1;
            }
            None => class_unattributed.push(class_name),
        }
    }

    // ---- Spells ----
    let spells_file = load_corpus_file(&root, "cr_spells.lst");
    let mut spell_written = 0u32;
    let mut spell_unattributed: Vec<String> = Vec::new();
    let mut spell_slugs_used: HashMap<u8, HashSet<String>> = HashMap::new();
    let mut current_spell_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    for entry in spell_list::SPELL_LIST {
        let mod_identity = format!("{}.MOD", entry.key);
        let mut found: Option<(u32, String)> = None;
        for (idx, line) in spells_file.lines.iter().enumerate() {
            if line.starts_with('#') {
                continue;
            }
            if identity_field(line) == Some(mod_identity.as_str())
                && desc_tokens(line).iter().any(|t| t.len() > "DESC:".len())
            {
                found = Some(((idx + 1) as u32, mod_identity.clone()));
                break;
            }
        }
        if found.is_none() {
            for (idx, line) in spells_file.lines.iter().enumerate() {
                if line.starts_with('#') {
                    continue;
                }
                if identity_field(line) == Some(entry.key)
                    && desc_tokens(line).iter().any(|t| t.len() > "DESC:".len())
                {
                    found = Some(((idx + 1) as u32, entry.key.to_string()));
                    break;
                }
            }
        }
        // `.COPY=<entry.key>` racial spell-like-ability variant
        // (decisions.md §15, 2026-08-17): `CLASSES:.CLEARALL` rows carry no
        // `DESC:` of their own (spell properties/level/description inherit
        // from the parent named before `.COPY=`, already resolved onto
        // `entry` itself), so this branch is deliberately the only one that
        // does not require a `DESC:` token on the matched line -- it still
        // requires the row's own identity field to literally read
        // `<parent>.COPY=<entry.key>`, so it can only ever match a row that
        // really does copy-declare this exact spell name.
        if found.is_none() {
            let copy_suffix = format!(".COPY={}", entry.key);
            for (idx, line) in spells_file.lines.iter().enumerate() {
                if line.starts_with('#') {
                    continue;
                }
                if let Some(identity) = identity_field(line)
                    && identity.ends_with(&copy_suffix)
                {
                    found = Some(((idx + 1) as u32, identity.to_string()));
                    break;
                }
            }
        }

        match found {
            Some((line_no, record_key)) => {
                let (mut license, mut pi_field, mut pi_marker, mut stored_desc) =
                    pi_screening::classify_field("description", entry.description);
                // t9-onboarding-pi-last-leak-and-generators cycle: the SAME
                // supplementary strong-scan re-screen `cache_gen::
                // equipment_gap`/`cache_gen::ultimate_equipment`'s own
                // "third defect" fix already established -- `classify_field`
                // screens via the weak, bare-substring, case-SENSITIVE scan
                // only. Never weakens an existing redaction, only
                // strengthens a miss.
                if stored_desc != codex::rules_core::shape_b_v1::REDACTED_PI_MARKER
                    && pi_screening::blacklist_term_hit_including_concatenated(&stored_desc).is_some()
                {
                    license = codex::rules_core::shape_b_v1::License::PiRedacted;
                    pi_marker = Some(codex::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
                    pi_field = Some(match pi_field.take() {
                        Some(existing) if existing.split(',').any(|p| p == "description") => existing,
                        Some(existing) => format!("{existing},description"),
                        None => "description".to_string(),
                    });
                    stored_desc = codex::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string();
                }
                // t9-onboarding-pi-last-leak-and-generators cycle: this
                // generator never screened `key` at all -- only
                // `description`. Same shape `cache_gen::{acg,apg,
                // beastiary1,equipment_gap,ultimate_equipment}` were already
                // fixed for; the eighth instance in this bundle. `SpellCacheData`
                // carries no separate display-name field (`key` doubles as
                // both identity and the player-facing string), so only
                // `key` needs the union scan -- there is no declared-PI
                // (`NAMEISPI:`) reader wired into this generator at all
                // (a separate, pre-existing gap, not this fix's scope);
                // this union is the blacklist-term half only, matching the
                // dispatch's own named shape.
                let key_is_pi = pi_screening::blacklist_term_hit_including_concatenated(entry.key).is_some();
                let (_unused_name, renamed_key, codex_generated_name, rename_info, divergence) =
                    resolve_name_or_rename(key_is_pi, "spell", "core_rulebook", &spells_file.relative_path, line_no, entry.key, entry.key);
                if divergence.is_some() {
                    license = codex::rules_core::shape_b_v1::License::PiRedacted;
                    pi_marker = Some(codex::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
                    pi_field = Some(match pi_field.take() {
                        Some(existing) if existing.split(',').any(|p| p == "key") => existing,
                        Some(existing) => format!("{existing},key"),
                        None => "key".to_string(),
                    });
                }
                let data = SpellCacheData {
                    key: renamed_key.clone(),
                    school: format!("{:?}", entry.school),
                    level: entry.level,
                    description: stored_desc,
                };
                let source = CorpusSource::LstToken {
                    path: spells_file.relative_path.clone(),
                    sha256: spells_file.sha256.clone(),
                    line: line_no,
                    record_key: if codex_generated_name { renamed_key.clone() } else { record_key },
                };
                let (wiring_class, wiring_class_signals) =
                    wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
                let record = CorpusRecord {
                    population: Population::InScope,
                    completeness: Completeness::Full,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    wiring_class,
                    wiring_class_signals,
                    license: Some(license),
                    pi_field,
                    pi_marker,
                    codex_generated_name,
                    rename: rename_info,
                };
                current_spell_keys.insert(renamed_key.clone());
                let used = spell_slugs_used.entry(entry.level).or_default();
                // `§24b`-2's directory/filename guard (`cache_gen::
                // equipment_gap`/`class_feature`'s identical precedent): the
                // on-disk slug is derived from the (possibly-renamed)
                // OUTPUT key, never the original `entry.key`, so a
                // blacklisted key cannot leak into the file path either.
                let slug = unique_slug(used, &slugify(&renamed_key));
                let path = out_root
                    .join("spell")
                    .join(format!("level_{}", entry.level))
                    .join(format!("{slug}.json"));
                // `SD31-E6-F9-005`-shaped guard: a file already on disk
                // (including one `enrich_spell_raw_tokens.rs` has since
                // written `raw_tokens` into) is left completely untouched.
                if !path.exists() {
                    write_record(&path, &record);
                }
                spell_written += 1;
            }
            None => spell_unattributed.push(entry.key.to_string()),
        }
    }
    if out_root.join("spell").exists() {
        // Single writer of `core_rulebook/spell/` (verified: `core_rulebook`
        // is in neither `cache_gen::spell_lane_dump`'s nor
        // `cache_gen::spell_mod_access`'s book lists -- SD-32 cross-generator
        // sweep, 2026-08-23).
        codex::rules_core::cache_gen::ultimate_equipment::remove_stale_owned_files(
            &out_root.join("spell"),
            &current_spell_keys,
            &|_path, _line| true,
        );
    }

    // ---- Equipment ----
    let arms_armor_file = load_corpus_file(&root, "cr_equip_arms_armor.lst");
    let general_file = load_corpus_file(&root, "cr_equip_general.lst");
    let magic_items_file = load_corpus_file(&root, "cr_equip_magic_items.lst");
    let equipmods_file = load_corpus_file(&root, "cr_equipmods.lst");

    let arms_armor_index = build_line_index(&arms_armor_file);
    let general_index = build_line_index(&general_file);
    let magic_items_index = build_line_index(&magic_items_file);
    let equipmods_index = build_line_index(&equipmods_file);

    let all_equipment = equipment_tables::equipment_tables();

    // Per decisions.md §11.6: equipmods.rs has 314/658 duplicate-`key`
    // shell records (`name == key`, near-empty). De-duplicate at
    // cache-write time (preferring the non-shell entry per key) rather
    // than silently writing duplicate keys into the JSON cache; the
    // underlying `equipmods.rs` defect itself is forwarded as a
    // DISCOVERED item (this cycle's file-touch grant is the cache, not
    // the generator's source data file).
    let mut equipmods_by_key: HashMap<&str, &EquipmentTableEntry> = HashMap::new();
    let mut equipmods_dup_keys: HashSet<&str> = HashSet::new();
    for entry in all_equipment.iter().filter(|e| e.category == EquipmentCategory::Equipmods) {
        match equipmods_by_key.get(entry.key) {
            None => {
                equipmods_by_key.insert(entry.key, entry);
            }
            Some(existing) => {
                equipmods_dup_keys.insert(entry.key);
                let existing_is_shell = existing.name == existing.key;
                let candidate_is_shell = entry.name == entry.key;
                if existing_is_shell && !candidate_is_shell {
                    equipmods_by_key.insert(entry.key, entry);
                }
                // else: keep the existing (already non-shell, or both are
                // shells and the first one wins deterministically).
            }
        }
    }

    let mut equipment_written = 0u32;
    let mut equipment_unattributed: Vec<String> = Vec::new();
    let mut equipment_has_description = 0u32;
    let mut equipment_slugs_used: HashMap<&'static str, HashSet<String>> = HashMap::new();

    let mut equipment_iter: Vec<&EquipmentTableEntry> = Vec::new();
    for entry in all_equipment.iter() {
        match entry.category {
            EquipmentCategory::Equipmods => {
                if equipmods_by_key.get(entry.key) == Some(&entry) {
                    equipment_iter.push(entry);
                }
            }
            _ => equipment_iter.push(entry),
        }
    }

    for entry in equipment_iter {
        let (file, index) = match entry.category {
            EquipmentCategory::ArmsArmor => (&arms_armor_file, &arms_armor_index),
            EquipmentCategory::General => (&general_file, &general_index),
            EquipmentCategory::MagicItems => (&magic_items_file, &magic_items_index),
            EquipmentCategory::Equipmods => (&equipmods_file, &equipmods_index),
        };
        match equipment_source(file, index, entry) {
            Some(source) => {
                if entry.description.is_some() {
                    equipment_has_description += 1;
                }
                let completeness = if entry.description.is_some() {
                    Completeness::Full
                } else if entry.cost_gp.is_some() || entry.weight_lbs.is_some() {
                    Completeness::ChassisPlusExtract
                } else {
                    Completeness::ChassisOnly
                };
                let (mut license, mut pi_field, mut pi_marker, mut stored_desc) =
                    pi_screening::classify_optional_field("description", entry.description);
                // t9-onboarding-pi-last-leak-and-generators cycle: the SAME
                // supplementary strong-scan re-screen `cache_gen::
                // equipment_gap`/`cache_gen::ultimate_equipment`'s own
                // "third defect" fix already established -- never weakens
                // an existing redaction, only strengthens a miss the weak
                // `classify_optional_field` scan let through.
                if let Some(v) = &stored_desc {
                    if v.as_str() != codex::rules_core::shape_b_v1::REDACTED_PI_MARKER
                        && pi_screening::blacklist_term_hit_including_concatenated(v).is_some()
                    {
                        license = codex::rules_core::shape_b_v1::License::PiRedacted;
                        pi_marker = Some(codex::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
                        pi_field = Some(match pi_field.take() {
                            Some(existing) if existing.split(',').any(|p| p == "description") => existing,
                            Some(existing) => format!("{existing},description"),
                            None => "description".to_string(),
                        });
                        stored_desc = Some(codex::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string());
                    }
                }
                // t9-onboarding-pi-last-leak-and-generators cycle: `name`/
                // `key` were NEVER screened at all -- only `description`.
                // Same shape `cache_gen::{acg,apg,beastiary1,equipment_gap,
                // ultimate_equipment}` were already fixed for; the eighth
                // instance in this bundle.
                let name_is_pi = pi_screening::blacklist_term_hit_including_concatenated(entry.name).is_some()
                    || pi_screening::blacklist_term_hit_including_concatenated(entry.key).is_some();
                let (source_file, source_line) = source_coordinate(&source);
                let kind = if entry.category == EquipmentCategory::Equipmods {
                    "equipment_modifier"
                } else {
                    "equipment"
                };
                let (renamed_name, renamed_key, codex_generated_name, rename_info, divergence) =
                    resolve_name_or_rename(name_is_pi, kind, "core_rulebook", &source_file, source_line, entry.name, entry.key);
                if divergence.is_some() {
                    license = codex::rules_core::shape_b_v1::License::PiRedacted;
                    pi_marker = Some(codex::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
                    let mut fields: Vec<&str> = Vec::new();
                    if pi_field.as_deref().is_some_and(|f| f.split(',').any(|p| p == "description")) {
                        fields.push("description");
                    }
                    fields.push("name");
                    pi_field = Some(fields.join(","));
                }
                let data = EquipmentCacheData {
                    key: renamed_key.clone(),
                    category: equipment_category_slug(entry.category).to_string(),
                    name: renamed_name,
                    cost_gp: entry.cost_gp,
                    weight_lbs: entry.weight_lbs,
                    description: stored_desc,
                };
                let (wiring_class, wiring_class_signals) =
                    wiring_class_for_source(&wiring_index, &mut wiring_lines, &source);
                let record = CorpusRecord {
                    population: Population::InScope,
                    completeness,
                    ingested_at: ingested_at.clone(),
                    data,
                    source,
                    wiring_class,
                    wiring_class_signals,
                    license: Some(license),
                    pi_field,
                    pi_marker,
                    codex_generated_name,
                    rename: rename_info,
                };
                let category_slug = equipment_category_slug(entry.category);
                let used = equipment_slugs_used.entry(category_slug).or_default();
                // `§24b`-2's directory/filename guard: derive the slug from
                // the (possibly-renamed) OUTPUT key, never `entry.key`.
                let slug = unique_slug(used, &slugify(&renamed_key));
                let path = out_root.join("equipment").join(category_slug).join(format!("{slug}.json"));
                // Exists-guard only, deliberately NO stale-key sweep here --
                // `cache_gen::equipment_gap` ("CRB" routing) also writes
                // real records into this same `equipment` directory; a
                // sweep keyed on this loop's own entries would delete them.
                if !path.exists() {
                    write_record(&path, &record);
                }
                equipment_written += 1;
            }
            None => equipment_unattributed.push(format!("{:?}:{}", entry.category, entry.key)),
        }
    }

    println!("SD-26 3.1 core_rulebook cache generation report");
    println!("  classes written: {class_written} / {}", ClassId::ALL.len());
    if !class_unattributed.is_empty() {
        println!("  classes UNATTRIBUTED (skipped, not written): {class_unattributed:?}");
    }
    println!("  spells written: {spell_written} / {}", spell_list::SPELL_LIST.len());
    if !spell_unattributed.is_empty() {
        println!(
            "  spells UNATTRIBUTED (skipped, not written): {} -- {:?}",
            spell_unattributed.len(),
            spell_unattributed
        );
    }
    println!(
        "  equipment written: {equipment_written} / {} real records ({} unique equipmods keys of {} total)",
        all_equipment.len() - equipmods_dup_keys.len(),
        equipmods_by_key.len(),
        all_equipment.iter().filter(|e| e.category == EquipmentCategory::Equipmods).count()
    );
    println!(
        "  equipment has_description: {equipment_has_description} / {equipment_written} ({:.1}%)",
        100.0 * equipment_has_description as f64 / equipment_written as f64
    );
    if !equipment_unattributed.is_empty() {
        println!(
            "  equipment UNATTRIBUTED (skipped, not written): {} -- first 20: {:?}",
            equipment_unattributed.len(),
            equipment_unattributed.iter().take(20).collect::<Vec<_>>()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- t9-onboarding-pi-last-leak-and-generators: `name`/`key`
    // blacklist screening (`decisions.md §17`: the SAME shape
    // `cache_gen::{acg,apg,beastiary1,equipment_gap,ultimate_equipment}`
    // already established, reused here rather than reinvented). This
    // binary's own record-construction loops are not factored into
    // injectable functions (unlike `cache_gen::ultimate_equipment`'s
    // `generate_equipment`), so `source_coordinate` -- the one new pure
    // helper this fix introduced -- is what's directly testable without a
    // real `PCGEN_CORPUS_ROOT` checkout; the underlying scan/rename
    // primitives (`resolve_name_or_rename`,
    // `blacklist_term_hit_including_concatenated`) are already
    // exhaustively tested in `cache_gen::equipment_gap`'s own test module.

    #[test]
    fn source_coordinate_reads_path_and_line_for_every_lst_backed_variant() {
        let lst_token = CorpusSource::LstToken {
            path: "book/file.lst".to_string(),
            sha256: "x".to_string(),
            line: 42,
            record_key: "k".to_string(),
        };
        assert_eq!(source_coordinate(&lst_token), ("book/file.lst".to_string(), 42));

        let inherited = CorpusSource::LstInheritedCopy {
            path: "book/file2.lst".to_string(),
            sha256: "x".to_string(),
            line: 7,
            record_key: "k".to_string(),
            inherited_from_record_key: "base".to_string(),
        };
        assert_eq!(source_coordinate(&inherited), ("book/file2.lst".to_string(), 7));

        let corrected = CorpusSource::LstCorrectedIngest {
            path: "book/file3.lst".to_string(),
            sha256: "x".to_string(),
            line: 9,
            record_key: "k".to_string(),
            original_ingest_defect: "d".to_string(),
        };
        assert_eq!(source_coordinate(&corrected), ("book/file3.lst".to_string(), 9));
    }

    /// A `WebSecondSource` citation carries no LST line at all -- proves
    /// the fallback still produces a stable, deterministic
    /// `(source_file, source_line)` pair (`url`, `0`) rather than
    /// panicking or fabricating a line number.
    #[test]
    fn source_coordinate_falls_back_to_the_url_for_web_second_source() {
        let web = CorpusSource::WebSecondSource {
            url: "https://example.invalid/item".to_string(),
            fetched_at: "2026-01-01".to_string(),
            identity_match_basis: "name".to_string(),
        };
        assert_eq!(
            source_coordinate(&web),
            ("https://example.invalid/item".to_string(), 0)
        );
    }

    /// End-to-end proof against the real `resolve_name_or_rename` this
    /// binary now calls (imported, not reimplemented, `§24b`-1): an
    /// undeclared blacklisted key still renames -- referencing the term by
    /// index (`§24b`-2: never write a blacklist term literally into a
    /// test).
    #[test]
    fn an_undeclared_blacklisted_key_renames_via_the_shared_helper() {
        let term = pi_screening::PI_BLACKLIST_TERMS[9];
        let name = format!("{term}'s Blessed Blade");
        let hit = pi_screening::blacklist_term_hit_including_concatenated(&name).is_some();
        assert!(hit, "sanity: the strong scan must catch the blacklisted term");
        let (renamed_name, renamed_key, codex_generated_name, rename_info, divergence) =
            resolve_name_or_rename(hit, "equipment", "core_rulebook", "cr_equip_general.lst", 12, &name, &name);
        assert!(codex_generated_name);
        assert!(!renamed_name.contains(term));
        assert!(!renamed_key.contains(term));
        assert!(rename_info.is_some());
        assert!(divergence.is_some());
    }

    /// The negative case: an ordinary undeclared name with no blacklist
    /// hit passes through unchanged -- zero behaviour change for every
    /// already-shipped CRB record.
    #[test]
    fn an_ordinary_key_passes_through_the_shared_helper_unchanged() {
        let (renamed_name, renamed_key, codex_generated_name, rename_info, divergence) = resolve_name_or_rename(
            false,
            "equipment",
            "core_rulebook",
            "cr_equip_general.lst",
            12,
            "Masterwork Backpack",
            "Masterwork Backpack",
        );
        assert!(!codex_generated_name);
        assert_eq!(renamed_name, "Masterwork Backpack");
        assert_eq!(renamed_key, "Masterwork Backpack");
        assert!(rename_info.is_none());
        assert!(divergence.is_none());
    }
}
