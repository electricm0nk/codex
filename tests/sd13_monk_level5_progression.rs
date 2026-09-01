//! SD13-E5 Monk level-5 progression grounding proof.
//!
//! Widens the accepted Monk level-1/level-2/level-3/level-4 martial chassis
//! baseline (`tests/sd13_monk_level1_chassis_baseline.rs`,
//! `tests/sd13_monk_level2_progression.rs`,
//! `tests/sd13_monk_level3_progression.rs`,
//! `tests/sd13_monk_level4_progression.rs`) to monk level 5, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Cleric/Druid level-range-gate idiom
//! (`supported_monk_level` is generalized from `1..=4` to `1..=5` via
//! `MAX_SUPPORTED_MONK_LEVEL = 5`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Monk class table) were read directly before writing
//! any code or test: level 5 base attack bonus is +3, all three base saves
//! are +4, unarmed damage stays 1d8 (the 1d8 band runs levels 4-7), the
//! Flurry of Blows attack bonus improves to +3/+3 (two attacks, unchanged
//! count), and the level-5 special feature list is "High jump, purity of
//! body." It proves:
//!
//! - base attack bonus at level 5 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-4: `5 * 3 / 4 = 3`
//!   (integer division), matching the primary sources' level-5 row.
//! - base saves at level 5 are grounded by the same all-good formula
//!   (`level / 2 + 2`) already grounded at levels 1-4: `5 / 2 + 2 = 4` for
//!   Fortitude, Reflex, and Will alike.
//! - the unarmed strike damage die stays `8` (i.e. `1d8`) at level 5 —
//!   confirmed via the same formula/value, not a new record; the PF1 CRB
//!   Medium-monk damage table's 1d8 band runs levels 4-7.
//! - the Flurry of Blows flat attack bonus is grounded by the same
//!   `level - 2` formula already grounded at levels 1-4, correctly
//!   producing `3` at level 5 (`5 - 2 = 3`, matching the primary sources'
//!   "+3/+3"), and the attack count stays `2` at level 5 (Monk gains a third
//!   flurry attack only at a much higher level), confirmed via the same
//!   formula, not a new record.
//! - Evasion, Still Mind, the ki pool's flat size, and Slow Fall all stay
//!   granted at level 5 (not re-derived), grounded as the same bounded
//!   identity/flat-magnitude records already grounded at levels 2, 3, and 4.
//!   The ki pool's flat size stays numerically `5` at level 5 too
//!   (`5 / 2 + 3 = 5` via integer division), which is the same value as
//!   level 4 (`4 / 2 + 3 = 5`) — a coincidence of integer division, not a
//!   sign the formula stopped scaling with level.
//! - Purity of Body, the PF1 CRB's other 5th-level Monk class feature
//!   (verified independently against d20pfsrd and legacy.aonprd.com: "at
//!   5th level, a monk gains immunity to all diseases, including
//!   supernatural and magical diseases"), is newly grounded as a bounded
//!   grant-only identity record (`class_chassis.monk.purity_of_body`),
//!   mirroring the Barbarian/Rogue Uncanny Dodge / Monk Slow Fall
//!   grant-only idiom: no disease-resolution engine exists anywhere in this
//!   codebase to apply the immunity to.
//!
//! It deliberately does NOT ground High Jump, the level-5 class table's
//! OTHER "Special" column entry (verified independently against both
//! primary sources and confirmed NOT flat: "a monk adds his monk level to
//! Acrobatics checks made to jump... and my spend 1 ki point to gain a +20
//! bonus on a jump check" — this requires wiring the monk's level into an
//! Acrobatics-check total, an integrated-total engine that does not exist
//! in this codebase, AND a ki-point-spending action, an action-
//! economy/resource-consumption engine this codebase deliberately does not
//! implement for the ki pool either) — so High Jump stays named but
//! unproven, exactly like the Wild Shape / Rage Power / animal-companion
//! precedent of checking a "Special" column entry and confirming it is not
//! flat before declining to fabricate a record for it.
//!
//! It also deliberately does not implement Fast Movement or Maneuver
//! Training (unchanged from level 3, still named-but-unproven), the
//! recognized bonus feat's own mechanics (unchanged from level 1), the
//! level-2 bonus feat grant (PF1 grants monks a SEPARATE bonus feat at 2nd
//! level; not recognized by this or any prior slice), any ki-power
//! execution engine, any fall-damage-resolution engine, any disease-
//! resolution engine, any attack-resolution or damage-resolution engine,
//! and it does not ground Monk level 6+. It also preserves the accepted
//! Monk level-1/level-2/level-3/level-4 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const MONK_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level4_sd13_deterministic_input.txt");

