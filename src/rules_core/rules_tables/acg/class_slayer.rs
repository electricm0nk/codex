//! ACG Slayer class chassis table — one row per level.
//!
//! Source: PCGen `acg_classes.lst`, `CLASS:Slayer` record (line 327 of
//! the SD-22 Epic 4 corpus checkout), parsed via
//! `pcgen_import::lst_parser::class` (SD-22 Epic 4 widened
//! `MARTIAL_CLASS_NAMES` to recognize it — see that module's doc
//! comment; the real record carries no `SPELLSTAT:` line, the same
//! non-caster posture as Cavalier/Brawler). The real record's
//! chassis-bearing tokens:
//!
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE` — full BAB (no fractional divisor).
//! - `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Fortitude and Reflex saves (one combined token).
//! - `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3` — poor Will save.
//! - `MAXLEVEL:20`.
//!
//! `tests/sd22_acg_class_slayer_resolves.rs`'s
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens` test
//! (real-corpus-gated on `PCGEN_CORPUS_ROOT`) re-parses that exact line
//! so these constants stay tied to the source record.
//!
//! Mirrors `rules_tables::acg::class_brawler`'s scope boundary: only
//! the BAB/save chassis is transcribed here. Named per-level features
//! (Studied Target, Track, Slayer Talents, Sneak Attack, Stalker,
//! Quarry, ...) are out of scope for this cycle — transcribing them
//! without going back through the LST's per-level feature blocks
//! (`acg_abilities_class.lst`) in a dedicated ingest slice would be
//! exactly the fabricated-data risk `class_tables.rs`'s own doc comment
//! and `AGENTS.md` rule out.

use super::ClassTableRow;

/// `MAXLEVEL:20` on the real `CLASS:Slayer` record.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

fn base_attack_bonus(level: u8) -> i16 {
    level as i16
}

/// `good` selects the Fortitude/Reflex formula (`level/2+2`, from the
/// real record's combined `BASE.Fortitude,BASE.Reflex` token); the Will
/// save uses the poor formula (`level/3`, from the real record's
/// `BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3` token).
fn save_bonus(level: u8, good: bool) -> i16 {
    let level = level as i16;
    if good { level / 2 + 2 } else { level / 3 }
}

/// Builds the Slayer class table: one row per level, from level 1
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

/// `HD:10` on the real `CLASS:Slayer` record (Slayer HD:10 in
/// `advanced_class_guide/acg_classes.lst`).
pub const HIT_DIE: u8 = 10;
