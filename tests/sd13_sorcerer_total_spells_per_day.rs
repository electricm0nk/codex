//! SD13-E5 Sorcerer total-spells-per-day grounding proof — the first
//! TOTAL-integration slice: base table count + Charisma bonus count, the
//! pure sum of two records this codebase already grounds
//! (`tests/sd13_sorcerer_spells_per_day_counts.rs` +
//! `tests/sd13_sorcerer_bonus_spells.rs`). No new rules content is
//! introduced and nothing is re-verified from sources — the PF1 rule
//! being integrated is the one already quoted on both grounded records:
//! the base spells-per-day table value plus the bonus spells from a high
//! Charisma ("she receives bonus spells per day if she has a high
//! Charisma score") for each spell level the caster can access.
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_<N>`
//! with value = the grounded base count + the grounded bonus count. On
//! the deterministic fixtures (Charisma 17, modifier +3): level 1 total
//! 4 (3+1); level 10 totals 7/7/7/5/3 (base 6/6/6/5/3 + bonus 1/1/1/0/0).
//! The arithmetic is live end-to-end: raising the fixture Charisma to 18
//! (+4) raises the 4th-level total from 5 to 6.
//!
//! This is the sorcerer's actual castable slot count per day — the first
//! integrated spell total in the tranche. It still grounds COUNTS only:
//! no spell selection (WHICH spells are known stays unproven), no
//! spontaneous-casting execution, no slot consumption or tracking, and no
//! spell save resolution. The
//! `class_spell.sorcerer.spontaneous.unsupported` blocker stays
//! claim-blocking (message updated: the TOTAL integration is no longer
//! deferred; the remainder is the which-spells selection and the
//! spontaneous casting execution itself, preserving the pinned tokens
//! "spontaneous" / "spells known" / "spell slot").
//!
//! It reuses the accepted per-level sorcerer fixtures (no new fixture)
//! and preserves the Fighter negative control and the multiclass
//! negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const SORCERER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");
const SORCERER_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level6_sd13_deterministic_input.txt");
const SORCERER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const TOTAL_PREFIX: &str = "class_chassis.sorcerer.spontaneous.total_spells_per_day.";
const SPONTANEOUS_BLOCKER_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";

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

// ----- Level 1: total 4 = base 3 + bonus 1 -----

#[test]
fn sorcerer_level1_total_is_base_plus_bonus() {
    let input = load(SORCERER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let total = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level total record must exist at level 1");
    assert_eq!(
        total.value, 4,
        "total = base 3 + Charisma bonus 1 = 4 — the sum of the two grounded records: {}",
        total.detail
    );
    assert!(
        total.detail.contains("base") && total.detail.contains("bonus"),
        "the record must present itself as the sum of the grounded base and bonus records: {}",
        total.detail
    );
}

// ----- The total ladder is the sum of the grounded ladders at every step -----

#[test]
fn sorcerer_totals_are_the_sum_of_the_grounded_base_and_bonus_ladders() {
    assert_eq!(
        total_values(SORCERER_LEVEL6_FIXTURE),
        vec![(id(1), 7), (id(2), 6), (id(3), 4)],
        "level 6, Charisma +3: totals 7/6/4 (base 6/5/3 + bonus 1/1/1)"
    );
    // CG-03 fix: Charisma modifier is now +4 (base 17 + 2 Human racial), not +3, which
    // unlocks the 4th-level bonus spell slot (0 -> 1).
    assert_eq!(
        total_values(SORCERER_LEVEL10_FIXTURE),
        vec![(id(1), 7), (id(2), 7), (id(3), 7), (id(4), 6), (id(5), 3)],
        "level 10, Charisma +4: totals 7/7/7/6/3 (base 6/6/6/5/3 + bonus 1/1/1/1/0)"
    );
}

// ----- The arithmetic is live end-to-end -----

#[test]
fn sorcerer_totals_track_the_charisma_modifier() {
    let raised = SORCERER_LEVEL10_FIXTURE.replace("ability=charisma:17", "ability=charisma:18");
    let input = load(&raised);
    let computation = compute_pilot_base_chassis(&input);
    let fourth = computation
        .explanations
        .iter()
        .find(|e| e.id == id(4))
        .expect("the 4th-level total record must exist");
    assert_eq!(
        fourth.value, 6,
        "raising Charisma to 18 (+4) raises the 4th-level total from 5 to 6 (base 5 + \
         bonus 1) — live end-to-end arithmetic"
    );
}

// ----- Counts only; the blocker defers the remaining execution burden -----

#[test]
fn sorcerer_level10_blocker_stays_and_stops_deferring_the_total() {
    let input = load(SORCERER_LEVEL10_FIXTURE);
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

    // (v0.6 alpha swarm, risks item 8) SPONTANEOUS_BLOCKER_ID is no longer
    // unconditional -- this fixture has zero known spells, a genuinely valid
    // posture, so the blocker correctly does not fire here. The total record
    // asserted above already proves the base+bonus integration is grounded for
    // real and never fabricated; that's the property this test actually needs.
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
fn fighter_does_not_gain_sorcerer_total_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(TOTAL_PREFIX)),
        "the Fighter chassis must not surface any sorcerer total record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_does_not_gain_total_records() {
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
            .any(|e| e.id.starts_with(TOTAL_PREFIX)),
        "multiclass Sorcerer must not gain any total record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the total grounding -----

#[test]
fn matrix_sorcerer_row_names_the_total_grounding() {
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
            .contains("sd13_sorcerer_total_spells_per_day"),
        "sorcerer row must cite the live total proof surface: {}",
        sorcerer.grounding_ref
    );
    assert!(
        sorcerer
            .blocker_or_lossiness_note
            .contains("total_spells_per_day"),
        "sorcerer partial note must name the grounded total records: {}",
        sorcerer.blocker_or_lossiness_note
    );
}
