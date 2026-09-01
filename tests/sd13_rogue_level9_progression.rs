//! SD13-E5 Rogue level-9 progression grounding proof.
//!
//! Widens the accepted Rogue level-1..level-8 martial-skirmisher baseline
//! (`tests/sd13_rogue_pillars.rs` and the level-2 through level-8
//! progression files, most recently `tests/sd13_rogue_level8_progression.rs`)
//! to Rogue level 9 — the first level-9 slice in the tranche, opening the
//! level-9 band the same way the Rogue slice opened the level-8 band.
//! Mirrors the sibling-class level-range-gate idiom
//! (`supported_rogue_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_ROGUE_LEVEL = 9`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Rogue class table) were read directly before
//! writing any code or test:
//!
//! - level 9 base attack bonus is +6 (`9 * 3 / 4 = 6`, the Rogue's 3/4-BAB
//!   progression, numerically unchanged from level 8 — an integer-division
//!   coincidence, not a stalled formula; the class table's own "+6/+1"
//!   iterative-attack notation is not modeled anywhere in this codebase,
//!   only the flat base value) and base saves are +3 Fortitude (poor,
//!   `9 / 3 = 3`, genuinely risen from +2), +6 Reflex (good,
//!   `9 / 2 + 2 = 6`, numerically unchanged from level 8, another
//!   integer-division coincidence), and +3 Will (poor, `9 / 3 = 3`,
//!   genuinely risen from +2) — confirmed by the same formulas already
//!   grounded at levels 1-8, not re-derived.
//! - the PF1 Core Rulebook Rogue class table's level-9 "Special" column
//!   reads "Sneak attack +5d6, trap sense +3" (verified independently
//!   against both primary sources, checked rather than assumed away) —
//!   BOTH entries are tier-rises on already-grounded formula pillars, not
//!   new class features: the sneak attack die count genuinely rises to 5
//!   (the pre-existing `(level + 1) / 2` formula: `(9 + 1) / 2 = 5`, up
//!   from 4 at levels 7-8, matching the odd-level rise cadence) and Trap
//!   Sense's flat magnitude genuinely rises to +3 (the pre-existing
//!   `level / 3` formula: `9 / 3 = 3`, up from +2 at levels 6-8) — so no
//!   new pillar record is grounded at level 9, only the existing pillars
//!   are widened. Notably level 9 is NOT a rogue-talent level (talents
//!   land at 2/4/6/8/10...), so nothing new is left unproven for the
//!   talent tree here either.
//! - Trapfinding stays 4 (`max(9 / 2, 1) = 4`, numerically unchanged from
//!   level 8, an integer-division coincidence); Evasion, Uncanny Dodge,
//!   and Improved Uncanny Dodge all stay granted, not re-derived.
//!
//! It deliberately does not touch the rogue-talent tree, any
//! check-execution engine, or sneak-attack damage application (all stay
//! named-but-unproven, unchanged from levels 1-8), and it does not ground
//! Rogue level 10+. It also preserves the accepted Rogue level-1..level-8
//! truth (unchanged), the Fighter negative control, and the multiclass
//! negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level8_sd13_deterministic_input.txt");

const ROGUE_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level9_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus at level 9 -----

#[test]
fn rogue_level9_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(ROGUE_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Rogue level 9 3/4-BAB progression (9 * 3 / 4) must equal 6 — numerically unchanged \
         from level 8, an integer-division coincidence: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 9 (good Reflex, poor Fortitude/Will) -----

#[test]
fn rogue_level9_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(ROGUE_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 3,
        "Rogue level 9 poor Fortitude (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 6, "Rogue level 9 good Reflex (9/2+2) must equal 6");

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 3,
        "Rogue level 9 poor Will (9/3) must equal 3, genuinely risen from 2 at level 8"
    );
}

// ----- Sneak attack die count genuinely rises to 5 at level 9 -----

#[test]
fn rogue_level9_sneak_attack_die_count_rises_to_five() {
    let input = load(ROGUE_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 5,
        "Rogue level 9 sneak attack die count ((9 + 1) / 2) must equal 5 (5d6), genuinely \
         risen from 4 at levels 7-8, matching the odd-level rise cadence: {}",
        sneak_attack.detail
    );
}

// ----- Trap Sense genuinely rises to +3 at level 9 -----

#[test]
fn rogue_level9_trap_sense_rises_to_three() {
    let input = load(ROGUE_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 3,
        "Rogue level 9 Trap Sense (9 / 3) must equal +3, genuinely risen from +2 at levels \
         6-8: {}",
        trap_sense.detail
    );
}

// ----- Trapfinding stays 4 at level 9 (integer-division coincidence) -----

#[test]
fn rogue_level9_trapfinding_stays_four() {
    let input = load(ROGUE_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 4,
        "Rogue level 9 Trapfinding (max(9/2, 1)) must stay 4 — an integer-division \
         coincidence with level 8, not a stalled formula: {}",
        trapfinding.detail
    );
}

// ----- Granted features stay granted at level 9 -----

#[test]
fn rogue_level9_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, ROGUE_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion recognition must carry no fabricated mechanical value at level 9"
    );

    let uncanny_dodge = explanation(&computation, ROGUE_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge recognition must carry no fabricated mechanical value at level 9"
    );

    let improved_uncanny_dodge = explanation(&computation, ROGUE_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge recognition must carry no fabricated mechanical value at \
         level 9"
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn rogue_level8_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 4, "Rogue level 8 sneak attack die count must stay 4");

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 2, "Rogue level 8 Trap Sense must stay +2");

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Rogue level 8 poor Fortitude must stay 2");
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn rogue_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = ROGUE_LEVEL9_FIXTURE.replace("class:rogue:9", "class:rogue:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "level-10 Rogue is now recognized by the later level-10 widening slice \
         (tests/sd13_rogue_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level9_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_level9_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL9_FIXTURE.replace(
        "class_level=class:rogue:9",
        "class_level=class:rogue:9\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "multiclass Rogue must not gain any bounded rogue explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_rogue_row_names_level_9_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        rogue.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        rogue.grounding_ref.contains("sd13_rogue_level9_progression"),
        "rogue row must cite the live SD13-E5 level-9 proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "rogue partial note must name the level-9 widening: {note}"
    );
}
