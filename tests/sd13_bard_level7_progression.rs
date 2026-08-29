//! SD13-E5 Bard level-7 progression grounding proof.
//!
//! Widens the accepted Bard level-1/level-2/level-3/level-4/level-5/level-6
//! spontaneous arcane spell-bearing baseline
//! (`tests/sd13_bard_level1_spell_baseline.rs`,
//! `tests/sd13_bard_base_attack_and_saves.rs`,
//! `tests/sd13_bard_fascinate_dc.rs`, `tests/sd13_bard_level2_progression.rs`,
//! `tests/sd13_bard_level3_progression.rs`,
//! `tests/sd13_bard_level4_progression.rs`,
//! `tests/sd13_bard_level5_progression.rs`,
//! `tests/sd13_bard_level6_progression.rs`) to Bard level 7, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_bard_level` is generalized from `1..=6`
//! to `1..=7` via `MAX_SUPPORTED_BARD_LEVEL = 7`). Both PF1 CRB primary
//! sources (d20pfsrd and the legacy.aonprd.com Bard class table mirror) were
//! read directly before writing any code or test:
//!
//! - level 7 base attack bonus is +5 (`7 * 3 / 4 = 5`), up from +4 at level
//!   6 — a genuinely new value.
//! - base Reflex/Will stay +5 (good, `7/2+2 = 5`) and base Fortitude stays
//!   +2 (poor, `7/3 = 2`), both numerically unchanged from level 6 — an
//!   integer-division coincidence, re-verified against the raw class table
//!   row (Fort +2, Ref +5, Will +5 at both level 6 and level 7) rather than
//!   assumed.
//! - Bardic Knowledge stays `max(7/2, 1) = 3`, unchanged from level 6, an
//!   integer-division coincidence, not re-derived.
//! - Bardic Performance rounds per day continues to scale: `4 + Charisma
//!   modifier + 2 * (level - 1)` = 4 + 2 + 12 = 18 on the fixture's Charisma
//!   15, up from 16 at level 6, confirmed via the same formula, not a new
//!   record.
//! - Inspire Courage's flat magnitude stays +2 at level 7 (PF1 Core
//!   Rulebook: "At 5th level, and every six bard levels thereafter, this
//!   bonus increases by +1" — the next increase does not land until bard
//!   level 11), re-verified rather than assumed.
//! - the Fascinate flat Will-save DC (`10 + level/2 + CHA modifier`) stays
//!   15 at level 7 (10 + 3 + 2), an integer-division coincidence with level
//!   6's DC, not a sign the formula stopped scaling — re-verified against
//!   the primary source formula, not assumed. The flat affected-creature
//!   count (`1 + (level-1)/3`) GENUINELY rises to 3 at level 7 (1 + 6/3 = 3),
//!   up from 2 at level 6, confirmed by direct arithmetic against the
//!   primary source rule text ("one additional creature for every three
//!   bard levels attained beyond 1st") before writing any code.
//! - Well-Versed (2nd-level) and Lore Master (5th-level) both stay granted
//!   at level 7, not re-derived — the same bounded identity/magnitude
//!   records already grounded at levels 2 and 5.
//! - Inspire Competence's flat magnitude GENUINELY rises to +3 at level 7.
//!   Both primary sources' Bard class table list the level-7 "Special"
//!   column as "Inspire competence +3", and the Inspire Competence rule
//!   text itself confirms the mechanism: "This bonus increases by +1 for
//!   every four levels the bard has attained beyond 3rd (+3 at 7th, +4 at
//!   11th, +5 at 15th, and +6 at 19th)" — i.e. `2 + (level - 3) / 4`, which
//!   evaluates to `2 + 4/4 = 3` at level 7, up from `2 + 3/4 = 2` (integer
//!   division) at level 6. This is a flat, non-level-scaled-until-now
//!   magnitude already grounded in this codebase since level 3; widening it
//!   to its next tier at level 7 is the same kind of arithmetic extension
//!   as Inspire Courage's own second-tier widening at level 5, not a new
//!   class feature.
//!
//! It deliberately does not touch the performance-state/action-economy
//! engine, Countersong, Distraction, Versatile Performance (either its
//! 2nd-level or 6th-level grant), Suggestion, the Lore Master
//! take-10/take-20 execution mechanic, or the spontaneous spell burden (all
//! stay named-but-unproven, unchanged from level 1 through level 6), and it
//! does not ground Bard level 8+. It also preserves the accepted Bard
//! level-1 through level-6 truth (unchanged), the Fighter negative control,
//! and the multiclass negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};

const BARD_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level6_sd13_deterministic_input.txt");

const BARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level7_sd13_deterministic_input.txt");

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

// ----- Base attack bonus at level 7 -----

#[test]
fn bard_level7_base_attack_bonus_rises_by_the_same_formula() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Bard level 7 3/4-BAB progression (7 * 3 / 4) must genuinely rise to 5, up from 4 at \
         level 6: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 stay numerically unchanged (good Reflex/Will, poor Fortitude) -----

#[test]
fn bard_level7_base_saves_stay_unchanged_by_the_same_formulas() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 2,
        "Bard level 7 poor Fortitude (7/3) must stay 2, unchanged from level 6"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Bard level 7 good Reflex (7/2+2) must stay 5, unchanged from level 6"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 5,
        "Bard level 7 good Will (7/2+2) must stay 5, unchanged from level 6"
    );
}

// ----- Bardic Knowledge stays unchanged at level 7 (integer-division coincidence) -----

