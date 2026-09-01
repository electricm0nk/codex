//! SD18 Paladin level-12 widening grounding proof.
//!
//! Widens the accepted Paladin level-1..level-11 hybrid chassis baseline
//! (`tests/sd18_paladin_level11_aura_of_justice.rs`, the loop's
//! previous Paladin ceiling) to Paladin level 12 -- the ninth SD-18
//! §3.2 class-row level-12 widening (after Barbarian, Bard, Cleric, Druid,
//! Fighter, and Monk), mirroring the sibling-class level-range-gate idiom
//! (`supported_paladin_level` is generalized from `1..=11` to `1..=12` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 12`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Paladin class table) were read directly
//! before writing any code or test:
//!
//! - level 12 base attack bonus is +12 (full BAB, genuinely risen from +11
//!   -- the table's own "+12/+7/+2" iterative notation is not modeled
//!   anywhere in this codebase, only the flat base value); ALL THREE base
//!   saves GENUINELY RISE this time (good Fortitude and Will `12/2+2=8`,
//!   poor Reflex `12/3=4`), unlike the level-11 widening where all three
//!   stayed numerically unchanged.
//! - Smite Evil's uses per day STAY 4/day (`1 + (12-1)/3 = 4`, unchanged
//!   from level 11 -- the next rise lands at 13th) and its attack bonus
//!   stays the flat Charisma modifier (+2), while its damage bonus
//!   GENUINELY RISES to 12 (= paladin level, up from 11).
//! - Lay on Hands GENUINELY RISES on both axes (uses per day `12/2+2=8`, up
//!   from 7; heal dice `12/2=6`, up from 5); Divine Grace stays the flat
//!   Charisma-modifier save bonus (+2).
//! - Channel Positive Energy's die count STAYS 6d6 (`(12+1)/2=6`, an
//!   integer-division coincidence with level 11 -- the effective-cleric
//!   dice rise at odd levels, so the next rise lands at 13th).
//! - the partial-caster effective caster level GENUINELY RISES to 9
//!   (`12-3`, up from 8).
//! - the partial-caster spell-level access ladder STAYS 3 (4th-level
//!   paladin spells begin at 13, outside this widening).
//! - the 1st-level spell slot's base count and total STAY numerically
//!   unchanged (base 2, total 3, an integer-division coincidence since the
//!   Charisma bonus stays 1); the 2nd-level spell slot's base count and
//!   total GENUINELY RISE (base 1->2, total 2->3, since level 12 shows
//!   "2/2/1/--" in the raw spells-per-day table, verified against both
//!   primary sources); the 3rd-level spell slot's base count and total
//!   STAY numerically unchanged (base 1, total 1).
//! - the PF1 Core Rulebook Paladin class table's level-12 "Special" column
//!   reads "Mercy" ONLY (verified independently against both primary
//!   sources, checked rather than assumed away) -- 12th IS a
//!   repeat-Mercy-grant level (the 3rd/6th/9th/12th cadence). Grounded here
//!   as a FOURTH numbered mercy choice slot
//!   (`class_chassis.paladin.mercy_4_choice`, gate 12,
//!   `choice:paladin_mercy_4`), mirroring the proven slot-2/slot-3 idiom
//!   exactly: an open-ended +0 recognition of whichever raw mercy string
//!   was selected, with the verified 12th-level CRB tier additions (Blinded,
//!   Deafened, Paralyzed, Stunned; legacy.aonprd.com Core Rulebook text,
//!   d20pfsrd's superset containing them plus non-CRB expansions Amputated,
//!   Ensorcelled, Petrified outside this pf1.core_rulebook seam) cited in
//!   the detail. No mercy's effect on lay on hands is computed.
//!
//! It deliberately does not touch the mercy-effect resolution, channel
//! execution, Divine Bond, Aura of Justice's own smite-sharing resolution,
//! or the partial-caster prepared-spell posture burden (all stay
//! named-but-unproven, unchanged from levels 1-11), and it does not ground
//! Paladin level 13+. It also preserves the accepted Paladin
//! level-1..level-11 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const PALADIN_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level11_sd18_aura_of_justice_deterministic_input.txt"
);

const PALADIN_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level12_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BASE_ATTACK_ID: &str = "class_chassis.paladin.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.paladin.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.paladin.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.paladin.base_save.will";
const SMITE_EVIL_USES_PER_DAY_ID: &str = "class_chassis.paladin.smite_evil_uses_per_day";
const SMITE_EVIL_DAMAGE_BONUS_ID: &str = "class_chassis.paladin.smite_evil_damage_bonus";
const LAY_ON_HANDS_USES_PER_DAY_ID: &str = "class_chassis.paladin.lay_on_hands_uses_per_day";
const LAY_ON_HANDS_HEAL_AMOUNT_ID: &str = "class_chassis.paladin.lay_on_hands_heal_amount";
const EFFECTIVE_CASTER_LEVEL_ID: &str =
    "class_chassis.paladin.partial_caster.effective_caster_level";
