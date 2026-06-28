//! GE06-E2-F3 end-to-end headless receipt path gate.
//!
//! Proves the GE-06 pilot compute surface can emit one bounded, library-first,
//! headless receipt for the accepted deterministic pilot: it preserves case and
//! source-package identity, reports a simple status that distinguishes computed
//! evidence from a blocked posture, carries the already-grounded computed outputs
//! and their explanations proven across F2a/F2b/F2c/F2d, and surfaces claim-blocking
//! diagnostics without fabricating a success state when the integrated path is
//! broken.
//!
//! This is headless computed evidence only. It is not oracle-checked parity, and
//! this slice adds no CLI, no dependency, and no broad reporting framework. It does
//! not assert any new computed value beyond what prior slices already grounded.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, PilotHeadlessReceipt, build_pilot_headless_receipt,
};

const DETERMINISTIC_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

fn load(fixture: &str) -> CharacterInput {
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

fn has_explanation(receipt: &PilotHeadlessReceipt, id: &str) -> bool {
    receipt.computation.explanations.iter().any(|e| e.id == id)
}

#[test]
fn supported_deterministic_pilot_yields_computed_receipt() {
    let input = load(DETERMINISTIC_FIXTURE);

    let receipt = build_pilot_headless_receipt(&input);

    // Case and source-package identity are preserved from the loaded input.
    assert_eq!(receipt.case_id.as_deref(), Some("pf1-crb-human-fighter-level1"));
    assert_eq!(receipt.source_package_id, "pf1.core_rulebook");

    // The integrated path produced evidence, not a blocker.
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);

    // The receipt exposes the already-grounded bounded outputs without changing
    // their values (F2a ability mods, F2a chassis, F2b combat, F2c total saves,
    // F2d selected skills).
    let computation = &receipt.computation;
    assert_eq!(computation.ability_modifiers.strength, 3);
    assert_eq!(computation.ability_modifiers.dexterity, 2);
    assert_eq!(computation.ability_modifiers.constitution, 2);
    assert_eq!(computation.ability_modifiers.intelligence, 0);
    assert_eq!(computation.ability_modifiers.wisdom, 1);
    assert_eq!(computation.ability_modifiers.charisma, -1);

    assert_eq!(computation.base_attack_bonus, 1);
    assert_eq!(computation.base_saves.fortitude, 2);
    assert_eq!(computation.base_saves.reflex, 0);
    assert_eq!(computation.base_saves.will, 0);

    assert_eq!(computation.baseline_melee_attack_bonus, 5);
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.total_saves.fortitude, 4);
    assert_eq!(computation.total_saves.reflex, 2);
    assert_eq!(computation.total_saves.will, 1);

    assert_eq!(computation.selected_skill_modifiers.climb, 5);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 5);

    // The integrated receipt preserves access to representative explanation ids
    // grounded by prior slices.
    for id in [
        "ability_modifier.strength",
        "class_chassis.base_attack_bonus",
        "combat.baseline_melee_attack_bonus",
        "defense.baseline_armor_class",
        "defense.total_save.fortitude",
        "skill.selected_modifier.climb",
    ] {
        assert!(
            has_explanation(&receipt, id),
            "computed receipt must expose explanation id '{id}', got {:?}",
            receipt.computation.explanations
        );
    }

    // The supported deterministic fixture yields no claim-blocking diagnostic.
    assert!(
        !receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "supported deterministic pilot must not block claims: {:?}",
        receipt.computation.diagnostics
    );
}

#[test]
fn broken_prerequisite_yields_blocked_receipt() {
    // Mutate one supported prerequisite in memory: replace the Fighter level-1
    // chassis with a Rogue level-1 chassis. The integrated receipt must report a
    // blocked posture with claim-blocking diagnostics rather than a counterfeit
    // success state.
    let mutated =
        DETERMINISTIC_FIXTURE.replace("class_level=class:fighter:1", "class_level=class:rogue:1");
    assert!(
        mutated.contains("class_level=class:rogue:1"),
        "test setup should have mutated the class chassis"
    );
    let input = load(&mutated);

    let receipt = build_pilot_headless_receipt(&input);

    // Identity is still preserved on the blocker receipt.
    assert_eq!(receipt.case_id.as_deref(), Some("pf1-crb-human-fighter-level1"));
    assert_eq!(receipt.source_package_id, "pf1.core_rulebook");

    // The integrated path is blocked, not computed.
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
    assert_ne!(receipt.status, HeadlessReceiptStatus::Computed);

    // At least one claim-blocking diagnostic is preserved.
    assert!(
        receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "blocked receipt must preserve at least one claim-blocking diagnostic: {:?}",
        receipt.computation.diagnostics
    );
}
