//! SD18 Paladin level-11 widening grounding proof.
//!
//! Widens the accepted Paladin level-1..level-10 hybrid chassis baseline
//! (`tests/sd13_paladin_level10_progression.rs`, the SD13 tranche's declared
//! ceiling) to Paladin level 11 -- the seventh SD-18 §3.2 class-row
//! widening, mirroring the sibling-class level-range-gate idiom
//! (`supported_paladin_level` is generalized from `1..=10` to `1..=11` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 11`, exactly as `cycle-2026-07-13T1255`
//! widened `MAX_SUPPORTED_BARBARIAN_LEVEL`, `cycle-2026-07-13T1830` widened
//! `MAX_SUPPORTED_BARD_LEVEL`, `cycle-2026-07-13T2007` widened
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `cycle-2026-07-13T1851` widened
//! `MAX_SUPPORTED_DRUID_LEVEL`, `cycle-2026-07-13T1941` widened
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, and `cycle-2026-07-13T2100` widened
//! `MAX_SUPPORTED_MONK_LEVEL`, all from 10 to 11). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Paladin class table) were read
//! directly before writing any code or test:
//!
//! - level 11 base attack bonus is +11 (full BAB, genuinely risen from +10
//!   -- the table's own "+11/+6/+1" iterative notation is not modeled
//!   anywhere in this codebase, only the flat base value), while all three
//!   base saves STAY numerically unchanged from level 10 (good Fortitude
//!   and Will `11/2+2=7`, poor Reflex `11/3=3`, both integer-division
//!   coincidences).
//! - Smite Evil's uses per day STAY 4/day (`1 + (11-1)/3 = 4`, unchanged
//!   from level 10 -- the next rise lands at 13th) and its attack bonus
//!   stays the flat Charisma modifier (+2), while its damage bonus
//!   GENUINELY RISES to 11 (= paladin level, up from 10).
//! - Lay on Hands STAYS numerically unchanged on both axes (uses per day
//!   `11/2+2=7`, heal dice `11/2=5`, both integer-division coincidences
//!   with level 10); Divine Grace stays the flat Charisma-modifier save
//!   bonus (+2).
//! - 11th is NOT a repeat-Mercy-grant level (the 3rd/6th/9th cadence), so
//!   the single grounded level-3 selection (mercy:shaken) carries over
//!   unchanged.
//! - Channel Positive Energy's die count GENUINELY RISES to 6d6
//!   (`ceil(11/2)=6`, up from 5d6 at level 10 -- the effective-cleric dice
//!   rise at odd levels, confirmed rather than assumed).
//! - the partial-caster effective caster level GENUINELY RISES to 8
//!   (`11-3`, up from 7).
//! - the partial-caster spell-level access ladder STAYS 3 (4th-level
//!   paladin spells begin at 13, outside this widening).
//! - the 3rd-level spell slot's base count GENUINELY RISES to 1 (up from
//!   the honest ZERO at level 10 -- level 11 shows "2/1/1/--" in the raw
//!   spells-per-day table, verified against both primary sources), so the
//!   3rd-level spell's integrated TOTAL genuinely rises from 0 to 1 (base
//!   count 1 plus a Charisma bonus of 0, since Charisma modifier 2 stays
//!   below spell level 3); the 1st- and 2nd-level base counts and totals
//!   stay numerically unchanged (2/1 base, 3/2 total).
//! - the PF1 Core Rulebook Paladin class table's level-11 "Special" column
//!   reads "Aura of justice" ONLY (verified independently against both
//!   primary sources, checked rather than assumed away) -- a paladin can
//!   expend two uses of her smite evil ability to share the smite-evil
//!   ability with allies within 10 feet, using her own bonuses. Grounded
//!   here as a new bounded grant-only identity record
//!   (`class_chassis.paladin.aura_of_justice`, value 0, non-fabricated),
//!   mirroring exactly how Monk's Diamond Body poison-immunity grant was
//!   grounded at 11th level: no ally-aura/positional engine and no
//!   smite-evil-resource-sharing execution engine exists anywhere in this
//!   codebase to apply the shared smite to.
//!
//! It deliberately does not touch the mercy-effect resolution, channel
//! execution, Divine Bond, Aura of Justice's own smite-sharing resolution,
//! or the partial-caster prepared-spell posture burden (all stay
//! named-but-unproven, unchanged from levels 1-10), and it does not ground
//! Paladin level 12+. It also preserves the accepted Paladin
//! level-1..level-10 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const PALADIN_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level10_sd13_deterministic_input.txt");

