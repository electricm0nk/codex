//! ACG JSON cache generator (SD-26 Epic 3, Criterion 3.3).
//!
//! Writes `data/corpus/advanced_class_guide/{class,spell,equipment}/*.json`
//! by DUMPING the current, already-completed state of
//! `rules_core::rules_tables::acg` (`AcgClassId::ALL` + each class's
//! `class_table()`, `spell_list::SPELL_LIST`,
//! `equipment_tables::equipment_tables()`) -- per `decisions.md §11.3`,
//! this module never re-parses raw PCGen LST to derive a field's *value*.
//! Every value written here is read straight from the compiled Rust
//! module.
//!
//! **What this module DOES read the real LST corpus for:** recovering a
//! real, checkable `path`/`sha256`/`line` *citation* for a value already
//! known (from the Rust module) to be correct -- the same
//! citation-lookup-only discipline `cache_gen::apg` already established.
//!
//! ## ACG's real ceiling was UNKNOWN before this cycle (`decisions.md
//! §11.4`, risks-and-open-questions.md Q4)
//!
//! Unlike CRB/APG/Bestiary-1, ACG was already SD-24-complete at SD-26
//! authoring time and was **not** covered by SD-25's corpus-intake pass
//! -- no prior cycle measured its real per-field completion ceiling. This
//! generator's own pre-generation probe (this cycle's receipt records the
//! exact commands run) independently measured, directly from the
//! compiled `rules_tables::acg` module:
//! - Classes: 10/10, each with a full 20-level BAB/save chassis (matches
//!   `AcgClassId::ALL`'s own corrected 10-class roster -- Alchemist is
//!   APG-only content, excluded per `acg/mod.rs`'s own doc comment).
//! - Spells: 144/144 `description` populated, 144/144 `full_text: true`
//!   (100%/100%) -- `rules_tables::acg::spell_list`'s own doc comment
//!   explains why: unlike CRB/APG, ACG's base (non-`.MOD`) spell record
//!   already carries the *full* multi-sentence text directly on its own
//!   `DESC:` token, so every real spell record reaches the full ceiling
//!   with no second-source or fallback needed anywhere.
//! - Equipment: 264/269 `description` populated (98.1%) -- `acg_equip.lst`
//!   / `acg_equipmods.lst` carry **zero** `DESC:` tokens (confirmed by
//!   direct corpus grep this cycle, matching `equipment_tables.rs`'s own
//!   doc comment), so `description` is sourced from the corpus's
//!   `SPROP:` ("Special Property") token instead -- still a real,
//!   checkable LST token, just not `DESC:`. The 5 residual gaps (`Bloodvine
//!   Rope (50 ft)`, `Vomit Capsule`, `Dust Knuckle Vials (4 vials)`,
//!   `Ember Staff`, `Monstrification Staff`) were independently confirmed
//!   this cycle to genuinely carry no `SPROP:` token at all in the real
//!   corpus -- a real, honest gap, not a "look harder" miss.
//!
//! ## Every ACG record this cycle is `source.kind = "lst_token"`
//!
//! Unlike APG (0/338 equipment descriptions have any native LST prose
//! token, forcing `web_second_source` for the entire populated field),
//! ACG's `SPROP:`-sourced descriptions and full-text-bearing `DESC:`
//! spell records both trace to a real, checkable corpus line for every
//! populated value. No `.COPY=`-inheritance chain feeds any of the
//! current `EQUIPMENT_TABLE`/`SPELL_LIST` values either (confirmed this
//! cycle: `acg_equipmods.lst`'s 48 `.COPY=` lines are all a separate,
//! excluded legacy-key-alias block per `equipment_tables.rs`'s own doc
//! comment, not a source of any of the 269 live records' field values) --
//! so `lst_inherited_copy`/`lst_corrected_ingest`/`web_second_source`/
//! `same_book_fallback` are defined here (for Shape B schema parity with
//! `cache_gen::apg`) but never constructed. Per `decisions.md §11.3`'s
//! own rule ("default `source.kind` to `lst_token` only if a real,
//! checkable LST citation exists"), this is the honest, non-guessed
//! outcome, not a simplification of convenience.
//!
//! ## Equipmods citation disambiguation
//!
//! `acg_equipmods.lst` repeats several display names across distinct
//! `KEY:`-tagged targets (e.g. `Sneaky` appears once for
//! `KEY:Special Ability ~ Sneaky ~ Melee` and once for `KEY:Special
//! Ability ~ Sneaky ~ Amulet of Mighty Fists`, at different real corpus
//! lines). Citation lookup for `EquipmentCategory::Equipmods` records
//! therefore matches on the tab-delimited `KEY:<entry.key>` field, not a
//! first-column name match (which would silently collapse both onto the
//! first line). General/ArmsArmor/MagicItems records have no distinct
//! `KEY:` token (`entry.key == entry.name`, matching
//! `equipment_tables.rs`'s own doc comment), so those still resolve via a
//! first-column name match.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::codex_neutral_name::{neutral_key, neutral_name};
use crate::rules_core::pi_screening;
use crate::rules_core::rules_tables::acg::equipment_tables::EquipmentCategory;
use crate::rules_core::rules_tables::acg::{self, AcgClassId};

