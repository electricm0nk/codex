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
//! `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT|TOHIT,DAMAGE>|<n>|
//! TYPE=Enhancement` — confirmed directly against the real corpus on the
//! canonical "+1 (Enhancement to Weapon)" through "+5 (Enhancement to
//! Weapon)" records (`KEY:Special Ability ~ +1 ~ Weapon` ... `~ +5 ~
//! Weapon`, each carrying `BONUS:WEAPON|DAMAGE,TOHIT|<n>|
//! TYPE=Enhancement`), on the `Masterwork`/`Adamantine`/`Mithral`
//! weapon-material records (each carrying `BONUS:WEAPON|TOHIT|1|
//! TYPE=Enhancement`), and on `Maul of the Titans`/`Mattock of the
//! Titans` (`BONUS:WEAPON|TOHIT,DAMAGE|3|TYPE=Enhancement` — the reverse
//! pipe order of the canonical records, proving the affected-roll set
//! must accept both orders, not just `DAMAGE,TOHIT`).
//!
//! **NOT recognized, deliberately (`SD31-W17-INTEGRATE-001`, wave-17
//! adversarial review):** the Amulet of Mighty Fists family's own
//! `BONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|<n>|TYPE=Enhancement`
//! chain (`KEY:Special Ability ~ +1 ~ Amulet of Mighty Fists` through
//! `~ +5 ~`) was widened into this same match in wave 17 and then
//! reverted here after review: `WEAPONPROF=TYPE.Natural` scopes the
//! bonus to NATURAL attacks only, but `WeaponEnhancementBonus` carries no
//! field able to represent that scope, and the live consumer
//! (`damage_total::resolve_weapon_enhancement_modifier`) sums every
//! equipped item's `weapon_enhancement_bonus` into EVERY weapon a
//! character wields — so treating this chain the same as a bare `WEAPON`
//! chain gave an equipped Amulet of Mighty Fists +5 a wrongful +5
//! attack/+5 damage on an ordinary longsword, reachable in the shipped
//! desktop app (proven live: `apps/desktop/src-tauri/src/
//! character_hub.rs`'s `attach_equipment_modifier_at_root` gates only on
//! catalog recognition, target-equipped and funds, no legality check).
//! Recognizing this family correctly needs a NEW field (a natural-attack
//! scope flag) plus a NEW consumer that applies it only to natural
//! weapons — real, new engine capability, not a probe widen. Logged as
//! an engineering follow-up rather than re-attempted here; the Amulet of
//! Mighty Fists family's `equipment_modifier` units stay `held`.
//!
//! Deliberately requires the trailing `TYPE=Enhancement` qualifier — this
//! excludes the `BONUS:WEAPON|WIELDCATEGORY|...` chains (Wield Size
//! records, which shift a weapon's effective wield category, not its
//! attack/damage rolls) and the bare `BONUS:WEAPON|TOHIT|<n>` chain some
//! Wield-Size "No Penalty" records carry with no `TYPE=` qualifier at all
//! (a size-handling to-hit offset, not a magic enhancement bonus).
//! Folding either into the same field would misrepresent a
//! wielding-mechanic delta as an enhancement bonus. The affected-roll
//! requirement (`TOHIT`/`DAMAGE`/`DAMAGE,TOHIT`/`TOHIT,DAMAGE`) is a
//! second, independent guard — every real corpus chain reaching this
//! function today already carries `TYPE=Enhancement` AND one of these
//! four roll shapes together, so the two checks are redundant on the
//! CURRENT corpus, but each is real and independently testable (see
//! `weapon_chain_with_unrecognized_affected_roll_has_no_weapon_
//! enhancement_bonus` below, which defeats the `TYPE=Enhancement` guard
//! on purpose to exercise the roll check on its own). Many other
//! `equipmods` records (charge trackers, spell-effect triggers,
//! artisan's tools with only a skill bonus, plain materials like Cloth,
//! ...) carry no matching chain at all, so `None` for those is an honest
//! absence, not a fabricated zero. No field here is hand-rolled; every
//! value traces back to a real, verbatim corpus token, read the same way
//! `arms_armor.rs`, `general.rs`, and `magic_items.rs` read their own
//! tokens straight off the resolved record.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;

