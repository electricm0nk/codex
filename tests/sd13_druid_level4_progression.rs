//! SD13-E5 Druid level-4 progression grounding proof.
//!
//! Widens the accepted Druid level-1/level-2/level-3 prepared divine
//! spell-bearing baseline (`tests/sd13_druid_level1_spell_baseline.rs`,
//! `tests/sd13_druid_base_attack_and_saves.rs`,
//! `tests/sd13_druid_level2_progression.rs`,
//! `tests/sd13_druid_level3_progression.rs`) to druid level 4, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_druid_level` is generalized from
//! `1..=3` to `1..=4` via `MAX_SUPPORTED_DRUID_LEVEL = 4`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Druid class table) were
//! read directly before writing any code or test: level 4 base attack bonus
//! is +3, base Fortitude/Will are +4 (good), base Reflex is +1 (poor), and
//! the level-4 "Special" column reads "Resist nature's lure, wild shape
//! (1/day)" — TWO distinct entries. It proves:
//!
//! - base attack bonus at level 4 is grounded by the same 3/4-BAB formula
//!   (`level * 3 / 4`) already grounded at levels 1-3: `4 * 3 / 4 = 3`.
//! - base saves at level 4 are grounded by the same formulas already
//!   grounded at levels 1-3 (`level / 2 + 2` for good Fortitude/Will,
//!   `level / 3` for poor Reflex): Fortitude/Will = 4, Reflex = 1.
//! - Wild Empathy's modifier is level-generic by construction and grounds
//!   correctly to 5 (4 + Charisma modifier 1) at level 4, via the same
//!   formula, not a new record.
//! - Nature Sense stays the flat, level-independent PF1 CRB +2 bonus,
//!   confirmed unchanged at level 4 via the same formula, not a new record.
//! - the nature-bond choice recognition is not level-gated; it still fires
//!   at level 4 for the same fixture selection
//!   (`choice:druid_nature_bond -> bond:animal_companion`).
//! - Woodland Stride (granted starting at level 2) and Trackless Step
//!   (granted starting at level 3) both stay granted at level 4, not
//!   re-derived, grounded as the same bounded identity/recognition records
//!   already grounded at levels 2/3.
//! - Resist Nature's Lure, one of the two distinct entries in the PF1 Core
//!   Rulebook's level-4 "Special" column (verified independently against
//!   d20pfsrd and legacy.aonprd.com: "a druid gains a +4 bonus on saving
//!   throws against the spell-like and supernatural abilities of fey. This
//!   bonus also applies to spells and effects that utilize or target
//!   plants, such as blight, entangle, spike growth, and warp wood."), is
//!   flat/identity-shaped — a standalone +4 magnitude, never applied to any
//!   actual save total — and is grounded as a bounded flat-magnitude
//!   identity record mirroring exactly how Bravery/Divine Grace/Trap Sense
//!   were grounded: a level-gate-absence record below level 4, a
//!   flat-magnitude record at or above it, with no saving-throw resolution
//!   engine implemented.
//! - the OTHER level-4 "Special" entry, Wild Shape (1/day), was checked and
//!   confirmed NOT flat (a full shapeshifting subsystem — new form, new
//!   stat block, duration tracking — with no execution engine anywhere in
//!   this codebase), so it is deliberately left named-but-unproven, exactly
//!   like the animal-companion execution burden: no explanation record and
//!   no diagnostic for it is fabricated by this slice.
//!
//! It deliberately does not touch the animal companion stat block/
//! advancement/link-share-spells burden, the Wild Shape execution burden, or
//! the prepared divine spell posture burden (all stay named-but-unproven,
//! unchanged from level 1), and it does not ground Druid level 5+. It also
//! preserves the accepted Druid level-1/level-2/level-3 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level3_sd13_deterministic_input.txt");

const DRUID_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level4_sd13_deterministic_input.txt");

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

// ----- Base attack bonus at level 4 -----

#[test]
fn druid_level4_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Druid level 4 3/4-BAB progression (4 * 3 / 4) must equal 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 4 (good Fortitude/Will, poor Reflex) -----

#[test]
fn druid_level4_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 4, "Druid level 4 good Fortitude (4/2+2) must equal 4");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 1, "Druid level 4 poor Reflex (4/3) must equal 1");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 4, "Druid level 4 good Will (4/2+2) must equal 4");
}

// ----- Wild Empathy at level 4 -----

#[test]
fn druid_level4_wild_empathy_modifier_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture: Charisma 12 -> modifier +1. Druid level 4 + Cha modifier +1 = 5.
    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 5,
        "Druid level 4 wild empathy modifier must equal druid level + Cha modifier (4 + 1 = 5): {}",
        wild_empathy.detail
    );
}

// ----- Nature Sense at level 4 (flat, level-independent) -----

#[test]
fn druid_level4_nature_sense_bonus_is_unchanged() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(
        nature_sense.value, 2,
        "Druid level 4 Nature Sense must stay the flat PF1 CRB +2 bonus: {}",
        nature_sense.detail
    );
}

// ----- Nature bond choice recognition at level 4 -----

#[test]
fn druid_level4_still_recognizes_nature_bond_choice() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.druid.nature_bond_choice"),
        "level-4 Druid must still recognize the nature-bond choice: {:?}",
        computation.explanations
    );
}

// ----- Woodland Stride / Trackless Step stay granted at level 4, not re-derived -----

