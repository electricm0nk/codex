//! APG (Advanced Player's Guide) book-level module. SD-22 Epic 3
//! content-source ingest — sibling directory to `rules_tables::crb` per
//! `SD-19-corpus-aware-compute-seam/decisions.md` §9. Alchemist is the
//! first class ingested (`decisions.md §5`'s corrected real-LST-corpus
//! sourcing, corrected 2026-07-19).

pub mod class_alchemist;

use crate::rules_core::rules_tables::RuleSetId;

/// Identifies which APG class a chassis-table query targets. Grows by
/// one variant per per-class Epic 3 cycle (Cavalier, Gunslinger, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApgClassId {
    Alchemist,
}

/// Resolves an APG class's chassis-table row for `level`, scoped to
/// `RuleSetId::Apg`. Returns `None` for any other rule set — an APG
/// class chassis is never a valid answer for a `RuleSetId::Crb` query
/// (cross-book invariant, `corpus-source-inventory.md` §1.3), and
/// `None` when `level` exceeds the class's real `MAXLEVEL` ceiling.
pub fn class_chassis_resolve(
    class_id: ApgClassId,
    level: u8,
    rule_set: RuleSetId,
) -> Option<class_alchemist::ClassTableRow> {
    if rule_set != RuleSetId::Apg {
        return None;
    }
    match class_id {
        ApgClassId::Alchemist => class_alchemist::class_table()
            .into_iter()
            .find(|row| row.level == level),
    }
}
