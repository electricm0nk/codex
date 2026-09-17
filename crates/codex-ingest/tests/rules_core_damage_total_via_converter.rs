// -- split from `tests` in src/rules_core/damage_total.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::damage_total::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
    use codex::rules_core::equipment_effects::compute_equipment_effects;
    use codex::rules_core::source_content::SourceRef;

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst`.
    #[test]
    fn longsword_base_yields_its_real_damage_dice() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        let resolved = resolve_base_damage_dice("Longsword (Base)", &corpus)
            .expect("Longsword (Base) must resolve");
        assert_eq!(
            resolved.base_dice,
            DiceExpression {
                count: 1,
                die_size: 8
            }
        );
        assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
    }

    /// Real verbatim tokens copied from `KEY:Leather Armor (Base)` — no
    /// `DAMAGE:` token on armor.
    #[test]
    fn armor_record_has_no_base_dice() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
        let corpus = corpus_from(text);

        assert!(resolve_base_damage_dice("Leather Armor (Base)", &corpus).is_none());
    }

    /// `AT-34-E3-003` (bucket `M`, equipment sub-cause
    /// `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`):
    /// a real CRB record, `Crossbow (Light)`, carries no `DAMAGE:` token on
    /// its own row at all — its only mechanically relevant token is
    /// `BASEITEM:Light Crossbow (Base)`, PCGen's own "inherit this record's
    /// stats" convention. `equipment_id_resolve` (the same resolver every
    /// prior work-unit in this file already uses) already parses `BASEITEM`
    /// into a real token; nothing previously chased it to a second record.
    /// The real base record, `Light Crossbow (Base)`
    /// (`core_rulebook/cr_equip_arms_armor.lst`), carries `DAMAGE:1d8` —
    /// confirmed live, with the ingest array's field name taken from its one
    /// definition on the converter side rather than spelled out here:
    /// `` TOK=$(grep -oP '(?<=INGEST_TOKENS_FIELD: &str = ")[^"]+' src/pcgen_import/ingest_payload.rs); jq --arg t "$TOK" '.data[$t][] | select(.key=="DAMAGE")' data/corpus/core_rulebook/equipment/arms_armor/light_crossbow_base.json `` →
    /// `{"key": "DAMAGE", "value": "1d8"}`. Before this fix,
    /// `resolve_weapon_damage_breakdown` (this function's real consumer,
    /// feeding the desktop app's `WeaponDamageBreakdown`) returned `None`
    /// for a player who selected `Crossbow (Light)` on their sheet — a real
    /// player-facing gap, not only a bucket-M classifier miss.
    #[test]
    fn baseitem_alias_chases_to_its_base_records_damage_dice() {
        let text = "Light Crossbow\tKEY:Light Crossbow (Base)\tTYPE:Weapon.Ranged.Martial\tCOST:35\tWT:4\tCRITMULT:x2\tCRITRANGE:19\tDAMAGE:1d8\n\
                    Crossbow, Light\tKEY:Crossbow (Light)\tBASEITEM:Light Crossbow (Base)\tEQMOD:Material ~ Wood\n";
        let corpus = corpus_from(text);

        let resolved = resolve_base_damage_dice("Crossbow (Light)", &corpus)
            .expect("Crossbow (Light) must chase BASEITEM to its base record's DAMAGE token");
        assert_eq!(
            resolved.base_dice,
            DiceExpression {
                count: 1,
                die_size: 8
            }
        );
        // The record's own identity is preserved even though the dice came
        // from its BASEITEM -- this is still the alias the player selected.
        assert_eq!(resolved.weapon_record_key, "Crossbow (Light)");
    }

    /// Negative control: a `BASEITEM:` naming a record that does not
    /// resolve against the corpus at all must stay `None`, never panic or
    /// silently fabricate a value.
    #[test]
    fn baseitem_naming_an_unresolvable_record_stays_none() {
        let text =
            "Ghost Item\tKEY:Ghost Item\tBASEITEM:Nonexistent Base Record\tTYPE:Weapon.Melee\n";
        let corpus = corpus_from(text);

        assert!(resolve_base_damage_dice("Ghost Item", &corpus).is_none());
    }

    /// A record with its own real `DAMAGE:` token must use that value, not
    /// fall through to a `BASEITEM:` chase it does not need — the fallback
    /// only fires when the record's own row has nothing.
    #[test]
    fn a_records_own_damage_token_wins_over_its_baseitem() {
        let text = "Base Weapon\tKEY:Base Weapon\tDAMAGE:1d4\tTYPE:Weapon.Melee\n\
                    Overridden\tKEY:Overridden\tBASEITEM:Base Weapon\tDAMAGE:2d6\tTYPE:Weapon.Melee\n";
        let corpus = corpus_from(text);

        let resolved =
            resolve_base_damage_dice("Overridden", &corpus).expect("Overridden must resolve");
        assert_eq!(
            resolved.base_dice,
            DiceExpression {
                count: 2,
                die_size: 6
            }
        );
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `WIELD:OneHanded`.
    #[test]
    fn longsword_one_handed_primary_hand_adds_full_str_modifier() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\n";
        let corpus = corpus_from(text);

        let resolved =
            resolve_str_damage_modifier("Longsword (Base)", &corpus, 3, WeaponHandSlot::Primary)
                .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.wield_category, WieldCategory::OneHanded);
        assert_eq!(resolved.str_damage_modifier, 3);
    }

    /// Real verbatim tokens copied from `KEY:Longspear (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `WIELD:TwoHanded`.
    #[test]
    fn longspear_two_handed_adds_one_and_a_half_times_str_modifier() {
        let text = "Longspear\tKEY:Longspear (Base)\tTYPE:Weapon.Melee.Simple\tCOST:5\tWT:9\tCRITMULT:x3\tCRITRANGE:1\tDAMAGE:1d8\tWIELD:TwoHanded\n";
        let corpus = corpus_from(text);

        let resolved =
            resolve_str_damage_modifier("Longspear (Base)", &corpus, 3, WeaponHandSlot::Primary)
                .expect("Longspear (Base) must resolve");
        assert_eq!(resolved.wield_category, WieldCategory::TwoHanded);
        assert_eq!(resolved.str_damage_modifier, 4, "floor(1.5 * 3) = 4");
    }

    /// Real verbatim tokens copied from `KEY:Dagger (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `WIELD:Light`.
    #[test]
    fn dagger_off_hand_adds_half_str_modifier_rounded_down() {
        let text = "Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Melee.Simple\tCOST:2\tWT:1\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d4\tWIELD:Light\n";
        let corpus = corpus_from(text);

        let resolved =
            resolve_str_damage_modifier("Dagger (Base)", &corpus, 3, WeaponHandSlot::OffHand)
                .expect("Dagger (Base) must resolve");
        assert_eq!(resolved.wield_category, WieldCategory::Light);
        assert_eq!(resolved.str_damage_modifier, 1, "floor(0.5 * 3) = 1");
    }

    #[test]
    fn armor_record_has_no_str_damage_modifier() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
        let corpus = corpus_from(text);

        assert!(resolve_str_damage_modifier(
            "Leather Armor (Base)",
            &corpus,
            3,
            WeaponHandSlot::Primary
        )
        .is_none());
    }

    /// Real verbatim tokens: `KEY:Longsword (Base)`
    /// (`core_rulebook/cr_equip_arms_armor.lst`) plus `KEY:Special
    /// Ability ~ +1 ~ Weapon` (`core_rulebook/cr_equipmods.lst` line 219,
    /// `BONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement`).
    #[test]
    fn plus_one_weapon_enhancement_adds_to_both_attack_and_damage() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Longsword (Base)"),
            selection("Special Ability ~ +1 ~ Weapon"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.weapon_record_key, "Longsword (Base)");
        assert_eq!(resolved.attack_bonus, 1);
        assert_eq!(resolved.damage_bonus, 1);
    }

    /// Real verbatim tokens: `KEY:Material ~ Adamantine ~ Weapon`
    /// (`core_rulebook/cr_equipmods.lst` line 101,
    /// `BONUS:WEAPON|TOHIT|1|TYPE=Enhancement`) — a `TOHIT`-only chain,
    /// proving the engine reads the affected-roll set off the token
    /// rather than assuming every enhancement source hits both rolls.
    #[test]
    fn tohit_only_enhancement_does_not_add_to_damage() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
