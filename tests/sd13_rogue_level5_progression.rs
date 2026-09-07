//! SD13-E5 Rogue level-5 progression grounding proof.
//!
//! Widens the accepted Rogue level-1/level-2/level-3/level-4 chassis baseline
//! (`tests/sd13_rogue_level1_chassis_baseline.rs`,
//! `tests/sd13_rogue_level2_progression.rs`,
//! `tests/sd13_rogue_level3_progression.rs`,
//! `tests/sd13_rogue_level4_progression.rs`) to rogue level 5, mirroring the
//! Fighter/Paladin/Barbarian/Monk level-range-gate idiom (`supported_rogue_level`
//! is generalized from `1..=4` to `1..=5` via `MAX_SUPPORTED_ROGUE_LEVEL = 5`).
//! It proves:
//!
//! - base attack bonus at level 5 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-4: `5 * 3 / 4 = 3`.
//! - base saves at level 5 are grounded by the same good-Reflex/poor-Fortitude/
//!   poor-Will formulas already grounded at levels 1-4, extended to level 5:
//!   Fortitude `5 / 3 = 1`, Reflex `5 / 2 + 2 = 4`, Will `5 / 3 = 1`.
//! - sneak attack die count genuinely increases to `3` (i.e. `3d6`) at level 5
//!   — the PF1 Core Rulebook Rogue class table increases the sneak attack die
//!   every two rogue levels (1d6 at levels 1-2, 2d6 at levels 3-4, 3d6 at
//!   level 5+); this is confirmed via the same pre-existing formula, not a
//!   new record: `(5 + 1) / 2 = 3`.
//! - Trapfinding at level 5 stays `max(5 / 2, 1) = 2`, the same value as
//!   level 4, confirmed via the same formula (verified against the PF1 CRB
//!   this stays flat at level 5).
//! - Evasion stays granted at level 5 (not re-derived), grounded as the same
//!   bounded identity/recognition record already grounded at level 2.
//! - Trap Sense stays granted at level 5 (not re-derived), and its magnitude
//!   stays `1` (rogue level / 3 = 1; it next rises at 6th level, beyond this
//!   bounded slice), grounded as the same bounded flat-magnitude record
//!   already grounded at level 3.
//! - Uncanny Dodge stays granted at level 5 (not re-derived), grounded as the
//!   same bounded identity/recognition record already grounded at level 4.
//!
//! This cycle was specifically briefed to check whether Rogue gains an
//! actual new class feature at 5th level per the PF1 CRB class table's
//! "Special" column: verified independently against d20pfsrd and
//! legacy.aonprd.com, the level-5 row's "Special" column reads only "Sneak
//! attack +3d6" — the same die-count increase the pre-existing formula
//! already grounds, not a distinct new pillar. So this slice grounds no new
//! pillar beyond the arithmetic extension above.
//!
//! It deliberately does not implement rogue talents (a level-2+ choice-list
//! feature and a genuinely open-ended talent tree — a new-subsystem-shaped
//! burden left named but unproven), any check-execution engine, trap DC
//! resolution, magic-trap disarm engine, or sneak-attack trigger-condition
//! engine, and it does not ground Rogue level 6+. It also preserves the
//! accepted Rogue level-1/level-2/level-3/level-4 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const ROGUE_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level4_sd13_deterministic_input.txt");

const ROGUE_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level5_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";

// ----- Base attack bonus at level 5 -----

#[test]
fn rogue_level5_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(ROGUE_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Rogue level 5 3/4-BAB progression (5 * 3 / 4) must equal 3: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "rogue base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 5 -----

#[test]
fn rogue_level5_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(ROGUE_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 1, "Rogue level 5 poor Fortitude (5/3) must equal 1");
    assert!(
        fortitude.detail.to_lowercase().contains("poor"),
        "rogue Fortitude explanation must name it as a poor save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 4, "Rogue level 5 good Reflex (5/2+2) must equal 4");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "rogue Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 1, "Rogue level 5 poor Will (5/3) must equal 1");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "rogue Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Sneak attack die count genuinely increases to 3d6 at level 5 -----

#[test]
fn rogue_level5_sneak_attack_die_count_increases_to_three_d6() {
    let input = load(ROGUE_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 3,
        "Rogue level 5 sneak attack die count must increase to 3 (i.e. 3d6): {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.contains("3d6"),
        "rogue sneak-attack explanation must name the 3d6 damage die at level 5: {}",
        sneak_attack.detail
    );
}

// ----- Trapfinding stays the same value at level 5 -----

#[test]
fn rogue_level5_trapfinding_is_grounded_via_the_same_formula() {
    let input = load(ROGUE_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 2,
        "rogue Trapfinding bonus at level 5 must be max(5 / 2, 1) = 2, got {}",
        trapfinding.value
    );
    assert!(
        trapfinding.detail.contains("Trapfinding"),
        "rogue Trapfinding explanation must name the Trapfinding class feature: {}",
        trapfinding.detail
    );
}

// ----- Evasion stays granted at level 5, not re-derived -----

#[test]
fn rogue_level5_evasion_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, ROGUE_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 5 must state it is granted, not absent: {}",
        evasion.detail
    );
}

