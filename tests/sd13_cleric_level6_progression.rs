//! SD13-E5 Cleric level-6 progression grounding proof.
//!
//! Widens the accepted Cleric level-1/level-2/level-3/level-4/level-5 prepared divine
//! spell-bearing baseline (`tests/sd13_cleric_level1_spell_baseline.rs`,
//! `tests/sd13_cleric_base_attack_and_saves.rs`, `tests/sd13_cleric_domain_powers.rs`,
//! `tests/sd13_cleric_level2_progression.rs`, `tests/sd13_cleric_level3_progression.rs`,
//! `tests/sd13_cleric_level4_progression.rs`, `tests/sd13_cleric_level5_progression.rs`)
//! to Cleric level 6, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_cleric_level` is generalized from `1..=5`
//! to `1..=6` via `MAX_SUPPORTED_CLERIC_LEVEL = 6`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Cleric class table, and the Cleric
//! spells-per-day table) were read directly before writing any code or test:
//!
//! - level 6 base attack bonus is +4 (`6 * 3 / 4 = 4`, the Cleric's own
//!   3/4-BAB progression, the same shape as Rogue/Monk/Druid) and base saves
//!   are +5 Fortitude (good, `6/2+2 = 5`), +2 Reflex (poor, `6/3 = 2`), +5
//!   Will (good, `6/2+2 = 5`) — confirmed by the same formulas already
//!   grounded at levels 1-5, not re-derived.
//! - Channel Energy's die count stays 3d6 at level 6: both primary sources
//!   confirm the die count rises every odd cleric level (1d6 at 1st, 2d6 at
//!   3rd, 3d6 at 5th, and so on), so the pre-existing `ceil(cleric level / 2)`
//!   formula confirms this without re-derivation: `ceil(6 / 2) = 3`, unchanged
//!   from level 5 (it next increases only at level 7).
//! - Channel Energy's uses-per-day count (3 + Charisma modifier) is
//!   level-independent and unchanged at level 6.
//! - the domain choice seam still fires at level 6 for the same fixture
//!   selections (Good, Healing), not level-gated.
//! - the domain spell slot count stays 3 at level 6: the raw Cleric
//!   spells-per-day table's level-6 row (verified independently against both
//!   primary sources) still shows "—" in the 4th-level spell column (only the
//!   0th/1st/2nd/3rd-level columns are populated), so a level-6 cleric does
//!   NOT yet cast 4th-level cleric spells — the domain spell slot count
//!   (PF1 Core Rulebook Domains: "one domain spell slot per level of cleric
//!   spells she can cast, 1st and up") stays exactly 3 (one 1st-level, one
//!   2nd-level, one 3rd-level domain slot), unchanged from level 5, via the
//!   same pre-existing formula, not re-derived.
//! - the Good domain's Touch of Good sacred bonus genuinely CHANGES at level
//!   6: half cleric level, minimum 1 (`max(6/2, 1) = 3`, integer division),
//!   up from 2 at level 5 — verified independently against the PF1 Core
//!   Rulebook Good Domain granted-power rule text — via the same pre-existing
//!   formula, not re-derived.
//! - both domain powers' uses-per-day counts (3 + Wisdom modifier, for Touch
//!   of Good and Rebuke Death alike) are level-independent and unchanged at
//!   level 6.
//! - the Cleric class table's level-6 "Special" column is genuinely blank
//!   (verified independently against both primary sources: no new Cleric
//!   class feature is gained at 6th level), so this slice adds no new pillar
//!   record for level 6 — only the Touch of Good pillar is widened to a
//!   genuinely new value; every other pillar stays numerically unchanged.
//!
//! It deliberately does not touch domain spell-list contents, the prepared
//! divine spell posture burden, or the Rebuke Death heal amount (all three
//! stay named-but-unproven, unchanged from levels 1-5), and it does not
//! ground Cleric level 7+. It also preserves the accepted Cleric level-1/
//! level-2/level-3/level-4/level-5 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const CLERIC_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level5_sd13_deterministic_input.txt");

const CLERIC_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level6_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 6 -----

