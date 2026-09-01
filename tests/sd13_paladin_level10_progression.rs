//! SD13-E5 Paladin level-10 progression grounding proof — the level-10 band
//! closer: with this slice every one of the ten level-banded class rows is
//! grounded through level 10, the tranche ceiling.
//!
//! Widens the accepted Paladin level-1..level-9 hybrid chassis baseline
//! (most recently `tests/sd13_paladin_level9_progression.rs`) to Paladin
//! level 10, mirroring the sibling-class level-range-gate idiom
//! (`supported_paladin_level` is generalized from `1..=9` to `1..=10` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 10`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Paladin class table) were read directly
//! before writing any code or test:
//!
//! - level 10 base attack bonus is +10 (full BAB, genuinely risen from +9 —
//!   the table's own "+10/+5" iterative notation is not modeled anywhere in
//!   this codebase, only the flat base value) and base saves are +7
//!   Fortitude and +7 Will (both good, `10 / 2 + 2 = 7`, both genuinely
//!   risen from +6) and +3 Reflex (poor, `10 / 3 = 3`, numerically unchanged
//!   from level 9, an integer-division coincidence) — confirmed by the same
//!   formulas already grounded at levels 1-9, not re-derived.
//! - the PF1 Core Rulebook Paladin class table's level-10 "Special" column
//!   reads "Smite evil 4/day" only (verified independently against both
//!   primary sources, checked rather than assumed away) — Smite Evil's
//!   uses per day GENUINELY RISE to 4 via the already-grounded threshold
//!   formula (`1 + (10 - 1) / 3 = 4`), its damage bonus genuinely rises to
//!   10 (equal to paladin level), and its attack bonus stays the flat
//!   Charisma modifier (+2). No new named feature appears at 10th, so
//!   nothing new is left unproven by this slice.
//! - Lay on Hands GENUINELY RISES on both axes: uses per day to 7
//!   (`10 / 2 + 2` Charisma modifier) and heal dice count to 5 (`10 / 2`);
//!   Divine Grace stays the flat Charisma-modifier save bonus (+2).
//! - the partial-caster effective caster level genuinely rises to 7
//!   (`10 - 3`), while the partial-caster spell burden itself stays
//!   claim-blocked.
//! - Channel Positive Energy's dice count stays 5 (the paladin channels as
//!   an effective cleric of her paladin level; a cleric's channel dice rise
//!   at odd levels, so the next rise lands at 11th — a threshold stasis
//!   checked rather than assumed).
//! - 10th is NOT a repeat-Mercy-grant level (the 3rd/6th/9th cadence); the
//!   single grounded level-3 selection (mercy:shaken) carries over
//!   unchanged, and its recognitions still fire.
//!
//! It deliberately does not touch the mercy-effect resolution, channel
//! execution, Divine Bond, Aura of Resolve/Courage/Divine Health, or the
//! partial-caster spell posture burden (all stay named-but-unproven,
//! unchanged from levels 1-9), and it does not ground Paladin level 11+. It
//! also preserves the accepted Paladin level-1..level-9 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const PALADIN_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level9_sd13_deterministic_input.txt");

const PALADIN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BASE_ATTACK_ID: &str = "class_chassis.paladin.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.paladin.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.paladin.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.paladin.base_save.will";
const SMITE_EVIL_USES_PER_DAY_ID: &str = "class_chassis.paladin.smite_evil_uses_per_day";
const SMITE_EVIL_ATTACK_BONUS_ID: &str = "class_chassis.paladin.smite_evil_attack_bonus";
const SMITE_EVIL_DAMAGE_BONUS_ID: &str = "class_chassis.paladin.smite_evil_damage_bonus";
const LAY_ON_HANDS_USES_PER_DAY_ID: &str = "class_chassis.paladin.lay_on_hands_uses_per_day";
const LAY_ON_HANDS_HEAL_AMOUNT_ID: &str = "class_chassis.paladin.lay_on_hands_heal_amount";
const DIVINE_GRACE_SAVE_BONUS_ID: &str = "class_chassis.paladin.divine_grace_save_bonus";
const EFFECTIVE_CASTER_LEVEL_ID: &str =
    "class_chassis.paladin.partial_caster.effective_caster_level";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";
const MERCY_GRANTED_ID: &str = "class_chassis.paladin.mercy_granted";
const MERCY_CHOICE_ID: &str = "class_chassis.paladin.mercy_choice";
const CHANNEL_POSITIVE_ENERGY_DICE_ID: &str =
    "class_chassis.paladin.channel_positive_energy_dice";

// ----- Base attack bonus and saves at level 10 -----

#[test]
fn paladin_level10_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 10,
        "Paladin level 10 full-BAB progression must equal 10, genuinely risen from 9: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 7,
        "Paladin level 10 good Fortitude (10/2+2) must equal 7, genuinely risen from 6"
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 3,
        "Paladin level 10 poor Reflex (10/3) must stay 3 — an integer-division coincidence \
         with level 9"
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 7,
        "Paladin level 10 good Will (10/2+2) must equal 7, genuinely risen from 6"
    );
}

// ----- Smite Evil at level 10: uses genuinely rise to 4, damage to 10 -----

#[test]
fn paladin_level10_smite_evil_uses_rise_to_four_and_damage_rises_to_ten() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 4,
        "Paladin level 10 Smite Evil must genuinely rise to 4/day (1 + (10 - 1)/3), matching \
         the CRB level-10 \"Special\" column \"Smite evil 4/day\": {}",
        uses_per_day.detail
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let attack_bonus = explanation(&computation, SMITE_EVIL_ATTACK_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 3,
        "Paladin level 10 Smite Evil attack bonus must stay the flat Charisma modifier (+3)"
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 10,
        "Paladin level 10 Smite Evil damage bonus (equal to paladin level) must equal 10, \
         genuinely risen from 9: {}",
        damage_bonus.detail
    );
}

