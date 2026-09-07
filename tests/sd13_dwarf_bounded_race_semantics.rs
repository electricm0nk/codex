//! SD13-E2 Dwarf bounded race-semantics recognition proof.
//!
//! Proves the first race-semantics recognition slice for a non-Human core race
//! (mirroring the SD13-E6-F3a Human trait bundle pattern): the live rules-core
//! surface ingests a deterministic Dwarf input and surfaces four grounded PF1 Core
//! Rulebook Dwarf racial trait records as direct runtime evidence — ability
//! modifiers (+2 Constitution / -2 Charisma), size (Medium), speed (20 ft, never
//! modified by armor or encumbrance), and senses (Darkvision 60 ft) — rather than
//! treating the Dwarf identity as an undocumented packet placeholder gated behind
//! the generic `race.semantics.unverified` diagnostic.
//!
//! It is intentionally not a Dwarf racial trait engine. It grounds no numeric
//! contribution to attack rolls, AC, skill checks, ability checks, base speed, CMD,
//! or any other chassis output. It grounds no racial bonus feat (PF1 core Dwarves
//! gain none), no skill or derived-stat modifier (Stonecunning, Appraise, Defensive
//! Training, Hardy, Stability, Hatred), and no weapon familiarity grant — those
//! remain named as still unproven. It also preserves the Human race seam, the
//! generic `race.semantics.unverified` diagnostic for every other non-Human race,
//! and the accepted Fighter/Barbarian/hybrid/spell-baseline class truth.

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

const DWARF_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_dwarf_fighter_level1_sd13_deterministic_input.txt");
const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BUNDLE_IDS: &[&str] = &[
    "race.dwarf.trait_bundle.ability_modifiers",
    "race.dwarf.trait_bundle.size",
    "race.dwarf.trait_bundle.speed",
    "race.dwarf.trait_bundle.senses",
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

// ----- All four Dwarf trait bundle records exist on a Dwarf input -----

#[test]
fn dwarf_input_surfaces_all_four_trait_bundle_records() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            has_explanation(&computation, id),
            "Dwarf input must surface trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
}

// ----- ability modifiers: grounded +2 Con / -2 Cha, no arithmetic performed here -----

#[test]
fn dwarf_ability_modifiers_record_names_con_bonus_and_cha_penalty() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.dwarf.trait_bundle.ability_modifiers");
    assert!(
        ability.detail.contains("+2") && ability.detail.contains("Constitution"),
        "Dwarf ability modifiers record must name the +2 Constitution adjustment: {}",
        ability.detail
    );
    assert!(
        ability.detail.contains("-2") && ability.detail.contains("Charisma"),
        "Dwarf ability modifiers record must name the -2 Charisma adjustment: {}",
        ability.detail
    );
    // Recognition record only: the chosen ability scores are understood to already
    // reflect the fixed racial adjustment; no arithmetic is performed on this seam.
    assert_eq!(
        ability.value, 0,
        "Dwarf ability modifiers record must carry no fabricated mechanical value (+0)"
    );
    // The chosen Constitution/Charisma scores still compute their own modifiers
    // independently (CON 16 -> +3, CHA 6 -> -2).
    assert_eq!(computation.ability_modifiers.constitution, 3);
    assert_eq!(computation.ability_modifiers.charisma, -2);
}

// ----- v0.6 alpha swarm: the record was missing its real +2 Wisdom adjustment
// entirely (same systemic gap as Elf's missing +2 Intelligence) -- verified
// independently against the real PCGen corpus
// (core_essentials/races/dwarf/dwarf_abilities_race.lst:18's "Dwarf Racial
// Default" row: BONUS:STAT|CON,WIS|2|TYPE=Racial, BONUS:STAT|CHA|-2|TYPE=Racial)
// before writing this test, not just trusting the fix commit's own citation. -----

#[test]
fn dwarf_ability_modifiers_record_now_names_the_wisdom_adjustment_too() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.dwarf.trait_bundle.ability_modifiers");
    assert!(
        ability.detail.contains("+2") && ability.detail.contains("Wisdom"),
        "Dwarf ability modifiers record must name the +2 Wisdom adjustment: {}",
        ability.detail
    );
    // The Con/Cha adjustments must still be named alongside the new Wisdom one.
    assert!(
        ability.detail.contains("Constitution") && ability.detail.contains("Charisma"),
        "Dwarf ability modifiers record must still name Constitution and Charisma: {}",
        ability.detail
    );
    assert_eq!(
        ability.value, 0,
        "Dwarf ability modifiers record must still carry no fabricated mechanical value (+0)"
    );
    // Fixture Wisdom is 10 -> modifier +0; independently verified, not assumed.
    assert_eq!(computation.ability_modifiers.wisdom, 0);
}

// ----- size: grounded PF1 Dwarf Medium size, no fabricated value -----

#[test]
fn dwarf_size_trait_bundle_record_names_medium_category_and_carries_no_value() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let size = explanation(&computation, "race.dwarf.trait_bundle.size");
    assert!(
        size.detail.contains("Medium"),
        "Dwarf size trait bundle record must name the PF1 Dwarf Medium size category: {}",
        size.detail
    );
    assert_eq!(
        size.value, 0,
        "Dwarf size trait bundle record must carry no fabricated mechanical value (+0)"
    );
}

