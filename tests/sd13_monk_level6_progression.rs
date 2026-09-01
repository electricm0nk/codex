//! SD13-E5 Monk level-6 progression grounding proof.
//!
//! Widens the accepted Monk level-1/level-2/level-3/level-4/level-5 martial
//! chassis baseline (`tests/sd13_monk_level1_chassis_baseline.rs`,
//! `tests/sd13_monk_level2_progression.rs`,
//! `tests/sd13_monk_level3_progression.rs`,
//! `tests/sd13_monk_level4_progression.rs`,
//! `tests/sd13_monk_level5_progression.rs`) to monk level 6, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Cleric/Druid/Bard/Sorcerer/Ranger/Wizard
//! level-range-gate idiom (`supported_monk_level` is generalized from
//! `1..=5` to `1..=6` via `MAX_SUPPORTED_MONK_LEVEL = 6`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Monk class table) were
//! read directly before writing any code or test: level 6 base attack bonus
//! is +4, all three base saves are +5, unarmed damage stays 1d8 (the 1d8
//! band runs levels 4-7), the Flurry of Blows attack bonus improves to
//! +4/+4 (still two attacks — verified against both primary sources'
//! verbatim Flurry of Blows rule text, which states the monk gains a third
//! attack only "at 8th level"), and the level-6 special feature list is
//! "Bonus feat, slow fall 30 ft." It proves:
//!
//! - base attack bonus at level 6 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-5: `6 * 3 / 4 = 4`.
//! - base saves at level 6 are grounded by the same all-good formula
//!   (`level / 2 + 2`) already grounded at levels 1-5: `6 / 2 + 2 = 5` for
//!   Fortitude, Reflex, and Will alike.
//! - the unarmed strike damage die stays `8` (i.e. `1d8`) at level 6 —
//!   confirmed via the same formula/value, not a new record; the PF1 CRB
//!   Medium-monk damage table's 1d8 band runs levels 4-7.
//! - the Flurry of Blows flat attack bonus is grounded by the same
//!   `level - 2` formula already grounded at levels 1-5, correctly
//!   producing `4` at level 6 (`6 - 2 = 4`, matching the primary sources'
//!   "+4/+4"), and the attack count stays `2` at level 6 — verified
//!   independently against both primary sources' verbatim Flurry of Blows
//!   rule text ("At 8th level, the monk can make two additional attacks
//!   when he uses flurry of blows"), confirming the third attack is NOT
//!   gained at level 6, confirmed via the same formula, not a new record.
//! - Evasion, Still Mind, the ki pool's flat size, and Purity of Body all
//!   stay granted at level 6 (not re-derived), grounded as the same bounded
//!   identity/flat-magnitude records already grounded at levels 2, 3, 4,
//!   and 5. The ki pool's flat size genuinely rises to `6` at level 6
//!   (`6 / 2 + 3 = 6`), up from `5` at levels 4-5.
//! - Slow Fall stays granted at level 6 (not re-derived as a new record),
//!   but its own reach magnitude genuinely rises from 20 ft to 30 ft at
//!   level 6 (verified independently against d20pfsrd and
//!   legacy.aonprd.com: the Monk class table's level-6 "Special" column
//!   reads "Bonus feat, slow fall 30 ft."), mirroring the Rogue Trap Sense
//!   flat-magnitude-increase idiom — the record's own `value` field stays
//!   `0` (still a bounded grant-only identity record; no fall-damage-
//!   resolution engine exists in this codebase to apply any reduction to),
//!   only the descriptive reach figure in the detail text is level-accurate.
//!
//! This cycle was specifically briefed to check whether Monk gains an
//! actual new class feature at 6th level per the PF1 CRB class table's
//! "Special" column: verified independently against d20pfsrd and
//! legacy.aonprd.com, the level-6 row's "Special" column reads "Bonus feat,
//! slow fall 30 ft." — Slow Fall's own reach increase is the pre-existing
//! grant's own genuine rise (grounded above, not a new record), while the
//! "Bonus feat" entry is a second, separate repeat bonus-feat choice-list
//! grant (PF1 grants monks bonus feats at 1st, 2nd, 6th, and 10th level,
//! all drawn from the same restricted list already recognized at level 1).
//! This slice deliberately does not implement it (an open-ended
//! choice-list grant, not a flat arithmetic burden), mirroring exactly the
//! level-2 monk bonus-feat precedent and the Rogue level-6 "second Rogue
//! Talent slot" precedent: no new choice-slot and no new diagnostic was
//! added for it. Improved Trip, named in the operator brief as a candidate
//! for a possible new 6th-level feature, was checked and confirmed to be
//! merely one of the five feats already recognized as a possible *choice*
//! for this (and every other) Monk bonus feat grant, not a new automatic
//! class feature.
//!
//! It deliberately does not implement Fast Movement or Maneuver Training
//! (unchanged from level 3, still named-but-unproven), the recognized
//! bonus feat's own mechanics (unchanged from level 1), the level-2/level-6
//! bonus feat grants themselves, High Jump's own Acrobatics-check-total/
//! ki-point-spending mechanics, any ki-power execution engine, any
//! fall-damage-resolution engine, any disease-resolution engine, any
//! attack-resolution or damage-resolution engine, and it does not ground
//! Monk level 7+. It also preserves the accepted Monk level-1/level-2/
//! level-3/level-4/level-5 truth (unchanged), the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const MONK_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level5_sd13_deterministic_input.txt");

