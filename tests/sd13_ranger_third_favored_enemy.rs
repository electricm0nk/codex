//! SD13-E5 Ranger THIRD-favored-enemy interval grounding proof — the second
//! priority-2 burden slice.
//!
//! Grounds the PF1 Core Rulebook Ranger 10th-level "3rd favored enemy"
//! interval that `tests/sd13_ranger_level10_progression.rs` discovered and
//! deliberately deferred. Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Ranger page) were read directly before writing any code,
//! and both state the exact rule text: "At 5th level and every five levels
//! thereafter (10th, 15th, and 20th level), the ranger may select an
//! additional favored enemy. In addition, at each such interval, the bonus
//! against any one favored enemy (including the one just selected, if so
//! desired) increases by +2." — each interval grants its OWN
//! bonus-increase-target choice, so the 10th-level interval is grounded as
//! the exact structural mirror of the already-grounded 5th-level interval
//! (`choice:ranger_favored_enemy_2` +
//! `choice:ranger_favored_enemy_bonus_increase_target`) and of the
//! 8th-level favored-terrain interval grounded by
//! `tests/sd13_ranger_second_favored_terrain.rs`:
//!
//! - a THIRD enemy-type selection (`choice:ranger_favored_enemy_3`,
//!   open-ended raw-string recognition, no restricted-list validation) is
//!   recognized as a bounded `+0` identity record at ranger level 10+, with
//!   its own flat skill and attack/damage magnitudes
//!   (`class_chassis.ranger.favored_enemy_3_skill_bonus` /
//!   `class_chassis.ranger.favored_enemy_3_attack_damage_bonus`, base +2,
//!   or +4 when the 10th-level interval's bonus-increase target names it);
//! - the 10th-level interval's OWN bonus-increase TARGET selection
//!   (`choice:ranger_favored_enemy_bonus_increase_target_2`, restricted to
//!   `enemy:first` / `enemy:second` / `enemy:third`) is recognized as a
//!   bounded `+0` record at ranger level 10+, and the targeted enemy's flat
//!   magnitudes genuinely rise by +2 — STACKING with the 5th-level
//!   interval's increase when both target the same enemy (first enemy at
//!   +6 on this fixture: 2 base + 2 at 5th + 2 at 10th);
//! - absent either selection in chosen input, nothing is fabricated: the
//!   level-10 baseline fixture computes exactly as before (first enemy +4,
//!   second +2, no third-enemy records), preserving every existing sibling
//!   truth including the level-10 file's own non-fabrication control;
//! - below the 10th-level interval gate the selections are correctly not
//!   recognized even when present in chosen input (the 5th-level interval's
//!   records still compute unchanged).
//!
//! It deliberately does not touch the favored-enemy
//! conditional-application engine (no attack, damage, or skill total is
//! modified by any of these records — no target-type matching engine exists
//! in this codebase), the favored-terrain records, the combat-style feats,
//! the ranger spell burden, or Ranger level 11+ (the 15th/20th intervals
//! stay out of scope); and it preserves the Fighter negative control and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL10_BASELINE_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level10_sd13_deterministic_input.txt");

const RANGER_THIRD_ENEMY_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level10_sd13_third_favored_enemy_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const FAVORED_ENEMY_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_skill_bonus";
const FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_attack_damage_bonus";
const FAVORED_ENEMY_2_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_2_skill_bonus";
const FAVORED_ENEMY_3_CHOICE_ID: &str = "class_chassis.ranger.favored_enemy_3_choice";
const FAVORED_ENEMY_3_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_3_skill_bonus";
const FAVORED_ENEMY_3_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_3_attack_damage_bonus";
const FAVORED_ENEMY_BONUS_INCREASE_2_CHOICE_ID: &str =
    "class_chassis.ranger.favored_enemy_bonus_increase_2_choice";

// ----- The third enemy selection is recognized at level 10 -----

