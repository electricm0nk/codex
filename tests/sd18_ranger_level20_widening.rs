//! SD18 Ranger level-20 fifth Favored Enemy / Master Hunter widening
//! grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-19
//! hybrid chassis (`tests/sd18_ranger_level19_widening.rs`, the loop's most
//! recent Ranger ceiling) to Ranger level 20
//! (`supported_ranger_level` is generalized from `1..=19` to `1..=20` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 20`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants) — the loop's SEVENTH §3.2
//! level-20 landing, after Cleric, Wizard, Barbarian, Bard, Fighter, and
//! Paladin, and the FINAL level within PF1's 1-20 character-level cap for
//! this class row.
//!
//! §3.1 race rows and §3.3 interaction rows are structurally
//! exhausted/blocked (cited in the progress doc, not re-derived this
//! cycle); §3.4/§3.5 are structurally blocked (same root cause, also
//! cited, not re-derived). Druid stays capped at level 15 (a confirmed
//! permanent dead end -- widening to level 16 would establish a brand-new
//! Wild Shape frequency pillar with no existing precedent to widen) and
//! Monk at level 12 (a confirmed permanent dead end -- level 13's sole
//! change is Diamond Soul, which needs a spell-resistance concept this
//! codebase has never grounded), both documented structural exceptions,
//! not re-attempted here.
//!
//! Ranger was picked per the prior cycle's own "Next cycle instructions"
//! (alphabetically first among the three remaining §3.2 level-20
//! candidates: Ranger, Rogue, Sorcerer), verified fresh rather than
//! assumed. Both PF1 CRB primary sources (d20pfsrd and the Archives of
//! Nethys aonprd.com mirror) were fetched directly via raw curl + tag-strip
//! (not summarized WebFetch) before writing any code or test, and both
//! agree byte-for-byte on the level-20 class table row: BAB
//! +20/+15/+10/+5 (full-BAB progression, base value 20), Fort +12, Ref
//! +12, Will +6, Special "5th favored enemy, master hunter", base spells
//! per day 4/4/3/3 (1st/2nd/3rd/4th). The level-18/19 neighboring rows
//! were fetched in the same pass to rule out level misattribution: level
//! 18 = "4th favored terrain, combat style feat", level 19 = "Improved
//! quarry" — both distinct from level 20's own row, confirming this is
//! genuinely level 20's own row, not an adjacent level's text.
//!
//! - level 20 base attack bonus GENUINELY RISES to 20 (full BAB, up from
//!   19 at level 19); both good saves (Fortitude, Reflex) GENUINELY RISE
//!   to 12 (`20/2+2 = 12`, up from 11 at level 19); poor Will STAYS 6
//!   (`20/3 = 6`, an integer-division coincidence with level 19).
//! - the base spells-per-day table's level-20 row is `4/4/3/3`
//!   (1st/2nd/3rd/4th), verified independently against both primary
//!   sources: the 1st/3rd-level columns stay 4/3 unchanged from level 19,
//!   and the 2nd/4th-level columns BOTH GENUINELY RISE (2nd: 3 -> 4, 4th:
//!   2 -> 3) — two columns rising at once, a literal table lookup value,
//!   not a formula.
//! - "5th favored enemy" is the Favored Enemy rule's own 5th interval
//!   (5th, 10th, 15th, 20th ranger level; the last one, so no further
//!   interval remains within PF1's 1-20 level cap), the exact structural
//!   mirror of the already-grounded 2nd/3rd/4th interval widenings: a
//!   fifth favored-enemy TYPE selection (`choice:ranger_favored_enemy_5`,
//!   mirroring the open-ended, non-restricted-list recognition idiom
//!   exactly) plus this interval's own restricted bonus-increase TARGET
//!   choice (`choice:ranger_favored_enemy_bonus_increase_target_4`,
//!   widened to the five-enemy set). The fixture targets the SECOND
//!   favored enemy at this interval, so the second favored enemy's own
//!   magnitude genuinely rises from +2 (level 19) to +4 (level 20) while
//!   the first and fourth favored enemies' magnitudes stay unchanged (a
//!   real, checked case, not an assumption) and the new fifth favored
//!   enemy grounds at the flat +2 base (not targeted by this interval's
//!   own increase).
//! - "master hunter" is a brand-new 20th-level capstone with no player
//!   choice involved (verified independently, both primary sources
//!   byte-for-byte identical: "A ranger of 20th level becomes a master
//!   hunter. He can always move at full speed while using Survival to
//!   follow tracks without penalty. He can, as a standard action, make a
//!   single attack against a favored enemy at his full attack bonus. If
//!   the attack hits, the target takes damage normally and must make a
//!   Fortitude save or die..."), grounded as a bounded grant-only
//!   identity record (value 0, mirroring the Paladin Holy Champion
//!   capstone idiom exactly, including the level-gate absence/grant
//!   if/else shape) — no action-economy engine, no attack-resolution
//!   engine, and no saving-throw-resolution engine exists anywhere in
//!   this codebase, so this grounds no actual mechanic.
//!
//! It deliberately does not touch the favored-terrain/favored-enemy
//! conditional-application engines (target-type matching, whether any
//! specific check or attack is actually made against a favored enemy),
//! any of the five combat-style bonus feats' own mechanics, Hunter's Bond
//! ally-bonus application or the animal-companion form, Woodland
//! Stride's/Swift Tracker's/Camouflage's/Hide in Plain Sight's own
//! application, the ranger Wisdom prepared-posture/spell-source-lineage
//! burden (all stay named-but-unproven, unchanged from levels 1-19), any
//! actual save-resolution/damage-resolution math for Evasion/Improved
//! Evasion or Master Hunter, nor any target-selection/conditional-
//! application engine for Quarry's or Improved Quarry's own effects. It
//! also preserves the accepted Ranger level-1..level-19 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.
//! This closes Ranger's own per-level arithmetic-widening frontier: level
//! 20 is the final level within PF1's 1-20 character-level cap.
//!
//! This slice also fixes five pre-existing stale sibling negative controls
//! that this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s,
//! `tests/sd18_ranger_level11_quarry.rs`'s,
//! `tests/sd18_ranger_level12_widening.rs`'s,
//! `tests/sd18_ranger_level13_widening.rs`'s, and
//! `tests/sd18_ranger_level14_widening.rs`'s own
//! `ranger_level_20_is_not_promoted_by_this_slice`, all moved to a
//! level-21 boundary in the same commit (a pure implementation-gate check,
//! since PF1 has no 21st character level);
//! `tests/sd18_ranger_level19_widening.rs`'s own level-20 negative-control
//! test is removed rather than moved, since level 20 is now itself the
//! supported/grounded row, mirroring the
//! Barbarian/Bard/Cleric/Fighter/Paladin level-N-to-level-(N+1) sibling-fix
//! precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level19_sd18_improved_quarry_deterministic_input.txt"
);

