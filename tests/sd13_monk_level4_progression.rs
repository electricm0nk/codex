//! SD13-E5 Monk level-4 progression grounding proof.
//!
//! Widens the accepted Monk level-1/level-2/level-3 martial chassis baseline
//! (`tests/sd13_monk_level1_chassis_baseline.rs`,
//! `tests/sd13_monk_level2_progression.rs`,
//! `tests/sd13_monk_level3_progression.rs`) to monk level 4, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Cleric/Druid level-range-gate idiom
//! (`supported_monk_level` is generalized from `1..=3` to `1..=4` via
//! `MAX_SUPPORTED_MONK_LEVEL = 4`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Monk class table) were read directly before writing
//! any code or test: level 4 base attack bonus is +3, all three base saves
//! are +4, the unarmed strike damage die steps up to 1d8 (the class table's
//! 1d6 band ends at level 3; the 1d8 band runs levels 4-7), the Flurry of
//! Blows attack bonus improves to +2/+2 (two attacks, unchanged count), and
//! the level-4 special feature list is "Ki pool (magic), slow fall 20 ft."
//! It proves:
//!
//! - base attack bonus at level 4 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-3: `4 * 3 / 4 = 3`.
//! - base saves at level 4 are grounded by the same all-good formula
//!   (`level / 2 + 2`) already grounded at levels 1-3: `4 / 2 + 2 = 4` for
//!   Fortitude, Reflex, and Will alike.
//! - the unarmed strike damage die steps from `6` (1d6) to `8` (1d8) at
//!   level 4 — the first die-size change this seam grounds, verified
//!   independently against both primary sources' Medium-monk damage table
//!   (1d6 at 1-3, 1d8 at 4-7, 1d10 at 8-11, 2d6 at 12-15, 2d8 at 16-19, 2d10
//!   at 20).
//! - the Flurry of Blows flat attack bonus is grounded by the same
//!   `level - 2` formula already grounded at levels 1-3, correctly
//!   producing `2` at level 4 (`4 - 2 = 2`, matching the primary source's
//!   "+2/+2"), and the attack count stays `2` at level 4 (Monk gains a third
//!   flurry attack only at a much higher level), confirmed via the same
//!   formula, not a new record.
//! - Evasion and Still Mind both stay granted at level 4 (not re-derived),
//!   grounded as the same bounded identity/flat-magnitude records already
//!   grounded at levels 2 and 3.
//! - the ki pool's flat size is newly grounded as a standalone bounded
//!   flat-magnitude record (`class_chassis.monk.ki_pool_size`), a 4th-level
//!   Monk class feature verified independently against d20pfsrd and
//!   legacy.aonprd.com ("the number of points in a monk's ki pool is equal
//!   to 1/2 his monk level + his Wisdom modifier" — neither primary source
//!   states a minimum floor on the pool itself): `4 / 2 + 3 = 5` on the
//!   fixture's Wisdom 17 (+3 modifier). This grounds only the flat pool-size
//!   number; it computes no ki-point consumption tracking, no action-economy
//!   engine, and no application of any ki power (the extra-attack, +4 AC
//!   dodge-bonus, or +20-ft.-speed uses) — mirroring the Barbarian rage
//!   rounds-per-day / Paladin lay-on-hands-uses-per-day idiom.
//! - Slow Fall, the PF1 CRB's other 4th-level Monk class feature, is newly
//!   grounded as a bounded grant-only identity record
//!   (`class_chassis.monk.slow_fall`), verified independently against both
//!   primary sources ("a monk within arm's reach of a wall can use it to
//!   slow his descent... treats the fall as if it were 20 feet shorter"),
//!   mirroring the Barbarian Uncanny Dodge / Rogue Uncanny Dodge / Druid
//!   Woodland Stride grant-only idiom: no fall-damage-resolution engine
//!   exists anywhere in this codebase to apply the 20-foot reduction to.
//!
//! It deliberately does not implement Fast Movement or Maneuver Training
//! (unchanged from level 3, still named-but-unproven), the recognized bonus
//! feat's own mechanics (unchanged from level 1), the level-2 bonus feat
//! grant (PF1 grants monks a SEPARATE bonus feat at 2nd level; not
//! recognized by this or any prior slice), any ki-power execution engine, any
//! fall-damage-resolution engine, any attack-resolution or damage-resolution
//! engine, and it does not ground Monk level 5+. It also preserves the
//! accepted Monk level-1/level-2/level-3 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const MONK_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level3_sd13_deterministic_input.txt");

