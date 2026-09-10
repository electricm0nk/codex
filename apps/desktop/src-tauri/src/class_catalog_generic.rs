//! SD-32 T12 Epic 10 row 20 cycle 4 — generic BAB/save progression chassis
//! for the conventional PC classes spread across the 14 `classes`-family
//! books, for the reference-library browser.
//!
//! # Why generic, not 62 hand-authored tables
//!
//! `decisions.md §17` ("stop treating every object as a snowflake... a
//! generic ingest already exists") rules directly against the shape the
//! existing `rules_tables::crb::class_tables` layer used (137,002
//! hand-authored lines across the CRB/PU classes). Every conventional class
//! carries its own base-attack-bonus and three base-save progressions, so one
//! generic function serves them all rather than 62 per-class match arms.
//!
//! # SD-35 `AT-35-E6-001`: read the CONVERTED chassis, not a PCGen token
//!
//! Until that cycle this module walked `data/corpus/<book>/class/*.json`,
//! pulled the `BONUS:COMBAT|BASEAB` / `BONUS:SAVE` formula STRINGS out of
//! `raw_tokens`, and ran them through the PCGen formula interpreter at
//! browse time — a second, parallel copy of the same derivation
//! `pilot_compute::generic_class_chassis` was doing for character creation.
//! `decisions.md` §11 puts an end to reading a PCGen token on the live side:
//! conversion happens at ingest, and
//! `codex::rules_core::pilot_compute::class_chassis_sheet_rules` is the ONE
//! live reader of the converted `data/sheet_rules/<book>/class/<slug>.json`
//! chassis. This module now calls it, which also collapses the two parallel
//! copies into one derivation — the desynchronization risk both modules'
//! doc comments used to warn about is gone rather than warned about.
//!
//! Everything the old per-record reading logic decided is decided by the
//! converted record itself now:
//!
//!   * **BASEAB disambiguation.** Only `ultimate_intrigue/vigilante.json`
//!     states two base-attack progressions (PCGen's Vigilante social/combat
//!     identity toggle). Its converted record keeps both, distinguished by
//!     their own gates, and the principal row is the moderate default —
//!     the same row the old `,0` (toggle-off) heuristic picked. Still pinned
//!     by `exactly_one_class_needs_baseab_disambiguation` below.
//!   * **The level ceiling.** `MAXLEVEL` converts to an `applies` gate
//!     (`ClassLevel(slug) <= n`); a record stating none defaults to 10 for a
//!     prestige class and 20 otherwise, exactly as before. Ulfen Guard, the
//!     record that states none, is still pinned below.
//!   * **Demoniac resolves.** Its bare `classlevel()` was the one formula the
//!     run-time interpreter's grammar refused; the converter reads it, so
//!     there is no `unresolved` population left to name.
//!
//! # Population: 62, and why it is not the old 61
//!
//! Two independent movements, both re-derivable:
//!
//!   * **+2** — `adventurers_guide`'s Pathfinder Delver and Pathfinder
//!     Savant. `data/corpus/adventurers_guide/class/` holds 9 records and
//!     neither of these; the converter reads the pinned oracle corpus
//!     directly, so `data/sheet_rules/` carries both with a complete chassis.
//!   * **−1** — `inner_sea_gods`'s Evangelist. Its converted record carries a
//!     degradation on another of its own tokens, and the converter drops every
//!     magnitude on a degraded record to the rule's own WORDS rather than
//!     folding a partly-read number into a sheet total. Under `decisions.md`
//!     §1 that record is done as prose; it is not a chassis, and this module
//!     refuses it rather than inventing one.
//!
//! # Reachability, honestly scoped
//!
//! This module builds the progression TABLE — the same artifact
//! `class_tables()` and `pathfinder_unchained::class_chassis` already are
//! for the CRB/PU classes, and what `class_catalog.rs`'s own doc comment
//! names as "the 16 APG/ACG classes are still absent... a separate piece
//! of work with its own row-count expectations." It does **not** wire a
//! character-creation-time `ClassId` picker (that touches
//! `character_hub.rs`/`pf1_adapter.rs`, live territory this cycle stayed
//! out of per the cycle 3 receipt's own coordination discipline) — that is
//! real, separate, cross-file work for a later cycle. Recorded here rather
//! than silently narrowed: the catalog browser reads every one of the 61
//! today; character creation does not yet.

