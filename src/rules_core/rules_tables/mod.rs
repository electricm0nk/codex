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
pub mod beastiary1;
pub mod class_spell_levels;
pub mod crb;
pub mod feats_all;
pub mod pathfinder_unchained;
pub mod ultimate_campaign;

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
    // future: Um, ...
}
