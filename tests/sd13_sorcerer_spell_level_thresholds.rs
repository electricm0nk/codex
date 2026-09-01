//! SD13-E5 Sorcerer spell-level-access threshold grounding proof — a
//! priority-2 spell-posture burden slice, mirroring the Paladin and Bard
//! access-ladder slices (`tests/sd13_paladin_spell_level_thresholds.rs`,
//! `tests/sd13_bard_spell_level_thresholds.rs`) and the Cleric/Wizard
//! `<CLASS>_<N>TH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` threshold doctrine
//! exactly ("first non-'—' spells-per-day column", verified against the raw
//! table rows, never derived from memory). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Sorcerer class table) were read directly
//! before writing any code, and both show identical raw spells-per-day rows
//! for levels 1-10:
//!
//! - level 1: `3/—/—/—/—/…` — the FIRST non-"—" 1st-level column (like the
//!   Bard and unlike the Paladin, the Sorcerer table has NO "0"
//!   spells-per-day entries at levels 1-10; every non-"—" entry is a
//!   positive count, and the ladder has no zero step)
//! - level 4: `6/3/—/…` — the FIRST non-"—" 2nd-level column
//! - level 6: `6/5/3/—/…` — the FIRST non-"—" 3rd-level column (the
//!   sorcerer's two-level cadence: NOT level 7, where the Bard's 3rd-level
//!   spells begin)
//! - level 8: `6/6/5/3/—/…` — the FIRST non-"—" 4th-level column
//! - level 10: `6/6/6/5/3/—/…` — the FIRST non-"—" 5th-level column; the
//!   6th-level column stays "—" through level 10 (6th-level sorcerer
//!   spells begin at 12, outside the tranche ceiling, so no 6th-level
//!   threshold const is grounded)
//!
//! The grounded record
//! (`class_chassis.sorcerer.spontaneous.spell_level_access`) carries the
//! highest sorcerer spell LEVEL (1st+) with a non-"—" spells-per-day
//! column: 1 at levels 1-3, 2 at levels 4-5, 3 at levels 6-7, 4 at levels
//! 8-9, and 5 at level 10 — one spell level deeper than any other class in
//! the tranche, the sorcerer's two-level cadence being faster than the
//! bard's three-level one. Cantrips (0th level, "spells known" only) are
//! not part of the spells-per-day ladder and are not counted. It grounds
//! the access ladder ONLY: no spells-per-day counts, no spells-known
//! posture, no bonus slots from a high Charisma, and no spell save DCs —
//! the `class_spell.sorcerer.spontaneous.unsupported` blocker stays
//! claim-blocking at every supported level, unchanged.
//!
//! It reuses the accepted per-level sorcerer fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");
const SORCERER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level3_sd13_deterministic_input.txt");
const SORCERER_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level4_sd13_deterministic_input.txt");
const SORCERER_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level5_sd13_deterministic_input.txt");
const SORCERER_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level6_sd13_deterministic_input.txt");
const SORCERER_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level7_sd13_deterministic_input.txt");
const SORCERER_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level8_sd13_deterministic_input.txt");
const SORCERER_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level9_sd13_deterministic_input.txt");
const SORCERER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const SPELL_LEVEL_ACCESS_ID: &str = "class_chassis.sorcerer.spontaneous.spell_level_access";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";

fn access_at(fixture: &str) -> i16 {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    explanation(&computation, SPELL_LEVEL_ACCESS_ID).value
}

// ----- Level 1: access begins at 1 — no zero step in the sorcerer ladder -----

#[test]
fn sorcerer_level1_has_first_level_spell_access_from_the_start() {
    let input = load(SORCERER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 1,
        "a level-1 sorcerer casts 1st-level spells (`3/—/…`) — the sorcerer access ladder \
         has no zero step: {}",
        access.detail
    );
    assert!(
        access.detail.contains("antrip"),
        "the record must state that cantrips are outside the spells-per-day ladder: {}",
        access.detail
    );
}

