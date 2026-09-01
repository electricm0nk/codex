//! SD18 Bard level-20 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-19 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level19_widening.rs`, the SD18 loop's own
//! prior ceiling) to Bard level 20 — the final remaining level within
//! PF1's 1-20 character-level cap for this class row, and the loop's
//! FOURTH §3.2 level-20 landing, after Cleric, Wizard, and Barbarian —
//! mirroring the sibling per-level-gate idiom (`supported_bard_level` is
//! generalized from `1..=19` to `1..=20` via `MAX_SUPPORTED_BARD_LEVEL =
//! 20`). TWO primary PF1 CRB sources were read fresh this cycle before
//! writing any code or test: a raw HTML parse of d20pfsrd.com's own class
//! table (bypassing AI-summarization) and the Archives of Nethys
//! aonprd.com mirror via `ClassDisplay.aspx`, fetching the
//! levels-17-through-20 block in one pass so the level-20 row's
//! neighbors were visible in context (guards against
//! level-misattribution): level 18 "Mass suggestion, versatile
//! performance", level 19 "Inspire competence +6", level 20 "Deadly
//! performance" — both sources agree byte-for-byte on the level-20 row
//! ("+15/+10/+5 | +6 | +12 | +12 | Deadly performance"), and no source
//! disagreement was found (a third source was not required).
//!
//! - level 20 base attack bonus GENUINELY RISES to +15 (`20 * 3 / 4 =
//!   15`, up from +14 at level 19); poor Fortitude STAYS +6 (`20 / 3 =
//!   6`, an integer-division coincidence with level 19) while both good
//!   saves (Reflex, Will) GENUINELY RISE to +12 (`20 / 2 + 2 = 12`, up
//!   from +11) — checked directly against both primary sources rather
//!   than assumed.
//! - Bardic Knowledge GENUINELY RISES to 10 (`max(20 / 2, 1) = 10`, up
//!   from 9 at level 19).
//! - the Bardic Performance rounds-per-day pool GENUINELY RISES to 44
//!   (`4 + Charisma modifier 2 + 2 x (20 - 1)`, up from 42 at level 19).
//! - the Fascinate DC GENUINELY RISES to 22 (`10 + 20/2 + Charisma
//!   modifier 2`, up from 21), while the Fascinate affected-creature
//!   count STAYS 7 (`1 + (20 - 1) / 3 = 7`, an integer-division
//!   coincidence with level 19).
//! - Frightening Tune's DC (the same formula shape as the Fascinate DC)
//!   likewise GENUINELY RISES to 22.
//! - Inspire Courage stays at its level-17 fourth tier (+4; the next
//!   tier lands at bard level 23, out of scope).
//! - Inspire Competence stays at its level-19 fifth tier (+6; no further
//!   tier is defined within PF1's Core Rulebook).
//! - Lore Master stays at its level-17 third tier (3/day take-20 uses;
//!   no further tier is defined within this bounded slice's ceiling).
//! - Inspire Heroics' flat save-bonus (+4), AC-bonus (+4), and base
//!   target count (2, set at level 18) all carry over unchanged (the
//!   next target-count rise, "+1 creature per three bard levels beyond
//!   15th", lands at bard level 21, out of scope).
//! - Soothing Performance carries over unchanged as a bounded grant-only
//!   identity record.
//!
//! The level-20 "Special" column's sole entry, Deadly Performance (the
//! class capstone), IS genuinely grounded this cycle — checked directly
//! rather than assumed to require no new machinery: its named Will-save
//! DC ("DC 10 + 1/2 the bard's level + the bard's Cha modifier") is the
//! EXACT SAME formula shape as the already-grounded Fascinate DC and
//! Frightening Tune DC, so only that flat DC magnitude is grounded here
//! (mirroring the Frightening Tune idiom exactly) — it genuinely rises
//! to 22 (`10 + 20/2 + Charisma modifier 2`). No
//! audible/visual-performance-requirement checking, no Will-save
//! resolution, and no death-effect application to any target is computed
//! because no targeting/range, save-resolution, or
//! death-effect-resolution engine exists anywhere in this codebase.
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing, condition-resolution, fear-resolution, spell-like-ability
//! casting/targeting, save/AC-application, or death-effect-resolution
//! engine (including Inspire Courage's own competence/morale-bonus
//! application, Lore Master's own take-10/take-20 skill-check-resolution
//! execution, and Deadly Performance's own Will-save/death-effect
//! execution), or the spontaneous spell posture burden (all stay
//! named-but-unproven, unchanged from levels 1-19); the spontaneous
//! spell-level-access ladder and the base spells-per-day / spells-known
//! table lookups stay at their pre-existing level-10 ceiling exactly as
//! left by the level-11 through level-19 cycles (no 5th-level
//! spell-access threshold is grounded). It also preserves the accepted
//! Bard level-1..level-19 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control. No new record type or
//! choice slot is added this slice beyond the one new Deadly Performance
//! DC magnitude — only one new tier constant
//! (`BARD_DEADLY_PERFORMANCE_LEVEL`) on the already-generalized flat-DC
//! formula shape.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level19_sd18_widening_deterministic_input.txt"
);

