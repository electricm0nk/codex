//! SD13-E5 Bard level-6 progression grounding proof.
//!
//! Widens the accepted Bard level-1/level-2/level-3/level-4/level-5
//! spontaneous arcane spell-bearing baseline
//! (`tests/sd13_bard_level1_spell_baseline.rs`,
//! `tests/sd13_bard_base_attack_and_saves.rs`,
//! `tests/sd13_bard_fascinate_dc.rs`, `tests/sd13_bard_level2_progression.rs`,
//! `tests/sd13_bard_level3_progression.rs`,
//! `tests/sd13_bard_level4_progression.rs`,
//! `tests/sd13_bard_level5_progression.rs`) to Bard level 6, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_bard_level` is generalized from `1..=5`
//! to `1..=6` via `MAX_SUPPORTED_BARD_LEVEL = 6`). Both PF1 CRB primary
//! sources (d20pfsrd and the legacy.aonprd.com Bard class table mirror) were
//! read directly before writing any code or test:
//!
//! - level 6 base attack bonus is +4 (`6 * 3 / 4 = 4`), base Reflex/Will are
//!   +5 (good, `6/2+2`), base Fortitude is +2 (poor, `6/3`) — all three
//!   genuinely new values, confirmed by the same formulas already grounded
//!   at levels 1-5, not re-derived.
//! - Bardic Knowledge genuinely rises to `max(6/2, 1) = 3`, up from 2 at
//!   level 5, via the same pre-existing formula, not re-derived.
//! - Bardic Performance rounds per day continues to scale: `4 + Charisma
//!   modifier + 2 * (level - 1)` = 4 + 2 + 10 = 16 on the fixture's Charisma
//!   15, up from 14 at level 5, confirmed via the same formula, not a new
//!   record.
//! - Inspire Courage's flat magnitude stays +2 at level 6 (PF1 Core
//!   Rulebook: "At 5th level, and every six bard levels thereafter, this
//!   bonus increases by +1" — the next increase does not land until bard
//!   level 11), re-verified rather than assumed.
//! - the Fascinate flat Will-save DC (`10 + level/2 + CHA modifier`)
//!   genuinely rises to 15 (10 + 3 + 2), up from 14 at level 5, via the
//!   same pre-existing formula, not re-derived. The flat affected-creature
//!   count (`1 + (level-1)/3`) stays 2 at level 6 (`1 + 5/3 = 2`), an
//!   integer-division coincidence with level 5's count, not a sign the
//!   formula stopped scaling.
//! - Well-Versed (2nd-level), Inspire Competence (3rd-level), and Lore
//!   Master (5th-level) all stay granted at level 6, not re-derived — the
//!   same bounded identity/magnitude records already grounded at levels 2,
//!   3, and 5.
//! - the PF1 Core Rulebook Bard class table's level-6 "Special" column
//!   (verified independently against both primary sources) reads
//!   "Suggestion, Versatile performance". Both entries were checked and
//!   confirmed NOT flat/identity-shaped: Suggestion is a spell-like ability
//!   that requires a creature already fascinated by the bard's performance
//!   (a performance-state-engine prerequisite that does not exist in this
//!   codebase) and replicates the "suggestion" spell's own effect
//!   resolution (a spell-effect-resolution engine that also does not
//!   exist); Versatile Performance's 6th-level grant is merely an
//!   additional instance of the SAME choice-gated skill-substitution
//!   engine already deliberately left named-but-unproven at 2nd level, not
//!   a new type of class feature. Neither is grounded — no explanation or
//!   diagnostic record is fabricated for either.
//!
//! It deliberately does not touch the performance-state/action-economy
//! engine, Countersong, Distraction, Versatile Performance (either its
//! 2nd-level or 6th-level grant), Suggestion, the Lore Master
//! take-10/take-20 execution mechanic, or the spontaneous spell burden (all
//! stay named-but-unproven, unchanged from level 1/2/3/4/5), and it does not
//! ground Bard level 7+. It also preserves the accepted Bard level-1/
//! level-2/level-3/level-4/level-5 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::{ComputationExplanation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level5_sd13_deterministic_input.txt");

