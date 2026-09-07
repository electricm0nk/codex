//! SD13-E5 Ranger level-5 progression grounding proof.
//!
//! Widens the accepted Ranger level-1/level-2/level-3/level-4 per-pillar
//! decomposition (`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`,
//! `tests/sd13_ranger_base_attack_and_saves.rs`, `tests/sd13_ranger_level2_progression.rs`,
//! `tests/sd13_ranger_level3_progression.rs`, `tests/sd13_ranger_favored_terrain_choice.rs`,
//! `tests/sd13_ranger_level4_progression.rs`) to Ranger level 5, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_ranger_level` is generalized from `1..=4`
//! to `1..=5` via `MAX_SUPPORTED_RANGER_LEVEL = 5`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Ranger class table) were read directly before
//! writing any code or test:
//!
//! - level 5 base attack bonus is +5 (full BAB), base Fortitude/Reflex are +4
//!   (good, unchanged from level 4 -- `5/2+2 = 4` via integer division, a
//!   coincidence, not a sign the formula stopped scaling), base Will is +1
//!   (poor, unchanged from level 4 -- `5/3 = 1`) -- all via the same formulas
//!   already grounded at levels 1-4, not re-derived.
//! - Track stays `max(5/2, 1) = 2` at level 5, unchanged from level 4 (another
//!   coincidence of integer division).
//! - the PF1 Core Rulebook Ranger class table's level-5 "Special" column reads
//!   "2nd favored enemy" (verified independently against both primary sources)
//!   -- no OTHER new class feature is gained at 5th level (checked rather than
//!   assumed, mirroring the Rogue/Bard/Cleric/Sorcerer/Wizard precedent of
//!   confirming a table's "Special" column rather than assuming it away).
//! - the Favored Enemy rule's exact 5th-level text (both sources): "At 5th level
//!   and every five levels thereafter (10th, 15th, and 20th level), the ranger
//!   may select an additional favored enemy. In addition, at each such
//!   interval, the bonus against any one favored enemy (including the one just
//!   selected, if so desired) increases by 2." This is NOT an automatic bump to
//!   the FIRST favored enemy -- it is the ranger's own free choice which ONE of
//!   the (now two) favored enemies receives the +2 increase. Grounding this
//!   honestly therefore requires a THIRD choice-slot
//!   (`choice:ranger_favored_enemy_bonus_increase_target` -> `enemy:first` or
//!   `enemy:second`, mirroring the Hunter's Bond/combat-style restricted
//!   two-option choice idiom) alongside the second favored-enemy selection
//!   itself (`choice:ranger_favored_enemy_2`, mirroring the first favored-enemy
//!   choice's open-ended recognition idiom exactly). Only when a target choice
//!   is actually present in chosen input does either favored enemy's magnitude
//!   rise to +4; absent an explicit target selection, both stay the flat +2,
//!   since fabricating which one the player picked is exactly the kind of
//!   unearned promotion this tranche refuses to ship.
//! - Endurance, Favored Terrain, combat style, and Hunter's Bond (all granted at
//!   or before level 4) all stay granted at level 5, not re-derived. Hunter's
//!   Bond's own ally-bonus magnitude (half the favored-enemy bonus) naturally
//!   recomputes from the same unchanged formula once the first favored enemy's
//!   own magnitude rises to +4 (no new formula, just a wider input).
//!
//! It deliberately does not touch the favored-enemy conditional-application
//! engine, the combat-style bonus feat's own mechanics, the Favored Terrain
//! level-8th/13th/18th breadth, Hunter's Bond ally-bonus application, the
//! animal-companion form, the ranger spell burden, or Ranger level 6+ (all stay
//! named-but-unproven, unchanged from level 4), and it preserves the accepted
//! Ranger level-1/level-2/level-3/level-4 truth, the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level5_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BASE_ATTACK_ID: &str = "class_chassis.ranger.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.ranger.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.ranger.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.ranger.base_save.will";
const TRACK_ID: &str = "class_chassis.ranger.track";
const FAVORED_ENEMY_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_skill_bonus";
const FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_attack_damage_bonus";
const FAVORED_ENEMY_2_CHOICE_ID: &str = "class_chassis.ranger.favored_enemy_2_choice";
const FAVORED_ENEMY_2_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_2_skill_bonus";
const FAVORED_ENEMY_2_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_2_attack_damage_bonus";
const FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID: &str =
    "class_chassis.ranger.favored_enemy_bonus_increase_choice";
