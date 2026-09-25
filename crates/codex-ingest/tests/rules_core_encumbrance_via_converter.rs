// -- split from `tests` in src/rules_core/encumbrance.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::encumbrance::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
    use codex::rules_core::size::SizeCategory;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use codex::rules_core::source_content::SourceRef;

    #[test]
    fn compute_encumbrance_sums_real_corpus_weight_for_carried_items() {
        let corpus = corpus_from(FIXTURE_TEXT);
        let equipment_selections = vec![
            selection("item:leather_armor", ActiveState::EquippedActive),
            selection("item:buckler", ActiveState::SelectedInactive),
            selection("item:longsword", ActiveState::EquippedActive),
        ];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 10, SizeCategory::Medium);

        assert_eq!(computation.total_carried_weight_lbs, 15.0 + 5.0 + 4.0);
        assert!(computation.unresolved_item_ids.is_empty(), "{:?}", computation.unresolved_item_ids);
        assert_eq!(computation.per_item.len(), 3);
        assert_eq!(computation.thresholds, carrying_capacity_thresholds(10, SizeCategory::Medium));
        // 24 lbs total is within Strength 10's light max (33 lbs).
        assert_eq!(computation.level, EncumbranceLevel::Light);
    }

    /// Regression test for the real bug found by SD-27's Advanced Race Guide
    /// PCGen parity run: a non-Core-Rulebook item (here, ARG's own Dogslicer,
    /// verbatim `COST:8 WT:1` from `arg_equip_arms_armor.lst`) resolves
    /// through `equipment_converted_resolve` (already book-agnostic) but weight and
    /// cost were both silently dropped when a second, CRB-only lookup ran
    /// against `rules_tables::crb::equipment_tables()`. Reading both directly
    /// off the resolved record's own settled weight and price
    /// fixes this for every book, not just ARG. See
    /// `docs/release/v0.6/book-agnostic-backend-gaps-scoping.md` finding 1.
    #[test]
    fn compute_encumbrance_resolves_weight_and_cost_for_a_non_crb_book_item() {
        const ARG_FIXTURE_TEXT: &str = "\
Dogslicer\tKEY:Dogslicer\tTYPE:Weapon.Resizable.Melee.Slashing.Goblin\tCOST:8\tWT:1\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d4\n";
        let result = parse_equipment_entries("arg_equip_arms_armor.lst", ARG_FIXTURE_TEXT);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { source_path: "arg_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("advanced_race_guide", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }

        let equipment_selections = vec![selection("Dogslicer", ActiveState::EquippedActive)];
        let computation = compute_encumbrance(&equipment_selections, &corpus, 10, SizeCategory::Medium);

        assert!(computation.unresolved_item_ids.is_empty(), "{:?}", computation.unresolved_item_ids);
        assert_eq!(computation.total_carried_weight_lbs, 1.0, "Dogslicer's real WT:1 must be counted");
        assert_eq!(computation.total_carried_cost_gp, 8.0, "Dogslicer's real COST:8 must be counted");
        assert_eq!(
            computation.per_item,
            vec![CarriedItem { item_id: "Dogslicer".to_owned(), weight_lbs: 1.0, cost_gp: Some(8.0) }]
        );
    }

    #[test]
    fn compute_encumbrance_excludes_absent_selections_and_flags_unresolvable_ones() {
        let corpus = corpus_from(FIXTURE_TEXT);
        let equipment_selections = vec![
            selection("item:leather_armor", ActiveState::EquippedActive),
            selection("item:buckler", ActiveState::Absent),
            selection("item:not_a_real_item", ActiveState::EquippedActive),
        ];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 10, SizeCategory::Medium);

        assert_eq!(computation.total_carried_weight_lbs, 15.0, "Absent items must not contribute weight");
        assert_eq!(computation.unresolved_item_ids, vec!["item:not_a_real_item".to_owned()]);
    }

    #[test]
    fn compute_encumbrance_classifies_medium_and_heavy_loads() {
        let corpus = corpus_from(FIXTURE_TEXT);
        // Strength 1: light max 3, medium max 6, heavy max 10. Leather Armor
        // alone (15 lbs) exceeds even the heavy max.
        let equipment_selections = vec![selection("item:leather_armor", ActiveState::EquippedActive)];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 1, SizeCategory::Medium);

        assert_eq!(computation.level, EncumbranceLevel::OverHeavyCapacity);
    }

    /// `AT-34-E3-003` (bucket `M`, EQUIPMENT sub-causes, cycle 6): real
    /// corpus record (`core_rulebook:equipment:horn_of_valhalla_brass`,
    /// `data/corpus/core_rulebook/equipment/magic_items/
    /// horn_of_valhalla_brass.json`'s own ingest token array, verbatim) whose
    /// only `MAGNITUDE_TOKENS` fields are `COST:`/`WT:` -- no `BONUS:`,
    /// `TEMPBONUS:`, or any other chain. A real, already-wired consumer
    /// (`compute_encumbrance`) resolves its weight; the probe must now
    /// observe that.
    #[test]
    fn equipment_key_resolves_a_carried_weight_true_for_a_real_cost_wt_only_record() {
        const HORN_TEXT: &str = "\
Horn of Valhalla (Brass)\tKEY:Horn of Valhalla (Brass)\tTYPE:Magic.Wondrous.Instrument.Wind\tCOST:50000\tWT:2\n";
        let corpus = corpus_from(HORN_TEXT);
        assert!(equipment_key_resolves_a_carried_weight("Horn of Valhalla (Brass)", &corpus));
    }

    /// Negative control: a record with no `WT:` token at all (only
    /// `COST:`) must NOT be reported as resolving a carried weight --
    /// `compute_encumbrance` itself would mark it unresolved (weight is
    /// the required field, cost alone is supplementary), so the probe
    /// widening must not diverge from the real consumer's own gate.
    #[test]
    fn equipment_key_resolves_a_carried_weight_false_when_only_cost_is_present() {
        const COST_ONLY_TEXT: &str =
            "Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";
        let corpus = corpus_from(COST_ONLY_TEXT);
        // This particular real record has WT missing on its own line
        // (equipmods carry no weight of their own); assert the false
        // path against a record that genuinely has COST but no WT.
        const NO_WT_TEXT: &str =
            "Legendary Intelligent Item / Align (CG)\tKEY:Legendary Item ~ Intelligent Item ~ Alignment / Chaotic Good\tTYPE:Mythic.Intelligent.Alignment\tCOST:0\tBONUS:VAR|IntItemAlignment|20\n";
        let corpus2 = corpus_from(NO_WT_TEXT);
        assert!(!equipment_key_resolves_a_carried_weight(
            "Special Quality ~ Masterwork ~ Weapon",
            &corpus
        ));
        assert!(!equipment_key_resolves_a_carried_weight(
            "Legendary Item ~ Intelligent Item ~ Alignment / Chaotic Good",
            &corpus2
        ));
    }

    /// Negative control: an item ID absent from the corpus entirely must
    /// not resolve.
    #[test]
    fn equipment_key_resolves_a_carried_weight_false_when_unresolvable() {
        let corpus = corpus_from(FIXTURE_TEXT);
        assert!(!equipment_key_resolves_a_carried_weight("item:not_a_real_item", &corpus));
    }

    fn corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
        assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
        let source_ref = SourceRef { source_path: "cr_equip_arms_armor.lst".to_string(), line: 1 };
        let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
        for record in result.entries {
            let record: &'static EquipmentRecord = Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }

    fn selection(item_id: &str, state: ActiveState) -> EquipmentSelection {
        EquipmentSelection {
            item_id: item_id.to_owned(),
            equipped_or_active: state == ActiveState::EquippedActive,
            active_state: state,
            applied_modifiers: Vec::new(),
        }
    }

    /// Real verbatim rows mirroring `tests/sd20_equipment_arms_armor.rs`'s
    /// own fixture (same three CRB records, same `WT:` values) -- kept
    /// in-module rather than shared so this file has no dependency on
    /// `tests/**`, which this task does not own.
    const FIXTURE_TEXT: &str = "\
Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Buckler\tKEY:Buckler (Base)\tTYPE:Shield.Buckler\tCOST:5\tWT:5\tACCHECK:-1\tSPELLFAILURE:5\tBONUS:COMBAT|AC|1|TYPE=Shield|PREVAREQ:DisableShieldBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
";


}
