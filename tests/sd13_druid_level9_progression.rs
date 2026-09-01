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

use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

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

// ----- The animal companion: a deliberately level-generic, multiclass-reachable seam -----

/// True for the animal-companion records, the one druid-namespaced family that
/// is deliberately NOT withheld by the `supported_druid_level` gate.
///
/// `explain_druid_level1_spell_baseline` validates both Druid burdens *before*
/// that single-class/level gate, so a multiclass Druid cannot silently escape
/// them (v0.6 alpha swarm, risks item 8, seventh slice, 2026-07-25). Commit
/// `ae63aa4c` then grounded the companion progression from the corpus at all
/// twenty master levels, which turned that pre-gate validation from a
/// claim-blocking diagnostic into real, corpus-derived records. Those records
/// are consequently reachable from a multiclass mix, and from a druid level
/// above the chassis ceiling -- correctly so in PF1 RAW, where a companion's
/// effective druid level is the druid CLASS level alone and is unaffected by
/// levels in any other class (`CompanionMasterLVL_Druid|DruidLVL`,
/// `core_rulebook/cr_abilities_class.lst`).
///
/// The negative controls below exclude exactly this family and nothing else,
/// so the bounded druid *chassis* stays as tightly fenced as it always was.
fn is_animal_companion_record(id: &str) -> bool {
    id.starts_with("class_chassis.druid.animal_companion.")
        || id.starts_with("class_feature.druid.animal_companion.")
}

/// True for any druid-namespaced record the `supported_druid_level` gate is
/// supposed to withhold from an unsupported input -- i.e. the bounded druid
/// chassis, its spell baseline, and its class-feature identity records, but
/// not the level-generic animal companion above.
fn is_gated_druid_chassis_record(id: &str) -> bool {
    !is_animal_companion_record(id)
        && (id.starts_with("class_chassis.druid.")
            || id.starts_with("class_feature.druid.")
            || id == "class_chassis.spell_baseline.druid")
}

/// Asserts the Wolf animal companion's entire standalone stat block at
/// `master_level`.
///
/// Every expected value passed in by the callers below is transcribed from the
/// PCGen corpus, never copied from engine output:
///
/// - Hit Dice: a 2 HD floor (`MONSTERCLASS:Companion:2` on `Companion (Wolf)`,
///   `core_rulebook/cr_races_companion.lst`) plus one further `HD:1` at master
///   levels 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, 16, 17, 18 and 20, and nowhere
///   else (`core_rulebook/cr_companionmods.lst`) -- giving
///   2,3,3,4,5,6,6,7,8,9,9,10,11,12,12,13,14,15,15,16.
/// - Natural armor: the Wolf race's own `BONUS:VAR|AC_Natural_Armor|2|TYPE=Base`
///   plus the companion class's `2*floor(MasterLevel/3)`
///   (`cr_abilities_companion.lst`, `Animal Companion ~ AC Bonus`).
/// - Strength: the Wolf's base 13 plus the companion class's
///   `floor(MasterLevel/3)` (`cr_abilities_companion.lst`,
///   `Animal Companion ~ Stat Bonus`).
/// - Base attack bonus `HD*3/4`, base Fortitude/Reflex `HD/2+2` and base Will
///   `HD/3` (PF1 CRB Animal Companion Base Statistics); primary natural attack
///   damage `floor(1.5 * Strength modifier)` (PF1 CRB natural-attack rule); hit
///   points on this codebase's own maximized-first-die-plus-average idiom
///   across the companion's real Hit Dice.
///
/// This pins the whole progression, so a regression anywhere in the companion
/// table, in the natural-armor/Strength advances, or in the effective-druid-level
/// keying fails loudly instead of silently shipping a wrong stat block.
#[allow(clippy::too_many_arguments)]
fn assert_wolf_companion_stat_block(
    computation: &PilotBaseChassisComputation,
    master_level: u8,
    expected_hit_dice: u8,
    expected_attack_bonus: i16,
    expected_fortitude_and_reflex: i16,
    expected_will: i16,
    expected_armor_class: i16,
    expected_bite_damage_bonus: i16,
    expected_hit_points: i16,
) {
    let record = |id: &str| -> &ComputationExplanation {
        computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| {
                panic!(
                    "expected grounded animal-companion record '{id}' at master level \
                     {master_level}, got {:?}",
                    computation.explanations
                )
            })
    };

    let stat_block = record("class_chassis.druid.animal_companion.wolf_stat_block");
    assert_eq!(
        stat_block.value, 0,
        "the companion stat-block header is a recognition record only and must carry no \
         fabricated mechanical value: {}",
        stat_block.detail
    );

    let attack = record("class_chassis.druid.animal_companion.base_attack_bonus");
    assert_eq!(
        attack.value, expected_attack_bonus,
        "Wolf companion attack bonus (HD*3/4 + Strength modifier) at master level \
         {master_level}: {}",
        attack.detail
    );
    assert!(
        attack.detail.contains(&format!("at {expected_hit_dice} HD")),
        "the companion must be advanced to the corpus's own {expected_hit_dice} Hit Dice at \
         master level {master_level}: {}",
        attack.detail
    );

    let fortitude = record("class_chassis.druid.animal_companion.base_save.fortitude");
    assert_eq!(
        fortitude.value, expected_fortitude_and_reflex,
        "Wolf companion base Fortitude save (HD/2+2) at master level {master_level}: {}",
        fortitude.detail
    );
    let reflex = record("class_chassis.druid.animal_companion.base_save.reflex");
    assert_eq!(
        reflex.value, expected_fortitude_and_reflex,
        "Wolf companion base Reflex save (HD/2+2) at master level {master_level}: {}",
        reflex.detail
    );
    let will = record("class_chassis.druid.animal_companion.base_save.will");
    assert_eq!(
        will.value, expected_will,
        "Wolf companion base Will save (HD/3) at master level {master_level}: {}",
        will.detail
    );

    let armor_class = record("class_chassis.druid.animal_companion.armor_class");
    assert_eq!(
        armor_class.value, expected_armor_class,
        "Wolf companion armor class (10 + base natural armor 2 + 2*floor({master_level}/3)): {}",
        armor_class.detail
    );

    let bite = record("class_chassis.druid.animal_companion.bite_attack");
    assert_eq!(
        bite.value, expected_bite_damage_bonus,
        "Wolf companion bite damage bonus (1.5x Strength modifier, floored) at master level \
         {master_level}: {}",
        bite.detail
    );

    let hit_points = record("class_chassis.druid.animal_companion.hit_points");
    assert_eq!(
        hit_points.value, expected_hit_points,
        "Wolf companion hit points across {expected_hit_dice} d8 Hit Dice at master level \
         {master_level}: {}",
        hit_points.detail
    );

    for vacuous_id in [
        "class_feature.druid.animal_companion.link_vacuous",
        "class_feature.druid.animal_companion.share_spells_vacuous",
    ] {
        let vacuous = record(vacuous_id);
        assert_eq!(
            vacuous.value, 0,
            "'{vacuous_id}' is a vacuity-correction record and must carry no mechanical \
             value: {}",
            vacuous.detail
        );
    }
}

