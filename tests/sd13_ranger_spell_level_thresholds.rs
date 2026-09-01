//! SD13-E5 Ranger partial-caster spell-level-access threshold grounding
//! proof — a priority-2 spell-posture burden slice, completing the
//! access-ladder family (Paladin, Bard, Sorcerer before it) by giving the
//! Ranger the same two partial-caster identity records the Paladin already
//! carries: the effective-caster-level gate arithmetic and the spell-level
//! ACCESS ladder, per the Cleric/Wizard "first non-'—' spells-per-day
//! column" threshold doctrine. Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Ranger page) were read directly before writing any
//! code, and both show identical raw spells-per-day rows and rule text:
//!
//! - levels 1-3: no spells-per-day columns at all — no spell access
//! - level 4: `0/—/—/—` — the FIRST non-"—" 1st-level column (a "0" entry:
//!   "When Table: Ranger indicates that the ranger gets 0 spells per day of
//!   a given spell level, he gains only the bonus spells he would be
//!   entitled to based on his Wisdom score for that spell level" — Wisdom,
//!   not the Paladin's Charisma)
//! - level 7: `1/0/—/—` — the FIRST non-"—" 2nd-level column
//! - level 10: `2/1/0/—` — the FIRST non-"—" 3rd-level column; the
//!   4th-level column stays "—" through level 10 (4th-level ranger spells
//!   begin at 13, outside the tranche ceiling)
//! - caster level: "At 4th level and higher, his caster level is equal to
//!   his ranger level – 3" — the exact rule shape already grounded for the
//!   Paladin (max(level - 3, 0))
//!
//! The two grounded records
//! (`class_chassis.ranger.partial_caster.effective_caster_level`,
//! `class_chassis.ranger.partial_caster.spell_level_access`) mirror the
//! Paladin pair record-for-record. They ground gate arithmetic and the
//! access ladder ONLY: no spells-per-day counts, no spells-known/prepared
//! posture (the ranger prepares from the ranger list, Wisdom-based), no
//! bonus slots, and no spell save DCs. NO new claim-blocking diagnostic is
//! added — the ranger spell burden stays named by the accepted F6 level-1
//! hybrid spell blocker and the row's matrix note, and the computation
//! stays claim-blocked overall at every supported level via the generic
//! chassis diagnostics (asserted below).
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
const RANGER_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level6_sd13_deterministic_input.txt");
const RANGER_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level7_sd13_deterministic_input.txt");
const RANGER_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level9_sd13_deterministic_input.txt");
const RANGER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const EFFECTIVE_CASTER_LEVEL_ID: &str =
    "class_chassis.ranger.partial_caster.effective_caster_level";
const SPELL_LEVEL_ACCESS_ID: &str = "class_chassis.ranger.partial_caster.spell_level_access";

fn values_at(fixture: &str) -> (i16, i16) {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    (
        explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID).value,
        explanation(&computation, SPELL_LEVEL_ACCESS_ID).value,
    )
}

// ----- Below the level-4 gate: both records are correct zero absences -----

#[test]
fn ranger_level3_partial_caster_records_are_correct_zero_absences() {
    let (caster_level, access) = values_at(RANGER_LEVEL3_FIXTURE);
    assert_eq!(
        caster_level, 0,
        "a level-3 ranger has no caster level (the rule begins at 4th level: max(3 - 3, 0))"
    );
    assert_eq!(
        access, 0,
        "a level-3 ranger has no spell access at all (no spells-per-day columns below \
         level 4)"
    );
}

// ----- Level 4: the partial-caster identity begins -----

#[test]
fn ranger_level4_gains_caster_level_one_and_first_level_spell_access() {
    let input = load(RANGER_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 1,
        "PF1 CRB: at 4th level and higher, caster level = ranger level - 3 = 1: {}",
        caster_level.detail
    );

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 1,
        "level 4 shows the first non-\"—\" 1st-level spells-per-day column (a \"0\" entry): {}",
        access.detail
    );
    assert!(
        access.detail.contains("Wisdom"),
        "the record must surface the PF1 \"0 spells per day\" rule nuance with the ranger's \
         Wisdom (not the paladin's Charisma) as the bonus-spell stat: {}",
        access.detail
    );
}

