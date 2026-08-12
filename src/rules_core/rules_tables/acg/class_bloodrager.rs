//! ACG Bloodrager class chassis table — one row per level.
//!
//! Source: PCGen `acg_classes.lst`, `CLASS:Bloodrager` record (line 40 of
//! the SD-22 Epic 4 corpus checkout), parsed via
//! `pcgen_import::lst_parser::spellcasting_class` (SD-22 Epic 4 widened
//! `SPELLCASTING_CLASS_NAMES` to recognize it — see that module's doc
//! comment; the real record's `SPELLSTAT:CHA MEMORIZE:NO` line carries
//! the same spontaneous posture as Sorcerer/Bard/Oracle/Summoner). The
//! real record's chassis-bearing tokens:
//!
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE` — full BAB (no fractional divisor).
//! - `BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Fortitude save.
//! - `BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3` — poor Reflex and Will saves.
//! - `MAXLEVEL:20`.
//!
//! `tests/sd22_acg_class_bloodrager_resolves.rs`'s
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens` test
//! (real-corpus-gated on `PCGEN_CORPUS_ROOT`) re-parses that exact line
//! so these constants stay tied to the source record.
//!
//! Mirrors `rules_tables::acg::class_arcanist`'s scope boundary: only the
//! BAB/save chassis is transcribed here. Named per-level features
//! (Bloodline powers, Bloodrage, Fast Movement, ...) and the bloodrager
//! spell-per-day table are out of scope for this cycle — transcribing
//! them without going back through the LST's per-level feature blocks
//! (`acg_abilities_class.lst`) in a dedicated ingest slice would be
//! exactly the fabricated-data risk `class_tables.rs`'s own doc comment
//! and `AGENTS.md` rule out.

use super::ClassTableRow;

/// `MAXLEVEL:20` on the real `CLASS:Bloodrager` record.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

fn base_attack_bonus(level: u8) -> i16 {
    level as i16
}

fn save_bonus(level: u8, good: bool) -> i16 {
    let level = level as i16;
    if good { level / 2 + 2 } else { level / 3 }
}

/// Builds the Bloodrager class table: one row per level, from level 1
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

/// `HD:10` on the real `CLASS:Bloodrager` record (Bloodrager HD:10 in
/// `advanced_class_guide/acg_classes.lst`).
pub const HIT_DIE: u8 = 10;
