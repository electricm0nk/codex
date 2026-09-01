//! SD13-E5 Monk level-8 progression grounding proof.
//!
//! Widens the accepted Monk level-1/level-2/level-3/level-4/level-5/level-6/
//! level-7 martial chassis baseline
//! (`tests/sd13_monk_level1_chassis_baseline.rs` through
//! `tests/sd13_monk_level7_progression.rs`) to monk level 8, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Cleric/Druid/Bard/Sorcerer/Ranger/Wizard
//! level-range-gate idiom (`supported_monk_level` is generalized from
//! `1..=7` to `1..=8` via `MAX_SUPPORTED_MONK_LEVEL = 8`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Monk class table) were
//! read directly before writing any code or test: level 8 base attack bonus
//! is +6, all three base saves are +6, unarmed damage genuinely rises to
//! 1d10 (the 1d10 band runs levels 8-11), the Flurry of Blows flat attack
//! bonus improves to +6/+6 and the attack count genuinely rises to three —
//! verified independently against both primary sources' verbatim Flurry of
//! Blows rule text ("At 8th level, the monk can make two additional attacks
//! when he uses flurry of blows, as if using Improved Two-Weapon Fighting"),
//! Slow Fall's own reach magnitude genuinely rises to 40 ft., and the
//! level-8 "Special" column names only the Slow Fall reach rise — checked
//! and specifically confirmed NOT Improved Uncanny Dodge, which Monk never
//! gains at any level per either primary source (unlike other classes'
//! 8th-level tables, which was the exact commonly-repeated wrong assumption
//! this cycle was briefed to verify against before coding). It proves:
//!
//! - base attack bonus at level 8 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-7: `8 * 3 / 4 = 6`.
//! - base saves at level 8 are grounded by the same all-good formula
//!   (`level / 2 + 2`) already grounded at levels 1-7: `8 / 2 + 2 = 6` for
//!   Fortitude, Reflex, and Will alike.
//! - the unarmed strike damage die genuinely rises to `10` (i.e. `1d10`) at
//!   level 8 — confirmed via the same formula, a real value change (not a
//!   new record); the PF1 CRB Medium-monk damage table's 1d10 band runs
//!   levels 8-11.
//! - the Flurry of Blows flat attack bonus is grounded by the same
//!   `level - 2` formula already grounded at levels 1-7, correctly
//!   producing `6` at level 8 (`8 - 2 = 6`, matching the primary sources'
//!   "+6/+6"), and the attack count genuinely rises to `3` at level 8 —
//!   verified independently against both primary sources' verbatim Flurry
//!   of Blows rule text, confirming the third attack IS gained at 8th
//!   level specifically (not level 6 or 7, correcting a prior web-search
//!   error this cycle was briefed to re-verify).
//! - Evasion, Still Mind, the ki pool's flat size, Slow Fall, and Purity of
//!   Body all stay granted at level 8 (not re-derived), grounded as the same
//!   bounded identity/flat-magnitude records already grounded at levels 2,
//!   3, 4, 4, and 5 respectively. Still Mind's own magnitude stays a flat
//!   `+2` (non-level-scaled). The ki pool's flat size genuinely rises to `7`
//!   at level 8 (`8 / 2 + 3 = 7`, up from `6` at level 7). Slow Fall's own
//!   reach magnitude genuinely rises to `40` feet at level 8 (up from `30`
//!   at level 7 — the next rise is at 10th level, beyond this bounded
//!   slice).
//!
//! This cycle was specifically briefed to check whether Monk gains an
//! actual new class feature at 8th level per the PF1 CRB class table's
//! "Special" column: verified independently against d20pfsrd and
//! legacy.aonprd.com, the level-8 row's "Special" column names only "Slow
//! fall 40 ft." — a rise in the already-grounded Slow Fall record's own
//! reach magnitude, not a brand-new class feature. Both primary sources
//! were checked specifically for Improved Uncanny Dodge (a
//! commonly-repeated but WRONG assumption for Monk, carried over from other
//! classes' 8th-level tables): neither source lists it anywhere on the Monk
//! class table at any level, so no such record is grounded or fabricated
//! here. Wholeness of Body (granted at level 7) stays granted-but-unexecuted,
//! unchanged.
//!
//! It deliberately does not implement Fast Movement or Maneuver Training
//! (unchanged from level 3, still named-but-unproven), the recognized
//! bonus feat's own mechanics (unchanged from level 1), the level-2/level-6
//! bonus feat grants themselves, High Jump's own Acrobatics-check-total/
//! ki-point-spending mechanics, Wholeness of Body's own execution, any
//! ki-power execution engine, any fall-damage-resolution engine, any
//! disease-resolution engine, and it does not ground Monk level 9+. It also
//! preserves the accepted Monk level-1 through level-7 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const MONK_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level7_sd13_deterministic_input.txt");

