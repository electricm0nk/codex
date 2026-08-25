//! APG JSON cache generator (SD-26 Epic 3, Criterion 3.2).
//!
//! Writes `data/corpus/advanced_players_guide/{class,spell,equipment}/*.json`
//! by DUMPING the current, already-completed state of
//! `rules_core::rules_tables::apg` (`ApgClassId::ALL` + each class's
//! `class_table()`, `spell_list::SPELL_LIST`, `equipment_tables::EQUIPMENT_TABLE`)
//! -- per `decisions.md §11.3`, this module never re-parses raw PCGen LST
//! to derive a field's *value*. Every value written here is read straight
//! from the compiled Rust module, exactly as `corpus_ingest_diagnostic.rs`
//! (SD-25 Epic 5) already reads it via the same public accessors.
//!
//! **What this module DOES read the real LST corpus for:** recovering a
//! real, checkable `path`/`sha256`/`line` *citation* for a value already
//! known (from the Rust module) to be correct. Locating where a known-good
//! value lives in the corpus is not the same operation as re-deriving that
//! value from the corpus; the module doc comment on each per-class Rust
//! file already records line-number citations by the same method (hand
//! transcription against the LST at ingest time) -- this module automates
//! that same citation-lookup for spell/equipment records, which don't
//! carry hand-written line citations today.
//!
//! ## Shape B `source` discriminated union (`decisions.md §11.2`)
//!
//! A JSON record's `source` field is judged by this generator's own
//! design decision (documented in this cycle's receipt) to describe the
//! provenance of the record's **most informative field** -- the BAB/save
//! chassis for a class, `description`/`full_text` for a spell or
//! equipment record -- not a per-field breakdown. Shape B's schema
//! defines one `source` per JSON record; equipment/spell records mix
//! LST-native identity fields (`name`/`cost`/`weight`, `school`/`level`)
//! with a possibly differently-sourced `description`, and this generator
//! resolves that tension by keying `source` off `description` (the field
//! SD-25's corpus-intake work actually closed this pass, and the field
//! `decisions.md §11.2`'s discriminated union was designed to represent).
//!
//! ## Equipment `web_second_source.url` citation precision (judgment call)
//!
//! SD-25's `corpus-intake-apg-description` cycle receipt names roughly 35
//! equipment records individually (rings/cursed-items/named-armor/
//! named-weapons categories) with an exact source page each, but the
//! remaining ~296 populated descriptions were sourced from 10
//! category-aggregate pages covering "nearly all" of a category without
//! an exhaustive per-item name list. Since `equipment_data.rs` does not
//! store a per-record source URL (SD-25 worked directly in Rust source,
//! not through this cache), this generator cannot recover an exact
//! per-item URL for every record without redoing SD-25's web research --
//! outside this cycle's charter (`decisions.md §11.3`: dump the Rust
//! state, don't re-derive it). Every record therefore gets a REAL URL
//! (never fabricated) at one of two confidence tiers, both surfaced via
//! the additive, non-schema-required `citation_precision` field:
//! - `"exact_item_page"` -- record name matches SD-25's receipt-cited
//!   named-item list exactly; `url` is that item's specific page.
//! - `"category_aggregate_page"` -- record falls back to the single
//!   best-documented aggregate page for its `EquipmentCategory`
//!   (`General`/mundane `ArmsArmor` -> `advancedGear.html`, the only page
//!   the receipt cites for either; residual `MagicItems` ->
//!   `wondrousItems.html`, the largest single named MagicItems bucket).

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::codex_neutral_name::{neutral_key, neutral_name};
use crate::rules_core::pi_screening;
use crate::rules_core::rules_tables::apg::equipment_tables::EquipmentCategory;
use crate::rules_core::rules_tables::apg::{self, ApgClassId};

/// `wiring_class`'s corpus-wide book id for APG.
const WIRING_CLASS_BOOK_ID: &str = "advanced_players_guide";