#[test]
fn cleric_level6_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 4,
        "Cleric level 6 3/4-BAB progression (6 * 3 / 4) must equal 4: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 6 (good Fortitude/Will, poor Reflex) -----

#[test]
fn cleric_level6_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 5, "Cleric level 6 good Fortitude (6/2+2) must equal 5");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 2, "Cleric level 6 poor Reflex (6/3) must equal 2");

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(will.value, 5, "Cleric level 6 good Will (6/2+2) must equal 5");
}

// ----- Channel Energy dice stays 3d6 at level 6 -----

#[test]
fn cleric_level6_channel_energy_dice_stays_three_d6() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 3,
        "Cleric level 6 Channel Energy die count must stay 3 (i.e. 3d6), unchanged from level \
         5 (it next increases only at level 7): {}",
        dice.detail
    );
}

#[test]
fn cleric_level6_channel_energy_uses_per_day_is_unchanged() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 6 Channel Energy uses per day (3 + Charisma modifier 2) must equal 5: {}",
        uses.detail
    );
}

// ----- Domain choice seam still fires; domain spell slot count stays 3 -----

#[test]
fn cleric_level6_domain_choice_is_recognized() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.cleric.domain_choice"),
        "level-6 Cleric must still recognize the domain choice seam: {:?}",
        computation.explanations
    );
}

#[test]
fn cleric_level6_domain_spell_slot_count_stays_three() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 3,
        "Cleric level 6 domain spell slot count must stay 3 (the raw spells-per-day table's \
         4th-level spell column is still \"—\" at level 6, so a level-6 cleric does not yet \
         cast 4th-level cleric spells), unchanged from level 5: {}",
        slot.detail
    );
}

// ----- Domain powers at level 6: Touch of Good genuinely rises to 3, Rebuke Death unchanged -----

#[test]
fn cleric_level6_touch_of_good_bonus_rises_to_three() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 3,
        "Cleric level 6 Touch of Good sacred bonus (half cleric level, minimum 1: max(3,1)) \
         must genuinely rise to 3, up from 2 at level 5: {}",
        bonus.detail
    );

    let uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 6 Touch of Good uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

#[test]
fn cleric_level6_grounds_rebuke_death_uses_per_day() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 6 Rebuke Death uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 6 -----

#[test]
fn cleric_level6_still_claim_blocks_domain_powers_and_prepared_divine_burdens() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-6 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- The chassis recognition record is still present at level 6 -----

#[test]
fn cleric_level6_still_recognizes_the_spell_bearing_baseline() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.cleric"),
        "level-6 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Negative control (superseded): level 7 was later widened into the
// supported tranche by `tests/sd13_cleric_level7_progression.rs`, whose new
// level-8 negative control now carries the "next level stays unrecognized"
// coverage. -----

#[test]
fn cleric_level_7_was_later_widened_into_the_supported_tranche() {
    let level_7 = CLERIC_LEVEL6_FIXTURE.replace("class:cleric:6", "class:cleric:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")),
        "level-7 Cleric was later widened into the supported tranche and must now gain \
         bounded cleric chassis explanations: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-5 fixture is unaffected by this widening -----

#[test]
fn cleric_level5_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Cleric level 5 base attack bonus must stay 3");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 3, "Cleric level 5 Channel Energy die count must stay 3");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 3,
        "Cleric level 5 domain spell slot count must stay 3, unaffected by the level-6 widening"
    );

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 2,
        "Cleric level 5 Touch of Good bonus must stay 2, unaffected by the level-6 widening"
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level6_recognition() {
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
fn multiclass_cleric_level6_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL6_FIXTURE.replace(
        "class_level=class:cleric:6",
        "class_level=class:cleric:6\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-6 widening -----

#[test]
fn matrix_cleric_row_names_level_6_widening_and_touch_of_good_increase() {
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
        cleric.grounding_ref.contains("sd13_cleric_level6_progression"),
        "cleric row must cite the live SD13-E5 level-6 proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 6") || note.contains("level-6"),
        "cleric partial note must name the level-6 widening: {note}"
    );
    assert!(
        note.contains("Touch of Good") || note.contains("touch of good"),
        "cleric partial note must name the level-6 Touch of Good increase: {note}"
    );
}
