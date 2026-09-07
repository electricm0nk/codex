//! SD-20 Epic 2-7 → boundary-contract wiring project, Cycle 5a
//! (`damage:aggregate_weapons`) — see
//! `~/.claude/plans/adaptive-squishing-mccarthy.md` for the full project
//! plan.
//!
//! `damage_total.rs` landed six narrow `resolve_*` functions (Epic 6),
//! each keyed by `weapon_item_id: &str` except the feat one — but no
//! aggregator loops over a character's equipped items and assembles a
//! full per-weapon breakdown. This is the ONLY cycle in the whole wiring
//! project that doesn't touch `src/rules_core/contract.rs` — it lands the
//! new `resolve_weapon_damage_breakdown` aggregator in `damage_total.rs`
//! itself, matching every other epic's own precedent (aggregation logic
//! lives in the epic module).
//!
//! Identification mechanism: `resolve_base_damage_dice` returning `None`
//! IS the "not a weapon" signal (e.g. armor has no `DAMAGE:` token) — a
//! non-weapon equipped item is silently skipped, not included in the
//! output vec at all. Feat effects are gathered ONCE per character (not
//! per weapon) and attached identically to every weapon in the output.
//!
//! Builds a corpus from real, verbatim tokens already used by
//! `damage_total.rs`'s own unit tests and `tests/sd20_damage_total_parity.rs`
//! / `tests/sd20_damage_weapon_enhancement.rs`: `KEY:Longsword (Base)`
//! (`core_rulebook/cr_equip_arms_armor.lst`) and `KEY:Leather Armor
//! (Base)` (same file) as the non-weapon control record. Feat data comes
//! from the real, already-landed `rules_tables::crb::feats::feat_tables()`
//! catalog (`KEY:Weapon Specialization`, `BONUS:WEAPONPROF=%LIST|DAMAGE|2`,
//! CRB p.137) — no fabricated values anywhere in this test.

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection,
};
use codex::rules_core::damage_total::resolve_weapon_damage_breakdown;
use codex::rules_core::equipment_effects::compute_equipment_effects;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Real verbatim rows: `KEY:Longsword (Base)` (a real weapon, carries
/// `DAMAGE:1d8`) and `KEY:Leather Armor (Base)` (a real non-weapon,
/// carries no `DAMAGE:` token at all) — both copied verbatim from
/// `core_rulebook/cr_equip_arms_armor.lst`, matching the exact tokens
/// `damage_total.rs`'s own `armor_record_has_no_base_dice` unit test
/// already uses.
const FIXTURE_TEXT: &str = "\
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8\tWIELD:OneHanded\n\
Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";

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

fn equipped_selection(item_id: &str) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    }
}

fn character_with(
    equipment_selections: Vec<EquipmentSelection>,
    selected_feats: Vec<String>,
) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_damage_aggregate_weapons".to_string()),
        source_package_id: "sd20_damage_aggregate_weapons".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "fighter".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 12,
                constitution: 13,
                intelligence: 10,
                wisdom: 10,
                charisma: 8,
            },
            selected_feats,
            skill_allocations: Vec::new(),
            equipment_selections,
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn two_equipped_items_one_weapon_one_armor_yields_exactly_one_breakdown() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![
        equipped_selection("Longsword (Base)"),
        equipped_selection("Leather Armor (Base)"),
    ];
    let equipment_effects = compute_equipment_effects(&equipment_selections, &corpus);
    let character = character_with(equipment_selections, Vec::new());

    let breakdown = resolve_weapon_damage_breakdown(&character, &corpus, &equipment_effects, 3);

    assert_eq!(
        breakdown.len(),
        1,
        "the non-weapon Leather Armor (no DAMAGE: token) must be silently skipped, not included: {breakdown:?}"
    );
    assert_eq!(breakdown[0].weapon_item_id, "Longsword (Base)");
    let base_dice = breakdown[0]
        .base_dice
        .as_ref()
        .expect("the real weapon must resolve a base-dice slice");
    assert_eq!(base_dice.weapon_record_key, "Longsword (Base)");
    assert_eq!(base_dice.base_dice.count, 1);
    assert_eq!(base_dice.base_dice.die_size, 8);
}

#[test]
fn selected_feat_with_constant_damage_bonus_shows_up_in_every_weapons_feat_effects() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![equipped_selection("Longsword (Base)")];
    let equipment_effects = compute_equipment_effects(&equipment_selections, &corpus);
    let character = character_with(
        equipment_selections,
        vec!["Weapon Specialization".to_string()],
    );

    let breakdown = resolve_weapon_damage_breakdown(&character, &corpus, &equipment_effects, 3);

    assert_eq!(breakdown.len(), 1);
    assert_eq!(
        breakdown[0].feat_effects.len(),
        1,
        "Weapon Specialization is a real Combat feat with a constant BONUS: token"
    );
    let feat_effect = &breakdown[0].feat_effects[0];
    assert_eq!(feat_effect.feat_key, "Weapon Specialization");
    assert_eq!(
        feat_effect.damage_bonus, 2,
        "CRB p.137: a +2 bonus on all damage rolls you make using the selected weapon"
    );
}
