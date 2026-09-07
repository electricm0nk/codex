//! SD13-E5 Paladin level-9 progression grounding proof — the level-9 band
//! closer: with this slice every one of the ten level-banded class rows
//! (Fighter's own gate already reaches 10) is grounded through level 9.
//!
//! Widens the accepted Paladin level-1..level-8 hybrid chassis baseline
//! (most recently `tests/sd13_paladin_level8_progression.rs`) to Paladin
//! level 9, mirroring the sibling-class level-range-gate idiom
//! (`supported_paladin_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 9`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Paladin class table) were read directly
//! before writing any code or test:
//!
//! - level 9 base attack bonus is +9 (full BAB, genuinely risen from +8 —
//!   the table's own "+9/+4" iterative notation is not modeled anywhere in
//!   this codebase, only the flat base value) and base saves are +6
//!   Fortitude and +6 Will (both good, `9 / 2 + 2 = 6`, numerically
//!   unchanged from level 8, integer-division coincidences) and +3 Reflex
//!   (poor, `9 / 3 = 3`, genuinely risen from +2) — confirmed by the same
//!   formulas already grounded at levels 1-8, not re-derived.
//! - Smite Evil stays 3/day (the next rise lands at 10th, a threshold
//!   stasis checked rather than assumed); its attack bonus stays the flat
//!   Charisma modifier (+2) and its damage bonus genuinely rises to 9
//!   (equal to paladin level) via the same formulas.
//! - Lay on Hands' uses per day stays 6 (`9 / 2 + 2` Charisma modifier —
//!   an integer-division coincidence with level 8) and its heal dice count
//!   stays 4 (`9 / 2`, likewise a coincidence); Divine Grace stays the flat
//!   Charisma-modifier save bonus (+2).
//! - the partial-caster effective caster level genuinely rises to 6
//!   (`9 - 3`), while the partial-caster spell burden itself stays
//!   claim-blocked.
//! - Channel Positive Energy's dice count GENUINELY RISES to 5 (the paladin
//!   channels as an effective cleric of her paladin level; a cleric's
//!   channel dice rise at odd levels, so effective cleric level 9 yields
//!   5d6, up from 4d6 at levels 7-8).
//! - the PF1 Core Rulebook Paladin class table's level-9 "Special" column
//!   reads "Mercy" (verified independently against both primary sources,
//!   checked rather than assumed away) — 9th level IS a repeat-Mercy-grant
//!   level (the 3rd/6th/9th cadence), but exactly like the level-6 repeat
//!   grant before it, recognizing a SECOND mercy selection needs the
//!   mercy-list-growth mechanism this codebase has never grounded, so the
//!   repeat grant stays deliberately named-but-unproven; the single
//!   grounded level-3 selection (mercy:shaken) carries over unchanged, and
//!   its recognitions still fire.
//!
//! It deliberately does not touch the mercy-effect resolution, channel
//! execution, Divine Bond, Aura of Resolve/Courage/Divine Health, or the
//! partial-caster spell posture burden (all stay named-but-unproven,
//! unchanged from levels 1-8), and it does not ground Paladin level 10+. It
//! also preserves the accepted Paladin level-1..level-8 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const PALADIN_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level8_sd13_deterministic_input.txt");

const PALADIN_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level9_sd13_deterministic_input.txt");

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

// ----- Base attack bonus and saves at level 9 -----

#[test]
fn paladin_level9_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(PALADIN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 9,
        "Paladin level 9 full-BAB progression must equal 9, genuinely risen from 8: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 6, "Paladin level 9 good Fortitude (9/2+2) must equal 6");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 3,
        "Paladin level 9 poor Reflex (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 6, "Paladin level 9 good Will (9/2+2) must equal 6");
}

// ----- Smite Evil at level 9: uses/attack stay, damage rises to 9 -----

#[test]
fn paladin_level9_smite_evil_uses_stay_three_and_damage_rises_to_nine() {
    let input = load(PALADIN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 3,
        "Paladin level 9 Smite Evil must stay 3/day (the next rise lands at 10th): {}",
        uses_per_day.detail
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let attack_bonus = explanation(&computation, SMITE_EVIL_ATTACK_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 3,
        "Paladin level 9 Smite Evil attack bonus must stay the flat Charisma modifier (+3)"
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 9,
        "Paladin level 9 Smite Evil damage bonus (equal to paladin level) must equal 9, \
         genuinely risen from 8: {}",
        damage_bonus.detail
    );
}

// ----- Lay on Hands and Divine Grace carry over at level 9 -----

#[test]
fn paladin_level9_lay_on_hands_and_divine_grace_carry_over() {
    let input = load(PALADIN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 7,
        "Paladin level 9 Lay on Hands uses per day (9/2 + Charisma modifier 3) must stay 7 — \
         an integer-division coincidence with level 8: {}",
        uses_per_day.detail
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 4,
        "Paladin level 9 Lay on Hands heal dice count (9/2 d6) must stay 4 — a coincidence \
         with level 8: {}",
        heal_amount.detail
    );

    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "Paladin Divine Grace must stay the flat Charisma-modifier save bonus (+3) at level 9"
    );
}

// ----- Partial-caster effective caster level genuinely rises to 6 -----

#[test]
fn paladin_level9_effective_caster_level_rises_to_six() {
    let input = load(PALADIN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 6,
        "Paladin level 9 effective caster level (9 - 3) must equal 6, genuinely risen from 5: \
         {}",
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
                "no spells are fabricated at paladin level 9: {daily_prep:?}"
            );
        }
    }
}

// ----- Channel Positive Energy dice genuinely rise to 5 at level 9 -----

#[test]
fn paladin_level9_channel_positive_energy_dice_rise_to_five() {
    let input = load(PALADIN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 5,
        "Paladin level 9 Channel Positive Energy (as an effective cleric of paladin level 9) \
         must rise to 5d6 — a cleric's channel dice rise at odd levels: {}",
        dice.detail
    );
}

// ----- Mercy: single grounded selection carries over; repeat grant stays unproven -----

#[test]
fn paladin_level9_mercy_recognitions_carry_over_unchanged() {
    let input = load(PALADIN_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy-granted recognition must carry no fabricated mechanical value at level 9"
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "mercy-choice recognition must carry no fabricated mechanical value at level 9"
    );
    assert!(
        choice.detail.contains("shaken"),
        "mercy-choice recognition must still name the single grounded shaken selection at \
         level 9 (the 9th-level repeat grant stays named-but-unproven per the level-6 \
         precedent): {}",
        choice.detail
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn paladin_level8_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 8, "Paladin level 8 base attack bonus must stay 8");

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(dice.value, 4, "Paladin level 8 Channel Positive Energy must stay 4d6");

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(caster_level.value, 5, "Paladin level 8 effective caster level must stay 5");
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn paladin_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = PALADIN_LEVEL9_FIXTURE.replace("class:paladin:9", "class:paladin:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "level-10 Paladin is now recognized by the later level-10 widening slice \
         (tests/sd13_paladin_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the paladin path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_level9_recognition() {
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
fn multiclass_paladin_level9_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL9_FIXTURE.replace(
        "class_level=class:paladin:9",
        "class_level=class:paladin:9\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_paladin_row_names_level_9_widening() {
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
            .contains("sd13_paladin_level9_progression"),
        "paladin row must cite the live SD13-E5 level-9 proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "paladin partial note must name the level-9 widening: {note}"
    );
}
