//! SD13-E5 Monk level-10 progression grounding proof.
//!
//! Widens the accepted Monk level-1..level-9 martial chassis baseline (most
//! recently `tests/sd13_monk_level9_progression.rs`) to Monk level 10 — the
//! tranche's declared ceiling — mirroring the sibling-class level-range-gate
//! idiom (`supported_monk_level` is generalized from `1..=9` to `1..=10` via
//! `MAX_SUPPORTED_MONK_LEVEL = 10`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Monk class table) were read directly before writing
//! any code or test:
//!
//! - level 10 base attack bonus is +7 (`10 * 3 / 4 = 7`, genuinely risen
//!   from +6 at level 9 — the class table's own "+7/+2" iterative notation
//!   is not modeled anywhere in this codebase, only the flat base value)
//!   and all three base saves are +7 (all good, `10 / 2 + 2 = 7`, all
//!   genuinely risen from +6) — confirmed by the same formulas already
//!   grounded at levels 1-9, not re-derived.
//! - the unarmed strike damage die STAYS 1d10 (the 1d10 band spans monk
//!   levels 8-11 per both primary sources).
//! - the Flurry of Blows flat attack modifier GENUINELY RISES to +8 (monk
//!   level - 2) while the attack count STAYS 3 (the next count change lands
//!   at 15th via the Greater Two-Weapon Fighting upgrade).
//! - the ki pool GENUINELY RISES to 8 (`10 / 2 + Wisdom modifier 3`), and
//!   Slow Fall's reach GENUINELY RISES to 50 ft (the class table's level-10
//!   "Special" column names "slow fall 50 ft." explicitly), via a new
//!   50-ft tier on the same grant-only identity record (task #49 later
//!   unified this and every other Slow Fall reach gate into
//!   `monk_slow_fall_reach_feet`'s single floor(MonkLVL/2)*10 formula).
//! - Still Mind stays the flat +2; Purity of Body, Evasion, and Improved
//!   Evasion stay granted +0 identity records; the level-1 bonus-feat
//!   choice recognition still fires and its execution burden still
//!   claim-blocks.
//! - the level-10 "Special" column's OTHER two entries (verified
//!   independently against both primary sources, checked rather than
//!   assumed away): the repeat "Bonus feat" grant stays named-but-unproven
//!   exactly like the level-2/level-6 repeat grants before it (no repeat
//!   bonus-feat recognition exists), and "ki pool (lawful)" — the ki
//!   strike upgrade treating unarmed strikes as lawful weapons for
//!   overcoming damage reduction — needs a DR/attack-resolution engine that
//!   does not exist in this codebase, so it stays named-but-unproven too,
//!   mirroring how the 4th-level magic and 7th-level cold-iron/silver ki
//!   strike properties were never fabricated either. A dedicated negative
//!   test pins that no ki-strike record or diagnostic is fabricated.
//!
//! It deliberately does not touch the ki-power execution burden, the repeat
//! bonus-feat grants, High Jump, Wholeness of Body, or any
//! fall-damage/attack-resolution engine (all stay named-but-unproven,
//! unchanged from levels 1-9), and it does not ground Monk level 11+. It
//! also preserves the accepted Monk level-1..level-9 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const MONK_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level9_sd13_deterministic_input.txt");

const MONK_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_IMPROVED_EVASION_ID: &str = "class_feature.monk.improved_evasion";
const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_PURITY_OF_BODY_ID: &str = "class_chassis.monk.purity_of_body";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";

// ----- Base attack bonus and saves at level 10 -----

#[test]
fn monk_level10_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(MONK_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Monk level 10 3/4-BAB progression (10 * 3 / 4) must equal 7, genuinely risen from 6 \
         at level 9: {}",
        base_attack.detail
    );

    for (id, label) in [
        ("class_chassis.monk.base_save.fortitude", "Fortitude"),
        ("class_chassis.monk.base_save.reflex", "Reflex"),
        ("class_chassis.monk.base_save.will", "Will"),
    ] {
        let save = explanation(&computation, id);
        assert_eq!(
            save.value, 7,
            "Monk level 10 good {label} (10/2+2) must equal 7, genuinely risen from 6"
        );
    }
}

// ----- Unarmed die stays 1d10; Flurry bonus rises to +8, count stays 3 -----