const PALADIN_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level11_sd18_aura_of_justice_deterministic_input.txt"
);

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
const SPELL_LEVEL_ACCESS_ID: &str = "class_chassis.paladin.partial_caster.spell_level_access";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";
const MERCY_GRANTED_ID: &str = "class_chassis.paladin.mercy_granted";
const MERCY_CHOICE_ID: &str = "class_chassis.paladin.mercy_choice";
const CHANNEL_POSITIVE_ENERGY_DICE_ID: &str =
    "class_chassis.paladin.channel_positive_energy_dice";
const AURA_OF_JUSTICE_ID: &str = "class_chassis.paladin.aura_of_justice";
const BASE_SPELLS_SPELL_LEVEL_3_ID: &str =
    "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_3";
const TOTAL_SPELLS_SPELL_LEVEL_3_ID: &str =
    "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_3";

// ----- Base attack bonus rises, saves stay unchanged at level 11 -----

#[test]
fn paladin_level11_base_attack_rises_and_saves_stay_unchanged() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 11,
        "Paladin level 11 full-BAB progression must equal 11, genuinely risen from 10: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 7,
        "Paladin level 11 good Fortitude (11/2+2) must stay 7, an integer-division \
         coincidence with level 10"
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 3,
        "Paladin level 11 poor Reflex (11/3) must stay 3, an integer-division coincidence \
         with level 10"
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 7,
        "Paladin level 11 good Will (11/2+2) must stay 7, an integer-division coincidence \
         with level 10"
    );
}

// ----- Smite Evil at level 11: uses stay 4/day, damage genuinely rises to 11 -----

#[test]
fn paladin_level11_smite_evil_uses_stay_and_damage_rises_to_eleven() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 4,
        "Paladin level 11 Smite Evil must stay 4/day (1 + (11 - 1)/3), an integer-division \
         coincidence with level 10 -- the next rise lands at 13th: {}",
        uses_per_day.detail
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let attack_bonus = explanation(&computation, SMITE_EVIL_ATTACK_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 3,
        "Paladin level 11 Smite Evil attack bonus must stay the flat Charisma modifier (+3)"
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 11,
        "Paladin level 11 Smite Evil damage bonus (equal to paladin level) must genuinely \
         rise to 11: {}",
        damage_bonus.detail
    );
}

// ----- Lay on Hands stays unchanged on both axes; Divine Grace carries over -----

#[test]
fn paladin_level11_lay_on_hands_stays_and_divine_grace_carries_over() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 8,
        "Paladin level 11 Lay on Hands uses per day (11/2 + Charisma modifier 3) must stay 8, \
         an integer-division coincidence with level 10: {}",
        uses_per_day.detail
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 5,
        "Paladin level 11 Lay on Hands heal dice count (11/2 d6) must stay 5, an \
         integer-division coincidence with level 10: {}",
        heal_amount.detail
    );

    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "Paladin Divine Grace must stay the flat Charisma-modifier save bonus (+3) at level 11"
    );
}

// ----- Channel Positive Energy genuinely rises to 6 at level 11 -----

#[test]
fn paladin_level11_channel_positive_energy_dice_rise_to_six() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 6,
        "Paladin level 11 Channel Positive Energy (as an effective cleric of paladin level \
         11) must genuinely rise to 6d6, up from 5d6 at level 10: {}",
        dice.detail
    );
}

// ----- Effective caster level genuinely rises to 8; access ladder stays 3 -----

#[test]
fn paladin_level11_effective_caster_level_rises_and_spell_access_stays() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 8,
        "Paladin level 11 effective caster level (11 - 3) must genuinely rise to 8, up from \
         7 at level 10: {}",
        caster_level.detail
    );

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 3,
        "Paladin level 11 spell-level access must stay 3 (4th-level paladin spells begin at \
         13, outside this widening): {}",
        access.detail
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
                "no spells are fabricated at paladin level 11: {daily_prep:?}"
            );
        }
    }
}

// ----- 3rd-level spell base count and total genuinely rise from zero to one -----

