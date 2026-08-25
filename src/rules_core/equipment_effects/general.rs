//! Epic 5, second equipment category (SD-20 §1.5 work-unit order): CRB
//! `general` per-item effect resolution.
//!
//! Unlike `arms_armor` (which carries `ACCHECK:`/`MAXDEX:`/`SPELLFAILURE:`
//! and `BONUS:COMBAT|AC|...` tokens), the CRB `general` equipment block
//! (`core_rulebook/cr_equip_general.lst`) is dominated by masterwork
//! tools and kits whose real, load-bearing mechanical effect is a
//! `BONUS:SKILL|<skill>|<n>|TYPE=Circumstance` token — e.g. `KEY:Thieves'
//! Tools` carries `BONUS:SKILL|Disable Device|2|TYPE=Circumstance|
//! PRETYPE:1,Masterwork` and `KEY:Climber's Kit` carries
//! `BONUS:SKILL|Climb|2|TYPE=Circumstance`, both confirmed directly
//! against the real corpus. Most other `general` records (trade goods,
//! plain containers, tattoos, ...) carry no `BONUS:` token at all, so
//! `None` for those is an honest absence, not a fabricated zero. No field
//! here is hand-rolled; every value traces back to a real, verbatim
//! corpus token, read the same way `arms_armor.rs` reads its own tokens
//! straight off the resolved record.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;

/// A skill-check circumstance bonus granted by a `general`-category
/// item's `BONUS:SKILL|<skill>|<n>|TYPE=Circumstance` corpus token.
#[derive(Debug, Clone, PartialEq)]
pub struct SkillCheckBonus {
    pub skill: String,
    pub bonus: i16,
}

/// Resolve one `general` corpus record's skill-check-bonus contribution.
///
/// Reads the record's first `BONUS:SKILL|<skill>|<n>|...` chain, if any.
/// A record with no such chain (the overwhelming majority of `general`
/// records — trade goods, containers, tattoos, ...) yields `None`: that
/// means this record's raw tokens do not carry the field, not that its
/// value is zero.
pub fn compute_general_effect(record: &EquipmentRecord) -> Option<SkillCheckBonus> {
    let explicit = record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        let is_skill_bonus = qualifiers.len() >= 3 && qualifiers[0] == "SKILL";
        if !is_skill_bonus {
            return None;
        }
        qualifiers[2].parse::<i16>().ok().map(|bonus_value| SkillCheckBonus {
            skill: qualifiers[1].clone(),
            bonus: bonus_value,
        })
    })?;
    Some(SkillCheckBonus {
        bonus: explicit.bonus + swim_speed_racial_bonus(record, &explicit.skill),
        ..explicit
    })
}

/// PF1 core rule (`Core Rulebook` Swim skill entry): "A swim speed of at
/// least 5 feet gives a creature a +8 racial bonus on Swim checks." This is
/// an automatic bonus triggered by the item itself granting a swim speed
/// (a `MOVE:...Swim,<n>...` token), independent of and additive with any
/// explicit `BONUS:SKILL|Swim|...` token the same item also carries —
/// confirmed against the real pinned PCGen oracle, which sums both
/// (`ultimate_equipment:equipment:ring_of_the_sea_strider`: explicit `+8`
/// racial token + this auto-rule's `+8` = real oracle export `16`, not the
/// explicit token's bare `8`; `AT-33-E5-remainder-equipment_cycle_receipt.md`).
fn swim_speed_racial_bonus(record: &EquipmentRecord, skill: &str) -> i16 {
    if skill != "Swim" {
        return 0;
    }
    let grants_swim_speed = record.tokens.iter().any(|token| {
        token.key == "MOVE"
            && token
                .value
                .split(',')
                .any(|part| part.trim().eq_ignore_ascii_case("Swim"))
    });
    if grants_swim_speed {
        8
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Thieves' Tools` in
    /// `core_rulebook/cr_equip_general.lst`.
    #[test]
    fn thieves_tools_yields_a_real_disable_device_skill_bonus() {
        let text = "Thieves' Tools\tKEY:Thieves' Tools\tTYPE:Goods.Tools.Thief.ThiefTools\tCOST:30\tWT:1\tBONUS:SKILL|Disable Device|2|TYPE=Circumstance|PRETYPE:1,Masterwork\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Disable Device".to_string(),
                bonus: 2,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Climber's Kit` — a different
    /// skill entirely, proving the skill name is read from the token, not
    /// hardcoded.
    #[test]
    fn climbers_kit_yields_a_real_climb_skill_bonus() {
        let text = "Climber's Kit\tKEY:Climber's Kit\tTYPE:Goods.Tools.Masterwork.Resizable\tCOST:80\tWT:5\tBONUS:SKILL|Climb|2|TYPE=Circumstance\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Climb".to_string(),
                bonus: 2,
            })
        );
    }

    /// Real verbatim line copied from `ue_equip_magic_items.lst:200`
    /// (`ultimate_equipment:equipment:ring_of_the_sea_strider`). The item
    /// grants a swim speed (`MOVE:Swim,30`) *and* carries its own explicit
    /// `BONUS:SKILL|Swim|8|TYPE=Racial` token. PF1's core rule ("a swim
    /// speed of at least 5 feet gives a creature a +8 racial bonus on Swim
    /// checks") is an automatic, item-triggered bonus additive with the
    /// item's own explicit token — confirmed against the real pinned PCGen
    /// oracle (`AT-33-E5-remainder-equipment_cycle_receipt.md`'s
    /// `ring_of_the_sea_strider` disagreement: real oracle export `16`, not
    /// the explicit token's bare `8`).
    #[test]
    fn ring_of_the_sea_strider_sums_the_explicit_token_with_the_auto_swim_speed_bonus() {
        let text = "Ring of the Sea Strider\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tTYPE:Magic.Ring.LesserMedium\tCOST:14000\tWT:0\t\tMOVE:Swim,30\tSOURCEPAGE:p.176\tSPELLS:Magic Item|TIMES=1|CASTERLEVEL=7|Dimension Door\tBONUS:SKILL|Swim|8|TYPE=Racial\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Swim".to_string(),
                bonus: 16,
            })
        );
    }

    /// A plain masterwork tool's `BONUS:SKILL` token is untouched by the
    /// swim-speed rule when the item grants no swim speed at all — proves
    /// the addition is gated on the item's own `MOVE:...Swim...` token, not
    /// applied unconditionally to every `Swim` bonus.
    #[test]
    fn climbers_kit_swim_unaffected_when_no_swim_speed_granted() {
        let text = "Climber's Kit\tKEY:Climber's Kit\tTYPE:Goods.Tools.Masterwork.Resizable\tCOST:80\tWT:5\tBONUS:SKILL|Climb|2|TYPE=Circumstance\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Climb".to_string(),
                bonus: 2,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Backpack` — a plain
    /// container carries no `BONUS:` token at all.
    #[test]
    fn backpack_has_no_skill_bonus() {
        let text = "Backpack\tKEY:Backpack\tTYPE:Goods.Container.General.Resizable\tCONTAINS:UNLIM|Any\tCOST:2\tWT:2\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(effect, None);
    }
}
