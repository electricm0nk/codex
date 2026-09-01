//! SD13-E5 Rogue level-10 progression grounding proof — the first level-10
//! slice, opening the tranche's final level band (SD-13's declared ceiling
//! is level 10).
//!
//! Widens the accepted Rogue level-1..level-9 martial-skirmisher baseline
//! (most recently `tests/sd13_rogue_level9_progression.rs`) to Rogue level
//! 10, mirroring the sibling-class level-range-gate idiom
//! (`supported_rogue_level` is generalized from `1..=9` to `1..=10` via
//! `MAX_SUPPORTED_ROGUE_LEVEL = 10`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Rogue class table) were read directly before
//! writing any code or test:
//!
//! - level 10 base attack bonus is +7 (`10 * 3 / 4 = 7`, the Rogue's
//!   3/4-BAB progression, genuinely risen from +6 at level 9 — the class
//!   table's own "+7/+2" iterative notation is not modeled anywhere in this
//!   codebase, only the flat base value) and base saves are +3 Fortitude
//!   (poor, `10 / 3 = 3`, numerically unchanged from level 9, an
//!   integer-division coincidence), +7 Reflex (good, `10 / 2 + 2 = 7`,
//!   genuinely risen from +6), and +3 Will (poor, `10 / 3 = 3`, likewise
//!   unchanged) — confirmed by the same formulas already grounded at levels
//!   1-9, not re-derived.
//! - the PF1 Core Rulebook Rogue class table's level-10 "Special" column
//!   reads "Advanced talents, rogue talent" (verified independently against
//!   both primary sources, checked rather than assumed away) — BOTH parts
//!   belong to the same genuinely open-ended choice-list feature already
//!   deliberately left named-but-unproven at levels 2/4/6/8: the
//!   advanced-talent unlock ("At 10th level, and every two levels
//!   thereafter, a rogue can choose one of the following advanced talents
//!   in place of a rogue talent") is a list expansion of that feature, not
//!   a new pillar — so no new pillar record is grounded at level 10, only
//!   the existing pillars are widened.
//! - the sneak attack die count STAYS 5 (`(10 + 1) / 2 = 5`, the odd-level
//!   rise cadence — the next rise lands at 11th, checked rather than
//!   assumed); Trap Sense STAYS +3 (`10 / 3 = 3`, its next rise landing at
//!   12th); Trapfinding GENUINELY RISES to 5 (`max(10 / 2, 1) = 5`, up from
//!   4 at levels 8-9); Evasion, Uncanny Dodge, and Improved Uncanny Dodge
//!   all stay granted, not re-derived.
//!
//! It deliberately does not touch the rogue-talent tree (standard or
//! advanced), any check-execution engine, or sneak-attack damage
//! application (all stay named-but-unproven, unchanged from levels 1-9),
//! and it does not ground Rogue level 11+. It also preserves the accepted
//! Rogue level-1..level-9 truth (unchanged), the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level9_sd13_deterministic_input.txt");

const ROGUE_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus at level 10 -----

#[test]
fn rogue_level10_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(ROGUE_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Rogue level 10 3/4-BAB progression (10 * 3 / 4) must equal 7, genuinely risen from 6 \
         at level 9: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 10 (good Reflex, poor Fortitude/Will) -----

#[test]
fn rogue_level10_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(ROGUE_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 3,
        "Rogue level 10 poor Fortitude (10/3) must equal 3 — unchanged from level 9, an \
         integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 7,
        "Rogue level 10 good Reflex (10/2+2) must equal 7, genuinely risen from 6 at level 9"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 3, "Rogue level 10 poor Will (10/3) must equal 3");
}

// ----- Sneak attack die count stays 5 at level 10 -----

#[test]
fn rogue_level10_sneak_attack_die_count_stays_five() {
    let input = load(ROGUE_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 5,
        "Rogue level 10 sneak attack die count ((10 + 1) / 2) must stay 5 (5d6) — the \
         odd-level rise cadence puts the next rise at 11th: {}",
        sneak_attack.detail
    );
}

// ----- Trap Sense stays +3, Trapfinding rises to +5 at level 10 -----

