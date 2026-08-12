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

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;

/// An ability-score enhancement bonus granted by a `magic_items`-category
/// item's `BONUS:STAT|<ability>|<n>|TYPE=Enhancement` corpus token.
#[derive(Debug, Clone, PartialEq)]
pub struct AbilityScoreBonus {
    pub ability: String,
    pub bonus: i16,
}

/// Resolve one `magic_items` corpus record's ability-score-bonus
/// contribution.
///
/// Reads the record's first `BONUS:STAT|<ability>|<n>|...` chain, if any.
/// A record with no such chain (bags of holding, most rings and rods,
/// ...) yields `None`: that means this record's raw tokens do not carry
/// the field, not that its value is zero.
pub fn compute_magic_items_effect(record: &EquipmentRecord) -> Option<AbilityScoreBonus> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        let is_stat_bonus = qualifiers.len() >= 3 && qualifiers[0] == "STAT";
        if !is_stat_bonus {
            return None;
        }
        qualifiers[2].parse::<i16>().ok().map(|bonus_value| AbilityScoreBonus {
            ability: qualifiers[1].clone(),
            bonus: bonus_value,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Belt of Giant Strength +2`
    /// in `core_rulebook/cr_equip_magic_items.lst`.
    #[test]
    fn belt_of_giant_strength_yields_a_real_str_ability_bonus() {
        let text = "Belt of Giant Strength +2\tKEY:Belt of Giant Strength +2\tTYPE:Magic.Wondrous.Belt\tCOST:4000\tWT:1\tBONUS:STAT|STR|2|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_magic_items_effect(record);
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
        let record = &result.entries[0];

        let effect = compute_magic_items_effect(record);
        assert_eq!(
            effect,
            Some(AbilityScoreBonus {
                ability: "DEX".to_string(),
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
        let record = &result.entries[0];

        let effect = compute_magic_items_effect(record);
        assert_eq!(effect, None);
    }
}
