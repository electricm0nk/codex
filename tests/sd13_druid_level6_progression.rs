//! SD13-E5 Druid level-6 progression grounding proof.
//!
//! Widens the accepted Druid level-1/level-2/level-3/level-4/level-5 prepared divine
//! spell-bearing baseline (`tests/sd13_druid_level1_spell_baseline.rs`,
//! `tests/sd13_druid_base_attack_and_saves.rs`, `tests/sd13_druid_level2_progression.rs`,
//! `tests/sd13_druid_level3_progression.rs`, `tests/sd13_druid_level4_progression.rs`,
//! `tests/sd13_druid_level5_progression.rs`) to druid level 6, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_druid_level` is generalized from `1..=5`
//! to `1..=6` via `MAX_SUPPORTED_DRUID_LEVEL = 6`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Druid class table) were read directly before
//! writing any code or test:
//!
//! - level 6 base attack bonus is +4 (`6 * 3 / 4 = 4`, the Druid's own 3/4-BAB
//!   progression, the same shape as Rogue/Monk/Cleric/Bard) and base saves are
//!   +5 Fortitude (good, `6/2+2 = 5`), +2 Reflex (poor, `6/3 = 2`), +5 Will
//!   (good, `6/2+2 = 5`) — both genuinely new values, up from +3/+1/+4 at
//!   level 5, confirmed by the same formulas already grounded at levels 1-5,
//!   not re-derived.
//! - Wild Empathy's modifier is level-generic by construction and grounds
//!   correctly to 7 (6 + Charisma modifier 1) at level 6, via the same
//!   formula, not a new record.
//! - Nature Sense stays the flat, level-independent PF1 CRB +2 bonus,
//!   confirmed unchanged at level 6 via the same formula, not a new record.
//! - the nature-bond choice recognition is not level-gated; it still fires at
//!   level 6 for the same fixture selection
//!   (`choice:druid_nature_bond -> bond:animal_companion`).
//! - Woodland Stride (granted starting at level 2), Trackless Step (granted
//!   starting at level 3), and Resist Nature's Lure (granted starting at
//!   level 4) all stay granted at level 6, not re-derived, grounded as the
//!   same bounded identity/flat-magnitude records already grounded at levels
//!   2/3/4/5.
//! - the PF1 CRB Druid class table's level-6 "Special" column reads "Wild
//!   shape (2/day)" (verified independently against both primary sources).
//!   This was checked per the operator brief's explicit instruction to verify
//!   whether Druid gains an actual new class feature at 6th level, and
//!   confirmed NOT a genuinely separable flat/identity-shaped element: the
//!   rule text bundles the "2/day" frequency increase together with a form-
//!   list expansion (a druid can now wild shape into a Large or Tiny animal
//!   or a Small elemental) and a functioning-level upgrade (the animal form
//!   now functions as beast shape II, the elemental form as elemental body
//!   I) — none of which exist in this codebase's engine-free record set, and
//!   none of which are separable from the "2/day" numeral without misrepresenting
//!   the bundled feature as fully flat. Wild Shape (including its level-6
//!   frequency increase and form-list expansion) is therefore deliberately
//!   left entirely named-but-unproven, exactly as it was at level 4/5 — no
//!   explanation or diagnostic record is fabricated for it this slice either.
//!
//! It deliberately does not touch the animal companion stat block/
//! advancement/link-share-spells burden, the Wild Shape execution burden (its
//! frequency increase and form-list expansion checked and confirmed not
//! flat), or the prepared divine spell posture burden (all stay
//! named-but-unproven, unchanged from level 1), and it does not ground Druid
//! level 7+. It also preserves the accepted Druid level-1/level-2/level-3/
//! level-4/level-5 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const DRUID_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level5_sd13_deterministic_input.txt");

const DRUID_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level6_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DRUID_WOODLAND_STRIDE_ID: &str = "class_feature.druid.woodland_stride";
const DRUID_TRACKLESS_STEP_ID: &str = "class_feature.druid.trackless_step";
const DRUID_RESIST_NATURES_LURE_ID: &str = "class_feature.druid.resist_natures_lure";

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

// ----- Base attack bonus at level 6 -----

#[test]
fn druid_level6_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 4,
        "Druid level 6 3/4-BAB progression (6 * 3 / 4) must equal 4: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 6 (good Fortitude/Will, poor Reflex) -----

#[test]
fn druid_level6_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 5, "Druid level 6 good Fortitude (6/2+2) must equal 5");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(reflex.value, 2, "Druid level 6 poor Reflex (6/3) must equal 2");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 5, "Druid level 6 good Will (6/2+2) must equal 5");
}

// ----- Wild Empathy at level 6 -----

#[test]
fn druid_level6_wild_empathy_modifier_is_grounded_by_the_same_formula() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture: Charisma 12 -> modifier +1. Druid level 6 + Cha modifier +1 = 7.
    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 7,
        "Druid level 6 wild empathy modifier must equal druid level + Cha modifier (6 + 1 = 7): {}",
        wild_empathy.detail
    );
}

// ----- Nature Sense at level 6 (flat, level-independent) -----

#[test]
fn druid_level6_nature_sense_bonus_is_unchanged() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(
        nature_sense.value, 2,
        "Druid level 6 Nature Sense must stay the flat PF1 CRB +2 bonus: {}",
        nature_sense.detail
    );
}

