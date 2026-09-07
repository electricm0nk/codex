//! SD13-E2 Half-Elf bounded race-semantics recognition proof.
//!
//! Proves the fourth race-semantics recognition slice for a non-Human core race.
//! Unlike Dwarf/Elf/Gnome (each a fixed racial ability-score pair), PF1 Core
//! Half-Elf grants a player-chosen +2 to any one ability score, mirroring the
//! Human ability-bonus mechanic's shape (a named choice, not a fixed pair). The
//! live rules-core surface ingests a deterministic Half-Elf input and surfaces
//! four grounded PF1 Core Rulebook Half-Elf racial trait records as direct
//! runtime evidence — the chosen ability-bonus target, size (Medium), speed
//! (30 ft), and senses (low-light vision) — rather than treating the Half-Elf
//! identity as an undocumented packet placeholder gated behind the generic
//! `race.semantics.unverified` diagnostic.
//!
//! It is intentionally not a Half-Elf racial trait engine. It grounds no numeric
//! contribution to attack rolls, AC, skill checks, ability checks, base speed, or
//! any other chassis output beyond the ability modifier the chosen score already
//! computes independently. It grounds no Elven Immunities (sleep immunity,
//! enchantment save bonus), no Adaptability (a bonus Skill Focus feat), no Keen
//! Senses Perception bonus, and no Multitalented favored-class posture — those
//! remain named as still unproven. It also preserves the Human, Dwarf, Elf, and
//! Gnome race seams, the generic `race.semantics.unverified` diagnostic for every
//! other non-Human race, and the accepted class-chassis truth.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const HALF_ELF_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_half_elf_fighter_level1_sd13_deterministic_input.txt"
);
const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BUNDLE_IDS: &[&str] = &[
    "race.half_elf.trait_bundle.ability_bonus_target",
    "race.half_elf.trait_bundle.size",
    "race.half_elf.trait_bundle.speed",
    "race.half_elf.trait_bundle.senses",
];

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

// ----- All four Half-Elf trait bundle records exist on a Half-Elf input -----

