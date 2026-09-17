// -- split from `eqm_weightdiv_tests` in src/rules_core/equipment_effects.rs (pcgen-touching items only) --
mod eqm_weightdiv_tests {
    use codex::rules_core::equipment_effects::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::source_content::SourceRef;

    /// Real verbatim tokens: `KEY:Outfit (Explorer's)` (`WT:8`,
    /// `core_rulebook/cr_equip_general.lst`) with `EQMOD:Material ~
    /// Darkleaf Cloth ~ Clothing` baked in, plus the real modifier record
    /// (`advanced_race_guide/arg_equipmods.lst`, `BONUS:EQM|WEIGHTDIV|2`)
    /// -- the pair the live oracle confirmed this cycle (`WT:8` -> `4`).
    #[test]
    fn weightdiv_halves_a_real_hosts_weight() {
        let text = "\
Outfit (Explorer's) Darkleaf\tKEY:Outfit ~ Darkleaf Test\tTYPE:Goods.Clothing.Resizable.Starting\tCOST:10\tWT:8\tEQMOD:Material ~ Darkleaf Cloth ~ Clothing\n\
Darkleaf Cloth\tKEY:Material ~ Darkleaf Cloth ~ Clothing\tTYPE:BaseMaterial.MasterworkQuality.Cloth.Clothing\tCOST:500\tBONUS:EQM|WEIGHTDIV|2\n";
        let corpus = corpus_from(text, "arg_equip_general.lst");

        let weight = resolve_eqm_weightdiv_effect("Outfit ~ Darkleaf Test", &corpus);

        assert_eq!(weight, Some(4.0), "8 lbs divided by WEIGHTDIV:2 must be 4");
    }

    #[test]
    fn a_host_with_no_eqmod_yields_none_not_a_fabricated_weight() {
        let text = "Outfit (Explorer's)\tKEY:Outfit (Base) Test\tTYPE:Goods.Clothing\tCOST:10\tWT:8\n";
        let corpus = corpus_from(text, "cr_equip_general.lst");

        let weight = resolve_eqm_weightdiv_effect("Outfit (Base) Test", &corpus);

        assert_eq!(weight, None, "no EQMOD attached means no WEIGHTDIV chain to apply -- honest None");
    }

    fn corpus_from(text: &str, source_file: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries(source_file, text);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: source_file.to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("advanced_race_guide", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }


}

// -- split from `attack_bonus_delta_tests` in src/rules_core/equipment_effects.rs (pcgen-touching items only) --
mod attack_bonus_delta_tests {
    use codex::rules_core::equipment_effects::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex::rules_core::character_input::EquipmentSelection;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::character_input::ActiveState;
    use codex::rules_core::source_content::SourceRef;

