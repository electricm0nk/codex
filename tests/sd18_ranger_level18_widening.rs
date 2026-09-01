//! SD18 Ranger level-18 fourth-favored-terrain and fifth-combat-style-
//! bonus-feat widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-17
//! hybrid chassis (`tests/sd18_ranger_level17_hide_in_plain_sight.rs`, the
//! loop's most recent Ranger ceiling) to Ranger level 18
//! (`supported_ranger_level` is generalized from `1..=17` to `1..=18` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 18`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants) — the loop's SEVENTH §3.2
//! level-18 landing, after Wizard, Cleric, Paladin, Fighter, Barbarian, and
//! Rogue.
//!
//! §3.1 race rows and §3.3 interaction rows are structurally
//! exhausted/blocked (cited in the progress doc, not re-derived this
//! cycle); §3.4/§3.5 are structurally blocked (same root cause, also
//! cited, not re-derived). Druid stays capped at level 15 and Monk at
//! level 12, both documented structural exceptions, not re-attempted here.
//!
//! Ranger was picked per the brief's explicit direction to re-verify the
//! prior cycle's carried-forward hypothesis fresh rather than trust it by
//! analogy. Both PF1 CRB primary sources (d20pfsrd and the Archives of
//! Nethys aonprd.com mirror) were fetched directly before writing any code
//! or test, and both agree byte-for-byte on the level-18 class table row:
//! BAB +18/+13/+8/+3 (full-BAB progression, base value 18), Fort +11, Ref
//! +11, Will +6, Special "4th favored terrain, combat style feat", base
//! spells per day 4/3/2/2 (1st/2nd/3rd/4th). The level-15/16/17/19/20
//! neighboring rows were fetched in the same passes to rule out level
//! misattribution: level 17 = "Hide in plain sight", level 19 = "Improved
//! quarry", level 20 = "5th favored enemy, master hunter" — all distinct
//! from level 18's own row, confirming this is genuinely level 18's own
//! row, not an adjacent level's text. This exactly confirms the prior
//! cycle's carried-forward hypothesis with no discrepancy.
//!
//! - level 18 base attack bonus GENUINELY RISES to 18 (full BAB
//!   progression, up from 17 at level 17); good Fortitude and good Reflex
//!   BOTH GENUINELY RISE to 11 (`18/2+2 = 11`, up from 10 at level 17);
//!   poor Will GENUINELY RISES to 6 (`18/3 = 6`, up from 5 at level 17) —
//!   unlike level 17's own all-integer-division-coincidence row, every
//!   base-save value genuinely rises at level 18.
//! - the base spells-per-day table's level-18 row is `4/3/2/2`
//!   (1st/2nd/3rd/4th), verified independently against both primary
//!   sources: the 1st/2nd/3rd-level columns stay 4/3/2 unchanged from
//!   level 17, and the 4th-level column GENUINELY RISES from 1 to 2 (a
//!   literal table lookup value, not a formula, and numerically identical
//!   to the already-landed Paladin level-18 row) — the spell-level access
//!   ladder itself stays at 4 (already widened at level 13; ranger spells
//!   never reach a 5th spell level at any level).
//! - the PF1 Core Rulebook Ranger class table's level-18 "Special" column
//!   reads "4th favored terrain, combat style feat" (verified
//!   independently against both primary sources, byte-for-byte
//!   identical). Favored Terrain's own rule text ("At 8th level and every
//!   five levels thereafter... the ranger may select an additional
//!   favored terrain. In addition, at each such interval, the skill bonus
//!   and initiative bonus in any one favored terrain... increases by +2")
//!   is the EXACT structural mirror of the already-grounded 8th/13th-level
//!   intervals — not a new class feature. This slice grounds: a FOURTH
//!   favored-terrain TYPE selection (open-ended, mirroring the
//!   first/second/third favored terrains' own choice-recognition idiom
//!   exactly), a restricted four-option choice recognizing WHICH ONE of
//!   the four favored terrains is this interval's bonus-increase target
//!   (`terrain:first` / `terrain:second` / `terrain:third` /
//!   `terrain:fourth`, mirroring the 13th-level interval's own restricted-
//!   choice idiom widened by one option, and mirroring the Favored
//!   Enemy 15th-level interval's own four-way widening exactly), and the
//!   resulting flat magnitude increase applied only to whichever favored
//!   terrain the target choice actually names (an increase can stack
//!   across all three grounded intervals on the same terrain, totaling
//!   base 2 plus 2 per grounded interval targeting it, up to +8). Combat
//!   Style Feat's own rule text (bonus feats land at ranger levels 2, 6,
//!   10, 14, and 18) grants a FIFTH combat-style bonus feat at 18th
//!   level; mirroring the 14th-level fourth bonus feat's own reasoning
//!   (verified independently against three sources dedicated to the
//!   combat-style feat lists themselves: the PF1 Core Rulebook's printed
//!   Archery/Two-Weapon-Combat feat lists do not tabulate any named
//!   options beyond the 10th-level tier), this fifth slot is grounded as
//!   an OPEN-ENDED +0 recognition record, NOT a restricted-list match,
//!   mirroring the fourth bonus feat's own idiom exactly.
//!
//! It deliberately does not touch the favored-terrain/favored-enemy
//! conditional-application engines, any of the five combat-style bonus
//! feats' own mechanics, Hunter's Bond ally-bonus application or the
//! animal-companion form, Woodland Stride's/Swift Tracker's/Quarry's/
//! Camouflage's/Hide in Plain Sight's own application, the ranger Wisdom
//! prepared-posture/spell-source-lineage burden (all stay
//! named-but-unproven, unchanged from levels 1-17), or any actual
//! save-resolution/damage-halving math for Evasion or Improved Evasion
//! (both stay bounded identity records only, no saving-throw-resolution
//! or damage-resolution engine exists anywhere in this codebase). It does
//! not ground Ranger level 19+ (the Favored Enemy rule's own 20th-level
//! interval and Improved Quarry/Master Hunter stay out of scope). It also
//! preserves the accepted Ranger level-1..level-17 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.
//!
//! This slice also fixes five pre-existing stale sibling negative controls
//! that this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s,
//! `tests/sd18_ranger_level11_quarry.rs`'s,
//! `tests/sd18_ranger_level12_widening.rs`'s,
//! `tests/sd18_ranger_level13_widening.rs`'s, and
//! `tests/sd18_ranger_level14_widening.rs`'s own
//! `ranger_level_18_is_not_promoted_by_this_slice` (formerly named for
//! level 18), all moved to a level-19 boundary in the same commit;
//! `tests/sd18_ranger_level17_hide_in_plain_sight.rs`'s own level-18
//! negative-control test is removed rather than moved, since level 18 is
//! now itself the supported/grounded row, mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin/Rogue/Sorcerer
//! level-N-to-level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level17_sd18_hide_in_plain_sight_deterministic_input.txt"
);

