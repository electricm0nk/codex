//! Canonical Paizo-table store. SD-19 foundation slice.
//!
//! CRB (`crb`) and APG (`apg`) are the populated rule sets today. Future
//! rule books (UM, ACG, Bestiary 1, ...) get sibling directories and
//! their own `RuleSetId` variants in their own STC sub-bundle — see
//! `SD-19-corpus-aware-compute-seam/decisions.md` §9 and
//! `SD-22-content-source-ingest-and-dm-toolkit/decisions.md` §5.

pub mod apg;
pub mod crb;

/// Identifies which Paizo rule book a table cell or resolved corpus
/// record belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleSetId {
    Crb,
    Apg,
    // future: Um, Acg, Bestiary1, ...
}
