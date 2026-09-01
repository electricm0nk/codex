//! SD13-E5 Bard level-5 progression grounding proof.
//!
//! Widens the accepted Bard level-1/level-2/level-3/level-4 spontaneous
//! arcane spell-bearing baseline (`tests/sd13_bard_level1_spell_baseline.rs`,
//! `tests/sd13_bard_base_attack_and_saves.rs`, `tests/sd13_bard_fascinate_dc.rs`,
//! `tests/sd13_bard_level2_progression.rs`, `tests/sd13_bard_level3_progression.rs`,
//! `tests/sd13_bard_level4_progression.rs`) to Bard level 5, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_bard_level` is generalized from `1..=4`
//! to `1..=5` via `MAX_SUPPORTED_BARD_LEVEL = 5`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Bard class table) were read
//! directly before writing any code or test:
//!
//! - level 5 base attack bonus is +3 (`5 * 3 / 4 = 3`), base Reflex/Will are
//!   +4 (good, `5/2+2`), base Fortitude is +1 (poor, `5/3`) — confirmed by
//!   the same formulas already grounded at levels 1-4, not re-derived (all
//!   three save values happen to stay numerically unchanged from level 4, an
//!   integer-division coincidence, not a sign the formulas stopped scaling).
//! - Bardic Knowledge stays `max(5/2, 1) = 2` at level 5, unchanged from
//!   level 4 (`4/2 = 5/2 = 2` under integer division), confirmed via the
//!   same formula, not a new record.
//! - Bardic Performance rounds per day continues to scale: `4 + Charisma
//!   modifier + 2 * (level - 1)` = 4 + 2 + 8 = 14 on the fixture's Charisma
//!   15, up from 12 at level 4, confirmed via the same formula, not a new
//!   record.
//! - Inspire Courage's flat magnitude genuinely CHANGES at level 5: both PF1
//!   CRB primary sources' level-5 "Special" column reads "Inspire courage
//!   +2, lore master 1/day", and the rule's own text ("At 5th level, and
//!   every six bard levels thereafter, this bonus increases by +1") confirms
//!   the increase from +1 to +2 lands EXACTLY at level 5 — re-verified this
//!   cycle rather than trusted from an earlier cycle's "stays +1 through
//!   level 5" framing, which turns out to have been precise (level 4 stays
//!   +1, level 5 is the level the increase lands on).
//! - the Fascinate flat Will-save DC (`10 + level/2 + CHA modifier`) and the
//!   flat affected-creature count (`1 + (level-1)/3`) both already take bard
//!   level as an input variable, so both extend correctly to level 5 with no
//!   re-derivation: DC 14 (10 + 2 + 2) and count 2 (1 + 1) on the fixture —
//!   both numerically unchanged from level 4, an integer-division
//!   coincidence (`4/2 = 5/2` and `3/3 = 4/3`), not a sign either formula
//!   stopped scaling.
//! - Well-Versed (the 2nd-level Bard class feature) and Inspire Competence
//!   (the 3rd-level Bard class feature) both stay granted at level 5, not
//!   re-derived — the same bounded identity/magnitude records already
//!   grounded at levels 2 and 3.
//! - a NEW class feature is gained at 5th level: Lore Master (verified
//!   independently against both primary sources' level-5 "Special" column,
//!   which reads "Inspire courage +2, lore master 1/day"). The rule grants
//!   two capabilities: an at-will take-10-on-Knowledge-checks-with-ranks
//!   capability (no flat magnitude to ground — a resolution-mode toggle, not
//!   a countable resource) and a flat "once per day... take 20 on any
//!   Knowledge skill check" capability. Only the flat 1/day usage-count
//!   magnitude is grounded, as a bounded grant-only identity record
//!   mirroring the Paladin Smite Evil / Wizard Force Missile uses-per-day
//!   idiom; the take-10 and take-20 mechanics themselves are NOT executed
//!   against any actual Knowledge check, since no skill-check-resolution
//!   engine exists anywhere in this codebase.
//!
//! It deliberately does not touch the performance-state/action-economy
//! engine, Countersong, Distraction, Versatile Performance, the Lore Master
//! take-10/take-20 execution mechanic, or the spontaneous spell burden (all
//! stay named-but-unproven, unchanged from level 1/2/3/4), and it does not
//! ground Bard level 6+. It also preserves the accepted Bard level-1/
//! level-2/level-3/level-4 truth (unchanged), the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level4_sd13_deterministic_input.txt");

