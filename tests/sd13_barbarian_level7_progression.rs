//! SD13-E5 Barbarian level-7 progression grounding proof.
//!
//! Widens the accepted Barbarian level-1/level-2/level-3/level-4/level-5/
//! level-6 martial chassis baseline (`tests/sd13_barbarian_level1_chassis_baseline.rs`,
//! `tests/sd13_barbarian_level2_progression.rs`,
//! `tests/sd13_barbarian_level3_progression.rs`,
//! `tests/sd13_barbarian_level4_progression.rs`,
//! `tests/sd13_barbarian_level5_progression.rs`,
//! `tests/sd13_barbarian_level6_progression.rs`) to barbarian level 7,
//! mirroring the Fighter/Paladin/Rogue/Monk level-range-gate idiom
//! (`supported_barbarian_level` is generalized from `1..=6` to `1..=7` via
//! `MAX_SUPPORTED_BARBARIAN_LEVEL = 7`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Barbarian class table) were read directly
//! before writing any code or test: level 7 base attack bonus is +7, saves
//! are Fort +5 / Ref +2 / Will +2, and the level-7 "Special" column reads
//! "Damage reduction 1/-" only -- a genuinely new class feature, NOT another
//! Rage Power grant (Rage Powers are granted at 2nd, 4th, 6th, 8th, and 10th
//! barbarian level, not 7th). It proves:
//!
//! - base attack bonus at level 7 is grounded by the same full-BAB formula
//!   (`classlevel`) already grounded at levels 1-6: `7`.
//! - base saves at level 7 are grounded by the same good-Fortitude/poor-
//!   Reflex/poor-Will formulas already grounded at levels 1-6, extended to
//!   level 7: Fortitude `7 / 2 + 2 = 5`, Reflex `7 / 3 = 2`, Will `7 / 3 = 2`.
//! - fast movement stays the flat +10 ft. value at level 7, confirmed via the
//!   same formula, not a new record -- the PF1 Core Rulebook fast-movement
//!   bonus does not scale with level.
//! - rage rounds per day at level 7 grows by the PF1 Core Rulebook Rage rule
//!   ("at each level after 1st, she can rage for 2 additional rounds"):
//!   `4 + Constitution modifier + 2 * (level - 1)`. On the Con 16 fixture
//!   (modifier +3) this is `4 + 3 + 2 * 6 = 19` at level 7 (was `17` at level
//!   6) -- i.e. Con modifier + 16, matching the PF1 CRB's own progression.
//! - the four flat while-raging constants (+4 Str, +4 Con, +2 Will, -2 AC)
//!   stay exactly the same magnitudes at level 7, confirmed via the same
//!   formula, not new records -- the PF1 Core Rulebook Rage constants do not
//!   scale with level.
//! - the illiteracy-absence rules-correction record still applies,
//!   unconditionally, at level 7.
//! - Uncanny Dodge and Improved Uncanny Dodge stay granted at level 7 (not
//!   re-derived), grounded as the same bounded identity/recognition records
//!   already grounded at levels 2-6 and 5-6, respectively.
//! - Trap Sense stays granted at level 7 with the SAME +2 magnitude already
//!   grounded at level 6 (barbarian level / 3, floor: `7 / 3 = 2`), not
//!   re-derived -- verified independently against both primary sources that
//!   the Trap Sense bonus does NOT increase again until barbarian level 9.
//!
//! It also verifies (per the operator brief) whether Barbarian gains an
//! actual new class feature at 7th level. Both primary sources' level-7
//! "Special" column reads "Damage reduction 1/-" -- verified independently
//! against d20pfsrd and legacy.aonprd.com: "At 7th level, a barbarian gains
//! damage reduction. Subtract 1 from the damage the barbarian takes each
//! time she is dealt damage from a weapon or a natural attack." Both primary
//! sources' level-7 row was also checked for a Rage Power grant (the
//! genuinely open-ended choice-list feature already deliberately left
//! named-but-unproven at levels 2, 4, and 6): neither source names a Rage
//! Power at 7th level (Rage Powers are granted at 2nd, 4th, 6th, 8th, and
//! 10th barbarian level per the class table's own "Special" column), so
//! there is no new Rage Power grant to leave named-but-unproven here, and no
//! rage-power-selection-slot-count engine is invented. Damage Reduction's OWN
//! magnitude (1 point, not bypassed by any weapon type -- "/-") is
//! flat/identity-shaped -- a pure numeric constant with a fixed damage-source
//! restriction, exactly mirroring how Trap Sense's own flat magnitude was
//! grounded (a bonus applied against a named damage source, never wired into
//! an actual resolution total) -- so it is newly grounded here
//! (`class_feature.barbarian.damage_reduction`, value 1, level-gate absence
//! below level 7, granted-but-unexecuted at or above it). The rule's own
//! APPLICATION piece (subtracting the value from incoming weapon/natural-attack
//! damage) is NOT computed: no damage-resolution engine, no
//! damage-reduction-bypass-material engine, and no incoming-damage total
//! exists anywhere in this codebase, so this slice grounds only the bounded
//! flat magnitude, never applying it to any actual damage total.
//!
//! It deliberately does not implement the rage-state execution engine
//! (activation, round consumption, fatigue, stat application), weapon
//! familiarity, the Rage Power choice-list feature (a genuinely open-ended
//! choice-list feature, a new-subsystem-shaped burden), the Improved Uncanny
//! Dodge flanking-resolution/rogue-level-comparison engine, the Damage
//! Reduction application/resolution engine, flat-footed-state tracking,
//! Armor Class computation, invisibility detection, or level-8+ Barbarian
//! progression. It also preserves the accepted Barbarian
//! level-1/level-2/level-3/level-4/level-5/level-6 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const BARBARIAN_LEVEL6_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level6_sd13_deterministic_input.txt"
);

