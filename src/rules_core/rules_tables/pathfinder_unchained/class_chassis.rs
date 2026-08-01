//! Pathfinder Unchained — the **class-selection registry and chassis
//! resolver** for the book's four Unchained classes.
//!
//! This is the seam `pilot_compute.rs` dispatches through, mirroring
//! `rules_tables::apg::{class_chassis_resolve, hit_die_for}` and
//! `rules_tables::acg::{class_chassis_resolve, hit_die_for}` exactly. The
//! four sibling `*_features.rs` modules in this directory hold the
//! hand-modelled feature magnitudes (`decisions.md §24.1`); this module
//! holds only identity and the base-attack-bonus / base-save / hit-die
//! chassis, so that a class can be *selected* and *computed* at all.
//!
//! # These four are REPLACEMENTS, not additions — how they stay distinct
//!
//! Pathfinder Unchained declares **zero `CLASS:` objects**. Each Unchained
//! class is a `CATEGORY:CLASS` selection ability layered over an existing
//! class in a single-slot PCGen ability pool, so a character holds exactly
//! one of the pair — the mutual exclusion is declared by the corpus, not
//! invented here (see each `*_features.rs` module's own header for the
//! exact pool rows). The corresponding CRB/APG class is untouched and
//! remains selectable.
//!
//! Three axes keep the pair apart in this repo, and each one is asserted by
//! a test below rather than merely asserted in prose:
//!
//! | axis | CRB/APG class | Unchained class |
//! |---|---|---|
//! | **identity** | `class:barbarian`, `class:monk`, `class:rogue`, `class:summoner` | `class:unchained_barbarian`, `class:unchained_monk`, `class:unchained_rogue`, `class:unchained_summoner` ([`PuClassId::name`]) |
//! | **display** | "Barbarian", "Monk", … | "Unchained Barbarian", "Unchained Monk", … — the corpus `name` field of `data/corpus/pathfinder_unchained/class/*.json`, verbatim ([`PuClassId::display_name`]) |
//! | **computation** | `rules_tables::crb` / `rules_tables::apg`, `RuleSetId::Crb` / `RuleSetId::Apg` | `rules_tables::pathfinder_unchained`, `RuleSetId::Pu` ([`class_chassis_resolve`] refuses every other rule set) |
//!
//! No id string, display label, module path or `RuleSetId` is shared, so
//! neither can shadow or silently substitute for the other. A character
//! record carrying `class:unchained_rogue` can never resolve CRB Rogue
//! content and vice versa.
//!
//! # Where each chassis actually comes from — and why three of four borrow
//!
//! The ingested `class` record for each Unchained class records whether it
//! overrides the base class's chassis fields. Three of the four override
//! **nothing** (`hit_die`, `bab`, `save_fort`, `save_ref`, `save_will` are
//! all `null`), so the base class's own already-shipped, already-verified
//! table *is* their chassis. This module therefore reads that table rather
//! than transcribing a second copy — a duplicate would be a competing
//! statement of one fact, which is exactly the drift this repo has been
//! burned by:
//!
//! - **Unchained Barbarian** → `crb::class_tables` `ClassId::Barbarian`
//!   (`barbarian_unchained_class.json`: all five chassis fields `null`).
//! - **Unchained Rogue** → `crb::class_tables` `ClassId::Rogue`
//!   (`rogue_unchained_class.json`: all five `null`).
//! - **Unchained Summoner** → `apg::class_chassis_resolve` for
//!   `ApgClassId::Summoner` (`summoner_unchained_class.json`: all five
//!   `null`; its `base_class_book` is `advanced_players_guide`, the only
//!   one of the four whose base class is not CRB).
//! - **Unchained Monk** → its own [`super::monk_features`] functions.
//!   `monk_unchained_class.json` is the one record that *does* override:
//!   `hit_die: 10`, `bab: "level"`, `save_fort: "level/2+2"`,
//!   `save_ref: "level/2+2"`, `save_will: "level/3"` — d10, full BAB, good
//!   Fort/Ref, **poor Will**, against the CRB Monk's d8 / three-quarter BAB
//!   / three good saves. That divergence is the loudest reason these four
//!   cannot be aliased onto their namesakes.
//!
//! # `RuleSetId` gating
//!
//! [`class_chassis_resolve`] returns `None` for any rule set other than
//! `RuleSetId::Pu`, matching the APG/ACG resolvers' cross-book invariant.
//! [`hit_die_for`] is deliberately *not* rule-set gated, matching
//! `apg::hit_die_for`'s own stated reasoning (a die size carries no
//! per-book collision risk the way a class name does).

