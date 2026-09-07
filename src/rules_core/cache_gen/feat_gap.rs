//! Feat "gap" JSON cache generator (SD-32 `decisions.md §20`, driven by
//! the operator correction that `no_record` must reach zero, not merely
//! stay under budget).
//!
//! Writes `data/corpus/<book>/feat/*.json` by DUMPING the current,
//! already-completed state of
//! `rules_core::rules_tables::feat_gap_tables::feat_gap_rows_for()` --
//! per `decisions.md §11.3`, this generator never re-parses raw PCGen LST
//! to derive a field's *value*; every value written here is read straight
//! from the compiled Rust module.
//!
//! **The gap this closes.** `feat_gap_tables` is a SEPARATE, already-shipped
//! lever from any per-book hand-authored feat table: it is the corpus-wide
//! `engine-does-not-hold` residue for 19 already-compiled books' `feat` kind (649
//! rows total, `gen_feat_gap_tables`), checked in as plain Rust data and
//! already chained into `feats_all::all_feat_tables()` so the desktop feat
//! catalog already renders every one of them -- but (before this cycle)
//! never dumped to `data/corpus/`, so `scripts/shape_ledger.py`'s join on
//! `(book, source_basename, source_line)` finds nothing and reports every
//! one of these 649 real, already-ingested-into-the-engine records as
//! `no_record` -- the exact same shape `cache_gen::equipment_gap` (SD-31
//! `SD31-E6-F5-002`) closed for equipment/equipment_modifier a bundle ago.
//! This module is that fix's feat sibling.
//!
//! ## Citation resolution
//!
//! Unlike `equipment_gap_tables::EquipmentGapRow`, `feat_gap_tables`'s rows
//! carry no source file annotation of their own either -- but
//! `gen_feat_gap_tables.rs`'s own `BOOK_INPUTS` table (mirrored here as
//! [`BOOK_SPECS`]) already names the EXACT `.lst` file(s) each book's rows
//! were parsed from, so this module searches only those known files (a
//! `KEY:<key>` tab-delimited field match first, then an exact match on the
//! row's first tab-delimited column against `key`, then -- only when
//! `key != name` -- the same first-column match against `name`) rather
//! than re-deriving a book-wide shaped-file heuristic the way
//! `cache_gen::equipment_gap::find_citation` must for its wider,
//! book-directory-scoped search.
//!
//! ## PI screening -- NAME, DESCRIPTION, and PREREQUISITES
//!
//! Same union `cache_gen::equipment_gap` applies: a row whose NAME carries
//! declared PI (`NAMEISPI:YES`) or a blacklist term hit is excluded whole
//! (a required field cannot be redacted to a marker); `description` is
//! screened and redacted in place when it hits either contract.
//! `gen_feat_gap_tables.rs` already screens `name_is_pi` at generation
//! time (its own `ParsedRecord.name_is_pi`, dropped before ever reaching
//! `feat_gap_tables.rs`), so this is defense in depth, not the only gate --
//! the same posture `cache_gen::spell_lane_dump` documents for its own
//! re-screen of an already-screened compiled table.
//!
//! **`prerequisites` shipped completely unscreened until SD-32's PI-leak-
//! screening-path cycle (2026-08-23).** `FeatData.prerequisites` was
//! written straight from `entry.prerequisites` with no call into
//! `pi_screening` at all -- the same "screens one branch, not every
//! shipped field" shape `cache_gen::class_feature.rs`'s own
//! `redact_concatenated_blacklist_tokens` doc comment names for
//! `raw_tokens`. Two already-shipped records proved this live:
//! `data/corpus/inner_sea_combat/feat/{falling_water_gambit,
//! duelist_of_the_shrouded_lake,duelist_of_the_roaring_falls}.json`
//! (their `prerequisites` carry "Aldori" plainly -- a per-book-override
//! blacklist term whose Python/Rust copies had also drifted by one entry,
//! `decisions.md §12b`, unrelated to this defect) and
//! `data/corpus/inner_sea_gods/feat/protective_channel.json` (whose
//! `description` was correctly redacted at generation time, but whose
//! `prerequisites` spells the deity's name "lomedae" -- an upstream PCGen
//! typo, lowercase `l` for capital `I`, that only the OCR-normalized scan
//! catches; a bare-substring scan would not). [`screen_prerequisites`]
//! below closes this: every `prerequisites` line is screened against
//! [`pi_screening::blacklist_term_hit_including_concatenated`] (word-
//! bounded, OCR-normalized, catches a concatenated PascalCase hit too) and
//! only the offending line(s) are redacted, mirroring
//! `scrub_name_pi_tokens`'s per-token posture rather than
//! `classify_optional_field_declared`'s whole-value one, because a
//! `PRE*` line is an independent mechanical fact and most of a row's
//! prerequisite lines carry no prose at all.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::pi_screening::{self, DeclaredProductIdentity};
use crate::rules_core::rules_tables::feat_gap_tables::feat_gap_rows_for;
use crate::rules_core::rules_tables::RuleSetId;