// ----- Lay on Hands genuinely rises on both axes; Divine Grace carries over -----

#[test]
fn paladin_level10_lay_on_hands_rises_and_divine_grace_carries_over() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 8,
        "Paladin level 10 Lay on Hands uses per day (10/2 + Charisma modifier 3) must \
         genuinely rise to 8: {}",
        uses_per_day.detail
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 5,
        "Paladin level 10 Lay on Hands heal dice count (10/2 d6) must genuinely rise to 5: {}",
        heal_amount.detail
    );

    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "Paladin Divine Grace must stay the flat Charisma-modifier save bonus (+3) at level 10"
    );
}

// ----- Partial-caster effective caster level genuinely rises to 7 -----

#[test]
fn paladin_level10_effective_caster_level_rises_to_seven() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 7,
        "Paladin level 10 effective caster level (10 - 3) must equal 7, genuinely risen from \
         6: {}",
        caster_level.detail
    );

    // (v0.6 alpha swarm, risks item 8, 2026-07-25) `PARTIAL_CASTER_BLOCKER_ID`
    // is no longer unconditional: it's a real, conditional validation of
    // AcquisitionMode::Prepared selections. This fixture predates
    // spells_selected (zero prepared), so the posture is genuinely valid and
    // the blocker correctly does not fire -- the real "no spell slots are
    // fabricated" guarantee now comes from the daily-preparation record's own
    // count being honestly 0.
    match computation.diagnostics.iter().find(|d| d.id == PARTIAL_CASTER_BLOCKER_ID) {
        Some(spell_blocker) => assert!(
            spell_blocker.claim_blocking,
            "if the spell blocker fires at all, it must be claim-blocking"
        ),
        None => {
            let daily_prep = explanation(&computation, "class_spell.paladin.daily_preparation");
            assert_eq!(
                daily_prep.value, 0,
                "no spells are fabricated at paladin level 10: {daily_prep:?}"
            );
        }
    }
}

// ----- Channel Positive Energy dice stay 5 at level 10 (rise lands at 11th) -----

#[test]
fn paladin_level10_channel_positive_energy_dice_stay_five() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 5,
        "Paladin level 10 Channel Positive Energy (as an effective cleric of paladin level \
         10) must stay 5d6 — a cleric's channel dice rise at odd levels, so the next rise \
         lands at 11th: {}",
        dice.detail
    );
}

// ----- Mercy: single grounded selection carries over at level 10 -----

#[test]
fn paladin_level10_mercy_recognitions_carry_over_unchanged() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy-granted recognition must carry no fabricated mechanical value at level 10"
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "mercy-choice recognition must carry no fabricated mechanical value at level 10"
    );
    assert!(
        choice.detail.contains("shaken"),
        "mercy-choice recognition must still name the single grounded shaken selection at \
         level 10 (10th is not a repeat-Mercy-grant level): {}",
        choice.detail
    );
}

// ----- Negative control: the level-9 fixture is unaffected by this widening -----

#[test]
fn paladin_level9_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 9, "Paladin level 9 base attack bonus must stay 9");

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(uses_per_day.value, 3, "Paladin level 9 Smite Evil must stay 3/day");

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 6, "Paladin level 9 good Fortitude must stay 6");

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(caster_level.value, 6, "Paladin level 9 effective caster level must stay 6");
}

// ----- Negative control: level 14 stays unrecognized by this slice -----
// (level 11 was later widened into the supported tranche by SD18's
// cycle-2026-07-13T2334 Aura of Justice slice, level 12 by SD18's
// cycle-2026-07-15T0700 widening slice, level 13 by SD18's
// cycle-2026-07-15T1800 widening slice, level 14 by SD18's
// cycle-2026-07-15T2500 widening slice, level 15 by SD18's
// cycle-2026-07-15T4300 widening slice, level 16 by SD18's
// cycle-2026-07-15T5400 widening slice, and level 17 by SD18's
// cycle-2026-07-15T10700 widening slice; see
// tests/sd18_paladin_level11_aura_of_justice.rs,
// tests/sd18_paladin_level12_widening.rs,
// tests/sd18_paladin_level13_widening.rs,
// tests/sd18_paladin_level14_widening.rs,
// tests/sd18_paladin_level15_widening.rs,
// tests/sd18_paladin_level16_widening.rs, and
// tests/sd18_paladin_level17_widening.rs for their own boundaries.)

#[test]
fn paladin_level_21_is_not_promoted_by_this_slice() {
    let level_21 = PALADIN_LEVEL10_FIXTURE.replace("class:paladin:10", "class:paladin:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "level-21 Paladin must not gain any bounded paladin chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the paladin path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_level10_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "the Fighter chassis must not surface any paladin-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Paladin is not promoted -----

#[test]
fn multiclass_paladin_level10_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL10_FIXTURE.replace(
        "class_level=class:paladin:10",
        "class_level=class:paladin:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "multiclass Paladin must not gain any bounded paladin chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-10 widening -----

#[test]
fn matrix_paladin_row_names_level_10_widening() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_paladin_level10_progression"),
        "paladin row must cite the live SD13-E5 level-10 proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "paladin partial note must name the level-10 widening: {note}"
    );
}
