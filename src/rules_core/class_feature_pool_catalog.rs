//! Option-pool `class_feature` browsable catalog (SD31-W22-POOLMEMBER-001).
//!
//! # Why this module exists
//!
//! `class_feature_effect_wired` / `CLASS_FEATURE_POOLS`
//! (`v06_work_inventory.rs`) already prove, per record, whether SELECTING a
//! specific option-pool member (a rogue talent, a bloodline power, a witch
//! hex, ...) moves an observable engine fact. That answers "is this record's
//! magnitude computed" for the minority of pool members whose selection
//! changes something the engine renders.
//!
//! It cannot answer Decision 7's question for the majority: a genuinely
//! prose-only, zero-magnitude pool member (e.g. Rogue Talent ~ Ledge
//! Walker: "you move along narrow surfaces at full speed") never changes any
//! observable fact whether or not it is selected — there is nothing to
//! compute — so the consumer-delta probe correctly reports
//! `NoConsumerDelta`/"not held", and `Kind::ClassFeature`'s own doc comment
//! (`v06_work_inventory.rs`, the `class_feature_owner_matched_by_name_but_
//! record_not_held_by_engine` branch) names exactly the missing precondition
//! for `text-complete`: **"no generic class_feature catalog exists anywhere
//! in this engine, unlike feat/spell/equipment"** (`decisions.md §42`,
//! `SD28-E24`). `feat`/`spell`/`equipment` each have a real catalog that
//! serves every record's description to a player regardless of whether that
//! record is currently held — a browsable reference, not a per-character
//! computation. This module is that catalog for `class_feature` option-pool
//! records, built for ONE pool (`REGISTERED_POOL_GROUPS`) as the dispatch
//! brief asked: a precise answer on one pool, not a stub across all of them.
//!
//! # Scope: Rogue Talent only, deliberately
//!
//! `v06_work_inventory.rs`'s `CLASS_FEATURE_POOLS` registers 27 pools.
//! Widening `REGISTERED_POOL_GROUPS` to all of them is mechanical — the same
//! walk, the same render-and-refuse gate — but each pool's corpus rows would
//! need the same one-pool spot-check this cycle ran on Rogue Talent (are the
//! `%N` argument shapes the same, is `data.class` really the bare pool name
//! for every book that prints it) before being trusted at scale. Left named,
//! not built, per the dispatch's own "report what it would cost to extend"
//! ask.
//!
//! # The render-and-refuse gate is the whole safety property
//!
//! A pool member's corpus `description` is the RAW, unresolved `.lst` `DESC:`
//! string — for a record like `Rogue Talent ~ Bleeding Attack`, that string
//! is `"...take %1 additional points of damage...|SneakAttackDice"`:
//! `SneakAttackDice` is a bare cross-reference to a character-specific value
//! this catalog has no character to resolve against, so
//! `wiring_class::has_prose_formula_segment` (deliberately) leaves it
//! undetermined rather than guessing, and Decision 7's condition 2 ("nothing
//! to compute") genuinely fails for it — a player cannot read a complete
//! sentence without the engine computing a number this catalog is not given.
//! [`render_pcgen_desc`] already reports exactly this as a dropped `%N`
//! argument; this module refuses to serve any record whose render drops one,
//! which is simultaneously the leak guard every sibling catalog
//! (`monster_catalog`, `companion_catalog`, `class_feature_descriptions`)
//! already runs AND the correct Decision-7 disposition for a record that
//! genuinely still needs a computation. The two never conflict here.
//!
//! # PI screening
//!
//! Already discharged upstream, same trust boundary as
//! `class_feature_descriptions.rs`: `cache_gen::class_feature::generate`
//! screens NAME and DESCRIPTION (SD-30 `§52.3`/`§53.5`) before a record is
//! ever written to `data/corpus/`. This module reads only that
//! already-screened output and re-runs no PI check of its own.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::rules_core::pcgen_desc::{leaked_pcgen_syntax, render_pcgen_desc};

/// Corpus `data.class` values this catalog recognises as an option-pool
/// group rather than a real engine-modelled class. See the module doc for
/// why the list is one entry long today.
pub const REGISTERED_POOL_GROUPS: &[&str] = &["Rogue Talent"];

/// One option-pool member's real corpus row, with a description proven to
/// render with nothing missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolCatalogEntry {
    /// The corpus book directory this record was read from.
    pub book: String,
    /// The registered pool group this record belongs to (`data.class`,
    /// verbatim — e.g. `"Rogue Talent"`).
    pub pool_group: String,
    /// The corpus `KEY:` token verbatim (e.g. `"Rogue Talent ~ Ledge
    /// Walker"`).
    pub key: String,
    pub name: String,
    /// Rendered through [`render_pcgen_desc`], with every unsubstituted
    /// `%N` argument refused rather than served (see the module doc's
    /// "render-and-refuse" section). Never empty, `.CLEAR`/`.CLEARALL`, or
    /// the PI-redaction marker — those never reach this struct at all.
    pub description: String,
}

/// Reproduced from `v06_work_inventory.rs`/`class_feature_descriptions.rs`'s
/// own copies — this crate's disjoint-file-touch convention, so a
/// consumer-territory module never has to coordinate an edit with an
/// ingest-territory one for a three-line predicate.
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

