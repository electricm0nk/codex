//! SD13-E5 Ranger Favored Terrain choice-and-magnitude grounding proof.
//!
//! Grounds the next honest Ranger pillar burden named in the SD13-E5 level-3
//! progression cycle's analysis: Favored Terrain, the PF1 Core Rulebook's other
//! 3rd-level Ranger "Special" column entry alongside Endurance (verified
//! independently against two primary sources -- d20pfsrd and legacy.aonprd.com --
//! both list "Endurance, favored terrain" as the level-3 entry, and both state the
//! exact bonus text: "+2 bonus on Initiative checks and Knowledge (geography),
//! Perception, Stealth, and Survival skill checks" made when the ranger is in the
//! chosen terrain, drawn from Table: Ranger Favored Terrains' fixed eleven-entry
//! list -- Cold, Desert, Forest, Jungle, Mountain, Plains, Planes, Swamp,
//! Underground, Urban, Water).
//!
//! A prior cycle correctly deferred this burden rather than fabricate it: Favored
//! Terrain needed a new choice-slot with no existing fixture selection, mirroring
//! the same gap the Sorcerer bloodline-class-skill-choice cycle closed for the
//! Arcane bloodline's own class-skill grant.
//!
//! This slice adds a `choice:ranger_favored_terrain` choice-slot to the
//! deterministic Human Ranger level-3 fixture (selection: `terrain:forest`, an
//! arbitrary but deterministic pick among the eleven legal terrain types) and
//! grounds two records, mirroring the already-landed Favored Enemy flat-surface
//! idiom exactly:
//!
//! - a `+0` recognition record naming whichever terrain was selected (raw string
//!   interpolation, no restricted-list validation, exactly mirroring how Favored
//!   Enemy's own choice recognition names whichever enemy type was selected), and
//! - the rule's own flat `+2` magnitude on Initiative/Knowledge (geography)/
//!   Perception/Stealth/Survival checks made in the chosen terrain, grounded as a
//!   standalone, non-applied record, level-gated at 3rd level exactly like
//!   Endurance/Trap Sense/Still Mind (a level-gate absence record below level 3,
//!   the flat magnitude at or above it).
//!
//! It grounds no terrain-detection engine and no application of the +2 to any
//! actual Initiative total or skill-check total anywhere in this codebase, and it
//! does not touch the level-8th/13th/18th additional-terrain and bonus-increase
//! progression (out of scope for this bounded slice).

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level2_sd13_deterministic_input.txt");

const RANGER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level3_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const FAVORED_TERRAIN_CHOICE_ID: &str = "class_chassis.ranger.favored_terrain_choice";
const FAVORED_TERRAIN_BONUS_ID: &str = "class_feature.ranger.favored_terrain";

// ----- Favored Terrain choice recognition at level 3 -----

#[test]
fn ranger_level3_recognizes_the_favored_terrain_choice() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_TERRAIN_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "favored-terrain choice recognition is a recognition record only and must carry no \
         fabricated mechanical value, got {}",
        choice.value
    );
    assert!(
        choice.detail.contains("favored terrain") || choice.detail.contains("Favored Terrain"),
        "favored-terrain choice recognition must name the Favored Terrain feature: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("terrain:forest"),
        "favored-terrain choice recognition must name the chosen terrain from the fixture's \
         choice:ranger_favored_terrain selection: {}",
        choice.detail
    );
}

#[test]
fn ranger_level3_recognizes_whichever_terrain_was_actually_selected() {
    // PF1 grants a genuine choice among eleven terrain types; a different
    // deterministic selection must be recognized identically (raw interpolation,
    // not a hardcoded "Forest").
    let fixture = RANGER_LEVEL3_FIXTURE.replace("terrain:forest", "terrain:underground");
    let input = load(&fixture);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_TERRAIN_CHOICE_ID);
    assert_eq!(choice.value, 0);
    assert!(
        choice.detail.contains("terrain:underground"),
        "favored-terrain choice recognition must name whichever terrain was actually selected, \
         not a hardcoded Forest: {}",
        choice.detail
    );
}

// ----- Favored Terrain flat +2 magnitude at level 3 -----

