//! SD13-E5 Ranger level-2 progression grounding proof.
//!
//! Widens the accepted Ranger level-1 per-pillar decomposition
//! (`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`,
//! `tests/sd13_ranger_base_attack_and_saves.rs`,
//! `tests/sd13_ranger_combat_style_level_gate.rs`) to Ranger level 2, mirroring the
//! Fighter `supported_fighter_level` / Paladin `supported_paladin_level` / Rogue
//! `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
//! `supported_monk_level` / Cleric `supported_cleric_level` / Bard
//! `supported_bard_level` / Druid `supported_druid_level` / Sorcerer
//! `supported_sorcerer_level` / Wizard `supported_wizard_level` level-range-gate
//! idiom (the level-1-only gate `is_single_class_ranger_level1` is generalized to
//! `supported_ranger_level`, an `Option<u8>` helper gated by
//! `MAX_SUPPORTED_RANGER_LEVEL = 2`). Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Ranger class table) were read directly before writing any code
//! or test:
//!
//! - level 2 base attack bonus is +2 (full BAB), base Fortitude/Reflex are +3
//!   (good), base Will is +0 (poor) — confirmed by the same formulas already
//!   grounded at level 1 (`classlevel` and `classlevel/2+2` / `classlevel/3`), not
//!   re-derived.
//! - Track stays `max(ranger level / 2, 1) = 1` at level 2, reached naturally
//!   (`max(2/2, 1) = 1`), not via the level-1 floor.
//! - the Favored Enemy flat surface (choice recognition, +2 skill bonus, +2
//!   attack/damage bonus) is confirmed unchanged at level 2 — PF1 Core Rulebook only
//!   increases the Favored Enemy bonus at 4th ranger level and beyond, so level 2
//!   stays at the flat +2 via the same formulas, not new records.
//! - Combat Style Feat, the PF1 Core Rulebook 2nd-level Ranger class-table entry
//!   (verified independently against legacy.aonprd.com's Core Rulebook Ranger page:
//!   "At 2nd level, a ranger must select one combat style to pursue... He can
//!   choose feats from his selected combat style"), is finally grounded for real at
//!   the level-2 gate it was always named for: the STYLE CHOICE (Archery or
//!   Two-Weapon Combat, the two PF1 Core Rulebook options) is recognized as chosen
//!   input when a `choice:ranger_combat_style` selection is present, and the
//!   specific BONUS FEAT drawn from that style's own restricted 2nd-level feat list
//!   (Archery: Far Shot, Point-Blank Shot, Precise Shot, Rapid Shot; Two-Weapon
//!   Combat: Double Slice, Improved Shield Bash, Quick Draw, Two-Weapon Fighting —
//!   both lists verified against legacy.aonprd.com) is recognized when a
//!   `choice:ranger_combat_style_bonus_feat` selection is present and the style
//!   itself was recognized. Both are bounded identity/recognition records only
//!   (value 0 each): no feat's own mechanical effect (e.g. Point-Blank Shot's
//!   attack/damage bonus within 30 ft.) is computed anywhere in this codebase, and
//!   below the level-2 gate the record stays the pre-existing correct level-gate
//!   ABSENCE (`class_chassis.ranger.level_gate.combat_style`), unchanged.
//!
//! It deliberately does not touch the favored-enemy conditional-application engine,
//! the combat-style bonus feat's own mechanics, the ranger spell burden, or Ranger
//! level 3+ (all stay named-but-unproven, unchanged from level 1), and it preserves
//! the accepted Ranger level-1 truth, the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const RANGER_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level2_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BASE_ATTACK_ID: &str = "class_chassis.ranger.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.ranger.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.ranger.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.ranger.base_save.will";
const TRACK_ID: &str = "class_chassis.ranger.track";
const FAVORED_ENEMY_CHOICE_ID: &str = "class_chassis.ranger.favored_enemy_choice";
const FAVORED_ENEMY_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_skill_bonus";
const FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_attack_damage_bonus";
const COMBAT_STYLE_LEVEL_GATE_ID: &str = "class_chassis.ranger.level_gate.combat_style";
const COMBAT_STYLE_CHOICE_ID: &str = "class_chassis.ranger.combat_style_choice";
const COMBAT_STYLE_BONUS_FEAT_CHOICE_ID: &str =
    "class_chassis.ranger.combat_style_bonus_feat_choice";

