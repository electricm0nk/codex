//! SD13-E5 Ranger bonus-spells-per-day grounding proof — the fourth and
//! final bonus-spells slice, completing the bonus family across all four
//! partial/spontaneous casters and, with it, the per-class flat spell
//! surface short of the base+bonus TOTAL integration. Mirrors the
//! Sorcerer/Bard/Paladin bonus slices against PF1's shared Table: Ability
//! Modifiers and Bonus Spells (verified this session against both primary
//! sources' ability-scores pages, quoted identically: for modifier m and
//! spell level N (1st+), 0 when m < N, otherwise (m - N)/4 + 1). The
//! ranger-specific rule text was verified on both class pages this
//! session in the per-day cycle: "he gains only the bonus spells he would
//! be entitled to based on his Wisdom score for that spell level" — the
//! family's only WISDOM caster.
//!
//! One record per ACCESSIBLE spell level:
//! `class_chassis.ranger.partial_caster.bonus_spells_per_day.spell_level_<N>`.
//! On the deterministic fixtures (Wisdom 12, modifier +1): bonus 1/0/0
//! across spell levels 1-3 at level 10 — the +1 row grants a bonus at 1st
//! only; below the level-4 spell gate no bonus record exists at all. The
//! level-4 "0"-base/1-bonus side-by-side pin from the paladin slice
//! repeats here with Wisdom. Live arithmetic: raising the fixture Wisdom
//! to 16 (+3) turns the 2nd- and 3rd-level bonuses to 1.
//!
//! Bonus COUNTS only: never added to the base per-day counts, no spell
//! selection, no DCs. NO new claim-blocking diagnostic (the ranger has no
//! per-level spell blocker); the computation stays claim-blocked overall
//! via the generic chassis diagnostics. The sibling threshold test's
//! exactly-two-records control is extended to exclude the new
//! bonus_spells_per_day family, preserving its claim.
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
const RANGER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BONUS_PREFIX: &str = "class_chassis.ranger.partial_caster.bonus_spells_per_day.";
const BASE_FIRST_ID: &str =
    "class_chassis.ranger.partial_caster.base_spells_per_day.spell_level_1";

fn bonus_values(fixture: &str) -> Vec<(String, i16)> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(BONUS_PREFIX))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

fn id(spell_level: u8) -> String {
    format!("{BONUS_PREFIX}spell_level_{spell_level}")
}

// ----- Below the level-4 spell gate: no bonus records at all -----

#[test]
fn ranger_level3_has_no_bonus_records() {
    assert!(
        bonus_values(RANGER_LEVEL3_FIXTURE).is_empty(),
        "a level-3 ranger has no accessible spell level, so no bonus-spells record may exist"
    );
}

// ----- Level 4: the "0"-base / 1-bonus interplay with Wisdom -----

#[test]
fn ranger_level4_zero_base_and_one_bonus_are_the_bonus_only_access() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base = computation
        .explanations
        .iter()
        .find(|e| e.id == BASE_FIRST_ID)
        .expect("the grounded base per-day record must exist at level 4");
    assert_eq!(base.value, 0, "the level-4 base count is the literal \"0\" table entry");

    let bonus = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level bonus record must exist at level 4");
    assert_eq!(
        bonus.value, 1,
        "Wisdom +1 grants 1 bonus 1st-level spell — together with the \"0\" base entry this \
         is the Wisdom bonus-spells-only access the PF1 rule describes: {}",
        bonus.detail
    );
    assert!(
        bonus.detail.contains("Ability Modifiers and Bonus Spells")
            && bonus.detail.contains("Wisdom"),
        "the record must cite the shared PF1 table with the ranger's WISDOM modifier: {}",
        bonus.detail
    );
}

// ----- The bonus ladder tracks modifier and access at every step -----

#[test]
fn ranger_bonus_spells_track_the_table_and_the_access_ladder() {
    assert_eq!(
        bonus_values(RANGER_LEVEL4_FIXTURE),
        vec![(id(1), 1)],
        "level 4, Wisdom +1: one accessible spell level, bonus 1"
    );
    assert_eq!(
        bonus_values(RANGER_LEVEL10_FIXTURE),
        vec![(id(1), 1), (id(2), 0), (id(3), 0)],
        "level 10, Wisdom +1: bonus 1/0/0 — the +1 row grants a bonus at 1st only; honest \
         computed 0s at 2nd and 3rd"
    );
}

// ----- The arithmetic is live: a higher Wisdom fills in the bonuses -----

#[test]
fn ranger_bonus_spells_track_the_wisdom_modifier() {
    let raised = RANGER_LEVEL10_FIXTURE.replace("ability=wisdom:12", "ability=wisdom:16");
    let input = load(&raised);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        bonus_values(&raised),
        vec![(id(1), 1), (id(2), 1), (id(3), 1)],
        "raising Wisdom to 16 (+3) fills the 2nd- and 3rd-level bonuses (+3 row: 1/1/1) — \
         live arithmetic over the chosen ability score"
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "the raised-Wisdom variant must stay claim-blocked overall"
    );
}

// ----- Bonus counts only; still claim-blocked overall -----

#[test]
fn ranger_level10_stays_claim_blocked_and_grounds_bonus_counts_only() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("the 1st-level bonus record must exist");
    assert!(
        bonus.detail.contains("never added") || bonus.detail.contains("no total"),
        "the record must state the bonus is never added to the base counts: {}",
        bonus.detail
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "the level-10 Ranger must stay claim-blocked overall (prepared posture, lineage, \
         and the base+bonus TOTAL integration are still unproven): {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_bonus_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(BONUS_PREFIX)),
        "the Fighter chassis must not surface any ranger bonus-spells record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_does_not_gain_bonus_records() {
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
            .any(|e| e.id.starts_with(BONUS_PREFIX)),
        "multiclass Ranger must not gain any bonus-spells record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the bonus-spells grounding -----

#[test]
fn matrix_ranger_row_names_the_bonus_spells_grounding() {
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
        ranger.grounding_ref.contains("sd13_ranger_bonus_spells"),
        "ranger row must cite the live bonus-spells proof surface: {}",
        ranger.grounding_ref
    );
    assert!(
        ranger
            .blocker_or_lossiness_note
            .contains("bonus_spells_per_day"),
        "ranger partial note must name the grounded bonus-spells records: {}",
        ranger.blocker_or_lossiness_note
    );
}