const BARD_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level20_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const INSPIRE_COURAGE_ID: &str = "class_chassis.bard.inspire_courage_bonus";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";
const SOOTHING_PERFORMANCE_ID: &str = "class_feature.bard.soothing_performance";
const FRIGHTENING_TUNE_DC_ID: &str = "class_chassis.bard.frightening_tune_dc";
const DEADLY_PERFORMANCE_DC_ID: &str = "class_feature.bard.deadly_performance_dc";
const INSPIRE_HEROICS_SAVE_BONUS_ID: &str = "class_feature.bard.inspire_heroics_save_bonus";
const INSPIRE_HEROICS_AC_BONUS_ID: &str = "class_feature.bard.inspire_heroics_ac_bonus";
const INSPIRE_HEROICS_TARGET_COUNT_ID: &str = "class_feature.bard.inspire_heroics_target_count";

// ----- Base attack bonus genuinely rises; poor Fortitude stays put; good saves genuinely rise -----

#[test]
fn bard_level20_base_attack_and_good_saves_genuinely_rise_fortitude_stays_put() {
    let input = load(BARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 15,
        "Bard level 20 3/4-BAB progression (20 * 3 / 4) must genuinely rise to 15, up from 14 \
         at level 19: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Bard level 20 poor Fortitude (20/3) must stay 6, an integer-division coincidence with \
         level 19"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 12,
        "Bard level 20 good Reflex (20/2+2) must genuinely rise to 12, up from 11 at level 19"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 12,
        "Bard level 20 good Will (20/2+2) must genuinely rise to 12, up from 11 at level 19"
    );
}

// ----- Bardic Knowledge and Bardic Performance rounds both genuinely rise -----

#[test]
fn bard_level20_knowledge_and_rounds_genuinely_rise() {
    let input = load(BARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 10,
        "Bardic Knowledge (max(20/2, 1)) must genuinely rise to 10, up from 9 at level 19"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 45,
        "Bard level 20 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (20 - 1)) must \
         genuinely rise to 45, up from 43 at level 19: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 23,
        "the Fascinate DC (10 + 20/2 + Charisma modifier 3) must genuinely rise to 23, up from \
         22 at level 19: {}",
        dc.detail
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 7,
        "the Fascinate affected-creature count (1 + (20-1)/3) must stay 7, an integer-division \
         coincidence with level 19"
    );
}

// ----- Frightening Tune's flat DC magnitude genuinely rises -----

#[test]
fn bard_level20_frightening_tune_dc_genuinely_rises() {
    let input = load(BARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, FRIGHTENING_TUNE_DC_ID);
    assert_eq!(
        dc.value, 23,
        "Frightening Tune's Will save DC (10 + 1/2 bard level + Charisma modifier) must \
         genuinely rise to 23 at level 20, the same formula shape as the Fascinate DC: {}",
        dc.detail
    );
}

// ----- Deadly Performance's flat DC magnitude is genuinely grounded for the first time -----

#[test]
fn bard_level20_deadly_performance_dc_is_grounded() {
    let input = load(BARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, DEADLY_PERFORMANCE_DC_ID);
    assert_eq!(
        dc.value, 23,
        "Deadly Performance's Will save DC (10 + 1/2 bard level + Charisma modifier) must be \
         grounded at 23 at level 20, the same formula shape as the Fascinate DC and Frightening \
         Tune DC: {}",
        dc.detail
    );
}