#[test]
fn monk_level10_unarmed_die_stays_and_flurry_bonus_rises() {
    let input = load(MONK_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 10,
        "Monk level 10 unarmed strike die must stay 1d10 (the band spans levels 8-11): {}",
        unarmed.detail
    );

    let attack_bonus = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_bonus",
    );
    assert_eq!(
        attack_bonus.value, 8,
        "Monk level 10 Flurry flat attack modifier (monk level - 2) must equal 8, genuinely \
         risen from 7 at level 9: {}",
        attack_bonus.detail
    );

    let attack_count = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_count",
    );
    assert_eq!(
        attack_count.value, 3,
        "Monk level 10 Flurry attack count must stay 3 (the next count change lands at 15th)"
    );
}

// ----- Ki pool rises to 8, Slow Fall reach rises to 50 ft at level 10 -----

#[test]
fn monk_level10_ki_pool_and_slow_fall_both_rise() {
    let input = load(MONK_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 9,
        "Monk level 10 ki pool (10/2 + Wisdom modifier 4) must equal 9, genuinely risen from \
         8 at level 9: {}",
        ki_pool.detail
    );

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall recognition must carry no fabricated mechanical value at level 10"
    );
    assert!(
        slow_fall.detail.contains("50"),
        "Slow Fall's reach must rise to 50 ft at level 10 (named explicitly in the class \
         table's level-10 Special column): {}",
        slow_fall.detail
    );
}

// ----- Granted features and bonus-feat recognition carry over at level 10 -----

#[test]
fn monk_level10_granted_features_and_choices_carry_over() {
    let input = load(MONK_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(still_mind.value, 2, "Still Mind must stay the flat +2 at level 10");

    for id in [MONK_EVASION_ID, MONK_IMPROVED_EVASION_ID, MONK_PURITY_OF_BODY_ID] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 10"
        );
    }

    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(
        choice.value, 0,
        "the level-1 bonus-feat choice recognition must still fire at level 10"
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
            && d.claim_blocking),
        "level-10 Monk must still claim-block on the bonus-feat execution burden: {:?}",
        computation.diagnostics
    );
}

// ----- Ki strike (lawful) stays entirely named-but-unproven at level 10 -----

#[test]
fn monk_level10_does_not_fabricate_ki_strike() {
    let input = load(MONK_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("ki_strike")
                || e.id.to_lowercase().contains("lawful")),
        "level-10 Monk must not fabricate any ki-strike record (the lawful DR-bypass property \
         needs a DR/attack-resolution engine that does not exist in this codebase): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("ki_strike")),
        "level-10 Monk must not fabricate any ki-strike diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-9 fixture is unaffected by this widening -----

#[test]
fn monk_level9_truth_is_unchanged_by_this_slice() {
    let input = load(MONK_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(base_attack.value, 6, "Monk level 9 base attack bonus must stay 6");

    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(ki_pool.value, 8, "Monk level 9 ki pool must stay 8");

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert!(
        slow_fall.detail.contains("40"),
        "Monk level 9 Slow Fall reach must stay 40 ft: {}",
        slow_fall.detail
    );
}

// ----- Level 13 was later widened into the supported tranche (task #49) -----

#[test]
fn monk_level_13_was_later_widened_into_the_supported_tranche() {
    let level_13 = MONK_LEVEL10_FIXTURE.replace("class:monk:10", "class:monk:13");
    let input = load(&level_13);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")),
        "level-13 Monk is now recognized by the later task #49 level-20-capstone widening \
         slice (tests/sd49_monk_level20_capstone.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level10_recognition() {
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
fn multiclass_monk_level10_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL10_FIXTURE.replace(
        "class_level=class:monk:10",
        "class_level=class:monk:10\nclass_level=class:fighter:1",
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
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Monk must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-10 widening -----

#[test]
fn matrix_monk_row_names_level_10_widening() {
    let matrix = seeded_current_truth();
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(monk.support_state, SupportState::Supported);
    assert_eq!(monk.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        monk.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        monk.grounding_ref.contains("sd13_monk_level10_progression"),
        "monk row must cite the live SD13-E5 level-10 proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "monk partial note must name the level-10 widening: {note}"
    );
}
