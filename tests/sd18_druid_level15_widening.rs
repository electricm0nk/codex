//! SD18 Druid level-15 widening grounding proof.
//!
//! Widens the accepted Druid level-1..level-14 prepared divine spell-bearing
//! baseline (`tests/sd18_druid_level14_widening.rs`, the loop's most recent
//! Druid ceiling) to Druid level 15 — mirroring the sibling-class
//! level-range-gate idiom (`supported_druid_level` is generalized from
//! `1..=14` to `1..=15` via `MAX_SUPPORTED_DRUID_LEVEL = 15`, the loop's
//! FIFTH §3.2 level-15 landing after Barbarian, Rogue, Fighter, and Cleric).
//! All three primary sources (d20pfsrd, Archives of Nethys aonprd.com, and
//! legacy.aonprd.com) were read directly before writing any code or test and
//! agree byte-for-byte:
//!
//! - level 15 base attack bonus GENUINELY RISES to +11 (`15 * 3 / 4 = 11`,
//!   up from +10 at level 14) and poor Reflex GENUINELY RISES to +5
//!   (`15 / 3 = 5`, up from +4), while both good saves (Fortitude, Will)
//!   STAY +9 (`15 / 2 + 2 = 9`, an integer-division coincidence with level
//!   14) — checked rather than assumed.
//! - Wild Empathy GENUINELY RISES to 16 (druid level 15 + Charisma modifier
//!   1) via the same level-generic formula.
//! - The PF1 Core Rulebook Druid class table's level-15 "Special" column
//!   reads "Timeless body" ONLY — UNLIKE every prior widened level's
//!   Wild-Shape-shaped "Special" column entry (levels 4/6/8/10/12/14), and
//!   unlike level 13's "A thousand faces", level 15 names a DIFFERENT single
//!   flat class feature with no accompanying Wild Shape frequency increase
//!   (the next one, "Wild shape (7/day)", does not land until 16th level,
//!   confirmed directly rather than assumed). Timeless Body is a genuinely
//!   flat/identity-shaped, no-choice, no-magnitude, no-duration-tracking
//!   grant (a druid no longer takes ability score penalties for old age and
//!   cannot be magically aged), mirroring exactly how Venom Immunity (level
//!   9) and A Thousand Faces (level 13) were grounded: a bounded +0
//!   identity/recognition record, with no aging-penalty-resolution engine
//!   fabricated.
//! - Nature Sense, Woodland Stride, Trackless Step, Resist Nature's Lure,
//!   Venom Immunity, A Thousand Faces, and the nature-bond choice
//!   recognition all carry over unchanged, not re-derived.
//!
//! It deliberately does not touch the animal-companion execution burden, the
//! Wild Shape execution burden, or the prepared divine spell posture burden
//! (all three stay named-but-unproven, unchanged from levels 1-14), and it
//! does not ground Druid level 16+. It also preserves the accepted Druid
//! level-1..level-14 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level14_sd18_widening_deterministic_input.txt"
);

const DRUID_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level15_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DRUID_WOODLAND_STRIDE_ID: &str = "class_feature.druid.woodland_stride";
const DRUID_TRACKLESS_STEP_ID: &str = "class_feature.druid.trackless_step";
const DRUID_RESIST_NATURES_LURE_ID: &str = "class_feature.druid.resist_natures_lure";
const DRUID_VENOM_IMMUNITY_ID: &str = "class_feature.druid.venom_immunity";
const DRUID_A_THOUSAND_FACES_ID: &str = "class_feature.druid.a_thousand_faces";
const DRUID_TIMELESS_BODY_ID: &str = "class_feature.druid.timeless_body";

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

// ----- Base attack bonus and saves at level 15 -----

#[test]
fn druid_level15_base_attack_and_reflex_rise_while_good_saves_stay() {
    let input = load(DRUID_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 11,
        "Druid level 15 3/4-BAB progression (15 * 3 / 4) must genuinely rise to 11: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Druid level 15 good Fortitude (15/2+2) must stay 9, an integer-division coincidence \
         with level 14"
    );

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Druid level 15 poor Reflex (15/3) must genuinely rise to 5"
    );

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(
        will.value, 9,
        "Druid level 15 good Will (15/2+2) must stay 9, an integer-division coincidence with \
         level 14"
    );
}

// ----- Wild Empathy genuinely rises to sixteen -----

#[test]
fn druid_level15_wild_empathy_rises_to_sixteen() {
    let input = load(DRUID_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 16,
        "Druid level 15 Wild Empathy (druid level 15 + Charisma modifier +1) must equal 16, \
         genuinely risen from 15 at level 14: {}",
        wild_empathy.detail
    );
}

// ----- Timeless Body newly grounded at level 15 -----

#[test]
fn druid_level15_grounds_timeless_body() {
    let input = load(DRUID_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let timeless_body = explanation(&computation, DRUID_TIMELESS_BODY_ID);
    assert_eq!(
        timeless_body.value, 0,
        "Timeless Body must be a bounded +0 identity/recognition record, not a fabricated \
         aging-penalty resolution: {}",
        timeless_body.detail
    );
}

#[test]
fn druid_level14_does_not_yet_ground_timeless_body() {
    let input = load(DRUID_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.explanations.iter().any(|e| e.id == DRUID_TIMELESS_BODY_ID),
        "level-14 Druid must not yet gain Timeless Body: {:?}",
        computation.explanations
    );
}

// ----- Remaining pillars carry over unchanged at level 15 -----

#[test]
fn druid_level15_remaining_pillars_carry_over_unchanged() {
    let input = load(DRUID_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(nature_sense.value, 2, "Nature Sense must stay the flat +2 at level 15");

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
            "'{id}' must carry over unchanged at level 15: {}",
            record.detail
        );
    }

    let choice = explanation(&computation, "class_chassis.druid.nature_bond_choice");
    assert_eq!(
        choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value at level 15"
    );
}

// ----- Wild Shape stays entirely named-but-unproven at level 15 -----

#[test]
fn druid_level15_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-15 Druid must not fabricate any wild-shape explanation record: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-15 Druid must not fabricate any wild-shape diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 15 -----

#[test]
fn druid_level15_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "level-15 Druid must still claim-block on the animal-companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking
        ),
        "level-15 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-14 fixture is unaffected by this widening -----

#[test]
fn druid_level14_truth_is_unchanged_by_this_slice() {
    let input = load(DRUID_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 15, "Druid level 14 Wild Empathy must stay 15");

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 10, "Druid level 14 base attack bonus must stay 10");
}

// ----- Negative control: level 16 stays unrecognized by this slice -----

#[test]
fn druid_level_16_is_not_promoted_by_this_slice() {
    let level_16 = DRUID_LEVEL15_FIXTURE.replace("class:druid:15", "class:druid:16");
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
fn fighter_does_not_gain_druid_level15_recognition() {
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
fn multiclass_druid_level15_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL15_FIXTURE.replace(
        "class_level=class:druid:15",
        "class_level=class:druid:15\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_druid_row_names_level_15_widening() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid progression_and_spell_burden row must exist");

    assert_eq!(druid.support_state, SupportState::Partial);
    assert_eq!(druid.evidence_tier, EvidenceTier::Computed);
    assert_eq!(druid.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        druid.grounding_ref.contains("sd18_druid_level15_widening"),
        "druid row must cite the live SD18 level-15 widening proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "druid partial note must name the level-15 widening: {note}"
    );
}
