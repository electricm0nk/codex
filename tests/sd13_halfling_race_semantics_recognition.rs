//! SD13-E2 Halfling bounded race-semantics recognition proof.
//!
//! Proves the sixth and final race-semantics recognition slice for a non-Human
//! core race (mirroring the fixed-ability-pair Dwarf/Elf/Gnome pattern): the
//! live rules-core surface ingests a deterministic Halfling input and surfaces
//! four grounded PF1 Core Rulebook Halfling racial trait records as direct
//! runtime evidence — ability modifiers (+2 Dexterity / -2 Strength), size
//! (Small), speed (20 ft), and senses (no special senses, the human-sense
//! baseline) — rather than treating the Halfling identity as an undocumented
//! packet placeholder gated behind the generic `race.semantics.unverified`
//! diagnostic.
//!
//! It is intentionally not a Halfling racial trait engine. It grounds no
//! numeric contribution to attack rolls, AC, skill checks, ability checks, base
//! speed, or any other chassis output. It grounds no racial bonus feat (PF1
//! core Halflings gain none), no Fearless (save bonus against fear), no
//! Halfling Luck (luck bonus on all saves), no Keen Senses Perception bonus, no
//! Sure-Footed (Acrobatics/Climb bonus), and no weapon familiarity grant —
//! those remain named as still unproven. It also preserves the Human, Dwarf,
//! Elf, Gnome, Half-Elf, and Half-Orc race seams, and the accepted
//! class-chassis truth. With this slice landed, every core race row carries
//! runtime evidence.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const HALFLING_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_halfling_fighter_level1_sd13_deterministic_input.txt");
const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BUNDLE_IDS: &[&str] = &[
    "race.halfling.trait_bundle.ability_modifiers",
    "race.halfling.trait_bundle.size",
    "race.halfling.trait_bundle.speed",
    "race.halfling.trait_bundle.senses",
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

// ----- All four Halfling trait bundle records exist on a Halfling input -----

#[test]
fn halfling_input_surfaces_all_four_trait_bundle_records() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            has_explanation(&computation, id),
            "Halfling input must surface trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
}

// ----- ability modifiers: grounded +2 Dex / -2 Str, no arithmetic performed here -----

#[test]
fn halfling_ability_modifiers_record_names_dex_bonus_and_str_penalty() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.halfling.trait_bundle.ability_modifiers");
    assert!(
        ability.detail.contains("+2") && ability.detail.contains("Dexterity"),
        "Halfling ability modifiers record must name the +2 Dexterity adjustment: {}",
        ability.detail
    );
    assert!(
        ability.detail.contains("-2") && ability.detail.contains("Strength"),
        "Halfling ability modifiers record must name the -2 Strength adjustment: {}",
        ability.detail
    );
    assert_eq!(
        ability.value, 0,
        "Halfling ability modifiers record must carry no fabricated mechanical value (+0)"
    );
    // The chosen Dexterity/Strength scores still compute their own modifiers
    // independently (DEX 16 -> +3, STR 10 -> +0).
    assert_eq!(computation.ability_modifiers.dexterity, 3);
    assert_eq!(computation.ability_modifiers.strength, 0);
}

// ----- size: grounded PF1 Halfling Small size, no fabricated value -----

#[test]
fn halfling_size_trait_bundle_record_names_small_category_and_carries_no_value() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let size = explanation(&computation, "race.halfling.trait_bundle.size");
    assert!(
        size.detail.contains("Small"),
        "Halfling size trait bundle record must name the PF1 Halfling Small size category: {}",
        size.detail
    );
    assert_eq!(
        size.value, 0,
        "Halfling size trait bundle record must carry no fabricated mechanical value (+0)"
    );
}

// ----- speed: grounded 20 ft base land speed -----

