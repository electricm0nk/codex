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

    // floor(score / 2) - 5 against the GE-06 deterministic ability scores. Strength
    // is CG-03's fixed bug: the fixture's chosen strength score (16) is the PRE-bonus
    // base, and the pilot's `choice:human_ability_bonus` selection targets strength,
    // so the PF1 Core Rulebook Standard Human +2 racial bonus must be applied BEFORE
    // the modifier is derived (16 + 2 = 18 -> floor(18/2)-5 = +4), matching the real
    // PCGen oracle output (`STAT:STR|SCORE:16` input, +4 STR modifier output). Every
    // other ability here is unaffected by the racial-bonus choice and keeps its
    // unadjusted floor(score/2)-5 value.
    assert_eq!(computation.ability_modifiers.strength, 4);
    assert_eq!(computation.ability_modifiers.dexterity, 2);
    assert_eq!(computation.ability_modifiers.constitution, 2);
    assert_eq!(computation.ability_modifiers.intelligence, 0);
    assert_eq!(computation.ability_modifiers.wisdom, 1);
    assert_eq!(computation.ability_modifiers.charisma, -1);

    // Each ability modifier has a machine-checkable explanation tied to its value.
    // Strength's explanation now references the racial-bonus-adjusted score (18,
    // the result of applying the Human +2 to the base chosen score of 16) rather
    // than the raw chosen score, since the modifier is derived from the adjusted
    // score. Every other ability's explanation still references its raw chosen
    // score directly, since no racial adjustment touches them.
    let expected = [
        ("ability_modifier.strength", 4, 18),
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

    // The real arithmetic behind strength's adjustment (base 16 + 2 racial = 18)
    // is recorded as its own explicit, machine-checkable explanation -- this is
    // the audit trail the no-stub-mvp doctrine requires for any computed value
    // that isn't a bare pass-through of chosen input.
    let bonus_applied = explanation_value(&computation, "race.human.ability_bonus_applied");
    assert_eq!(bonus_applied.value, 18);
    assert!(bonus_applied.detail.contains("16"));
    assert!(bonus_applied.detail.contains('2'));
    assert!(bonus_applied.detail.contains("18"));
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
    //
    // The negative-control class is `class:monk:1` (was `class:barbarian:1`
    // until the v0.6 alpha swarm's Barbarian rage-execution-engine pass gave
    // Barbarian its own real class_chassis.* computation via the
    // table-driven dispatch path, so Barbarian stopped being an unsupported
    // input -- Monk still is, confirmed against `table_class_id`; was
    // `class:cleric:1` before that, for the same reason). As of the
    // SD13-E3 Rogue chassis-recognition slice, every core-roster class now
    // emits its own bounded, non-fabricating `class_chassis.*` recognition
    // record (mirroring the earlier Wizard-vs-Rogue collision this comment
    // used to document), so this test no longer asserts the absence of every
    // `class_chassis.*` explanation. It asserts the narrower, still-true
    // claim: no FIGHTER-shaped chassis value is fabricated for a
    // non-Fighter input.
    let result = load_character_input_fixture(
        "case_id=non-fighter\n\
         source_package_id=pf1.core_rulebook\n\
         race_id=race:human\n\
         class_level=class:monk:1\n\
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

    // No fabricated Fighter chassis explanation may exist for unsupported input.
    assert!(
        computation.base_attack_bonus == 0
            && !computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.base_attack_bonus"),
        "unsupported class chassis must not fabricate a Fighter base-attack-bonus \
         explanation: {:?}",
        computation.explanations
    );
}
