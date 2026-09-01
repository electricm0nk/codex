//! SD13-E5 Monk level-7 progression grounding proof.
//!
//! Widens the accepted Monk level-1/level-2/level-3/level-4/level-5/level-6
//! martial chassis baseline (`tests/sd13_monk_level1_chassis_baseline.rs`,
//! `tests/sd13_monk_level2_progression.rs`,
//! `tests/sd13_monk_level3_progression.rs`,
//! `tests/sd13_monk_level4_progression.rs`,
//! `tests/sd13_monk_level5_progression.rs`,
//! `tests/sd13_monk_level6_progression.rs`) to monk level 7, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Cleric/Druid/Bard/Sorcerer/Ranger/Wizard
//! level-range-gate idiom (`supported_monk_level` is generalized from
//! `1..=6` to `1..=7` via `MAX_SUPPORTED_MONK_LEVEL = 7`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Monk class table) were
//! read directly before writing any code or test: level 7 base attack bonus
//! is +5, all three base saves are +5, unarmed damage stays 1d8 (the 1d8
//! band runs levels 4-7), the Flurry of Blows attack bonus improves to
//! +5/+5 (still two attacks — verified against both primary sources'
//! verbatim Flurry of Blows rule text, which states the monk gains a third
//! attack only "at 8th level"), and the level-7 special feature list names
//! Wholeness of Body (and, per both primary sources, a ki pool material
//! upgrade against damage reduction). It proves:
//!
//! - base attack bonus at level 7 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-6: `7 * 3 / 4 = 5`.
//! - base saves at level 7 are grounded by the same all-good formula
//!   (`level / 2 + 2`) already grounded at levels 1-6: `7 / 2 + 2 = 5` for
//!   Fortitude, Reflex, and Will alike.
//! - the unarmed strike damage die stays `8` (i.e. `1d8`) at level 7 —
//!   confirmed via the same formula/value, not a new record; the PF1 CRB
//!   Medium-monk damage table's 1d8 band runs levels 4-7.
//! - the Flurry of Blows flat attack bonus is grounded by the same
//!   `level - 2` formula already grounded at levels 1-6, correctly
//!   producing `5` at level 7 (`7 - 2 = 5`, matching the primary sources'
//!   "+5/+5"), and the attack count stays `2` at level 7 — verified
//!   independently against both primary sources' verbatim Flurry of Blows
//!   rule text ("At 8th level, the monk can make two additional attacks
//!   when he uses flurry of blows"), confirming the third attack is NOT
//!   gained at level 7 either, confirmed via the same formula, not a new
//!   record.
//! - Evasion, Still Mind, the ki pool's flat size, Slow Fall, and Purity of
//!   Body all stay granted at level 7 (not re-derived), grounded as the same
//!   bounded identity/flat-magnitude records already grounded at levels 2,
//!   3, 4, 4, and 5 respectively. Still Mind's own magnitude stays a flat
//!   `+2` (non-level-scaled). The ki pool's flat size stays `6` at level 7
//!   (`7 / 2 + 3 = 6`, an integer-division coincidence with level 6). Slow
//!   Fall's own reach magnitude stays `30` feet at level 7 (unchanged from
//!   level 6 — the next rise is at 8th level, beyond this bounded slice).
//!
//! This cycle was specifically briefed to check whether Monk gains an
//! actual new class feature at 7th level per the PF1 CRB class table's
//! "Special" column: verified independently against d20pfsrd and
//! legacy.aonprd.com, the level-7 row's "Special" column names Wholeness of
//! Body (and, per both primary sources, an upgrade to the ki pool's
//! damage-reduction-bypass material). Wholeness of Body is checked and
//! confirmed NOT flat: "at 7th level, a monk can heal his own wounds as a
//! standard action... a number of hit points of damage equal to his monk
//! level by using 2 points from his ki pool" — this requires both a
//! ki-point-consumption/action-economy engine and a healing-resolution
//! engine, neither of which exists anywhere in this codebase (mirroring
//! exactly why the ki pool's own point-spending was already left
//! unimplemented at level 4). The ki pool's material-bypass upgrade
//! likewise requires a damage-reduction-bypass-resolution engine that does
//! not exist here. Neither is grounded — no explanation or diagnostic
//! record is fabricated for either, mirroring the Paladin Divine Bond /
//! Rogue second-talent-slot / Barbarian Rage Power / Monk bonus-feat /
//! Monk High Jump / Bard Suggestion precedent exactly.
//!
//! It deliberately does not implement Fast Movement or Maneuver Training
//! (unchanged from level 3, still named-but-unproven), the recognized
//! bonus feat's own mechanics (unchanged from level 1), the level-2/level-6
//! bonus feat grants themselves, High Jump's own Acrobatics-check-total/
//! ki-point-spending mechanics, Wholeness of Body, the ki pool's
//! material-bypass upgrade, any ki-power execution engine, any
//! fall-damage-resolution engine, any disease-resolution engine, and it
//! does not ground Monk level 8+. It also preserves the accepted Monk
//! level-1/level-2/level-3/level-4/level-5/level-6 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const MONK_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level6_sd13_deterministic_input.txt");

