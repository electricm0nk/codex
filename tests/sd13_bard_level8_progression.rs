//! SD13-E5 Bard level-8 progression grounding proof.
//!
//! Widens the accepted Bard level-1/level-2/level-3/level-4/level-5/level-6/
//! level-7 spontaneous arcane spell-bearing baseline
//! (`tests/sd13_bard_level1_spell_baseline.rs`,
//! `tests/sd13_bard_base_attack_and_saves.rs`, `tests/sd13_bard_fascinate_dc.rs`,
//! `tests/sd13_bard_level2_progression.rs`, `tests/sd13_bard_level3_progression.rs`,
//! `tests/sd13_bard_level4_progression.rs`, `tests/sd13_bard_level5_progression.rs`,
//! `tests/sd13_bard_level6_progression.rs`, `tests/sd13_bard_level7_progression.rs`)
//! to Bard level 8, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_bard_level` is generalized from `1..=7`
//! to `1..=8` via `MAX_SUPPORTED_BARD_LEVEL = 8`). Both PF1 CRB primary
//! sources (d20pfsrd and the legacy.aonprd.com Bard class table mirror) were
//! read directly before writing any code or test:
//!
//! - level 8 base attack bonus is +6 (`8 * 3 / 4 = 6`), up from +5 at level
//!   7 — a genuinely new value. The class table's own iterative-attack
//!   notation "+6/+1" is not modeled anywhere in this codebase, only the
//!   flat base value, mirroring the Cleric level-8 precedent.
//! - base Fortitude stays +2 (poor, `8/3 = 2`), an integer-division
//!   coincidence with level 7, re-verified against the raw class table row
//!   rather than assumed. Base Reflex and Will both GENUINELY rise to +6
//!   (good, `8/2+2 = 6`), up from +5 at level 7.
//! - Bardic Knowledge GENUINELY rises to 4 (`max(8/2, 1) = 4`), up from 3 at
//!   level 7.
//! - Bardic Performance rounds per day continues to scale: `4 + Charisma
//!   modifier + 2 * (level - 1)` = 4 + 2 + 14 = 20 on the fixture's Charisma
//!   15, up from 18 at level 7, confirmed via the same formula, not a new
//!   record.
//! - Inspire Courage's flat magnitude stays +2 at level 8 (PF1 Core
//!   Rulebook: the next increase does not land until bard level 11),
//!   re-verified rather than assumed.
//! - the Fascinate flat Will-save DC (`10 + level/2 + CHA modifier`)
//!   GENUINELY rises to 16 at level 8 (10 + 4 + 2), up from 15 at level 7.
//!   The flat affected-creature count (`1 + (level-1)/3`) stays 3 at level 8
//!   (`1 + 7/3 = 1 + 2 = 3`, integer division) — confirmed by direct
//!   arithmetic against the primary source rule text before writing any
//!   code, an integer-division coincidence with level 7's value, not a sign
//!   the formula stopped scaling.
//! - Well-Versed (2nd-level) and Lore Master (5th-level) both stay granted
//!   at level 8, not re-derived.
//! - Inspire Competence's flat magnitude stays +3 at level 8 (PF1 Core
//!   Rulebook: "This bonus increases by +1 for every four levels the bard
//!   has attained beyond 3rd (+3 at 7th, +4 at 11th...)" — the next increase
//!   does not land until bard level 11), re-verified rather than assumed.
//! - the PF1 CRB Bard class table's level-8 "Special" column (verified
//!   independently against both primary sources) reads "Dirge of doom" — a
//!   genuinely NEW bardic-performance type, checked and confirmed NOT
//!   flat/identity-shaped: Dirge of Doom requires both the same
//!   performance-state engine already left ungrounded (start/maintain
//!   action economy, round tracking/consumption) AND a fear/shaken-condition
//!   resolution engine, neither of which exists in this codebase. It is
//!   deliberately left named-but-unproven this bounded slice, mirroring the
//!   Suggestion / Countersong / Distraction precedent exactly: no execution
//!   engine is fabricated for it, and it gains no explanation record.
//!
//! It deliberately does not touch the performance-state/action-economy
//! engine, Countersong, Distraction, Versatile Performance (either its
//! 2nd-level or 6th-level grant), Suggestion, Dirge of Doom, the Lore Master
//! take-10/take-20 execution mechanic, or the spontaneous spell burden (all
//! stay named-but-unproven, unchanged from level 1 through level 7), and it
//! does not ground Bard level 9+. It also preserves the accepted Bard
//! level-1 through level-7 truth (unchanged), the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};

const BARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level7_sd13_deterministic_input.txt");

const BARD_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level8_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";

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

// ----- Base attack bonus at level 8 -----

#[test]
fn bard_level8_base_attack_bonus_rises_by_the_same_formula() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Bard level 8 3/4-BAB progression (8 * 3 / 4) must genuinely rise to 6, up from 5 at \
         level 7: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8: Fortitude stays, Reflex/Will genuinely rise -----

#[test]
fn bard_level8_base_saves_fortitude_stays_and_reflex_will_rise() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 2,
        "Bard level 8 poor Fortitude (8/3) must stay 2, unchanged from level 7, an \
         integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Bard level 8 good Reflex (8/2+2) must genuinely rise to 6, up from 5 at level 7"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 6,
        "Bard level 8 good Will (8/2+2) must genuinely rise to 6, up from 5 at level 7"
    );
}

