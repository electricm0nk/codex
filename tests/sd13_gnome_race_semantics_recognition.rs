//! SD13-E2 Gnome bounded race-semantics recognition proof.
//!
//! Proves the third race-semantics recognition slice for a non-Human core race
//! (mirroring the SD13-E2 Dwarf and Elf recognition pattern): the live rules-core
//! surface ingests a deterministic Gnome input and surfaces four grounded PF1
//! Core Rulebook Gnome racial trait records as direct runtime evidence — ability
//! modifiers (+2 Constitution / -2 Strength), size (Small), speed (20 ft), and
//! senses (low-light vision) — rather than treating the Gnome identity as an
//! undocumented packet placeholder gated behind the generic
//! `race.semantics.unverified` diagnostic.
//!
//! It is intentionally not a Gnome racial trait engine. It grounds no numeric
//! contribution to attack rolls, AC, skill checks, ability checks, base speed, or
//! any other chassis output. It grounds no racial bonus feat (PF1 core Gnomes
//! gain none), no Defensive Training (dodge bonus to AC against giants), no
//! Illusion Resistance (save bonus against illusions), no Hatred (attack bonus
//! against reptilian humanoids and goblinoids), no Keen Senses Perception bonus,
//! no Gnome Magic spell-like abilities, and no weapon familiarity grant — those
//! remain named as still unproven. It also preserves the Human, Dwarf, and Elf
//! race seams, the generic `race.semantics.unverified` diagnostic for every other
//! non-Human race, and the accepted class-chassis truth.

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

const GNOME_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_gnome_fighter_level1_sd13_race_semantics_recognition_input.txt"
);
const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BUNDLE_IDS: &[&str] = &[
    "race.gnome.trait_bundle.ability_modifiers",
    "race.gnome.trait_bundle.size",
    "race.gnome.trait_bundle.speed",
    "race.gnome.trait_bundle.senses",
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

// ----- All four Gnome trait bundle records exist on a Gnome input -----

#[test]
fn gnome_input_surfaces_all_four_trait_bundle_records() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            has_explanation(&computation, id),
            "Gnome input must surface trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
}

// ----- ability modifiers: grounded +2 Con / -2 Str, no arithmetic performed here -----

#[test]
fn gnome_ability_modifiers_record_names_con_bonus_and_str_penalty() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.gnome.trait_bundle.ability_modifiers");
    assert!(
        ability.detail.contains("+2") && ability.detail.contains("Constitution"),
        "Gnome ability modifiers record must name the +2 Constitution adjustment: {}",
        ability.detail
    );
    assert!(
        ability.detail.contains("-2") && ability.detail.contains("Strength"),
        "Gnome ability modifiers record must name the -2 Strength adjustment: {}",
        ability.detail
    );
    assert_eq!(
        ability.value, 0,
        "Gnome ability modifiers record must carry no fabricated mechanical value (+0)"
    );
    // The chosen Constitution/Strength scores still compute their own modifiers
    // independently (CON 14 -> +2, STR 10 -> +0).
    assert_eq!(computation.ability_modifiers.constitution, 2);
    assert_eq!(computation.ability_modifiers.strength, 0);
}

// ----- v0.6 alpha swarm: the record was missing its real +2 Charisma
// adjustment entirely (same systemic gap as Elf's missing +2 Intelligence) --
// verified independently against the real PCGen corpus
// (core_essentials/races/gnome/gnome_abilities_race.lst:18's "Gnome Racial
// Default" row: BONUS:STAT|CON,CHA|2|TYPE=Racial, BONUS:STAT|STR|-2|TYPE=Racial)
// before writing this test, not just trusting the fix commit's own citation. -----

#[test]
fn gnome_ability_modifiers_record_now_names_the_charisma_adjustment_too() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.gnome.trait_bundle.ability_modifiers");
    assert!(
        ability.detail.contains("+2") && ability.detail.contains("Charisma"),
        "Gnome ability modifiers record must name the +2 Charisma adjustment: {}",
        ability.detail
    );
    assert!(
        ability.detail.contains("Constitution") && ability.detail.contains("Strength"),
        "Gnome ability modifiers record must still name Constitution and Strength: {}",
        ability.detail
    );
    assert_eq!(
        ability.value, 0,
        "Gnome ability modifiers record must still carry no fabricated mechanical value (+0)"
    );
    // Fixture Charisma is 10 -> modifier +0; independently verified, not assumed.
    assert_eq!(computation.ability_modifiers.charisma, 0);
}

// ----- v0.6 alpha swarm: the size record falsely implied real PF1 Small size
// has zero numeric effect; it only has zero effect in THIS codebase (no
// size-modifier term exists in the combat baseline for any race yet). The
// corrected text must name the real PF1 Small-size effect while still
// explaining why it isn't applied here, not merely erase the false claim. -----

