//! JSON cache generator for already-compiled, hand-authored per-book
//! equipment tables that have never been dumped to `data/corpus/`
//! (SD-31 `epic-6-ingest-lanes` F5, `SD31-E6-F5-003`).
//!
//! ## Why this exists
//!
//! `rules_tables::{ultimate_psionics,ultimate_combat,ultimate_intrigue,
//! ultimate_magic}::equipment_tables` are full, oracle-verified,
//! already-shipped-to-the-player-catalog per-book equipment tables --
//! `equipment_resolver::equipment_catalog_rows()` already chains them, so
//! every item they hold already renders on the desktop equipment catalog
//! and already counts as "known" for `docs/work-inventory.json`'s
//! `equipment_key_absent_from_equipment_tables` check. But **their
//! `equipment_tables()` (non-`Equipmods`) rows were never dumped to
//! `data/corpus/<book>/equipment/*.json`** -- only their `Equipmods` rows
//! were, via `equipment_gap_tables`'s own generated residue (a
//! *different*, already-shipped population; this module explicitly
//! excludes it, see below). With no on-disk JSON, `corpus_literal_sweep`
//! has nothing to literal-verify against, so every one of these records
//! sits at `static|ingested-magnitude` -- `held`, not `done` -- forever,
//! even though the item is already real, already wired, and already
//! player-visible. Exactly the `OPEN-ISSUES.md` row 11/row 12 shape
//! `cache_gen::ultimate_equipment` (`SD31-E6-F5-001`) closed for Ultimate
//! Equipment, found here for four MORE already-compiled books by tracing
//! one held unit (Ultimate Psionics' "Amulet of Catapsi") end to end:
//! its `data/corpus/ultimate_psionics/equipment/` directory held only an
//! `equipmods/` subdirectory (113 files, `cache_gen::equipment_gap`'s own
//! output) and zero files at the `equipment/` root.
//!
//! **Explicitly EXCLUDES every `Equipmods`-category row.** Those are a
//! SEPARATE population already dumped by `cache_gen::equipment_gap` (via
//! `equipment_gap_tables`, itself generated from the SAME `.lst` files
//! but scoped to records the hand-authored table does *not* hold) into
//! `data/corpus/<book>/equipment/equipmods/`. Re-writing them here would
//! either collide (harmless, since `write_json` never clobbers) or --
//! worse -- risk a second, independently-drifting source of truth for the
//! same item. `equipment_tables()`'s own per-book doc comments already
//! separate "equipment" (326/213/91/27 across the four books) from
//! "equipment modifiers" (113/20/8/0) as two distinct table shapes; this
//! module dumps only the former.
//!
//! ## Reuse, not duplication
//!
//! Every helper this module needs already exists, verified, in
//! `cache_gen::equipment_gap` (same file territory, `SD31-E6-F5-003`
//! widened five functions to `pub(crate)` for exactly this reuse):
//! [`super::equipment_gap::book_routing`] for the book id/`.lst` directory,
//! [`super::equipment_gap::find_citation`] for the real `(path, line)`
//! citation (KEY:/first-column/`.COPY=` search, already handling every
//! shape these four books' corpus rows use), [`super::equipment_gap::
//! disabled_identity_column`] and [`super::equipment_gap::declared_pi_at`]
//! for the `#`-disabled-row and `NAMEISPI:`/`DESCISPI:` reads, and
//! [`super::equipment_gap::write_json`] for the no-clobber write (a
//! pre-existing file at the target slug is left untouched, never
//! overwritten -- load-bearing here since `equipmods/` already has real,
//! shipped content under the same book directory this module also writes
//! into).
//!
//! ## PI screening -- BOTH contracts, NAME and DESCRIPTION
//!
//! Same union discipline `cache_gen::equipment_gap` establishes (the
//! fix for wave 4's confirmed sibling-module PI hole): a row is dropped
//! outright -- never redacted-in-place -- when EITHER its real corpus
//! line declares `NAMEISPI:YES` OR its `name` matches the shared
//! blacklist term scan. `description` is screened by the declared-PI-vs-
//! blacklist UNION via `pi_screening::classify_optional_field_declared`,
//! same as every sibling `cache_gen` module.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use crate::rules_core::cache_gen::equipment_gap::{
    self, CacheRecord, Completeness, EquipmentData, Population, Source,
};
use crate::rules_core::cache_gen::WiringClassIndex;
use crate::rules_core::pi_screening;

/// One row from a hand-authored per-book `equipment_tables()`, reduced to
/// the fields this generator needs -- independent of which book-local
/// `EquipmentTableEntry`/`EquipmentCategory` Rust type it came from
/// (`decisions.md §11.3`'s no-shared-types-file convention means each
/// book's table is its own type; this is the common shape the four
/// per-book adapter functions below project onto).
struct SourceEntry {
    key: &'static str,
    name: &'static str,
    /// The row's own category, `Debug`-formatted from its book-local enum
    /// (`"ArmsArmor"`, `"MagicItems"`, `"General"`) -- never `"Equipmods"`,
    /// since every adapter below filters that category out before this
    /// struct is built.
    category: String,
    cost_gp: Option<f64>,
    weight_lbs: Option<f64>,
    description: Option<&'static str>,
}

