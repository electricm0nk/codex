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

use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, PilotHeadlessReceipt, build_pilot_headless_receipt,
};
mod common;
use common::load;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn has_explanation(receipt: &PilotHeadlessReceipt, id: &str) -> bool {
    receipt.computation.explanations.iter().any(|e| e.id == id)
}

#[test]
fn supported_deterministic_pilot_yields_computed_receipt() {
    let input = load(DETERMINISTIC_FIXTURE);

    let receipt = build_pilot_headless_receipt(&input);

    // Case and source-package identity are preserved from the loaded input.
    assert_eq!(
        receipt.case_id.as_deref(),
        Some("pf1-crb-human-fighter-level1")
    );
    assert_eq!(receipt.source_package_id, "pf1.core_rulebook");

    // The integrated path produced evidence, not a blocker.
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);

    // The receipt exposes the already-grounded bounded outputs without changing
    // their values (F2a ability mods, F2a chassis, F2b combat, F2c total saves,
    // F2d selected skills).
    let computation = &receipt.computation;
    // CG-03 fix: the Human ability-bonus choice's +2 racial Strength adjustment is now
    // applied before ability modifiers are derived, raising Strength from +3 to +4.
    assert_eq!(computation.ability_modifiers.strength, 4);
    assert_eq!(computation.ability_modifiers.dexterity, 2);
    assert_eq!(computation.ability_modifiers.constitution, 2);
    assert_eq!(computation.ability_modifiers.intelligence, 0);
    assert_eq!(computation.ability_modifiers.wisdom, 1);
    assert_eq!(computation.ability_modifiers.charisma, -1);

    assert_eq!(computation.base_attack_bonus, 1);
    assert_eq!(computation.base_saves.fortitude, 2);
    assert_eq!(computation.base_saves.reflex, 0);
    assert_eq!(computation.base_saves.will, 0);

    assert_eq!(computation.baseline_melee_attack_bonus, 6);
    assert_eq!(computation.baseline_armor_class, 17);

    assert_eq!(computation.total_saves.fortitude, 4);
    assert_eq!(computation.total_saves.reflex, 2);
    assert_eq!(computation.total_saves.will, 1);

    assert_eq!(computation.selected_skill_modifiers.climb, 6);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 6);

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
fn computed_receipt_exposes_explicit_human_race_seam() {
    let input = load(DETERMINISTIC_FIXTURE);

    let receipt = build_pilot_headless_receipt(&input);

    // The receipt must make the grounded Human race seam explicit rather than leaving
    // it an incidental side effect of the numeric outputs.
    assert!(
        has_explanation(&receipt, "race.human.ability_bonus_target"),
        "computed receipt must expose the Human ability-bonus race explanation, got {:?}",
        receipt.computation.explanations
    );
    assert!(
        has_explanation(&receipt, "race.human.bonus_feat_grant"),
        "computed receipt must expose the Human bonus-feat race explanation, got {:?}",
        receipt.computation.explanations
    );

    let ability = receipt
        .computation
        .explanations
        .iter()
        .find(|e| e.id == "race.human.ability_bonus_target")
        .expect("ability-bonus race explanation present");
    // Derived strictly from the chosen human_ability_bonus -> ability:strength selection
    // and the already-computed, now-correctly-racial-bonus-adjusted Strength modifier
    // (+4, CG-03 fix).
    assert_eq!(ability.value, 4);
    assert!(
        ability.detail.contains("human_ability_bonus") && ability.detail.contains("strength"),
        "ability-bonus detail must name the chosen Human ability-bonus seam: {}",
        ability.detail
    );

    let bonus_feat = receipt
        .computation
        .explanations
        .iter()
        .find(|e| e.id == "race.human.bonus_feat_grant")
        .expect("bonus-feat race explanation present");
    // Derived strictly from the chosen human_bonus_feat -> feat:dodge selection and the
    // already-grounded Dodge armor-class contribution (+1).
    assert_eq!(bonus_feat.value, 1);
    assert!(
        bonus_feat.detail.contains("human_bonus_feat") && bonus_feat.detail.contains("Dodge"),
        "bonus-feat detail must name the chosen Human bonus-feat Dodge seam: {}",
        bonus_feat.detail
    );

    // Making broader Human race semantics absent must be explicit but non-blocking, so
    // the deterministic pilot still reports a computed receipt.
    assert!(
        receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "receipt must carry a bounded, non-claim-blocking Human race-semantics note: {:?}",
        receipt.computation.diagnostics
    );
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
}

#[test]
fn broken_prerequisite_yields_blocked_receipt() {
    // Mutate one supported prerequisite in memory: replace the Fighter level-1
    // chassis with a Cleric level-1 chassis. The integrated receipt must report a
    // blocked posture with claim-blocking diagnostics rather than a counterfeit
    // success state. (Was Rogue level-1 -- see ge06_failure_classifier.rs for why
    // that stopped being an unsupported negative control.)
    let mutated =
        DETERMINISTIC_FIXTURE.replace("class_level=class:fighter:1", "class_level=class:cleric:1");
    assert!(
        mutated.contains("class_level=class:cleric:1"),
        "test setup should have mutated the class chassis"
    );
    let input = load(&mutated);

    let receipt = build_pilot_headless_receipt(&input);

    // Identity is still preserved on the blocker receipt.
    assert_eq!(
        receipt.case_id.as_deref(),
        Some("pf1-crb-human-fighter-level1")
    );
    assert_eq!(receipt.source_package_id, "pf1.core_rulebook");

    // The integrated path is blocked, not computed.
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

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