// ----- Base attack bonus at level 2 -----

#[test]
fn ranger_level2_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(RANGER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 2,
        "Ranger level 2 full-BAB progression (classlevel) must equal 2: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 2 (good Fortitude/Reflex, poor Will) -----

#[test]
fn ranger_level2_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(fortitude.value, 3, "Ranger level 2 good Fortitude (2/2+2) must equal 3");

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 3, "Ranger level 2 good Reflex (2/2+2) must equal 3");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 0, "Ranger level 2 poor Will (2/3) must equal 0");
}

// ----- Track at level 2 (stays 1, reached naturally not via the level-1 floor) -----

#[test]
fn ranger_level2_track_stays_one_reached_naturally() {
    let input = load(RANGER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 1,
        "Ranger level 2 Track bonus (max(2/2, 1)) must equal 1: {}",
        track.detail
    );
}

// ----- Favored Enemy flat surface at level 2 (unchanged) -----

#[test]
fn ranger_level2_favored_enemy_flat_surface_is_unchanged() {
    let input = load(RANGER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(has_explanation(&computation, FAVORED_ENEMY_CHOICE_ID));

    let skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        skill.value, 2,
        "Ranger level 2 favored-enemy skill bonus must stay the flat +2 (PF1 CRB increases only \
         at 4th ranger level): {}",
        skill.detail
    );

    let attack_damage = explanation(&computation, FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_damage.value, 2,
        "Ranger level 2 favored-enemy attack/damage bonus must stay the flat +2: {}",
        attack_damage.detail
    );
}

// ----- Combat Style Feat: finally grounded for real at the 2nd-level gate -----

#[test]
fn ranger_level1_still_grounds_combat_style_as_a_level_gate_absence() {
    let input = load(RANGER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let combat_style = explanation(&computation, COMBAT_STYLE_LEVEL_GATE_ID);
    assert_eq!(
        combat_style.value, 0,
        "level-1 combat-style record must stay a correct absence: {}",
        combat_style.detail
    );
    assert!(
        !has_explanation(&computation, COMBAT_STYLE_CHOICE_ID),
        "level-1 Ranger must not fabricate a combat-style choice recognition: {:?}",
        computation.explanations
    );
    assert!(
        !has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_CHOICE_ID),
        "level-1 Ranger must not fabricate a combat-style bonus-feat recognition: {:?}",
        computation.explanations
    );
}

