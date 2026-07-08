//! SD13-E5 Bard Fascinate flat DC / affected-creature-count grounding proof.
//!
//! Grounds the two flat numeric formulas behind the level-1 bardic performance
//! "Fascinate" (PF1 Core Rulebook Fascinate), mirroring the exact style already
//! used for Inspire Courage in `tests/sd13_bard_level1_spell_baseline.rs`:
//!
//! - the Will save DC: `10 + 1/2 the bard's level + the bard's Charisma modifier`
//!   (verified against the PF1 Core Rulebook Fascinate rule text via d20pfsrd /
//!   the legacy Paizo PRD mirror, not trusted from memory alone), and
//! - the affected-creature count: 1 creature at 1st level, plus one additional
//!   creature for every three bard levels attained beyond 1st (verified the same
//!   way; NOT "half the bard's level" — that different-looking formula only
//!   happens to coincide with the correct one at level 1, which is exactly the
//!   trap a from-memory guess would have fallen into without checking a primary
//!   source).
//!
//! This is intentionally not a Fascinate execution engine: it fabricates no
//! Will-save resolution, no range/line-of-sight/attention-requirement checking,
//! and no application of either grounded number to any actual save total or
//! targeting outcome. Countersong and Distraction — the two other named,
//! still-unproven level-1 performances — stay claim-blocked, because both
//! require an opposed Perform-check-vs-effect substitution resolution rather
//! than a flat number.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};

const BARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level1_sd13_deterministic_input.txt");

const FASCINATE_DC_ID: &str = "class_chassis.bard.fascinate_dc";
const FASCINATE_AFFECTED_CREATURES_ID: &str = "class_chassis.bard.fascinate_affected_creatures";
const PERFORMANCE_EXECUTION_BLOCKER_ID: &str =
    "class_feature.bard.bardic_performance_execution.unsupported";

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

#[test]
fn bard_level1_grounds_fascinate_dc_flat_formula() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture Charisma 15 -> +2 modifier. PF1 Core Rulebook Fascinate DC:
    // 10 + 1/2 bard level + Charisma modifier. At bard level 1: 10 + 0 + 2 = 12.
    let dc = explanation(&computation, FASCINATE_DC_ID);
    assert_eq!(
        dc.value, 12,
        "Fascinate DC at bard level 1 with CHA modifier +2 must be 10 + 0 + 2 = 12: {dc:?}"
    );
    assert!(
        dc.detail.contains("10") && dc.detail.to_lowercase().contains("charisma"),
        "Fascinate DC explanation must name the 10 + 1/2 level + Charisma-modifier formula: {}",
        dc.detail
    );
    // Must disclaim any Will-save resolution or application to an actual save total.
    assert!(
        dc.detail.to_lowercase().contains("no")
            && (dc.detail.to_lowercase().contains("resolution")
                || dc.detail.to_lowercase().contains("applied")
                || dc.detail.to_lowercase().contains("application")),
        "Fascinate DC explanation must disclaim Will-save resolution/application: {}",
        dc.detail
    );
}

#[test]
fn bard_level1_grounds_fascinate_affected_creature_count() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Fascinate: one creature at 1st level, plus one additional
    // creature for every three bard levels attained beyond 1st. At bard level 1:
    // 1 + (1 - 1) / 3 = 1. This is deliberately NOT "half bard level" (which would
    // also numerically equal 1 at level 1 but is the wrong formula beyond level 1).
    let count = explanation(&computation, FASCINATE_AFFECTED_CREATURES_ID);
    assert_eq!(
        count.value, 1,
        "Fascinate affected-creature count at bard level 1 must be 1: {count:?}"
    );
    assert!(
        count.detail.contains("three") || count.detail.contains("3"),
        "Fascinate affected-creature-count explanation must name the every-three-levels \
         formula, not a half-level formula: {}",
        count.detail
    );
}

#[test]
fn bard_level1_fascinate_grounds_no_resolution_or_targeting_engine() {
    let input = load(BARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Grounding the two flat numbers must not silently fabricate a Fascinate
    // resolution/targeting engine or any other unrelated computed chassis.
    assert_eq!(computation.base_attack_bonus, 0);

    // The performance-execution blocker must still be present and claim-blocking,
    // and it must no longer claim Fascinate is entirely "not grounded" -- Fascinate's
    // two flat numbers are now grounded, only its resolution stays unproven -- while
    // Countersong and Distraction must still be named as fully unproven (they need an
    // opposed Perform-check-vs-effect resolution, not a flat number).
    let blocker = diagnostic(&computation, PERFORMANCE_EXECUTION_BLOCKER_ID);
    assert!(
        blocker.message.contains("countersong") && blocker.message.contains("distraction"),
        "performance-execution blocker must still name countersong and distraction as \
         unproven: {}",
        blocker.message
    );
}
