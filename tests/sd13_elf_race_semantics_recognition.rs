//! SD13-E2 Elf bounded race-semantics recognition proof.
//!
//! Proves the second race-semantics recognition slice for a non-Human core race
//! (mirroring the SD13-E2 Dwarf recognition pattern): the live rules-core surface
//! ingests a deterministic Elf input and surfaces four grounded PF1 Core Rulebook
//! Elf racial trait records as direct runtime evidence — ability modifiers (+2
//! Dexterity / -2 Constitution), size (Medium), speed (30 ft), and senses
//! (low-light vision) — rather than treating the Elf identity as an undocumented
//! packet placeholder gated behind the generic `race.semantics.unverified`
//! diagnostic.
//!
//! It is intentionally not an Elf racial trait engine. It grounds no numeric
//! contribution to attack rolls, AC, skill checks, ability checks, base speed, or
//! any other chassis output. It grounds no racial bonus feat (PF1 core Elves gain
//! none), no Elven Immunities (sleep immunity, enchantment save bonus), no Keen
//! Senses Perception bonus, no weapon familiarity grant, and no bonus language
//! grant — those remain named as still unproven. It also preserves the Human and
//! Dwarf race seams, the generic `race.semantics.unverified` diagnostic for every
//! other non-Human race, and the accepted class-chassis truth.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
mod common;
use common::{load, explanation, has_explanation};

const ELF_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_elf_fighter_level1_sd13_deterministic_input.txt");
const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);
const DWARF_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_dwarf_fighter_level1_sd13_deterministic_input.txt");

const BUNDLE_IDS: &[&str] = &[
    "race.elf.trait_bundle.ability_modifiers",
    "race.elf.trait_bundle.size",
    "race.elf.trait_bundle.speed",
    "race.elf.trait_bundle.senses",
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

// ----- All four Elf trait bundle records exist on an Elf input -----

#[test]
fn elf_input_surfaces_all_four_trait_bundle_records() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            has_explanation(&computation, id),
            "Elf input must surface trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
}

// ----- ability modifiers: grounded +2 Dex / +2 Int / -2 Con, no arithmetic performed here -----

#[test]
fn elf_ability_modifiers_record_names_dex_bonus_and_con_penalty() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.elf.trait_bundle.ability_modifiers");
    assert!(
        ability.detail.contains("+2") && ability.detail.contains("Dexterity"),
        "Elf ability modifiers record must name the +2 Dexterity adjustment: {}",
        ability.detail
    );
    assert!(
        ability.detail.contains("-2") && ability.detail.contains("Constitution"),
        "Elf ability modifiers record must name the -2 Constitution adjustment: {}",
        ability.detail
    );
    assert_eq!(
        ability.value, 0,
        "Elf ability modifiers record must carry no fabricated mechanical value (+0)"
    );
    // The chosen Dexterity/Constitution scores still compute their own modifiers
    // independently (DEX 16 -> +3, CON 10 -> +0).
    assert_eq!(computation.ability_modifiers.dexterity, 3);
    assert_eq!(computation.ability_modifiers.constitution, 0);
}

// ----- v0.6 alpha swarm fix (9ec0e036): the record now also names +2 Intelligence, -----
// ----- the real CRB-standard Elf default that was previously miscategorized as an  -----
// ----- out-of-scope alternate variant -- independently re-verified against the     -----
// ----- real PCGen corpus, not transcribed from the fix's own commit message        -----

#[test]
fn elf_ability_modifiers_record_now_names_the_intelligence_adjustment_too() {
    // Independently re-verified against the real PCGen corpus before writing this
    // assertion: core_essentials/races/elf/elf_abilities_race.lst:18's ability-score
    // row is explicitly typed "...Elf Racial Default..." (not an alternate/optional
    // variant) and carries BONUS:STAT|DEX,INT|2|TYPE=Racial alongside
    // BONUS:STAT|CON|-2|TYPE=Racial -- +2 Intelligence, +2 Dexterity, -2 Constitution
    // is the CRB-standard Elf default, matching the fix exactly.
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ability = explanation(&computation, "race.elf.trait_bundle.ability_modifiers");
    assert!(
        ability.detail.contains("+2") && ability.detail.contains("Intelligence"),
        "Elf ability modifiers record must name the +2 Intelligence adjustment: {}",
        ability.detail
    );
    assert!(
        !ability.detail.contains("Intelligence Elf variant is out of scope"),
        "the stale \"alternate variant, out of scope\" framing must be gone: {}",
        ability.detail
    );
    assert_eq!(
        ability.value, 0,
        "still a bounded recognition record: no arithmetic, no fabricated value"
    );
    // The chosen Intelligence score still computes its own modifier independently
    // (fixture Intelligence 14 -> floor(14/2)-5 = +2), same pattern as the existing
    // Dexterity/Constitution checks above.
    assert_eq!(computation.ability_modifiers.intelligence, 2);
}