use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::rules_tables::apg::{self, ApgClassId};
use crate::rules_core::rules_tables::crb::class_tables::{self, ClassId};

use super::monk_features;

/// One Unchained class's chassis-table row: level, BAB, and the three
/// saves. Same shape as `rules_tables::apg::ClassTableRow` and
/// `rules_tables::acg::ClassTableRow`, kept book-local for the same reason
/// those two are.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassTableRow {
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// Identifies which Pathfinder Unchained class a query targets. These are
/// the four and only four `CATEGORY:CLASS` selection abilities the book
/// declares, and exactly the four `data/corpus/pathfinder_unchained/class/`
/// records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PuClassId {
    UnchainedBarbarian,
    UnchainedMonk,
    UnchainedRogue,
    UnchainedSummoner,
}

impl PuClassId {
    /// The full four-class roster, in corpus declaration order
    /// (`pu_abilities_class.lst:114..117`).
    pub const ALL: [PuClassId; 4] = [
        PuClassId::UnchainedBarbarian,
        PuClassId::UnchainedMonk,
        PuClassId::UnchainedRogue,
        PuClassId::UnchainedSummoner,
    ];

    /// Lowercase class name, matching the `class_id` string convention
    /// `pilot_compute.rs` uses (`"class:<name>"`). Deliberately prefixed
    /// `unchained_` so `class:unchained_rogue` can never be confused with
    /// CRB's `class:rogue` by a string comparison anywhere in the stack —
    /// including in persisted character records on disk.
    pub const fn name(self) -> &'static str {
        match self {
            PuClassId::UnchainedBarbarian => "unchained_barbarian",
            PuClassId::UnchainedMonk => "unchained_monk",
            PuClassId::UnchainedRogue => "unchained_rogue",
            PuClassId::UnchainedSummoner => "unchained_summoner",
        }
    }

    /// The player-facing label, verbatim from the `name` field of this
    /// class's own `data/corpus/pathfinder_unchained/class/*.json` record.
    /// Not derived by capitalising [`name`](Self::name): the corpus states
    /// it, so the corpus supplies it.
    pub const fn display_name(self) -> &'static str {
        match self {
            PuClassId::UnchainedBarbarian => "Unchained Barbarian",
            PuClassId::UnchainedMonk => "Unchained Monk",
            PuClassId::UnchainedRogue => "Unchained Rogue",
            PuClassId::UnchainedSummoner => "Unchained Summoner",
        }
    }

    /// The `key` field of this class's corpus record — PCGen's own
    /// `KEY:` token for the selection ability.
    pub const fn corpus_key(self) -> &'static str {
        match self {
            PuClassId::UnchainedBarbarian => "Barbarian ~ Unchained Class",
            PuClassId::UnchainedMonk => "Monk ~ Unchained Class",
            PuClassId::UnchainedRogue => "Rogue ~ Unchained Class",
            PuClassId::UnchainedSummoner => "Summoner ~ Unchained Class",
        }
    }

    /// The `class_id` string of the class this one *replaces* — the other
    /// member of its single-slot PCGen selection pool. Stated so callers
    /// (and auditors) can check the two never appear together, and so the
    /// replacement relationship is machine-readable rather than prose-only.
    pub const fn replaces_class_id(self) -> &'static str {
        match self {
            PuClassId::UnchainedBarbarian => "class:barbarian",
            PuClassId::UnchainedMonk => "class:monk",
            PuClassId::UnchainedRogue => "class:rogue",
            PuClassId::UnchainedSummoner => "class:summoner",
        }
    }

    /// Reverse of [`name`](Self::name): resolves a `"class:<name>"` id
    /// string back to a `PuClassId`. Mirrors
    /// `apg::ApgClassId::from_class_id_str` / `acg::AcgClassId::from_class_id_str`.
    pub fn from_class_id_str(class_id_str: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|id| class_id_str == format!("class:{}", id.name()))
    }

    /// This class's `MAXLEVEL` ceiling, from its own feature module's
    /// `MAX_SUPPORTED_LEVEL` constant (each of which cites the base
    /// class's real `MAXLEVEL:20` record — PU adds no levels).
    pub const fn max_supported_level(self) -> u8 {
        match self {
            PuClassId::UnchainedBarbarian => super::barbarian_features::MAX_SUPPORTED_LEVEL,
            PuClassId::UnchainedMonk => monk_features::MAX_SUPPORTED_LEVEL,
            PuClassId::UnchainedRogue => super::rogue_features::MAX_SUPPORTED_LEVEL,
            PuClassId::UnchainedSummoner => super::summoner_features::MAX_SUPPORTED_LEVEL,
        }
    }
}