const BARBARIAN_LEVEL7_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level7_sd13_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BARBARIAN_UNCANNY_DODGE_ID: &str = "class_feature.barbarian.uncanny_dodge";
const BARBARIAN_TRAP_SENSE_ID: &str = "class_feature.barbarian.trap_sense";
const BARBARIAN_IMPROVED_UNCANNY_DODGE_ID: &str =
    "class_feature.barbarian.improved_uncanny_dodge";
const BARBARIAN_DAMAGE_REDUCTION_ID: &str = "class_feature.barbarian.damage_reduction";

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

// ----- Base attack bonus at level 7 -----

#[test]
fn barbarian_level7_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Barbarian level 7 full-BAB progression (classlevel) must equal 7: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("level 7"),
        "barbarian base-attack explanation must name level 7: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 -----

#[test]
fn barbarian_level7_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 5, "Barbarian level 7 good Fortitude (7/2+2) must equal 5");
    assert!(
        fortitude.detail.to_lowercase().contains("good"),
        "barbarian Fortitude explanation must name it as a good save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 2, "Barbarian level 7 poor Reflex (7/3) must equal 2");
    assert!(
        reflex.detail.to_lowercase().contains("poor"),
        "barbarian Reflex explanation must name it as a poor save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 2, "Barbarian level 7 poor Will (7/3) must equal 2");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "barbarian Will explanation must name it as a poor save: {}",
        will.detail
    );
}

// ----- Fast movement stays the flat +10 ft. value at level 7 -----

#[test]
fn barbarian_level7_fast_movement_stays_flat_ten_feet() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(
        fast_movement.value, 10,
        "Barbarian fast movement must stay +10 ft. at level 7, not a new record: {}",
        fast_movement.detail
    );
}

// ----- Rage rounds per day grows by the PF1 CRB +2-rounds-per-level rule -----

#[test]
fn barbarian_level7_rage_rounds_per_day_grows_by_two_again() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Con 16 -> modifier +3. Level 7: 4 + 3 + 2 * (7 - 1) = 19 (was 17 at level
    // 6) -- i.e. Con modifier + 16, matching the PF1 CRB's own "+2 rounds per
    // level after 1st" progression through level 7.
    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 19,
        "Barbarian level 7 rage rounds per day must be 4 + Con modifier (+3) + 2 * (level - 1) \
         = 19: {}",
        rage_rounds.detail
    );
    assert!(
        rage_rounds.detail.contains("2 additional")
            || rage_rounds.detail.contains("2 * (level"),
        "rage rounds per day at level 7 must document the +2-rounds-per-level-after-1st rule: {}",
        rage_rounds.detail
    );
}

// ----- Flat rage constants are unchanged at level 7 -----

#[test]
fn barbarian_level7_flat_rage_constants_are_unchanged() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
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
            "rage constant '{id}' must stay {expected} at level 7, not a new record"
        );
    }
}

// ----- Illiteracy-absence record still applies at level 7 -----

#[test]
fn barbarian_level7_illiteracy_absence_still_applies() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the illiteracy-absent record documents a rules correction; it carries no mechanical value"
    );
}

// ----- Uncanny Dodge stays granted at level 7, not re-derived -----

#[test]
fn barbarian_level7_keeps_uncanny_dodge_grounded() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uncanny_dodge = explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID);
    assert_eq!(
        uncanny_dodge.value, 0,
        "Uncanny Dodge must carry no fabricated mechanical value at level 7: {}",
        uncanny_dodge.detail
    );
    assert!(
        uncanny_dodge.detail.to_lowercase().contains("granted"),
        "uncanny dodge explanation at level 7 must state it is granted, not absent: {}",
        uncanny_dodge.detail
    );
}

// ----- Trap Sense stays granted at level 7, at the SAME +2 magnitude as level 6 -----

#[test]
fn barbarian_level7_trap_sense_stays_at_the_same_plus_two_magnitude() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 2,
        "Trap Sense at level 7 must stay at the PF1 Core Rulebook's barbarian-level/3 formula \
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
        "trap sense explanation at level 7 must state it is granted, not absent: {}",
        trap_sense.detail
    );
}

