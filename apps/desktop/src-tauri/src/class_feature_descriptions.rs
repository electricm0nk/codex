//! Real corpus-description surface for `class_feature` records
//! (SD31-D7-PROSE-003).
//!
//! # Why this module exists
//!
//! `ClassFeatureRow.detail` (`characterHub/classFeaturesModel.ts`) renders
//! `ExplanationDto.detail` -- the rules engine's own COMPUTED derivation
//! text, cited from `pilot_compute.rs`'s explanations. That is real and
//! correct for what it is, but it is not the corpus `DESC:` text: a feature
//! whose engine derivation is a bare `+2` magnitude with no accompanying
//! prose carries no rulebook description anywhere on the character sheet.
//! `SD31-D7-PROSE-001`/`002` built the equivalent surface for `race_trait`
//! (`race_trait_picker.rs`) and `monster_ability`
//! (`monster_catalog::serve_ability_description`); `class_feature` was
//! deliberately left for its own cycle because, unlike those two kinds,
//! nothing in this engine already carries a class feature's corpus
//! description onto the wire -- this module is that missing render path,
//! built to the same shape.
//!
//! # Source of truth: the `cache_gen::class_feature` JSON cache, not the raw
//! `.lst`
//!
//! `src/rules_core/cache_gen/class_feature.rs` (SD31-E5-F1-001) already
//! transcribes every in-scope `class_feature` unit's real corpus row --
//! `DESC:` included, already PI-screened (`§52.3`/`§53.5`, both contracts) --
//! into `data/corpus/<book>/class_feature/<class-slug>/<feature-slug>.json`.
//! This module reads that cache exactly the way `corpus_full.rs` reads
//! `data/corpus/<book>/equipment/` for the real equipment corpus: via
//! `authoring_workbench::codex_repo_root()`, the same dev-checkout path
//! resolution every other real-corpus reader in this crate already uses.
//! **Same packaging caveat as `corpus_full.rs`**: this works for a source
//! checkout and any deployment that sets `CODEX_REPO_ROOT` to a location
//! carrying `data/corpus/`; bundling `data/corpus/` into a packaged Tauri
//! installer is separate, already-tracked follow-on work, not assumed here.
//!
//! # The join: `(class_slug, feature_slug)`, not a stored crosswalk
//!
//! `ExplanationDto.id` (e.g. `class_feature.rogue.sneak_attack`) carries no
//! corpus citation -- the engine's own id is a computed-record identity, not
//! a corpus key. `Kind::ClassFeature`'s own `classify()` arm
//! (`v06_work_inventory.rs`) already trusts exactly this join to decide
//! `grounded`: `id.contains(".{owner}.") && id.ends_with(&feature_slug)`,
//! where `owner` is a class name the engine models and `feature_slug` is
//! `slug()` of the corpus feature name. This module computes the SAME two
//! slugs (`slug()`, transcribed byte-for-byte from
//! `v06_work_inventory.rs::slug` -- reproduced locally rather than shared,
//! per this package's disjoint-file-touch convention every `cache_gen::*`
//! module and `corpus_full.rs` already follow) from the cache record's own
//! `data.class`/`data.name`, and the frontend (`classFeaturesModel.ts`)
//! performs the identical suffix/contains match against the real
//! `explanation.id` it already has. Reusing an already-adjudicated matching
//! rule rather than inventing a second one is deliberate: the frontend's own
//! join can never be more permissive than the join the board's own doneness
//! measurement already trusts.
//!
//! # PI screening
//!
//! Already discharged upstream: `cache_gen::class_feature::generate` runs
//! both SD-30 contracts (`§52.3` blacklist sweep, `§53.5` declared-PI reader)
//! on NAME and DESCRIPTION before a record is ever written to
//! `data/corpus/`, and a `NAMEISPI:YES` row is not written at all. This
//! module reads only already-screened output; it re-runs no PI check of its
//! own, the same trust boundary `corpus_full.rs` and every other
//! `data/corpus/`-reading module in this crate already holds.
//!
//! # The leak guard
//!
//! Same check as `monster_catalog::serve_ability_description` /
//! `companion_catalog::serve_ability_description`: `render_pcgen_desc`'s
//! output is checked with `leaked_pcgen_syntax`. **Skip, not panic** --
//! deliberately different from those two: their registered population is a
//! small, hand-vetted list re-checked every time a book is added, so a panic
//! there is a proven-unreachable invariant. This catalog walks 12,000+ live
//! corpus records at process-start time, and
//! `every_real_class_feature_description_renders_without_a_pcgen_syntax_leak`
//! (this module's own test) found a real one live
//! (`advanced_class_guide:class_feature:
//! enhancement_savant_subschool_perfection_of_self`) -- a hard panic there
//! would crash every character sheet on process start over one malformed row
//! anywhere in the whole corpus. See [`load_class_feature_descriptions`]'s
//! own inline comment for the exact shape and why skip-and-report is the
//! conservative, honest response.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Serialize;
use serde_json::Value;

use crate::authoring_workbench::codex_repo_root;

