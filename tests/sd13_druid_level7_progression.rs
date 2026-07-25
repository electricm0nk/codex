//! SD13-E5 Druid level-7 progression grounding proof.
//!
//! Widens the accepted Druid level-1/level-2/level-3/level-4/level-5/level-6
//! prepared divine spell-bearing baseline
//! (`tests/sd13_druid_level1_spell_baseline.rs`,
//! `tests/sd13_druid_base_attack_and_saves.rs`, `tests/sd13_druid_level2_progression.rs`,
//! `tests/sd13_druid_level3_progression.rs`, `tests/sd13_druid_level4_progression.rs`,
//! `tests/sd13_druid_level5_progression.rs`, `tests/sd13_druid_level6_progression.rs`)
//! to druid level 7, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_druid_level` is generalized from `1..=6`
//! to `1..=7` via `MAX_SUPPORTED_DRUID_LEVEL = 7`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Druid class table) were read directly before
//! writing any code or test:
//!
//! - level 7 base attack bonus is +5 (`7 * 3 / 4 = 5`, the Druid's own 3/4-BAB
//!   progression, the same shape as Rogue/Monk/Cleric/Bard), a genuinely new
//!   value, up from +4 at level 6.
//! - base saves are +5 Fortitude (good, `7/2+2 = 5`), +2 Reflex (poor,
//!   `7/3 = 2`), +5 Will (good, `7/2+2 = 5`) — all three numerically
//!   unchanged from level 6 (integer-division coincidences of the same
//!   formulas, not a sign any formula stopped scaling), extended via the same
//!   formulas, not re-derived.
//! - Wild Empathy's modifier is level-generic by construction and grounds
//!   correctly to 8 (7 + Charisma modifier 1) at level 7, via the same
//!   formula, not a new record.
//! - Nature Sense stays the flat, level-independent PF1 CRB +2 bonus,
//!   confirmed unchanged at level 7 via the same formula, not a new record.
//! - the nature-bond choice recognition is not level-gated; it still fires at
//!   level 7 for the same fixture selection
//!   (`choice:druid_nature_bond -> bond:animal_companion`).
//! - Woodland Stride (granted starting at level 2), Trackless Step (granted
//!   starting at level 3), and Resist Nature's Lure (granted starting at
//!   level 4) all stay granted at level 7, not re-derived, grounded as the
//!   same bounded identity/flat-magnitude records already grounded at levels
//!   2/3/4/5/6.
//! - the PF1 CRB Druid class table's level-7 "Special" column is genuinely
//!   blank (verified independently against both primary sources rather than
//!   assumed): the druid's Wild Shape usage-count next increases at 8th
//!   level ("Wild shape (3/day)"), not 7th, so this slice makes no Wild
//!   Shape claim at level 7 either way — it stays exactly as unproven as at
//!   every earlier level.
//!
//! It deliberately does not touch the animal companion stat block/
//! advancement/link-share-spells burden, the Wild Shape execution burden, or
//! the prepared divine spell posture burden (all stay named-but-unproven,
//! unchanged from level 1), and it does not ground Druid level 8+. It also
//! preserves the accepted Druid level-1 through level-6 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level6_sd13_deterministic_input.txt");

const DRUID_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level7_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DRUID_WOODLAND_STRIDE_ID: &str = "class_feature.druid.woodland_stride";
const DRUID_TRACKLESS_STEP_ID: &str = "class_feature.druid.trackless_step";
const DRUID_RESIST_NATURES_LURE_ID: &str = "class_feature.druid.resist_natures_lure";

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Base attack bonus at level 7 -----

#[test]
fn druid_level7_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Druid level 7 3/4-BAB progression (7 * 3 / 4) must equal 5: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 (good Fortitude/Will, poor Reflex) -----

#[test]
fn druid_level7_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 5, "Druid level 7 good Fortitude (7/2+2) must equal 5");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 2, "Druid level 7 poor Reflex (7/3) must equal 2");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 5, "Druid level 7 good Will (7/2+2) must equal 5");
}

// ----- Wild Empathy at level 7 -----

#[test]
fn druid_level7_wild_empathy_modifier_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture: Charisma 12 -> modifier +1. Druid level 7 + Cha modifier +1 = 8.
    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 8,
        "Druid level 7 wild empathy modifier must equal druid level + Cha modifier (7 + 1 = 8): {}",
        wild_empathy.detail
    );
}

// ----- Nature Sense at level 7 (flat, level-independent) -----

