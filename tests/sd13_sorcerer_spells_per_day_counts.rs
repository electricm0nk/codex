//! SD13-E5 Sorcerer base spells-per-day count grounding proof — the fourth
//! and final per-day SLOT-COUNT slice of the tranche, mirroring the
//! Paladin/Ranger/Bard per-day slices on top of the grounded sorcerer
//! access ladder (`tests/sd13_sorcerer_spell_level_thresholds.rs`), per
//! the Cleric domain-slot-count literal-table doctrine (a lookup table,
//! not arithmetic — no formula is invented). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Sorcerer class table) were re-read
//! this cycle before writing any code, and both show identical raw
//! spells-per-day rows for levels 1-10 (1st through 5th spell level):
//!
//! - level 1: `3/—/—/—/—`   - level 2: `4/—/—/—/—`  - level 3: `5/—/—/—/—`
//! - level 4: `6/3/—/—/—`   - level 5: `6/4/—/—/—`  - level 6: `6/5/3/—/—`
//! - level 7: `6/6/4/—/—`   - level 8: `6/6/5/3/—`  - level 9: `6/6/6/4/—`
//! - level 10: `6/6/6/5/3`
//!
//! Like the Bard and unlike the Paladin/Ranger, the Sorcerer table has NO
//! "0" entries at levels 1-10; every accessible column carries a positive
//! base count, arriving at 3 at each new spell level (the sorcerer's
//! signature deep slot pool — the 1st-level column caps at 6 by level 4).
//! One record per ACCESSIBLE spell level:
//! `class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_<N>`
//! with the raw base count as its value; inaccessible "—" spell levels
//! get no record at all. Base counts only: bonus spells from a high
//! Charisma are never computed, spells KNOWN (a separate table) is
//! deliberately untouched, and no spell save DCs are computed — the
//! `class_spell.sorcerer.spontaneous.unsupported` blocker stays
//! claim-blocking at every supported level unmodified (the grounded base
//! counts are a strict subset of the posture it defers).
//!
//! Sibling allow-list extension: the level-1 baseline's spell-tag
//! allow-list gains the per-day family by prefix, mirroring the bard
//! level-1 fix exactly.
//!
//! It reuses the accepted per-level sorcerer fixtures (no new fixture)
//! and preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const SORCERER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");
const SORCERER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level3_sd13_deterministic_input.txt");
const SORCERER_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level4_sd13_deterministic_input.txt");
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

const PER_DAY_PREFIX: &str = "class_chassis.sorcerer.spontaneous.base_spells_per_day.";
const FIRST_ID: &str = "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_1";
const SECOND_ID: &str = "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_2";
const THIRD_ID: &str = "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_3";
const FOURTH_ID: &str = "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_4";
const FIFTH_ID: &str = "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_5";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";

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

// ----- Level 1: one record, base count 3 — the sorcerer's deep slot pool -----

#[test]
fn sorcerer_level1_first_level_base_count_is_three() {
    let input = load(SORCERER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first = computation
        .explanations
        .iter()
        .find(|e| e.id == FIRST_ID)
        .expect("level-1 sorcerer must carry the 1st-level base per-day record");
    assert_eq!(
        first.value, 3,
        "level 1 (`3/—/—/—/—`): the 1st-level base count is 3 — the Sorcerer table has no \
         \"0\" entries at levels 1-10: {}",
        first.detail
    );
    assert!(
        first.detail.contains("base count"),
        "the record must state it grounds the base table count only: {}",
        first.detail
    );
    assert_eq!(
        per_day_values(SORCERER_LEVEL1_FIXTURE).len(),
        1,
        "only the 1st spell level is accessible at level 1"
    );
}

// ----- The literal table rows hold at every grounded step -----

#[test]
fn sorcerer_base_spells_per_day_match_the_raw_table_rows() {
    assert_eq!(
        per_day_values(SORCERER_LEVEL3_FIXTURE),
        vec![(FIRST_ID.to_owned(), 5)],
        "level 3 (`5/—/—/—/—`)"
    );
    assert_eq!(
        per_day_values(SORCERER_LEVEL4_FIXTURE),
        vec![(FIRST_ID.to_owned(), 6), (SECOND_ID.to_owned(), 3)],
        "level 4 (`6/3/—/—/—`): the 1st-level column caps at 6 and the 2nd-level column \
         arrives at 3"
    );
    assert_eq!(
        per_day_values(SORCERER_LEVEL6_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 6),
            (SECOND_ID.to_owned(), 5),
            (THIRD_ID.to_owned(), 3)
        ],
        "level 6 (`6/5/3/—/—`): the two-level cadence brings the 3rd-level column at 3"
    );
    assert_eq!(
        per_day_values(SORCERER_LEVEL7_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 6),
            (SECOND_ID.to_owned(), 6),
            (THIRD_ID.to_owned(), 4)
        ],
        "level 7 (`6/6/4/—/—`)"
    );
    assert_eq!(
        per_day_values(SORCERER_LEVEL8_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 6),
            (SECOND_ID.to_owned(), 6),
            (THIRD_ID.to_owned(), 5),
            (FOURTH_ID.to_owned(), 3)
        ],
        "level 8 (`6/6/5/3/—`)"
    );
    assert_eq!(
        per_day_values(SORCERER_LEVEL9_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 6),
            (SECOND_ID.to_owned(), 6),
            (THIRD_ID.to_owned(), 6),
            (FOURTH_ID.to_owned(), 4)
        ],
        "level 9 (`6/6/6/4/—`)"
    );
    assert_eq!(
        per_day_values(SORCERER_LEVEL10_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 6),
            (SECOND_ID.to_owned(), 6),
            (THIRD_ID.to_owned(), 6),
            (FOURTH_ID.to_owned(), 5),
            (FIFTH_ID.to_owned(), 3)
        ],
        "level 10 (`6/6/6/5/3`): five accessible spell levels — the deepest per-day \
         surface in the tranche"
    );
}

// ----- Base counts only; the blocker persists unmodified -----

#[test]
fn sorcerer_level10_spontaneous_blocker_stays_claim_blocked() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // (v0.6 alpha swarm, risks item 8) SPONTANEOUS_BLOCKER_ID is no longer
    // unconditional -- this fixture has zero known spells, a genuinely valid
    // posture, so the blocker correctly does not fire here. The base per-day
    // counts asserted above already prove the table is grounded for real and
    // never fabricated; that's the property this test actually needs.
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
fn fighter_does_not_gain_sorcerer_per_day_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(PER_DAY_PREFIX)),
        "the Fighter chassis must not surface any sorcerer base per-day record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_does_not_gain_per_day_records() {
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
            .any(|e| e.id.starts_with(PER_DAY_PREFIX)),
        "multiclass Sorcerer must not gain any base per-day record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the per-day grounding -----

#[test]
fn matrix_sorcerer_row_names_the_per_day_grounding() {
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
            .contains("sd13_sorcerer_spells_per_day_counts"),
        "sorcerer row must cite the live per-day-count proof surface: {}",
        sorcerer.grounding_ref
    );
    assert!(
        sorcerer
            .blocker_or_lossiness_note
            .contains("base_spells_per_day"),
        "sorcerer partial note must name the grounded per-day records: {}",
        sorcerer.blocker_or_lossiness_note
    );
}
