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
    record
        .bonus_chains
        .iter()
        .find_map(|bonus| {
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
        .or_else(|| tempbonus_stat_fallback(record))
}

/// `AT-34-E3-003` (bucket `M`, equipment sub-causes, cycle 3): the
/// `magic_items`-category sibling of `general.rs`'s own
/// `tempbonus_skill_fallback` — the CRB ability-score potions (`Potion of
/// Bull's Strength`, `Potion of Cat's Grace`, `Potion of Owl's Wisdom`,
/// `Potion of Eagle's Splendor`, `Potion of Fox's Cunning`, `Potion of
/// Bear's Endurance`) carry their real `+4 enhancement bonus to <Ability>`
/// effect as `TEMPBONUS:ANYPC|STAT|<Ability>|4|TYPE=Enhancement`, never a
/// `BONUS:STAT` chain (confirmed against the live corpus: `raw_bonus_
/// chains` is empty on every one of the six). Only fires when no explicit
/// `BONUS:STAT` chain exists, and only for a `PC`/`ANYPC` target — same
/// discipline as the skill-side fallback, see its own doc comment for the
/// `TEMPBONUS:EQ|...` negative case this excludes.
fn tempbonus_stat_fallback(record: &EquipmentRecord) -> Option<AbilityScoreBonus> {
    record.tokens.iter().find_map(|token| {
        if token.key != "TEMPBONUS" {
            return None;
        }
        let parts: Vec<&str> = token.value.split('|').collect();
        if parts.len() < 4 || (parts[0] != "PC" && parts[0] != "ANYPC") || parts[1] != "STAT" {
            return None;
        }
        let ability = parts[2];
        if ability.is_empty() || ability.contains(',') {
            return None;
        }
        parts[3].parse::<i16>().ok().map(|bonus_value| AbilityScoreBonus {
            ability: ability.to_string(),
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

    /// `AT-34-E3-003` (bucket `M`, equipment sub-causes, cycle 3): real
    /// verbatim tokens copied from `KEY:Potion of Bull's Strength` in
    /// `core_rulebook/cr_equip_magic_items.lst` — carries no `BONUS:STAT`
    /// chain at all, only `TEMPBONUS:ANYPC|STAT|STR|4|TYPE=Enhancement`.
    #[test]
    fn potion_of_bulls_strength_yields_a_real_str_bonus_from_tempbonus() {
        let text = "Potion of Bull's Strength\tKEY:Potion of Bull's Strength\tTYPE:Magic.Potion.Consumable.Combat Gear\tCOST:300\tWT:0\tTEMPBONUS:ANYPC|STAT|STR|4|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_magic_items_effect(record);
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
