//! SD18 Paladin level-15 widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Paladin level-1..level-14
//! hybrid chassis (`tests/sd18_paladin_level14_widening.rs`, the loop's most
//! recent Paladin ceiling) to Paladin level 15 -- the loop's EIGHTH §3.2
//! class-row level-15 landing (after Barbarian, Rogue, Fighter, Cleric,
//! Druid, Ranger, and Wizard's level-15 landings), and the fourth
//! hybrid/partial-caster class -- after Ranger and, at levels 13/14, itself
//! -- to reach it (`supported_paladin_level` is generalized from `1..=14`
//! to `1..=15` via `MAX_SUPPORTED_PALADIN_LEVEL = 15`, exactly as prior
//! cycles widened the sibling `MAX_SUPPORTED_*_LEVEL` constants). §3.1 race
//! rows and §3.3 interaction rows are structurally exhausted/blocked (cited
//! in the progress doc, not re-derived this cycle); §3.4/§3.5 are
//! structurally blocked (same root cause, also cited, not re-derived). Monk
//! is a confirmed dead end at level 13 (Diamond Soul needs spell
//! resistance, which does not exist anywhere in this codebase) -- not
//! re-attempted.
//!
//! Both primary sources checked for this cycle (d20pfsrd and the Archives
//! of Nethys aonprd.com mirror) agreed byte-for-byte, with no
//! self-contradictory fetches, so a third source was not required:
//!
//! - level 15 base attack bonus GENUINELY RISES to +15 (full BAB
//!   progression, up from +14 at level 14; the table's own "+15/+10/+5"
//!   iterative notation is not modeled anywhere in this codebase, only the
//!   flat base value); good Fortitude and good Will STAY numerically
//!   unchanged at 9 (`15/2+2 = 9`, an integer-division coincidence with
//!   level 14), while poor Reflex GENUINELY RISES to 5 (`15/3 = 5`, up
//!   from 4).
//! - the PF1 Core Rulebook Paladin class table's level-15 "Special" column
//!   reads only "Mercy" (verified independently against both sources) --
//!   15th IS a repeat-Mercy-grant level (the 3rd/6th/9th/12th/15th
//!   cadence), grounded here as a FIFTH numbered mercy choice slot
//!   (`class_chassis.paladin.mercy_5_choice`, gate 15,
//!   `choice:paladin_mercy_5`), mirroring the proven slot-2/3/4 idiom
//!   exactly: an open-ended +0 recognition of whichever raw mercy string
//!   was selected, with no tier-membership validation. Unlike the
//!   6th/9th/12th-level repeat grants, both sources agree the CRB's named
//!   mercy-list tiers stop growing after 12th level -- no new named mercy
//!   condition is introduced at 15th level, so the fifth slot's cited
//!   tier text says exactly that (a fifth pick from the existing pool),
//!   rather than naming a new tier addition. No mercy's effect on lay on
//!   hands is computed.
//! - the base spells-per-day table's level-15 row is `3/2/2/1`
//!   (1st/2nd/3rd/4th), verified independently against both sources: the
//!   1st/2nd/4th-level columns stay 3/2/1 numerically unchanged from level
//!   14, and the 3rd-level column GENUINELY RISES from 1 to 2. The
//!   spell-level access ladder
//!   (`class_chassis.paladin.partial_caster.spell_level_access`) stays 4
//!   (already widened at level 13, unchanged here), and the base
//!   spell-save-DC and Charisma-bonus-spells families both continue to
//!   extend to the 4th spell level automatically (live arithmetic, no new
//!   formula invented).
//!
//! It deliberately does not touch mercy-effect resolution, channel
//! positive energy healing/damage-resolution execution, Divine Bond
//! (weapon bond / mount bond) selection or execution, Aura of Justice's or
//! Aura of Faith's own resolution engines, or the partial-caster
//! prepared-spell posture burden (all stay named-but-unproven, unchanged
//! from levels 1-14), and it does not ground Paladin level 16+. It also
//! preserves the accepted Paladin level-1..level-14 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.
//!
//! This slice also fixes four pre-existing stale sibling negative
//! controls that this widening would otherwise have broken:
//! `tests/sd13_paladin_level10_progression.rs`'s,
//! `tests/sd18_paladin_level11_aura_of_justice.rs`'s,
//! `tests/sd18_paladin_level12_widening.rs`'s, and
//! `tests/sd18_paladin_level13_widening.rs`'s
//! `paladin_level_1N_is_not_promoted_by_this_slice` controls, and
//! `tests/sd18_paladin_level14_widening.rs`'s own level-15 negative
//! control, all moved to a level-16 boundary in the same commit,
//! mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Ranger/Rogue/Wizard
//! level-N-to-level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level14_sd18_widening_deterministic_input.txt"
);

const PALADIN_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level15_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus genuinely rises; poor Reflex genuinely rises; good saves stay -----