const MONK_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level8_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";
const MONK_PURITY_OF_BODY_ID: &str = "class_chassis.monk.purity_of_body";

// ----- Base attack bonus at level 8 -----

#[test]
fn monk_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Monk level 8 3/4-BAB progression (8 * 3 / 4) must equal 6: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "monk base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 (all three good) -----

#[test]
fn monk_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.monk.base_save.fortitude");
    assert_eq!(fortitude.value, 6, "Monk level 8 good Fortitude (8/2+2) must equal 6");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "monk Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.monk.base_save.reflex");
    assert_eq!(reflex.value, 6, "Monk level 8 good Reflex (8/2+2) must equal 6");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "monk Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.monk.base_save.will");
    assert_eq!(will.value, 6, "Monk level 8 good Will (8/2+2) must equal 6");
    assert!(
        will.detail.to_lowercase().contains("good"),
        "monk Will explanation must name it as a good save: {}",
        will.detail
    );
}

// ----- Unarmed strike damage die genuinely rises to 1d10 at level 8 -----

#[test]
fn monk_level8_unarmed_strike_damage_die_rises_to_one_d10() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 10,
        "Monk level 8 unarmed strike damage die must genuinely rise to 10 (i.e. 1d10): {}",
        unarmed.detail
    );
    assert!(
        unarmed.detail.contains("1d10"),
        "monk unarmed-strike explanation must name the 1d10 damage die at level 8: {}",
        unarmed.detail
    );
}

// ----- Flurry of Blows flat attack bonus/count at level 8 -----

#[test]
fn monk_level8_flurry_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_bonus");
    assert_eq!(
        attack_bonus.value, 6,
        "Monk level 8 Flurry of Blows flat attack modifier (8 - 2) must equal 6, matching the \
         PF1 CRB table's +6/+6 at level 8: {}",
        attack_bonus.detail
    );
}

#[test]
fn monk_level8_flurry_attack_count_rises_to_three() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_count = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_count");
    assert_eq!(
        attack_count.value, 3,
        "Monk level 8 Flurry of Blows attack count must genuinely rise to 3, per both primary \
         sources' verbatim Flurry of Blows rule text (\"At 8th level, the monk can make two \
         additional attacks\"): {}",
        attack_count.detail
    );
}

// ----- Evasion, Still Mind, ki pool, Slow Fall, and Purity of Body all stay granted at
// level 8 -----

#[test]
fn monk_level8_keeps_evasion_grounded() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, MONK_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value at level 8: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 8 must state it is granted, not absent: {}",
        evasion.detail
    );
}

#[test]
fn monk_level8_keeps_still_mind_grounded_unchanged() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(
        still_mind.value, 2,
        "Still Mind must keep its flat +2 magnitude grounded, unchanged at level 8: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("granted"),
        "still mind explanation at level 8 must state it is granted, not absent: {}",
        still_mind.detail
    );
}

#[test]
fn monk_level8_ki_pool_rises_to_seven() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 8,
        "Monk level 8 ki pool (1/2 monk level + Wisdom modifier = 8/2 + 4) must equal 8, \
         genuinely rising from 7 at level 7: {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("granted"),
        "ki pool explanation at level 8 must state it is granted, not absent: {}",
        ki_pool.detail
    );
}

#[test]
fn monk_level8_keeps_purity_of_body_grounded() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let purity = explanation(&computation, MONK_PURITY_OF_BODY_ID);
    assert_eq!(
        purity.value, 0,
        "Purity of Body must carry no fabricated mechanical value at level 8: {}",
        purity.detail
    );
    assert!(
        purity.detail.to_lowercase().contains("granted"),
        "purity of body explanation at level 8 must state it is granted, not absent: {}",
        purity.detail
    );
}

#[test]
fn monk_level8_slow_fall_reach_rises_to_forty_feet() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall must carry no fabricated mechanical value at level 8: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.contains("40 feet") || slow_fall.detail.to_lowercase().contains("40 ft"),
        "slow fall explanation at level 8 must name the genuinely-risen 40-foot reach: {}",
        slow_fall.detail
    );
}

// ----- Wholeness of Body is still deliberately NOT fabricated at level 8 -----

