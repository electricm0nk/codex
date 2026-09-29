// -- split from `tests` in src/rules_core/equipment_effects/equipmods.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::equipment_effects::equipmods::*;
    use codex_ingest::pcgen_import::ir_converter::equipment_record_to_corpus;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::equipment_record::CorpusEquipmentRecord;

    /// Real verbatim tokens copied from `KEY:Special Ability ~ +1 ~
    /// Weapon` in `core_rulebook/cr_equipmods.lst`.
    #[test]
    fn plus_one_weapon_enhancement_yields_a_real_damage_tohit_bonus() {
        let text = "+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(&converted(record));
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(1),
                damage_bonus: Some(1),
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

        let effect = compute_equipmods_effect(&converted(record));
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(1),
                damage_bonus: None,
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

        let effect = compute_equipmods_effect(&converted(record));
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(3),
                damage_bonus: Some(3),
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

        let effect = compute_equipmods_effect(&converted(record));
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

        let effect = compute_equipmods_effect(&converted(record));
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

        let effect = compute_equipmods_effect(&converted(record));
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(-2),
                damage_bonus: Some(-2),
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

        let effect = compute_equipmods_effect(&converted(record));
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(4),
                damage_bonus: None,
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

        let effect = compute_equipmods_effect(&converted(record));
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

        let effect = compute_equipmods_effect(&converted(record));
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(1),
                damage_bonus: Some(1),
                natural_attack_only: true,
                weapon_prof_scope: None,
            })
        );
    }

    /// SD-33 remediation wave 5 (`AT-33-E5-002`/`003`, weapon-token-family
    /// lane): real verbatim tokens copied from
    /// `data/corpus/ultimate_equipment/equipment/heavy_hammer.json`'s
    /// declared bonus chains
    /// (`pcgen_import::ingest_record::bonus_chain_qualifiers`) — a TOHIT-only
    /// `WEAPONPROF=Warhammer|TOHIT|-2`
    /// chain and a SEPARATE DAMAGE-only `WEAPONPROF=Warhammer|DAMAGE|4`
    /// chain on the SAME record (plus an unrelated `MOVEADD` chain, which
    /// this test also carries to prove it's correctly skipped). Before
    /// this cycle `compute_equipmods_effect` used `find_map` and stopped
    /// at the first qualifying chain, so only `-2`/`TOHIT` was ever seen —
    /// the real, player-facing `+4` damage bonus never reached
    /// `WeaponEnhancementBonus` at all. Confirmed against the pinned
    /// oracle (direct-java runner, Heavy Hammer worn as its own weapon,
    /// `PROFICIENCY WEAPON|Warhammer`): `WEAPON.n.MAGICHIT=-2`,
    /// `WEAPON.n.MAGICDAMAGE=+4` — both magnitudes real, and different.
    #[test]
    fn record_with_two_separately_scoped_chains_sums_both_rolls_independently() {
        let text = "Heavy Hammer\tKEY:Heavy Hammer\tTYPE:Magic.Cursed.Weapon\tPROFICIENCY:WEAPON|Warhammer\tCOST:0\tWT:20\tBONUS:MOVEADD|TYPE.All|-10\tBONUS:WEAPONPROF=Warhammer|TOHIT|-2\tBONUS:WEAPONPROF=Warhammer|DAMAGE|4\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(&converted(record));
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(-2),
                damage_bonus: Some(4),
                natural_attack_only: false,
                weapon_prof_scope: Some("Warhammer".to_string()),
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

        let effect = compute_equipmods_effect(&converted(record));
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

        let effect = compute_equipmods_effect(&converted(record));
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

        assert_eq!(resolve_spell_resistance_bonus(&converted(record)), Some(13));
    }

    /// The same family's `/ 19 ~ Armor` tier, proving the value is read
    /// from the token rather than hardcoded to `13`. Real verbatim tokens
    /// copied from `cr_equipmods.lst:346`.
    #[test]
    fn spell_resistance_19_armor_yields_a_real_spell_resistance_bonus() {
        let text = "Spell Resistance 19\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 19 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:8\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:19\tSPROP:grants spell resistance 19\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(&converted(record)), Some(19));
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

        assert_eq!(resolve_spell_resistance_bonus(&converted(record)), None);
    }

    /// A record with no `SR:` token anywhere (the canonical `+1`
    /// weapon-enhancement record already used above) yields `None`, not a
    /// fabricated zero.
    #[test]
    fn weapon_enhancement_record_has_no_spell_resistance_bonus() {
        let text = "+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(&converted(record)), None);
    }

    /// SD-33 remediation wave 6 (`AT-33-E5-last39-skill-combat`): real
    /// verbatim tokens copied from `ultimate_psionics/up_equipmods.lst:141`
    /// (`KEY:Special Quality ~ Dissonance / Enhancement Bonus / Main`,
    /// `data/corpus/ultimate_psionics/equipment/equipmods/
    /// special_quality_dissonance_enhancement_bonus_main.json`). Two real
    /// gaps this record exposes together, both closed by this cycle:
    /// (1) the chain's affected-roll magnitude is the NAME of a variable
    /// (`DissonanceEnhancementBonusMain`), not a literal integer --
    /// `qualifiers[2].parse::<i16>()` fails closed on a bare name. The
    /// record's OWN sibling `BONUS:VAR|DissonanceEnhancementBonusMain|1`
    /// chain states that variable's own contribution as a real literal
    /// `1`, and the record's `DEFINE:DissonanceEnhancementBonusMain|0`
    /// token (base 0, no other source in an isolated single-item read)
    /// means the variable's total, knowable value is `0 + 1 = 1` -- not a
    /// guess, cross-referenced against the record's own chain.
    /// (2) the chain's `TYPE=` qualifier is `TYPE=ENHANCEMENT`
    /// (uppercase), which `qualifiers[3] == "TYPE=Enhancement"`'s exact
    /// string match never matches -- named but not fixed by
    /// `AT-33-E5-last75_cycle_receipt.md` Finding 4 and reconfirmed still
    /// open by `AT-33-E5-last67-skill-combat_cycle_receipt.md`. Before this
    /// cycle: `compute_equipmods_effect` returned `None` (matches
    /// neither gate). After: real `tohit_bonus`/`damage_bonus` of `Some(1)`
    /// each, traced to the record's own two chains, no formula evaluator
    /// and no fabricated number.
    #[test]
    fn dissonance_enhancement_bonus_var_referenced_chain_resolves_via_its_own_sibling_var_chain() {
        let text = "Enhancement Bonus for Dissonance Main\tKEY:Special Quality ~ Dissonance / Enhancement Bonus / Main\tTYPE:Weapon\tVISIBLE:NO\tBONUS:VAR|DissonanceEnhancementBonusMain|1\tBONUS:WEAPON|DAMAGE,TOHIT|DissonanceEnhancementBonusMain|TYPE=ENHANCEMENT\tDEFINE:DissonanceEnhancementBonusMain|0\n";
        let result = parse_equipment_entries("up_equipmods.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(&converted(record));
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(1),
                damage_bonus: Some(1),
                natural_attack_only: false,
                weapon_prof_scope: None,
            })
        );
    }

    /// Negative control: a bare `WEAPON` chain whose magnitude names a
    /// variable with NO sibling `VAR` chain on the same record must still
    /// yield `None` -- the substitution is scoped to the record's own
    /// verified `VAR` chain, never a blind lookup that could fabricate a
    /// value for an unrelated or undefined variable name.
    #[test]
    fn weapon_chain_referencing_an_undefined_variable_name_has_no_weapon_enhancement_bonus() {
        let text = "Fabricated\tKEY:Fabricated Test Record\tTYPE:Weapon\tBONUS:WEAPON|TOHIT|SomeUndefinedVariable|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(compute_equipmods_effect(&converted(record)), None);
    }

    /// The ingest-time conversion every live reader below is proved over.
    fn converted(record: &EquipmentRecord) -> CorpusEquipmentRecord {
        equipment_record_to_corpus(record)
    }


}
