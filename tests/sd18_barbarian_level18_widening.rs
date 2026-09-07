//! SD18 Barbarian level-18 widening grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-17 martial chassis baseline
//! (`tests/sd18_barbarian_level17_widening.rs`, the SD18 loop's own prior
//! ceiling) to Barbarian level 18 — the loop's FIFTH §3.2 level-18
//! landing, after Wizard, Cleric, Paladin, and Fighter — mirroring the
//! sibling per-level-gate idiom (`supported_barbarian_level` is
//! generalized from `1..=17` to `1..=18` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 18`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, fetching the full
//! levels-15-through-20 block in one pass so the level-18 row's neighbors
//! were visible in context (guards against level-misattribution), and agree
//! byte-for-byte at every level in that block — no third source was
//! required:
//!
//! - level 18 base attack bonus is +18/+13/+8/+3 (full BAB, genuinely risen
//!   from +17); good Fortitude GENUINELY RISES to +11 (`18 / 2 + 2 = 11`),
//!   and poor Reflex and poor Will both GENUINELY RISE to +6 (`18 / 3 = 6`).
//! - the rage rounds-per-day pool genuinely rises to 41 (`4 + Constitution
//!   modifier 3 + 2 x (18 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-18 "Special" column
//!   reads "Rage power, trap sense +6" (verified independently against both
//!   primary sources, checked rather than assumed): 18 IS a rage-power
//!   level (powers land at 2/4/6/8/10/12/14/16/18/20), so a NINTH numbered
//!   rage power slot is added, mirroring the proven repeat-grant idiom
//!   exactly (no rage-power-EFFECT engine invented); Trap Sense GENUINELY
//!   RISES to +6 (18/3, the same pre-existing formula, up from +5 at level
//!   17).
//! - Damage Reduction stays 4/- (next rise 19th); Indomitable Will's flat
//!   +4 magnitude carries over unchanged from level 14; Tireless Rage
//!   carries over unchanged from level 17; the Greater Rage constants
//!   (+6/+6/+3/-2) stay unchanged from level 11; Fast Movement stays the
//!   flat +10; the illiteracy-absence classification, Uncanny Dodge, and
//!   Improved Uncanny Dodge all carry over unchanged, not re-derived.
//!
//! This is another clean widening shape: only two production-code changes
//! beyond `MAX_SUPPORTED_BARBARIAN_LEVEL` (17 -> 18) — a ninth numbered
//! slot appended to `BARBARIAN_RAGE_POWER_SLOTS`, mirroring the
//! already-proven repeat-grant idiom exactly (Trap Sense, base attack, and
//! all three base saves needed ZERO formula changes, since they were
//! already level-generic formulas) — zero new record types, zero new
//! choice-slot mechanisms, zero new subsystems.
//!
//! It deliberately does not touch the rage execution burden, any rage-power
//! EFFECT, the damage-reduction application engine, or the
//! saving-throw-resolution engine Indomitable Will would need to actually
//! apply (all stay named-but-unproven, unchanged from levels 1-17), and it
//! does not ground Barbarian level 19+. It also preserves the accepted
//! Barbarian level-1..level-17 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level17_sd18_widening_deterministic_input.txt"
);

const BARBARIAN_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level18_sd18_widening_deterministic_input.txt"
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
const BARBARIAN_TIRELESS_RAGE_ID: &str = "class_chassis.barbarian.tireless_rage";

// ----- Base attack bonus and all three base saves genuinely rise at level 18 -----

#[test]
fn barbarian_level18_base_attack_and_saves_rise() {
    let input = load(BARBARIAN_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 18,
        "Barbarian level 18 full-BAB progression must equal 18, genuinely risen from 17: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 11,
        "Barbarian level 18 good Fortitude (18/2+2) must genuinely rise to 11, up from 10 at \
         level 17"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Barbarian level 18 poor Reflex (18/3) must genuinely rise to 6, up from 5 at level 17"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 6,
        "Barbarian level 18 poor Will (18/3) must genuinely rise to 6, up from 5 at level 17"
    );
}

// ----- Rage rounds per day genuinely rises to 41 at level 18 -----