const MONK_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level7_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";
const MONK_PURITY_OF_BODY_ID: &str = "class_chassis.monk.purity_of_body";

// ----- Base attack bonus at level 7 -----

#[test]
fn monk_level7_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Monk level 7 3/4-BAB progression (7 * 3 / 4) must equal 5: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "monk base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 (all three good) -----

#[test]
fn monk_level7_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.monk.base_save.fortitude");
    assert_eq!(fortitude.value, 5, "Monk level 7 good Fortitude (7/2+2) must equal 5");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "monk Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.monk.base_save.reflex");
    assert_eq!(reflex.value, 5, "Monk level 7 good Reflex (7/2+2) must equal 5");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "monk Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.monk.base_save.will");
    assert_eq!(will.value, 5, "Monk level 7 good Will (7/2+2) must equal 5");
    assert!(
        will.detail.to_lowercase().contains("good"),
        "monk Will explanation must name it as a good save: {}",
        will.detail
    );
}

// ----- Unarmed strike damage die stays 1d8 at level 7 -----

#[test]
fn monk_level7_unarmed_strike_damage_die_stays_one_d8() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 8,
        "Monk level 7 unarmed strike damage die must stay 8 (i.e. 1d8), not a new record: {}",
        unarmed.detail
    );
    assert!(
        unarmed.detail.contains("1d8"),
        "monk unarmed-strike explanation must name the 1d8 damage die at level 7: {}",
        unarmed.detail
    );
}

// ----- Flurry of Blows flat attack bonus/count at level 7 -----

#[test]
fn monk_level7_flurry_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_bonus");
    assert_eq!(
        attack_bonus.value, 5,
        "Monk level 7 Flurry of Blows flat attack modifier (7 - 2) must equal 5, matching the \
         PF1 CRB table's +5/+5 at level 7: {}",
        attack_bonus.detail
    );
}

#[test]
fn monk_level7_flurry_attack_count_stays_two() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_count = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_count");
    assert_eq!(
        attack_count.value, 2,
        "Monk level 7 Flurry of Blows attack count must stay 2, not a new record: the third \
         flurry attack is not gained until 8th level per both primary sources' verbatim rule \
         text: {}",
        attack_count.detail
    );
}

// ----- Evasion, Still Mind, ki pool, Slow Fall, and Purity of Body all stay granted at
// level 7 -----

#[test]
fn monk_level7_keeps_evasion_grounded() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, MONK_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value at level 7: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 7 must state it is granted, not absent: {}",
        evasion.detail
    );
}

#[test]
fn monk_level7_keeps_still_mind_grounded_unchanged() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(
        still_mind.value, 2,
        "Still Mind must keep its flat +2 magnitude grounded, unchanged at level 7: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("granted"),
        "still mind explanation at level 7 must state it is granted, not absent: {}",
        still_mind.detail
    );
}

#[test]
fn monk_level7_ki_pool_stays_six() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 7,
        "Monk level 7 ki pool (1/2 monk level + Wisdom modifier = 7/2 + 4) must equal 7, \
         unchanged from level 6 (an integer-division coincidence): {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("granted"),
        "ki pool explanation at level 7 must state it is granted, not absent: {}",
        ki_pool.detail
    );
}

#[test]
fn monk_level7_keeps_purity_of_body_grounded() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let purity = explanation(&computation, MONK_PURITY_OF_BODY_ID);
    assert_eq!(
        purity.value, 0,
        "Purity of Body must carry no fabricated mechanical value at level 7: {}",
        purity.detail
    );
    assert!(
        purity.detail.to_lowercase().contains("granted"),
        "purity of body explanation at level 7 must state it is granted, not absent: {}",
        purity.detail
    );
}

#[test]
fn monk_level7_slow_fall_reach_stays_thirty_feet_unchanged() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall must carry no fabricated mechanical value at level 7: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.contains("30 feet") || slow_fall.detail.to_lowercase().contains("30 ft"),
        "slow fall explanation at level 7 must keep naming the level-6-set 30-foot reach, \
         unchanged: {}",
        slow_fall.detail
    );
}

