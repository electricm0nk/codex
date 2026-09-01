//! SD18 Paladin level-13 widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Paladin level-1..level-12
//! hybrid chassis (`tests/sd18_paladin_level12_widening.rs`, the loop's most
//! recent Paladin ceiling) to Paladin level 13 -- the eighth SD-18 §3.2
//! class-row level-13 landing (after Rogue, Barbarian, Fighter, Ranger,
//! Cleric, Druid, and Bard), and the second hybrid/partial-caster class
//! (after Ranger) to reach it (`supported_paladin_level` is generalized
//! from `1..=12` to `1..=13` via `MAX_SUPPORTED_PALADIN_LEVEL = 13`,
//! exactly as prior cycles widened the sibling `MAX_SUPPORTED_*_LEVEL`
//! constants). §3.1 race rows and §3.3 interaction rows are structurally
//! exhausted/blocked (cited in the progress doc, not re-derived this
//! cycle); §3.4/§3.5 are structurally blocked (same root cause, also
//! cited, not re-derived). Monk is a confirmed dead end at level 13
//! (Diamond Soul needs spell resistance, which does not exist anywhere in
//! this codebase).
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! cross-checked against a third mirror (legacy.aonprd.com) and a fourth
//! (roll20.net) once the raw fetches disagreed on the level-12/level-13
//! base spells-per-day 4th-level column (two independent fetches showed a
//! nonsensical decrease from a "1" at level 12 to a "0" at level 13, which
//! cannot be correct for a monotonically non-decreasing spells-per-day
//! table -- a known tool-extraction artifact). d20pfsrd and
//! legacy.aonprd.com agreed byte-for-byte with each other AND with the
//! internally consistent pattern already proven for Paladin's own
//! 1st/2nd/3rd spell-level columns (each new column opens with a literal
//! "0" entry exactly 3 levels after the previous column opened: 4, 7, 10,
//! 13), and exactly match the already-landed Ranger level-13 widening's
//! own base spells-per-day row (Paladin and Ranger share an identical
//! spells-per-day table shape in the PF1 Core Rulebook), so the
//! self-contradictory fetches were rejected as the artifact:
//!
//! - level 13 base attack bonus GENUINELY RISES to +13 (full BAB
//!   progression, up from +12 at level 12; the table's own "+13/+8/+3"
//!   iterative notation is not modeled anywhere in this codebase, only the
//!   flat base value); all three base saves STAY numerically unchanged
//!   from level 12 (good Fortitude/Will `13/2+2 = 8`, poor Reflex
//!   `13/3 = 4`) -- integer-division coincidences, re-verified rather than
//!   assumed.
//! - the PF1 Core Rulebook Paladin class table's level-13 "Special" column
//!   reads only "Smite evil 5/day" (verified independently against both
//!   primary sources) -- this is NOT a new named feature requiring fresh
//!   grounding: the smite-evil-uses-per-day formula
//!   (`1 + (paladin level - 1) / 3`) already grounded since SD13 is
//!   level-generic and already yields 5 at level 13 (`1 + 12/3 = 5`)
//!   without any code change. 13th is NOT one of the repeat-Mercy-grant
//!   levels (3rd/6th/9th/12th/15th cadence), so no fifth mercy slot is
//!   introduced by this widening.
//! - the base spells-per-day table's level-13 row is `3/2/1/0`
//!   (1st/2nd/3rd/4th), verified independently against d20pfsrd and
//!   legacy.aonprd.com: the 1st-level column GENUINELY RISES from 2 to 3
//!   (a literal table lookup value, not a formula), the 2nd/3rd-level
//!   columns stay 2/1 unchanged, and the 4th-level column NEWLY OPENS at 0
//!   (a genuine table entry, not an absence) -- 4th-level paladin spells
//!   begin at paladin level 13 exactly, checked rather than assumed away.
//!   The spell-level access ladder
//!   (`class_chassis.paladin.partial_caster.spell_level_access`)
//!   correspondingly widens from 3 to 4 for the first time
//!   (`PALADIN_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 13`), and the
//!   base spell-save-DC and Charisma-bonus-spells families both extend to
//!   the new 4th spell level automatically (live arithmetic over the
//!   widened access ladder, no new formula invented) -- the exact same
//!   table-lookup-array-widening idiom already used for Ranger's own
//!   level-13 widening.
//!
//! It deliberately does not touch mercy-effect resolution, channel
//! positive energy healing/damage-resolution execution, Divine Bond
//! (weapon bond / mount bond) selection or execution, Aura of Justice's
//! own smite-sharing resolution, or the partial-caster prepared-spell
//! posture burden (all stay named-but-unproven, unchanged from levels
//! 1-12), and it does not ground Paladin level 14+. It also preserves the
//! accepted Paladin level-1..level-12 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.
//!
//! This slice also fixes three pre-existing stale sibling negative
//! controls that this widening would otherwise have broken:
//! `tests/sd13_paladin_level10_progression.rs`'s,
//! `tests/sd18_paladin_level11_aura_of_justice.rs`'s, and
//! `tests/sd18_paladin_level12_widening.rs`'s
//! `paladin_level_13_is_not_promoted_by_this_slice` controls, all moved to
//! a level-14 boundary in the same commit, mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Monk/Ranger/Rogue level-N-to-
//! level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level12_sd18_widening_deterministic_input.txt"
);

