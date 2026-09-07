//! SD-20 Epic 2-7 -> boundary-contract wiring project, Cycle 5b
//! (`contract:damage_wiring`, per `adaptive-squishing-mccarthy.md`).
//!
//! Before this cycle, `PilotReceipt` never called Epic 6's real
//! `damage_total::resolve_weapon_damage_breakdown` at all -- a weapon's
//! full damage breakdown (base dice, STR modifier, weapon enhancement,
//! critical threat range/multiplier, feat effects) was reachable only via
//! a direct call to the engine, never through the boundary contract. This
//! file proves two things about the fix:
//!
//! (a) **Parity**: `to_pilot_receipt(...).weapon_damage` matches exactly
//!     what a direct call to `resolve_weapon_damage_breakdown(&input,
//!     &corpus, &equipment_effects, str_modifier)` produces, using the
//!     *same* `equipment_effects` (a fresh, independently-filtered
//!     `EquippedActive` slice fed through `compute_equipment_effects`,
//!     mirroring Cycle 4's own parity-test discipline) and the *same*
//!     STR modifier (`receipt.chassis.ability_modifiers.strength`) the
//!     wired path uses internally -- proving the wiring composes the
//!     engine's real output over the exact same inputs, it does not
//!     reinterpret or duplicate them. Exercised with the Fighter tabletop
//!     fixture's real equipped weapon (Longsword), so the parity
//!     assertion is not vacuously true over an empty `Vec`.
//! (b) **Non-weapon exclusion**: the breakdown correctly excludes
//!     equipped non-weapon items (Chain Shirt, the Masterwork weapon
//!     quality record) -- reusing Cycle 5a's own "an equipped item is a
//!     weapon only when `resolve_base_damage_dice` returns `Some`"
//!     identification logic, this test just proves that logic composes
//!     correctly through the newly-wired `to_pilot_receipt` path, not
//!     just in a direct `resolve_weapon_damage_breakdown` call.
//!
//! Per `adaptive-squishing-mccarthy.md`'s Cycle 5b scope: this cycle adds
//! NO new `printed_sheet_cell_map` cells. No summed "damage roll total"
//! formula (base dice + STR + enhancement + feat bonuses combined into
//! one number) exists anywhere in this codebase, and inventing one here
//! would be exactly the fabrication this project's discipline forbids.
//! `receipt.weapon_damage` stays the structured per-weapon breakdown,
//! reachable directly, not flattened into a cell.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection,
};
use codex::rules_core::contract::to_pilot_receipt;
use codex::rules_core::damage_total::resolve_weapon_damage_breakdown;
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// The same real CRB equipment records `tests/sd20_contract_equipment_wiring.rs`'s
/// `FIGHTER_GEAR_FIXTURE_TEXT` uses (Longsword, Chain Shirt, the
/// Masterwork weapon quality) -- the Fighter tabletop fixture's real
/// equipped items.
const FIGHTER_GEAR_FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Resizable.Melee.Martial.OneHanded.Slashing.Sword.BladeHeavy.Weapon Group Blades Heavy\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\tSIZE:M
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
";

fn corpus_with_fighter_gear() -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", FIGHTER_GEAR_FIXTURE_TEXT);
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

fn base_fighter_input(equipment_selections: Vec<EquipmentSelection>) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_damage_wiring_fighter".to_string()),
        source_package_id: "sd20_contract_damage_wiring_fighter".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections,
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn equipped(item_id: &str) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    }
}

