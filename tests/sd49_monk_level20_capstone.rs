//! Task #49: `MAX_SUPPORTED_MONK_LEVEL` widened 12 -> 20, admitting the whole
//! Monk capstone band.
//!
//! Every gate and magnitude asserted here was read directly off the PCGen
//! corpus (`cr_abilities_class.lst`), not from remembered rule text. Four of
//! the seven features carry real magnitudes; three are genuinely grant-only
//! (no numeric token in their own records), and are asserted as value-0
//! identity records rather than given fabricated numbers.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};

const MONK_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level20_sd49_capstone_deterministic_input.txt"
);
const MONK_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level12_sd18_widening_deterministic_input.txt"
);

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result.character_input.expect("valid fixture")
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("expected explanation id '{id}'"))
}

fn has(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

/// The widening's precondition: a level-20 Monk must be recognized at all.
/// Before this task `supported_monk_level` returned `None` above 12, so a
/// level-20 Monk grounded no Monk record whatsoever.
#[test]
fn a_level_twenty_monk_is_recognized_at_all() {
    let computation = compute_pilot_base_chassis(&load(MONK_LEVEL20_FIXTURE));
    assert!(
        has(&computation, "class_chassis.monk.bounded_progression"),
        "a level-20 Monk must now be recognized -- the cap widening's whole point"
    );
}

/// Diamond Soul: `DiamondSoul = 10 + DiamondSoulLVL`, `DiamondSoulLVL = MonkLVL`.
#[test]
fn diamond_soul_grounds_spell_resistance_from_thirteenth_level() {
    let computation = compute_pilot_base_chassis(&load(MONK_LEVEL20_FIXTURE));
    assert_eq!(explanation(&computation, "class_chassis.monk.diamond_soul").value, 30);

    let level12 = compute_pilot_base_chassis(&load(MONK_LEVEL12_FIXTURE));
    assert!(
        !has(&level12, "class_chassis.monk.diamond_soul")
            || explanation(&level12, "class_chassis.monk.diamond_soul").value == 0,
        "Diamond Soul must not grant spell resistance below its level-13 gate"
    );
}

/// Quivering Palm: DC `10+(QuiveringPalmLVL/2)+WIS`, duration `QuiveringPalmLVL`.
/// The fixture's Wisdom 17 +2 Human racial = 19, a +4 modifier, so the DC is
/// 10 + 10 + 4 = 24 at level 20.
#[test]
fn quivering_palm_grounds_its_save_dc_and_duration() {
    let computation = compute_pilot_base_chassis(&load(MONK_LEVEL20_FIXTURE));
    assert_eq!(explanation(&computation, "class_chassis.monk.quivering_palm_save_dc").value, 24);
    assert_eq!(
        explanation(&computation, "class_chassis.monk.quivering_palm_duration_days").value,
        20
    );
}

/// Abundant Step's caster level for its dimension door effect equals the monk
/// level. Gate 12 -- inside the OLD cap already, so this is a pre-existing gap
/// rather than one the widening admits.
#[test]
fn abundant_step_grounds_its_caster_level_at_both_twelve_and_twenty() {
    let level20 = compute_pilot_base_chassis(&load(MONK_LEVEL20_FIXTURE));
    assert_eq!(
        explanation(&level20, "class_chassis.monk.abundant_step_caster_level").value,
        20
    );

    let level12 = compute_pilot_base_chassis(&load(MONK_LEVEL12_FIXTURE));
    assert_eq!(
        explanation(&level12, "class_chassis.monk.abundant_step_caster_level").value,
        12,
        "Abundant Step's gate is 12, already inside the pre-widening range"
    );
}

/// Perfect Self's DR clause. Its granting branch only becomes reachable once
/// the cap admits level 20 -- while the cap was 12 it was correctly left
/// unwritten rather than shipped as dead code.
#[test]
fn perfect_self_now_grants_its_real_damage_reduction() {
    let computation = compute_pilot_base_chassis(&load(MONK_LEVEL20_FIXTURE));
    let dr = explanation(&computation, "class_chassis.monk.perfect_self_damage_reduction");
    assert_eq!(dr.value, 10, "DR 10/chaotic at level 20: {}", dr.detail);
    assert!(
        dr.detail.to_lowercase().contains("chaotic"),
        "must name its /chaotic bypass: {}",
        dr.detail
    );
    assert!(
        dr.detail.to_lowercase().contains("outsider"),
        "must still name the deferred Outsider-type clause: {}",
        dr.detail
    );
}

/// The three grant-only capstones carry no numeric token in the corpus, so
/// they ground as value-0 identity records rather than fabricated magnitudes.
#[test]
fn the_three_grant_only_capstones_ground_as_zero_valued_identity_records() {
    let computation = compute_pilot_base_chassis(&load(MONK_LEVEL20_FIXTURE));
    for id in [
        "class_chassis.monk.timeless_body",
        "class_chassis.monk.tongue_of_the_sun_and_moon",
        "class_chassis.monk.empty_body",
    ] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, 0, "{id} carries no corpus magnitude: {}", record.detail);
    }
}

/// The three formulas fixed in this task's first commit, now actually
/// reachable: the unarmed die's final 2d10 band, flurry's fourth attack, and
/// Slow Fall's unlimited case.
#[test]
fn the_previously_capped_formulas_read_their_level_twenty_values() {
    let computation = compute_pilot_base_chassis(&load(MONK_LEVEL20_FIXTURE));

    assert_eq!(
        explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die").value,
        10,
        "2d10 at level 20 -- the die-face facet"
    );
    assert_eq!(
        explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die_count").value,
        2,
        "2d10 at level 20 -- the die-count facet"
    );
    assert_eq!(
        explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_count").value,
        4,
        "flurry gains its fourth attack at 15th level"
    );

    let slow_fall = explanation(&computation, "class_chassis.monk.slow_fall");
    assert!(
        slow_fall.detail.contains("any distance"),
        "Slow Fall at 20 is unlimited, not a finite 100 ft: {}",
        slow_fall.detail
    );
}

/// A level-12 Monk's already-shipped records must be untouched by the
/// widening -- the regression guard for every level the cap already covered.
#[test]
fn widening_the_cap_does_not_disturb_level_twelve() {
    let computation = compute_pilot_base_chassis(&load(MONK_LEVEL12_FIXTURE));
    for (id, want) in [
        ("class_chassis.monk.base_attack_bonus", 9),
        ("class_chassis.monk.ki_pool_size", 10),
        ("class_chassis.monk.fast_movement", 40),
        ("class_chassis.monk.high_jump", 12),
        ("class_chassis.monk.wholeness_of_body", 12),
        ("class_chassis.monk.unarmed_strike_damage_die", 6),
        ("class_chassis.monk.flurry_of_blows_attack_count", 3),
    ] {
        assert_eq!(explanation(&computation, id).value, want, "{id} at level 12");
    }
}
