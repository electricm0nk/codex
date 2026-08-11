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
}
