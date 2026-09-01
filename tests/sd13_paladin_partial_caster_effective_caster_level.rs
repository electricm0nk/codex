//! SD13-E5 Paladin partial-caster effective-caster-level gate proof.
//!
//! This sits on top of the accepted SD13-E3/E4/E5 Paladin level-1 chassis
//! decomposition (`explain_paladin_level1_chassis_and_spell_burden_separation`
//! in `src/rules_core/pilot_compute.rs`), which already grounds Smite Evil for
//! real and grounds lay on hands / divine grace / mercy as correct PF1 CRB
//! level-gate absences, while keeping one distinct claim-blocking diagnostic
//! for the later partial-caster spell burden
//! (`class_spell.paladin.partial_caster.unsupported`).
//!
//! This slice grounds the partial-caster IDENTITY itself as a flat, honest
//! numeric record, distinct from the still-ungrounded spell burden: PF1 Core
//! Rulebook Paladin effective caster level = max(paladin level - 3, 0)
//! (spells begin at paladin level 4). At the bounded level-1 baseline this
//! correctly grounds to 0 — the same "correct absence" idiom already used for
//! the lay on hands / divine grace / mercy level gates.
//!
//! It deliberately grounds ONLY that one arithmetic gate. It does not
//! fabricate a spell list, spells known, spells prepared, spells per day,
//! bonus spell slots from a high ability score, or spell save DCs — the
//! `class_spell.paladin.partial_caster.unsupported` diagnostic remains
//! claim-blocking for all of that, exactly as before this slice.

use codex::rules_core::character_input::{AcquisitionMode, SpellSelection};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

const PALADIN_EFFECTIVE_CASTER_LEVEL_ID: &str =
    "class_chassis.paladin.partial_caster.effective_caster_level";

const PALADIN_PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

#[test]
fn paladin_level1_grounds_effective_caster_level_as_correct_zero_absence() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook: paladin effective caster level = max(paladin level -
    // 3, 0); spells begin at paladin level 4. At level 1 this is correctly 0.
    let gate = explanation(&computation, PALADIN_EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        gate.value, 0,
        "paladin effective caster level at level 1 must ground to 0: {gate:?}"
    );
    assert!(
        gate.detail.contains("level - 3"),
        "effective-caster-level gate must state the formula (paladin level - 3): {}",
        gate.detail
    );
    assert!(
        gate.detail.contains("level 4"),
        "effective-caster-level gate must name that spells begin at paladin level 4: {}",
        gate.detail
    );
}

#[test]
fn paladin_effective_caster_level_grounding_does_not_retire_the_spell_burden_blocker() {
    // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25)
    // PALADIN_PARTIAL_CASTER_BLOCKER_ID is no longer unconditional: this
    // fixture's bare (zero prepared spells) posture is genuinely valid, so
    // the blocker would not fire at all. This test is specifically about
    // the blocker remaining a real, non-fabricated diagnostic, so a
    // genuinely invalid preparation (an off-list spell) is added to make it
    // fire for real -- proving the spell burden hasn't been silently
    // retired, just made conditional on a genuine violation.
    let mut input = load(PALADIN_FIXTURE);
    input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Magic Missile".to_owned(),
        source_class_id: "class:paladin".to_owned(),
        acquisition_mode: AcquisitionMode::Prepared,
    });
    let computation = compute_pilot_base_chassis(&input);

    // Grounding the caster-level gate arithmetic must not silently retire the
    // still-ungrounded spell burden: no spellbook, no spells known/prepared,
    // no spells-per-day, no bonus spell slots, no spell save DCs are computed
    // by this slice.
    let blocker = claim_blocking(&computation, PALADIN_PARTIAL_CASTER_BLOCKER_ID);
    assert!(
        !blocker.message.is_empty(),
        "the partial-caster spell burden must remain a non-empty claim-blocking diagnostic"
    );

    // No fabricated spell math leaks in alongside the caster-level gate.
    for forbidden_id in [
        "class_chassis.paladin.spells_known",
        "class_chassis.paladin.spells_per_day",
        "class_chassis.paladin.spell_save_dc",
        "class_chassis.paladin.bonus_spell_slots",
    ] {
        assert!(
            !has_explanation(&computation, forbidden_id),
            "grounding the caster-level gate must not fabricate '{forbidden_id}'"
        );
    }
}

#[test]
fn paladin_effective_caster_level_gate_does_not_leak_to_ranger_or_fighter() {
    let ranger_input = load(RANGER_FIXTURE);
    let ranger_computation = compute_pilot_base_chassis(&ranger_input);
    assert!(
        !has_explanation(&ranger_computation, PALADIN_EFFECTIVE_CASTER_LEVEL_ID),
        "ranger must not gain the Paladin-only effective-caster-level gate"
    );

    let fighter_input = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter_input);
    assert!(
        !has_explanation(&fighter_computation, PALADIN_EFFECTIVE_CASTER_LEVEL_ID),
        "fighter must not gain the Paladin-only effective-caster-level gate"
    );
}

#[test]
fn matrix_paladin_row_note_still_names_the_partial_caster_facts_and_stays_partial() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must exist");

    // This slice grounds one more flat arithmetic gate; the row is now
    // Supported via the Class Progression Catalog browser's UI-surfacing.
    assert_eq!(paladin.support_state, SupportState::Supported);

    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level - 3") && note.contains("level 4"),
        "paladin note must still carry the corrected partial-caster facts: {note}"
    );
}