const MONK_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level6_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";
const MONK_PURITY_OF_BODY_ID: &str = "class_chassis.monk.purity_of_body";

// ----- Base attack bonus at level 6 -----

#[test]
fn monk_level6_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 4,
        "Monk level 6 3/4-BAB progression (6 * 3 / 4) must equal 4: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "monk base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 6 (all three good) -----

#[test]
fn monk_level6_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.monk.base_save.fortitude");
    assert_eq!(fortitude.value, 5, "Monk level 6 good Fortitude (6/2+2) must equal 5");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "monk Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.monk.base_save.reflex");
    assert_eq!(reflex.value, 5, "Monk level 6 good Reflex (6/2+2) must equal 5");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "monk Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.monk.base_save.will");
    assert_eq!(will.value, 5, "Monk level 6 good Will (6/2+2) must equal 5");
    assert!(
        will.detail.to_lowercase().contains("good"),
        "monk Will explanation must name it as a good save: {}",
        will.detail
    );
}

// ----- Unarmed strike damage die stays 1d8 at level 6 -----

#[test]
fn monk_level6_unarmed_strike_damage_die_stays_one_d8() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 8,
        "Monk level 6 unarmed strike damage die must stay 8 (i.e. 1d8), not a new record: {}",
        unarmed.detail
    );
    assert!(
        unarmed.detail.contains("1d8"),
        "monk unarmed-strike explanation must name the 1d8 damage die at level 6: {}",
        unarmed.detail
    );
}

// ----- Flurry of Blows flat attack bonus/count at level 6 -----

#[test]
fn monk_level6_flurry_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_bonus");
    assert_eq!(
        attack_bonus.value, 4,
        "Monk level 6 Flurry of Blows flat attack modifier (6 - 2) must equal 4, matching the \
         PF1 CRB table's +4/+4 at level 6: {}",
        attack_bonus.detail
    );
}

#[test]
fn monk_level6_flurry_attack_count_stays_two() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_count = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_count");
    assert_eq!(
        attack_count.value, 2,
        "Monk level 6 Flurry of Blows attack count must stay 2, not a new record: the third \
         flurry attack is not gained until 8th level per both primary sources' verbatim rule \
         text: {}",
        attack_count.detail
    );
}

// ----- Evasion, Still Mind, ki pool, and Purity of Body all stay granted at level 6 -----

#[test]
fn monk_level6_keeps_evasion_grounded() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, MONK_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value at level 6: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 6 must state it is granted, not absent: {}",
        evasion.detail
    );
}

#[test]
fn monk_level6_keeps_still_mind_grounded() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(
        still_mind.value, 2,
        "Still Mind must keep its flat +2 magnitude grounded at level 6: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("granted"),
        "still mind explanation at level 6 must state it is granted, not absent: {}",
        still_mind.detail
    );
}

#[test]
fn monk_level6_ki_pool_genuinely_rises_to_six() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 7,
        "Monk level 6 ki pool (1/2 monk level + Wisdom modifier = 6/2 + 4) must equal 7, up \
         from 6 at levels 4-5: {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("granted"),
        "ki pool explanation at level 6 must state it is granted, not absent: {}",
        ki_pool.detail
    );
}

#[test]
fn monk_level6_keeps_purity_of_body_grounded() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let purity = explanation(&computation, MONK_PURITY_OF_BODY_ID);
    assert_eq!(
        purity.value, 0,
        "Purity of Body must carry no fabricated mechanical value at level 6: {}",
        purity.detail
    );
    assert!(
        purity.detail.to_lowercase().contains("granted"),
        "purity of body explanation at level 6 must state it is granted, not absent: {}",
        purity.detail
    );
}

// ----- Slow Fall stays granted at level 6, and its own reach magnitude genuinely
// rises from 20 ft to 30 ft -----

