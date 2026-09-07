//! SD13-E5 Barbarian level-8 progression grounding proof.
//!
//! Widens the accepted Barbarian level-1/level-2/level-3/level-4/level-5/
//! level-6/level-7 martial chassis baseline (`tests/sd13_barbarian_level1_chassis_baseline.rs`,
//! `tests/sd13_barbarian_level2_progression.rs`,
//! `tests/sd13_barbarian_level3_progression.rs`,
//! `tests/sd13_barbarian_level4_progression.rs`,
//! `tests/sd13_barbarian_level5_progression.rs`,
//! `tests/sd13_barbarian_level6_progression.rs`,
//! `tests/sd13_barbarian_level7_progression.rs`) to barbarian level 8,
//! mirroring the Fighter/Paladin/Rogue/Monk level-range-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=7` to `1..=8` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 8`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Barbarian class table) were read directly
//! before writing any code or test: level 8 base attack bonus is +8/+3,
//! saves are Fort +6 / Ref +2 / Will +2, and the level-8 "Special" column
//! reads "Rage power" only -- confirmed NOT another Damage Reduction rise
//! (Damage Reduction rises to 2/- at level 10, not 8th) and confirmed to BE
//! another Rage Power grant (Rage Powers are granted at 2nd, 4th, 6th, 8th,
//! and 10th barbarian level). It proves:
//!
//! - base attack bonus at level 8 is grounded by the same full-BAB formula
//!   (`classlevel`) already grounded at levels 1-7: `8`.
//! - base saves at level 8 are grounded by the same good-Fortitude/poor-
//!   Reflex/poor-Will formulas already grounded at levels 1-7, extended to
//!   level 8: Fortitude `8 / 2 + 2 = 6`, Reflex `8 / 3 = 2`, Will `8 / 3 = 2`.
//! - fast movement stays the flat +10 ft. value at level 8, confirmed via the
//!   same formula, not a new record -- the PF1 Core Rulebook fast-movement
//!   bonus does not scale with level.
//! - rage rounds per day at level 8 grows by the PF1 Core Rulebook Rage rule
//!   ("at each level after 1st, she can rage for 2 additional rounds"):
//!   `4 + Constitution modifier + 2 * (level - 1)`. On the Con 16 fixture
//!   (modifier +3) this is `4 + 3 + 2 * 7 = 21` at level 8 (was `19` at level
//!   7) -- i.e. Con modifier + 18, matching the PF1 CRB's own progression.
//! - the four flat while-raging constants (+4 Str, +4 Con, +2 Will, -2 AC)
//!   stay exactly the same magnitudes at level 8, confirmed via the same
//!   formula, not new records -- the PF1 Core Rulebook Rage constants do not
//!   scale with level.
//! - the illiteracy-absence rules-correction record still applies,
//!   unconditionally, at level 8.
//! - Uncanny Dodge and Improved Uncanny Dodge stay granted at level 8 (not
//!   re-derived), grounded as the same bounded identity/recognition records
//!   already grounded at levels 2-7 and 5-7, respectively.
//! - Trap Sense stays granted at level 8 with the SAME +2 magnitude already
//!   grounded at levels 6-7 (barbarian level / 3, floor: `8 / 3 = 2`), not
//!   re-derived -- verified independently against both primary sources that
//!   the Trap Sense bonus does NOT increase again until barbarian level 9.
//! - Damage Reduction stays granted at level 8 with the SAME 1-point
//!   magnitude already grounded at level 7, not re-derived -- verified
//!   independently against both primary sources that Damage Reduction does
//!   NOT rise again until barbarian level 10 (to 2/-).
//!
//! It also verifies (per the operator brief) whether Barbarian gains an
//! actual new class feature at 8th level. Both primary sources' level-8
//! "Special" column reads "Rage power" only -- verified independently
//! against d20pfsrd and legacy.aonprd.com. This is the SAME genuinely
//! open-ended choice-list feature already deliberately left named-but-
//! unproven at levels 2, 4, and 6 (a new-subsystem-shaped burden), not a new
//! type of class feature, so this widening grounds no new pillar beyond the
//! arithmetic extension above: no new choice-slot and no new diagnostic is
//! added for it, and no rage-power-selection-slot-count engine is invented.
//!
//! It deliberately does not implement the rage-state execution engine
//! (activation, round consumption, fatigue, stat application), weapon
//! familiarity, the Rage Power choice-list feature, the Improved Uncanny
//! Dodge flanking-resolution/rogue-level-comparison engine, the Damage
//! Reduction application/resolution engine, flat-footed-state tracking,
//! Armor Class computation, invisibility detection, or level-9+ Barbarian
//! progression. It also preserves the accepted Barbarian
//! level-1/level-2/level-3/level-4/level-5/level-6/level-7 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::{
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARBARIAN_LEVEL7_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level7_sd13_deterministic_input.txt"
);

