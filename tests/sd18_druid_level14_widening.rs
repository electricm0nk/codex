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

use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

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

// ----- The animal companion is grounded; the prepared divine burden still fires at level 14 -----

#[test]
fn druid_level14_grounds_the_animal_companion_and_holds_the_prepared_divine_posture() {
    let input = load(DRUID_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Superseded premise, corrected 2026-07-29. This test used to require
    // `class_feature.druid.animal_companion.unsupported` to fire claim-blocking
    // at level 14. Commit `ae63aa4c` grounded the animal companion's whole
    // progression from the PCGen corpus at all twenty master levels, so at
    // level 14 the burden is genuinely closed and that blocker is correctly
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
        &computation, 14, 12, 12, 8, 4, 20, 4, 87,
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

// ----- Level 16 was widened in; the boundary control moves to level 21 -----
// (Superseded boundary: a still further SD18 slice (the loop's FIFTH §3.2
// level-15 landing, tests/sd18_druid_level15_widening.rs) widened
// MAX_SUPPORTED_DRUID_LEVEL from 14 to 15, so this file's own
// negative-control boundary moves from 15 to 16.)

//
// Final move, 2026-07-29: commit `72d83e75` widened Druid all the way to the
// PF1 cap -- MAX_SUPPORTED_DRUID_LEVEL and class_tables.rs's own CLASS_META
// both 15 -> 20 -- making Druid the 10th fully-computing class. Level 16 is
// therefore genuinely promoted now, so the absence assertion is superseded and
// is replaced below by the widened truth it was standing in for. The boundary
// control itself keeps its job and makes its last possible move, to level 21:
// PF1 has no 21st character level (`MAXLEVEL:20`, core_rulebook/cr_classes.lst
// CLASS:Druid), so that is a pure implementation-gate check -- the same idiom
// every other already-capped class uses (`bard_level_21_is_not_promoted...`,
// `barbarian_level_21_is_not_promoted...`, `sorcerer_level_21_is_not_promoted...`).
#[test]
fn druid_level_16_was_later_widened_into_the_supported_tranche() {
    // Superseded premise, corrected 2026-07-29 (see the note above). A
    // level-16 Druid is now promoted, correctly and by design, so this control
    // pins the widened truth rather than asserting an absence -- exactly the
    // `druid_level_3_was_later_widened_into_the_supported_tranche` /
    // `druid_level_4_...` precedent already established in
    // tests/sd13_druid_level2_progression.rs.
    //
    // Every value is re-derived from the corpus, not copied from the engine
    // (`core_rulebook/cr_classes.lst`, CLASS:Druid, MAXLEVEL:20): base attack
    // `classlevel("APPLIEDAS=NONEPIC")*3/4` = 16*3/4 = 12; good Fortitude and
    // Will `classlevel/2+2` = 16/2+2 = 10; poor Reflex `classlevel/3` = 16/3 =
    // 5 (an integer-division value that must be checked, not assumed to keep
    // climbing). Wild Empathy is the druid's own level plus the Charisma
    // modifier (+1 on this fixture) = 17; Nature Sense stays the flat PF1 CRB
    // +2.
    let level_16 = DRUID_LEVEL14_FIXTURE.replace("class:druid:14", "class:druid:16");
    let input = load(&level_16);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Druid level 16 3/4-BAB progression (16 * 3 / 4) must equal 12: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Druid level 16 good Fortitude (16/2+2) must equal 10: {}",
        fortitude.detail
    );
    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Druid level 16 poor Reflex (16/3) must equal 5: {}",
        reflex.detail
    );
    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(
        will.value, 10,
        "Druid level 16 good Will (16/2+2) must equal 10: {}",
        will.detail
    );

    let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
    assert_eq!(
        wild_empathy.value, 17,
        "Druid level 16 wild empathy must equal druid level + Cha modifier (16 + 1): {}",
        wild_empathy.detail
    );
    let nature_sense = explanation(&computation, "class_chassis.druid.nature_sense");
    assert_eq!(
        nature_sense.value, 2,
        "Druid level 16 Nature Sense must stay the flat PF1 CRB +2 bonus: {}",
        nature_sense.detail
    );

    // The companion advances with its master to master level 16 -- 13 Hit Dice
    // by the corpus table, and the natural-armor/Strength advances that ride on
    // the same master level.
    assert_wolf_companion_stat_block(&computation, 16, 13, 13, 8, 4, 22, 6, 94);
}

#[test]
fn druid_level_21_is_not_promoted_by_this_slice() {
    // The boundary control's new home. PF1 has no 21st character level, so
    // this is a pure implementation-gate check that the level range really is
    // bounded rather than open-ended.
    let level_21 = DRUID_LEVEL14_FIXTURE.replace("class:druid:14", "class:druid:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);

    // Note this deliberately uses the raw prefixes rather than
    // `is_gated_druid_chassis_record`: at level 21 the animal companion must be
    // absent too, because the corpus companion progression is only defined
    // across master levels 1-20. This is the one place the companion family is
    // NOT excused.
    assert!(
        !computation.explanations.iter().any(|e| e
            .id
            .starts_with("class_chassis.druid.")
            || e.id.starts_with("class_feature.druid.")
            || e.id == "class_chassis.spell_baseline.druid"),
        "level-21 Druid must not gain any bounded druid explanation: {:?}",
        computation.explanations
    );

    // ...and the companion burden falls back to its catch-all claim-blocker
    // rather than fabricating a 21st-level master's companion or panicking on
    // the companion table's own domain guard.
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_feature.druid.animal_companion.unsupported" && d.claim_blocking
        ),
        "an out-of-domain druid level must fall through to the catch-all \
         animal-companion blocker: {:?}",
        computation.diagnostics
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
    // Druid 14/Fighter 1 really does have a companion, at effective druid
    // level 14. Rather than silently tolerate the whole family, pin the
    // thing that actually matters -- the companion is keyed to the DRUID class
    // level (14), not to the character's total level (15). If the engine
    // ever started advancing a multiclass character's companion off total
    // level, these values would be those of master level 15 and this fails.
    assert_wolf_companion_stat_block(
        &computation, 14, 12, 12, 8, 4, 20, 4, 87,
    );

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_druid_row_names_level_14_widening() {
    let matrix = seeded_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid progression_and_spell_burden row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(druid.support_state, SupportState::Supported);
    assert_eq!(druid.evidence_tier, EvidenceTier::ProductVisible);
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
