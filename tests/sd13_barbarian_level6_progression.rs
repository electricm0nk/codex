//! SD13-E5 Barbarian level-6 progression grounding proof.
//!
//! Widens the accepted Barbarian level-1/level-2/level-3/level-4/level-5
//! martial chassis baseline (`tests/sd13_barbarian_level1_chassis_baseline.rs`,
//! `tests/sd13_barbarian_level2_progression.rs`,
//! `tests/sd13_barbarian_level3_progression.rs`,
//! `tests/sd13_barbarian_level4_progression.rs`,
//! `tests/sd13_barbarian_level5_progression.rs`) to barbarian level 6,
//! mirroring the Fighter/Paladin/Rogue/Monk level-range-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=5` to `1..=6` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 6`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Barbarian class table) were read directly
//! before writing any code or test: level 6 base attack bonus is +6, saves
//! are Fort +5 / Ref +2 / Will +2, and the level-6 "Special" column reads
//! "Rage power, trap sense +2" -- another Rage Power grant (the same
//! open-ended choice-list feature already named-but-unproven at levels 2 and
//! 4, NOT a new feature type) plus Trap Sense's own flat magnitude rising to
//! +2. It proves:
//!
//! - base attack bonus at level 6 is grounded by the same full-BAB formula
//!   (`classlevel`) already grounded at levels 1-5: `6`.
//! - base saves at level 6 are grounded by the same good-Fortitude/poor-
//!   Reflex/poor-Will formulas already grounded at levels 1-5, extended to
//!   level 6: Fortitude `6 / 2 + 2 = 5`, Reflex `6 / 3 = 2`, Will `6 / 3 = 2`.
//! - fast movement stays the flat +10 ft. value at level 6, confirmed via the
//!   same formula, not a new record -- the PF1 Core Rulebook fast-movement
//!   bonus does not scale with level.
//! - rage rounds per day at level 6 grows by the PF1 Core Rulebook Rage rule
//!   ("at each level after 1st, she can rage for 2 additional rounds"):
//!   `4 + Constitution modifier + 2 * (level - 1)`. On the Con 16 fixture
//!   (modifier +3) this is `4 + 3 + 2 * 5 = 17` at level 6 (was `15` at level
//!   5) -- i.e. Con modifier + 14, matching the PF1 CRB's own progression.
//! - the four flat while-raging constants (+4 Str, +4 Con, +2 Will, -2 AC)
//!   stay exactly the same magnitudes at level 6, confirmed via the same
//!   formula, not new records -- the PF1 Core Rulebook Rage constants do not
//!   scale with level.
//! - the illiteracy-absence rules-correction record still applies,
//!   unconditionally, at level 6.
//! - Uncanny Dodge and Improved Uncanny Dodge stay granted at level 6 (not
//!   re-derived), grounded as the same bounded identity/recognition records
//!   already grounded at levels 2-5 and 5, respectively.
//! - Trap Sense's own flat magnitude GENUINELY RISES at level 6 (barbarian
//!   level / 3, floor: `6 / 3 = 2`, up from `1` at levels 3-5) -- verified
//!   independently against both primary sources that the Trap Sense bonus
//!   "increases by +1 every three barbarian levels thereafter (6th, 9th,
//!   12th, 15th, and 18th level)", matching the class table's own level-6
//!   "Trap sense +2" entry exactly. This is the same pre-existing formula
//!   (`barbarian level / 3`), not a new record or a re-derivation.
//!
//! It also verifies (per the operator brief) whether Barbarian gains any new,
//! flat-shaped class feature at 6th level beyond another Rage Power grant.
//! Both primary sources' level-6 "Special" column reads "Rage power, trap
//! sense +2" -- the Rage Power entry is another instance of the same
//! genuinely open-ended choice-list feature already deliberately left
//! named-but-unproven at levels 2 and 4 (a new-subsystem-shaped burden), NOT
//! a new type of class feature, and Trap Sense's own rise is already
//! accounted for above via the pre-existing formula. This slice therefore
//! grounds no new pillar/record at level 6 beyond the arithmetic extensions
//! above, mirroring exactly how the level-2 and level-4 Rage Power grants
//! were each left unrecognized.
//!
//! It deliberately does not implement the rage-state execution engine
//! (activation, round consumption, fatigue, stat application), weapon
//! familiarity, the Rage Power choice-list feature (a genuinely open-ended
//! choice-list feature, a new-subsystem-shaped burden, now including the
//! level-6 Rage Power grant), the Improved Uncanny Dodge
//! flanking-resolution/rogue-level-comparison engine, flat-footed-state
//! tracking, Armor Class computation, invisibility detection, or level-7+
//! Barbarian progression. It also preserves the accepted Barbarian
//! level-1/level-2/level-3/level-4/level-5 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::{
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const BARBARIAN_LEVEL5_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level5_sd13_deterministic_input.txt"
);