const RANGER_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level20_sd18_fifth_favored_enemy_and_master_hunter_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

const MASTER_HUNTER_ID: &str = "class_feature.ranger.master_hunter";
const FAVORED_ENEMY_1_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_skill_bonus";
const FAVORED_ENEMY_2_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_2_skill_bonus";
const FAVORED_ENEMY_4_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_4_skill_bonus";
const FAVORED_ENEMY_5_CHOICE_ID: &str = "class_chassis.ranger.favored_enemy_5_choice";
const FAVORED_ENEMY_5_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_5_skill_bonus";
const FAVORED_ENEMY_5_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_5_attack_damage_bonus";
const FAVORED_ENEMY_BONUS_INCREASE_4_CHOICE_ID: &str =
    "class_chassis.ranger.favored_enemy_bonus_increase_4_choice";

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

// ----- Base attack bonus genuinely rises at level 20 -----

#[test]
fn ranger_level20_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 20,
        "Ranger level 20 full-BAB progression must equal 20, genuinely risen from 19: {}",
        base_attack.detail
    );
}

// ----- Both good saves genuinely rise; poor Will stays unchanged -----

#[test]
fn ranger_level20_base_saves() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 12,
        "Ranger level 20 good Fortitude (20/2+2) must genuinely rise to 12, up from 11"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(
        reflex.value, 12,
        "Ranger level 20 good Reflex (20/2+2) must genuinely rise to 12, up from 11"
    );

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(
        will.value, 6,
        "Ranger level 20 poor Will (20/3) must stay 6, unchanged from level 19"
    );
}

// ----- Base spells per day widen at level 20: 2nd and 4th columns both rise -----

#[test]
fn ranger_level20_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 3),
        ],
        "level 20 (`4/4/3/3`): the 1st/3rd-level columns stay 4/3 unchanged, and the \
         2nd/4th-level columns both genuinely rise (3 -> 4, 2 -> 3)"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn ranger_level20_spell_level_access_stays_four() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_level_access",
    );
    assert_eq!(
        access.value, 4,
        "Ranger level 20 spell-level access must stay 4 (already widened at level 13): {}",
        access.detail
    );
}

// ----- The fifth favored enemy is granted as a choice recognition record -----

#[test]
fn ranger_level20_fifth_favored_enemy_is_recognized() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_ENEMY_5_CHOICE_ID);
    assert_eq!(choice.value, 0, "the fifth favored enemy choice must be a +0 recognition record");
    assert!(
        choice.detail.contains("dragon"),
        "the record must name the chosen fifth favored enemy type: {}",
        choice.detail
    );
}

// ----- The fifth favored enemy is absent below the level-20 gate -----

#[test]
fn ranger_level19_fifth_favored_enemy_grounds_no_record() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_5_CHOICE_ID),
        "the fifth favored enemy must not appear at all below its level-20 gate: {:?}",
        computation.explanations
    );
}

// ----- The fifth favored enemy grounds the flat +2 base magnitude (not targeted) -----