#[test]
fn ranger_level2_grounds_combat_style_choice_recognition() {
    let input = load(RANGER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The level-gate absence record must be retired at level 2 -- it is a level-1
    // ONLY absence, and level 2 is the actual grant level.
    assert!(
        !has_explanation(&computation, COMBAT_STYLE_LEVEL_GATE_ID),
        "level-2 Ranger must not keep the level-1-only combat-style absence record: {:?}",
        computation.explanations
    );

    let style_choice = explanation(&computation, COMBAT_STYLE_CHOICE_ID);
    assert_eq!(
        style_choice.value, 0,
        "combat-style choice recognition is a recognition record only and must carry no \
         fabricated mechanical value: {}",
        style_choice.value
    );
    assert!(
        style_choice.detail.contains("Archery"),
        "combat-style choice recognition must name the chosen style (Archery): {}",
        style_choice.detail
    );
    assert!(
        style_choice.detail.contains("style:archery"),
        "combat-style choice recognition must cite the raw fixture selection: {}",
        style_choice.detail
    );
}

#[test]
fn ranger_level2_grounds_combat_style_bonus_feat_recognition() {
    let input = load(RANGER_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus_feat = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_CHOICE_ID);
    assert_eq!(
        bonus_feat.value, 0,
        "combat-style bonus-feat recognition is a recognition record only and must carry no \
         fabricated mechanical value: {}",
        bonus_feat.value
    );
    assert!(
        bonus_feat.detail.contains("Point-Blank Shot"),
        "combat-style bonus-feat recognition must name the chosen feat (Point-Blank Shot): {}",
        bonus_feat.detail
    );
    assert!(
        bonus_feat.detail.contains("Archery"),
        "combat-style bonus-feat recognition must name the style whose restricted list it drew \
         from: {}",
        bonus_feat.detail
    );
    // Must disclaim the feat's own mechanical effect -- no execution engine exists.
    assert!(
        bonus_feat.detail.contains("no")
            && (bonus_feat.detail.contains("mechanic") || bonus_feat.detail.contains("execution")),
        "combat-style bonus-feat recognition must disclaim the feat's own mechanical effect: {}",
        bonus_feat.detail
    );
}

#[test]
fn ranger_level2_without_combat_style_choice_grounds_nothing_fabricated() {
    // Without a chosen combat style, there is nothing to recognize -- mirroring the
    // Favored Enemy choice-absence idiom. No style or bonus-feat record may be
    // fabricated, and the retired level-1-only absence record must not resurface
    // either (level 2 is past that gate).
    let without_choice = RANGER_LEVEL2_FIXTURE
        .replace("choice=choice:ranger_combat_style:style:archery\n", "")
        .replace(
            "choice=choice:ranger_combat_style_bonus_feat:feat:point_blank_shot\n",
            "",
        );
    let input = load(&without_choice);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, COMBAT_STYLE_CHOICE_ID),
        "without a chosen combat style there is nothing to recognize: {:?}",
        computation.explanations
    );
    assert!(
        !has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_CHOICE_ID),
        "without a chosen combat style there is no bonus feat to recognize: {:?}",
        computation.explanations
    );
    assert!(
        !has_explanation(&computation, COMBAT_STYLE_LEVEL_GATE_ID),
        "the level-1-only absence record must not resurface at level 2: {:?}",
        computation.explanations
    );
}

// ----- The accepted Ranger level-1 truth is unaffected -----

#[test]
fn ranger_level1_truth_is_unchanged_by_this_widening() {
    let input = load(RANGER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 1, "Ranger level 1 base attack bonus must stay 1");

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(track.value, 1, "Ranger level 1 Track bonus must stay 1");
}

// ----- Negative control: Ranger level 3 was later widened into the supported tranche -----

#[test]
fn ranger_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file was written, Ranger level 3+ progression was out of
    // scope (the level-range gate `supported_ranger_level` only recognized 1..=2).
    // A later SD13-E5 slice (`tests/sd13_ranger_level3_progression.rs`) widened the
    // gate to 1..=3 and extended this exact base-attack/save/Track/favored-enemy
    // grounding to level 3 via the same formulas, plus grounded Endurance as a new
    // 3rd-level record. This control now pins the widened truth instead of the
    // stale level-2-only absence, and a level-4 negative control takes over the
    // "still out of scope" role.
    let level_3 = RANGER_LEVEL2_FIXTURE.replace("class:ranger:2", "class:ranger:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "level-3 Ranger must now gain the widened bounded ranger chassis explanations: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 4 was later widened into the supported tranche -----

#[test]
fn ranger_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_ranger_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk level-range
    // gate idiom) and grounded Hunter's Bond; this negative control is
    // superseded, not violated — pin the new truth here too so this file stays
    // internally consistent.
    let level_4 = RANGER_LEVEL2_FIXTURE.replace("class:ranger:2", "class:ranger:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "level-4 Ranger is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level2_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level2_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL2_FIXTURE.replace(
        "class_level=class:ranger:2",
        "class_level=class:ranger:2\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-2 widening -----

#[test]
fn matrix_ranger_row_names_level_2_widening_and_combat_style_grounding() {
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
        ranger.grounding_ref.contains("sd13_ranger_level2_progression"),
        "ranger row must cite the live SD13-E5 level-2 proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 2") || note.contains("level-2"),
        "ranger partial note must name the level-2 widening: {note}"
    );
    assert!(
        note.contains("combat style") || note.contains("Combat Style"),
        "ranger partial note must still name the combat-style pillar: {note}"
    );
    // The still-unproven burdens stay named.
    for token in ["spell", "conditional-application"] {
        assert!(
            note.contains(token),
            "ranger partial note must still name the unproven '{token}' burden: {note}"
        );
    }
}
