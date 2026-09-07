//! SD13-E5 Rogue level-3 progression grounding proof.
//!
//! Widens the accepted Rogue level-1/level-2 chassis baseline
//! (`tests/sd13_rogue_level1_chassis_baseline.rs`,
//! `tests/sd13_rogue_level2_progression.rs`) to rogue level 3, mirroring the
//! Fighter/Paladin/Barbarian/Monk level-range-gate idiom (`supported_rogue_level`
//! is generalized from `1..=2` to `1..=3` via `MAX_SUPPORTED_ROGUE_LEVEL = 3`).
//! It proves:
//!
//! - base attack bonus at level 3 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-2: `3 * 3 / 4 = 2`.
//! - base saves at level 3 are grounded by the same good-Reflex/poor-Fortitude/
//!   poor-Will formulas already grounded at levels 1-2, extended to level 3:
//!   Fortitude `3 / 3 = 1`, Reflex `3 / 2 + 2 = 3`, Will `3 / 3 = 1`.
//! - sneak attack die count increases to `2` (i.e. `2d6`) at level 3 — the PF1
//!   Core Rulebook Rogue class table increases the sneak attack die every two
//!   rogue levels (1d6 at levels 1-2, 2d6 at level 3+); this is confirmed via
//!   the same formula, not a new record: `(3 + 1) / 2 = 2`.
//! - Trapfinding at level 3 is `max(3 / 2, 1) = 1`, the same value shape as
//!   levels 1-2, confirmed via the same formula (verified against the PF1 CRB
//!   this remains flat at level 3).
//! - Evasion stays granted at level 3 (not re-derived), grounded as the same
//!   bounded identity/recognition record already grounded at level 2.
//! - Trap Sense, the PF1 Core Rulebook's 3rd-level Rogue class feature
//!   (verified independently against d20pfsrd and legacy.aonprd.com: "+1
//!   bonus on Reflex saves made to avoid traps and a +1 dodge bonus to AC
//!   against attacks made by traps," rising further at 6th/9th/12th/15th/18th
//!   rogue level), is grounded as a bounded flat-magnitude record only
//!   (`rogue level / 3`, floor; `1` at level 3) — mirroring the Fighter
//!   Bravery / Paladin Divine Grace idiom: the magnitude is never applied to
//!   any actual Reflex-save total or AC total, since no saving-throw-resolution
//!   or armor-class-resolution engine exists in this codebase, and no
//!   trap-detection or trap-triggering engine exists to decide when it would
//!   apply.
//!
//! It deliberately does not implement rogue talents (a level-2+ choice-list
//! feature and a genuinely open-ended talent tree — a new-subsystem-shaped
//! burden left named but unproven), any check-execution engine, trap DC
//! resolution, magic-trap disarm engine, or sneak-attack trigger-condition
//! engine, and it does not ground Rogue level 4+. It also preserves the
//! accepted Rogue level-1/level-2 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const ROGUE_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level2_sd13_deterministic_input.txt");

const ROGUE_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level3_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";

// ----- Base attack bonus at level 3 -----

#[test]
fn rogue_level3_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(ROGUE_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 2,
        "Rogue level 3 3/4-BAB progression (3 * 3 / 4) must equal 2: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "rogue base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 3 -----

#[test]
fn rogue_level3_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(ROGUE_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 1, "Rogue level 3 poor Fortitude (3/3) must equal 1");
    assert!(
        fortitude.detail.to_lowercase().contains("poor"),
        "rogue Fortitude explanation must name it as a poor save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 3, "Rogue level 3 good Reflex (3/2+2) must equal 3");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "rogue Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 1, "Rogue level 3 poor Will (3/3) must equal 1");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "rogue Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Sneak attack die count increases to 2d6 at level 3 -----

#[test]
fn rogue_level3_sneak_attack_die_count_increases_to_two_d6() {
    let input = load(ROGUE_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 2,
        "Rogue level 3 sneak attack die count must increase to 2 (i.e. 2d6): {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.contains("2d6"),
        "rogue sneak-attack explanation must name the +2d6 damage die at level 3: {}",
        sneak_attack.detail
    );
}

// ----- Trapfinding stays the same value shape at level 3 -----

#[test]
fn rogue_level3_trapfinding_is_grounded_via_the_same_formula() {
    let input = load(ROGUE_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 1,
        "rogue Trapfinding bonus at level 3 must be max(3 / 2, 1) = 1, got {}",
        trapfinding.value
    );
    assert!(
        trapfinding.detail.contains("Trapfinding"),
        "rogue Trapfinding explanation must name the Trapfinding class feature: {}",
        trapfinding.detail
    );
}

// ----- Evasion stays granted at level 3, not re-derived -----

#[test]
fn rogue_level3_evasion_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, ROGUE_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 3 must state it is granted, not absent: {}",
        evasion.detail
    );
}

