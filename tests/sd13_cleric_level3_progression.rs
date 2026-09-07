//! SD13-E5 Cleric level-3 progression grounding proof.
//!
//! Widens the accepted Cleric level-1/level-2 prepared divine spell-bearing
//! baseline (`tests/sd13_cleric_level1_spell_baseline.rs`,
//! `tests/sd13_cleric_base_attack_and_saves.rs`, `tests/sd13_cleric_domain_powers.rs`,
//! `tests/sd13_cleric_level2_progression.rs`) to Cleric level 3, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_cleric_level` is generalized from `1..=2`
//! to `1..=3` via `MAX_SUPPORTED_CLERIC_LEVEL = 3`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Cleric class table) were read
//! directly before writing any code or test:
//!
//! - level 3 base attack bonus is +2 (`3 * 3 / 4 = 2`, the Cleric's own
//!   3/4-BAB progression, the same shape as Rogue/Monk/Druid) and base saves
//!   are +3 Fortitude (good, `3/2+2 = 3`), +1 Reflex (poor, `3/3 = 1`), +3
//!   Will (good, `3/2+2 = 3`) — confirmed by the same formulas already
//!   grounded at levels 1-2, not re-derived.
//! - Channel Energy's die count CHANGES for real at level 3: the PF1 Core
//!   Rulebook Cleric class table's level-3 "Special" column reads "Channel
//!   energy 2d6" (verified independently against both primary sources), and
//!   the pre-existing `ceil(cleric level / 2)` formula already produces this
//!   for real without re-derivation: `ceil(3 / 2) = 2`, so the die count
//!   becomes 2 (i.e. 2d6), up from 1 at levels 1-2.
//! - Channel Energy's uses-per-day count (3 + Charisma modifier) is
//!   level-independent and unchanged at level 3.
//! - the domain choice seam still fires at level 3 for the same fixture
//!   selections (Good, Healing), not level-gated.
//! - the domain spell slot count CHANGES for real at level 3: the raw Cleric
//!   spells-per-day table rows (verified independently against both primary
//!   sources) show a level-3 cleric casts 2nd-level cleric spells for the
//!   first time (level 2 row: "4/2+1/—", level 3 row: "4/2+1/1+1" — the
//!   first non-"—" 2nd-level column), so "one domain spell slot per level of
//!   cleric spells she can cast, 1st and up" (PF1 Core Rulebook Domains) now
//!   evaluates to two slots — one 1st-level domain slot plus one 2nd-level
//!   domain slot — up from 1 at levels 1-2. This still grounds the flat
//!   count only: no slot contents, no domain spell-list contents, and no
//!   prepared-spell posture are computed.
//! - the Good domain's Touch of Good sacred bonus (half cleric level,
//!   minimum 1) stays 1 at level 3 (`max(3/2, 1) = 1`), reached via the
//!   floor-and-min, not re-derived (it next increases only at level 4,
//!   `4/2 = 2`).
//! - both domain powers' uses-per-day counts (3 + Wisdom modifier, for Touch
//!   of Good and Rebuke Death alike) are level-independent and unchanged at
//!   level 3.
//! - the Cleric class table's level-3 "Special" column names only the
//!   Channel Energy die-count increase (verified independently against both
//!   primary sources: no other new Cleric class feature is gained at 3rd
//!   level, unlike Rogue/Monk/Barbarian's Trap Sense/Still Mind/Trap Sense),
//!   so this slice adds no new pillar record for level 3 — only the existing
//!   pillars are widened (two of them, Channel Energy dice and the domain
//!   spell slot count, widened to genuinely new values rather than staying
//!   flat).
//!
//! It deliberately does not touch domain spell-list contents, the prepared
//! divine spell posture burden, or the Rebuke Death heal amount (all three
//! stay named-but-unproven, unchanged from levels 1-2), and it does not
//! ground Cleric level 4+. It also preserves the accepted Cleric level-1/
//! level-2 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const CLERIC_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level2_sd13_deterministic_input.txt");

const CLERIC_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level3_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 3 -----

