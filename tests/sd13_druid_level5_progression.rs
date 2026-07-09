//! SD13-E5 Druid level-5 progression grounding proof.
//!
//! Widens the accepted Druid level-1/level-2/level-3/level-4 prepared divine
//! spell-bearing baseline (`tests/sd13_druid_level1_spell_baseline.rs`,
//! `tests/sd13_druid_base_attack_and_saves.rs`,
//! `tests/sd13_druid_level2_progression.rs`,
//! `tests/sd13_druid_level3_progression.rs`,
//! `tests/sd13_druid_level4_progression.rs`) to druid level 5, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_druid_level` is generalized from
//! `1..=4` to `1..=5` via `MAX_SUPPORTED_DRUID_LEVEL = 5`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Druid class table) were
//! read directly before writing any code or test: level 5 base attack bonus
//! is +3, base Fortitude/Will are +4 (good), base Reflex is +1 (poor) — all
//! three numerically unchanged from level 4 (`5 * 3 / 4 = 3`, integer
//! division; `5 / 2 + 2 = 4`; `5 / 3 = 1`), and the level-5 "Special" column
//! is genuinely blank — checked independently rather than assumed, since the
//! level-4 row's "Resist nature's lure, wild shape (1/day)" entry could have
//! continued a pattern. It proves:
//!
//! - base attack bonus at level 5 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-4: `5 * 3 / 4 = 3`, an
//!   integer-division coincidence with level 4's value, not a sign the
//!   formula stopped scaling.
//! - base saves at level 5 are grounded by the same formulas already
//!   grounded at levels 1-4 (`level / 2 + 2` for good Fortitude/Will,
//!   `level / 3` for poor Reflex): Fortitude/Will = 4, Reflex = 1, both
//!   integer-division coincidences with level 4's values.
//! - Wild Empathy's modifier is level-generic by construction and grounds
//!   correctly to 6 (5 + Charisma modifier 1) at level 5, via the same
//!   formula, not a new record.
//! - Nature Sense stays the flat, level-independent PF1 CRB +2 bonus,
//!   confirmed unchanged at level 5 via the same formula, not a new record.
//! - the nature-bond choice recognition is not level-gated; it still fires
//!   at level 5 for the same fixture selection
//!   (`choice:druid_nature_bond -> bond:animal_companion`).
//! - Woodland Stride (granted starting at level 2), Trackless Step (granted
//!   starting at level 3), and Resist Nature's Lure (granted starting at
//!   level 4) all stay granted at level 5, not re-derived, grounded as the
//!   same bounded identity/flat-magnitude records already grounded at
//!   levels 2/3/4.
//! - the PF1 CRB Druid class table's level-5 "Special" column is genuinely
//!   blank (verified independently against both primary sources), so this
//!   slice grounds no new pillar at level 5 — only the existing pillars are
//!   widened, mirroring the Sorcerer-level-4 / Wizard-level-2 / Cleric-
//!   level-2 "blank Special column" idiom exactly.
//!
//! It deliberately does not touch the animal companion stat block/
//! advancement/link-share-spells burden, the Wild Shape execution burden
//! (still 1/day, unchanged — Wild Shape usage next increases at druid level
//! 6, out of scope here), or the prepared divine spell posture burden (all
//! stay named-but-unproven, unchanged from level 1), and it does not ground
//! Druid level 6+. It also preserves the accepted Druid
//! level-1/level-2/level-3/level-4 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level4_sd13_deterministic_input.txt");

const DRUID_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level5_sd13_deterministic_input.txt");

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

// ----- Base attack bonus at level 5 -----

#[test]
fn druid_level5_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Druid level 5 3/4-BAB progression (5 * 3 / 4) must equal 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 5 (good Fortitude/Will, poor Reflex) -----

#[test]
fn druid_level5_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 4, "Druid level 5 good Fortitude (5/2+2) must equal 4");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 1, "Druid level 5 poor Reflex (5/3) must equal 1");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 4, "Druid level 5 good Will (5/2+2) must equal 4");
}

// ----- Wild Empathy at level 5 -----

