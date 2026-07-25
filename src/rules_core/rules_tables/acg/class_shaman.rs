//! ACG Shaman class chassis table — one row per level.
//!
//! Source: PCGen `acg_classes.lst`, `CLASS:Shaman` record (line 221 of
//! the SD-22 Epic 4 corpus checkout), parsed via
//! `pcgen_import::lst_parser::spellcasting_class` (SD-22 Epic 4 widened
//! `SPELLCASTING_CLASS_NAMES` to recognize it — see that module's doc
//! comment; the real record's `SPELLSTAT:WIS MEMORIZE:YES` line, with no
//! `SPELLBOOK:YES` and no `MEMORIZE:NO`, carries the same
//! standard-prepared posture as APG's Witch). The real record's
//! chassis-bearing tokens:
//!
//! - `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4|TYPE=Base.REPLACE` — three-quarter BAB.
//! - `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Will save.
//! - `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/3` — poor Fortitude and Reflex saves (one combined token).
//! - `MAXLEVEL:20`.
//!
//! `tests/sd22_acg_class_shaman_resolves.rs`'s
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens` test
//! (real-corpus-gated on `PCGEN_CORPUS_ROOT`) re-parses that exact line
//! so these constants stay tied to the source record.
//!
//! Mirrors `rules_tables::acg::class_investigator`'s scope boundary:
//! only the BAB/save chassis is transcribed here. Named per-level
//! features (Spirit, Spirit Magic, Wandering Spirit, Manifestation,
//! Totem Transformation, ...) and the Shaman's full-9-level
//! spell-per-day table are out of scope for this cycle — transcribing
//! them without going back through the LST's per-level feature blocks
//! (`acg_abilities_class.lst`) in a dedicated ingest slice would be
//! exactly the fabricated-data risk `class_tables.rs`'s own doc comment
//! and `AGENTS.md` rule out.

use super::ClassTableRow;

/// `MAXLEVEL:20` on the real `CLASS:Shaman` record.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

fn base_attack_bonus(level: u8) -> i16 {
    (level as i16 * 3) / 4
}

fn poor_save(level: u8) -> i16 {
    level as i16 / 3
}

fn good_save(level: u8) -> i16 {
    level as i16 / 2 + 2
}

/// Builds the Shaman class table: one row per level, from level 1
/// through `MAX_SUPPORTED_LEVEL`.
pub fn class_table() -> Vec<ClassTableRow> {
    (1..=MAX_SUPPORTED_LEVEL)
        .map(|level| ClassTableRow {
            level,
            base_attack_bonus: base_attack_bonus(level),
            fort_save: poor_save(level),
            ref_save: poor_save(level),
            will_save: good_save(level),
        })
        .collect()
}

/// `HD:8` on the real `CLASS:Shaman` record (Shaman HD:8 in
/// `advanced_class_guide/acg_classes.lst`).
pub const HIT_DIE: u8 = 8;
