//! SD18 Druid level-13 widening grounding proof.
//!
//! Widens the accepted Druid level-1..level-12 prepared divine spell-bearing
//! baseline (`tests/sd18_druid_level12_widening.rs`, the loop's most recent
//! Druid ceiling) to Druid level 13 — mirroring the sibling-class
//! level-range-gate idiom (`supported_druid_level` is generalized from
//! `1..=12` to `1..=13` via `MAX_SUPPORTED_DRUID_LEVEL = 13`, exactly as
//! `cycle-2026-07-15T1500` widened `MAX_SUPPORTED_CLERIC_LEVEL` from 12 to
//! 13, the loop's fifth §3.2 level-13 landing after Rogue, Barbarian,
//! Fighter, and Ranger). All three primary sources (d20pfsrd, Archives of
//! Nethys aonprd.com, and legacy.aonprd.com Druid class table and
//! spells-per-day table) were read directly before writing any code or
//! test:
//!
//! - level 13 base attack bonus STAYS +9 (`13 * 3 / 4 = 9`, an
//!   integer-division coincidence with level 12) and base saves STAY +8
//!   Fortitude and +8 Will (both good, `13 / 2 + 2 = 8`) and +4 Reflex
//!   (poor, `13 / 3 = 4`) — all four coincidences, checked rather than
//!   assumed.
//! - Wild Empathy GENUINELY RISES to 14 (druid level 13 + Charisma modifier
//!   1) via the same level-generic formula.
//! - UNLIKE every prior widened level's Wild-Shape-shaped entries (4, 6, 8,
//!   10, 12), the PF1 Core Rulebook Druid class table's level-13 "Special"
//!   column reads "A thousand faces" — a DIFFERENT class feature, not a
//!   Wild Shape frequency increase. Verified independently against all
//!   three primary sources: in PF1 (unlike the D&D 3.5 version, which
//!   referenced the stronger `alter self` spell), A Thousand Faces grants
//!   the druid the ability to change her own apparent appearance at will,
//!   as if using `disguise self`, but only while in her normal (unshifted)
//!   form. This is a genuinely flat/identity-shaped, no-choice,
//!   no-magnitude, no-duration-tracking grant — mirroring exactly how Venom
//!   Immunity was grounded at level 9 — so it is grounded here as a bounded
//!   +0 identity/recognition record: no illusion-effect execution engine
//!   and no disguise-check-resolution engine exists anywhere in this
//!   codebase, so no actual appearance-change or Disguise-check outcome is
//!   fabricated.
//! - Nature Sense stays the flat +2; Woodland Stride, Trackless Step,
//!   Resist Nature's Lure, Venom Immunity, and the nature-bond choice
//!   recognition all carry over unchanged, not re-derived.
//!
//! It deliberately does not touch the animal-companion execution burden, the
//! Wild Shape execution burden, or the prepared divine spell posture burden
//! (all three stay named-but-unproven, unchanged from levels 1-12), and it
//! does not ground Druid level 14+. It also preserves the accepted Druid
//! level-1..level-12 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level12_sd18_widening_deterministic_input.txt"
);

const DRUID_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level13_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus and saves stay unchanged at level 13 -----

#[test]
fn druid_level13_base_attack_and_saves_stay_at_level12_values() {
    let input = load(DRUID_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Druid level 13 3/4-BAB progression (13 * 3 / 4) must stay 9, an integer-division \
         coincidence with level 12: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(
        fortitude.value, 8,
        "Druid level 13 good Fortitude (13/2+2) must stay 8, unchanged from level 12"
    );

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Druid level 13 poor Reflex (13/3) must stay 4, unchanged from level 12"
    );

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(
        will.value, 8,
        "Druid level 13 good Will (13/2+2) must stay 8, unchanged from level 12"
    );
}

// ----- Wild Empathy genuinely rises to fourteen -----

#[test]
fn druid_level13_wild_empathy_rises_to_fourteen() {
    let input = load(DRUID_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 14,
        "Druid level 13 Wild Empathy (druid level 13 + Charisma modifier +1) must equal 14, \
         genuinely risen from 13 at level 12: {}",
        wild_empathy.detail
    );
}