    #[test]
    fn exactly_one_weapon_plus_a_real_enhancement_yields_a_real_attack_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (Base)",
            &["Special Ability ~ +1 ~ Weapon"],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, Some(1), "unambiguous single weapon: +1 TOHIT applies");
        assert_eq!(effects.per_item[0].to_hit_bonus, Some(1));
    }

    #[test]
    fn a_masterwork_tohit_only_bonus_also_counts() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (Base)",
            &["Special Quality ~ Masterwork ~ Weapon"],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, Some(1));
    }

    #[test]
    fn a_damage_only_enhancement_does_not_affect_the_attack_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (Base)",
            &["Special Ability ~ Flaming ~ Weapon"],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "a DAMAGE-only enhancement must not contribute to the TOHIT-affecting attack bonus, \
             but the field itself is still a real, unambiguous Some(0), not None"
        );
    }

    #[test]
    fn exactly_one_weapon_with_no_enhancement_yields_a_real_zero() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Longsword (Base)"), equipped("Chain Shirt (Base)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "no ambiguity with one weapon; the real value is honestly zero, not absent"
        );
    }

    /// v0.6 alpha swarm sub-task 2: a modifier that exists in the corpus and
    /// is genuinely equipped, but is NOT listed in any weapon's
    /// `applied_modifiers`, must not silently attach to "the only weapon
    /// equipped" any more -- proves attachment is now required, not merely
    /// unambiguous-by-elimination (the old heuristic this replaced would
    /// have applied it here).
    #[test]
    fn an_equipped_but_unattached_modifier_does_not_apply_to_any_weapon() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Longsword (Base)"), equipped("Special Ability ~ +1 ~ Weapon")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "the +1 modifier is equipped but not attached to the Longsword via applied_modifiers, \
             so it must not count -- explicit attachment is required now"
        );
    }

    /// v0.6 alpha swarm sub-task 2: the real, closed 2+-weapon gap. The
    /// aggregate `attack_bonus_delta` stays honestly `None` (a single
    /// scalar can't represent two different weapons' bonuses at once --
    /// see its own doc comment), but each weapon's own `to_hit_bonus` in
    /// `per_item` is now real and unambiguous, since each modifier is
    /// explicitly attached to the specific weapon it enhances.
    #[test]
    fn two_weapons_each_with_their_own_attached_modifier_resolve_independently() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped_with_modifiers("Longsword (Base)", &["Special Ability ~ +1 ~ Weapon"]),
            equipped("Dagger (Base)"),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta, None,
            "the aggregate scalar stays None for 2+ weapons -- it cannot represent both at once"
        );
        let longsword = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Longsword (Base)")
            .expect("Longsword must resolve");
        let dagger = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Dagger (Base)")
            .expect("Dagger must resolve");
        assert_eq!(longsword.to_hit_bonus, Some(1), "Longsword's own attached +1 resolves independently");
        assert_eq!(dagger.to_hit_bonus, Some(0), "Dagger has no attached modifier, a real zero");
    }

    #[test]
    fn zero_weapons_equipped_leaves_the_attack_bonus_honestly_absent() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Chain Shirt (Base)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, None, "no weapon at all -- nothing to attach a bonus to");
    }

    /// Regression test for the real bug found by wiring the real corpus into
    /// the desktop app: a standard armor+shield+one-weapon loadout used to
    /// resolve as "2+ weapons, ambiguous" the moment a shield's own real
    /// `DAMAGE:` bash-attack token reached this path (the tiny desktop
    /// fixture bundle never carried a real shield record, so this was never
    /// exercised before). A shield must not count toward the single-weapon
    /// ambiguity check.
    #[test]
    fn a_shield_with_a_real_bash_damage_token_does_not_count_as_a_second_weapon() {
        const SHIELD_FIXTURE_TEXT: &str = "\
Heavy Wooden Shield\tKEY:Heavy Wooden Shield (Base)\tTYPE:Shield.Heavy.Weapon.Resizable.Melee.ShieldBash.Close.Weapon Group Close.Nonmetal\tCOST:7\tWT:10\tACCHECK:-2\tDAMAGE:1d4\tBONUS:COMBAT|AC|2|TYPE=Shield|PREVAREQ:DisableShieldBonus,0\n";
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", SHIELD_FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let mut corpus = corpus_with_fixture();
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }

        let equipped_items = vec![equipped("Longsword (Base)"), equipped("Heavy Wooden Shield (Base)")];
        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "exactly one real weapon (the Longsword) -- the shield's own bash-damage token must not count as a second"
        );
        let shield = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Heavy Wooden Shield (Base)")
            .expect("shield must resolve");
        assert_eq!(shield.armor_class_bonus, Some(2), "the shield's own real AC bonus still resolves");
    }

    fn equipped(item_id: &str) -> EquipmentSelection {
        equipped_with_modifiers(item_id, &[])
    }

    fn equipped_with_modifiers(item_id: &str, applied_modifiers: &[&str]) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: applied_modifiers.iter().map(|id| id.to_string()).collect(),
        }
    }

    const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Dagger\tKEY:Dagger (Base)\tTYPE:Weapon.Melee.Simple\tCOST:2\tWT:1\tCRITMULT:x2\tDAMAGE:1d4
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement
Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
Flaming\tKEY:Special Ability ~ Flaming ~ Weapon\tTYPE:Weapon\tCOST:0\tBONUS:WEAPON|DAMAGE|2|TYPE=Enhancement
";

    fn corpus_with_fixture() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }


}

// -- split from `book_agnostic_resolution_tests` in src/rules_core/equipment_effects.rs (pcgen-touching items only) --
mod book_agnostic_resolution_tests {
    use codex::rules_core::equipment_effects::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex::rules_core::character_input::EquipmentSelection;
    use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::character_input::ActiveState;
    use codex::rules_core::source_content::SourceRef;

