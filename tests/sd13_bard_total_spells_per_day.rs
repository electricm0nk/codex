//! SD13-E5 Bard total-spells-per-day grounding proof — the second
//! TOTAL-integration slice, mirroring the Sorcerer total slice
//! (`tests/sd13_sorcerer_total_spells_per_day.rs`): base table count +
//! Charisma bonus count, the pure sum of two records this codebase
//! already grounds (`tests/sd13_bard_spells_per_day_counts.rs` +
//! `tests/sd13_bard_bonus_spells.rs`), each carrying its own two-source
//! verification. No new rules content and no source fetch.
//!
//! One record per ACCESSIBLE spell level (1st+; cantrips have no per-day
//! column and no bonus, so no total exists for them):
//! `class_chassis.bard.spontaneous.total_spells_per_day.spell_level_<N>`.
//! On the deterministic fixtures (Charisma 15, modifier +2): level 1
//! total 2 (1+1); level 7 totals 5/4/1 (base 4/3/1 + bonus 1/1/0); level
//! 10 totals 6/5/3/1 (base 5/4/3/1 + bonus 1/1/0/0) — the bard's actual
//! castable slot count per day. Live end-to-end: raising the fixture
//! Charisma to 16 (+3) raises the 3rd-level total from 3 to 4.
//!
//! Counts only: no which-spells selection, no spontaneous-casting
//! execution, no slot consumption or tracking, no save resolution. The
//! `class_spell.bard.spontaneous_known_and_per_day.unsupported` blocker
//! stays claim-blocking (message updated: the TOTAL integration is no
//! longer deferred; the remainder is the casting execution and the
//! which-spells selection, preserving the pinned tokens "spontaneous" /
//! "spells known" / "spells per day").
//!
//! Sibling allow-lists: the established pattern (level-1 baseline by
//! prefix; level-4/5/6/7/8 `known_bard_ids` by exact ids).
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
const BARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level7_sd13_deterministic_input.txt");
const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const TOTAL_PREFIX: &str = "class_chassis.bard.spontaneous.total_spells_per_day.";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.bard.spontaneous_known_and_per_day.unsupported";

fn total_values(fixture: &str) -> Vec<(String, i16)> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(TOTAL_PREFIX))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

fn id(spell_level: u8) -> String {
    format!("{TOTAL_PREFIX}spell_level_{spell_level}")
}

// ----- Level 1: total 2 = base 1 + bonus 1; no cantrip total -----

#[test]
fn bard_level1_total_is_base_plus_bonus_with_no_cantrip_total() {
    let input = load(BARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        total_values(BARD_LEVEL1_FIXTURE),
        vec![(id(1), 2)],
        "level 1, Charisma +2: total 2 (base 1 + bonus 1); cantrips have no per-day column \
         and no bonus, so no 0th-level total exists"
    );

    let total = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level total record must exist");
    assert!(
        total.detail.contains("base") && total.detail.contains("bonus"),
        "the record must present itself as the sum of the grounded base and bonus records: {}",
        total.detail
    );
}

// ----- The total ladder is the sum of the grounded ladders at every step -----

#[test]
fn bard_totals_are_the_sum_of_the_grounded_base_and_bonus_ladders() {
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), unlocking the
    // 3rd-level bonus spell slot (0 -> 1) that the sibling bonus-spells test file
    // documents in full.
    assert_eq!(
        total_values(BARD_LEVEL7_FIXTURE),
        vec![(id(1), 5), (id(2), 4), (id(3), 2)],
        "level 7, Charisma +3: totals 5/4/2 (base 4/3/1 + bonus 1/1/1)"
    );
    assert_eq!(
        total_values(BARD_LEVEL10_FIXTURE),
        vec![(id(1), 6), (id(2), 5), (id(3), 4), (id(4), 1)],
        "level 10, Charisma +3: totals 6/5/4/1 (base 5/4/3/1 + bonus 1/1/1/0)"
    );
}

// ----- The arithmetic is live end-to-end -----

#[test]
fn bard_totals_track_the_charisma_modifier() {
    let raised = BARD_LEVEL10_FIXTURE.replace("ability=charisma:15", "ability=charisma:16");
    let input = load(&raised);
    let computation = compute_pilot_base_chassis(&input);
    let third = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level total record must exist");
    assert_eq!(
        third.value, 4,
        "raising Charisma to 16 (+3) raises the 3rd-level total from 3 to 4 (base 3 + \
         bonus 1) — live end-to-end arithmetic"
    );
}

// ----- Counts only; the blocker defers the remaining execution burden -----

#[test]
fn bard_level10_blocker_stays_and_stops_deferring_the_total() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let total = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level total record must exist");
    assert!(
        total.detail.contains("no slot consumption") || total.detail.contains("no spell selection"),
        "the record must state it grounds the count only, never execution: {}",
        total.detail
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
fn fighter_does_not_gain_bard_total_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(TOTAL_PREFIX)),
        "the Fighter chassis must not surface any bard total record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_does_not_gain_total_records() {
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
            .any(|e| e.id.starts_with(TOTAL_PREFIX)),
        "multiclass Bard must not gain any total record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the total grounding -----

#[test]
fn matrix_bard_row_names_the_total_grounding() {
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
            .contains("sd13_bard_total_spells_per_day"),
        "bard row must cite the live total proof surface: {}",
        bard.grounding_ref
    );
    assert!(
        bard.blocker_or_lossiness_note
            .contains("total_spells_per_day"),
        "bard partial note must name the grounded total records: {}",
        bard.blocker_or_lossiness_note
    );
}
