//! SD13-E5 Barbarian level-10 progression grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-9 martial chassis baseline
//! (most recently `tests/sd13_barbarian_level9_progression.rs`) to Barbarian
//! level 10 — the tranche's declared ceiling — mirroring the sibling-class
//! level-range-gate idiom (`supported_barbarian_level` is generalized from
//! `1..=9` to `1..=10` via `MAX_SUPPORTED_BARBARIAN_LEVEL = 10`). Both PF1
//! CRB primary sources (d20pfsrd and legacy.aonprd.com Barbarian class
//! table) were read directly before writing any code or test:
//!
//! - level 10 base attack bonus is +10 (full BAB, genuinely risen from +9 —
//!   the class table's own "+10/+5" iterative notation is not modeled
//!   anywhere in this codebase, only the flat base value) and base saves
//!   are +7 Fortitude (good, `10 / 2 + 2 = 7`, genuinely risen from +6),
//!   +3 Reflex and +3 Will (both poor, `10 / 3 = 3`, numerically unchanged
//!   from level 9, integer-division coincidences) — confirmed by the same
//!   formulas already grounded at levels 1-9, not re-derived.
//! - the rage rounds-per-day pool GENUINELY RISES to 25 (`4 + Constitution
//!   modifier 3 + 2 × (10 - 1)`); the four flat rage-surface magnitudes
//!   stay at their standard-rage values (Greater Rage lands at 11th,
//!   checked rather than assumed).
//! - the PF1 Core Rulebook Barbarian class table's level-10 "Special"
//!   column reads "Damage reduction 2/—, rage power" (verified
//!   independently against both primary sources, checked rather than
//!   assumed away): the DR entry is a tier-rise on the already-grounded
//!   Damage Reduction flat-magnitude pillar — GENUINELY RISING to 2 (DR
//!   begins at 1/— at 7th and rises by 1 at 10th and every three levels
//!   thereafter) — and the rage-power entry is the same genuinely
//!   open-ended choice-list feature already deliberately left
//!   named-but-unproven at levels 2/4/6/8, so no new pillar is grounded at
//!   level 10, only the existing pillars are widened.
//! - Trap Sense stays +3 (`10 / 3 = 3`, its next rise landing at 12th);
//!   Fast Movement stays the flat +10; the illiteracy-absence
//!   classification, Uncanny Dodge, and Improved Uncanny Dodge all carry
//!   over unchanged, not re-derived.
//!
//! It deliberately does not touch the rage execution burden, the rage-power
//! list, or any damage-reduction application engine (all stay
//! named-but-unproven, unchanged from levels 1-9), and it does not ground
//! Barbarian level 11+. It also preserves the accepted Barbarian
//! level-1..level-9 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_barbarian_level9_sd13_deterministic_input.txt");

const BARBARIAN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_barbarian_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";
const BARBARIAN_DAMAGE_REDUCTION_ID: &str = "class_feature.barbarian.damage_reduction";

// ----- Base attack bonus and saves at level 10 -----

#[test]
fn barbarian_level10_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 10,
        "Barbarian level 10 full-BAB progression must equal 10, genuinely risen from 9: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 7,
        "Barbarian level 10 good Fortitude (10/2+2) must equal 7, genuinely risen from 6"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 3, "Barbarian level 10 poor Reflex (10/3) must equal 3");

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 3, "Barbarian level 10 poor Will (10/3) must equal 3");
}

// ----- Rage rounds per day genuinely rises to 25 at level 10 -----

#[test]
fn barbarian_level10_rage_rounds_rise_to_twenty_five() {
    let input = load(BARBARIAN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 25,
        "Barbarian level 10 rage rounds per day (4 + Constitution modifier 3 + 2 × (10 - 1)) \
         must equal 25, genuinely risen from 23 at level 9: {}",
        rage_rounds.detail
    );

    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 4),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 4),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 2),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "the rage surface magnitude '{id}' must stay at its standard-rage value at level \
             10 (Greater Rage lands at 11th)"
        );
    }
}

// ----- Damage Reduction genuinely rises to 2/- at level 10 -----

