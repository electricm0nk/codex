//! Ultimate Combat (UC) Ninja class chassis table — one row per level.
//! `SD31-E4-F1-003` (epic-4-mechanism F1): the second UC class to get a
//! real BAB/save chassis, chosen off `SD31-E3-F1-001`'s clearance table
//! (measured `wired_able: 0`, `named_raw: 0` before this cycle) --
//! **the `named_raw: 0` figure is corrected by this cycle, not merely
//! inherited**: the clearance table's own `grep -oE
//! 'Ninja_Archetype_[A-Za-z0-9]+'` evidence method searched
//! `uc_abilities_class.lst` only and found nothing, but Ninja's one real
//! archetype (`Ninja Archetype ~ Scout`) lives in a NESTED subdirectory
//! (`ultimate_combat/support/uc_abilities_class_apg.lst`) the same
//! single-level-join gap `OPEN-ISSUES.md` row 1 already names for
//! `wiring_class::CorpusLines::line()` -- the grep never reached the file.
//! Confirmed against `docs/work-inventory.json`'s own already-ingested
//! `corpus_key: "Ninja Archetype ~ Scout"` record, then verified against
//! the raw `.lst` row directly (`class_ninja.rs`'s sibling
//! `archetype_tables.rs` addition carries the full citation).
//!
//! Source: PCGen `uc_classes.lst`, `CLASS:Ninja` record
//! (`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! ultimate_combat/uc_classes.lst:19`), read directly rather than
//! table-transcribed:
//!
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4|TYPE=Base.REPLACE`
//!   — 3/4 (moderate) BAB, the same shape `crb::class_tables`'s Rogue row
//!   states (Ninja is `TYPE:Base.PC.Rogue`, a Rogue-family class).
//! - `BONUS:SAVE|BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2` — good
//!   Reflex save only (unlike Gunslinger's combined Fort+Reflex token).
//! - `BONUS:SAVE|BASE.Fortitude,BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3`
//!   — poor Fortitude AND poor Will (one combined token, the mirror image
//!   of Gunslinger's good-pair token).
//! - `MAXLEVEL:20`.
//!
//! Formula-computed rather than a hand-typed 20-row literal table, the
//! same choice `class_gunslinger.rs` documents for the identical reason:
//! the three formulas above are exact and re-derivable from the quoted
//! corpus line at any time. No row is invented.

use super::ClassTableRow;

/// `MAXLEVEL:20` on the real `CLASS:Ninja` record.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

/// 3/4 (moderate) BAB: `classlevel*3/4`, integer division, from the real
/// record's `BONUS:COMBAT|BASEAB|classlevel(...)*3/4` token.
fn base_attack_bonus(level: u8) -> i16 {
    (i16::from(level) * 3) / 4
}

/// `good` selects the Reflex-only formula (`level/2+2`, from the real
/// record's `BASE.Reflex` token); Fortitude and Will both use the poor
/// formula (`level/3`, from the real record's combined
/// `BASE.Fortitude,BASE.Will` token).
fn save_bonus(level: u8, good: bool) -> i16 {
    let level = i16::from(level);
    if good { level / 2 + 2 } else { level / 3 }
}

/// Builds the Ninja class table: one row per level, from level 1 through
/// `MAX_SUPPORTED_LEVEL`.
pub fn class_table() -> Vec<ClassTableRow> {
    (1..=MAX_SUPPORTED_LEVEL)
        .map(|level| ClassTableRow {
            level,
            base_attack_bonus: base_attack_bonus(level),
            fort_save: save_bonus(level, false),
            ref_save: save_bonus(level, true),
            will_save: save_bonus(level, false),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 3/4 BAB, one good save (Reflex), two poor saves (Fort/Will), at a
    /// spread of levels including 1 and the 20th-level ceiling. Matches
    /// the real corpus formulas quoted in this module's own doc comment,
    /// not a RAW recollection -- and independently matches the standard
    /// published Ninja class table (BAB +0/+7/+15, Fort +0/+3/+6,
    /// Ref +2/+7/+12, Will +0/+3/+6 at levels 1/10/20).
    #[test]
    fn matches_the_real_corpus_formulas_at_levels_1_10_and_20() {
        for (level, bab, fort, reflex, will) in
            [(1u8, 0i16, 0i16, 2i16, 0i16), (10, 7, 3, 7, 3), (20, 15, 6, 12, 6)]
        {
            assert_eq!(base_attack_bonus(level), bab, "level {level} BAB");
            assert_eq!(save_bonus(level, false), fort, "level {level} Fortitude");
            assert_eq!(save_bonus(level, true), reflex, "level {level} Reflex");
            assert_eq!(save_bonus(level, false), will, "level {level} Will");
        }
    }

    #[test]
    fn class_table_has_exactly_twenty_rows_in_order() {
        let table = class_table();
        assert_eq!(table.len(), 20);
        for (i, row) in table.iter().enumerate() {
            assert_eq!(row.level, (i + 1) as u8);
        }
    }
}
