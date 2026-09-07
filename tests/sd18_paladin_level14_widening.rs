//! SD18 Paladin level-14 widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Paladin level-1..level-13
//! hybrid chassis (`tests/sd18_paladin_level13_widening.rs`, the loop's most
//! recent Paladin ceiling) to Paladin level 14 -- the loop's EIGHTH §3.2
//! class-row level-14 landing, after Barbarian, Fighter, Rogue, Ranger,
//! Bard, Cleric, and Druid, and the third hybrid/partial-caster class
//! (after Ranger and, at level 13, itself) to reach it
//! (`supported_paladin_level` is generalized from `1..=13` to `1..=14` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 14`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants). §3.1 race rows and §3.3
//! interaction rows are structurally exhausted/blocked (cited in the
//! progress doc, not re-derived this cycle); §3.4/§3.5 are structurally
//! blocked (same root cause, also cited, not re-derived). Monk is a
//! confirmed dead end at level 13 (Diamond Soul needs spell resistance,
//! which does not exist anywhere in this codebase) -- not re-attempted.
//!
//! All three primary sources checked for this cycle (d20pfsrd, the
//! Archives of Nethys aonprd.com mirror, and legacy.aonprd.com) agreed
//! byte-for-byte, with no self-contradictory fetches this time:
//!
//! - level 14 base attack bonus GENUINELY RISES to +14 (full BAB
//!   progression, up from +13 at level 13; the table's own "+14/+9/+4"
//!   iterative notation is not modeled anywhere in this codebase, only the
//!   flat base value); ALL THREE base saves GENUINELY RISE too (good
//!   Fortitude/Will `14/2+2 = 9`, up from 8; poor Reflex `14/3 = 4`,
//!   staying numerically unchanged -- an integer-division coincidence,
//!   re-verified rather than assumed).
//! - the PF1 Core Rulebook Paladin class table's level-14 "Special" column
//!   reads only "Aura of faith" (verified independently against all three
//!   sources) -- a genuinely NEW class feature, grounded here as a bounded
//!   grant-only identity record (`class_chassis.paladin.aura_of_faith`,
//!   value 0, non-fabricated), mirroring the Aura of Justice / Monk
//!   Diamond Body idiom exactly: "At 14th level, a paladin's weapons are
//!   treated as good-aligned for the purposes of overcoming damage
//!   reduction. Additionally, any attack made against an enemy within 10
//!   feet of her is treated as good-aligned for the purposes of overcoming
//!   damage reduction." No alignment-treatment execution engine and no
//!   damage-reduction-overcoming resolution engine exists anywhere in this
//!   codebase to apply this to. 14th is NOT one of the repeat-Mercy-grant
//!   levels (3rd/6th/9th/12th/15th cadence), so no fifth mercy slot is
//!   introduced by this widening. Smite Evil's uses-per-day formula
//!   (already level-generic) stays 5/day at level 14
//!   (`1 + (14 - 1) / 3 = 5`), an integer-division coincidence with level
//!   13, re-verified rather than assumed; the level-14 "Special" column
//!   does not even name Smite Evil, confirming no new rise is claimed.
//! - the base spells-per-day table's level-14 row is `3/2/1/1`
//!   (1st/2nd/3rd/4th), verified independently against all three sources:
//!   the 1st/2nd/3rd-level columns stay 3/2/1 numerically unchanged from
//!   level 13, and the 4th-level column GENUINELY RISES from 0 to 1 -- the
//!   first castable 4th-level paladin spell slot, checked rather than
//!   assumed. The spell-level access ladder
//!   (`class_chassis.paladin.partial_caster.spell_level_access`) stays 4
//!   (already widened at level 13, unchanged here), and the base
//!   spell-save-DC and Charisma-bonus-spells families both continue to
//!   extend to the 4th spell level automatically (live arithmetic, no new
//!   formula invented).
//!
//! It deliberately does not touch mercy-effect resolution, channel
//! positive energy healing/damage-resolution execution, Divine Bond
//! (weapon bond / mount bond) selection or execution, Aura of Justice's
//! own smite-sharing resolution, Aura of Faith's own
//! good-alignment-weapon-treatment resolution, or the partial-caster
//! prepared-spell posture burden (all stay named-but-unproven, unchanged
//! from levels 1-13), and it does not ground Paladin level 15+. It also
//! preserves the accepted Paladin level-1..level-13 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.
//!
//! This slice also fixes four pre-existing stale sibling negative
//! controls that this widening would otherwise have broken:
//! `tests/sd13_paladin_level10_progression.rs`'s,
//! `tests/sd18_paladin_level11_aura_of_justice.rs`'s,
//! `tests/sd18_paladin_level12_widening.rs`'s, and
//! `tests/sd18_paladin_level13_widening.rs`'s
//! `paladin_level_1N_is_not_promoted_by_this_slice` controls, all moved to
//! a level-15 boundary in the same commit, mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Ranger/Rogue level-N-to-level-(N+1)
//! sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level13_sd18_widening_deterministic_input.txt"
);

const PALADIN_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level14_sd18_widening_deterministic_input.txt"
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
const AURA_OF_JUSTICE_ID: &str = "class_chassis.paladin.aura_of_justice";
const AURA_OF_FAITH_ID: &str = "class_chassis.paladin.aura_of_faith";

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

// ----- Base attack bonus and all three base saves genuinely rise at level 14 -----