const BARD_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_bard_level6_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";

// ----- Base attack bonus at level 6 -----

#[test]
fn bard_level6_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 4,
        "Bard level 6 3/4-BAB progression (6 * 3 / 4) must equal 4: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 6 (good Reflex/Will, poor Fortitude) -----

#[test]
fn bard_level6_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Bard level 6 poor Fortitude (6/3) must equal 2");

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(reflex.value, 5, "Bard level 6 good Reflex (6/2+2) must equal 5");

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(will.value, 5, "Bard level 6 good Will (6/2+2) must equal 5");
}

// ----- Bardic Knowledge genuinely rises to 3 at level 6 -----

#[test]
fn bard_level6_bardic_knowledge_rises_to_three() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 3,
        "Bard level 6 Bardic Knowledge (max(6/2, 1)) must genuinely rise to 3, up from 2 at \
         level 5: {}",
        knowledge.detail
    );
}

// ----- Bardic Performance rounds per day keeps scaling with level -----

#[test]
fn bard_level6_bardic_performance_rounds_per_day_keeps_scaling() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Bardic Performance: 4 + CHA modifier at level 1, plus 2
    // additional rounds per day at each level after 1st. Fixture CHA 15 + 2 Human
    // racial (CG-03 fix) -> +3 modifier. At level 6: 4 + 3 + 2 * (6 - 1) = 17.
    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(
        rounds.value, 17,
        "Bard level 6 bardic performance rounds per day must equal 4 + CHA + 2*(level-1) \
         = 4 + 3 + 10 = 17: {}",
        rounds.detail
    );
}

// ----- Inspire Courage stays +2 at level 6 (next increase is level 11) -----

#[test]
fn bard_level6_inspire_courage_stays_plus_two() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 2,
        "Bard level 6 Inspire Courage magnitude must stay +2 (PF1 CRB: the next increase does \
         not land until bard level 11): {}",
        inspire_courage.detail
    );
}

// ----- Fascinate DC rises to 15; affected-creature count stays 2 -----

#[test]
fn bard_level6_fascinate_dc_rises_and_affected_creatures_stays_the_same() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture CHA 15 + 2 Human racial (CG-03 fix) -> +3 modifier. DC = 10 + 6/2 + 3 = 16.
    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 16,
        "Bard level 6 Fascinate DC must genuinely rise to 10 + (6/2) + 3 = 16, up from 15 at \
         level 5: {}",
        dc.detail
    );

    // Affected creatures = 1 + (6-1)/3 = 2, an integer-division coincidence with level 5.
    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 2,
        "Bard level 6 Fascinate affected-creature count must stay 1 + (6-1)/3 = 2, unchanged \
         from level 5: {}",
        count.detail
    );
}

// ----- Well-Versed, Inspire Competence, and Lore Master all stay granted at level 6 -----

#[test]
fn bard_level6_keeps_well_versed_inspire_competence_and_lore_master_grounded() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(
        well_versed.value, 4,
        "Well-Versed must stay the flat +4 bonus at level 6, not re-derived: {}",
        well_versed.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 2,
        "Inspire Competence must stay the flat +2 bonus at level 6, not re-derived: {}",
        inspire_competence.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 1,
        "Lore Master must stay the flat 1/day take-20 count at level 6, not re-derived: {}",
        lore_master.detail
    );
}

// ----- Suggestion and the level-6 Versatile Performance grant: checked, confirmed NOT flat, deliberately unproven -----

