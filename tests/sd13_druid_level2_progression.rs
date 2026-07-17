//! SD13-E5 Druid level-2 progression grounding proof.
//!
//! Widens the accepted Druid level-1 prepared divine spell-bearing baseline
//! (`tests/sd13_druid_level1_spell_baseline.rs`, `tests/sd13_druid_base_attack_and_saves.rs`)
//! to Druid level 2, mirroring the Fighter `supported_fighter_level` / Paladin
//! `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
//! `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
//! `supported_cleric_level` / Bard `supported_bard_level` level-range-gate idiom
//! (the level-1-only gate `is_single_class_druid_level1` is generalized to
//! `supported_druid_level`, an `Option<u8>` helper gated by
//! `MAX_SUPPORTED_DRUID_LEVEL = 2`). Both PF1 CRB primary sources (d20pfsrd and
//! legacy.aonprd.com Druid class table) were read directly before writing any code
//! or test:
//!
//! - level 2 base attack bonus is +1, base Fortitude/Will are +3 (good), base Reflex
//!   is +0 (poor) — confirmed by the same formulas already grounded at level 1
//!   (`classlevel * 3 / 4` and `classlevel/2+2` / `classlevel/3`), not re-derived.
//! - Wild Empathy's modifier (druid level + Charisma modifier) is level-generic by
//!   construction once the level-1-only gate is widened, so it grounds correctly at
//!   level 2 (2 + Cha modifier 1 = 3) via the same formula, not a new record.
//! - Nature Sense stays the flat, level-independent PF1 CRB +2 bonus, confirmed
//!   unchanged at level 2 via the same formula, not a new record.
//! - the nature-bond choice recognition is not level-gated; it still fires at level 2
//!   for the same fixture selection (`choice:druid_nature_bond -> bond:animal_companion`).
//! - the Druid class table's level-2 "Special" column reads "Woodland stride" (verified
//!   independently against both primary sources: "Starting at 2nd level, a druid may
//!   move through any sort of undergrowth ... at her normal speed and without taking
//!   damage or suffering any other impairment"). This is flat/identity-shaped — a
//!   binary recognition that the rule applies, no numeric formula — and is grounded as
//!   a bounded identity record (value 0) mirroring exactly how Rogue's/Monk's own
//!   `class_feature.rogue.evasion` / `class_feature.monk.evasion` were grounded: a
//!   level-gate-absence record below level 2, an identity/recognition record at or
//!   above it, with no terrain-detection engine and no movement-execution engine
//!   implemented.
//!
//! It deliberately does not touch the animal companion stat block/advancement/
//! link-share-spells burden or the prepared divine spell posture burden (both stay
//! named-but-unproven, unchanged from level 1), and it does not ground Druid level 3+.
//! It also preserves the accepted Druid level-1 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level1_sd13_deterministic_input.txt");

const DRUID_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level2_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

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

// ----- Base attack bonus at level 2 -----

#[test]
fn druid_level2_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 1,
        "Druid level 2 3/4-BAB progression (2 * 3 / 4) must equal 1: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 2 (good Fortitude/Will, poor Reflex) -----

#[test]
fn druid_level2_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Druid level 2 good Fortitude (2/2+2) must equal 3");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 0, "Druid level 2 poor Reflex (2/3) must equal 0");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 3, "Druid level 2 good Will (2/2+2) must equal 3");
}

// ----- Wild Empathy at level 2 -----

#[test]
fn druid_level2_wild_empathy_modifier_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture: Charisma 12 -> modifier +1. Druid level 2 + Cha modifier +1 = 3.
    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 3,
        "Druid level 2 wild empathy modifier must equal druid level + Cha modifier (2 + 1 = 3): {}",
        wild_empathy.detail
    );
}

// ----- Nature Sense at level 2 (flat, level-independent) -----

