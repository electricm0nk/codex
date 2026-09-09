//! SD18 Bard level-17 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-16 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level16_widening.rs`, the SD18 loop's own
//! prior ceiling) to Bard level 17 — the loop's SECOND §3.2 level-17
//! landing (after Ranger) — mirroring the sibling per-level-gate idiom
//! (`supported_bard_level` is generalized from `1..=16` to `1..=17` via
//! `MAX_SUPPORTED_BARD_LEVEL = 17`). TWO primary PF1 CRB sources were read
//! directly before writing any code or test (d20pfsrd and the Archives of
//! Nethys aonprd.com mirror), including the levels-13-through-18 block in
//! one pass so the level-17 row's neighbors were visible in context (guards
//! against the level-misattribution failure mode prior cycles' own lessons
//! flagged): level 13 blank, level 14 "Frightening tune, versatile
//! performance", level 15 "Inspire competence +5, inspire heroics", level
//! 16 blank, level 17 "Inspire courage +4, lore master 3/day", level 18
//! "Mass suggestion, versatile performance" — both sources agree
//! byte-for-byte on the level-17 row ("+12/+7/+2 | +5 | +10 | +10 |
//! Inspire courage +4, lore master 3/day"), and no source disagreement was
//! found (a third source was not required). This resolves the level-16
//! cycle's own carried-forward note (which had already peeked ahead at the
//! raw level-17 row while investigating level 16) with a fresh,
//! independent re-verification rather than trusting it at face value.
//!
//! - level 17 base attack bonus STAYS +12 (`17 * 3 / 4 = 12`, an
//!   integer-division coincidence with level 16, not a formula change) and
//!   both good saves (Reflex, Will) STAY +10 (`17 / 2 + 2 = 10`), while
//!   poor Fortitude STAYS +5 (`17 / 3 = 5`) — every base-chassis pillar is
//!   numerically unchanged from level 16, confirmed directly against both
//!   primary sources rather than assumed.
//! - Bardic Knowledge STAYS 8 (`max(17 / 2, 1) = 8`, another
//!   integer-division coincidence with level 16).
//! - the Bardic Performance rounds-per-day pool GENUINELY RISES to 38
//!   (`4 + Charisma modifier 2 + 2 x (17 - 1)`, up from 36 at level 16).
//! - the Fascinate DC STAYS 20 (`10 + 17/2 + Charisma modifier 2`) and the
//!   Fascinate affected-creature count STAYS 6 (`1 + (17 - 1) / 3`), both
//!   integer-division coincidences with level 16.
//! - Frightening Tune's DC (the same formula shape as the Fascinate DC)
//!   likewise STAYS 20 for the same reason.
//! - the PF1 Core Rulebook Bard class table's level-17 "Special" column
//!   reads "Inspire courage +4, lore master 3/day":
//!   - Inspire Courage's flat magnitude GENUINELY RISES from +3 to +4 — a
//!     fourth tier on the already-generalized tiered if/else chain (the
//!     same "every six bard levels thereafter" cadence that produced the
//!     third tier at level 11), the same arithmetic-widening idiom as
//!     Inspire Competence's own third/fourth tier additions, needing no
//!     new grounding machinery beyond one new tier constant pair.
//!   - Lore Master's flat take-20 usage-count magnitude GENUINELY RISES
//!     from 2/day to 3/day — a third tier on the already-generalized
//!     tiered if/else chain (the same every-six-bard-levels-after-5th
//!     cadence as Inspire Courage), mirroring the same idiom.
//!   - Inspire Competence stays at its level-15 fourth tier (+5; its next
//!     tier lands at bard level 19, out of scope).
//!   - Inspire Heroics' flat save-bonus (+4) and AC-bonus (+4) magnitudes
//!     and base target count (1) all carry over unchanged (the "+1
//!     creature per three bard levels beyond 15th" scaling lands at bard
//!     level 18, beyond this bounded slice's ceiling).
//!   - Soothing Performance carries over unchanged as a bounded grant-only
//!     identity record.
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing, condition-resolution, fear-resolution, or
//! save/AC-application engine (including Inspire Courage's own
//! competence/morale-bonus application and Lore Master's own
//! take-10/take-20 skill-check-resolution execution), or the spontaneous
//! spell posture burden (all stay named-but-unproven, unchanged from
//! levels 1-16); the spontaneous spell-level-access ladder and the base
//! spells-per-day / spells-known table lookups stay at their pre-existing
//! level-10 ceiling exactly as left by the level-11 through level-16
//! cycles (no 5th-level spell-access threshold is grounded), and it does
//! not ground Bard level 18+. It also preserves the accepted Bard
//! level-1..level-16 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. No new record type or choice slot is
//! added this slice — only two new tier constant pairs on
//! already-generalized tiered if/else chains.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level16_sd18_widening_deterministic_input.txt"
);

const BARD_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level17_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus and all three base saves stay unchanged (coincidence) -----

#[test]
fn bard_level17_base_attack_and_saves_stay_unchanged() {
    let input = load(BARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Bard level 17 3/4-BAB progression (17 * 3 / 4) must stay 12, an integer-division \
         coincidence with level 16: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Bard level 17 poor Fortitude (17/3) must stay 5, an integer-division coincidence with \
         level 16"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 10,
        "Bard level 17 good Reflex (17/2+2) must stay 10, an integer-division coincidence with \
         level 16"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 10,
        "Bard level 17 good Will (17/2+2) must stay 10, an integer-division coincidence with \
         level 16"
    );
}