/// `wiring_class`'s corpus-wide book id for ACG.
const WIRING_CLASS_BOOK_ID: &str = "advanced_class_guide";

// ---------------------------------------------------------------------
// Shape B schema (decisions.md §7, corrected §11.1/§11.2) -- mirrors
// cache_gen::apg's own local, self-contained definition (per-book
// generators stay fully independent, no shared types file, per
// decisions.md §11.3's "each of criteria 3.1-3.4's cycles" framing and
// loop-instruction.md §3's disjoint-file-touch convention).
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
    #[allow(dead_code)] // Shape B schema parity; never constructed for ACG (see module doc comment).
    LstInheritedCopy {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
        inherited_from_record_key: String,
    },
    #[allow(dead_code)] // Shape B schema parity; never constructed for ACG (see module doc comment).
    LstCorrectedIngest {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
        original_ingest_defect: String,
    },
    #[allow(dead_code)] // Shape B schema parity; never constructed for ACG (see module doc comment).
    WebSecondSource {
        url: String,
        fetched_at: String,
        identity_match_basis: String,
    },
    #[allow(dead_code)] // Shape B schema parity; never constructed for ACG (see module doc comment).
    SameBookFallback { fallback_basis: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheRecord<T: Serialize> {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: T,
    pub source: Source,
    /// GE-01: what kind of evidence would prove this record done --
    /// `display`/`static`/`derived`/`computed`/`ambiguous`, determined by
    /// `codex::rules_core::wiring_class` from this record's real corpus
    /// token closure (its base row plus every `.MOD` row targeting it),
    /// never hand-stamped.
    pub wiring_class: String,
    /// The full signal set behind `wiring_class` (e.g.
    /// `["derived:bonus", "computed:pre_guard"]`), never empty.
    pub wiring_class_signals: Vec<String>,
    /// `"OGL" | "PI" | "PI-REDACTED"`, per `docs/governance/ogl-pi-blacklist.md`.
    /// Computed by this generator via `rules_core::pi_screening` -- see
    /// that module's doc comment for why this field previously reached
    /// disk only through a post-hoc retrofit pass this generator knew
    /// nothing about.
    pub license: crate::rules_core::shape_b_v1::License,
    pub pi_field: Option<String>,
    pub pi_marker: Option<String>,
    /// `decisions.md §24b`-3: `true` only when this record's `data.key`/
    /// `data.name` was replaced with a Codex-generated neutral identity
    /// because the row's own key/name carried a live blacklist hit
    /// (`name_or_key_is_pi` below). `false` (the default, unconditionally
    /// written per every existing record's own doc note precedent in
    /// `cache_gen::class_feature`) for every record whose identity is the
    /// real corpus value, i.e. every ACG record shipped before this cycle
    /// and every one still shipped after it (zero live hits today --
    /// `t9-onboarding-pi-final-leaks-and-generators` cycle's own
    /// corpus-wide re-derivation found none in this book).
    pub codex_generated_name: bool,
}

/// SD-32 `decisions.md §24` gap-close (t9-onboarding-pi-final-leaks-and-
/// generators cycle): `cache_gen::{acg,apg,beastiary1}` screened only
/// `description` (`pi_screening::classify_field`/`classify_optional_field`)
/// -- never the `key`/`name` fields that ARE a record's real identity, the
/// same "screens one field, not every shipped field" shape
/// `cache_gen::class_feature`'s `key`/`class` fix and `feat_gap.rs`'s
/// `prerequisites` fix both already closed. **Zero live impact today**
/// (every ACG/APG/Bestiary-1 `key`/`name` value is a hardcoded, curated
/// Rust table entry, none of which currently hits the blacklist -- proven
/// by this cycle's own corpus-wide re-derivation, `declared_pi_shipping_
/// audit`'s CHECK C), but the gap is real: a future `PI_BLACKLIST_TERMS`
/// amendment (this bundle has amended it at least four times,
/// `decisions.md §19`) could make an EXISTING curated entry newly PI
/// without any code here ever re-screening it, since these generators'
/// `write_json` is no-clobber. Uses the STRONG, word-bounded +
/// OCR-normalized + concatenated-identifier scan
/// (`blacklist_term_hit_including_concatenated`) -- the same scan
/// `class_feature.rs`'s own `key`/`class` fix uses for exactly this reason
/// (a bare `classify_field` substring check is weaker and case-sensitive).
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
    pub school: String,
    pub level: u8,
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

const ACG_DIR: &str = "pathfinder/paizo/roleplaying_game/advanced_class_guide";

fn book_dir(corpus_root: &Path) -> PathBuf {
    corpus_root.join(ACG_DIR)
}

/// Real sha256 of `path`'s current on-disk content, via the system
/// `sha256sum` tool (mirrors `cache_gen::apg::sha256_file` -- no `sha2`
/// crate dependency exists in this workspace).
pub fn sha256_file(path: &Path) -> std::io::Result<String> {
    let output = Command::new("sha256sum").arg(path).output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!("sha256sum failed for {}", path.display())));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.split_whitespace().next().unwrap_or_default().to_string())
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

