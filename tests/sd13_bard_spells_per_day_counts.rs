//! SD13-E5 Bard base spells-per-day count grounding proof — the third
//! per-day SLOT-COUNT slice, mirroring the Paladin and Ranger per-day
//! slices on top of the grounded bard access ladder
//! (`tests/sd13_bard_spell_level_thresholds.rs`), per the Cleric
//! domain-slot-count literal-table doctrine (a lookup table, not
//! arithmetic — no formula is invented). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Bard class table) were read directly
//! this session before writing any code, and both show identical raw
//! spells-per-day rows for levels 1-10:
//!
//! - level 1: `1/—/—/—`     - level 2: `2/—/—/—`    - level 3: `3/—/—/—`
//! - level 4: `3/1/—/—`     - level 5: `4/2/—/—`    - level 6: `4/3/—/—`
//! - level 7: `4/3/1/—`     - level 8: `4/4/2/—`    - level 9: `5/4/3/—`
//! - level 10: `5/4/3/1`
//!
//! Unlike the Paladin/Ranger tables, the Bard table has NO "0" entries at
//! levels 1-10 — every accessible spell level carries a positive base
//! count from the moment its column appears. One record per ACCESSIBLE
//! spell level:
//! `class_chassis.bard.spontaneous.base_spells_per_day.spell_level_<N>`
//! with the raw base count as its value; inaccessible "—" spell levels
//! get no record at all. Base counts only: bonus spells from a high
//! Charisma are never computed, no spells-known posture is grounded
//! (spells KNOWN is a separate table from spells per day — deliberately
//! untouched), and no spell save DCs are computed — the
//! `class_spell.bard.spontaneous_known_and_per_day.unsupported` blocker
//! stays claim-blocking at every supported level (its message already
//! defers the whole posture; the base per-day counts grounded here are a
//! strict subset, so its wording stays honest without modification —
//! "spells per day (from the Bard table plus CHA modifier)" remains
//! unproven as the CHA-modified TOTAL; only the base table column is
//! grounded).
//!
//! Sibling allow-list extensions (the new records fire at every
//! supported level): the level-1 baseline's spell/bardic-tag allow-list
//! gains the per-day family, and the level-4/5/6/7/8 `known_bard_ids`
//! no-new-class-feature controls gain the family too — each with the
//! same comment pattern the access-ladder slice established.
//!
//! It reuses the accepted per-level bard fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

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
const BARD_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level8_sd13_deterministic_input.txt");
const BARD_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level9_sd13_deterministic_input.txt");
const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.bard.spontaneous.base_spells_per_day.";
const FIRST_ID: &str = "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_1";
const SECOND_ID: &str = "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_2";
const THIRD_ID: &str = "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_3";
const FOURTH_ID: &str = "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_4";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.bard.spontaneous_known_and_per_day.unsupported";

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

// ----- Level 1: one record, base count 1 — no zero step, no "0" entries -----

#[test]
fn bard_level1_first_level_base_count_is_one() {
    let input = load(BARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first = computation
        .explanations
        .iter()
        .find(|e| e.id == FIRST_ID)
        .expect("level-1 bard must carry the 1st-level base per-day record");
    assert_eq!(
        first.value, 1,
        "level 1 (`1/—/—/—`): the 1st-level base count is 1 — the Bard table has no \"0\" \
         entries at levels 1-10: {}",
        first.detail
    );
    assert!(
        first.detail.contains("base count"),
        "the record must state it grounds the base table count only: {}",
        first.detail
    );
    assert_eq!(
        per_day_values(BARD_LEVEL1_FIXTURE).len(),
        1,
        "only the 1st spell level is accessible at level 1"
    );
}

// ----- The literal table rows hold at every grounded step -----

#[test]
fn bard_base_spells_per_day_match_the_raw_table_rows() {
    assert_eq!(
        per_day_values(BARD_LEVEL3_FIXTURE),
        vec![(FIRST_ID.to_owned(), 3)],
        "level 3 (`3/—/—/—`)"
    );
    assert_eq!(
        per_day_values(BARD_LEVEL4_FIXTURE),
        vec![(FIRST_ID.to_owned(), 3), (SECOND_ID.to_owned(), 1)],
        "level 4 (`3/1/—/—`): the 2nd-level column arrives at a positive 1, not a \"0\""
    );
    assert_eq!(
        per_day_values(BARD_LEVEL6_FIXTURE),
        vec![(FIRST_ID.to_owned(), 4), (SECOND_ID.to_owned(), 3)],
        "level 6 (`4/3/—/—`)"
    );
    assert_eq!(
        per_day_values(BARD_LEVEL7_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 4),
            (SECOND_ID.to_owned(), 3),
            (THIRD_ID.to_owned(), 1)
        ],
        "level 7 (`4/3/1/—`)"
    );
    assert_eq!(
        per_day_values(BARD_LEVEL8_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 4),
            (SECOND_ID.to_owned(), 4),
            (THIRD_ID.to_owned(), 2)
        ],
        "level 8 (`4/4/2/—`)"
    );
    assert_eq!(
        per_day_values(BARD_LEVEL9_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 5),
            (SECOND_ID.to_owned(), 4),
            (THIRD_ID.to_owned(), 3)
        ],
        "level 9 (`5/4/3/—`)"
    );
    assert_eq!(
        per_day_values(BARD_LEVEL10_FIXTURE),
        vec![
            (FIRST_ID.to_owned(), 5),
            (SECOND_ID.to_owned(), 4),
            (THIRD_ID.to_owned(), 3),
            (FOURTH_ID.to_owned(), 1)
        ],
        "level 10 (`5/4/3/1`): the 4th-level column arrives at a positive 1"
    );
}

// ----- Base counts only; the blocker persists unmodified -----

#[test]
fn bard_level10_spontaneous_blocker_stays_claim_blocked() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

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
fn fighter_does_not_gain_bard_per_day_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(PER_DAY_PREFIX)),
        "the Fighter chassis must not surface any bard base per-day record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_does_not_gain_per_day_records() {
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
            .any(|e| e.id.starts_with(PER_DAY_PREFIX)),
        "multiclass Bard must not gain any base per-day record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the per-day grounding -----

#[test]
fn matrix_bard_row_names_the_per_day_grounding() {
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
            .contains("sd13_bard_spells_per_day_counts"),
        "bard row must cite the live per-day-count proof surface: {}",
        bard.grounding_ref
    );
    assert!(
        bard.blocker_or_lossiness_note
            .contains("base_spells_per_day"),
        "bard partial note must name the grounded per-day records: {}",
        bard.blocker_or_lossiness_note
    );
}