// ----- The ladders hold at every grounded step -----

#[test]
fn ranger_partial_caster_ladders_match_the_raw_table_rows() {
    assert_eq!(
        values_at(RANGER_LEVEL6_FIXTURE),
        (3, 1),
        "level 6 (`1/—/—/—`): caster level 3, still 1st-level access only"
    );
    assert_eq!(
        values_at(RANGER_LEVEL7_FIXTURE),
        (4, 2),
        "level 7 (`1/0/—/—`): caster level 4, and the first non-\"—\" 2nd-level column — \
         access genuinely rises to 2"
    );
    assert_eq!(
        values_at(RANGER_LEVEL9_FIXTURE),
        (6, 2),
        "level 9 (`2/1/—/—`): caster level 6, access stays 2"
    );
    assert_eq!(
        values_at(RANGER_LEVEL10_FIXTURE),
        (7, 3),
        "level 10 (`2/1/0/—`): caster level 7, and the first non-\"—\" 3rd-level column — \
         access genuinely rises to 3"
    );
}

// ----- No counts fabricated; the computation stays claim-blocked overall -----

#[test]
fn ranger_level10_stays_claim_blocked_and_fabricates_no_spell_counts() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert!(
        access.detail.contains("no spells-per-day counts")
            || access.detail.contains("no spells per day"),
        "the record must state that it grounds the access ladder only, never per-day \
         counts: {}",
        access.detail
    );

    // No third spell-tagged ranger record beyond the grounded pair — excluding
    // the base_spells_per_day family a further SD13-E5 slice grounded
    // (tests/sd13_ranger_spells_per_day_counts.rs carries its proof), which
    // this control could not have known about; its original claim (no
    // fabricated third IDENTITY record) is preserved.
    let spell_tagged: Vec<_> = computation
        .explanations
        .iter()
        .filter(|e| {
            e.id.starts_with("class_chassis.ranger.partial_caster.")
                && !e
                    .id
                    .starts_with("class_chassis.ranger.partial_caster.base_spells_per_day.")
                // The spell_save_dc family (a further SD13-E5 slice,
                // tests/sd13_ranger_spell_save_dcs.rs) is likewise excluded;
                // the original no-fabricated-third-identity-record claim holds.
                && !e
                    .id
                    .starts_with("class_chassis.ranger.partial_caster.spell_save_dc.")
                // The bonus_spells_per_day family (a further SD13-E5 slice,
                // tests/sd13_ranger_bonus_spells.rs) is likewise excluded.
                && !e
                    .id
                    .starts_with("class_chassis.ranger.partial_caster.bonus_spells_per_day.")
                // And the total_spells_per_day family (a further SD13-E5
                // slice, tests/sd13_ranger_total_spells_per_day.rs).
                && !e
                    .id
                    .starts_with("class_chassis.ranger.partial_caster.total_spells_per_day.")
        })
        .collect();
    assert_eq!(
        spell_tagged.len(),
        2,
        "exactly the effective-caster-level and access-ladder records are allowed: \
         {spell_tagged:?}"
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "the level-10 Ranger must stay claim-blocked overall (the spell burden beyond the \
         grounded gate arithmetic is still unproven): {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_partial_caster_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.partial_caster.")),
        "the Fighter chassis must not surface any ranger partial-caster record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_does_not_gain_partial_caster_records() {
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
            .any(|e| e.id.starts_with("class_chassis.ranger.partial_caster.")),
        "multiclass Ranger must not gain any partial-caster record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the partial-caster grounding -----

#[test]
fn matrix_ranger_row_names_the_partial_caster_grounding() {
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
            .contains("sd13_ranger_spell_level_thresholds"),
        "ranger row must cite the live partial-caster proof surface: {}",
        ranger.grounding_ref
    );
    assert!(
        ranger
            .blocker_or_lossiness_note
            .contains("spell_level_access"),
        "ranger partial note must name the grounded access-ladder record: {}",
        ranger.blocker_or_lossiness_note
    );
}