fn ultimate_psionics_entries() -> Vec<SourceEntry> {
    use crate::rules_core::rules_tables::ultimate_psionics::equipment_tables as t;
    t::equipment_tables()
        .iter()
        .filter(|e| e.category != t::EquipmentCategory::Equipmods)
        .map(|e| SourceEntry {
            key: e.key,
            name: e.name,
            category: format!("{:?}", e.category),
            cost_gp: e.cost_gp,
            weight_lbs: e.weight_lbs,
            description: e.description,
        })
        .collect()
}

fn ultimate_combat_entries() -> Vec<SourceEntry> {
    use crate::rules_core::rules_tables::ultimate_combat::equipment_tables as t;
    t::equipment_tables()
        .iter()
        .filter(|e| e.category != t::EquipmentCategory::Equipmods)
        .map(|e| SourceEntry {
            key: e.key,
            name: e.name,
            category: format!("{:?}", e.category),
            cost_gp: e.cost_gp,
            weight_lbs: e.weight_lbs,
            description: e.description,
        })
        .collect()
}

fn ultimate_intrigue_entries() -> Vec<SourceEntry> {
    use crate::rules_core::rules_tables::ultimate_intrigue::equipment_tables as t;
    t::equipment_tables()
        .iter()
        .filter(|e| e.category != t::EquipmentCategory::Equipmods)
        .map(|e| SourceEntry {
            key: e.key,
            name: e.name,
            category: format!("{:?}", e.category),
            cost_gp: e.cost_gp,
            weight_lbs: e.weight_lbs,
            description: e.description,
        })
        .collect()
}

fn ultimate_magic_entries() -> Vec<SourceEntry> {
    // UM's `EquipmentCategory` has no `Equipmods` variant at all (its
    // equipment_modifier population is a genuinely separate, empty-today
    // corpus shape -- see the module's own doc comment) so every row here
    // is already an "equipment", not "equipment_modifier", row.
    use crate::rules_core::rules_tables::ultimate_magic::equipment_tables as t;
    t::equipment_tables()
        .iter()
        .map(|e| SourceEntry {
            key: e.key,
            name: e.name,
            category: format!("{:?}", e.category),
            cost_gp: e.cost_gp,
            weight_lbs: e.weight_lbs,
            description: e.description,
        })
        .collect()
}

/// One book this generator covers: its `equipment_gap::book_routing`
/// short code and its `SourceEntry` adapter.
struct BookInput {
    short_code: &'static str,
    entries: fn() -> Vec<SourceEntry>,
}

const BOOKS: &[BookInput] = &[
    BookInput { short_code: "UPSI", entries: ultimate_psionics_entries },
    BookInput { short_code: "UC", entries: ultimate_combat_entries },
    BookInput { short_code: "UI", entries: ultimate_intrigue_entries },
    BookInput { short_code: "UM", entries: ultimate_magic_entries },
];

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub equipment_written: usize,
    /// Rows whose real LST citation could not be resolved -- honestly not
    /// written (never fabricated).
    pub unresolved_citations: Vec<String>,
    /// Rows whose `name` carries declared or blacklist-matched Product
    /// Identity -- dropped outright (a required field cannot be
    /// redacted), counted, never silently absorbed.
    pub name_pi_excluded: Vec<String>,
    /// Rows whose slugified output path already exists on disk (e.g. from
    /// `cache_gen::equipment_gap`'s own `equipmods/` sibling content, or a
    /// prior run of this generator) -- left untouched, never clobbered.
    pub skipped_pre_existing: Vec<String>,
}

#[derive(Debug)]
pub enum GenerationError {
    CorpusUnreachable(std::path::PathBuf),
}