const BARD_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level5_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";

// ----- Base attack bonus at level 5 -----

#[test]
fn bard_level5_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Bard level 5 3/4-BAB progression (5 * 3 / 4) must equal 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 5 (good Reflex/Will, poor Fortitude) -----

#[test]
fn bard_level5_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(fortitude.value, 1, "Bard level 5 poor Fortitude (5/3) must equal 1");

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(reflex.value, 4, "Bard level 5 good Reflex (5/2+2) must equal 4");

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(will.value, 4, "Bard level 5 good Will (5/2+2) must equal 4");
}

// ----- Bardic Knowledge stays max(level/2, 1) = 2 at level 5 (unchanged from level 4) -----

#[test]
fn bard_level5_bardic_knowledge_stays_two() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 2,
        "Bard level 5 Bardic Knowledge (max(5/2, 1)) must equal 2: {}",
        knowledge.detail
    );
}

// ----- Bardic Performance rounds per day keeps scaling with level -----

#[test]
fn bard_level5_bardic_performance_rounds_per_day_keeps_scaling() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bardic Performance: 4 + CHA modifier at level 1, plus 2
    // additional rounds per day at each level after 1st. Fixture CHA 15 + 2 Human
    // racial (CG-03 fix) -> +3 modifier. At level 5: 4 + 3 + 2 * (5 - 1) = 15.
    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(
        rounds.value, 15,
        "Bard level 5 bardic performance rounds per day must equal 4 + CHA + 2*(level-1) \
         = 4 + 3 + 8 = 15: {}",
        rounds.detail
    );
}

// ----- Inspire Courage genuinely increases to +2 at level 5 -----

#[test]
fn bard_level5_inspire_courage_increases_to_plus_two() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 2,
        "Bard level 5 Inspire Courage magnitude must increase to +2 (PF1 CRB: \"At 5th level, \
         and every six bard levels thereafter, this bonus increases by +1\"): {}",
        inspire_courage.detail
    );
}

// ----- Fascinate DC and affected-creature count extend to level 5 (unchanged from level 4) -----

#[test]
fn bard_level5_fascinate_dc_and_affected_creatures_extend_by_the_same_formulas() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture CHA 15 + 2 Human racial (CG-03 fix) -> +3 modifier. DC = 10 + 5/2 + 3 = 15.
    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 15,
        "Bard level 5 Fascinate DC must equal 10 + (5/2) + 3 = 15: {}",
        dc.detail
    );

    // Affected creatures = 1 + (5-1)/3 = 2.
    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 2,
        "Bard level 5 Fascinate affected-creature count must equal 1 + (5-1)/3 = 2: {}",
        count.detail
    );
}

// ----- Well-Versed and Inspire Competence both stay granted at level 5, not re-derived -----

#[test]
fn bard_level5_keeps_well_versed_and_inspire_competence_grounded() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(
        well_versed.value, 4,
        "Well-Versed must stay the flat +4 bonus at level 5, not re-derived: {}",
        well_versed.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 2,
        "Inspire Competence must stay the flat +2 bonus at level 5, not re-derived: {}",
        inspire_competence.detail
    );
}

// ----- Lore Master is newly granted at level 5 (flat 1/day take-20 count only) -----

#[test]
fn bard_level5_grounds_lore_master_flat_uses_per_day_only() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 1,
        "Lore Master's take-20 usage must ground only the flat 1/day count: {}",
        lore_master.detail
    );
}

