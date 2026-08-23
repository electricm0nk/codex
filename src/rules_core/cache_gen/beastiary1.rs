//! Bestiary 1 JSON cache generator (SD-26 Epic 3, Criterion 3.4).
//!
//! Writes `data/corpus/beastiary/{monster,equipment}/*.json` by DUMPING
//! the current, already-completed state of
//! `rules_core::rules_tables::beastiary1` (`MonsterId::ALL` +
//! `monster_resolve()`, `equipment_tables::EQUIPMENT_TABLE`) -- per
//! `decisions.md §11.3`, this module never re-parses raw PCGen LST to
//! derive a field's *value*. Every value written here is read straight
//! from the compiled Rust module.
//!
//! **What this module DOES read the real LST corpus for:** recovering a
//! real, checkable `path`/`sha256`/`line` *citation* for a value already
//! known (from the Rust module) to be correct -- the same
//! citation-lookup-only discipline `cache_gen::apg`/`cache_gen::acg`
//! already established.
//!
//! ## `MonsterId::ALL` (register from `decisions.md §11.6`)
//!
//! `beastiary1::mod.rs` had no public `ALL`/count constant before this
//! cycle, unlike `ClassId::ALL`/`ApgClassId::ALL`/`AcgClassId::ALL` on
//! the other three books --
//! `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` had to
//! hand-maintain its own duplicate 41-entry workaround list
//! (`ALL_BESTIARY1_MONSTERS`) to work around the gap. This cycle adds the
//! real `MonsterId::ALL` constant to `beastiary1::mod.rs` itself (a
//! genuine, in-scope code fix per `decisions.md §11.6`, not scope creep)
//! and this generator uses it directly -- it does not duplicate the
//! workaround a third time. `corpus_ingest_diagnostic.rs` was also
//! updated to read the same real constant instead of its own copy.
//!
//! ## No spell-list concept exists for this book (register A13, carried
//! forward from `equipment_tables.rs`'s own doc comment)
//!
//! Confirmed directly against the live corpus this cycle (see this
//! cycle's receipt): no `b1_spells.lst` (or any `*spell*`-named file)
//! exists under
//! `pathfinder/paizo/roleplaying_game/bestiary/`. "Spells" is correctly
//! N/A for this book, not an unclaimed gap -- this generator only writes
//! `monster/` and `equipment/` directories.
//!
//! ## Real, measured coverage ceilings this cycle re-verified
//! (`decisions.md §11.4`; raised from 41/41 to 46/46 by SD28-E16 subset
//! 09, 2026-08-07)
//!
//! - **Monsters: 46/46** real, corrected-roster stat blocks
//!   (`MonsterId::ALL`), each with the full set of fields this book's
//!   `MonsterStatBlock` schema tracks (name/CR/size/speed/race
//!   type+subtype/source page/natural attacks -- AC/HP/saves are
//!   deliberately out of scope per `beastiary1::MonsterStatBlock`'s own
//!   doc comment, so `completeness: chassis_only` mirrors
//!   `cache_gen::acg`'s own class-chassis usage of the same enum value
//!   for the same "deliberately bounded, not a partial-data gap" reason).
//! - **Equipment: 4/4 (100%)** -- independently re-counted directly
//!   against the live corpus this cycle (`b1_equip_general.lst` ×1,
//!   `b1_equip_arms_armor.lst` ×2, `b1_equip_magic_items.lst` ×1), **not**
//!   the ~7 a prior SD-25 cycle-doc estimate assumed (`decisions.md
//!   §11.4`'s own correction note). 3 of the 4 records source
//!   `description` from the corpus's `SPROP:` ("Special Property") token
//!   -- a real, checkable LST token, just not `DESC:` (register A10,
//!   same convention `cache_gen::acg` already established) -- and 1
//!   (`Rag Armor (Dark Creeper)`) has neither `DESC:` nor `SPROP:` and is
//!   `web_second_source`-sourced (SD-25's own web pass, carried forward
//!   verbatim -- see `equipment_data.rs`'s doc comment for the exact
//!   URLs/fetch date/identity-match basis).

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::codex_neutral_name::{neutral_key, neutral_name};
use crate::rules_core::pi_screening;
use crate::rules_core::rules_tables::beastiary1::equipment_tables::EquipmentTableEntry;
use crate::rules_core::rules_tables::beastiary1::natural_attack_provenance::{
    self, AttackSource as ProvenanceSource,
};
use crate::rules_core::rules_tables::beastiary1::{self, MonsterId, MonsterStatBlock, NaturalAttack};
use crate::rules_core::rules_tables::RuleSetId;