#[test]
fn rogue_level10_trap_sense_stays_and_trapfinding_rises() {
    let input = load(ROGUE_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 3,
        "Rogue level 10 Trap Sense (10 / 3) must stay +3 — its next rise lands at 12th: {}",
        trap_sense.detail
    );

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 5,
        "Rogue level 10 Trapfinding (max(10/2, 1)) must equal 5, genuinely risen from 4 at \
         levels 8-9: {}",
        trapfinding.detail
    );
}

// ----- Granted features stay granted at level 10 -----

#[test]
fn rogue_level10_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 10"
        );
    }
}

// ----- No talent record is fabricated at level 10 -----

#[test]
fn rogue_level10_does_not_fabricate_talent_records() {
    let input = load(ROGUE_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("talent")),
        "level-10 Rogue must not fabricate any rogue-talent or advanced-talent record (the \
         level-10 'Advanced talents, rogue talent' entry is the same open-ended choice-list \
         feature left unproven at levels 2/4/6/8): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("talent")),
        "level-10 Rogue must not fabricate any talent diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-9 fixture is unaffected by this widening -----

#[test]
fn rogue_level9_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 6, "Rogue level 9 base attack bonus must stay 6");

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(trapfinding.value, 4, "Rogue level 9 Trapfinding must stay 4");

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 6, "Rogue level 9 good Reflex must stay 6");
}

// ----- Negative control: level 15 stays unrecognized by this slice -----
//
// SD18 widening (cycle-2026-07-14T2000, tests/sd18_rogue_level11_sneak_attack.rs):
// Rogue level 11 is now genuinely recognized (base attack bonus and sneak
// attack die count both rise), so this boundary control moved to level 12,
// mirroring the exact same boundary move each of the Barbarian/Bard/Cleric/
// Druid/Fighter/Monk/Paladin/Wizard level-11 widening cycles made for their
// own sibling level-10 progression tests. A further SD18 widening
// (cycle-2026-07-15T0800, tests/sd18_rogue_level12_widening.rs) now
// genuinely recognizes level 12 too, so this boundary control moved again,
// to level 13. A further SD18 widening (cycle-2026-07-15T1100,
// tests/sd18_rogue_level13_widening.rs) now genuinely recognizes level 13
// too, so this boundary control moved again, to level 14. A further SD18
// widening (cycle-2026-07-15T2000, tests/sd18_rogue_level14_widening.rs) now
// genuinely recognizes level 14 too, so this boundary control moved again,
// to level 16, per SD18 cycle-2026-07-15T2900 (tests/sd18_rogue_level15_widening.rs).
// A further SD18 widening (cycle-2026-07-15T5200,
// tests/sd18_rogue_level16_widening.rs) now genuinely recognizes level 16
// too, so this boundary control moved again, to level 17. A further SD18
// widening (cycle-2026-07-15T8100, tests/sd18_rogue_level17_widening.rs) now
// genuinely recognizes level 17 too, so this boundary control moved again,
// to level 18. A further SD18 widening (cycle-2026-07-16T0212,
// tests/sd18_rogue_level18_widening.rs) now genuinely recognizes level 18
// too, so this boundary control moved again, to level 19. A further SD18
// widening (cycle-2026-07-16T3600, tests/sd18_rogue_level19_widening.rs)
// now genuinely recognizes level 20 too, so this boundary control moves
// again, to level 21 (PF1 has no 21st character level; this is a pure
// implementation-gate check).
#[test]
fn rogue_level_21_is_not_promoted_by_this_slice() {
    let level_21 = ROGUE_LEVEL10_FIXTURE.replace("class:rogue:10", "class:rogue:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "level-21 Rogue must not gain any bounded rogue explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level10_recognition() {
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
fn multiclass_rogue_level10_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL10_FIXTURE.replace(
        "class_level=class:rogue:10",
        "class_level=class:rogue:10\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-10 widening -----

#[test]
fn matrix_rogue_row_names_level_10_widening() {
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
        rogue.grounding_ref.contains("sd13_rogue_level10_progression"),
        "rogue row must cite the live SD13-E5 level-10 proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "rogue partial note must name the level-10 widening: {note}"
    );
}
