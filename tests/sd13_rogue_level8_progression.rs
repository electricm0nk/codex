//! SD13-E5 Rogue level-8 progression grounding proof.
//!
//! Widens the accepted Rogue level-1/level-2/level-3/level-4/level-5/level-6/
//! level-7 chassis baseline (`tests/sd13_rogue_level1_chassis_baseline.rs`,
//! `tests/sd13_rogue_level2_progression.rs`,
//! `tests/sd13_rogue_level3_progression.rs`,
//! `tests/sd13_rogue_level4_progression.rs`,
//! `tests/sd13_rogue_level5_progression.rs`,
//! `tests/sd13_rogue_level6_progression.rs`,
//! `tests/sd13_rogue_level7_progression.rs`) to rogue level 8, mirroring the
//! Fighter/Paladin/Barbarian/Monk level-range-gate idiom (`supported_rogue_level`
//! is generalized from `1..=7` to `1..=8` via `MAX_SUPPORTED_ROGUE_LEVEL = 8`).
//! It proves:
//!
//! - base attack bonus at level 8 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-7: `8 * 3 / 4 = 6`.
//! - base saves at level 8 are grounded by the same good-Reflex/poor-Fortitude/
//!   poor-Will formulas already grounded at levels 1-7, extended to level 8:
//!   Fortitude `8 / 3 = 2`, Reflex `8 / 2 + 2 = 6`, Will `8 / 3 = 2`.
//! - sneak attack die count STAYS at `4` (i.e. `4d6`) at level 8, unchanged
//!   from level 7 — the PF1 Core Rulebook Rogue class table only increases
//!   the sneak attack die at odd rogue levels (1, 3, 5, 7, 9); this is
//!   confirmed via the same pre-existing formula, not a new record:
//!   `(8 + 1) / 2 = 4`.
//! - Trapfinding at level 8 genuinely rises to `max(8 / 2, 1) = 4`, up from
//!   `3` at level 7, via the same pre-existing formula (not a new record).
//! - Evasion stays granted at level 8 (not re-derived), grounded as the same
//!   bounded identity/recognition record already grounded at level 2.
//! - Trap Sense stays granted at level 8, and its magnitude stays `2`
//!   (rogue level / 3 = 8 / 3 = 2, unchanged from level 7 — the next rise is
//!   at 9th level, beyond this bounded slice), grounded via the same
//!   pre-existing flat-magnitude formula already grounded at level 3 (not a
//!   new record).
//! - Uncanny Dodge stays granted at level 8 (not re-derived), grounded as the
//!   same bounded identity/recognition record already grounded at level 4.
//! - Improved Uncanny Dodge, the PF1 CRB Rogue class table's 8th-level
//!   "Special" entry (verified independently against d20pfsrd and
//!   legacy.aonprd.com: "Improved uncanny dodge, rogue talent"), is newly
//!   grounded as a bounded identity/recognition record only (value 0,
//!   non-fabricated), mirroring exactly how Barbarian's own Improved Uncanny
//!   Dodge was grounded at barbarian level 5: no flanking-resolution engine,
//!   no attacker-level-comparison engine, and no sneak-attack-trigger engine
//!   exists anywhere in this codebase, so this grounds no actual flanking
//!   immunity or sneak-attack denial.
//!
//! This cycle was specifically briefed to check whether Rogue gains an
//! actual new class feature at 8th level per the PF1 CRB class table's
//! "Special" column: verified independently against d20pfsrd and
//! legacy.aonprd.com, the level-8 row's "Special" column reads "Improved
//! uncanny dodge, rogue talent" — Improved Uncanny Dodge is genuinely new
//! and flat/identity-shaped (grounded above), while the second named entry,
//! a third Rogue Talent slot, is deliberately left named-but-unproven,
//! mirroring the level-2/level-4/level-6 rogue-talent precedent. The
//! sneak-attack die count was specifically verified NOT to rise at level 8
//! (it rises only at odd rogue levels: 1, 3, 5, 7, 9), and Trap Sense was
//! specifically verified NOT to rise at level 8 either (it rises every 3
//! levels: 3, 6, 9).
//!
//! It deliberately does not implement rogue talents (a level-2+/4+/6+/8+
//! choice-list feature and a genuinely open-ended talent tree — a
//! new-subsystem-shaped burden left named but unproven), any
//! check-execution engine, trap DC resolution, magic-trap disarm engine,
//! sneak-attack trigger-condition engine, or Improved Uncanny Dodge's own
//! conditional flanking-resolution/attacker-level-comparison piece, and it
//! does not ground Rogue level 9+. It also preserves the accepted Rogue
//! level-1/level-2/level-3/level-4/level-5/level-6/level-7 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level7_sd13_deterministic_input.txt");

const ROGUE_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level8_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus at level 8 -----

#[test]
fn rogue_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Rogue level 8 3/4-BAB progression (8 * 3 / 4) must equal 6: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "rogue base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 -----

#[test]
fn rogue_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Rogue level 8 poor Fortitude (8/3) must equal 2");
    assert!(
        fortitude.detail.to_lowercase().contains("poor"),
        "rogue Fortitude explanation must name it as a poor save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 6, "Rogue level 8 good Reflex (8/2+2) must equal 6");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "rogue Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 2, "Rogue level 8 poor Will (8/3) must equal 2");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "rogue Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Sneak attack die count stays 4d6 at level 8 (odd-level-only rise) -----