    #[test]
    fn a_non_crb_armor_item_resolves_all_four_arms_armor_stats() {
        let corpus = arg_corpus(ARG_ARMOR_FIXTURE_TEXT, "arg_equip_arms_armor.lst");
        let equipped_items = vec![equipped("Reinforced Leather (ARG)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item.len(), 1, "the item must not be silently dropped");
        let item = &effects.per_item[0];
        assert_eq!(item.armor_class_bonus, Some(3));
        assert_eq!(item.max_dex, Some(5));
        assert_eq!(item.spell_failure, Some(15.0));
        assert_eq!(item.armor_check_penalty, Some(-1));
        assert_eq!(item.category, EquipmentCategory::ArmsArmor);
        assert_eq!(effects.armor_class_delta, 3);
        assert_eq!(effects.max_dex_cap, Some(5));
        assert_eq!(effects.spell_failure_chance, Some(15.0));
        assert_eq!(effects.armor_check_penalty_total, -1);
    }

    #[test]
    fn a_non_crb_equipment_modifier_still_applies_its_tohit_bonus() {
        let corpus = arg_corpus(ARG_EQUIPMOD_FIXTURE_TEXT, "arg_equipmods.lst");
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (ARG)",
            &["Special Ability ~ Keen ~ Weapon (ARG)"],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.attack_bonus_delta, Some(1), "a non-CRB weapon modifier must still apply");
        assert_eq!(effects.per_item[0].to_hit_bonus, Some(1));
    }

    /// SD-33 remediation wave 4 (`AT-33-E5-003`): end-to-end proof that
    /// `compute_equipment_effects` itself (not just
    /// `arms_armor::apply_eqmod_armor_class_bonus` in isolation) resolves
    /// a base item's `EQMOD:`-referenced modifier record ACROSS THE
    /// WHOLE CORPUS and sums its real `COMBAT|AC` contribution -- real
    /// verbatim tokens from `inner_sea_races:equipment:armor_of_grim_
    /// triumph` (base) and Core Rulebook's own `Special Ability ~ +1 ~
    /// Armor` (the modifier, in a DIFFERENT source line, proving
    /// resolution is not book-scoped). Before this fix,
    /// `item.armor_class_bonus` was `Some(6)` (the base item's own chain
    /// alone) -- a real, confirmed disagreement against the pinned
    /// oracle's `7` (`AT-33-E5-003.combined-oracle-results.json`).
    #[test]
    fn eqmod_referenced_modifier_sums_across_the_whole_corpus() {
        let text = "\
Armor of Grim Triumph\tKEY:Armor of Grim Triumph\tTYPE:Armor.Magic.Medium.ArmorProfMedium.Suit.Specific\tCOST:250\tWT:40\tACCHECK:-4\tEQMOD:Special Ability ~ Enhancement Cost|12600.Special Ability ~ +1 ~ Armor.Special Quality ~ Spikes ~ Armor.Material ~ Steel\tMAXDEX:3\tSPELLFAILURE:25\tBONUS:COMBAT|AC|6|TYPE=Armor\n\
+1 (Enhancement to Armor)\tKEY:Special Ability ~ +1 ~ Armor\tTYPE:Armor\tPLUS:1\tBONUS:COMBAT|AC|1|TYPE=ArmorEnhancement|PREVAREQ:DisableArmorBonus,0\n\
Armor Spikes\tKEY:Special Quality ~ Spikes ~ Armor\tTYPE:Armor\tCOST:50\n";
        let corpus = arg_corpus(text, "isr_equip_arms_armor.lst");
        let equipped_items = vec![equipped("Armor of Grim Triumph")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item.len(), 1);
        assert_eq!(
            effects.per_item[0].armor_class_bonus,
            Some(7),
            "base item's own 6 plus the EQMOD-referenced +1 Armor modifier's own separate chain"
        );
        assert_eq!(effects.armor_class_delta, 7);
    }

    /// SD-33 remediation wave 6 (`AT-33-E5-003`'s escalated blocker,
    /// `rending_claw_blades`): end-to-end proof of the shape the corpus-
    /// extraction fix (`fbc945f198`) newly surfaced. Real verbatim tokens
    /// from `advanced_race_guide:equipment:rending_claw_blades`'s own
    /// post-fix corpus record (`data/corpus/advanced_race_guide/equipment/
    /// rending_claw_blades.json`) and Core Rulebook's own `Special Ability
    /// ~ +1 ~ Weapon` (already proven in isolation by
    /// `equipmods::tests::plus_one_weapon_enhancement_yields_a_real_
    /// damage_tohit_bonus`).
    ///
    /// Two distinct gaps, both real, both closed by this cycle:
    ///
    /// 1. The base record itself carries TWO separate `EQMOD:` tokens
    ///    (one from its own line, `Material ~ Steel`; a second folded in
    ///    from a `.MOD` row citing `Keen`/`+1`/`Material ~ Steel` --
    ///    `enrich_equipment_raw_tokens.rs`'s fold appends rather than
    ///    merges, confirmed against the real committed JSON). Before this
    ///    fix, `eqmod_referenced_records` read only the FIRST `EQMOD:`
    ///    token (`.find()`), so the richer, `.MOD`-folded token -- the one
    ///    that actually names the `+1` modifier -- was never even
    ///    inspected, regardless of any weapon-dimension gap.
    /// 2. `compute_equipment_effects`'s weapon path summed only the base
    ///    record's own declared bonus chains
    ///    (`pcgen_import::ingest_record::bonus_chain_qualifiers`; `TOHIT`
    ///    only, per its own real
    ///    chain) into `weapon_enhancement_bonus`, unlike the AC dimension,
    ///    which already folds `EQMOD:`-referenced records in
    ///    (`resolve_category_effect`). The `+1 Weapon` modifier's own
    ///    `DAMAGE` contribution was never reachable at all.
    ///
    /// Before this fix: `tohit_bonus` matches the oracle (`Some(1)`,
    /// TOHIT already resolves), `damage_bonus` is `None` against the
    /// oracle's real `MAGICDAMAGE=+1` -- the exact
    /// `AT-33-E5-003.combined-oracle-results.json` disagreement (`ours=0
    /// oracle=1`, dimension `DAMAGE`).
    #[test]
    fn eqmod_referenced_modifier_sums_into_weapon_enhancement_bonus_across_two_eqmod_tokens() {
        let text = "\
Rending Claw Blades\tKEY:Rending Claw Blades\tTYPE:Weapon.Resizable.Light.Melee.Slashing.Finesseable\tCOST:305\tWT:2\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d4\tEQMOD:Material ~ Steel\tEQMOD:Special Ability ~ Keen ~ Weapon.Special Ability ~ +1 ~ Weapon.Material ~ Steel\tWIELD:Light\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n\
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n\
Keen\tKEY:Special Ability ~ Keen ~ Weapon\tTYPE:Weapon\tCOST:0\n\
Material Steel\tKEY:Material ~ Steel\tTYPE:Weapon\tCOST:0\n";
        let corpus = arg_corpus(text, "arg_equip_arms_armor.lst");
        let equipped_items = vec![equipped("Rending Claw Blades")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item.len(), 1);
        let bonus = effects.per_item[0]
            .weapon_enhancement_bonus
            .as_ref()
            .expect("the base record's own TOHIT chain alone already makes this Some");
        assert_eq!(
            bonus.tohit_bonus,
            Some(1),
            "base TOHIT|1 and the +1 modifier's own TOHIT|1 are the SAME TYPE=Enhancement bonus -- \
             Pathfinder's same-type stacking rule takes the higher, not the sum (max(1, 1) = 1); \
             live-oracle-confirmed MAGICHIT=+1, never +2"
        );
        assert_eq!(
            bonus.damage_bonus,
            Some(1),
            "the EQMOD-referenced +1 Weapon modifier's own separate DAMAGE chain must fold in -- \
             this is the real rending_claw_blades disagreement (ours=0 oracle=1, dimension DAMAGE); \
             base contributes no DAMAGE chain at all, so the modifier's 1 is simply the result"
        );
    }

    fn equipped(item_id: &str) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }
    }

    fn arg_corpus(text: &str, source_file: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries(source_file, text);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: source_file.to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("advanced_race_guide", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    /// Not a literal single verbatim ARG record (no clean single-line
    /// arms_armor example exists in the real ARG source -- the book adds
    /// exotic weapons and accessories, not new base armor), but real,
    /// verified CRB armor token grammar (matches this file's own
    /// `Chain Shirt (Base)` fixture exactly), tagged as coming from
    /// `advanced_race_guide` to exercise the book-agnostic path
    /// specifically. Stated plainly rather than implied as verbatim.
    const ARG_ARMOR_FIXTURE_TEXT: &str = "\
Reinforced Leather (ARG)\tKEY:Reinforced Leather (ARG)\tTYPE:Armor.Light\tCOST:120\tWT:18\tACCHECK:-1\tMAXDEX:5\tSPELLFAILURE:15\tBONUS:COMBAT|AC|3|TYPE=Armor\n";

    const ARG_EQUIPMOD_FIXTURE_TEXT: &str = "\
Longsword (ARG)\tKEY:Longsword (ARG)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\n\
Keen (ARG)\tKEY:Special Ability ~ Keen ~ Weapon (ARG)\tTYPE:Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";

    fn equipped_with_modifiers(item_id: &str, applied_modifiers: &[&str]) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: applied_modifiers.iter().map(|id| id.to_string()).collect(),
        }
    }


}

