//! SD13-E5 Ranger level-1 base attack bonus and base save progression proof.
//!
//! Grounds the one foundational martial pillar every other class row in this matrix
//! (Fighter, Barbarian, Monk, Rogue, Paladin, and by now Druid/Cleric/Bard/Sorcerer/
//! Wizard) already has and Ranger has never had: the base attack bonus and base save
//! progression at Ranger level 1. Both formulas are verified against the PF1 Core
//! Rulebook Ranger class table (d20pfsrd and the legacy Paizo PRD mirror, reading the
//! raw level 1-5 table rows directly and cross-checking the level 4/5 base-attack-bonus
//! values to disambiguate the fraction, since level 1 alone floors a 3/4 progression to
//! +0 while a full progression already shows +1) before writing any code:
//! - base attack bonus: FULL BAB, the same formula shape as Fighter/Barbarian/Paladin
//!   (`classlevel`), +1 at level 1 (level 4: +4, level 5: +5 -- confirms full, not 3/4,
//!   which would show +3/+3 at those levels);
//! - base save progression: good Fortitude, good Reflex, poor Will
//!   (`classlevel/2+2` for the two good saves, `classlevel/3` for the one poor save),
//!   +2 / +2 / +0 at level 1 (level 4: +4/+4/+1, confirming the same formula shape).
//!
//! Both are grounded as flat, standalone `ComputationExplanation` records mirroring the
//! exact "standalone, not wired into the integrated `PilotBaseChassisComputation`"
//! idiom already used for Barbarian's/Monk's/Druid's/Cleric's/Bard's/Sorcerer's/
//! Wizard's own base-attack/base-save grounding: neither record is wired into
//! `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`, or
//! `compute_combat_baseline`, so the integrated pilot surface still reports a blocked
//! posture on this input.
//!
//! This slice does NOT touch the combat-style pillar's actual bonus-feat grant, the
//! ranger spell burden, or Ranger level 2+ -- those stay named-but-unproven exactly as
//! before. Track, Favored Enemy, and the combat-style level-gate absence record
//! (grounded by earlier SD13-E3/E5 slices) are unaffected.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{SupportState, seeded_sd13_e1_f1_current_truth};

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const BASE_ATTACK_ID: &str = "class_chassis.ranger.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.ranger.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.ranger.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.ranger.base_save.will";

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
fn ranger_level1_grounds_base_attack_bonus() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Ranger class table: full BAB, same formula shape as
    // Fighter/Barbarian/Paladin (classlevel). At level 1: 1.
    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 1,
        "Ranger level 1 base attack bonus (full BAB) must be +1: {base_attack:?}"
    );
    assert!(
        base_attack.detail.contains("full") || base_attack.detail.contains("classlevel = 1"),
        "base attack detail must cite the full-BAB formula: {}",
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
fn ranger_level1_grounds_base_save_progression() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Ranger class table: good Fortitude, good Reflex, poor Will.
    // At level 1: Fortitude/Reflex = classlevel/2+2 = 2, Will = classlevel/3 = 0.
    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 2, "Ranger level 1 good Fortitude save must be +2");
    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 2, "Ranger level 1 good Reflex save must be +2");
    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 0, "Ranger level 1 poor Will save must be +0");

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
fn ranger_level1_base_attack_and_saves_are_not_wired_into_integrated_totals() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The grounded standalone records exist...
    assert!(has_explanation(&computation, BASE_ATTACK_ID));
    assert!(has_explanation(&computation, BASE_SAVE_FORTITUDE_ID));
    assert!(has_explanation(&computation, BASE_SAVE_REFLEX_ID));
    assert!(has_explanation(&computation, BASE_SAVE_WILL_ID));

    // ...but the integrated Fighter-shaped chassis compute path (still unsupported
    // for Ranger) is untouched: no fabricated base_attack_bonus field, and no
    // supported Fighter-style base-attack chassis explanation leaks in.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "the standalone ranger base-attack explanation must not be wired into the integrated \
         base_attack_bonus field"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "ranger baseline must not surface a supported Fighter base-attack chassis explanation"
    );
}

// ----- Existing Ranger grounded pillars and blockers are unaffected -----

#[test]
fn ranger_level1_base_attack_and_saves_do_not_disturb_existing_pillars_or_blockers() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Track, the favored-enemy flat surface, and the combat-style level-gate absence
    // stay grounded exactly as before.
    assert!(has_explanation(&computation, "class_chassis.ranger.track"));
    assert!(has_explanation(
        &computation,
        "class_chassis.ranger.favored_enemy_skill_bonus"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.ranger.favored_enemy_attack_damage_bonus"
    ));
    assert!(has_explanation(
        &computation,
        "class_chassis.ranger.level_gate.combat_style"
    ));

    // Both claim-blocking burdens (the combined hybrid class-feature burden, the
    // later hybrid spell burden) still fire; this slice grounds no combat-style feat
    // mechanics and no spell math.
    let feature_blocker = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.hybrid.ranger.unsupported")
        .expect("hybrid ranger class-feature blocker must still fire");
    assert!(feature_blocker.claim_blocking);
    let spell_blocker = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.hybrid.ranger.unsupported")
        .expect("hybrid ranger spell blocker must still fire");
    assert!(spell_blocker.claim_blocking);
}

// ----- Negative control: Ranger level 2 stays unrecognized (out of scope this slice) -----

#[test]
fn ranger_level_2_stays_unrecognized_by_this_slice() {
    // Ranger level 2+ progression is deliberately out of scope for this slice (the
    // combat-style bonus-feat grant, the favored-enemy conditional-application engine,
    // and the ranger spell burden all remain unproven). The level-1-only gate
    // (`is_single_class_ranger_level1`) is unchanged, so level 2 does not gain this
    // slice's base-attack/base-save grounding.
    let level_2 = RANGER_FIXTURE.replace("class:ranger:1", "class:ranger:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, BASE_ATTACK_ID),
        "level-2 Ranger must not gain the bounded level-1-only base-attack grounding: {:?}",
        computation.explanations
    );
    assert!(
        !has_explanation(&computation, BASE_SAVE_FORTITUDE_ID),
        "level-2 Ranger must not gain the bounded level-1-only base-save grounding: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_paladin_do_not_gain_ranger_base_attack_or_save_grounding() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&fighter_computation, BASE_SAVE_FORTITUDE_ID));

    let paladin_fixture = RANGER_FIXTURE.replace("class:ranger:1", "class:paladin:1");
    let paladin = load(&paladin_fixture);
    let paladin_computation = compute_pilot_base_chassis(&paladin);
    assert!(!has_explanation(&paladin_computation, BASE_ATTACK_ID));
    assert!(!has_explanation(&paladin_computation, BASE_SAVE_FORTITUDE_ID));
}

// ----- Control plane: the matrix row's note names the newly grounded pillar -----

#[test]
fn matrix_ranger_row_note_names_base_attack_and_base_save_as_grounded() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");

    assert_eq!(ranger.support_state, SupportState::Partial);
    assert_ne!(ranger.support_state, SupportState::Supported);
    for token in ["base attack", "base save", "standalone"] {
        assert!(
            ranger.blocker_or_lossiness_note.contains(token),
            "ranger blocker note must name '{token}' now that base attack/base save are \
             grounded: {}",
            ranger.blocker_or_lossiness_note
        );
    }
    // The still-unproven burdens stay named.
    for token in ["combat-style", "spell"] {
        assert!(
            ranger.blocker_or_lossiness_note.contains(token),
            "ranger blocker note must still name the unproven '{token}' burden: {}",
            ranger.blocker_or_lossiness_note
        );
    }
}
