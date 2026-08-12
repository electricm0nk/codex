//! SD-20 Epic 5 (equipment-effect engine): third CRB equipment category
//! per `scope-draft.md` §1.5's work-unit order — `magic_items` (after
//! `arms_armor`, `general`).
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equip_magic_items.lst` (`KEY:Belt of Giant Strength
//! +2`, `KEY:Belt of Incredible Dexterity +2`, `KEY:Bag of Holding (Type
//! I)`) — the same parser (`parse_equipment_entries`) and IR converter
//! (`convert_equipment_record`) `equipment_resolver.rs`'s own unit tests
//! and the sibling `sd20_equipment_arms_armor.rs` / `sd20_equipment_general.rs`
//! cycles both use — so this test genuinely proves the engine's real
//! `compute_equipment_effects` seam resolves a `CharacterInput`'s
//! `equipment_selections` into real per-item magic-item stats (ability-
//! score enhancement bonuses), not a hand-rolled fixture.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows copied from `core_rulebook/cr_equip_magic_items.lst`
/// (two wondrous belts that each grant a `BONUS:STAT|...|TYPE=Enhancement`
/// ability-score bonus, and a control record — a bag of holding — with no
/// `BONUS:` token at all).
const MAGIC_ITEMS_FIXTURE_TEXT: &str = "\
Belt of Giant Strength +2\tKEY:Belt of Giant Strength +2\tTYPE:Magic.Wondrous.Belt\tCOST:4000\tWT:1\tBONUS:STAT|STR|2|TYPE=Enhancement
Belt of Incredible Dexterity +2\tKEY:Belt of Incredible Dexterity +2\tTYPE:Magic.Wondrous.Belt\tCOST:4000\tWT:1\tBONUS:STAT|DEX|2|TYPE=Enhancement
Bag of Holding (Type I)\tKEY:Bag of Holding (Type I)\tTYPE:Magic.Wondrous.Container\tCOST:2500\tWT:15
";

fn corpus_from(text: &str) -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
    assert!(
        result.diagnostics.is_empty(),
        "fixture text must parse cleanly: {:?}",
        result.diagnostics
    );
    let source_ref = SourceRef {
        lst_file: "cr_equip_magic_items.lst".to_string(),
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
fn magic_items_resolve_real_per_item_ability_bonuses_from_the_corpus() {
    let corpus = corpus_from(MAGIC_ITEMS_FIXTURE_TEXT);
    let equipped_items = vec![
        equipped("Belt of Giant Strength +2"),
        equipped("Belt of Incredible Dexterity +2"),
        equipped("Bag of Holding (Type I)"),
    ];

    let effects = compute_equipment_effects(&equipped_items, &corpus);

    assert_eq!(effects.per_item.len(), 3, "all three selections must resolve");

    let belt_of_strength = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Belt of Giant Strength +2")
        .expect("Belt of Giant Strength +2 must resolve");
    assert_eq!(belt_of_strength.category, EquipmentCategory::MagicItems);
    let bonus = belt_of_strength
        .ability_bonus
        .as_ref()
        .expect("Belt of Giant Strength +2 carries a real BONUS:STAT|STR|2|TYPE=Enhancement token");
    assert_eq!(bonus.ability, "STR");
    assert_eq!(bonus.bonus, 2);
    assert!(
        belt_of_strength.table_cell.is_some(),
        "Belt of Giant Strength +2 is a real canonical-store entry and must carry a TableCellRef"
    );

    let belt_of_dexterity = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Belt of Incredible Dexterity +2")
        .expect("Belt of Incredible Dexterity +2 must resolve");
    let bonus = belt_of_dexterity
        .ability_bonus
        .as_ref()
        .expect("Belt of Incredible Dexterity +2 carries a real BONUS:STAT|DEX|2|TYPE=Enhancement token");
    assert_eq!(bonus.ability, "DEX");
    assert_eq!(bonus.bonus, 2);

    let bag_of_holding = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Bag of Holding (Type I)")
        .expect("Bag of Holding (Type I) must resolve");
    assert_eq!(
        bag_of_holding.ability_bonus, None,
        "a bag of holding carries no BONUS:STAT token and must not get a fabricated bonus"
    );
    // A magic item never contributes an armor/shield stat or a
    // general-category skill-check bonus from this category's own
    // resolver — those fields stay honestly absent (not zero) here.
    assert_eq!(bag_of_holding.armor_class_bonus, None);
    assert_eq!(bag_of_holding.max_dex, None);
    assert_eq!(bag_of_holding.spell_failure, None);
    assert_eq!(bag_of_holding.skill_bonus, None);
}

#[test]
fn unresolvable_item_id_is_skipped_not_fabricated() {
    let corpus = corpus_from(MAGIC_ITEMS_FIXTURE_TEXT);
    let equipped_items = vec![equipped("item:does-not-exist-in-this-corpus")];

    let effects = compute_equipment_effects(&equipped_items, &corpus);

    assert!(effects.per_item.is_empty());
}
