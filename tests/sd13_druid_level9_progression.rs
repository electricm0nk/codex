//! SD13-E5 Druid level-9 progression grounding proof.
//!
//! Widens the accepted Druid level-1..level-8 prepared-divine baseline (most
//! recently `tests/sd13_druid_level8_progression.rs`) to Druid level 9,
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_druid_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_DRUID_LEVEL = 9`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Druid class table) were read directly before
//! writing any code or test:
//!
//! - level 9 base attack bonus is +6 (`9 * 3 / 4 = 6`, the Druid's 3/4-BAB
//!   progression, numerically unchanged from level 8 — an integer-division
//!   coincidence; the table's own "+6/+1" iterative notation is not modeled
//!   anywhere in this codebase, only the flat base value) and base saves
//!   are +6 Fortitude and +6 Will (both good, `9 / 2 + 2 = 6`, numerically
//!   unchanged from level 8, coincidences) and +3 Reflex (poor,
//!   `9 / 3 = 3`, genuinely risen from +2) — confirmed by the same formulas
//!   already grounded at levels 1-8, not re-derived.
//! - Wild Empathy genuinely rises to 10 (druid level 9 + Charisma modifier
//!   1) via the same level-generic formula, not re-derived.
//! - Nature Sense stays the flat +2; Woodland Stride, Trackless Step, and
//!   Resist Nature's Lure (+4) all stay granted, not re-derived; the
//!   nature-bond choice recognition is not level-gated and still fires.
//! - the PF1 Core Rulebook Druid class table's level-9 "Special" column
//!   reads "Venom immunity" (verified independently against both primary
//!   sources, checked rather than assumed away) — a genuinely NEW class
//!   feature at 9th level ("At 9th level, a druid gains immunity to all
//!   poisons"), and a genuinely flat/identity-shaped, no-choice,
//!   no-magnitude grant — exactly like Monk's Purity of Body (immunity to
//!   disease). This slice grounds it as a +0 identity/recognition record
//!   ONLY (`class_feature.druid.venom_immunity`): no poison-application or
//!   condition-resolution engine exists in this codebase, so no immunity
//!   effect is fabricated from the record.
//! - Wild Shape's uses stay 3/day at level 9 (the next rise lands at 10th,
//!   checked rather than assumed) and Wild Shape stays entirely
//!   named-but-unproven per the level-4/6/8 precedent — a dedicated
//!   negative test pins that no wild-shape record or diagnostic is
//!   fabricated.
//!
//! It deliberately does not touch the animal-companion execution burden or
//! the prepared divine spell posture burden (both stay named-but-unproven,
//! unchanged from levels 1-8), and it does not ground Druid level 10+. It
//! also preserves the accepted Druid level-1..level-8 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level8_sd13_deterministic_input.txt");

const DRUID_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level9_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DRUID_WOODLAND_STRIDE_ID: &str = "class_feature.druid.woodland_stride";
const DRUID_TRACKLESS_STEP_ID: &str = "class_feature.druid.trackless_step";
const DRUID_RESIST_NATURES_LURE_ID: &str = "class_feature.druid.resist_natures_lure";
const DRUID_VENOM_IMMUNITY_ID: &str = "class_feature.druid.venom_immunity";

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

// ----- Base attack bonus and saves at level 9 -----

#[test]
fn druid_level9_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Druid level 9 3/4-BAB progression (9 * 3 / 4) must equal 6 — unchanged from level 8, \
         an integer-division coincidence: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 6, "Druid level 9 good Fortitude (9/2+2) must equal 6");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(
        reflex.value, 3,
        "Druid level 9 poor Reflex (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 6, "Druid level 9 good Will (9/2+2) must equal 6");
}

// ----- Wild Empathy genuinely rises to 10 at level 9 -----

#[test]
fn druid_level9_wild_empathy_rises_to_ten() {
    let input = load(DRUID_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 10,
        "Druid level 9 Wild Empathy (druid level 9 + Charisma modifier +1) must equal 10, \
         genuinely risen from 9 at level 8: {}",
        wild_empathy.detail
    );
}

// ----- Venom Immunity is newly grounded as a +0 identity record at level 9 -----

#[test]
fn druid_level9_grounds_venom_immunity_as_identity_recognition_only() {
    let input = load(DRUID_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let venom_immunity = explanation(&computation, DRUID_VENOM_IMMUNITY_ID);
    assert_eq!(
        venom_immunity.value, 0,
        "Venom Immunity must be grounded as a +0 identity/recognition record only — no \
         poison-application or condition-resolution engine exists in this codebase: {}",
        venom_immunity.detail
    );
    assert!(
        venom_immunity.detail.contains("immunity to all poisons"),
        "Venom Immunity's record must carry the rule's own immunity-to-all-poisons identity: \
         {}",
        venom_immunity.detail
    );
}

// ----- Nature Sense and granted features carry over at level 9 -----

#[test]
fn druid_level9_remaining_pillars_carry_over_unchanged() {
    let input = load(DRUID_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(nature_sense.value, 2, "Nature Sense must stay the flat +2 at level 9");

    for (id, expected) in [
        (DRUID_WOODLAND_STRIDE_ID, 0),
        (DRUID_TRACKLESS_STEP_ID, 0),
        (DRUID_RESIST_NATURES_LURE_ID, 4),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "'{id}' must carry over unchanged at level 9: {}",
            record.detail
        );
    }

    let choice = explanation(&computation, "class_chassis.druid.nature_bond_choice");
    assert_eq!(
        choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value at level 9"
    );
}

// ----- Wild Shape stays entirely named-but-unproven at level 9 -----

#[test]
fn druid_level9_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-9 Druid must not fabricate any wild-shape explanation record (uses stay 3/day \
         at level 9, the next rise landing at 10th): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-9 Druid must not fabricate any wild-shape diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 9 -----

#[test]
fn druid_level9_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "level-9 Druid must still claim-block on the animal-companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking
        ),
        "level-9 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn druid_level8_truth_is_unchanged_by_this_slice() {
    let input = load(DRUID_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 9, "Druid level 8 Wild Empathy must stay 9");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 2, "Druid level 8 poor Reflex must stay 2");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == DRUID_VENOM_IMMUNITY_ID),
        "level-8 Druid must NOT gain the Venom Immunity record — it is a 9th-level feature: \
         {:?}",
        computation.explanations
    );
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn druid_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = DRUID_LEVEL9_FIXTURE.replace("class:druid:9", "class:druid:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")),
        "level-10 Druid is now recognized by the later level-10 widening slice \
         (tests/sd13_druid_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level9_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id.starts_with("class_feature.druid.")),
        "the Fighter chassis must not surface any druid-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Druid is not promoted -----

#[test]
fn multiclass_druid_level9_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL9_FIXTURE.replace(
        "class_level=class:druid:9",
        "class_level=class:druid:9\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id.starts_with("class_feature.druid.")),
        "multiclass Druid must not gain any bounded druid explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_druid_row_names_level_9_widening() {
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
        druid.grounding_ref.contains("sd13_druid_level9_progression"),
        "druid row must cite the live SD13-E5 level-9 proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "druid partial note must name the level-9 widening: {note}"
    );
}