#[test]
fn gnome_size_record_no_longer_falsely_claims_small_size_has_no_real_pf1_effect() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let size = explanation(&computation, "race.gnome.trait_bundle.size");
    assert!(
        !size.detail.contains("contributes no numeric effect"),
        "Gnome size record must not repeat the stale blanket 'no numeric effect' claim: {}",
        size.detail
    );
    assert!(
        size.detail.contains("+1 AC") && size.detail.contains("Stealth"),
        "Gnome size record must name the real PF1 Small-size effect (+1 AC, +4 Stealth, etc.): {}",
        size.detail
    );
    assert_eq!(
        size.value, 0,
        "Gnome size record must still carry no fabricated mechanical value (+0) -- no \
         size-modifier term is wired into the combat baseline yet"
    );
}

// ----- size: grounded PF1 Gnome Small size, no fabricated value -----

#[test]
fn gnome_size_trait_bundle_record_names_small_category_and_carries_no_value() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let size = explanation(&computation, "race.gnome.trait_bundle.size");
    assert!(
        size.detail.contains("Small"),
        "Gnome size trait bundle record must name the PF1 Gnome Small size category: {}",
        size.detail
    );
    assert_eq!(
        size.value, 0,
        "Gnome size trait bundle record must carry no fabricated mechanical value (+0)"
    );
}

// ----- speed: grounded 20 ft base land speed -----

#[test]
fn gnome_speed_trait_bundle_record_names_20_ft_base() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let speed = explanation(&computation, "race.gnome.trait_bundle.speed");
    assert!(
        speed.detail.contains("20 ft"),
        "Gnome speed trait bundle record must name the 20 ft base land speed: {}",
        speed.detail
    );
    assert_eq!(
        speed.value, 20,
        "Gnome speed trait bundle record value must carry the grounded 20 ft recognition value"
    );
}

// ----- senses: grounded low-light vision -----

#[test]
fn gnome_senses_trait_bundle_record_names_low_light_vision() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let senses = explanation(&computation, "race.gnome.trait_bundle.senses");
    assert!(
        senses.detail.contains("low-light vision"),
        "Gnome senses trait bundle record must name low-light vision: {}",
        senses.detail
    );
}

// ----- The bounded note names the remaining unproven Gnome families honestly -----

#[test]
fn gnome_bounded_semantics_note_names_remaining_unproven_families() {
    let input = load(GNOME_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.gnome.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.gnome.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    for token in [
        "Defensive Training",
        "Illusion Resistance",
        "Hatred",
        "Keen Senses",
        "Gnome Magic",
        "weapon familiarity",
    ] {
        assert!(
            bounded.message.contains(token),
            "race.gnome.bounded_semantics must name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("no racial bonus feat"),
        "race.gnome.bounded_semantics must explicitly note Gnomes gain no racial bonus feat: {}",
        bounded.message
    );
}

// ----- The Gnome trait bundle records do NOT leak onto Human or other races -----

#[test]
fn human_input_does_not_surface_gnome_trait_bundle_records() {
    let input = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Human input must not surface Gnome trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.gnome.bounded_semantics"),
        "Human input must not surface the Gnome bounded-semantics note"
    );
}

#[test]
fn other_non_human_race_still_gets_the_generic_unverified_diagnostic() {
    // Every named SD-13 roster race now has its own dedicated seam, so this
    // exercises the generic-diagnostic fallback with a race identity outside
    // the seven-race roster entirely.
    let other = GNOME_FIXTURE.replace("race_id=race:gnome", "race_id=race:tiefling");
    let input = load(&other);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Tiefling input must not surface Gnome trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    let unverified = diagnostic(&computation, "race.semantics.unverified");
    assert!(!unverified.claim_blocking);
}

// ----- Control plane: the matrix reclassifies the gnome row to Partial/Computed -----

#[test]
fn matrix_gnome_row_is_partial_computed_and_names_four_recognized_families() {
    let matrix = seeded_current_truth();
    let gnome = matrix
        .row("race.gnome.bounded_semantics")
        .expect("gnome row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Race Trait
    // Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(gnome.support_state, SupportState::Supported);
    assert_eq!(gnome.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        gnome.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        gnome
            .grounding_ref
            .contains("sd13_gnome_race_semantics_recognition"),
        "gnome row must cite the SD13-E2 gnome proof surface: {}",
        gnome.grounding_ref
    );
    let note = gnome.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "gnome partial row must carry a note");
    for token in [
        "Defensive Training",
        "Illusion Resistance",
        "Hatred",
        "Keen Senses",
        "Gnome Magic",
        "weapon familiarity",
    ] {
        assert!(
            note.contains(token),
            "gnome partial note must name the still-unproven '{token}' family: {note}"
        );
    }
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_current_truth();

    // Later promoted to Supported/ProductVisible alongside every other race
    // row by SD-19's Race Trait Catalog browser UI-surfacing work (2026-07-16).
    for id in ["race.dwarf.bounded_semantics", "race.elf.bounded_semantics"] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Supported,
            "row {id} must keep its later-accepted Supported posture after the gnome slice"
        );
    }

    // Half-Elf, Half-Orc, and Halfling were later promoted to Partial/Computed
    // by their own SD13-E2 recognition slices; this Gnome-slice snapshot no
    // longer has any untouched sibling race to assert.

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
        "the gnome slice must not promote any row to Supported or Lossy"
    );
}
