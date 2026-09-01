//! SD18 Barbarian level-16 widening grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-15 martial chassis baseline
//! (`tests/sd18_barbarian_level15_widening.rs`, the SD18 loop's own prior
//! ceiling) to Barbarian level 16 — the loop's FIRST §3.2 level-16 landing,
//! opening the level-16 sweep — mirroring the sibling per-level-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=15` to `1..=16` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 16`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, and agree byte-for-byte:
//!
//! - level 16 base attack bonus is +16/+11/+6/+1 (full BAB, genuinely risen
//!   from +15); good Fortitude GENUINELY RISES to +10 (`16 / 2 + 2 = 10`,
//!   up from +9 at level 15), while poor Reflex and poor Will both STAY +5
//!   (`16 / 3 = 5`, an integer-division coincidence with level 15).
//! - the rage rounds-per-day pool genuinely rises to 37 (`4 + Constitution
//!   modifier 3 + 2 x (16 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-16 "Special" column
//!   reads "Damage reduction 4/-, rage power" (verified independently
//!   against both primary sources, checked rather than assumed): Damage
//!   Reduction is a magnitude-rise on the already-grounded flat Damage
//!   Reduction formula pillar via a FOURTH named tier constant
//!   (`BARBARIAN_DAMAGE_REDUCTION_FOUR_LEVEL`), genuinely rising from 3/- to
//!   4/- (the "10th level and every three barbarian levels thereafter"
//!   cadence: 10, 13, 16) — mirroring exactly how the level-10/level-13
//!   two-tier-then-three-tier idiom was established; the rage-power entry
//!   is the SAME open-ended choice-list feature already left
//!   named-but-unproven-in-effect at levels 2/4/6/8/10/12/14, grounded here
//!   as an EIGHTH numbered slot (`BARBARIAN_RAGE_POWER_SLOTS`), mirroring
//!   the proven repeat-grant idiom exactly: no rage-power-EFFECT engine is
//!   invented.
//! - Trap Sense stays +5 (next rise 18th); Indomitable Will's flat +4
//!   magnitude carries over unchanged from level 14; the Greater Rage
//!   constants (+6/+6/+3/-2) stay unchanged from level 11; Fast Movement
//!   stays the flat +10; the illiteracy-absence classification, Uncanny
//!   Dodge, and Improved Uncanny Dodge all carry over unchanged, not
//!   re-derived.
//!
//! This is the SECOND-CLEANEST widening shape checked in the level-16 sweep
//! opening: only two production-code changes beyond
//! `MAX_SUPPORTED_BARBARIAN_LEVEL` (15 -> 16) — a fourth Damage Reduction
//! tier constant and an eighth `BARBARIAN_RAGE_POWER_SLOTS` entry, both
//! mirroring already-proven idioms exactly, zero new record types, zero new
//! choice-slot mechanisms.
//!
//! It deliberately does not touch the rage execution burden, any rage-power
//! EFFECT, the damage-reduction application engine, or the
//! saving-throw-resolution engine Indomitable Will would need to actually
//! apply (all stay named-but-unproven, unchanged from levels 1-15), and it
//! does not ground Barbarian level 17+. It also preserves the accepted
//! Barbarian level-1..level-15 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level15_sd18_widening_deterministic_input.txt"
);

const BARBARIAN_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level16_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus and good Fortitude genuinely rise; Reflex/Will stay at level 16 -----

#[test]
fn barbarian_level16_base_attack_and_fortitude_rise_poor_saves_stay() {
    let input = load(BARBARIAN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 16,
        "Barbarian level 16 full-BAB progression must equal 16, genuinely risen from 15: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Barbarian level 16 good Fortitude (16/2+2) must genuinely rise to 10, up from 9 at \
         level 15"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Barbarian level 16 poor Reflex (16/3) must stay 5, unchanged from level 15"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 5,
        "Barbarian level 16 poor Will (16/3) must stay 5, unchanged from level 15"
    );
}