const MONK_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level5_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";
const MONK_PURITY_OF_BODY_ID: &str = "class_chassis.monk.purity_of_body";

// ----- Base attack bonus at level 5 -----

#[test]
fn monk_level5_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Monk level 5 3/4-BAB progression (5 * 3 / 4) must equal 3: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "monk base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 5 (all three good) -----

#[test]
fn monk_level5_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.monk.base_save.fortitude");
    assert_eq!(fortitude.value, 4, "Monk level 5 good Fortitude (5/2+2) must equal 4");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "monk Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.monk.base_save.reflex");
    assert_eq!(reflex.value, 4, "Monk level 5 good Reflex (5/2+2) must equal 4");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "monk Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.monk.base_save.will");
    assert_eq!(will.value, 4, "Monk level 5 good Will (5/2+2) must equal 4");
    assert!(
        will.detail.to_lowercase().contains("good"),
        "monk Will explanation must name it as a good save: {}",
        will.detail
    );
}

// ----- Unarmed strike damage die stays 1d8 at level 5 -----

#[test]
fn monk_level5_unarmed_strike_damage_die_stays_one_d8() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 8,
        "Monk level 5 unarmed strike damage die must stay 8 (i.e. 1d8), not a new record: {}",
        unarmed.detail
    );
    assert!(
        unarmed.detail.contains("1d8"),
        "monk unarmed-strike explanation must name the 1d8 damage die at level 5: {}",
        unarmed.detail
    );
}

// ----- Flurry of Blows flat attack bonus/count at level 5 -----

#[test]
fn monk_level5_flurry_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_bonus");
    assert_eq!(
        attack_bonus.value, 3,
        "Monk level 5 Flurry of Blows flat attack modifier (5 - 2) must equal 3, matching the \
         PF1 CRB table's +3/+3 at level 5: {}",
        attack_bonus.detail
    );
}

#[test]
fn monk_level5_flurry_attack_count_stays_two() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_count = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_count");
    assert_eq!(
        attack_count.value, 2,
        "Monk level 5 Flurry of Blows attack count must stay 2, not a new record: {}",
        attack_count.detail
    );
}

// ----- Evasion, Still Mind, ki pool, and Slow Fall all stay granted at level 5 -----

#[test]
fn monk_level5_keeps_evasion_grounded() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, MONK_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value at level 5: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 5 must state it is granted, not absent: {}",
        evasion.detail
    );
}

#[test]
fn monk_level5_keeps_still_mind_grounded() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(
        still_mind.value, 2,
        "Still Mind must keep its flat +2 magnitude grounded at level 5: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("granted"),
        "still mind explanation at level 5 must state it is granted, not absent: {}",
        still_mind.detail
    );
}

#[test]
fn monk_level5_keeps_ki_pool_grounded() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 6,
        "Monk level 5 ki pool (1/2 monk level + Wisdom modifier = 5/2 + 4, integer division) \
         must equal 6: {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("granted"),
        "ki pool explanation at level 5 must state it is granted, not absent: {}",
        ki_pool.detail
    );
}

#[test]
fn monk_level5_keeps_slow_fall_grounded() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall must carry no fabricated mechanical value at level 5: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.to_lowercase().contains("granted"),
        "slow fall explanation at level 5 must state it is granted, not absent: {}",
        slow_fall.detail
    );
}

// ----- Purity of Body is newly granted at level 5 as an identity record -----

#[test]
fn monk_level5_grounds_purity_of_body_as_identity_record_only() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let purity = explanation(&computation, MONK_PURITY_OF_BODY_ID);
    assert_eq!(
        purity.value, 0,
        "Purity of Body must carry no fabricated mechanical value: {}",
        purity.detail
    );
    assert!(
        purity.detail.to_lowercase().contains("disease"),
        "purity of body explanation must name the disease immunity: {}",
        purity.detail
    );
    assert!(
        purity.detail.to_lowercase().contains("granted"),
        "purity of body explanation at level 5 must state it is granted, not absent: {}",
        purity.detail
    );
    assert!(
        purity.detail.to_lowercase().contains("no disease")
            || purity.detail.to_lowercase().contains("no disease-resolution"),
        "purity of body explanation must disclaim any disease-resolution engine: {}",
        purity.detail
    );
}