#[test]
fn paladin_level11_third_level_spell_base_and_total_rise_from_zero() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base = explanation(&computation, BASE_SPELLS_SPELL_LEVEL_3_ID);
    assert_eq!(
        base.value, 1,
        "Paladin level 11 3rd-level spell base count (the raw table row \"2/1/1/--\") must \
         genuinely rise to 1, up from the honest ZERO at level 10: {}",
        base.detail
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2, which
    // meets spell level 3's bonus-spell threshold (modifier >= spell level), unlocking a
    // genuine +1 Charisma-bonus spell here.
    let total = explanation(&computation, TOTAL_SPELLS_SPELL_LEVEL_3_ID);
    assert_eq!(
        total.value, 2,
        "Paladin level 11 3rd-level spell total (base 1 + Charisma-bonus 1, modifier 3 meets \
         spell level 3) must genuinely rise to 2, up from the honest ZERO at level 10: {}",
        total.detail
    );
}

// ----- Aura of Justice is newly granted at level 11 -----

#[test]
fn paladin_level11_aura_of_justice_is_newly_granted() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_JUSTICE_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Justice must be a bounded grant-only identity record (value 0, \
         non-fabricated): {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("smite"),
        "Aura of Justice's detail must name the smite-sharing grant: {}",
        aura.detail
    );

    let level10_input = load(PALADIN_LEVEL10_FIXTURE);
    let level10_computation = compute_pilot_base_chassis(&level10_input);
    let level10_aura = explanation(&level10_computation, AURA_OF_JUSTICE_ID);
    assert_eq!(
        level10_aura.value, 0,
        "Aura of Justice must correctly be absent (value 0, level-gate placeholder) below \
         level 11"
    );
    assert!(
        !level10_aura.detail.to_lowercase().contains("granted"),
        "Aura of Justice must not claim to be granted at level 10: {}",
        level10_aura.detail
    );
}

// ----- Mercy: single grounded selection carries over at level 11 -----

#[test]
fn paladin_level11_mercy_recognitions_carry_over_unchanged() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy-granted recognition must carry no fabricated mechanical value at level 11"
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "mercy-choice recognition must carry no fabricated mechanical value at level 11"
    );
    assert!(
        choice.detail.contains("shaken"),
        "mercy-choice recognition must still name the single grounded shaken selection at \
         level 11 (11th is not a repeat-Mercy-grant level): {}",
        choice.detail
    );
}

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn paladin_level10_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 10, "Paladin level 10 base attack bonus must stay 10");

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(uses_per_day.value, 4, "Paladin level 10 Smite Evil must stay 4/day");

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(dice.value, 5, "Paladin level 10 Channel Positive Energy dice must stay 5");

    let base = explanation(&computation, BASE_SPELLS_SPELL_LEVEL_3_ID);
    assert_eq!(base.value, 0, "Paladin level 10 3rd-level spell base count must stay 0");
}

// ----- Negative control: level 18 stays unrecognized by this slice -----
// (level 12 was later widened into the supported tranche by SD18's
// cycle-2026-07-15T0700 widening slice, level 13 by SD18's
// cycle-2026-07-15T1800 widening slice, level 14 by SD18's
// cycle-2026-07-15T2500 widening slice, level 15 by SD18's
// cycle-2026-07-15T4300 widening slice, level 16 by SD18's
// cycle-2026-07-15T5400 widening slice, and level 17 by SD18's
// cycle-2026-07-15T10700 widening slice; see
// tests/sd18_paladin_level12_widening.rs,
// tests/sd18_paladin_level13_widening.rs,
// tests/sd18_paladin_level14_widening.rs,
// tests/sd18_paladin_level15_widening.rs,
// tests/sd18_paladin_level16_widening.rs, and
// tests/sd18_paladin_level17_widening.rs for their own boundaries.)

#[test]
fn paladin_level_21_is_not_promoted_by_this_slice() {
    let level_21 = PALADIN_LEVEL11_FIXTURE.replace("class:paladin:11", "class:paladin:21");
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
fn fighter_does_not_gain_paladin_level11_recognition() {
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
fn multiclass_paladin_level11_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL11_FIXTURE.replace(
        "class_level=class:paladin:11",
        "class_level=class:paladin:11\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_paladin_row_names_level_11_widening() {
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
            .contains("sd18_paladin_level11_aura_of_justice"),
        "paladin row must cite the live SD18 level-11 widening proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "paladin partial note must name the level-11 widening: {note}"
    );
}
