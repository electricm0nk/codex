//! Epic 5, first equipment category (SD-20 §1.5 work-unit order): CRB
//! `arms_armor` per-item effect resolution.
//!
//! The canonical CRB equipment-table store
//! (`rules_tables::crb::equipment_tables`) only carries `key` / `category`
//! / `name` / `cost_gp` per record (see that module's own doc comment: a
//! "bootstrap coverage" projection). It does not yet carry the
//! armor/shield stat columns (`ACCHECK:` / `MAXDEX:` / `SPELLFAILURE:` /
//! `BONUS:COMBAT|AC|...`) this epic needs. Those columns *are* present on
//! the real PCGen corpus record (`EquipmentRecord.tokens` /
//! `EquipmentRecord.bonus_chains`), verified directly against
//! `core_rulebook/cr_equip_arms_armor.lst` (e.g. `KEY:Leather Armor
//! (Base)` carries `ACCHECK:0`, `MAXDEX:6`, `SPELLFAILURE:10`, and
//! `BONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0`) — so this
//! function reads them straight off the resolved record, exactly the
//! pattern `equipment_resolver.rs` already uses for `KEY:`/`name`
//! resolution and `spell_resolver.rs` uses for spell school. No field
//! here is hand-rolled or fabricated; every value traces back to a real,
//! verbatim corpus token.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::rules_core::equipment_effects::EquipmentStatEffect;

/// Resolve one `arms_armor` corpus record's armor/shield stat
/// contribution.
///
/// - `armor_class_bonus` comes from the record's first
///   `BONUS:COMBAT|AC|<n>|TYPE=Armor` (or `TYPE=Shield`) chain — the
///   record's own "Broken" penalty chain
///   (`PRETYPE:1,EQMOD=Special Quality ~ Broken ~ Armor`) is a
///   conditional variant that only applies to a broken item and is never
///   the first `COMBAT|AC` chain on an unbroken record, so taking the
///   first match is the correct default (non-broken) armor/shield bonus.
/// - `max_dex` and `spell_failure` come straight off the `MAXDEX:` and
///   `SPELLFAILURE:` tokens when present (weapons and shieldless items
///   carry neither, so both are `None` for e.g. a longsword).
/// - `armor_check_penalty` comes straight off the `ACCHECK:` token (v0.6
///   alpha swarm item 1, shape (c)) — present on every armor/shield
///   record (`0` for no penalty, a negative number for a real one), the
///   same token this module's own doc comment already cited as present
///   on `KEY:Leather Armor (Base)` before this field existed to hold it.
///
/// Absence (`None`) is honest: it means this record's raw tokens do not
/// carry that field, not that the field's value is zero.
pub fn compute_arms_armor_effect(record: &EquipmentRecord) -> EquipmentStatEffect {
    EquipmentStatEffect {
        armor_class_bonus: armor_class_bonus_from_bonus_chains(record),
        max_dex: token_i16(record, "MAXDEX"),
        spell_failure: token_value(record, "SPELLFAILURE").and_then(|value| value.parse().ok()),
        armor_check_penalty: token_i16(record, "ACCHECK"),
    }
}

fn token_value<'a>(record: &'a EquipmentRecord, key: &str) -> Option<&'a str> {
    record
        .tokens
        .iter()
        .find(|token| token.key == key)
        .map(|token| token.value.as_str())
}

fn token_i16(record: &EquipmentRecord, key: &str) -> Option<i16> {
    token_value(record, key).and_then(|value| value.parse().ok())
}

fn armor_class_bonus_from_bonus_chains(record: &EquipmentRecord) -> Option<i16> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        let is_default_armor_or_shield_ac_bonus = qualifiers.len() >= 3
            && qualifiers[0] == "COMBAT"
            && qualifiers[1] == "AC"
            && qualifiers
                .iter()
                .any(|qualifier| qualifier == "TYPE=Armor" || qualifier == "TYPE=Shield");
        if is_default_armor_or_shield_ac_bonus {
            qualifiers[2].parse::<i16>().ok()
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Leather Armor (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst`.
    #[test]
    fn leather_armor_base_yields_real_armor_stats() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\tBONUS:COMBAT|AC|-1|TYPE=Armor|PRETYPE:1,EQMOD=Special Quality ~ Broken ~ Armor\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(record);
        assert_eq!(effect.armor_class_bonus, Some(2));
        assert_eq!(effect.max_dex, Some(6));
        assert_eq!(effect.spell_failure, Some(10.0));
        assert_eq!(effect.armor_check_penalty, Some(0), "Leather Armor's real ACCHECK is 0");
    }

    /// Real verbatim tokens copied from `KEY:Buckler (Base)` (no
    /// `MAXDEX:` token on a buckler in the real corpus).
    #[test]
    fn buckler_base_has_shield_ac_bonus_but_no_max_dex_token() {
        let text = "Buckler\tKEY:Buckler (Base)\tTYPE:Shield.Buckler\tCOST:5\tWT:5\tACCHECK:-1\tSPELLFAILURE:5\tBONUS:COMBAT|AC|1|TYPE=Shield|PREVAREQ:DisableShieldBonus,0\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(record);
        assert_eq!(effect.armor_class_bonus, Some(1));
        assert_eq!(effect.max_dex, None);
        assert_eq!(effect.spell_failure, Some(5.0));
        assert_eq!(effect.armor_check_penalty, Some(-1), "Buckler's real ACCHECK is -1");
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` — a
    /// weapon carries none of the armor-defining tokens at all.
    #[test]
    fn longsword_base_has_no_armor_stats() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        let record = &result.entries[0];

        let effect = compute_arms_armor_effect(record);
        assert_eq!(effect.armor_class_bonus, None);
        assert_eq!(effect.max_dex, None);
        assert_eq!(effect.spell_failure, None);
        assert_eq!(effect.armor_check_penalty, None, "a weapon carries no ACCHECK token at all");
    }
}
