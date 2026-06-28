//! GE06-E4-F1 pilot view-model contract test.
//!
//! Tests that the pilot view-model adapter projects the real GE-06 pilot receipt
//! into a machine-checkable UI-consumer boundary. It must preserve pilot identity,
//! real computed snapshot values when computed, explicit blocked posture with real
//! diagnostics when blocked, the primary failure owner, and explanation payloads
//! or stable references for surfaced values.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{HeadlessReceiptStatus, build_pilot_headless_receipt};
use codex::rules_core::pilot_failure::FailureClassifier;
use codex::rules_core::pilot_view_model::PilotViewModel;

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

#[test]
fn computed_receipt_projects_real_snapshot() {
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);
    let classifier = FailureClassifier::new(&receipt);

    let view_model = PilotViewModel::from_receipt(&receipt, &classifier);

    // Case and source-package identity are preserved.
    assert_eq!(view_model.case_id.as_deref(), Some("pf1-crb-human-fighter-level1"));
    assert_eq!(view_model.source_package_id, "pf1.core_rulebook");

    // Status is Computed.
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);

    // Primary owner from classifier is preserved.
    assert_eq!(
        view_model.primary_owner,
        codex::rules_core::pilot_failure::PrimaryOwner::OracleGap
    );

    // Snapshot is present (not None) when computed.
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("computed receipt must have a snapshot");

    // Real snapshot values are preserved from the receipt computation.
    assert_eq!(snapshot.ability_modifiers.strength, 3);
    assert_eq!(snapshot.ability_modifiers.dexterity, 2);
    assert_eq!(snapshot.ability_modifiers.constitution, 2);
    assert_eq!(snapshot.ability_modifiers.intelligence, 0);
    assert_eq!(snapshot.ability_modifiers.wisdom, 1);
    assert_eq!(snapshot.ability_modifiers.charisma, -1);

    assert_eq!(snapshot.base_attack_bonus, 1);
    assert_eq!(snapshot.base_saves.fortitude, 2);
    assert_eq!(snapshot.base_saves.reflex, 0);
    assert_eq!(snapshot.base_saves.will, 0);

    assert_eq!(snapshot.baseline_melee_attack_bonus, 5);
    assert_eq!(snapshot.baseline_armor_class, 17);

    assert_eq!(snapshot.total_saves.fortitude, 4);
    assert_eq!(snapshot.total_saves.reflex, 2);
    assert_eq!(snapshot.total_saves.will, 1);

    assert_eq!(snapshot.selected_skill_modifiers.climb, 5);
    assert_eq!(snapshot.selected_skill_modifiers.intimidate, 3);
    assert_eq!(snapshot.selected_skill_modifiers.swim, 5);

    // Explanations are preserved.
    assert!(!snapshot.explanations.is_empty());
    for id in [
        "ability_modifier.strength",
        "class_chassis.base_attack_bonus",
        "combat.baseline_melee_attack_bonus",
        "defense.baseline_armor_class",
        "defense.total_save.fortitude",
        "skill.selected_modifier.climb",
    ] {
        assert!(
            snapshot.explanations.iter().any(|e| e.id == id),
            "snapshot must preserve explanation id '{id}'"
        );
    }

    // Diagnostics are preserved (should be empty or non-blocking for computed case).
    assert!(
        !snapshot.diagnostics.iter().any(|d| d.claim_blocking),
        "computed snapshot must not have claim-blocking diagnostics"
    );
}

#[test]
fn blocked_receipt_projects_explicit_blocked_posture() {
    // Mutate the deterministic fixture to a blocked state by replacing Fighter
    // class with Rogue class, breaking the expected class chassis.
    let mutated =
        DETERMINISTIC_FIXTURE.replace("class_level=class:fighter:1", "class_level=class:rogue:1");
    assert!(
        mutated.contains("class_level=class:rogue:1"),
        "test setup should have mutated the class chassis"
    );
    let input = load(&mutated);
    let receipt = build_pilot_headless_receipt(&input);
    let classifier = FailureClassifier::new(&receipt);

    let view_model = PilotViewModel::from_receipt(&receipt, &classifier);

    // Case and source-package identity are preserved even on blocked path.
    assert_eq!(view_model.case_id.as_deref(), Some("pf1-crb-human-fighter-level1"));
    assert_eq!(view_model.source_package_id, "pf1.core_rulebook");

    // Status is explicitly Blocked.
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);

    // Primary owner is preserved (will be EngineFlaw for blocked status).
    assert_eq!(
        view_model.primary_owner,
        codex::rules_core::pilot_failure::PrimaryOwner::EngineFlaw
    );

    // Snapshot is None (no faux success values).
    assert_eq!(
        view_model.snapshot, None,
        "blocked view-model must not have a faux success snapshot"
    );

    // Real diagnostics are preserved.
    assert!(!view_model.diagnostics.is_empty());
    assert!(
        view_model.diagnostics.iter().any(|d| d.claim_blocking),
        "blocked view-model must preserve claim-blocking diagnostics"
    );
}
