//! Character-creation-time chassis dispatch for the 60 conventional PC
//! classes `class_catalog_generic.rs` (`apps/desktop/src-tauri`, SD-32 T12
//! `epic-10-reference-library-residual-reach` row 20 cycle 4) already
//! re-derives a BAB/save progression TABLE for, browsable in the reference
//! catalog. That table has no `compute_class_chassis` dispatch arm reading
//! it -- a character actually PICKING one of those 60 classes at creation
//! could not reach a real base attack bonus or save chassis at all, only
//! the read-only reference browser. Row 20 cycle 5 closes that gap: this
//! module is the crate-internal (`pilot_compute` lives in the core `codex`
//! crate; the apps/desktop catalog module is a separate, downstream crate
//! and cannot be imported from here) sibling of `class_catalog_generic.rs`,
//! re-running the SAME classification/extraction logic that module's own
//! doc comment already verified against all 61 candidate records, wired
//! into `compute_class_chassis` below via [`resolve`].
//!
//! # Why a second copy of the read logic, not a shared one
//!
//! `class_catalog_generic.rs`'s own read/classify/evaluate functions
//! (`classify_class_record`, `select_baseab_formula`, `select_save_
//! formulas`, `max_level_for`) live in `apps/desktop/src-tauri`, a crate
//! that DEPENDS ON `codex` (this crate), never the reverse -- `pilot_
//! compute::mod.rs`'s own `compute_class_chassis` cannot import from it
//! without an illegal reverse dependency. This module re-implements the
//! same, already-verified logic at the crate boundary it is actually
//! needed at, the same "parallel per-family module" shape `class_slayer.rs`/
//! `class_ultimate_combat.rs` (this module's own siblings) already use
//! rather than a single monolith. `resolve`'s own doc-comment tests below
//! independently reproduce the parent cycle's 60/61 population split, so
//! any future drift between the two copies is caught by CI rather than
//! silently diverging.
//!
//! # All 61 resolve -- Demoniac closed on rebase, mid-cycle
//!
//! `class_catalog_generic.rs` (row 20 cycle 4) named Demoniac's bare
//! `classlevel()` as the one record `formula_interpreter.rs`'s grammar
//! refused (row 18's live territory). This cycle's own rebase (`git fetch
//! origin tranche/12 && git rebase`, step 6) picked up row 18 cycle 9,
//! which widened the grammar to PARSE a bare `classlevel()` call -- but
//! deliberately left EVALUATION refusing until a caller explicitly binds
//! the empty `CLASSLEVEL::` sentinel key that widening introduced (its own
//! doc comment: "No caller today binds `CLASSLEVEL::` (empty key)"). This
//! module's own [`resolve`] IS that caller: a bare `classlevel()` inside a
//! class's own record can only mean that SAME class's own level (there is
//! no other class in scope for a single-class record to name), so `resolve`
//! binds the empty key to the record's own already-known `level` — never a
//! guess. Per the brief's own instruction ("if the widening has landed,
//! close your 61st and say so"): **it landed, and this closes it** — 61 of
//! 61 conventional classes now resolve a real chassis, not 60.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use super::class_chassis_sheet_rules::{self, ClassChassis};

/// See `class_catalog_generic.rs`'s own doc comment, "Reachability, honestly
/// scoped" — same 14 books, same population.
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

pub(crate) struct GenericChassisRow {
    pub(crate) display_name: String,
    pub(crate) base_attack_bonus: i16,
    pub(crate) fort_save: i16,
    pub(crate) ref_save: i16,
    pub(crate) will_save: i16,
}

/// Loaded once per process, keyed by the converted record's own slug -- the
/// same `"class:<slug>"` id convention every other dispatch arm in
/// `compute_class_chassis` uses. Two books stating the same class slug keep
/// the first in `CLASS_FAMILY_BOOKS` order, as the corpus read this replaces
/// already did.
fn generic_class_records() -> &'static BTreeMap<String, ClassChassis> {
    static TABLE: OnceLock<BTreeMap<String, ClassChassis>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out: BTreeMap<String, ClassChassis> = BTreeMap::new();
        for ((_, slug), chassis) in class_chassis_sheet_rules::records(&CLASS_FAMILY_BOOKS) {
            if !chassis.is_conventional() {
                continue;
            }
            out.entry(slug).or_insert(chassis);
        }
        out
    })
}

