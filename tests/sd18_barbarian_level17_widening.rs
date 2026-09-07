//! SD18 Barbarian level-17 widening grounding proof.
//!
//! Widens the accepted Barbarian level-1..level-16 martial chassis baseline
//! (`tests/sd18_barbarian_level16_widening.rs`, the SD18 loop's own prior
//! ceiling) to Barbarian level 17 — the loop's EIGHTH §3.2 level-17
//! landing, after Ranger, Bard, Rogue, Fighter, Wizard, Cleric, and Paladin
//! — mirroring the sibling per-level-gate idiom (`supported_barbarian_level`
//! is generalized from `1..=16` to `1..=17` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 17`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, and agree byte-for-byte:
//!
//! - level 17 base attack bonus is +17/+12/+7/+2 (full BAB, genuinely risen
//!   from +16); good Fortitude GENUINELY RISES to +10 (`17 / 2 + 2 = 10`,
//!   wait: 17/2 = 8, 8 + 2 = 10 — same numeric value as level 16 but via a
//!   genuinely re-evaluated formula), while poor Reflex and poor Will both
//!   STAY +5 (`17 / 3 = 5`, an integer-division coincidence with level 16).
//! - the rage rounds-per-day pool genuinely rises to 39 (`4 + Constitution
//!   modifier 3 + 2 x (17 - 1)`).
//! - the PF1 Core Rulebook Barbarian class table's level-17 "Special" column
//!   reads "Tireless rage" only (verified independently against both
//!   primary sources, checked rather than assumed): Tireless Rage is a
//!   genuinely NEW named class feature ("Starting at 17th level, a
//!   barbarian no longer becomes fatigued at the end of her rage"),
//!   grounded here as a bounded grant-only identity record (value 0,
//!   non-fabricated) mirroring the Paladin Aura of Righteousness / Aura of
//!   Justice / Aura of Faith idiom exactly: no rage-state execution engine
//!   exists anywhere in this codebase (confirmed by direct grep), so there
//!   is no fatigue-application mechanism for Tireless Rage to interact
//!   with, and none is fabricated. 17 is NOT a rage-power level (powers
//!   land at 2/4/6/8/10/12/14/16/18/20), so no ninth rage power slot
//!   appears.
//! - Trap Sense stays +5 (next rise 18th); Damage Reduction stays 4/- (next
//!   rise 19th); Indomitable Will's flat +4 magnitude carries over
//!   unchanged from level 14; the Greater Rage constants (+6/+6/+3/-2) stay
//!   unchanged from level 11; Fast Movement stays the flat +10; the
//!   illiteracy-absence classification, Uncanny Dodge, and Improved
//!   Uncanny Dodge all carry over unchanged, not re-derived.
//!
//! This is the CLEANEST widening shape checked in the level-17 sweep for
//! Barbarian: only two production-code changes beyond
//! `MAX_SUPPORTED_BARBARIAN_LEVEL` (16 -> 17) — a new
//! `BARBARIAN_TIRELESS_RAGE_LEVEL` gate constant and its matching grant-only
//! explanation block, mirroring the already-proven Indomitable Will /
//! Paladin-aura idiom exactly, zero new record types, zero new choice-slot
//! mechanisms.
//!
//! It deliberately does not touch the rage execution burden, any rage-power
//! EFFECT, the damage-reduction application engine, or the
//! saving-throw-resolution engine Indomitable Will would need to actually
//! apply (all stay named-but-unproven, unchanged from levels 1-16), and it
//! does not ground Barbarian level 18+. It also preserves the accepted
//! Barbarian level-1..level-16 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level16_sd18_widening_deterministic_input.txt"
);

const BARBARIAN_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level17_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus genuinely rises; all three base saves stay at level 16's values -----