#[test]
fn rogue_level8_sneak_attack_die_count_stays_four_d6() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 4,
        "Rogue level 8 sneak attack die count must stay 4 (i.e. 4d6), unchanged from level 7: {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.contains("4d6"),
        "rogue sneak-attack explanation must name the 4d6 damage die at level 8: {}",
        sneak_attack.detail
    );
}

// ----- Trapfinding genuinely rises to 4 at level 8 -----

#[test]
fn rogue_level8_trapfinding_rises_to_four() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 4,
        "rogue Trapfinding bonus at level 8 must be max(8 / 2, 1) = 4, got {}",
        trapfinding.value
    );
    assert!(
        trapfinding.detail.contains("Trapfinding"),
        "rogue Trapfinding explanation must name the Trapfinding class feature: {}",
        trapfinding.detail
    );
}

// ----- Evasion stays granted at level 8, not re-derived -----

#[test]
fn rogue_level8_evasion_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, ROGUE_EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Evasion must carry no fabricated mechanical value: {}",
        evasion.detail
    );
    assert!(
        evasion.detail.to_lowercase().contains("granted"),
        "evasion explanation at level 8 must state it is granted, not absent: {}",
        evasion.detail
    );
}

// ----- Trap Sense stays at +2 at level 8, not re-derived -----

#[test]
fn rogue_level8_trap_sense_magnitude_stays_two() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 2,
        "Trap Sense magnitude at level 8 must stay rogue level / 3 = 2, unchanged from level 7: \
         {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 8 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

// ----- Uncanny Dodge stays granted at level 8, not re-derived -----

#[test]
fn rogue_level8_uncanny_dodge_stays_granted_not_re_derived() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
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
        "uncanny dodge explanation at level 8 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

// ----- Improved Uncanny Dodge is newly granted at level 8 -----

#[test]
fn rogue_level8_improved_uncanny_dodge_is_newly_granted() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_uncanny_dodge = explanation(&computation, ROGUE_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge must carry no fabricated mechanical value: {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        improved_uncanny_dodge.detail.to_lowercase().contains("flanked"),
        "improved uncanny dodge explanation must state the actual rule text (can no longer be \
         flanked): {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        improved_uncanny_dodge.detail.to_lowercase().contains("granted"),
        "improved uncanny dodge explanation at level 8 must state it is granted, not absent: {}",
        improved_uncanny_dodge.detail
    );
}

// ----- Improved Uncanny Dodge is correctly absent below level 8 -----

#[test]
fn rogue_level7_improved_uncanny_dodge_is_correctly_absent() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_uncanny_dodge = explanation(&computation, ROGUE_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge must carry no fabricated mechanical value: {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        improved_uncanny_dodge.detail.to_lowercase().contains("absent"),
        "improved uncanny dodge explanation at level 7 must state it is correctly absent: {}",
        improved_uncanny_dodge.detail
    );
}

// ----- No named Rogue pillar diagnostic remains at level 8 -----

#[test]
fn rogue_level8_leaves_no_named_pillar_diagnostic() {
    let input = load(ROGUE_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.id.starts_with("class_feature.rogue.")
            && d.id != ROGUE_EVASION_ID
            && d.id != ROGUE_TRAP_SENSE_ID
            && d.id != ROGUE_UNCANNY_DODGE_ID
            && d.id != ROGUE_IMPROVED_UNCANNY_DODGE_ID),
        "no named rogue pillar diagnostic may remain at level 8: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-8 Rogue must still be claim-blocked by the generic chassis diagnostics"
    );
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn rogue_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = ROGUE_LEVEL8_FIXTURE.replace("class:rogue:8", "class:rogue:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "level-9 Rogue is now recognized by the later level-9 widening slice \
         (tests/sd13_rogue_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-7 fixture is unaffected by this widening -----

#[test]
fn rogue_level7_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 3,
        "Rogue level 7 Trapfinding bonus must stay 3, unaffected by the level-8 widening: {}",
        trapfinding.detail
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level8_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id == ROGUE_EVASION_ID
                || e.id == ROGUE_TRAP_SENSE_ID
                || e.id == ROGUE_UNCANNY_DODGE_ID
                || e.id == ROGUE_IMPROVED_UNCANNY_DODGE_ID),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_level8_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL8_FIXTURE.replace(
        "class_level=class:rogue:8",
        "class_level=class:rogue:8\nclass_level=class:fighter:1",
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
                || e.id == ROGUE_UNCANNY_DODGE_ID
                || e.id == ROGUE_IMPROVED_UNCANNY_DODGE_ID),
        "multiclass Rogue must not gain any bounded rogue chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_rogue_row_names_level_8_widening_and_improved_uncanny_dodge() {
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
        rogue.grounding_ref.contains("sd13_rogue_level8_progression"),
        "rogue row must cite the live SD13-E5 level-8 proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("improved uncanny dodge") || note.contains("Improved Uncanny Dodge"),
        "rogue partial note must name the level-8 Improved Uncanny Dodge grant: {note}"
    );
    assert!(
        note.contains("rogue talent"),
        "rogue partial note must keep naming rogue talents as unproven: {note}"
    );
}
