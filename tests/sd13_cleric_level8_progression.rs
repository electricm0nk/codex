//! SD13-E5 Cleric level-8 progression grounding proof.
//!
//! Widens the accepted Cleric level-1/level-2/level-3/level-4/level-5/level-6/
//! level-7 prepared divine spell-bearing baseline
//! (`tests/sd13_cleric_level1_spell_baseline.rs`,
//! `tests/sd13_cleric_base_attack_and_saves.rs`, `tests/sd13_cleric_domain_powers.rs`,
//! `tests/sd13_cleric_level2_progression.rs`, `tests/sd13_cleric_level3_progression.rs`,
//! `tests/sd13_cleric_level4_progression.rs`, `tests/sd13_cleric_level5_progression.rs`,
//! `tests/sd13_cleric_level6_progression.rs`, `tests/sd13_cleric_level7_progression.rs`)
//! to Cleric level 8, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_cleric_level` is generalized from `1..=7`
//! to `1..=8` via `MAX_SUPPORTED_CLERIC_LEVEL = 8`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Cleric class table, and the Cleric
//! spells-per-day table) were read directly before writing any code or test:
//!
//! - level 8 base attack bonus is +6 (`8 * 3 / 4 = 6`, the Cleric's own
//!   3/4-BAB progression, the same shape as Rogue/Monk/Druid — the class
//!   table's own iterative-attack notation "+6/+1" is not modeled anywhere in
//!   this codebase, only the flat base value) and base saves are +6 Fortitude
//!   (good, `8/2+2 = 6`), +2 Reflex (poor, `8/3 = 2`), +6 Will (good,
//!   `8/2+2 = 6`) — confirmed by the same formulas already grounded at levels
//!   1-7, not re-derived.
//! - Channel Energy's die count stays 4d6 at level 8: both primary sources
//!   confirm the die count rises only every odd cleric level (1st, 3rd, 5th,
//!   7th, 9th, ...), so level 8 is not one of those levels; the pre-existing
//!   `ceil(cleric level / 2)` formula confirms this without re-derivation:
//!   `ceil(8 / 2) = 4`, unchanged from level 7.
//! - Channel Energy's uses-per-day count (3 + Charisma modifier) is
//!   level-independent and unchanged at level 8.
//! - the domain choice seam still fires at level 8 for the same fixture
//!   selections (Good, Healing), not level-gated.
//! - the domain spell slot count stays 4 at level 8: the raw Cleric
//!   spells-per-day table's level-8 row (verified independently against both
//!   primary sources) still shows "—" in the 5th-level spell column — a
//!   level-8 cleric does not cast 5th-level cleric spells for the first time
//!   until level 9 — so the flat domain spell slot count (one 1st-level, one
//!   2nd-level, one 3rd-level, one 4th-level domain slot) stays unchanged from
//!   level 7, via the same pre-existing formula, not re-derived.
//! - the Good domain's Touch of Good sacred bonus GENUINELY increases to 4 at
//!   level 8: half cleric level, minimum 1 (`max(8/2, 1) = 4`, integer
//!   division), up from 3 at level 7 — verified independently against the PF1
//!   Core Rulebook Good Domain granted-power rule text — via the same
//!   pre-existing formula, not re-derived.
//! - both domain powers' uses-per-day counts (3 + Wisdom modifier, for Touch
//!   of Good and Rebuke Death alike) are level-independent and unchanged at
//!   level 8.
//! - the Cleric class table's level-8 "Special" column is genuinely blank
//!   (verified independently against both primary sources) — no new Cleric
//!   class feature is gained at 8th level — so this slice adds no new pillar
//!   record for level 8 beyond widening the Touch of Good bonus pillar to a
//!   genuinely new value; every other pillar stays numerically unchanged from
//!   level 7 via the same formulas.
//!
//! It deliberately does not touch domain spell-list contents, the prepared
//! divine spell posture burden, or the Rebuke Death heal amount (all three
//! stay named-but-unproven, unchanged from levels 1-7), and it does not
//! ground Cleric level 9+. It also preserves the accepted Cleric level-1/
//! level-2/level-3/level-4/level-5/level-6/level-7 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.
//!
//! The prior sibling file's negative control naming level 8 as unrecognized
//! has been renamed and flipped in `tests/sd13_cleric_level7_progression.rs` to
//! `cleric_level_8_was_later_widened_into_the_supported_tranche`, whose coverage
//! moves here now that level 8 is itself promoted into the supported tranche.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const CLERIC_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level7_sd13_deterministic_input.txt");