#[test]
fn druid_level2_nature_sense_bonus_is_unchanged() {
    let input = load(DRUID_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(
        nature_sense.value, 2,
        "Druid level 2 Nature Sense must stay the flat PF1 CRB +2 bonus: {}",
        nature_sense.detail
    );
}

// ----- Nature bond choice recognition at level 2 -----

#[test]
fn druid_level2_still_recognizes_nature_bond_choice() {
    let input = load(DRUID_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.druid.nature_bond_choice"),
        "level-2 Druid must still recognize the nature-bond choice: {:?}",
        computation.explanations
    );
}

// ----- Woodland Stride: new 2nd-level class feature, identity-shaped -----

#[test]
fn druid_level1_correctly_lacks_woodland_stride() {
    let input = load(DRUID_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, "class_feature.druid.woodland_stride");
    assert_eq!(
        woodland_stride.value, 0,
        "Woodland Stride absence record must carry no fabricated value: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.contains("absent") || woodland_stride.detail.contains("correctly"),
        "Woodland Stride detail at level 1 must state the correct level-gate absence: {}",
        woodland_stride.detail
    );
}

#[test]
fn druid_level2_grounds_woodland_stride_as_bounded_identity_record() {
    let input = load(DRUID_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, "class_feature.druid.woodland_stride");
    assert_eq!(
        woodland_stride.value, 0,
        "Woodland Stride is a bounded identity record only, carrying no fabricated mechanical \
         value: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.contains("undergrowth")
            || woodland_stride.detail.contains("normal speed"),
        "Woodland Stride detail must cite the PF1 CRB rule text: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.contains("no")
            && (woodland_stride.detail.contains("terrain") || woodland_stride.detail.contains("movement")),
        "Woodland Stride detail must disclaim any terrain-detection/movement-execution engine: {}",
        woodland_stride.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 2 -----

#[test]
fn druid_level2_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking),
        "level-2 Druid must still claim-block on the animal companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking),
        "level-2 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- The accepted Druid level-1 truth is unaffected -----

#[test]
fn druid_level1_truth_is_unchanged_by_this_widening() {
    let input = load(DRUID_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 0, "Druid level 1 base attack bonus must stay 0");

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 2, "Druid level 1 wild empathy modifier must stay 2");
}

// ----- Negative control: level 3 was later widened into the supported tranche -----

#[test]
fn druid_level_3_was_later_widened_into_the_supported_tranche() {
    // Level 3 is now recognized by a later SD13-E5 slice
    // (`tests/sd13_druid_level3_progression.rs`); this control now asserts the
    // widened truth rather than an absence, mirroring the Rogue/Barbarian/Monk/
    // Cleric/Bard precedent.
    let level_3 = DRUID_LEVEL2_FIXTURE.replace("class:druid:2", "class:druid:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.druid.base_attack_bonus"),
        "level-3 Druid is now recognized by a later slice and must gain the bounded druid \
         chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Druid level 4 was later widened into the supported tranche -----

#[test]
fn druid_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 stayed unrecognized. A later
    // SD13-E5 slice (tests/sd13_druid_level4_progression.rs) widened the
    // level-range gate to level 4; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent. The equivalent level-5 negative control now lives in the new
    // tests/sd13_druid_level4_progression.rs file where the coverage moved.
    let level_4 = DRUID_LEVEL2_FIXTURE.replace("class:druid:2", "class:druid:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.druid.base_attack_bonus"),
        "level-4 Druid is now recognized by a later slice and must gain the bounded druid \
         chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level2_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id == "class_chassis.spell_baseline.druid"
                || e.id == "class_feature.druid.woodland_stride"),
        "the Fighter chassis must not surface any druid-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Druid is not promoted -----

#[test]
fn multiclass_druid_level2_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL2_FIXTURE.replace(
        "class_level=class:druid:2",
        "class_level=class:druid:2\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id == "class_chassis.spell_baseline.druid"
                || e.id == "class_feature.druid.woodland_stride"),
        "multiclass Druid must not gain any bounded druid chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-2 widening -----

#[test]
fn matrix_druid_row_names_level_2_widening() {
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
        druid.grounding_ref.contains("sd13_druid_level2_progression"),
        "druid row must cite the live SD13-E5 level-2 proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 2") || note.contains("level-2"),
        "druid partial note must name the level-2 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("woodland stride"),
        "druid partial note must name the newly-grounded Woodland Stride identity record: {note}"
    );
}