#[test]
fn druid_level4_keeps_woodland_stride_and_trackless_step_grounded() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, DRUID_WOODLAND_STRIDE_ID);
    assert_eq!(
        woodland_stride.value, 0,
        "Woodland Stride must carry no fabricated mechanical value at level 4: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.contains("granted"),
        "Woodland Stride detail at level 4 must state it is granted, not absent: {}",
        woodland_stride.detail
    );

    let trackless_step = explanation(&computation, DRUID_TRACKLESS_STEP_ID);
    assert_eq!(
        trackless_step.value, 0,
        "Trackless Step must carry no fabricated mechanical value at level 4: {}",
        trackless_step.detail
    );
    assert!(
        trackless_step.detail.contains("granted"),
        "Trackless Step detail at level 4 must state it is granted, not absent: {}",
        trackless_step.detail
    );
}

// ----- Resist Nature's Lure: new 4th-level class feature, flat-magnitude-shaped -----

#[test]
fn druid_level3_correctly_lacks_resist_natures_lure() {
    let input = load(DRUID_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let resist_natures_lure = explanation(&computation, DRUID_RESIST_NATURES_LURE_ID);
    assert_eq!(
        resist_natures_lure.value, 0,
        "Resist Nature's Lure absence record must carry no fabricated value: {}",
        resist_natures_lure.detail
    );
    assert!(
        resist_natures_lure.detail.to_lowercase().contains("absent"),
        "Resist Nature's Lure detail at level 3 must state the correct level-gate absence: {}",
        resist_natures_lure.detail
    );
}

#[test]
fn druid_level4_grounds_resist_natures_lure_as_bounded_flat_magnitude_record() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let resist_natures_lure = explanation(&computation, DRUID_RESIST_NATURES_LURE_ID);
    assert_eq!(
        resist_natures_lure.value, 4,
        "Resist Nature's Lure must ground the PF1 CRB flat +4 magnitude: {}",
        resist_natures_lure.detail
    );
    assert!(
        resist_natures_lure.detail.contains("fey"),
        "Resist Nature's Lure detail must cite the PF1 CRB rule text (fey): {}",
        resist_natures_lure.detail
    );
    assert!(
        resist_natures_lure.detail.to_lowercase().contains("granted"),
        "Resist Nature's Lure detail at level 4 must state it is granted, not absent: {}",
        resist_natures_lure.detail
    );
    assert!(
        resist_natures_lure.detail.to_lowercase().contains("no")
            && resist_natures_lure.detail.to_lowercase().contains("engine"),
        "Resist Nature's Lure detail must disclaim any saving-throw-resolution engine: {}",
        resist_natures_lure.detail
    );
}

// ----- Wild Shape (the OTHER level-4 "Special" entry) must not be fabricated -----

#[test]
fn druid_level4_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-4 Druid must not fabricate any Wild Shape explanation record (checked and \
         confirmed not flat — a full shapeshifting subsystem): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-4 Druid must not fabricate any Wild Shape diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 4 -----

#[test]
fn druid_level4_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking),
        "level-4 Druid must still claim-block on the animal companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking),
        "level-4 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- The accepted Druid level-3 truth is unaffected -----

#[test]
fn druid_level3_truth_is_unchanged_by_this_widening() {
    let input = load(DRUID_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 2, "Druid level 3 base attack bonus must stay 2");

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 4, "Druid level 3 wild empathy modifier must stay 4");
}

// ----- Druid level 5 was later widened into the supported tranche -----

#[test]
fn druid_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 stayed unrecognized. A later
    // SD13-E5 slice (tests/sd13_druid_level5_progression.rs) widened the
    // level-range gate to level 5 and extended the base-attack/base-save/Wild
    // Empathy/Nature Sense formulas, kept Woodland Stride/Trackless Step/Resist
    // Nature's Lure granted, and confirmed the level-5 "Special" column is
    // genuinely blank; this negative control is superseded, not violated — pin
    // the new truth here too so this file stays internally consistent. The
    // equivalent level-6 negative control now lives in the new
    // tests/sd13_druid_level5_progression.rs file where the coverage moved.
    let level_5 = DRUID_LEVEL4_FIXTURE.replace("class:druid:4", "class:druid:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.druid.base_attack_bonus"),
        "level-5 Druid is now recognized by a later slice and must gain the bounded druid \
         chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == DRUID_RESIST_NATURES_LURE_ID),
        "level-5 Druid is now recognized by a later slice and must keep Resist Nature's \
         Lure grounded: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level4_recognition() {
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
fn multiclass_druid_level4_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL4_FIXTURE.replace(
        "class_level=class:druid:4",
        "class_level=class:druid:4\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-4 widening and Resist Nature's Lure -----

#[test]
fn matrix_druid_row_names_level_4_widening_and_resist_natures_lure() {
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
        druid.grounding_ref.contains("sd13_druid_level3_progression")
            || druid.grounding_ref.contains("sd13_druid_level4_progression"),
        "druid row must cite a live SD13-E5 proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 4") || note.contains("level-4"),
        "druid partial note must name the level-4 widening: {note}"
    );
    assert!(
        note.to_lowercase().contains("resist nature's lure")
            || note.to_lowercase().contains("resist nature’s lure"),
        "druid partial note must name the newly-grounded Resist Nature's Lure identity record: {note}"
    );
}