#[test]
fn ranger_level10_third_favored_enemy_selection_is_recognized() {
    let input = load(RANGER_THIRD_ENEMY_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_ENEMY_3_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the third favored-enemy selection must be a +0 recognition record only"
    );
    assert!(
        choice.detail.contains("dragon"),
        "the recognition record must name the raw chosen enemy string (enemy:dragon): {}",
        choice.detail
    );
}

// ----- The third enemy's own flat magnitudes are grounded at base +2 -----

#[test]
fn ranger_level10_third_favored_enemy_magnitudes_are_grounded_at_base_two() {
    let input = load(RANGER_THIRD_ENEMY_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let skill = explanation(&computation, FAVORED_ENEMY_3_SKILL_BONUS_ID);
    assert_eq!(
        skill.value, 2,
        "the third favored enemy's skill bonus must be the flat base +2 (this fixture's \
         10th-level increase target names the FIRST enemy): {}",
        skill.detail
    );

    let attack_damage = explanation(&computation, FAVORED_ENEMY_3_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_damage.value, 2,
        "the third favored enemy's attack/damage bonus must be the flat base +2: {}",
        attack_damage.detail
    );
}

// ----- The 10th-level increase stacks on the first enemy (+6) -----

#[test]
fn ranger_level10_second_increase_target_stacks_the_first_enemy_to_six() {
    let input = load(RANGER_THIRD_ENEMY_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let target = explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_2_CHOICE_ID);
    assert_eq!(
        target.value, 0,
        "the 10th-level bonus-increase target selection must be a +0 recognition record only"
    );
    assert!(
        target.detail.contains("first favored enemy"),
        "the recognition record must name the first favored enemy as the boosted one: {}",
        target.detail
    );

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 6,
        "the first favored enemy's skill bonus must genuinely rise to +6 when both the \
         5th-level and 10th-level interval increases target it (2 + 2 + 2): {}",
        first_skill.detail
    );

    let first_attack = explanation(&computation, FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        first_attack.value, 6,
        "the first favored enemy's attack/damage bonus must genuinely rise to +6 when both \
         interval increases target it"
    );

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 2,
        "the second favored enemy must stay at the flat base +2, untargeted at both intervals"
    );
}

// ----- Targeting the third enemy boosts it instead -----

#[test]
fn ranger_level10_second_increase_target_third_boosts_the_third_enemy() {
    let swapped = RANGER_THIRD_ENEMY_FIXTURE.replace(
        "choice=choice:ranger_favored_enemy_bonus_increase_target_2:enemy:first",
        "choice=choice:ranger_favored_enemy_bonus_increase_target_2:enemy:third",
    );
    let input = load(&swapped);
    let computation = compute_pilot_base_chassis(&input);

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "the first favored enemy must keep only its 5th-level increase (+4) when the \
         10th-level target names the third enemy"
    );

    let third_skill = explanation(&computation, FAVORED_ENEMY_3_SKILL_BONUS_ID);
    assert_eq!(
        third_skill.value, 4,
        "the third favored enemy's skill bonus must genuinely rise to +4 when the 10th-level \
         increase targets it (\"including the one just selected, if so desired\"): {}",
        third_skill.detail
    );

    let third_attack = explanation(&computation, FAVORED_ENEMY_3_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        third_attack.value, 4,
        "the third favored enemy's attack/damage bonus must genuinely rise to +4 when the \
         10th-level increase targets it"
    );
}

// ----- Targeting the second enemy boosts it instead -----

#[test]
fn ranger_level10_second_increase_target_second_boosts_the_second_enemy() {
    let swapped = RANGER_THIRD_ENEMY_FIXTURE.replace(
        "choice=choice:ranger_favored_enemy_bonus_increase_target_2:enemy:first",
        "choice=choice:ranger_favored_enemy_bonus_increase_target_2:enemy:second",
    );
    let input = load(&swapped);
    let computation = compute_pilot_base_chassis(&input);

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(first_skill.value, 4, "the first favored enemy must keep only +4");

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 4,
        "the second favored enemy's skill bonus must genuinely rise to +4 when the \
         10th-level increase targets it: {}",
        second_skill.detail
    );

    let third_skill = explanation(&computation, FAVORED_ENEMY_3_SKILL_BONUS_ID);
    assert_eq!(third_skill.value, 2, "the third favored enemy must stay at the base +2");
}