const COMBAT_STYLE_CHOICE_ID: &str = "class_chassis.ranger.combat_style_choice";
const ENDURANCE_ID: &str = "class_feature.ranger.endurance";
const FAVORED_TERRAIN_ID: &str = "class_feature.ranger.favored_terrain";
const HUNTERS_BOND_ID: &str = "class_feature.ranger.hunters_bond";
const HUNTERS_BOND_ALLY_BONUS_ID: &str = "class_chassis.ranger.hunters_bond_ally_bonus";

// ----- Base attack bonus at level 5 -----

#[test]
fn ranger_level5_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 5,
        "Ranger level 5 full-BAB progression (classlevel) must equal 5: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 5 (good Fortitude/Reflex, poor Will) -----

#[test]
fn ranger_level5_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 4, "Ranger level 5 good Fortitude (5/2+2) must equal 4");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 4, "Ranger level 5 good Reflex (5/2+2) must equal 4");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 1, "Ranger level 5 poor Will (5/3) must equal 1");
}

// ----- Track at level 5 (unchanged from level 4, integer-division coincidence) -----

#[test]
fn ranger_level5_track_stays_two() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 2,
        "Ranger level 5 Track bonus (max(5/2, 1)) must equal 2: {}",
        track.detail
    );
}

// ----- Favored Enemy: bonus-increase target names the FIRST favored enemy -----

#[test]
fn ranger_level5_first_favored_enemy_rises_to_four_when_targeted() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        skill.value, 4,
        "Ranger level 5 first favored-enemy skill bonus must rise to +4 when the bonus-increase \
         target choice names the first favored enemy: {}",
        skill.detail
    );

    let attack_damage = explanation(&computation, FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_damage.value, 4,
        "Ranger level 5 first favored-enemy attack/damage bonus must rise to +4 when targeted: {}",
        attack_damage.detail
    );
}

#[test]
fn ranger_level5_grounds_bonus_increase_target_choice_recognition() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the bonus-increase target choice recognition must carry no fabricated mechanical value: {}",
        choice.detail
    );
    assert!(
        choice.detail.to_lowercase().contains("first"),
        "the bonus-increase target choice explanation must name the selected target: {}",
        choice.detail
    );
}

// ----- Favored Enemy: second favored-enemy selection is grounded -----

#[test]
fn ranger_level5_grounds_second_favored_enemy_choice_recognition() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_ENEMY_2_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the second favored-enemy choice recognition must carry no fabricated mechanical value: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("undead"),
        "the second favored-enemy choice explanation must name the selected enemy type: {}",
        choice.detail
    );
}

#[test]
fn ranger_level5_second_favored_enemy_stays_at_two_when_not_targeted() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        skill.value, 2,
        "the second favored enemy must stay at the flat +2 when the bonus-increase target names \
         the first favored enemy instead: {}",
        skill.detail
    );

    let attack_damage = explanation(&computation, FAVORED_ENEMY_2_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_damage.value, 2,
        "the second favored enemy's attack/damage bonus must stay the flat +2 when not targeted: {}",
        attack_damage.detail
    );
}

#[test]
fn ranger_level5_bonus_increase_target_can_name_the_second_favored_enemy_instead() {
    let targeted_second = RANGER_LEVEL5_FIXTURE.replace(
        "choice=choice:ranger_favored_enemy_bonus_increase_target:enemy:first",
        "choice=choice:ranger_favored_enemy_bonus_increase_target:enemy:second",
    );
    let input = load(&targeted_second);
    let computation = compute_pilot_base_chassis(&input);

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 2,
        "the first favored enemy must stay the flat +2 once the target choice names the second \
         favored enemy instead: {}",
        first_skill.detail
    );

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 4,
        "the second favored enemy must rise to +4 once the target choice names it: {}",
        second_skill.detail
    );
}