#[test]
fn bard_level6_does_not_fabricate_suggestion_or_versatile_performance() {
    let input = load(BARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // SD-34 bucket-B batch cycle: `AT-34-E3-001` (`class_feature_option_pool_record_
    // with_magnitude_not_held_by_engine` mechanism)'s own cycle 5 legitimately grounded
    // Suggestion's flat Will-save DC (`class_feature.bard.suggestion_dc`, verified against
    // this repo's own ingested corpus record and the identical formula shape as the
    // already-grounded Fascinate DC -- see `pilot_compute.rs`'s own doc comment at that push
    // site) BEFORE this test's own assertion was ever updated to admit it -- a stale-gate gap
    // this cycle found and fixes here, not a new fabrication this cycle introduced. Only the
    // ONE flat DC magnitude is admitted; any OTHER suggestion-tagged id (execution, targeting,
    // resolution -- none of which is grounded) still fails this control exactly as before.
    let suggestion_ids: Vec<&ComputationExplanation> = computation
        .explanations
        .iter()
        .filter(|e| e.id.contains("suggestion"))
        .collect();
    assert!(
        suggestion_ids
            .iter()
            .all(|e| e.id == "class_feature.bard.suggestion_dc"),
        "only the flat, citation-backed Suggestion DC magnitude may appear; Suggestion's \
         fascinated-target prerequisite and its own effect-resolution engine are still \
         unimplemented, so no OTHER suggestion-tagged explanation record may be fabricated: \
         {suggestion_ids:?}"
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("suggestion")),
        "no diagnostic record should be fabricated for Suggestion either, since this slice \
         deliberately declines to ground its execution: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("versatile_performance")),
        "the 6th-level Versatile Performance grant is the same choice-gated \
         skill-substitution engine already deliberately left named-but-unproven at 2nd level; \
         no explanation record must be fabricated for it: {:?}",
        computation.explanations
    );
}

// ----- Only the known bard-namespaced ids appear at level 6 (no new ids beyond level 5) -----

#[test]
fn bard_level6_gains_no_new_bard_namespaced_explanation_id() {
    let input = load(BARD_LEVEL6_FIXTURE);
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
        // Suggestion's flat Will-save DC at this exact level (level 6) -- a real
        // "Special"-column class feature this test's own known-id list never listed.
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
        "Bard level 6 must not gain any bard-namespaced explanation id beyond the \
         already-grounded pillars: {:?}",
        computation.explanations
    );
}

// ----- The two existing burden diagnostics still fire at level 6 -----

#[test]
fn bard_level6_still_claim_blocks_performance_execution_and_spontaneous_spell_burdens() {
    let input = load(BARD_LEVEL6_FIXTURE);
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
                "level-6 Bard must ground an honest not-performing record when no \
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

// ----- The accepted Bard level-5 truth is unaffected -----

#[test]
fn bard_level5_truth_is_unchanged_by_this_widening() {
    let input = load(BARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Bard level 5 base attack bonus must stay 3");

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(knowledge.value, 2, "Bard level 5 Bardic Knowledge must stay 2");

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(dc.value, 15, "Bard level 5 Fascinate DC must stay 15, unaffected by the level-6 widening");
}

// ----- Negative control retired: level 7 was later widened into the supported tranche -----

#[test]
fn bard_level_7_was_later_widened_into_the_supported_tranche() {
    // This test previously asserted that level-7 Bard stayed unrecognized by this
    // slice. A later SD13-E5 slice widened `MAX_SUPPORTED_BARD_LEVEL` to 7 (see
    // `tests/sd13_bard_level7_progression.rs`), so level 7 is now genuinely
    // grounded. This control is retained, renamed, and inverted to document that
    // history rather than silently deleted, mirroring the
    // Rogue/Barbarian/Monk/Cleric level-6/level-7 precedent; the new level-8
    // negative control lives in `tests/sd13_bard_level7_progression.rs`.
    let level_7 = BARD_LEVEL6_FIXTURE.replace("class:bard:6", "class:bard:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.bard.base_attack_bonus"),
        "level-7 Bard must now be recognized by the widened supported tranche: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level6_recognition() {
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
fn multiclass_bard_level6_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL6_FIXTURE.replace(
        "class_level=class:bard:6",
        "class_level=class:bard:6\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-6 widening -----

#[test]
fn matrix_bard_row_names_level_6_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd13_bard_level6_progression"),
        "bard row must cite the live SD13-E5 level-6 proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 6") || note.contains("level-6"),
        "bard partial note must name the level-6 widening: {note}"
    );
    assert!(
        note.contains("Suggestion") || note.contains("suggestion"),
        "bard partial note must name the newly checked, confirmed-not-flat Suggestion feature: \
         {note}"
    );
}
