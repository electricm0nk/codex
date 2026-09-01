//! SD13-E5 Bard level-10 progression grounding proof.
//!
//! Widens the accepted Bard level-1..level-9 spontaneous spell-burden
//! baseline (most recently `tests/sd13_bard_level9_progression.rs`) to Bard
//! level 10 — the tranche's declared ceiling — mirroring the sibling-class
//! level-range-gate idiom (`supported_bard_level` is generalized from `1..=9`
//! to `1..=10` via `MAX_SUPPORTED_BARD_LEVEL = 10`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Bard class table) were read
//! directly before writing any code or test:
//!
//! - level 10 base attack bonus is +7 (`10 * 3 / 4 = 7`, genuinely risen
//!   from +6 at level 9 — the table's own "+7/+2" iterative notation is not
//!   modeled anywhere in this codebase, only the flat base value) and base
//!   saves are +3 Fortitude (poor, `10 / 3 = 3`, numerically unchanged from
//!   level 9, an integer-division coincidence), +7 Reflex and +7 Will (both
//!   good, `10 / 2 + 2 = 7`, both genuinely risen from +6) — confirmed by
//!   the same formulas already grounded at levels 1-9, not re-derived.
//! - the Bardic Performance rounds-per-day pool GENUINELY RISES to 24
//!   (`4 + Charisma modifier 2 + 2 × (10 - 1)`).
//! - Bardic Knowledge GENUINELY RISES to 5 (`max(10 / 2, 1)`); the
//!   Fascinate DC GENUINELY RISES to 17 (`10 + 10 / 2 + Charisma modifier
//!   2`) and its affected-creature count GENUINELY RISES to 4
//!   (`1 + (10 - 1) / 3`); Inspire Courage stays +2 and Inspire Competence
//!   stays +3 (both next tiers land at 11th); Well-Versed stays +4 and Lore
//!   Master stays 1/day (next rise at 11th).
//! - the PF1 Core Rulebook Bard class table's level-10 "Special" column
//!   reads "Jack-of-all-trades, versatile performance" (verified
//!   independently against both primary sources, checked rather than
//!   assumed away):
//!   - Jack-of-All-Trades' 10th-level piece ("the bard can use any skill,
//!     even if the skill normally requires him to be trained") is a
//!     genuinely flat, no-choice, no-magnitude grant, grounded by this
//!     slice as a +0 identity/recognition record
//!     (`class_feature.bard.jack_of_all_trades`), mirroring the Woodland
//!     Stride / Purity of Body idiom — no trained-only skill gating exists
//!     in this codebase to lift, so no untrained-use effect is fabricated;
//!     its 16th- and 19th-level tiers land beyond the tranche ceiling.
//!   - the repeat Versatile Performance grant stays named-but-unproven
//!     exactly like the 2nd/6th-level grants before it (a dedicated
//!     negative test pins that no versatile-performance record is
//!     fabricated).
//!
//! It deliberately does not touch the performance-state engine, any
//! condition-resolution engine, or the spontaneous spell posture burden (all
//! stay named-but-unproven, unchanged from levels 1-9), and it does not
//! ground Bard level 11+. It also preserves the accepted Bard
//! level-1..level-9 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level9_sd13_deterministic_input.txt");

const BARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";
const JACK_OF_ALL_TRADES_ID: &str = "class_feature.bard.jack_of_all_trades";

// ----- Base attack bonus and saves at level 10 -----

#[test]
fn bard_level10_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Bard level 10 3/4-BAB progression (10 * 3 / 4) must equal 7, genuinely risen from 6 \
         at level 9: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Bard level 10 poor Fortitude (10/3) must equal 3");

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 7,
        "Bard level 10 good Reflex (10/2+2) must equal 7, genuinely risen from 6"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 7,
        "Bard level 10 good Will (10/2+2) must equal 7, genuinely risen from 6"
    );
}

// ----- Performance rounds, knowledge, and fascinate all genuinely rise -----

