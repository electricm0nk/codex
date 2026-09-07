//! SD18 Barbarian level-20 widening grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-19 martial chassis baseline
//! (`tests/sd18_barbarian_level19_widening.rs`, the SD18 loop's own prior
//! ceiling) to Barbarian level 20 — the FINAL level within PF1's 1-20
//! character-level cap, opening the loop's §3.2 level-20 sweep's Barbarian
//! landing — mirroring the sibling per-level-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=19` to `1..=20`
//! via `MAX_SUPPORTED_BARBARIAN_LEVEL = 20`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, fetching the full class table
//! (both give the identical byte-for-byte class-table row) — no third
//! source was required:
//!
//! - level 20 base attack bonus is +20/+15/+10/+5 (full BAB, genuinely
//!   risen from +19); good Fortitude genuinely rises to +12 (`20 / 2 + 2`,
//!   up from +11), and poor Reflex and poor Will both stay +6 (`20 / 3 =
//!   6`, an integer-division coincidence with level 19).
//! - the rage rounds-per-day pool genuinely rises to 45 (`4 + Constitution
//!   modifier 3 + 2 x (20 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-20 "Special"
//!   column reads "Mighty rage, Rage power" (verified independently against
//!   both primary sources, checked rather than assumed): Mighty Rage is a
//!   genuine THIRD tier on the SAME flat rage-surface constants already
//!   grounded at level 1 (Rage) and widened at level 11 (Greater Rage) —
//!   the Strength/Constitution morale bonus rises from +6 to +8 and the
//!   Will-save morale bonus rises from +3 to +4 — mirroring the Greater
//!   Rage precedent exactly, via a new `BARBARIAN_MIGHTY_RAGE_LEVEL` gate
//!   constant; level 20 IS a rage-power level (powers land at
//!   2/4/6/8/10/12/14/16/18/20), so a TENTH and FINAL numbered slot
//!   (`choice:barbarian_rage_power_10`) is appended to
//!   `BARBARIAN_RAGE_POWER_SLOTS`, mirroring the proven repeat-grant idiom
//!   exactly.
//! - Damage Reduction stays 5/- (next rise would be 22nd, outside the PF1
//!   1-20 level range); Trap Sense stays +6 (20/3, next rise would be
//!   21st, also outside range); Indomitable Will's flat +4 magnitude
//!   carries over unchanged from level 14; Tireless Rage carries over
//!   unchanged from level 17; Fast Movement stays the flat +10; the
//!   illiteracy-absence classification, Uncanny Dodge, and Improved
//!   Uncanny Dodge all carry over unchanged, not re-derived.
//!
//! This is another clean widening shape: only production-code changes
//! beyond `MAX_SUPPORTED_BARBARIAN_LEVEL` (19 -> 20) — a new Mighty Rage
//! magnitude tier and a tenth numbered rage-power slot (base attack, all
//! three base saves, rage rounds, Trap Sense, and Damage Reduction all
//! needed ZERO formula changes, since they were already level-generic
//! formulas or already at their final tier) — zero new record types, zero
//! new choice-slot mechanisms, zero new subsystems. This closes the
//! Barbarian per-level arithmetic-widening frontier: level 20 is the final
//! level within PF1's 1-20 character-level cap, so no further per-level
//! widening cycle remains for this row.
//!
//! It deliberately does not touch the rage execution burden, any rage-power
//! EFFECT, the damage-reduction application engine, or the
//! saving-throw-resolution engine Indomitable Will would need to actually
//! apply (all stay named-but-unproven, unchanged from levels 1-19). It also
//! preserves the accepted Barbarian level-1..level-19 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level19_sd18_widening_deterministic_input.txt"
);

const BARBARIAN_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level20_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus genuinely rises at level 20; Fortitude rises, Reflex/Will stay -----

#[test]
fn barbarian_level20_base_attack_and_fortitude_rise() {
    let input = load(BARBARIAN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 20,
        "Barbarian level 20 full-BAB progression must equal 20, genuinely risen from 19: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 12,
        "Barbarian level 20 good Fortitude (20/2+2) must genuinely rise to 12, up from 11 at \
         level 19: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Barbarian level 20 poor Reflex (20/3) must stay 6, unchanged from level 19 (an \
         integer-division coincidence)"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 6,
        "Barbarian level 20 poor Will (20/3) must stay 6, unchanged from level 19 (an \
         integer-division coincidence)"
    );
}

// ----- Rage rounds per day genuinely rises to 45 at level 20 -----

