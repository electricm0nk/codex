//! SD-20 Epic 5 (equipment-effect engine): second CRB equipment category
//! per `scope-draft.md` §1.5's work-unit order — `general` (after
//! `arms_armor`).
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equip_general.lst` (`KEY:Thieves' Tools`,
//! `KEY:Climber's Kit`, `KEY:Backpack`) — the same parser
//! (`parse_equipment_entries`) and IR converter
//! (`convert_equipment_record`) `equipment_resolver.rs`'s own unit tests
//! and the sibling `sd20_equipment_arms_armor.rs` cycle both use — so this
//! test genuinely proves the engine's real `compute_equipment_effects`
//! seam resolves a `CharacterInput`'s `equipment_selections` into real
//! per-item general-equipment stats (skill-check circumstance bonuses),
//! not a hand-rolled fixture.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows copied from `core_rulebook/cr_equip_general.lst`
/// (two masterwork-tool records that each grant a `BONUS:SKILL|...|TYPE=
/// Circumstance` check bonus, and a control record — a plain container —
/// with no `BONUS:` token at all).
const GENERAL_FIXTURE_TEXT: &str = "\
Thieves' Tools\tKEY:Thieves' Tools\tTYPE:Goods.Tools.Thief.ThiefTools\tCOST:30\tWT:1\tBONUS:SKILL|Disable Device|2|TYPE=Circumstance|PRETYPE:1,Masterwork
Climber's Kit\tKEY:Climber's Kit\tTYPE:Goods.Tools.Masterwork.Resizable\tCOST:80\tWT:5\tBONUS:SKILL|Climb|2|TYPE=Circumstance
Backpack\tKEY:Backpack\tTYPE:Goods.Container.General.Resizable\tCONTAINS:UNLIM|Any\tCOST:2\tWT:2
";

fn corpus_from(text: &str) -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_general.lst", text);
    assert!(
        result.diagnostics.is_empty(),
        "fixture text must parse cleanly: {:?}",
        result.diagnostics
    );
    let source_ref = SourceRef {
        lst_file: "cr_equip_general.lst".to_string(),
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
        applied_modifiers: Vec::new(),
    }
}

#[test]
fn general_items_resolve_real_per_item_skill_bonuses_from_the_corpus() {
    let corpus = corpus_from(GENERAL_FIXTURE_TEXT);
    let equipped_items = vec![
        equipped("Thieves' Tools"),
        equipped("Climber's Kit"),
        equipped("Backpack"),
    ];

    let effects = compute_equipment_effects(&equipped_items, &corpus);

    assert_eq!(effects.per_item.len(), 3, "all three selections must resolve");

    let thieves_tools = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Thieves' Tools")
        .expect("Thieves' Tools must resolve");
    assert_eq!(thieves_tools.category, EquipmentCategory::General);
    let bonus = thieves_tools
        .skill_bonus
        .as_ref()
        .expect("Thieves' Tools carries a real BONUS:SKILL|Disable Device|2|TYPE=Circumstance token");
    assert_eq!(bonus.skill, "Disable Device");
    assert_eq!(bonus.bonus, 2);
    assert!(
        thieves_tools.table_cell.is_some(),
        "Thieves' Tools is a real canonical-store entry and must carry a TableCellRef"
    );

    let climbers_kit = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Climber's Kit")
        .expect("Climber's Kit must resolve");
    let bonus = climbers_kit
        .skill_bonus
        .as_ref()
        .expect("Climber's Kit carries a real BONUS:SKILL|Climb|2|TYPE=Circumstance token");
    assert_eq!(bonus.skill, "Climb");
    assert_eq!(bonus.bonus, 2);

    let backpack = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Backpack")
        .expect("Backpack must resolve");
    assert_eq!(
        backpack.skill_bonus, None,
        "a plain container carries no BONUS:SKILL token and must not get a fabricated bonus"
    );
    // A general item never contributes an armor/shield stat — those
    // fields stay honestly absent (not zero) for this category.
    assert_eq!(backpack.armor_class_bonus, None);
    assert_eq!(backpack.max_dex, None);
    assert_eq!(backpack.spell_failure, None);
}

#[test]
fn unresolvable_item_id_is_skipped_not_fabricated() {
    let corpus = corpus_from(GENERAL_FIXTURE_TEXT);
    let equipped_items = vec![equipped("item:does-not-exist-in-this-corpus")];

    let effects = compute_equipment_effects(&equipped_items, &corpus);

    assert!(effects.per_item.is_empty());
}