const SPELL_LEVEL_ACCESS_ID: &str = "class_chassis.paladin.partial_caster.spell_level_access";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";
const CHANNEL_POSITIVE_ENERGY_DICE_ID: &str =
    "class_chassis.paladin.channel_positive_energy_dice";
const AURA_OF_JUSTICE_ID: &str = "class_chassis.paladin.aura_of_justice";
const BASE_SPELLS_SPELL_LEVEL_1_ID: &str =
    "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_1";
const BASE_SPELLS_SPELL_LEVEL_2_ID: &str =
    "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_2";
const BASE_SPELLS_SPELL_LEVEL_3_ID: &str =
    "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_3";
const TOTAL_SPELLS_SPELL_LEVEL_2_ID: &str =
    "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_2";
const MERCY_4_CHOICE_ID: &str = "class_chassis.paladin.mercy_4_choice";

// ----- Base attack bonus AND all three base saves genuinely rise at level 12 -----

#[test]
fn paladin_level12_base_attack_and_all_saves_genuinely_rise() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 12,
        "Paladin level 12 full-BAB progression must equal 12, genuinely risen from 11: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 8,
        "Paladin level 12 good Fortitude (12/2+2) must genuinely rise to 8, up from 7 at \
         level 11: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 4,
        "Paladin level 12 poor Reflex (12/3) must genuinely rise to 4, up from 3 at level 11: {}",
        reflex.detail
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 8,
        "Paladin level 12 good Will (12/2+2) must genuinely rise to 8, up from 7 at level 11: {}",
        will.detail
    );
}

// ----- Smite Evil at level 12: uses stay 4/day, damage genuinely rises to 12 -----

#[test]
fn paladin_level12_smite_evil_uses_stay_and_damage_rises_to_twelve() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 4,
        "Paladin level 12 Smite Evil must stay 4/day (1 + (12 - 1)/3), an integer-division \
         coincidence with level 11 -- the next rise lands at 13th: {}",
        uses_per_day.detail
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 12,
        "Paladin level 12 Smite Evil damage bonus (equal to paladin level) must genuinely \
         rise to 12: {}",
        damage_bonus.detail
    );
}

// ----- Lay on Hands genuinely rises on both axes at level 12 -----

#[test]
fn paladin_level12_lay_on_hands_genuinely_rises() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 9,
        "Paladin level 12 Lay on Hands uses per day (12/2 + Charisma modifier 3) must \
         genuinely rise to 9, up from 8 at level 11: {}",
        uses_per_day.detail
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 6,
        "Paladin level 12 Lay on Hands heal dice count (12/2 d6) must genuinely rise to 6, up \
         from 5 at level 11: {}",
        heal_amount.detail
    );
}

// ----- Channel Positive Energy stays 6 at level 12 (integer-division coincidence) -----

#[test]
fn paladin_level12_channel_positive_energy_dice_stay_six() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 6,
        "Paladin level 12 Channel Positive Energy must stay 6d6, an integer-division \
         coincidence with level 11 (the effective-cleric dice rise at odd levels): {}",
        dice.detail
    );
}

// ----- Effective caster level genuinely rises to 9; access ladder stays 3 -----

#[test]
fn paladin_level12_effective_caster_level_rises_and_spell_access_stays() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 9,
        "Paladin level 12 effective caster level (12 - 3) must genuinely rise to 9, up from \
         8 at level 11: {}",
        caster_level.detail
    );

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 3,
        "Paladin level 12 spell-level access must stay 3 (4th-level paladin spells begin at \
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
                "no spells are fabricated at paladin level 12: {daily_prep:?}"
            );
        }
    }
}

// ----- 2nd-level spell base count and total genuinely rise; 1st/3rd stay unchanged -----

