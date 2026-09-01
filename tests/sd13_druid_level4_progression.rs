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

use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

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

// ----- The animal companion is grounded; the prepared divine burden still fires at level 4 -----

#[test]
fn druid_level4_grounds_the_animal_companion_and_holds_the_prepared_divine_posture() {
    let input = load(DRUID_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Superseded premise, corrected 2026-07-29. This test used to require
    // `class_feature.druid.animal_companion.unsupported` to fire claim-blocking
    // at level 4. Commit `ae63aa4c` grounded the animal companion's whole
    // progression from the PCGen corpus at all twenty master levels, so at
    // level 4 the burden is genuinely closed and that blocker is correctly
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
        &computation, 4, 4, 5, 4, 1, 14, 3, 31,
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
    // Druid 4/Fighter 1 really does have a companion, at effective druid
    // level 4. Rather than silently tolerate the whole family, pin the
    // thing that actually matters -- the companion is keyed to the DRUID class
    // level (4), not to the character's total level (5). If the engine
    // ever started advancing a multiclass character's companion off total
    // level, these values would be those of master level 5 and this fails.
    assert_wolf_companion_stat_block(
        &computation, 4, 4, 5, 4, 1, 14, 3, 31,
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-4 widening and Resist Nature's Lure -----

#[test]
fn matrix_druid_row_names_level_4_widening_and_resist_natures_lure() {
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
