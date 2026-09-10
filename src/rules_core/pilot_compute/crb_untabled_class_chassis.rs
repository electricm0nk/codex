//! Base-attack-bonus / base-save chassis for Core Rulebook's **NPC and
//! `Ex-*` classes** (SD-34, `AT-34-E3-001`, mechanism
//! `class_absent_from_ClassId_ALL_and_book_class_id_enums`): `ClassId::ALL`
//! (`rules_tables::crb::class_tables`) carries only CRB's eleven `Base.PC`
//! classes -- Adept, Aristocrat, Commoner, Expert, Warrior (`TYPE:Base.NPC`)
//! and Ex-Barbarian, Ex-Paladin (`TYPE:Base.PC`, `VISIBLE:NO`) are ingested,
//! real corpus records with their own genuine `BONUS:COMBAT|BASEAB` /
//! `BONUS:SAVE` formulas, and no `compute_class_chassis` dispatch arm reads
//! them, so `modelled_class_books()` never learned their names and every
//! `Kind::Class` unit for them reported `engine-does-not-hold` with the
//! `class_absent_from_ClassId_ALL_and_book_class_id_enums` evidence
//! (`docs/release/SD-34-book-completion/decisions.md §14`) regardless of
//! what the engine could otherwise do with them.
//!
//! # Scope: these 7, not CRB's ten prestige classes
//!
//! CRB's ten `PC.Prestige` classes are already registered a different, real
//! way -- `prestige_class_entry_gate.rs`'s own corpus-derived registry
//! evaluates their genuine `PRE*` entry requirements, and that module's own
//! doc comment states plainly that a full base-attack-bonus/save chassis for
//! six of the ten is deferred pending a caster-level-stacking mechanism this
//! codebase does not have yet. This module does not reopen that deferral --
//! `modelled_class_books()` registers the ten prestige classes straight from
//! `prestige_class_entry_gate::prestige_class_entry_requirements()`, no new
//! chassis code, respecting the existing SD-32 decision. This module's own
//! job is strictly the seven classes nothing else names at all: five NPC
//! classes and two `Ex-*` variant states.
//!
//! # Method: the converted chassis, read through the one live reader
//!
//! SD-35 `AT-35-E6-001` (`decisions.md` §11). Until that cycle this module
//! read `data/corpus/core_rulebook/class/<slug>.json`'s ingest token array, pulled
//! the `BONUS:COMBAT|BASEAB` / `BONUS:SAVE` formula STRINGS out of them, and
//! evaluated those strings through the PCGen formula interpreter at render
//! time. Nothing on the live side reads a PCGen token or formula any more:
//! the converter writes each class's four progressions as converted `Expr`s
//! and its `MAXLEVEL` ceiling as an `applies` gate, and
//! [`class_chassis_sheet_rules`](super::class_chassis_sheet_rules) is the one
//! live reader of that shape. This module keeps its own scope (these seven
//! records, registered nowhere else) and its own public surface; only the
//! source of the numbers changed, and its tests pin the same values as
//! before.
//!
//! Still a second, parallel module rather than a widening of
//! `generic_class_chassis.rs`'s own `CLASS_FAMILY_BOOKS`: that module's
//! population (61 conventional PC classes) is mirrored in
//! `apps/desktop/src-tauri`'s `class_catalog_generic.rs` reference-library
//! browser, and CRB's NPC/`Ex-*` classes are not "conventional PC classes" in
//! that browser's own sense.

use std::sync::OnceLock;

use super::class_chassis_sheet_rules::{self, ClassChassis};

/// The seven CRB classes this module covers, by their `"class:<slug>"` id
/// convention -- five `TYPE:Base.NPC` classes and two `TYPE:Base.PC,
/// VISIBLE:NO` `Ex-*` variant states. Fixed, not derived from a directory
/// walk over all 28 CRB class records: CRB's eleven real base classes
/// (`ClassId::ALL`) and ten prestige classes (`prestige_class_entry_gate`)
/// are each registered their own way, and this module must never
/// double-register or shadow either.
const COVERED_SLUGS: [&str; 7] =
    ["adept", "aristocrat", "commoner", "expert", "warrior", "ex_barbarian", "ex_paladin"];