// ----- speed: grounded 20 ft base land speed, never modified by armor/encumbrance -----

#[test]
fn dwarf_speed_trait_bundle_record_names_20_ft_base_unmodified_by_armor() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let speed = explanation(&computation, "race.dwarf.trait_bundle.speed");
    assert!(
        speed.detail.contains("20 ft"),
        "Dwarf speed trait bundle record must name the 20 ft base land speed: {}",
        speed.detail
    );
    assert!(
        speed.detail.contains("armor") || speed.detail.contains("encumbrance"),
        "Dwarf speed trait bundle record must name the never-modified-by-armor/encumbrance trait: {}",
        speed.detail
    );
    assert_eq!(
        speed.value, 20,
        "Dwarf speed trait bundle record value must carry the grounded 20 ft recognition value"
    );
}

// ----- senses: grounded Darkvision 60 ft, distinct from Human's no-special-senses -----

#[test]
fn dwarf_senses_trait_bundle_record_names_darkvision_60_ft() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let senses = explanation(&computation, "race.dwarf.trait_bundle.senses");
    assert!(
        senses.detail.contains("Darkvision") && senses.detail.contains("60 ft"),
        "Dwarf senses trait bundle record must name Darkvision 60 ft: {}",
        senses.detail
    );
    assert_eq!(
        senses.value, 60,
        "Dwarf senses trait bundle record value must carry the grounded 60 ft recognition value"
    );
}

// ----- The bounded note names the remaining unproven Dwarf families honestly -----

#[test]
fn dwarf_bounded_semantics_note_names_remaining_unproven_families() {
    let input = load(DWARF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.dwarf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.dwarf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    for token in [
        "Stonecunning",
        "Defensive Training",
        "Hardy",
        "Stability",
        "Hatred",
        "weapon familiarity",
    ] {
        assert!(
            bounded.message.contains(token),
            "race.dwarf.bounded_semantics must name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
    // PF1 core Dwarves gain no racial bonus feat (unlike Human); the note must say so
    // explicitly rather than silently reusing the generic Human-shaped template.
    assert!(
        bounded.message.contains("no racial bonus feat"),
        "race.dwarf.bounded_semantics must explicitly note Dwarves gain no racial bonus feat: {}",
        bounded.message
    );
}

// ----- The Dwarf trait bundle records do NOT leak onto Human or other races -----

#[test]
fn human_input_does_not_surface_dwarf_trait_bundle_records() {
    let input = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Human input must not surface Dwarf trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.dwarf.bounded_semantics"),
        "Human input must not surface the Dwarf bounded-semantics note"
    );
}

#[test]
fn other_non_human_race_still_gets_the_generic_unverified_diagnostic() {
    // Every named SD-13 roster race now has its own dedicated seam, so this
    // exercises the generic-diagnostic fallback with a race identity outside
    // the seven-race roster entirely.
    let other = DWARF_FIXTURE.replace("race_id=race:dwarf", "race_id=race:tiefling");
    let input = load(&other);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Tiefling input must not surface Dwarf trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    let unverified = diagnostic(&computation, "race.semantics.unverified");
    assert!(!unverified.claim_blocking);
}

// ----- Control plane: the matrix reclassifies the dwarf row to Partial/Computed -----

#[test]
fn matrix_dwarf_row_is_partial_computed_and_names_four_recognized_families() {
    let matrix = seeded_current_truth();
    let dwarf = matrix
        .row("race.dwarf.bounded_semantics")
        .expect("dwarf row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Race Trait
    // Catalog browser UI-surfacing work (2026-07-16); the four families this
    // SD13-E2 slice named remain the same, now fully surfaced live.
    assert_eq!(dwarf.support_state, SupportState::Supported);
    assert_eq!(dwarf.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        dwarf.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        dwarf
            .grounding_ref
            .contains("sd13_dwarf_bounded_race_semantics"),
        "dwarf row must cite the SD13-E2 dwarf proof surface: {}",
        dwarf.grounding_ref
    );
    let note = dwarf.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "dwarf partial row must carry a note");
    for token in [
        "Stonecunning",
        "Defensive Training",
        "Hardy",
        "Stability",
        "Hatred",
        "weapon familiarity",
    ] {
        assert!(
            note.contains(token),
            "dwarf partial note must name the still-unproven '{token}' family: {note}"
        );
    }
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_current_truth();

    let human = matrix
        .row("race.human.pilot_semantics")
        .expect("human row must exist");
    // Later promoted to Supported/ProductVisible alongside every other race
    // row by SD-19's Race Trait Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(human.support_state, SupportState::Supported);

    // Elf, Gnome, Half-Elf, Half-Orc, and Halfling were later promoted to
    // Partial/Computed by their own SD13-E2 recognition slices; this
    // Dwarf-slice snapshot no longer has any untouched sibling race to assert.

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
        "the dwarf slice must not promote any row to Supported or Lossy"
    );
}