#[test]
fn ranger_level5_without_a_target_choice_both_favored_enemies_stay_at_two() {
    let no_target = RANGER_LEVEL5_FIXTURE
        .lines()
        .filter(|line| {
            !line.starts_with("choice=choice:ranger_favored_enemy_bonus_increase_target")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&no_target);
    let computation = compute_pilot_base_chassis(&input);

    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 2,
        "absent an explicit bonus-increase target choice, the first favored enemy must not be \
         fabricated as boosted: {}",
        first_skill.detail
    );

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 2,
        "absent an explicit bonus-increase target choice, the second favored enemy must not be \
         fabricated as boosted: {}",
        second_skill.detail
    );
    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID),
        "no bonus-increase target choice recognition should be emitted when the fixture supplies \
         no such selection"
    );
}

// ----- Combat style, Endurance, Favored Terrain, and Hunter's Bond stay granted -----

#[test]
fn ranger_level5_keeps_combat_style_endurance_favored_terrain_and_hunters_bond_grounded() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, COMBAT_STYLE_CHOICE_ID),
        "combat-style recognition must stay granted at level 5"
    );

    let endurance = explanation(&computation, ENDURANCE_ID);
    assert_eq!(endurance.value, 0);
    assert!(endurance.detail.to_lowercase().contains("granted"));

    let favored_terrain = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(favored_terrain.value, 2);
    assert!(favored_terrain.detail.to_lowercase().contains("granted"));

    let hunters_bond = explanation(&computation, HUNTERS_BOND_ID);
    assert_eq!(hunters_bond.value, 0);
    assert!(hunters_bond.detail.to_lowercase().contains("granted"));
}

#[test]
fn ranger_level5_hunters_bond_ally_bonus_recomputes_from_the_widened_favored_enemy_bonus() {
    let input = load(RANGER_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let ally_bonus = explanation(&computation, HUNTERS_BOND_ALLY_BONUS_ID);
    assert_eq!(
        ally_bonus.value, 2,
        "Hunter's Bond ally-bonus magnitude (half the FIRST favored-enemy bonus, now 4 / 2 = 2) \
         must recompute from the same unchanged formula once the input widens: {}",
        ally_bonus.detail
    );
}

// ----- Level 6 was later widened into the supported tranche -----
//
// This supersedes the original `ranger_level_6_is_not_promoted_by_this_slice`
// negative control now that a later SD13-E5 slice promotes level 6 into the
// supported tranche; the level-7 negative control now lives in
// `tests/sd13_ranger_level6_progression.rs`.

#[test]
fn ranger_level_6_was_later_widened_into_the_supported_tranche() {
    let level_6 = RANGER_LEVEL5_FIXTURE.replace("class:ranger:5", "class:ranger:6");
    let input = load(&level_6);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "level-6 Ranger was later widened into the supported tranche and must now gain bounded \
         ranger chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, ENDURANCE_ID),
        "level-6 Ranger was later widened and must now carry the Endurance explanation"
    );
    assert!(
        has_explanation(&computation, FAVORED_TERRAIN_ID),
        "level-6 Ranger was later widened and must now carry the Favored Terrain explanation"
    );
    assert!(
        has_explanation(&computation, HUNTERS_BOND_ID),
        "level-6 Ranger was later widened and must now carry the Hunter's Bond explanation"
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level5_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id == ENDURANCE_ID
                || e.id == FAVORED_TERRAIN_ID
                || e.id == HUNTERS_BOND_ID),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level5_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL5_FIXTURE.replace(
        "class_level=class:ranger:5",
        "class_level=class:ranger:5\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id == ENDURANCE_ID
                || e.id == FAVORED_TERRAIN_ID
                || e.id == HUNTERS_BOND_ID),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-5 widening -----

#[test]
fn matrix_ranger_row_names_level_5_widening_and_favored_enemy_second_selection() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger.grounding_ref.contains("sd13_ranger_level5_progression"),
        "ranger row must cite the live SD13-E5 level-5 proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 5") || note.contains("level-5"),
        "ranger partial note must name the level-5 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("second favored enemy")
            || note.to_lowercase().contains("2nd favored enemy"),
        "ranger partial note must name the second favored enemy selection as newly grounded: {note}"
    );
    // The still-unproven burdens stay named.
    for token in ["spell", "conditional-application", "companion"] {
        assert!(
            note.to_lowercase().contains(token),
            "ranger partial note must still name the unproven '{token}' burden: {note}"
        );
    }
}
