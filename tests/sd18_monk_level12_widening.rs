//! SD18 Monk level-12 widening grounding proof.
//!
//! Widens the accepted Human Monk level-1..level-11 martial chassis
//! (`tests/sd18_monk_level11_diamond_body.rs`, the loop's most recent Monk
//! ceiling) to Monk level 12 -- mirroring the sibling-class
//! level-range-gate idiom (`supported_monk_level` is generalized from
//! `1..=11` to `1..=12` via `MAX_SUPPORTED_MONK_LEVEL = 12`, exactly as
//! `cycle-2026-07-14T1814` widened `MAX_SUPPORTED_BARBARIAN_LEVEL`,
//! `cycle-2026-07-14T2359` widened `MAX_SUPPORTED_BARD_LEVEL`,
//! `cycle-2026-07-15T0200` widened `MAX_SUPPORTED_CLERIC_LEVEL`,
//! `cycle-2026-07-15T0500` widened `MAX_SUPPORTED_DRUID_LEVEL`, and
//! `cycle-2026-07-14T2300` widened `MAX_SUPPORTED_FIGHTER_LEVEL`, all from
//! 11 to 12). Both PF1 CRB primary sources (d20pfsrd and the Archives of
//! Nethys aonprd.com Monk class table) were read directly before writing
//! any code or test:
//!
//! - level 12 base attack bonus is +9 (`12 * 3 / 4 = 9`, genuinely risen
//!   from +8 at level 11).
//! - all three base saves genuinely rise to +8 (all good, `12 / 2 + 2 = 8`,
//!   up from +7 at level 11).
//! - the unarmed strike damage die genuinely steps up from 1d10 to 2d6 (the
//!   2d6 band spans monk levels 12-15 per both primary sources) -- grounded
//!   as two facets, mirroring the Flurry of Blows attack-bonus/attack-count
//!   split: the die-face facet (6, i.e. d6) and a NEW die-count facet
//!   (2), since every level 1-11 band was always a single die and this is
//!   the first level at which the count itself rises.
//! - the Flurry of Blows flat attack modifier genuinely rises to +10 (monk
//!   level - 2) while the attack count stays 3 (the next count change lands
//!   at 15th).
//! - the ki pool genuinely rises to 9 (`12 / 2 + Wisdom modifier 3 = 9`, up
//!   from 8 at level 11).
//! - Slow Fall's reach genuinely rises to 60 ft (the level-12 "Special"
//!   column names "slow fall 60 ft." explicitly).
//! - the PF1 Core Rulebook Monk class table's level-12 "Special" column
//!   reads "Abundant step, slow fall 60 ft." (verified independently
//!   against both primary sources). Abundant Step is checked and confirmed
//!   NOT flat this slice: it requires both a ki-point-spending
//!   action-economy engine and a dimension-door-equivalent
//!   teleportation-resolution engine, neither of which exists anywhere in
//!   this codebase -- mirroring exactly how Wholeness of Body and High Jump
//!   were left named-but-unproven rather than fabricated. No record or
//!   diagnostic is added for it.
//! - Still Mind, Evasion, Improved Evasion, Purity of Body, and Diamond
//!   Body all stay granted +0 identity records; the level-1 bonus-feat
//!   choice recognition still fires and its execution burden still
//!   claim-blocks.
//!
//! It deliberately does not touch the ki-power execution burden, the repeat
//! bonus-feat grants, High Jump, Wholeness of Body, Abundant Step's own
//! execution, the ki-strike (lawful) DR-bypass property, or any
//! fall-damage/attack-resolution engine (all stay named-but-unproven,
//! unchanged from levels 1-11), and it does not ground Monk level 13+. It
//! also preserves the accepted Monk level-1..level-11 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const MONK_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level11_sd18_diamond_body_deterministic_input.txt"
);

const MONK_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level12_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_IMPROVED_EVASION_ID: &str = "class_feature.monk.improved_evasion";
const MONK_PURITY_OF_BODY_ID: &str = "class_chassis.monk.purity_of_body";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";
const MONK_DIAMOND_BODY_ID: &str = "class_chassis.monk.diamond_body";
const MONK_UNARMED_DIE_ID: &str = "class_chassis.monk.unarmed_strike_damage_die";
const MONK_UNARMED_DIE_COUNT_ID: &str = "class_chassis.monk.unarmed_strike_damage_die_count";

// ----- Base attack bonus and base saves genuinely rise at level 12 -----

#[test]
fn monk_level12_base_attack_and_saves_genuinely_rise() {
    let input = load(MONK_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Monk level 12 3/4-BAB progression (12 * 3 / 4) must equal 9, genuinely risen from 8 \
         at level 11: {}",
        base_attack.detail
    );

    for (id, label) in [
        ("class_chassis.monk.base_save.fortitude", "Fortitude"),
        ("class_chassis.monk.base_save.reflex", "Reflex"),
        ("class_chassis.monk.base_save.will", "Will"),
    ] {
        let save = explanation(&computation, id);
        assert_eq!(
            save.value, 8,
            "Monk level 12 good {label} (12/2+2) must genuinely rise to 8, up from 7 at level \
             11"
        );
    }
}

// ----- Unarmed strike die genuinely steps up to 2d6; ki pool rises -----