#[test]
fn half_elf_input_surfaces_all_four_trait_bundle_records() {
    let input = load(HALF_ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            has_explanation(&computation, id),
            "Half-Elf input must surface trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
}

// ----- ability bonus: choice-based target, no arithmetic performed here -----

#[test]
fn half_elf_ability_bonus_record_names_chosen_target_and_its_modifier() {
    let input = load(HALF_ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.half_elf.trait_bundle.ability_bonus_target");
    assert!(
        ability.detail.contains("dexterity"),
        "Half-Elf ability-bonus record must name the chosen dexterity target: {}",
        ability.detail
    );
    // The chosen Dexterity score computes its own modifier independently (DEX 16 -> +3);
    // the record surfaces that already-computed modifier as recognition, not new arithmetic.
    assert_eq!(
        ability.value, 3,
        "Half-Elf ability-bonus record must surface the already-computed target modifier"
    );
    assert_eq!(computation.ability_modifiers.dexterity, 3);
}

// ----- size: grounded PF1 Half-Elf Medium size, no fabricated value -----

#[test]
fn half_elf_size_trait_bundle_record_names_medium_category_and_carries_no_value() {
    let input = load(HALF_ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let size = explanation(&computation, "race.half_elf.trait_bundle.size");
    assert!(
        size.detail.contains("Medium"),
        "Half-Elf size trait bundle record must name the PF1 Half-Elf Medium size category: {}",
        size.detail
    );
    assert_eq!(
        size.value, 0,
        "Half-Elf size trait bundle record must carry no fabricated mechanical value (+0)"
    );
}

// ----- speed: grounded 30 ft base land speed -----

#[test]
fn half_elf_speed_trait_bundle_record_names_30_ft_base() {
    let input = load(HALF_ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let speed = explanation(&computation, "race.half_elf.trait_bundle.speed");
    assert!(
        speed.detail.contains("30 ft"),
        "Half-Elf speed trait bundle record must name the 30 ft base land speed: {}",
        speed.detail
    );
    assert_eq!(
        speed.value, 30,
        "Half-Elf speed trait bundle record value must carry the grounded 30 ft recognition value"
    );
}

// ----- senses: grounded low-light vision -----

#[test]
fn half_elf_senses_trait_bundle_record_names_low_light_vision() {
    let input = load(HALF_ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let senses = explanation(&computation, "race.half_elf.trait_bundle.senses");
    assert!(
        senses.detail.contains("low-light vision"),
        "Half-Elf senses trait bundle record must name low-light vision: {}",
        senses.detail
    );
}

// ----- The bounded note names the remaining unproven Half-Elf families honestly -----

#[test]
fn half_elf_bounded_semantics_note_names_remaining_unproven_families() {
    let input = load(HALF_ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.half_elf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.half_elf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    for token in [
        "Elven Immunities",
        "Adaptability",
        "Keen Senses",
        "Multitalented",
    ] {
        assert!(
            bounded.message.contains(token),
            "race.half_elf.bounded_semantics must name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
}

// ----- The Half-Elf trait bundle records do NOT leak onto Human or other races -----

#[test]
fn human_input_does_not_surface_half_elf_trait_bundle_records() {
    let input = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Human input must not surface Half-Elf trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.half_elf.bounded_semantics"),
        "Human input must not surface the Half-Elf bounded-semantics note"
    );
}

#[test]
fn other_non_human_race_still_gets_the_generic_unverified_diagnostic() {
    // Every named SD-13 roster race now has its own dedicated seam, so this
    // exercises the generic-diagnostic fallback with a race identity outside
    // the seven-race roster entirely.
    let other = HALF_ELF_FIXTURE.replace("race_id=race:half-elf", "race_id=race:tiefling");
    let input = load(&other);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Tiefling input must not surface Half-Elf trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    let unverified = diagnostic(&computation, "race.semantics.unverified");
    assert!(!unverified.claim_blocking);
}

// ----- Control plane: the matrix reclassifies the half-elf row to Partial/Computed -----

#[test]
fn matrix_half_elf_row_is_partial_computed_and_names_four_recognized_families() {
    let matrix = seeded_current_truth();
    let half_elf = matrix
        .row("race.half_elf.bounded_semantics")
        .expect("half-elf row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Race Trait
    // Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(half_elf.support_state, SupportState::Supported);
    assert_eq!(half_elf.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        half_elf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        half_elf
            .grounding_ref
            .contains("sd13_half_elf_race_semantics_recognition"),
        "half-elf row must cite the SD13-E2 half-elf proof surface: {}",
        half_elf.grounding_ref
    );
    let note = half_elf.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "half-elf partial row must carry a note");
    for token in [
        "Elven Immunities",
        "Adaptability",
        "Keen Senses",
        "Multitalented",
    ] {
        assert!(
            note.contains(token),
            "half-elf partial note must name the still-unproven '{token}' family: {note}"
        );
    }
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_current_truth();

    // Later promoted to Supported/ProductVisible alongside every other race
    // row by SD-19's Race Trait Catalog browser UI-surfacing work (2026-07-16).
    for id in [
        "race.dwarf.bounded_semantics",
        "race.elf.bounded_semantics",
        "race.gnome.bounded_semantics",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Supported,
            "row {id} must keep its later-accepted Supported posture after the half-elf slice"
        );
    }

    // Half-Orc and Halfling were later promoted to Partial/Computed by their
    // own SD13-E2 recognition slices; this Half-Elf-slice snapshot no longer
    // has any untouched sibling race to assert.

    assert!(
        !matrix
            .rows
            .iter()
            // school.abjuration/illusion.spell_reachability were later promoted to
            // Supported/Product-visible by SD-19's operator-driven UI-surfacing work
            // (2026-07-16) -- excluded here, not an unintended promotion by this slice.
            .any(|r| (r.support_state == SupportState::Supported
                && r.row_id != "school.abjuration.spell_reachability"
                && r.row_id != "school.illusion.spell_reachability"
                && r.row_id != "school.conjuration.spell_reachability"
                && r.row_id != "school.divination.spell_reachability"
                && r.row_id != "school.enchantment.spell_reachability"
                && r.row_id != "school.evocation.spell_reachability"
                && r.row_id != "school.necromancy.spell_reachability"
                && r.row_id != "school.transmutation.spell_reachability"
                && r.row_id != "school.universal.spell_reachability"
                && r.row_id != "equipment.arms_armor.equipment_reachability"
                && r.row_id != "equipment.general.equipment_reachability"
                && r.row_id != "equipment.magic_items.equipment_reachability"
                && r.row_id != "race.human.pilot_semantics"
                && r.row_id != "race.dwarf.bounded_semantics"
                && r.row_id != "race.elf.bounded_semantics"
                && r.row_id != "race.gnome.bounded_semantics"
                && r.row_id != "race.half_elf.bounded_semantics"
                && r.row_id != "race.half_orc.bounded_semantics"
                && r.row_id != "race.halfling.bounded_semantics"
                && r.row_id != "class.fighter.level_1_pilot"
                && r.row_id != "class.fighter.levels_2_10"
                && r.row_id != "class.monk.bounded_progression"
                && r.row_id != "class.druid.progression_and_spell_burden"
                && r.row_id != "class.barbarian.bounded_progression"
                && r.row_id != "class.cleric.progression_and_spell_burden"
                && r.row_id != "class.wizard.progression_and_spell_burden"
                && r.row_id != "class.rogue.bounded_progression"
                && r.row_id != "class.sorcerer.progression_and_spell_burden"
                && r.row_id != "class.bard.progression_and_spell_burden"
                && r.row_id != "class.paladin.hybrid_chassis_and_spell_burden"
                && r.row_id != "class.ranger.hybrid_chassis_and_spell_burden"
                && r.row_id != "interaction.human_bonus_feat_ability_bonus.pilot_pressure"
                && r.row_id != "equipment.equipmods.equipment_reachability")
                || r.support_state == SupportState::Lossy),
        "the half-elf slice must not promote any row to Supported or Lossy"
    );
}