/// (a): `receipt.weapon_damage` matches a direct
/// `resolve_weapon_damage_breakdown` call over the same
/// `EquippedActive`-filtered `equipment_effects` and the same STR
/// modifier, with real, non-vacuous Longsword values (STR 16 -> +3 mod).
#[test]
fn pilot_receipt_weapon_damage_matches_direct_call_for_longsword() {
    let input = base_fighter_input(vec![
        equipped("Longsword (Base)"),
        equipped("Chain Shirt (Base)"),
        equipped("Special Quality ~ Masterwork ~ Weapon"),
    ]);
    let corpus = corpus_with_fighter_gear();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // Independently rebuild the same EquippedActive-filtered slice and
    // equipment_effects the wired path computes internally, for the
    // parity check.
    let equipped_active: Vec<EquipmentSelection> = input
        .chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .cloned()
        .collect();
    let equipment_effects = compute_equipment_effects(&equipped_active, &corpus);
    let str_modifier = receipt.chassis.ability_modifiers.strength;
    assert_eq!(str_modifier, 3, "STR 16 must yield a +3 modifier");

    let direct =
        resolve_weapon_damage_breakdown(&input, &corpus, &equipment_effects, str_modifier);

    assert_eq!(
        receipt.weapon_damage, direct,
        "receipt.weapon_damage must match a direct resolve_weapon_damage_breakdown(...) call \
         over the same EquippedActive-filtered equipment_effects and STR modifier exactly"
    );

    // Real values, not vacuous: exactly one weapon (the Longsword) with
    // its real base dice / STR contribution / enhancement / crit stats.
    assert_eq!(
        receipt.weapon_damage.len(),
        1,
        "expected exactly one weapon breakdown (the Longsword)"
    );
    let longsword = &receipt.weapon_damage[0];
    assert_eq!(longsword.weapon_item_id, "Longsword (Base)");

    let base_dice = longsword
        .base_dice
        .as_ref()
        .expect("Longsword must carry a resolved base_dice");
    assert_eq!(base_dice.base_dice.count, 1);
    assert_eq!(base_dice.base_dice.die_size, 8);

    let str_mod = longsword
        .str_modifier
        .as_ref()
        .expect("Longsword must carry a resolved str_modifier");
    assert_eq!(
        str_mod.str_damage_modifier, 3,
        "a OneHanded weapon wielded Primary gets the wielder's full STR modifier"
    );

    let enhancement = longsword
        .weapon_enhancement
        .as_ref()
        .expect("Longsword must carry a resolved weapon_enhancement (even if zero)");
    assert_eq!(
        enhancement.attack_bonus, 1,
        "the equipped Masterwork weapon quality contributes +1 TOHIT"
    );
    assert_eq!(
        enhancement.damage_bonus, 0,
        "Masterwork's BONUS: token affects TOHIT only, not DAMAGE"
    );

    let crit_range = longsword
        .critical_threat_range
        .as_ref()
        .expect("Longsword must carry a resolved critical_threat_range");
    assert_eq!(crit_range.critical_threat_range, (19, 20));

    let crit_mult = longsword
        .critical_multiplier
        .as_ref()
        .expect("Longsword must carry a resolved critical_multiplier");
    assert_eq!(crit_mult.critical_multiplier, 2);
}

/// (b): the equipped Chain Shirt (armor, no `DAMAGE:` token) and the
/// equipped Masterwork weapon-quality record (a quality modifier, not a
/// weapon itself, also no `DAMAGE:` token) are both silently excluded
/// from `receipt.weapon_damage` -- proving Cycle 5a's own weapon
/// -identification logic ("an item is a weapon iff
/// `resolve_base_damage_dice` returns `Some`") composes correctly through
/// the newly-wired `to_pilot_receipt` path.
#[test]
fn pilot_receipt_weapon_damage_excludes_non_weapon_equipped_items() {
    let input = base_fighter_input(vec![
        equipped("Longsword (Base)"),
        equipped("Chain Shirt (Base)"),
        equipped("Special Quality ~ Masterwork ~ Weapon"),
    ]);
    let corpus = corpus_with_fighter_gear();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    assert_eq!(
        receipt.weapon_damage.len(),
        1,
        "only the Longsword should appear; Chain Shirt and the Masterwork \
         weapon-quality record must both be excluded: {:?}",
        receipt.weapon_damage
    );
    assert!(
        !receipt
            .weapon_damage
            .iter()
            .any(|entry| entry.weapon_item_id == "Chain Shirt (Base)"),
        "Chain Shirt (armor, no DAMAGE: token) must not appear in weapon_damage"
    );
    assert!(
        !receipt
            .weapon_damage
            .iter()
            .any(|entry| entry.weapon_item_id == "Special Quality ~ Masterwork ~ Weapon"),
        "the Masterwork weapon-quality record (no DAMAGE: token of its own) \
         must not appear in weapon_damage"
    );
}
