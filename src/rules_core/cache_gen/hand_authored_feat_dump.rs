//! Hand-authored feat table JSON cache generator
//! (SD-32 `decisions.md §20`).
//!
//! **The gap this closes.** `cache_gen::feat_gap` dumps
//! `feat_gap_tables::feat_gap_rows_for()` — the residue a book's
//! hand-authored feat table does NOT hold. It deliberately EXCLUDES
//! anything already in `feats_all::hand_authored_feat_tables()`, because
//! that population was assumed already dumped to `data/corpus/` by an
//! earlier per-book cycle. Re-derived this cycle: it was NOT, for four
//! books whose `no_record` `feat` population sits on the exact file their
//! own hand-authored table was built from — `core_rulebook` (67, `crb::
//! feats`, 185 hand-authored entries, `data/corpus/core_rulebook/feat/`
//! held exactly 1 file before this generator ran), `ultimate_psionics`
//! (92), `advanced_class_guide` (39), `ultimate_campaign` (23). The
//! compiled table exists and the engine already serves every one of these
//! records; `scripts/shape_ledger.py`'s join just had nothing on disk to
//! find, the identical shape `cache_gen::feat_gap` closed for the OTHER
//! half of the same kind's population.
//!
//! **Never re-parses raw PCGen LST to derive a field's *value*** (per
//! `decisions.md §11.3`) — every field written here is read straight from
//! the compiled `FeatCatalogRecord`. The `.lst` file is read only to
//! recover the real, checkable line-number citation, via
//! `cache_gen::feat_gap::find_citation` (reused directly, not
//! reimplemented) against this module's own `BOOK_SPECS`.
//!
//! **No-clobber, same as every sibling generator**: a slug already
//! occupied on disk (this run or an earlier one) is left untouched. Books
//! NOT listed in this module's `BOOK_SPECS` (APG/ACG's siblings ARG/PU/UI/
//! UW/UC/UM/UPSI's already-`0`-or-near-`0` no_record counts suggest an
//! earlier cycle already dumped them by some other path) are simply not
//! touched by this pass — scoped to the four books actually re-derived
//! `no_record` this cycle, not run speculatively over every hand-authored
//! book.
//!
//! Run with `cargo run --locked --bin gen_cache_hand_authored_feat_dump`.
//! `PCGEN_CORPUS_ROOT` overrides the default
//! `$HOME/workspace/repos/pcgen/data`.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use crate::rules_core::cache_gen::feat_gap::{declared_pi_at, find_citation, screen_prerequisites, BookSpec};
use crate::rules_core::cache_gen::feat_gap::{CacheRecord, Completeness, FeatData, Population, Source};
use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::pi_screening;
use crate::rules_core::rules_tables::feats_all::hand_authored_feat_tables;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::shape_b_v1::License;

/// The four books whose hand-authored feat table was re-derived this cycle
/// to have little or no `data/corpus/<book>/feat/` cache — NOT a copy of
/// `cache_gen::feat_gap::BOOK_SPECS` (that table names every book with a
/// GAP-table row; this one names only books whose HAND-AUTHORED table
/// itself needs dumping).
const BOOK_SPECS: &[BookSpec] = &[
    BookSpec {
        rule_set: RuleSetId::Crb,
        book_id: "core_rulebook",
        dir: "pathfinder/paizo/roleplaying_game/core_rulebook",
        files: &["pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Upsi,
        book_id: "ultimate_psionics",
        dir: "pathfinder/dreamscarred_press/ultimate_psionics",
        files: &["pathfinder/dreamscarred_press/ultimate_psionics/up_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Acg,
        book_id: "advanced_class_guide",
        dir: "pathfinder/paizo/roleplaying_game/advanced_class_guide",
        files: &["pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_feats.lst"],
    },
    BookSpec {
        rule_set: RuleSetId::Uca,
        book_id: "ultimate_campaign",
        dir: "pathfinder/paizo/roleplaying_game/ultimate_campaign",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_campaign/uca_feats.lst"],
    },
];

fn slugify(name: &str, used: &mut BTreeSet<String>) -> String {
    let mut slug: String =
        name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
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

fn write_json<T: serde::Serialize>(out_dir: &Path, slug: &str, record: &CacheRecord<T>) -> std::io::Result<bool> {
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

fn sha256_file(path: &Path) -> std::io::Result<String> {
    let output = std::process::Command::new("sha256sum").arg(path).output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!("sha256sum failed for {}", path.display())));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.split_whitespace().next().unwrap_or_default().to_string())
}

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub feats_written: usize,
    pub unresolved_citations: Vec<String>,
    pub name_pi_excluded: Vec<String>,
    pub skipped_pre_existing: Vec<String>,
}

#[derive(Debug)]
pub enum GenerationError {
    CorpusUnreachable(PathBuf),
}

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

        let Some(table) = hand_authored_feat_tables().iter().find(|b| b.rule_set == spec.rule_set) else {
            continue;
        };

        for entry in table.entries {
            let Some((corpus_rel_path, line)) = find_citation(corpus_root, spec.files, entry.key, entry.name) else {
                report.unresolved_citations.push(format!("{}:{}", spec.book_id, entry.key));
                continue;
            };
            let abs_path = corpus_root.join(&corpus_rel_path);
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
            let (name_license, ..) = pi_screening::classify_field("name", entry.name);
            if declared.name || name_license == License::PiRedacted {
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

            // PI-leak-screening-path cycle (2026-08-23): `prerequisites` was
            // never screened here either -- same defect, same fix, as
            // `cache_gen::feat_gap::generate`'s own sibling change; see that
            // module's doc comment for the confirmed-leak records this
            // closed. Reuses `screen_prerequisites` directly rather than
            // forking a second copy.
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
                license = License::PiRedacted;
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

    #[test]
    fn every_book_spec_has_a_non_empty_hand_authored_table() {
        for spec in BOOK_SPECS {
            let table = hand_authored_feat_tables().iter().find(|b| b.rule_set == spec.rule_set);
            assert!(table.is_some(), "{} must have a hand_authored_feat_tables() entry", spec.book_id);
            assert!(
                !table.unwrap().entries.is_empty(),
                "{} was included in BOOK_SPECS as a HAND-AUTHORED book but its table is empty -- \
                 belongs in cache_gen::feat_gap instead",
                spec.book_id
            );
        }
    }

    #[test]
    fn generation_against_the_real_pinned_corpus_writes_records() {
        let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
            Ok(v) => PathBuf::from(v),
            Err(_) => {
                eprintln!("skipping live-corpus test: PCGEN_CORPUS_ROOT not set");
                return;
            }
        };
        if !corpus_root.is_dir() {
            eprintln!("skipping live-corpus test: {corpus_root:?} does not exist");
            return;
        }
        let out_root = std::env::temp_dir()
            .join(format!("codex_hand_authored_feat_dump_live_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&out_root);
        let report = generate(&corpus_root, &out_root, "2026-08-23T00:00:00Z").expect("generate");
        assert!(report.feats_written > 150, "expected >150 records, got {}", report.feats_written);
        let _ = std::fs::remove_dir_all(&out_root);
    }
}
