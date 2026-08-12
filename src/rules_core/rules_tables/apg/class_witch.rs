//! APG Witch class chassis table — one row per level.
//!
//! Source: PCGen `apg_classes.lst`, `CLASS:Witch` record (line 172 of
//! the SD-22 Epic 3 corpus checkout), parsed via
//! `pcgen_import::lst_parser::spellcasting_class` (SD-22 Epic 3 widened
//! `SPELLCASTING_CLASS_NAMES` to recognize it — see that module's doc
//! comment; the real record's `SPELLSTAT:INT` line carries no
//! `MEMORIZE:NO` and no `SPELLBOOK:YES` token, the same absent-signals
//! prepared-casting posture as Cleric/Druid). The real record's
//! chassis-bearing tokens:
//!
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")/2` — half BAB, poor (the first poor-BAB class in this roster).
//! - `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Will save.
//! - `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/3` — poor Fortitude and Reflex saves.
//! - `MAXLEVEL:20`.
//!
//! `tests/sd22_apg_class_witch_resolves.rs`'s
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens` test
//! (real-corpus-gated on `PCGEN_CORPUS_ROOT`) re-parses that exact line
//! so these constants stay tied to the source record.
//!
//! Mirrors `rules_tables::apg::class_oracle`'s scope boundary: only the
//! BAB/save chassis is transcribed here. Named per-level features
//! (Hexes, Patron, familiar rules) and the spell-per-day table are out
//! of scope for this cycle — transcribing them without going back
//! through the LST's per-level feature blocks (`apg_abilities_class.lst`)
//! in a dedicated ingest slice would be exactly the fabricated-data risk
//! `class_alchemist.rs`'s own doc comment and `AGENTS.md` rule out.

use super::ClassTableRow;

/// `MAXLEVEL:20` on the real `CLASS:Witch` record.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

/// `HD:6` on the real `CLASS:Witch` record (v0.6 alpha swarm, risks item
/// 8), verified directly against `apg_classes.lst` line 172.
pub const HIT_DIE: u8 = 6;

fn base_attack_bonus(level: u8) -> i16 {
    level as i16 / 2
}

fn save_bonus(level: u8, good: bool) -> i16 {
    let level = level as i16;
    if good { level / 2 + 2 } else { level / 3 }
}

/// Builds the Witch class table: one row per level, from level 1
/// through `MAX_SUPPORTED_LEVEL`.
pub fn class_table() -> Vec<ClassTableRow> {
    (1..=MAX_SUPPORTED_LEVEL)
        .map(|level| ClassTableRow {
            level,
            base_attack_bonus: base_attack_bonus(level),
            fort_save: save_bonus(level, false),
            ref_save: save_bonus(level, false),
            will_save: save_bonus(level, true),
        })
        .collect()
}
