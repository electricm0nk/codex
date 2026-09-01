//! SD18 Bard level-19 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-18 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level18_widening.rs`, the SD18 loop's own
//! prior ceiling) to Bard level 19 — the loop's FOURTH §3.2 level-19
//! landing (after Barbarian, Cleric, and Fighter), and alphabetically the
//! first of the six remaining eligible classes (Paladin, Ranger, Rogue,
//! Sorcerer, Bard, Wizard) picked per the prior cycle's own "Next cycle
//! instructions" — mirroring the sibling per-level-gate idiom
//! (`supported_bard_level` is generalized from `1..=18` to `1..=19` via
//! `MAX_SUPPORTED_BARD_LEVEL = 19`). TWO primary PF1 CRB sources were read
//! fresh this cycle before writing any code or test: a raw HTML parse of
//! d20pfsrd.com's own class table (bypassing AI-summarization) and the
//! Archives of Nethys aonprd.com mirror via `ClassDisplay.aspx`, fetching
//! the levels-15-through-20 block in one pass so the level-19 row's
//! neighbors were visible in context (guards against
//! level-misattribution): level 17 "Inspire courage +4, lore master
//! 3/day", level 18 "Mass suggestion, versatile performance", level 19
//! "Inspire competence +6", level 20 "Deadly performance" — both sources
//! agree byte-for-byte on the level-19 row ("+14/+9/+4 | +6 | +11 | +11 |
//! Inspire competence +6"), and no source disagreement was found (a third
//! source was not required).
//!
//! - level 19 base attack bonus GENUINELY RISES to +14 (`19 * 3 / 4 = 14`,
//!   up from +13 at level 18); poor Fortitude STAYS +6 (`19 / 3 = 6`, an
//!   integer-division coincidence with level 18) and both good saves
//!   (Reflex, Will) STAY +11 (`19 / 2 + 2 = 11`, also integer-division
//!   coincidences with level 18) — checked directly against both primary
//!   sources rather than assumed.
//! - Bardic Knowledge STAYS 9 (`max(19 / 2, 1) = 9`, an integer-division
//!   coincidence with level 18).
//! - the Bardic Performance rounds-per-day pool GENUINELY RISES to 42
//!   (`4 + Charisma modifier 2 + 2 x (19 - 1)`, up from 40 at level 18).
//! - the Fascinate DC STAYS 21 (`10 + 19/2 + Charisma modifier 2`, an
//!   integer-division coincidence with level 18), while the Fascinate
//!   affected-creature count GENUINELY RISES to 7 (`1 + (19 - 1) / 3 = 7`,
//!   up from 6 at level 18).
//! - Frightening Tune's DC (the same formula shape as the Fascinate DC)
//!   likewise STAYS 21.
//! - Inspire Courage stays at its level-17 fourth tier (+4; the next tier
//!   lands at bard level 23, out of scope).
//! - Inspire Competence GENUINELY RISES to its FIFTH tier (+6, up from +5
//!   at level 15-18) — a new tier constant on the already-generalized
//!   tiered if/else chain, exactly the tier the level-15 and level-17
//!   cycles' own doc comments already predicted ("the next increase (to
//!   +6) lands at bard level 19").
//! - Lore Master stays at its level-17 third tier (3/day take-20 uses; no
//!   further tier is defined within this bounded slice's ceiling).
//! - Inspire Heroics' flat save-bonus (+4), AC-bonus (+4), and base target
//!   count (2, set at level 18) all carry over unchanged (the next
//!   target-count rise, "+1 creature per three bard levels beyond 15th",
//!   lands at bard level 21, out of scope).
//! - Soothing Performance carries over unchanged as a bounded grant-only
//!   identity record.
//!
//! The level-19 "Special" column's sole entry, Inspire Competence, is the
//! ONLY named feature at this level — checked directly rather than
//! assumed to require no new machinery: it is a genuine numeric tier-rise
//! on an already-proven, already-generalized tiered if/else chain (the
//! same idiom as Inspire Courage's and Lore Master's own tier additions),
//! not a new class feature, not a new record type, and not a new choice
//! slot.
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing, condition-resolution, fear-resolution, spell-like-ability
//! casting/targeting, or save/AC-application engine (including Inspire
//! Courage's own competence/morale-bonus application and Lore Master's own
//! take-10/take-20 skill-check-resolution execution), or the spontaneous
//! spell posture burden (all stay named-but-unproven, unchanged from
//! levels 1-18); the spontaneous spell-level-access ladder and the base
//! spells-per-day / spells-known table lookups stay at their pre-existing
//! level-10 ceiling exactly as left by the level-11 through level-18
//! cycles (no 5th-level spell-access threshold is grounded), and it does
//! not ground Bard level 20. It also preserves the accepted Bard
//! level-1..level-18 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. No new record type or choice slot is
//! added this slice — only one new tier constant pair on an
//! already-generalized tiered if/else chain (Inspire Competence's flat
//! magnitude).

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level18_sd18_widening_deterministic_input.txt"
);