// ----- Lore Master is correctly absent below level 5 -----

#[test]
fn bard_level4_lore_master_is_correctly_absent() {
    let input = load(BARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 0,
        "Lore Master must be a correct level-gate absence below level 5: {}",
        lore_master.detail
    );
}

// ----- Only the known bard-namespaced ids appear at level 5 (Lore Master is the only new one) -----

#[test]
fn bard_level5_gains_only_lore_master_as_a_new_class_feature() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let known_bard_ids = [
        "class_chassis.spell_baseline.bard",
        // (v0.6 alpha swarm, risks item 8) bardic-performance-execution's
        // not-performing record is checked unconditionally at every level
        // once a bare Bard input exists, not a new class feature.
        "class_feature.bard.bardic_performance_execution.not_performing",
        "class_chassis.bard.base_attack_bonus",
        "class_chassis.bard.base_save.fortitude",
        "class_chassis.bard.base_save.reflex",
        "class_chassis.bard.base_save.will",
        "class_chassis.bard.bardic_knowledge",
        "class_chassis.bard.bardic_performance_rounds_per_day",
        "class_chassis.bard.inspire_courage_bonus",
        "class_chassis.bard.fascinate_dc",
        "class_chassis.bard.fascinate_affected_creatures",
        WELL_VERSED_ID,
        INSPIRE_COMPETENCE_ID,
        LORE_MASTER_ID,
        "class_feature.bard.soothing_performance",
        // Added by the further SD13-E5 access-ladder slice
        // (tests/sd13_bard_spell_level_thresholds.rs): a
        // spells-per-day-table access record that fires at every supported
        // level; it is not a "Special"-column class feature, so listing it
        // keeps this control accurate without weakening its claim.
        "class_chassis.bard.spontaneous.spell_level_access",
        // Added by the v0.6 Receipt-to-Sheet caster-level slice: the
        // corpus-transcribed caster level (a Bard casts at its full class
        // level -- cr_classes.lst:28
        // BONUS:CASTERLEVEL|Bard|Caster_Level_BL_Stripped_Bard, resolving
        // through cr_classes.lst:24
        // BONUS:VAR|Caster_Level_Bard|CL+Caster_Level_Bonus+CasterLevelBLBard).
        // Like spell_level_access above it fires at every supported level
        // rather than being gained at this one, and it is not a
        // "Special"-column class feature, so listing it keeps this control
        // accurate without weakening its claim.
        "class_chassis.bard.caster_level",
        // The base_spells_per_day family (a further SD13-E5 slice): literal
        // spells-per-day table records, not "Special"-column class features.
        "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_1",
        "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_2",
        "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_3",
        "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_4",
        // The spell_save_dc family (a further SD13-E5 slice): base DC
        // arithmetic records, not "Special"-column class features.
        "class_chassis.bard.spontaneous.spell_save_dc.spell_level_1",
        "class_chassis.bard.spontaneous.spell_save_dc.spell_level_2",
        "class_chassis.bard.spontaneous.spell_save_dc.spell_level_3",
        "class_chassis.bard.spontaneous.spell_save_dc.spell_level_4",
        // The spells_known family (a further SD13-E5 slice): base known-count
        // table records, not "Special"-column class features.
        "class_chassis.bard.spontaneous.spells_known.spell_level_0",
        "class_chassis.bard.spontaneous.spells_known.spell_level_1",
        "class_chassis.bard.spontaneous.spells_known.spell_level_2",
        "class_chassis.bard.spontaneous.spells_known.spell_level_3",
        "class_chassis.bard.spontaneous.spells_known.spell_level_4",
        // The bonus_spells_per_day family (a further SD13-E5 slice):
        // Charisma bonus-slot counts, not "Special"-column class features.
        "class_chassis.bard.spontaneous.bonus_spells_per_day.spell_level_1",
        "class_chassis.bard.spontaneous.bonus_spells_per_day.spell_level_2",
        "class_chassis.bard.spontaneous.bonus_spells_per_day.spell_level_3",
        "class_chassis.bard.spontaneous.bonus_spells_per_day.spell_level_4",
        // The total_spells_per_day family (a further SD13-E5 slice): pure
        // sums of grounded records, not "Special"-column class features.
        "class_chassis.bard.spontaneous.total_spells_per_day.spell_level_1",
        "class_chassis.bard.spontaneous.total_spells_per_day.spell_level_2",
        "class_chassis.bard.spontaneous.total_spells_per_day.spell_level_3",
        "class_chassis.bard.spontaneous.total_spells_per_day.spell_level_4",
    ];
    assert!(
        computation
            .explanations
            .iter()
            .filter(|e| e.id.starts_with("class_chassis.bard.")
                || e.id.starts_with("class_feature.bard.")
                || e.id == "class_chassis.spell_baseline.bard")
            .all(|e| known_bard_ids.contains(&e.id.as_str())
                // SD-34 decisions.md section 18: widened BY CONSTRUCTION, not narrowed --
                // class_feature_grant_consumer now emits real, citation-backed
                // corpus_record ids for Bard (previously wholesale-excluded); this shape
                // carve-out admits them without touching any existing known_bard_ids
                // entry or weakening the exhaustive check for anything else.
                || e.id.starts_with("class_feature.bard.corpus_record.")),
        "Bard level 5 must not gain any bard-namespaced explanation id beyond the \
         already-grounded pillars plus the new Lore Master grant: {:?}",
        computation.explanations
    );
}