// -- split from `intelligent_item_and_natural_attack_scope_tests` in src/rules_core/equipment_effects.rs (pcgen-touching items only) --
mod intelligent_item_and_natural_attack_scope_tests {
    use codex::rules_core::equipment_effects::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex::rules_core::character_input::EquipmentSelection;
    use codex::rules_core::equipment_resolver::equipment_converted_resolve;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::character_input::ActiveState;
    use codex::rules_core::equipment_effects::intelligent_item::ItemAlignment;
    use codex::rules_core::source_content::SourceRef;

    #[test]
    fn an_intelligent_longsword_yields_its_real_summed_stat_block() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers(
            "Longsword (Base)",
            &[
                "Intelligent Item ~ Base",
                "Intelligent Item ~ Ability Score / Wisdom 15",
                "Legendary Item ~ Intelligent Item ~ Alignment / Chaotic Good",
            ],
        )];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let item = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Longsword (Base)")
            .expect("Longsword must resolve");
        let intelligent_item = item
            .intelligent_item
            .as_ref()
            .expect("an intelligent longsword must yield a contribution");

        assert_eq!(intelligent_item.intelligence_bonus, 10, "10 (Base) + 0 (no INT modifier attached)");
        assert_eq!(intelligent_item.wisdom_bonus, 15, "10 (Base) + 5 (Wisdom 15's own delta) == 15");
        assert_eq!(intelligent_item.charisma_bonus, 10, "10 (Base) + 0");
        assert_eq!(intelligent_item.ego_bonus, 2, "Wisdom 15's own literal Ego contribution");
        assert_eq!(intelligent_item.alignment, Some(ItemAlignment::ChaoticGood));
    }

    #[test]
    fn an_ordinary_longsword_with_no_intelligent_item_modifiers_has_none() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped_with_modifiers("Longsword (Base)", &["Special Ability ~ +1 ~ Weapon"])];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let item = &effects.per_item[0];

        assert_eq!(item.intelligent_item, None, "a plain +1 weapon carries no intelligent-item token at all");
    }

    /// Regression guard: an intelligent item's stat block is scoped to the
    /// SPECIFIC selection it's attached to (`applied_modifiers`), not
    /// summed across the whole loadout the way `weapon_enhancement_bonus`
    /// bounded model does -- a second, ordinary weapon equipped alongside
    /// the intelligent longsword must not pick up its stat block.
    #[test]
    fn a_second_unattached_weapon_does_not_inherit_the_first_items_intelligence() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped_with_modifiers(
                "Longsword (Base)",
                &["Intelligent Item ~ Base", "Intelligent Item ~ Ability Score / Wisdom 15"],
            ),
            equipped_with_modifiers("Unarmed Strike", &[]),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let unarmed = effects
            .per_item
            .iter()
            .find(|item| item.item_id == "Unarmed Strike")
            .expect("Unarmed Strike must resolve");

        assert_eq!(
            unarmed.intelligent_item, None,
            "the Longsword's own intelligent-item stat block must not leak onto an unattached weapon"
        );
    }

    #[test]
    fn unarmed_strike_is_a_real_natural_attack() {
        let corpus = corpus_with_fixture();
        let record = equipment_converted_resolve("Unarmed Strike", &corpus)
            .expect("Unarmed Strike must resolve");
        assert!(is_natural_attack_weapon(record));
    }

    #[test]
    fn a_longsword_is_not_a_natural_attack() {
        let corpus = corpus_with_fixture();
        let record = equipment_converted_resolve("Longsword (Base)", &corpus)
            .expect("Longsword must resolve");
        assert!(!is_natural_attack_weapon(record));
    }

    /// `SD31-W17-INTEGRATE-001` (OPEN-ISSUES row 309): the Amulet of
    /// Mighty Fists family's `WEAPONPROF=TYPE.Natural` chain is now
    /// recognized (`equipmods.rs`) and tagged `natural_attack_only: true`
    /// -- proves the tag survives resolution through
    /// `compute_equipment_effects`, ready for `damage_total.rs`'s
    /// consumer to honour.
    #[test]
    fn an_amulet_of_mighty_fists_modifier_is_tagged_natural_attack_only() {
        let corpus = corpus_with_fixture();
        // Equipped as its own top-level selection, matching how
        // `equipmods.rs`'s own weapon-enhancement fixtures equip
        // "Special Ability ~ +1 ~ Weapon" directly -- `weapon_enhancement_bonus`
        // reads the SELECTION's own record, unlike `to_hit_bonus`/
        // `intelligent_item`, which read `applied_modifiers`.
        let equipped_items = vec![equipped_with_modifiers("Special Ability ~ +1 ~ Amulet of Mighty Fists", &[])];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let amulet = &effects.per_item[0];
        let bonus = amulet
            .weapon_enhancement_bonus
            .as_ref()
            .expect("the Amulet of Mighty Fists chain must now resolve to a real bonus");
        assert!(bonus.natural_attack_only, "the Amulet of Mighty Fists family scopes to natural attacks only");
        assert_eq!(bonus.tohit_bonus, Some(1));
        assert_eq!(bonus.damage_bonus, Some(1));
    }

    /// `SD31-W18-INTEGRATE-001` (adversarial review, `OPEN-ISSUES.md` row
    /// 309 re-opened): `damage_total::resolve_weapon_enhancement_modifier`
    /// was scope-guarded in wave 18, but `resolve_weapon_to_hit_bonus` --
    /// the function `compute_equipment_effects` actually calls for
    /// `to_hit_bonus`/`attack_bonus_delta` via `selection.applied_modifiers`,
    /// the SAME attachment shape the shipped desktop app's
    /// `attach_equipment_modifier_at_root` uses -- was not. Confirmed live
    /// before this fix: an Amulet of Mighty Fists +1 attached to an
    /// ordinary Longsword yielded `to_hit_bonus = Some(1)` /
    /// `attack_bonus_delta = Some(1)`, leaking the natural-attack-only
    /// bonus onto a non-natural weapon. Mutation-provable: removing the
    /// `bonus.natural_attack_only && !weapon_is_natural_attack` skip in
    /// `resolve_weapon_to_hit_bonus` reproduces `Some(1)` here.
    #[test]
    fn amulet_of_mighty_fists_attached_as_a_modifier_does_not_leak_onto_an_ordinary_longsword() {
        let corpus = corpus_with_fixture();
        let equipped_items =
            vec![equipped_with_modifiers("Longsword (Base)", &["Special Ability ~ +1 ~ Amulet of Mighty Fists"])];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let longsword = &effects.per_item[0];

        assert_eq!(
            longsword.to_hit_bonus,
            Some(0),
            "a natural-attack-only bonus attached to a non-natural weapon must not apply"
        );
        assert_eq!(
            effects.attack_bonus_delta,
            Some(0),
            "the single-weapon attack_bonus_delta scalar must reflect the same scope guard"
        );
    }

    /// Positive control for the same guard: a REAL natural attack
    /// (Unarmed Strike) with the Amulet attached as a modifier DOES
    /// receive the bonus -- proves the fix scopes the bonus, it does not
    /// simply zero it out everywhere.
    #[test]
    fn amulet_of_mighty_fists_attached_as_a_modifier_applies_to_a_real_natural_attack() {
        let corpus = corpus_with_fixture();
        let equipped_items =
            vec![equipped_with_modifiers("Unarmed Strike", &["Special Ability ~ +1 ~ Amulet of Mighty Fists"])];

        let effects = compute_equipment_effects(&equipped_items, &corpus);
        let unarmed = &effects.per_item[0];

        assert_eq!(unarmed.to_hit_bonus, Some(1), "a natural-attack-only bonus must apply to a real natural attack");
        assert_eq!(effects.attack_bonus_delta, Some(1));
    }

    const INTELLIGENT_ITEM_FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Intelligent Magic Item Base\tKEY:Intelligent Item ~ Base\tTYPE:Weapon.Armor.Goods\tCOST:500\tBONUS:VAR|IntItemStatINT|10\tBONUS:VAR|IntItemStatWIS|10\tBONUS:VAR|IntItemStatCHA|10\n\