// ----- A Thousand Faces is newly grounded as a flat +0 identity record -----

#[test]
fn druid_level13_a_thousand_faces_is_grounded_as_flat_identity_record() {
    let input = load(DRUID_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let record = explanation(&computation, DRUID_A_THOUSAND_FACES_ID);
    assert_eq!(
        record.value, 0,
        "A Thousand Faces must be a bounded +0 identity/recognition record, not a fabricated \
         mechanical value: {}",
        record.detail
    );
    assert!(
        record.detail.to_lowercase().contains("disguise self"),
        "A Thousand Faces detail must cite the PF1 disguise-self-shaped rule text: {}",
        record.detail
    );
}

// ----- A Thousand Faces is correctly absent below level 13 -----

#[test]
fn druid_level12_does_not_yet_have_a_thousand_faces() {
    let input = load(DRUID_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == DRUID_A_THOUSAND_FACES_ID),
        "level-12 Druid must not yet gain A Thousand Faces (a 13th-level druid class feature): \
         {:?}",
        computation.explanations
    );
}

// ----- Remaining pillars carry over unchanged at level 13 -----

#[test]
fn druid_level13_remaining_pillars_carry_over_unchanged() {
    let input = load(DRUID_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(nature_sense.value, 2, "Nature Sense must stay the flat +2 at level 13");

    for (id, expected) in [
        (DRUID_WOODLAND_STRIDE_ID, 0),
        (DRUID_TRACKLESS_STEP_ID, 0),
        (DRUID_RESIST_NATURES_LURE_ID, 4),
        (DRUID_VENOM_IMMUNITY_ID, 0),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "'{id}' must carry over unchanged at level 13: {}",
            record.detail
        );
    }

    let choice = explanation(&computation, "class_chassis.druid.nature_bond_choice");
    assert_eq!(
        choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value at level 13"
    );
}

// ----- Wild Shape stays entirely named-but-unproven at level 13 -----

#[test]
fn druid_level13_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-13 Druid must not fabricate any wild-shape explanation record: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-13 Druid must not fabricate any wild-shape diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 13 -----

#[test]
fn druid_level13_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "level-13 Druid must still claim-block on the animal-companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking
        ),
        "level-13 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-12 fixture is unaffected by this widening -----

#[test]
fn druid_level12_truth_is_unchanged_by_this_slice() {
    let input = load(DRUID_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 13, "Druid level 12 Wild Empathy must stay 13");

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Druid level 12 base attack bonus must stay 9");
}

// ----- Negative control: level 16 stays unrecognized by this slice -----
// (Superseded boundary: cycle-2026-07-15T2400 widened MAX_SUPPORTED_DRUID_LEVEL
// from 13 to 14, and a still further SD18 slice (the loop's FIFTH §3.2
// level-15 landing) widened it again from 14 to 15, so this file's own
// negative-control boundary moves from 14 to 15 to 16, mirroring the exact
// same boundary-move idiom applied to tests/sd18_cleric_level13_widening.rs
// when MAX_SUPPORTED_CLERIC_LEVEL widened from 13 to 14.)

#[test]
fn druid_level_16_is_not_promoted_by_this_slice() {
    let level_16 = DRUID_LEVEL13_FIXTURE.replace("class:druid:13", "class:druid:16");
    let input = load(&level_16);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id.starts_with("class_feature.druid.")
                || e.id == "class_chassis.spell_baseline.druid"),
        "level-16 Druid must not gain any bounded druid explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level13_recognition() {
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
fn multiclass_druid_level13_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL13_FIXTURE.replace(
        "class_level=class:druid:13",
        "class_level=class:druid:13\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-13 widening -----

#[test]
fn matrix_druid_row_names_level_13_widening() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid progression_and_spell_burden row must exist");

    assert_eq!(druid.support_state, SupportState::Partial);
    assert_eq!(druid.evidence_tier, EvidenceTier::Computed);
    assert_eq!(druid.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        druid.grounding_ref.contains("sd18_druid_level13_widening"),
        "druid row must cite the live SD18 level-13 widening proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 13") || note.contains("level-13"),
        "druid partial note must name the level-13 widening: {note}"
    );
}