#[test]
fn monk_level6_slow_fall_reach_genuinely_rises_to_thirty_feet() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall must carry no fabricated mechanical value at level 6: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.to_lowercase().contains("granted"),
        "slow fall explanation at level 6 must state it is granted, not absent: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.contains("30 feet") || slow_fall.detail.to_lowercase().contains("30 ft"),
        "slow fall explanation at level 6 must name the level-accurate 30-foot reach, not the \
         stale 20-foot figure: {}",
        slow_fall.detail
    );
    assert!(
        !slow_fall.detail.contains("20 feet") && !slow_fall.detail.to_lowercase().contains("20 ft"),
        "slow fall explanation at level 6 must not also claim the level-4/5 20-foot figure: {}",
        slow_fall.detail
    );
}

#[test]
fn monk_level5_slow_fall_reach_stays_twenty_feet_unaffected_by_this_slice() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall at level 5 must be unaffected by this widening, value stays 0: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.contains("20 feet") || slow_fall.detail.to_lowercase().contains("20 ft"),
        "slow fall explanation at level 5 must keep naming the 20-foot reach, unaffected by the \
         level-6 increase: {}",
        slow_fall.detail
    );
}

// ----- High Jump is still deliberately NOT grounded at level 6 -----

/// High Jump IS grounded from task #36 onward. The earlier guard here
/// asserted the OLD standalone-grounding bar, and was stale on two
/// counts: it demanded an "Acrobatics-check-total engine" that the
/// corrected bar (risks item 52) no longer requires for a standalone
/// fact, and it cited the ki cost as blocking the whole ability when the
/// corpus record's own DESC scopes the ki spend to a SEPARATE +20 clause.
/// The flat +MonkLVL is unconditional; only the +20 boost stays deferred.
#[test]
fn monk_level6_grounds_high_jump_as_a_flat_record() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fact = computation
        .explanations
        .iter()
        .find(|e| e.id == "class_chassis.monk.high_jump")
        .expect("High Jump must be grounded at monk level 6");
    assert_eq!(fact.value, 6, "High Jump is a flat +MonkLVL: {:?}", fact);
}

// ----- No new bonus-feat choice-slot is fabricated for the level-6 grant -----

#[test]
fn monk_level6_does_not_fabricate_a_second_bonus_feat_choice_slot() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        computation
            .explanations
            .iter()
            .filter(|e| e.id == "class_chassis.monk.bonus_feat_choice")
            .count(),
        1,
        "the level-6 repeat bonus feat grant must not fabricate a second bonus-feat-choice \
         explanation record: {:?}",
        computation.explanations
    );
}

// ----- The existing bonus-feat-mechanics diagnostic still fires at level 6 -----

#[test]
fn monk_level6_still_claim_blocks_the_recognized_bonus_feat_mechanics() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
                && d.claim_blocking),
        "level-6 Monk must still claim-block on the recognized bonus feat's own mechanics: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 7 stays unrecognized by this slice (level 7
// was later widened into the supported tranche by
// tests/sd13_monk_level7_progression.rs; the level-8 negative control now
// lives there) -----

#[test]
fn monk_level_7_was_later_widened_into_the_supported_tranche() {
    let level_7 = MONK_LEVEL6_FIXTURE.replace("class:monk:6", "class:monk:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")),
        "level-7 Monk was later widened into the supported tranche and must now gain bounded \
         monk chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, MONK_EVASION_ID),
        "level-7 Monk was later widened and must now carry the Evasion explanation"
    );
    assert!(
        has_explanation(&computation, MONK_STILL_MIND_ID),
        "level-7 Monk was later widened and must now carry the Still Mind explanation"
    );
    assert!(
        has_explanation(&computation, MONK_KI_POOL_ID),
        "level-7 Monk was later widened and must now carry the ki pool explanation"
    );
    assert!(
        has_explanation(&computation, MONK_SLOW_FALL_ID),
        "level-7 Monk was later widened and must now carry the Slow Fall explanation"
    );
    assert!(
        has_explanation(&computation, MONK_PURITY_OF_BODY_ID),
        "level-7 Monk was later widened and must now carry the Purity of Body explanation"
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level6_recognition() {
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
fn multiclass_monk_level6_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL6_FIXTURE.replace(
        "class_level=class:monk:6",
        "class_level=class:monk:6\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-6 widening and Slow Fall's increase -----

#[test]
fn matrix_monk_row_names_level_6_widening_and_slow_fall_increase() {
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
        monk.grounding_ref.contains("sd13_monk_level6_progression"),
        "monk row must cite the live SD13-E5 level-6 proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("slow fall"),
        "monk partial note must name Slow Fall's level-6 reach increase: {note}"
    );
    assert!(
        note.contains("bonus feat"),
        "monk partial note must keep naming the bonus feat's own mechanics as unproven: {note}"
    );
}
