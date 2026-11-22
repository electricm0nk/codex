//! GE06-E2-F2c total saving throws gate.
//!
//! Proves the GE-06 pilot compute surface can compute and explain the first
//! deterministic *total* saving throws (Fighter base save + relevant ability
//! modifier) for the accepted pilot, and refuses to fabricate them when the
//! Fighter level-1 chassis is absent. It does not assert feat-, item-, or
//! condition-based save modifiers, damage, parity, or UI.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::{load, explanation};

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

#[test]
fn computes_total_saves_with_contributors() {
    let input = load(DETERMINISTIC_FIXTURE);

    let computation = compute_pilot_base_chassis(&input);

    // Fighter base save + relevant ability modifier.
    assert_eq!(computation.total_saves.fortitude, 4); // base +2 + CON +2
    assert_eq!(computation.total_saves.reflex, 2); // base +0 + DEX +2
    assert_eq!(computation.total_saves.will, 1); // base +0 + WIS +1

    let fort = explanation(&computation, "defense.total_save.fortitude");
    assert_eq!(fort.value, 4);
    assert!(
        fort.detail.contains("base"),
        "fort detail must cite the base save: {}",
        fort.detail
    );
    assert!(
        fort.detail.contains("Constitution"),
        "fort detail must cite Constitution modifier: {}",
        fort.detail
    );
    assert!(
        fort.detail.contains('4'),
        "fort detail must cite the total: {}",
        fort.detail
    );

    let reflex = explanation(&computation, "defense.total_save.reflex");
    assert_eq!(reflex.value, 2);
    assert!(
        reflex.detail.contains("base"),
        "reflex detail must cite the base save: {}",
        reflex.detail
    );
    assert!(
        reflex.detail.contains("Dexterity"),
        "reflex detail must cite Dexterity modifier: {}",
        reflex.detail
    );
    assert!(
        reflex.detail.contains('2'),
        "reflex detail must cite the total: {}",
        reflex.detail
    );

    let will = explanation(&computation, "defense.total_save.will");
    assert_eq!(will.value, 1);
    assert!(
        will.detail.contains("base"),
        "will detail must cite the base save: {}",
        will.detail
    );
    assert!(
        will.detail.contains("Wisdom"),
        "will detail must cite Wisdom modifier: {}",
        will.detail
    );
    assert!(
        will.detail.contains('1'),
        "will detail must cite the total: {}",
        will.detail
    );

    // The supported deterministic pilot must not raise a claim-blocking diagnostic.
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic pilot should not block claims: {:?}",
        computation.diagnostics
    );
}

#[test]
fn unsupported_chassis_blocks_total_saves() {
    // Replace the Fighter level-1 chassis with a SYNTHETIC unrecognized
    // class. The total saves must refuse to fabricate values, withhold
    // total-save explanations, and emit a claim-blocking diagnostic.
    //
    // The control was Rogue level-1, then Cleric, then Barbarian, then
    // Monk -- each stopped being unsupported the moment `table_class_id`
    // learned it (see ge06_failure_classifier.rs for why Rogue went
    // first). Monk was the LAST real class outside that mapping (v0.6
    // alpha swarm, Monk/Summoner chassis-recognition closure,
    // 2026-07-29), so all 27 base classes are now recognized and no real
    // class can serve here again.
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
            .any(|e| e.id.starts_with("defense.total_save.")),
        "unsupported chassis must withhold total-save explanations: {:?}",
        computation.explanations
    );

    // Ability modifiers remain class-independent and still compute.
    assert_eq!(computation.ability_modifiers.constitution, 2);
}

#[test]
fn wrong_fighter_level_blocks_total_saves() {
    // Total saves are grounded only across the bounded Fighter milestone levels
    // 1-19 (SD13-E5 widened levels 9-10, Weapon Training 2 and the level-10 bonus
    // feat; SD18 widened level 11, Armor Training 3, level 12, a sixth
    // bonus-feat cadence slot, level 13, Weapon Training 3, level 14, a
    // seventh bonus-feat cadence slot and the Bravery magnitude rise, level
    // 15, Armor Training 4, level 16, an eighth bonus-feat cadence slot,
    // level 17, Weapon Training 4 and a fourth weapon-group choice slot,
    // level 18, a ninth bonus-feat cadence slot and a further Bravery
    // magnitude rise, level 19, the Armor Mastery flat-magnitude damage
    // reduction record, and level 20, a tenth bonus-feat cadence slot and
    // the Weapon Mastery grant-only capstone record -- the FINAL level
    // within PF1's 1-20 character-level cap -- into the supported tranche).
    // A Fighter above that bounded tranche (level 21+, which does not exist
    // as a PF1 character level) must be claim-blocked rather than silently
    // computed, just like a non-Fighter class.
    let mutated = DETERMINISTIC_FIXTURE
        .replace("class_level=class:fighter:1", "class_level=class:fighter:21");
    assert!(
        mutated.contains("class_level=class:fighter:21"),
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
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("defense.total_save.")),
        "wrong Fighter level must withhold total-save explanations: {:?}",
        computation.explanations
    );
}
