//! SD18 Barbarian level-19 widening grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-18 martial chassis baseline
//! (`tests/sd18_barbarian_level18_widening.rs`, the SD18 loop's own prior
//! ceiling) to Barbarian level 19 — the loop's FIRST §3.2 level-19 landing,
//! opening the level-19 sweep — mirroring the sibling per-level-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=18` to `1..=19`
//! via `MAX_SUPPORTED_BARBARIAN_LEVEL = 19`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, fetching the full
//! levels-15-through-20 block in one pass so the level-19 row's neighbors
//! were visible in context (guards against level-misattribution), and agree
//! byte-for-byte at every level in that block — no third source was
//! required:
//!
//! - level 19 base attack bonus is +19/+14/+9/+4 (full BAB, genuinely risen
//!   from +18); good Fortitude stays +11 (`19 / 2 + 2 = 11`, an
//!   integer-division coincidence with level 18), and poor Reflex and poor
//!   Will both stay +6 (`19 / 3 = 6`, also an integer-division coincidence
//!   with level 18).
//! - the rage rounds-per-day pool genuinely rises to 43 (`4 + Constitution
//!   modifier 3 + 2 x (19 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-19 "Special" column
//!   reads "Damage reduction 5/-" only (verified independently against both
//!   primary sources, checked rather than assumed): Damage Reduction
//!   GENUINELY RISES to 5/- via a FIFTH tier constant, mirroring exactly
//!   how the level-10/level-13/level-16 three-prior-tier idiom was
//!   established (the "10th level and every three barbarian levels
//!   thereafter" cadence: 10, 13, 16, 19); 19 is NOT a rage-power level
//!   (powers land at 2/4/6/8/10/12/14/16/18/20), so no tenth numbered slot
//!   is added.
//! - Trap Sense stays +6 (19/3, next rise would be 21st, outside the PF1
//!   1-20 level range); Indomitable Will's flat +4 magnitude carries over
//!   unchanged from level 14; Tireless Rage carries over unchanged from
//!   level 17; the Greater Rage constants (+6/+6/+3/-2) stay unchanged from
//!   level 11; Fast Movement stays the flat +10; the illiteracy-absence
//!   classification, Uncanny Dodge, and Improved Uncanny Dodge all carry
//!   over unchanged, not re-derived.
//!
//! This is another clean widening shape: only one production-code change
//! beyond `MAX_SUPPORTED_BARBARIAN_LEVEL` (18 -> 19) — a fifth Damage
//! Reduction tier constant and one new arm on the existing flat-magnitude
//! formula (base attack, all three base saves, rage rounds, and Trap Sense
//! all needed ZERO formula changes, since they were already level-generic
//! formulas) — zero new record types, zero new choice-slot mechanisms, zero
//! new subsystems.
//!
//! It deliberately does not touch the rage execution burden, any rage-power
//! EFFECT, the damage-reduction application engine, or the
//! saving-throw-resolution engine Indomitable Will would need to actually
//! apply (all stay named-but-unproven, unchanged from levels 1-18), and it
//! does not ground Barbarian level 20. It also preserves the accepted
//! Barbarian level-1..level-18 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level18_sd18_widening_deterministic_input.txt"
);

const BARBARIAN_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level19_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus genuinely rises at level 19; base saves stay put -----

#[test]
fn barbarian_level19_base_attack_rises_and_saves_stay() {
    let input = load(BARBARIAN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 19,
        "Barbarian level 19 full-BAB progression must equal 19, genuinely risen from 18: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 11,
        "Barbarian level 19 good Fortitude (19/2+2) must stay 11, unchanged from level 18 \
         (an integer-division coincidence)"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Barbarian level 19 poor Reflex (19/3) must stay 6, unchanged from level 18 (an \
         integer-division coincidence)"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 6,
        "Barbarian level 19 poor Will (19/3) must stay 6, unchanged from level 18 (an \
         integer-division coincidence)"
    );
}

// ----- Rage rounds per day genuinely rises to 43 at level 19 -----

#[test]
fn barbarian_level19_rage_rounds_rise_to_forty_three() {
    let input = load(BARBARIAN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 43,
        "Barbarian level 19 rage rounds per day (4 + Constitution modifier 3 + 2 x (19 - 1)) \
         must equal 43, genuinely risen from 41 at level 18: {}",
        rage_rounds.detail
    );
}

// ----- Damage Reduction genuinely rises to 5/- at level 19 -----

#[test]
fn barbarian_level19_damage_reduction_rises_to_five() {
    let input = load(BARBARIAN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 5,
        "Barbarian level 19 Damage Reduction must genuinely rise to 5/-, up from 4/- at level \
         18: {}",
        damage_reduction.detail
    );
}

// ----- No tenth rage power slot is granted at level 19 -----

#[test]
fn barbarian_level19_does_not_gain_a_tenth_rage_power_slot() {
    let input = load(BARBARIAN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.barbarian.rage_power_10_choice"),
        "Barbarian level 19 must not gain a tenth rage power slot (19 is not a rage-power \
         level): {:?}",
        computation.explanations
    );

    // The ninth slot (granted at level 18) still carries over.
    let slot_nine = explanation(&computation, "class_chassis.barbarian.rage_power_9_choice");
    assert_eq!(
        slot_nine.value, 0,
        "the ninth rage power slot must stay a bounded +0 recognition record at level 19: {}",
        slot_nine.detail
    );
}

// ----- Remaining pillars carry over unchanged -----

#[test]
fn barbarian_level19_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 6,
        "Barbarian level 19 Trap Sense must stay +6, unchanged from level 18 (next rise 21st, \
         outside the PF1 1-20 level range): {}",
        trap_sense.detail
    );

    let indomitable_will = explanation(&computation, BARBARIAN_INDOMITABLE_WILL_ID);
    assert_eq!(
        indomitable_will.value, 4,
        "Barbarian level 19 Indomitable Will must stay a flat +4 morale bonus, unchanged from \
         level 18: {}",
        indomitable_will.detail
    );

    let tireless_rage = explanation(&computation, BARBARIAN_TIRELESS_RAGE_ID);
    assert_eq!(
        tireless_rage.value, 0,
        "Barbarian level 19 Tireless Rage must stay a bounded grant-only identity record \
         (value 0), carried over unchanged from level 18: {}",
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
            "the rage surface magnitude '{id}' must stay at its Greater Rage value at level 19 \
             (unchanged from level 11)"
        );
    }

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 19");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 19"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 19");
    }
}

// ----- The rage execution burden still claim-blocks at level 19 -----

#[test]
fn barbarian_level19_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL19_FIXTURE);
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
                "level-19 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-18 fixture is unaffected by this widening -----

#[test]
fn barbarian_level18_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 18, "Barbarian level 18 base attack bonus must stay 18");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 41, "Barbarian level 18 rage rounds must stay 41");

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(damage_reduction.value, 4, "Barbarian level 18 Damage Reduction must stay 4/-");

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 6, "Barbarian level 18 Trap Sense must stay +6");
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level19_recognition() {
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
fn multiclass_barbarian_level19_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL19_FIXTURE.replace(
        "class_level=class:barbarian:19",
        "class_level=class:barbarian:19\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-19 widening -----

#[test]
fn matrix_barbarian_row_names_level_19_widening() {
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
            .contains("sd18_barbarian_level19_widening"),
        "barbarian row must cite the live SD18 level-19 widening proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 19") || note.contains("level-19"),
        "barbarian partial note must name the level-19 widening: {note}"
    );
}