// ----- Inspire Competence, Inspire Courage, and Lore Master all stay put -----

#[test]
fn bard_level20_inspire_and_lore_master_all_stay_put() {
    let input = load(BARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(
        inspire_courage.value, 4,
        "Inspire Courage must stay +4 at level 20 (the next tier lands at bard level 23, out of \
         scope): {}",
        inspire_courage.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 6,
        "Inspire Competence must stay at its fifth tier (+6) at level 20 (no further tier is \
         defined within PF1's Core Rulebook): {}",
        inspire_competence.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 3,
        "Lore Master's flat take-20 usage count must stay 3/day at level 20 (no further tier is \
         defined within this bounded slice's ceiling): {}",
        lore_master.detail
    );
}

// ----- Inspire Heroics' save/AC bonuses and target count all carry over -----

#[test]
fn bard_level20_inspire_heroics_carries_over() {
    let input = load(BARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let save_bonus = explanation(&computation, INSPIRE_HEROICS_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 4,
        "Inspire Heroics' morale bonus on saving throws must stay a flat +4 at level 20: {}",
        save_bonus.detail
    );

    let ac_bonus = explanation(&computation, INSPIRE_HEROICS_AC_BONUS_ID);
    assert_eq!(
        ac_bonus.value, 4,
        "Inspire Heroics' dodge bonus to AC must stay a flat +4 at level 20: {}",
        ac_bonus.detail
    );

    let target_count = explanation(&computation, INSPIRE_HEROICS_TARGET_COUNT_ID);
    assert_eq!(
        target_count.value, 2,
        "Inspire Heroics' base target count must stay 2 at level 20 (the next rise lands at \
         bard level 21, out of scope): {}",
        target_count.detail
    );
}

// ----- Soothing Performance carries over unchanged -----

#[test]
fn bard_level20_soothing_performance_carries_over() {
    let input = load(BARD_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must carry over as a bounded +0 grant-only identity record at \
         level 20: {}",
        soothing_performance.detail
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 20 -----

#[test]
fn bard_level20_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL20_FIXTURE);
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
                "level-20 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-19 fixture is unaffected by this widening -----

#[test]
fn bard_level19_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(inspire_competence.value, 6, "Bard level 19 Inspire Competence must stay +6");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 43, "Bard level 19 Bardic Performance rounds must stay 43");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == DEADLY_PERFORMANCE_DC_ID),
        "Bard level 19 must not gain the Deadly Performance DC explanation"
    );
}

// ----- Negative control: level 21 stays unrecognized by this slice (PF1 has no 21st level) -----

#[test]
fn bard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARD_LEVEL20_FIXTURE.replace("class:bard:20", "class:bard:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.bard.")
                || e.id.starts_with("class_feature.bard.")
                || e.id == "class_chassis.spell_baseline.bard")
                // (v0.6 alpha swarm, risks item 8) bardic-performance-
                // execution's not-performing explanation is checked
                // unconditionally, regardless of level bound or
                // single-class status (mirrors the spell-posture
                // classes' and Barbarian's gate-ordering fix)
                && e.id != "class_feature.bard.bardic_performance_execution.not_performing"),
        "level-21 Bard must not gain any bounded bard explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level20_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.bard.")
                || e.id.starts_with("class_feature.bard.")),
        "the Fighter chassis must not surface any bard-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Bard is not promoted -----

#[test]
fn multiclass_bard_level20_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL20_FIXTURE.replace(
        "class_level=class:bard:20",
        "class_level=class:bard:20\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.bard.")
                || e.id.starts_with("class_feature.bard."))
                // (v0.6 alpha swarm, risks item 8) bardic-performance-
                // execution's not-performing explanation is checked
                // unconditionally, regardless of level bound or
                // single-class status (mirrors the spell-posture
                // classes' and Barbarian's gate-ordering fix)
                && e.id != "class_feature.bard.bardic_performance_execution.not_performing"),
        "multiclass Bard must not gain any bounded bard explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_bard_row_names_level_20_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level20_widening"),
        "bard row must cite the live SD18 level-20 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "bard partial note must name the level-20 widening: {note}"
    );
}
