//! SD13-E5 Druid level-10 progression grounding proof.
//!
//! Widens the accepted Druid level-1..level-9 prepared-divine baseline (most
//! recently `tests/sd13_druid_level9_progression.rs`) to Druid level 10 — the
//! tranche's declared ceiling — mirroring the sibling-class level-range-gate
//! idiom (`supported_druid_level` is generalized from `1..=9` to `1..=10` via
//! `MAX_SUPPORTED_DRUID_LEVEL = 10`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Druid class table) were read directly before writing
//! any code or test:
//!
//! - level 10 base attack bonus is +7 (`10 * 3 / 4 = 7`, genuinely risen
//!   from +6 at level 9 — the table's own "+7/+2" iterative notation is not
//!   modeled anywhere in this codebase, only the flat base value) and base
//!   saves are +7 Fortitude and +7 Will (both good, `10 / 2 + 2 = 7`, both
//!   genuinely risen from +6) and +3 Reflex (poor, `10 / 3 = 3`,
//!   numerically unchanged from level 9, an integer-division coincidence) —
//!   confirmed by the same formulas already grounded at levels 1-9, not
//!   re-derived.
//! - Wild Empathy GENUINELY RISES to 11 (druid level 10 + Charisma modifier
//!   1) via the same level-generic formula.
//! - Nature Sense stays the flat +2; Woodland Stride, Trackless Step,
//!   Resist Nature's Lure, Venom Immunity, and the nature-bond choice
//!   recognition all carry over unchanged, not re-derived.
//! - the PF1 Core Rulebook Druid class table's level-10 "Special" column
//!   reads "Wild shape (4/day)" (verified independently against both
//!   primary sources, checked rather than assumed away) — per the
//!   level-4/6/8 precedent, the rule text bundles that frequency increase
//!   with a form-list expansion (a Large elemental or a Large plant
//!   creature) and functioning-level upgrades (elemental body III / plant
//!   shape II), none of which are separable from the "4/day" numeral
//!   without misrepresenting the bundled feature as flat — so Wild Shape
//!   stays entirely named-but-unproven, and no record or diagnostic is
//!   fabricated for it (pinned below).
//!
//! It deliberately does not touch the animal-companion execution burden or
//! the prepared divine spell posture burden (both stay named-but-unproven,
//! unchanged from levels 1-9), and it does not ground Druid level 11+. It
//! also preserves the accepted Druid level-1..level-9 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level9_sd13_deterministic_input.txt");

const DRUID_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level10_sd13_deterministic_input.txt");

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
fn druid_level10_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Druid level 10 3/4-BAB progression (10 * 3 / 4) must equal 7, genuinely risen from 6 \
         at level 10: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(
        fortitude.value, 7,
        "Druid level 10 good Fortitude (10/2+2) must equal 7, genuinely risen from 6"
    );

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 3, "Druid level 10 poor Reflex (10/3) must equal 3");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(
        will.value, 7,
        "Druid level 10 good Will (10/2+2) must equal 7, genuinely risen from 6"
    );
}

// ----- Wild Empathy genuinely rises to 10 at level 9 -----

#[test]
fn druid_level10_wild_empathy_rises_to_eleven() {
    let input = load(DRUID_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 11,
        "Druid level 10 Wild Empathy (druid level 10 + Charisma modifier +1) must equal 11, \
         genuinely risen from 10 at level 10: {}",
        wild_empathy.detail
    );
}

// ----- Venom Immunity is newly grounded as a +0 identity record at level 9 -----

#[test]
fn druid_level10_keeps_venom_immunity_identity_record() {
    let input = load(DRUID_LEVEL10_FIXTURE);
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
fn druid_level10_remaining_pillars_carry_over_unchanged() {
    let input = load(DRUID_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(nature_sense.value, 2, "Nature Sense must stay the flat +2 at level 10");

    for (id, expected) in [
        (DRUID_WOODLAND_STRIDE_ID, 0),
        (DRUID_TRACKLESS_STEP_ID, 0),
        (DRUID_RESIST_NATURES_LURE_ID, 4),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "'{id}' must carry over unchanged at level 10: {}",
            record.detail
        );
    }

    let choice = explanation(&computation, "class_chassis.druid.nature_bond_choice");
    assert_eq!(
        choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value at level 10"
    );
}

// ----- Wild Shape stays entirely named-but-unproven at level 9 -----

#[test]
fn druid_level10_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-10 Druid must not fabricate any wild-shape explanation record (the level-10 'Wild shape (4/day)' entry bundles frequency with the Large \
         elemental/plant form expansion and stays named-but-unproven): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-10 Druid must not fabricate any wild-shape diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 9 -----

#[test]
fn druid_level10_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "level-10 Druid must still claim-block on the animal-companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking
        ),
        "level-10 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn druid_level9_truth_is_unchanged_by_this_slice() {
    let input = load(DRUID_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 10, "Druid level 9 Wild Empathy must stay 10");

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 6, "Druid level 9 good Fortitude must stay 6");
}

// ----- Negative control: level 14 stays unrecognized by this slice -----
//
// Updated by the SD18 druid-level11-widening cycle: Druid level 11 is now
// genuinely promoted (MAX_SUPPORTED_DRUID_LEVEL widened to 11 by
// tests/sd18_druid_level11_widening.rs), so this row's own negative control
// boundary moved to level 12, mirroring the identical fix already made for
// Barbarian's, Bard's, and Cleric's own level-10 sibling tests. The SD18
// druid-level12-widening cycle (cycle-2026-07-15T0500) then genuinely
// promotes level 12 too (MAX_SUPPORTED_DRUID_LEVEL widened to 12 by
// tests/sd18_druid_level12_widening.rs), so this row's own negative control
// boundary moves again to level 13, mirroring the same boundary move
// cycle-2026-07-15T0200 made for Cleric. The SD18 druid-level13-widening
// cycle (cycle-2026-07-15T1600) then genuinely promotes level 13 too
// (MAX_SUPPORTED_DRUID_LEVEL widened to 13 by
// tests/sd18_druid_level13_widening.rs), so this row's own negative control
// boundary moves again to level 14, mirroring the same boundary move
// cycle-2026-07-15T1500 made for Cleric. The SD18 druid-level14-widening
// cycle (cycle-2026-07-15T2400) then genuinely promotes level 14 too
// (MAX_SUPPORTED_DRUID_LEVEL widened to 14 by
// tests/sd18_druid_level14_widening.rs), so this row's own negative control
// boundary moves again to level 15, mirroring the same boundary move
// cycle-2026-07-15T2300 made for Cleric. A still further SD18 slice (the
// loop's FIFTH §3.2 level-15 landing) then genuinely promotes level 15 too
// (MAX_SUPPORTED_DRUID_LEVEL widened to 15 by
// tests/sd18_druid_level15_widening.rs), so this row's own negative control
// boundary moves once more to level 16.
#[test]
fn druid_level_16_is_not_promoted_by_this_slice() {
    let level_16 = DRUID_LEVEL10_FIXTURE.replace("class:druid:10", "class:druid:16");
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
fn fighter_does_not_gain_druid_level10_recognition() {
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
fn multiclass_druid_level10_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL10_FIXTURE.replace(
        "class_level=class:druid:10",
        "class_level=class:druid:10\nclass_level=class:fighter:1",
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
fn matrix_druid_row_names_level_10_widening() {
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
        druid.grounding_ref.contains("sd13_druid_level10_progression"),
        "druid row must cite the live SD13-E5 level-10 proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 10") || note.contains("level-10"),
        "druid partial note must name the level-10 widening: {note}"
    );
}