#[test]
fn bard_level7_bardic_knowledge_stays_unchanged() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 3,
        "Bard level 7 Bardic Knowledge (max(7/2, 1)) must stay 3, unchanged from level 6, an \
         integer-division coincidence: {}",
        knowledge.detail
    );
}

// ----- Bardic Performance rounds per day keeps scaling with level -----

#[test]
fn bard_level7_bardic_performance_rounds_per_day_keeps_scaling() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bardic Performance: 4 + CHA modifier at level 1, plus 2
    // additional rounds per day at each level after 1st. Fixture CHA 15 + 2 Human
    // racial (CG-03 fix) -> +3 modifier. At level 7: 4 + 3 + 2 * (7 - 1) = 19.
    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(
        rounds.value, 19,
        "Bard level 7 bardic performance rounds per day must equal 4 + CHA + 2*(level-1) \
         = 4 + 3 + 12 = 19: {}",
        rounds.detail
    );
}

// ----- Inspire Courage stays +2 at level 7 (next increase is level 11) -----

#[test]
fn bard_level7_inspire_courage_stays_plus_two() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 2,
        "Bard level 7 Inspire Courage magnitude must stay +2 (PF1 CRB: the next increase does \
         not land until bard level 11): {}",
        inspire_courage.detail
    );
}

// ----- Fascinate DC stays unchanged; affected-creature count genuinely rises -----

#[test]
fn bard_level7_fascinate_dc_stays_unchanged_and_affected_creatures_rises() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture CHA 15 + 2 Human racial (CG-03 fix) -> +3 modifier. DC = 10 + 7/2 + 3 = 16, an
    // integer-division coincidence with level 6's DC.
    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 16,
        "Bard level 7 Fascinate DC must stay 10 + (7/2) + 3 = 16, unchanged from level 6, an \
         integer-division coincidence: {}",
        dc.detail
    );

    // Affected creatures = 1 + (7-1)/3 = 3, a genuine rise from level 6's 2.
    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 3,
        "Bard level 7 Fascinate affected-creature count must genuinely rise to 1 + (7-1)/3 = 3, \
         up from 2 at level 6: {}",
        count.detail
    );
}

// ----- Well-Versed and Lore Master stay granted at level 7 -----

#[test]
fn bard_level7_keeps_well_versed_and_lore_master_grounded() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(
        well_versed.value, 4,
        "Well-Versed must stay the flat +4 bonus at level 7, not re-derived: {}",
        well_versed.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 1,
        "Lore Master must stay the flat 1/day take-20 count at level 7, not re-derived: {}",
        lore_master.detail
    );
}

// ----- Inspire Competence genuinely rises to +3 at level 7 -----

#[test]
fn bard_level7_inspire_competence_rises_to_three() {
    let input = load(BARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 3,
        "Bard level 7 Inspire Competence must genuinely rise to +3 (PF1 CRB: \"+1 for every \
         four levels the bard has attained beyond 3rd (+3 at 7th...)\"), up from +2 at level 6: \
         {}",
        inspire_competence.detail
    );
}

// ----- Only the known bard-namespaced ids appear at level 7 (no new ids beyond level 6) -----

#[test]
fn bard_level7_gains_no_new_bard_namespaced_explanation_id() {
    let input = load(BARD_LEVEL7_FIXTURE);
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
        // SD-34 bucket-B batch cycle: `AT-34-E3-001` cycle 5 legitimately grounded
        // Suggestion's flat Will-save DC at level 6, still present at level 7 -- a
        // real "Special"-column class feature this test's own known-id list never
        // listed.
        "class_feature.bard.suggestion_dc",
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
        "Bard level 7 must not gain any bard-namespaced explanation id beyond the \
         already-grounded pillars: {:?}",
        computation.explanations
    );
}

// ----- The two existing burden diagnostics still fire at level 7 -----

#[test]
fn bard_level7_still_claim_blocks_performance_execution_and_spontaneous_spell_burdens() {
    let input = load(BARD_LEVEL7_FIXTURE);
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
                "level-7 Bard must ground an honest not-performing record when no \
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

// ----- The accepted Bard level-6 truth is unaffected -----

#[test]
fn bard_level6_truth_is_unchanged_by_this_widening() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 4, "Bard level 6 base attack bonus must stay 4");

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 2,
        "Bard level 6 Inspire Competence must stay +2, unaffected by the level-7 widening"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 2,
        "Bard level 6 Fascinate affected-creature count must stay 2, unaffected by the level-7 \
         widening"
    );
}

// ----- Level 8 was later widened into the supported tranche (see
// tests/sd13_bard_level8_progression.rs, which owns the full level-8 coverage) -----

#[test]
fn bard_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = BARD_LEVEL7_FIXTURE.replace("class:bard:7", "class:bard:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id == "class_chassis.spell_baseline.bard"
                || e.id == WELL_VERSED_ID
                || e.id == INSPIRE_COMPETENCE_ID
                || e.id == LORE_MASTER_ID),
        "level-8 Bard is now recognized by a later SD13-E5 slice \
         (tests/sd13_bard_level8_progression.rs): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level7_recognition() {
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
fn multiclass_bard_level7_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL7_FIXTURE.replace(
        "class_level=class:bard:7",
        "class_level=class:bard:7\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_bard_row_names_level_7_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd13_bard_level7_progression"),
        "bard row must cite the live SD13-E5 level-7 proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 7") || note.contains("level-7"),
        "bard partial note must name the level-7 widening: {note}"
    );
    assert!(
        note.contains("Inspire Competence") || note.contains("inspire competence"),
        "bard partial note must name the newly widened Inspire Competence magnitude: {note}"
    );
}
