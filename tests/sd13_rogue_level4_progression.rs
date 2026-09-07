//! SD13-E5 Rogue level-4 progression grounding proof.
//!
//! Widens the accepted Rogue level-1/level-2/level-3 chassis baseline
//! (`tests/sd13_rogue_level1_chassis_baseline.rs`,
//! `tests/sd13_rogue_level2_progression.rs`,
//! `tests/sd13_rogue_level3_progression.rs`) to rogue level 4, mirroring the
//! Fighter/Paladin/Barbarian/Monk level-range-gate idiom (`supported_rogue_level`
//! is generalized from `1..=3` to `1..=4` via `MAX_SUPPORTED_ROGUE_LEVEL = 4`).
//! It proves:
//!
//! - base attack bonus at level 4 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-3: `4 * 3 / 4 = 3`.
//! - base saves at level 4 are grounded by the same good-Reflex/poor-Fortitude/
//!   poor-Will formulas already grounded at levels 1-3, extended to level 4:
//!   Fortitude `4 / 3 = 1`, Reflex `4 / 2 + 2 = 4`, Will `4 / 3 = 1`.
//! - sneak attack die count stays `2` (i.e. `2d6`) at level 4 — the PF1 Core
//!   Rulebook Rogue class table increases the sneak attack die every two
//!   rogue levels (1d6 at levels 1-2, 2d6 at levels 3-4, 3d6 at level 5+);
//!   this is confirmed via the same formula, not a new record:
//!   `(4 + 1) / 2 = 2`.
//! - Trapfinding at level 4 is `max(4 / 2, 1) = 2`, grown from `1` at levels
//!   1-3 via the same formula (verified against the PF1 CRB: +1/2 rogue
//!   level, minimum +1).
//! - Evasion stays granted at level 4 (not re-derived), grounded as the same
//!   bounded identity/recognition record already grounded at level 2.
//! - Trap Sense stays granted at level 4 (not re-derived), and its magnitude
//!   stays `1` (rogue level / 3 = 1; it next rises at 6th level, beyond this
//!   bounded slice), grounded as the same bounded flat-magnitude record
//!   already grounded at level 3.
//! - Uncanny Dodge, the PF1 Core Rulebook's 4th-level Rogue class feature
//!   (verified independently against d20pfsrd and legacy.aonprd.com: the
//!   Rogue class table's level-4 "Special" column reads "Rogue talent,
//!   uncanny dodge" — NOT the same level as Barbarian's own 2nd-level
//!   Uncanny Dodge grant, verified rather than assumed to match), is grounded
//!   as a bounded identity/recognition record only (`class_feature.rogue.
//!   uncanny_dodge`, value 0) — mirroring the Barbarian Uncanny Dodge idiom
//!   exactly: a level-gate absence below level 4, a granted-but-unexecuted
//!   recognition at or above it, with no flat-footed-state tracking, no
//!   Armor Class computation, and no invisibility-detection engine
//!   implemented. The level-4 row's OTHER named entry, a Rogue Talent (an
//!   open-ended choice-list feature), is deliberately left named-but-unproven
//!   this slice, mirroring the Monk level-2 bonus feat grant / Barbarian Rage
//!   Power precedent.
//!
//! It deliberately does not implement rogue talents (a level-2+ choice-list
//! feature and a genuinely open-ended talent tree — a new-subsystem-shaped
//! burden left named but unproven), any check-execution engine, trap DC
//! resolution, magic-trap disarm engine, or sneak-attack trigger-condition
//! engine, and it does not ground Rogue level 5+. It also preserves the
//! accepted Rogue level-1/level-2/level-3 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const ROGUE_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level3_sd13_deterministic_input.txt");

const ROGUE_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level4_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";

// ----- Base attack bonus at level 4 -----

#[test]
fn rogue_level4_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Rogue level 4 3/4-BAB progression (4 * 3 / 4) must equal 3: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "rogue base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 4 -----

#[test]
fn rogue_level4_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 1, "Rogue level 4 poor Fortitude (4/3) must equal 1");
    assert!(
        fortitude.detail.to_lowercase().contains("poor"),
        "rogue Fortitude explanation must name it as a poor save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 4, "Rogue level 4 good Reflex (4/2+2) must equal 4");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "rogue Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 1, "Rogue level 4 poor Will (4/3) must equal 1");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "rogue Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Sneak attack die count stays 2d6 at level 4 -----

