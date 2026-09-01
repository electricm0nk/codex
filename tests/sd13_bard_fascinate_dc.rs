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

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
mod common;
use common::{load, explanation};

const BARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level1_sd13_deterministic_input.txt");

const FASCINATE_DC_ID: &str = "class_chassis.bard.fascinate_dc";
const FASCINATE_AFFECTED_CREATURES_ID: &str = "class_chassis.bard.fascinate_affected_creatures";
const OTHER_PERFORMANCES_NOT_MODELED_ID: &str =
    "class_feature.bard.bardic_performance_execution.other_performances_not_modeled";

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

    // Fixture Charisma 15 + 2 Human racial (CG-03 fix) -> +3 modifier. PF1 Core
    // Rulebook Fascinate DC: 10 + 1/2 bard level + Charisma modifier. At bard level 1:
    // 10 + 0 + 3 = 13.
    let dc = explanation(&computation, FASCINATE_DC_ID);
    assert_eq!(
        dc.value, 13,
        "Fascinate DC at bard level 1 with CHA modifier +3 must be 10 + 0 + 3 = 13: {dc:?}"
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

    // (v0.6 alpha swarm, risks item 8) The old unconditional performance-execution
    // blocker is retired; the permanently unconditional other-performances-not-modeled
    // note (non-blocking) is what still names Countersong and Distraction as fully
    // unproven (they need an opposed Perform-check-vs-effect resolution, not a flat
    // number) -- Fascinate's two flat numbers are grounded, only its resolution stays
    // unproven, so it is not named here.
    let blocker = diagnostic(&computation, OTHER_PERFORMANCES_NOT_MODELED_ID);
    assert!(
        !blocker.claim_blocking,
        "the other-performances-not-modeled note must not block a valid Inspire Courage posture"
    );
    assert!(
        blocker.message.contains("Countersong") && blocker.message.contains("Distraction"),
        "the other-performances-not-modeled note must still name Countersong and Distraction as \
         unproven: {}",
        blocker.message
    );
}