/// Asserts that the animal-companion burden is genuinely closed: the old
/// catch-all claim-blocker is gone, and the non-blocking advancement diagnostic
/// that replaced it still names every column left deferred.
fn assert_animal_companion_burden_is_closed_but_discloses_its_gaps(
    computation: &PilotBaseChassisComputation,
) {
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.druid.animal_companion.unsupported"),
        "the animal-companion burden is grounded from the corpus at every master level, so \
         the catch-all blocker must no longer fire for a chosen animal companion: {:?}",
        computation.diagnostics
    );

    let advancement = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.druid.animal_companion.advancement_absent")
        .unwrap_or_else(|| {
            panic!(
                "a grounded companion must still disclose the columns it does not ground: {:?}",
                computation.diagnostics
            )
        });
    assert!(
        !advancement.claim_blocking,
        "the advancement-absent disclosure names deferred columns that have no consumer in \
         this engine; it must not claim-block: {}",
        advancement.message
    );
    for deferred in ["bonus tricks", "Companion Stat Increase", "Evasion"] {
        assert!(
            advancement.message.contains(deferred),
            "the advancement-absent disclosure must keep naming the deferred '{deferred}' \
             column rather than quietly implying full support: {}",
            advancement.message
        );
    }
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

// ----- The animal companion is grounded; the prepared divine burden still fires at level 9 -----

#[test]
fn druid_level9_grounds_the_animal_companion_and_holds_the_prepared_divine_posture() {
    let input = load(DRUID_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Superseded premise, corrected 2026-07-29. This test used to require
    // `class_feature.druid.animal_companion.unsupported` to fire claim-blocking
    // at level 9. Commit `ae63aa4c` grounded the animal companion's whole
    // progression from the PCGen corpus at all twenty master levels, so at
    // level 9 the burden is genuinely closed and that blocker is correctly
    // gone -- the test's premise, not the engine, is what changed.
    //
    // The original protective intent (no unproven animal-companion support may
    // reach a caller unannounced) is preserved and strengthened rather than
    // dropped: instead of only checking that the seam was shut, the companion's
    // real corpus-derived stat block at this exact master level is now pinned
    // value by value, and the non-blocking disclosure that replaced the blocker
    // is required to keep naming every column still deferred.
    assert_animal_companion_burden_is_closed_but_discloses_its_gaps(&computation);
    assert_wolf_companion_stat_block(
        &computation, 9, 8, 9, 6, 2, 18, 4, 59,
    );

    // Unchanged: the prepared divine spell posture burden keeps its original
    // shape -- either the blocker fires claim-blocking, or no spell was
    // fabricated in its absence.
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

    // The bounded single-class druid chassis is still withheld from a
    // multiclass mix -- unchanged, and still the point of this control.
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| is_gated_druid_chassis_record(&e.id)),
        "multiclass Druid must not gain any bounded druid chassis explanation: {:?}",
        computation.explanations
    );

    // Superseded premise, corrected 2026-07-29. The animal-companion records
    // share the `class_chassis.druid.` prefix this control used to reject
    // wholesale, but they are deliberately reachable from a multiclass mix
    // (see `is_animal_companion_record`) and are correct there: in PF1 a
    // Druid 9/Fighter 1 really does have a companion, at effective druid
    // level 9. Rather than silently tolerate the whole family, pin the
    // thing that actually matters -- the companion is keyed to the DRUID class
    // level (9), not to the character's total level (10). If the engine
    // ever started advancing a multiclass character's companion off total
    // level, these values would be those of master level 10 and this fails.
    assert_wolf_companion_stat_block(
        &computation, 9, 8, 9, 6, 2, 18, 4, 59,
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_druid_row_names_level_9_widening() {
    let matrix = seeded_current_truth();
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