const BARBARIAN_LEVEL8_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level8_sd13_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";
const BARBARIAN_DAMAGE_REDUCTION_ID: &str = "class_feature.barbarian.damage_reduction";

// ----- Base attack bonus at level 8 -----

#[test]
fn barbarian_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Barbarian level 8 full-BAB progression (classlevel) must equal 8: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("level 8"),
        "barbarian base-attack explanation must name level 8: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 -----

#[test]
fn barbarian_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 6, "Barbarian level 8 good Fortitude (8/2+2) must equal 6");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "barbarian Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 2, "Barbarian level 8 poor Reflex (8/3) must equal 2");
    assert!(
        reflex.detail.to_lowercase().contains("poor"),
        "barbarian Reflex explanation must name it as a poor save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 2, "Barbarian level 8 poor Will (8/3) must equal 2");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "barbarian Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Fast movement stays the flat +10 ft. value at level 8 -----

#[test]
fn barbarian_level8_fast_movement_stays_flat_ten_feet() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(
        fast_movement.value, 10,
        "Barbarian fast movement must stay +10 ft. at level 8, not a new record: {}",
        fast_movement.detail
    );
}

// ----- Rage rounds per day grows by the PF1 CRB +2-rounds-per-level rule -----

#[test]
fn barbarian_level8_rage_rounds_per_day_grows_by_two_again() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Con 16 -> modifier +3. Level 8: 4 + 3 + 2 * (8 - 1) = 21 (was 19 at level
    // 7) -- i.e. Con modifier + 18, matching the PF1 CRB's own "+2 rounds per
    // level after 1st" progression through level 8.
    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 21,
        "Barbarian level 8 rage rounds per day must be 4 + Con modifier (+3) + 2 * (level - 1) \
         = 21: {}",
        rage_rounds.detail
    );
    assert!(
        rage_rounds.detail.contains("2 additional")
            || rage_rounds.detail.contains("2 * (level"),
        "rage rounds per day at level 8 must document the +2-rounds-per-level-after-1st rule: {}",
        rage_rounds.detail
    );
}

// ----- Flat rage constants are unchanged at level 8 -----

#[test]
fn barbarian_level8_flat_rage_constants_are_unchanged() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
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
            "rage constant '{id}' must stay {expected} at level 8, not a new record"
        );
    }
}

// ----- Illiteracy-absence record still applies at level 8 -----

#[test]
fn barbarian_level8_illiteracy_absence_still_applies() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the illiteracy-absent record documents a rules correction; it carries no mechanical value"
    );
}

// ----- Uncanny Dodge stays granted at level 8, not re-derived -----

#[test]
fn barbarian_level8_keeps_uncanny_dodge_grounded() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge must carry no fabricated mechanical value at level 8: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("granted"),
        "uncanny dodge explanation at level 8 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

// ----- Trap Sense stays granted at level 8, at the SAME +2 magnitude as level 6-7 -----