const MONK_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level4_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const MONK_EVASION_ID: &str = "class_feature.monk.evasion";
const MONK_STILL_MIND_ID: &str = "class_feature.monk.still_mind";
const MONK_KI_POOL_ID: &str = "class_chassis.monk.ki_pool_size";
const MONK_SLOW_FALL_ID: &str = "class_chassis.monk.slow_fall";

// ----- Base attack bonus at level 4 -----

#[test]
fn monk_level4_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.monk.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Monk level 4 3/4-BAB progression (4 * 3 / 4) must equal 3: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "monk base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 4 (all three good) -----

#[test]
fn monk_level4_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.monk.base_save.fortitude");
    assert_eq!(fortitude.value, 4, "Monk level 4 good Fortitude (4/2+2) must equal 4");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "monk Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.monk.base_save.reflex");
    assert_eq!(reflex.value, 4, "Monk level 4 good Reflex (4/2+2) must equal 4");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "monk Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.monk.base_save.will");
    assert_eq!(will.value, 4, "Monk level 4 good Will (4/2+2) must equal 4");
    assert!(
        will.detail.to_lowercase().contains("good"),
        "monk Will explanation must name it as a good save: {}",
        will.detail
    );
}

// ----- Unarmed strike damage die steps up to 1d8 at level 4 -----

#[test]
fn monk_level4_unarmed_strike_damage_die_steps_up_to_one_d8() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 8,
        "Monk level 4 unarmed strike damage die must step up to 8 (i.e. 1d8): {}",
        unarmed.detail
    );
    assert!(
        unarmed.detail.contains("1d8"),
        "monk unarmed-strike explanation must name the 1d8 damage die at level 4: {}",
        unarmed.detail
    );
}

#[test]
fn monk_level3_unarmed_strike_damage_die_still_stays_one_d6() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let unarmed = explanation(&computation, "class_chassis.monk.unarmed_strike_damage_die");
    assert_eq!(
        unarmed.value, 6,
        "Monk level 3 unarmed strike damage die must stay 6 (i.e. 1d6), unchanged by this \
         widening: {}",
        unarmed.detail
    );
}

// ----- Flurry of Blows flat attack bonus/count at level 4 -----

#[test]
fn monk_level4_flurry_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_bonus");
    assert_eq!(
        attack_bonus.value, 2,
        "Monk level 4 Flurry of Blows flat attack modifier (4 - 2) must equal 2, matching the \
         PF1 CRB table's +2/+2 at level 4: {}",
        attack_bonus.detail
    );
}

#[test]
fn monk_level4_flurry_attack_count_stays_two() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_count = explanation(&computation, "class_chassis.monk.flurry_of_blows_attack_count");
    assert_eq!(
        attack_count.value, 2,
        "Monk level 4 Flurry of Blows attack count must stay 2, not a new record: {}",
        attack_count.detail
    );
}

// ----- Evasion and Still Mind both stay granted at level 4, not re-derived -----

#[test]
fn monk_level4_keeps_evasion_grounded() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, MONK_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value at level 4: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 4 must state it is granted, not absent: {}",
        evasion.detail
    );
}

#[test]
fn monk_level4_keeps_still_mind_grounded() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let still_mind = explanation(&computation, MONK_STILL_MIND_ID);
    assert_eq!(
        still_mind.value, 2,
        "Still Mind must keep its flat +2 magnitude grounded at level 4: {}",
        still_mind.detail
    );
    assert!(
        still_mind.detail.to_lowercase().contains("granted"),
        "still mind explanation at level 4 must state it is granted, not absent: {}",
        still_mind.detail
    );
}

// ----- Ki pool flat size is newly grounded at level 4 -----

#[test]
fn monk_level4_grounds_ki_pool_flat_size() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Wisdom modifier is now +4 (base 17 + 2 Human racial), not +3.
    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 6,
        "Monk level 4 ki pool (1/2 monk level + Wisdom modifier = 4/2 + 4) must equal 6: {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("wisdom"),
        "ki pool explanation must name the Wisdom-modifier component of the formula: {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("no ki-point consumption")
            || ki_pool.detail.to_lowercase().contains("no consumption tracking"),
        "ki pool explanation must disclaim ki-point consumption tracking: {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("action-economy"),
        "ki pool explanation must disclaim an action-economy engine: {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("extra attack")
            && ki_pool.detail.to_lowercase().contains("ac")
            && ki_pool.detail.to_lowercase().contains("speed"),
        "ki pool explanation must disclaim application of any ki power (extra attack, AC bonus, \
         speed bonus): {}",
        ki_pool.detail
    );
}

