//! SD13-E5 Ranger level-10 progression grounding proof.
//!
//! Widens the accepted Ranger level-1..level-9 hybrid chassis baseline (most
//! recently `tests/sd13_ranger_level9_progression.rs`) to Ranger level 10 —
//! the tranche's declared ceiling — mirroring the sibling-class
//! level-range-gate idiom (`supported_ranger_level` is generalized from
//! `1..=9` to `1..=10` via `MAX_SUPPORTED_RANGER_LEVEL = 10`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Ranger class table) were
//! read directly before writing any code or test:
//!
//! - level 10 base attack bonus is +10 (full BAB, genuinely risen from +9 —
//!   the table's own "+10/+5" iterative notation is not modeled anywhere in
//!   this codebase, only the flat base value) and base saves are +7
//!   Fortitude and +7 Reflex (both good, `10 / 2 + 2 = 7`, both genuinely
//!   risen from +6) and +3 Will (poor, `10 / 3 = 3`, numerically unchanged
//!   from level 9, an integer-division coincidence) — confirmed by the same
//!   formulas already grounded at levels 1-9, not re-derived.
//! - Track GENUINELY RISES to 5 (`max(10 / 2, 1) = 5`, up from 4 at levels
//!   8-9).
//! - the PF1 Core Rulebook Ranger class table's level-10 "Special" column
//!   reads "3rd favored enemy, combat style feat" (verified independently
//!   against both primary sources, checked rather than assumed away):
//!   - the THIRD combat-style bonus feat is grounded by this slice as a
//!     restricted-list choice recognition
//!     (`class_chassis.ranger.combat_style_bonus_feat_3_choice`), mirroring
//!     the level-2/level-6 grants exactly — the Archery style's own
//!     10th-level list adds Pinpoint Targeting and Shot on the Run (the
//!     Two-Weapon Combat style's own adds Greater Two-Weapon Fighting and
//!     Two-Weapon Rend), and the fixture selects Shot on the Run; +0
//!     recognition only, no feat mechanics grounded.
//!   - the "3rd favored enemy" interval (a third enemy-type selection PLUS
//!     the rule's own second +2 bonus-increase-target choice, per "the
//!     bonus against any one favored enemy... increases by +2" at each of
//!     the 5th/10th/15th/20th intervals) is a real, newly-discovered
//!     multi-record burden deliberately left named-but-unproven this slice
//!     — mirroring exactly how the level-8 "2nd favored terrain"
//!     multi-record burden was deferred to its own future slice — so
//!     nothing is fabricated for it (a dedicated negative test pins the
//!     absence).
//! - the favored-enemy magnitudes carry over unchanged (+4/+2, the 5th-level
//!   interval's increase target still naming the first enemy); the
//!   favored-terrain count stays 2 (next grant at 13th); Hunter's Bond,
//!   Endurance, Woodland Stride, Swift Tracker, and Evasion all stay
//!   granted, not re-derived.
//!
//! It deliberately does not touch the hybrid spell burden, the
//! favored-enemy conditional application, or any check-execution engine
//! (all stay named-but-unproven, unchanged from levels 1-9), and it does
//! not ground Ranger level 11+. It also preserves the accepted Ranger
//! level-1..level-9 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const RANGER_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level9_sd13_deterministic_input.txt");

const RANGER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level10_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BASE_ATTACK_ID: &str = "class_chassis.ranger.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.ranger.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.ranger.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.ranger.base_save.will";
const TRACK_ID: &str = "class_chassis.ranger.track";
const FAVORED_ENEMY_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_skill_bonus";
const FAVORED_ENEMY_2_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_2_skill_bonus";
const COMBAT_STYLE_BONUS_FEAT_3_CHOICE_ID: &str =
    "class_chassis.ranger.combat_style_bonus_feat_3_choice";
const FAVORED_TERRAIN_ID: &str = "class_feature.ranger.favored_terrain";
const RANGER_EVASION_ID: &str = "class_feature.ranger.evasion";
const SWIFT_TRACKER_ID: &str = "class_feature.ranger.swift_tracker";

// ----- Base attack bonus and saves at level 10 -----

#[test]
fn ranger_level10_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 10,
        "Ranger level 10 full-BAB progression must equal 10, genuinely risen from 9: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 7,
        "Ranger level 10 good Fortitude (10/2+2) must equal 7, genuinely risen from 6"
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 7,
        "Ranger level 10 good Reflex (10/2+2) must equal 7, genuinely risen from 6"
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(will.value, 3, "Ranger level 10 poor Will (10/3) must equal 3");
}

// ----- Track genuinely rises to 5 at level 10 -----

#[test]
fn ranger_level10_track_rises_to_five() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(
        track.value, 5,
        "Ranger level 10 Track (max(10/2, 1)) must equal 5, genuinely risen from 4 at levels \
         8-9: {}",
        track.detail
    );
}

// ----- The THIRD combat-style bonus feat is newly recognized at level 10 -----

#[test]
fn ranger_level10_grounds_the_third_combat_style_bonus_feat_choice() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let third_feat = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_3_CHOICE_ID);
    assert_eq!(
        third_feat.value, 0,
        "the third combat-style bonus-feat recognition must carry no fabricated mechanical \
         value"
    );
    assert!(
        third_feat.detail.contains("Shot on the Run"),
        "the third combat-style bonus-feat recognition must name the Shot on the Run \
         selection from the Archery style's own 10th-level restricted list: {}",
        third_feat.detail
    );
}

