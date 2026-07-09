//! SD13-E5 Wizard level-1 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Paladin, Druid, Cleric, Bard, Sorcerer) already has
//! and Wizard has never had: the base attack bonus and base save progression at Wizard
//! level 1. Both formulas are verified against the PF1 Core Rulebook Wizard class table
//! (d20pfsrd and the legacy Paizo PRD mirror, cross-checked against the raw level 1-6
//! table rows since level 1 alone floors several different fractions to the same value)
//! before writing any code:
//! - base attack bonus: 1/2 BAB (`classlevel / 2`), +0 at level 1 — the SAME shape as
//!   Sorcerer, UNLIKE the 3/4 BAB shared by Rogue/Monk/Druid/Cleric/Bard; Wizard's own
//!   class table reads +0/+1/+1/+2/+2/+3 at levels 1-6, which is the 1/2 BAB
//!   progression, not 3/4 (which would read +0/+1/+2/+3/+3/+4);
//! - base save progression: good Will only, poor Fortitude, poor Reflex
//!   (`classlevel/2+2` for the one good save, `classlevel/3` for the two poor saves),
//!   +0 / +0 / +2 (Fortitude/Reflex/Will) at level 1 — confirmed against the raw Wizard
//!   class table rather than assumed from Sorcerer's own matching pattern.
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's, Monk's, Druid's, Cleric's, Bard's, and
//! Sorcerer's own base-attack/base-save grounding: neither record is wired into
//! `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`, or
//! `compute_combat_baseline`, so the integrated pilot surface still reports a blocked
//! posture on this input.
//!
//! This slice does NOT touch the opposed-school two-slot preparation cost, the prepared
//! spellbook/spells-per-day posture burden, or Wizard level 2+ — those stay
//! named-but-unproven exactly as before. School specialization choice recognition, the
//! specialist bonus slot, Intense Spells, and Force Missile are unaffected.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};

const WIZARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.wizard.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.wizard.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.wizard.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.wizard.base_save.will";

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

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Grounded: the base attack bonus pillar -----

#[test]
fn wizard_level1_grounds_base_attack_bonus() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Wizard class table: 1/2 BAB (classlevel / 2), the same shape as
    // Sorcerer, UNLIKE the 3/4 BAB shared by Rogue/Monk/Druid/Cleric/Bard. At level 1:
    // 1 / 2 = 0.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 0,
        "Wizard level 1 base attack bonus (1/2 BAB) must be +0: {base_attack:?}"
    );
    assert!(
        base_attack.detail.contains("1/2") || base_attack.detail.contains("classlevel / 2"),
        "base attack detail must cite the 1/2-BAB formula: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("standalone"),
        "base attack detail must state it is a standalone record: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("base_attack_bonus")
            || base_attack.detail.contains("compute_combat_baseline"),
        "base attack detail must name the integrated field/seam it is NOT wired into: {}",
        base_attack.detail
    );
}

// ----- Grounded: the base save progression pillar -----

#[test]
fn wizard_level1_grounds_base_save_progression() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Wizard class table: good Will only, poor Fortitude, poor
    // Reflex. At level 1: Will = classlevel/2+2 = 2, Fortitude/Reflex = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 0, "Wizard level 1 poor Fortitude save must be +0");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 0, "Wizard level 1 poor Reflex save must be +0");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 2, "Wizard level 1 good Will save must be +2");

    for (record, label) in [
        (fortitude, "Fortitude"),
        (reflex, "Reflex"),
        (will, "Will"),
    ] {
        assert!(
            record.detail.contains("standalone"),
            "{label} base-save detail must state it is a standalone record: {}",
            record.detail
        );
        assert!(
            record.detail.contains("compute_total_saves"),
            "{label} base-save detail must name compute_total_saves as the seam it is NOT wired \
             into: {}",
            record.detail
        );
    }
}

// ----- The grounded records are standalone: not wired into any integrated total -----

#[test]
fn wizard_level1_base_attack_and_saves_are_not_wired_into_integrated_totals() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // ...but the integrated Fighter-shaped chassis compute path (still unsupported for
    // Wizard) is untouched: no fabricated base_attack_bonus field, and no supported
    // Fighter-style base-attack chassis explanation leaks in.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "the standalone wizard base-attack explanation must not be wired into the integrated \
         base_attack_bonus field"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "wizard baseline must not surface a supported Fighter base-attack chassis explanation"
    );
}

// ----- Existing Wizard grounded pillars and blockers are unaffected -----

#[test]
fn wizard_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The prepared arcane spell-bearing recognition record stays grounded exactly as
    // before.
    assert!(has_explanation(&computation, "class_chassis.spell_baseline.wizard"));

    // Both claim-blocking burdens (school specialization, prepared spellbook posture)
    // still fire; this slice grounds no school-opposition math and no spell math.
    let school_powers = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported")
        .expect("school-power and opposed-school-cost blocker must still fire");
    assert!(school_powers.claim_blocking);
    let prepared_spellbook = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported")
        .expect("prepared spellbook posture blocker must still fire");
    assert!(prepared_spellbook.claim_blocking);
}

// ----- Wizard level 2+ stays out of scope for this slice -----

#[test]
fn wizard_level_2_does_not_gain_base_attack_or_save_grounding_from_this_slice() {
    let level_2 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, BASE_ATTACK_ID),
        "level-2 Wizard must not gain the level-1-bounded base-attack grounding: {:?}",
        computation.explanations
    );
    assert!(
        !has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-2 Wizard must not gain the level-1-bounded base-save grounding: {:?}",
        computation.explanations
    );
    assert!(!has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(!has_explanation(&computation, BASE_SAVE_WILL_ID));
}