const BARD_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level19_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const INSPIRE_COURAGE_ID: &str = "class_chassis.bard.inspire_courage_bonus";
const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";
const SOOTHING_PERFORMANCE_ID: &str = "class_feature.bard.soothing_performance";
const FRIGHTENING_TUNE_DC_ID: &str = "class_chassis.bard.frightening_tune_dc";
const INSPIRE_HEROICS_SAVE_BONUS_ID: &str = "class_feature.bard.inspire_heroics_save_bonus";
const INSPIRE_HEROICS_AC_BONUS_ID: &str = "class_feature.bard.inspire_heroics_ac_bonus";
const INSPIRE_HEROICS_TARGET_COUNT_ID: &str = "class_feature.bard.inspire_heroics_target_count";

// ----- Base attack bonus genuinely rises; base saves stay put (coincidence) -----

#[test]
fn bard_level19_base_attack_genuinely_rises_saves_stay_put() {
    let input = load(BARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 14,
        "Bard level 19 3/4-BAB progression (19 * 3 / 4) must genuinely rise to 14, up from 13 \
         at level 18: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Bard level 19 poor Fortitude (19/3) must stay 6, an integer-division coincidence with \
         level 18"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 11,
        "Bard level 19 good Reflex (19/2+2) must stay 11, an integer-division coincidence with \
         level 18"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 11,
        "Bard level 19 good Will (19/2+2) must stay 11, an integer-division coincidence with \
         level 18"
    );
}

// ----- Bardic Knowledge stays put; Bardic Performance rounds genuinely rises -----

#[test]
fn bard_level19_knowledge_stays_rounds_genuinely_rises() {
    let input = load(BARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 9,
        "Bardic Knowledge (max(19/2, 1)) must stay 9, an integer-division coincidence with \
         level 18"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 43,
        "Bard level 19 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (19 - 1)) must \
         genuinely rise to 43, up from 41 at level 18: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 22,
        "the Fascinate DC (10 + 19/2 + Charisma modifier 3) must stay 22, an integer-division \
         coincidence with level 18: {}",
        dc.detail
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 7,
        "the Fascinate affected-creature count (1 + (19-1)/3) must genuinely rise to 7, up from \
         6 at level 18"
    );
}

// ----- Frightening Tune's flat DC magnitude stays put -----

#[test]
fn bard_level19_frightening_tune_dc_stays_put() {
    let input = load(BARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, FRIGHTENING_TUNE_DC_ID);
    assert_eq!(
        dc.value, 22,
        "Frightening Tune's Will save DC (10 + 1/2 bard level + Charisma modifier) must stay 22 \
         at level 19, the same formula shape as the Fascinate DC: {}",
        dc.detail
    );
}

// ----- Inspire Competence genuinely rises to its fifth tier; Inspire Courage and Lore Master stay put -----

#[test]
fn bard_level19_inspire_competence_genuinely_rises_others_stay_put() {
    let input = load(BARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(
        inspire_courage.value, 4,
        "Inspire Courage must stay +4 at level 19 (the next tier lands at bard level 23, out of \
         scope): {}",
        inspire_courage.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 6,
        "Inspire Competence must genuinely rise to its fifth tier (+6) at level 19: {}",
        inspire_competence.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 3,
        "Lore Master's flat take-20 usage count must stay 3/day at level 19 (no further tier is \
         defined within this bounded slice's ceiling): {}",
        lore_master.detail
    );
}

// ----- Inspire Heroics' save/AC bonuses and target count all carry over -----

#[test]
fn bard_level19_inspire_heroics_carries_over() {
    let input = load(BARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let save_bonus = explanation(&computation, INSPIRE_HEROICS_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 4,
        "Inspire Heroics' morale bonus on saving throws must stay a flat +4 at level 19: {}",
        save_bonus.detail
    );

    let ac_bonus = explanation(&computation, INSPIRE_HEROICS_AC_BONUS_ID);
    assert_eq!(
        ac_bonus.value, 4,
        "Inspire Heroics' dodge bonus to AC must stay a flat +4 at level 19: {}",
        ac_bonus.detail
    );

    let target_count = explanation(&computation, INSPIRE_HEROICS_TARGET_COUNT_ID);
    assert_eq!(
        target_count.value, 2,
        "Inspire Heroics' base target count must stay 2 at level 19 (the next rise lands at \
         bard level 21, out of scope): {}",
        target_count.detail
    );
}

// ----- Soothing Performance carries over unchanged -----

#[test]
fn bard_level19_soothing_performance_carries_over() {
    let input = load(BARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must carry over as a bounded +0 grant-only identity record at \
         level 19: {}",
        soothing_performance.detail
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 19 -----

#[test]
fn bard_level19_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL19_FIXTURE);
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
                "level-19 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-18 fixture is unaffected by this widening -----

#[test]
fn bard_level18_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(inspire_competence.value, 5, "Bard level 18 Inspire Competence must stay +5");

    let target_count = explanation(&computation, INSPIRE_HEROICS_TARGET_COUNT_ID);
    assert_eq!(
        target_count.value, 2,
        "Bard level 18 Inspire Heroics target count must stay 2, before the level-21 threshold"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 41, "Bard level 18 Bardic Performance rounds must stay 41");
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level19_recognition() {
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
fn multiclass_bard_level19_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL19_FIXTURE.replace(
        "class_level=class:bard:19",
        "class_level=class:bard:19\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-19 widening -----

#[test]
fn matrix_bard_row_names_level_19_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level19_widening"),
        "bard row must cite the live SD18 level-19 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 19") || note.contains("level-19"),
        "bard partial note must name the level-19 widening: {note}"
    );
}