#[test]
fn halfling_speed_trait_bundle_record_names_20_ft_base() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let speed = explanation(&computation, "race.halfling.trait_bundle.speed");
    assert!(
        speed.detail.contains("20 ft"),
        "Halfling speed trait bundle record must name the 20 ft base land speed: {}",
        speed.detail
    );
    assert_eq!(
        speed.value, 20,
        "Halfling speed trait bundle record value must carry the grounded 20 ft recognition value"
    );
}

// ----- senses: bounded no-effect classification, distinct from Dwarf/Elf/Gnome -----

#[test]
fn halfling_senses_trait_bundle_record_classifies_as_bounded_no_effect() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let senses = explanation(&computation, "race.halfling.trait_bundle.senses");
    assert!(
        senses.detail.contains("no special senses"),
        "Halfling senses trait bundle record must classify PF1 Halfling as having no special \
         senses: {}",
        senses.detail
    );
    assert_eq!(
        senses.value, 0,
        "Halfling senses trait bundle record must carry the bounded no-effect value (+0)"
    );
}

// ----- The bounded note names the remaining unproven Halfling families honestly -----

#[test]
fn halfling_bounded_semantics_note_names_remaining_unproven_families() {
    let input = load(HALFLING_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.halfling.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.halfling.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    for token in ["Fearless", "Halfling Luck", "Keen Senses", "Sure-Footed", "weapon familiarity"]
    {
        assert!(
            bounded.message.contains(token),
            "race.halfling.bounded_semantics must name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("no racial bonus feat"),
        "race.halfling.bounded_semantics must explicitly note Halflings gain no racial bonus \
         feat: {}",
        bounded.message
    );
}

// ----- The Halfling trait bundle records do NOT leak onto Human or other races -----

#[test]
fn human_input_does_not_surface_halfling_trait_bundle_records() {
    let input = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Human input must not surface Halfling trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.halfling.bounded_semantics"),
        "Human input must not surface the Halfling bounded-semantics note"
    );
}

// ----- Control plane: the matrix reclassifies the halfling row to Partial/Computed -----

#[test]
fn matrix_halfling_row_is_partial_computed_and_names_four_recognized_families() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let halfling = matrix
        .row("race.halfling.bounded_semantics")
        .expect("halfling row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Race Trait
    // Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(halfling.support_state, SupportState::Supported);
    assert_eq!(halfling.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        halfling.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        halfling
            .grounding_ref
            .contains("sd13_halfling_race_semantics_recognition"),
        "halfling row must cite the SD13-E2 halfling proof surface: {}",
        halfling.grounding_ref
    );
    let note = halfling.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "halfling partial row must carry a note");
    for token in ["Fearless", "Halfling Luck", "Keen Senses", "Sure-Footed", "weapon familiarity"]
    {
        assert!(
            note.contains(token),
            "halfling partial note must name the still-unproven '{token}' family: {note}"
        );
    }
}

#[test]
fn matrix_all_six_non_human_race_rows_now_carry_runtime_evidence() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    // With this slice, every core race row (Human + all six non-Human races)
    // carries runtime evidence; none is a pure roster-scope placeholder any
    // longer, though several stay Partial rather than Supported.
    for id in [
        "race.human.pilot_semantics",
        "race.dwarf.bounded_semantics",
        "race.elf.bounded_semantics",
        "race.gnome.bounded_semantics",
        "race.half_elf.bounded_semantics",
        "race.half_orc.bounded_semantics",
        "race.halfling.bounded_semantics",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        // All seven race rows were later promoted to Supported/ProductVisible
        // by SD-19's Race Trait Catalog browser UI-surfacing work (2026-07-16).
        assert_eq!(
            row.support_state,
            SupportState::Supported,
            "row {id} must be Supported after SD-19's Race Trait Catalog browser work"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    }

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
                && r.row_id != "equipment.equipmods.equipment_reachability")
                || r.support_state == SupportState::Lossy),
        "the halfling slice must not promote any row to Supported or Lossy"
    );
}
