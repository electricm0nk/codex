//! Ultimate Combat (UC) Gunslinger class chassis table — one row per
//! level. `SD31-E4-F1-002` (epic-4-mechanism F1): the first UC class to
//! get a real BAB/save chassis, chosen off `SD31-E3-F1-001`'s clearance
//! table (measured `wired_able: 0`, `named_raw: 17` before this cycle).
//!
//! Source: PCGen `uc_classes.lst`, `CLASS:Gunslinger` record
//! (`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! ultimate_combat/uc_classes.lst:10`), read directly rather than
//! table-transcribed, because the real record states its progression as
//! plain formulas rather than a `Base.PC` alias into a shared table:
//!
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE`
//!   — full BAB (`level`), the same shape as `acg::class_slayer`'s.
//! - `BONUS:SAVE|BASE.Reflex,BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2`
//!   — good Fortitude and Reflex saves (one combined token).
//! - `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3` — poor Will
//!   save.
//! - `MAXLEVEL:20`.
//!
//! Formula-computed rather than a hand-typed 20-row literal table (unlike
//! `crb::class_tables`'s CRB rows, which the corpus states as literal
//! per-level columns): the three formulas above are exact and PF1's
//! standard full-BAB/two-good-saves shape, the same one `class_slayer.rs`
//! already computes this way. No row is invented; every value a caller
//! reads is the direct formula result, re-derivable from the corpus line
//! quoted above at any time.
//!
//! Mirrors `acg::class_slayer`'s own scope boundary: only the BAB/save
//! chassis lives here. Gunslinger's named per-level features (Grit, Gun
//! Training, Nimble, Gunslinger Initiative, ...) are grounded directly in
//! `pilot_compute.rs`, the same split `class_slayer.rs` documents for
//! Slayer.

use super::ClassTableRow;

/// `MAXLEVEL:20` on the real `CLASS:Gunslinger` record.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

fn base_attack_bonus(level: u8) -> i16 {
    level as i16
}

/// `good` selects the Fortitude/Reflex formula (`level/2+2`, from the
/// real record's combined `BASE.Reflex,BASE.Fortitude` token); the Will
/// save uses the poor formula (`level/3`, from the real record's
/// `BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3` token).
fn save_bonus(level: u8, good: bool) -> i16 {
    let level = level as i16;
    if good { level / 2 + 2 } else { level / 3 }
}

/// Builds the Gunslinger class table: one row per level, from level 1
/// through `MAX_SUPPORTED_LEVEL`.
pub fn class_table() -> Vec<ClassTableRow> {
    (1..=MAX_SUPPORTED_LEVEL)
        .map(|level| ClassTableRow {
            level,
            base_attack_bonus: base_attack_bonus(level),
            fort_save: save_bonus(level, true),
            ref_save: save_bonus(level, true),
            will_save: save_bonus(level, false),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Full BAB, two good saves (Fort/Reflex), one poor save (Will), at a
    /// spread of levels including 1 and the 20th-level ceiling. Matches
    /// the real corpus formulas quoted in this module's own doc comment,
    /// not a RAW recollection.
    #[test]
    fn matches_the_real_corpus_formulas_at_levels_1_10_and_20() {
        for (level, bab, fort, reflex, will) in
            [(1u8, 1i16, 2i16, 2i16, 0i16), (10, 10, 7, 7, 3), (20, 20, 12, 12, 6)]
        {
            assert_eq!(base_attack_bonus(level), bab, "level {level} BAB");
            assert_eq!(save_bonus(level, true), fort, "level {level} Fortitude");
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