#[test]
fn druid_level5_wild_empathy_modifier_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture: Charisma 12 -> modifier +1. Druid level 5 + Cha modifier +1 = 6.
    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 6,
        "Druid level 5 wild empathy modifier must equal druid level + Cha modifier (5 + 1 = 6): {}",
        wild_empathy.detail
    );
}

// ----- Nature Sense at level 5 (flat, level-independent) -----

#[test]
fn druid_level5_nature_sense_bonus_is_unchanged() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(
        nature_sense.value, 2,
        "Druid level 5 Nature Sense must stay the flat PF1 CRB +2 bonus: {}",
        nature_sense.detail
    );
}

// ----- Nature bond choice recognition at level 5 -----

#[test]
fn druid_level5_still_recognizes_nature_bond_choice() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.druid.nature_bond_choice"),
        "level-5 Druid must still recognize the nature-bond choice: {:?}",
        computation.explanations
    );
}

// ----- Woodland Stride / Trackless Step / Resist Nature's Lure stay granted at level 5 -----

#[test]
fn druid_level5_keeps_woodland_stride_trackless_step_and_resist_natures_lure_grounded() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, DRUID_WOODLAND_STRIDE_ID);
    assert_eq!(
        woodland_stride.value, 0,
        "Woodland Stride must carry no fabricated mechanical value at level 5: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.contains("granted"),
        "Woodland Stride detail at level 5 must state it is granted, not absent: {}",
        woodland_stride.detail
    );

    let trackless_step = explanation(&computation, DRUID_TRACKLESS_STEP_ID);
    assert_eq!(
        trackless_step.value, 0,
        "Trackless Step must carry no fabricated mechanical value at level 5: {}",
        trackless_step.detail
    );
    assert!(
        trackless_step.detail.contains("granted"),
        "Trackless Step detail at level 5 must state it is granted, not absent: {}",
        trackless_step.detail
    );

    let resist_natures_lure = explanation(&computation, DRUID_RESIST_NATURES_LURE_ID);
    assert_eq!(
        resist_natures_lure.value, 4,
        "Resist Nature's Lure must stay the flat PF1 CRB +4 magnitude at level 5, not \
         re-derived: {}",
        resist_natures_lure.detail
    );
    assert!(
        resist_natures_lure.detail.to_lowercase().contains("granted"),
        "Resist Nature's Lure detail at level 5 must state it is granted, not absent: {}",
        resist_natures_lure.detail
    );
}

// ----- Wild Shape must not be fabricated at level 5 -----

#[test]
fn druid_level5_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-5 Druid must not fabricate any Wild Shape explanation record: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-5 Druid must not fabricate any Wild Shape diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 5 -----

#[test]
fn druid_level5_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking),
        "level-5 Druid must still claim-block on the animal companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking),
        "level-5 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- The accepted Druid level-4 truth is unaffected -----

#[test]
fn druid_level4_truth_is_unchanged_by_this_widening() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Druid level 4 base attack bonus must stay 3");

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 5, "Druid level 4 wild empathy modifier must stay 5");
}

// ----- Negative control: level 6 stays unrecognized by this slice -----

#[test]
fn druid_level_6_is_not_promoted_by_this_slice() {
    let level_6 = DRUID_LEVEL5_FIXTURE.replace("class:druid:5", "class:druid:6");
    let input = load(&level_6);
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
        "level-6 Druid must not gain any bounded druid chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level5_recognition() {
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
fn multiclass_druid_level5_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL5_FIXTURE.replace(
        "class_level=class:druid:5",
        "class_level=class:druid:5\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-5 widening -----

#[test]
fn matrix_druid_row_names_level_5_widening() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid progression_and_spell_burden row must exist");

    assert_eq!(druid.support_state, SupportState::Partial);
    assert_eq!(druid.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        druid.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        druid.grounding_ref.contains("sd13_druid_level5_progression"),
        "druid row must cite the live SD13-E5 level-5 proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 5") || note.contains("level-5"),
        "druid partial note must name the level-5 widening: {note}"
    );
}