/// Resolves `class_id_str` (a `"class:<slug>"` string) at `level` into a
/// real BAB/save chassis row, or `None` when the class is not one of this
/// table's 60 (either genuinely unresolved, like Demoniac, or simply not a
/// conventional PC class at all — the caller's existing dispatch chain
/// already tried every other known family first, so a `None` here always
/// falls through to the same `class_chassis.unsupported` diagnostic every
/// other unrecognized class id already produces).
pub(crate) fn resolve(class_id_str: &str, level: u8) -> Option<GenericChassisRow> {
    let bare = class_id_str.strip_prefix("class:").unwrap_or(class_id_str);
    let record = generic_class_records().get(bare)?;
    let row = record.row_at(level)?;
    Some(GenericChassisRow {
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
    fn all_sixty_two_conventional_classes_resolve() {
        // SD-35 `AT-35-E6-001`: 62, not the 61 this module counted while it
        // read `data/corpus/<book>/class/`. Two independent movements, both
        // re-derivable and neither a relabel:
        //
        //   +2  `adventurers_guide`'s Pathfinder Delver and Pathfinder Savant.
        //       `data/corpus/adventurers_guide/class/` holds 9 records and
        //       neither of these; the converter reads the PINNED oracle corpus
        //       directly, so `data/sheet_rules/` carries both, each with a
        //       complete BAB + three-save chassis and a MAXLEVEL:10 ceiling.
        //       Re-derive: `ls data/corpus/adventurers_guide/class/ | wc -l`
        //       against `ls data/sheet_rules/adventurers_guide/class/ | wc -l`.
        //
        //   -1  `inner_sea_gods`'s Evangelist. Its converted record carries a
        //       degradation on another of its own tokens, and the converter's
        //       standing policy (`convert.rs`, "the partly-read magnitudes are
        //       dropped rather than folded into a sheet total -- a wrong
        //       computed number looks right, an omitted one does not") turns
        //       every magnitude on a degraded record into the rule's own
        //       WORDS. Under `decisions.md` §1 that record is done as prose;
        //       it is not a chassis, and this module refuses it rather than
        //       inventing one. Re-derive: the four
        //       `inner_sea_gods:class:evangelist` rules carry
        //       `"value":"Text"` and `"target":null`.
        assert_eq!(
            generic_class_records().len(),
            62,
            "the converted chassis population over CLASS_FAMILY_BOOKS"
        );
        let mut resolved = 0usize;
        for bare in generic_class_records().keys() {
            let class_id = format!("class:{bare}");
            assert!(resolve(&class_id, 1).is_some(), "{bare} must resolve a real chassis at level 1");
            resolved += 1;
        }
        assert_eq!(resolved, 62, "every conventional class must resolve a real chassis");
    }

    #[test]
    fn no_class_resolves_a_degenerate_all_zero_progression() {
        // The wrong-binding guard, corpus-wide rather than on the one record
        // that found it: a chassis whose `Expr::ClassLevel` id is bound to a
        // name the facts do not carry evaluates every level to zero and still
        // returns `Some(row)`. Every conventional class in this population has
        // a real base-attack progression, so a base attack bonus of 0 at the
        // class's own ceiling means the binding, not the book.
        for (bare, record) in generic_class_records() {
            let top = record.row_at(record.max_level).unwrap_or_else(|| {
                panic!("{bare} must resolve at its own ceiling {}", record.max_level)
            });
            assert!(
                top.base_attack_bonus > 0,
                "{bare}: base attack bonus 0 at level {} is the wrong-binding failure, not a \
                 progression any class prints",
                record.max_level
            );
        }
    }

    #[test]
    fn a_class_whose_converted_record_is_words_is_not_a_chassis() {
        // Evangelist, above: prose, not a number. Refusing is the contract --
        // never a guessed progression.
        assert!(resolve("class:evangelist", 1).is_none());
    }

    #[test]
    fn demoniac_resolves_via_the_bare_classlevel_binding() {
        // Demoniac's own real corpus tokens (`book_of_the_damned_volume_2/demoniac.json`):
        // `BONUS:COMBAT|BASEAB|classlevel()*3/4|...`, `BONUS:SAVE|BASE.Fortitude|
        // (classlevel()+1)/2`, `BONUS:SAVE|BASE.Will,BASE.Reflex|(classlevel()+1)/3` — Reflex is
        // packed with WILL, not Fortitude (confirmed by direct read, not assumed uniform). At
        // level 1 (integer division, floor): BAB = 1*3/4 = 0; Fort = (1+1)/2 = 1; Reflex = Will
        // = (1+1)/3 = 0.
        let row = resolve("class:demoniac", 1).expect("Demoniac must now resolve (row 18 cycle 9 landed)");
        assert_eq!(row.base_attack_bonus, 0);
        assert_eq!(row.fort_save, 1);
        assert_eq!(row.ref_save, 0);
        assert_eq!(row.will_save, 0);
    }

    #[test]
    fn vigilante_resolves_via_the_toggle_off_baseab_row_matching_class_catalog_generic() {
        // Same disambiguation `class_catalog_generic.rs`'s own test proves:
        // level-20 BAB is the moderate (,0 / toggle-off) progression, 15,
        // never the toggle-on alternative, 20. Vigilante is ALSO one of
        // the 20 untabled-base-class-chassis classes and so is dispatched
        // by that arm first in the real `compute_class_chassis` chain —
        // this module's own `resolve` is exercised directly here (not
        // through the full dispatcher) purely to prove its own formula
        // selection is correct in isolation.
        let row = resolve("class:vigilante", 20).expect("vigilante must resolve at level 20");
        assert_eq!(row.base_attack_bonus, 15);
    }

    #[test]
    fn a_level_past_max_level_refuses() {
        // Ulfen Guard (`inner_sea_combat`, TYPE: PC.Prestige, no MAXLEVEL
        // token) defaults to 10 per `class_catalog_generic.rs`'s own
        // documented prestige-class rule; level 11 must refuse.
        assert!(resolve("class:ulfen_guard", 11).is_none());
        assert!(resolve("class:ulfen_guard", 10).is_some());
    }

    #[test]
    fn an_unrecognized_class_id_refuses() {
        assert!(resolve("class:not_a_real_class", 1).is_none());
    }
}
