//! ACG (Advanced Class Guide) book-level module. SD-22 Epic 4
//! content-source ingest — sibling directory to `rules_tables::apg` and
//! `rules_tables::crb` per `SD-19-corpus-aware-compute-seam/decisions.md`
//! §9. Arcanist is the first class ingested (`decisions.md §5`'s
//! corrected real-LST-corpus sourcing, corrected 2026-07-19).
//!
//! **Roster correction (this cycle, mirrors the APG Gunslinger/Magus
//! correction in `apg/mod.rs`):** `corpus-source-inventory.md §2.1`'s
//! row 1, "Alchemist (ACG-side)", names a class with **no real
//! `CLASS:Alchemist` record anywhere in `acg_classes.lst`** — confirmed
//! by direct grep of the real corpus (`grep -c "^CLASS:Alchemist"
//! acg_classes.lst` → 0). Alchemist is APG-only content; ACG never
//! republishes a distinct Alchemist chassis. The real, complete 10-class
//! `CLASS:` roster in `acg_classes.lst` is: Arcanist, Bloodrager,
//! Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler,
//! Warpriest (plus an internal `Ex-Warpriest` `VISIBLE:NO` variant,
//! correctly excluded from the player-facing roster). `decisions.md
//! §3`'s stated ACG ordering also omits `Slayer` (which does have a
//! real record) while wrongly including "Alchemist" — the same
//! roster-defect shape as the resolved Gunslinger/Magus blocker. See
//! `docs/release/SD-22/progress.md`'s `## Open blockers` for the full
//! record. This module's roster starts at Arcanist, the first class
//! with a real record, and will grow to the corrected 10-class list as
//! subsequent Epic 4 cycles land.

pub mod class_arcanist;
pub mod class_bloodrager;
pub mod class_brawler;
pub mod class_hunter;

use crate::rules_core::rules_tables::RuleSetId;

/// One ACG class's chassis-table row: level, BAB, and the three saves.
/// Shared shape across every per-class module in this directory so
/// `class_chassis_resolve` can return a single type regardless of
/// which class was queried. Mirrors `rules_tables::apg::ClassTableRow`
/// (kept book-local rather than shared, per that module's own
/// established shape).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassTableRow {
    pub level: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

/// Identifies which ACG class a chassis-table query targets. Arcanist,
/// Bloodrager, Brawler, and Hunter are the first four real ACG classes
/// landed; the roster grows as subsequent Epic 4 cycles ingest the
/// remaining 6 real classes (Investigator, Shaman, Skald, Slayer,
/// Swashbuckler, Warpriest — see this module's doc comment for the
/// roster correction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AcgClassId {
    Arcanist,
    Bloodrager,
    Brawler,
    Hunter,
}

/// Resolves an ACG class's chassis-table row for `level`, scoped to
/// `RuleSetId::Acg`. Returns `None` for any other rule set — an ACG
/// class chassis is never a valid answer for a `RuleSetId::Crb` or
/// `RuleSetId::Apg` query (cross-book invariant,
/// `corpus-source-inventory.md` §2.3), and `None` when `level` exceeds
/// the class's real `MAXLEVEL` ceiling.
pub fn class_chassis_resolve(
    class_id: AcgClassId,
    level: u8,
    rule_set: RuleSetId,
) -> Option<ClassTableRow> {
    if rule_set != RuleSetId::Acg {
        return None;
    }
    match class_id {
        AcgClassId::Arcanist => class_arcanist::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Bloodrager => class_bloodrager::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Brawler => class_brawler::class_table()
            .into_iter()
            .find(|row| row.level == level),
        AcgClassId::Hunter => class_hunter::class_table()
            .into_iter()
            .find(|row| row.level == level),
    }
}