#[test]
fn ranger_level3_grounds_the_flat_favored_terrain_bonus() {
    let input = load(RANGER_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, FAVORED_TERRAIN_BONUS_ID);
    assert_eq!(
        bonus.value, 2,
        "Ranger Favored Terrain bonus at level 3 must be the flat +2 (PF1 CRB), got {}",
        bonus.value
    );
    for token in [
        "Initiative",
        "Knowledge (geography)",
        "Perception",
        "Stealth",
        "Survival",
    ] {
        assert!(
            bonus.detail.contains(token),
            "favored-terrain bonus must name the '{token}' check: {}",
            bonus.detail
        );
    }
    // Only the flat magnitude is grounded: no terrain-detection or
    // conditional-application engine decides whether the character is actually in
    // the chosen terrain.
    assert!(
        bonus.detail.to_lowercase().contains("terrain-detection"),
        "favored-terrain bonus must disclaim any terrain-detection engine: {}",
        bonus.detail
    );
}

#[test]
fn ranger_level2_favored_terrain_is_a_correct_level_gate_absence() {
    let input = load(RANGER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, FAVORED_TERRAIN_BONUS_ID);
    assert_eq!(
        bonus.value, 0,
        "Favored Terrain at level 2 must be a correct level-gate absence, value 0: {}",
        bonus.detail
    );
    assert!(
        bonus.detail.to_lowercase().contains("absent"),
        "favored-terrain explanation at level 2 must state it is correctly absent: {}",
        bonus.detail
    );
    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_CHOICE_ID),
        "no favored-terrain choice recognition may appear below the level-3 gate"
    );
}

#[test]
fn ranger_without_favored_terrain_choice_grounds_magnitude_but_not_recognition() {
    // The flat level-3 magnitude is a property of the class feature and stays
    // grounded even absent a recorded terrain selection; the choice-recognition
    // record derives strictly from chosen input and must not be fabricated --
    // mirroring the Favored Enemy idiom exactly.
    let without_choice = RANGER_LEVEL3_FIXTURE
        .replace("choice=choice:ranger_favored_terrain:terrain:forest", "");
    let input = load(&without_choice);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_CHOICE_ID),
        "without a chosen favored-terrain type there is nothing to recognize; the recognition \
         record must not be fabricated"
    );
    let bonus = explanation(&computation, FAVORED_TERRAIN_BONUS_ID);
    assert_eq!(
        bonus.value, 2,
        "the flat favored-terrain magnitude must stay grounded without the choice selection"
    );
}

// ----- Negative controls: no leakage onto other classes / levels -----

#[test]
fn fighter_does_not_gain_ranger_favored_terrain_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id == FAVORED_TERRAIN_CHOICE_ID || e.id == FAVORED_TERRAIN_BONUS_ID),
        "the Fighter chassis must not surface any ranger favored-terrain explanation: {:?}",
        fighter_computation.explanations
    );
}

#[test]
fn ranger_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and Favored Terrain did not extend to it. A later SD13-E5
    // slice (tests/sd13_ranger_level4_progression.rs) widened the level-range
    // gate to level 4 and confirmed Favored Terrain stays granted, unchanged;
    // this negative control is superseded, not violated — pin the new truth
    // here too so this file stays internally consistent.
    let level_4 = RANGER_LEVEL3_FIXTURE.replace("class:ranger:3", "class:ranger:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, FAVORED_TERRAIN_BONUS_ID),
        "level-4 Ranger keeps the Favored Terrain explanation grounded since the SD13-E5 level-4 \
         slice: {:?}",
        computation.explanations
    );
}

// ----- Control plane: the matrix row's note now grounds Favored Terrain -----

#[test]
fn matrix_ranger_row_note_names_favored_terrain_as_grounded() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    assert!(
        ranger.grounding_ref.contains("sd13_ranger_favored_terrain_choice"),
        "ranger row must cite the live favored-terrain proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    // Favored Terrain must still be named (by the plain phrase), but no longer
    // described as unimplemented/named-but-unproven.
    assert!(
        note.contains("Favored Terrain"),
        "ranger note must still name Favored Terrain: {note}"
    );
    assert!(
        !note.contains("Favored Terrain stays named-but-unproven"),
        "ranger note must retire the 'Favored Terrain stays named-but-unproven' framing now \
         that it is grounded: {note}"
    );
    // The genuinely still-unproven burdens stay named.
    for token in [
        "conditional-application",
        "spell",
        "level 4",
    ] {
        assert!(
            note.contains(token),
            "ranger note must still name the unproven '{token}' burden: {note}"
        );
    }

    assert!(
        !ranger
            .next_required_uplift
            .contains("Favored Terrain (a new choice-slot burden)"),
        "ranger next-required-uplift must drop the now-grounded Favored Terrain burden: {}",
        ranger.next_required_uplift
    );
}
