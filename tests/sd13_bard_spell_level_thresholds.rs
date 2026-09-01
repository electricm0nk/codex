//! SD13-E5 Bard spell-level-access threshold grounding proof — a priority-2
//! spell-posture burden slice, mirroring the Paladin access-ladder slice
//! (`tests/sd13_paladin_spell_level_thresholds.rs`) and the Cleric/Wizard
//! `<CLASS>_<N>TH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` threshold doctrine
//! exactly ("first non-'—' spells-per-day column", verified against the raw
//! table rows, never derived from memory). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Bard class table) were read directly
//! before writing any code, and both show identical raw spells-per-day rows
//! for levels 1-10:
//!
//! - level 1: `1/—/—/—/—/—` — the FIRST non-"—" 1st-level column (unlike
//!   the Paladin table, the Bard table has NO "0" spells-per-day entries at
//!   levels 1-10; every non-"—" entry is a positive count)
//! - levels 2-3: `2/—/…` and `3/—/…` — 1st-level access only
//! - level 4: `3/1/—/…` — the FIRST non-"—" 2nd-level column
//! - levels 5-6: `4/2/—/…` and `4/3/—/…` — 2nd-level access unchanged
//! - level 7: `4/3/1/—/…` — the FIRST non-"—" 3rd-level column
//! - levels 8-9: `4/4/2/—/…` and `5/4/3/—/…` — 3rd-level access unchanged
//! - level 10: `5/4/3/1/—/—` — the FIRST non-"—" 4th-level column; the
//!   5th-level column stays "—" through level 10 (5th-level bard spells
//!   begin at 13, outside the tranche ceiling, so no 5th-level threshold
//!   const is grounded)
//!
//! The grounded record
//! (`class_chassis.bard.spontaneous.spell_level_access`) carries the
//! highest bard spell LEVEL (1st+) with a non-"—" spells-per-day column: 1
//! at levels 1-3 (a bard casts 1st-level spells from level 1 — there is no
//! zero step in this ladder, unlike the Paladin's), 2 at levels 4-6, 3 at
//! levels 7-9, and 4 at level 10. Cantrips (0th level, "spells known" only)
//! are not part of the spells-per-day ladder and are not counted. It
//! grounds the access ladder ONLY: no spells-per-day counts, no
//! spells-known posture, no bonus slots from a high Charisma, and no spell
//! save DCs — the
//! `class_spell.bard.spontaneous_known_and_per_day.unsupported` blocker
//! stays claim-blocking at every supported level, unchanged.
//!
//! It reuses the accepted per-level bard fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level1_sd13_deterministic_input.txt");
const BARD_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level3_sd13_deterministic_input.txt");
const BARD_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level4_sd13_deterministic_input.txt");
const BARD_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level6_sd13_deterministic_input.txt");
const BARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level7_sd13_deterministic_input.txt");
const BARD_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level9_sd13_deterministic_input.txt");
const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const SPELL_LEVEL_ACCESS_ID: &str = "class_chassis.bard.spontaneous.spell_level_access";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.bard.spontaneous_known_and_per_day.unsupported";

fn access_at(fixture: &str) -> i16 {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    explanation(&computation, SPELL_LEVEL_ACCESS_ID).value
}

// ----- Level 1: access begins at 1 — no zero step in the bard ladder -----

#[test]
fn bard_level1_has_first_level_spell_access_from_the_start() {
    let input = load(BARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 1,
        "a level-1 bard casts 1st-level spells (`1/—/—/—/—/—`) — the bard access ladder has \
         no zero step, unlike the Paladin's: {}",
        access.detail
    );
    assert!(
        access.detail.contains("antrip"),
        "the record must state that cantrips are outside the spells-per-day ladder: {}",
        access.detail
    );
}

// ----- The ladder holds at every grounded step -----

#[test]
fn bard_spell_level_access_ladder_matches_the_raw_table_rows() {
    assert_eq!(
        access_at(BARD_LEVEL3_FIXTURE),
        1,
        "level 3 (`3/—/…`) must stay at 1st-level access only"
    );
    assert_eq!(
        access_at(BARD_LEVEL4_FIXTURE),
        2,
        "level 4 (`3/1/—/…`) shows the first non-\"—\" 2nd-level column — access genuinely \
         rises to 2"
    );
    assert_eq!(
        access_at(BARD_LEVEL6_FIXTURE),
        2,
        "level 6 (`4/3/—/…`) must stay at 2nd-level access"
    );
    assert_eq!(
        access_at(BARD_LEVEL7_FIXTURE),
        3,
        "level 7 (`4/3/1/—/…`) shows the first non-\"—\" 3rd-level column — access genuinely \
         rises to 3"
    );
    assert_eq!(
        access_at(BARD_LEVEL9_FIXTURE),
        3,
        "level 9 (`5/4/3/—/…`) must stay at 3rd-level access"
    );
    assert_eq!(
        access_at(BARD_LEVEL10_FIXTURE),
        4,
        "level 10 (`5/4/3/1/—/—`) shows the first non-\"—\" 4th-level column — access \
         genuinely rises to 4"
    );
}

// ----- The access record fabricates no counts and lifts no blocker -----

#[test]
fn bard_level10_spontaneous_burden_stays_claim_blocked_despite_the_access_record() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert!(
        access.detail.contains("no spells-per-day counts")
            || access.detail.contains("no spells per day"),
        "the record must state that it grounds the access ladder only, never per-day \
         counts: {}",
        access.detail
    );

    // (v0.6 alpha swarm, risks item 8, known-spell closure) SPONTANEOUS_BLOCKER_ID
    // is no longer unconditional -- this fixture has zero known spells, a
    // genuinely valid posture, so the blocker correctly does not fire here.
    match computation.diagnostics.iter().find(|d| d.id == SPONTANEOUS_BLOCKER_ID) {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.bard.known_spells")
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
fn fighter_does_not_gain_bard_spell_level_access() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SPELL_LEVEL_ACCESS_ID),
        "the Fighter chassis must not surface a bard spell-level-access record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_does_not_gain_spell_level_access() {
    let multiclass = BARD_LEVEL10_FIXTURE.replace(
        "class_level=class:bard:10",
        "class_level=class:bard:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SPELL_LEVEL_ACCESS_ID),
        "multiclass Bard must not gain a spell-level-access record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the access-ladder grounding -----

#[test]
fn matrix_bard_row_names_the_spell_level_access_grounding() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        bard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        bard.grounding_ref
            .contains("sd13_bard_spell_level_thresholds"),
        "bard row must cite the live spell-level-threshold proof surface: {}",
        bard.grounding_ref
    );
    assert!(
        bard.blocker_or_lossiness_note
            .contains("spell_level_access"),
        "bard partial note must name the grounded access-ladder record: {}",
        bard.blocker_or_lossiness_note
    );
}