Int Item / Stat Wisdom 15\tKEY:Intelligent Item ~ Ability Score / Wisdom 15\tTYPE:Weapon.Armor.Goods\tCOST:1400\tBONUS:VAR|IntelligentItemEgo|2\tBONUS:VAR|IntItemStatWIS|5\n\
Legendary Intelligent Item / Align (CG)\tKEY:Legendary Item ~ Intelligent Item ~ Alignment / Chaotic Good\tTYPE:Mythic.Intelligent.Alignment\tCOST:0\tBONUS:VAR|IntItemAlignment|20\n\
Unarmed Strike\tKEY:Unarmed Strike\tTYPE:Weapon.Resizable.Melee.Special.Unarmed.Monk.Bludgeoning.Finesseable.Close.Weapon Group Close.Weapon Group Monk.Weapon Group Natural.Natural.Light\tCOST:0\tWT:0\tCRITMULT:x2\tCRITRANGE:1\tDAMAGE:1d3\tWIELD:Light\n\
+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n\
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";

    fn corpus_with_fixture() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equipmods.lst", INTELLIGENT_ITEM_FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equipmods.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    fn equipped_with_modifiers(item_id: &str, applied_modifiers: &[&str]) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: applied_modifiers.iter().map(|id| id.to_string()).collect(),
        }
    }


}