#[test]
fn barbarian_level8_trap_sense_stays_at_the_same_plus_two_magnitude() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 2,
        "Trap Sense at level 8 must stay at the PF1 Core Rulebook's barbarian-level/3 formula \
         value (+2, unchanged since level 6 -- the bonus does not rise again until level 9): {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.contains("Trap Sense"),
        "trap sense explanation must name the Trap Sense class feature: {}",
        trap_sense.detail
    );
    assert!(
        trap_sense.detail.to_lowercase().contains("granted"),
        "trap sense explanation at level 8 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

// ----- Improved Uncanny Dodge stays granted at level 8, not re-derived -----

#[test]
fn barbarian_level8_keeps_improved_uncanny_dodge_grounded() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_uncanny_dodge = explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge must carry no fabricated mechanical value at level 8: {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        improved_uncanny_dodge.detail.to_lowercase().contains("granted"),
        "improved uncanny dodge explanation at level 8 must state it is granted, not absent: {}",
        improved_uncanny_dodge.detail
    );
}

// ----- Damage Reduction stays granted at level 8, at the SAME 1-point magnitude as level 7 -----

#[test]
fn barbarian_level8_damage_reduction_stays_at_the_same_flat_magnitude() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 1,
        "Damage Reduction at level 8 must stay at the flat PF1 Core Rulebook magnitude (1 \
         point, DR 1/-, unchanged since level 7 -- the magnitude does not rise again until \
         level 10): {}",
        damage_reduction.detail
    );
    assert!(
        damage_reduction.detail.to_lowercase().contains("granted"),
        "damage reduction explanation at level 8 must state it is granted: {}",
        damage_reduction.detail
    );
}

// ----- Still blocked: rage-state execution engine and generic diagnostics -----

#[test]
fn barbarian_level8_stays_blocked_on_rage_execution() {
    let input = load(BARBARIAN_LEVEL8_FIXTURE);
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
                "level-8 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    assert_eq!(
        computation.base_attack_bonus, 8,
        "barbarian is now recognized by table_class_id; level 8 full BAB is +8"
    );
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn barbarian_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = BARBARIAN_LEVEL8_FIXTURE.replace("class:barbarian:8", "class:barbarian:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "level-9 Barbarian is now recognized by the later level-9 widening slice \
         (tests/sd13_barbarian_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level8_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")
                || e.id == BARBARIAN_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_TRAP_SENSE_ID
                || e.id == BARBARIAN_IMPROVED_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_DAMAGE_REDUCTION_ID),
        "the Fighter chassis must not surface any barbarian-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Barbarian is not promoted -----

#[test]
fn multiclass_barbarian_level8_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL8_FIXTURE.replace(
        "class_level=class:barbarian:8",
        "class_level=class:barbarian:8\nclass_level=class:fighter:1",
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
                || e.id == BARBARIAN_IMPROVED_UNCANNY_DODGE_ID
                || e.id == BARBARIAN_DAMAGE_REDUCTION_ID),
        "multiclass Barbarian must not gain any bounded barbarian chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Barbarian level 1/level 2/level 3/level 4/level 5/level 6/level 7 stays unchanged -----

#[test]
fn barbarian_level7_truth_is_unchanged_by_the_level8_widening() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 7, "Barbarian level 7 full BAB must stay +7");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 19,
        "Barbarian level 7 rage rounds per day must stay 4 + Con modifier (+3) + 2 * 6 = 19"
    );

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 2, "Barbarian level 7 Trap Sense must stay +2");

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 1,
        "Barbarian level 7 Damage Reduction must stay at the flat 1-point magnitude"
    );
}

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_barbarian_row_names_level_8_widening() {
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
            .contains("sd13_barbarian_level8_progression"),
        "barbarian row must cite the live SD13-E5 level-8 proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 8") || note.contains("level-8"),
        "barbarian partial note must name the level-8 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("rage execution") || note.contains("rage-state execution"),
        "barbarian partial note must keep naming the rage-state execution engine as unproven: \
         {note}"
    );
    assert!(
        note.to_lowercase().contains("rage power"),
        "barbarian partial note must keep naming the Rage Power choice-list feature as \
         unproven: {note}"
    );
}