// ----- Bardic Knowledge stays unchanged; Bardic Performance rounds genuinely rises -----

#[test]
fn bard_level17_knowledge_stays_and_rounds_genuinely_rises() {
    let input = load(BARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 8,
        "Bardic Knowledge (max(17/2, 1)) must stay 8, an integer-division coincidence with \
         level 16"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 39,
        "Bard level 17 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (17 - 1)) must \
         genuinely rise to 39, up from 37 at level 16: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 21,
        "the Fascinate DC (10 + 17/2 + Charisma modifier 3) must stay 21, an integer-division \
         coincidence with level 16"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 6,
        "the Fascinate affected-creature count (1 + (17-1)/3) must stay 6, an integer-division \
         coincidence with level 16"
    );
}

// ----- Frightening Tune's flat DC magnitude stays unchanged (coincidence) -----

#[test]
fn bard_level17_frightening_tune_dc_stays_unchanged() {
    let input = load(BARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, FRIGHTENING_TUNE_DC_ID);
    assert_eq!(
        dc.value, 21,
        "Frightening Tune's Will save DC (10 + 1/2 bard level + Charisma modifier) must stay \
         21 at level 17, the same integer-division coincidence as the Fascinate DC: {}",
        dc.detail
    );
}

// ----- Inspire Competence stays at its level-15 fourth tier -----

#[test]
fn bard_level17_inspire_competence_stays_fourth_tier() {
    let input = load(BARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 5,
        "Inspire Competence must stay +5 at level 17 (the next tier lands at bard level 19, out \
         of scope): {}",
        inspire_competence.detail
    );
}

// ----- Inspire Courage and Lore Master both genuinely reach their next tier -----

#[test]
fn bard_level17_inspire_courage_and_lore_master_genuinely_rise() {
    let input = load(BARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(
        inspire_courage.value, 4,
        "Inspire Courage must genuinely rise to +4 at level 17 (PF1 Core Rulebook: \"Inspire \
         courage +4, lore master 3/day\"): {}",
        inspire_courage.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 3,
        "Lore Master's flat take-20 usage count must genuinely rise to 3/day at level 17: {}",
        lore_master.detail
    );
}

// ----- Inspire Heroics' flat magnitudes carry over unchanged -----

#[test]
fn bard_level17_inspire_heroics_carries_over_unchanged() {
    let input = load(BARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let save_bonus = explanation(&computation, INSPIRE_HEROICS_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 4,
        "Inspire Heroics' morale bonus on saving throws must stay a flat +4 at level 17: {}",
        save_bonus.detail
    );

    let ac_bonus = explanation(&computation, INSPIRE_HEROICS_AC_BONUS_ID);
    assert_eq!(
        ac_bonus.value, 4,
        "Inspire Heroics' dodge bonus to AC must stay a flat +4 at level 17: {}",
        ac_bonus.detail
    );

    let target_count = explanation(&computation, INSPIRE_HEROICS_TARGET_COUNT_ID);
    assert_eq!(
        target_count.value, 1,
        "Inspire Heroics' base target count must stay 1 at level 17, before the \
         every-three-levels-beyond-15th scaling (lands at level 18, out of scope): {}",
        target_count.detail
    );
}

// ----- Soothing Performance carries over unchanged -----

#[test]
fn bard_level17_soothing_performance_carries_over() {
    let input = load(BARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must carry over as a bounded +0 grant-only identity record at \
         level 17: {}",
        soothing_performance.detail
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 17 -----

#[test]
fn bard_level17_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL17_FIXTURE);
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
                "level-17 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-16 fixture is unaffected by this widening -----

#[test]
fn bard_level16_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(inspire_courage.value, 3, "Bard level 16 Inspire Courage must stay +3");

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(lore_master.value, 2, "Bard level 16 Lore Master must stay 2/day");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 37, "Bard level 16 Bardic Performance rounds must stay 37");
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level17_recognition() {
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
fn multiclass_bard_level17_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL17_FIXTURE.replace(
        "class_level=class:bard:17",
        "class_level=class:bard:17\nclass_level=class:fighter:1",
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
                && e.id != "class_feature.bard.bardic_performance_execution.not_performing"
                // SD-34 wave 34 lane A (`docs/release/SD-34-book-completion/artifacts/
                // bucket-d-mining/wave34_laneA_weapon_and_armor_proficiency_cycle_
                // receipt.md`): Bard's own Weapon and Armor Proficiency identity
                // grant is now genuinely grounded as a level-independent, always-on
                // +0 record (true since level 1, mirrors the same "no gate to lift"
                // idiom as Jack-of-All-Trades) -- not a bounded, level-gated feature
                // this slice's negative control is checking for.
                && e.id != "class_feature.bard.weapon_and_armor_proficiency"),
        "multiclass Bard must not gain any bounded bard explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Bard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_bard_row_names_level_17_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level17_widening"),
        "bard row must cite the live SD18 level-17 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 17") || note.contains("level-17"),
        "bard partial note must name the level-17 widening: {note}"
    );
}