const PALADIN_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level13_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus genuinely rises at level 13; all saves stay unchanged -----

#[test]
fn paladin_level13_base_attack_rises_and_saves_stay_unchanged() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 13,
        "Paladin level 13 full-BAB progression must equal 13, genuinely risen from 12: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 8,
        "Paladin level 13 good Fortitude (13/2+2) must stay 8, an integer-division coincidence: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 4,
        "Paladin level 13 poor Reflex (13/3) must stay 4: {}",
        reflex.detail
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 8,
        "Paladin level 13 good Will (13/2+2) must stay 8: {}",
        will.detail
    );
}

// ----- Smite Evil genuinely rises to 5/day at level 13 via the pre-existing formula -----

#[test]
fn paladin_level13_smite_evil_uses_genuinely_rise_to_five() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 5,
        "Paladin level 13 Smite Evil must genuinely rise to 5/day (1 + (13 - 1)/3), matching \
         the PF1 CRB level-13 \"Special\" column (\"Smite evil 5/day\") via the pre-existing \
         level-generic formula: {}",
        uses_per_day.detail
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 13,
        "Paladin level 13 Smite Evil damage bonus (equal to paladin level) must genuinely \
         rise to 13: {}",
        damage_bonus.detail
    );
}

// ----- Base spells per day widen at level 13, including a genuinely new 4th-level column -----

#[test]
fn paladin_level13_base_spells_per_day_match_the_raw_table_row() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 0),
        ],
        "level 13 (`3/2/1/0`): the 1st-level column rises from 2 to 3, the 2nd/3rd-level \
         columns stay 2/1 unchanged, and the 4th-level column newly opens at 0"
    );
}

// ----- The spell-level access ladder widens to 4 for the first time -----

#[test]
fn paladin_level13_spell_level_access_widens_to_four() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 4,
        "Paladin level 13 spell-level access must genuinely widen to 4: {}",
        access.detail
    );

    let dc4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.spell_save_dc.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &dc4.id),
        "the 4th-level spell save DC must be grounded now that the access ladder reaches 4"
    );

    let bonus4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.bonus_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &bonus4.id),
        "the 4th-level Charisma bonus-spells record must be grounded now that the access \
         ladder reaches 4"
    );

    let total4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &total4.id),
        "the 4th-level total spells-per-day record must be grounded now that the access ladder \
         reaches 4"
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
                "no spells are fabricated at paladin level 13: {daily_prep:?}"
            );
        }
    }
}

// ----- No fifth mercy slot is introduced at level 13 (not a repeat-grant level) -----

#[test]
fn paladin_level13_no_fifth_mercy_slot_is_introduced() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, "class_chassis.paladin.mercy_5_choice"),
        "13th is not a repeat-Mercy-grant level (3rd/6th/9th/12th/15th cadence); no fifth \
         mercy slot may be fabricated: {:?}",
        computation.explanations
    );
}

// ----- Aura of Justice carries over unchanged (granted at 11, still granted at 13) -----

#[test]
fn paladin_level13_aura_of_justice_carries_over() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_JUSTICE_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Justice must stay a bounded grant-only identity record (value 0, \
         non-fabricated) at level 13: {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("granted"),
        "Aura of Justice must still claim to be granted at level 13: {}",
        aura.detail
    );
}

// ----- The bounded Paladin computation stays claim-blocked overall -----

#[test]
fn paladin_level13_still_claim_blocks_overall() {
    let input = load(PALADIN_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-13 Paladin must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 12 truth is unchanged by this widening -----

#[test]
fn paladin_level12_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 12, "Paladin level 12 base attack bonus must stay 12");

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(uses_per_day.value, 4, "Paladin level 12 Smite Evil must stay 4/day");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
        ],
        "Paladin level 12 base spells per day must stay `2/2/1` with no 4th-level column"
    );
}

// ----- Negative control: level 18 stays unrecognized by this slice -----
// (level 14 was later widened into the supported tranche by SD18's
// cycle-2026-07-15T2500 widening slice, level 15 by SD18's
// cycle-2026-07-15T4300 widening slice, level 16 by SD18's
// cycle-2026-07-15T5400 widening slice, and level 17 by SD18's
// cycle-2026-07-15T10700 widening slice; see
// tests/sd18_paladin_level14_widening.rs,
// tests/sd18_paladin_level15_widening.rs,
// tests/sd18_paladin_level16_widening.rs, and
// tests/sd18_paladin_level17_widening.rs for their own boundaries.)

#[test]
fn paladin_level_21_is_not_promoted_by_this_slice() {
    let level_21 = PALADIN_LEVEL13_FIXTURE.replace("class:paladin:13", "class:paladin:21");
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
fn fighter_does_not_gain_paladin_level13_recognition() {
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
fn multiclass_paladin_level13_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL13_FIXTURE.replace(
        "class_level=class:paladin:13",
        "class_level=class:paladin:13\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-13 widening -----

#[test]
fn matrix_paladin_row_names_level_13_widening() {
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
            .contains("sd18_paladin_level13_widening"),
        "paladin row must cite the live SD18 level-13 widening proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 13") || note.contains("level-13"),
        "paladin partial note must name the level-13 widening: {note}"
    );
}