// ---------------------------------------------------------------------
// Shape B schema (decisions.md §7, corrected §11.1/§11.2)
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Population {
    InScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Completeness {
    ChassisOnly,
    Full,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Source {
    LstToken {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
    },
    LstCorrectedIngest {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
        original_ingest_defect: String,
    },
    WebSecondSource {
        url: String,
        fetched_at: String,
        identity_match_basis: String,
        citation_precision: String,
    },
    SameBookFallback {
        fallback_basis: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheRecord<T: Serialize> {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: T,
    pub source: Source,
    /// GE-01: what kind of evidence would prove this record done, from
    /// `codex::rules_core::wiring_class`'s real corpus token closure --
    /// see `cache_gen::acg::CacheRecord::wiring_class`'s doc comment.
    pub wiring_class: String,
    pub wiring_class_signals: Vec<String>,
    /// `"OGL" | "PI" | "PI-REDACTED"` -- see
    /// `cache_gen::acg::CacheRecord::license`'s doc comment.
    pub license: crate::rules_core::shape_b_v1::License,
    pub pi_field: Option<String>,
    pub pi_marker: Option<String>,
    /// `decisions.md §24b`-3 -- see `cache_gen::acg::CacheRecord::
    /// codex_generated_name`'s identical doc comment; same gap-close, same
    /// generation family, `t9-onboarding-pi-final-leaks-and-generators` cycle.
    pub codex_generated_name: bool,
}

/// `cache_gen::acg::name_or_key_is_pi`'s byte-identical sibling for APG --
/// same gap, same fix, `t9-onboarding-pi-final-leaks-and-generators` cycle.
/// Zero live impact today (this cycle's own corpus-wide re-derivation found
/// no hit in this book).
fn name_or_key_is_pi(values: &[&str]) -> bool {
    values.iter().any(|v| pi_screening::blacklist_term_hit_including_concatenated(v).is_some())
}

// ---------------------------------------------------------------------
// Content-kind data shapes
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct ClassChassisRow {
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClassData {
    pub class_id: String,
    pub maxlevel: u8,
    pub chassis: Vec<ClassChassisRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpellData {
    pub key: String,
    pub school: Option<String>,
    pub level: Option<u8>,
    pub description: Option<String>,
    pub full_text: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EquipmentData {
    pub key: String,
    pub category: String,
    pub name: String,
    pub cost_gp: Option<f64>,
    pub weight: Option<f64>,
    pub description: Option<String>,
}

// ---------------------------------------------------------------------
// Corpus-access helpers (citation lookup only, never value derivation)
// ---------------------------------------------------------------------

const APG_DIR: &str = "pathfinder/paizo/roleplaying_game/advanced_players_guide";

fn book_dir(corpus_root: &Path) -> PathBuf {
    corpus_root.join(APG_DIR)
}

/// Real sha256 of `path`'s current on-disk content, via the system
/// `sha256sum` tool (no `sha2` crate dependency exists in this workspace
/// yet; mirrors `pcgen_runner.rs`'s established pattern of wrapping a
/// real external tool for a generation-time-only concern rather than
/// adding a new crate dependency).
pub fn sha256_file(path: &Path) -> std::io::Result<String> {
    let output = Command::new("sha256sum").arg(path).output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!(
            "sha256sum failed for {}",
            path.display()
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.split_whitespace().next().unwrap_or_default().to_string())
}

/// A resolved LST citation: which file (relative to `APG_DIR`) and which
/// 1-indexed line the record was found on.
struct Citation {
    file_name: String,
    line: u32,
}

/// Finds `record_name` as an exact match on a line's first tab-delimited
/// column in `lst_path`. Real corpus lookup, not a value parse -- only
/// the line number is used.
fn find_exact_first_column(lst_path: &Path, record_name: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    for (idx, line) in content.lines().enumerate() {
        let first_col = line.split('\t').next().unwrap_or("");
        if first_col == record_name {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// Like [`find_exact_first_column`], but only returns a line that ALSO
/// carries a real `DESC:` token.
///
/// **Why this exists.** `resolve_citation`'s `prefer_mod` path exists so
/// a `full_text: true` record cites the `.MOD` row its rich text actually
/// came from -- but a record can have MORE than one `.MOD` row (e.g.
/// bookkeeping rows like `CLASSES:.CLEARALL` or `DOMAINS:...` that modify
/// the same name for an unrelated reason), and the first one by line
/// order is not necessarily the one carrying the description. GE-01's
/// 2026-08-03 regeneration surfaced exactly this: `apg_spells.lst`'s
/// `Beast Shape I (Animals Only)` has two `.MOD` rows (line 1059
/// `CLASSES:.CLEARALL`, line 1103 `DOMAINS:Fur Subdomain=3`), NEITHER
/// carrying `DESC:`, so the old unconditional "first `.MOD` row by name"
/// match cited whichever came first by accident of file order -- a
/// content-free bookkeeping row, strictly worse provenance than the
/// `.COPY=`-inherited declaration it replaced. This function requires the
/// matched row to actually carry the content `prefer_mod` exists to cite.
fn find_mod_with_desc(lst_path: &Path, record_name: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    for (idx, line) in content.lines().enumerate() {
        let mut fields = line.split('\t');
        let first_col = fields.next().unwrap_or("");
        if first_col == record_name && fields.any(|f| f.starts_with("DESC:")) {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// Finds a `.COPY=<record_name>` variant line's first column
/// (`<base>.COPY=<record_name>`) in `lst_path`.
fn find_copy_variant(lst_path: &Path, record_name: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    let needle = format!(".COPY={record_name}");
    for (idx, line) in content.lines().enumerate() {
        let first_col = line.split('\t').next().unwrap_or("");
        if first_col.ends_with(&needle) {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// Finds a line carrying the exact tab-delimited field `KEY:<record_key>`
/// in `lst_path` -- mirrors `cache_gen::acg`'s own `find_by_key_field`.
/// Required for the 9 APG Summoner `Summon Monster I`-`IX` spells: their
/// first column is the display name (`Summon Monster I`), but their real
/// corpus identity is `KEY:Summoner Summon Monster I` -- a first-column
/// match finds nothing (`entry.key` is the KEY-qualified identity), so
/// `resolve_citation` reported "no resolvable LST citation" for all 9
/// even though each has a real, single-line declaration
/// (`apg_spells.lst:649`-`657`). Same defect shape as the ACG Naturalist
/// fix (`053cfd51`): identity is `KEY:`, not field 0, and a resolver that
/// only checks field 0 either finds nothing or the wrong record.
fn find_by_key_field(lst_path: &Path, record_key: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    let needle = format!("KEY:{record_key}");
    for (idx, line) in content.lines().enumerate() {
        if line.split('\t').any(|field| field == needle) {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// Resolves a real citation for `record_name`, trying (in order): a
/// `KEY:<record_name>` field match (the record's real corpus identity,
/// when it differs from field 0 -- see `find_by_key_field`), the exact
/// base-record line, a `<record_name>.MOD` line (when `prefer_mod` -- the
/// record's rich text came from a `.MOD` stanza), a `.COPY=<record_name>`
/// variant line, and finally every other `*.lst` file directly under the
/// book directory (a small number of records, e.g. `Formula Book`, live
/// in a sibling file like `apg_templates.lst` rather than the category
/// file the record's Rust category would suggest).
fn resolve_citation(
    corpus_root: &Path,
    primary_file: &str,
    record_name: &str,
    prefer_mod: bool,
) -> std::io::Result<Option<Citation>> {
    let dir = book_dir(corpus_root);
    let primary_path = dir.join(primary_file);

    if prefer_mod {
        let mod_name = format!("{record_name}.MOD");
        if let Some(line) = find_mod_with_desc(&primary_path, &mod_name)? {
            return Ok(Some(Citation {
                file_name: primary_file.to_string(),
                line,
            }));
        }
    }
    if let Some(line) = find_exact_first_column(&primary_path, record_name)? {
        return Ok(Some(Citation {
            file_name: primary_file.to_string(),
            line,
        }));
    }
    if let Some(line) = find_copy_variant(&primary_path, record_name)? {
        return Ok(Some(Citation {
            file_name: primary_file.to_string(),
            line,
        }));
    }
    // A record whose real identity is `KEY:<record_name>` rather than its
    // field-0 display name (the 9 Summoner `Summon Monster I`-`IX`
    // records). Tried after the field-0-based checks above, not before:
    // those already correctly resolve every other APG spell, and a KEY:
    // field is comparatively rare on a spell row, so this only ever fires
    // for the records field-0 matching genuinely cannot find.
    if let Some(line) = find_by_key_field(&primary_path, record_name)? {
        return Ok(Some(Citation {
            file_name: primary_file.to_string(),
            line,
        }));
    }

    // Fallback: scan every other real *.lst file directly under the book
    // directory (non-recursive; skips the `_pfs` subdirectory).
    let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|ext| ext == "lst").unwrap_or(false))
        .collect();
    entries.sort();
    for path in entries {
        if path == primary_path {
            continue;
        }
        if let Some(line) = find_exact_first_column(&path, record_name)? {
            return Ok(Some(Citation {
                file_name: path.file_name().unwrap().to_string_lossy().to_string(),
                line,
            }));
        }
    }
    Ok(None)
}

// ---------------------------------------------------------------------
// Generation report
// ---------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub classes_written: usize,
    pub spells_written: usize,
    pub equipment_written: usize,
    /// Record keys whose real LST citation could not be resolved (should
    /// be empty for a clean generation run against the real corpus;
    /// surfaced rather than silently defaulted).
    pub unresolved_citations: Vec<String>,
}

#[derive(Debug)]
pub enum GenerationError {
    Io(std::io::Error),
    CorpusUnreachable(PathBuf),
}

impl From<std::io::Error> for GenerationError {
    fn from(e: std::io::Error) -> Self {
        GenerationError::Io(e)
    }
}

/// SD-32 Epic 5 protective sweep -- see `cache_gen::acg::write_json`'s
/// identical doc comment; same shape, same fix.
fn write_json<T: Serialize>(out_dir: &Path, slug: &str, record: &CacheRecord<T>) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let path = out_dir.join(format!("{slug}.json"));
    if path.exists() {
        return Ok(());
    }
    let json = serde_json::to_string_pretty(record)
        .expect("CacheRecord<T> is a plain-data shape; serialization cannot fail");
    std::fs::write(path, json)
}

fn slugify(name: &str, used: &mut BTreeSet<String>) -> String {
    let mut slug: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    while slug.contains("__") {
        slug = slug.replace("__", "_");
    }
    let slug = slug.trim_matches('_').to_string();
    let slug = if slug.is_empty() { "unnamed".to_string() } else { slug };

    if !used.contains(&slug) {
        used.insert(slug.clone());
        return slug;
    }
    let mut n = 2;
    loop {
        let candidate = format!("{slug}-{n}");
        if !used.contains(&candidate) {
            used.insert(candidate.clone());
            return candidate;
        }
        n += 1;
    }
}

// ---------------------------------------------------------------------
// Real per-class line citations (`apg_classes.lst`), transcribed from
// each class module's own doc comment -- see `class_<name>.rs:3`.
// ---------------------------------------------------------------------

fn class_line(class_id: ApgClassId) -> u32 {
    match class_id {
        ApgClassId::Alchemist => 11,
        ApgClassId::Cavalier => 42,
        ApgClassId::Inquisitor => 50,
        ApgClassId::Oracle => 107,
        ApgClassId::Summoner => 139,
        ApgClassId::Witch => 172,
    }
}

/// The real `CLASS:<Name>` token's capitalized form, matching each
/// class's own doc-comment citation (e.g. `CLASS:Alchemist`).
fn class_capitalized_name(class_id: ApgClassId) -> &'static str {
    match class_id {
        ApgClassId::Alchemist => "Alchemist",
        ApgClassId::Cavalier => "Cavalier",
        ApgClassId::Inquisitor => "Inquisitor",
        ApgClassId::Oracle => "Oracle",
        ApgClassId::Summoner => "Summoner",
        ApgClassId::Witch => "Witch",
    }
}

fn generate_classes(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let classes_file = "apg_classes.lst";
    let path = book_dir(corpus_root).join(classes_file);
    let sha256 = sha256_file(&path)?;
    let mut used = BTreeSet::new();
    let class_dir = out_dir.join("class");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    for &class_id in ApgClassId::ALL {
        let rows: Vec<ClassChassisRow> = match class_id {
            ApgClassId::Alchemist => apg::class_alchemist::class_table(),
            ApgClassId::Cavalier => apg::class_cavalier::class_table(),
            ApgClassId::Inquisitor => apg::class_inquisitor::class_table(),
            ApgClassId::Oracle => apg::class_oracle::class_table(),
            ApgClassId::Summoner => apg::class_summoner::class_table(),
            ApgClassId::Witch => apg::class_witch::class_table(),
        }
        .into_iter()
        .map(|row| ClassChassisRow {
            level: row.level,
            base_attack_bonus: row.base_attack_bonus,
            fort_save: row.fort_save,
            ref_save: row.ref_save,
            will_save: row.will_save,
        })
        .collect();
        let maxlevel = rows.last().map(|r| r.level).unwrap_or(0);
        let line = class_line(class_id);
        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            classes_file,
            line,
            class_capitalized_name(class_id),
            class_capitalized_name(class_id),
        );

        let (license, pi_field, pi_marker) = pi_screening::blanket_ogl();
        let record = CacheRecord {
            population: Population::InScope,
            completeness: Completeness::ChassisOnly,
            ingested_at: ingested_at.to_string(),
            data: ClassData {
                class_id: class_id.name().to_string(),
                maxlevel,
                chassis: rows,
            },
            source: Source::LstToken {
                path: format!("{APG_DIR}/{classes_file}"),
                sha256: sha256.clone(),
                line,
                record_key: format!("CLASS:{}", class_capitalized_name(class_id)),
            },
            wiring_class,
            wiring_class_signals,
            license,
            pi_field,
            pi_marker,
            codex_generated_name: false,
        };
        let slug = slugify(class_id.name(), &mut used);
        write_json(&class_dir, &slug, &record)?;
        report.classes_written += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Spell overrides (real provenance transcribed from
// `corpus-intake-apg-spell-text_cycle_receipt.md`, SD-25 Epic 7)
// ---------------------------------------------------------------------

/// (spell key) -> (url, identity_match_basis). All 6 real web-sourced
/// APG spell-text records this pass.
fn web_sourced_spell(key: &str) -> Option<(&'static str, &'static str)> {
    match key {
        "Fester" => Some((
            "https://www.d20pfsrd.com/magic/all-spells/f/fester/",
            "name+school+level",
        )),
        "Heroic Fortune" | "Heroic Fortune (Mass)" | "Malediction" | "Severed Fate" | "Unravel Destiny" => Some((
            "https://legacy.aonprd.com/advancedPlayersGuide/advancedNewRules.html",
            "name+school+level",
        )),
        _ => None,
    }
}

/// The 3 `PRESPELL`-linked `Threefold Aspect` sub-forms: same-book base
/// record narrates the sub-form by name (register A8-style extension of
/// the module's `.COPY=` fallback convention -- see `spell_list.rs`'s
/// doc comment).
fn same_book_fallback_spell(key: &str) -> bool {
    matches!(
        key,
        "Threefold Aspect (Young Adult)" | "Threefold Aspect (Adulthood)" | "Threefold Aspect (Elderly)"
    )
}

/// The 3 records whose real full text was recovered by fixing a
/// same-line `.MOD`-stanza-concatenation ingest defect this SD-25 pass
/// (`apg_spells.lst:1945` carries both `Fester (Mass).MOD` and `Fiery
/// Body.MOD`; `apg_spells.lst:2094` carries both `Transmogrify.MOD` and
/// `Transmute Potion to Poison.MOD`). Returns the concatenated line + a
/// description of the defect.
fn corrected_ingest_spell(key: &str) -> Option<(u32, &'static str)> {
    match key {
        "Fester (Mass)" => Some((
            1945,
            "same-line .MOD-stanza concatenation (with Fiery Body.MOD); prior parser \
             attributed the line's last DESC: token to this record instead of its own",
        )),
        "Fiery Body" => Some((
            1945,
            "same-line .MOD-stanza concatenation (with Fester (Mass).MOD); prior parser \
             attributed this record's real DESC: text to Fester (Mass) instead",
        )),
        "Transmute Potion to Poison" => Some((
            2094,
            "same-line .MOD-stanza concatenation (with Transmogrify.MOD); prior parser \
             did not recognize this record's DESC: text as its own full-text source",
        )),
        _ => None,
    }
}

fn spell_source(
    corpus_root: &Path,
    spell_file: &str,
    sha256: &str,
    entry: &apg::spell_list::SpellListEntry,
    fetched_at_web: &str,
    unresolved: &mut Vec<String>,
) -> Source {
    if let Some((url, basis)) = web_sourced_spell(entry.key) {
        return Source::WebSecondSource {
            url: url.to_string(),
            fetched_at: fetched_at_web.to_string(),
            identity_match_basis: basis.to_string(),
            citation_precision: "exact_item_page".to_string(),
        };
    }
    if same_book_fallback_spell(entry.key) {
        return Source::SameBookFallback {
            fallback_basis: "PRESPELL base record (Threefold Aspect) narrates this sub-form by name"
                .to_string(),
        };
    }
    if let Some((line, defect)) = corrected_ingest_spell(entry.key) {
        return Source::LstCorrectedIngest {
            path: format!("{APG_DIR}/{spell_file}"),
            sha256: sha256.to_string(),
            line,
            record_key: entry.key.to_string(),
            original_ingest_defect: defect.to_string(),
        };
    }

    // Default: plain corpus record. Prefer a `.MOD` citation when the
    // record's rich text is sourced from one (full_text: true).
    match resolve_citation(corpus_root, spell_file, entry.key, entry.full_text) {
        Ok(Some(citation)) => Source::LstToken {
            path: format!("{APG_DIR}/{}", citation.file_name),
            sha256: if citation.file_name == spell_file {
                sha256.to_string()
            } else {
                sha256_file(&book_dir(corpus_root).join(&citation.file_name)).unwrap_or_default()
            },
            line: citation.line,
            record_key: entry.key.to_string(),
        },
        _ => {
            unresolved.push(format!("spell:{}", entry.key));
            Source::LstToken {
                path: format!("{APG_DIR}/{spell_file}"),
                sha256: sha256.to_string(),
                line: 0,
                record_key: entry.key.to_string(),
            }
        }
    }
}

fn generate_spells(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    fetched_at_web: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let spell_file = "apg_spells.lst";
    let path = book_dir(corpus_root).join(spell_file);
    let sha256 = sha256_file(&path)?;
    let mut used = BTreeSet::new();
    let spell_dir = out_dir.join("spell");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    for entry in apg::spell_list::SPELL_LIST {
        let source = spell_source(
            corpus_root,
            spell_file,
            &sha256,
            entry,
            fetched_at_web,
            &mut report.unresolved_citations,
        );
        // `wiring_class` describes the CORPUS RECORD's own row shape, which
        // exists independent of where `source` (above) sourced the
        // record's DESCRIPTION prose from -- a web-second-sourced or
        // same-book-fallback spell still has a real base `.lst` row.
        // Resolved separately from `source` so a citation this record
        // does not use for its description is still used for wiring_class.
        let wiring_citation = resolve_citation(corpus_root, spell_file, entry.key, entry.full_text)
            .ok()
            .flatten();
        let (wiring_file, wiring_line) = match &wiring_citation {
            Some(c) => (c.file_name.as_str(), c.line),
            None => (spell_file, 0),
        };
        let (wiring_class, wiring_class_signals) =
            wiring_index.wiring_class_for(&mut wiring_lines, wiring_file, wiring_line, entry.key, entry.key);
        let completeness = if entry.description.is_some() {
            Completeness::Full
        } else {
            Completeness::ChassisOnly
        };
        let (mut license, mut pi_field, pi_marker, stored_desc) =
            pi_screening::classify_optional_field("description", entry.description);
        let key_is_pi = name_or_key_is_pi(&[entry.key]);
        let out_key = if key_is_pi {
            license = crate::rules_core::shape_b_v1::License::PiRedacted;
            let mut fields: Vec<&str> = Vec::new();
            if pi_field.as_deref() == Some("description") {
                fields.push("description");
            }
            fields.push("key");
            pi_field = Some(fields.join(","));
            neutral_key("spell", WIRING_CLASS_BOOK_ID, wiring_file, wiring_line)
        } else {
            entry.key.to_string()
        };
        let record = CacheRecord {
            population: Population::InScope,
            completeness,
            ingested_at: ingested_at.to_string(),
            data: SpellData {
                key: out_key,
                school: entry.school.map(|s| format!("{s:?}")),
                level: entry.level,
                description: stored_desc,
                full_text: entry.full_text,
            },
            source,
            wiring_class,
            wiring_class_signals,
            license,
            pi_field,
            pi_marker,
            codex_generated_name: key_is_pi,
        };
        // `cache_gen::acg::generate_spells`'s identical directory-placement-
        // fix precedent: slug from the (possibly-renamed) output key.
        let slug = slugify(&record.data.key, &mut used);
        write_json(&spell_dir, &slug, &record)?;
        report.spells_written += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Equipment overrides (real provenance transcribed from
// `corpus-intake-apg-description_cycle_receipt.md`, SD-25 Epic 7)
// ---------------------------------------------------------------------

const CURSED_ITEMS_URL: &str = "https://legacy.aonprd.com/advancedPlayersGuide/magicItems/cursedItems.html";
const ARMOR_URL: &str = "https://legacy.aonprd.com/advancedPlayersGuide/magicItems/armor.html";
const WEAPONS_URL: &str = "https://legacy.aonprd.com/advancedPlayersGuide/magicItems/weapons.html";
const ADVANCED_GEAR_URL: &str = "https://legacy.aonprd.com/advancedPlayersGuide/advancedGear.html";
const WONDROUS_ITEMS_URL: &str = "https://legacy.aonprd.com/advancedPlayersGuide/magicItems/wondrousItems.html";

const NAMED_ITEM_URLS: &[(&str, &str)] = &[
    // cursedItems.html (8 named + Rod of Arson)
    ("Buffoon's Sword", CURSED_ITEMS_URL),
    ("Cannibal Ring", CURSED_ITEMS_URL),
    ("Ring of Truth", CURSED_ITEMS_URL),
    ("Cloak of Immolation", CURSED_ITEMS_URL),
    ("Crown of Blindness", CURSED_ITEMS_URL),
    ("Hat of Hatreds", CURSED_ITEMS_URL),
    ("Girdle of Opposite Gender", CURSED_ITEMS_URL),
    ("Riot Pipes", CURSED_ITEMS_URL),
    ("Rod of Arson", CURSED_ITEMS_URL),
    // armor.html (named specific magic armors/shields)
    ("Mistmail", ARMOR_URL),
    ("Soothsayer's Raiment (Battlecry)", ARMOR_URL),
    ("Soothsayer's Raiment (Battlefield Clarity)", ARMOR_URL),
    ("Boneless Leather", ARMOR_URL),
    ("Buccaneer's Breastplate", ARMOR_URL),
    ("Daystar Half-Plate", ARMOR_URL),
    ("Folding Plate", ARMOR_URL),
    ("Forsaken Banded Mail", ARMOR_URL),
    ("Giant Hide (Ogre)", ARMOR_URL),
    ("Giant Hide (Hill)", ARMOR_URL),
    ("Giant Hide (Stone)", ARMOR_URL),
    ("Giant Hide (Fire)", ARMOR_URL),
    ("Giant Hide (Frost)", ARMOR_URL),
    ("Giant Hide (Troll)", ARMOR_URL),
    ("Giant Hide (Cloud)", ARMOR_URL),
    ("Giant Hide (Storm)", ARMOR_URL),
    ("Murderer's Blackcloth", ARMOR_URL),
    ("Armor of Insults", ARMOR_URL),
    ("Battlement Shield", ARMOR_URL),
    ("Fortress Shield", ARMOR_URL),
    // weapons.html (named specific magic weapons)
    ("Beaststrike Club", WEAPONS_URL),
    ("Mace (Boulderhead)", WEAPONS_URL),
    ("Guarding Blade", WEAPONS_URL),
    ("Hammer (Ricochet)", WEAPONS_URL),
    ("Starknife (Sparkwake)", WEAPONS_URL),
    ("Axe (Undercutting)", WEAPONS_URL),
    ("Trident of Stability", WEAPONS_URL),
    ("Blade of Binding", WEAPONS_URL),
    ("Lance (Jousting)", WEAPONS_URL),
    ("Lance (Shieldsplitter)", WEAPONS_URL),
    ("Spirit Blade", WEAPONS_URL),
    ("Arrow (Searing)", WEAPONS_URL),
    ("Bolt (Tangle)", WEAPONS_URL),
    ("Bullet (Dustburst)", WEAPONS_URL),
];

/// The corpus-quirk subset (`decisions.md §11.5`) whose own `COST:` token
/// is unreliable -- identity was cross-checked against `weight` instead.
const WEIGHT_BASIS_ITEMS: &[&str] = &[
    "Beaststrike Club",
    "Mace (Boulderhead)",
    "Guarding Blade",
    "Giant Hide (Ogre)",
    "Giant Hide (Hill)",
    "Giant Hide (Stone)",
    "Giant Hide (Fire)",
    "Giant Hide (Frost)",
    "Giant Hide (Troll)",
    "Giant Hide (Cloud)",
    "Giant Hide (Storm)",
    "Mistmail",
    "Boneless Leather",
];

fn equipment_category_file(category: EquipmentCategory) -> &'static str {
    match category {
        EquipmentCategory::General => "apg_equip_general.lst",
        EquipmentCategory::ArmsArmor => "apg_equip_arms_armor.lst",
        EquipmentCategory::MagicItems => "apg_equip_magic_items.lst",
    }
}

fn equipment_source(
    corpus_root: &Path,
    entry: &apg::equipment_tables::EquipmentTableEntry,
    sha_by_file: &HashMap<&'static str, String>,
    fetched_at_web: &str,
    unresolved: &mut Vec<String>,
) -> Source {
    let category_file = equipment_category_file(entry.category);

    if entry.description.is_none() {
        // Honest gap (7/338): no confident web match, or (Formula
        // Book/Bomb) no COST:/WT: token at all. Record identity itself
        // is still real -- cite its actual corpus line.
        return match resolve_citation(corpus_root, category_file, entry.key, false) {
            Ok(Some(citation)) => Source::LstToken {
                path: format!("{APG_DIR}/{}", citation.file_name),
                sha256: sha_by_file
                    .get(citation.file_name.as_str())
                    .cloned()
                    .unwrap_or_else(|| {
                        sha256_file(&book_dir(corpus_root).join(&citation.file_name)).unwrap_or_default()
                    }),
                line: citation.line,
                record_key: entry.key.to_string(),
            },
            _ => {
                unresolved.push(format!("equipment:{}", entry.key));
                Source::LstToken {
                    path: format!("{APG_DIR}/{category_file}"),
                    sha256: sha_by_file.get(category_file).cloned().unwrap_or_default(),
                    line: 0,
                    record_key: entry.key.to_string(),
                }
            }
        };
    }

    let (url, precision) = match NAMED_ITEM_URLS.iter().find(|(k, _)| *k == entry.key) {
        Some((_, url)) => (*url, "exact_item_page"),
        None => match entry.category {
            EquipmentCategory::General | EquipmentCategory::ArmsArmor => {
                (ADVANCED_GEAR_URL, "category_aggregate_page")
            }
            EquipmentCategory::MagicItems => (WONDROUS_ITEMS_URL, "category_aggregate_page"),
        },
    };
    let basis = if WEIGHT_BASIS_ITEMS.contains(&entry.key) {
        "name+weight"
    } else {
        "name+cost"
    };

    Source::WebSecondSource {
        url: url.to_string(),
        fetched_at: fetched_at_web.to_string(),
        identity_match_basis: basis.to_string(),
        citation_precision: precision.to_string(),
    }
}

fn generate_equipment(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    fetched_at_web: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let mut sha_by_file = HashMap::new();
    for file in [
        "apg_equip_general.lst",
        "apg_equip_arms_armor.lst",
        "apg_equip_magic_items.lst",
    ] {
        let sha = sha256_file(&book_dir(corpus_root).join(file))?;
        sha_by_file.insert(file, sha);
    }
    let mut used = BTreeSet::new();
    let equipment_dir = out_dir.join("equipment");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    for entry in apg::equipment_tables::EQUIPMENT_TABLE {
        let source = equipment_source(
            corpus_root,
            entry,
            &sha_by_file,
            fetched_at_web,
            &mut report.unresolved_citations,
        );
        // Same rationale as `generate_spells`: `wiring_class` reads the
        // corpus record's own row, resolved independently of whether
        // `source` (above) ended up web-second-sourced for its
        // description prose.
        let category_file = equipment_category_file(entry.category);
        let wiring_citation = resolve_citation(corpus_root, category_file, entry.key, false).ok().flatten();
        let (wiring_file, wiring_line) = match &wiring_citation {
            Some(c) => (c.file_name.as_str(), c.line),
            None => (category_file, 0),
        };
        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            wiring_file,
            wiring_line,
            entry.key,
            entry.key,
        );
        let completeness = if entry.description.is_some() {
            Completeness::Full
        } else {
            Completeness::ChassisOnly
        };
        let (mut license, mut pi_field, pi_marker, stored_desc) =
            pi_screening::classify_optional_field("description", entry.description);
        let name_is_pi = name_or_key_is_pi(&[entry.key, entry.name]);
        let (out_key, out_name) = if name_is_pi {
            license = crate::rules_core::shape_b_v1::License::PiRedacted;
            let mut fields: Vec<&str> = Vec::new();
            if pi_field.as_deref() == Some("description") {
                fields.push("description");
            }
            fields.push("name");
            pi_field = Some(fields.join(","));
            (
                neutral_key("equipment", WIRING_CLASS_BOOK_ID, wiring_file, wiring_line),
                neutral_name("equipment", WIRING_CLASS_BOOK_ID, wiring_file, wiring_line),
            )
        } else {
            (entry.key.to_string(), entry.name.to_string())
        };
        let record = CacheRecord {
            population: Population::InScope,
            completeness,
            ingested_at: ingested_at.to_string(),
            data: EquipmentData {
                key: out_key,
                category: format!("{:?}", entry.category),
                name: out_name,
                cost_gp: entry.cost_gp,
                weight: entry.weight,
                description: stored_desc,
            },
            source,
            wiring_class,
            wiring_class_signals,
            license,
            pi_field,
            pi_marker,
            codex_generated_name: name_is_pi,
        };
        // `cache_gen::acg::generate_equipment`'s identical directory-
        // placement-fix precedent: slug from the (possibly-renamed) key.
        let slug = slugify(&record.data.key, &mut used);
        write_json(&equipment_dir, &slug, &record)?;
        report.equipment_written += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------

/// Generates the full APG JSON cache under `out_dir`
/// (`data/corpus/advanced_players_guide/`), reading real LST citations
/// from `corpus_root` (a PCGen `data/` checkout,
/// e.g. `~/workspace/repos/pcgen/data`). `ingested_at` is stamped at
/// call time by the caller (real wall-clock ISO-8601, never derived from
/// git log -- `decisions.md §11.1`); `fetched_at_web` is the real commit
/// date of the SD-25 cycle that performed the actual web second-source
/// research this generator is dumping (a real, checkable proxy for "when
/// this was sourced", since the original web-fetch pass predates this
/// cache's existence and its own fetch timestamp was not persisted
/// anywhere machine-readable).
pub fn generate(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    fetched_at_web: &str,
) -> Result<GenerationReport, GenerationError> {
    if !book_dir(corpus_root).is_dir() {
        return Err(GenerationError::CorpusUnreachable(book_dir(corpus_root)));
    }
    let mut report = GenerationReport::default();
    generate_classes(corpus_root, out_dir, ingested_at, &mut report)?;
    generate_spells(corpus_root, out_dir, ingested_at, fetched_at_web, &mut report)?;
    generate_equipment(corpus_root, out_dir, ingested_at, fetched_at_web, &mut report)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- t9-onboarding-pi-final-leaks-and-generators: `name`/`key`
    // screening (mirrors `cache_gen::acg`'s own tests exactly). Never a
    // literal blacklist term -- indexes into
    // `pi_screening::PI_BLACKLIST_TERMS`, per `decisions.md §24b`-2.

    #[test]
    fn name_or_key_is_pi_is_false_for_an_ordinary_clean_value() {
        assert!(!name_or_key_is_pi(&["Longsword", "Cure Light Wounds"]));
    }

    #[test]
    fn name_or_key_is_pi_catches_a_blacklisted_key() {
        let term = pi_screening::PI_BLACKLIST_TERMS[9];
        assert!(name_or_key_is_pi(&[term, "clean"]));
    }

    #[test]
    fn name_or_key_is_pi_catches_a_blacklisted_name_even_when_key_is_clean() {
        let term = pi_screening::PI_BLACKLIST_TERMS[22];
        assert!(name_or_key_is_pi(&["clean_key", term]));
    }

    #[test]
    fn a_name_pi_equipment_entry_would_be_renamed_never_shipped_under_its_own_identity() {
        let term = pi_screening::PI_BLACKLIST_TERMS[24];
        assert!(name_or_key_is_pi(&[term, "Ordinary Item"]));
        let codex_key = neutral_key("equipment", WIRING_CLASS_BOOK_ID, "apg_equip_general.lst", 7);
        let codex_name = neutral_name("equipment", WIRING_CLASS_BOOK_ID, "apg_equip_general.lst", 7);
        assert_ne!(codex_key, term);
        assert_ne!(codex_name, term);
        assert!(codex_name.starts_with("Codex-Named Unit"));
    }

    #[test]
    fn slugify_handles_parens_and_collisions() {
        let mut used = BTreeSet::new();
        assert_eq!(slugify("Arrow (Searing)", &mut used), "arrow_searing");
        let mut used2 = BTreeSet::new();
        let a = slugify("Giant Hide (Ogre)", &mut used2);
        let b = slugify("Giant Hide (Ogre)", &mut used2);
        assert_ne!(a, b);
    }

    /// SD-32 Epic 5 protective sweep, mirrors `cache_gen::acg`'s identical
    /// finding: `write_json` clobbered an already-enriched file with no
    /// exists-guard. 622 of 670 `advanced_players_guide` spell/equipment
    /// on-disk records carry a `raw_tokens` field
    /// `enrich_spell_raw_tokens.rs`/`enrich_equipment_raw_tokens.rs` write
    /// AFTER this generator runs, which this generator's own
    /// `SpellData`/`EquipmentData` cannot reconstruct.
    #[test]
    fn write_json_never_overwrites_an_existing_file() {
        let dir = std::env::temp_dir().join(format!(
            "apg_write_json_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("foo.json");
        std::fs::write(&path, r#"{"data":{"key":"foo"},"raw_tokens":["ENRICHED-MARKER"]}"#).unwrap();

        let record = CacheRecord {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: "2026-08-22T00:00:00Z".to_string(),
            data: SpellData {
                key: "foo".to_string(),
                school: Some("Evocation".to_string()),
                level: Some(1),
                description: None,
                full_text: false,
            },
            source: Source::LstToken {
                path: "apg_spells.lst".to_string(),
                sha256: "deadbeef".to_string(),
                line: 1,
                record_key: "foo".to_string(),
            },
            wiring_class: "display".to_string(),
            wiring_class_signals: vec!["display".to_string()],
            license: crate::rules_core::shape_b_v1::License::Ogl,
            pi_field: None,
            pi_marker: None,
            codex_generated_name: false,
        };
        write_json(&dir, "foo", &record).expect("write_json must succeed");

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(
            content.contains("ENRICHED-MARKER"),
            "write_json clobbered a file a later enrichment pass had already written into: {content}"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
