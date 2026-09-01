//! SD13-E5 Ranger total-spells-per-day grounding proof — the fourth and
//! final TOTAL-integration slice, completing the integrated per-day
//! totals across all four partial/spontaneous casters and, with them,
//! the entire flat spell surface of the tranche. Base table count +
//! Wisdom bonus count, the pure sum of two records this codebase already
//! grounds (`tests/sd13_ranger_spells_per_day_counts.rs` +
//! `tests/sd13_ranger_bonus_spells.rs`), each carrying its own
//! two-source verification. No new rules content and no source fetch.
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.ranger.partial_caster.total_spells_per_day.spell_level_<N>`.
//! On the deterministic fixtures (Wisdom 12, modifier +1):
//!
//! - level 4: total 1 (base "0" entry + bonus 1) — the Wisdom
//!   bonus-spells-only access as arithmetic;
//! - level 7: totals 2/0 (base 1/0 + bonus 1/0) — the 2nd-level total is
//!   an honest zero at its gate level;
//! - level 10: totals 3/1/0 (base 2/1/0 + bonus 1/0/0) — the 3rd-level
//!   honest zero repeats the paladin-slice shape with Wisdom.
//!
//! Live end-to-end: raising the fixture Wisdom to 16 (+3) turns the
//! level-10 totals to 3/2/1. Counts only: no prepared-posture selection,
//! no casting execution, no slot consumption or tracking, no save
//! resolution. NO new claim-blocking diagnostic (the ranger has no
//! per-level spell blocker); the computation stays claim-blocked overall
//! via the generic chassis diagnostics. The sibling threshold test's
//! exactly-two-records control gains its fourth family exclusion.
//!
//! It reuses the accepted per-level ranger fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::load;

const RANGER_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level3_sd13_deterministic_input.txt");
const RANGER_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level4_sd13_deterministic_input.txt");
const RANGER_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level7_sd13_deterministic_input.txt");
const RANGER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const TOTAL_PREFIX: &str = "class_chassis.ranger.partial_caster.total_spells_per_day.";

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

// ----- Below the level-4 spell gate: no total records at all -----

#[test]
fn ranger_level3_has_no_total_records() {
    assert!(
        total_values(RANGER_LEVEL3_FIXTURE).is_empty(),
        "a level-3 ranger has no accessible spell level, so no total record may exist"
    );
}

// ----- Level 4: the Wisdom bonus-only access as arithmetic (0 + 1 = 1) -----

#[test]
fn ranger_level4_total_is_one_from_the_wisdom_bonus_alone() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let total = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level total record must exist at level 4");
    assert_eq!(
        total.value, 1,
        "total = base \"0\" entry + Wisdom bonus 1 = 1 — the level-4 ranger genuinely casts \
         one 1st-level spell per day: {}",
        total.detail
    );
    assert!(
        total.detail.contains("base") && total.detail.contains("bonus"),
        "the record must present itself as the sum of the grounded base and bonus records: {}",
        total.detail
    );
}

// ----- The total ladder, with honest zeros at the gate levels -----

#[test]
fn ranger_totals_are_the_sum_of_the_grounded_base_and_bonus_ladders() {
    assert_eq!(
        total_values(RANGER_LEVEL7_FIXTURE),
        vec![(id(1), 2), (id(2), 0)],
        "level 7, Wisdom +1: totals 2/0 (base 1/0 + bonus 1/0) — the 2nd-level total is an \
         honest zero at its gate level"
    );
    assert_eq!(
        total_values(RANGER_LEVEL10_FIXTURE),
        vec![(id(1), 3), (id(2), 1), (id(3), 0)],
        "level 10, Wisdom +1: totals 3/1/0 (base 2/1/0 + bonus 1/0/0) — the 3rd-level \
         honest zero repeats the paladin shape with Wisdom"
    );
}

// ----- The arithmetic is live end-to-end -----

#[test]
fn ranger_totals_track_the_wisdom_modifier() {
    let raised = RANGER_LEVEL10_FIXTURE.replace("ability=wisdom:12", "ability=wisdom:16");
    assert_eq!(
        total_values(&raised),
        vec![(id(1), 3), (id(2), 2), (id(3), 1)],
        "raising Wisdom to 16 (+3) turns the level-10 totals to 3/2/1 (base 2/1/0 + bonus \
         1/1/1) — live end-to-end arithmetic"
    );
}

// ----- Counts only; still claim-blocked overall -----

#[test]
fn ranger_level10_stays_claim_blocked_and_grounds_counts_only() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let total = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level total record must exist");
    assert!(
        total.detail.contains("no slot consumption")
            || total.detail.contains("no prepared-posture"),
        "the record must state it grounds the count only, never execution or posture: {}",
        total.detail
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "the level-10 Ranger must stay claim-blocked overall (prepared posture and lineage \
         are still unproven): {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_total_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(TOTAL_PREFIX)),
        "the Fighter chassis must not surface any ranger total record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_does_not_gain_total_records() {
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
            .any(|e| e.id.starts_with(TOTAL_PREFIX)),
        "multiclass Ranger must not gain any total record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the total grounding -----

#[test]
fn matrix_ranger_row_names_the_total_grounding() {
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
            .contains("sd13_ranger_total_spells_per_day"),
        "ranger row must cite the live total proof surface: {}",
        ranger.grounding_ref
    );
    assert!(
        ranger
            .blocker_or_lossiness_note
            .contains("total_spells_per_day"),
        "ranger partial note must name the grounded total records: {}",
        ranger.blocker_or_lossiness_note
    );
}
