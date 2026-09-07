//! SD18 Barbarian level-13 widening grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-12 martial chassis baseline
//! (`tests/sd18_barbarian_level12_widening.rs`, the SD18 loop's own prior
//! ceiling) to Barbarian level 13 — the loop's second §3.2 level-13 landing,
//! after Rogue's (`tests/sd18_rogue_level13_widening.rs`) — mirroring the
//! sibling per-level-gate idiom (`supported_barbarian_level` is generalized
//! from `1..=12` to `1..=13` via `MAX_SUPPORTED_BARBARIAN_LEVEL = 13`). Both
//! PF1 CRB primary sources (d20pfsrd and the Archives of Nethys aonprd.com
//! mirror) were read directly before writing any code or test, and agree
//! byte-for-byte:
//!
//! - level 13 base attack bonus is +13/+8/+3 (full BAB, genuinely risen from
//!   +12) while base saves STAY unchanged from level 12: +8 Fortitude
//!   (`13 / 2 + 2 = 8`), +4 Reflex and +4 Will (`13 / 3 = 4`), both
//!   integer-division coincidences with level 12.
//! - the rage rounds-per-day pool GENUINELY RISES to 31 (`4 + Constitution
//!   modifier 3 + 2 x (13 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-13 "Special" column
//!   reads "Damage reduction 3/-" (verified independently against both
//!   primary sources, checked rather than assumed): Damage Reduction
//!   GENUINELY RISES to 3/-, a third tier on the already-grounded
//!   flat-magnitude Damage Reduction pillar (mirroring the existing
//!   `BARBARIAN_DAMAGE_REDUCTION_LEVEL` / `_TWO_LEVEL` two-tier idiom with a
//!   new `BARBARIAN_DAMAGE_REDUCTION_THREE_LEVEL` third-tier constant) — no
//!   damage-resolution engine is invented, the value stays a bounded
//!   flat-magnitude record only.
//! - Trap Sense stays +4 (`13 / 3`, its next rise landing at 15th); the
//!   Greater Rage constants (+6/+6/+3/-2) stay unchanged from level 11; Fast
//!   Movement stays the flat +10; the illiteracy-absence classification,
//!   Uncanny Dodge, and Improved Uncanny Dodge all carry over unchanged, not
//!   re-derived; 13th level is NOT a rage-power level (powers land at
//!   2/4/6/8/10/12/14), so no seventh numbered choice-recognition slot is
//!   added.
//!
//! It deliberately does not touch the rage execution burden, any rage-power
//! EFFECT, or any damage-reduction application engine (all stay
//! named-but-unproven, unchanged from levels 1-12), and it does not ground
//! Barbarian level 14+. It also preserves the accepted Barbarian
//! level-1..level-12 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level12_sd18_widening_deterministic_input.txt"
);

const BARBARIAN_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level13_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";
const BARBARIAN_DAMAGE_REDUCTION_ID: &str = "class_feature.barbarian.damage_reduction";

// ----- Base attack bonus genuinely rises; base saves stay unchanged at level 13 -----

#[test]
fn barbarian_level13_base_attack_rises_saves_stay() {
    let input = load(BARBARIAN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 13,
        "Barbarian level 13 full-BAB progression must equal 13, genuinely risen from 12: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 8,
        "Barbarian level 13 good Fortitude (13/2+2) must stay 8, unchanged from level 12"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Barbarian level 13 poor Reflex (13/3) must stay 4, unchanged from level 12"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 4,
        "Barbarian level 13 poor Will (13/3) must stay 4, unchanged from level 12"
    );
}

// ----- Rage rounds per day genuinely rises to 31 at level 13 -----

#[test]
fn barbarian_level13_rage_rounds_rise_to_thirty_one() {
    let input = load(BARBARIAN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 31,
        "Barbarian level 13 rage rounds per day (4 + Constitution modifier 3 + 2 x (13 - 1)) \
         must equal 31, genuinely risen from 29 at level 12: {}",
        rage_rounds.detail
    );
}

// ----- Damage Reduction genuinely rises to 3/- at level 13 (the third tier) -----

#[test]
fn barbarian_level13_damage_reduction_rises_to_three() {
    let input = load(BARBARIAN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 3,
        "Barbarian level 13 Damage Reduction (a third tier on the already-grounded \
         flat-magnitude pillar) must rise to 3/-, genuinely risen from 2/- at level 12: {}",
        damage_reduction.detail
    );
}

// ----- Trap Sense and the remaining pillars carry over unchanged at level 13 -----

#[test]
fn barbarian_level13_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 4,
        "Barbarian level 13 Trap Sense (13 / 3) must stay +4, unchanged from level 12: {}",
        trap_sense.detail
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
            "the rage surface magnitude '{id}' must stay at its Greater Rage value at level 13 \
             (unchanged from level 11)"
        );
    }

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 13");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 13"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 13");
    }
}

// ----- No seventh rage power slot appears at level 13 (13 is not a rage-power level) -----

#[test]
fn barbarian_level13_gains_no_seventh_rage_power_slot() {
    let input = load(BARBARIAN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.barbarian.rage_power_7_choice"),
        "level 13 is not a rage-power grant level (powers land at 2/4/6/8/10/12/14), so no \
         seventh numbered slot should appear: {:?}",
        computation.explanations
    );

    // The sixth slot (granted at level 12) still carries over.
    let slot_six = explanation(&computation, "class_chassis.barbarian.rage_power_6_choice");
    assert_eq!(
        slot_six.value, 0,
        "the sixth rage power slot must stay a bounded +0 recognition record at level 13: {}",
        slot_six.detail
    );
}

// ----- The rage execution burden still claim-blocks at level 13 -----

#[test]
fn barbarian_level13_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL13_FIXTURE);
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
                "level-13 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-12 fixture is unaffected by this widening -----

#[test]
fn barbarian_level12_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 12, "Barbarian level 12 base attack bonus must stay 12");

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(damage_reduction.value, 2, "Barbarian level 12 Damage Reduction must stay 2/-");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 29, "Barbarian level 12 rage rounds must stay 29");
}

// ----- Negative control: level 16 stays unrecognized by this slice -----

#[test]
fn barbarian_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARBARIAN_LEVEL13_FIXTURE.replace("class:barbarian:13", "class:barbarian:21");
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
fn fighter_does_not_gain_barbarian_level13_recognition() {
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
fn multiclass_barbarian_level13_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL13_FIXTURE.replace(
        "class_level=class:barbarian:13",
        "class_level=class:barbarian:13\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-13 widening -----

#[test]
fn matrix_barbarian_row_names_level_13_widening() {
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
            .contains("sd18_barbarian_level13_widening"),
        "barbarian row must cite the live SD18 level-13 widening proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 13") || note.contains("level-13"),
        "barbarian partial note must name the level-13 widening: {note}"
    );
}