#[test]
fn bard_level10_performance_knowledge_and_fascinate_rise() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 25,
        "Bard level 10 Bardic Performance rounds (4 + Charisma modifier 3 + 2 × (10 - 1)) \
         must equal 25, genuinely risen from 23: {}",
        rounds.detail
    );

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 5,
        "Bardic Knowledge (max(10/2, 1)) must equal 5, genuinely risen from 4 at levels 8-9"
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 18,
        "the Fascinate DC (10 + 10/2 + Charisma modifier 3) must equal 18, genuinely risen \
         from 17"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 4,
        "the Fascinate affected-creature count (1 + (10-1)/3) must equal 4, genuinely risen \
         from 3 at levels 7-9"
    );
}

// ----- Inspire tiers / Well-Versed / Lore Master carry over at level 10 -----

#[test]
fn bard_level10_inspire_tiers_carry_over() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 2,
        "Inspire Courage must stay +2 at level 10 (the next tier lands at 11th)"
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 3,
        "Inspire Competence must stay +3 at level 10 (the next tier lands at 11th)"
    );

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(well_versed.value, 4, "Well-Versed must stay the flat +4 at level 10");

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 1,
        "Lore Master must stay 1/day at level 10 (the next rise lands at 11th)"
    );
}

// ----- Jack-of-All-Trades is newly grounded as a +0 identity record -----

#[test]
fn bard_level10_grounds_jack_of_all_trades_as_identity_recognition_only() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let joat = explanation(&computation, JACK_OF_ALL_TRADES_ID);
    assert_eq!(
        joat.value, 0,
        "Jack-of-All-Trades must be grounded as a +0 identity/recognition record only — no \
         trained-only skill gating exists in this codebase to lift: {}",
        joat.detail
    );
    assert!(
        joat.detail.contains("any skill"),
        "Jack-of-All-Trades' record must carry the rule's own use-any-skill-untrained \
         identity: {}",
        joat.detail
    );
}

// ----- The repeat Versatile Performance grant stays named-but-unproven -----

#[test]
fn bard_level10_does_not_fabricate_versatile_performance() {
    let input = load(BARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("versatile")),
        "level-10 Bard must not fabricate any versatile-performance record (the level-10 \
         repeat grant is the same choice-gated skill-substitution feature left unproven at \
         2nd/6th): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("versatile")),
        "level-10 Bard must not fabricate any versatile-performance diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-9 fixture is unaffected by this widening -----

#[test]
fn bard_level9_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 23, "Bard level 9 performance rounds must stay 23");

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(count.value, 3, "Bard level 9 Fascinate count must stay 3");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == JACK_OF_ALL_TRADES_ID),
        "level-9 Bard must NOT gain the Jack-of-All-Trades record — it is a 10th-level \
         feature: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
//
// This boundary was originally level 11 (the tranche-2 ceiling at the time
// this test was written), then moved to level 12 by the SD18
// bard-level11-inspire-widening cycle, then to level 13 by the SD18
// bard-level12-widening cycle, then progressively further by every
// subsequent SD18 bard level cycle. The SD18 bard-level20-widening cycle
// widened `supported_bard_level` again to `1..=20` (see
// `tests/sd18_bard_level20_widening.rs`) — the final remaining level within
// PF1's 1-20 character-level cap for this class row — so the correct
// negative control boundary for this file's own (level-10-era) baseline is
// now level 21 (a pure implementation-gate check, since PF1 has no 21st
// character level), mirroring exactly how every prior sibling class's own
// level cycle moved its own sibling negative control's boundary up by one
// level.

#[test]
fn bard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARD_LEVEL10_FIXTURE.replace("class:bard:10", "class:bard:21");
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
fn fighter_does_not_gain_bard_level10_recognition() {
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
fn multiclass_bard_level10_is_not_promoted_by_this_slice() {
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

// ----- Control plane: the matrix note names the level-10 widening -----

#[test]
fn matrix_bard_row_names_level_10_widening() {
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
        bard.grounding_ref.contains("sd13_bard_level10_progression"),
        "bard row must cite the live SD13-E5 level-10 proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "bard partial note must name the level-10 widening: {note}"
    );
}
