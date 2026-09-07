//! SD18 Paladin level-19 widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Paladin level-1..level-18
//! hybrid chassis (`tests/sd18_paladin_level18_widening.rs`, the loop's most
//! recent Paladin ceiling) to Paladin level 19 -- the loop's FIFTH §3.2
//! class-row level-19 landing (after Barbarian, Cleric, Fighter, and Bard),
//! and alphabetically the first of the five remaining eligible classes
//! (Ranger, Rogue, Sorcerer, Wizard, and, before this cycle, Paladin) picked
//! per the prior cycle's own "Next cycle instructions"
//! (`supported_paladin_level` is generalized from `1..=18` to `1..=19` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 19`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants). §3.1 race rows and §3.3
//! interaction rows are structurally exhausted/blocked (cited in the
//! progress doc, not re-derived this cycle); §3.4/§3.5 are structurally
//! blocked (same root cause, also cited, not re-derived). Monk is capped at
//! level 12 and Druid is capped at level 15, both documented structural
//! exceptions -- not re-attempted.
//!
//! Both primary sources checked for this cycle (a raw HTML parse of
//! d20pfsrd.com's own class table, bypassing AI-summarization, and the
//! Archives of Nethys aonprd.com mirror via `ClassDisplay.aspx`) agreed
//! byte-for-byte on the full levels-15-through-20 block fetched in one pass
//! (guards against level-misattribution): level 17 "Aura of righteousness",
//! level 18 "Mercy", level 19 "Smite evil 7/day", level 20 "Holy champion"
//! -- both sources agree byte-for-byte on the level-19 row
//! ("+19/+14/+9/+4 | +11 | +6 | +11 | Smite evil 7/day | 4/3/3/2"), and no
//! source disagreement was found (a third source was not required).
//!
//! - level 19 base attack bonus GENUINELY RISES to +19 (full BAB
//!   progression, up from +18 at level 18); BOTH good saves (Fortitude,
//!   Will) STAY +11 (`19/2+2=11`, an integer-division coincidence with
//!   level 18) and poor Reflex STAYS +6 (`19/3=6`, also an
//!   integer-division coincidence with level 18) -- checked directly
//!   against both primary sources rather than assumed.
//! - the PF1 Core Rulebook Paladin class table's level-19 "Special" column
//!   reads only "Smite evil 7/day" -- 19th is NOT one of the
//!   repeat-Mercy-grant levels (3rd/6th/9th/12th/15th/18th cadence; the
//!   next repeat grant lands at 21st, out of scope), so no seventh mercy
//!   choice slot is introduced this cycle. Smite Evil's uses-per-day
//!   formula (`1 + (paladin level - 1) / 3`, already level-generic and
//!   already proven) GENUINELY RISES to 7/day (`1 + (19-1)/3 = 7`, up from
//!   6/day at level 18 -- exactly the ceiling the level-18 cycle's own doc
//!   comment already predicted, "the next rise lands at level 19"), while
//!   its damage bonus (equal to paladin level) GENUINELY RISES to 19.
//! - the base spells-per-day table's level-19 row is `4/3/3/2`
//!   (1st/2nd/3rd/4th), verified independently against both sources: the
//!   1st/2nd/4th-level columns stay 4/3/2 numerically unchanged from
//!   level 18, and the 3rd-level column GENUINELY RISES from 2 to 3. The
//!   spell-level access ladder
//!   (`class_chassis.paladin.partial_caster.spell_level_access`) stays 4
//!   (already widened at level 13, unchanged here), and the base
//!   spell-save-DC and Charisma-bonus-spells families both continue to
//!   extend to the 4th spell level automatically (live arithmetic, no new
//!   formula invented).
//!
//! No new record type or choice slot is added this slice -- only one new
//! entry on the existing `paladin_base_spells_per_day` table lookup match
//! arm; the Smite Evil rise falls straight out of the already-generalized,
//! already-proven formula with no code change beyond widening the level
//! cap.
//!
//! It deliberately does not touch mercy-effect resolution, channel
//! positive energy healing/damage-resolution execution, Divine Bond
//! (weapon bond / mount bond) selection or execution, Aura of Justice's,
//! Aura of Faith's, or Aura of Righteousness's own resolution engines, or
//! the partial-caster prepared-spell posture burden (all stay
//! named-but-unproven, unchanged from levels 1-18), and it does not ground
//! Paladin level 20. It also preserves the accepted Paladin
//! level-1..level-18 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.
//!
//! This slice also fixes five pre-existing stale sibling negative
//! controls that this widening would otherwise have broken:
//! `tests/sd13_paladin_level10_progression.rs`'s,
//! `tests/sd18_paladin_level11_aura_of_justice.rs`'s,
//! `tests/sd18_paladin_level12_widening.rs`'s,
//! `tests/sd18_paladin_level13_widening.rs`'s, and
//! `tests/sd18_paladin_level14_widening.rs`'s
//! `paladin_level_19_is_not_promoted_by_this_slice` controls, all moved to
//! a level-20 boundary in the same commit; `tests/sd18_paladin_level18_widening.rs`'s
//! own level-19 negative control is REMOVED rather than moved, since
//! level 19 is now itself the supported/grounded row rather than the
//! out-of-range boundary, mirroring the Barbarian/Cleric/Fighter/Bard
//! level-N-to-level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level18_sd18_widening_deterministic_input.txt"
);