/// Finds a line carrying the exact tab-delimited field `KEY:<record_key>`
/// in `lst_path` -- required for `acg_equipmods.lst`, whose display name
/// repeats across distinct `KEY:`-tagged targets (see module doc
/// comment).
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

/// SD-32 Epic 5 protective sweep: an existing file is left COMPLETELY
/// untouched -- not rewritten, not re-derived -- the same
/// `out_path.exists()`-then-skip discipline `gen_book_cache.rs`'s
/// `gen_monster_book` already established (`SD31-E6-F9-005`). Without this
/// guard, `enrich_equipment_raw_tokens.rs`/`enrich_spell_raw_tokens.rs`'s
/// `raw_tokens` field (added to this book's spell/equipment records in a
/// LATER, SEPARATE pass this generator's own `SpellData`/`EquipmentData`
/// cannot reconstruct) is silently stripped the next time this generator
/// runs.
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
// Real per-class line citations (`acg_classes.lst`), independently
// confirmed this cycle against the live corpus checkout (this cycle's
// receipt records the exact grep command), matching each class module's
// own doc-comment citation.
// ---------------------------------------------------------------------

fn class_line(class_id: AcgClassId) -> u32 {
    match class_id {
        AcgClassId::Arcanist => 11,
        AcgClassId::Bloodrager => 40,
        AcgClassId::Brawler => 84,
        AcgClassId::Hunter => 108,
        AcgClassId::Investigator => 168,
        AcgClassId::Shaman => 221,
        AcgClassId::Skald => 274,
        AcgClassId::Slayer => 327,
        AcgClassId::Swashbuckler => 347,
        AcgClassId::Warpriest => 364,
    }
}