/// Reads every already-ingested `class_feature` cache record under
/// `<repo_root>/data/corpus/*/class_feature/**/*.json` whose `data.class`
/// names a [`REGISTERED_POOL_GROUPS`] entry, keeping only the ones whose
/// description renders with nothing missing (see the module doc's
/// render-and-refuse gate). Reads a NEW tree of nothing — every record
/// already lives in the committed `data/corpus/` cache
/// `cache_gen::class_feature::generate` writes; this module adds no new
/// corpus data of its own, only a new reading of what already exists.
pub fn load_pool_catalog(repo_root: &Path) -> Vec<PoolCatalogEntry> {
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
            if !REGISTERED_POOL_GROUPS.contains(&class) {
                continue;
            }
            let Some(raw_desc) = data["description"].as_str() else { continue };
            if !is_real_description_value(raw_desc) {
                continue;
            }
            let rendered = render_pcgen_desc(raw_desc);
            // The render-and-refuse gate: an unresolved `%N` means a real
            // computation this catalog cannot perform is still missing from
            // the sentence, which fails Decision 7's condition 2 (`nothing
            // to compute`) at the same time it would leak broken syntax.
            if !rendered.dropped_args.is_empty() {
                continue;
            }
            if leaked_pcgen_syntax(&rendered.text).is_some() {
                continue;
            }
            out.push(PoolCatalogEntry {
                book: book.clone(),
                pool_group: class.to_string(),
                key: key.to_string(),
                name: name.to_string(),
                description: rendered.text,
            });
        }
    }
    out
}

/// `(book, key) -> description` for every entry the catalog holds — the
/// shape `v06_work_inventory.rs`'s `EngineFacts` (and `Kind::ClassFeature`'s
/// classify arm) actually consults, mirroring `feat_served_descriptions`'
/// own `(book, key)` indexing.
pub fn pool_catalog_index(entries: &[PoolCatalogEntry]) -> BTreeMap<(String, String), String> {
    entries.iter().map(|e| ((e.book.clone(), e.key.clone()), e.description.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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

    /// The real corpus loads real, clean Rogue Talent records — proven
    /// against the live `data/corpus/` checkout, not a fixture. `Ledge
    /// Walker` is genuinely prose-only in the pinned oracle (no `%N`
    /// substitution, no `BONUS:`/`DEFINE:` token) and must be served intact.
    #[test]
    fn loads_a_real_clean_rogue_talent_from_the_live_corpus() {
        let entries = load_pool_catalog(&repo_root());
        let ledge_walker = entries
            .iter()
            .find(|e| e.book == "core_rulebook" && e.key == "Rogue Talent ~ Ledge Walker")
            .expect("core_rulebook's real Rogue Talent ~ Ledge Walker record must be in the catalog");
        assert_eq!(ledge_walker.pool_group, "Rogue Talent");
        assert_eq!(ledge_walker.name, "Ledge Walker");
        assert!(ledge_walker.description.starts_with("This ability allows you to move"));
        assert!(!ledge_walker.description.contains('|'), "no pipe-arg tail may leak into prose");
        assert!(!ledge_walker.description.contains('%'), "no unsubstituted argument may leak into prose");
    }

    /// The render-and-refuse gate's whole point: `Bleeding Attack`'s only
    /// magnitude is a bare cross-reference (`SneakAttackDice`) this catalog
    /// cannot resolve, so it must never be served — refused, not shipped
    /// with a dropped `%1` or a guessed number.
    #[test]
    fn bleeding_attack_is_refused_for_an_unresolvable_percent_argument() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            !entries.iter().any(|e| e.key == "Rogue Talent ~ Bleeding Attack"),
            "a record whose render drops a %N argument must never reach the catalog"
        );
        // The refusal is scoped to the one record, not the whole book.
        assert!(entries.iter().any(|e| e.book == "core_rulebook"));
    }

    /// Only registered pool groups are served — a real `Bloodline` or
    /// `Hex` record (unregistered today) must never appear, proving the
    /// scope guard is a real filter and not merely documentation.
    #[test]
    fn unregistered_pool_groups_are_never_served() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            entries.iter().all(|e| e.pool_group == "Rogue Talent"),
            "only REGISTERED_POOL_GROUPS may appear in the catalog"
        );
        assert!(
            !entries.iter().any(|e| e.key.starts_with("Bloodline ~ ") || e.key.starts_with("Hex ~ ")),
            "an unregistered pool's records must never leak into the catalog"
        );
    }

    /// No served description leaks unresolved PCGen syntax onto the screen
    /// — the same certification every sibling catalog runs, over the real
    /// cache rather than a hand-picked sample.
    #[test]
    fn every_served_description_renders_without_a_pcgen_syntax_leak() {
        let entries = load_pool_catalog(&repo_root());
        let mut checked = 0;
        for entry in &entries {
            if let Some(leak) = leaked_pcgen_syntax(&entry.description) {
                panic!("{:?} ({}): leaked {leak}", entry.key, entry.book);
            }
            checked += 1;
        }
        assert!(checked > 10, "no real descriptions were checked; the check proved nothing");
    }

    #[test]
    fn pool_catalog_index_is_keyed_by_book_and_key() {
        let entries = load_pool_catalog(&repo_root());
        let index = pool_catalog_index(&entries);
        assert_eq!(
            index.get(&("core_rulebook".to_string(), "Rogue Talent ~ Ledge Walker".to_string())),
            Some(&"This ability allows you to move along narrow surfaces at full speed using the Acrobatics skill without penalty. In addition, you are not flat-footed when using Acrobatics to move along narrow surfaces.".to_string())
        );
        assert!(index.get(&("core_rulebook".to_string(), "Rogue Talent ~ Bleeding Attack".to_string())).is_none());
    }
}