const PALADIN_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level19_sd18_widening_deterministic_input.txt"
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
const MERCY_6_CHOICE_ID: &str = "class_chassis.paladin.mercy_6_choice";

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

// ----- Base attack bonus genuinely rises; all three base saves stay put (coincidence) -----

#[test]
fn paladin_level19_base_attack_genuinely_rises_saves_stay_put() {
    let input = load(PALADIN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 19,
        "Paladin level 19 full-BAB progression must genuinely rise to 19, up from 18: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 11,
        "Paladin level 19 good Fortitude (19/2+2) must stay 11, an integer-division coincidence \
         with level 18: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 6,
        "Paladin level 19 poor Reflex (19/3) must stay 6, an integer-division coincidence with \
         level 18: {}",
        reflex.detail
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 11,
        "Paladin level 19 good Will (19/2+2) must stay 11, an integer-division coincidence with \
         level 18: {}",
        will.detail
    );
}

// ----- Smite Evil genuinely rises to its 7/day ceiling; damage bonus genuinely rises -----

#[test]
fn paladin_level19_smite_evil_uses_and_damage_bonus_genuinely_rise() {
    let input = load(PALADIN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 7,
        "Paladin level 19 Smite Evil must genuinely rise to 7/day (1 + (19 - 1)/3), the PF1 \
         Core Rulebook's stated ceiling: {}",
        uses_per_day.detail
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 19,
        "Paladin level 19 Smite Evil damage bonus (equal to paladin level) must genuinely rise \
         to 19: {}",
        damage_bonus.detail
    );
}

// ----- Base spells per day widen at level 19: only the 3rd-level column rises -----

#[test]
fn paladin_level19_base_spells_per_day_match_the_raw_table_row() {
    let input = load(PALADIN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 2),
        ],
        "level 19 (`4/3/3/2`): the 1st/2nd/4th-level columns stay 4/3/2 unchanged, and the \
         3rd-level column genuinely rises from 2 to 3"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn paladin_level19_spell_level_access_stays_four() {
    let input = load(PALADIN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 4,
        "Paladin level 19 spell-level access must stay 4, unchanged from level 18: {}",
        access.detail
    );

    let dc4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.spell_save_dc.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &dc4.id),
        "the 4th-level spell save DC must stay grounded at level 19"
    );

    let bonus4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.bonus_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &bonus4.id),
        "the 4th-level Charisma bonus-spells record must stay grounded at level 19"
    );

    let total4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &total4.id),
        "the 4th-level total spells-per-day record must stay grounded at level 19"
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
                "no spells are fabricated at paladin level 19: {daily_prep:?}"
            );
        }
    }
}

// ----- No seventh mercy slot is introduced at level 19 (not a repeat-grant level) -----

#[test]
fn paladin_level19_does_not_introduce_a_seventh_mercy_slot() {
    let input = load(PALADIN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.paladin.mercy_7_choice"),
        "level 19 is not one of the 3rd/6th/9th/12th/15th/18th repeat-Mercy-grant levels, so no \
         seventh mercy slot should be introduced: {:?}",
        computation.explanations
    );

    // The sixth mercy slot, granted at level 18, must still carry over unchanged.
    let slot_6 = explanation(&computation, MERCY_6_CHOICE_ID);
    assert_eq!(
        slot_6.value, 0,
        "mercy slot 6 recognition must carry no fabricated mechanical value at level 19: {}",
        slot_6.detail
    );
    assert!(
        slot_6.detail.contains("sickened"),
        "mercy slot 6 must recognize the deterministic fixture's raw selection: {}",
        slot_6.detail
    );
}

// ----- The bounded Paladin computation stays claim-blocked overall -----

#[test]
fn paladin_level19_still_claim_blocks_overall() {
    let input = load(PALADIN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-19 Paladin must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 18 truth is unchanged by this widening -----

#[test]
fn paladin_level18_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 18, "Paladin level 18 base attack bonus must stay 18");

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(uses_per_day.value, 6, "Paladin level 18 Smite Evil must stay 6/day");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 2),
        ],
        "Paladin level 18 base spells per day must stay `4/3/2/2`"
    );
}

// ----- Negative control: the paladin path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_level19_recognition() {
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
fn multiclass_paladin_level19_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL19_FIXTURE.replace(
        "class_level=class:paladin:19",
        "class_level=class:paladin:19\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-19 widening -----

#[test]
fn matrix_paladin_row_names_level_19_widening() {
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
            .contains("sd18_paladin_level19_widening"),
        "paladin row must cite the live SD18 level-19 widening proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 19") || note.contains("level-19"),
        "paladin partial note must name the level-19 widening: {note}"
    );
}