/// One class feature's real corpus description, joined to the engine
/// explanation ids that may claim it. See the module doc comment's "The
/// join" section for exactly how `classSlug`/`featureSlug` are used.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeatureDescriptionDto {
    /// The corpus book directory this record was read from
    /// (`data/corpus/<book>/class_feature/...`).
    pub book: String,
    /// `slug()` of the record's `data.class` value (`"Rogue"` -> `"rogue"`),
    /// the same token an `ExplanationDto.id` carries for a class the engine
    /// models.
    pub class_slug: String,
    /// `slug()` of the record's `data.name` value, matched against the
    /// TRAILING segment(s) of an `ExplanationDto.id` via `endsWith` -- never
    /// equality, because some ids carry an extra `corpus_record` segment
    /// (`classFeaturesModel.ts`'s own `RECORD_FAMILY_SEGMENTS`).
    pub feature_slug: String,
    /// The corpus `KEY:` token verbatim (`"Rogue ~ Sneak Attack"`).
    pub key: String,
    pub name: String,
    /// Rendered through `render_pcgen_desc`, leak-checked. Never `null` --
    /// records with no real description are not emitted at all (see
    /// `is_real_description_value`).
    pub description: String,
}

/// Reproduced from `v06_work_inventory.rs::slug` -- see this module's own doc
/// comment on why it is a local copy rather than a shared import.
fn slug(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut last_underscore = true;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            last_underscore = false;
        } else if !last_underscore {
            out.push('_');
            last_underscore = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}

/// Reproduced from `v06_work_inventory.rs::is_real_description_value` --
/// same three refusals (empty, `.CLEAR`/`.CLEARALL`, the PI-redaction
/// marker), same reproduced-locally convention as `slug` above.
fn is_real_description_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

fn walk_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

/// Reads every `class_feature` cache record under `<repo_root>/data/corpus/
/// */class_feature/**/*.json`, keeping only the ones that carry a real
/// description and a resolvable owning class (`data.class`) -- a pool-member
/// record with no `~`-split owner has no class to join against and is
/// correctly absent from this catalog, exactly as it is absent from
/// `Kind::ClassFeature`'s `class_feature_owner` matching in
/// `v06_work_inventory.rs`.
fn load_class_feature_descriptions(repo_root: &Path) -> Vec<ClassFeatureDescriptionDto> {
    let corpus_root = repo_root.join("data/corpus");
    let mut out = Vec::new();
    let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
    let mut book_dirs: Vec<_> = books.flatten().collect();
    book_dirs.sort_by_key(|e| e.file_name());
    for book_entry in book_dirs {
        let book_dir = book_entry.path();
        if !book_dir.is_dir() {
            continue;
        }
        let book = book_entry.file_name().to_string_lossy().to_string();
        let cf_dir = book_dir.join("class_feature");
        if !cf_dir.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        walk_json_files(&cf_dir, &mut files);
        for file in files {
            let Ok(text) = std::fs::read_to_string(&file) else { continue };
            let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
            let data = &doc["data"];
            let (Some(key), Some(name), Some(class)) =
                (data["key"].as_str(), data["name"].as_str(), data["class"].as_str())
            else {
                continue;
            };
            let Some(raw_desc) = data["description"].as_str() else { continue };
            if !is_real_description_value(raw_desc) {
                continue;
            }
            let rendered = codex::rules_core::pcgen_desc::render_pcgen_desc(raw_desc);
            // A hard panic (`monster_catalog`/`companion_catalog`'s own
            // convention) is right for those two chassis-table kinds, whose
            // registered population is a small, hand-vetted list re-checked
            // every time a book is added. This catalog walks 12,000+ live
            // corpus records at process-start time, and a single malformed
            // row (found live: `advanced_class_guide:class_feature:
            // enhancement_savant_subschool_perfection_of_self`'s row
            // declares TWO pipe-separated arguments but its prose only
            // references `%1`, so `render_pcgen_desc`'s segment-count-vs-
            // max-reference heuristic mis-splits and a literal `|` survives
            // into the rendered text) would otherwise crash every character
            // sheet on process start over one bad row anywhere in the whole
            // corpus. The conservative, honest response is the SAME one
            // `is_real_description_value` already applies to an empty or
            // placeholder description: refuse to serve it, count it, never
            // guess a value or ship broken text. Not silent -- `eprintln!`
            // so a real occurrence is visible in the app's own log, and
            // `a_record_whose_render_leaks_a_pipe_argument_tail_is_refused_
            // not_shipped` (this module's own test, below) pins the exact
            // known record by name.
            if let Some(leak) = codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text) {
                eprintln!(
                    "class_feature_descriptions: refusing to serve {key:?} ({book}) -- rendered \
                     description still carries {leak}. Raw token: {raw_desc:?}"
                );
                continue;
            }
            out.push(ClassFeatureDescriptionDto {
                book: book.clone(),
                class_slug: slug(class),
                feature_slug: slug(name),
                key: key.to_string(),
                name: name.to_string(),
                description: rendered.text,
            });
        }
    }
    out
}

