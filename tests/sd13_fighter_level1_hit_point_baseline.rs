//! SD13-E5 Fighter level-1 hit-point baseline proof.
//!
//! Grounds the Fighter level-1 hit-point milestone on the deterministic Human
//! Fighter level-1 pilot surface: level-1 hit points = 10 (the maximized d10
//! Fighter hit die at 1st level) + the Constitution modifier. On the GE-06
//! fixture Constitution is 14, so the modifier is floor(14 / 2) - 5 = +2 and
//! the grounded value is 10 + 2 = 12. The value is emitted as a standalone
//! grounded explanation record only; it is wired into no view-model total and
//! changes no derived combat/defense output.
//!
//! Honestly unproven and named rather than hidden: the favored-class +1 hp /
//! +1 skill-rank choice (no input surface exists for it), hit points at levels
//! 2 and above, and Toughness / feat hit-point interplay.
//!
//! Non-goals:
//! - no hit-point computation for Fighter levels 2+ (no average/rolled HD policy)
//! - no favored-class bonus input surface or computation
//! - no change to the Human Fighter L1 golden path's zero-claim-blocking posture
//! - no promotion of the `class.fighter.level_1_pilot` row beyond Partial

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

// ----- Runtime evidence: the grounded level-1 hit-point record -----

#[test]
fn fighter_level1_emits_grounded_level_1_hit_point_explanation() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let hit_points = explanation(&computation, "class_chassis.fighter.level_1_hit_points");
    // Level-1 hit points = 10 (max d10 HD at 1st level) + Constitution modifier
    // (+2 from the fixture's Constitution 14) = 12.
    assert_eq!(
        hit_points.value, 12,
        "Fighter level-1 hit points must be 10 (max d10 HD) + 2 (Constitution modifier) = 12"
    );
    // The detail must show the real derivation, not a bare number.
    for token in ["d10", "10", "Constitution", "12"] {
        assert!(
            hit_points.detail.contains(token),
            "level-1 hit-point detail must name derivation token '{token}': {}",
            hit_points.detail
        );
    }
}

#[test]
fn fighter_level1_hit_point_detail_names_the_unproven_burdens() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let hit_points = explanation(&computation, "class_chassis.fighter.level_1_hit_points");
    // The record must keep the honest boundary visible: the favored-class
    // +1 hp / +1 skill-rank choice (no input surface exists), hit points at
    // levels 2+, and Toughness / feat hit-point interplay stay unproven.
    for token in ["favored-class", "levels 2", "Toughness"] {
        assert!(
            hit_points.detail.contains(token),
            "level-1 hit-point detail must name the unproven burden '{token}': {}",
            hit_points.detail
        );
    }
}

#[test]
fn fighter_level1_hit_point_record_changes_no_derived_output_total() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The hit-point record is standalone: every previously proven derived
    // output on the deterministic L1 pilot surface keeps its exact value.
    assert_eq!(computation.base_attack_bonus, 1);
    assert_eq!(computation.baseline_melee_attack_bonus, 6);
    assert_eq!(computation.baseline_armor_class, 17);
}

#[test]
fn fighter_level1_golden_path_still_emits_zero_claim_blocking_diagnostics() {
    let input = load(LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let claim_blocking: Vec<_> = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .collect();
    assert!(
        claim_blocking.is_empty(),
        "the Human Fighter L1 golden path must stay free of claim-blocking diagnostics, got {claim_blocking:?}"
    );
}

// ----- Control plane: the matrix row reclassifies hit points as proven -----

#[test]
fn matrix_fighter_level_1_row_moves_hit_points_to_the_proven_surface() {
    let matrix = seeded_current_truth();
    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 pilot row must exist");
    let note = level_1.blocker_or_lossiness_note;

    // Hit points join the proven (computed) level-1 surface enumeration.
    assert!(
        note.contains("level-1 hit points"),
        "Fighter L1 note must name the proven level-1 hit-point milestone: {note}"
    );
    // The old remaining-unproven phrasing must be gone.
    assert!(
        !note.contains("hit point computation (Fighter d10 HD"),
        "Fighter L1 note must no longer list level-1 hit-point computation as unproven: {note}"
    );
    // The honest residue stays named.
    for token in ["favored-class", "levels 2+", "Toughness"] {
        assert!(
            note.contains(token),
            "Fighter L1 note must keep the unproven hit-point burden '{token}' named: {note}"
        );
    }
}

#[test]
fn matrix_fighter_level_1_row_stays_partial_and_cites_this_proof_surface() {
    let matrix = seeded_current_truth();
    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 pilot row must exist");

    // Grounding one more milestone did not silently promote the row at this
    // slice; it was later promoted to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(level_1.support_state, SupportState::Supported);
    assert_eq!(level_1.evidence_tier, EvidenceTier::ProductVisible);

    // The row must cite this slice's proof surface alongside the existing
    // mandatory-milestone classification proof (combined-literal idiom).
    assert!(
        level_1
            .grounding_ref
            .contains("sd13_fighter_level1_hit_point_baseline"),
        "Fighter L1 row must cite the hit-point baseline proof surface: {}",
        level_1.grounding_ref
    );
    assert!(
        level_1
            .grounding_ref
            .contains("sd13_fighter_level1_mandatory_milestone_classification"),
        "Fighter L1 row must keep citing the mandatory-milestone proof surface: {}",
        level_1.grounding_ref
    );
}