#[test]
fn monk_level3_ki_pool_is_a_correct_level_gate_absence() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ki_pool = explanation(&computation, MONK_KI_POOL_ID);
    assert_eq!(
        ki_pool.value, 0,
        "Ki pool at level 3 must be a correct level-gate absence, value 0: {}",
        ki_pool.detail
    );
    assert!(
        ki_pool.detail.to_lowercase().contains("absent"),
        "ki pool explanation at level 3 must state it is correctly absent: {}",
        ki_pool.detail
    );
}

// ----- Slow Fall is newly granted at level 4 as an identity record -----

#[test]
fn monk_level4_grounds_slow_fall_as_identity_record_only() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall must carry no fabricated mechanical value: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.to_lowercase().contains("20 feet")
            || slow_fall.detail.to_lowercase().contains("20 ft"),
        "slow fall explanation must name the 20-foot reduction: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.to_lowercase().contains("granted"),
        "slow fall explanation at level 4 must state it is granted, not absent: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.to_lowercase().contains("no fall-damage")
            || slow_fall.detail.to_lowercase().contains("fall-damage-resolution"),
        "slow fall explanation must disclaim any fall-damage-resolution engine: {}",
        slow_fall.detail
    );
}

#[test]
fn monk_level3_slow_fall_is_a_correct_level_gate_absence() {
    let input = load(MONK_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slow_fall = explanation(&computation, MONK_SLOW_FALL_ID);
    assert_eq!(
        slow_fall.value, 0,
        "Slow Fall at level 3 must be a correct level-gate absence, value 0: {}",
        slow_fall.detail
    );
    assert!(
        slow_fall.detail.to_lowercase().contains("absent"),
        "slow fall explanation at level 3 must state it is correctly absent: {}",
        slow_fall.detail
    );
}

// ----- The existing bonus-feat-mechanics diagnostic still fires at level 4 -----

#[test]
fn monk_level4_still_claim_blocks_the_recognized_bonus_feat_mechanics() {
    let input = load(MONK_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
                && d.claim_blocking),
        "level-4 Monk must still claim-block on the recognized bonus feat's own mechanics: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 5 was later widened into the supported tranche -----

#[test]
fn monk_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_monk_level5_progression.rs) widened the level-range gate to
    // level 5 (mirroring the Fighter/Paladin/Rogue/Barbarian/Cleric/Druid
    // level-range gate idiom) and grounded Purity of Body; this negative
    // control is superseded, not violated — pin the new truth here too so
    // this file stays internally consistent. The frontier this file's own
    // slice actually drew is now level 6, covered by
    // `tests/sd13_monk_level5_progression.rs::monk_level_6_is_not_promoted_by_this_slice`.
    let level_5 = MONK_LEVEL4_FIXTURE.replace("class:monk:4", "class:monk:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.monk.base_attack_bonus"),
        "level-5 Monk is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, MONK_EVASION_ID),
        "level-5 Monk must keep the Evasion explanation grounded at level 2"
    );
    assert!(
        has_explanation(&computation, MONK_STILL_MIND_ID),
        "level-5 Monk must keep the Still Mind explanation grounded at level 3"
    );
    assert!(
        has_explanation(&computation, MONK_KI_POOL_ID),
        "level-5 Monk must keep the ki pool explanation grounded at level 4"
    );
    assert!(
        has_explanation(&computation, MONK_SLOW_FALL_ID),
        "level-5 Monk must keep the Slow Fall explanation grounded at level 4"
    );
}

// ----- Negative control: the monk path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_monk_level4_recognition() {
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
fn multiclass_monk_level4_is_not_promoted_by_this_slice() {
    let multiclass = MONK_LEVEL4_FIXTURE.replace(
        "class_level=class:monk:4",
        "class_level=class:monk:4\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-4 widening, ki pool, and slow fall -----

#[test]
fn matrix_monk_row_names_level_4_widening_ki_pool_and_slow_fall() {
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
        monk.grounding_ref.contains("sd13_monk_level4_progression"),
        "monk row must cite the live SD13-E5 level-4 proof surface: {}",
        monk.grounding_ref
    );
    let note = monk.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("ki pool"),
        "monk partial note must name the ki pool as newly grounded: {note}"
    );
    assert!(
        note.to_lowercase().contains("slow fall"),
        "monk partial note must name Slow Fall as newly grounded: {note}"
    );
    assert!(
        note.contains("bonus feat"),
        "monk partial note must keep naming the bonus feat's own mechanics as unproven: {note}"
    );
}