// ----- Nature bond choice recognition at level 6 -----

#[test]
fn druid_level6_still_recognizes_nature_bond_choice() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.druid.nature_bond_choice"),
        "level-6 Druid must still recognize the nature-bond choice: {:?}",
        computation.explanations
    );
}

// ----- Woodland Stride / Trackless Step / Resist Nature's Lure stay granted at level 6 -----

#[test]
fn druid_level6_keeps_woodland_stride_trackless_step_and_resist_natures_lure_grounded() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let woodland_stride = explanation(&computation, DRUID_WOODLAND_STRIDE_ID);
    assert_eq!(
        woodland_stride.value, 0,
        "Woodland Stride must carry no fabricated mechanical value at level 6: {}",
        woodland_stride.detail
    );
    assert!(
        woodland_stride.detail.contains("granted"),
        "Woodland Stride detail at level 6 must state it is granted, not absent: {}",
        woodland_stride.detail
    );

    let trackless_step = explanation(&computation, DRUID_TRACKLESS_STEP_ID);
    assert_eq!(
        trackless_step.value, 0,
        "Trackless Step must carry no fabricated mechanical value at level 6: {}",
        trackless_step.detail
    );
    assert!(
        trackless_step.detail.contains("granted"),
        "Trackless Step detail at level 6 must state it is granted, not absent: {}",
        trackless_step.detail
    );

    let resist_natures_lure = explanation(&computation, DRUID_RESIST_NATURES_LURE_ID);
    assert_eq!(
        resist_natures_lure.value, 4,
        "Resist Nature's Lure must stay the flat PF1 CRB +4 magnitude at level 6, not \
         re-derived: {}",
        resist_natures_lure.detail
    );
    assert!(
        resist_natures_lure.detail.to_lowercase().contains("granted"),
        "Resist Nature's Lure detail at level 6 must state it is granted, not absent: {}",
        resist_natures_lure.detail
    );
}

// ----- Wild Shape must not be fabricated at level 6 -----

#[test]
fn druid_level6_does_not_fabricate_wild_shape_execution() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("wild_shape")),
        "level-6 Druid must not fabricate any Wild Shape explanation record, despite the class \
         table's level-6 \"Wild shape (2/day)\" Special-column entry — the frequency increase is \
         bundled with a non-flat form-list expansion and functioning-level upgrade, so it stays \
         named-but-unproven: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("wild_shape")),
        "level-6 Druid must not fabricate any Wild Shape diagnostic: {:?}",
        computation.diagnostics
    );
}

// ----- The animal companion is grounded; the prepared divine burden still fires at level 6 -----

#[test]
fn druid_level6_grounds_the_animal_companion_and_holds_the_prepared_divine_posture() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Superseded premise, corrected 2026-07-29. This test used to require
    // `class_feature.druid.animal_companion.unsupported` to fire claim-blocking
    // at level 6. Commit `ae63aa4c` grounded the animal companion's whole
    // progression from the PCGen corpus at all twenty master levels, so at
    // level 6 the burden is genuinely closed and that blocker is correctly
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
        &computation, 6, 6, 6, 5, 2, 16, 3, 45,
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

// ----- The chassis recognition record is still present at level 6 -----

#[test]
fn druid_level6_still_recognizes_the_spell_bearing_baseline() {
    let input = load(DRUID_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.druid"),
        "level-6 Druid must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the accepted Druid level-5 truth is unaffected -----

#[test]
fn druid_level5_truth_is_unchanged_by_this_widening() {
    let input = load(DRUID_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Druid level 5 base attack bonus must stay 3");

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(wild_empathy.value, 6, "Druid level 5 wild empathy modifier must stay 6");
}

// ----- Negative control: level 7 was later widened into the supported tranche -----

#[test]
fn druid_level_7_was_later_widened_into_the_supported_tranche() {
    let level_7 = DRUID_LEVEL6_FIXTURE.replace("class:druid:6", "class:druid:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.druid.base_attack_bonus"),
        "level-7 Druid was later widened into the supported tranche by \
         tests/sd13_druid_level7_progression.rs — the negative-control coverage for level 8 now \
         lives there: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_level6_recognition() {
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
fn multiclass_druid_level6_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL6_FIXTURE.replace(
        "class_level=class:druid:6",
        "class_level=class:druid:6\nclass_level=class:fighter:1",
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
    // Druid 6/Fighter 1 really does have a companion, at effective druid
    // level 6. Rather than silently tolerate the whole family, pin the
    // thing that actually matters -- the companion is keyed to the DRUID class
    // level (6), not to the character's total level (7). If the engine
    // ever started advancing a multiclass character's companion off total
    // level, these values would be those of master level 7 and this fails.
    assert_wolf_companion_stat_block(
        &computation, 6, 6, 6, 5, 2, 16, 3, 45,
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-6 widening -----

#[test]
fn matrix_druid_row_names_level_6_widening() {
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
        druid.grounding_ref.contains("sd13_druid_level6_progression"),
        "druid row must cite the live SD13-E5 level-6 proof surface: {}",
        druid.grounding_ref
    );
    let note = druid.blocker_or_lossiness_note;
    assert!(
        note.contains("level 6") || note.contains("level-6"),
        "druid partial note must name the level-6 widening: {note}"
    );
}