// ----- size: grounded PF1 Elf Medium size, no fabricated value -----

#[test]
fn elf_size_trait_bundle_record_names_medium_category_and_carries_no_value() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let size = explanation(&computation, "race.elf.trait_bundle.size");
    assert!(
        size.detail.contains("Medium"),
        "Elf size trait bundle record must name the PF1 Elf Medium size category: {}",
        size.detail
    );
    assert_eq!(
        size.value, 0,
        "Elf size trait bundle record must carry no fabricated mechanical value (+0)"
    );
}

// ----- speed: grounded 30 ft base land speed -----

#[test]
fn elf_speed_trait_bundle_record_names_30_ft_base() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let speed = explanation(&computation, "race.elf.trait_bundle.speed");
    assert!(
        speed.detail.contains("30 ft"),
        "Elf speed trait bundle record must name the 30 ft base land speed: {}",
        speed.detail
    );
    assert_eq!(
        speed.value, 30,
        "Elf speed trait bundle record value must carry the grounded 30 ft recognition value"
    );
}

// ----- senses: grounded low-light vision, distinct from Human's no-special-senses -----

#[test]
fn elf_senses_trait_bundle_record_names_low_light_vision() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let senses = explanation(&computation, "race.elf.trait_bundle.senses");
    assert!(
        senses.detail.contains("low-light vision"),
        "Elf senses trait bundle record must name low-light vision: {}",
        senses.detail
    );
}

// ----- The bounded note names the remaining unproven Elf families honestly -----

#[test]
fn elf_bounded_semantics_note_names_remaining_unproven_families() {
    let input = load(ELF_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bounded = diagnostic(&computation, "race.elf.bounded_semantics");
    assert!(
        !bounded.claim_blocking,
        "race.elf.bounded_semantics must remain non-claim-blocking: {bounded:?}"
    );
    for token in [
        "Elven Immunities",
        "Keen Senses",
        "weapon familiarity",
        "bonus language",
    ] {
        assert!(
            bounded.message.contains(token),
            "race.elf.bounded_semantics must name the still-unproven '{token}' trait: {}",
            bounded.message
        );
    }
    assert!(
        bounded.message.contains("no racial bonus feat"),
        "race.elf.bounded_semantics must explicitly note Elves gain no racial bonus feat: {}",
        bounded.message
    );
}

// ----- The Elf trait bundle records do NOT leak onto Human, Dwarf, or other races -----

#[test]
fn human_and_dwarf_inputs_do_not_surface_elf_trait_bundle_records() {
    for fixture in [FIGHTER_FIXTURE, DWARF_FIXTURE] {
        let input = load(fixture);
        let computation = compute_pilot_base_chassis(&input);

        for id in BUNDLE_IDS {
            assert!(
                !has_explanation(&computation, id),
                "input must not surface Elf trait bundle record '{id}', got explanations {:?}",
                computation.explanations
            );
        }
        assert!(
            !computation
                .diagnostics
                .iter()
                .any(|d| d.id == "race.elf.bounded_semantics"),
            "input must not surface the Elf bounded-semantics note"
        );
    }
}

#[test]
fn other_non_human_race_still_gets_the_generic_unverified_diagnostic() {
    // Every named SD-13 roster race now has its own dedicated seam, so this
    // exercises the generic-diagnostic fallback with a race identity outside
    // the seven-race roster entirely.
    let other = ELF_FIXTURE.replace("race_id=race:elf", "race_id=race:tiefling");
    let input = load(&other);
    let computation = compute_pilot_base_chassis(&input);

    for id in BUNDLE_IDS {
        assert!(
            !has_explanation(&computation, id),
            "Tiefling input must not surface Elf trait bundle record '{id}', got explanations {:?}",
            computation.explanations
        );
    }
    let unverified = diagnostic(&computation, "race.semantics.unverified");
    assert!(!unverified.claim_blocking);
}

// ----- Control plane: the matrix reclassifies the elf row to Partial/Computed -----