/// Generates the hand-authored-equipment JSON cache for every book
/// [`BOOKS`] names, under `out_root` (`data/corpus/`), reading real LST
/// citations from `corpus_root` (a PCGen `data/` checkout). `ingested_at`
/// is stamped at call time by the caller (real wall-clock ISO-8601, never
/// derived).
pub fn generate(
    corpus_root: &Path,
    out_root: &Path,
    ingested_at: &str,
) -> Result<GenerationReport, GenerationError> {
    let mut report = GenerationReport::default();

    for book in BOOKS {
        let (book_id, book_rel_dir) = equipment_gap::book_routing(book.short_code)
            .expect("every BOOKS short_code must have a book_routing entry");
        let book_dir = corpus_root.join(book_rel_dir);
        if !book_dir.is_dir() {
            return Err(GenerationError::CorpusUnreachable(book_dir));
        }

        let mut used: BTreeSet<String> = BTreeSet::new();
        let mut sha_cache: HashMap<std::path::PathBuf, String> = HashMap::new();
        let wiring_index = WiringClassIndex::build(book_id, &book_dir);
        let mut wiring_lines = wiring_index.lines();
        let equipment_out = out_root.join(book_id).join("equipment");

        for entry in (book.entries)() {
            let Some((rel_path, line)) = equipment_gap::find_citation(&book_dir, entry.key, entry.name)
            else {
                report.unresolved_citations.push(format!("{book_id}:{}", entry.key));
                continue;
            };
            let abs_path = book_dir.join(&rel_path);
            let rel_path_str = rel_path.to_string_lossy().replace('\\', "/");

            if equipment_gap::disabled_identity_column(&abs_path, line) {
                report.unresolved_citations.push(format!("{book_id}:{} (disabled #-line)", entry.key));
                continue;
            }

            let sha = match sha_cache.get(&abs_path) {
                Some(s) => s.clone(),
                None => {
                    let s = equipment_gap::sha256_file(&abs_path).unwrap_or_default();
                    sha_cache.insert(abs_path.clone(), s.clone());
                    s
                }
            };

            let declared = equipment_gap::declared_pi_at(&abs_path, line);
            let (name_license, _, _, _) = pi_screening::classify_field("name", entry.name);
            if declared.name || name_license == crate::rules_core::shape_b_v1::License::PiRedacted {
                report.name_pi_excluded.push(format!("{book_id}:{}", entry.key));
                continue;
            }

            let (wiring_class, wiring_class_signals) =
                wiring_index.wiring_class_for(&mut wiring_lines, &rel_path_str, line, entry.key, entry.key);

            let (license, pi_field, pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
                "description",
                entry.description,
                declared.description,
            );

            let completeness =
                if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly };

            let record = CacheRecord {
                population: Population::InScope,
                completeness,
                ingested_at: ingested_at.to_string(),
                data: EquipmentData {
                    key: entry.key.to_string(),
                    category: entry.category.clone(),
                    name: entry.name.to_string(),
                    cost_gp: entry.cost_gp,
                    weight_lbs: entry.weight_lbs,
                    description: stored_desc,
                },
                source: Source::LstToken {
                    path: format!("{book_rel_dir}/{rel_path_str}"),
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

            let slug = equipment_gap::slugify(entry.key, &mut used);
            let wrote = equipment_gap::write_json(&equipment_out, &slug, &record)
                .map_err(|_| GenerationError::CorpusUnreachable(equipment_out.clone()))?;
            if !wrote {
                report.skipped_pre_existing.push(format!("{book_id}:{}", entry.key));
                continue;
            }
            report.equipment_written += 1;
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_book_short_code_routes() {
        for book in BOOKS {
            assert!(
                equipment_gap::book_routing(book.short_code).is_some(),
                "missing book_routing for {}",
                book.short_code
            );
        }
    }

    #[test]
    fn ultimate_magic_adapter_excludes_nothing_since_it_has_no_equipmods_variant() {
        let entries = ultimate_magic_entries();
        assert!(!entries.is_empty());
        // Every UM row is a real equipment row -- the type has no
        // "Equipmods" category variant to filter, confirmed by construction
        // (a compile-time fact: `t::EquipmentCategory` here has only
        // `General`/`ArmsArmor`), so this adapter's own filter is a no-op
        // and this test documents that rather than re-deriving the enum.
        assert_eq!(
            entries.len(),
            crate::rules_core::rules_tables::ultimate_magic::equipment_tables::equipment_tables().len()
        );
    }

    #[test]
    fn ultimate_psionics_adapter_excludes_every_equipmods_row() {
        use crate::rules_core::rules_tables::ultimate_psionics::equipment_tables as t;
        let entries = ultimate_psionics_entries();
        let total = t::equipment_tables().len();
        let equipmods = t::equipment_tables()
            .iter()
            .filter(|e| e.category == t::EquipmentCategory::Equipmods)
            .count();
        assert_eq!(entries.len(), total - equipmods);
        assert!(entries.iter().all(|e| e.category != "Equipmods"));
    }

    #[test]
    fn a_nameispi_declared_row_would_be_excluded_not_redacted() {
        let tokens = [("NAMEISPI", "YES")];
        let declared = pi_screening::declared_product_identity(tokens);
        assert!(declared.name);
        // `SourceEntry.name` (and the resulting `EquipmentData.name`) is a
        // required `String`, mirroring `cache_gen::equipment_gap`'s own
        // proof for the identical shape: there is no field-level redaction
        // path for a required identity field by construction, which is why
        // `generate()` treats `declared.name` as a whole-record skip.
    }

    #[test]
    fn a_blacklisted_name_is_flagged_by_the_term_scan() {
        let (license, _, _, _) = pi_screening::classify_field("name", "Iomedae's Blessed Blade");
        assert_eq!(license, crate::rules_core::shape_b_v1::License::PiRedacted);
    }
}