#[test]
fn paladin_level12_second_level_spell_rises_first_and_third_stay() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_1 = explanation(&computation, BASE_SPELLS_SPELL_LEVEL_1_ID);
    assert_eq!(
        base_1.value, 2,
        "Paladin level 12 1st-level spell base count must stay 2, an integer-division \
         coincidence with level 11: {}",
        base_1.detail
    );

    let base_2 = explanation(&computation, BASE_SPELLS_SPELL_LEVEL_2_ID);
    assert_eq!(
        base_2.value, 2,
        "Paladin level 12 2nd-level spell base count (the raw table row \"2/2/1/--\") must \
         genuinely rise to 2, up from 1 at level 11: {}",
        base_2.detail
    );

    let total_2 = explanation(&computation, TOTAL_SPELLS_SPELL_LEVEL_2_ID);
    assert_eq!(
        total_2.value, 3,
        "Paladin level 12 2nd-level spell total (base 2 + Charisma-bonus 1) must genuinely \
         rise to 3, up from 2 at level 11: {}",
        total_2.detail
    );

    let base_3 = explanation(&computation, BASE_SPELLS_SPELL_LEVEL_3_ID);
    assert_eq!(
        base_3.value, 1,
        "Paladin level 12 3rd-level spell base count must stay 1, an integer-division \
         coincidence with level 11: {}",
        base_3.detail
    );
}

// ----- Aura of Justice carries over unchanged (granted at 11, still granted at 12) -----

#[test]
fn paladin_level12_aura_of_justice_carries_over() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_JUSTICE_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Justice must stay a bounded grant-only identity record (value 0, \
         non-fabricated) at level 12: {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("granted"),
        "Aura of Justice must still claim to be granted at level 12: {}",
        aura.detail
    );
}

// ----- Mercy: the FOURTH numbered slot is newly granted at level 12 -----

#[test]
fn paladin_level12_fourth_mercy_slot_is_newly_recognized() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_4 = explanation(&computation, MERCY_4_CHOICE_ID);
    assert_eq!(
        slot_4.value, 0,
        "mercy slot 4 recognition must carry no fabricated mechanical value at level 12: {}",
        slot_4.detail
    );
    assert!(
        slot_4.detail.contains("mercy:blinded") && slot_4.detail.contains("choice:paladin_mercy_4"),
        "mercy slot 4 must name the raw chosen mercy string: {}",
        slot_4.detail
    );
    assert!(
        slot_4.detail.contains("Blinded")
            && slot_4.detail.contains("Deafened")
            && slot_4.detail.contains("Paralyzed")
            && slot_4.detail.contains("Stunned"),
        "mercy slot 4 must cite the verified 12th-level CRB tier additions: {}",
        slot_4.detail
    );

    // The level-11 fixture carries no slot-4 selection; the gate must stay silent below 12.
    let level11_input = load(PALADIN_LEVEL11_FIXTURE);
    let level11_computation = compute_pilot_base_chassis(&level11_input);
    assert!(
        !level11_computation
            .explanations
            .iter()
            .any(|e| e.id == MERCY_4_CHOICE_ID),
        "mercy slot 4 (gate 12) must be silent at level 11"
    );
}

// ----- Negative control: the level-11 fixture stays unaffected by this widening -----

#[test]
fn paladin_level11_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 11, "Paladin level 11 base attack bonus must stay 11");

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 7, "Paladin level 11 good Fortitude must stay 7");

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(dice.value, 6, "Paladin level 11 Channel Positive Energy dice must stay 6");

    let base_2 = explanation(&computation, BASE_SPELLS_SPELL_LEVEL_2_ID);
    assert_eq!(base_2.value, 1, "Paladin level 11 2nd-level spell base count must stay 1");
}

// ----- Negative control: level 18 stays unrecognized by this slice -----
// (level 13 was later widened into the supported tranche by SD18's
// cycle-2026-07-15T1800 widening slice, level 14 by SD18's
// cycle-2026-07-15T2500 widening slice, level 15 by SD18's
// cycle-2026-07-15T4300 widening slice, level 16 by SD18's
// cycle-2026-07-15T5400 widening slice, and level 17 by SD18's
// cycle-2026-07-15T10700 widening slice; see
// tests/sd18_paladin_level13_widening.rs,
// tests/sd18_paladin_level14_widening.rs,
// tests/sd18_paladin_level15_widening.rs,
// tests/sd18_paladin_level16_widening.rs, and
// tests/sd18_paladin_level17_widening.rs for their own boundaries.)

#[test]
fn paladin_level_21_is_not_promoted_by_this_slice() {
    let level_21 = PALADIN_LEVEL12_FIXTURE.replace("class:paladin:12", "class:paladin:21");
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
fn fighter_does_not_gain_paladin_level12_recognition() {
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
fn multiclass_paladin_level12_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL12_FIXTURE.replace(
        "class_level=class:paladin:12",
        "class_level=class:paladin:12\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_paladin_row_names_level_12_widening() {
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
            .contains("sd18_paladin_level12_widening"),
        "paladin row must cite the live SD18 level-12 widening proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "paladin partial note must name the level-12 widening: {note}"
    );
}
