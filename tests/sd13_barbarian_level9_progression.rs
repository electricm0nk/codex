//! SD13-E5 Barbarian level-9 progression grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-8 martial chassis baseline
//! (most recently `tests/sd13_barbarian_level8_progression.rs`) to Barbarian
//! level 9, mirroring the sibling-class level-range-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 9`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Barbarian class table) were read directly
//! before writing any code or test:
//!
//! - level 9 base attack bonus is +9 (full BAB, genuinely risen from +8 at
//!   level 8 — the class table's own "+9/+4" iterative-attack notation is
//!   not modeled anywhere in this codebase, only the flat base value) and
//!   base saves are +6 Fortitude (good, `9 / 2 + 2 = 6`, numerically
//!   unchanged from level 8, an integer-division coincidence), +3 Reflex
//!   (poor, `9 / 3 = 3`, genuinely risen from +2), and +3 Will (poor,
//!   `9 / 3 = 3`, genuinely risen from +2) — confirmed by the same formulas
//!   already grounded at levels 1-8, not re-derived.
//! - the rage rounds-per-day pool GENUINELY RISES to 23 (`4 + Constitution
//!   modifier 3 + 2 × (9 - 1)`, via the same pre-existing formula: 4 + Con
//!   mod at 1st, plus 2 per level thereafter); the four flat rage-surface
//!   magnitudes (+4 Strength morale, +4 Constitution morale, +2 Will save
//!   morale, -2 AC penalty) all stay at their standard-rage values (the
//!   next change is Greater Rage at 11th, checked rather than assumed).
//! - Fast Movement stays the flat +10; Illiteracy's absence for the PF1
//!   barbarian stays a +0 classification record; Uncanny Dodge (2nd) and
//!   Improved Uncanny Dodge (5th) stay granted +0 identity records; Damage
//!   Reduction stays 1/— (the next DR rise lands at 10th level, checked
//!   rather than assumed).
//! - the PF1 Core Rulebook Barbarian class table's level-9 "Special" column
//!   reads "Trap sense +3" (verified independently against both primary
//!   sources, checked rather than assumed away) — a tier-rise on the
//!   already-grounded Trap Sense formula pillar (`level / 3 = 3`, up from
//!   +2 at levels 6-8), not a new class feature. Level 9 is NOT a
//!   rage-power level (powers land at 2/4/6/8/10...), so no new pillar is
//!   grounded and nothing new is left unproven for the rage-power list
//!   either.
//!
//! It deliberately does not touch the rage execution burden (activation,
//! round tracking, fatigue), the rage-power list, or any damage-reduction
//! application engine (all stay named-but-unproven, unchanged from levels
//! 1-8), and it does not ground Barbarian level 10+. It also preserves the
//! accepted Barbarian level-1..level-8 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_barbarian_level8_sd13_deterministic_input.txt");

const BARBARIAN_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_barbarian_level9_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";
const BARBARIAN_DAMAGE_REDUCTION_ID: &str = "class_feature.barbarian.damage_reduction";

// ----- Base attack bonus at level 9 -----

#[test]
fn barbarian_level9_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARBARIAN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Barbarian level 9 full-BAB progression must equal 9, genuinely risen from 8 at \
         level 8: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 9 (good Fortitude, poor Reflex/Will) -----

#[test]
fn barbarian_level9_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Barbarian level 9 good Fortitude (9/2+2) must equal 6 — unchanged from level 8, an \
         integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 3,
        "Barbarian level 9 poor Reflex (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 3,
        "Barbarian level 9 poor Will (9/3) must equal 3, genuinely risen from 2 at level 8"
    );
}

// ----- Rage rounds per day genuinely rises to 23 at level 9 -----

#[test]
fn barbarian_level9_rage_rounds_rise_to_twenty_three() {
    let input = load(BARBARIAN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 23,
        "Barbarian level 9 rage rounds per day (4 + Constitution modifier 3 + 2 × (9 - 1)) \
         must equal 23, genuinely risen from 21 at level 8: {}",
        rage_rounds.detail
    );
}

// ----- The four flat rage-surface magnitudes stay at standard-rage values -----

#[test]
fn barbarian_level9_rage_surface_magnitudes_stay_standard() {
    let input = load(BARBARIAN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for (id, expected, label) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 4, "Strength morale bonus"),
        (
            "class_chassis.barbarian.rage.constitution_morale_bonus",
            4,
            "Constitution morale bonus",
        ),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 2, "Will save morale bonus"),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2, "armor class penalty"),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "Barbarian level 9 rage {label} must stay at its standard-rage value (the next \
             change is Greater Rage at 11th): {}",
            record.detail
        );
    }
}

// ----- Trap Sense genuinely rises to +3 at level 9 -----

#[test]
fn barbarian_level9_trap_sense_rises_to_three() {
    let input = load(BARBARIAN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 3,
        "Barbarian level 9 Trap Sense (9 / 3) must equal +3, genuinely risen from +2 at \
         levels 6-8: {}",
        trap_sense.detail
    );
}

// ----- Fast Movement / Illiteracy-absence / granted features stay unchanged -----

#[test]
fn barbarian_level9_remaining_pillars_stay_unchanged() {
    let input = load(BARBARIAN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 9");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 9"
    );

    let uncanny_dodge = explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID);
    assert_eq!(uncanny_dodge.value, 0, "Uncanny Dodge must stay a granted +0 record");

    let improved_uncanny_dodge = explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge must stay a granted +0 record"
    );

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 1,
        "Barbarian level 9 Damage Reduction must stay 1/— (the next DR rise lands at 10th): \
         {}",
        damage_reduction.detail
    );
}

// ----- The rage execution burden still claim-blocks at level 9 -----

#[test]
fn barbarian_level9_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL9_FIXTURE);
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
                "level-9 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn barbarian_level8_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Barbarian level 8 base attack bonus must stay 8");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 21, "Barbarian level 8 rage rounds must stay 21");

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 2, "Barbarian level 8 Trap Sense must stay +2");
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn barbarian_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = BARBARIAN_LEVEL9_FIXTURE.replace("class:barbarian:9", "class:barbarian:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "level-10 Barbarian is now recognized by the later level-10 widening slice \
         (tests/sd13_barbarian_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level9_recognition() {
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
fn multiclass_barbarian_level9_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL9_FIXTURE.replace(
        "class_level=class:barbarian:9",
        "class_level=class:barbarian:9\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_barbarian_row_names_level_9_widening() {
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
            .contains("sd13_barbarian_level9_progression"),
        "barbarian row must cite the live SD13-E5 level-9 proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "barbarian partial note must name the level-9 widening: {note}"
    );
}