/// `class_id`'s hit-die size. Three of the four read the base class's own
/// already-verified `hit_die_for`; the Unchained Monk reads
/// [`super::monk_features::HIT_DIE`], the one genuine override (`d10`
/// against CRB Monk's `d8`).
///
/// Not `RuleSetId`-gated, for the same reason `apg::hit_die_for` is not.
pub fn hit_die_for(class_id: PuClassId) -> u8 {
    match class_id {
        // `class_tables::hit_die_for` returns `Option`, but only because it
        // must answer for a `ClassId` its own `CLASS_META` might not carry.
        // Barbarian and Rogue are both in `CLASS_META` (and the tests below
        // pin that), so the `expect` documents an invariant rather than
        // hiding a fallible lookup.
        PuClassId::UnchainedBarbarian => class_tables::hit_die_for(ClassId::Barbarian)
            .expect("crb::class_tables carries a Barbarian row"),
        PuClassId::UnchainedMonk => monk_features::HIT_DIE,
        PuClassId::UnchainedRogue => {
            class_tables::hit_die_for(ClassId::Rogue).expect("crb::class_tables carries a Rogue row")
        }
        PuClassId::UnchainedSummoner => apg::hit_die_for(ApgClassId::Summoner),
    }
}

/// The base class whose `crb::class_tables` row supplies an Unchained
/// class's chassis, for the three that override nothing. `None` for the
/// Unchained Monk (its own override) and the Unchained Summoner (whose base
/// class is APG, not CRB).
fn borrowed_crb_chassis_class(class_id: PuClassId) -> Option<ClassId> {
    match class_id {
        PuClassId::UnchainedBarbarian => Some(ClassId::Barbarian),
        PuClassId::UnchainedRogue => Some(ClassId::Rogue),
        PuClassId::UnchainedMonk | PuClassId::UnchainedSummoner => None,
    }
}

