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
//! **Re-landed correctly (`SD31-W17-INTEGRATE-001` OPEN-ISSUES row 309,
//! SD-31 wave 18):** the Amulet of Mighty Fists family's own
//! `BONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|<n>|TYPE=Enhancement`
//! chain (`KEY:Special Ability ~ +1 ~ Amulet of Mighty Fists` through
//! `~ +5 ~`) was widened into the same match as a bare `WEAPON` chain in
//! wave 17 and reverted after review: `WEAPONPROF=TYPE.Natural` scopes the
//! bonus to NATURAL attacks only, but `WeaponEnhancementBonus` carried no
//! field able to represent that scope, and the consumer
//! (`damage_total::resolve_weapon_enhancement_modifier`) summed every
//! equipped item's `weapon_enhancement_bonus` into EVERY weapon a
//! character wields — so treating this chain the same as a bare `WEAPON`
//! chain gave an equipped Amulet of Mighty Fists +5 a wrongful +5
//! attack/+5 damage on an ordinary longsword, reachable in the shipped
//! desktop app (proven live: `apps/desktop/src-tauri/src/
//! character_hub.rs`'s `attach_equipment_modifier_at_root` gates only on
//! catalog recognition, target-equipped and funds, no legality check).
//! Wave 18 adds the piece both review findings named: this module now
//! sets `WeaponEnhancementBonus::natural_attack_only` (real, distinct
//! from a bare `WEAPON` chain, never guessed).
//!
//! **`SD31-W18-INTEGRATE-001` correction (integration-cycle adversarial
//! review, `OPEN-ISSUES.md` row 309 re-opened a second time):** the
//! wave-18 lane guarded only `damage_total::
//! resolve_weapon_enhancement_modifier` (the `weapon_enhancement_bonus`
//! top-level-selection consumer). It left `equipment_effects::
//! resolve_weapon_to_hit_bonus` — the function actually called for
//! `to_hit_bonus`/`attack_bonus_delta` via `selection.applied_modifiers`,
//! the SAME attachment shape `attach_equipment_modifier_at_root` uses —
//! unguarded, so an equipped Amulet of Mighty Fists still leaked its
//! bonus onto an ordinary longsword's attack roll even after that fix.
//! BOTH consumers now check `equipment_effects::is_natural_attack_weapon`
//! on the specific weapon being resolved before applying a
//! `natural_attack_only` bonus to it — an ordinary longsword receives
//! neither the Amulet's attack nor damage bonus; only a real
//! natural-attack weapon (e.g. CRB's `Unarmed Strike`) does.
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
    /// `true` when the source chain's qualifier[0] subject is
    /// `WEAPONPROF=TYPE.Natural` (the Amulet of Mighty Fists family) —
    /// real, verbatim from the token, not inferred. `damage_total::
    /// resolve_weapon_enhancement_modifier` (`SD31-W17-INTEGRATE-001`
    /// OPEN-ISSUES row 309) must only apply a `true` bonus to a weapon
    /// `equipment_effects::is_natural_attack_weapon` confirms is a real
    /// natural attack. `false` for a bare `WEAPON` chain, which applies to
    /// any weapon per PF1's ordinary enhancement rule.
    pub natural_attack_only: bool,
    /// SD-33 Epic 5 combat/weapon lane: the specific weapon-proficiency
    /// name this bonus is scoped to, when the source chain's subject is a
    /// bare `WEAPONPROF=<name>` other than `TYPE.Natural` — e.g.
    /// `Some("Longsword")` for `BONUS:WEAPONPROF=Longsword|TOHIT,DAMAGE|
    /// <n>` (`ultimate_equipment`'s "Cursed Sword" family), `Some("Hoof")`
    /// for the Horseshoes of a Zealous Warhorse family. Real, verbatim
    /// from the token — never inferred. `None` for a bare `WEAPON` chain
    /// (applies broadly) and for `WEAPONPROF=TYPE.Natural`
    /// (`natural_attack_only` already carries that distinct scope, kept
    /// as its own field for every existing consumer's back-compat).
    /// Confirmed against real PCGen source
    /// (`pcgen.io.exporttoken.WeaponToken.getMagicHitToken`/
    /// `getMagicDamageToken`): PCGen sums a `WEAPONPROF=<name>` bonus onto
    /// a specific equipped weapon only when that weapon's own resolved
    /// proficiency name matches `<name>` exactly.
    pub weapon_prof_scope: Option<String>,
}