use std::path::Path;

use codex::rules_core::pilot_compute::class_chassis_sheet_rules;

use crate::class_catalog::ClassCatalogEntryDto;

/// The 14 `classes`-family book directories that hold at least one
/// conventional PC class. Shared, deliberately identical to
/// `codex::rules_core::pilot_compute::generic_class_chassis`'s own list --
/// the two modules serve the same population, one for the browser and one for
/// character creation.
const CLASS_FAMILY_BOOKS: [&str; 14] = [
    "adventurers_guide",
    "book_of_the_damned_volume_1",
    "book_of_the_damned_volume_2",
    "inner_sea_combat",
    "inner_sea_gods",
    "inner_sea_intrigue",
    "inner_sea_magic",
    "inner_sea_world_guide",
    "occult_adventures",
    "ultimate_combat",
    "ultimate_intrigue",
    "ultimate_magic",
    "ultimate_wilderness",
    "ultimate_psionics",
];

/// One conventional class's evaluated progression, still in the raw record
/// form the module test suite checks before it is mapped into
/// `ClassCatalogEntryDto` rows.
pub struct GenericClassRecord {
    /// Provenance only -- not read by `generic_class_catalog_entries` today
    /// (the catalog DTO has no book column, matching `class_tables()`'s own
    /// shape), kept for a future consumer/diagnostic rather than dropped.
    #[allow(dead_code)]
    pub book: String,
    pub name: String,
    /// The converted record's own slug -- the `"class:<slug>"` id
    /// `compute_class_chassis` dispatches on. Not always `slug(name)`: a record
    /// whose class name is redacted keeps a readable file slug while its
    /// display name is a codex-neutral id.
    pub slug: String,
    pub max_level: u8,
    pub rows: Vec<(u8, i16, i16, i16, i16)>, // level, bab, fort, ref, will
}

/// Every conventional class's full progression, read from the CONVERTED
/// chassis (`data/sheet_rules/<book>/class/<slug>.json`) through
/// `class_chassis_sheet_rules`. The second return value is the
/// `(book, name)` of any record that carries a chassis but whose progression
/// does not fully evaluate -- empty today, kept so a future gap surfaces
/// rather than disappearing.
///
/// `_repo_root` is unused: `class_chassis_sheet_rules` resolves the package
/// from the `codex` crate's own manifest directory, the single place the
/// converted rules live. The parameter stays so the two callers
/// (`class_catalog.rs`, `character_hub.rs`) keep their signatures.
pub fn load_generic_class_progressions(
    _repo_root: &Path,
) -> (Vec<GenericClassRecord>, Vec<(String, String)>) {
    let mut out = Vec::new();
    let mut unresolved = Vec::new();
    for ((book, slug), chassis) in class_chassis_sheet_rules::records(&CLASS_FAMILY_BOOKS) {
        if !chassis.is_conventional() {
            continue;
        }
        match chassis.full_progression() {
            Some(rows) => out.push(GenericClassRecord {
                book,
                name: chassis.display_name.clone(),
                slug,
                max_level: chassis.max_level,
                rows,
            }),
            None => unresolved.push((book, chassis.display_name.clone())),
        }
    }
    (out, unresolved)
}

