//! Task #49 Monk level-20 capstone widening grounding proof.
//!
//! Widens the accepted Human Monk level-1..level-12 martial chassis
//! (`tests/sd18_monk_level12_widening.rs`, the previous Monk ceiling) all
//! the way to Monk level 20 -- the full PF1 Core Rulebook capstone range --
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_monk_level` is generalized from `1..=12` to `1..=20` via
//! `MAX_SUPPORTED_MONK_LEVEL = 20`, `pilot_compute.rs`;
//! `class_tables.rs`'s own mirrored `CLASS_META` row is widened
//! identically). Both PF1 CRB primary sources (d20pfsrd and the Archives of
//! Nethys aonprd.com mirror) were read directly this session before writing
//! any code or test.
//!
//! This single commit does two things together, per the task brief's own
//! required sequencing (the formula fixes landed FIRST, as their own
//! commit, provably inert while the cap was still 12):
//!
//! 1. Re-verifies and extends three already-shipped formulas through their
//!    newly-reachable upper bands: the unarmed strike damage die (2d8 at
//!    16-19, 2d10 at 20 -- `min(5, MonkLVL/4)` proving it stops there);
//!    Flurry of Blows' fourth attack at 15th level (Greater Two-Weapon
//!    Fighting); and Slow Fall's reach through 90 ft (18th), becoming
//!    unlimited ("fall any distance without harm", the feature's own rule
//!    text) at 20th rather than a naive 100-ft extrapolation.
//! 2. Grounds seven capstone-band features: Abundant Step (12th -- already
//!    inside the OLD 1..=12 range, a pre-existing gap rather than one this
//!    widening itself admits, built here since nobody built it while the
//!    cap already covered it), Diamond Soul (13th, spell resistance = 10 +
//!    monk level), Quivering Palm (15th, DC = 10 + 1/2 monk level + Wisdom
//!    modifier, duration = monk level days), Timeless Body (17th,
//!    grant-only), Tongue of the Sun and Moon (17th, grant-only), Empty
//!    Body (19th, grant-only -- its 3-ki-point cost and 1-minute duration
//!    are fixed, not level-scaled), and Perfect Self (20th, DR
//!    10/chaotic -- previously provably dead code under the old
//!    `MAX_SUPPORTED_MONK_LEVEL = 12` ceiling, since level 20 could never
//!    be reached at all; genuinely reachable and grounded fresh here).
//!
//! It deliberately does not touch the recognized bonus feat's own
//! execution mechanics, the level-2/level-6 repeat bonus-feat grant
//! recognition, Wholeness of Body's/Abundant Step's/Quivering Palm's/Empty
//! Body's own execution engines, Diamond Soul's spell-resistance-check
//! application, or Perfect Self's Outsider-type clause (no numeric
//! magnitude, needs a creature-type-conditioned resolution engine that
//! does not exist here -- stays deferred regardless of level, unchanged
//! from the earlier superseded attempt's own conclusion).

use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const MONK_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level12_sd18_widening_deterministic_input.txt"
);

const MONK_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level20_sd49_capstone_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

fn find<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> Option<&'a ComputationExplanation> {
    computation.explanations.iter().find(|e| e.id == id)
}

// ----- Base attack bonus and base saves genuinely rise at level 20 -----

#[test]
fn monk_level20_base_attack_and_saves_genuinely_rise() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 15,
        "Monk level 20 3/4-BAB progression (20 * 3 / 4) must equal 15: {}",
        base_attack.detail
    );

    for id in [
        "class_chassis.monk.base_save.fortitude",
        "class_chassis.monk.base_save.reflex",
        "class_chassis.monk.base_save.will",
    ] {
        let save = explanation(&computation, id);
        assert_eq!(save.value, 12, "Monk level 20 good save (20/2+2) must equal 12");
    }
}

// ----- The three re-verified formulas reach their final bands -----

#[test]
fn monk_level20_unarmed_strike_die_reaches_its_final_2d10_band() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(unarmed.value, 10, "Monk level 20 unarmed strike die face must be 10 (2d10)");
    assert!(unarmed.detail.contains("2d10"), "{}", unarmed.detail);

    let unarmed_count =
        explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die_count");
    assert_eq!(unarmed_count.value, 2, "Monk level 20 unarmed strike die count stays 2 (2d10)");
}

#[test]
fn monk_level20_flurry_of_blows_reaches_its_fourth_attack() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_count = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_count",
    );
    assert_eq!(
        attack_count.value, 4,
        "Monk level 20 Flurry attack count must genuinely rise to 4 (the fourth attack lands \
         at 15th, Greater Two-Weapon Fighting)"
    );

    let attack_bonus = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_bonus",
    );
    assert_eq!(attack_bonus.value, 18, "Monk level 20 Flurry flat attack modifier is 20 - 2 = 18");
}

#[test]
fn monk_level20_slow_fall_reach_is_genuinely_unlimited() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, "class_chassis.monk.slow_fall");
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall must stay a bounded grant-only identity record (value 0, non-fabricated): {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.contains("any distance"),
        "Monk level 20 Slow Fall must state the reach is unlimited (\"fall any distance without \
         harm\"), not a fabricated finite number: {}",
        slow_fall.detail
    );
    assert!(
        !slow_fall.detail.contains("100 feet") && !slow_fall.detail.contains("100-foot"),
        "Monk level 20 Slow Fall must NOT fabricate a naive 100-ft extrapolation: {}",
        slow_fall.detail
    );
}

// ----- Abundant Step's caster-level magnitude (12th, pre-existing gap) -----

#[test]
fn monk_level20_abundant_step_caster_level_equals_monk_level() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let abundant_step = explanation(&computation, "class_chassis.monk.abundant_step_caster_level");
    assert_eq!(
        abundant_step.value, 20,
        "Monk level 20 Abundant Step caster level must equal monk level: {}",
        abundant_step.detail
    );
}

#[test]
fn monk_level12_abundant_step_caster_level_equals_monk_level() {
    // The pre-existing gap: level 12 was already inside the OLD 1..=12
    // range, so this is not something task #49's cap widening itself
    // admits -- it is grounded here alongside the other six capstones.
    let input = load(MONK_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let abundant_step = explanation(&computation, "class_chassis.monk.abundant_step_caster_level");
    assert_eq!(abundant_step.value, 12, "Monk level 12 Abundant Step caster level must equal 12");
}

// ----- Diamond Soul (13th): spell resistance = 10 + monk level -----

#[test]
fn monk_level20_diamond_soul_spell_resistance() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let diamond_soul = explanation(&computation, "class_chassis.monk.diamond_soul_spell_resistance");
    assert_eq!(diamond_soul.value, 30, "Monk level 20 Diamond Soul SR must be 10 + 20 = 30");
}

#[test]
fn monk_level13_diamond_soul_is_newly_granted() {
    let level_13 = MONK_LEVEL12_FIXTURE.replace("class:monk:12", "class:monk:13");
    let input = load(&level_13);
    let computation = compute_pilot_base_chassis(&input);

    let diamond_soul = explanation(&computation, "class_chassis.monk.diamond_soul_spell_resistance");
    assert_eq!(diamond_soul.value, 23, "Monk level 13 Diamond Soul SR must be 10 + 13 = 23");
    assert!(diamond_soul.detail.contains("granted at monk level 13"), "{}", diamond_soul.detail);
}

// ----- Quivering Palm (15th): DC and duration -----

#[test]
fn monk_level20_quivering_palm_dc_and_duration() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // WIS 17 base + 2 Human racial (choice=human_ability_bonus:wisdom) = 19 -> +4 modifier.
    let dc = explanation(&computation, "class_chassis.monk.quivering_palm_dc");
    assert_eq!(dc.value, 24, "Monk level 20 Quivering Palm DC = 10 + 20/2 + 4 = 24: {}", dc.detail);

    let duration = explanation(&computation, "class_chassis.monk.quivering_palm_duration_days");
    assert_eq!(duration.value, 20, "Monk level 20 Quivering Palm duration must equal monk level");
}

#[test]
fn monk_level14_quivering_palm_is_correctly_absent() {
    let level_14 = MONK_LEVEL12_FIXTURE.replace("class:monk:12", "class:monk:14");
    let input = load(&level_14);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, "class_chassis.monk.quivering_palm_dc");
    assert_eq!(dc.value, 0, "Monk level 14 Quivering Palm DC must be the correct absence (0)");
    assert!(dc.detail.contains("correctly absent"), "{}", dc.detail);

    let duration = explanation(&computation, "class_chassis.monk.quivering_palm_duration_days");
    assert_eq!(duration.value, 0, "Monk level 14 Quivering Palm duration must be the correct absence (0)");
}

// ----- Timeless Body, Tongue of the Sun and Moon, Empty Body: grant-only -----

#[test]
fn monk_level20_timeless_body_tongue_and_empty_body_are_grant_only_records() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_chassis.monk.timeless_body",
        "class_chassis.monk.tongue_of_the_sun_and_moon",
        "class_chassis.monk.empty_body",
    ] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must be a bounded grant-only identity record (value 0)");
        assert!(
            record.detail.contains("granted at monk level 20"),
            "'{id}' must cite the real grant text at level 20: {}",
            record.detail
        );
    }
}

#[test]
fn monk_level16_timeless_body_and_tongue_are_correctly_absent() {
    let level_16 = MONK_LEVEL12_FIXTURE.replace("class:monk:12", "class:monk:16");
    let input = load(&level_16);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_chassis.monk.timeless_body",
        "class_chassis.monk.tongue_of_the_sun_and_moon",
    ] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' absence record must still be value 0");
        assert!(record.detail.contains("correctly absent"), "{}", record.detail);
    }
}

#[test]
fn monk_level18_empty_body_is_correctly_absent() {
    let level_18 = MONK_LEVEL12_FIXTURE.replace("class:monk:12", "class:monk:18");
    let input = load(&level_18);
    let computation = compute_pilot_base_chassis(&input);

    let empty_body = explanation(&computation, "class_chassis.monk.empty_body");
    assert_eq!(empty_body.value, 0);
    assert!(empty_body.detail.contains("correctly absent"), "{}", empty_body.detail);
}

// ----- Perfect Self (20th): damage reduction 10/chaotic -----

#[test]
fn monk_level20_perfect_self_grants_damage_reduction_ten_chaotic() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let perfect_self = explanation(&computation, "class_chassis.monk.perfect_self_damage_reduction");
    assert_eq!(perfect_self.value, 10, "Monk level 20 Perfect Self must grant DR 10: {}", perfect_self.detail);
    assert!(perfect_self.detail.contains("10/chaotic"), "{}", perfect_self.detail);
    assert!(
        perfect_self.detail.contains("outsider") && perfect_self.detail.contains("deferred"),
        "Perfect Self's Outsider-type clause must be named as deferred, not fabricated: {}",
        perfect_self.detail
    );
}

#[test]
fn monk_level19_perfect_self_is_correctly_absent() {
    let level_19 = MONK_LEVEL12_FIXTURE.replace("class:monk:12", "class:monk:19");
    let input = load(&level_19);
    let computation = compute_pilot_base_chassis(&input);

    let perfect_self = explanation(&computation, "class_chassis.monk.perfect_self_damage_reduction");
    assert_eq!(perfect_self.value, 0, "Monk level 19 Perfect Self DR must be the correct absence (0)");
    assert!(perfect_self.detail.contains("correctly absent"), "{}", perfect_self.detail);
}

// ----- Below every capstone gate: level 11 grounds none of the seven -----

#[test]
fn monk_level11_stays_below_every_capstone_gate() {
    let level_11_fixture = include_str!(
        "fixtures/rules_core/pf1_human_monk_level11_sd18_diamond_body_deterministic_input.txt"
    );
    let input = load(level_11_fixture);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_chassis.monk.abundant_step_caster_level",
        "class_chassis.monk.diamond_soul_spell_resistance",
        "class_chassis.monk.quivering_palm_dc",
        "class_chassis.monk.quivering_palm_duration_days",
        "class_chassis.monk.timeless_body",
        "class_chassis.monk.tongue_of_the_sun_and_moon",
        "class_chassis.monk.empty_body",
        "class_chassis.monk.perfect_self_damage_reduction",
    ] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must be the correct absence (0) at level 11");
        assert!(record.detail.contains("correctly absent"), "'{id}': {}", record.detail);
    }
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level20_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id.starts_with("class_feature.monk.")),
        "the Fighter chassis must not surface any monk-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Monk is not promoted -----

#[test]
fn multiclass_monk_level20_is_not_promoted() {
    let multiclass = MONK_LEVEL20_FIXTURE.replace(
        "class_level=class:monk:20",
        "class_level=class:monk:20\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id.starts_with("class_feature.monk.")),
        "multiclass Monk must not gain any bounded monk explanation: {:?}",
        computation.explanations
    );
}

// ----- Sanity: level 21 (one past the true PF1 capstone) stays unrecognized -----

#[test]
fn monk_level_21_stays_unrecognized() {
    let level_21 = MONK_LEVEL20_FIXTURE.replace("class:monk:20", "class:monk:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id.starts_with("class_feature.monk.")),
        "level-21 Monk (past the true PF1 capstone) must not gain any bounded monk explanation: \
         {:?}",
        computation.explanations
    );
}

// ----- Every prior granted feature carries over unchanged -----

#[test]
fn monk_level20_earlier_granted_features_carry_over() {
    let input = load(MONK_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        "class_feature.monk.evasion",
        "class_feature.monk.improved_evasion",
        "class_chassis.monk.purity_of_body",
        "class_chassis.monk.diamond_body",
    ] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "'{id}' must stay a granted +0 identity record at level 20");
    }

    let still_mind = explanation(&computation, "class_feature.monk.still_mind");
    assert_eq!(still_mind.value, 2, "Still Mind must stay the flat +2 at level 20");

    assert!(
        find(&computation, "class_chassis.monk.wholeness_of_body")
            .is_some_and(|e| e.value == 20),
        "Wholeness of Body healing must genuinely rise to 20 at level 20"
    );
}

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_monk_row_names_level_20_widening() {
    let matrix = seeded_current_truth();
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk bounded_progression row must exist");

    assert_eq!(monk.support_state, SupportState::Supported);
    assert_eq!(monk.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(monk.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        monk.grounding_ref.contains("sd49_monk_level20_capstone"),
        "monk row must cite the live task #49 level-20 capstone proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "monk partial note must name the level-20 widening: {note}"
    );
}