#[test]
fn ranger_level20_fifth_favored_enemy_bonus_is_flat_base() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let skill_bonus = explanation(&computation, FAVORED_ENEMY_5_SKILL_BONUS_ID);
    assert_eq!(
        skill_bonus.value, 2,
        "the fifth favored enemy skill bonus must be the flat +2 base (not targeted by the \
         20th-level interval's own bonus increase in this fixture): {}",
        skill_bonus.detail
    );

    let attack_damage_bonus = explanation(&computation, FAVORED_ENEMY_5_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_damage_bonus.value, 2,
        "the fifth favored enemy attack/damage bonus must be the flat +2 base: {}",
        attack_damage_bonus.detail
    );
}

// ----- The 20th-level interval's bonus-increase target choice is recognized -----

#[test]
fn ranger_level20_bonus_increase_target_choice_is_recognized() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_4_CHOICE_ID);
    assert_eq!(choice.value, 0, "the bonus-increase target choice must be a +0 recognition record");
    assert!(
        choice.detail.contains("second"),
        "the record must name the second favored enemy as this interval's target: {}",
        choice.detail
    );
}

// ----- The second favored enemy's magnitude genuinely rises via the 20th-level target -----

#[test]
fn ranger_level20_second_favored_enemy_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, FAVORED_ENEMY_2_BONUS_ID);
    assert_eq!(
        bonus.value, 4,
        "the second favored enemy's skill bonus must genuinely rise to +4 at level 20 (targeted \
         by the 20th-level interval's own bonus increase), up from +2 at level 19: {}",
        bonus.detail
    );
}

// ----- The first and fourth favored enemies' magnitudes stay unchanged at level 20 -----

#[test]
fn ranger_level20_first_and_fourth_favored_enemy_bonuses_stay_unchanged() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let first = explanation(&computation, FAVORED_ENEMY_1_BONUS_ID);
    assert_eq!(
        first.value, 4,
        "the first favored enemy's skill bonus must stay +4 at level 20 (not targeted by the \
         20th-level interval's own bonus increase in this fixture): {}",
        first.detail
    );

    let fourth = explanation(&computation, FAVORED_ENEMY_4_BONUS_ID);
    assert_eq!(
        fourth.value, 4,
        "the fourth favored enemy's skill bonus must stay +4 at level 20 (not targeted by the \
         20th-level interval's own bonus increase in this fixture): {}",
        fourth.detail
    );
}

// ----- Master Hunter is granted as a bounded identity record at level 20 -----

#[test]
fn ranger_level20_master_hunter_is_granted() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // SD-32 Epic 1 (compute-library wiring): the fixture's Wisdom 12
    // (modifier +1) is now run through the SAME formula_interpreter-backed
    // `resolve_pcgen_var_chain` mechanism already fixture-checked at
    // `tests/fixtures/rules_core/derived-evaluator-fixtures.json`'s
    // `class_feature_description_entries` (`ranger_master_hunter`), which
    // pins `10+(MasterHunterLVL/2)+WIS` byte-identical against the pinned
    // oracle's `cr_abilities_class.lst:1427`. At level 20 with WIS modifier
    // +1: `10 + 20/2 + 1 = 21`. This is no longer a fabricated value -- it
    // is the corpus's own formula, evaluated by the already-authorised
    // interpreter (`decisions.md §3`, operator ruling §20).
    let master_hunter = explanation(&computation, MASTER_HUNTER_ID);
    assert_eq!(
        master_hunter.value, 21,
        "Master Hunter's save DC at ranger level 20 with Wisdom modifier +1 is \
         10 + level/2 + WIS = 21, computed via the corpus's own BONUS:VAR formula: {}",
        master_hunter.detail
    );
    assert!(
        master_hunter.detail.to_lowercase().contains("fortitude"),
        "the record must name the Fortitude-save-or-die upgrade: {}",
        master_hunter.detail
    );
    assert!(
        master_hunter.detail.contains("21"),
        "the detail text must name the computed DC, not just describe the rule: {}",
        master_hunter.detail
    );
}

// ----- Master Hunter is a correct level-gate absence below level 20 -----

#[test]
fn ranger_level19_master_hunter_is_a_level_gate_absence() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let master_hunter = explanation(&computation, MASTER_HUNTER_ID);
    assert_eq!(
        master_hunter.value, 0,
        "Master Hunter must be a correct level-gate absence (value 0) below level 20: {}",
        master_hunter.detail
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level20_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-20 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 19 truth is unchanged by this widening -----

#[test]
fn ranger_level19_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 19, "Ranger level 19 base attack bonus must stay 19");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 2),
        ],
        "Ranger level 19 base spells per day must stay `4/3/3/2`"
    );
}

// ----- Negative control: level 21 stays unrecognized by this slice -----

#[test]
fn ranger_level_21_is_not_promoted_by_this_slice() {
    let level_21 = RANGER_LEVEL20_FIXTURE.replace("class:ranger:20", "class:ranger:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "level-21 Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level20_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_ranger_row_names_level_20_widening() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger.grounding_ref.contains("sd18_ranger_level20_widening"),
        "ranger row must cite the live SD18 level-20 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "ranger partial note must name the level-20 widening: {note}"
    );
}