const CLERIC_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level8_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 8 -----

#[test]
fn cleric_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Cleric level 8 3/4-BAB progression (8 * 3 / 4) must equal 6: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 (good Fortitude/Will, poor Reflex) -----

#[test]
fn cleric_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 6, "Cleric level 8 good Fortitude (8/2+2) must equal 6");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 2, "Cleric level 8 poor Reflex (8/3) must equal 2");

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(will.value, 6, "Cleric level 8 good Will (8/2+2) must equal 6");
}

// ----- Channel Energy dice stays at 4d6 at level 8 -----

#[test]
fn cleric_level8_channel_energy_dice_stays_four_d6() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 4,
        "Cleric level 8 Channel Energy die count must stay 4 (i.e. 4d6), unchanged from level 7 \
         (the die count only rises at odd cleric levels): {}",
        dice.detail
    );
}

#[test]
fn cleric_level8_channel_energy_uses_per_day_is_unchanged() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 8 Channel Energy uses per day (3 + Charisma modifier 2) must equal 5: {}",
        uses.detail
    );
}

// ----- Domain choice seam still fires; domain spell slot count stays 4 -----

#[test]
fn cleric_level8_domain_choice_is_recognized() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.cleric.domain_choice"),
        "level-8 Cleric must still recognize the domain choice seam: {:?}",
        computation.explanations
    );
}

#[test]
fn cleric_level8_domain_spell_slot_count_stays_four() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 4,
        "Cleric level 8 domain spell slot count must stay 4, unchanged from level 7 (the raw \
         spells-per-day table's 5th-level spell column is still \"—\" at level 8; 5th-level \
         cleric spells do not begin until level 9): {}",
        slot.detail
    );
}

// ----- Domain powers at level 8: Touch of Good genuinely rises, Rebuke Death unchanged -----

#[test]
fn cleric_level8_touch_of_good_bonus_rises_to_four() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 4,
        "Cleric level 8 Touch of Good sacred bonus (half cleric level, minimum 1: max(4,1)) \
         must genuinely rise to 4, up from 3 at level 7: {}",
        bonus.detail
    );

    let uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 8 Touch of Good uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

#[test]
fn cleric_level8_grounds_rebuke_death_uses_per_day() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 8 Rebuke Death uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 8 -----

#[test]
fn cleric_level8_still_claim_blocks_domain_powers_and_prepared_divine_burdens() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-8 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- The chassis recognition record is still present at level 8 -----

#[test]
fn cleric_level8_still_recognizes_the_spell_bearing_baseline() {
    let input = load(CLERIC_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.cleric"),
        "level-8 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn cleric_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = CLERIC_LEVEL8_FIXTURE.replace("class:cleric:8", "class:cleric:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")),
        "level-9 Cleric is now recognized by the later level-9 widening slice \
         (tests/sd13_cleric_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-7 fixture is unaffected by this widening -----

#[test]
fn cleric_level7_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 5, "Cleric level 7 base attack bonus must stay 5");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 4, "Cleric level 7 Channel Energy die count must stay 4");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 4,
        "Cleric level 7 domain spell slot count must stay 4, unaffected by the level-8 widening"
    );

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 3,
        "Cleric level 7 Touch of Good bonus must stay 3, unaffected by the level-8 widening"
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level8_recognition() {
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
fn multiclass_cleric_level8_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL8_FIXTURE.replace(
        "class_level=class:cleric:8",
        "class_level=class:cleric:8\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_cleric_row_names_level_8_widening_and_touch_of_good_increase() {
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
        cleric.grounding_ref.contains("sd13_cleric_level8_progression"),
        "cleric row must cite the live SD13-E5 level-8 proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 8") || note.contains("level-8"),
        "cleric partial note must name the level-8 widening: {note}"
    );
    assert!(
        note.contains("Touch of Good") || note.contains("touch of good"),
        "cleric partial note must name the level-8 Touch of Good bonus increase: {note}"
    );
}
