// -- split from `tests` in src/rules_core/equipment_effects/intelligent_item.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::equipment_effects::intelligent_item::*;
    use codex_ingest::pcgen_import::ir_converter::equipment_record_to_corpus;
    use codex_ingest::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Base` in
    /// `core_rulebook/cr_equipmods.lst` line 354 (trimmed to the tokens
    /// this resolver reads -- the record's real `PREVARGTEQ`-conditional
    /// `IntItemNegativeLevel` chains and formula-valued `IntelligentItemEgo`
    /// chain are omitted from this fixture on purpose to isolate the
    /// unconditional-literal assertion below; the full-record shape is
    /// covered by `the_base_record_skips_its_own_formula_ego_chain`).
    #[test]
    fn intelligent_item_base_yields_the_literal_ten_point_baseline_in_each_mental_ability() {
        let text = "Intelligent Magic Item Base\tKEY:Intelligent Item ~ Base\tTYPE:Weapon.Armor.Goods\tCOST:500\tBONUS:VAR|IntItemStatINT|10\tBONUS:VAR|IntItemStatWIS|10\tBONUS:VAR|IntItemStatCHA|10\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record).expect("Base record must yield a contribution");
        assert_eq!(effect.intelligence_bonus, 10);
        assert_eq!(effect.wisdom_bonus, 10);
        assert_eq!(effect.charisma_bonus, 10);
        assert_eq!(effect.ego_bonus, 0, "Base record fixture above carries no literal Ego chain");
        assert_eq!(effect.alignment, None);
    }

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Base`'s
    /// full line, including its formula-valued
    /// `BONUS:VAR|IntelligentItemEgo|(BaseCostTracker>=1001)+...` chain
    /// and its `PREVARGTEQ:`-conditional `IntItemNegativeLevel` chains --
    /// neither is a literal `[VAR, name, value]` triple this resolver
    /// reads, so both must be honestly skipped rather than fabricated
    /// into a fake Ego/negative-level number.
    #[test]
    fn the_base_record_skips_its_own_formula_ego_chain_and_its_conditional_chains() {
        let text = "Intelligent Magic Item Base\tKEY:Intelligent Item ~ Base\tTYPE:Weapon.Armor.Goods\tCOST:500\tBONUS:VAR|IntItemNegativeLevel|1|PREVARGTEQ:IntelligentItemEgo,20\tBONUS:VAR|IntItemStatINT|10\tBONUS:VAR|IntItemStatWIS|10\tBONUS:VAR|IntItemStatCHA|10\tBONUS:VAR|BaseCostTracker|COST\tBONUS:VAR|IntelligentItemEgo|(BaseCostTracker>=1001)+(BaseCostTracker>=5001)\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record).expect("literal INT/WIS/CHA chains still resolve");
        assert_eq!(effect.intelligence_bonus, 10);
        assert_eq!(effect.wisdom_bonus, 10);
        assert_eq!(effect.charisma_bonus, 10);
        assert_eq!(
            effect.ego_bonus, 0,
            "the formula-valued IntelligentItemEgo chain must not be parsed into a fabricated number"
        );
    }

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Ability
    /// Score / Wisdom 15` in `core_rulebook/cr_equipmods.lst` line 377 --
    /// carries BOTH its own Ego contribution and its Wisdom delta in the
    /// same record, proving both chains are read out of one record, not
    /// just the first match (unlike `compute_equipmods_effect`'s
    /// single-chain `find_map`).
    #[test]
    fn wisdom_fifteen_yields_its_real_stat_delta_and_its_real_ego_contribution() {
        let text = "Int Item / Stat Wisdom 15\tKEY:Intelligent Item ~ Ability Score / Wisdom 15\tTYPE:Weapon.Armor.Goods\tCOST:1400\tBONUS:VAR|IntelligentItemEgo|2\tBONUS:VAR|IntItemStatWIS|5\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record).expect("must yield a contribution");
        assert_eq!(effect.wisdom_bonus, 5, "10 (base, not asserted by this record) + 5 == 15, the record's own name");
        assert_eq!(effect.ego_bonus, 2);
        assert_eq!(effect.intelligence_bonus, 0);
        assert_eq!(effect.charisma_bonus, 0);
    }

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Ability
    /// Score / Charisma 20` (the top of the CHA ladder,
    /// `core_rulebook/cr_equipmods.lst`) -- a different ability entirely,
    /// proving the ability is read from the token name, not hardcoded to
    /// Wisdom.
    #[test]
    fn charisma_twenty_yields_its_real_cha_delta() {
        let text = "Int Item / Stat Charisma 20\tKEY:Intelligent Item ~ Ability Score / Charisma 20\tTYPE:Weapon.Armor.Goods\tCOST:8000\tBONUS:VAR|IntelligentItemEgo|5\tBONUS:VAR|IntItemStatCHA|10\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record).expect("must yield a contribution");
        assert_eq!(effect.charisma_bonus, 10);
        assert_eq!(effect.ego_bonus, 5);
    }

    /// Real verbatim tokens copied from `KEY:Legendary Item ~ Intelligent
    /// Item ~ Alignment / Chaotic Good` in
    /// `mythic_adventures/ma_equipmods.lst` line 96 -- the alignment
    /// family's own literal `IntItemAlignment|20` code, decoded to
    /// `ChaoticGood`.
    #[test]
    fn chaotic_good_alignment_decodes_from_its_real_two_digit_code() {
        let text = "Legendary Intelligent Item / Align (CG)\tKEY:Legendary Item ~ Intelligent Item ~ Alignment / Chaotic Good\tTYPE:Mythic.Intelligent.Alignment\tCOST:0\tBONUS:VAR|IntItemAlignment|20\n";
        let result = parse_equipment_entries("ma_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record).expect("must yield a contribution");
        assert_eq!(effect.alignment, Some(ItemAlignment::ChaoticGood));
        assert_eq!(effect.ego_bonus, 0, "the alignment chain grants no Ego of its own");
    }

    /// Real verbatim tokens copied from `KEY:Legendary Item ~ Intelligent
    /// Item ~ Alignment / Lawful Evil` -- the opposite corner of the
    /// alignment grid, proving the axis decoding is read from the code,
    /// not hardcoded to one alignment.
    #[test]
    fn lawful_evil_alignment_decodes_from_its_real_two_digit_code() {
        let text = "Legendary Intelligent Item / Align (LE)\tKEY:Legendary Item ~ Intelligent Item ~ Alignment / Lawful Evil\tTYPE:Mythic.Intelligent.Alignment\tCOST:0\tBONUS:VAR|IntItemAlignment|02\n";
        let result = parse_equipment_entries("ma_equipmods.lst", text);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record).expect("must yield a contribution");
        assert_eq!(effect.alignment, Some(ItemAlignment::LawfulEvil));
    }

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Power /
    /// Change Shape` (`core_rulebook/cr_equipmods.lst` line 432) -- a
    /// Power-family record, deliberately NOT resolved by this module (see
    /// its own doc comment): its own headline power is not represented
    /// here, but its real literal Ego contribution still is, since it's
    /// the same `IntelligentItemEgo` chain every family member carries.
    #[test]
    fn a_power_family_record_still_yields_its_literal_ego_contribution_but_no_ability_or_alignment() {
        let text = "Int Item / Power Change shape\tKEY:Intelligent Item ~ Power / Change Shape\tTYPE:Weapon.Armor.Goods\tCOST:10000\tBONUS:VAR|IntelligentItemEgo|2\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record).expect("must yield a contribution");
        assert_eq!(effect.ego_bonus, 2);
        assert_eq!(effect.intelligence_bonus, 0);
        assert_eq!(effect.wisdom_bonus, 0);
        assert_eq!(effect.charisma_bonus, 0);
        assert_eq!(effect.alignment, None);
    }

    /// Real verbatim tokens copied from `KEY:Masterwork (Weapon)` -- an
    /// ordinary, non-intelligent-item `equipmods` record carries none of
    /// this family's tokens at all.
    #[test]
    fn an_ordinary_equipmod_has_no_intelligent_item_contribution() {
        let text = "Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record);
        assert_eq!(effect, None);
    }

    /// A chain naming one of this family's variables but carrying a
    /// trailing `PREVARGTEQ:` condition (a 4-part chain, not the family's
    /// real unconditional 3-part shape) must not be asserted -- it is not
    /// unconditionally true. Regression guard for the exact shape the
    /// Base record's own `IntItemNegativeLevel` chains carry.
    #[test]
    fn a_conditional_var_chain_naming_a_family_variable_is_not_asserted() {
        let text = "Conditional Proxy\tKEY:Conditional Proxy\tTYPE:Weapon.Armor.Goods\tCOST:0\tBONUS:VAR|IntItemStatWIS|99|PREVARGTEQ:SomeVar,1\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = equipment_record_to_corpus(&result.entries[0]);

        let effect = compute_intelligent_item_effect(&record);
        assert_eq!(effect, None, "a conditional chain must not be asserted as an unconditional +99 Wisdom");
    }


}