#[test]
fn barbarian_level10_damage_reduction_rises_to_two() {
    let input = load(BARBARIAN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 2,
        "Barbarian level 10 Damage Reduction must rise to 2/— (DR begins at 1/— at 7th and \
         rises by 1 at 10th and every three levels thereafter): {}",
        damage_reduction.detail
    );
}

// ----- Trap Sense stays +3; other pillars carry over at level 10 -----

#[test]
fn barbarian_level10_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 3,
        "Barbarian level 10 Trap Sense (10 / 3) must stay +3 — its next rise lands at 12th"
    );

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 10");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 10"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 10"
        );
    }
}

// ----- The rage execution burden still claim-blocks at level 10 -----

#[test]
fn barbarian_level10_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.barbarian.rage_execution.rounds_exceeded")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let not_raging = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.barbarian.rage_execution.not_raging");
            assert!(
                not_raging.is_some(),
                "level-10 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-9 fixture is unaffected by this widening -----

#[test]
fn barbarian_level9_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Barbarian level 9 base attack bonus must stay 9");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 23, "Barbarian level 9 rage rounds must stay 23");

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(damage_reduction.value, 1, "Barbarian level 9 Damage Reduction must stay 1/—");
}

// ----- Negative control: level 15 stays unrecognized by this slice -----
//
// This boundary was originally level 11 (the tranche-2 ceiling at the time
// this test was written); the SD18 barbarian-level11-greater-rage cycle
// widened `supported_barbarian_level` to `1..=11` (see
// `tests/sd18_barbarian_level11_greater_rage.rs`), moving this boundary to
// level 12; the SD18 barbarian-level12-widening cycle then widened
// `supported_barbarian_level` to `1..=12` (see
// `tests/sd18_barbarian_level12_widening.rs`), and the SD18
// barbarian-level13-widening cycle then widened it again to `1..=13` (see
// `tests/sd18_barbarian_level13_widening.rs`), so the correct negative
// control boundary for this file's own (level-10-era) baseline is now
// level 14, mirroring exactly how each earlier per-level Barbarian cycle
// moved this same negative control's boundary up by one level.

#[test]
fn barbarian_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARBARIAN_LEVEL10_FIXTURE.replace("class:barbarian:10", "class:barbarian:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.barbarian.")
                || e.id.starts_with("class_feature.barbarian."))
                // (v0.6 alpha swarm, risks item 8) rage-execution's
                // not-raging explanation is checked unconditionally,
                // regardless of level bound or single-class status
                // (mirrors the spell-posture classes' gate-ordering fix)
                && e.id != "class_feature.barbarian.rage_execution.not_raging"),
        "level-21 Barbarian must not gain any bounded barbarian explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level10_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")
                || e.id.starts_with("class_feature.barbarian.")),
        "the Fighter chassis must not surface any barbarian-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Barbarian is not promoted -----

#[test]
fn multiclass_barbarian_level10_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL10_FIXTURE.replace(
        "class_level=class:barbarian:10",
        "class_level=class:barbarian:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.barbarian.")
                || e.id.starts_with("class_feature.barbarian."))
                // (v0.6 alpha swarm, risks item 8) rage-execution's
                // not-raging explanation is checked unconditionally,
                // regardless of level bound or single-class status
                // (mirrors the spell-posture classes' gate-ordering fix)
                && e.id != "class_feature.barbarian.rage_execution.not_raging"),
        "multiclass Barbarian must not gain any bounded barbarian explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-10 widening -----

#[test]
fn matrix_barbarian_row_names_level_10_widening() {
    let matrix = seeded_current_truth();
    let barbarian = matrix
        .row("class.barbarian.bounded_progression")
        .expect("barbarian bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(barbarian.support_state, SupportState::Supported);
    assert_eq!(barbarian.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        barbarian.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        barbarian
            .grounding_ref
            .contains("sd13_barbarian_level10_progression"),
        "barbarian row must cite the live SD13-E5 level-10 proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "barbarian partial note must name the level-10 widening: {note}"
    );
}