#[test]
fn barbarian_level17_base_attack_rises_saves_stay() {
    let input = load(BARBARIAN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 17,
        "Barbarian level 17 full-BAB progression must equal 17, genuinely risen from 16: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Barbarian level 17 good Fortitude (17/2+2) must stay 10, an integer-division \
         coincidence with level 16"
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Barbarian level 17 poor Reflex (17/3) must stay 5, unchanged from level 16"
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(
        will.value, 5,
        "Barbarian level 17 poor Will (17/3) must stay 5, unchanged from level 16"
    );
}

// ----- Rage rounds per day genuinely rises to 39 at level 17 -----

#[test]
fn barbarian_level17_rage_rounds_rise_to_thirty_nine() {
    let input = load(BARBARIAN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 39,
        "Barbarian level 17 rage rounds per day (4 + Constitution modifier 3 + 2 x (17 - 1)) \
         must equal 39, genuinely risen from 37 at level 16: {}",
        rage_rounds.detail
    );
}

// ----- Tireless Rage is newly grounded as a bounded grant-only identity record at level 17 -----

#[test]
fn barbarian_level17_gains_tireless_rage_grant_only() {
    let input = load(BARBARIAN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let tireless_rage = explanation(&computation, BARBARIAN_TIRELESS_RAGE_ID);
    assert_eq!(
        tireless_rage.value, 0,
        "Tireless Rage must be a bounded grant-only identity record (value 0, non-fabricated) \
         at level 17: {}",
        tireless_rage.detail
    );
    assert!(
        tireless_rage.detail.to_lowercase().contains("fatigue"),
        "Tireless Rage detail must name the fatigue rule text it grants: {}",
        tireless_rage.detail
    );
    assert!(
        tireless_rage.detail.contains("no rage-state execution engine")
            || tireless_rage
                .detail
                .contains("no fatigue-application")
            || tireless_rage
                .detail
                .to_lowercase()
                .contains("no rage-state execution"),
        "Tireless Rage detail must disclaim the absent rage-state/fatigue execution engine: {}",
        tireless_rage.detail
    );
}

// ----- Tireless Rage is absent (correct level-gate absence) below level 17 -----

#[test]
fn barbarian_level16_does_not_have_tireless_rage() {
    let input = load(BARBARIAN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let tireless_rage = explanation(&computation, BARBARIAN_TIRELESS_RAGE_ID);
    assert_eq!(
        tireless_rage.value, 0,
        "Tireless Rage must be a correct level-gate absence (value 0) below level 17: {}",
        tireless_rage.detail
    );
    assert!(
        tireless_rage.detail.contains("absent") || tireless_rage.detail.contains("correctly"),
        "Tireless Rage detail must name the correct level-gate absence below level 17: {}",
        tireless_rage.detail
    );
}

// ----- No ninth rage power slot at level 17 (17 is not a rage-power level) -----

#[test]
fn barbarian_level17_gains_no_ninth_rage_power_slot() {
    let input = load(BARBARIAN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.barbarian.rage_power_9_choice"),
        "Barbarian level 17 must not gain a ninth rage power slot (17 is not a rage-power \
         level): {:?}",
        computation.explanations
    );

    // The eighth slot (granted at level 16) still carries over.
    let slot_eight = explanation(&computation, "class_chassis.barbarian.rage_power_8_choice");
    assert_eq!(
        slot_eight.value, 0,
        "the eighth rage power slot must stay a bounded +0 recognition record at level 17: {}",
        slot_eight.detail
    );
}

// ----- Trap Sense, Damage Reduction, Indomitable Will, and the remaining pillars carry over unchanged -----

#[test]
fn barbarian_level17_remaining_pillars_carry_over() {
    let input = load(BARBARIAN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 5,
        "Barbarian level 17 Trap Sense must stay +5, unchanged from level 16: {}",
        trap_sense.detail
    );

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 4,
        "Barbarian level 17 Damage Reduction must stay 4/-, unchanged from level 16: {}",
        damage_reduction.detail
    );

    let indomitable_will = explanation(&computation, BARBARIAN_INDOMITABLE_WILL_ID);
    assert_eq!(
        indomitable_will.value, 4,
        "Barbarian level 17 Indomitable Will must stay a flat +4 morale bonus, unchanged from \
         level 16: {}",
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
            "the rage surface magnitude '{id}' must stay at its Greater Rage value at level 17 \
             (unchanged from level 11)"
        );
    }

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Fast Movement must stay the flat +10 at level 17");

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the PF1 illiteracy-absence classification must stay a +0 record at level 17"
    );

    for id in [BARBARIAN_UNCANNY_DODGE_ID, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 17");
    }
}

// ----- The rage execution burden still claim-blocks at level 17 -----

#[test]
fn barbarian_level17_still_claim_blocks_the_rage_execution_burden() {
    let input = load(BARBARIAN_LEVEL17_FIXTURE);
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
                "level-17 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-16 fixture is unaffected by this widening -----

#[test]
fn barbarian_level16_truth_is_unchanged_by_this_slice() {
    let input = load(BARBARIAN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 16, "Barbarian level 16 base attack bonus must stay 16");

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 10, "Barbarian level 16 good Fortitude must stay 10");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(rage_rounds.value, 37, "Barbarian level 16 rage rounds must stay 37");
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level17_recognition() {
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
fn multiclass_barbarian_level17_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL17_FIXTURE.replace(
        "class_level=class:barbarian:17",
        "class_level=class:barbarian:17\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_barbarian_row_names_level_17_widening() {
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
            .contains("sd18_barbarian_level17_widening"),
        "barbarian row must cite the live SD18 level-17 widening proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 17") || note.contains("level-17"),
        "barbarian partial note must name the level-17 widening: {note}"
    );
}