pub fn generic_class_catalog_entries(repo_root: &Path) -> Vec<ClassCatalogEntryDto> {
    let (records, _unresolved) = load_generic_class_progressions(repo_root);
    let mut entries = Vec::new();
    for record in records {
        for (level, bab, fort, refl, will) in record.rows {
            entries.push(ClassCatalogEntryDto {
                class_id: record.name.clone(),
                level,
                base_attack_bonus: bab,
                fort_save: fort,
                ref_save: refl,
                will_save: will,
            });
        }
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::authoring_workbench::codex_repo_root;

    fn repo() -> PathBuf {
        codex_repo_root().expect("repo root")
    }

    #[test]
    fn a_monster_companion_pseudo_class_is_excluded() {
        // ultimate_psionics's "Astral Warrior" -- `TYPE:...Monster...`, which
        // the converted record carries as a `Monster` tag.
        let astral = class_chassis_sheet_rules::record("ultimate_psionics", "astral_warrior");
        assert!(
            astral.is_none_or(|c| !c.is_conventional()),
            "a monster pseudo-class must never enter the browser's population"
        );
        let (records, _) = load_generic_class_progressions(&repo());
        assert!(!records.iter().any(|r| r.name == "Astral Warrior"));
    }

    #[test]
    fn a_support_shell_is_excluded() {
        // ultimate_intrigue's "VCabalist": `TYPE: Support`, no BASEAB/SAVE, so
        // its converted record carries no chassis rows at all.
        assert!(class_chassis_sheet_rules::record("ultimate_intrigue", "vcabalist").is_none());
    }

    #[test]
    fn the_converted_package_carries_sixty_two_conventional_classes() {
        let (records, unresolved) = load_generic_class_progressions(&repo());
        assert!(
            unresolved.is_empty(),
            "every conventional class's converted progression must evaluate: {unresolved:?}"
        );
        // 62, not the old 61 -- see the module doc's "Population" section for
        // the two movements and how to re-derive each.
        assert_eq!(records.len(), 62);
    }

    #[test]
    fn demoniac_resolves_from_the_converted_record() {
        // The one record the run-time interpreter's grammar refused (a bare
        // `classlevel()` with no argument). At level 1: BAB = 1*3/4 = 0;
        // Fortitude = (1+1)/2 = 1; Reflex = Will = (1+1)/3 = 0.
        let (records, _) = load_generic_class_progressions(&repo());
        let demoniac = records.iter().find(|r| r.name == "Demoniac").expect("Demoniac must resolve");
        assert_eq!(demoniac.rows[0], (1, 0, 1, 0, 0));
    }

    #[test]
    fn exactly_one_class_needs_baseab_disambiguation() {
        let (records, _) = load_generic_class_progressions(&repo());
        let vigilante = records
            .iter()
            .find(|r| r.name == "Vigilante")
            .expect("Vigilante must resolve");
        // Moderate (3/4) BAB progression at level 20 is 15, not 20 (which
        // the alternate full-BAB toggle row would have produced) -- proves
        // the default row was actually selected, not merely that a row exists.
        let (level, bab, ..) = vigilante.rows[19];
        assert_eq!(level, 20);
        assert_eq!(bab, 15);
    }

    #[test]
    fn ulfen_guard_prestige_class_defaults_to_max_level_10() {
        let (records, _) = load_generic_class_progressions(&repo());
        let ulfen = records
            .iter()
            .find(|r| r.name == "Ulfen Guard")
            .expect("Ulfen Guard must resolve");
        assert_eq!(ulfen.max_level, 10);
        assert_eq!(ulfen.rows.len(), 10);
    }

    #[test]
    fn kineticist_level_20_bab_and_saves_match_hand_derivation() {
        // BAB: level*3/4 -> floor(20*3/4) = 15 (moderate progression).
        // Fort/Reflex: level/2+2 -> 10+2 = 12 (good progression).
        // Will: level/3 -> 6 (poor progression), truncated toward zero.
        let (records, _) = load_generic_class_progressions(&repo());
        let kin = records
            .iter()
            .find(|r| r.name == "Kineticist")
            .expect("Kineticist must resolve");
        let (level, bab, fort, refl, will) = kin.rows[19];
        assert_eq!(level, 20);
        assert_eq!(bab, 15);
        assert_eq!(fort, 12);
        assert_eq!(refl, 12);
        assert_eq!(will, 6);
    }

    #[test]
    fn generic_catalog_entries_cover_every_class_with_no_overlap_into_crb_pu_names() {
        let entries = generic_class_catalog_entries(&repo());
        let distinct: std::collections::BTreeSet<_> =
            entries.iter().map(|e| e.class_id.as_str()).collect();
        assert_eq!(distinct.len(), 62);
        // None shares a display name with an existing CRB/PU row (would
        // silently merge into an unrelated progression otherwise).
        let crb_pu_names = [
            "Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger",
            "Rogue", "Sorcerer", "Wizard", "Unchained Barbarian", "Unchained Monk",
            "Unchained Rogue", "Unchained Summoner",
        ];
        for name in crb_pu_names {
            assert!(!distinct.contains(name), "{name} collides with a generic-catalog row");
        }
    }
}
