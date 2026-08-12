//! Canonical Paizo-table store. SD-19 foundation slice.
//!
//! CRB (`crb`), APG (`apg`), ACG (`acg`), Bestiary 1 (`beastiary1`),
//! Advanced Race Guide (`advanced_race_guide`), and Pathfinder Unchained
//! (`pathfinder_unchained`) are the populated rule sets today. Future rule
//! books (UM, ...) get sibling directories and their own `RuleSetId`
//! variants in their own STC sub-bundle — see
//! `SD-19-corpus-aware-compute-seam/decisions.md` §9 and
//! `SD-22-content-source-ingest-and-dm-toolkit/decisions.md` §5.

pub mod acg;
pub mod advanced_race_guide;
pub mod apg;
pub mod archetype_swap;
pub mod beastiary1;
pub mod bonus_bestiary;
pub mod class_spell_levels;
pub mod crb;
pub mod equipment_gap_tables;
pub mod feat_gap_tables;
pub mod feats_all;
pub mod monster_chassis;
pub mod monster_codex;
pub mod pathfinder_unchained;
pub mod ultimate_campaign;
pub mod ultimate_equipment;
pub mod ultimate_intrigue;
pub mod ultimate_combat;
pub mod ultimate_magic;
pub mod ultimate_psionics;
pub mod ultimate_wilderness;

/// Identifies which Paizo rule book a table cell or resolved corpus
/// record belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleSetId {
    Crb,
    Apg,
    Acg,
    Bestiary1,
    Arg,
    Pu,
    Uca,
    /// Ultimate Intrigue. SD-28 Epic 24 -- first record family (feats).
    Ui,
    /// Ultimate Equipment. SD-28 Epic 25 -- first record family (equipment).
    Ue,
    /// Ultimate Wilderness. SD-28 Epic 26 -- first record family (feats).
    Uw,
    /// Ultimate Combat. SD-28 Epic 27 -- first record family (feats).
    Uc,
    /// Ultimate Magic. SD-28 Epic 28 -- first record family (feats).
    Um,
    /// Ultimate Psionics. SD-28 Epic 29 -- first record family (feats).
    /// Dreamscarred Press, not Paizo -- the last Ultimate book.
    Upsi,
    /// Bonus Bestiary. SD-29 Epic 5 pilot -- first book to ingest the merged
    /// `monster` + `monster_ability` chassis (`corpus-work-channels.md §9.2`).
    BonusBestiary,
    /// Monster Codex. SD-29 Epic 6 pilot (race-trait lane, `decisions.md §43`)
    /// and Epic 5's second monster book (`rules_tables::monster_codex`, 2
    /// monsters + 3 monster abilities).
    ///
    /// Its `race_trait` records are still served off disk from
    /// `data/corpus/monster_codex/race_trait/` rather than from a compiled
    /// table: `decisions.md §24` rules out the formula interpreter a compiled
    /// race-trait table would need. So this book is the one place where the two
    /// halves of "the engine has compiled this book" are visibly different
    /// kinds of thing -- a compiled monster table and a disk-served race-trait
    /// family -- and `COMPILED_RULE_SETS` answers for both.
    MonsterCodex,
    /// Inner Sea Races. SD-29 Epic 6 round 2 (race-trait lane, extend). Like
    /// `MonsterCodex`, its one ingested family is `race_trait`, served off disk
    /// from `data/corpus/inner_sea_races/race_trait/` rather than from a
    /// compiled table (`decisions.md §24` rules out the formula interpreter a
    /// compiled race-trait table would need). It is the largest single
    /// alternate-racial-trait contribution after ARG's.
    Isr,
}
