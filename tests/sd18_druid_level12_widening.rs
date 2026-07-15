//! SD18 Druid level-12 widening grounding proof.
//!
//! Widens the accepted Druid level-1..level-11 prepared divine spell-bearing
//! baseline (`tests/sd18_druid_level11_widening.rs`, the loop's most recent
//! Druid ceiling) to Druid level 12 — mirroring the sibling-class
//! level-range-gate idiom (`supported_druid_level` is generalized from
//! `1..=11` to `1..=12` via `MAX_SUPPORTED_DRUID_LEVEL = 12`, exactly as
//! `cycle-2026-07-14T1814` widened `MAX_SUPPORTED_BARBARIAN_LEVEL`,
//! `cycle-2026-07-14T2359` widened `MAX_SUPPORTED_BARD_LEVEL`, and
//! `cycle-2026-07-15T0200` widened `MAX_SUPPORTED_CLERIC_LEVEL`, all from 11
//! to 12). Both PF1 CRB primary sources (d20pfsrd and legacy.aonprd.com
//! Druid class table) were read directly before writing any code or test:
//!
//! - level 12 base attack bonus is +9 (`12 * 3 / 4 = 9`, genuinely risen
//!   from +8 at level 11) and base saves are +8 Fortitude and +8 Will (both
//!   good, `12 / 2 + 2 = 8`, genuinely risen from +7) and +4 Reflex (poor,
//!   `12 / 3 = 4`, genuinely risen from +3) — confirmed by the same formulas
//!   already grounded at levels 1-11, not re-derived.
//! - UNLIKE level 11 (whose "Special" column was genuinely blank), the PF1
//!   Core Rulebook Druid class table's level-12 "Special" column reads
//!   "Wild shape (5/day)" (verified independently against both primary
//!   sources, checked rather than assumed away) — but per the
//!   level-4/6/8/10 precedent (see `tests/sd13_druid_level4_progression.rs`
//!   onward) that frequency rise is bundled with a form-list expansion
//!   (Huge elemental or Huge plant creature) and a functioning-level
//!   upgrade (elemental body IV / plant shape III), neither of which is
//!   separable from the "5/day" numeral without misrepresenting the bundled
//!   feature as flat, so Wild Shape stays entirely named-but-unproven and
//!   this slice grounds no new pillar at level 12 either.
//! - Wild Empathy GENUINELY RISES to 13 (druid level 12 + Charisma modifier
//!   1) via the same level-generic formula.
//! - Nature Sense stays the flat +2; Woodland Stride, Trackless Step,
//!   Resist Nature's Lure, Venom Immunity, and the nature-bond choice
//!   recognition all carry over unchanged, not re-derived.
//!
//! It deliberately does not touch the animal-companion execution burden, the
//! Wild Shape execution burden (including its own level-12 form-list/
//! functioning-level upgrade), or the prepared divine spell posture burden
//! (all three stay named-but-unproven, unchanged from levels 1-11), and it
//! does not ground Druid level 13+. It also preserves the accepted Druid
//! level-1..level-11 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const DRUID_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level11_sd18_widening_deterministic_input.txt"
);

const DRUID_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level12_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus and saves genuinely rise at level 12 -----

#[test]
fn druid_level12_base_attack_and_saves_genuinely_rise() {
    let input = load(DRUID_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Druid level 12 3/4-BAB progression (12 * 3 / 4) must equal 9, genuinely risen from \
         8 at level 11: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(
        fortitude.value, 8,
        "Druid level 12 good Fortitude (12/2+2) must genuinely rise to 8, up from 7 at level 11"
    );

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Druid level 12 poor Reflex (12/3) must genuinely rise to 4, up from 3 at level 11"
    );

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(
        will.value, 8,
        "Druid level 12 good Will (12/2+2) must genuinely rise to 8, up from 7 at level 11"
    );
}

// ----- Wild Empathy genuinely rises to thirteen -----

#[test]
fn druid_level12_wild_empathy_rises_to_thirteen() {
    let input = load(DRUID_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 13,
        "Druid level 12 Wild Empathy (druid level 12 + Charisma modifier +1) must equal 13, \
         genuinely risen from 12 at level 11: {}",
        wild_empathy.detail
    );
}

// ----- Remaining pillars carry over unchanged at level 12 -----

#[test]
fn druid_level12_remaining_pillars_carry_over_unchanged() {
    let input = load(DRUID_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(nature_sense.value, 2, "Nature Sense must stay the flat +2 at level 12");

    for (id, expected) in [
        (DRUID_WOODLAND_STRIDE_ID, 0),
        (DRUID_TRACKLESS_STEP_ID, 0),
        (DRUID_RESIST_NATURES_LURE_ID, 4),
        (DRUID_VENOM_IMMUNITY_ID, 0),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, expected,
            "'{id}' must carry over unchanged at level 12: {}",
            record.detail
        );
    }

    let choice = explanation(&computation, "class_chassis.druid.nature_bond_choice");
    assert_eq!(
        choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value at level 12"
    );
}

// ----- Wild Shape stays entirely named-but-unproven at level 12, despite the -----
// ----- class table's "Wild shape (5/day)" level-12 Special-column entry -----

#[test]
fn druid_level12_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-12 Druid must not fabricate any wild-shape explanation record (the level-12 \
         'Wild shape (5/day)' Special-column entry bundles a non-separable form-list expansion \
         and functioning-level upgrade, per the level-4/6/8/10 precedent): {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-12 Druid must not fabricate any wild-shape diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- The two existing burden diagnostics still fire at level 12 -----

#[test]
fn druid_level12_still_claim_blocks_animal_companion_and_prepared_divine_burdens() {
    let input = load(DRUID_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "level-12 Druid must still claim-block on the animal-companion execution burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking
        ),
        "level-12 Druid must still claim-block on the prepared divine spell posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-11 fixture is unaffected by this widening -----

#[test]
fn druid_level11_truth_is_unchanged_by_this_slice() {
    let input = load(DRUID_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 12, "Druid level 11 Wild Empathy must stay 12");

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Druid level 11 base attack bonus must stay 8");
}

// ----- Negative control: level 16 stays unrecognized by this slice -----
// (Superseded boundary: cycle-2026-07-15T1600 widened MAX_SUPPORTED_DRUID_LEVEL
// from 12 to 13, cycle-2026-07-15T2400 widened it again from 13 to 14, and a
// still further SD18 slice (the loop's FIFTH §3.2 level-15 landing) widened
// it once more from 14 to 15, so this file's own negative-control boundary
// moves from 13 to 14 to 15 to 16, mirroring the exact same boundary-move
// idiom applied to tests/sd18_cleric_level12_widening.rs when
// MAX_SUPPORTED_CLERIC_LEVEL widened from 12 to 13 and then 13 to 14.)

#[test]
fn druid_level_16_is_not_promoted_by_this_slice() {
    let level_16 = DRUID_LEVEL12_FIXTURE.replace("class:druid:12", "class:druid:16");
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
fn fighter_does_not_gain_druid_level12_recognition() {
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
fn multiclass_druid_level12_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL12_FIXTURE.replace(
        "class_level=class:druid:12",
        "class_level=class:druid:12\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_druid_row_names_level_12_widening() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid progression_and_spell_burden row must exist");

    assert_eq!(druid.support_state, SupportState::Partial);
    assert_eq!(druid.evidence_tier, EvidenceTier::Computed);
    assert_eq!(druid.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        druid.grounding_ref.contains("sd18_druid_level12_widening"),
        "druid row must cite the live SD18 level-12 widening proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "druid partial note must name the level-12 widening: {note}"
    );
}