const RANGER_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level18_sd18_fourth_favored_terrain_and_fifth_combat_style_feat_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

const FAVORED_TERRAIN_4_CHOICE_ID: &str = "class_chassis.ranger.favored_terrain_4_choice";
const FAVORED_TERRAIN_BONUS_INCREASE_3_CHOICE_ID: &str =
    "class_chassis.ranger.favored_terrain_bonus_increase_3_choice";
const FAVORED_TERRAIN_1_BONUS_ID: &str = "class_feature.ranger.favored_terrain";
const FAVORED_TERRAIN_4_BONUS_ID: &str = "class_feature.ranger.favored_terrain_4";
const COMBAT_STYLE_BONUS_FEAT_5_CHOICE_ID: &str =
    "class_chassis.ranger.combat_style_bonus_feat_5_choice";

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

// ----- Base attack bonus genuinely rises at level 18 -----

#[test]
fn ranger_level18_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 18,
        "Ranger level 18 full-BAB progression must equal 18, genuinely risen from 17: {}",
        base_attack.detail
    );
}

// ----- All three base saves genuinely rise (unlike level 17's coincidences) -----

#[test]
fn ranger_level18_all_base_saves_genuinely_rise() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 11,
        "Ranger level 18 good Fortitude (18/2+2) must genuinely rise to 11, up from 10"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(
        reflex.value, 11,
        "Ranger level 18 good Reflex (18/2+2) must genuinely rise to 11, up from 10"
    );

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(
        will.value, 6,
        "Ranger level 18 poor Will (18/3) must genuinely rise to 6, up from 5"
    );
}

// ----- Base spells per day widen at level 18: the 4th-level column genuinely rises -----

#[test]
fn ranger_level18_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 2),
        ],
        "level 18 (`4/3/2/2`): the 1st/2nd/3rd-level columns stay 4/3/2 unchanged, and the \
         4th-level column genuinely rises from 1 to 2"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn ranger_level18_spell_level_access_stays_four() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_level_access",
    );
    assert_eq!(
        access.value, 4,
        "Ranger level 18 spell-level access must stay 4 (already widened at level 13): {}",
        access.detail
    );
}

// ----- The fourth favored terrain is recognized as an open-ended type selection -----