#[test]
fn paladin_level14_base_attack_and_all_saves_genuinely_rise() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 14,
        "Paladin level 14 full-BAB progression must equal 14, genuinely risen from 13: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 9,
        "Paladin level 14 good Fortitude (14/2+2) must genuinely rise to 9: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 4,
        "Paladin level 14 poor Reflex (14/3) must stay 4, an integer-division coincidence: {}",
        reflex.detail
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 9,
        "Paladin level 14 good Will (14/2+2) must genuinely rise to 9: {}",
        will.detail
    );
}

// ----- Smite Evil stays 5/day at level 14 (integer-division coincidence with 13) -----

#[test]
fn paladin_level14_smite_evil_uses_stay_five_damage_rises() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 5,
        "Paladin level 14 Smite Evil must stay 5/day (1 + (14 - 1)/3), an integer-division \
         coincidence with level 13 -- the level-14 \"Special\" column does not name Smite \
         Evil at all: {}",
        uses_per_day.detail
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 14,
        "Paladin level 14 Smite Evil damage bonus (equal to paladin level) must genuinely \
         rise to 14: {}",
        damage_bonus.detail
    );
}

// ----- Base spells per day widen at level 14: only the 4th-level column rises -----

#[test]
fn paladin_level14_base_spells_per_day_match_the_raw_table_row() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "level 14 (`3/2/1/1`): the 1st/2nd/3rd-level columns stay 3/2/1 unchanged, and the \
         4th-level column genuinely rises from 0 to 1"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn paladin_level14_spell_level_access_stays_four() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 4,
        "Paladin level 14 spell-level access must stay 4, unchanged from level 13: {}",
        access.detail
    );

    let dc4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.spell_save_dc.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &dc4.id),
        "the 4th-level spell save DC must stay grounded at level 14"
    );

    let bonus4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.bonus_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &bonus4.id),
        "the 4th-level Charisma bonus-spells record must stay grounded at level 14"
    );

    let total4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &total4.id),
        "the 4th-level total spells-per-day record must stay grounded at level 14"
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
                "no spells are fabricated at paladin level 14: {daily_prep:?}"
            );
        }
    }
}

// ----- No fifth mercy slot is introduced at level 14 (not a repeat-grant level) -----

#[test]
fn paladin_level14_no_fifth_mercy_slot_is_introduced() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, "class_chassis.paladin.mercy_5_choice"),
        "14th is not a repeat-Mercy-grant level (3rd/6th/9th/12th/15th cadence); no fifth \
         mercy slot may be fabricated: {:?}",
        computation.explanations
    );
}

// ----- Aura of Justice carries over unchanged (granted at 11, still granted at 14) -----

#[test]
fn paladin_level14_aura_of_justice_carries_over() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_JUSTICE_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Justice must stay a bounded grant-only identity record (value 0, \
         non-fabricated) at level 14: {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("granted"),
        "Aura of Justice must still claim to be granted at level 14: {}",
        aura.detail
    );
}

// ----- Aura of Faith is newly granted at level 14, a bounded grant-only identity record -----

#[test]
fn paladin_level14_aura_of_faith_is_newly_granted() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_FAITH_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Faith must be a bounded grant-only identity record (value 0, non-fabricated) \
         at level 14: {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("granted"),
        "Aura of Faith must claim to be granted at level 14: {}",
        aura.detail
    );
}

// ----- Aura of Faith is correctly ABSENT below level 14 -----

#[test]
fn paladin_level13_aura_of_faith_is_correctly_absent() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_FAITH_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Faith must compute a correct zero absence below level 14: {}",
        aura.detail
    );
    assert!(
        !aura.detail.to_lowercase().contains("granted"),
        "Aura of Faith must NOT claim to be granted below level 14: {}",
        aura.detail
    );
}

// ----- The bounded Paladin computation stays claim-blocked overall -----

#[test]
fn paladin_level14_still_claim_blocks_overall() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-14 Paladin must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 13 truth is unchanged by this widening -----

#[test]
fn paladin_level13_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 13, "Paladin level 13 base attack bonus must stay 13");

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(uses_per_day.value, 5, "Paladin level 13 Smite Evil must stay 5/day");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 0),
        ],
        "Paladin level 13 base spells per day must stay `3/2/1/0`"
    );
}

// ----- Negative control: level 18 stays unrecognized by this slice -----
// (level 15 was later widened into the supported tranche by SD18's
// cycle-2026-07-15T4300 widening slice, level 16 by SD18's
// cycle-2026-07-15T5400 widening slice, and level 17 by SD18's
// cycle-2026-07-15T10700 widening slice; see
// tests/sd18_paladin_level15_widening.rs,
// tests/sd18_paladin_level16_widening.rs, and
// tests/sd18_paladin_level17_widening.rs for their own boundaries.)

#[test]
fn paladin_level_21_is_not_promoted_by_this_slice() {
    let level_21 = PALADIN_LEVEL14_FIXTURE.replace("class:paladin:14", "class:paladin:21");
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
fn fighter_does_not_gain_paladin_level14_recognition() {
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
fn multiclass_paladin_level14_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL14_FIXTURE.replace(
        "class_level=class:paladin:14",
        "class_level=class:paladin:14\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_paladin_row_names_level_14_widening() {
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
            .contains("sd18_paladin_level14_widening"),
        "paladin row must cite the live SD18 level-14 widening proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 14") || note.contains("level-14"),
        "paladin partial note must name the level-14 widening: {note}"
    );
}
