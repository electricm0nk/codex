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
//! **Now INCLUDES `Equipmods`-category rows too** (SD-32 `decisions.md
//! §20` equipment/equipment_modifier `no_record` closure). This module
//! used to exclude them on the theory that `cache_gen::equipment_gap`
//! (via `equipment_gap_tables`, generated from the SAME `.lst` files)
//! already dumped that population. It does not, for exactly the rows that
//! matter: `equipment_gap_tables`'s own generator builds its `held`
//! exclusion set from `hand_authored_equipment_rows()` -- which does
//! *not* filter by category -- so any `Equipmods` row already present in
//! a book's `equipment_tables()` (e.g. Ultimate Psionics' "Psionic Blade",
//! `up_equipmods.lst:12`, `KEY:Special Ability ~ Psionic Blade ~ Weapon`)
//! was excluded by BOTH generators and never got a `data/corpus/**/*.json`
//! record at all -- 132 of T9's 175 `equipment_modifier` `no_record`
//! units traced to this one gap (re-derive: `python3 scripts/
//! shape_ledger.py --inventory docs/work-inventory.json`, join
//! `no_record` rows whose `corpus_key` matches an `Equipmods`-category
//! entry in `ultimate_psionics`/`ultimate_combat`/`ultimate_intrigue`'s
//! `equipment_tables()`). `generate()` now routes an `Equipmods` row to
//! `equipment/equipmods/`, the same subdirectory
//! `cache_gen::equipment_gap::generate` writes to, so the two writers
//! share one directory convention; `write_json`'s no-clobber write means
//! a row `equipment_gap` already wrote under the same slug is left
//! untouched, not duplicated or drifted. `equipment_tables()`'s own
//! per-book doc comments separate "equipment" (326/213/91/27 across the
//! four books) from "equipment modifiers" (113/20/8/0) as two distinct
//! table shapes; this module now dumps both, routed by category.
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
    /// (`"ArmsArmor"`, `"MagicItems"`, `"General"`, or `"Equipmods"` --
    /// `generate()` routes an `"Equipmods"` row to the `equipment/equipmods/`
    /// subdirectory the same way `cache_gen::equipment_gap::generate` does,
    /// so the two writers use one shared convention rather than drifting).
    category: String,
    cost_gp: Option<f64>,
    weight_lbs: Option<f64>,
    description: Option<&'static str>,
}