#[test]
fn rogue_level4_sneak_attack_die_count_stays_two_d6() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 2,
        "Rogue level 4 sneak attack die count must stay 2 (i.e. 2d6), not a new record: {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.contains("2d6"),
        "rogue sneak-attack explanation must name the 2d6 damage die at level 4: {}",
        sneak_attack.detail
    );
}

// ----- Trapfinding grows to 2 at level 4 -----

#[test]
fn rogue_level4_trapfinding_is_grounded_via_the_same_formula() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 2,
        "rogue Trapfinding bonus at level 4 must be max(4 / 2, 1) = 2, got {}",
        trapfinding.value
    );
    assert!(
        trapfinding.detail.contains("Trapfinding"),
        "rogue Trapfinding explanation must name the Trapfinding class feature: {}",
        trapfinding.detail
    );
}

// ----- Evasion stays granted at level 4, not re-derived -----

#[test]
fn rogue_level4_evasion_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, ROGUE_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 4 must state it is granted, not absent: {}",
        evasion.detail
    );
}

// ----- Trap Sense stays granted at level 4, not re-derived -----

#[test]
fn rogue_level4_trap_sense_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 1,
        "Trap Sense magnitude at level 4 must stay rogue level / 3 = 1: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 4 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

// ----- Uncanny Dodge is newly granted at level 4, as a bounded identity record -----

#[test]
fn rogue_level4_grounds_uncanny_dodge_as_identity_record_only() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
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
        "uncanny dodge explanation at level 4 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

#[test]
fn rogue_level3_uncanny_dodge_is_a_correct_level_gate_absence() {
    let input = load(ROGUE_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, ROGUE_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge at level 3 must be a correct level-gate absence, value 0: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("absent"),
        "uncanny dodge explanation at level 3 must state it is correctly absent: {}",
        uncanny_dodge.detail
    );
}

// ----- No named Rogue pillar diagnostic remains at level 4 -----

#[test]
fn rogue_level4_leaves_no_named_pillar_diagnostic() {
    let input = load(ROGUE_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.id.starts_with("class_feature.rogue.")
            && d.id != ROGUE_EVASION_ID
            && d.id != ROGUE_TRAP_SENSE_ID
            && d.id != ROGUE_UNCANNY_DODGE_ID),
        "no named rogue pillar diagnostic may remain at level 4: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-4 Rogue must still be claim-blocked by the generic chassis diagnostics"
    );
}

// ----- Negative control: level 5 was later widened into the supported tranche -----

#[test]
fn rogue_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_rogue_level5_progression.rs) widened the level-range gate
    // to level 5 (mirroring the Fighter/Paladin level-range gate idiom) and
    // grounded the genuine sneak-attack die-count increase to 3d6; this
    // negative control is superseded, not violated — pin the new truth here
    // too so this file stays internally consistent. The frontier this
    // file's own slice actually drew is now level 6, covered by
    // `rogue_level_6_is_not_promoted_by_this_slice` in
    // `tests/sd13_rogue_level5_progression.rs`.
    let level_5 = ROGUE_LEVEL4_FIXTURE.replace("class:rogue:4", "class:rogue:5");
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
    assert!(
        has_explanation(&computation, ROGUE_UNCANNY_DODGE_ID),
        "level-5 Rogue must keep the Uncanny Dodge explanation grounded at level 4"
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level4_recognition() {
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
fn multiclass_rogue_level4_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL4_FIXTURE.replace(
        "class_level=class:rogue:4",
        "class_level=class:rogue:4\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-4 widening and Uncanny Dodge -----

#[test]
fn matrix_rogue_row_names_level_4_widening_and_uncanny_dodge() {
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
        rogue.grounding_ref.contains("sd13_rogue_level4_progression"),
        "rogue row must cite the live SD13-E5 level-4 proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("uncanny dodge"),
        "rogue partial note must name Uncanny Dodge as newly grounded: {note}"
    );
    assert!(
        note.contains("rogue talent"),
        "rogue partial note must keep naming rogue talents as unproven: {note}"
    );
}
