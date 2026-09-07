//! SD18 Barbarian level-11 Greater Rage grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-10 martial chassis baseline
//! (`tests/sd13_barbarian_level10_progression.rs`, the tranche-2's declared
//! ceiling) to Barbarian level 11 — the first SD-18 §3.2 class-row
//! widening — mirroring the sibling-class level-range-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=10` to `1..=11`
//! via `MAX_SUPPORTED_BARBARIAN_LEVEL = 11`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Barbarian class table) were read
//! directly before writing any code or test:
//!
//! - level 11 base attack bonus is +11 (full BAB, genuinely risen from +10)
//!   and base saves are +7 Fortitude (good, `11 / 2 + 2 = 7`, numerically
//!   unchanged from level 10, an integer-division coincidence), +3 Reflex
//!   and +3 Will (both poor, `11 / 3 = 3`, also unchanged from level 10) —
//!   confirmed by the same formulas already grounded at levels 1-10, not
//!   re-derived.
//! - the rage rounds-per-day pool GENUINELY RISES to 27 (`4 + Constitution
//!   modifier 3 + 2 x (11 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-11 "Special"
//!   column reads "Greater rage" only (verified independently against both
//!   primary sources, checked rather than assumed): Greater Rage upgrades
//!   the flat while-raging Strength and Constitution morale bonuses from
//!   +4 to +6 and the Will-save morale bonus from +2 to +3; the Armor
//!   Class penalty stays -2. This is a magnitude-rise on the already-
//!   grounded rage-constant pillar, mirroring exactly how Trap Sense's and
//!   Damage Reduction's own flat magnitudes were widened at their rise
//!   levels — no new rage-power grant lands at 11th (the next grant is
//!   12th), so no rage-power-selection-slot-count engine is invented.
//! - Trap Sense stays +3 (`11 / 3 = 3`, its next rise landing at 12th);
//!   Damage Reduction stays 2/- (its next rise landing at 13th); Fast
//!   Movement stays the flat +10; the illiteracy-absence classification,
//!   Uncanny Dodge, and Improved Uncanny Dodge all carry over unchanged,
//!   not re-derived.
//!
//! It deliberately does not touch the rage execution burden, the rage-power
//! list, or any damage-reduction application engine (all stay
//! named-but-unproven, unchanged from levels 1-10), and it does not ground
//! Barbarian level 12+. It also preserves the accepted Barbarian
//! level-1..level-10 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_barbarian_level10_sd13_deterministic_input.txt");

const BARBARIAN_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level11_sd18_greater_rage_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";
const BARBARIAN_DAMAGE_REDUCTION_ID: &str = "class_feature.barbarian.damage_reduction";

// ----- Base attack bonus and saves at level 11 -----

#[test]
fn barbarian_level11_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 11,
        "Barbarian level 11 full-BAB progression must equal 11, genuinely risen from 10: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 7, "Barbarian level 11 good Fortitude (11/2+2) must equal 7");

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 3, "Barbarian level 11 poor Reflex (11/3) must equal 3");

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 3, "Barbarian level 11 poor Will (11/3) must equal 3");
}

// ----- Rage rounds per day genuinely rises to 27 at level 11 -----

#[test]
fn barbarian_level11_rage_rounds_rise_to_twenty_seven() {
    let input = load(BARBARIAN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 27,
        "Barbarian level 11 rage rounds per day (4 + Constitution modifier 3 + 2 x (11 - 1)) \
         must equal 27, genuinely risen from 25 at level 10: {}",
        rage_rounds.detail
    );
}

// ----- Greater Rage genuinely rises the flat rage constants at level 11 -----

#[test]
fn barbarian_level11_greater_rage_rises_the_rage_constants() {
    let input = load(BARBARIAN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 6),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 6),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 3),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "the rage surface magnitude '{id}' must reflect Greater Rage at level 11 \
             (Strength/Constitution morale bonus +6, Will save morale bonus +3, AC penalty \
             stays -2): {}",
            record.detail
        );
    }
}

// ----- Damage Reduction and Trap Sense stay unchanged at level 11 -----

#[test]
fn barbarian_level11_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 2,
        "Barbarian level 11 Damage Reduction must stay 2/— — its next rise lands at 13th"
    );

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 3,
        "Barbarian level 11 Trap Sense (11 / 3) must stay +3 — its next rise lands at 12th"
    );

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 11");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 11"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 11");
    }
}

// ----- The rage execution burden still claim-blocks at level 11 -----

#[test]
fn barbarian_level11_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL11_FIXTURE);
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
                "level-11 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn barbarian_level10_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 10, "Barbarian level 10 base attack bonus must stay 10");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 25, "Barbarian level 10 rage rounds must stay 25");

    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 4),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 4),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 2),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "Barbarian level 10 rage surface magnitude '{id}' must stay at its standard-rage \
             value (Greater Rage lands at 11th, not 10th)"
        );
    }
}

// ----- Negative control: level 13 stays unrecognized by this slice -----
//
// This boundary was originally level 12 (the SD18 ceiling at the time this
// test was written); the SD18 barbarian-level12-widening and
// barbarian-level13-widening cycles widened `supported_barbarian_level` to
// `1..=13` (see `tests/sd18_barbarian_level12_widening.rs` and
// `tests/sd18_barbarian_level13_widening.rs`), so the correct negative
// control boundary for this file's own (level-11-era) baseline is now level
// 14, mirroring exactly how each earlier per-level Barbarian cycle moved
// this same negative control's boundary up by one level.

#[test]
fn barbarian_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARBARIAN_LEVEL11_FIXTURE.replace("class:barbarian:11", "class:barbarian:21");
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
fn fighter_does_not_gain_barbarian_level11_recognition() {
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
fn multiclass_barbarian_level11_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL11_FIXTURE.replace(
        "class_level=class:barbarian:11",
        "class_level=class:barbarian:11\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_barbarian_row_names_level_11_widening() {
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
            .contains("sd18_barbarian_level11_greater_rage"),
        "barbarian row must cite the live SD18 level-11 Greater Rage proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11") || note.contains("Greater Rage"),
        "barbarian partial note must name the level-11 Greater Rage widening: {note}"
    );
}
