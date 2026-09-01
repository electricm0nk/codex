//! SD18 Bard level-11 Inspire widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-10 spontaneous spell-bearing
//! baseline (`tests/sd13_bard_level10_progression.rs`, the tranche-2's
//! declared ceiling) to Bard level 11 — the second SD-18 §3.2 class-row
//! widening, and the first to land on a spell-bearing class — mirroring the
//! sibling-class level-range-gate idiom (`supported_bard_level` is
//! generalized from `1..=10` to `1..=11` via `MAX_SUPPORTED_BARD_LEVEL =
//! 11`, exactly as `cycle-2026-07-13T1255` widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL` from 10 to 11). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Bard class table) were read
//! directly before writing any code or test:
//!
//! - level 11 base attack bonus is +8 (`11 * 3 / 4 = 8`, genuinely risen
//!   from +7 at level 10) and base saves are +3 Fortitude (poor, `11 / 3 =
//!   3`, numerically unchanged from level 10, an integer-division
//!   coincidence), +7 Reflex and +7 Will (both good, `11 / 2 + 2 = 7`, also
//!   unchanged from level 10, another integer-division coincidence) —
//!   confirmed by the same formulas already grounded at levels 1-10, not
//!   re-derived.
//! - the Bardic Performance rounds-per-day pool GENUINELY RISES to 26
//!   (`4 + Charisma modifier 2 + 2 x (11 - 1)`).
//! - Bardic Knowledge stays 5 (`max(11 / 2, 1)`, an integer-division
//!   coincidence with level 10); the Fascinate DC stays 17
//!   (`10 + 11 / 2 + Charisma modifier 2`) and its affected-creature count
//!   stays 4 (`1 + (11 - 1) / 3`), both integer-division coincidences with
//!   level 10; Well-Versed stays the flat +4.
//! - the PF1 Core Rulebook Bard class table's level-11 "Special" column
//!   reads "Inspire competence +4, inspire courage +3, lore master 2/day"
//!   (verified independently against both primary sources, checked rather
//!   than assumed away): Inspire Competence GENUINELY RISES from +3 to +4
//!   (`2 + (11 - 3) / 4`), Inspire Courage GENUINELY RISES from +2 to +3
//!   ("at 5th level, and every six bard levels thereafter, this bonus
//!   increases by +1" — 5th then 11th), and Lore Master's flat take-20
//!   usage count GENUINELY RISES from 1/day to 2/day — three
//!   magnitude-rises on already-grounded flat-constant pillars, mirroring
//!   exactly how the Barbarian Greater Rage magnitude-rise was grounded at
//!   its own 11th-level tier. No new class feature (no new choice slot) is
//!   granted at 11th level, so no new engine is invented.
//! - Jack-of-All-Trades (granted at 10th) carries over unchanged as a +0
//!   identity record; the repeat Versatile Performance grant stays
//!   named-but-unproven.
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any condition-resolution engine, or the spontaneous spell posture burden
//! (all stay named-but-unproven, unchanged from levels 1-10), and it does
//! not ground Bard level 12+. It also preserves the accepted Bard
//! level-1..level-10 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const BARD_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level11_sd18_inspire_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";
const JACK_OF_ALL_TRADES_ID: &str = "class_feature.bard.jack_of_all_trades";
const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";

// ----- Base attack bonus and saves at level 11 -----

#[test]
fn bard_level11_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Bard level 11 3/4-BAB progression (11 * 3 / 4) must equal 8, genuinely risen from 7 \
         at level 10: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Bard level 11 poor Fortitude (11/3) must stay 3");

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(reflex.value, 7, "Bard level 11 good Reflex (11/2+2) must stay 7");

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(will.value, 7, "Bard level 11 good Will (11/2+2) must stay 7");
}

// ----- Performance rounds rise; knowledge and fascinate carry over -----

#[test]
fn bard_level11_performance_rounds_rise_knowledge_and_fascinate_carry_over() {
    let input = load(BARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 27,
        "Bard level 11 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (11 - 1)) must \
         equal 27, genuinely risen from 25: {}",
        rounds.detail
    );

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 5,
        "Bardic Knowledge (max(11/2, 1)) must stay 5, an integer-division coincidence with \
         level 10"
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 18,
        "the Fascinate DC (10 + 11/2 + Charisma modifier 3) must stay 18, an integer-division \
         coincidence with level 10"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 4,
        "the Fascinate affected-creature count (1 + (11-1)/3) must stay 4, an integer-division \
         coincidence with level 10"
    );
}