// ----- Rage rounds per day genuinely rises to 37 at level 16 -----

#[test]
fn barbarian_level16_rage_rounds_rise_to_thirty_seven() {
    let input = load(BARBARIAN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 37,
        "Barbarian level 16 rage rounds per day (4 + Constitution modifier 3 + 2 x (16 - 1)) \
         must equal 37, genuinely risen from 35 at level 15: {}",
        rage_rounds.detail
    );
}

// ----- Damage Reduction genuinely rises to 4/- at level 16 -----

#[test]
fn barbarian_level16_damage_reduction_rises_to_four() {
    let input = load(BARBARIAN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 4,
        "Barbarian level 16 Damage Reduction (a fourth named tier) must genuinely rise to 4/-, \
         up from 3/- at level 15: {}",
        damage_reduction.detail
    );
}

// ----- An eighth rage power slot appears at level 16 (16 IS a rage-power level) -----

#[test]
fn barbarian_level16_gains_an_eighth_rage_power_slot() {
    let input = load(BARBARIAN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_eight = explanation(&computation, "class_chassis.barbarian.rage_power_8_choice");
    assert_eq!(
        slot_eight.value, 0,
        "the eighth rage power slot must be a bounded +0 recognition record at level 16: {}",
        slot_eight.detail
    );
    assert!(
        slot_eight.detail.contains("roused_anger"),
        "the eighth rage power slot must name the fixture's selection: {}",
        slot_eight.detail
    );

    // The seventh slot (granted at level 14) still carries over.
    let slot_seven = explanation(&computation, "class_chassis.barbarian.rage_power_7_choice");
    assert_eq!(
        slot_seven.value, 0,
        "the seventh rage power slot must stay a bounded +0 recognition record at level 16: {}",
        slot_seven.detail
    );
}

// ----- Trap Sense, Indomitable Will, and the remaining pillars carry over unchanged -----

#[test]
fn barbarian_level16_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 5,
        "Barbarian level 16 Trap Sense must stay +5, unchanged from level 15: {}",
        trap_sense.detail
    );

    let indomitable_will = explanation(&computation, BARBARIAN_INDOMITABLE_WILL_ID);
    assert_eq!(
        indomitable_will.value, 4,
        "Barbarian level 16 Indomitable Will must stay a flat +4 morale bonus, unchanged from \
         level 15: {}",
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
            "the rage surface magnitude '{id}' must stay at its Greater Rage value at level 16 \
             (unchanged from level 11)"
        );
    }

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 16");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 16"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 16");
    }
}

// ----- The rage execution burden still claim-blocks at level 16 -----

#[test]
fn barbarian_level16_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL16_FIXTURE);
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
                "level-16 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-15 fixture is unaffected by this widening -----

#[test]
fn barbarian_level15_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 15, "Barbarian level 15 base attack bonus must stay 15");

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 9, "Barbarian level 15 good Fortitude must stay 9");

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(damage_reduction.value, 3, "Barbarian level 15 Damage Reduction must stay 3/-");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 35, "Barbarian level 15 rage rounds must stay 35");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.barbarian.rage_power_8_choice"),
        "Barbarian level 15 must not gain an eighth rage power slot: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 17 is now the supported row; its own test file
// (tests/sd18_barbarian_level17_widening.rs) owns that boundary going forward, so
// this file's own level-17 negative control is removed rather than moved, mirroring
// the exact fix every prior level-N Barbarian cycle made for its own siblings. -----

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level16_recognition() {
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
fn multiclass_barbarian_level16_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL16_FIXTURE.replace(
        "class_level=class:barbarian:16",
        "class_level=class:barbarian:16\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-16 widening -----

#[test]
fn matrix_barbarian_row_names_level_16_widening() {
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
            .contains("sd18_barbarian_level16_widening"),
        "barbarian row must cite the live SD18 level-16 widening proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 16") || note.contains("level-16"),
        "barbarian partial note must name the level-16 widening: {note}"
    );
}
