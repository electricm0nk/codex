//! SD18 Paladin level-20 widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Paladin level-1..level-19
//! hybrid chassis (`tests/sd18_paladin_level19_widening.rs`, the loop's most
//! recent Paladin ceiling) to Paladin level 20 -- the loop's SIXTH §3.2
//! class-row level-20 landing (after Cleric, Wizard, Barbarian, Bard, and
//! Fighter), and alphabetically the first eligible of the four remaining
//! level-20 candidates (Paladin, Ranger, Rogue, Sorcerer) named in the prior
//! (Fighter) cycle's own recommendation
//! (`supported_paladin_level` is generalized from `1..=19` to `1..=20` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 20`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants -- level 20 is the final level
//! within PF1's 1-20 character-level cap, so this closes Paladin's own
//! per-level arithmetic-widening frontier). §3.1 race rows and §3.3
//! interaction rows are structurally exhausted/blocked (cited in the
//! progress doc, not re-derived this cycle); §3.4/§3.5 are structurally
//! blocked (same root cause, also cited, not re-derived). Monk is capped at
//! level 12 and Druid is capped at level 15, both documented structural
//! exceptions -- not re-attempted.
//!
//! Both primary sources checked for this cycle (a raw `curl` fetch of
//! d20pfsrd.com's own class table HTML, bypassing AI-summarization, and a
//! raw `curl` fetch of the Archives of Nethys aonprd.com mirror's
//! `ClassDisplay.aspx` HTML) agreed byte-for-byte on the level-20 row:
//! "+20/+15/+10/+5 | +12 | +6 | +12 | Holy champion | 4 | 4 | 3 | 3".
//!
//! - level 20 base attack bonus GENUINELY RISES to +20 (full BAB
//!   progression, up from +19 at level 19); BOTH good saves (Fortitude,
//!   Will) GENUINELY RISE to +12 (`20/2+2=12`, up from +11 at level 19)
//!   while poor Reflex STAYS +6 (`20/3=6`, an integer-division coincidence
//!   with level 19) -- checked directly against both primary sources'
//!   raw HTML rather than assumed.
//! - the PF1 Core Rulebook Paladin class table's level-20 "Special" column
//!   reads only "Holy champion" -- the class capstone, a genuinely NEW
//!   class feature, grounded as a FOURTH bounded grant-only identity record
//!   mirroring the Aura of Justice / Aura of Faith / Aura of Righteousness
//!   idiom exactly: no damage-reduction-application engine, no
//!   banishment-spell-effect-resolution engine, and no
//!   healing-maximization execution engine exists anywhere in this
//!   codebase to apply "her DR increases to 10/evil... the outsider is
//!   also subject to a banishment... whenever she channels positive energy
//!   or uses lay on hands to heal a creature, she heals the maximum
//!   possible amount" to. 20th is NOT one of the repeat-Mercy-grant levels
//!   (3rd/6th/9th/12th/15th/18th cadence; the next lands at 21st, out of
//!   scope), so no seventh mercy slot is introduced. Smite Evil's
//!   uses-per-day formula (`1 + (paladin level - 1) / 3`, already
//!   level-generic and already proven) STAYS at its 7/day ceiling
//!   (`1 + (20-1)/3 = 7`, an integer-division coincidence with level 19),
//!   while its damage bonus (equal to paladin level) GENUINELY RISES to
//!   20.
//! - the base spells-per-day table's level-20 row is `4/4/3/3`
//!   (1st/2nd/3rd/4th), verified independently against both sources: the
//!   1st/3rd-level columns stay 4/3 numerically unchanged, and the
//!   2nd-level AND 4th-level columns BOTH genuinely rise simultaneously
//!   (2nd from 3 to 4, 4th from 2 to 3) -- the first level in this row's
//!   own widening history where two columns rise at once, a deliberate
//!   deviation from the single-column-rise pattern seen at every prior
//!   level (levels 13/14/15/16/17/18/19 each rose exactly one column), so
//!   both raw HTML fetches were checked directly (not AI-summarized) to
//!   guard against a tool-extraction artifact; no disagreement was found.
//!   The spell-level access ladder
//!   (`class_chassis.paladin.partial_caster.spell_level_access`) stays 4
//!   (already widened at level 13, unchanged here), and the base
//!   spell-save-DC and Charisma-bonus-spells families both continue to
//!   extend to the 4th spell level automatically (live arithmetic, no new
//!   formula invented).
//!
//! No new record type or choice slot is added this slice beyond the one new
//! Holy Champion grant-only identity record -- the Smite Evil rise falls
//! straight out of the already-generalized, already-proven formula, and the
//! base-spells-per-day widening is only one new match-arm entry on the
//! existing table lookup.
//!
//! It deliberately does not touch mercy-effect resolution, channel
//! positive energy healing/damage-resolution execution, Divine Bond
//! (weapon bond / mount bond) selection or execution, Aura of Justice's,
//! Aura of Faith's, Aura of Righteousness's, or Holy Champion's own
//! resolution engines, or the partial-caster prepared-spell posture burden
//! (all stay named-but-unproven, unchanged from levels 1-19). It also
//! preserves the accepted Paladin level-1..level-19 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.
//!
//! This slice also fixes five pre-existing stale sibling negative
//! controls that this widening would otherwise have broken:
//! `tests/sd13_paladin_level10_progression.rs`'s,
//! `tests/sd18_paladin_level11_aura_of_justice.rs`'s,
//! `tests/sd18_paladin_level12_widening.rs`'s,
//! `tests/sd18_paladin_level13_widening.rs`'s, and
//! `tests/sd18_paladin_level14_widening.rs`'s
//! `paladin_level_2N_is_not_promoted_by_this_slice` controls, all moved to
//! a level-21 boundary in the same commit (a pure implementation-gate
//! check, since PF1 has no 21st character level); `tests/sd18_paladin_level19_widening.rs`'s
//! own level-20 negative control is REMOVED rather than moved, since
//! level 20 is now itself the supported/grounded row rather than the
//! out-of-range boundary, mirroring the Barbarian/Cleric/Wizard/Bard/
//! Fighter level-N-to-level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level19_sd18_widening_deterministic_input.txt"
);