const BARBARIAN_LEVEL6_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level6_sd13_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";

// ----- Base attack bonus at level 6 -----

#[test]
fn barbarian_level6_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Barbarian level 6 full-BAB progression (classlevel) must equal 6: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("level 6"),
        "barbarian base-attack explanation must name level 6: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 6 -----

#[test]
fn barbarian_level6_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 5, "Barbarian level 6 good Fortitude (6/2+2) must equal 5");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "barbarian Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 2, "Barbarian level 6 poor Reflex (6/3) must equal 2");
    assert!(
        reflex.detail.to_lowercase().contains("poor"),
        "barbarian Reflex explanation must name it as a poor save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 2, "Barbarian level 6 poor Will (6/3) must equal 2");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "barbarian Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Fast movement stays the flat +10 ft. value at level 6 -----

#[test]
fn barbarian_level6_fast_movement_stays_flat_ten_feet() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(
        fast_movement.value, 10,
        "Barbarian fast movement must stay +10 ft. at level 6, not a new record: {}",
        fast_movement.detail
    );
}

// ----- Rage rounds per day grows by the PF1 CRB +2-rounds-per-level rule -----

#[test]
fn barbarian_level6_rage_rounds_per_day_grows_by_two_again() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Con 16 -> modifier +3. Level 6: 4 + 3 + 2 * (6 - 1) = 17 (was 15 at level
    // 5, 13 at level 4) -- i.e. Con modifier + 14, matching the PF1 CRB's own
    // "+2 rounds per level after 1st" progression through level 6.
    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 17,
        "Barbarian level 6 rage rounds per day must be 4 + Con modifier (+3) + 2 * (level - 1) \
         = 17: {}",
        rage_rounds.detail
    );
    assert!(
        rage_rounds.detail.contains("2 additional")
            || rage_rounds.detail.contains("2 * (level"),
        "rage rounds per day at level 6 must document the +2-rounds-per-level-after-1st rule: {}",
        rage_rounds.detail
    );
}

// ----- Flat rage constants are unchanged at level 6 -----

#[test]
fn barbarian_level6_flat_rage_constants_are_unchanged() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 4),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 4),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 2),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "rage constant '{id}' must stay {expected} at level 6, not a new record"
        );
    }
}

// ----- Illiteracy-absence record still applies at level 6 -----

#[test]
fn barbarian_level6_illiteracy_absence_still_applies() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the illiteracy-absent record documents a rules correction; it carries no mechanical value"
    );
}

// ----- Uncanny Dodge stays granted at level 6, not re-derived -----

#[test]
fn barbarian_level6_keeps_uncanny_dodge_grounded() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge must carry no fabricated mechanical value at level 6: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("granted"),
        "uncanny dodge explanation at level 6 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

// ----- Trap Sense genuinely RISES to +2 at level 6 -----

#[test]
fn barbarian_level6_trap_sense_rises_to_plus_two() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 2,
        "Trap Sense at level 6 must rise to the PF1 Core Rulebook's barbarian-level/3 formula \
         value (+2, up from +1 at levels 3-5 -- the bonus rises again every three barbarian \
         levels thereafter): {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.contains("Trap Sense"),
        "trap sense explanation must name the Trap Sense class feature: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 6 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

// ----- Improved Uncanny Dodge stays granted at level 6, not re-derived -----

#[test]
fn barbarian_level6_keeps_improved_uncanny_dodge_grounded() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_uncanny_dodge = explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge must carry no fabricated mechanical value at level 6: {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        improved_uncanny_dodge.detail.to_lowercase().contains("granted"),
        "improved uncanny dodge explanation at level 6 must state it is granted, not absent: {}",
        improved_uncanny_dodge.detail
    );
}

