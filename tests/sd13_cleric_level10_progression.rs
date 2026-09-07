//! SD13-E5 Cleric level-10 progression grounding proof.
//!
//! Widens the accepted Cleric level-1..level-9 prepared-divine baseline (most
//! recently `tests/sd13_cleric_level9_progression.rs`) to Cleric level 10 —
//! the tranche's declared ceiling — mirroring the sibling-class
//! level-range-gate idiom (`supported_cleric_level` is generalized from
//! `1..=9` to `1..=10` via `MAX_SUPPORTED_CLERIC_LEVEL = 10`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Cleric class table and
//! spells-per-day table) were read directly before writing any code or test:
//!
//! - level 10 base attack bonus is +7 (`10 * 3 / 4 = 7`, genuinely risen
//!   from +6 at level 9 — the table's own "+7/+2" iterative notation is not
//!   modeled anywhere in this codebase, only the flat base value) and base
//!   saves are +7 Fortitude and +7 Will (both good, `10 / 2 + 2 = 7`, both
//!   genuinely risen from +6) and +3 Reflex (poor, `10 / 3 = 3`,
//!   numerically unchanged from level 9, an integer-division coincidence) —
//!   confirmed by the same formulas already grounded at levels 1-9, not
//!   re-derived.
//! - the PF1 Core Rulebook Cleric class table's level-10 "Special" column
//!   is genuinely BLANK (verified independently against both primary
//!   sources, checked rather than assumed away) — Channel Energy's die-count
//!   rises land at odd levels, so no new class feature is gained at 10th and
//!   this slice grounds no new pillar record.
//! - Channel Energy's die count STAYS 5 (`(10 + 1) / 2 = 5`, its next rise
//!   landing at 11th) and its uses-per-day pool stays the level-independent
//!   3 + Charisma modifier (5).
//! - the domain spell slot count STAYS 5 (the level-10 spells-per-day row
//!   is "4/4+1/4+1/3+1/3+1/2+1" with the 6th-level column still "—" —
//!   6th-level cleric spells first appear at 11th, checked rather than
//!   assumed).
//! - Touch of Good's bonus GENUINELY RISES to 5 (`10 / 2`, up from 4 at
//!   levels 8-9, via the same half-cleric-level formula); its uses-per-day
//!   pool and Rebuke Death's uses-per-day pool both stay the
//!   level-independent 3 + Wisdom modifier (6); the domain choice
//!   recognitions (Good, Healing) are not level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden or the
//! prepared divine spell posture burden (both stay named-but-unproven,
//! unchanged from levels 1-9), and it does not ground Cleric level 11+. It
//! also preserves the accepted Cleric level-1..level-9 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level9_sd13_deterministic_input.txt");

const CLERIC_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and saves at level 10 -----

#[test]
fn cleric_level10_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Cleric level 10 3/4-BAB progression (10 * 3 / 4) must equal 7, genuinely risen from \
         6 at level 9: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 7,
        "Cleric level 10 good Fortitude (10/2+2) must equal 7, genuinely risen from 6"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 3, "Cleric level 10 poor Reflex (10/3) must equal 3");

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 7,
        "Cleric level 10 good Will (10/2+2) must equal 7, genuinely risen from 6"
    );
}

// ----- Channel Energy stays 5d6 / 5 uses at level 10 -----