/// Resolve one `equipmods` corpus record's weapon-enhancement-bonus
/// contribution.
///
/// Reads the record's first `BONUS:<WEAPON|WEAPONPROF=TYPE.Natural>|
/// <TOHIT|DAMAGE|DAMAGE,TOHIT|TOHIT,DAMAGE>|<n>|TYPE=Enhancement` chain, if
/// any. A record with no such chain (the majority of `equipmods` records)
/// yields `None`: that means this record's raw tokens do not carry the
/// field, not that its value is zero. `BONUS:WEAPON|WIELDCATEGORY|...`
/// chains and `TYPE=Enhancement`-less `BONUS:WEAPON|...` chains are
/// deliberately not matched (see module doc comment).
pub fn compute_equipmods_effect(record: &EquipmentRecord) -> Option<WeaponEnhancementBonus> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        let subject = qualifiers.first().map(String::as_str);
        let natural_attack_only = subject == Some("WEAPONPROF=TYPE.Natural");
        let is_roll_shape = qualifiers.len() >= 2
            && matches!(qualifiers[1].as_str(), "TOHIT" | "DAMAGE" | "DAMAGE,TOHIT" | "TOHIT,DAMAGE");

        if (subject == Some("WEAPON") || natural_attack_only) && is_roll_shape {
            // Unchanged from before this cycle: a bare `WEAPON` chain or
            // `WEAPONPROF=TYPE.Natural` chain still requires the trailing
            // `TYPE=Enhancement` qualifier (see module doc comment for
            // why: it excludes `WIELDCATEGORY` and untyped Wield-Size
            // to-hit-offset chains, which are real but are not a magic
            // enhancement bonus).
            if qualifiers.len() >= 4 && qualifiers[3] == "TYPE=Enhancement" {
                return qualifiers[2].parse::<i16>().ok().map(|bonus_value| WeaponEnhancementBonus {
                    affects: qualifiers[1].clone(),
                    bonus: bonus_value,
                    natural_attack_only,
                    weapon_prof_scope: None,
                });
            }
            return None;
        }

        // SD-33 Epic 5 combat/weapon lane: a bare `WEAPONPROF=<name>|
        // <TOHIT|DAMAGE|...>|<n>` chain scoped to one SPECIFIC named
        // proficiency (e.g. Longsword, Hoof, Bite) -- distinct from both
        // the broadly-applying bare `WEAPON` chain and the natural-
        // attack-only `TYPE.Natural` chain. Every real corpus record of
        // this shape (`ultimate_equipment`'s "Cursed <Weapon>" and
        // "Horseshoes of a Zealous Warhorse" families) carries this
        // WITHOUT a trailing `TYPE=Enhancement` qualifier at all -- real
        // PCGen source (`pcgen.io.exporttoken.WeaponToken.
        // getMagicHitToken`/`getMagicDamageToken`) sums a
        // `WEAPONPROF=<name>` bonus unconditionally, with no `TYPE=`
        // filter either, so no such gate applies here.
        if let Some(name) = subject.and_then(|s| s.strip_prefix("WEAPONPROF=")) {
            // Excludes every `TYPE.`-prefixed subject, not just the
            // literal `TYPE.Natural` string: a `WEAPONPROF=TYPE.<x>`
            // chain names a whole weapon-TYPE category (a hypothetical
            // shape this module's own pre-existing negative-control test,
            // `a_different_weaponprof_subject_has_no_weapon_enhancement_
            // bonus`, deliberately keeps unrecognized), never a single
            // literal proficiency name PCGen's own `getProfName(eq)`
            // would compare a specific weapon's proficiency against.
            if !name.starts_with("TYPE.") && is_roll_shape && qualifiers.len() >= 3 {
                return qualifiers[2].parse::<i16>().ok().map(|bonus_value| WeaponEnhancementBonus {
                    affects: qualifiers[1].clone(),
                    bonus: bonus_value,
                    natural_attack_only: false,
                    weapon_prof_scope: Some(name.to_string()),
                });
            }
        }
        None
    })
}

