//! SD18 Druid level-14 widening grounding proof.
//!
//! Widens the accepted Druid level-1..level-13 prepared divine spell-bearing
//! baseline (`tests/sd18_druid_level13_widening.rs`, the loop's most recent
//! Druid ceiling) to Druid level 14 — mirroring the sibling-class
//! level-range-gate idiom (`supported_druid_level` is generalized from
//! `1..=13` to `1..=14` via `MAX_SUPPORTED_DRUID_LEVEL = 14`, exactly as
//! `cycle-2026-07-15T2300` widened `MAX_SUPPORTED_CLERIC_LEVEL` from 13 to
//! 14, the loop's sixth §3.2 level-14 landing after Barbarian, Fighter,
//! Rogue, Ranger, and Bard). Both primary sources (d20pfsrd and Archives of
//! Nethys aonprd.com) were read directly before writing any code or test and
//! agree byte-for-byte:
//!
//! - level 14 base attack bonus GENUINELY RISES to +10 (`14 * 3 / 4 = 10`,
//!   up from +9 at level 13) and both good saves (Fortitude, Will) GENUINELY
//!   RISE to +9 (`14 / 2 + 2 = 9`, up from +8), while poor Reflex STAYS +4
//!   (`14 / 3 = 4`, an integer-division coincidence with level 13) — checked
//!   rather than assumed.
//! - Wild Empathy GENUINELY RISES to 15 (druid level 14 + Charisma modifier
//!   1) via the same level-generic formula.
//! - The PF1 Core Rulebook Druid class table's level-14 "Special" column
//!   reads "Wild shape (6/day)". Per the level-4/6/8/10/12 precedent (each
//!   already checked and confirmed a non-separable frequency-plus-form-list
//!   bundle with no execution engine anywhere in this codebase), this
//!   frequency increase stays entirely named-but-unproven: no new
//!   explanation or diagnostic record is fabricated for it. This is case (c)
//!   of the sweep's established patterns — a magnitude rise inside an
//!   already-named, already-unproven burden that does not gate the
//!   arithmetic pillars, so only the arithmetic is widened and the existing
//!   Wild Shape named-but-unproven posture is cited, not re-derived.
//! - Nature Sense, Woodland Stride, Trackless Step, Resist Nature's Lure,
//!   Venom Immunity, A Thousand Faces, and the nature-bond choice
//!   recognition all carry over unchanged, not re-derived.
//!
//! It deliberately does not touch the animal-companion execution burden, the
//! Wild Shape execution burden, or the prepared divine spell posture burden
//! (all three stay named-but-unproven, unchanged from levels 1-13), and it
//! does not ground Druid level 15+. It also preserves the accepted Druid
//! level-1..level-13 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level13_sd18_widening_deterministic_input.txt"
);

const DRUID_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level14_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DRUID_WOODLAND_STRIDE_ID: &str = "class_feature.druid.woodland_stride";
const DRUID_TRACKLESS_STEP_ID: &str = "class_feature.druid.trackless_step";
const DRUID_RESIST_NATURES_LURE_ID: &str = "class_feature.druid.resist_natures_lure";
const DRUID_VENOM_IMMUNITY_ID: &str = "class_feature.druid.venom_immunity";
const DRUID_A_THOUSAND_FACES_ID: &str = "class_feature.druid.a_thousand_faces";

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

// ----- Base attack bonus and saves genuinely rise at level 14 -----

#[test]
fn druid_level14_base_attack_and_good_saves_rise() {
    let input = load(DRUID_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 10,
        "Druid level 14 3/4-BAB progression (14 * 3 / 4) must genuinely rise to 10: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Druid level 14 good Fortitude (14/2+2) must genuinely rise to 9"
    );

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Druid level 14 poor Reflex (14/3) must stay 4, unchanged from level 13"
    );

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(
        will.value, 9,
        "Druid level 14 good Will (14/2+2) must genuinely rise to 9"
    );
}

// ----- Wild Empathy genuinely rises to fifteen -----

#[test]
fn druid_level14_wild_empathy_rises_to_fifteen() {
    let input = load(DRUID_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 15,
        "Druid level 14 Wild Empathy (druid level 14 + Charisma modifier +1) must equal 15, \
         genuinely risen from 14 at level 13: {}",
        wild_empathy.detail
    );
}

// ----- Remaining pillars carry over unchanged at level 14 -----

#[test]
fn druid_level14_remaining_pillars_carry_over_unchanged() {
    let input = load(DRUID_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(nature_sense.value, 2, "Nature Sense must stay the flat +2 at level 14");

    for (id, expected) in [
        (DRUID_WOODLAND_STRIDE_ID, 0),
        (DRUID_TRACKLESS_STEP_ID, 0),
        (DRUID_RESIST_NATURES_LURE_ID, 4),
        (DRUID_VENOM_IMMUNITY_ID, 0),
        (DRUID_A_THOUSAND_FACES_ID, 0),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "'{id}' must carry over unchanged at level 14: {}",
            record.detail
        );
    }

    let choice = explanation(&computation, "class_chassis.druid.nature_bond_choice");
    assert_eq!(
        choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value at level 14"
    );
}

// ----- Wild Shape stays entirely named-but-unproven at level 14 -----

#[test]
fn druid_level14_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-14 Druid must not fabricate any wild-shape explanation record: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-14 Druid must not fabricate any wild-shape diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 14 -----

#[test]
fn druid_level14_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "level-14 Druid must still claim-block on the animal-companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking
        ),
        "level-14 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-13 fixture is unaffected by this widening -----

#[test]
fn druid_level13_truth_is_unchanged_by_this_slice() {
    let input = load(DRUID_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 14, "Druid level 13 Wild Empathy must stay 14");

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Druid level 13 base attack bonus must stay 9");
}

// ----- Negative control: level 15 stays unrecognized by this slice -----

#[test]
fn druid_level_15_is_not_promoted_by_this_slice() {
    let level_15 = DRUID_LEVEL14_FIXTURE.replace("class:druid:14", "class:druid:15");
    let input = load(&level_15);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id.starts_with("class_feature.druid.")
                || e.id == "class_chassis.spell_baseline.druid"),
        "level-15 Druid must not gain any bounded druid explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level14_recognition() {
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
fn multiclass_druid_level14_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL14_FIXTURE.replace(
        "class_level=class:druid:14",
        "class_level=class:druid:14\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_druid_row_names_level_14_widening() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid progression_and_spell_burden row must exist");

    assert_eq!(druid.support_state, SupportState::Partial);
    assert_eq!(druid.evidence_tier, EvidenceTier::Computed);
    assert_eq!(druid.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        druid.grounding_ref.contains("sd18_druid_level14_widening"),
        "druid row must cite the live SD18 level-14 widening proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 14") || note.contains("level-14"),
        "druid partial note must name the level-14 widening: {note}"
    );
}
