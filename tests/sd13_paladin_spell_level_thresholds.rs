//! SD13-E5 Paladin spell-level-access threshold grounding proof — a
//! priority-2 spell-posture burden slice.
//!
//! Grounds the Paladin partial-caster spell-level ACCESS ladder — the first
//! piece of the paladin spell burden the matrix names as the row's next
//! required uplift — mirroring the Cleric/Wizard
//! `<CLASS>_<N>TH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` threshold doctrine
//! exactly ("first non-'—' spells-per-day column", verified against the raw
//! table rows, never derived from memory). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Paladin class table) were read directly
//! before writing any code, and both show identical raw spells-per-day rows:
//!
//! - level 4: `0/—/—/—` — the FIRST non-"—" 1st-level column (a "0" entry:
//!   per the PF1 rule text quoted identically by both sources, "When Table:
//!   Paladin indicates that the paladin gets 0 spells per day of a given
//!   spell level, she gains only the bonus spells she would be entitled to
//!   based on her Charisma score for that spell level")
//! - levels 5-6: `1/—/—/—` — 1st-level access only
//! - level 7: `1/0/—/—` — the FIRST non-"—" 2nd-level column
//! - levels 8-9: `1/1/—/—` and `2/1/—/—` — 2nd-level access unchanged
//! - level 10: `2/1/0/—` — the FIRST non-"—" 3rd-level column; the
//!   4th-level column stays "—" through level 10 (4th-level paladin spells
//!   begin at 13, outside the tranche ceiling, so no 4th-level threshold
//!   const is grounded)
//!
//! The grounded record
//! (`class_chassis.paladin.partial_caster.spell_level_access`) carries the
//! highest paladin spell LEVEL with a non-"—" spells-per-day column: 0 at
//! levels 1-3 (a correct level-gate absence — the paladin has no spell
//! access at all below her level-4 gate), 1 at levels 4-6, 2 at levels 7-9,
//! and 3 at level 10. It grounds the access ladder ONLY: no spell slot
//! counts (the "0"-vs-"1"-vs-"2" per-day values are NOT computed), no
//! spells known/prepared posture, no bonus slots from a high Charisma, and
//! no spell save DCs — the `class_spell.paladin.partial_caster.unsupported`
//! blocker stays claim-blocking at every supported level, unchanged.
//!
//! It reuses the accepted per-level paladin fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const PALADIN_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level3_sd13_deterministic_input.txt");
const PALADIN_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level4_sd13_deterministic_input.txt");
const PALADIN_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level6_sd13_deterministic_input.txt");
const PALADIN_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level7_sd13_deterministic_input.txt");
const PALADIN_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level9_sd13_deterministic_input.txt");
const PALADIN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const SPELL_LEVEL_ACCESS_ID: &str = "class_chassis.paladin.partial_caster.spell_level_access";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

fn access_at(fixture: &str) -> i16 {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    explanation(&computation, SPELL_LEVEL_ACCESS_ID).value
}

// ----- Below the level-4 gate: access is a correct zero absence -----

#[test]
fn paladin_level3_spell_level_access_is_a_correct_zero_absence() {
    let input = load(PALADIN_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 0,
        "a level-3 paladin has no spell access at all (every spells-per-day column is \
         \"—\" below level 4): {}",
        access.detail
    );
}

// ----- Level 4: 1st-level access begins (a \"0\" bonus-spells-only entry) -----

#[test]
fn paladin_level4_gains_first_level_spell_access() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 1,
        "level 4 shows the first non-\"—\" 1st-level spells-per-day column (a \"0\" entry): {}",
        access.detail
    );
    assert!(
        access.detail.contains("bonus spells"),
        "the record must surface the PF1 \"0 spells per day\" rule nuance (access is via \
         Charisma bonus spells only at the gate level): {}",
        access.detail
    );
}

// ----- The ladder holds at every grounded step -----

#[test]
fn paladin_spell_level_access_ladder_matches_the_raw_table_rows() {
    assert_eq!(
        access_at(PALADIN_LEVEL6_FIXTURE),
        1,
        "level 6 (`1/—/—/—`) must stay at 1st-level access only"
    );
    assert_eq!(
        access_at(PALADIN_LEVEL7_FIXTURE),
        2,
        "level 7 (`1/0/—/—`) shows the first non-\"—\" 2nd-level column — access genuinely \
         rises to 2"
    );
    assert_eq!(
        access_at(PALADIN_LEVEL9_FIXTURE),
        2,
        "level 9 (`2/1/—/—`) must stay at 2nd-level access"
    );
    assert_eq!(
        access_at(PALADIN_LEVEL10_FIXTURE),
        3,
        "level 10 (`2/1/0/—`) shows the first non-\"—\" 3rd-level column — access genuinely \
         rises to 3"
    );
}

// ----- The access record fabricates no slot counts and lifts no blocker -----

#[test]
fn paladin_level10_spell_burden_stays_claim_blocked_despite_the_access_record() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert!(
        access.detail.contains("no spell slot counts")
            || access.detail.contains("no spells per day"),
        "the record must state that it grounds the access ladder only, never per-day slot \
         counts: {}",
        access.detail
    );

    // (v0.6 alpha swarm, risks item 8, 2026-07-25) `PARTIAL_CASTER_BLOCKER_ID`
    // is no longer unconditional: it's a real, conditional validation of
    // AcquisitionMode::Prepared selections. PALADIN_LEVEL10_FIXTURE predates
    // spells_selected (zero prepared), so the posture is genuinely valid and
    // the blocker correctly does not fire -- the real "no slot counts
    // fabricated" guarantee now comes from the daily-preparation record's
    // own count being honestly 0.
    match computation.diagnostics.iter().find(|d| d.id == PARTIAL_CASTER_BLOCKER_ID) {
        Some(blocker) => assert!(blocker.claim_blocking),
        None => {
            let daily_prep = explanation(&computation, "class_spell.paladin.daily_preparation");
            assert_eq!(
                daily_prep.value, 0,
                "no spells are fabricated at paladin level 10: {daily_prep:?}"
            );
        }
    }
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_spell_level_access() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SPELL_LEVEL_ACCESS_ID),
        "the Fighter chassis must not surface a paladin spell-level-access record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Paladin is not promoted -----

#[test]
fn multiclass_paladin_does_not_gain_spell_level_access() {
    let multiclass = PALADIN_LEVEL10_FIXTURE.replace(
        "class_level=class:paladin:10",
        "class_level=class:paladin:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == SPELL_LEVEL_ACCESS_ID),
        "multiclass Paladin must not gain a spell-level-access record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the access-ladder grounding -----

#[test]
fn matrix_paladin_row_names_the_spell_level_access_grounding() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_paladin_spell_level_thresholds"),
        "paladin row must cite the live spell-level-threshold proof surface: {}",
        paladin.grounding_ref
    );
    assert!(
        paladin
            .blocker_or_lossiness_note
            .contains("spell_level_access"),
        "paladin partial note must name the grounded access-ladder record: {}",
        paladin.blocker_or_lossiness_note
    );
}
