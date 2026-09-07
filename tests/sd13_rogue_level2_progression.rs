//! SD13-E5 Rogue level-2 progression grounding proof.
//!
//! Widens the accepted Rogue level-1 chassis baseline
//! (`tests/sd13_rogue_level1_chassis_baseline.rs`) to rogue level 2, mirroring
//! the Fighter `supported_fighter_level` / Paladin `supported_paladin_level`
//! level-range-gate idiom (the level-1-only gate `is_single_class_rogue_level1`
//! is generalized to `supported_rogue_level`, an `Option<u8>` helper gated by
//! `MAX_SUPPORTED_ROGUE_LEVEL = 2`). It proves:
//!
//! - base attack bonus at level 2 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at level 1: `2 * 3 / 4 = 1`.
//! - base saves at level 2 are grounded by the same good-Reflex/poor-Fortitude/
//!   poor-Will formulas already grounded at level 1, extended to level 2:
//!   Fortitude `2 / 3 = 0`, Reflex `2 / 2 + 2 = 3`, Will `2 / 3 = 0`.
//! - sneak attack die count stays `1` (i.e. `1d6`) at level 2 — the PF1 Core
//!   Rulebook Rogue class table only increases the sneak attack die every two
//!   rogue levels (1d6 at levels 1-2, 2d6 at level 3+); this is confirmed via
//!   the same formula, not a new record.
//! - Trapfinding at level 2 is `max(2 / 2, 1) = 1`, the same value shape as
//!   level 1, confirmed via the same formula.
//! - Evasion is granted at level 2, grounded as a bounded
//!   identity/recognition record only (value 0, non-fabricated): no damage on
//!   a successful Reflex save against an effect that normally allows half
//!   damage on a successful save, no benefit on a failed save — mirroring how
//!   Divine Grace and Bravery were grounded as flat rules-text records
//!   without folding into an actual saving-throw-resolution or
//!   damage-resolution engine.
//!
//! It deliberately does not implement rogue talents (a level-2+ choice-list
//! feature and a genuinely open-ended talent tree — a new-subsystem-shaped
//! burden left named but unproven), any check-execution engine, trap DC
//! resolution, magic-trap disarm engine, or sneak-attack trigger-condition
//! engine, and it does not ground Rogue level 3+. It also preserves the
//! accepted Rogue level-1 truth (unchanged), the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const ROGUE_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level1_sd13_deterministic_input.txt");

const ROGUE_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level2_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";

// ----- Base attack bonus at level 2 -----

