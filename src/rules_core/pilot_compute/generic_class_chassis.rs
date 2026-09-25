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
/// scoped" — same books, same order, same population.
///
/// A slug two books both state keeps the FIRST book in this list
/// ([`generic_class_records`] reads the books one at a time, in this order).
/// SD-36 Epic F2a appended `core_rulebook` and `advanced_players_guide` LAST,
/// so they can never displace a class an earlier book already gave;
/// `class_family_books_end_with_crb_then_apg_and_every_shadowed_slug_is_named`
/// lists every shadowed slug by name (the two appended books shadow none).
/// Their base classes also have their own, earlier dispatch arms in
/// `compute_class_chassis`, so what the append newly reaches is their 18
/// prestige classes' converted chassis rows.
const CLASS_FAMILY_BOOKS: [&str; 16] = [
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
    "core_rulebook",
    "advanced_players_guide",
];

pub(crate) struct GenericChassisRow {
    pub(crate) display_name: String,
    pub(crate) base_attack_bonus: i16,
    pub(crate) fort_save: i16,
    pub(crate) ref_save: i16,
    pub(crate) will_save: i16,
}

/// One covered class's registration: its `"class:<slug>"` id, the book its
/// converted record actually came from (book-precedence deduplicated, see
/// [`generic_class_records`]), its converted level ceiling, and its
/// converted `tags`. SD-36 Epic F0a
/// (`docs/release/SD-36-consolidation/epic-f-class-completion.md` §2): the
/// permanent class-census instrument (`rules_core::class_census`) merges
/// this list with every other class registry the engine reads, the same
/// shape [`crb_untabled_class_chassis::covered_classes`](super::crb_untabled_class_chassis::covered_classes)
/// already exposes for its own population.
pub(crate) struct GenericChassisMeta {
    pub(crate) class_id: String,
    pub(crate) book: String,
    pub(crate) max_level: u8,
    pub(crate) tags: Vec<String>,
}

/// The full registration list -- every conventional class
/// [`generic_class_records`] actually resolved a chassis for (122 as of
/// SD-36 Epic F2a; see `every_conventional_class_in_class_family_books_resolves`
/// below for how that count is itself re-derived).
pub(crate) fn covered_classes() -> Vec<GenericChassisMeta> {
    generic_class_records()
        .iter()
        .map(|(slug, chassis)| GenericChassisMeta {
            class_id: format!("class:{slug}"),
            book: chassis.book.clone(),
            max_level: chassis.max_level,
            tags: chassis.tags.clone(),
        })
        .collect()
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
        // One book at a time, in `CLASS_FAMILY_BOOKS` order: `records` keys its
        // map by `(book, slug)`, so reading every book in one call would visit
        // books alphabetically and let an appended book (`advanced_players_guide`
        // sorts first) win a slug an earlier-listed book already gave.
        for book in CLASS_FAMILY_BOOKS {
            for ((_, slug), chassis) in class_chassis_sheet_rules::records(&[book]) {
                if !chassis.is_conventional() {
                    continue;
                }
                out.entry(slug).or_insert(chassis);
            }
        }
        out
    })
}

/// `true` when `class_id_str` (a `"class:<slug>"` id) is one of
/// this module's classes AND its converted record's principal rule is tagged
/// `Prestige` (the record's own `TYPE:` head). A slug this population does not
/// carry answers `false`: this reads the record, it never guesses. SD-36 Epic
/// F2a: the shared chassis gate's generic class-family arm
/// (`class_shared_core::is_supported_generic_class_family_single_class`)
/// excludes exactly these, since a prestige class cannot be a character's
/// only class.
pub(crate) fn is_prestige(class_id_str: &str) -> bool {
    let Some(bare) = class_id_str.strip_prefix("class:") else {
        return false;
    };
    generic_class_records()
        .get(bare)
        .is_some_and(|record| record.tags.iter().any(|t| t == "Prestige"))
}