#[test]
fn monk_level12_unarmed_die_steps_up_and_ki_pool_rises() {
    let input = load(MONK_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, MONK_UNARMED_DIE_ID);
    assert_eq!(
        unarmed.value, 6,
        "Monk level 12 unarmed strike die face must be 6 (2d6, up from the 1d10 band at levels \
         8-11): {}",
        unarmed.detail
    );
    assert!(
        unarmed.detail.contains("2d6"),
        "Monk level 12 unarmed strike detail must name 2d6: {}",
        unarmed.detail
    );

    let unarmed_count = explanation(&computation, MONK_UNARMED_DIE_COUNT_ID);
    assert_eq!(
        unarmed_count.value, 2,
        "Monk level 12 unarmed strike die count must genuinely rise to 2 (the first level at \
         which the die count itself rises, not just the face size): {}",
        unarmed_count.detail
    );

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 10,
        "Monk level 12 ki pool (12/2 + Wisdom modifier 4) must genuinely rise to 10, up from 9 \
         at level 11: {}",
        ki_pool.detail
    );
}

// ----- Flurry of Blows attack bonus rises; attack count stays 3 -----

#[test]
fn monk_level12_flurry_attack_bonus_rises() {
    let input = load(MONK_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_bonus",
    );
    assert_eq!(
        attack_bonus.value, 10,
        "Monk level 12 Flurry flat attack modifier (monk level - 2) must equal 10, genuinely \
         risen from 9 at level 11: {}",
        attack_bonus.detail
    );

    let attack_count = explanation(
        &computation,
        "class_chassis.monk.flurry_of_blows_attack_count",
    );
    assert_eq!(
        attack_count.value, 3,
        "Monk level 12 Flurry attack count must stay 3 (the next count change lands at 15th)"
    );
}

// ----- Slow Fall's reach genuinely rises to 60 ft -----

#[test]
fn monk_level12_slow_fall_reach_rises() {
    let input = load(MONK_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall must stay a bounded grant-only identity record (value 0, non-fabricated): {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.contains("60"),
        "Monk level 12 Slow Fall reach must genuinely rise to 60 ft: {}",
        slow_fall.detail
    );
}

// ----- Abundant Step's caster-level magnitude was later grounded by task #49 -----

#[test]
fn monk_level12_abundant_step_caster_level_was_later_grounded() {
    // At the time this file's own slice landed, Abundant Step was checked
    // and confirmed not flat (it requires a ki-point-spending
    // action-economy engine and a teleportation-resolution engine, neither
    // of which exists in this codebase) and deliberately left unfabricated.
    // Task #49 (2026-07-28) revisited it: the ONE flat magnitude the
    // feature's own rule text names -- "his caster level for this effect is
    // equal to his monk level" -- is grounded standalone (mirroring the
    // Wholeness of Body / High Jump precedent), while the dimension-door
    // execution itself stays unmodeled. This is the same "was later
    // widened" flip `tests/sd13_monk_level9_progression.rs` established for
    // its own superseded boundary.
    let input = load(MONK_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let abundant_step = explanation(&computation, "class_chassis.monk.abundant_step_caster_level");
    assert_eq!(
        abundant_step.value, 12,
        "Monk level 12 Abundant Step caster level must equal monk level (12): {}",
        abundant_step.detail
    );
}

// ----- Remaining granted features and choices carry over unchanged -----

#[test]
fn monk_level12_granted_features_and_choices_carry_over() {
    let input = load(MONK_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(still_mind.value, 2, "Still Mind must stay the flat +2 at level 12");

    for id in [
        MONK_EVASION_ID,
        MONK_IMPROVED_EVASION_ID,
        MONK_PURITY_OF_BODY_ID,
        MONK_DIAMOND_BODY_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 12"
        );
    }

    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(
        choice.value, 0,
        "the level-1 bonus-feat choice recognition must still fire at level 12"
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
            && d.claim_blocking),
        "level-12 Monk must still claim-block on the bonus-feat execution burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-11 fixture is unaffected by this widening -----

#[test]
fn monk_level11_truth_is_unchanged_by_this_slice() {
    let input = load(MONK_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Monk level 11 base attack bonus must stay 8");

    let unarmed = explanation(&computation, MONK_UNARMED_DIE_ID);
    assert_eq!(unarmed.value, 10, "Monk level 11 unarmed strike die must stay 1d10");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == MONK_UNARMED_DIE_COUNT_ID),
        "level-11 Monk must not gain the new level-12 unarmed strike die count facet"
    );

    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(ki_pool.value, 9, "Monk level 11 ki pool must stay 9");
}

// ----- Level 13 was later widened into the supported tranche by task #49 -----

#[test]
fn monk_level_13_was_later_widened_into_the_supported_tranche() {
    let level_13 = MONK_LEVEL12_FIXTURE.replace("class:monk:12", "class:monk:13");
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
fn fighter_does_not_gain_monk_level12_recognition() {
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
fn multiclass_monk_level12_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL12_FIXTURE.replace(
        "class_level=class:monk:12",
        "class_level=class:monk:12\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_monk_row_names_level_12_widening() {
    let matrix = seeded_current_truth();
    let monk = matrix
        .row("class.monk.bounded_progression")
        .expect("monk bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(monk.support_state, SupportState::Supported);
    assert_eq!(monk.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(monk.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        monk.grounding_ref.contains("sd18_monk_level12_widening"),
        "monk row must cite the live SD18 level-12 widening proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "monk partial note must name the level-12 widening: {note}"
    );
}
