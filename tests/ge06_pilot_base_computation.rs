//! GE06-E2-F2a base ability modifier and Fighter class chassis computation gate.
//!
//! Proves the rules-core layer can compute and explain only the first base
//! outputs from the GE-06 deterministic pilot input: ability modifiers, Fighter
//! level-1 base attack bonus, and Fighter level-1 base saves. It deliberately
//! does not assert armor class, attack bonus, skills, equipment effects, feat
//! prerequisites, or oracle parity.

use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

fn load_pilot_input() -> codex::rules_core::character_input::CharacterInput {
    let fixture =
        include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid GE-06 deterministic fixture should produce a character input record")
}

fn explanation_value<'a>(
    computation: &'a codex::rules_core::pilot_compute::PilotBaseChassisComputation,
    id: &str,
) -> &'a codex::rules_core::pilot_compute::ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation with id '{id}', got {:?}",
                computation.explanations
            )
        })
}

#[test]
fn computes_ge06_pilot_ability_modifiers_with_explanations() {
    let input = load_pilot_input();

    let computation = compute_pilot_base_chassis(&input);

    // floor(score / 2) - 5 against the GE-06 deterministic ability scores.
    assert_eq!(computation.ability_modifiers.strength, 3);
    assert_eq!(computation.ability_modifiers.dexterity, 2);
    assert_eq!(computation.ability_modifiers.constitution, 2);
    assert_eq!(computation.ability_modifiers.intelligence, 0);
    assert_eq!(computation.ability_modifiers.wisdom, 1);
    assert_eq!(computation.ability_modifiers.charisma, -1);

    // Each ability modifier has a machine-checkable explanation tied to its value
    // and referencing the source score.
    let expected = [
        ("ability_modifier.strength", 3, 16),
        ("ability_modifier.dexterity", 2, 14),
        ("ability_modifier.constitution", 2, 14),
        ("ability_modifier.intelligence", 0, 10),
        ("ability_modifier.wisdom", 1, 12),
        ("ability_modifier.charisma", -1, 8),
    ];
    for (id, value, score) in expected {
        let explanation = explanation_value(&computation, id);
        assert_eq!(explanation.value, value, "value mismatch for {id}");
        assert!(
            explanation.detail.contains(&score.to_string()),
            "explanation {id} should reference source score {score}: {}",
            explanation.detail
        );
    }
}

#[test]
fn computes_ge06_fighter_base_chassis_with_explanations() {
    let input = load_pilot_input();

    let computation = compute_pilot_base_chassis(&input);

    // Fighter level 1 base chassis (class/base values only; no ability modifiers).
    assert_eq!(computation.base_attack_bonus, 1);
    assert_eq!(computation.base_saves.fortitude, 2);
    assert_eq!(computation.base_saves.reflex, 0);
    assert_eq!(computation.base_saves.will, 0);

    let bab = explanation_value(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 1);

    let fort = explanation_value(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(fort.value, 2);
    let reflex = explanation_value(&computation, "class_chassis.base_save.reflex");
    assert_eq!(reflex.value, 0);
    let will = explanation_value(&computation, "class_chassis.base_save.will");
    assert_eq!(will.value, 0);

    // The supported pilot input must not raise a claim-blocking diagnostic.
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported Fighter level 1 input should not block claims: {:?}",
        computation.diagnostics
    );
}

#[test]
fn missing_fighter_chassis_input_produces_claim_blocking_diagnostic() {
    // A loadable but non-Fighter input still yields ability modifiers, but the
    // narrow class chassis must refuse to fabricate Fighter values and must emit
    // a claim-blocking diagnostic instead.
    let result = load_character_input_fixture(
        "case_id=non-fighter\n\
         source_package_id=pf1.core_rulebook\n\
         race_id=race:human\n\
         class_level=class:wizard:1\n\
         ability=strength:16\n\
         ability=dexterity:14\n\
         ability=constitution:14\n\
         ability=intelligence:10\n\
         ability=wisdom:12\n\
         ability=charisma:8\n",
    );
    let input = result
        .character_input
        .expect("non-fighter input should still load as a valid character record");

    let computation = compute_pilot_base_chassis(&input);

    // Ability modifiers are class-independent and still compute.
    assert_eq!(computation.ability_modifiers.strength, 3);

    // The chassis must be claim-blocked rather than silently producing values.
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "missing Fighter level 1 chassis must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );

    // No fabricated Fighter chassis explanations may exist for unsupported input.
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.")),
        "unsupported class chassis must not emit chassis explanations: {:?}",
        computation.explanations
    );
}
