//! SD13-E5 Druid level-8 progression grounding proof.
//!
//! Widens the accepted Druid level-1..level-7 prepared-divine-spell-burden
//! baseline (`tests/sd13_druid_level1_spell_baseline.rs`,
//! `tests/sd13_druid_base_attack_and_saves.rs`,
//! `tests/sd13_druid_level2_progression.rs`,
//! `tests/sd13_druid_level3_progression.rs`,
//! `tests/sd13_druid_level4_progression.rs`,
//! `tests/sd13_druid_level5_progression.rs`,
//! `tests/sd13_druid_level6_progression.rs`,
//! `tests/sd13_druid_level7_progression.rs`) to Druid level 8, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_druid_level` is generalized from `1..=7`
//! to `1..=8` via `MAX_SUPPORTED_DRUID_LEVEL = 8`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Druid class table) were read
//! directly before writing any code or test:
//!
//! - level 8 base attack bonus is +6 (`8 * 3 / 4 = 6`, the Druid's 3/4-BAB
//!   progression, genuinely risen from +5 at level 7 — the class table's own
//!   "+6/+1" iterative-attack notation is not modeled anywhere in this
//!   codebase, only the flat base value) and base saves are +6 Fortitude
//!   (good, `8 / 2 + 2 = 6`, genuinely risen from +5), +2 Reflex (poor,
//!   `8 / 3 = 2`, numerically unchanged from level 7, an integer-division
//!   coincidence), and +6 Will (good, `8 / 2 + 2 = 6`, genuinely risen from
//!   +5) — confirmed by the same formulas already grounded at levels 1-7,
//!   not re-derived.
//! - Wild Empathy genuinely rises to 9 (druid level 8 + Charisma modifier 1)
//!   via the same level-generic formula, not re-derived.
//! - Nature Sense stays the flat +2 bonus; Woodland Stride, Trackless Step,
//!   and Resist Nature's Lure (+4) all stay granted, not re-derived; the
//!   nature-bond choice recognition is not level-gated, so it still fires at
//!   level 8 for the same fixture selection.
//! - the PF1 Core Rulebook Druid class table's level-8 "Special" column reads
//!   "Wild shape (3/day)" (verified independently against both primary
//!   sources, checked rather than assumed away) — UNLIKE the blank level-7
//!   column. Per the level-4/level-6 precedent, the rule text bundles that
//!   frequency increase with a form-list expansion (a Huge or Diminutive
//!   animal, a Medium elemental, or a Small/Medium plant creature) and
//!   functioning-level upgrades (beast shape III / elemental body II / plant
//!   shape I), none of which exist in this codebase's engine-free record set
//!   and none of which are separable from the "3/day" numeral without
//!   misrepresenting the bundled feature as flat — so Wild Shape (including
//!   its level-8 frequency increase and form expansion) stays entirely
//!   named-but-unproven, and no explanation or diagnostic record is
//!   fabricated for it this slice either.
//!
//! It deliberately does not touch the animal-companion execution burden or
//! the prepared divine spell posture burden (both stay named-but-unproven,
//! unchanged from levels 1-7), and it does not ground Druid level 9+. It
//! also preserves the accepted Druid level-1..level-7 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level7_sd13_deterministic_input.txt");

const DRUID_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level8_sd13_deterministic_input.txt");

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

// ----- Base attack bonus at level 8 -----

#[test]
fn druid_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Druid level 8 3/4-BAB progression (8 * 3 / 4) must equal 6, genuinely risen from 5 \
         at level 7: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 (good Fortitude/Will, poor Reflex) -----

#[test]
fn druid_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Druid level 8 good Fortitude (8/2+2) must equal 6, genuinely risen from 5 at level 7"
    );

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 2, "Druid level 8 poor Reflex (8/3) must equal 2");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(
        will.value, 6,
        "Druid level 8 good Will (8/2+2) must equal 6, genuinely risen from 5 at level 7"
    );
}

// ----- Wild Empathy genuinely rises to 9 at level 8 -----

#[test]
fn druid_level8_wild_empathy_rises_to_nine() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 9,
        "Druid level 8 Wild Empathy (druid level 8 + Charisma modifier +1) must equal 9, \
         genuinely risen from 8 at level 7: {}",
        wild_empathy.detail
    );
}

// ----- Nature Sense stays the flat +2 at level 8 -----

#[test]
fn druid_level8_nature_sense_stays_flat_two() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(
        nature_sense.value, 2,
        "Druid Nature Sense must stay the flat +2 at level 8: {}",
        nature_sense.detail
    );
}

// ----- Woodland Stride / Trackless Step / Resist Nature's Lure still granted -----

#[test]
fn druid_level8_still_recognizes_the_granted_feature_records() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, DRUID_WOODLAND_STRIDE_ID);
    assert_eq!(
        woodland_stride.value, 0,
        "Woodland Stride recognition must carry no fabricated mechanical value at level 8"
    );

    let trackless_step = explanation(&computation, DRUID_TRACKLESS_STEP_ID);
    assert_eq!(
        trackless_step.value, 0,
        "Trackless Step recognition must carry no fabricated mechanical value at level 8"
    );

    let resist_natures_lure = explanation(&computation, DRUID_RESIST_NATURES_LURE_ID);
    assert_eq!(
        resist_natures_lure.value, 4,
        "Resist Nature's Lure must stay the flat +4 at level 8: {}",
        resist_natures_lure.detail
    );
}

// ----- Nature-bond choice recognition still fires at level 8 -----

#[test]
fn druid_level8_still_recognizes_the_nature_bond_choice() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.druid.nature_bond_choice");
    assert_eq!(
        choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("animal companion") || choice.detail.contains("Animal Companion"),
        "nature-bond recognition must still name the animal-companion selection at level 8: {}",
        choice.detail
    );
}

// ----- Wild Shape stays entirely named-but-unproven at level 8 -----

#[test]
fn druid_level8_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-8 Druid must not fabricate any wild-shape explanation record (the level-8 \
         'Wild shape (3/day)' entry bundles frequency with form expansion and is not flat): \
         {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-8 Druid must not fabricate any wild-shape diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 8 -----

#[test]
fn druid_level8_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "level-8 Druid must still claim-block on the animal-companion execution burden: {:?}",
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

// ----- The chassis recognition record is still present at level 8 -----

#[test]
fn druid_level8_still_recognizes_the_spell_bearing_baseline() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.druid"),
        "level-8 Druid must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-7 fixture is unaffected by this widening -----

#[test]
fn druid_level7_truth_is_unchanged_by_this_slice() {
    let input = load(DRUID_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 5, "Druid level 7 base attack bonus must stay 5");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 5, "Druid level 7 good Will save must stay 5");

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 8, "Druid level 7 Wild Empathy must stay 8");
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn druid_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = DRUID_LEVEL8_FIXTURE.replace("class:druid:8", "class:druid:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")),
        "level-9 Druid is now recognized by the later level-9 widening slice \
         (tests/sd13_druid_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level8_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id == "class_chassis.spell_baseline.druid"),
        "the Fighter chassis must not surface any druid-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Druid is not promoted -----

#[test]
fn multiclass_druid_level8_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL8_FIXTURE.replace(
        "class_level=class:druid:8",
        "class_level=class:druid:8\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id == "class_chassis.spell_baseline.druid"),
        "multiclass Druid must not gain any bounded druid chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_druid_row_names_level_8_widening() {
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
        druid.grounding_ref.contains("sd13_druid_level8_progression"),
        "druid row must cite the live SD13-E5 level-8 proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 8") || note.contains("level-8"),
        "druid partial note must name the level-8 widening: {note}"
    );
}