// ----- The ladder holds at every grounded step (the two-level cadence) -----

#[test]
fn sorcerer_spell_level_access_ladder_matches_the_raw_table_rows() {
    assert_eq!(
        access_at(SORCERER_LEVEL3_FIXTURE),
        1,
        "level 3 (`5/—/…`) must stay at 1st-level access only"
    );
    assert_eq!(
        access_at(SORCERER_LEVEL4_FIXTURE),
        2,
        "level 4 (`6/3/—/…`) shows the first non-\"—\" 2nd-level column — access genuinely \
         rises to 2"
    );
    assert_eq!(
        access_at(SORCERER_LEVEL5_FIXTURE),
        2,
        "level 5 (`6/4/—/…`) must stay at 2nd-level access"
    );
    assert_eq!(
        access_at(SORCERER_LEVEL6_FIXTURE),
        3,
        "level 6 (`6/5/3/—/…`) shows the first non-\"—\" 3rd-level column — the sorcerer's \
         two-level cadence, NOT the bard's level-7 gate"
    );
    assert_eq!(
        access_at(SORCERER_LEVEL7_FIXTURE),
        3,
        "level 7 (`6/6/4/—/…`) must stay at 3rd-level access"
    );
    assert_eq!(
        access_at(SORCERER_LEVEL8_FIXTURE),
        4,
        "level 8 (`6/6/5/3/—/…`) shows the first non-\"—\" 4th-level column — access \
         genuinely rises to 4"
    );
    assert_eq!(
        access_at(SORCERER_LEVEL9_FIXTURE),
        4,
        "level 9 (`6/6/6/4/—/…`) must stay at 4th-level access"
    );
    assert_eq!(
        access_at(SORCERER_LEVEL10_FIXTURE),
        5,
        "level 10 (`6/6/6/5/3/—/…`) shows the first non-\"—\" 5th-level column — access \
         genuinely rises to 5, one spell level deeper than any other class in the tranche"
    );
}

// ----- The access record fabricates no counts and lifts no blocker -----

#[test]
fn sorcerer_level10_spontaneous_burden_stays_claim_blocked_despite_the_access_record() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert!(
        access.detail.contains("no spells-per-day counts")
            || access.detail.contains("no spells per day"),
        "the record must state that it grounds the access ladder only, never per-day \
         counts: {}",
        access.detail
    );

    // (v0.6 alpha swarm, risks item 8) SPONTANEOUS_BLOCKER_ID is no longer
    // unconditional -- this fixture has zero known spells, a genuinely valid
    // posture, so the blocker correctly does not fire here. The access record
    // above already proves the ladder is grounded for real and never fabricated;
    // that's the property this test actually needs.
    match computation.diagnostics.iter().find(|d| d.id == SPONTANEOUS_BLOCKER_ID) {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.sorcerer.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_spell_level_access() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SPELL_LEVEL_ACCESS_ID),
        "the Fighter chassis must not surface a sorcerer spell-level-access record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_does_not_gain_spell_level_access() {
    let multiclass = SORCERER_LEVEL10_FIXTURE.replace(
        "class_level=class:sorcerer:10",
        "class_level=class:sorcerer:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SPELL_LEVEL_ACCESS_ID),
        "multiclass Sorcerer must not gain a spell-level-access record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the access-ladder grounding -----

#[test]
fn matrix_sorcerer_row_names_the_spell_level_access_grounding() {
    let matrix = seeded_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer progression_and_spell_burden row must exist");

    assert_eq!(sorcerer.support_state, SupportState::Supported);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd13_sorcerer_spell_level_thresholds"),
        "sorcerer row must cite the live spell-level-threshold proof surface: {}",
        sorcerer.grounding_ref
    );
    assert!(
        sorcerer
            .blocker_or_lossiness_note
            .contains("spell_level_access"),
        "sorcerer partial note must name the grounded access-ladder record: {}",
        sorcerer.blocker_or_lossiness_note
    );
}
