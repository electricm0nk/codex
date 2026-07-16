//! Canonical Paizo-table store. SD-19 foundation slice.
//!
//! CRB (`crb`) is the only populated rule set today. Future rule books
//! (UM, APG, ...) get sibling directories and their own `RuleSetId`
//! variants in their own STC sub-bundle — see
//! `SD-19-corpus-aware-compute-seam/decisions.md` §9.

pub mod crb;

/// Identifies which Paizo rule book a table cell or resolved corpus
/// record belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleSetId {
    Crb,
    // future: Um, Apg, ...
}