const PALADIN_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level20_sd18_widening_deterministic_input.txt"
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
const AURA_OF_RIGHTEOUSNESS_ID: &str = "class_chassis.paladin.aura_of_righteousness";
const HOLY_CHAMPION_ID: &str = "class_chassis.paladin.holy_champion";
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

// ----- Base attack bonus and both good saves genuinely rise; poor Reflex stays put -----

#[test]
fn paladin_level20_base_attack_and_good_saves_genuinely_rise() {
    let input = load(PALADIN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 20,
        "Paladin level 20 full-BAB progression must genuinely rise to 20, up from 19: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 12,
        "Paladin level 20 good Fortitude (20/2+2) must genuinely rise to 12, up from 11: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 6,
        "Paladin level 20 poor Reflex (20/3) must stay 6, an integer-division coincidence with \
         level 19: {}",
        reflex.detail
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 12,
        "Paladin level 20 good Will (20/2+2) must genuinely rise to 12, up from 11: {}",
        will.detail
    );
}

// ----- Smite Evil stays at its 7/day ceiling; damage bonus genuinely rises -----

#[test]
fn paladin_level20_smite_evil_uses_stay_but_damage_bonus_rises() {
    let input = load(PALADIN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 7,
        "Paladin level 20 Smite Evil must stay at its 7/day ceiling (1 + (20 - 1)/3), an \
         integer-division coincidence with level 19: {}",
        uses_per_day.detail
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 20,
        "Paladin level 20 Smite Evil damage bonus (equal to paladin level) must genuinely rise \
         to 20: {}",
        damage_bonus.detail
    );
}

// ----- Base spells per day widen at level 20: the 2nd and 4th columns both rise -----

#[test]
fn paladin_level20_base_spells_per_day_match_the_raw_table_row() {
    let input = load(PALADIN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 3),
        ],
        "level 20 (`4/4/3/3`): the 1st/3rd-level columns stay 4/3 unchanged, and the 2nd-level \
         and 4th-level columns both genuinely rise (2nd from 3 to 4, 4th from 2 to 3)"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn paladin_level20_spell_level_access_stays_four() {
    let input = load(PALADIN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(&computation, SPELL_LEVEL_ACCESS_ID);
    assert_eq!(
        access.value, 4,
        "Paladin level 20 spell-level access must stay 4, unchanged from level 19: {}",
        access.detail
    );

    let dc4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.spell_save_dc.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &dc4.id),
        "the 4th-level spell save DC must stay grounded at level 20"
    );

    let bonus4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.bonus_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &bonus4.id),
        "the 4th-level Charisma bonus-spells record must stay grounded at level 20"
    );

    let total4 = explanation(
        &computation,
        "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &total4.id),
        "the 4th-level total spells-per-day record must stay grounded at level 20"
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
                "no spells are fabricated at paladin level 20: {daily_prep:?}"
            );
        }
    }
}

// ----- No seventh mercy slot is introduced at level 20 (not a repeat-grant level) -----

