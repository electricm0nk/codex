//! SD13-E2 Gnome bounded race-semantics classification slice.
//!
//! Proves the first truthful SD13-E2 race-semantics classification for the
//! Gnome row: the live rules-core surface ingests a deterministic
//! `race:gnome` + `class:fighter:1` input, leaves direct computed evidence that
//! (a) the bounded pilot still produces computed outputs through a non-Human
//! race, (b) the Gnome race seam receives the same non-claim-blocking
//! `race.semantics.unverified` diagnostic the seam emits for any non-Human
//! race, and (c) no Gnome-specific explanation records or Human-only
//! explanations are fabricated. The matrix row for `race.gnome.bounded_semantics`
//! therefore stays honestly classified as `Unverified` / `Observed` /
//! `AwaitingInitialEvidence` — *with* a real grounding reference to the live
//! compute proof surface, instead of being a bare SD-13 roster-scope placeholder.
//!
//! It is intentionally not a Gnome race engine. It grounds no Gnome small-size,
//! slow-speed, low-light vision, defensive training, illusion resistance,
//! hatred, keen senses, or other racial trait math. It does not promote the
//! row off `Unverified`. It only pins the honest bounded classification that
//! the live deterministic pilot emits today.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const GNOME_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_gnome_fighter_level1_sd13_deterministic_input.txt");

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn has_explanation(
    computation: &codex::rules_core::pilot_compute::PilotBaseChassisComputation,
    id: &str,
) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Direct runtime evidence: the bounded Gnome pilot classification -----

#[test]
fn gnome_pilot_produces_computed_outputs_through_a_non_human_race() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The bounded pilot still runs through a Gnome chassis: the deterministic
    // compute path is not claim-blocked by a non-Human race, so it must
    // surface computed Fighter level-1 chassis evidence (base attack bonus
    // and the three base saves) using the established per-stat chassis
    // explanation ids.
    for fighter_chassis_id in [
        "class_chassis.base_attack_bonus",
        "class_chassis.base_save.fortitude",
        "class_chassis.base_save.reflex",
        "class_chassis.base_save.will",
    ] {
        assert!(
            has_explanation(&computation, fighter_chassis_id),
            "Gnome pilot must still surface Fighter level-1 chassis evidence '{fighter_chassis_id}': {:?}",
            computation.explanations
        );
    }

    // The Gnome race seam must surface the bounded, non-claim-blocking
    // unverified-race diagnostic that the seam emits for every non-Human race.
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.semantics.unverified" && !d.claim_blocking),
        "Gnome pilot must emit the bounded non-Human race semantics unverified diagnostic: {:?}",
        computation.diagnostics
    );

    // The Gnome race seam must NOT emit any Human-only race explanation records.
    // Gnome-specific trait math (small size, slow speed, low-light vision,
    // defensive training, illusion resistance, hatred, keen senses) is not
    // implemented and must not be fabricated.
    for human_only in [
        "race.human.ability_bonus_target",
        "race.human.bonus_feat_grant",
    ] {
        assert!(
            !has_explanation(&computation, human_only),
            "Gnome pilot must not fabricate Human-only race explanations: '{human_only}' present in {:?}",
            computation.explanations
        );
    }

    // The Human-only bounded note must not be emitted for a Gnome chassis.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics"),
        "Gnome pilot must not surface the Human-only bounded race note: {:?}",
        computation.diagnostics
    );
}

// ----- Control plane: the matrix keeps the honest Gnome classification -----

#[test]
fn matrix_gnome_row_is_unverified_observed_awaiting_initial_evidence() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let gnome = matrix
        .row("race.gnome.bounded_semantics")
        .expect("gnome bounded race semantics row must exist in the seeded matrix");

    // The bounded Gnome classification remains honest: no Gnome-specific
    // trait math is grounded by this slice, so the row stays Unverified with
    // Observed-tier evidence and AwaitingInitialEvidence freshness.
    assert_eq!(
        gnome.subject_type,
        codex::rules_core::support_state_matrix::MatrixSubjectType::Race
    );
    assert_eq!(gnome.subject_id, "race:gnome");
    assert_eq!(gnome.support_state, SupportState::Unverified);
    assert_eq!(gnome.evidence_tier, EvidenceTier::Observed);
    assert_eq!(
        gnome.evidence_freshness,
        EvidenceFreshness::AwaitingInitialEvidence
    );

    // The slice must point the grounding reference at the live compute proof
    // surface so the row is anchored to a real, re-runnable source path —
    // upgrading it from a bare SD-13 roster-scope placeholder to an honest
    // Observed reference without silently flipping it to Computed. The
    // parallel pattern for the Human row also grounds to `pilot_compute`.
    assert!(
        gnome.grounding_ref.contains("pilot_compute"),
        "gnome row grounding_ref must cite the live pilot compute proof surface: {}",
        gnome.grounding_ref
    );
    // Combined-ref idiom (paladin/human precedent): the row also cites this
    // dedicated proof surface alongside the live compute seam.
    assert!(
        gnome
            .grounding_ref
            .contains("sd13_race_gnome_bounded_semantics"),
        "gnome row grounding_ref must also cite this dedicated proof surface: {}",
        gnome.grounding_ref
    );
    // The dimension must not claim the catch-all diagnostic covers "every
    // non-Human race": since PR #95 the race seam is a dispatcher and Half-Elf
    // carries its own recognition-only diagnostic.
    assert!(
        !gnome.dimension.contains("every non-Human race"),
        "gnome row dimension must not describe the retired every-non-Human catch-all: {}",
        gnome.dimension
    );
}