// ----- Bardic Knowledge genuinely rises at level 8 -----

#[test]
fn bard_level8_bardic_knowledge_genuinely_rises() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 4,
        "Bard level 8 Bardic Knowledge (max(8/2, 1)) must genuinely rise to 4, up from 3 at \
         level 7: {}",
        knowledge.detail
    );
}

// ----- Bardic Performance rounds per day keeps scaling with level -----

#[test]
fn bard_level8_bardic_performance_rounds_per_day_keeps_scaling() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bardic Performance: 4 + CHA modifier at level 1, plus 2
    // additional rounds per day at each level after 1st. Fixture CHA 15 + 2 Human
    // racial (CG-03 fix) -> +3 modifier. At level 8: 4 + 3 + 2 * (8 - 1) = 21.
    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(
        rounds.value, 21,
        "Bard level 8 bardic performance rounds per day must equal 4 + CHA + 2*(level-1) \
         = 4 + 3 + 14 = 21: {}",
        rounds.detail
    );
}

// ----- Inspire Courage stays +2 at level 8 (next increase is level 11) -----

#[test]
fn bard_level8_inspire_courage_stays_plus_two() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 2,
        "Bard level 8 Inspire Courage magnitude must stay +2 (PF1 CRB: the next increase does \
         not land until bard level 11): {}",
        inspire_courage.detail
    );
}

// ----- Fascinate DC genuinely rises; affected-creature count stays unchanged -----

#[test]
fn bard_level8_fascinate_dc_rises_and_affected_creatures_stays_unchanged() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture CHA 15 + 2 Human racial (CG-03 fix) -> +3 modifier. DC = 10 + 8/2 + 3 = 17, a
    // genuine rise from level 7's 16.
    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 17,
        "Bard level 8 Fascinate DC must genuinely rise to 10 + (8/2) + 3 = 17, up from 16 at \
         level 7: {}",
        dc.detail
    );

    // Affected creatures = 1 + (8-1)/3 = 1 + 7/3 = 3, unchanged from level 7's 3
    // (an integer-division coincidence, confirmed by direct arithmetic against
    // the primary source rule text, not a sign the formula stopped scaling).
    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 3,
        "Bard level 8 Fascinate affected-creature count must stay 1 + (8-1)/3 = 3, unchanged \
         from level 7, an integer-division coincidence: {}",
        count.detail
    );
}

// ----- Well-Versed and Lore Master stay granted at level 8 -----

#[test]
fn bard_level8_keeps_well_versed_and_lore_master_grounded() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(
        well_versed.value, 4,
        "Well-Versed must stay the flat +4 bonus at level 8, not re-derived: {}",
        well_versed.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 1,
        "Lore Master must stay the flat 1/day take-20 count at level 8, not re-derived: {}",
        lore_master.detail
    );
}

// ----- Inspire Competence stays +3 at level 8 (next increase is level 11) -----

#[test]
fn bard_level8_inspire_competence_stays_plus_three() {
    let input = load(BARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 3,
        "Bard level 8 Inspire Competence must stay +3 (PF1 CRB: the next increase does not \
         land until bard level 11), unchanged from level 7: {}",
        inspire_competence.detail
    );
}

// ----- Only the known bard-namespaced ids appear at level 8 (Dirge of Doom is NOT grounded) -----

#[test]
fn bard_level8_gains_no_new_bard_namespaced_explanation_id() {
    let input = load(BARD_LEVEL8_FIXTURE);
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
        "Bard level 8 must not gain any bard-namespaced explanation id beyond the \
         already-grounded pillars (Dirge of Doom must not be fabricated): {:?}",
        computation.explanations
    );
}

// ----- The two existing burden diagnostics still fire at level 8 -----

#[test]
fn bard_level8_still_claim_blocks_performance_execution_and_spontaneous_spell_burdens() {
    let input = load(BARD_LEVEL8_FIXTURE);
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
                "level-8 Bard must ground an honest not-performing record when no \
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

// ----- The accepted Bard level-7 truth is unaffected -----

#[test]
fn bard_level7_truth_is_unchanged_by_this_widening() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 5, "Bard level 7 base attack bonus must stay 5");

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Bard level 7 base Reflex save must stay 5, unaffected by the level-8 widening"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 3,
        "Bard level 7 Fascinate affected-creature count must stay 3, unaffected by the level-8 \
         widening"
    );
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn bard_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = BARD_LEVEL8_FIXTURE.replace("class:bard:8", "class:bard:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")),
        "level-9 Bard is now recognized by the later level-9 widening slice \
         (tests/sd13_bard_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level8_recognition() {
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
fn multiclass_bard_level8_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL8_FIXTURE.replace(
        "class_level=class:bard:8",
        "class_level=class:bard:8\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_bard_row_names_level_8_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd13_bard_level8_progression"),
        "bard row must cite the live SD13-E5 level-8 proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 8") || note.contains("level-8"),
        "bard partial note must name the level-8 widening: {note}"
    );
    assert!(
        note.contains("Dirge of Doom") || note.contains("Dirge of doom"),
        "bard partial note must name the newly-discovered, deliberately unproven Dirge of Doom \
         class feature: {note}"
    );
}
