//! SD13-E5 Rogue level-7 progression grounding proof.
//!
//! Widens the accepted Rogue level-1/level-2/level-3/level-4/level-5/level-6
//! chassis baseline (`tests/sd13_rogue_level1_chassis_baseline.rs`,
//! `tests/sd13_rogue_level2_progression.rs`,
//! `tests/sd13_rogue_level3_progression.rs`,
//! `tests/sd13_rogue_level4_progression.rs`,
//! `tests/sd13_rogue_level5_progression.rs`,
//! `tests/sd13_rogue_level6_progression.rs`) to rogue level 7, mirroring the
//! Fighter/Paladin/Barbarian/Monk level-range-gate idiom (`supported_rogue_level`
//! is generalized from `1..=6` to `1..=7` via `MAX_SUPPORTED_ROGUE_LEVEL = 7`).
//! It proves:
//!
//! - base attack bonus at level 7 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-6: `7 * 3 / 4 = 5`.
//! - base saves at level 7 are grounded by the same good-Reflex/poor-Fortitude/
//!   poor-Will formulas already grounded at levels 1-6, extended to level 7:
//!   Fortitude `7 / 3 = 2`, Reflex `7 / 2 + 2 = 5`, Will `7 / 3 = 2`.
//! - sneak attack die count genuinely rises to `4` (i.e. `4d6`) at level 7,
//!   up from `3` at level 6 — the PF1 Core Rulebook Rogue class table
//!   increases the sneak attack die every two rogue levels (1d6 at levels
//!   1-2, 2d6 at levels 3-4, 3d6 at levels 5-6, 4d6 at level 7); this is
//!   confirmed via the same pre-existing formula, not a new record:
//!   `(7 + 1) / 2 = 4`.
//! - Trapfinding at level 7 stays `max(7 / 2, 1) = 3`, unchanged from level 6
//!   (an integer-division coincidence), confirmed via the same formula.
//! - Evasion stays granted at level 7 (not re-derived), grounded as the same
//!   bounded identity/recognition record already grounded at level 2.
//! - Trap Sense stays granted at level 7, and its magnitude stays `2`
//!   (rogue level / 3 = 7 / 3 = 2, unchanged from level 6 — the next rise is
//!   at 9th level, beyond this bounded slice), grounded via the same
//!   pre-existing flat-magnitude formula already grounded at level 3 (not a
//!   new record).
//! - Uncanny Dodge stays granted at level 7 (not re-derived), grounded as the
//!   same bounded identity/recognition record already grounded at level 4.
//!
//! This cycle was specifically briefed to check whether Rogue gains an
//! actual new class feature at 7th level per the PF1 CRB class table's
//! "Special" column: verified independently against d20pfsrd and
//! legacy.aonprd.com, the level-7 row's "Special" column reads only "Sneak
//! attack +4d6" — no other named entry, unlike the level-6 row's second
//! Rogue Talent slot. The sneak-attack die-count increase is the
//! pre-existing formula's own genuine rise (grounded above, not a new
//! record); no new choice-slot, identity record, or diagnostic was added at
//! level 7.
//!
//! It deliberately does not implement rogue talents (a level-2+/4+/6+
//! choice-list feature and a genuinely open-ended talent tree — a
//! new-subsystem-shaped burden left named but unproven), any
//! check-execution engine, trap DC resolution, magic-trap disarm engine, or
//! sneak-attack trigger-condition engine, and it does not ground Rogue level
//! 8+. It also preserves the accepted Rogue
//! level-1/level-2/level-3/level-4/level-5/level-6 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const ROGUE_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level6_sd13_deterministic_input.txt");

const ROGUE_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level7_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";

// ----- Base attack bonus at level 7 -----

#[test]
fn rogue_level7_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Rogue level 7 3/4-BAB progression (7 * 3 / 4) must equal 5: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "rogue base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 -----