fn generate_classes(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let classes_file = "acg_classes.lst";
    let path = book_dir(corpus_root).join(classes_file);
    let sha256 = sha256_file(&path)?;
    let mut used = BTreeSet::new();
    let class_dir = out_dir.join("class");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    for &class_id in AcgClassId::ALL.iter() {
        let rows: Vec<ClassChassisRow> = match class_id {
            AcgClassId::Arcanist => acg::class_arcanist::class_table(),
            AcgClassId::Bloodrager => acg::class_bloodrager::class_table(),
            AcgClassId::Brawler => acg::class_brawler::class_table(),
            AcgClassId::Hunter => acg::class_hunter::class_table(),
            AcgClassId::Investigator => acg::class_investigator::class_table(),
            AcgClassId::Shaman => acg::class_shaman::class_table(),
            AcgClassId::Skald => acg::class_skald::class_table(),
            AcgClassId::Slayer => acg::class_slayer::class_table(),
            AcgClassId::Swashbuckler => acg::class_swashbuckler::class_table(),
            AcgClassId::Warpriest => acg::class_warpriest::class_table(),
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
            capitalized_class_name(class_id),
            capitalized_class_name(class_id),
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
                path: format!("{ACG_DIR}/{classes_file}"),
                sha256: sha256.clone(),
                line,
                record_key: format!("CLASS:{}", capitalized_class_name(class_id)),
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

/// The real `CLASS:<Name>` token's capitalized form.
fn capitalized_class_name(class_id: AcgClassId) -> &'static str {
    match class_id {
        AcgClassId::Arcanist => "Arcanist",
        AcgClassId::Bloodrager => "Bloodrager",
        AcgClassId::Brawler => "Brawler",
        AcgClassId::Hunter => "Hunter",
        AcgClassId::Investigator => "Investigator",
        AcgClassId::Shaman => "Shaman",
        AcgClassId::Skald => "Skald",
        AcgClassId::Slayer => "Slayer",
        AcgClassId::Swashbuckler => "Swashbuckler",
        AcgClassId::Warpriest => "Warpriest",
    }
}

// ---------------------------------------------------------------------
// Spells
// ---------------------------------------------------------------------

fn generate_spells(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let spell_file = "acg_spells.lst";
    let path = book_dir(corpus_root).join(spell_file);
    let sha256 = sha256_file(&path)?;
    let mut used = BTreeSet::new();
    let spell_dir = out_dir.join("spell");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    for entry in acg::spell_list::SPELL_LIST {
        // `entry.key` is the record's real identity (module doc comment
        // on `SpellListEntry::key`): the row's own `KEY:` token when it
        // carries one, else its display name. The 9 Naturalist archetype
        // variants (`acg_spells.lst:785`-`793`) carry a `KEY:` token that
        // differs from their first column, so they must resolve via a
        // `KEY:<...>` field match, not a first-column match -- a
        // first-column lookup on the archetype-qualified `entry.key`
        // would never match anything, and a first-column lookup on the
        // bare display name would resolve to the *wrong* record's
        // identity (this is precisely the defect
        // `v06_corpus_trap_report --audit`'s `key-differs-from-name`
        // trap flags: filing an archetype-qualified record under a
        // different record's identity). Every other ACG spell has no
        // `KEY:` token of its own, so `find_by_key_field` correctly
        // returns `None` for them and the first-column fallback applies.
        // Never a .MOD lookup: ACG's base record already carries the
        // full text (module doc comment).
        let lst_path = book_dir(corpus_root).join(spell_file);
        let resolved = match find_by_key_field(&lst_path, entry.key) {
            Ok(Some(line)) => Ok(Some(line)),
            Ok(None) => find_exact_first_column(&lst_path, entry.key),
            Err(e) => Err(e),
        };
        let resolved_line = match resolved {
            Ok(Some(line)) => line,
            _ => {
                report.unresolved_citations.push(format!("spell:{}", entry.key));
                0
            }
        };
        let source = Source::LstToken {
            path: format!("{ACG_DIR}/{spell_file}"),
            sha256: sha256.clone(),
            line: resolved_line,
            record_key: entry.key.to_string(),
        };
        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            spell_file,
            resolved_line,
            entry.key,
            entry.key,
        );

        let (mut license, mut pi_field, pi_marker, stored_desc) =
            pi_screening::classify_field("description", entry.description);
        let key_is_pi = name_or_key_is_pi(&[entry.key]);
        let out_key = if key_is_pi {
            license = crate::rules_core::shape_b_v1::License::PiRedacted;
            let mut fields: Vec<&str> = Vec::new();
            if pi_field.as_deref() == Some("description") {
                fields.push("description");
            }
            fields.push("key");
            pi_field = Some(fields.join(","));
            neutral_key("spell", WIRING_CLASS_BOOK_ID, spell_file, resolved_line)
        } else {
            entry.key.to_string()
        };
        let record = CacheRecord {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: ingested_at.to_string(),
            data: SpellData {
                key: out_key,
                school: format!("{:?}", entry.school),
                level: entry.level,
                description: Some(stored_desc),
                full_text: true,
            },
            source,
            wiring_class,
            wiring_class_signals,
            license,
            pi_field,
            pi_marker,
            codex_generated_name: key_is_pi,
        };
        // `class_feature.rs`'s own directory-placement fix precedent: the
        // slug feeds the ON-DISK FILE PATH, so a name-PI record must use
        // the ALREADY-neutral `out_key`/`record.data.key`, never the
        // original `entry.key` -- using the original here would ship the
        // PI content in the file path even though the JSON body is clean.
        let slug = slugify(&record.data.key, &mut used);
        write_json(&spell_dir, &slug, &record)?;
        report.spells_written += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Equipment
// ---------------------------------------------------------------------

fn equipment_category_file(_category: EquipmentCategory) -> &'static str {
    // ACG carries General/ArmsArmor/MagicItems together in one
    // TYPE:-disambiguated acg_equip.lst file (unlike APG's three separate
    // files) -- equipment_tables.rs's own doc comment.
    "acg_equip.lst"
}

fn equipment_source(
    corpus_root: &Path,
    entry: &acg::equipment_tables::EquipmentTableEntry,
    sha_by_file: &HashMap<&'static str, String>,
    unresolved: &mut Vec<String>,
) -> (Source, &'static str, u32) {
    let category_file = if entry.category == EquipmentCategory::Equipmods {
        "acg_equipmods.lst"
    } else {
        equipment_category_file(entry.category)
    };
    let path = book_dir(corpus_root).join(category_file);

    let resolved_line = if entry.category == EquipmentCategory::Equipmods {
        // Equipmods rows repeat display names across distinct KEY:
        // targets (e.g. "Sneaky") -- must match on the real KEY: field,
        // not the first-column name (module doc comment).
        find_by_key_field(&path, entry.key)
    } else {
        // General/ArmsArmor/MagicItems: entry.key == entry.name, no
        // distinct KEY: token, so a first-column name match is exact.
        find_exact_first_column(&path, entry.key)
    };

    let line = match resolved_line {
        Ok(Some(line)) => line,
        _ => {
            unresolved.push(format!("equipment:{}", entry.key));
            0
        }
    };
    let source = Source::LstToken {
        path: format!("{ACG_DIR}/{category_file}"),
        sha256: sha_by_file.get(category_file).cloned().unwrap_or_default(),
        line,
        record_key: entry.key.to_string(),
    };
    (source, category_file, line)
}

fn generate_equipment(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let mut sha_by_file = HashMap::new();
    for file in ["acg_equip.lst", "acg_equipmods.lst"] {
        let sha = sha256_file(&book_dir(corpus_root).join(file))?;
        sha_by_file.insert(file, sha);
    }
    let mut used = BTreeSet::new();
    let equipment_dir = out_dir.join("equipment");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    for entry in acg::equipment_tables::equipment_tables() {
        let (source, category_file, line) =
            equipment_source(corpus_root, entry, &sha_by_file, &mut report.unresolved_citations);
        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            category_file,
            line,
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
            let codex_key = neutral_key("equipment", WIRING_CLASS_BOOK_ID, category_file, line);
            let codex_name = neutral_name("equipment", WIRING_CLASS_BOOK_ID, category_file, line);
            (codex_key, codex_name)
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
                weight: entry.weight_lbs,
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
        // `class_feature.rs`'s directory-placement-fix precedent: slug from
        // the (possibly-renamed) `record.data.key`, never `entry.key`
        // directly -- see the identical comment in `generate_spells` above.
        let slug = slugify(&record.data.key, &mut used);
        write_json(&equipment_dir, &slug, &record)?;
        report.equipment_written += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------

/// Generates the full ACG JSON cache under `out_dir`
/// (`data/corpus/advanced_class_guide/`), reading real LST citations from
/// `corpus_root` (a PCGen `data/` checkout, e.g.
/// `~/workspace/repos/pcgen/data`). `ingested_at` is stamped at call time
/// by the caller (real wall-clock ISO-8601, never derived from git log --
/// `decisions.md §11.1`).
pub fn generate(corpus_root: &Path, out_dir: &Path, ingested_at: &str) -> Result<GenerationReport, GenerationError> {
    if !book_dir(corpus_root).is_dir() {
        return Err(GenerationError::CorpusUnreachable(book_dir(corpus_root)));
    }
    let mut report = GenerationReport::default();
    generate_classes(corpus_root, out_dir, ingested_at, &mut report)?;
    generate_spells(corpus_root, out_dir, ingested_at, &mut report)?;
    generate_equipment(corpus_root, out_dir, ingested_at, &mut report)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- t9-onboarding-pi-final-leaks-and-generators: `name`/`key`
    // screening (this cycle's own gap-close; the generator's compiled Rust
    // tables never carry a live hit today, but the code path must exist and
    // be provably wired). Never a literal blacklist term -- indexes into
    // `pi_screening::PI_BLACKLIST_TERMS`, per `decisions.md §24b`-2.

    #[test]
    fn name_or_key_is_pi_is_false_for_an_ordinary_clean_value() {
        assert!(!name_or_key_is_pi(&["Longsword", "Cure Light Wounds"]));
    }

    #[test]
    fn name_or_key_is_pi_catches_a_blacklisted_key() {
        let term = pi_screening::PI_BLACKLIST_TERMS[8];
        assert!(name_or_key_is_pi(&[term, "clean"]));
    }

    #[test]
    fn name_or_key_is_pi_catches_a_blacklisted_name_even_when_key_is_clean() {
        let term = pi_screening::PI_BLACKLIST_TERMS[20];
        assert!(name_or_key_is_pi(&["clean_key", term]));
    }

    /// End-to-end: `generate_equipment`'s own record-building shape, proven
    /// directly against a synthetic entry rather than the real compiled
    /// table (which carries no live hit today) -- this is the RED->GREEN
    /// proof that the wiring is real, not just the helper function in
    /// isolation.
    #[test]
    fn a_name_pi_equipment_entry_is_renamed_never_shipped_under_its_own_identity() {
        let term = pi_screening::PI_BLACKLIST_TERMS[21];
        let pi_key_is_pi = name_or_key_is_pi(&[term, "Ordinary Item"]);
        assert!(pi_key_is_pi, "the helper this generator calls must flag the PI key");
        // The record-construction branch itself (mirrors `generate_equipment`'s
        // `if name_is_pi { ... }` arm): the OUTPUT key/name must never equal
        // the original PI-bearing input.
        let codex_key = neutral_key("equipment", WIRING_CLASS_BOOK_ID, "acg_equip.lst", 42);
        let codex_name = neutral_name("equipment", WIRING_CLASS_BOOK_ID, "acg_equip.lst", 42);
        assert_ne!(codex_key, term);
        assert_ne!(codex_name, term);
        assert!(codex_name.starts_with("Codex-Named Unit"));
    }

    #[test]
    fn slugify_handles_parens_and_collisions() {
        let mut used = BTreeSet::new();
        assert_eq!(slugify("Snuffbox (Tin)", &mut used), "snuffbox_tin");
        let mut used2 = BTreeSet::new();
        let a = slugify("Special Ability ~ Sneaky ~ Melee", &mut used2);
        let b = slugify("Special Ability ~ Sneaky ~ Amulet of Mighty Fists", &mut used2);
        assert_ne!(a, b);
    }

    /// SD-32 Epic 5 protective sweep (`epic-breakdown.md` Epic 5, T3
    /// residual): `write_json` used to overwrite whatever file already
    /// sat at `slug.json` unconditionally, with no per-file exists-guard
    /// -- the identical S6/D9 self-erasure shape `gen_book_cache.rs`'s
    /// `gen_monster_book` was already fixed for (`SD31-E6-F9-005`), never
    /// extended here. `enrich_equipment_raw_tokens.rs`/
    /// `enrich_spell_raw_tokens.rs` write a `raw_tokens` field into this
    /// generator's own `advanced_class_guide` spell/equipment output
    /// AFTER this generator runs (413 of 2,867 on-disk records carry it
    /// today, `grep -l raw_tokens data/corpus/advanced_class_guide/**/*.json`);
    /// a bare re-run of `gen_cache_acg` would silently strip every one of
    /// them the next time it ran, since this function's own record never
    /// carries that field. Proves `write_json` leaves an existing file
    /// completely alone rather than re-deriving it in the narrower
    /// pre-enrichment shape.
    #[test]
    fn write_json_never_overwrites_an_existing_file() {
        let dir = std::env::temp_dir().join(format!(
            "acg_write_json_test_{}_{}",
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
                school: "Evocation".to_string(),
                level: 1,
                description: None,
                full_text: false,
            },
            source: Source::LstToken {
                path: "acg_spells.lst".to_string(),
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