// ----- The two existing burden diagnostics still fire at level 5 -----

#[test]
fn bard_level5_still_claim_blocks_performance_execution_and_spontaneous_spell_burdens() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.bard.bardic_performance_execution.rounds_exceeded")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let not_performing = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.bard.bardic_performance_execution.not_performing");
            assert!(
                not_performing.is_some(),
                "level-5 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.bard.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- The accepted Bard level-4 truth is unaffected -----

#[test]
fn bard_level4_truth_is_unchanged_by_this_widening() {
    let input = load(BARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Bard level 4 base attack bonus must stay 3");

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 1,
        "Bard level 4 Inspire Courage magnitude must stay +1 (it only increases at level 5)"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 13, "Bard level 4 bardic performance rounds per day must stay 13");
}

// ----- Negative control retired: level 6 Bard
// was later widened into the supported tranche by
// tests/sd13_bard_level6_progression.rs; the level-7 negative control now
// lives there) -----

#[test]
fn bard_level_6_was_later_widened_into_the_supported_tranche() {
    let level_6 = BARD_LEVEL5_FIXTURE.replace("class:bard:5", "class:bard:6");
    let input = load(&level_6);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")),
        "level-6 Bard was later widened into the supported tranche and must now gain bounded \
         bard chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        computation.explanations.iter().any(|e| e.id == LORE_MASTER_ID),
        "level-6 Bard was later widened and must now carry the Lore Master explanation"
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level5_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID
                || e.id == INSPIRE_COMPETENCE_ID
                || e.id == LORE_MASTER_ID),
        "the Fighter chassis must not surface any bard-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_level5_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL5_FIXTURE.replace(
        "class_level=class:bard:5",
        "class_level=class:bard:5\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID
                || e.id == INSPIRE_COMPETENCE_ID
                || e.id == LORE_MASTER_ID),
        "multiclass Bard must not gain any bounded bard chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-5 widening -----

#[test]
fn matrix_bard_row_names_level_5_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd13_bard_level5_progression"),
        "bard row must cite the live SD13-E5 level-5 proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 5") || note.contains("level-5"),
        "bard partial note must name the level-5 widening: {note}"
    );
    assert!(
        note.contains("Lore Master") || note.contains("lore master") || note.contains("Lore master"),
        "bard partial note must name the newly grounded Lore Master feature: {note}"
    );
}