#[test]
fn monk_level4_purity_of_body_is_a_correct_level_gate_absence() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let purity = explanation(&computation, MONK_PURITY_OF_BODY_ID);
    assert_eq!(
        purity.value, 0,
        "Purity of Body at level 4 must be a correct level-gate absence, value 0: {}",
        purity.detail
    );
    assert!(
        purity.detail.to_lowercase().contains("absent"),
        "purity of body explanation at level 4 must state it is correctly absent: {}",
        purity.detail
    );
}

// ----- High Jump is deliberately NOT grounded (checked, confirmed not flat) -----

/// High Jump IS grounded from task #36 onward. The earlier guard here
/// asserted the OLD standalone-grounding bar, and was stale on two
/// counts: it demanded an "Acrobatics-check-total engine" that the
/// corrected bar (risks item 52) no longer requires for a standalone
/// fact, and it cited the ki cost as blocking the whole ability when the
/// corpus record's own DESC scopes the ki spend to a SEPARATE +20 clause.
/// The flat +MonkLVL is unconditional; only the +20 boost stays deferred.
#[test]
fn monk_level5_grounds_high_jump_as_a_flat_record() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fact = computation
        .explanations
        .iter()
        .find(|e| e.id == "class_chassis.monk.high_jump")
        .expect("High Jump must be grounded at monk level 5");
    assert_eq!(fact.value, 5, "High Jump is a flat +MonkLVL: {:?}", fact);
}

// ----- The existing bonus-feat-mechanics diagnostic still fires at level 5 -----

#[test]
fn monk_level5_still_claim_blocks_the_recognized_bonus_feat_mechanics() {
    let input = load(MONK_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
                && d.claim_blocking),
        "level-5 Monk must still claim-block on the recognized bonus feat's own mechanics: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 6 stays unrecognized by this slice (level 6
// was later widened into the supported tranche by
// tests/sd13_monk_level6_progression.rs; the level-7 negative control now
// lives there) -----

#[test]
fn monk_level_6_was_later_widened_into_the_supported_tranche() {
    let level_6 = MONK_LEVEL5_FIXTURE.replace("class:monk:5", "class:monk:6");
    let input = load(&level_6);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.monk.")),
        "level-6 Monk was later widened into the supported tranche and must now gain bounded \
         monk chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, MONK_EVASION_ID),
        "level-6 Monk was later widened and must now carry the Evasion explanation"
    );
    assert!(
        has_explanation(&computation, MONK_STILL_MIND_ID),
        "level-6 Monk was later widened and must now carry the Still Mind explanation"
    );
    assert!(
        has_explanation(&computation, MONK_KI_POOL_ID),
        "level-6 Monk was later widened and must now carry the ki pool explanation"
    );
    assert!(
        has_explanation(&computation, MONK_SLOW_FALL_ID),
        "level-6 Monk was later widened and must now carry the Slow Fall explanation"
    );
    assert!(
        has_explanation(&computation, MONK_PURITY_OF_BODY_ID),
        "level-6 Monk was later widened and must now carry the Purity of Body explanation"
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level5_recognition() {
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
fn multiclass_monk_level5_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL5_FIXTURE.replace(
        "class_level=class:monk:5",
        "class_level=class:monk:5\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-5 widening and Purity of Body -----

#[test]
fn matrix_monk_row_names_level_5_widening_and_purity_of_body() {
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
        monk.grounding_ref.contains("sd13_monk_level5_progression"),
        "monk row must cite the live SD13-E5 level-5 proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("purity of body"),
        "monk partial note must name Purity of Body as newly grounded: {note}"
    );
    assert!(
        note.to_lowercase().contains("high jump"),
        "monk partial note must name High Jump as checked and confirmed not flat: {note}"
    );
    assert!(
        note.contains("bonus feat"),
        "monk partial note must keep naming the bonus feat's own mechanics as unproven: {note}"
    );
}