// -- split from `spell_resistance_tests` in src/rules_core/equipment_effects.rs (pcgen-touching items only) --
mod spell_resistance_tests {
    use codex::rules_core::equipment_effects::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex::rules_core::character_input::EquipmentSelection;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::character_input::ActiveState;
    use codex::rules_core::source_content::SourceRef;

    #[test]
    fn spell_resistance_13_armor_yields_a_real_per_item_and_aggregate_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Special Ability ~ Spell Resistance / 13 ~ Armor")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item[0].spell_resistance_bonus, Some(13));
        assert_eq!(effects.spell_resistance_total, Some(13));
    }

    /// PF1's real rule: "If a creature has multiple sources of spell
    /// resistance, only the highest value applies" -- unlike
    /// `armor_class_delta` (armor and shield AC bonuses genuinely stack),
    /// two simultaneous SR sources must yield the HIGHEST single value,
    /// never their sum. Equipping both the 13 and 19 tiers together must
    /// read `Some(19)`, not `Some(32)`.
    #[test]
    fn two_spell_resistance_sources_yield_the_highest_not_the_sum() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![
            equipped("Special Ability ~ Spell Resistance / 13 ~ Armor"),
            equipped("Special Ability ~ Spell Resistance / 19 ~ Armor"),
        ];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(
            effects.spell_resistance_total,
            Some(19),
            "PF1: multiple SR sources take the highest value, they do not stack"
        );
    }

    #[test]
    fn an_ordinary_weapon_has_no_spell_resistance_bonus() {
        let corpus = corpus_with_fixture();
        let equipped_items = vec![equipped("Longsword (Base)")];

        let effects = compute_equipment_effects(&equipped_items, &corpus);

        assert_eq!(effects.per_item[0].spell_resistance_bonus, None);
        assert_eq!(effects.spell_resistance_total, None);
    }

    fn equipped(item_id: &str) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }
    }

    const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Spell Resistance 13\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 13 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:2\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:13\tSPROP:grants spell resistance 13
Spell Resistance 19\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 19 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:8\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:19\tSPROP:grants spell resistance 19
";

    fn corpus_with_fixture() -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equipmods.lst", FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { lst_file: "cr_equipmods.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }


}