Adamantine\tKEY:Material ~ Adamantine ~ Weapon\tTYPE:BaseMaterial.MasterworkQuality.Weapon\tCOST:3000\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Longsword (Base)"),
            selection("Material ~ Adamantine ~ Weapon"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.attack_bonus, 1);
        assert_eq!(resolved.damage_bonus, 0);
    }

    #[test]
    fn no_enhancement_equipped_yields_honest_zero_not_fabricated() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);
        let equipped = vec![selection("Longsword (Base)")];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.attack_bonus, 0);
        assert_eq!(resolved.damage_bonus, 0);
    }

    #[test]
    fn unresolvable_weapon_yields_none_not_fabricated() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);
        let effects = compute_equipment_effects(&[], &corpus);

        assert!(resolve_weapon_enhancement_modifier(
            "item:does-not-exist-in-this-corpus",
            &corpus,
            &effects
        )
        .is_none());
    }

    /// `SD31-W17-INTEGRATE-001` fix (OPEN-ISSUES row 309, SD-31 wave 18):
    /// real verbatim tokens for `KEY:Unarmed Strike`
    /// (`core_rulebook/cr_equip_arms_armor.lst` line 296, a real
    /// natural-attack weapon -- carries the `Natural` `TYPE:` segment) plus
    /// `KEY:Special Ability ~ +1 ~ Amulet of Mighty Fists`
    /// (`WEAPONPROF=TYPE.Natural`). The Amulet's bonus applies to the
    /// natural attack.
    #[test]
    fn amulet_of_mighty_fists_applies_to_a_real_natural_attack() {
        let text = "Unarmed Strike\tKEY:Unarmed Strike\tTYPE:Weapon.Resizable.Melee.Special.Unarmed.Monk.Bludgeoning.Finesseable.Close.Weapon Group Close.Weapon Group Monk.Weapon Group Natural.Natural.Light\tCOST:0\tWT:0\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d3\tWIELD:Light\n\
+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Unarmed Strike"),
            selection("Special Ability ~ +1 ~ Amulet of Mighty Fists"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Unarmed Strike", &corpus, &effects)
            .expect("Unarmed Strike must resolve");
        assert_eq!(resolved.attack_bonus, 1, "a natural attack must receive the Amulet's bonus");
        assert_eq!(resolved.damage_bonus, 1);
    }

    /// The exact regression wave 17 shipped and review reverted: an
    /// equipped Amulet of Mighty Fists must NOT bonus an ordinary weapon.
    /// Same fixture as the passing case above, but resolved against the
    /// Longsword instead of the Unarmed Strike in the SAME loadout.
    #[test]
    fn amulet_of_mighty_fists_does_not_apply_to_an_ordinary_weapon() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
