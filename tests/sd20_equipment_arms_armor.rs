//! SD-20 Epic 5 (equipment-effect engine): first CRB equipment category
//! per `scope-draft.md` §1.5's work-unit order — `arms_armor`.
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equip_arms_armor.lst` (`KEY:Leather Armor (Base)`,
//! `KEY:Buckler (Base)`, `KEY:Longsword (Base)`) — the same parser
//! (`parse_equipment_entries`) and IR converter
//! (`convert_equipment_record`) `equipment_resolver.rs`'s own unit tests
//! use — so this test genuinely proves the engine's real
//! `compute_equipment_effects` seam resolves a `CharacterInput`'s
//! `equipment_selections` into real per-item armor/shield stats and a
//! correctly-aggregated `EquipmentEffects`, not a hand-rolled fixture.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows copied from `core_rulebook/cr_equip_arms_armor.lst`
/// (armor block, shield block, and a weapon-block control record with no
/// armor-defining tokens at all).
const ARMS_ARMOR_FIXTURE_TEXT: &str = "\
Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\tBONUS:COMBAT|AC|-1|TYPE=Armor|PRETYPE:1,EQMOD=Special Quality ~ Broken ~ Armor
Buckler\tKEY:Buckler (Base)\tTYPE:Shield.Buckler\tCOST:5\tWT:5\tACCHECK:-1\tSPELLFAILURE:5\tBONUS:COMBAT|AC|1|TYPE=Shield|PREVAREQ:DisableShieldBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
";

fn corpus_from(text: &str) -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
    assert!(
        result.diagnostics.is_empty(),
        "fixture text must parse cleanly: {:?}",
        result.diagnostics
    );
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

fn equipped(item_id: &str) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
    }
}

#[test]
fn arms_armor_items_resolve_real_per_item_stats_from_the_corpus() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);
    let equipped_items = vec![
        equipped("Leather Armor (Base)"),
        equipped("Buckler (Base)"),
        equipped("Longsword (Base)"),
    ];

    let effects = compute_equipment_effects(&equipped_items, &corpus);

    assert_eq!(effects.per_item.len(), 3, "all three selections must resolve");

    let leather = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Leather Armor (Base)")
        .expect("Leather Armor (Base) must resolve");
    assert_eq!(leather.category, EquipmentCategory::ArmsArmor);
    assert_eq!(leather.armor_class_bonus, Some(2));
    assert_eq!(leather.max_dex, Some(6));
    assert_eq!(leather.spell_failure, Some(10.0));
    assert!(
        leather.table_cell.is_some(),
        "Leather Armor (Base) is a real canonical-store entry and must carry a TableCellRef"
    );

    let buckler = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Buckler (Base)")
        .expect("Buckler (Base) must resolve");
    assert_eq!(buckler.armor_class_bonus, Some(1));
    assert_eq!(buckler.max_dex, None, "the real Buckler (Base) record carries no MAXDEX: token");
    assert_eq!(buckler.spell_failure, Some(5.0));

    let longsword = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Longsword (Base)")
        .expect("Longsword (Base) must resolve");
    assert_eq!(longsword.armor_class_bonus, None, "a weapon carries no armor/shield AC bonus");
    assert_eq!(longsword.max_dex, None);
    assert_eq!(longsword.spell_failure, None);

    // Aggregate: armor and shield AC bonuses stack (PF1 rule); max_dex_cap
    // is the tightest cap among items that carry one (only the armor
    // here); spell_failure_chance stacks additively across armor+shield.
    assert_eq!(effects.armor_class_delta, 3, "2 (armor) + 1 (shield) stack");
    assert_eq!(effects.max_dex_cap, Some(6), "only the armor carries a MAXDEX: cap");
    assert_eq!(
        effects.spell_failure_chance,
        Some(15.0),
        "10 (armor) + 5 (shield) stack additively"
    );
}

#[test]
fn unresolvable_item_id_is_skipped_not_fabricated() {
    let corpus = corpus_from(ARMS_ARMOR_FIXTURE_TEXT);
    let equipped_items = vec![equipped("item:does-not-exist-in-this-corpus")];

    let effects = compute_equipment_effects(&equipped_items, &corpus);

    assert!(effects.per_item.is_empty());
    assert_eq!(effects.armor_class_delta, 0);
    assert_eq!(effects.max_dex_cap, None);
    assert_eq!(effects.spell_failure_chance, None);
}