/// A weapon to-hit/damage enhancement bonus granted by an
/// `equipmods`-category item's
/// `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT|TOHIT,DAMAGE>|<n>|
/// TYPE=Enhancement` corpus token.
#[derive(Debug, Clone, PartialEq)]
pub struct WeaponEnhancementBonus {
    /// The affected roll(s), verbatim from the corpus token's second
    /// pipe-delimited segment — `"TOHIT"`, `"DAMAGE"`, `"DAMAGE,TOHIT"`,
    /// or `"TOHIT,DAMAGE"`.
    pub affects: String,
    pub bonus: i16,
}

/// Resolve one `equipmods` corpus record's weapon-enhancement-bonus
/// contribution.
///
/// Reads the record's first `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT|
/// TOHIT,DAMAGE>|<n>|TYPE=Enhancement` chain, if any. A record with no
/// such chain (the majority of `equipmods` records) yields `None`: that
/// means this record's raw tokens do not carry the field, not that its
/// value is zero. `BONUS:WEAPON|WIELDCATEGORY|...` chains,
/// `TYPE=Enhancement`-less `BONUS:WEAPON|...` chains, and
/// `BONUS:WEAPONPROF=...` chains (natural-attack-scoped; see module doc
/// comment) are deliberately not matched.
pub fn compute_equipmods_effect(record: &EquipmentRecord) -> Option<WeaponEnhancementBonus> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        let is_weapon_enhancement_bonus = qualifiers.len() >= 4
            && qualifiers[0] == "WEAPON"
            && matches!(
                qualifiers[1].as_str(),
                "TOHIT" | "DAMAGE" | "DAMAGE,TOHIT" | "TOHIT,DAMAGE"
            )
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

    /// Real verbatim tokens copied from `KEY:Special Ability ~ +3 ~
    /// Weapon` on `Maul of the Titans`/`Mattock of the Titans`
    /// (`core_rulebook/cr_equip_arms_armor.lst`) — the SAME mechanic as
    /// the canonical `+1..+5` records above, but with the affected-roll
    /// segment in the opposite pipe order (`TOHIT,DAMAGE`, not
    /// `DAMAGE,TOHIT`), proving both orders are read, not just one.
    #[test]
    fn reversed_roll_order_weapon_enhancement_yields_a_real_bonus() {
        let text = "+3 (Enhancement to Weapon)\tKEY:Special Ability ~ +3 ~ Weapon Reversed\tTYPE:Weapon\tPLUS:3\tBONUS:WEAPON|TOHIT,DAMAGE|3|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                affects: "TOHIT,DAMAGE".to_string(),
                bonus: 3,
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

    /// `SD31-W17-INTEGRATE-001`: the Amulet of Mighty Fists family's own
    /// `WEAPONPROF=TYPE.Natural` chain must NEVER match — recognizing it
    /// the same as a bare `WEAPON` chain was tried in wave 17 and
    /// reverted after review found it applies the bonus to every equipped
    /// weapon, not just natural attacks (see module doc comment). This is
    /// the negative control that guards against that regression
    /// recurring: real verbatim tokens copied from `KEY:Special Ability ~
    /// +1 ~ Amulet of Mighty Fists`.
    #[test]
    fn amulet_of_mighty_fists_weaponprof_chain_has_no_weapon_enhancement_bonus() {
        let text = "+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }

    /// Defeats the `TYPE=Enhancement` guard on purpose (an otherwise
    /// well-formed bare-`WEAPON` chain with 4 qualifiers) so the
    /// affected-roll check is exercised on its own rather than always
    /// being shadowed by arity or the qualifier[0]/TYPE=Enhancement
    /// checks — `SD31-W17-INTEGRATE-001` (review) found the PRIOR
    /// negative-control test for this shape could never fail because its
    /// fixture was excluded earlier by arity alone.
    #[test]
    fn weapon_chain_with_unrecognized_affected_roll_has_no_weapon_enhancement_bonus() {
        let text = "Weapon Focus Proxy\tKEY:Special Quality ~ Weapon Focus Proxy\tTYPE:Weapon\tBONUS:WEAPON|CRITMULT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }
}