// ----- Still blocked: rage-state execution engine and generic diagnostics -----

#[test]
fn barbarian_level6_stays_blocked_on_rage_execution() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.barbarian.rage_execution.rounds_exceeded")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let not_raging = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.barbarian.rage_execution.not_raging");
            assert!(
                not_raging.is_some(),
                "level-6 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    assert_eq!(
        computation.base_attack_bonus, 6,
        "barbarian is now recognized by table_class_id; level 6 full BAB is +6"
    );
}

// ----- Negative control: level 7 was later widened into the supported tranche -----

#[test]
fn barbarian_level_7_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 7 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_barbarian_level7_progression.rs) widened the level-range gate
    // to level 7 (mirroring the Rogue/Monk level-range gate idiom) and grounded
    // Damage Reduction 1/- as a new flat-magnitude record; this negative control
    // is superseded, not violated — pin the new truth here too so this file
    // stays internally consistent. The frontier this file's own slice actually
    // drew is now level 8, covered by `barbarian_level_8_is_not_promoted_by_this_slice`
    // in `tests/sd13_barbarian_level7_progression.rs`.
    let level_7 = BARBARIAN_LEVEL6_FIXTURE.replace("class:barbarian:6", "class:barbarian:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.barbarian.base_attack_bonus"),
        "level-7 Barbarian is supported since the SD13-E5 level-7 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID),
        "level-7 Barbarian must keep the Uncanny Dodge explanation grounded at level 2"
    );
    assert!(
        has_explanation(&computation, BARBARIAN_TRAP_SENSE_ID),
        "level-7 Barbarian must keep the Trap Sense explanation grounded at level 3"
    );
    assert!(
        has_explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID),
        "level-7 Barbarian must keep the Improved Uncanny Dodge explanation grounded at level 5"
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level6_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")
                || e.id == BARBARIAN_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_TRAP_SENSE_ID
                || e.id == BARBARIAN_IMPROVED_UNCANNY_DODGE_ID),
        "the Fighter chassis must not surface any barbarian-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Barbarian is not promoted -----

#[test]
fn multiclass_barbarian_level6_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL6_FIXTURE.replace(
        "class_level=class:barbarian:6",
        "class_level=class:barbarian:6\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")
                || e.id == BARBARIAN_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_TRAP_SENSE_ID
                || e.id == BARBARIAN_IMPROVED_UNCANNY_DODGE_ID),
        "multiclass Barbarian must not gain any bounded barbarian chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Barbarian level 1/level 2/level 3/level 4/level 5 stays unchanged -----

#[test]
fn barbarian_level5_truth_is_unchanged_by_the_level6_widening() {
    let input = load(BARBARIAN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 5, "Barbarian level 5 full BAB must stay +5");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 15,
        "Barbarian level 5 rage rounds per day must stay 4 + Con modifier (+3) + 2 * 4 = 15"
    );

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 1, "Barbarian level 5 Trap Sense must stay +1");

    let improved_uncanny_dodge = explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Barbarian level 5 Improved Uncanny Dodge must stay a grant-only value-0 record"
    );
}

// ----- Control plane: the matrix note names the level-6 widening -----

#[test]
fn matrix_barbarian_row_names_level_6_widening() {
    let matrix = seeded_current_truth();
    let barbarian = matrix
        .row("class.barbarian.bounded_progression")
        .expect("barbarian bounded_progression row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(barbarian.support_state, SupportState::Supported);
    assert_eq!(barbarian.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        barbarian.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        barbarian
            .grounding_ref
            .contains("sd13_barbarian_level6_progression"),
        "barbarian row must cite the live SD13-E5 level-6 proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 6") || note.contains("level-6"),
        "barbarian partial note must name the level-6 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("trap sense"),
        "barbarian partial note must keep naming Trap Sense: {note}"
    );
    assert!(
        note.contains("rage execution") || note.contains("rage-state execution"),
        "barbarian partial note must keep naming the rage-state execution engine as unproven: \
         {note}"
    );
    assert!(
        note.to_lowercase().contains("rage power"),
        "barbarian partial note must keep naming the Rage Power choice-list feature as \
         unproven: {note}"
    );
}