// ----- The 3rd-favored-enemy interval stays entirely named-but-unproven -----

#[test]
fn ranger_level10_does_not_fabricate_the_third_favored_enemy_interval() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("favored_enemy_3")),
        "level-10 Ranger must not fabricate any third-favored-enemy record (the 10th-level \
         interval is a multi-record burden deferred to its own slice, mirroring the \
         2nd-favored-terrain precedent): {:?}",
        computation.explanations
    );

    // The existing magnitudes carry over unchanged.
    let first_skill = explanation(&computation, FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        first_skill.value, 4,
        "the first favored enemy's skill bonus must stay +4 at level 10 (only the 5th-level \
         interval's increase is grounded; the 10th-level interval's own increase stays \
         unproven)"
    );

    let second_skill = explanation(&computation, FAVORED_ENEMY_2_SKILL_BONUS_ID);
    assert_eq!(
        second_skill.value, 2,
        "the second favored enemy's skill bonus must stay +2 at level 10"
    );
}

// ----- Favored terrain / granted features carry over at level 10 -----

#[test]
fn ranger_level10_granted_features_carry_over() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let favored_terrain = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        favored_terrain.value, 2,
        "the favored-terrain count must stay 2 at level 10 (the next grant lands at 13th)"
    );

    for id in [RANGER_EVASION_ID, SWIFT_TRACKER_ID] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 10"
        );
    }
}

// ----- Negative control: the level-9 fixture is unaffected by this widening -----

#[test]
fn ranger_level9_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 9, "Ranger level 9 base attack bonus must stay 9");

    let track = explanation(&computation, TRACK_ID);
    assert_eq!(track.value, 4, "Ranger level 9 Track must stay 4");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == COMBAT_STYLE_BONUS_FEAT_3_CHOICE_ID),
        "level-9 Ranger must NOT gain the third combat-style feat record — it is a 10th-level \
         grant: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 13 stays unrecognized by this slice -----
//
// SD18 widening (cycle-2026-07-14T2300, tests/sd18_ranger_level11_quarry.rs):
// Ranger level 11 is now genuinely recognized (base attack bonus rises and
// Quarry is newly grounded), so this boundary control moved to level 12,
// mirroring the exact same boundary move each of the Barbarian/Bard/Cleric/
// Druid/Fighter/Monk/Paladin/Rogue/Sorcerer/Wizard level-11 widening cycles
// made for their own sibling level-10 progression tests. A further SD18
// widening (cycle-2026-07-15T0900, tests/sd18_ranger_level12_widening.rs)
// now genuinely recognizes Ranger level 12 too (base attack/saves/Track all
// rise and Camouflage is newly grounded), so this boundary control moved
// once more to level 13, mirroring the same sibling classes' own
// level-11-to-level-12 boundary move. A still further SD18 widening
// (cycle-2026-07-15T1400, tests/sd18_ranger_level13_widening.rs) now
// genuinely recognizes Ranger level 13 too (base attack rises, the third
// favored terrain and the spell-level access ladder's 4th column are newly
// grounded), so this boundary control moved once more to level 14. A still
// further SD18 widening (cycle-2026-07-15T2100,
// tests/sd18_ranger_level14_widening.rs) now genuinely recognizes Ranger
// level 14 too (base attack and both good saves rise, the fourth
// combat-style bonus feat and the base spells-per-day table's 4th-level
// column are newly grounded), again to level 15 (base attack rises,
// poor Will rises, the fourth favored enemy and the base spells-per-day
// table's 3rd-level column rise are newly grounded), again to level 16
// (base attack and both good saves rise, Improved Evasion and the base
// spells-per-day table's 1st-level column rise are newly grounded), and
// again to level 17 (cycle-2026-07-15T7000,
// tests/sd18_ranger_level17_hide_in_plain_sight.rs: base attack rises,
// Hide in Plain Sight and the base spells-per-day table's 1st-level
// column rise are newly grounded), and again to level 18
// (cycle-2026-07-16T0244, tests/sd18_ranger_level18_widening.rs: base
// attack and all three base saves rise, the fourth favored terrain, the
// fifth combat style bonus feat, and the base spells-per-day table's
// 4th-level column rise are newly grounded), and again to level 19
// (cycle-2026-07-16T3200, tests/sd18_ranger_level19_widening.rs: base
// attack rises, Improved Quarry and the base spells-per-day table's
// 3rd-level column rise are newly grounded), and again to level 20
// (cycle-2026-07-16T1600, tests/sd18_ranger_level20_widening.rs: base
// attack and both good saves rise, the fifth favored enemy, Master
// Hunter, and the base spells-per-day table's 2nd/4th-level columns rise
// are newly grounded), so this boundary control moves once more to level
// 21 (a pure implementation-gate check, since PF1 has no 21st character
// level).
#[test]
fn ranger_level_21_is_not_promoted_by_this_slice() {
    let level_21 = RANGER_LEVEL10_FIXTURE.replace("class:ranger:10", "class:ranger:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "level-21 Ranger must not gain any bounded ranger explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level10_recognition() {
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
fn multiclass_ranger_level10_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL10_FIXTURE.replace(
        "class_level=class:ranger:10",
        "class_level=class:ranger:10\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "multiclass Ranger must not gain any bounded ranger explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-10 widening -----

#[test]
fn matrix_ranger_row_names_level_10_widening() {
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
            .contains("sd13_ranger_level10_progression"),
        "ranger row must cite the live SD13-E5 level-10 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "ranger partial note must name the level-10 widening: {note}"
    );
}
