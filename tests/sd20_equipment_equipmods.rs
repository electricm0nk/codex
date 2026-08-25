//! SD-20 Epic 5 (equipment-effect engine): fourth and final CRB equipment
//! category per `scope-draft.md` §1.5's work-unit order — `equipmods`
//! (after `arms_armor`, `general`, `magic_items`). Landing this closes
//! Epic 5 (all four CRB equipment categories done).
//!
//! Builds a corpus from real, verbatim tokens copied from
//! `core_rulebook/cr_equipmods.lst` (`KEY:Special Ability ~ +1 ~ Weapon`,
//! `KEY:Material ~ Adamantine ~ Weapon`, `KEY:Material ~ Cloth`,
//! `KEY:Special Quality ~ Wield Size / 1 Step Greater`) — the same parser
//! (`parse_equipment_entries`) and IR converter
//! (`convert_equipment_record`) `equipment_resolver.rs`'s own unit tests
//! and the sibling `sd20_equipment_arms_armor.rs` /
//! `sd20_equipment_general.rs` / `sd20_equipment_magic_items.rs` cycles
//! all use — so this test genuinely proves the engine's real
//! `compute_equipment_effects` seam resolves a `CharacterInput`'s
//! `equipment_selections` into real per-item equipmod stats (weapon
//! to-hit/damage enhancement bonuses), not a hand-rolled fixture.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows copied from `core_rulebook/cr_equipmods.lst`: the
/// canonical "+1 (Enhancement to Weapon)" special ability
/// (`BONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement`), the Adamantine
/// weapon material (`BONUS:WEAPON|TOHIT|1|TYPE=Enhancement` — a
/// different affected-roll shape, proving the affected-roll set is read
/// from the token, not hardcoded), the Wield Size shift (a
/// `BONUS:WEAPON|WIELDCATEGORY|...` chain that is deliberately NOT a
/// weapon-enhancement bonus), and a control record — the Cloth material —
/// with no `BONUS:` token at all.
const EQUIPMODS_FIXTURE_TEXT: &str = "\
+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement
Adamantine\tKEY:Material ~ Adamantine ~ Weapon\tTYPE:BaseMaterial.MasterworkQuality.Weapon\tCOST:3000\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
Wield One Step Greater\tKEY:Special Quality ~ Wield Size / 1 Step Greater\tTYPE:Weapon.Melee\tBONUS:WEAPON|WIELDCATEGORY|-1
Cloth\tKEY:Material ~ Cloth\tTYPE:BaseMaterial.Mundane.Ammunition.Armor.Shield.Weapon.Instruments.Tools.Goods\tCOST:0
";

fn corpus_from(text: &str) -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equipmods.lst", text);
    assert!(
        result.diagnostics.is_empty(),
        "fixture text must parse cleanly: {:?}",
        result.diagnostics
    );
    let source_ref = SourceRef {
        lst_file: "cr_equipmods.lst".to_string(),
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
fn equipmods_resolve_real_per_item_weapon_enhancement_bonuses_from_the_corpus() {
    let corpus = corpus_from(EQUIPMODS_FIXTURE_TEXT);
    let equipped_items = vec![
        equipped("Special Ability ~ +1 ~ Weapon"),
        equipped("Material ~ Adamantine ~ Weapon"),
        equipped("Special Quality ~ Wield Size / 1 Step Greater"),
        equipped("Material ~ Cloth"),
    ];

    let effects = compute_equipment_effects(&equipped_items, &corpus);

    assert_eq!(effects.per_item.len(), 4, "all four selections must resolve");

    let plus_one_weapon = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Special Ability ~ +1 ~ Weapon")
        .expect("Special Ability ~ +1 ~ Weapon must resolve");
    assert_eq!(plus_one_weapon.category, EquipmentCategory::Equipmods);
    let bonus = plus_one_weapon
        .weapon_enhancement_bonus
        .as_ref()
        .expect("+1 (Enhancement to Weapon) carries a real BONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement token");
    assert_eq!(bonus.tohit_bonus, Some(1));
    assert_eq!(bonus.damage_bonus, Some(1));
    assert!(
        plus_one_weapon.table_cell.is_some(),
        "Special Ability ~ +1 ~ Weapon is a real canonical-store entry and must carry a TableCellRef"
    );

    let adamantine = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Material ~ Adamantine ~ Weapon")
        .expect("Material ~ Adamantine ~ Weapon must resolve");
    let bonus = adamantine
        .weapon_enhancement_bonus
        .as_ref()
        .expect("Adamantine carries a real BONUS:WEAPON|TOHIT|1|TYPE=Enhancement token");
    assert_eq!(bonus.tohit_bonus, Some(1));
    assert_eq!(bonus.damage_bonus, None);

    let wield_size = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Special Quality ~ Wield Size / 1 Step Greater")
        .expect("Special Quality ~ Wield Size / 1 Step Greater must resolve");
    assert_eq!(
        wield_size.weapon_enhancement_bonus, None,
        "a WIELDCATEGORY shift is not a weapon-enhancement bonus and must not get a fabricated bonus"
    );

    let cloth = effects
        .per_item
        .iter()
        .find(|item| item.equipment_record_key == "Material ~ Cloth")
        .expect("Material ~ Cloth must resolve");
    assert_eq!(
        cloth.weapon_enhancement_bonus, None,
        "Material ~ Cloth carries no BONUS:WEAPON token and must not get a fabricated bonus"
    );
    // An equipmod never contributes an armor/shield stat, a
    // general-category skill-check bonus, or a magic-items ability-score
    // bonus from this category's own resolver — those fields stay
    // honestly absent (not zero) here.
    assert_eq!(cloth.armor_class_bonus, None);
    assert_eq!(cloth.max_dex, None);
    assert_eq!(cloth.spell_failure, None);
    assert_eq!(cloth.skill_bonus, None);
    assert_eq!(cloth.ability_bonus, None);
}

#[test]
fn unresolvable_item_id_is_skipped_not_fabricated() {
    let corpus = corpus_from(EQUIPMODS_FIXTURE_TEXT);
    let equipped_items = vec![equipped("item:does-not-exist-in-this-corpus")];

    let effects = compute_equipment_effects(&equipped_items, &corpus);

    assert!(effects.per_item.is_empty());
}
