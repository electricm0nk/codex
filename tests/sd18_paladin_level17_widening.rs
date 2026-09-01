//! SD18 Paladin level-17 widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Paladin level-1..level-16
//! hybrid chassis (`tests/sd18_paladin_level16_widening.rs`, the loop's most
//! recent Paladin ceiling) to Paladin level 17 -- the loop's SEVENTH §3.2
//! class-row level-17 landing (after Ranger, Bard, Rogue, Fighter, Wizard,
//! and Cleric's level-17 landings), and the second hybrid/partial-caster
//! class-row widening to reach level 17
//! (`supported_paladin_level` is generalized from `1..=16` to `1..=17` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 17`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants). §3.1 race rows and §3.3
//! interaction rows are structurally exhausted/blocked (cited in the
//! progress doc, not re-derived this cycle); §3.4/§3.5 are structurally
//! blocked (same root cause, also cited, not re-derived). Monk is capped at
//! level 12 and Druid is capped at level 15, both documented structural
//! exceptions -- not re-attempted.
//!
//! Both primary sources checked for this cycle (d20pfsrd and the Archives
//! of Nethys aonprd.com mirror) agreed byte-for-byte, with no
//! self-contradictory fetches, so a third source was not required:
//!
//! - level 17 base attack bonus GENUINELY RISES to +17 (full BAB
//!   progression, up from +16 at level 16; the table's own
//!   "+17/+12/+7/+2" iterative notation is not modeled anywhere in this
//!   codebase, only the flat base value); ALL THREE base saves STAY
//!   numerically unchanged from level 16 (good Fortitude/Will `17/2+2 = 10`
//!   and poor Reflex `17/3 = 5`, both integer-division coincidences).
//! - the PF1 Core Rulebook Paladin class table's level-17 "Special" column
//!   reads only "Aura of righteousness" (verified independently against
//!   both sources) -- a genuinely NEW class feature, grounded as a THIRD
//!   bounded grant-only identity record mirroring the Aura of Justice /
//!   Aura of Faith idiom exactly: no damage-reduction-application engine
//!   and no compulsion-immunity-check engine exists anywhere in this
//!   codebase to apply "DR 5/evil and immunity to compulsion spells and
//!   spell-like abilities" to. 17th is NOT a repeat-Mercy-grant level (the
//!   3rd/6th/9th/12th/15th cadence), so no sixth mercy slot is introduced.
//! - the base spells-per-day table's level-17 row is `4/3/2/1`
//!   (1st/2nd/3rd/4th), verified independently against both sources: the
//!   2nd/3rd/4th-level columns stay 3/2/1 numerically unchanged from
//!   level 16, and the 1st-level column GENUINELY RISES from 3 to 4. The
//!   spell-level access ladder
//!   (`class_chassis.paladin.partial_caster.spell_level_access`) stays 4
//!   (already widened at level 13, unchanged here), and the base
//!   spell-save-DC and Charisma-bonus-spells families both continue to
//!   extend to the 4th spell level automatically (live arithmetic, no new
//!   formula invented).
//! - Smite Evil's uses-per-day formula (`1 + (paladin level - 1) / 3`)
//!   STAYS numerically unchanged at 6/day (an integer-division coincidence
//!   with level 16; the next rise lands at level 19), while its damage
//!   bonus (equal to paladin level) GENUINELY RISES to 17.
//!
//! It deliberately does not touch mercy-effect resolution, channel
//! positive energy healing/damage-resolution execution, Divine Bond
//! (weapon bond / mount bond) selection or execution, Aura of Justice's,
//! Aura of Faith's, or Aura of Righteousness's own resolution engines, or
//! the partial-caster prepared-spell posture burden (all stay
//! named-but-unproven, unchanged from levels 1-16), and it does not ground
//! Paladin level 18+. It also preserves the accepted Paladin
//! level-1..level-16 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.
//!
//! This slice also fixes five pre-existing stale sibling negative
//! controls that this widening would otherwise have broken:
//! `tests/sd13_paladin_level10_progression.rs`'s,
//! `tests/sd18_paladin_level11_aura_of_justice.rs`'s,
//! `tests/sd18_paladin_level12_widening.rs`'s, and
//! `tests/sd18_paladin_level13_widening.rs`'s, and
//! `tests/sd18_paladin_level14_widening.rs`'s
//! `paladin_level_1N_is_not_promoted_by_this_slice` controls, all moved to
//! a level-18 boundary in the same commit; `tests/sd18_paladin_level16_widening.rs`'s
//! own level-17 negative control is REMOVED rather than moved, since
//! level 17 is now itself the supported/grounded row rather than the
//! out-of-range boundary, mirroring the Barbarian/Fighter/Wizard/Rogue/
//! Cleric level-N-to-level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level16_sd18_widening_deterministic_input.txt"
);