// ----- Trap Sense stays granted at level 5, not re-derived -----

#[test]
fn rogue_level5_trap_sense_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 1,
        "Trap Sense magnitude at level 5 must stay rogue level / 3 = 1: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 5 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

// ----- Uncanny Dodge stays granted at level 5, not re-derived -----

#[test]
fn rogue_level5_uncanny_dodge_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, ROGUE_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge must carry no fabricated mechanical value: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("flat-footed"),
        "uncanny dodge explanation must state the actual rule text (cannot be caught \
         flat-footed): {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("granted"),
        "uncanny dodge explanation at level 5 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

// ----- No named Rogue pillar diagnostic remains at level 5 -----

#[test]
fn rogue_level5_leaves_no_named_pillar_diagnostic() {
    let input = load(ROGUE_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.id.starts_with("class_feature.rogue.")
            && d.id != ROGUE_EVASION_ID
            && d.id != ROGUE_TRAP_SENSE_ID
            && d.id != ROGUE_UNCANNY_DODGE_ID),
        "no named rogue pillar diagnostic may remain at level 5: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-5 Rogue must still be claim-blocked by the generic chassis diagnostics"
    );
}

// ----- Negative control: level 6 stays unrecognized by this slice (level 6
// was later widened into the supported tranche by
// tests/sd13_rogue_level6_progression.rs; the level-7 negative control now
// lives there) -----

#[test]
fn rogue_level_6_was_later_widened_into_the_supported_tranche() {
    let level_6 = ROGUE_LEVEL5_FIXTURE.replace("class:rogue:5", "class:rogue:6");
    let input = load(&level_6);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "level-6 Rogue was later widened into the supported tranche and must now gain bounded \
         rogue chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ROGUE_EVASION_ID),
        "level-6 Rogue was later widened and must now carry the Evasion explanation"
    );
    assert!(
        has_explanation(&computation, ROGUE_TRAP_SENSE_ID),
        "level-6 Rogue was later widened and must now carry the Trap Sense explanation"
    );
    assert!(
        has_explanation(&computation, ROGUE_UNCANNY_DODGE_ID),
        "level-6 Rogue was later widened and must now carry the Uncanny Dodge explanation"
    );
}

// ----- Negative control: the level-4 fixture is unaffected by this widening -----

#[test]
fn rogue_level4_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 2,
        "Rogue level 4 sneak attack die count must stay 2 (i.e. 2d6), unaffected by the level-5 \
         widening: {}",
        sneak_attack.detail
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level5_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id == ROGUE_EVASION_ID
                || e.id == ROGUE_TRAP_SENSE_ID
                || e.id == ROGUE_UNCANNY_DODGE_ID),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_level5_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL5_FIXTURE.replace(
        "class_level=class:rogue:5",
        "class_level=class:rogue:5\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id == ROGUE_EVASION_ID
                || e.id == ROGUE_TRAP_SENSE_ID
                || e.id == ROGUE_UNCANNY_DODGE_ID),
        "multiclass Rogue must not gain any bounded rogue chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-5 widening -----

#[test]
fn matrix_rogue_row_names_level_5_widening_and_sneak_attack_die_increase() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        rogue.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        rogue.grounding_ref.contains("sd13_rogue_level5_progression"),
        "rogue row must cite the live SD13-E5 level-5 proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("3d6"),
        "rogue partial note must name the level-5 sneak attack die increase to 3d6: {note}"
    );
    assert!(
        note.contains("rogue talent"),
        "rogue partial note must keep naming rogue talents as unproven: {note}"
    );
}
