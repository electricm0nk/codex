//! SD-20 Epic 1 (boundary contract): `CharacterInput` types — cycle 1.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.1 and
//! `technical-design.md` §1.1, the boundary contract's "Inputs" section
//! names a `CharacterInput` shape for each of three canonical
//! permutations: brand-new, mid-build, and multiclass. This is the first
//! Epic-1 work-unit (`CharacterInput` types land first, per the loop
//! instruction's Step 2), landing in `src/rules_core/contract.rs` as
//! `CharacterInputPermutation` plus the `classify_character_input`
//! function that operationalizes the contract clause: given an existing
//! (already-landed, SD-19-shaped) `CharacterInput` value, which of the
//! three canonical permutations does it represent.
//!
//! This cycle does not touch `PilotReceipt` shapes or the printed-sheet
//! cell map (those are later Epic-1 work-units per Step 2) and does not
//! add a wire-fixture JSON (that is the fourth Epic-1 work-unit).

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput,
    ChosenCharacterState, EquipmentSelection, SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::contract::{classify_character_input, CharacterInputPermutation};

fn empty_input(class_levels: Vec<CharacterClassLevel>) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_character_input".to_string()),
        source_package_id: "sd20_contract_character_input".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels,
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn brand_new_single_class_level_one_no_choices_classifies_brand_new() {
    let input = empty_input(vec![CharacterClassLevel {
        class_id: "fighter".to_string(),
        level: 1,
    }]);

    assert_eq!(
        classify_character_input(&input),
        CharacterInputPermutation::BrandNew
    );
}

#[test]
fn no_class_levels_at_all_classifies_brand_new() {
    let input = empty_input(Vec::new());

    assert_eq!(
        classify_character_input(&input),
        CharacterInputPermutation::BrandNew
    );
}

#[test]
fn single_class_with_a_selected_feat_classifies_mid_build() {
    let mut input = empty_input(vec![CharacterClassLevel {
        class_id: "fighter".to_string(),
        level: 1,
    }]);
    input.chosen.selected_feats.push("Power Attack".to_string());

    assert_eq!(
        classify_character_input(&input),
        CharacterInputPermutation::MidBuild
    );
}

#[test]
fn single_class_above_level_one_classifies_mid_build() {
    let input = empty_input(vec![CharacterClassLevel {
        class_id: "wizard".to_string(),
        level: 5,
    }]);

    assert_eq!(
        classify_character_input(&input),
        CharacterInputPermutation::MidBuild
    );
}

#[test]
fn single_class_with_equipment_skills_and_spells_classifies_mid_build() {
    let mut input = empty_input(vec![CharacterClassLevel {
        class_id: "cleric".to_string(),
        level: 1,
    }]);
    input.chosen.skill_allocations.push(SkillAllocation {
        skill_id: "heal".to_string(),
        ranks: 1,
    });
    input.chosen.equipment_selections.push(EquipmentSelection {
        item_id: "mace-heavy".to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    });
    input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Bless".to_string(),
        source_class_id: "cleric".to_string(),
        acquisition_mode: AcquisitionMode::Prepared,
    });
    input.chosen.selected_choices.push(SelectedChoice {
        choice_set_id: "domain".to_string(),
        selection_id: "war".to_string(),
    });

    assert_eq!(
        classify_character_input(&input),
        CharacterInputPermutation::MidBuild
    );
}

#[test]
fn two_class_levels_classifies_multiclass_even_at_level_one() {
    let input = empty_input(vec![
        CharacterClassLevel {
            class_id: "fighter".to_string(),
            level: 1,
        },
        CharacterClassLevel {
            class_id: "wizard".to_string(),
            level: 1,
        },
    ]);

    assert_eq!(
        classify_character_input(&input),
        CharacterInputPermutation::Multiclass
    );
}

#[test]
fn multiclass_with_choices_recorded_still_classifies_multiclass() {
    let mut input = empty_input(vec![
        CharacterClassLevel {
            class_id: "fighter".to_string(),
            level: 2,
        },
        CharacterClassLevel {
            class_id: "wizard".to_string(),
            level: 1,
        },
    ]);
    input.chosen.selected_feats.push("Toughness".to_string());

    assert_eq!(
        classify_character_input(&input),
        CharacterInputPermutation::Multiclass
    );
}