const PALADIN_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level17_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.paladin.partial_caster.base_spells_per_day.";

const BASE_ATTACK_ID: &str = "class_chassis.paladin.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.paladin.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.paladin.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.paladin.base_save.will";
const SMITE_EVIL_USES_PER_DAY_ID: &str = "class_chassis.paladin.smite_evil_uses_per_day";
const SMITE_EVIL_DAMAGE_BONUS_ID: &str = "class_chassis.paladin.smite_evil_damage_bonus";
const SPELL_LEVEL_ACCESS_ID: &str = "class_chassis.paladin.partial_caster.spell_level_access";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";
const AURA_OF_FAITH_ID: &str = "class_chassis.paladin.aura_of_faith";
const AURA_OF_RIGHTEOUSNESS_ID: &str = "class_chassis.paladin.aura_of_righteousness";
const DAMAGE_REDUCTION_ID: &str = "class_chassis.paladin.damage_reduction";
const MERCY_5_CHOICE_ID: &str = "class_chassis.paladin.mercy_5_choice";

fn values_with_prefix(
    computation: &PilotBaseChassisComputation,
    prefix: &str,
) -> Vec<(String, i16)> {
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(prefix))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

// ----- Base attack bonus genuinely rises; all three base saves stay unchanged -----

#[test]
fn paladin_level17_base_attack_rises_and_saves_stay_unchanged() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 17,
        "Paladin level 17 full-BAB progression must equal 17, genuinely risen from 16: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 10,
        "Paladin level 17 good Fortitude (17/2+2) must stay 10, an integer-division \
         coincidence with level 16: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 5,
        "Paladin level 17 poor Reflex (17/3) must stay 5, an integer-division coincidence \
         with level 16: {}",
        reflex.detail
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 10,
        "Paladin level 17 good Will (17/2+2) must stay 10, an integer-division coincidence \
         with level 16: {}",
        will.detail
    );
}

// ----- Smite Evil stays 6/day at level 17 (integer-division coincidence); damage bonus rises -----

#[test]
fn paladin_level17_smite_evil_uses_stay_but_damage_bonus_rises() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 6,
        "Paladin level 17 Smite Evil must stay 6/day (1 + (17 - 1)/3), an integer-division \
         coincidence with level 16 (the next rise lands at level 19): {}",
        uses_per_day.detail
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 17,
        "Paladin level 17 Smite Evil damage bonus (equal to paladin level) must genuinely \
         rise to 17: {}",
        damage_bonus.detail
    );
}

// ----- Base spells per day widen at level 17: only the 1st-level column rises -----

#[test]
fn paladin_level17_base_spells_per_day_match_the_raw_table_row() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "level 17 (`4/3/2/1`): the 2nd/3rd/4th-level columns stay 3/2/1 unchanged, and the \
         1st-level column genuinely rises from 3 to 4"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn paladin_level17_spell_level_access_stays_four() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 4,
        "Paladin level 17 spell-level access must stay 4, unchanged from level 16: {}",
        access.detail
    );

    let dc4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.spell_save_dc.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &dc4.id),
        "the 4th-level spell save DC must stay grounded at level 17"
    );

    let bonus4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.bonus_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &bonus4.id),
        "the 4th-level Charisma bonus-spells record must stay grounded at level 17"
    );

    let total4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &total4.id),
        "the 4th-level total spells-per-day record must stay grounded at level 17"
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
                "no spells are fabricated at paladin level 17: {daily_prep:?}"
            );
        }
    }
}

// ----- No sixth mercy slot is introduced at level 17 (not a repeat-grant level) -----

#[test]
fn paladin_level17_no_sixth_mercy_slot_is_introduced() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, "class_chassis.paladin.mercy_6_choice"),
        "mercy slot 6 must not be introduced at level 17 (not a repeat-Mercy-grant level; the \
         cadence is 3rd/6th/9th/12th/15th): {:?}",
        computation.explanations
    );

    // The fifth mercy slot, granted at level 15, must still carry over unchanged.
    let slot_5 = explanation(&computation, MERCY_5_CHOICE_ID);
    assert_eq!(
        slot_5.value, 0,
        "mercy slot 5 recognition must carry no fabricated mechanical value at level 17: {}",
        slot_5.detail
    );
}

// ----- Aura of Faith carries over unchanged (granted at 14, still granted at 17) -----

#[test]
fn paladin_level17_aura_of_faith_carries_over() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_FAITH_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Faith must stay a bounded grant-only identity record (value 0, \
         non-fabricated) at level 17: {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("granted"),
        "Aura of Faith must still claim to be granted at level 17: {}",
        aura.detail
    );
}

