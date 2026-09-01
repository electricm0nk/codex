//! GE06-E2-F2d selected deterministic skill modifiers gate.
//!
//! Proves the GE-06 pilot compute surface can compute and explain the first
//! bounded selected skill modifiers — Climb, Intimidate, and Swim — for the
//! accepted deterministic pilot, applying the already-grounded Chain Shirt
//! armor-check penalty only to the armor-check skills (Climb, Swim), and refuses
//! to fabricate them when the selected-skill posture or the deterministic Chain
//! Shirt posture is absent or widened. It does not assert a broad skill engine,
//! feat/racial/item-skill bonuses, encumbrance, parity, or UI.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::{load, explanation};

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

#[test]
fn computes_selected_skill_modifiers_with_contributors() {
    let input = load(DETERMINISTIC_FIXTURE);

    let computation = compute_pilot_base_chassis(&input);

    // Climb: rank 1 + STR +4 (CG-03 fix: the Human +2 racial bonus is now applied to
    // the base 16 before the modifier is derived) + class-skill +3 + Chain Shirt
    // armor-check -2 = 6.
    assert_eq!(computation.selected_skill_modifiers.climb, 6);
    // Intimidate: rank 1 + CHA -1 + class-skill +3 = 3 (no armor-check; CHA-based;
    // unaffected by the Strength fix).
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    // Swim: rank 1 + STR +4 + class-skill +3 + Chain Shirt armor-check -2 = 6.
    assert_eq!(computation.selected_skill_modifiers.swim, 6);

    let climb = explanation(&computation, "skill.selected_modifier.climb");
    assert_eq!(climb.value, 6);
    assert!(
        climb.detail.contains("rank"),
        "climb detail must cite the rank allocation: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains("Strength"),
        "climb detail must cite the Strength modifier: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains("class-skill"),
        "climb detail must cite the class-skill bonus: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains("armor-check"),
        "climb detail must cite the Chain Shirt armor-check penalty: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains('6'),
        "climb detail must cite the total: {}",
        climb.detail
    );

    let intimidate = explanation(&computation, "skill.selected_modifier.intimidate");
    assert_eq!(intimidate.value, 3);
    assert!(
        intimidate.detail.contains("rank"),
        "intimidate detail must cite the rank allocation: {}",
        intimidate.detail
    );
    assert!(
        intimidate.detail.contains("Charisma"),
        "intimidate detail must cite the Charisma modifier: {}",
        intimidate.detail
    );
    assert!(
        intimidate.detail.contains("class-skill"),
        "intimidate detail must cite the class-skill bonus: {}",
        intimidate.detail
    );
    // Intimidate is not an armor-check skill; it must not claim an armor-check penalty.
    assert!(
        !intimidate.detail.contains("armor-check"),
        "intimidate detail must not cite an armor-check penalty: {}",
        intimidate.detail
    );
    assert!(
        intimidate.detail.contains('3'),
        "intimidate detail must cite the total: {}",
        intimidate.detail
    );

    let swim = explanation(&computation, "skill.selected_modifier.swim");
    assert_eq!(swim.value, 6);
    assert!(
        swim.detail.contains("rank"),
        "swim detail must cite the rank allocation: {}",
        swim.detail
    );
    assert!(
        swim.detail.contains("Strength"),
        "swim detail must cite the Strength modifier: {}",
        swim.detail
    );
    assert!(
        swim.detail.contains("class-skill"),
        "swim detail must cite the class-skill bonus: {}",
        swim.detail
    );
    assert!(
        swim.detail.contains("armor-check"),
        "swim detail must cite the Chain Shirt armor-check penalty: {}",
        swim.detail
    );
    assert!(
        swim.detail.contains('6'),
        "swim detail must cite the total: {}",
        swim.detail
    );

    // The supported deterministic pilot must not raise a claim-blocking diagnostic.
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic pilot should not block claims: {:?}",
        computation.diagnostics
    );
}

#[test]
fn missing_selected_skill_allocation_blocks_skill_modifiers() {
    // Remove the Swim rank allocation. The selected-skill surface must refuse to
    // fabricate any selected-skill totals, withhold all selected-skill
    // explanations, and emit a claim-blocking diagnostic.
    let mutated = DETERMINISTIC_FIXTURE.replace("skill=skill:swim:1\n", "");
    assert!(
        !mutated.contains("skill=skill:swim:1"),
        "test setup should have removed the Swim allocation"
    );
    let input = load(&mutated);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "missing selected skill allocation must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("skill.selected_modifier.")),
        "missing selected skill allocation must withhold selected-skill explanations: {:?}",
        computation.explanations
    );
    assert_eq!(computation.selected_skill_modifiers.climb, 0);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 0);
    assert_eq!(computation.selected_skill_modifiers.swim, 0);
}

#[test]
fn widened_selected_skill_allocation_blocks_skill_modifiers() {
    // Widen beyond this slice by adding an out-of-scope skill allocation. The
    // narrow selected-skill surface must refuse rather than silently extend.
    let mutated = DETERMINISTIC_FIXTURE.replace(
        "skill=skill:swim:1\n",
        "skill=skill:swim:1\nskill=skill:stealth:1\n",
    );
    assert!(
        mutated.contains("skill=skill:stealth:1"),
        "test setup should have widened the skill allocations"
    );
    let input = load(&mutated);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "widened selected skill allocation must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("skill.selected_modifier.")),
        "widened selected skill allocation must withhold selected-skill explanations: {:?}",
        computation.explanations
    );
}

#[test]
fn absent_chain_shirt_blocks_skill_modifiers() {
    // Break the deterministic Chain Shirt posture that grounds the Climb/Swim
    // armor-check penalty. The selected-skill surface must refuse to fabricate
    // totals that pretend the equipment-effect surface is grounded.
    let mutated = DETERMINISTIC_FIXTURE.replace(
        "equipment=item:chain_shirt:equipped_worn_active",
        "equipment=item:chain_shirt:absent",
    );
    assert!(
        mutated.contains("equipment=item:chain_shirt:absent"),
        "test setup should have broken the Chain Shirt posture"
    );
    let input = load(&mutated);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "absent Chain Shirt must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("skill.selected_modifier.")),
        "absent Chain Shirt must withhold selected-skill explanations: {:?}",
        computation.explanations
    );
}

#[test]
fn unsupported_chassis_blocks_skill_modifiers() {
    // Replace the Fighter level-1 chassis with a Monk level-1 chassis. The
    // selected-skill surface is grounded only on the Fighter class posture
    // and must refuse rather than fabricate Fighter class-skill bonuses.
    // The negative control is a SYNTHETIC class id, not a real class. It
    // was Rogue level-1, then Cleric, then Barbarian, then Monk -- each
    // stopped being unsupported the moment `table_class_id` learned it
    // (see ge06_failure_classifier.rs for why Rogue went first). Monk was
    // the LAST real class outside that mapping (v0.6 alpha swarm,
    // Monk/Summoner chassis-recognition closure, 2026-07-29), so all 27
    // base classes are now recognized and no real class can serve here
    // again.
    let mutated = DETERMINISTIC_FIXTURE
        .replace("class_level=class:fighter:1", "class_level=class:not_a_real_pf1_class:1");
    assert!(
        mutated.contains("class_level=class:not_a_real_pf1_class:1"),
        "test setup should have mutated the class chassis"
    );
    let input = load(&mutated);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "unsupported chassis must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("skill.selected_modifier.")),
        "unsupported chassis must withhold selected-skill explanations: {:?}",
        computation.explanations
    );
}