// ----- Trap Sense is newly granted at level 3, as a bounded flat-magnitude record -----

#[test]
fn rogue_level3_grounds_trap_sense_as_flat_magnitude_record() {
    let input = load(ROGUE_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 1,
        "Trap Sense magnitude at level 3 must be rogue level / 3 = 1: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.contains("Trap Sense"),
        "trap sense explanation must name the Trap Sense class feature: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("reflex")
            && trap_sense.detail.to_lowercase().contains("ac"),
        "trap sense explanation must state the actual rule text (Reflex saves against traps and \
         AC against trap attacks): {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("never applied")
            || trap_sense.detail.to_lowercase().contains("not applied")
            || trap_sense.detail.to_lowercase().contains("no saving-throw-resolution"),
        "trap sense explanation must disclaim being applied to any actual save/AC total: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 3 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

#[test]
fn rogue_level2_trap_sense_is_a_correct_level_gate_absence() {
    let input = load(ROGUE_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 0,
        "Trap Sense at level 2 must be a correct level-gate absence, value 0: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("absent"),
        "trap sense explanation at level 2 must state it is correctly absent: {}",
        trap_sense.detail
    );
}

// ----- No named Rogue pillar diagnostic remains at level 3 -----

#[test]
fn rogue_level3_leaves_no_named_pillar_diagnostic() {
    let input = load(ROGUE_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.id.starts_with("class_feature.rogue.")
            && d.id != ROGUE_EVASION_ID
            && d.id != ROGUE_TRAP_SENSE_ID),
        "no named rogue pillar diagnostic may remain at level 3: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-3 Rogue must still be claim-blocked by the generic chassis diagnostics"
    );
}

// ----- Negative control: level 4 was later widened into the supported tranche -----

#[test]
fn rogue_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_rogue_level4_progression.rs) widened the level-range gate
    // to level 4 (mirroring the Fighter/Paladin level-range gate idiom) and
    // grounded Uncanny Dodge; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent. The frontier this file's own slice actually drew is now
    // level 5, covered by `rogue_level_5_is_not_promoted_by_this_slice` below.
    let level_4 = ROGUE_LEVEL3_FIXTURE.replace("class:rogue:3", "class:rogue:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.rogue.base_attack_bonus"),
        "level-4 Rogue is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ROGUE_EVASION_ID),
        "level-4 Rogue must keep the Evasion explanation grounded at level 2"
    );
    assert!(
        has_explanation(&computation, ROGUE_TRAP_SENSE_ID),
        "level-4 Rogue must keep the Trap Sense explanation grounded at level 3"
    );
}

// ----- Negative control: level 5 was later widened into the supported tranche -----

#[test]
fn rogue_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 stayed unrecognized. Two
    // later SD13-E5 slices (tests/sd13_rogue_level4_progression.rs,
    // tests/sd13_rogue_level5_progression.rs) widened the level-range gate
    // through level 5 (mirroring the Fighter/Paladin level-range gate idiom)
    // and grounded Uncanny Dodge (level 4) and the genuine sneak-attack
    // die-count increase to 3d6 (level 5); this negative control is
    // superseded, not violated — pin the new truth here too so this file
    // stays internally consistent.
    let level_5 = ROGUE_LEVEL3_FIXTURE.replace("class:rogue:3", "class:rogue:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.rogue.base_attack_bonus"),
        "level-5 Rogue is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ROGUE_EVASION_ID),
        "level-5 Rogue must keep the Evasion explanation grounded at level 2"
    );
    assert!(
        has_explanation(&computation, ROGUE_TRAP_SENSE_ID),
        "level-5 Rogue must keep the Trap Sense explanation grounded at level 3"
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level3_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id == ROGUE_EVASION_ID
                || e.id == ROGUE_TRAP_SENSE_ID),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_level3_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL3_FIXTURE.replace(
        "class_level=class:rogue:3",
        "class_level=class:rogue:3\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id == ROGUE_EVASION_ID
                || e.id == ROGUE_TRAP_SENSE_ID),
        "multiclass Rogue must not gain any bounded rogue chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-3 widening and Trap Sense -----

#[test]
fn matrix_rogue_row_names_level_3_widening_and_trap_sense() {
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
        rogue.grounding_ref.contains("sd13_rogue_level3_progression"),
        "rogue row must cite the live SD13-E5 level-3 proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("trap sense"),
        "rogue partial note must name Trap Sense as newly grounded: {note}"
    );
    assert!(
        note.contains("rogue talent"),
        "rogue partial note must keep naming rogue talents as unproven: {note}"
    );
}
