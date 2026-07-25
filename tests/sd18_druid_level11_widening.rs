//! SD18 Druid level-11 widening grounding proof.
//!
//! Widens the accepted Druid level-1..level-10 prepared divine spell-bearing
//! baseline (`tests/sd13_druid_level10_progression.rs`, the SD13 tranche's
//! declared ceiling) to Druid level 11 — the fourth SD-18 §3.2 class-row
//! widening, mirroring the sibling-class level-range-gate idiom
//! (`supported_druid_level` is generalized from `1..=10` to `1..=11` via
//! `MAX_SUPPORTED_DRUID_LEVEL = 11`, exactly as `cycle-2026-07-13T1255`
//! widened `MAX_SUPPORTED_BARBARIAN_LEVEL`, `cycle-2026-07-13T1830` widened
//! `MAX_SUPPORTED_BARD_LEVEL`, and `cycle-2026-07-13T2007` widened
//! `MAX_SUPPORTED_CLERIC_LEVEL`, all from 10 to 11). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Druid class table) were read
//! directly before writing any code or test:
//!
//! - level 11 base attack bonus is +8 (`11 * 3 / 4 = 8`, genuinely risen
//!   from +7 at level 10) and base saves are +7 Fortitude and +7 Will (both
//!   good, `11 / 2 + 2 = 7`, numerically unchanged from level 10, an
//!   integer-division coincidence) and +3 Reflex (poor, `11 / 3 = 3`, also
//!   an integer-division coincidence with level 10) — confirmed by the same
//!   formulas already grounded at levels 1-10, not re-derived.
//! - the PF1 Core Rulebook Druid class table's level-11 "Special" column is
//!   genuinely blank (verified independently against both primary sources,
//!   checked rather than assumed away: neither source lists any new class
//!   feature at 11th level — the level-10 "Wild shape (4/day)" entry does
//!   not rise again until 12th level, "Wild shape (5/day)"), so this slice
//!   grounds no new pillar at level 11 either — unlike Barbarian's Greater
//!   Rage, Bard's Inspire widening, and Cleric's Channel Energy/domain-slot
//!   rise, Druid's own level-11 tier adds no named-feature magnitude rise at
//!   all; only the already-grounded arithmetic pillars widen.
//! - Wild Empathy GENUINELY RISES to 12 (druid level 11 + Charisma modifier
//!   1) via the same level-generic formula.
//! - Nature Sense stays the flat +2; Woodland Stride, Trackless Step,
//!   Resist Nature's Lure, Venom Immunity, and the nature-bond choice
//!   recognition all carry over unchanged, not re-derived.
//!
//! It deliberately does not touch the animal-companion execution burden or
//! the prepared divine spell posture burden (both stay named-but-unproven,
//! unchanged from levels 1-10), and it does not ground Druid level 12+. It
//! also preserves the accepted Druid level-1..level-10 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level10_sd13_deterministic_input.txt");

const DRUID_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level11_sd18_widening_deterministic_input.txt"
);

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

// ----- Base attack bonus and saves at level 11 -----

#[test]
fn druid_level11_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Druid level 11 3/4-BAB progression (11 * 3 / 4) must equal 8, genuinely risen from \
         7 at level 10: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 7, "Druid level 11 good Fortitude (11/2+2) must stay 7");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 3, "Druid level 11 poor Reflex (11/3) must stay 3");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 7, "Druid level 11 good Will (11/2+2) must stay 7");
}

// ----- Wild Empathy genuinely rises to twelve -----

#[test]
fn druid_level11_wild_empathy_rises_to_twelve() {
    let input = load(DRUID_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 12,
        "Druid level 11 Wild Empathy (druid level 11 + Charisma modifier +1) must equal 12, \
         genuinely risen from 11 at level 10: {}",
        wild_empathy.detail
    );
}

// ----- Remaining pillars carry over unchanged at level 11 -----

#[test]
fn druid_level11_remaining_pillars_carry_over_unchanged() {
    let input = load(DRUID_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(nature_sense.value, 2, "Nature Sense must stay the flat +2 at level 11");

    for (id, expected) in [
        (DRUID_WOODLAND_STRIDE_ID, 0),
        (DRUID_TRACKLESS_STEP_ID, 0),
        (DRUID_RESIST_NATURES_LURE_ID, 4),
        (DRUID_VENOM_IMMUNITY_ID, 0),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "'{id}' must carry over unchanged at level 11: {}",
            record.detail
        );
    }

    let choice = explanation(&computation, "class_chassis.druid.nature_bond_choice");
    assert_eq!(
        choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value at level 11"
    );
}

// ----- Wild Shape stays entirely named-but-unproven at level 11 -----

#[test]
fn druid_level11_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-11 Druid must not fabricate any wild-shape explanation record (the PF1 CRB \
         Druid class table's level-11 'Special' column is genuinely blank): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-11 Druid must not fabricate any wild-shape diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 11 -----

#[test]
fn druid_level11_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "level-11 Druid must still claim-block on the animal-companion execution burden: {:?}",
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

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn druid_level10_truth_is_unchanged_by_this_slice() {
    let input = load(DRUID_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 11, "Druid level 10 Wild Empathy must stay 11");

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 7, "Druid level 10 base attack bonus must stay 7");
}

// ----- Negative control: level 16 stays unrecognized by this slice -----
// (Druid level 12, level 13, level 14, and level 15 were widened into scope
// by later SD18 slices — tests/sd18_druid_level12_widening.rs,
// tests/sd18_druid_level13_widening.rs, tests/sd18_druid_level14_widening.rs,
// and tests/sd18_druid_level15_widening.rs — so this negative control's
// boundary moves from 12 to 13 to 14 to 15 to 16, mirroring the exact same
// boundary-move idiom applied to tests/sd18_cleric_level11_widening.rs.)

#[test]
fn druid_level_16_is_not_promoted_by_this_slice() {
    let level_16 = DRUID_LEVEL11_FIXTURE.replace("class:druid:11", "class:druid:16");
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
fn fighter_does_not_gain_druid_level11_recognition() {
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
fn multiclass_druid_level11_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL11_FIXTURE.replace(
        "class_level=class:druid:11",
        "class_level=class:druid:11\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_druid_row_names_level_11_widening() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid progression_and_spell_burden row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(druid.support_state, SupportState::Supported);
    assert_eq!(druid.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(druid.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        druid.grounding_ref.contains("sd18_druid_level11_widening"),
        "druid row must cite the live SD18 level-11 widening proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "druid partial note must name the level-11 widening: {note}"
    );
}