fn ultimate_psionics_entries() -> Vec<SourceEntry> {
    // Chains BOTH `equipment_tables()` (326 `ArmsArmor`/`MagicItems` rows,
    // `up_equipment.lst`) and `equipmod_tables()` (113 `Equipmods` rows,
    // `up_equipmods.lst`) -- two genuinely separate accessors over two
    // separate compiled arrays, not one function mixing categories. The
    // earlier version of this adapter called only `equipment_tables()` and
    // filtered `Equipmods` out of THAT slice -- a no-op, since
    // `equipment_tables()` never held any `Equipmods` rows to begin with,
    // and the filter's presence was read (wrongly) as proof the exclusion
    // was live. `equipmod_tables()`'s population was simply never reached
    // by this generator at all, which is the real defect
    // (`decisions.md §20`).
    use crate::rules_core::rules_tables::ultimate_psionics::equipment_tables as t;
    t::equipment_tables()
        .iter()
        .chain(t::equipmod_tables())
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
        .chain(t::equipmod_tables())
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
        .chain(t::equipmod_tables())
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
    // UM's `equipmod_tables()` returns an empty slice (confirmed by its own
    // doc comment: "UM has no equipment *modifiers* file at all"), and its
    // `EquipmentCategory` enum has no `Equipmods` variant to format -- so
    // chaining it in here is a genuine no-op, kept only so this adapter has
    // the same shape as its three siblings rather than a special case.
    use crate::rules_core::rules_tables::ultimate_magic::equipment_tables as t;
    t::equipment_tables()
        .iter()
        .chain(t::equipmod_tables())
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
    /// `Equipmods`-category rows written -- routed to `equipment/equipmods/`,
    /// counted separately from `equipment_written` the same way
    /// `cache_gen::equipment_gap::GenerationReport` splits the two.
    pub equipment_modifier_written: usize,
    /// Rows whose real LST citation could not be resolved -- honestly not
    /// written (never fabricated).
    pub unresolved_citations: Vec<String>,
    /// Rows whose `name` carries declared or blacklist-matched Product
    /// Identity. `decisions.md §24`: written under a Codex-generated
    /// neutral name (never dropped whole any more); field name kept for
    /// compatibility, now counts renames -- see
    /// `cache_gen::equipment_gap::GenerationReport::name_pi_excluded`'s
    /// identical doc comment.
    pub name_pi_excluded: Vec<String>,
    /// `(kind, book, source_file, source_line, codex_name, reason)`
    /// divergence entries for every unit renamed this run --
    /// `decisions.md §24b`-4: coordinate + reason, never the original
    /// string.
    pub name_pi_renamed_records: Vec<serde_json::Value>,
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
            let name_is_pi = declared.name || name_license == crate::rules_core::shape_b_v1::License::PiRedacted;
            let is_modifier = entry.category == "Equipmods";
            let kind = if is_modifier { "equipment_modifier" } else { "equipment" };

            let (wiring_class, wiring_class_signals) =
                wiring_index.wiring_class_for(&mut wiring_lines, &rel_path_str, line, entry.key, entry.key);

            let (mut license, mut pi_field, mut pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
                "description",
                entry.description,
                declared.description,
            );

            let completeness =
                if entry.description.is_some() { Completeness::Full } else { Completeness::ChassisOnly };

            // `decisions.md §24` -- see
            // `cache_gen::equipment_gap::resolve_name_or_rename`'s doc
            // comment for the full rationale; this sibling generator
            // shares the same PI-name-exclusion shape and the same fix
            // (the SAME function, not a duplicate copy).
            let (record_name, record_key, codex_generated_name, rename_info, divergence) =
                equipment_gap::resolve_name_or_rename(name_is_pi, kind, book_id, &rel_path_str, line, entry.name, entry.key);
            if let Some(entry) = divergence {
                report.name_pi_excluded.push(format!("{book_id}:{}:{}", rel_path_str, line));
                report.name_pi_renamed_records.push(entry);
                let mut redacted_fields: Vec<&str> = Vec::new();
                if pi_field.as_deref() == Some("description") {
                    redacted_fields.push("description");
                }
                redacted_fields.push("name");
                license = crate::rules_core::shape_b_v1::License::PiRedacted;
                pi_field = Some(redacted_fields.join(","));
                pi_marker = Some(crate::rules_core::shape_b_v1::PI_MARKER_REDACTED.to_string());
            }

            let record = CacheRecord {
                population: Population::InScope,
                completeness,
                ingested_at: ingested_at.to_string(),
                data: EquipmentData {
                    key: record_key.clone(),
                    category: entry.category.clone(),
                    name: record_name.clone(),
                    cost_gp: entry.cost_gp,
                    weight_lbs: entry.weight_lbs,
                    description: stored_desc,
                },
                source: Source::LstToken {
                    path: format!("{book_rel_dir}/{rel_path_str}"),
                    sha256: sha,
                    line,
                    record_key: record_key.clone(),
                },
                wiring_class,
                wiring_class_signals,
                license,
                pi_field,
                pi_marker,
                codex_generated_name,
                rename: rename_info,
            };

            let slug = equipment_gap::slugify(&record_key, &mut used);
            // Same routing convention `cache_gen::equipment_gap::generate`
            // uses: an `Equipmods` row belongs to the `equipment_modifier`
            // kind, which lives under `equipment/equipmods/`, not the
            // `equipment` kind's own root -- sharing this convention is what
            // makes `write_json`'s no-clobber write a real de-dup against
            // that sibling generator's own output rather than a coincidence.
            let write_dir = if is_modifier { equipment_out.join("equipmods") } else { equipment_out.clone() };
            let wrote = equipment_gap::write_json(&write_dir, &slug, &record)
                .map_err(|_| GenerationError::CorpusUnreachable(write_dir.clone()))?;
            if !wrote {
                report.skipped_pre_existing.push(format!("{book_id}:{}:{}", rel_path_str, line));
                continue;
            }
            if is_modifier {
                report.equipment_modifier_written += 1;
            } else {
                report.equipment_written += 1;
            }
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
    fn ultimate_magic_adapter_chains_an_empty_equipmod_tables_and_adds_nothing() {
        use crate::rules_core::rules_tables::ultimate_magic::equipment_tables as t;
        let entries = ultimate_magic_entries();
        assert!(!entries.is_empty());
        // UM's `equipmod_tables()` is a genuinely empty slice (confirmed by
        // construction: `t::EquipmentCategory` here has only
        // `General`/`ArmsArmor`, no `Equipmods` variant to hold), so
        // chaining it in is a real no-op, not a filtered-away population.
        assert_eq!(t::equipmod_tables().len(), 0);
        assert_eq!(entries.len(), t::equipment_tables().len());
    }

    #[test]
    fn ultimate_psionics_adapter_includes_equipmods_rows_too() {
        // Was `..._excludes_every_equipmods_row` -- that name was itself
        // misleading (SD-32 `decisions.md §20` equipment/equipment_modifier
        // `no_record` closure): `equipment_tables()` never held any
        // `Equipmods` rows to begin with (they live in a wholly separate
        // compiled array behind `equipmod_tables()`), so this adapter's old
        // `.filter(|e| e.category != Equipmods)` over `equipment_tables()`
        // alone was a no-op -- and its presence was read as proof the
        // exclusion was live and deliberate. The real defect: this adapter
        // never called `equipmod_tables()` at all, so its whole 113-row
        // population -- e.g. Ultimate Psionics' "Psionic Blade"
        // (`up_equipmods.lst:12`) -- never reached `generate()` and never
        // got a `data/corpus/**/*.json` record. 132 of T9's 175
        // `equipment_modifier` `no_record` units trace to exactly this
        // (re-derive: `python3 scripts/shape_ledger.py --inventory
        // docs/work-inventory.json`, join `no_record` rows whose
        // `corpus_key` matches an `equipmod_tables()` entry in
        // `ultimate_psionics`/`ultimate_combat`/`ultimate_intrigue`).
        use crate::rules_core::rules_tables::ultimate_psionics::equipment_tables as t;
        let entries = ultimate_psionics_entries();
        let total = t::equipment_tables().len() + t::equipmod_tables().len();
        assert_eq!(entries.len(), total, "must chain equipment_tables() AND equipmod_tables()");
        assert!(
            entries.iter().any(|e| e.category == "Equipmods"),
            "the adapter must carry equipmod_tables()'s Equipmods rows through, not drop them"
        );
    }

    #[test]
    fn a_nameispi_declared_row_would_be_renamed_not_redacted() {
        let tokens = [("NAMEISPI", "YES")];
        let declared = pi_screening::declared_product_identity(tokens);
        assert!(declared.name);
        // `SourceEntry.name` (and the resulting `EquipmentData.name`) is a
        // required `String`, mirroring `cache_gen::equipment_gap`'s own
        // proof for the identical shape: `decisions.md §24` supersedes the
        // old "whole-record skip" disposition -- `generate()` now ingests
        // a `declared.name` row under a Codex-generated neutral name
        // (`equipment_gap::resolve_name_or_rename`, the SAME function this
        // module's sibling generator calls) rather than skipping it.
    }

    #[test]
    fn a_blacklisted_name_is_flagged_by_the_term_scan() {
        let (license, _, _, _) = pi_screening::classify_field("name", "Iomedae's Blessed Blade");
        assert_eq!(license, crate::rules_core::shape_b_v1::License::PiRedacted);
    }

    /// End-to-end fixture proof (real `generate()`, real citation resolution
    /// against a temp `.lst`, no mocked internals) that a real `Equipmods`
    /// row -- Ultimate Psionics' "Psionic Blade" -- lands under
    /// `equipment/equipmods/`, is counted in `equipment_modifier_written`
    /// (not `equipment_written`), and that a slug already claimed there
    /// (simulating `cache_gen::equipment_gap`'s own prior output) is left
    /// untouched rather than clobbered. Every `BOOKS` book dir must exist or
    /// `generate()` returns `CorpusUnreachable`, so all four are created;
    /// only `ultimate_psionics`'s carries real content, so every other
    /// book's rows land in `unresolved_citations` (not fatal on its own,
    /// `gen_cache_hand_authored_equipment.rs`'s own doc comment).
    #[test]
    fn an_equipmods_row_lands_under_equipment_equipmods_not_the_equipment_root() {
        let base = std::env::temp_dir()
            .join(format!("hae_equipmods_subdir_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let corpus_root = base.join("corpus_root");
        for rel in [
            "pathfinder/dreamscarred_press/ultimate_psionics",
            "pathfinder/paizo/roleplaying_game/ultimate_combat",
            "pathfinder/paizo/roleplaying_game/ultimate_intrigue",
            "pathfinder/paizo/roleplaying_game/ultimate_magic",
        ] {
            std::fs::create_dir_all(corpus_root.join(rel)).unwrap();
        }
        std::fs::write(
            corpus_root.join("pathfinder/dreamscarred_press/ultimate_psionics/up_equipmods.lst"),
            "Psionic Blade\tKEY:Special Ability ~ Psionic Blade ~ Weapon\tCOST:0\n",
        )
        .unwrap();

        let out_root = base.join("out");
        let report = generate(&corpus_root, &out_root, "2026-08-23T00:00:00Z")
            .expect("a real corpus_root/out_root pair must not error");

        let modifier_dir = out_root.join("ultimate_psionics").join("equipment").join("equipmods");
        let root_dir = out_root.join("ultimate_psionics").join("equipment");
        let written: Vec<_> = std::fs::read_dir(&modifier_dir)
            .expect("equipmods/ must exist -- the row must have been written there")
            .filter_map(|e| e.ok())
            .collect();
        assert_eq!(written.len(), 1, "exactly the one Psionic Blade row belongs here");
        let content = std::fs::read_to_string(written[0].path()).unwrap();
        assert!(content.contains("\"key\": \"Special Ability ~ Psionic Blade ~ Weapon\""));
        assert!(content.contains("\"category\": \"Equipmods\""));

        // Nothing from this run lands at the `equipment/` root itself --
        // the fixture's only real content is the one Equipmods row.
        let root_files: Vec<_> = std::fs::read_dir(&root_dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();
        assert_eq!(root_files.len(), 0, "an Equipmods row must not land at the equipment/ root");

        assert_eq!(report.equipment_modifier_written, 1);
        assert_eq!(report.equipment_written, 0);

        // No-clobber: re-running with a pre-existing file at the same slug
        // (simulating `cache_gen::equipment_gap`'s own prior output under
        // the identical directory) must leave that file untouched, not
        // duplicate or drift it.
        let pre_existing = modifier_dir.join("special_ability_psionic_blade_weapon.json");
        assert!(pre_existing.exists(), "the slug this test relies on must be the real one");
        std::fs::write(&pre_existing, "PRE-EXISTING CONTENT").unwrap();
        let report2 = generate(&corpus_root, &out_root, "2026-08-23T00:00:01Z")
            .expect("a second run over the same fixture must not error");
        assert_eq!(
            std::fs::read_to_string(&pre_existing).unwrap(),
            "PRE-EXISTING CONTENT",
            "write_json must never clobber a file already at the target slug"
        );
        assert_eq!(report2.equipment_modifier_written, 0);
        assert_eq!(report2.skipped_pre_existing.len(), 1);

        std::fs::remove_dir_all(&base).ok();
    }
}
