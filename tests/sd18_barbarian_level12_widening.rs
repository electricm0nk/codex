//! SD18 Barbarian level-12 widening grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-11 martial chassis baseline
//! (`tests/sd18_barbarian_level11_greater_rage.rs`, the SD18 loop's own
//! prior ceiling) to Barbarian level 12 — the loop's first §3.2 level-12
//! widening — mirroring the sibling per-level-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=11` to `1..=12`
//! via `MAX_SUPPORTED_BARBARIAN_LEVEL = 12`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, and agree byte-for-byte:
//!
//! - level 12 base attack bonus is +12 (full BAB, genuinely risen from +11)
//!   and base saves are +8 Fortitude (good, `12 / 2 + 2 = 8`, genuinely
//!   risen from +7), +4 Reflex and +4 Will (both poor, `12 / 3 = 4`,
//!   genuinely risen from +3) — confirmed by the same formulas already
//!   grounded at levels 1-11, not re-derived.
//! - the rage rounds-per-day pool GENUINELY RISES to 29 (`4 + Constitution
//!   modifier 3 + 2 x (12 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-12 "Special"
//!   column reads "Rage power, trap sense +4" (verified independently
//!   against both primary sources, checked rather than assumed): Trap
//!   Sense GENUINELY RISES to +4 (`12 / 3 = 4`), a magnitude-rise on the
//!   already-grounded Trap Sense flat-magnitude formula pillar, mirroring
//!   exactly how the level-6/level-9 Trap Sense rises and the level-10/
//!   level-11 Damage Reduction/Greater Rage rises were widened; the
//!   rage-power entry is the SAME genuinely open-ended choice-list feature
//!   already deliberately left named-but-unproven-in-effect at levels
//!   2/4/6/8/10 (the sixth numbered slot, gate 12, added to
//!   `BARBARIAN_RAGE_POWER_SLOTS` mirroring the proven repeat-grant idiom
//!   exactly — no power-list validation, no rage-state execution engine
//!   invented).
//! - Damage Reduction stays 2/- (its next rise landing at 13th); the
//!   Greater Rage constants (+6/+6/+3/-2) stay unchanged from level 11;
//!   Fast Movement stays the flat +10; the illiteracy-absence
//!   classification, Uncanny Dodge, and Improved Uncanny Dodge all carry
//!   over unchanged, not re-derived.
//!
//! It deliberately does not touch the rage execution burden, any rage-power
//! EFFECT, or any damage-reduction application engine (all stay
//! named-but-unproven, unchanged from levels 1-11), and it does not ground
//! Barbarian level 13+. It also preserves the accepted Barbarian
//! level-1..level-11 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level11_sd18_greater_rage_deterministic_input.txt"
);

const BARBARIAN_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level12_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";
const BARBARIAN_DAMAGE_REDUCTION_ID: &str = "class_feature.barbarian.damage_reduction";

// ----- Base attack bonus and saves at level 12 -----

#[test]
fn barbarian_level12_base_attack_and_saves_genuinely_rise() {
    let input = load(BARBARIAN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Barbarian level 12 full-BAB progression must equal 12, genuinely risen from 11: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 8,
        "Barbarian level 12 good Fortitude (12/2+2) must equal 8, genuinely risen from 7"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Barbarian level 12 poor Reflex (12/3) must equal 4, genuinely risen from 3"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 4,
        "Barbarian level 12 poor Will (12/3) must equal 4, genuinely risen from 3"
    );
}

// ----- Rage rounds per day genuinely rises to 29 at level 12 -----

#[test]
fn barbarian_level12_rage_rounds_rise_to_twenty_nine() {
    let input = load(BARBARIAN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 29,
        "Barbarian level 12 rage rounds per day (4 + Constitution modifier 3 + 2 x (12 - 1)) \
         must equal 29, genuinely risen from 27 at level 11: {}",
        rage_rounds.detail
    );
}

// ----- Trap Sense genuinely rises to +4 at level 12 -----

#[test]
fn barbarian_level12_trap_sense_rises_to_four() {
    let input = load(BARBARIAN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 4,
        "Barbarian level 12 Trap Sense (12 / 3) must rise to +4, genuinely risen from +3: {}",
        trap_sense.detail
    );
}

// ----- Greater Rage constants and Damage Reduction stay unchanged at level 12 -----

#[test]
fn barbarian_level12_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL12_FIXTURE);
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
            "the rage surface magnitude '{id}' must stay at its Greater Rage value at level 12 \
             (unchanged from level 11)"
        );
    }

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 2,
        "Barbarian level 12 Damage Reduction must stay 2/— — its next rise lands at 13th"
    );

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 12");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 12"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 12");
    }
}

// ----- The sixth rage power slot fires at level 12, mirroring slots 1-5 -----

#[test]
fn barbarian_level12_sixth_rage_power_slot_is_recognized() {
    let input = load(BARBARIAN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_six = explanation(&computation, "class_chassis.barbarian.rage_power_6_choice");
    assert_eq!(
        slot_six.value, 0,
        "the sixth rage power slot must be a bounded +0 recognition record, non-fabricated: {}",
        slot_six.detail
    );
    assert!(
        slot_six.detail.contains("superstition"),
        "the sixth rage power slot's recognition must name the selected power raw string: {}",
        slot_six.detail
    );
}

// ----- The rage execution burden still claim-blocks at level 12 -----

#[test]
fn barbarian_level12_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL12_FIXTURE);
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
                "level-12 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-11 fixture is unaffected by this widening -----

#[test]
fn barbarian_level11_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 11, "Barbarian level 11 base attack bonus must stay 11");

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 3, "Barbarian level 11 Trap Sense must stay +3");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 27, "Barbarian level 11 rage rounds must stay 27");
}

// ----- Negative control: level 15 stays unrecognized by this slice -----
//
// This boundary was originally level 13 (the SD18 ceiling at the time this
// test was written); the SD18 barbarian-level13-widening cycle widened
// `supported_barbarian_level` to `1..=13` (see
// `tests/sd18_barbarian_level13_widening.rs`), so the correct negative
// control boundary for this file's own (level-12-era) baseline is now level
// 14, mirroring exactly how each earlier per-level Barbarian cycle moved
// this same negative control's boundary up by one level.

#[test]
fn barbarian_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARBARIAN_LEVEL12_FIXTURE.replace("class:barbarian:12", "class:barbarian:21");
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
fn fighter_does_not_gain_barbarian_level12_recognition() {
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
fn multiclass_barbarian_level12_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL12_FIXTURE.replace(
        "class_level=class:barbarian:12",
        "class_level=class:barbarian:12\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_barbarian_row_names_level_12_widening() {
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
            .contains("sd18_barbarian_level12_widening"),
        "barbarian row must cite the live SD18 level-12 widening proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "barbarian partial note must name the level-12 widening: {note}"
    );
}