/// Wholeness of Body IS grounded from task #36 onward. The earlier guard
/// asserted the OLD bar, demanding a healing engine that the corrected
/// bar (risks item 52) no longer requires for a standalone magnitude.
#[test]
fn monk_level8_grounds_wholeness_of_body_as_a_flat_record() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fact = computation
        .explanations
        .iter()
        .find(|e| e.id == "class_chassis.monk.wholeness_of_body")
        .expect("Wholeness of Body must be grounded at monk level 8");
    assert_eq!(fact.value, 8, "heals MonkLVL hp: {:?}", fact);
}

// ----- High Jump is still deliberately NOT grounded at level 8 -----

/// High Jump IS grounded from task #36 onward. The earlier guard here
/// asserted the OLD standalone-grounding bar, and was stale on two
/// counts: it demanded an "Acrobatics-check-total engine" that the
/// corrected bar (risks item 52) no longer requires for a standalone
/// fact, and it cited the ki cost as blocking the whole ability when the
/// corpus record's own DESC scopes the ki spend to a SEPARATE +20 clause.
/// The flat +MonkLVL is unconditional; only the +20 boost stays deferred.
#[test]
fn monk_level8_grounds_high_jump_as_a_flat_record() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fact = computation
        .explanations
        .iter()
        .find(|e| e.id == "class_chassis.monk.high_jump")
        .expect("High Jump must be grounded at monk level 8");
    assert_eq!(fact.value, 8, "High Jump is a flat +MonkLVL: {:?}", fact);
}

// ----- No Improved Uncanny Dodge / new class-feature record is fabricated at level 8 -----

#[test]
fn monk_level8_does_not_fabricate_an_improved_uncanny_dodge_record() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("uncanny_dodge")),
        "Monk must not fabricate an Improved Uncanny Dodge record at level 8 (Monk never gains \
         it at any level per either primary source): {:?}",
        computation.explanations
    );
}

// ----- No new bonus-feat choice-slot is fabricated for the level-8 grant -----

#[test]
fn monk_level8_does_not_fabricate_a_second_bonus_feat_choice_slot() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        computation
            .explanations
            .iter()
            .filter(|e| e.id == "class_chassis.monk.bonus_feat_choice")
            .count(),
        1,
        "level 8 must not fabricate a second bonus-feat-choice explanation record: {:?}",
        computation.explanations
    );
}

// ----- The existing bonus-feat-mechanics diagnostic still fires at level 8 -----

#[test]
fn monk_level8_still_claim_blocks_the_recognized_bonus_feat_mechanics() {
    let input = load(MONK_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
                && d.claim_blocking),
        "level-8 Monk must still claim-block on the recognized bonus feat's own mechanics: {:?}",
        computation.diagnostics
    );
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn monk_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = MONK_LEVEL8_FIXTURE.replace("class:monk:8", "class:monk:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")),
        "level-9 Monk is now recognized by the later level-9 widening slice \
         (tests/sd13_monk_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-7 fixture is unaffected by this widening -----

#[test]
fn monk_level7_truth_is_unchanged_by_this_slice() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_bonus");
    assert_eq!(
        attack_bonus.value, 5,
        "Monk level 7 Flurry of Blows flat attack modifier must stay 5, unaffected by the \
         level-8 widening: {}",
        attack_bonus.detail
    );

    let attack_count = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_count");
    assert_eq!(
        attack_count.value, 2,
        "Monk level 7 Flurry of Blows attack count must stay 2, unaffected by the level-8 \
         widening: {}",
        attack_count.detail
    );

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 8,
        "Monk level 7 unarmed strike damage die must stay 8 (1d8), unaffected by the level-8 \
         widening: {}",
        unarmed.detail
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level8_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id == MONK_EVASION_ID
                || e.id == MONK_STILL_MIND_ID),
        "the Fighter chassis must not surface any monk-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Monk is not promoted -----

#[test]
fn multiclass_monk_level8_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL8_FIXTURE.replace(
        "class_level=class:monk:8",
        "class_level=class:monk:8\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")
                || e.id == MONK_EVASION_ID
                || e.id == MONK_STILL_MIND_ID),
        "multiclass Monk must not gain any bounded monk chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Monk must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_monk_row_names_level_8_widening() {
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
        monk.grounding_ref.contains("sd13_monk_level8_progression"),
        "monk row must cite the live SD13-E5 level-8 proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.contains("40 ft") || note.contains("40 feet"),
        "monk partial note must name the level-8 Slow Fall reach rise: {note}"
    );
    assert!(
        note.contains("bonus feat"),
        "monk partial note must keep naming the bonus feat's own mechanics as unproven: {note}"
    );
}