#[test]
fn druid_level7_nature_sense_bonus_is_unchanged() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(
        nature_sense.value, 2,
        "Druid level 7 Nature Sense must stay the flat PF1 CRB +2 bonus: {}",
        nature_sense.detail
    );
}

// ----- Nature bond choice recognition at level 7 -----

#[test]
fn druid_level7_still_recognizes_nature_bond_choice() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.druid.nature_bond_choice"),
        "level-7 Druid must still recognize the nature-bond choice: {:?}",
        computation.explanations
    );
}

// ----- Woodland Stride / Trackless Step / Resist Nature's Lure stay granted at level 7 -----

#[test]
fn druid_level7_keeps_woodland_stride_trackless_step_and_resist_natures_lure_grounded() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, DRUID_WOODLAND_STRIDE_ID);
    assert_eq!(
        woodland_stride.value, 0,
        "Woodland Stride must carry no fabricated mechanical value at level 7: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.contains("granted"),
        "Woodland Stride detail at level 7 must state it is granted, not absent: {}",
        woodland_stride.detail
    );

    let trackless_step = explanation(&computation, DRUID_TRACKLESS_STEP_ID);
    assert_eq!(
        trackless_step.value, 0,
        "Trackless Step must carry no fabricated mechanical value at level 7: {}",
        trackless_step.detail
    );
    assert!(
        trackless_step.detail.contains("granted"),
        "Trackless Step detail at level 7 must state it is granted, not absent: {}",
        trackless_step.detail
    );

    let resist_natures_lure = explanation(&computation, DRUID_RESIST_NATURES_LURE_ID);
    assert_eq!(
        resist_natures_lure.value, 4,
        "Resist Nature's Lure must stay the flat PF1 CRB +4 magnitude at level 7, not \
         re-derived: {}",
        resist_natures_lure.detail
    );
    assert!(
        resist_natures_lure.detail.to_lowercase().contains("granted"),
        "Resist Nature's Lure detail at level 7 must state it is granted, not absent: {}",
        resist_natures_lure.detail
    );
}

// ----- Wild Shape must not be fabricated at level 7 -----

#[test]
fn druid_level7_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-7 Druid must not fabricate any Wild Shape explanation record: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-7 Druid must not fabricate any Wild Shape diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 7 -----

#[test]
fn druid_level7_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking),
        "level-7 Druid must still claim-block on the animal companion execution burden: {:?}",
        computation.diagnostics
    );
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.druid.prepared_divine.unsupported")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let prepared_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.druid.daily_preparation")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                prepared_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- The chassis recognition record is still present at level 7 -----

#[test]
fn druid_level7_still_recognizes_the_spell_bearing_baseline() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.druid"),
        "level-7 Druid must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the accepted Druid level-6 truth is unaffected -----

#[test]
fn druid_level6_truth_is_unchanged_by_this_widening() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 4, "Druid level 6 base attack bonus must stay 4");

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 7, "Druid level 6 wild empathy modifier must stay 7");
}

// ----- Level 8 was later widened into the supported tranche by a further slice -----

#[test]
fn druid_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = DRUID_LEVEL7_FIXTURE.replace("class:druid:7", "class:druid:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")),
        "level-8 Druid is now recognized by the later level-8 widening slice \
         (tests/sd13_druid_level8_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level7_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id == "class_chassis.spell_baseline.druid"
                || e.id == DRUID_WOODLAND_STRIDE_ID
                || e.id == DRUID_TRACKLESS_STEP_ID
                || e.id == DRUID_RESIST_NATURES_LURE_ID),
        "the Fighter chassis must not surface any druid-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Druid is not promoted -----

#[test]
fn multiclass_druid_level7_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL7_FIXTURE.replace(
        "class_level=class:druid:7",
        "class_level=class:druid:7\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id == "class_chassis.spell_baseline.druid"
                || e.id == DRUID_WOODLAND_STRIDE_ID
                || e.id == DRUID_TRACKLESS_STEP_ID
                || e.id == DRUID_RESIST_NATURES_LURE_ID),
        "multiclass Druid must not gain any bounded druid chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_druid_row_names_level_7_widening() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid progression_and_spell_burden row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(druid.support_state, SupportState::Supported);
    assert_eq!(druid.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        druid.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        druid.grounding_ref.contains("sd13_druid_level7_progression"),
        "druid row must cite the live SD13-E5 level-7 proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 7") || note.contains("level-7"),
        "druid partial note must name the level-7 widening: {note}"
    );
}
