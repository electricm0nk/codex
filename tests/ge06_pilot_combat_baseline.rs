//! GE06-E2-F2b baseline combat values gate.
//!
//! Proves the rules-core pilot compute surface can compute and explain only the
//! first deterministic combat-facing totals from the accepted pilot loadout:
//! baseline melee attack bonus for the Longsword-primary loadout and baseline
//! armor class for the Chain Shirt / DEX / Dodge / no-shield posture. It refuses
//! to fabricate totals when the exact deterministic posture is not met. It does
//! not assert damage, active Power Attack math, initiative, skills, or parity.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

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

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

#[test]
fn computes_baseline_melee_attack_bonus_with_contributors() {
    let input = load(DETERMINISTIC_FIXTURE);

    let computation = compute_pilot_base_chassis(&input);

    // Fighter BAB (+1) + STR (+3) + Weapon Focus Longsword (+1); Power Attack inactive (+0).
    assert_eq!(computation.baseline_melee_attack_bonus, 5);

    let attack = explanation(&computation, "combat.baseline_melee_attack_bonus");
    assert_eq!(attack.value, 5);
    let detail = attack.detail.as_str();
    assert!(
        detail.contains("Fighter"),
        "attack detail must cite Fighter BAB: {detail}"
    );
    assert!(
        detail.contains("Strength"),
        "attack detail must cite Strength modifier: {detail}"
    );
    assert!(
        detail.contains("Weapon Focus"),
        "attack detail must cite Weapon Focus: {detail}"
    );
    assert!(
        detail.contains("Longsword"),
        "attack detail must cite the Longsword: {detail}"
    );
    assert!(
        detail.contains("Power Attack"),
        "attack detail must cite Power Attack: {detail}"
    );
    assert!(
        detail.contains("inactive"),
        "attack detail must note Power Attack inactive posture: {detail}"
    );
}

#[test]
fn computes_baseline_armor_class_with_contributors() {
    let input = load(DETERMINISTIC_FIXTURE);

    let computation = compute_pilot_base_chassis(&input);

    // 10 + Chain Shirt (+4) + DEX (+2, within MAXDEX 4) + Dodge (+1) + no shield (+0).
    assert_eq!(computation.baseline_armor_class, 17);

    let ac = explanation(&computation, "defense.baseline_armor_class");
    assert_eq!(ac.value, 17);
    let detail = ac.detail.as_str();
    assert!(
        detail.contains("10"),
        "ac detail must cite base 10: {detail}"
    );
    assert!(
        detail.contains("Chain Shirt"),
        "ac detail must cite Chain Shirt armor bonus: {detail}"
    );
    assert!(
        detail.contains("Dexterity"),
        "ac detail must cite Dexterity contribution: {detail}"
    );
    assert!(
        detail.contains("MAXDEX:4"),
        "ac detail must cite the MAXDEX:4 limit: {detail}"
    );
    assert!(
        detail.contains("Dodge"),
        "ac detail must cite Dodge bonus: {detail}"
    );
    assert!(
        detail.contains("shield"),
        "ac detail must cite the absent shield posture: {detail}"
    );

    // The supported deterministic posture must not raise a claim-blocking diagnostic.
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic posture should not block claims: {:?}",
        computation.diagnostics
    );
}

#[test]
fn unsupported_loadout_posture_blocks_combat_totals() {
    // Equip the shield (the deterministic baseline requires it absent). The combat
    // baseline must refuse to fabricate totals and must withhold combat/defense
    // explanations while still emitting a claim-blocking diagnostic.
    let mutated = DETERMINISTIC_FIXTURE.replace(
        "equipment=item:shield:absent",
        "equipment=item:shield:equipped_worn_active",
    );
    assert!(
        mutated.contains("equipment=item:shield:equipped_worn_active"),
        "test setup should have mutated the shield posture"
    );
    let input = load(&mutated);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "unsupported combat posture must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation.explanations.iter().any(|e| {
            e.id == "combat.baseline_melee_attack_bonus" || e.id == "defense.baseline_armor_class"
        }),
        "unsupported posture must withhold combat/defense explanations: {:?}",
        computation.explanations
    );

    // The prior F2a outputs remain available even when combat is blocked.
    assert_eq!(computation.base_attack_bonus, 1);
    assert_eq!(computation.ability_modifiers.strength, 3);
}

#[test]
fn wrong_fighter_level_blocks_combat_totals() {
    // The deterministic baseline is grounded only across the bounded Fighter
    // milestone levels 1-12 (SD13-E5 widened levels 9-10, Weapon Training 2 and
    // the level-10 bonus feat; SD18 widened level 11, Armor Training 3, and
    // level 12, a sixth bonus-feat cadence slot, into the supported tranche).
    // A Fighter above that bounded tranche (level 13+, beyond the SD-13/SD-18
    // level-12 progression matrix) must be treated as an unsupported posture,
    // not silently computed, even though every other loadout/feat/choice
    // condition still holds.
    let mutated = DETERMINISTIC_FIXTURE
        .replace("class_level=class:fighter:1", "class_level=class:fighter:13");
    assert!(
        mutated.contains("class_level=class:fighter:13"),
        "test setup should have mutated the Fighter level"
    );
    let input = load(&mutated);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "wrong Fighter level must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation.explanations.iter().any(|e| {
            e.id == "combat.baseline_melee_attack_bonus" || e.id == "defense.baseline_armor_class"
        }),
        "wrong Fighter level must withhold combat/defense explanations: {:?}",
        computation.explanations
    );
}
