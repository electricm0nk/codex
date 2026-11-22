//! Epic 5, third equipment category (SD-20 §1.5 work-unit order): CRB
//! `magic_items` per-item effect resolution.
//!
//! Unlike `arms_armor` (`ACCHECK:`/`MAXDEX:`/`SPELLFAILURE:`/
//! `BONUS:COMBAT|AC|...`) and `general` (`BONUS:SKILL|...`), the CRB
//! `magic_items` block (`core_rulebook/cr_equip_magic_items.lst`) is
//! dominated by wondrous items whose real, load-bearing mechanical effect
//! is a `BONUS:STAT|<ability>|<n>|TYPE=Enhancement` ability-score
//! enhancement bonus — confirmed directly against the real corpus: it is
//! the single most common `BONUS:` token in this category's file (50
//! occurrences, more than `BONUS:COMBAT`'s 24, `BONUS:SKILL`'s 17, or any
//! other single `BONUS:` type), e.g. `KEY:Belt of Giant Strength +2`
//! carries `BONUS:STAT|STR|2|TYPE=Enhancement` and `KEY:Belt of
//! Incredible Dexterity +2` carries `BONUS:STAT|DEX|2|TYPE=Enhancement`.
//! Many other `magic_items` records (bags of holding, most rings and
//! rods, ...) carry no `BONUS:STAT` token at all, so `None` for those is
//! an honest absence, not a fabricated zero. No field here is
//! hand-rolled; every value traces back to a real, verbatim corpus
//! token, read the same way `arms_armor.rs` and `general.rs` read their
//! own tokens straight off the resolved record.

use crate::rules_core::equipment_record::CorpusEquipmentRecord;

/// An ability-score enhancement bonus a `magic_items`-category item grants.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 10: the value is settled at ingest, by
/// [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`], and this
/// module reads it off the converted record. Which source token stated it --
/// an unconditional bonus chain, or the `TEMPBONUS:` form the CRB
/// ability-score potions use -- is the converter's business, not the sheet's.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AbilityScoreBonus {
    pub ability: String,
    pub bonus: i16,
}

/// One `magic_items` corpus record's ability-score-bonus contribution.
///
/// `None` for a record that states none (bags of holding, most rings and
/// rods): honest absence, not a zero.
pub fn compute_magic_items_effect(record: &CorpusEquipmentRecord) -> Option<AbilityScoreBonus> {
    record.ability_score_bonus.clone()
}