// ----- An unrecognized target string fabricates no boost -----

#[test]
fn ranger_level10_unrecognized_second_increase_target_boosts_nothing() {
    let unrecognized = RANGER_THIRD_ENEMY_FIXTURE.replace(
        "choice=choice:ranger_favored_enemy_bonus_increase_target_2:enemy:first",
        "choice=choice:ranger_favored_enemy_bonus_increase_target_2:enemy:fourth",
    );
    let input = load(&unrecognized);
    let computation = compute_pilot_base_chassis(&input);

    let target = explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_2_CHOICE_ID);
    assert_eq!(target.value, 0);
    assert!(
        target.detail.contains("restricted"),
        "an unrecognized target must be surfaced as outside the restricted set, with no \
         target identity grounded: {}",
        target.detail
    );

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "the first enemy must keep only its 5th-level increase; no 10th-level boost may be \
         fabricated from an unrecognized target"
    );
    let third_skill = explanation(&computation, FAVORED_ENEMY_3_SKILL_BONUS_ID);
    assert_eq!(third_skill.value, 2, "no boost may be fabricated from an unrecognized target");
}

// ----- Negative control: the baseline level-10 fixture is unchanged -----

#[test]
fn ranger_level10_baseline_without_the_selections_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL10_BASELINE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_3_CHOICE_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_3_SKILL_BONUS_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_3_ATTACK_DAMAGE_BONUS_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_2_CHOICE_ID),
        "absent the chosen selections, no third-enemy or second-increase record may be \
         fabricated: {:?}",
        computation.explanations
    );

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "the baseline first favored enemy must stay at +4, exactly as the level-10 chassis \
         slice left it"
    );
}

// ----- Negative control: below the 10th-level interval gate -----

#[test]
fn ranger_level9_does_not_recognize_the_third_enemy_selections() {
    let level_9 = RANGER_THIRD_ENEMY_FIXTURE
        .replace("class:ranger:10", "class:ranger:9")
        .replace(
            "choice=choice:ranger_combat_style_bonus_feat_3:feat:shot_on_the_run\n",
            "",
        );
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_3_CHOICE_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_3_SKILL_BONUS_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_2_CHOICE_ID),
        "the 10th-level interval selections must not be recognized below the PF1 CRB gate \
         even when present in chosen input: {:?}",
        computation.explanations
    );

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "below the 10th-level gate the first favored enemy must keep only its 5th-level \
         increase (+4)"
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_third_favored_enemy_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_3_CHOICE_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_3_SKILL_BONUS_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_2_CHOICE_ID),
        "the Fighter chassis must not surface any third-favored-enemy record: {:?}",
        computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_does_not_gain_third_favored_enemy_recognition() {
    let multiclass = RANGER_THIRD_ENEMY_FIXTURE.replace(
        "class_level=class:ranger:10",
        "class_level=class:ranger:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_3_CHOICE_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_3_SKILL_BONUS_ID)
            && !has_explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_2_CHOICE_ID),
        "multiclass Ranger must not gain any third-favored-enemy record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix names the third-favored-enemy grounding -----

#[test]
fn matrix_ranger_row_names_the_third_favored_enemy_grounding() {
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
        ranger
            .grounding_ref
            .contains("sd13_ranger_third_favored_enemy"),
        "ranger row must cite the live third-favored-enemy proof surface: {}",
        ranger.grounding_ref
    );
    assert!(
        ranger
            .blocker_or_lossiness_note
            .contains("favored_enemy_3"),
        "ranger partial note must name the grounded third-favored-enemy records: {}",
        ranger.blocker_or_lossiness_note
    );
}