/// The converted chassis record behind `class_id_str` (a `"class:<slug>"`
/// id), book precedence applied -- or `None` for a bare slug or a class this
/// population does not carry. SD-36 F3b: the multiclass fold reads a class's
/// save shapes, exact save values and hit die off this record.
pub(crate) fn record(class_id_str: &str) -> Option<&'static ClassChassis> {
    generic_class_records().get(class_id_str.strip_prefix("class:")?)
}

/// Resolves `class_id_str` (a `"class:<slug>"` string) at `level` into a
/// real BAB/save chassis row, or `None` when the class is not one of this
/// table's 60 (either genuinely unresolved, like Demoniac, or simply not a
/// conventional PC class at all — the caller's existing dispatch chain
/// already tried every other known family first, so a `None` here always
/// falls through to the same `class_chassis.unsupported` diagnostic every
/// other unrecognized class id already produces).
pub(crate) fn resolve(class_id_str: &str, level: u8) -> Option<GenericChassisRow> {
    // Only a real `"class:<slug>"` id. A bare slug (`"wizard"`) is not a class
    // id any other dispatch arm accepts either (`sd20_contract_pilot_receipt`
    // relies on bare `"wizard"` being unsupported); before SD-36 Epic F2a no
    // bare slug collided with this population, but `core_rulebook`'s base
    // classes would.
    let bare = class_id_str.strip_prefix("class:")?;
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
    fn every_conventional_class_in_class_family_books_resolves() {
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
        //   -1  `inner_sea_gods`'s Evangelist. Its converted record carried a
        //       degradation on another of its own tokens, and the converter's
        //       OLD, record-wide degradation policy turned every magnitude on
        //       the record into the rule's own WORDS even though the class's
        //       own BAB/save formulas were perfectly clean. 62 was the count
        //       under that bug.
        //
        // SD-36 Epic E CONV-05 fixed degradation to be per-occurrence rather
        // than record-wide (`convert.rs`, `RecordCtx::current_seq_degraded`):
        // a class record's BAB/save formulas now print their real numbers
        // whenever THEY converted cleanly, regardless of an unrelated
        // degrading token elsewhere on the same record. This un-hid a real,
        // correct chassis for Evangelist (¾ BAB, good Reflex -- a genuine PF1
        // progression, verified directly against
        // `data/sheet_rules/inner_sea_gods/class/evangelist.json`) and 15
        // other prestige classes across `CLASS_FAMILY_BOOKS` that had the
        // exact same masking bug, moving the count from 62 to 78. Re-derive:
        // `python3 -c "import json,glob; tokens=json.load(open('data/sheet_rules/_tokens.json')); degraded={e['id'] for e in tokens['entries'] if e.get('degradations')}; print(sum(1 for f in glob.glob('data/sheet_rules/*/class/*.json') if (d:=json.load(open(f))) and d[0]['id'] in degraded and any(r.get('target')=='BaseAttack' for r in d)))"`
        // counts the previously-masked, now-resolving records (20 corpus-wide;
        // the subset inside `CLASS_FAMILY_BOOKS`, deduplicated by slug against
        // book precedence order, is this test's +16).
        //
        // SD-36 Epic F2a: 78 -> 122, `core_rulebook` and `advanced_players_guide`
        // appended LAST. +44, zero shadowed (the shadowed-slug pin below lists
        // every collision; neither appended book has one):
        //   core_rulebook          +27 = 10 base PC (every CRB base class but
        //                                Monk, whose converted record carries no
        //                                `BaseAttack` row) + 5 NPC + 2 `Ex-*` +
        //                                10 prestige
        //   advanced_players_guide +17 = 6 base (the `ApgClassId` six) +
        //                                Antipaladin + 2 `Ex-*` (Ex-Antipaladin,
        //                                Ex-Inquisitor) + 8 prestige (Eidolon is
        //                                `Monster`-tagged, not conventional)
        // The spec's ceiling (`epic-f-class-completion.md` §4, F2.1) was 96 =
        // 78 + the 18 CRB/APG prestige classes, "less any slug a bespoke arm or
        // earlier book already owns". The other 26 are base classes: 24 of them
        // a bespoke arm already owns and dispatches FIRST (CRB table 10, APG
        // table 6, `untabled_base_class_chassis` Antipaladin, `crb_untabled_
        // class_chassis` 5 NPC + Ex-Barbarian + Ex-Paladin), so this module's
        // `resolve` -- the LAST arm of `compute_class_chassis` -- never answers
        // for them; the remaining 2 (Ex-Antipaladin, Ex-Inquisitor) no other
        // registry claims. 78 + 18 + 24 + 2 = 122. Re-derive:
        // `generic_class_records().len()`.
        assert_eq!(
            generic_class_records().len(),
            122,
            "the converted chassis population over CLASS_FAMILY_BOOKS"
        );
        let mut resolved = 0usize;
        for bare in generic_class_records().keys() {
            let class_id = format!("class:{bare}");
            assert!(resolve(&class_id, 1).is_some(), "{bare} must resolve a real chassis at level 1");
            resolved += 1;
        }
        assert_eq!(resolved, 122, "every conventional class must resolve a real chassis");
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
        // Evangelist now DOES resolve (a real, correct chassis CONV-05 un-hid,
        // see `every_conventional_class_in_class_family_books_resolves` above), so it
        // no longer exercises this guard. `occult_adventures:class:psychic_
        // detective` genuinely carries no `BaseAttack`/`BaseSave` row at all
        // (verified: `data/sheet_rules/occult_adventures/class/psychic_
        // detective.json` has exactly one line, a `CasterLevel` Number, no
        // `target: BaseAttack`) -- refusing it is the contract, never a
        // guessed progression.
        assert!(resolve("class:psychic_detective", 1).is_none());
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

    /// SD-36 Epic F2a (`epic-f-class-completion.md` §4): Core Rulebook and
    /// Advanced Player's Guide are appended LAST, and a slug two books both
    /// state keeps the FIRST book in `CLASS_FAMILY_BOOKS` order. This pin
    /// names every shadowed slug, by name, so an appended book can never
    /// silently lose (or silently take over) a class an earlier book gave.
    #[test]
    fn class_family_books_end_with_crb_then_apg_and_every_shadowed_slug_is_named() {
        let n = CLASS_FAMILY_BOOKS.len();
        assert_eq!(
            &CLASS_FAMILY_BOOKS[n - 2..],
            &["core_rulebook", "advanced_players_guide"],
            "CRB then APG must be the last two books (first-in-list wins)"
        );
        let mut winner: BTreeMap<String, String> = BTreeMap::new();
        let mut shadowed: Vec<(String, String, String)> = Vec::new();
        for book in CLASS_FAMILY_BOOKS {
            for ((_, slug), chassis) in class_chassis_sheet_rules::records(&[book]) {
                if !chassis.is_conventional() {
                    continue;
                }
                match winner.get(&slug) {
                    Some(first) => shadowed.push((slug, first.clone(), book.to_string())),
                    None => {
                        winner.insert(slug, book.to_string());
                    }
                }
            }
        }
        let expected: Vec<(String, String, String)> = [
            ("cyphermage", "adventurers_guide", "inner_sea_magic"),
            ("hellknight", "adventurers_guide", "inner_sea_world_guide"),
            ("red_mantis_assassin", "adventurers_guide", "inner_sea_world_guide"),
        ]
        .iter()
        .map(|(s, a, b)| (s.to_string(), a.to_string(), b.to_string()))
        .collect();
        let mut shadowed_sorted = shadowed.clone();
        shadowed_sorted.sort();
        assert_eq!(
            shadowed_sorted, expected,
            "(slug, winning book, shadowed book): every slug an earlier book already gave; \
             core_rulebook and advanced_players_guide shadow none"
        );
        // The winning book is the one `generic_class_records` actually kept.
        for (slug, book) in &winner {
            assert_eq!(&generic_class_records()[slug].book, book, "{slug}: first-in-list must win");
        }
    }

    #[test]
    fn is_prestige_reads_the_record_s_own_prestige_tag() {
        // CRB/APG prestige classes, reachable now that both books are appended.
        assert!(is_prestige("class:arcane_archer"));
        assert!(is_prestige("class:loremaster"));
        // A bare slug is not a class id.
        assert!(!is_prestige("loremaster"));
        assert!(is_prestige("class:battle_herald"));
        // A prestige class from the original 14 books.
        assert!(is_prestige("class:hellknight"));
        // Base classes -- the tag is `Base`, never `Prestige`.
        assert!(!is_prestige("class:ninja"));
        assert!(!is_prestige("class:kineticist"));
        assert!(!is_prestige("class:fighter"));
        // Not in the population at all: not a prestige class this module knows.
        assert!(!is_prestige("class:not_a_real_class"));
    }

    #[test]
    fn crb_and_apg_prestige_classes_resolve_a_real_chassis() {
        // Hand-worked from the PF1 Core Rulebook's own prestige tables (oracle
        // first): prestige classes use the prestige save progressions, good
        // `(level+1)/2` and poor `(level+1)/3`, NOT the base-class `level/2+2`.
        // Arcane Archer: full BAB, good Fort and Ref, poor Will -- the book's
        // level-10 row prints +10 / +5 / +5 / +3.
        let row = resolve("class:arcane_archer", 10).expect("arcane archer resolves at 10");
        assert_eq!(
            (row.base_attack_bonus, row.fort_save, row.ref_save, row.will_save),
            (10, 5, 5, 3)
        );
        // Loremaster: half BAB, good Will only -- level 10 prints +5 / +3 / +3 / +5.
        let row = resolve("class:loremaster", 10).expect("loremaster resolves at 10");
        assert_eq!(
            (row.base_attack_bonus, row.fort_save, row.ref_save, row.will_save),
            (5, 3, 3, 5)
        );
        // A prestige class never runs past its own ceiling of 10.
        assert!(resolve("class:loremaster", 11).is_none());
    }

    /// The two classes the CRB/APG append brings in that no other registry
    /// claims (`class_census`'s `GenericOnly`): APG's `Ex-*` variant classes.
    /// Hand-worked from PF1 (oracle first): an ex-antipaladin keeps the
    /// Antipaladin's table (full BAB, good Fortitude and Will), an
    /// ex-inquisitor the Inquisitor's (3/4 BAB, good Fortitude and Will).
    #[test]
    fn the_two_apg_ex_classes_resolve_their_parent_class_chassis() {
        // Antipaladin level 10: +10 / +7 / +3 / +7.
        let row = resolve("class:ex_antipaladin", 10).expect("ex-antipaladin resolves at 10");
        assert_eq!(
            (row.base_attack_bonus, row.fort_save, row.ref_save, row.will_save),
            (10, 7, 3, 7)
        );
        // Inquisitor level 8: +6 / +6 / +2 / +6.
        let row = resolve("class:ex_inquisitor", 8).expect("ex-inquisitor resolves at 8");
        assert_eq!(
            (row.base_attack_bonus, row.fort_save, row.ref_save, row.will_save),
            (6, 6, 2, 6)
        );
        assert!(!is_prestige("class:ex_antipaladin"));
        assert!(!is_prestige("class:ex_inquisitor"));
    }

    #[test]
    fn an_unrecognized_class_id_refuses() {
        assert!(resolve("class:not_a_real_class", 1).is_none());
        // A bare slug is not a class id, even for a class this module carries.
        assert!(resolve("class:wizard", 1).is_some());
        assert!(resolve("wizard", 1).is_none());
    }
}