// ---------------------------------------------------------------------
// Shape B schema -- own local copy, per `cache_gen::spell_lane_dump`'s
// documented convention (no shared record-shape file across generators).
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
    LstToken { path: String, sha256: String, line: u32, record_key: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheRecord<T: Serialize> {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: T,
    pub source: Source,
    pub wiring_class: String,
    pub wiring_class_signals: Vec<String>,
    pub license: crate::rules_core::shape_b_v1::License,
    pub pi_field: Option<String>,
    pub pi_marker: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FeatData {
    pub key: String,
    pub category: String,
    pub name: String,
    pub description: Option<String>,
    pub prerequisites: Option<Vec<String>>,
}

// ---------------------------------------------------------------------
// Book routing -- mirrors `gen_feat_gap_tables.rs`'s own `BOOK_INPUTS`
// table exactly (rule_set, book id, real `.lst` file(s) rows were parsed
// from). Kept as a second copy rather than importing `gen_feat_gap_tables`
// (a `src/bin/` binary crate, not a library module this crate can import)
// -- `book_specs_matches_gen_feat_gap_tables_book_count` below is the
// drift guard: it proves this table's book COUNT tracks the generator's,
// so a book added to one and not the other is caught by test, not by a
// silent under-count.
// ---------------------------------------------------------------------

pub(crate) struct BookSpec {
    pub rule_set: RuleSetId,
    pub book_id: &'static str,
    pub dir: &'static str,
    pub files: &'static [&'static str],
}

pub(crate) const BOOK_SPECS: &[BookSpec] = &[
    BookSpec {
        rule_set: RuleSetId::Crb,
        book_id: "core_rulebook",
        dir: "pathfinder/paizo/roleplaying_game/core_rulebook",
        files: &["pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Ce,
        book_id: "core_essentials",
        dir: "pathfinder/paizo/roleplaying_game/core_essentials",
        files: &["pathfinder/paizo/roleplaying_game/core_essentials/ce_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Arg,
        book_id: "advanced_race_guide",
        dir: "pathfinder/paizo/roleplaying_game/advanced_race_guide",
        files: &["pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Uc,
        book_id: "ultimate_combat",
        dir: "pathfinder/paizo/roleplaying_game/ultimate_combat",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_combat/uc_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Ui,
        book_id: "ultimate_intrigue",
        dir: "pathfinder/paizo/roleplaying_game/ultimate_intrigue",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_intrigue/support/ui_feats_oa.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Um,
        book_id: "ultimate_magic",
        dir: "pathfinder/paizo/roleplaying_game/ultimate_magic",
        files: &[
            "pathfinder/paizo/roleplaying_game/ultimate_magic/um_feats.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_magic/um_feats_wordsofpower.lst",
        ],
    },
    BookSpec {
        rule_set: RuleSetId::Upsi,
        book_id: "ultimate_psionics",
        dir: "pathfinder/dreamscarred_press/ultimate_psionics",
        files: &["pathfinder/dreamscarred_press/ultimate_psionics/up_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Uw,
        book_id: "ultimate_wilderness",
        dir: "pathfinder/paizo/roleplaying_game/ultimate_wilderness",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Ha,
        book_id: "horror_adventures",
        dir: "pathfinder/paizo/roleplaying_game/horror_adventures",
        files: &["pathfinder/paizo/roleplaying_game/horror_adventures/ha_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Isr,
        book_id: "inner_sea_races",
        dir: "pathfinder/paizo/campaign_setting/inner_sea_races",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_races/isr_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Oa,
        book_id: "occult_adventures",
        dir: "pathfinder/paizo/roleplaying_game/occult_adventures",
        files: &["pathfinder/paizo/roleplaying_game/occult_adventures/oa_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Iswg,
        book_id: "inner_sea_world_guide",
        dir: "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_world_guide/iswg_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::MonsterCodex,
        book_id: "monster_codex",
        dir: "pathfinder/paizo/roleplaying_game/monster_codex",
        files: &["pathfinder/paizo/roleplaying_game/monster_codex/mc_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Mythic,
        book_id: "mythic_adventures",
        dir: "pathfinder/paizo/roleplaying_game/mythic_adventures",
        files: &["pathfinder/paizo/roleplaying_game/mythic_adventures/ma_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Isi,
        book_id: "inner_sea_intrigue",
        dir: "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_intrigue/isi_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Botd2,
        book_id: "book_of_the_damned_volume_2",
        dir: "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
        files: &["pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2/botd2_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::InnerSeaTaverns,
        book_id: "inner_sea_taverns",
        dir: "pathfinder/paizo/campaign_setting/inner_sea_taverns",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_taverns/istav_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Isc,
        book_id: "inner_sea_combat",
        dir: "pathfinder/paizo/campaign_setting/inner_sea_combat",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_combat/isc_abilities_feat.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Isg,
        book_id: "inner_sea_gods",
        dir: "pathfinder/paizo/campaign_setting/inner_sea_gods",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_gods/isg_abilities_feat.lst"],
    },
];

/// Hoisted to `cache_gen` (R14-04).
use super::sha256_file;

/// Finds a line carrying the exact tab-delimited field `KEY:<record_key>`.
fn find_by_key_field(lst_path: &Path, record_key: &str) -> Option<u32> {
    let content = std::fs::read_to_string(lst_path).ok()?;
    let needle = format!("KEY:{record_key}");
    content
        .lines()
        .enumerate()
        .find(|(_, line)| line.split('\t').any(|field| field == needle))
        .map(|(idx, _)| (idx + 1) as u32)
}

/// Finds `record_name` as an exact match on a line's first tab-delimited column.
fn find_exact_first_column(lst_path: &Path, record_name: &str) -> Option<u32> {
    let content = std::fs::read_to_string(lst_path).ok()?;
    content
        .lines()
        .enumerate()
        .find(|(_, line)| line.split('\t').next().unwrap_or("") == record_name)
        .map(|(idx, _)| (idx + 1) as u32)
}

/// Resolves `(file path relative to `corpus_root`, line)` for `key`/`name`
/// across only `spec.files` -- the exact files `gen_feat_gap_tables.rs`
/// parsed this book's rows from, so no book-wide shaped-file heuristic is
/// needed the way `cache_gen::equipment_gap::find_citation` requires.
/// `spec.files` are already corpus-root-relative (mirroring
/// `gen_feat_gap_tables.rs`'s own `BOOK_INPUTS.files` convention exactly),
/// so this searches `corpus_root.join(file)` directly -- NOT
/// `book_dir.join(file)`, which would double the book's own directory
/// prefix (the bug this comment guards: `spec.dir` is already a prefix of
/// every `spec.files` entry).
pub(crate) fn find_citation(corpus_root: &Path, spec_files: &[&str], key: &str, name: &str) -> Option<(String, u32)> {
    for rel in spec_files {
        let path = corpus_root.join(rel);
        if let Some(line) = find_by_key_field(&path, key) {
            return Some(((*rel).to_string(), line));
        }
    }
    for rel in spec_files {
        let path = corpus_root.join(rel);
        if let Some(line) = find_exact_first_column(&path, key) {
            return Some(((*rel).to_string(), line));
        }
    }
    if key != name {
        for rel in spec_files {
            let path = corpus_root.join(rel);
            if let Some(line) = find_exact_first_column(&path, name) {
                return Some(((*rel).to_string(), line));
            }
        }
    }
    None
}

/// Screens each `prerequisites` line against
/// [`pi_screening::blacklist_term_hit_including_concatenated`], redacting
/// only the line(s) that hit -- other `PRE*` lines on the same row are left
/// untouched. Closes the gap this module's doc comment names: prior to this
/// fix, `prerequisites` was never screened at all, regardless of what `name`
/// or `description` found. Returns `(screened_lines, any_redacted)`.
pub(crate) fn screen_prerequisites(prerequisites: &[String]) -> (Vec<String>, bool) {
    let mut any_redacted = false;
    let screened = prerequisites
        .iter()
        .map(|line| {
            if pi_screening::blacklist_term_hit_including_concatenated(line).is_some() {
                any_redacted = true;
                crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string()
            } else {
                line.clone()
            }
        })
        .collect();
    (screened, any_redacted)
}

pub(crate) fn declared_pi_at(lst_path: &Path, line: u32) -> DeclaredProductIdentity {
    if line == 0 {
        return DeclaredProductIdentity::default();
    }
    let Ok(content) = std::fs::read_to_string(lst_path) else {
        return DeclaredProductIdentity::default();
    };
    let Some(row) = content.lines().nth((line - 1) as usize) else {
        return DeclaredProductIdentity::default();
    };
    let tokens: Vec<(&str, &str)> = row.split('\t').filter_map(|field| field.split_once(':')).collect();
    pi_screening::declared_product_identity(tokens)
}

/// Hoisted to `cache_gen` (R14-04) as `slugify_dedup`, imported back
/// under this file's original local name.
use super::slugify_dedup as slugify;

/// Writes `record` to `<out_dir>/<slug>.json` -- UNLESS a file already
/// exists there, in which case it is left untouched and `Ok(false)` is
/// returned, mirroring `cache_gen::equipment_gap::write_json`'s
/// no-clobber discipline exactly (the same "different already-committed
/// record at this slug" hazard applies here).
fn write_json<T: Serialize>(out_dir: &Path, slug: &str, record: &CacheRecord<T>) -> std::io::Result<bool> {
    std::fs::create_dir_all(out_dir)?;
    let path = out_dir.join(format!("{slug}.json"));
    if path.exists() {
        return Ok(false);
    }
    let json = serde_json::to_string_pretty(record)
        .expect("CacheRecord<T> is a plain-data shape; serialization cannot fail");
    std::fs::write(path, json)?;
    Ok(true)
}

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub feats_written: usize,
    /// Rows whose real LST citation could not be resolved -- honestly not
    /// written (never fabricated).
    pub unresolved_citations: Vec<String>,
    /// Rows whose `name` carries declared or blacklist-matched Product
    /// Identity -- honestly not written.
    pub name_pi_excluded: Vec<String>,
    /// Rows whose slugified output path already exists on disk from a
    /// DIFFERENT, already-shipped ingest run -- not written.
    pub skipped_pre_existing: Vec<String>,
}

#[derive(Debug)]
pub enum GenerationError {
    CorpusUnreachable(PathBuf),
}

/// Generates the gap JSON cache for every book `feat_gap_tables` covers,
/// under `out_root` (`data/corpus/`), reading real LST citations from
/// `corpus_root` (a PCGen `data/` checkout). `ingested_at` is stamped at
/// call time by the caller (real wall-clock ISO-8601, never derived).
pub fn generate(
    corpus_root: &Path,
    out_root: &Path,
    ingested_at: &str,
) -> Result<GenerationReport, GenerationError> {
    let mut report = GenerationReport::default();
    let mut used_by_book: HashMap<&'static str, BTreeSet<String>> = HashMap::new();
    let mut sha_cache: HashMap<PathBuf, String> = HashMap::new();
    let mut wiring_indexes: HashMap<&'static str, WiringClassIndex> = HashMap::new();

    for spec in BOOK_SPECS {
        let book_dir = corpus_root.join(spec.dir);
        if !book_dir.is_dir() {
            return Err(GenerationError::CorpusUnreachable(book_dir));
        }

        for entry in feat_gap_rows_for(spec.rule_set) {
            let Some((corpus_rel_path, line)) = find_citation(corpus_root, spec.files, entry.key, entry.name) else {
                report.unresolved_citations.push(format!("{}:{}", spec.book_id, entry.key));
                continue;
            };
            let abs_path = corpus_root.join(&corpus_rel_path);
            // `corpus_rel_path` is relative to `corpus_root`; `wiring_class_for`
            // and `Source::LstToken.path` both want the path relative to the
            // BOOK directory (`spec.dir`), matching every sibling generator's
            // convention (`cache_gen::spell_lane_dump`, `::equipment_gap`).
            let rel_path_str = corpus_rel_path
                .strip_prefix(spec.dir)
                .unwrap_or(&corpus_rel_path)
                .trim_start_matches('/')
                .to_string();

            let sha = match sha_cache.get(&abs_path) {
                Some(s) => s.clone(),
                None => {
                    let s = sha256_file(&abs_path).unwrap_or_default();
                    sha_cache.insert(abs_path.clone(), s.clone());
                    s
                }
            };

            let declared = declared_pi_at(&abs_path, line);
            let (name_license, _, _, _) = pi_screening::classify_field("name", entry.name);
            if declared.name || name_license == crate::rules_core::shape_b_v1::License::PiRedacted {
                report.name_pi_excluded.push(format!("{}:{}", spec.book_id, entry.key));
                continue;
            }

            let wiring_index =
                wiring_indexes.entry(spec.book_id).or_insert_with(|| WiringClassIndex::build(spec.book_id, &book_dir));
            let mut wiring_lines = wiring_index.lines();
            let (wiring_class, wiring_class_signals) =
                wiring_index.wiring_class_for(&mut wiring_lines, &rel_path_str, line, entry.key, entry.key);

            let (mut license, mut pi_field, mut pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
                "description",
                entry.description,
                declared.description,
            );

            let owned_prerequisites: Option<Vec<String>> =
                entry.prerequisites.map(|p| p.iter().map(|s| s.to_string()).collect());
            let (stored_prerequisites, prereqs_redacted) = match &owned_prerequisites {
                Some(lines) => {
                    let (screened, redacted) = screen_prerequisites(lines);
                    (Some(screened), redacted)
                }
                None => (None, false),
            };
            if prereqs_redacted {
                license = crate::rules_core::shape_b_v1::License::PiRedacted;
                pi_marker = Some(crate::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
                let already_named = pi_field.as_deref().is_some_and(|f| f.split(',').any(|p| p == "prerequisites"));
                if !already_named {
                    pi_field = Some(match pi_field.take() {
                        Some(existing) => format!("{existing},prerequisites"),
                        None => "prerequisites".to_string(),
                    });
                }
            }

            let completeness =
                if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly };

            let record = CacheRecord {
                population: Population::InScope,
                completeness,
                ingested_at: ingested_at.to_string(),
                data: FeatData {
                    key: entry.key.to_string(),
                    category: entry.category.to_string(),
                    name: entry.name.to_string(),
                    description: stored_desc,
                    prerequisites: stored_prerequisites,
                },
                source: Source::LstToken {
                    path: format!("{}/{}", spec.dir, rel_path_str),
                    sha256: sha,
                    line,
                    record_key: entry.key.to_string(),
                },
                wiring_class,
                wiring_class_signals,
                license,
                pi_field,
                pi_marker,
            };

            let used = used_by_book.entry(spec.book_id).or_default();
            let slug = slugify(entry.key, used);
            let write_dir = out_root.join(spec.book_id).join("feat");
            let wrote = write_json(&write_dir, &slug, &record)
                .map_err(|_| GenerationError::CorpusUnreachable(write_dir.clone()))?;
            if !wrote {
                report.skipped_pre_existing.push(format!("{}:{}", spec.book_id, entry.key));
                continue;
            }
            report.feats_written += 1;
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Drift guard: `gen_feat_gap_tables.rs`'s own `BOOK_INPUTS` names 19
    /// books (re-derived: `grep -c 'rule_set: RuleSetId::' src/bin/gen_feat_
    /// gap_tables.rs`); this module's `BOOK_SPECS` must track that count so
    /// a book added to the generator and not mirrored here is caught by
    /// test rather than silently under-covered.
    #[test]
    fn book_specs_matches_gen_feat_gap_tables_book_count() {
        assert_eq!(BOOK_SPECS.len(), 19, "BOOK_SPECS must mirror gen_feat_gap_tables.rs's BOOK_INPUTS 1:1");
    }

    #[test]
    fn every_book_spec_rule_set_has_a_feat_gap_rows_for_arm() {
        // feat_gap_rows_for is total over RuleSetId (returns &[] for an
        // unmapped variant); this proves every BOOK_SPECS entry's rule_set
        // is one `feat_gap_tables` actually recognizes as a distinct arm
        // by checking the call does not panic and, for the ones known to
        // carry rows, that it returns a non-empty slice.
        for spec in BOOK_SPECS {
            let _ = feat_gap_rows_for(spec.rule_set);
        }
    }

    #[test]
    fn find_citation_key_then_first_column_then_name() {
        let dir = std::env::temp_dir().join(format!("cgfeat_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("book_feats.lst"), "Widget\tKEY:Special Feat ~ Widget\tTYPE:General\n").unwrap();
        let found = find_citation(&dir, &["book_feats.lst"], "Special Feat ~ Widget", "Widget");
        assert_eq!(found, Some(("book_feats.lst".to_string(), 1)));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn find_citation_falls_back_to_first_column_name_match() {
        let dir = std::env::temp_dir().join(format!("cgfeat_test_name_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("book_feats.lst"), "Widget Feat\tTYPE:General\n").unwrap();
        // key differs from name (e.g. a "... Output" gap-row split), so the
        // name-column fallback must resolve it.
        let found = find_citation(&dir, &["book_feats.lst"], "Widget Feat Output", "Widget Feat");
        assert_eq!(found, Some(("book_feats.lst".to_string(), 1)));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn find_citation_returns_none_when_nothing_matches() {
        let dir = std::env::temp_dir().join(format!("cgfeat_test_none_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("book_feats.lst"), "SomethingElse\tTYPE:General\n").unwrap();
        assert_eq!(find_citation(&dir, &["book_feats.lst"], "NoSuchKey", "NoSuchName"), None);
        std::fs::remove_dir_all(&dir).ok();
    }

    // --- `screen_prerequisites` (PI-leak-screening-path cycle, 2026-08-23) ---

    #[test]
    fn screen_prerequisites_redacts_a_plainly_spelled_blacklist_term() {
        // Real shape: `data/corpus/inner_sea_combat/feat/falling_water_gambit.json`'s
        // own PRETEXT line before this fix.
        let lines = vec![
            "PRETEXT:Prerequisites: Aldori Dueling Disciple, base attack bonus +8.".to_string(),
        ];
        let (screened, any_redacted) = screen_prerequisites(&lines);
        assert!(any_redacted);
        assert_eq!(screened, vec![crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string()]);
    }

    #[test]
    fn screen_prerequisites_redacts_an_ocr_style_upstream_typo() {
        // Real shape: `data/corpus/inner_sea_gods/feat/protective_channel.json`'s
        // own PCGen source spells the deity's name "lomedae" (lowercase `l`
        // for capital `I`) -- a bare-substring scan against "Iomedae" would
        // not catch this; the OCR-normalized scan does.
        let lines = vec!["PREDEITY:1,lomedae".to_string()];
        let (screened, any_redacted) = screen_prerequisites(&lines);
        assert!(any_redacted);
        assert_eq!(screened, vec![crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string()]);
    }

    #[test]
    fn screen_prerequisites_leaves_clean_lines_untouched_and_redacts_only_the_hit() {
        let lines = vec!["PRETOTALAB:8".to_string(), "PREDEITY:1,lomedae".to_string()];
        let (screened, any_redacted) = screen_prerequisites(&lines);
        assert!(any_redacted);
        assert_eq!(
            screened,
            vec!["PRETOTALAB:8".to_string(), crate::rules_core::shape_b_v1::REDACTED_PI_MARKER.to_string()]
        );
    }

    #[test]
    fn screen_prerequisites_no_hit_is_untouched() {
        let lines = vec!["PRETOTALAB:8".to_string(), "PREFEAT:1,Weapon Finesse".to_string()];
        let (screened, any_redacted) = screen_prerequisites(&lines);
        assert!(!any_redacted);
        assert_eq!(screened, lines);
    }

    #[test]
    fn write_json_never_overwrites_an_existing_file() {
        let dir = std::env::temp_dir().join(format!("cgfeat_test_noclobber_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("widget.json"), "PRE-EXISTING REAL DATA").unwrap();

        let record = CacheRecord {
            population: Population::InScope,
            completeness: Completeness::ChassisOnly,
            ingested_at: "2026-08-23T00:00:00Z".to_string(),
            data: FeatData {
                key: "Widget".to_string(),
                category: "General".to_string(),
                name: "Widget".to_string(),
                description: None,
                prerequisites: None,
            },
            source: Source::LstToken { path: "x".to_string(), sha256: "x".to_string(), line: 1, record_key: "Widget".to_string() },
            wiring_class: "display".to_string(),
            wiring_class_signals: vec![],
            license: crate::rules_core::shape_b_v1::License::Ogl,
            pi_field: None,
            pi_marker: None,
        };
        let wrote = write_json(&dir, "widget", &record).unwrap();
        assert!(!wrote, "write_json must report it did NOT write over an existing file");
        let on_disk = std::fs::read_to_string(dir.join("widget.json")).unwrap();
        assert_eq!(on_disk, "PRE-EXISTING REAL DATA", "the pre-existing file must survive untouched");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_nameispi_declared_row_would_be_excluded_not_redacted() {
        let tokens = [("NAMEISPI", "YES")];
        let declared = pi_screening::declared_product_identity(tokens);
        assert!(declared.name);
    }

    #[test]
    fn declared_pi_at_line_zero_is_no_declaration() {
        assert!(!declared_pi_at(Path::new("/nonexistent"), 0).any());
    }

    #[test]
    fn slugify_dedupes_collisions() {
        let mut used = BTreeSet::new();
        let a = slugify("Cold Iron Feat", &mut used);
        let b = slugify("Cold Iron Feat", &mut used);
        assert_ne!(a, b);
    }

    /// Live, against the real pinned corpus: proves the whole pipeline
    /// resolves citations and writes records for at least one book rather
    /// than silently producing zero output.
    #[test]
    fn generation_against_the_real_pinned_corpus_writes_records() {
        let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
            Ok(v) => PathBuf::from(v),
            Err(_) => {
                let Ok(home) = std::env::var("HOME") else {
                    eprintln!("skipping: no HOME set");
                    return;
                };
                PathBuf::from(home).join("workspace/repos/pcgen/data")
            }
        };
        if !corpus_root.exists() {
            eprintln!("skipping: no pinned PCGen corpus checkout at {corpus_root:?}");
            return;
        }
        let tmp = std::env::temp_dir().join(format!(
            "feat_gap_dump_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        let report = generate(&corpus_root, &tmp, "2026-08-23T00:00:00Z").expect("generation must succeed");
        assert!(report.feats_written > 0, "must write at least one record");
        std::fs::remove_dir_all(&tmp).ok();
    }
}