// ----- Wholeness of Body is deliberately NOT fabricated at level 7 -----

/// Wholeness of Body IS grounded from task #36 onward. The earlier guard
/// asserted the OLD bar, demanding a healing engine that the corrected
/// bar (risks item 52) no longer requires for a standalone magnitude.
#[test]
fn monk_level7_grounds_wholeness_of_body_as_a_flat_record() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fact = computation
        .explanations
        .iter()
        .find(|e| e.id == "class_chassis.monk.wholeness_of_body")
        .expect("Wholeness of Body must be grounded at monk level 7");
    assert_eq!(fact.value, 7, "heals MonkLVL hp: {:?}", fact);
}

// ----- High Jump is still deliberately NOT grounded at level 7 -----

/// High Jump IS grounded from task #36 onward. The earlier guard here
/// asserted the OLD standalone-grounding bar, and was stale on two
/// counts: it demanded an "Acrobatics-check-total engine" that the
/// corrected bar (risks item 52) no longer requires for a standalone
/// fact, and it cited the ki cost as blocking the whole ability when the
/// corpus record's own DESC scopes the ki spend to a SEPARATE +20 clause.
/// The flat +MonkLVL is unconditional; only the +20 boost stays deferred.
#[test]
fn monk_level7_grounds_high_jump_as_a_flat_record() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fact = computation
        .explanations
        .iter()
        .find(|e| e.id == "class_chassis.monk.high_jump")
        .expect("High Jump must be grounded at monk level 7");
    assert_eq!(fact.value, 7, "High Jump is a flat +MonkLVL: {:?}", fact);
}

// ----- No new bonus-feat choice-slot is fabricated for the level-7 grant -----

#[test]
fn monk_level7_does_not_fabricate_a_second_bonus_feat_choice_slot() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        computation
            .explanations
            .iter()
            .filter(|e| e.id == "class_chassis.monk.bonus_feat_choice")
            .count(),
        1,
        "level 7 must not fabricate a second bonus-feat-choice explanation record: {:?}",
        computation.explanations
    );
}

// ----- The existing bonus-feat-mechanics diagnostic still fires at level 7 -----

#[test]
fn monk_level7_still_claim_blocks_the_recognized_bonus_feat_mechanics() {
    let input = load(MONK_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
                && d.claim_blocking),
        "level-7 Monk must still claim-block on the recognized bonus feat's own mechanics: {:?}",
        computation.diagnostics
    );
}

// ----- Level 8 was later widened into the supported tranche -----

#[test]
fn monk_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = MONK_LEVEL7_FIXTURE.replace("class:monk:7", "class:monk:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")),
        "level-8 Monk was later widened into the supported tranche and must now gain bounded \
         monk chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, MONK_EVASION_ID),
        "level-8 Monk was later widened and must now carry the Evasion explanation"
    );
    assert!(
        has_explanation(&computation, MONK_STILL_MIND_ID),
        "level-8 Monk was later widened and must now carry the Still Mind explanation"
    );
    assert!(
        has_explanation(&computation, MONK_KI_POOL_ID),
        "level-8 Monk was later widened and must now carry the ki pool explanation"
    );
    assert!(
        has_explanation(&computation, MONK_SLOW_FALL_ID),
        "level-8 Monk was later widened and must now carry the Slow Fall explanation"
    );
    assert!(
        has_explanation(&computation, MONK_PURITY_OF_BODY_ID),
        "level-8 Monk was later widened and must now carry the Purity of Body explanation"
    );
}

// ----- Negative control: the level-6 fixture is unaffected by this widening -----

#[test]
fn monk_level6_truth_is_unchanged_by_this_slice() {
    let input = load(MONK_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_bonus");
    assert_eq!(
        attack_bonus.value, 4,
        "Monk level 6 Flurry of Blows flat attack modifier must stay 4, unaffected by the \
         level-7 widening: {}",
        attack_bonus.detail
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level7_recognition() {
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
fn multiclass_monk_level7_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL7_FIXTURE.replace(
        "class_level=class:monk:7",
        "class_level=class:monk:7\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_monk_row_names_level_7_widening() {
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
        monk.grounding_ref.contains("sd13_monk_level7_progression"),
        "monk row must cite the live SD13-E5 level-7 proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.contains("Wholeness of Body") || note.contains("wholeness of body"),
        "monk partial note must name Wholeness of Body as checked-but-unproven: {note}"
    );
    assert!(
        note.contains("bonus feat"),
        "monk partial note must keep naming the bonus feat's own mechanics as unproven: {note}"
    );
}
