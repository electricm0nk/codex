//! SD13-E5 Cleric level-9 progression grounding proof.
//!
//! Widens the accepted Cleric level-1..level-8 prepared-divine baseline (most
//! recently `tests/sd13_cleric_level8_progression.rs`) to Cleric level 9,
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_cleric_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_CLERIC_LEVEL = 9`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Cleric class table and spells-per-day table) were
//! read directly before writing any code or test:
//!
//! - level 9 base attack bonus is +6 (`9 * 3 / 4 = 6`, the Cleric's 3/4-BAB
//!   progression, numerically unchanged from level 8 — an integer-division
//!   coincidence; the table's own "+6/+1" iterative notation is not modeled
//!   anywhere in this codebase, only the flat base value) and base saves are
//!   +6 Fortitude (good, `9 / 2 + 2 = 6`, unchanged, a coincidence), +3
//!   Reflex (poor, `9 / 3 = 3`, genuinely risen from +2), and +6 Will (good,
//!   `9 / 2 + 2 = 6`, unchanged, a coincidence) — confirmed by the same
//!   formulas already grounded at levels 1-8, not re-derived.
//! - the PF1 Core Rulebook Cleric class table's level-9 "Special" column
//!   reads "Channel energy 5d6" (verified independently against both primary
//!   sources, checked rather than assumed away) — a tier-rise on the
//!   already-grounded Channel Energy die-count pillar (`(9 + 1) / 2 = 5`, up
//!   from 4d6 at levels 7-8, matching the odd-level rise cadence), not a new
//!   class feature. Channel Energy's uses-per-day pool (3 + Charisma
//!   modifier) is level-independent and stays 5.
//! - 5th-level cleric spells first appear at 9th level (the spells-per-day
//!   table's level-9 row is "4/4+1/4+1/3+1/2+1/1+1"), so the domain spell
//!   slot count GENUINELY RISES to 5 via the same
//!   one-slot-per-castable-spell-level rule already grounded (mirroring the
//!   level-7 rise to 4 when 4th-level spells arrived).
//! - the domain choice recognitions (Good, Healing) are not level-gated and
//!   still fire; Touch of Good's bonus stays 4 (`9 / 2`, an
//!   integer-division coincidence with level 8) and its uses-per-day pool
//!   stays 6 (3 + Wisdom modifier 3, level-independent); Rebuke Death's
//!   uses-per-day pool stays 6 (3 + Wisdom modifier 3, level-independent).
//!
//! It deliberately does not touch the domain-power execution burden or the
//! prepared divine spell posture burden (both stay named-but-unproven,
//! unchanged from levels 1-8), and it does not ground Cleric level 10+. It
//! also preserves the accepted Cleric level-1..level-8 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level8_sd13_deterministic_input.txt");

const CLERIC_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level9_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and saves at level 9 -----

#[test]
fn cleric_level9_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Cleric level 9 3/4-BAB progression (9 * 3 / 4) must equal 6 — unchanged from level \
         8, an integer-division coincidence: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 6, "Cleric level 9 good Fortitude (9/2+2) must equal 6");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 3,
        "Cleric level 9 poor Reflex (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(will.value, 6, "Cleric level 9 good Will (9/2+2) must equal 6");
}

// ----- Channel Energy: dice rise to 5d6, uses stay 5 -----

#[test]
fn cleric_level9_channel_energy_dice_rise_to_five() {
    let input = load(CLERIC_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 5,
        "Cleric level 9 Channel Energy die count ((9 + 1) / 2) must equal 5 (5d6), genuinely \
         risen from 4 at levels 7-8, matching the odd-level rise cadence: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 9 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5, \
         level-independent: {}",
        uses.detail
    );
}

// ----- Domain spell slot count genuinely rises to 5 at level 9 -----

#[test]
fn cleric_level9_domain_spell_slot_count_rises_to_five() {
    let input = load(CLERIC_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 5,
        "Cleric level 9 domain spell slot count must rise to 5 — 5th-level cleric spells \
         first appear at 9th per both primary sources' spells-per-day tables, so the \
         one-slot-per-castable-spell-level rule now spans 1st through 5th: {}",
        slot.detail
    );
}

// ----- Domain choice recognition and domain-power flat facets at level 9 -----

#[test]
fn cleric_level9_domain_choice_and_power_facets_carry_over() {
    let input = load(CLERIC_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(
        choice.value, 0,
        "domain choice recognition must carry no fabricated mechanical value at level 9"
    );

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 4,
        "Touch of Good's bonus (9 / 2) must stay 4 — an integer-division coincidence with \
         level 8: {}",
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
}

// ----- The two existing burden diagnostics still fire at level 9 -----

#[test]
fn cleric_level9_still_claim_blocks_domain_power_and_prepared_divine_burdens() {
    let input = load(CLERIC_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking
        ),
        "level-9 Cleric must still claim-block on the domain-power execution burden: {:?}",
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

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn cleric_level8_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 4, "Cleric level 8 Channel Energy die count must stay 4");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 4, "Cleric level 8 domain spell slot count must stay 4");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 2, "Cleric level 8 poor Reflex must stay 2");
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn cleric_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = CLERIC_LEVEL9_FIXTURE.replace("class:cleric:9", "class:cleric:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")),
        "level-10 Cleric is now recognized by the later level-10 widening slice \
         (tests/sd13_cleric_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level9_recognition() {
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
fn multiclass_cleric_level9_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL9_FIXTURE.replace(
        "class_level=class:cleric:9",
        "class_level=class:cleric:9\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_cleric_row_names_level_9_widening() {
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
        cleric.grounding_ref.contains("sd13_cleric_level9_progression"),
        "cleric row must cite the live SD13-E5 level-9 proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "cleric partial note must name the level-9 widening: {note}"
    );
}