// ---------------------------------------------------------------------
// Shape B schema (decisions.md §7, corrected §11.1/§11.2) -- mirrors
// cache_gen::apg's/cache_gen::acg's own local, self-contained
// definitions (per-book generators stay fully independent, no shared
// types file, per decisions.md §11.3's "each of criteria 3.1-3.4's
// cycles" framing and loop-instruction.md §3's disjoint-file-touch
// convention).
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
    #[allow(dead_code)] // Shape B schema parity; never constructed for Bestiary 1 (see module doc comment).
    LstInheritedCopy {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
        inherited_from_record_key: String,
    },
    #[allow(dead_code)] // Shape B schema parity; never constructed for Bestiary 1 (see module doc comment).
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
    },
    #[allow(dead_code)] // Shape B schema parity; never constructed for Bestiary 1 (see module doc comment).
    SameBookFallback { fallback_basis: String },
}

/// Provenance for a **single field**, where the record-level [`Source`]
/// cannot tell the whole truth.
///
/// `decisions.md §11.2`'s discriminated union is a *record*-level shape:
/// it assumes one provenance kind per record. Twelve Bestiary 1 monsters
/// break that assumption — their chassis fields and their attack *names*
/// really do come from their own `b1_races.lst` row (`lst_token`), but
/// their attack *damage dice* do not exist anywhere in the corpus and are
/// grounded from published values instead.
///
/// Emitting the whole record as `web_second_source` would be wrong (and
/// would misattribute the CR/size/speed/type/page fields); leaving it as
/// a bare `lst_token` would silently imply the dice came from that line.
/// `§11.3` anticipated exactly this and directs the generator to "flag
/// anything it can't confidently attribute rather than guessing" — this
/// field is that flag. The record-level `source` stays truthfully
/// `lst_token`, and this array narrows the claim for the affected
/// fields.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FieldSource {
    LstToken {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
    },
    WebSecondSource {
        /// At least two independent agreeing sources, allowed domains
        /// only (`§11.5`).
        urls: Vec<String>,
        fetched_at: String,
        identity_match_basis: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldProvenance {
    /// JSON pointer-ish path of the field this describes, e.g.
    /// `natural_attacks[1].damage_dice`.
    pub field: String,
    /// The value at `field`, duplicated so the citation is readable
    /// without cross-indexing.
    pub value: String,
    /// The real token on the monster's own row that *names* this attack
    /// (only the dice needed grounding).
    pub corpus_name_token: String,
    /// Verbatim published "Melee" text the dice were read from.
    pub published_melee_text: String,
    pub source: FieldSource,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheRecord<T: Serialize> {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: T,
    pub source: Source,
    /// Present only on records where some field's provenance differs
    /// from the record-level `source` (the 12 grounded monsters). Absent
    /// everywhere else, so equipment records and the 29 fully
    /// corpus-transcribed monsters are byte-identical to before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_provenance: Option<Vec<FieldProvenance>>,
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

/// `cache_gen::acg::name_or_key_is_pi`'s byte-identical sibling for
/// Bestiary 1's `equipment` records -- same gap, same fix,
/// `t9-onboarding-pi-final-leaks-and-generators` cycle. Zero live impact
/// today (this cycle's own corpus-wide re-derivation found no hit in this
/// book's 4 equipment records). `MonsterData.name` is a related, NOT-fixed
/// gap this cycle's own receipt names explicitly (its `blanket_ogl()` path
/// screens no field at all, not even `description`) -- see this cycle's
/// receipt's generator-audit table.
fn name_or_key_is_pi(values: &[&str]) -> bool {
    values.iter().any(|v| pi_screening::blacklist_term_hit_including_concatenated(v).is_some())
}

// ---------------------------------------------------------------------
// Content-kind data shapes
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct NaturalAttackData {
    pub name: String,
    pub damage_dice: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterData {
    /// Canonical `beastiary1:monster:<slug>` key, matching
    /// `beastiary1::monster_key_resolve`'s own key shape.
    pub id: String,
    pub name: String,
    pub challenge_rating: f32,
    pub size: String,
    pub speed_ft: u32,
    pub race_type: String,
    pub race_subtype: Option<String>,
    pub source_page: String,
    pub natural_attacks: Vec<NaturalAttackData>,
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

const BOOK_DIR: &str = "pathfinder/paizo/roleplaying_game/bestiary";
/// `wiring_class`'s corpus-wide book id for Bestiary 1 -- the directory
/// basename of `BOOK_DIR`, matching `v06_work_inventory`'s own book id for
/// this book (its `RuleSetId` is `bestiary_1`, a different namespace).
const WIRING_CLASS_BOOK_ID: &str = "bestiary";
const MONSTERS_FILE: &str = "b1_races.lst";
/// Carries the `Crocodile ~ Tail Slap` record, the one cross-file
/// `NATURALATTACKS:` token this book's grounded attacks recover from the
/// real corpus (see `rules_tables::beastiary1::natural_attack_provenance`).
const RACE_ABILITIES_FILE: &str = "b1_abilities_race.lst";

fn book_dir(corpus_root: &Path) -> PathBuf {
    corpus_root.join(BOOK_DIR)
}

/// Real sha256 of `path`'s current on-disk content, via the system
/// `sha256sum` tool (mirrors `cache_gen::apg`/`cache_gen::acg`'s own
/// `sha256_file` -- no `sha2` crate dependency exists in this
/// workspace).
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

// ---------------------------------------------------------------------
// Generation report
// ---------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub monsters_written: usize,
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

fn slugify(name: &str) -> String {
    let mut slug: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    while slug.contains("__") {
        slug = slug.replace("__", "_");
    }
    let slug = slug.trim_matches('_').to_string();
    if slug.is_empty() {
        "unnamed".to_string()
    } else {
        slug
    }
}

// ---------------------------------------------------------------------
// Monsters
// ---------------------------------------------------------------------

fn generate_monsters(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let path = book_dir(corpus_root).join(MONSTERS_FILE);
    let sha256 = sha256_file(&path)?;
    let monster_dir = out_dir.join("monster");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    // Hashed once up front: only the Crocodile Tail Slap provenance row
    // cites it, but the citation must carry a real, checkable sha256
    // like every other `lst_token` citation in this cache.
    let race_abilities_path = book_dir(corpus_root).join(RACE_ABILITIES_FILE);
    let race_abilities_sha256 = sha256_file(&race_abilities_path)?;

    for &monster_id in MonsterId::ALL {
        let stat_block: MonsterStatBlock = beastiary1::monster_resolve(monster_id, RuleSetId::Bestiary1)
            .unwrap_or_else(|| panic!("{monster_id:?}: MonsterId::ALL must resolve for RuleSetId::Bestiary1"));

        let resolved_line = find_exact_first_column(&path, &stat_block.name).ok().flatten();
        let source = match resolved_line {
            Some(line) => Source::LstToken {
                path: format!("{BOOK_DIR}/{MONSTERS_FILE}"),
                sha256: sha256.clone(),
                line,
                record_key: stat_block.name.clone(),
            },
            None => {
                report.unresolved_citations.push(format!("monster:{}", stat_block.name));
                Source::LstToken {
                    path: format!("{BOOK_DIR}/{MONSTERS_FILE}"),
                    sha256: sha256.clone(),
                    line: 0,
                    record_key: stat_block.name.clone(),
                }
            }
        };
        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            MONSTERS_FILE,
            resolved_line.unwrap_or(0),
            &stat_block.name,
            &stat_block.name,
        );

        let slug = slugify(&stat_block.name);
        let natural_attacks: Vec<NaturalAttackData> = stat_block
            .natural_attacks
            .iter()
            .map(|a: &NaturalAttack| NaturalAttackData { name: a.name.clone(), damage_dice: a.damage_dice.clone() })
            .collect();

        // Narrow the provenance claim for any attack whose dice are not
        // transcribed from this monster's own row. Built from the same
        // `natural_attack_provenance` table the shipped Rust tables and
        // `tests/v06_beastiary1_natural_attack_grounding.rs` use, so the
        // JSON citation cannot drift from the value it describes.
        let monster_key = format!("beastiary1:monster:{slug}");
        let grounded = natural_attack_provenance::provenance_for(&monster_key);
        let field_provenance: Option<Vec<FieldProvenance>> = if grounded.is_empty() {
            None
        } else {
            Some(
                grounded
                    .iter()
                    .filter_map(|g| {
                        let index = stat_block.natural_attacks.iter().position(|a| a.name == g.attack_name)?;
                        let source = match g.source {
                            ProvenanceSource::LstToken { path: p, line, record_key } => FieldSource::LstToken {
                                path: format!("{BOOK_DIR}/{}", p.rsplit('/').next().unwrap_or(p)),
                                sha256: race_abilities_sha256.clone(),
                                line,
                                record_key: record_key.to_string(),
                            },
                            ProvenanceSource::WebSecondSource { urls, fetched_at, identity_match_basis } => {
                                FieldSource::WebSecondSource {
                                    urls: urls.iter().map(|u| (*u).to_string()).collect(),
                                    fetched_at: fetched_at.to_string(),
                                    identity_match_basis: identity_match_basis.to_string(),
                                }
                            }
                        };
                        Some(FieldProvenance {
                            field: format!("natural_attacks[{index}].damage_dice"),
                            value: g.damage_dice.to_string(),
                            corpus_name_token: g.corpus_name_token.to_string(),
                            published_melee_text: g.published_melee_text.to_string(),
                            source,
                        })
                    })
                    .collect(),
            )
        };

        let (license, pi_field, pi_marker) = pi_screening::blanket_ogl();
        let record = CacheRecord {
            population: Population::InScope,
            completeness: Completeness::ChassisOnly,
            ingested_at: ingested_at.to_string(),
            data: MonsterData {
                id: monster_key.clone(),
                name: stat_block.name.clone(),
                challenge_rating: stat_block.challenge_rating,
                size: stat_block.size.clone(),
                speed_ft: stat_block.speed_ft,
                race_type: stat_block.race_type.clone(),
                race_subtype: stat_block.race_subtype.clone(),
                source_page: stat_block.source_page.clone(),
                natural_attacks,
            },
            source,
            field_provenance,
            wiring_class,
            wiring_class_signals,
            license,
            pi_field,
            pi_marker,
            codex_generated_name: false,
        };
        write_json(&monster_dir, &slug, &record)?;
        report.monsters_written += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Equipment
// ---------------------------------------------------------------------

/// `Rag Armor (Dark Creeper)`'s real, cited web second source
/// (`equipment_data.rs`'s doc comment; SD-25's own corpus-intake pass,
/// carried forward verbatim per `decisions.md §11.3` -- this generator
/// never re-derives or re-fetches the value, only dumps it).
const RAG_ARMOR_URL: &str = "https://www.d20pfsrd.com/bestiary/monster-listings/humanoids/dark-creeper/";
const RAG_ARMOR_IDENTITY_MATCH_BASIS: &str = "name (\"Rag Armor\") + category (armor) + the corpus's own PRERACE:1,Dark Creeper qualifier + Bestiary 1 source page p.53, all matching the Dark Creeper monster's own \"Rag Armor\" (Ex) special quality";

fn equipment_source(
    corpus_root: &Path,
    entry: &EquipmentTableEntry,
    sha256: &str,
    fetched_at_web: &str,
    unresolved: &mut Vec<String>,
) -> Source {
    if entry.key == "Rag Armor (Dark Creeper)" {
        return Source::WebSecondSource {
            url: RAG_ARMOR_URL.to_string(),
            fetched_at: fetched_at_web.to_string(),
            identity_match_basis: RAG_ARMOR_IDENTITY_MATCH_BASIS.to_string(),
        };
    }

    let category_file = entry.category.corpus_file_name();
    let path = book_dir(corpus_root).join(category_file);
    match find_exact_first_column(&path, entry.key) {
        Ok(Some(line)) => Source::LstToken {
            path: format!("{BOOK_DIR}/{category_file}"),
            sha256: sha256.to_string(),
            line,
            record_key: entry.key.to_string(),
        },
        _ => {
            unresolved.push(format!("equipment:{}", entry.key));
            Source::LstToken {
                path: format!("{BOOK_DIR}/{category_file}"),
                sha256: sha256.to_string(),
                line: 0,
                record_key: entry.key.to_string(),
            }
        }
    }
}

fn generate_equipment(
    corpus_root: &Path,
    out_dir: &Path,
    ingested_at: &str,
    fetched_at_web: &str,
    report: &mut GenerationReport,
) -> Result<(), GenerationError> {
    let equipment_dir = out_dir.join("equipment");
    let wiring_index = WiringClassIndex::build(WIRING_CLASS_BOOK_ID, &book_dir(corpus_root));
    let mut wiring_lines = wiring_index.lines();

    for entry in beastiary1::equipment_tables::EQUIPMENT_TABLE {
        let category_file = entry.category.corpus_file_name();
        let sha256 = sha256_file(&book_dir(corpus_root).join(category_file))?;

        let source = equipment_source(corpus_root, entry, &sha256, fetched_at_web, &mut report.unresolved_citations);
        // Same rationale as `apg::generate_equipment`: `wiring_class` reads
        // the corpus record's own row independent of whether `source`
        // (above) is web-second-sourced (`Rag Armor (Dark Creeper)`).
        let wiring_line = find_exact_first_column(&book_dir(corpus_root).join(category_file), entry.key)
            .ok()
            .flatten()
            .unwrap_or(0);
        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            category_file,
            wiring_line,
            entry.key,
            entry.key,
        );
        let completeness = if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly };

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
                neutral_key("equipment", WIRING_CLASS_BOOK_ID, category_file, wiring_line),
                neutral_name("equipment", WIRING_CLASS_BOOK_ID, category_file, wiring_line),
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
                weight: entry.weight_lbs,
                description: stored_desc,
            },
            source,
            // Equipment provenance is fully expressible at record level
            // (each record's description has exactly one source), so no
            // record here needs the per-field narrowing.
            field_provenance: None,
            wiring_class,
            wiring_class_signals,
            license,
            pi_field,
            pi_marker,
            codex_generated_name: name_is_pi,
        };
        // `cache_gen::acg::generate_equipment`'s identical directory-
        // placement-fix precedent: slug from the (possibly-renamed) key.
        let slug = slugify(&record.data.key);
        write_json(&equipment_dir, &slug, &record)?;
        report.equipment_written += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------

/// Generates the full Bestiary 1 JSON cache under `out_dir`
/// (`data/corpus/beastiary/`), reading real LST citations from
/// `corpus_root` (a PCGen `data/` checkout, e.g.
/// `~/workspace/repos/pcgen/data`). `ingested_at` is stamped at call time
/// by the caller (real wall-clock ISO-8601, never derived from git log --
/// `decisions.md §11.1`). `fetched_at_web` is the real timestamp of the
/// SD-25 web second-source pass this cache's one `web_second_source`
/// record (`Rag Armor (Dark Creeper)`) carries forward verbatim.
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
    generate_monsters(corpus_root, out_dir, ingested_at, &mut report)?;
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
        assert!(!name_or_key_is_pi(&["Studded Leather", "Ration"]));
    }

    #[test]
    fn name_or_key_is_pi_catches_a_blacklisted_key() {
        let term = pi_screening::PI_BLACKLIST_TERMS[10];
        assert!(name_or_key_is_pi(&[term, "clean"]));
    }

    #[test]
    fn name_or_key_is_pi_catches_a_blacklisted_name_even_when_key_is_clean() {
        let term = pi_screening::PI_BLACKLIST_TERMS[25];
        assert!(name_or_key_is_pi(&["clean_key", term]));
    }

    #[test]
    fn a_name_pi_equipment_entry_would_be_renamed_never_shipped_under_its_own_identity() {
        let term = pi_screening::PI_BLACKLIST_TERMS[26];
        assert!(name_or_key_is_pi(&[term, "Ordinary Item"]));
        let codex_key = neutral_key("equipment", WIRING_CLASS_BOOK_ID, "b1_equip_general.lst", 3);
        let codex_name = neutral_name("equipment", WIRING_CLASS_BOOK_ID, "b1_equip_general.lst", 3);
        assert_ne!(codex_key, term);
        assert_ne!(codex_name, term);
        assert!(codex_name.starts_with("Codex-Named Unit"));
    }

    #[test]
    fn slugify_handles_parens_and_spaces() {
        assert_eq!(slugify("Rag Armor (Dark Creeper)"), "rag_armor_dark_creeper");
        assert_eq!(slugify("Goblin Dog"), "goblin_dog");
        assert_eq!(slugify("Heartstone (Night Hag)"), "heartstone_night_hag");
    }

    /// SD-32 Epic 5 protective sweep, same shape as `cache_gen::acg`'s
    /// finding: `write_json` clobbered an already-enriched file with no
    /// exists-guard. `enrich_equipment_raw_tokens.rs` lists `"beastiary"`
    /// among its books and writes `raw_tokens` onto this generator's own
    /// equipment output AFTER it runs (3 of the book's 4 on-disk equipment
    /// records carry it today) -- this generator's own `EquipmentData`
    /// cannot reconstruct that field, so a bare re-run would silently
    /// strip it.
    #[test]
    fn write_json_never_overwrites_an_existing_file() {
        let dir = std::env::temp_dir().join(format!(
            "beastiary1_write_json_test_{}_{}",
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
            data: EquipmentData {
                key: "foo".to_string(),
                category: "General".to_string(),
                name: "Foo".to_string(),
                cost_gp: None,
                weight: None,
                description: None,
            },
            source: Source::LstToken {
                path: "b1_equip.lst".to_string(),
                sha256: "deadbeef".to_string(),
                line: 1,
                record_key: "foo".to_string(),
            },
            field_provenance: None,
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