/// Resolves an Unchained class's chassis-table row for `level`, scoped to
/// `RuleSetId::Pu`. Returns `None` for any other rule set — a Pathfinder
/// Unchained class chassis is never a valid answer for a `RuleSetId::Crb`,
/// `RuleSetId::Apg` or `RuleSetId::Acg` query — and `None` when `level` is
/// outside `1..=max_supported_level()`.
pub fn class_chassis_resolve(
    class_id: PuClassId,
    level: u8,
    rule_set: RuleSetId,
) -> Option<ClassTableRow> {
    if rule_set != RuleSetId::Pu {
        return None;
    }
    if level == 0 || level > class_id.max_supported_level() {
        return None;
    }

    if let Some(base) = borrowed_crb_chassis_class(class_id) {
        return class_tables::class_tables()
            .into_iter()
            .find(|row| row.class_id == base && row.level == level)
            .map(|row| ClassTableRow {
                level: row.level,
                base_attack_bonus: row.base_attack_bonus,
                fort_save: row.fort_save,
                ref_save: row.ref_save,
                will_save: row.will_save,
            });
    }

    match class_id {
        PuClassId::UnchainedSummoner => {
            apg::class_chassis_resolve(ApgClassId::Summoner, level, RuleSetId::Apg).map(|row| {
                ClassTableRow {
                    level: row.level,
                    base_attack_bonus: row.base_attack_bonus,
                    fort_save: row.fort_save,
                    ref_save: row.ref_save,
                    will_save: row.will_save,
                }
            })
        }
        // The one class with its own chassis. Every cell is the feature
        // module's own transcription of `pu_abilities_class.lst:115`'s three
        // `BONUS:` tokens; nothing is recomputed here.
        PuClassId::UnchainedMonk => Some(ClassTableRow {
            level,
            base_attack_bonus: monk_features::base_attack_bonus(level),
            fort_save: monk_features::fort_save(level),
            ref_save: monk_features::ref_save(level),
            will_save: monk_features::will_save(level),
        }),
        // Unreachable: both remaining variants took the borrowed-CRB path
        // above. Written as an explicit arm rather than `_ => None` so a
        // fifth variant would be a compile error, not a silent `None`.
        PuClassId::UnchainedBarbarian | PuClassId::UnchainedRogue => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_roster_is_the_four_corpus_class_records() {
        assert_eq!(PuClassId::ALL.len(), 4);
        let names: Vec<&str> = PuClassId::ALL.iter().map(|id| id.name()).collect();
        assert_eq!(
            names,
            vec![
                "unchained_barbarian",
                "unchained_monk",
                "unchained_rogue",
                "unchained_summoner"
            ]
        );
    }

    /// The identity axis of the replacement invariant: no Unchained class
    /// id string equals, or is a prefix/suffix of, the id it replaces.
    #[test]
    fn no_unchained_class_id_collides_with_the_class_it_replaces() {
        for id in PuClassId::ALL {
            let own = format!("class:{}", id.name());
            assert_ne!(own, id.replaces_class_id());
            assert_ne!(PuClassId::from_class_id_str(id.replaces_class_id()), Some(id));
            assert_eq!(PuClassId::from_class_id_str(&own), Some(id));
        }
    }

    /// The display axis. Verbatim from each corpus record's `name` field.
    #[test]
    fn display_names_are_the_corpus_names() {
        assert_eq!(
            PuClassId::UnchainedBarbarian.display_name(),
            "Unchained Barbarian"
        );
        assert_eq!(PuClassId::UnchainedMonk.display_name(), "Unchained Monk");
        assert_eq!(PuClassId::UnchainedRogue.display_name(), "Unchained Rogue");
        assert_eq!(
            PuClassId::UnchainedSummoner.display_name(),
            "Unchained Summoner"
        );
    }

    /// A CRB/APG class id must never resolve to a PU class, and vice versa.
    #[test]
    fn resolution_never_crosses_between_the_books() {
        for base in ["class:barbarian", "class:monk", "class:rogue", "class:summoner"] {
            assert_eq!(PuClassId::from_class_id_str(base), None, "{base}");
        }
        for id in PuClassId::ALL {
            let own = format!("class:{}", id.name());
            assert_eq!(ApgClassId::from_class_id_str(&own), None, "{own}");
        }
    }

    #[test]
    fn the_chassis_resolver_answers_only_for_the_pu_rule_set() {
        for id in PuClassId::ALL {
            assert!(class_chassis_resolve(id, 1, RuleSetId::Pu).is_some());
            for other in [
                RuleSetId::Crb,
                RuleSetId::Apg,
                RuleSetId::Acg,
                RuleSetId::Bestiary1,
                RuleSetId::Arg,
            ] {
                assert_eq!(class_chassis_resolve(id, 1, other), None);
            }
        }
    }

    #[test]
    fn every_class_resolves_every_level_one_through_twenty_and_nothing_outside() {
        for id in PuClassId::ALL {
            assert_eq!(id.max_supported_level(), 20);
            for level in 1..=20u8 {
                assert!(
                    class_chassis_resolve(id, level, RuleSetId::Pu).is_some(),
                    "{} level {level}",
                    id.name()
                );
            }
            assert_eq!(class_chassis_resolve(id, 0, RuleSetId::Pu), None);
            assert_eq!(class_chassis_resolve(id, 21, RuleSetId::Pu), None);
        }
    }

    /// The three borrowing classes must be byte-identical to the base class
    /// table they borrow — that is the whole justification for not
    /// transcribing a second copy. If a future edit invents a PU-local
    /// table, this fails.
    #[test]
    fn the_three_non_overriding_classes_match_their_base_class_row_exactly() {
        for level in 1..=20u8 {
            let barbarian = class_tables::class_tables()
                .into_iter()
                .find(|row| row.class_id == ClassId::Barbarian && row.level == level)
                .expect("CRB Barbarian row");
            let unchained =
                class_chassis_resolve(PuClassId::UnchainedBarbarian, level, RuleSetId::Pu)
                    .expect("Unchained Barbarian row");
            assert_eq!(unchained.base_attack_bonus, barbarian.base_attack_bonus);
            assert_eq!(unchained.fort_save, barbarian.fort_save);
            assert_eq!(unchained.ref_save, barbarian.ref_save);
            assert_eq!(unchained.will_save, barbarian.will_save);

            let rogue = class_tables::class_tables()
                .into_iter()
                .find(|row| row.class_id == ClassId::Rogue && row.level == level)
                .expect("CRB Rogue row");
            let unchained = class_chassis_resolve(PuClassId::UnchainedRogue, level, RuleSetId::Pu)
                .expect("Unchained Rogue row");
            assert_eq!(unchained.base_attack_bonus, rogue.base_attack_bonus);
            assert_eq!(unchained.fort_save, rogue.fort_save);
            assert_eq!(unchained.ref_save, rogue.ref_save);
            assert_eq!(unchained.will_save, rogue.will_save);

            let summoner = apg::class_chassis_resolve(ApgClassId::Summoner, level, RuleSetId::Apg)
                .expect("APG Summoner row");
            let unchained =
                class_chassis_resolve(PuClassId::UnchainedSummoner, level, RuleSetId::Pu)
                    .expect("Unchained Summoner row");
            assert_eq!(unchained.base_attack_bonus, summoner.base_attack_bonus);
            assert_eq!(unchained.fort_save, summoner.fort_save);
            assert_eq!(unchained.ref_save, summoner.ref_save);
            assert_eq!(unchained.will_save, summoner.will_save);
        }
    }

    /// The computation axis of the replacement invariant, for the one class
    /// whose chassis genuinely differs. A future edit that aliases the
    /// Unchained Monk onto the CRB Monk fails here loudly.
    #[test]
    fn the_unchained_monk_chassis_diverges_from_the_crb_monk_at_every_level() {
        for level in 1..=20u8 {
            let crb = class_tables::class_tables()
                .into_iter()
                .find(|row| row.class_id == ClassId::Monk && row.level == level)
                .expect("CRB Monk row");
            let pu = class_chassis_resolve(PuClassId::UnchainedMonk, level, RuleSetId::Pu)
                .expect("Unchained Monk row");

            // Full BAB here, three-quarter in CRB: never lower, and
            // strictly higher at every level from 1 up.
            assert_eq!(pu.base_attack_bonus, i16::from(level), "full BAB");
            assert!(pu.base_attack_bonus >= crb.base_attack_bonus);

            // Will is poor here and good in CRB.
            assert_eq!(pu.will_save, i16::from(level) / 3);
            assert_eq!(crb.will_save, i16::from(level) / 2 + 2);
            assert!(pu.will_save < crb.will_save);

            // Fort/Ref are good in both, so those two cells agree — recorded
            // rather than left implicit, so "they differ" is never read as
            // "every cell differs".
            assert_eq!(pu.fort_save, crb.fort_save);
            assert_eq!(pu.ref_save, crb.ref_save);
        }
    }

    #[test]
    fn hit_dice_come_from_the_stated_source_per_class() {
        assert_eq!(hit_die_for(PuClassId::UnchainedBarbarian), 12);
        assert_eq!(hit_die_for(PuClassId::UnchainedRogue), 8);
        assert_eq!(
            hit_die_for(PuClassId::UnchainedSummoner),
            apg::hit_die_for(ApgClassId::Summoner)
        );
        // The one override: d10, against the CRB Monk's operator-ruled d8.
        assert_eq!(hit_die_for(PuClassId::UnchainedMonk), 10);
        assert_eq!(class_tables::hit_die_for(ClassId::Monk), Some(8));
    }
}