// ----- Inspire Courage, Inspire Competence, and Lore Master all genuinely rise -----

#[test]
fn bard_level11_third_tier_magnitudes_genuinely_rise() {
    let input = load(BARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 3,
        "Inspire Courage must genuinely rise to +3 at level 11 (the third tier, six levels \
         after the level-5 second tier): {}",
        inspire_courage.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 4,
        "Inspire Competence must genuinely rise to +4 at level 11 (2 + (11-3)/4): {}",
        inspire_competence.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 2,
        "Lore Master's flat take-20 usage count must genuinely rise to 2/day at level 11: {}",
        lore_master.detail
    );

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(well_versed.value, 4, "Well-Versed must stay the flat +4 at level 11");
}

// ----- Jack-of-All-Trades and Versatile Performance carry over unchanged -----

#[test]
fn bard_level11_jack_of_all_trades_carries_over_and_versatile_performance_stays_unproven() {
    let input = load(BARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let joat = explanation(&computation, JACK_OF_ALL_TRADES_ID);
    assert_eq!(
        joat.value, 0,
        "Jack-of-All-Trades must carry over as a +0 identity/recognition record at level 11: {}",
        joat.detail
    );

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("versatile")),
        "level-11 Bard must not fabricate any versatile-performance record: {:?}",
        computation.explanations
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 11 -----

#[test]
fn bard_level11_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.bard.bardic_performance_execution.rounds_exceeded")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let not_performing = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.bard.bardic_performance_execution.not_performing");
            assert!(
                not_performing.is_some(),
                "level-11 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn bard_level10_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(inspire_courage.value, 2, "Bard level 10 Inspire Courage must stay +2");

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(inspire_competence.value, 3, "Bard level 10 Inspire Competence must stay +3");

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(lore_master.value, 1, "Bard level 10 Lore Master must stay 1/day");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 25, "Bard level 10 performance rounds must stay 25");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (Bard levels 12 through 17 were widened into scope by later SD18 slices —
// up through tests/sd18_bard_level17_widening.rs — so this negative
// control's boundary moves from 12 to 18, mirroring the exact same
// boundary-move idiom applied to every prior sibling class's own level
// widening cycle, then moved again from 18 to 19 by the SD18
// bard-level18-widening cycle, then to 20 by the SD18
// bard-level19-widening cycle, then to 21 (a pure implementation-gate
// check, since PF1 has no 21st character level) by the SD18
// bard-level20-widening cycle.)

#[test]
fn bard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARD_LEVEL11_FIXTURE.replace("class:bard:11", "class:bard:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.bard.")
                || e.id.starts_with("class_feature.bard.")
                || e.id == "class_chassis.spell_baseline.bard")
                // (v0.6 alpha swarm, risks item 8) bardic-performance-
                // execution's not-performing explanation is checked
                // unconditionally, regardless of level bound or
                // single-class status (mirrors the spell-posture
                // classes' and Barbarian's gate-ordering fix)
                && e.id != "class_feature.bard.bardic_performance_execution.not_performing"),
        "level-21 Bard must not gain any bounded bard explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level11_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id.starts_with("class_feature.bard.")),
        "the Fighter chassis must not surface any bard-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_level11_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL11_FIXTURE.replace(
        "class_level=class:bard:11",
        "class_level=class:bard:11\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.bard.")
                || e.id.starts_with("class_feature.bard."))
                // (v0.6 alpha swarm, risks item 8) bardic-performance-
                // execution's not-performing explanation is checked
                // unconditionally, regardless of level bound or
                // single-class status (mirrors the spell-posture
                // classes' and Barbarian's gate-ordering fix)
                && e.id != "class_feature.bard.bardic_performance_execution.not_performing"),
        "multiclass Bard must not gain any bounded bard explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_bard_row_names_level_11_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level11_inspire_widening"),
        "bard row must cite the live SD18 level-11 Inspire-widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "bard partial note must name the level-11 widening: {note}"
    );
}