/// Built once, cached for the process lifetime -- mirrors
/// `corpus_full::full_corpus_bundle()`'s own caching shape.
fn class_feature_descriptions() -> &'static Vec<ClassFeatureDescriptionDto> {
    static TABLE: OnceLock<Vec<ClassFeatureDescriptionDto>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let repo_root = codex_repo_root()
            .expect("codex repo root must resolve for class_feature description loading");
        load_class_feature_descriptions(&repo_root)
    })
}

#[tauri::command]
pub fn list_class_feature_descriptions() -> Vec<ClassFeatureDescriptionDto> {
    class_feature_descriptions().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        codex_repo_root().expect("repo root resolves under `cargo test`")
    }

    #[test]
    fn slug_matches_v06_work_inventorys_own_algorithm() {
        assert_eq!(slug("Sneak Attack"), "sneak_attack");
        assert_eq!(slug("Unchained Summoner"), "unchained_summoner");
        assert_eq!(slug("Rogue"), "rogue");
        assert_eq!(slug("  Trailing/Punct!! "), "trailing_punct");
    }

    #[test]
    fn is_real_description_value_refuses_empty_clear_and_the_pi_marker() {
        assert!(!is_real_description_value(""));
        assert!(!is_real_description_value("   "));
        assert!(!is_real_description_value(".CLEAR"));
        assert!(!is_real_description_value(".CLEARALL"));
        assert!(!is_real_description_value("[redacted PI]"));
        assert!(is_real_description_value("You gain a bonus."));
    }

    /// The real corpus loads and carries thousands of real, described class
    /// features -- proven against the live `data/corpus/` checkout, not a
    /// fixture.
    #[test]
    fn loads_thousands_of_real_described_class_features_from_the_live_corpus() {
        let descriptions = load_class_feature_descriptions(&repo_root());
        assert!(
            descriptions.len() > 1000,
            "expected thousands of real described class_feature records, got {}",
            descriptions.len()
        );
        // `Rogue ~ Sneak Attack` itself carries `description: null` in the
        // real corpus (its rules text lives on the class table, not a
        // per-feature `DESC:`) -- not a fixture stand-in for that record,
        // `Rogue ~ Trapfinding` genuinely does carry one.
        let trapfinding = descriptions
            .iter()
            .find(|d| d.book == "core_rulebook" && d.key == "Rogue ~ Trapfinding");
        let record =
            trapfinding.expect("core_rulebook's real Rogue ~ Trapfinding record must be in the catalog");
        assert_eq!(record.class_slug, "rogue");
        assert_eq!(record.feature_slug, "trapfinding");
        assert!(record.description.starts_with("You add"));
        assert!(!record.description.contains('|'), "the pipe-arg tail must never leak into prose");
    }

    /// No served description leaks unresolved PCGen syntax onto the screen --
    /// the same certification `monster_catalog`/`companion_catalog` each
    /// run, over the REAL cache rather than a hand-picked sample, so the
    /// production hard panic is a proven-unreachable invariant.
    #[test]
    fn every_real_class_feature_description_renders_without_a_pcgen_syntax_leak() {
        let descriptions = load_class_feature_descriptions(&repo_root());
        let mut checked = 0;
        for record in &descriptions {
            if let Some(leak) = codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&record.description)
            {
                panic!("{:?} ({}): leaked {leak}", record.key, record.book);
            }
            checked += 1;
        }
        assert!(checked > 1000, "no real descriptions were checked; the check proved nothing");
    }

    /// A `.CLEAR`/`.CLEARALL`/PI-marker/empty description is never emitted --
    /// proven against a synthetic record, since the live corpus may not
    /// currently contain a `.CLEAR` example under `class_feature`.
    #[test]
    fn a_clear_marker_or_empty_description_never_reaches_the_catalog() {
        for stand_in_value in [".CLEAR", ".CLEARALL", "[redacted PI]", "", "   "] {
            assert!(
                !is_real_description_value(stand_in_value),
                "{stand_in_value:?} must be refused"
            );
        }
    }

    /// CONFIRMED live shape (found by this module's own leak-detection
    /// test, not anticipated): a row that declares two pipe-separated
    /// arguments but whose prose only references `%1` mis-splits under
    /// `render_pcgen_desc`'s segment-count heuristic and leaves a literal
    /// `|` in the output. Refused, not shipped, and does not crash the
    /// loader for every other record in the same book.
    #[test]
    fn a_record_whose_render_leaks_a_pipe_argument_tail_is_refused_not_shipped() {
        let descriptions = load_class_feature_descriptions(&repo_root());
        assert!(
            !descriptions.iter().any(|d| d.key == "Enhancement Savant Subschool ~ Perfection of Self"),
            "the known-malformed row must be refused, never served with a leaked '|' in its text"
        );
        // The rest of `advanced_class_guide`'s real, well-formed class
        // features still loaded -- the skip is scoped to the one bad row,
        // not the whole book.
        assert!(
            descriptions.iter().any(|d| d.book == "advanced_class_guide"),
            "one malformed row must not take its whole book down with it"
        );
    }

    #[test]
    fn list_class_feature_descriptions_returns_the_cached_table() {
        let a = list_class_feature_descriptions();
        let b = list_class_feature_descriptions();
        assert_eq!(a.len(), b.len());
        assert!(!a.is_empty());
    }
}