Unarmed Strike\tKEY:Unarmed Strike\tTYPE:Weapon.Resizable.Melee.Special.Unarmed.Monk.Bludgeoning.Finesseable.Close.Weapon Group Close.Weapon Group Monk.Weapon Group Natural.Natural.Light\tCOST:0\tWT:0\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d3\tWIELD:Light\n\
+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Longsword (Base)"),
            selection("Unarmed Strike"),
            selection("Special Ability ~ +1 ~ Amulet of Mighty Fists"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(
            resolved.attack_bonus, 0,
            "the Amulet of Mighty Fists must not bonus an ordinary weapon -- SD31-W17-INTEGRATE-001 row 309"
        );
        assert_eq!(resolved.damage_bonus, 0);
    }

    /// An ordinary (non-natural-attack-scoped) enhancement bonus is
    /// unaffected by the natural-attack-scope check -- proves the fix is
    /// additive, not a regression on the existing `WEAPON`-subject path.
    #[test]
    fn an_ordinary_enhancement_bonus_still_applies_regardless_of_natural_attack_scope() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let corpus = corpus_from(text);
        let equipped = vec![
            selection("Longsword (Base)"),
            selection("Special Ability ~ +1 ~ Weapon"),
        ];
        let effects = compute_equipment_effects(&equipped, &corpus);

        let resolved = resolve_weapon_enhancement_modifier("Longsword (Base)", &corpus, &effects)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.attack_bonus, 1);
        assert_eq!(resolved.damage_bonus, 1);
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `CRITRANGE:2`.
    #[test]
    fn longsword_critrange_2_threatens_19_to_20() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        let resolved = resolve_critical_threat_range("Longsword (Base)", &corpus)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.critical_threat_range, (19, 20));
    }

    /// Real verbatim tokens copied from `KEY:Rapier (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `CRITRANGE:3`.
    #[test]
    fn rapier_critrange_3_threatens_18_to_20() {
        let text = "Rapier\tKEY:Rapier (Base)\tTYPE:Weapon.Melee.Martial\tCOST:20\tWT:2\tCRITMULT:x2\tCRITRANGE:3\tDAMAGE:1d6\n";
        let corpus = corpus_from(text);

        let resolved = resolve_critical_threat_range("Rapier (Base)", &corpus)
            .expect("Rapier (Base) must resolve");
        assert_eq!(resolved.critical_threat_range, (18, 20));
    }

    #[test]
    fn armor_record_has_no_critical_threat_range() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
        let corpus = corpus_from(text);

        assert!(resolve_critical_threat_range("Leather Armor (Base)", &corpus).is_none());
    }

    #[test]
    fn critical_threat_range_unresolvable_item_yields_none_not_fabricated() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        assert!(
            resolve_critical_threat_range("item:does-not-exist-in-this-corpus", &corpus)
                .is_none()
        );
    }

    /// Real verbatim tokens copied from `KEY:Longsword (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `CRITMULT:x2`.
    #[test]
    fn longsword_critmult_x2_yields_multiplier_2() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        let resolved = resolve_critical_multiplier("Longsword (Base)", &corpus)
            .expect("Longsword (Base) must resolve");
        assert_eq!(resolved.critical_multiplier, 2);
    }

    /// Real verbatim tokens copied from `KEY:Scythe (Base)` in
    /// `core_rulebook/cr_equip_arms_armor.lst` — `CRITMULT:x4`.
    #[test]
    fn scythe_critmult_x4_yields_multiplier_4() {
        let text = "Scythe\tKEY:Scythe (Base)\tTYPE:Weapon.Melee.Martial\tCOST:18\tWT:10\tCRITMULT:x4\tCRITRANGE:1\tDAMAGE:2d4\n";
        let corpus = corpus_from(text);

        let resolved = resolve_critical_multiplier("Scythe (Base)", &corpus)
            .expect("Scythe (Base) must resolve");
        assert_eq!(resolved.critical_multiplier, 4);
    }

    #[test]
    fn armor_record_has_no_critical_multiplier() {
        let text = "Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
        let corpus = corpus_from(text);

        assert!(resolve_critical_multiplier("Leather Armor (Base)", &corpus).is_none());
    }

    #[test]
    fn critical_multiplier_unresolvable_item_yields_none_not_fabricated() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n";
        let corpus = corpus_from(text);

        assert!(
            resolve_critical_multiplier("item:does-not-exist-in-this-corpus", &corpus).is_none()
        );
    }

    fn selection(item_id: &str) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }
    }

    fn corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let source_ref = SourceRef {
            lst_file: "cr_equip_arms_armor.lst".to_string(),
            line: 1,
        };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }


}

