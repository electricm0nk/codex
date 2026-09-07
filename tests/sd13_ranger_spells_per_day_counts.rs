//! SD13-E5 Ranger base spells-per-day count grounding proof — the second
//! per-day SLOT-COUNT slice, mirroring the Paladin per-day slice
//! (`tests/sd13_paladin_spells_per_day_counts.rs`) record-for-record on
//! top of the grounded ranger partial-caster identity pair
//! (`tests/sd13_ranger_spell_level_thresholds.rs`), per the Cleric
//! domain-slot-count literal-table doctrine (a lookup table, not
//! arithmetic — no formula is invented). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Ranger class table) were read directly
//! this session before writing any code, and both show identical raw
//! spells-per-day rows — numerically identical to the Paladin's, with the
//! ranger's Wisdom (not the paladin's Charisma) as the bonus-spell stat:
//!
//! - level 4: `0/—/—/—`     - level 5: `1/—/—/—`    - level 6: `1/—/—/—`
//! - level 7: `1/0/—/—`     - level 8: `1/1/—/—`    - level 9: `2/1/—/—`
//! - level 10: `2/1/0/—`
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.ranger.partial_caster.base_spells_per_day.spell_level_<N>`
//! with the raw base count as its value — including genuine "0" entries
//! ("When Table: Ranger indicates that the ranger gets 0 spells per day
//! of a given spell level, he gains only the bonus spells he would be
//! entitled to based on his Wisdom score for that spell level").
//! Inaccessible "—" spell levels get no record at all. Base counts only:
//! bonus spells from a high Wisdom are never computed, no prepared
//! posture or spell-source lineage is grounded, and no spell save DCs are
//! computed. NO new claim-blocking diagnostic is added (the ranger has no
//! per-level spell blocker — the burden stays named by the F6 level-1
//! hybrid blocker and the matrix note), and the computation stays
//! claim-blocked overall via the generic chassis diagnostics.
//!
//! The sibling `tests/sd13_ranger_spell_level_thresholds.rs` pinned
//! "exactly two partial_caster records"; that control is extended to
//! exclude the new base_spells_per_day family it could not have known
//! about, preserving its original claim.
//!
//! It reuses the accepted per-level ranger fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const RANGER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level3_sd13_deterministic_input.txt");
const RANGER_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level4_sd13_deterministic_input.txt");
const RANGER_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level5_sd13_deterministic_input.txt");
const RANGER_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level7_sd13_deterministic_input.txt");
const RANGER_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level8_sd13_deterministic_input.txt");
const RANGER_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level9_sd13_deterministic_input.txt");
const RANGER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";
const FIRST_ID: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.spell_level_1";
const SECOND_ID: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.spell_level_2";
const THIRD_ID: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.spell_level_3";

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
fn ranger_level3_has_no_per_day_records() {
    assert!(
        per_day_values(RANGER_LEVEL3_FIXTURE).is_empty(),
        "a level-3 ranger has no accessible spell level, so no base per-day record may exist"
    );
}

// ----- Level 4: a genuine "0" base count with the Wisdom nuance -----

#[test]
fn ranger_level4_first_level_base_count_is_a_genuine_zero_entry() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first = explanation(&computation, FIRST_ID);
    assert_eq!(
        first.value, 0,
        "level 4 (`0/—/—/—`): the 1st-level base count is a genuine \"0\" table entry: {}",
        first.detail
    );
    assert!(
        first.detail.contains("bonus spells") && first.detail.contains("Wisdom"),
        "the record must surface the PF1 \"0 spells per day\" rule with the ranger's Wisdom \
         (not the paladin's Charisma) as the bonus-spell stat: {}",
        first.detail
    );
    assert_eq!(
        per_day_values(RANGER_LEVEL4_FIXTURE).len(),
        1,
        "only the 1st spell level is accessible at level 4 — no 2nd/3rd-level record"
    );
}

// ----- The literal table rows hold at every grounded step -----

#[test]
fn ranger_base_spells_per_day_match_the_raw_table_rows() {
    assert_eq!(
        per_day_values(RANGER_LEVEL5_FIXTURE),
        vec![(FIRST_ID.to_owned(), 1)],
        "level 5 (`1/—/—/—`)"
    );
    assert_eq!(
        per_day_values(RANGER_LEVEL7_FIXTURE),
        vec![(FIRST_ID.to_owned(), 1), (SECOND_ID.to_owned(), 0)],
        "level 7 (`1/0/—/—`): the 2nd-level column arrives as a genuine \"0\" entry"
    );
    assert_eq!(
        per_day_values(RANGER_LEVEL8_FIXTURE),
        vec![(FIRST_ID.to_owned(), 1), (SECOND_ID.to_owned(), 1)],
        "level 8 (`1/1/—/—`)"
    );
    assert_eq!(
        per_day_values(RANGER_LEVEL9_FIXTURE),
        vec![(FIRST_ID.to_owned(), 2), (SECOND_ID.to_owned(), 1)],
        "level 9 (`2/1/—/—`)"
    );
    assert_eq!(
        per_day_values(RANGER_LEVEL10_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 2),
            (SECOND_ID.to_owned(), 1),
            (THIRD_ID.to_owned(), 0)
        ],
        "level 10 (`2/1/0/—`): the 3rd-level column arrives as a genuine \"0\" entry"
    );
}

// ----- Base counts only; still claim-blocked overall -----

#[test]
fn ranger_level10_stays_claim_blocked_and_grounds_base_counts_only() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let third = explanation(&computation, THIRD_ID);
    assert!(
        third.detail.contains("base count"),
        "the record must state it grounds the base table count only: {}",
        third.detail
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "the level-10 Ranger must stay claim-blocked overall (prepared posture, Wisdom \
         bonus slots, lineage, and DCs are still unproven): {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_per_day_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(PER_DAY_PREFIX)),
        "the Fighter chassis must not surface any ranger base per-day record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_does_not_gain_per_day_records() {
    let multiclass = RANGER_LEVEL10_FIXTURE.replace(
        "class_level=class:ranger:10",
        "class_level=class:ranger:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(PER_DAY_PREFIX)),
        "multiclass Ranger must not gain any base per-day record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the per-day grounding -----

#[test]
fn matrix_ranger_row_names_the_per_day_grounding() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger
            .grounding_ref
            .contains("sd13_ranger_spells_per_day_counts"),
        "ranger row must cite the live per-day-count proof surface: {}",
        ranger.grounding_ref
    );
    assert!(
        ranger
            .blocker_or_lossiness_note
            .contains("base_spells_per_day"),
        "ranger partial note must name the grounded per-day records: {}",
        ranger.blocker_or_lossiness_note
    );
}