#[test]
fn cleric_level3_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 2,
        "Cleric level 3 3/4-BAB progression (3 * 3 / 4) must equal 2: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 3 (good Fortitude/Will, poor Reflex) -----

#[test]
fn cleric_level3_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Cleric level 3 good Fortitude (3/2+2) must equal 3");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 1, "Cleric level 3 poor Reflex (3/3) must equal 1");

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(will.value, 3, "Cleric level 3 good Will (3/2+2) must equal 3");
}

// ----- Channel Energy dice becomes 2d6 at level 3 -----

#[test]
fn cleric_level3_channel_energy_dice_becomes_two_d6() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 2,
        "Cleric level 3 Channel Energy die count must become 2 (i.e. 2d6): {}",
        dice.detail
    );
}

#[test]
fn cleric_level3_channel_energy_uses_per_day_is_unchanged() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 3 Channel Energy uses per day (3 + Charisma modifier 2) must equal 5: {}",
        uses.detail
    );
}

// ----- Domain choice seam still fires; domain spell slot count becomes 2 -----

#[test]
fn cleric_level3_domain_choice_is_recognized() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.cleric.domain_choice"),
        "level-3 Cleric must still recognize the domain choice seam: {:?}",
        computation.explanations
    );
}

#[test]
fn cleric_level3_domain_spell_slot_count_becomes_two() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 2,
        "Cleric level 3 domain spell slot count must become 2 (a level-3 cleric casts 1st AND \
         2nd-level spells for the first time, gaining one domain slot of each): {}",
        slot.detail
    );
}

// ----- Domain powers at level 3: Touch of Good stays 1, Rebuke Death unchanged -----

#[test]
fn cleric_level3_touch_of_good_bonus_stays_one() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 1,
        "Cleric level 3 Touch of Good sacred bonus (half cleric level, minimum 1: max(1,1)) must \
         stay 1: {}",
        bonus.detail
    );

    let uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 3 Touch of Good uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

#[test]
fn cleric_level3_grounds_rebuke_death_uses_per_day() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 3 Rebuke Death uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 3 -----

#[test]
fn cleric_level3_still_claim_blocks_domain_powers_and_prepared_divine_burdens() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-3 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- The chassis recognition record is still present at level 3 -----

#[test]
fn cleric_level3_still_recognizes_the_spell_bearing_baseline() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.cleric"),
        "level-3 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- The accepted Cleric level-2 truth is unaffected -----

#[test]
fn cleric_level2_truth_is_unchanged_by_this_widening() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 1, "Cleric level 2 base attack bonus must stay 1");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 1, "Cleric level 2 Channel Energy die count must stay 1");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 1,
        "Cleric level 2 domain spell slot count must stay 1, unaffected by the level-3 widening"
    );
}

// ----- Negative control (was: level 4 stays unrecognized). Level 4 was later
// widened into the supported tranche by tests/sd13_cleric_level4_progression.rs;
// coverage for that level moved there. -----

#[test]
fn cleric_level_4_was_later_widened_into_the_supported_tranche() {
    let level_4 = CLERIC_LEVEL3_FIXTURE.replace("class:cleric:3", "class:cleric:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.cleric"),
        "level-4 Cleric is now supported by a later SD13-E5 slice \
         (tests/sd13_cleric_level4_progression.rs): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level3_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")
                || e.id == "class_chassis.spell_baseline.cleric"),
        "the Fighter chassis must not surface any cleric-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Cleric is not promoted -----

#[test]
fn multiclass_cleric_level3_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL3_FIXTURE.replace(
        "class_level=class:cleric:3",
        "class_level=class:cleric:3\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")
                || e.id == "class_chassis.spell_baseline.cleric"),
        "multiclass Cleric must not gain any bounded cleric chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Cleric must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-3 widening -----

#[test]
fn matrix_cleric_row_names_level_3_widening() {
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
        cleric.grounding_ref.contains("sd13_cleric_level3_progression"),
        "cleric row must cite the live SD13-E5 level-3 proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 3") || note.contains("level-3"),
        "cleric partial note must name the level-3 widening: {note}"
    );
}