#[test]
fn paladin_level20_does_not_introduce_a_seventh_mercy_slot() {
    let input = load(PALADIN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.paladin.mercy_7_choice"),
        "level 20 is not one of the 3rd/6th/9th/12th/15th/18th repeat-Mercy-grant levels, so no \
         seventh mercy slot should be introduced: {:?}",
        computation.explanations
    );

    // The sixth mercy slot, granted at level 18, must still carry over unchanged.
    let slot_6 = explanation(&computation, MERCY_6_CHOICE_ID);
    assert_eq!(
        slot_6.value, 0,
        "mercy slot 6 recognition must carry no fabricated mechanical value at level 20: {}",
        slot_6.detail
    );
    assert!(
        slot_6.detail.contains("sickened"),
        "mercy slot 6 must recognize the deterministic fixture's raw selection: {}",
        slot_6.detail
    );
}

// ----- Aura of Righteousness carries over unchanged (granted at 17, still granted at 20) -----

#[test]
fn paladin_level20_aura_of_righteousness_carries_over() {
    let input = load(PALADIN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let aura = explanation(&computation, AURA_OF_RIGHTEOUSNESS_ID);
    assert_eq!(
        aura.value, 0,
        "Aura of Righteousness must stay a bounded grant-only identity record (value 0, \
         non-fabricated) at level 20: {}",
        aura.detail
    );
    assert!(
        aura.detail.to_lowercase().contains("granted"),
        "Aura of Righteousness must still claim to be granted at level 20: {}",
        aura.detail
    );

    // The DR clause's real magnitude carries over too. PF1 gives Paladin a
    // flat DR 5/evil with no further progression, so 20 must read the same 5
    // as 17 -- pinned here so a future tiering change cannot silently drift it.
    let dr = explanation(&computation, "class_chassis.paladin.damage_reduction");
    assert_eq!(
        dr.value, 5,
        "Paladin DR stays a flat 5/evil at level 20 -- PF1 grants no further tier: {}",
        dr.detail
    );
}

// ----- Holy Champion is newly granted at level 20 -----

#[test]
fn paladin_level20_holy_champion_is_newly_granted() {
    let input = load(PALADIN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let holy_champion = explanation(&computation, HOLY_CHAMPION_ID);
    assert_eq!(
        holy_champion.value, 0,
        "Holy Champion must be a bounded grant-only identity record (value 0, non-fabricated) \
         at level 20: {}",
        holy_champion.detail
    );
    assert!(
        holy_champion.detail.to_lowercase().contains("granted"),
        "Holy Champion must claim to be granted at level 20: {}",
        holy_champion.detail
    );

    // Below the level-20 gate, level 19 must carry a correct absence record instead.
    let level19_input = load(PALADIN_LEVEL19_FIXTURE);
    let level19_computation = compute_pilot_base_chassis(&level19_input);
    let level19_holy_champion = explanation(&level19_computation, HOLY_CHAMPION_ID);
    assert_eq!(
        level19_holy_champion.value, 0,
        "Holy Champion at level 19 must be a correct level-gate absence (value 0): {}",
        level19_holy_champion.detail
    );
    assert!(
        level19_holy_champion.detail.to_lowercase().contains("absent")
            || level19_holy_champion.detail.to_lowercase().contains("correctly"),
        "Holy Champion at level 19 must name the correct absence, not a grant: {}",
        level19_holy_champion.detail
    );
}

// ----- The bounded Paladin computation stays claim-blocked overall -----

#[test]
fn paladin_level20_still_claim_blocks_overall() {
    let input = load(PALADIN_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-20 Paladin must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 19 truth is unchanged by this widening -----

#[test]
fn paladin_level19_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 19, "Paladin level 19 base attack bonus must stay 19");

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(uses_per_day.value, 7, "Paladin level 19 Smite Evil must stay 7/day");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 2),
        ],
        "Paladin level 19 base spells per day must stay `4/3/3/2`"
    );
}

// ----- Negative control: level 21 stays unrecognized (PF1 has no 21st character level) -----

#[test]
fn paladin_level_21_is_not_promoted_by_this_slice() {
    let level_21 = PALADIN_LEVEL20_FIXTURE.replace("class:paladin:20", "class:paladin:21");
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
fn fighter_does_not_gain_paladin_level20_recognition() {
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
fn multiclass_paladin_level20_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL20_FIXTURE.replace(
        "class_level=class:paladin:20",
        "class_level=class:paladin:20\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_paladin_row_names_level_20_widening() {
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
            .contains("sd18_paladin_level20_widening"),
        "paladin row must cite the live SD18 level-20 widening proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "paladin partial note must name the level-20 widening: {note}"
    );
}
