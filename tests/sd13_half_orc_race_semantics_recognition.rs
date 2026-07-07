//! SD13-E2 Half-Orc bounded race-semantics recognition proof.
//!
//! Proves the fifth race-semantics recognition slice for a non-Human core race.
//! Like Half-Elf, PF1 Core Half-Orc grants a player-chosen +2 to any one ability
//! score, mirroring the Human ability-bonus mechanic's shape rather than a fixed
//! pair. The live rules-core surface ingests a deterministic Half-Orc input and
//! surfaces four grounded PF1 Core Rulebook Half-Orc racial trait records as
//! direct runtime evidence — the chosen ability-bonus target, size (Medium),
//! speed (30 ft), and senses (Darkvision 60 ft) — rather than treating the
//! Half-Orc identity as an undocumented packet placeholder gated behind the
//! generic `race.semantics.unverified` diagnostic.
//!
//! It is intentionally not a Half-Orc racial trait engine. It grounds no numeric
//! contribution to attack rolls, AC, skill checks, ability checks, base speed, or
//! any other chassis output beyond the ability modifier the chosen score already
//! computes independently. It grounds no Intimidating skill bonus, no Orc
//! Ferocity, and no weapon familiarity grant — those remain named as still
//! unproven. It also preserves the Human, Dwarf, Elf, Gnome, and Half-Elf race
//! seams, the generic `race.semantics.unverified` diagnostic for every other
//! non-Human race, and the accepted class-chassis truth.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const HALF_ORC_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_race_semantics_recognition_input.txt"
);
const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BUNDLE_IDS: &[&str] = &[
    "race.half_orc.trait_bundle.ability_bonus_target",
    "race.half_orc.trait_bundle.size",
    "race.half_orc.trait_bundle.speed",
    "race.half_orc.trait_bundle.senses",
];

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

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

fn diagnostic<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        })
}

// ----- All four Half-Orc trait bundle records exist on a Half-Orc input -----

#[test]
fn half_orc_input_surfaces_all_four_trait_bundle_records() {
    let input = load(HALF_ORC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            has_explanation(&computation, id),
            "Half-Orc input must surface trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
}

// ----- ability bonus: choice-based target, no arithmetic performed here -----

#[test]
fn half_orc_ability_bonus_record_names_chosen_target_and_its_modifier() {
    let input = load(HALF_ORC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.half_orc.trait_bundle.ability_bonus_target");
    assert!(
        ability.detail.contains("strength"),
        "Half-Orc ability-bonus record must name the chosen strength target: {}",
        ability.detail
    );
    // The chosen Strength score computes its own modifier independently (STR 16 -> +3);
    // the record surfaces that already-computed modifier as recognition, not new arithmetic.
    assert_eq!(
        ability.value, 3,
        "Half-Orc ability-bonus record must surface the already-computed target modifier"
    );
    assert_eq!(computation.ability_modifiers.strength, 3);
}

// ----- size: grounded PF1 Half-Orc Medium size, no fabricated value -----

#[test]
fn half_orc_size_trait_bundle_record_names_medium_category_and_carries_no_value() {
    let input = load(HALF_ORC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let size = explanation(&computation, "race.half_orc.trait_bundle.size");
    assert!(
        size.detail.contains("Medium"),
        "Half-Orc size trait bundle record must name the PF1 Half-Orc Medium size category: {}",
        size.detail
    );
    assert_eq!(
        size.value, 0,
        "Half-Orc size trait bundle record must carry no fabricated mechanical value (+0)"
    );
}

// ----- speed: grounded 30 ft base land speed -----

#[test]
fn half_orc_speed_trait_bundle_record_names_30_ft_base() {
    let input = load(HALF_ORC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let speed = explanation(&computation, "race.half_orc.trait_bundle.speed");
    assert!(
        speed.detail.contains("30 ft"),
        "Half-Orc speed trait bundle record must name the 30 ft base land speed: {}",
        speed.detail
    );
    assert_eq!(
        speed.value, 30,
        "Half-Orc speed trait bundle record value must carry the grounded 30 ft recognition value"
    );
}

// ----- senses: grounded Darkvision 60 ft -----

#[test]
fn half_orc_senses_trait_bundle_record_names_darkvision_60_ft() {
    let input = load(HALF_ORC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let senses = explanation(&computation, "race.half_orc.trait_bundle.senses");
    assert!(
        senses.detail.contains("Darkvision") && senses.detail.contains("60 ft"),
        "Half-Orc senses trait bundle record must name Darkvision 60 ft: {}",
        senses.detail
    );
    assert_eq!(
        senses.value, 60,
        "Half-Orc senses trait bundle record value must carry the grounded 60 ft recognition value"
    );
}

// ----- The bounded note names the remaining unproven Half-Orc families honestly -----

#[test]
fn half_orc_bounded_semantics_note_names_remaining_unproven_families() {
    let input = load(HALF_ORC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.half_orc.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.half_orc.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    for token in ["Intimidating", "Orc Ferocity", "weapon familiarity"] {
        assert!(
            bounded.message.contains(token),
            "race.half_orc.bounded_semantics must name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
}

// ----- The Half-Orc trait bundle records do NOT leak onto Human or other races -----

#[test]
fn human_input_does_not_surface_half_orc_trait_bundle_records() {
    let input = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Human input must not surface Half-Orc trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.half_orc.bounded_semantics"),
        "Human input must not surface the Half-Orc bounded-semantics note"
    );
}

#[test]
fn other_non_human_race_still_gets_the_generic_unverified_diagnostic() {
    let halfling = HALF_ORC_FIXTURE.replace("race_id=race:half-orc", "race_id=race:halfling");
    let input = load(&halfling);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Halfling input must not surface Half-Orc trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    let unverified = diagnostic(&computation, "race.semantics.unverified");
    assert!(!unverified.claim_blocking);
}

// ----- Control plane: the matrix reclassifies the half-orc row to Partial/Computed -----

#[test]
fn matrix_half_orc_row_is_partial_computed_and_names_four_recognized_families() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let half_orc = matrix
        .row("race.half_orc.bounded_semantics")
        .expect("half-orc row must exist");

    assert_eq!(half_orc.support_state, SupportState::Partial);
    assert_eq!(half_orc.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        half_orc.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        half_orc
            .grounding_ref
            .contains("sd13_half_orc_race_semantics_recognition"),
        "half-orc row must cite the SD13-E2 half-orc proof surface: {}",
        half_orc.grounding_ref
    );
    let note = half_orc.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "half-orc partial row must carry a note");
    for token in ["Intimidating", "Orc Ferocity", "weapon familiarity"] {
        assert!(
            note.contains(token),
            "half-orc partial note must name the still-unproven '{token}' family: {note}"
        );
    }
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    for id in [
        "race.dwarf.bounded_semantics",
        "race.elf.bounded_semantics",
        "race.gnome.bounded_semantics",
        "race.half_elf.bounded_semantics",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {id} must keep its accepted Partial posture after the half-orc slice"
        );
    }

    let halfling = matrix
        .row("race.halfling.bounded_semantics")
        .expect("halfling row must exist");
    assert_eq!(
        halfling.support_state,
        SupportState::Unverified,
        "halfling row must stay Unverified after the half-orc slice"
    );

    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the half-orc slice must not promote any row to Supported or Lossy"
    );
}