#[test]
fn rogue_level7_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Rogue level 7 poor Fortitude (7/3) must equal 2");
    assert!(
        fortitude.detail.to_lowercase().contains("poor"),
        "rogue Fortitude explanation must name it as a poor save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 5, "Rogue level 7 good Reflex (7/2+2) must equal 5");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "rogue Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 2, "Rogue level 7 poor Will (7/3) must equal 2");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "rogue Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Sneak attack die count genuinely rises to 4d6 at level 7 -----

#[test]
fn rogue_level7_sneak_attack_die_count_rises_to_four_d6() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 4,
        "Rogue level 7 sneak attack die count must rise to 4 (i.e. 4d6): {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.contains("4d6"),
        "rogue sneak-attack explanation must name the 4d6 damage die at level 7: {}",
        sneak_attack.detail
    );
}

// ----- Trapfinding stays 3 at level 7 -----

#[test]
fn rogue_level7_trapfinding_is_grounded_via_the_same_formula() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 3,
        "rogue Trapfinding bonus at level 7 must be max(7 / 2, 1) = 3, got {}",
        trapfinding.value
    );
    assert!(
        trapfinding.detail.contains("Trapfinding"),
        "rogue Trapfinding explanation must name the Trapfinding class feature: {}",
        trapfinding.detail
    );
}

// ----- Evasion stays granted at level 7, not re-derived -----

#[test]
fn rogue_level7_evasion_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, ROGUE_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 7 must state it is granted, not absent: {}",
        evasion.detail
    );
}

// ----- Trap Sense stays at +2 at level 7, not re-derived -----

#[test]
fn rogue_level7_trap_sense_magnitude_stays_two() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 2,
        "Trap Sense magnitude at level 7 must stay rogue level / 3 = 2, unchanged from level 6: \
         {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 7 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

// ----- Uncanny Dodge stays granted at level 7, not re-derived -----

#[test]
fn rogue_level7_uncanny_dodge_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
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
        "uncanny dodge explanation at level 7 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

// ----- No named Rogue pillar diagnostic remains at level 7 -----

#[test]
fn rogue_level7_leaves_no_named_pillar_diagnostic() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.id.starts_with("class_feature.rogue.")
            && d.id != ROGUE_EVASION_ID
            && d.id != ROGUE_TRAP_SENSE_ID
            && d.id != ROGUE_UNCANNY_DODGE_ID),
        "no named rogue pillar diagnostic may remain at level 7: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-7 Rogue must still be claim-blocked by the generic chassis diagnostics"
    );
}

// ----- Negative control retired: level 8 was later widened into the supported tranche -----

#[test]
fn rogue_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = ROGUE_LEVEL7_FIXTURE.replace("class:rogue:7", "class:rogue:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "level-8 Rogue was later widened into the supported tranche and must now gain bounded \
         rogue chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ROGUE_EVASION_ID),
        "level-8 Rogue was later widened and must now carry the Evasion explanation"
    );
    assert!(
        has_explanation(&computation, ROGUE_TRAP_SENSE_ID),
        "level-8 Rogue was later widened and must now carry the Trap Sense explanation"
    );
    assert!(
        has_explanation(&computation, ROGUE_UNCANNY_DODGE_ID),
        "level-8 Rogue was later widened and must now carry the Uncanny Dodge explanation"
    );
}

// ----- Negative control: the level-6 fixture is unaffected by this widening -----

#[test]
fn rogue_level6_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 3,
        "Rogue level 6 sneak attack die count must stay 3 (i.e. 3d6), unaffected by the \
         level-7 widening: {}",
        sneak_attack.detail
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level7_recognition() {
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
fn multiclass_rogue_level7_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL7_FIXTURE.replace(
        "class_level=class:rogue:7",
        "class_level=class:rogue:7\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_rogue_row_names_level_7_widening_and_sneak_attack_increase() {
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
        rogue.grounding_ref.contains("sd13_rogue_level7_progression"),
        "rogue row must cite the live SD13-E5 level-7 proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("4d6") || note.contains("4 sneak attack") || note.contains("sneak attack"),
        "rogue partial note must name the level-7 sneak attack die-count increase: {note}"
    );
    assert!(
        note.contains("rogue talent"),
        "rogue partial note must keep naming rogue talents as unproven: {note}"
    );
}
