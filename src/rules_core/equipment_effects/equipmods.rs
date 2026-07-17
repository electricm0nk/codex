//! Epic 5, fourth and final equipment category (SD-20 §1.5 work-unit
//! order): CRB `equipmods` per-item effect resolution. Landing this
//! closes Epic 5 — all four CRB equipment categories (`arms_armor`,
//! `general`, `magic_items`, `equipmods`) are done.
//!
//! Unlike `arms_armor` (AC/max-dex/spell-failure), `general`
//! (`BONUS:SKILL|...`), and `magic_items` (`BONUS:STAT|...`), the CRB
//! `equipmods` block (`core_rulebook/cr_equipmods.lst`) is a mixed bag of
//! materials, masterwork qualities, and weapon special abilities. Its
//! single most common per-item mechanical bonus that is genuinely
//! player-facing (not an internal cost/formula token like `BONUS:VAR` or
//! `BONUS:ITEMCOST`, which dominate this file's `BONUS:` token count) is
//! a weapon to-hit/damage enhancement bonus carried by
//! `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT>|<n>|TYPE=Enhancement` —
//! confirmed directly against the real corpus on the canonical "+1
//! (Enhancement to Weapon)" through "+5 (Enhancement to Weapon)" records
//! (`KEY:Special Ability ~ +1 ~ Weapon` ... `~ +5 ~ Weapon`, each
//! carrying `BONUS:WEAPON|DAMAGE,TOHIT|<n>|TYPE=Enhancement`), and on the
//! `Masterwork`/`Adamantine`/`Mithral` weapon-material records (each
//! carrying `BONUS:WEAPON|TOHIT|1|TYPE=Enhancement`).
//!
//! Deliberately requires the trailing `TYPE=Enhancement` qualifier and an
//! affected-roll of `TOHIT`, `DAMAGE`, or `DAMAGE,TOHIT` — this excludes
//! the `BONUS:WEAPON|WIELDCATEGORY|...` chains (Wield Size records, which
//! shift a weapon's effective wield category, not its attack/damage
//! rolls) and the bare `BONUS:WEAPON|TOHIT|<n>` chain some Wield-Size
//! "No Penalty" records carry with no `TYPE=` qualifier at all (a
//! size-handling to-hit offset, not a magic enhancement bonus). Folding
//! either into the same field would misrepresent a wielding-mechanic
//! delta as an enhancement bonus. Many other `equipmods` records (charge
//! trackers, spell-effect triggers, artisan's tools with only a skill
//! bonus, plain materials like Cloth, ...) carry no matching chain at
//! all, so `None` for those is an honest absence, not a fabricated zero.
//! No field here is hand-rolled; every value traces back to a real,
//! verbatim corpus token, read the same way `arms_armor.rs`,
//! `general.rs`, and `magic_items.rs` read their own tokens straight off
//! the resolved record.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;

/// A weapon to-hit/damage enhancement bonus granted by an
/// `equipmods`-category item's
/// `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT>|<n>|TYPE=Enhancement`
/// corpus token.
#[derive(Debug, Clone, PartialEq)]
pub struct WeaponEnhancementBonus {
    /// The affected roll(s), verbatim from the corpus token's second
    /// pipe-delimited segment — `"TOHIT"`, `"DAMAGE"`, or
    /// `"DAMAGE,TOHIT"`.
    pub affects: String,
    pub bonus: i16,
}

/// Resolve one `equipmods` corpus record's weapon-enhancement-bonus
/// contribution.
///
/// Reads the record's first `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT>|<n>|
/// TYPE=Enhancement` chain, if any. A record with no such chain (the
/// majority of `equipmods` records) yields `None`: that means this
/// record's raw tokens do not carry the field, not that its value is
/// zero. `BONUS:WEAPON|WIELDCATEGORY|...` chains and `TYPE=Enhancement`-
/// less `BONUS:WEAPON|...` chains are deliberately not matched (see
/// module doc comment).
pub fn compute_equipmods_effect(record: &EquipmentRecord) -> Option<WeaponEnhancementBonus> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        let is_weapon_enhancement_bonus = qualifiers.len() >= 4
            && qualifiers[0] == "WEAPON"
            && matches!(qualifiers[1].as_str(), "TOHIT" | "DAMAGE" | "DAMAGE,TOHIT")
            && qualifiers[3] == "TYPE=Enhancement";
        if !is_weapon_enhancement_bonus {
            return None;
        }
        qualifiers[2].parse::<i16>().ok().map(|bonus_value| WeaponEnhancementBonus {
            affects: qualifiers[1].clone(),
            bonus: bonus_value,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Special Ability ~ +1 ~
    /// Weapon` in `core_rulebook/cr_equipmods.lst`.
    #[test]
    fn plus_one_weapon_enhancement_yields_a_real_damage_tohit_bonus() {
        let text = "+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                affects: "DAMAGE,TOHIT".to_string(),
                bonus: 1,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Material ~ Adamantine ~
    /// Weapon` — a different affected-roll shape (`TOHIT` alone), proving
    /// the affected-roll set is read from the token, not hardcoded.
    #[test]
    fn adamantine_weapon_yields_a_real_tohit_only_bonus() {
        let text = "Adamantine\tKEY:Material ~ Adamantine ~ Weapon\tTYPE:BaseMaterial.MasterworkQuality.Weapon\tCOST:3000\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                affects: "TOHIT".to_string(),
                bonus: 1,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Special Quality ~ Wield Size
    /// / 1 Step Greater` — carries a `BONUS:WEAPON|WIELDCATEGORY|...`
    /// chain, which is deliberately not a weapon-enhancement bonus (see
    /// module doc comment).
    #[test]
    fn wield_size_shift_has_no_weapon_enhancement_bonus() {
        let text = "Wield One Step Greater\tKEY:Special Quality ~ Wield Size / 1 Step Greater\tTYPE:Weapon.Melee\tBONUS:WEAPON|WIELDCATEGORY|-1\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }

    /// Real verbatim tokens copied from `KEY:Special Quality ~ Wield Size
    /// / 1 Step Greater / No Penalty` — carries both a `WIELDCATEGORY`
    /// chain and a bare `BONUS:WEAPON|TOHIT|2` chain with no `TYPE=`
    /// qualifier at all (a size-handling to-hit offset, not a magic
    /// enhancement bonus); neither chain matches.
    #[test]
    fn wield_size_no_penalty_has_no_weapon_enhancement_bonus() {
        let text = "Wield One Step Greater No Penalty\tKEY:Special Quality ~ Wield Size / 1 Step Greater / No Penalty\tTYPE:Weapon.Melee\tBONUS:WEAPON|WIELDCATEGORY|-1\tBONUS:WEAPON|TOHIT|2\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }

    /// Real verbatim tokens copied from `KEY:Material ~ Cloth` — a plain
    /// material carries no `BONUS:` token at all.
    #[test]
    fn cloth_material_has_no_weapon_enhancement_bonus() {
        let text = "Cloth\tKEY:Material ~ Cloth\tTYPE:BaseMaterial.Mundane.Ammunition.Armor.Shield.Weapon.Instruments.Tools.Goods\tCOST:0\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }
}