#[test]
fn paladin_level15_base_attack_and_reflex_genuinely_rise() {
    let input = load(PALADIN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 15,
        "Paladin level 15 full-BAB progression must equal 15, genuinely risen from 14: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 9,
        "Paladin level 15 good Fortitude (15/2+2) must stay 9, an integer-division \
         coincidence with level 14: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 5,
        "Paladin level 15 poor Reflex (15/3) must genuinely rise to 5, up from 4: {}",
        reflex.detail
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 9,
        "Paladin level 15 good Will (15/2+2) must stay 9, an integer-division coincidence \
         with level 14: {}",
        will.detail
    );
}

// ----- Smite Evil stays 5/day at level 15 (integer-division coincidence with 13/14) -----

#[test]
fn paladin_level15_smite_evil_uses_stay_five_damage_rises() {
    let input = load(PALADIN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 5,
        "Paladin level 15 Smite Evil must stay 5/day (1 + (15 - 1)/3), an integer-division \
         coincidence with levels 13-14 -- the level-15 \"Special\" column does not name Smite \
         Evil at all: {}",
        uses_per_day.detail
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 15,
        "Paladin level 15 Smite Evil damage bonus (equal to paladin level) must genuinely \
         rise to 15: {}",
        damage_bonus.detail
    );
}

// ----- Base spells per day widen at level 15: only the 3rd-level column rises -----

#[test]
fn paladin_level15_base_spells_per_day_match_the_raw_table_row() {
    let input = load(PALADIN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "level 15 (`3/2/2/1`): the 1st/2nd/4th-level columns stay 3/2/1 unchanged, and the \
         3rd-level column genuinely rises from 1 to 2"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn paladin_level15_spell_level_access_stays_four() {
    let input = load(PALADIN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 4,
        "Paladin level 15 spell-level access must stay 4, unchanged from level 14: {}",
        access.detail
    );

    let dc4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.spell_save_dc.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &dc4.id),
        "the 4th-level spell save DC must stay grounded at level 15"
    );

    let bonus4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.bonus_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &bonus4.id),
        "the 4th-level Charisma bonus-spells record must stay grounded at level 15"
    );

    let total4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &total4.id),
        "the 4th-level total spells-per-day record must stay grounded at level 15"
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
                "no spells are fabricated at paladin level 15: {daily_prep:?}"
            );
        }
    }
}

// ----- The FIFTH mercy slot is newly recognized at level 15 (repeat-grant level) -----

#[test]
fn paladin_level15_fifth_mercy_slot_is_newly_recognized() {
    let input = load(PALADIN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_5 = explanation(&computation, MERCY_5_CHOICE_ID);
    assert_eq!(
        slot_5.value, 0,
        "mercy slot 5 recognition must carry no fabricated mechanical value at level 15: {}",
        slot_5.detail
    );
    assert!(
        slot_5.detail.contains("mercy:deafened")
            && slot_5.detail.contains("choice:paladin_mercy_5"),
        "mercy slot 5 must name the raw chosen mercy string: {}",
        slot_5.detail
    );
}

// ----- No fifth mercy slot is introduced at level 14 (not yet the repeat-grant level) -----

#[test]
fn paladin_level14_fifth_mercy_slot_is_not_yet_introduced() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, MERCY_5_CHOICE_ID),
        "mercy slot 5 (gate 15) must be silent at level 14: {:?}",
        computation.explanations
    );
}

// ----- Aura of Faith carries over unchanged (granted at 14, still granted at 15) -----

#[test]
fn paladin_level15_aura_of_faith_carries_over() {
    let input = load(PALADIN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_FAITH_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Faith must stay a bounded grant-only identity record (value 0, \
         non-fabricated) at level 15: {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("granted"),
        "Aura of Faith must still claim to be granted at level 15: {}",
        aura.detail
    );
}

// ----- The bounded Paladin computation stays claim-blocked overall -----

#[test]
fn paladin_level15_still_claim_blocks_overall() {
    let input = load(PALADIN_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-15 Paladin must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 14 truth is unchanged by this widening -----

#[test]
fn paladin_level14_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 14, "Paladin level 14 base attack bonus must stay 14");

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(uses_per_day.value, 5, "Paladin level 14 Smite Evil must stay 5/day");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "Paladin level 14 base spells per day must stay `3/2/1/1`"
    );
}

// ----- Negative control: the paladin path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_level15_recognition() {
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
fn multiclass_paladin_level15_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL15_FIXTURE.replace(
        "class_level=class:paladin:15",
        "class_level=class:paladin:15\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_paladin_row_names_level_15_widening() {
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
            .contains("sd18_paladin_level15_widening"),
        "paladin row must cite the live SD18 level-15 widening proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "paladin partial note must name the level-15 widening: {note}"
    );
}