/// Resolve one `equipmods` corpus record's flat Spell Resistance
/// contribution.
///
/// Reads the record's own `SR:<n>` token, when present and a literal
/// integer -- the armor-slot "Spell Resistance" special ability family
/// (`KEY:Special Ability ~ Spell Resistance / 13 ~ Armor` through `/ 19 ~
/// Armor`, `core_rulebook/cr_equipmods.lst:343-346`). Decision 7 REFINED
/// (`SD31-D7-PROSE-004`) names this exact shape as the paradigm UNIVERSAL
/// case: it applies unconditionally whenever the wearer's Spell
/// Resistance is checked, so text alone ("grants spell resistance 13")
/// does not satisfy the done-bar -- it must be COMPUTED.
///
/// Deliberately does NOT match `BNS_SPL_RST` ("Bonus Spell Resistance",
/// `KEY:Special Ability ~ Bonus Spell Resistance`), whose own `SR:%CHOICE`
/// token carries a PCGen chooser placeholder rather than a literal
/// integer -- `str::parse` fails on `"%CHOICE"` and correctly yields
/// `None`, the same "no fabricated number" discipline every other
/// resolver in this module follows. That record is a genuine player
/// CHOICE (`CHOOSE:NUMBER|MIN=13|MAX=32`), not a flat grant, and stays out
/// of this function's scope until a chosen-value resolution mechanism
/// exists.
pub fn resolve_spell_resistance_bonus(record: &EquipmentRecord) -> Option<i16> {
    record.tokens.iter().find(|token| token.key == "SR").and_then(|token| token.value.parse().ok())
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
                natural_attack_only: false,
                weapon_prof_scope: None,
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
                natural_attack_only: false,
                weapon_prof_scope: None,
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
                natural_attack_only: false,
                weapon_prof_scope: None,
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

    /// SD-33 Epic 5 combat/weapon lane: real verbatim tokens copied from
    /// `data/corpus/ultimate_equipment/equipment/cursed_sword_2.json`
    /// (`ue_equip_magic_items.lst`) — a bare `WEAPONPROF=Longsword|
    /// TOHIT,DAMAGE|<n>` chain, no trailing `TYPE=Enhancement` qualifier
    /// at all (confirmed against the real record: arity 3, not 4). Before
    /// this cycle `compute_equipmods_effect` only recognized `WEAPON` and
    /// `WEAPONPROF=TYPE.Natural` subjects, so this real, comparable,
    /// negative (`-2`, a cursed item) magnitude silently resolved to
    /// `None`.
    #[test]
    fn named_weaponprof_scope_yields_a_real_bonus_with_no_type_enhancement_gate() {
        let text = "Cursed Sword\tKEY:Cursed Sword\tTYPE:Weapon.Melee.Martial\tCOST:1\tWT:4\tCRITMULT:x2\tDAMAGE:1d8\tBONUS:WEAPONPROF=Longsword|TOHIT,DAMAGE|-2\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                affects: "TOHIT,DAMAGE".to_string(),
                bonus: -2,
                natural_attack_only: false,
                weapon_prof_scope: Some("Longsword".to_string()),
            })
        );
    }

    /// The same named-`WEAPONPROF=` shape with a single affected roll
    /// (`TOHIT` alone, not the combined `TOHIT,DAMAGE` pair) — real
    /// verbatim tokens copied from
    /// `data/corpus/ultimate_equipment/equipment/belt_of_teeth.json`.
    #[test]
    fn named_weaponprof_scope_single_roll_yields_a_real_bonus() {
        let text = "Belt of Teeth\tKEY:Belt of Teeth\tTYPE:Magic.Belt\tCOST:1\tWT:1\tBONUS:WEAPONPROF=Bite|TOHIT|4\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                affects: "TOHIT".to_string(),
                bonus: 4,
                natural_attack_only: false,
                weapon_prof_scope: Some("Bite".to_string()),
            })
        );
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

    /// `SD31-W17-INTEGRATE-001` (OPEN-ISSUES row 309), re-landed wave 18:
    /// the Amulet of Mighty Fists family's own `WEAPONPROF=TYPE.Natural`
    /// chain now resolves to a real bonus, correctly tagged
    /// `natural_attack_only: true` — real verbatim tokens copied from
    /// `KEY:Special Ability ~ +1 ~ Amulet of Mighty Fists`. Wave 17
    /// recognized this chain without the tag and without a scope-aware
    /// consumer, which wrongly bonused every equipped weapon; the scope
    /// itself (this test) is now real, and
    /// `damage_total::resolve_weapon_enhancement_modifier`'s own tests
    /// prove the consumer honours it.
    #[test]
    fn amulet_of_mighty_fists_weaponprof_chain_yields_a_natural_attack_only_bonus() {
        let text = "+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                affects: "TOHIT,DAMAGE".to_string(),
                bonus: 1,
                natural_attack_only: true,
                weapon_prof_scope: None,
            })
        );
    }

    /// A `WEAPONPROF=` subject other than `TYPE.Natural` (e.g. a
    /// hypothetical class-specific proficiency scope) must not be treated
    /// as the natural-attack family — only the exact literal
    /// `WEAPONPROF=TYPE.Natural` string this family's real corpus records
    /// carry is recognized, never a substring or prefix match.
    #[test]
    fn a_different_weaponprof_subject_has_no_weapon_enhancement_bonus() {
        let text = "Proxy\tKEY:Special Quality ~ WeaponProf Proxy\tTYPE:Weapon\tBONUS:WEAPONPROF=TYPE.Bow|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
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

    /// `SD31-W21-EQUIPMOD-001`: real verbatim tokens copied from
    /// `KEY:Special Ability ~ Spell Resistance / 13 ~ Armor`
    /// (`core_rulebook/cr_equipmods.lst:343`) -- a flat, unconditional
    /// `SR:13` token, the paradigm UNIVERSAL magnitude Decision 7 REFINED
    /// names (`SD31-D7-PROSE-004`: "a modifier to a value the character
    /// sheet computes, that applies UNCONDITIONALLY... Must be COMPUTED").
    #[test]
    fn spell_resistance_13_armor_yields_a_real_spell_resistance_bonus() {
        let text = "Spell Resistance 13\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 13 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:2\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:13\tSPROP:grants spell resistance 13\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(record), Some(13));
    }

    /// The same family's `/ 19 ~ Armor` tier, proving the value is read
    /// from the token rather than hardcoded to `13`. Real verbatim tokens
    /// copied from `cr_equipmods.lst:346`.
    #[test]
    fn spell_resistance_19_armor_yields_a_real_spell_resistance_bonus() {
        let text = "Spell Resistance 19\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 19 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:8\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:19\tSPROP:grants spell resistance 19\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(record), Some(19));
    }

    /// `KEY:Special Ability ~ Bonus Spell Resistance` (`BNS_SPL_RST`,
    /// `cr_equipmods.lst:617`) carries `SR:%CHOICE`, not a literal
    /// integer -- a real player CHOICE (`CHOOSE:NUMBER|MIN=13|MAX=32`),
    /// not a flat grant. `str::parse` fails on `"%CHOICE"` and this
    /// resolver must yield `None`, never a fabricated number, the same
    /// "no invented value" discipline every other resolver in this module
    /// follows for a chain it does not recognize.
    #[test]
    fn bonus_spell_resistance_choice_token_has_no_flat_spell_resistance_bonus() {
        let text = "BNS_SPL_RST\tVISIBLE:NO\tKEY:Special Ability ~ Bonus Spell Resistance\tTYPE:Weapon.Belt.Body\tCOST:10000*(%CHOICE-12)\tSR:%CHOICE\tSPROP:base spell resistance of %CHOICE\tCHOOSE:NUMBER|MIN=13|MAX=32|NOSIGN|TITLE=Spell Resistance\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(record), None);
    }

    /// A record with no `SR:` token anywhere (the canonical `+1`
    /// weapon-enhancement record already used above) yields `None`, not a
    /// fabricated zero.
    #[test]
    fn weapon_enhancement_record_has_no_spell_resistance_bonus() {
        let text = "+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(record), None);
    }
}