// -- split from `eqmweapon_damagesize_tests` in src/rules_core/damage_total.rs (pcgen-touching items only) --
mod eqmweapon_damagesize_tests {
    use codex::rules_core::damage_total::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::source_content::SourceRef;

    /// Real verbatim tokens: `KEY:Heavy Wooden Shield (Base)`'s own
    /// `DAMAGE:1d4` shield-bash token (`core_rulebook/
    /// cr_equip_arms_armor.lst`) with `EQMOD:Special Quality ~ Spikes ~
    /// Shieldbash` baked in, plus the real modifier record
    /// (`cr_equipmods.lst`, `BONUS:EQMWEAPON|DAMAGESIZE|1`) -- the exact
    /// pair the live oracle confirmed this cycle (`WEAPON.0.DAMAGE`
    /// `1d4` -> `1d6+3`, the `+3` being the wielder's own STR modifier,
    /// not part of this die-size effect).
    #[test]
    fn damagesize_steps_a_real_shieldbash_hosts_die() {
        let text = "\
Heavy Wooden Shield Spiked\tKEY:Shield ~ Spiked Test\tTYPE:Shield.Heavy.Weapon.Resizable.Melee.ShieldBash.Close\tCOST:7\tWT:10\tACCHECK:-2\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d4\tWIELD:OneHanded\tSIZE:M\tSPELLFAILURE:15\tEQMOD:Special Quality ~ Spikes ~ Shieldbash\n\
Shield Spikes\tKEY:Special Quality ~ Spikes ~ Shieldbash\tTYPE:Shieldbash\tCOST:0\tBONUS:EQMWEAPON|DAMAGESIZE|1\n";
        let corpus = corpus_from(text);

        let stepped = resolve_eqmweapon_damagesize_effect("Shield ~ Spiked Test", &corpus);

        assert_eq!(
            stepped,
            Some(DiceExpression { count: 1, die_size: 6 }),
            "the shield's own 1d4 base stepped by DAMAGESIZE:1 must be 1d6"
        );
    }

    #[test]
    fn a_host_with_no_eqmod_yields_none_not_a_fabricated_step() {
        let text = "Heavy Wooden Shield\tKEY:Shield ~ Base Test\tTYPE:Shield.Heavy.Weapon.Melee.ShieldBash\tCOST:7\tWT:10\tACCHECK:-2\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d4\tWIELD:OneHanded\tSIZE:M\n";
        let corpus = corpus_from(text);

        let stepped = resolve_eqmweapon_damagesize_effect("Shield ~ Base Test", &corpus);

        assert_eq!(stepped, None, "no EQMOD attached means no DAMAGESIZE chain to apply -- honest None");
    }

    fn corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }


}