#[test]
fn barbarian_level20_rage_rounds_rise_to_forty_five() {
    let input = load(BARBARIAN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 45,
        "Barbarian level 20 rage rounds per day (4 + Constitution modifier 3 + 2 x (20 - 1)) \
         must equal 45, genuinely risen from 43 at level 19: {}",
        rage_rounds.detail
    );
}

// ----- Mighty Rage genuinely rises the rage magnitude constants to +8/+8/+4 -----

#[test]
fn barbarian_level20_mighty_rage_rises_magnitudes() {
    let input = load(BARBARIAN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 8),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 8),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 4),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "the rage surface magnitude '{id}' must genuinely rise to its Mighty Rage value at \
             level 20 (up from the Greater Rage value at level 11): {}",
            record.detail
        );
    }
}

// ----- Damage Reduction, Trap Sense stay at their final tier at level 20 -----

#[test]
fn barbarian_level20_damage_reduction_and_trap_sense_stay_put() {
    let input = load(BARBARIAN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 5,
        "Barbarian level 20 Damage Reduction must stay 5/- (next rise would be 22nd, outside \
         the PF1 1-20 level range): {}",
        damage_reduction.detail
    );

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 6,
        "Barbarian level 20 Trap Sense must stay +6 (next rise would be 21st, outside the PF1 \
         1-20 level range): {}",
        trap_sense.detail
    );
}

// ----- A tenth and FINAL rage power slot is granted at level 20 -----

#[test]
fn barbarian_level20_gains_a_tenth_rage_power_slot() {
    let input = load(BARBARIAN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_ten = explanation(&computation, "class_chassis.barbarian.rage_power_10_choice");
    assert_eq!(
        slot_ten.value, 0,
        "the tenth rage power slot must be a bounded +0 recognition record at level 20: {}",
        slot_ten.detail
    );

    // The ninth slot (granted at level 18) still carries over.
    let slot_nine = explanation(&computation, "class_chassis.barbarian.rage_power_9_choice");
    assert_eq!(
        slot_nine.value, 0,
        "the ninth rage power slot must stay a bounded +0 recognition record at level 20: {}",
        slot_nine.detail
    );
}

// ----- Remaining pillars carry over unchanged -----

#[test]
fn barbarian_level20_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let indomitable_will = explanation(&computation, BARBARIAN_INDOMITABLE_WILL_ID);
    assert_eq!(
        indomitable_will.value, 4,
        "Barbarian level 20 Indomitable Will must stay a flat +4 morale bonus, unchanged from \
         level 14: {}",
        indomitable_will.detail
    );

    let tireless_rage = explanation(&computation, BARBARIAN_TIRELESS_RAGE_ID);
    assert_eq!(
        tireless_rage.value, 0,
        "Barbarian level 20 Tireless Rage must stay a bounded grant-only identity record \
         (value 0), carried over unchanged from level 17: {}",
        tireless_rage.detail
    );

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 20");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 20"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 20");
    }
}

// ----- The rage execution burden still claim-blocks at level 20 -----

#[test]
fn barbarian_level20_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL20_FIXTURE);
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
                "level-20 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-19 fixture is unaffected by this widening -----

#[test]
fn barbarian_level19_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 19, "Barbarian level 19 base attack bonus must stay 19");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 43, "Barbarian level 19 rage rounds must stay 43");

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(damage_reduction.value, 5, "Barbarian level 19 Damage Reduction must stay 5/-");

    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 6),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 6),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 3),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "Barbarian level 19 rage surface magnitude '{id}' must stay at its Greater Rage \
             value (Mighty Rage does not apply below level 20)"
        );
    }

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.barbarian.rage_power_10_choice"),
        "Barbarian level 19 must not gain a tenth rage power slot (19 is not a rage-power \
         level): {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 21 stays unrecognized by this slice (no such PF1 level) -----

#[test]
fn barbarian_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARBARIAN_LEVEL20_FIXTURE.replace("class:barbarian:20", "class:barbarian:21");
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
fn fighter_does_not_gain_barbarian_level20_recognition() {
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
fn multiclass_barbarian_level20_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL20_FIXTURE.replace(
        "class_level=class:barbarian:20",
        "class_level=class:barbarian:20\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_barbarian_row_names_level_20_widening() {
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
            .contains("sd18_barbarian_level20_widening"),
        "barbarian row must cite the live SD18 level-20 widening proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "barbarian partial note must name the level-20 widening: {note}"
    );
}
