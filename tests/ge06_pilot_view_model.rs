//! GE06-E4-F1 pilot view-model contract proof.
//!
//! Proves that the bounded GE-06 pilot view-model adapter projects the merged
//! headless receipt into a machine-checkable UI-consumer contract. The contract
//! preserves real pilot identity, computed snapshot values, explicit blocked
//! posture, primary failure ownership, explanation payloads, and diagnostics
//! without fabricating a success snapshot for blocked receipts.

use codex::rules_core::pilot_compute::{HeadlessReceiptStatus, build_pilot_headless_receipt};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
mod common;
use common::load;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn has_explanation(view_model: &PilotViewModel, id: &str) -> bool {
    view_model
        .explanations
        .iter()
        .any(|explanation| explanation.id == id)
}

#[test]
fn supported_deterministic_pilot_projects_computed_view_model() {
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    let view_model = PilotViewModel::from_receipt(&receipt);

    assert_eq!(
        view_model.case_id.as_deref(),
        Some("pf1-crb-human-fighter-level1")
    );
    assert_eq!(view_model.source_package_id, "pf1.core_rulebook");
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
    assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
    assert_eq!(view_model.explanations, receipt.computation.explanations);
    assert_eq!(view_model.diagnostics, receipt.computation.diagnostics);

    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("computed receipt must yield a snapshot");

    // CG-03 fix: the Human ability-bonus choice's +2 racial Strength adjustment is now
    // applied before ability modifiers are derived, raising Strength from +3 to +4.
    assert_eq!(snapshot.ability_modifiers.strength, 4);
    assert_eq!(snapshot.ability_modifiers.dexterity, 2);
    assert_eq!(snapshot.ability_modifiers.constitution, 2);
    assert_eq!(snapshot.ability_modifiers.intelligence, 0);
    assert_eq!(snapshot.ability_modifiers.wisdom, 1);
    assert_eq!(snapshot.ability_modifiers.charisma, -1);

    assert_eq!(snapshot.base_attack_bonus, 1);
    assert_eq!(snapshot.base_saves.fortitude, 2);
    assert_eq!(snapshot.base_saves.reflex, 0);
    assert_eq!(snapshot.base_saves.will, 0);

    assert_eq!(snapshot.combat.baseline_melee_attack_bonus, 6);
    assert_eq!(snapshot.defense.baseline_armor_class, 17);
    assert_eq!(snapshot.defense.total_save.fortitude, 4);
    assert_eq!(snapshot.defense.total_save.reflex, 2);
    assert_eq!(snapshot.defense.total_save.will, 1);

    assert_eq!(snapshot.skill.selected_modifier.climb, 6);
    assert_eq!(snapshot.skill.selected_modifier.intimidate, 3);
    assert_eq!(snapshot.skill.selected_modifier.swim, 6);

    for id in [
        "ability_modifier.strength",
        "class_chassis.base_attack_bonus",
        "combat.baseline_melee_attack_bonus",
        "defense.baseline_armor_class",
        "defense.total_save.fortitude",
        "skill.selected_modifier.climb",
    ] {
        assert!(
            has_explanation(&view_model, id),
            "computed view model must expose explanation id '{id}', got {:?}",
            view_model.explanations
        );
    }

    assert!(
        !view_model
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.claim_blocking),
        "supported deterministic pilot must not block claims: {:?}",
        view_model.diagnostics
    );
}

#[test]
fn computed_view_model_exposes_explicit_human_race_seam() {
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    let view_model = PilotViewModel::from_receipt(&receipt);

    // The explicit Human race-seam explanations must propagate through the bounded
    // view-model projection.
    for id in [
        "race.human.ability_bonus_target",
        "race.human.bonus_feat_grant",
    ] {
        assert!(
            has_explanation(&view_model, id),
            "computed view model must expose race-seam explanation id '{id}', got {:?}",
            view_model.explanations
        );
    }

    // The bounded Human race-semantics note propagates as a non-claim-blocking
    // diagnostic and must not fabricate a blocked posture.
    assert!(
        view_model
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "view model must carry the bounded, non-claim-blocking Human race note: {:?}",
        view_model.diagnostics
    );
    assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
    assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
}

#[test]
fn blocked_receipt_projects_blocked_view_model_without_faux_success_snapshot() {
    // Cleric level-1 as the unsupported negative control (was Rogue level-1 --
    // see ge06_failure_classifier.rs for why that stopped being unsupported).
    let mutated =
        DETERMINISTIC_FIXTURE.replace("class_level=class:fighter:1", "class_level=class:cleric:1");
    assert!(
        mutated.contains("class_level=class:cleric:1"),
        "test setup should have mutated the class chassis"
    );
    let input = load(&mutated);
    let receipt = build_pilot_headless_receipt(&input);

    let view_model = PilotViewModel::from_receipt(&receipt);

    assert_eq!(
        view_model.case_id.as_deref(),
        Some("pf1-crb-human-fighter-level1")
    );
    assert_eq!(view_model.source_package_id, "pf1.core_rulebook");
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert_eq!(view_model.snapshot, None);
    assert_eq!(view_model.explanations, receipt.computation.explanations);
    assert_eq!(view_model.diagnostics, receipt.computation.diagnostics);
    assert!(
        view_model
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.claim_blocking),
        "blocked view model must preserve claim-blocking diagnostics: {:?}",
        view_model.diagnostics
    );
}