pub struct CrbUntabledClassChassisRow {
    pub display_name: String,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// One covered class's registration: its `"class:<slug>"` id and the book
/// it belongs to (always `"core_rulebook"` for this module). Exposed so
/// `modelled_class_books()` (`src/bin/v06_work_inventory.rs`) can register
/// these seven names as classes the engine models, the same shape it
/// already uses for `untabled_base_class_chassis::untabled_base_class_
/// registry()`.
pub struct CrbUntabledClassMeta {
    pub class_id: String,
    pub display_name: String,
}

fn load_records() -> Vec<(&'static str, ClassChassis)> {
    let mut out = Vec::new();
    for slug in COVERED_SLUGS {
        // A class whose converted record carries no complete chassis is
        // honestly absent, never half-built -- the same contract the corpus
        // read this replaces already kept.
        if let Some(chassis) = class_chassis_sheet_rules::record("core_rulebook", slug) {
            out.push((slug, chassis.clone()));
        }
    }
    out
}

fn records() -> &'static [(&'static str, ClassChassis)] {
    static TABLE: OnceLock<Vec<(&'static str, ClassChassis)>> = OnceLock::new();
    TABLE.get_or_init(load_records).as_slice()
}

/// The full registration list -- every covered class this module's own
/// corpus read actually resolved a record for (never assumed to be all
/// seven; a JSON file that failed to parse or carried no BASEAB/SAVE
/// formula is honestly absent, not silently substituted).
pub fn covered_classes() -> Vec<CrbUntabledClassMeta> {
    records()
        .iter()
        .map(|(slug, record)| CrbUntabledClassMeta {
            class_id: format!("class:{slug}"),
            display_name: record.display_name.clone(),
        })
        .collect()
}

fn find_by_class_id(class_id_str: &str) -> Option<&'static ClassChassis> {
    let bare = class_id_str.strip_prefix("class:").unwrap_or(class_id_str);
    records().iter().find(|(slug, _)| *slug == bare).map(|(_, record)| record)
}

/// Resolves `class_id_str` at `level` into a real base-attack-bonus/save
/// chassis row, evaluating this class's own CONVERTED progressions through
/// `sheet_rule::evaluate_expr_from_facts`. `None` when `class_id_str` names
/// no class this module covers, or `level` exceeds the class's own converted
/// `MAXLEVEL` ceiling.
pub fn resolve(class_id_str: &str, level: u8) -> Option<CrbUntabledClassChassisRow> {
    let record = find_by_class_id(class_id_str)?;
    let row = record.row_at(level)?;
    Some(CrbUntabledClassChassisRow {
        display_name: record.display_name.clone(),
        base_attack_bonus: row.base_attack_bonus,
        fort_save: row.fort_save,
        ref_save: row.ref_save,
        will_save: row.will_save,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_seven_covered_classes_resolve_a_real_chassis_at_level_one() {
        let covered = covered_classes();
        assert_eq!(
            covered.len(),
            7,
            "every one of the seven converted class records must carry a complete chassis"
        );
        for meta in &covered {
            let row = resolve(&meta.class_id, 1);
            assert!(row.is_some(), "{} must resolve a real chassis at level 1", meta.class_id);
        }
    }

    #[test]
    fn warrior_full_bab_matches_the_corpus_classlevel_formula_at_level_ten() {
        // Warrior is a full-BAB class, so level 10 must resolve to base
        // attack bonus 10.
        let row = resolve("class:warrior", 10).expect("warrior must resolve");
        assert_eq!(row.base_attack_bonus, 10);
        // Good Fortitude (`level/2 + 2`) -> 10/2+2 = 7.
        assert_eq!(row.fort_save, 7);
        // Poor Reflex and Will (`level/3`) -> 10/3 = 3.
        assert_eq!(row.ref_save, 3);
        assert_eq!(row.will_save, 3);
    }

    #[test]
    fn commoner_half_bab_and_all_poor_saves_match_the_corpus_formula() {
        // Commoner is a half-BAB class (`level/2`).
        let row = resolve("class:commoner", 9).expect("commoner must resolve");
        assert_eq!(row.base_attack_bonus, 4); // 9/2 = 4 (integer division)
        assert_eq!(row.fort_save, 3); // 9/3 = 3, all three saves poor
        assert_eq!(row.ref_save, 3);
        assert_eq!(row.will_save, 3);
    }

    #[test]
    fn a_level_beyond_max_level_resolves_nothing() {
        assert!(resolve("class:warrior", 21).is_none(), "the converted ceiling of 20 must cap resolution");
    }

    #[test]
    fn an_uncovered_class_id_resolves_nothing() {
        assert!(
            resolve("class:arcane_archer", 1).is_none(),
            "prestige classes are registered by prestige_class_entry_gate, never this module"
        );
        assert!(
            resolve("class:fighter", 1).is_none(),
            "CRB's real base classes are registered by ClassId::ALL, never this module"
        );
    }
}
