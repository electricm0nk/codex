//! Ultimate Combat (UC) Samurai class chassis table — one row per level.
//! `SD31-E4-F1-004` (epic-4-mechanism F1): the third UC class to get a
//! real BAB/save chassis, continuing `SD31-E4-F1-002`'s Gunslinger and
//! `SD31-E4-F1-003`'s Ninja off `SD31-E3-F1-001`'s clearance table
//! (measured `wired_able: 0`, `named_raw: 0` -- Samurai's `named_raw: 0`
//! is RE-VERIFIED this cycle, not merely inherited: `grep -rn "Samurai
//! Archetype"` across the full pinned oracle tree (not just
//! `uc_abilities_class.lst`, the same nested-directory gap `class_ninja.rs`
//! found for Ninja's Scout) turns up exactly two hits, both structural --
//! `uc_abilities_globalvar.lst:8`'s `ABILITY:Samurai Archetype|AUTOMATIC|
//! Archetype Samurai` grant and `uc_abilitycategories.lst:40`'s
//! `ABILITYCATEGORY:Samurai Archetype` category definition -- neither a
//! real swappable archetype entry. Samurai genuinely has zero real
//! archetype content in the 23-book scope, so this cycle wires base
//! chassis + class-feature mechanism only, no supersession branch.
//!
//! Source: PCGen `uc_classes.lst`, `CLASS:Samurai` record
//! (`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! ultimate_combat/uc_classes.lst:34`), read directly rather than
//! table-transcribed:
//!
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE`
//!   — full (good) BAB, the same shape `crb::class_tables`'s Fighter row
//!   states.
//! - `BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2` —
//!   good Fortitude save only.
//! - `BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3`
//!   — poor Reflex AND poor Will (one combined token).
//! - `MAXLEVEL:20`.
//!
//! Formula-computed rather than a hand-typed 20-row literal table, the
//! same choice `class_gunslinger.rs`/`class_ninja.rs` document for the
//! identical reason: the three formulas above are exact and re-derivable
//! from the quoted corpus line at any time. No row is invented.

use super::ClassTableRow;

/// `MAXLEVEL:20` on the real `CLASS:Samurai` record.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

/// Full (good) BAB: `classlevel`, from the real record's
/// `BONUS:COMBAT|BASEAB|classlevel(...)` token.
fn base_attack_bonus(level: u8) -> i16 {
    i16::from(level)
}

/// `good` selects the Fortitude-only formula (`level/2+2`, from the real
/// record's `BASE.Fortitude` token); Reflex and Will both use the poor
/// formula (`level/3`, from the real record's combined
/// `BASE.Reflex,BASE.Will` token).
fn save_bonus(level: u8, good: bool) -> i16 {
    let level = i16::from(level);
    if good { level / 2 + 2 } else { level / 3 }
}

/// Builds the Samurai class table: one row per level, from level 1
/// through `MAX_SUPPORTED_LEVEL`.
pub fn class_table() -> Vec<ClassTableRow> {
    (1..=MAX_SUPPORTED_LEVEL)
        .map(|level| ClassTableRow {
            level,
            base_attack_bonus: base_attack_bonus(level),
            fort_save: save_bonus(level, true),
            ref_save: save_bonus(level, false),
            will_save: save_bonus(level, false),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Full BAB, one good save (Fortitude), two poor saves (Reflex/Will),
    /// at a spread of levels including 1 and the 20th-level ceiling.
    /// Matches the real corpus formulas quoted in this module's own doc
    /// comment, not a RAW recollection -- and independently matches the
    /// standard published Samurai class table (BAB +1/+10/+20,
    /// Fort +2/+7/+12, Ref +0/+3/+6, Will +0/+3/+6 at levels 1/10/20).
    #[test]
    fn matches_the_real_corpus_formulas_at_levels_1_10_and_20() {
        for (level, bab, fort, reflex, will) in
            [(1u8, 1i16, 2i16, 0i16, 0i16), (10, 10, 7, 3, 3), (20, 20, 12, 6, 6)]
        {
            assert_eq!(base_attack_bonus(level), bab, "level {level} BAB");
            assert_eq!(save_bonus(level, true), fort, "level {level} Fortitude");
            assert_eq!(save_bonus(level, false), reflex, "level {level} Reflex");
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
