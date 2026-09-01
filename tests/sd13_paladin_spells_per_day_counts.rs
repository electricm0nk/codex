//! SD13-E5 Paladin base spells-per-day count grounding proof — the first
//! per-day SLOT-COUNT slice, building directly on the grounded spell-level
//! access ladder (`tests/sd13_paladin_spell_level_thresholds.rs`) and
//! mirroring the Cleric domain-slot-count precedent (literal table consts,
//! never a derived formula — the PF1 spells-per-day table is a lookup
//! table, not arithmetic). Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Paladin class table) were read directly before
//! writing any code, and both show identical raw spells-per-day rows:
//!
//! - level 4: `0/—/—/—`     - level 5: `1/—/—/—`    - level 6: `1/—/—/—`
//! - level 7: `1/0/—/—`     - level 8: `1/1/—/—`    - level 9: `2/1/—/—`
//! - level 10: `2/1/0/—`
//!
//! One record per ACCESSIBLE spell level (a level whose column is
//! non-"—"; inaccessible spell levels get no record at all — the access
//! ladder already carries that distinction):
//! `class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_<N>`
//! with the raw base count as its value — INCLUDING genuine "0" values,
//! which are real table entries meaning bonus-spells-only access ("When
//! Table: Paladin indicates that the paladin gets 0 spells per day of a
//! given spell level, she gains only the bonus spells she would be
//! entitled to based on her Charisma score for that spell level"), not
//! absences. The records ground the BASE counts only: bonus spells per
//! day from a high Charisma ("she receives bonus spells per day if she
//! has a high Charisma score") are never computed, no prepared posture or
//! spell-source lineage is grounded, and no spell save DCs are computed —
//! the `class_spell.paladin.partial_caster.unsupported` blocker stays
//! claim-blocking at every supported level (its message is updated to
//! stop deferring the now-grounded base counts, keeping it honest).
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
const PALADIN_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level5_sd13_deterministic_input.txt");
const PALADIN_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level7_sd13_deterministic_input.txt");
const PALADIN_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level8_sd13_deterministic_input.txt");
const PALADIN_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level9_sd13_deterministic_input.txt");
const PALADIN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.paladin.partial_caster.base_spells_per_day.";
const FIRST_ID: &str = "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_1";
const SECOND_ID: &str = "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_2";
const THIRD_ID: &str = "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_3";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

fn per_day_values(fixture: &str) -> Vec<(String, i16)> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(PER_DAY_PREFIX))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

// ----- Below the level-4 gate: no per-day records at all -----

#[test]
fn paladin_level3_has_no_per_day_records() {
    assert!(
        per_day_values(PALADIN_LEVEL3_FIXTURE).is_empty(),
        "a level-3 paladin has no accessible spell level, so no base per-day record may \
         exist"
    );
}

// ----- Level 4: a genuine "0" base count, not an absence -----

#[test]
fn paladin_level4_first_level_base_count_is_a_genuine_zero_entry() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first = explanation(&computation, FIRST_ID);
    assert_eq!(
        first.value, 0,
        "level 4 (`0/—/—/—`): the 1st-level base count is a genuine \"0\" table entry: {}",
        first.detail
    );
    assert!(
        first.detail.contains("bonus spells") && first.detail.contains("Charisma"),
        "the record must surface the PF1 \"0 spells per day\" bonus-spells-only rule: {}",
        first.detail
    );
    assert_eq!(
        per_day_values(PALADIN_LEVEL4_FIXTURE).len(),
        1,
        "only the 1st spell level is accessible at level 4 — no 2nd/3rd-level record"
    );
}

// ----- The literal table rows hold at every grounded step -----

#[test]
fn paladin_base_spells_per_day_match_the_raw_table_rows() {
    assert_eq!(
        per_day_values(PALADIN_LEVEL5_FIXTURE),
        vec![(FIRST_ID.to_owned(), 1)],
        "level 5 (`1/—/—/—`)"
    );
    assert_eq!(
        per_day_values(PALADIN_LEVEL7_FIXTURE),
        vec![(FIRST_ID.to_owned(), 1), (SECOND_ID.to_owned(), 0)],
        "level 7 (`1/0/—/—`): the 2nd-level column arrives as a genuine \"0\" entry"
    );
    assert_eq!(
        per_day_values(PALADIN_LEVEL8_FIXTURE),
        vec![(FIRST_ID.to_owned(), 1), (SECOND_ID.to_owned(), 1)],
        "level 8 (`1/1/—/—`)"
    );
    assert_eq!(
        per_day_values(PALADIN_LEVEL9_FIXTURE),
        vec![(FIRST_ID.to_owned(), 2), (SECOND_ID.to_owned(), 1)],
        "level 9 (`2/1/—/—`)"
    );
    assert_eq!(
        per_day_values(PALADIN_LEVEL10_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 2),
            (SECOND_ID.to_owned(), 1),
            (THIRD_ID.to_owned(), 0)
        ],
        "level 10 (`2/1/0/—`): the 3rd-level column arrives as a genuine \"0\" entry"
    );
}

// ----- Base counts only; the blocker stays and stops deferring them -----

#[test]
fn paladin_level10_blocker_stays_and_no_bonus_slots_are_fabricated() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let third = explanation(&computation, THIRD_ID);
    assert!(
        third.detail.contains("base count") || third.detail.contains("BASE count"),
        "the record must state it grounds the base table count only: {}",
        third.detail
    );

    // (v0.6 alpha swarm, risks item 8, 2026-07-25) `PARTIAL_CASTER_BLOCKER_ID`
    // is no longer unconditional, and its message shape changed entirely
    // (it's a real, conditional validation now, not a static deferred-work
    // list naming "lineage"/"prepared posture"/"TOTAL"). PALADIN_LEVEL10_FIXTURE
    // predates spells_selected (zero prepared), so the posture is genuinely
    // valid and the blocker correctly does not fire -- the real "base counts
    // grounded, nothing fabricated" guarantee now comes from the
    // daily-preparation record's own count being honestly 0.
    match computation.diagnostics.iter().find(|d| d.id == PARTIAL_CASTER_BLOCKER_ID) {
        Some(blocker) => assert!(blocker.claim_blocking, "the blocker must stay claim-blocking"),
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
fn fighter_does_not_gain_paladin_per_day_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(PER_DAY_PREFIX)),
        "the Fighter chassis must not surface any paladin base per-day record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Paladin is not promoted -----

#[test]
fn multiclass_paladin_does_not_gain_per_day_records() {
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
            .any(|e| e.id.starts_with(PER_DAY_PREFIX)),
        "multiclass Paladin must not gain any base per-day record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the per-day grounding -----

#[test]
fn matrix_paladin_row_names_the_per_day_grounding() {
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
            .contains("sd13_paladin_spells_per_day_counts"),
        "paladin row must cite the live per-day-count proof surface: {}",
        paladin.grounding_ref
    );
    assert!(
        paladin
            .blocker_or_lossiness_note
            .contains("base_spells_per_day"),
        "paladin partial note must name the grounded per-day records: {}",
        paladin.blocker_or_lossiness_note
    );
}