#[test]
fn rogue_level2_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(ROGUE_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 1,
        "Rogue level 2 3/4-BAB progression (2 * 3 / 4) must equal 1: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "rogue base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 2 -----

#[test]
fn rogue_level2_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(ROGUE_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 0, "Rogue level 2 poor Fortitude (2/3) must equal 0");
    assert!(
        fortitude.detail.to_lowercase().contains("poor"),
        "rogue Fortitude explanation must name it as a poor save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 3, "Rogue level 2 good Reflex (2/2+2) must equal 3");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "rogue Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 0, "Rogue level 2 poor Will (2/3) must equal 0");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "rogue Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Sneak attack die count stays 1d6 at level 2 -----

#[test]
fn rogue_level2_sneak_attack_die_count_stays_one_d6() {
    let input = load(ROGUE_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 1,
        "Rogue level 2 sneak attack die count must stay 1 (i.e. 1d6), not a new record: {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.contains("1d6"),
        "rogue sneak-attack explanation must name the +1d6 damage die at level 2: {}",
        sneak_attack.detail
    );
}

// ----- Trapfinding stays the same value shape at level 2 -----

#[test]
fn rogue_level2_trapfinding_is_grounded_via_the_same_formula() {
    let input = load(ROGUE_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 1,
        "rogue Trapfinding bonus at level 2 must be max(2 / 2, 1) = 1, got {}",
        trapfinding.value
    );
    assert!(
        trapfinding.detail.contains("Trapfinding"),
        "rogue Trapfinding explanation must name the Trapfinding class feature: {}",
        trapfinding.detail
    );
}

// ----- Evasion is granted at level 2, as an identity/recognition record only -----

#[test]
fn rogue_level2_grounds_evasion_as_identity_record_only() {
    let input = load(ROGUE_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, ROGUE_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.contains("Evasion"),
        "evasion explanation must name the Evasion class feature: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("reflex")
            && evasion.detail.to_lowercase().contains("half damage"),
        "evasion explanation must state the actual rule text (successful Reflex save against \
         an effect that normally allows half damage): {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("no effect")
            || evasion.detail.to_lowercase().contains("no benefit"),
        "evasion explanation must disclaim any effect on a failed save: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.contains("saving-throw-resolution")
            && evasion.detail.contains("damage-resolution"),
        "evasion explanation must disclaim both a saving-throw-resolution engine and a \
         damage-resolution engine: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 2 must state it is granted, not absent: {}",
        evasion.detail
    );
}

#[test]
fn rogue_level1_evasion_is_a_correct_level_gate_absence() {
    let input = load(ROGUE_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, ROGUE_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion at level 1 must be a correct level-gate absence, value 0: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("absent"),
        "evasion explanation at level 1 must state it is correctly absent: {}",
        evasion.detail
    );
}

// ----- No named Rogue pillar diagnostic remains at level 2 -----

#[test]
fn rogue_level2_leaves_no_named_pillar_diagnostic() {
    let input = load(ROGUE_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.rogue.") && d.id != ROGUE_EVASION_ID),
        "no named rogue pillar diagnostic may remain at level 2: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Rogue must still be claim-blocked by the generic chassis diagnostics"
    );
}

// ----- Negative control: level 3 was later widened into the supported tranche -----

#[test]
fn rogue_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_rogue_level3_progression.rs) widened the level-range gate
    // to level 3 (mirroring the Fighter/Paladin level-range gate idiom) and
    // grounded Trap Sense; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent. The frontier this file's own slice actually drew is now
    // level 4, covered by `rogue_level_4_is_not_promoted_by_this_slice` below.
    let level_3 = ROGUE_LEVEL2_FIXTURE.replace("class:rogue:2", "class:rogue:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.rogue.base_attack_bonus"),
        "level-3 Rogue is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ROGUE_EVASION_ID),
        "level-3 Rogue must keep the Evasion explanation grounded at level 2"
    );
}

// ----- Negative control: level 4 was later widened into the supported tranche -----

#[test]
fn rogue_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 stayed unrecognized. A
    // later SD13-E5 slice (tests/sd13_rogue_level4_progression.rs) widened
    // the level-range gate to level 4 (mirroring the Fighter/Paladin
    // level-range gate idiom) and grounded Uncanny Dodge; this negative
    // control is superseded, not violated — pin the new truth here too so
    // this file stays internally consistent. The frontier is tracked going
    // forward by tests/sd13_rogue_level3_progression.rs and
    // tests/sd13_rogue_level4_progression.rs.
    let level_4 = ROGUE_LEVEL2_FIXTURE.replace("class:rogue:2", "class:rogue:4");
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
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level2_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.") || e.id == ROGUE_EVASION_ID),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_level2_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL2_FIXTURE.replace(
        "class_level=class:rogue:2",
        "class_level=class:rogue:2\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.") || e.id == ROGUE_EVASION_ID),
        "multiclass Rogue must not gain any bounded rogue chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-2 widening -----

#[test]
fn matrix_rogue_row_names_level_2_widening_and_evasion() {
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
        rogue
            .grounding_ref
            .contains("sd13_rogue_level2_progression"),
        "rogue row must cite the live SD13-E5 level-2 proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("evasion"),
        "rogue partial note must name Evasion as newly grounded: {note}"
    );
    assert!(
        note.contains("rogue talent"),
        "rogue partial note must keep naming rogue talents as unproven: {note}"
    );
}