#[test]
fn cleric_level10_channel_energy_stays_at_level9_values() {
    let input = load(CLERIC_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 5,
        "Cleric level 10 Channel Energy die count ((10 + 1) / 2) must stay 5 — the odd-level \
         cadence puts the next rise at 11th: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 10 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );
}

// ----- Domain spell slot count stays 5 at level 10 -----

#[test]
fn cleric_level10_domain_spell_slot_count_stays_five() {
    let input = load(CLERIC_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 5,
        "Cleric level 10 domain spell slot count must stay 5 — 6th-level cleric spells first \
         appear at 11th per both primary sources' spells-per-day tables (the level-10 row's \
         6th-level column is still \"—\"): {}",
        slot.detail
    );
}

// ----- Touch of Good genuinely rises to 5; other domain facets carry over -----

#[test]
fn cleric_level10_touch_of_good_rises_and_other_facets_carry_over() {
    let input = load(CLERIC_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 5,
        "Touch of Good's bonus (10 / 2) must rise to 5 at level 10, genuinely risen from 4 at \
         levels 8-9: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(
        tog_uses.value, 7,
        "Touch of Good's uses per day (3 + Wisdom modifier 4) must stay 7, level-independent"
    );

    let rd_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(
        rd_uses.value, 7,
        "Rebuke Death's uses per day (3 + Wisdom modifier 4) must stay 7, level-independent"
    );

    let choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(
        choice.value, 0,
        "domain choice recognition must carry no fabricated mechanical value at level 10"
    );
}

// ----- The two existing burden diagnostics still fire at level 10 -----

#[test]
fn cleric_level10_still_claim_blocks_domain_power_and_prepared_divine_burdens() {
    let input = load(CLERIC_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking
        ),
        "level-10 Cleric must still claim-block on the domain-power execution burden: {:?}",
        computation.diagnostics
    );
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.cleric.prepared_divine.unsupported")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let prepared_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.cleric.daily_preparation")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                prepared_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-9 fixture is unaffected by this widening -----

#[test]
fn cleric_level9_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 6, "Cleric level 9 base attack bonus must stay 6");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 4, "Cleric level 9 Touch of Good bonus must stay 4");

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 6, "Cleric level 9 good Fortitude must stay 6");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (widened from level 11 by the SD18 cycle-2026-07-13T2007 level-11
// widening, then moved to level 12, then level 13, then level 14, then level
// 15 by later cycles' own boundary moves — see
// tests/sd18_cleric_level11_widening.rs, tests/sd18_cleric_level12_widening.rs,
// tests/sd18_cleric_level13_widening.rs, and
// tests/sd18_cleric_level14_widening.rs. The SD18 cycle-2026-07-15T3100
// level-15 widening genuinely promotes level 15 — see
// tests/sd18_cleric_level15_widening.rs — so the correct negative control
// boundary for this file's own (level-10-era) baseline moved to level 17 by
// cycle-2026-07-15T5300, mirroring the exact same boundary move
// cycle-2026-07-15T3000 made for Fighter; cycle-2026-07-15T9600 moved this
// boundary again, from 17 to 18, since level 17 was then itself Cleric's
// supported/grounded row; cycle-2026-07-15T14300 moved this boundary again,
// from 18 to 19, since level 18 was then itself Cleric's supported/grounded
// row; cycle-2026-07-16T1100 moved this boundary again, from 19 to 20, since
// level 19 was then itself Cleric's supported/grounded row;
// cycle-2026-07-16T0844 moves this boundary again, from 20 to 21, since
// level 20 is now itself Cleric's supported/grounded row — and the final
// level within PF1's 1-20 character-level cap, so this boundary check is now
// a pure implementation-gate check with no further real level to move to.)

#[test]
fn cleric_level_21_is_not_promoted_by_this_slice() {
    let level_21 = CLERIC_LEVEL10_FIXTURE.replace("class:cleric:10", "class:cleric:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")
                || e.id == "class_chassis.spell_baseline.cleric"),
        "level-21 Cleric must not gain any bounded cleric chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level10_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")),
        "the Fighter chassis must not surface any cleric-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Cleric is not promoted -----

#[test]
fn multiclass_cleric_level10_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL10_FIXTURE.replace(
        "class_level=class:cleric:10",
        "class_level=class:cleric:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")),
        "multiclass Cleric must not gain any bounded cleric chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Cleric must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-10 widening -----

#[test]
fn matrix_cleric_row_names_level_10_widening() {
    let matrix = seeded_current_truth();
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric progression_and_spell_burden row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class Progression
    // Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(cleric.support_state, SupportState::Supported);
    assert_eq!(cleric.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        cleric.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        cleric.grounding_ref.contains("sd13_cleric_level10_progression"),
        "cleric row must cite the live SD13-E5 level-10 proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "cleric partial note must name the level-10 widening: {note}"
    );
}
