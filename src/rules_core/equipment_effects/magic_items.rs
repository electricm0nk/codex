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
#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::equipment_record_to_corpus;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Belt of Giant Strength +2`
    /// in `core_rulebook/cr_equip_magic_items.lst`.
    #[test]
    fn belt_of_giant_strength_yields_a_real_str_ability_bonus() {
        let text = "Belt of Giant Strength +2\tKEY:Belt of Giant Strength +2\tTYPE:Magic.Wondrous.Belt\tCOST:4000\tWT:1\tBONUS:STAT|STR|2|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_magic_items_effect(&record);
        assert_eq!(
            effect,
            Some(AbilityScoreBonus {
                ability: "STR".to_string(),
                bonus: 2,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Belt of Incredible Dexterity
    /// +2` — a different ability entirely, proving the ability name is
    /// read from the token, not hardcoded.
    #[test]
    fn belt_of_incredible_dexterity_yields_a_real_dex_ability_bonus() {
        let text = "Belt of Incredible Dexterity +2\tKEY:Belt of Incredible Dexterity +2\tTYPE:Magic.Wondrous.Belt\tCOST:4000\tWT:1\tBONUS:STAT|DEX|2|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_magic_items_effect(&record);
        assert_eq!(
            effect,
            Some(AbilityScoreBonus {
                ability: "DEX".to_string(),
                bonus: 2,
            })
        );
    }

    /// `AT-34-E3-003` (bucket `M`, equipment sub-causes, cycle 3): real
    /// verbatim tokens copied from `KEY:Potion of Bull's Strength` in
    /// `core_rulebook/cr_equip_magic_items.lst` — carries no `BONUS:STAT`
    /// chain at all, only `TEMPBONUS:ANYPC|STAT|STR|4|TYPE=Enhancement`.
    #[test]
    fn potion_of_bulls_strength_yields_a_real_str_bonus_from_tempbonus() {
        let text = "Potion of Bull's Strength\tKEY:Potion of Bull's Strength\tTYPE:Magic.Potion.Consumable.Combat Gear\tCOST:300\tWT:0\tTEMPBONUS:ANYPC|STAT|STR|4|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_magic_items_effect(&record);
        assert_eq!(
            effect,
            Some(AbilityScoreBonus {
                ability: "STR".to_string(),
                bonus: 4,
            })
        );
    }

    /// A record's own explicit `BONUS:STAT` chain always wins over a
    /// `TEMPBONUS` fallback — negative control mirroring `general.rs`'s
    /// `explicit_bonus_skill_wins_over_a_tempbonus_on_the_same_record`.
    #[test]
    fn explicit_bonus_stat_wins_over_a_tempbonus_on_the_same_record() {
        let text = "Hybrid\tKEY:Hybrid\tTYPE:Magic.Wondrous.Belt\tCOST:1\tWT:1\tBONUS:STAT|STR|2|TYPE=Enhancement\tTEMPBONUS:ANYPC|STAT|DEX|99|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_magic_items_effect(&record);
        assert_eq!(
            effect,
            Some(AbilityScoreBonus {
                ability: "STR".to_string(),
                bonus: 2,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Bag of Holding (Type I)` —
    /// a magic item carries no `BONUS:STAT` token at all.
    #[test]
    fn bag_of_holding_has_no_ability_bonus() {
        let text = "Bag of Holding (Type I)\tKEY:Bag of Holding (Type I)\tTYPE:Magic.Wondrous.Container\tCOST:2500\tWT:15\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_magic_items_effect(&record);
        assert_eq!(effect, None);
    }
}
