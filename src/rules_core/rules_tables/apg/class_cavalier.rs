//! APG Cavalier class chassis table — one row per level.
//!
//! Source: PCGen `apg_classes.lst`, `CLASS:Cavalier` record (line 42 of
//! the SD-22 Epic 3 corpus checkout), parsed via
//! `pcgen_import::lst_parser::class` (SD-22 Epic 3 widened
//! `MARTIAL_CLASS_NAMES` to recognize it — see that module's doc
//! comment; the real record has no `SPELLSTAT:` line, the same
//! non-caster posture as the six original martial classes). The real
//! record's chassis-bearing tokens:
//!
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")` — full BAB (no fractional divisor).
//! - `BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Fortitude save.
//! - `BONUS:SAVE|BASE.Will,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/3` — poor Will and Reflex saves.
//! - `MAXLEVEL:20`.
//!
//! `tests/sd22_apg_class_cavalier_resolves.rs`'s
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens` test
//! (real-corpus-gated on `PCGEN_CORPUS_ROOT`) re-parses that exact line
//! so these constants stay tied to the source record.
//!
//! Mirrors `rules_tables::apg::class_alchemist`'s scope boundary: only
//! the BAB/save chassis is transcribed here. Named per-level features
//! (Order, Challenge, Tactician, Banner, Expert Trainer, ...) are out
//! of scope for this cycle — transcribing them without going back
//! through the LST's per-level feature blocks
//! (`apg_abilities_class.lst`) in a dedicated ingest slice would be
//! exactly the fabricated-data risk `class_alchemist.rs`'s own doc
//! comment and `AGENTS.md` rule out.

use super::ClassTableRow;

/// `MAXLEVEL:20` on the real `CLASS:Cavalier` record.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

/// `HD:10` on the real `CLASS:Cavalier` record (v0.6 alpha swarm, risks
/// item 8), verified directly against `apg_classes.lst` line 42.
pub const HIT_DIE: u8 = 10;

fn base_attack_bonus(level: u8) -> i16 {
    level as i16
}

fn save_bonus(level: u8, good: bool) -> i16 {
    let level = level as i16;
    if good { level / 2 + 2 } else { level / 3 }
}

/// Builds the Cavalier class table: one row per level, from level 1
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