// ----- Improved Uncanny Dodge stays granted at level 7, not re-derived -----

#[test]
fn barbarian_level7_keeps_improved_uncanny_dodge_grounded() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_uncanny_dodge = explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Improved Uncanny Dodge must carry no fabricated mechanical value at level 7: {}",
        improved_uncanny_dodge.detail
    );
    assert!(
        improved_uncanny_dodge.detail.to_lowercase().contains("granted"),
        "improved uncanny dodge explanation at level 7 must state it is granted, not absent: {}",
        improved_uncanny_dodge.detail
    );
}

// ----- Damage Reduction is newly granted at level 7 -----

#[test]
fn barbarian_level7_grounds_damage_reduction_flat_magnitude_only() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 1,
        "Damage Reduction must ground the flat PF1 Core Rulebook magnitude (1 point, DR 1/-): {}",
        damage_reduction.detail
    );
    assert!(
        damage_reduction.detail.to_lowercase().contains("granted"),
        "damage reduction explanation at level 7 must state it is granted: {}",
        damage_reduction.detail
    );
    assert!(
        damage_reduction.detail.to_lowercase().contains("damage reduction"),
        "damage reduction explanation must name the Damage Reduction class feature: {}",
        damage_reduction.detail
    );
}

#[test]
fn barbarian_level6_does_not_yet_grant_damage_reduction() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let damage_reduction = explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID);
    assert_eq!(
        damage_reduction.value, 0,
        "Damage Reduction absence record carries no mechanical value either: {}",
        damage_reduction.detail
    );
    assert!(
        !damage_reduction.detail.to_lowercase().contains("granted"),
        "damage reduction explanation at level 6 must state it is absent, not granted: {}",
        damage_reduction.detail
    );
}

// ----- Still blocked: rage-state execution engine and generic diagnostics -----

#[test]
fn barbarian_level7_stays_blocked_on_rage_execution() {
    let input = load(BARBARIAN_LEVEL7_FIXTURE);
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
                "level-7 Barbarian must ground an honest not-raging record when no rage \
                 posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    assert_eq!(
        computation.base_attack_bonus, 7,
        "barbarian is now recognized by table_class_id; level 7 full BAB is +7"
    );
}

// ----- Positive control: level 8 was later widened into the supported tranche -----

#[test]
fn barbarian_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = BARBARIAN_LEVEL7_FIXTURE.replace("class:barbarian:7", "class:barbarian:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "level-8 Barbarian was later widened into the supported tranche and must now gain \
         bounded barbarian chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, BARBARIAN_UNCANNY_DODGE_ID),
        "level-8 Barbarian was later widened and must now carry the Uncanny Dodge explanation"
    );
    assert!(
        has_explanation(&computation, BARBARIAN_TRAP_SENSE_ID),
        "level-8 Barbarian was later widened and must now carry the Trap Sense explanation"
    );
    assert!(
        has_explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID),
        "level-8 Barbarian was later widened and must now carry the Improved Uncanny Dodge \
         explanation"
    );
    assert!(
        has_explanation(&computation, BARBARIAN_DAMAGE_REDUCTION_ID),
        "level-8 Barbarian was later widened and must now carry the Damage Reduction explanation"
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_barbarian_level7_recognition() {
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
fn multiclass_barbarian_level7_is_not_promoted_by_this_slice() {
    let multiclass = BARBARIAN_LEVEL7_FIXTURE.replace(
        "class_level=class:barbarian:7",
        "class_level=class:barbarian:7\nclass_level=class:fighter:1",
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

// ----- Barbarian level 1/level 2/level 3/level 4/level 5/level 6 stays unchanged -----

#[test]
fn barbarian_level6_truth_is_unchanged_by_the_level7_widening() {
    let input = load(BARBARIAN_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 6, "Barbarian level 6 full BAB must stay +6");

    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 17,
        "Barbarian level 6 rage rounds per day must stay 4 + Con modifier (+3) + 2 * 5 = 17"
    );

    let trap_sense = explanation(&computation, BARBARIAN_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 2, "Barbarian level 6 Trap Sense must stay +2");

    let improved_uncanny_dodge = explanation(&computation, BARBARIAN_IMPROVED_UNCANNY_DODGE_ID);
    assert_eq!(
        improved_uncanny_dodge.value, 0,
        "Barbarian level 6 Improved Uncanny Dodge must stay a grant-only value-0 record"
    );
}

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_barbarian_row_names_level_7_widening() {
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
            .contains("sd13_barbarian_level7_progression"),
        "barbarian row must cite the live SD13-E5 level-7 proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(
        note.contains("level 7") || note.contains("level-7"),
        "barbarian partial note must name the level-7 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("damage reduction"),
        "barbarian partial note must name the newly grounded Damage Reduction magnitude: {note}"
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