// ----- Aura of Righteousness is newly granted at level 17 -----

#[test]
fn paladin_level17_aura_of_righteousness_is_newly_granted() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_RIGHTEOUSNESS_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Righteousness must be a bounded grant-only identity record (value 0, \
         non-fabricated) at level 17: {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("granted"),
        "Aura of Righteousness must claim to be granted at level 17: {}",
        aura.detail
    );

    // Below the level-17 gate, level 16 must carry a correct absence record instead.
    let level16_input = load(PALADIN_LEVEL16_FIXTURE);
    let level16_computation = compute_pilot_base_chassis(&level16_input);
    let level16_aura = explanation(&level16_computation, AURA_OF_RIGHTEOUSNESS_ID);
    assert_eq!(
        level16_aura.value, 0,
        "Aura of Righteousness at level 16 must be a correct level-gate absence (value 0): {}",
        level16_aura.detail
    );
    assert!(
        level16_aura.detail.to_lowercase().contains("absent")
            || level16_aura.detail.to_lowercase().contains("correctly"),
        "Aura of Righteousness at level 16 must name the correct absence, not a grant: {}",
        level16_aura.detail
    );
}

// ----- Aura of Righteousness's DR clause carries a real magnitude -----

/// The DR half of Aura of Righteousness is a flat, self-scoped magnitude
/// (corpus `DR:5/Evil`), grounded as its own record rather than folded into
/// the aura's identity record -- which stays a value-0 grant marker, since
/// the feature has three clauses and only this one is grounded.
///
/// This reverses a stale deferral, not a verified one: the aura record's own
/// text cited "no damage-reduction-application engine ... exists anywhere in
/// this codebase", which is the same reasoning three shipped DR records
/// already reject (`class_feature.barbarian.damage_reduction`,
/// `class_feature.acg.skald.damage_reduction`, Fighter's
/// `ARMOR_MASTERY_DAMAGE_REDUCTION`). A DR magnitude is a property of the
/// character, not of any incoming attack.
#[test]
fn paladin_level17_damage_reduction_grounds_its_real_magnitude() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dr = explanation(&computation, DAMAGE_REDUCTION_ID);
    assert_eq!(
        dr.value, 5,
        "Aura of Righteousness grants a real DR 5/evil at level 17: {}",
        dr.detail
    );
    assert!(
        dr.detail.to_lowercase().contains("evil"),
        "the DR record must name its /evil bypass, which is the whole rule: {}",
        dr.detail
    );

    // The aura's own identity record stays a value-0 grant marker: two of its
    // three clauses (compulsion immunity, the ally +4 morale bonus) remain
    // genuinely ungrounded, so the feature as a whole is not "computed".
    let aura = explanation(&computation, AURA_OF_RIGHTEOUSNESS_ID);
    assert_eq!(
        aura.value, 0,
        "the aura record must stay a grant-only marker even once its DR clause is grounded: {}",
        aura.detail
    );
}

#[test]
fn paladin_level16_damage_reduction_is_a_correct_level_gate_absence() {
    let input = load(PALADIN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dr = explanation(&computation, DAMAGE_REDUCTION_ID);
    assert_eq!(
        dr.value, 0,
        "Paladin DR must be a correct level-gate absence below 17, not a fabricated 5: {}",
        dr.detail
    );
    assert!(
        dr.detail.to_lowercase().contains("absent")
            || dr.detail.to_lowercase().contains("correctly"),
        "the level-16 DR record must name the correct absence, not a grant: {}",
        dr.detail
    );
}

// ----- The bounded Paladin computation stays claim-blocked overall -----

#[test]
fn paladin_level17_still_claim_blocks_overall() {
    let input = load(PALADIN_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-17 Paladin must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 16 truth is unchanged by this widening -----

#[test]
fn paladin_level16_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 16, "Paladin level 16 base attack bonus must stay 16");

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(uses_per_day.value, 6, "Paladin level 16 Smite Evil must stay 6/day");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "Paladin level 16 base spells per day must stay `3/3/2/1`"
    );
}

// ----- Negative control: the paladin path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_level17_recognition() {
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
fn multiclass_paladin_level17_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL17_FIXTURE.replace(
        "class_level=class:paladin:17",
        "class_level=class:paladin:17\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_paladin_row_names_level_17_widening() {
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
            .contains("sd18_paladin_level17_widening"),
        "paladin row must cite the live SD18 level-17 widening proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 17") || note.contains("level-17"),
        "paladin partial note must name the level-17 widening: {note}"
    );
}
