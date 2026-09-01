//! SD18 Barbarian level-15 widening grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-14 martial chassis baseline
//! (`tests/sd18_barbarian_level14_widening.rs`, the SD18 loop's own prior
//! ceiling) to Barbarian level 15 — the loop's FIRST §3.2 level-15 landing,
//! opening the level-15 sweep — mirroring the sibling per-level-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=14` to `1..=15` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 15`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, and agree byte-for-byte:
//!
//! - level 15 base attack bonus is +15/+10/+5 (full BAB, genuinely risen from
//!   +14); good Fortitude STAYS +9 (`15 / 2 + 2 = 9`, an integer-division
//!   coincidence with level 14), while poor Reflex and poor Will both
//!   GENUINELY RISE to +5 (`15 / 3 = 5`, up from +4 at level 14).
//! - the rage rounds-per-day pool genuinely rises to 35 (`4 + Constitution
//!   modifier 3 + 2 x (15 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-15 "Special" column
//!   reads "Trap sense +5" (verified independently against both primary
//!   sources, checked rather than assumed): Trap Sense is a magnitude-rise
//!   on the already-grounded flat Trap Sense formula pillar (barbarian
//!   level / 3), genuinely rising from +4 to +5 via the SAME pre-existing
//!   formula — no new tier constant, no new record type, and no new choice
//!   slot are needed at all, since the formula is already level-generic.
//!   Level 15 is NOT a rage-power level (powers land at
//!   2/4/6/8/10/12/14/16/18/20), so no eighth rage-power slot appears.
//! - Damage Reduction stays 3/- (next rise 16th); Indomitable Will's flat
//!   +4 magnitude carries over unchanged from level 14; the Greater Rage
//!   constants (+6/+6/+3/-2) stay unchanged from level 11; Fast Movement
//!   stays the flat +10; the illiteracy-absence classification, Uncanny
//!   Dodge, and Improved Uncanny Dodge all carry over unchanged, not
//!   re-derived.
//!
//! This is the CLEANEST possible widening shape checked so far in the
//! level-15 sweep: zero new record types, zero new named pillars, zero new
//! choice slots — the entire widening runs through pre-existing,
//! already-level-generic formulas (`level_value`, `level_value / 2 + 2`,
//! `level_value / 3`) with `MAX_SUPPORTED_BARBARIAN_LEVEL` as the only
//! production-code change.
//!
//! It deliberately does not touch the rage execution burden, any rage-power
//! EFFECT, the damage-reduction application engine, or the
//! saving-throw-resolution engine Indomitable Will would need to actually
//! apply (all stay named-but-unproven, unchanged from levels 1-14), and it
//! does not ground Barbarian level 16+. It also preserves the accepted
//! Barbarian level-1..level-14 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level14_sd18_widening_deterministic_input.txt"
);

const BARBARIAN_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level15_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";
const BARBARIAN_DAMAGE_REDUCTION_ID: &str = "class_feature.barbarian.damage_reduction";
const BARBARIAN_INDOMITABLE_WILL_ID: &str = "class_feature.barbarian.indomitable_will";

// ----- Base attack bonus and Reflex/Will genuinely rise; Fortitude stays at level 15 -----

#[test]
fn barbarian_level15_base_attack_and_poor_saves_rise_fortitude_stays() {
    let input = load(BARBARIAN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 15,
        "Barbarian level 15 full-BAB progression must equal 15, genuinely risen from 14: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Barbarian level 15 good Fortitude (15/2+2) must stay 9, unchanged from level 14"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Barbarian level 15 poor Reflex (15/3) must genuinely rise to 5, up from 4 at level 14"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 5,
        "Barbarian level 15 poor Will (15/3) must genuinely rise to 5, up from 4 at level 14"
    );
}

// ----- Rage rounds per day genuinely rises to 35 at level 15 -----

#[test]
fn barbarian_level15_rage_rounds_rise_to_thirty_five() {
    let input = load(BARBARIAN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 35,
        "Barbarian level 15 rage rounds per day (4 + Constitution modifier 3 + 2 x (15 - 1)) \
         must equal 35, genuinely risen from 33 at level 14: {}",
        rage_rounds.detail
    );
}

// ----- Trap Sense genuinely rises to +5 at level 15 -----

#[test]
fn barbarian_level15_trap_sense_rises_to_five() {
    let input = load(BARBARIAN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 5,
        "Barbarian level 15 Trap Sense (15 / 3) must genuinely rise to +5, up from +4 at level \
         14: {}",
        trap_sense.detail
    );
}

// ----- No eighth rage power slot appears at level 15 (15 is NOT a rage-power level) -----

#[test]
fn barbarian_level15_gains_no_eighth_rage_power_slot() {
    let input = load(BARBARIAN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.barbarian.rage_power_8_choice"),
        "Barbarian level 15 must not gain an eighth rage power slot: {:?}",
        computation.explanations
    );

    // The seventh slot (granted at level 14) still carries over.
    let slot_seven = explanation(&computation, "class_chassis.barbarian.rage_power_7_choice");
    assert_eq!(
        slot_seven.value, 0,
        "the seventh rage power slot must stay a bounded +0 recognition record at level 15: {}",
        slot_seven.detail
    );
}

// ----- Damage Reduction, Indomitable Will, and the remaining pillars carry over unchanged -----

#[test]
fn barbarian_level15_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 3,
        "Barbarian level 15 Damage Reduction must stay 3/-, unchanged from level 14: {}",
        damage_reduction.detail
    );

    let indomitable_will = explanation(&computation, BARBARIAN_INDOMITABLE_WILL_ID);
    assert_eq!(
        indomitable_will.value, 4,
        "Barbarian level 15 Indomitable Will must stay a flat +4 morale bonus, unchanged from \
         level 14: {}",
        indomitable_will.detail
    );

    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 6),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 6),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 3),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "the rage surface magnitude '{id}' must stay at its Greater Rage value at level 15 \
             (unchanged from level 11)"
        );
    }

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 15");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 15"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 15");
    }
}

// ----- The rage execution burden still claim-blocks at level 15 -----

#[test]
fn barbarian_level15_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL15_FIXTURE);
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
                "level-15 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-14 fixture is unaffected by this widening -----

#[test]
fn barbarian_level14_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 14, "Barbarian level 14 base attack bonus must stay 14");

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 4, "Barbarian level 14 poor Reflex must stay 4");

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 4, "Barbarian level 14 Trap Sense must stay +4");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 33, "Barbarian level 14 rage rounds must stay 33");
}

// ----- Negative control: level 16 stays unrecognized by this slice -----

#[test]
fn barbarian_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARBARIAN_LEVEL15_FIXTURE.replace("class:barbarian:15", "class:barbarian:21");
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
fn fighter_does_not_gain_barbarian_level15_recognition() {
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
fn multiclass_barbarian_level15_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL15_FIXTURE.replace(
        "class_level=class:barbarian:15",
        "class_level=class:barbarian:15\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_barbarian_row_names_level_15_widening() {
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
            .contains("sd18_barbarian_level15_widening"),
        "barbarian row must cite the live SD18 level-15 widening proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "barbarian partial note must name the level-15 widening: {note}"
    );
}
