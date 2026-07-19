//! APG (Advanced Player's Guide) book-level module. SD-22 Epic 3
//! content-source ingest — sibling directory to `rules_tables::crb` per
//! `SD-19-corpus-aware-compute-seam/decisions.md` §9. Alchemist is the
//! first class ingested, Cavalier the second, Inquisitor the third,
//! Oracle the fourth, Summoner the fifth (`decisions.md §5`'s corrected
//! real-LST-corpus sourcing, corrected 2026-07-19). Gunslinger and Magus
//! are not real
//! APG content and are permanently excluded from this roster (corrected
//! 2026-07-19, `corpus-source-inventory.md §1`) — the real PCGen corpus
//! has no `CLASS:Gunslinger` or `CLASS:Magus` record anywhere under
//! `advanced_players_guide/`; both live in `ultimate_combat/uc_classes.lst`
//! and `ultimate_magic/um_classes.lst` respectively, books
//! `decisions.md §1` explicitly excludes from SD-22.

pub mod class_alchemist;
pub mod class_cavalier;
pub mod class_inquisitor;
pub mod class_oracle;
pub mod class_summoner;

use crate::rules_core::rules_tables::RuleSetId;

/// One APG class's chassis-table row: level, BAB, and the three saves.
/// Shared shape across every per-class module in this directory so
/// `class_chassis_resolve` can return a single type regardless of
/// which class was queried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassTableRow {
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// Identifies which APG class a chassis-table query targets. Grows by
/// one variant per per-class Epic 3 cycle (Witch remains — Gunslinger
/// and Magus are not real APG content in the PCGen corpus, see this
/// module's doc comment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApgClassId {
    Alchemist,
    Cavalier,
    Inquisitor,
    Oracle,
    Summoner,
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
) -> Option<ClassTableRow> {
    if rule_set != RuleSetId::Apg {
        return None;
    }
    match class_id {
        ApgClassId::Alchemist => class_alchemist::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Cavalier => class_cavalier::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Inquisitor => class_inquisitor::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Oracle => class_oracle::class_table()
            .into_iter()
            .find(|row| row.level == level),
        ApgClassId::Summoner => class_summoner::class_table()
            .into_iter()
            .find(|row| row.level == level),
    }
}