#[test]
fn ranger_level18_fourth_favored_terrain_is_recognized_open_ended() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_TERRAIN_4_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the fourth favored terrain selection must be a +0 recognition record only"
    );
    assert!(
        choice.detail.contains("urban"),
        "the recognition record must name the raw chosen terrain string: {}",
        choice.detail
    );
}

// ----- The 18th-level interval's own bonus-increase target is recognized -----

#[test]
fn ranger_level18_bonus_increase_target_names_the_first_favored_terrain() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_3_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the 18th-level-interval bonus-increase target selection must be a +0 recognition \
         record only"
    );
    assert!(
        choice.detail.contains("first favored terrain"),
        "the recognition record must name the first favored terrain as the target: {}",
        choice.detail
    );
}

// ----- The first favored terrain's bonus stacks across all three grounded intervals -----

#[test]
fn ranger_level18_first_favored_terrain_bonus_stacks_across_all_three_intervals() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, FAVORED_TERRAIN_1_BONUS_ID);
    assert_eq!(
        bonus.value, 8,
        "the first favored terrain bonus must be +8 (2 base + 2 from the 8th-level interval \
         + 2 from the 13th-level interval + 2 from the 18th-level interval, all targeting the \
         same terrain): {}",
        bonus.detail
    );
}

// ----- The fourth favored terrain's own bonus stays base +2 when not self-targeted -----

#[test]
fn ranger_level18_fourth_favored_terrain_bonus_is_base_two_when_not_self_targeted() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, FAVORED_TERRAIN_4_BONUS_ID);
    assert_eq!(
        bonus.value, 2,
        "the fourth favored terrain bonus must stay the flat base +2 when the 18th-level \
         interval's own bonus-increase target names the first favored terrain instead: {}",
        bonus.detail
    );
}

// ----- The fifth combat style bonus feat is recognized as an open-ended choice slot -----

#[test]
fn ranger_level18_fifth_combat_style_bonus_feat_is_recognized_open_ended() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_5_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the fifth combat style bonus feat must be a +0 recognition record only: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("improved_precise_shot"),
        "the recognition record must name the raw chosen feat string: {}",
        choice.detail
    );
}

// ----- Absent selections fabricate nothing -----

#[test]
fn ranger_level18_fourth_favored_terrain_and_fifth_feat_absent_ground_nothing() {
    let without_new_selections = RANGER_LEVEL18_FIXTURE
        .replace("choice=choice:ranger_favored_terrain_4:terrain:urban\n", "")
        .replace(
            "choice=choice:ranger_favored_terrain_bonus_increase_target_3:terrain:first\n",
            "",
        )
        .replace(
            "choice=choice:ranger_combat_style_bonus_feat_5:feat:improved_precise_shot\n",
            "",
        );
    let input = load(&without_new_selections);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_4_CHOICE_ID),
        "absent fourth favored terrain selection must fabricate no recognition record"
    );
    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_3_CHOICE_ID),
        "absent 18th-level-interval bonus-increase target must fabricate no recognition record"
    );
    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_4_BONUS_ID),
        "absent fourth favored terrain selection must fabricate no bonus record"
    );
    assert!(
        !has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_5_CHOICE_ID),
        "absent fifth combat style bonus feat selection must fabricate no recognition record"
    );

    let bonus = explanation(&computation, FAVORED_TERRAIN_1_BONUS_ID);
    assert_eq!(
        bonus.value, 6,
        "without the 18th-level interval's own target selection, the first favored terrain \
         bonus must fall back to +6 (2 base + 2 from the 8th-level interval + 2 from the \
         13th-level interval only): {}",
        bonus.detail
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level18_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-18 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 17 truth is unchanged by this widening -----

#[test]
fn ranger_level17_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 17, "Ranger level 17 base attack bonus must stay 17");

    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_4_CHOICE_ID),
        "level-17 Ranger must not gain any fourth-favored-terrain record: {:?}",
        computation.explanations
    );
    assert!(
        !has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_5_CHOICE_ID),
        "level-17 Ranger must not gain any fifth-combat-style-bonus-feat record: {:?}",
        computation.explanations
    );

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "Ranger level 17 base spells per day must stay `4/3/2/1`"
    );
}

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level18_recognition() {
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

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level18_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL18_FIXTURE.replace(
        "class_level=class:ranger:18",
        "class_level=class:ranger:18\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-18 widening -----

#[test]
fn matrix_ranger_row_names_level_18_widening() {
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
        ranger.grounding_ref.contains("sd18_ranger_level18_widening"),
        "ranger row must cite the live SD18 level-18 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 18") || note.contains("level-18"),
        "ranger partial note must name the level-18 widening: {note}"
    );
}