#[test]
fn barbarian_level18_rage_rounds_rise_to_forty_one() {
    let input = load(BARBARIAN_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 41,
        "Barbarian level 18 rage rounds per day (4 + Constitution modifier 3 + 2 x (18 - 1)) \
         must equal 41, genuinely risen from 39 at level 17: {}",
        rage_rounds.detail
    );
}

// ----- Trap Sense genuinely rises to +6 at level 18 -----

#[test]
fn barbarian_level18_trap_sense_rises_to_six() {
    let input = load(BARBARIAN_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 6,
        "Barbarian level 18 Trap Sense (18/3) must genuinely rise to +6, up from +5 at level \
         17: {}",
        trap_sense.detail
    );
}

// ----- Ninth rage power slot is newly grounded at level 18 -----

#[test]
fn barbarian_level18_gains_ninth_rage_power_slot() {
    let input = load(BARBARIAN_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_nine = explanation(&computation, "class_chassis.barbarian.rage_power_9_choice");
    assert_eq!(
        slot_nine.value, 0,
        "the ninth rage power slot must be a bounded +0 recognition record at level 18: {}",
        slot_nine.detail
    );
    assert!(
        slot_nine.detail.contains("knockback"),
        "the ninth rage power slot detail must name the level-18 fixture's chosen selection: {}",
        slot_nine.detail
    );

    // The eighth slot (granted at level 16) still carries over.
    let slot_eight = explanation(&computation, "class_chassis.barbarian.rage_power_8_choice");
    assert_eq!(
        slot_eight.value, 0,
        "the eighth rage power slot must stay a bounded +0 recognition record at level 18: {}",
        slot_eight.detail
    );
}

// ----- Remaining pillars carry over unchanged -----

#[test]
fn barbarian_level18_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 4,
        "Barbarian level 18 Damage Reduction must stay 4/-, unchanged from level 17 (next rise \
         19th): {}",
        damage_reduction.detail
    );

    let indomitable_will = explanation(&computation, BARBARIAN_INDOMITABLE_WILL_ID);
    assert_eq!(
        indomitable_will.value, 4,
        "Barbarian level 18 Indomitable Will must stay a flat +4 morale bonus, unchanged from \
         level 17: {}",
        indomitable_will.detail
    );

    let tireless_rage = explanation(&computation, BARBARIAN_TIRELESS_RAGE_ID);
    assert_eq!(
        tireless_rage.value, 0,
        "Barbarian level 18 Tireless Rage must stay a bounded grant-only identity record \
         (value 0), carried over unchanged from level 17: {}",
        tireless_rage.detail
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
            "the rage surface magnitude '{id}' must stay at its Greater Rage value at level 18 \
             (unchanged from level 11)"
        );
    }

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 18");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 18"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 18");
    }
}

// ----- The rage execution burden still claim-blocks at level 18 -----

#[test]
fn barbarian_level18_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL18_FIXTURE);
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
                "level-18 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-17 fixture is unaffected by this widening -----

#[test]
fn barbarian_level17_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 17, "Barbarian level 17 base attack bonus must stay 17");

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 10, "Barbarian level 17 good Fortitude must stay 10");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 39, "Barbarian level 17 rage rounds must stay 39");

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 5, "Barbarian level 17 Trap Sense must stay +5");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.barbarian.rage_power_9_choice"),
        "Barbarian level 17 must not gain a ninth rage power slot: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 19 stays unrecognized by this slice -----

#[test]
fn barbarian_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARBARIAN_LEVEL18_FIXTURE.replace("class:barbarian:18", "class:barbarian:21");
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
fn fighter_does_not_gain_barbarian_level18_recognition() {
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
fn multiclass_barbarian_level18_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL18_FIXTURE.replace(
        "class_level=class:barbarian:18",
        "class_level=class:barbarian:18\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-18 widening -----

#[test]
fn matrix_barbarian_row_names_level_18_widening() {
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
            .contains("sd18_barbarian_level18_widening"),
        "barbarian row must cite the live SD18 level-18 widening proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 18") || note.contains("level-18"),
        "barbarian partial note must name the level-18 widening: {note}"
    );
}
