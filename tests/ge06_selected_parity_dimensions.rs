//! GE06-E3-F1 selected parity-dimension adapter proof.
//!
//! Proves that the selected-parity-dimensions adapter projects the merged GE06-E2-F3
//! headless receipt into a machine-checkable selected-dimension surface for the
//! mandatory pilot dimensions only. The adapter preserves current new-system values
//! or references and keeps the claim-tier floor at `Computed` without implying
//! oracle-checked parity or normalization.

use codex::oracle_validation::selected_parity_dimensions::{
    ClaimTierFloor, SelectedParityDimensions,
};
use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn load(fixture: &str) -> codex::rules_core::character_input::CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

#[test]
fn supported_deterministic_pilot_yields_selected_dimensions() {
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    // The adapter projects the merged receipt into selected parity dimensions.
    let dimensions = SelectedParityDimensions::from_receipt(&receipt);

    // The adapter emits exactly the mandatory selected pilot dimensions.
    let dimension_ids: Vec<&str> = dimensions
        .dimensions
        .iter()
        .map(|d| d.id.as_str())
        .collect();
    assert_eq!(
        dimension_ids.len(),
        9,
        "Expected exactly 9 selected dimensions"
    );
    assert!(dimension_ids.contains(&"character.identity"));
    assert!(dimension_ids.contains(&"combat.baseline_melee_attack_bonus"));
    assert!(dimension_ids.contains(&"defense.baseline_armor_class"));
    assert!(dimension_ids.contains(&"defense.total_save.fortitude"));
    assert!(dimension_ids.contains(&"defense.total_save.reflex"));
    assert!(dimension_ids.contains(&"defense.total_save.will"));
    assert!(dimension_ids.contains(&"skill.selected_modifier.climb"));
    assert!(dimension_ids.contains(&"skill.selected_modifier.intimidate"));
    assert!(dimension_ids.contains(&"skill.selected_modifier.swim"));

    // Find each dimension by ID and verify its value.
    let find_dimension = |id: &str| {
        dimensions
            .dimensions
            .iter()
            .find(|d| d.id == id)
            .unwrap_or_else(|| panic!("dimension {id} should exist"))
    };

    // character.identity preserves pilot identity from the merged receipt.
    let char_identity = find_dimension("character.identity");
    assert_eq!(
        char_identity.value_string,
        Some("pf1-crb-human-fighter-level1".to_string())
    );
    assert_eq!(char_identity.source_package_id, "pf1.core_rulebook");

    // The remaining selected dimensions preserve the already-grounded new-system values.
    let melee_bonus = find_dimension("combat.baseline_melee_attack_bonus");
    assert_eq!(melee_bonus.value_i16, Some(5));

    let armor_class = find_dimension("defense.baseline_armor_class");
    assert_eq!(armor_class.value_i16, Some(17));

    let fort_save = find_dimension("defense.total_save.fortitude");
    assert_eq!(fort_save.value_i16, Some(4));

    let reflex_save = find_dimension("defense.total_save.reflex");
    assert_eq!(reflex_save.value_i16, Some(2));

    let will_save = find_dimension("defense.total_save.will");
    assert_eq!(will_save.value_i16, Some(1));

    let climb_mod = find_dimension("skill.selected_modifier.climb");
    assert_eq!(climb_mod.value_i16, Some(5));

    let intimidate_mod = find_dimension("skill.selected_modifier.intimidate");
    assert_eq!(intimidate_mod.value_i16, Some(3));

    let swim_mod = find_dimension("skill.selected_modifier.swim");
    assert_eq!(swim_mod.value_i16, Some(5));

    // The carrier keeps its claim-tier floor at `Computed` and does not imply
    // oracle-checked parity.
    assert_eq!(dimensions.claim_tier_floor, ClaimTierFloor::Computed);
}
