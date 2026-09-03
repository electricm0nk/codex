//! SD18 Bard level-16 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-15 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level15_widening.rs`, the SD18 loop's own
//! prior ceiling) to Bard level 16 — the loop's EIGHTH §3.2 level-16
//! landing (after Barbarian, Fighter, Wizard, Rogue, Cleric, Paladin, and
//! Sorcerer) — mirroring the sibling per-level-gate idiom
//! (`supported_bard_level` is generalized from `1..=15` to `1..=16` via
//! `MAX_SUPPORTED_BARD_LEVEL = 16`). THREE primary PF1 CRB sources were
//! read directly before writing any code or test, and all three agree
//! byte-for-byte on the level-16 row ("+12/+7/+2 | +5 | +10 | +10 | —"):
//! d20pfsrd, the Archives of Nethys aonprd.com mirror, and
//! legacy.aonprd.com's corerulebook mirror. This resolves a prior cycle's
//! carried-forward risk-map note claiming a source disagreement (aonprd.com
//! allegedly reading "Versatile performance" at level 16): that text
//! belongs to level 14's own already-grounded Special column
//! ("Frightening tune, Versatile performance"), misattributed to level 16
//! by an earlier cycle's transcription — the exact same
//! carried-forward-risk-map-drift failure mode the Sorcerer level-16
//! cycle's own investigation flagged and fixed for its own row.
//!
//! - level 16 base attack bonus GENUINELY RISES to +12 (`16 * 3 / 4 = 12`,
//!   up from +11 at level 15) and both good saves (Reflex, Will) GENUINELY
//!   RISE to +10 (`16 / 2 + 2 = 10`, up from +9), while poor Fortitude
//!   STAYS +5 (`16 / 3 = 5`, an integer-division coincidence with level
//!   15).
//! - Bardic Knowledge GENUINELY RISES to 8 (`max(16 / 2, 1) = 8`, up from
//!   7); the Bardic Performance rounds-per-day pool GENUINELY RISES to 36
//!   (`4 + Charisma modifier 2 + 2 x (16 - 1)`, up from 34); the Fascinate
//!   DC GENUINELY RISES to 20 (`10 + 16/2 + Charisma modifier 2`, up from
//!   19); the Fascinate affected-creature count GENUINELY RISES to 6
//!   (`1 + (16 - 1) / 3`, up from 5).
//! - Frightening Tune's DC (the same formula shape as the Fascinate DC)
//!   likewise GENUINELY RISES to 20 for the same reason.
//! - the PF1 Core Rulebook Bard class table's level-16 "Special" column is
//!   genuinely BLANK on all three primary sources checked — a pure
//!   ceiling raise, mirroring the Wizard/Cleric/Sorcerer level-16 cycles'
//!   own blank-Special pure ceiling raises:
//!   - Inspire Competence stays at its level-15 fourth tier (+5; its next
//!     tier lands at bard level 19, out of scope).
//!   - Inspire Courage and Lore Master stay at their level-11 third tier
//!     (+3, 2/day; their own next tier lands at bard level 17, confirmed
//!     directly against the raw level-17 table row "inspire courage +4,
//!     lore master 3/day", out of scope).
//!   - Inspire Heroics' flat save-bonus (+4) and AC-bonus (+4) magnitudes
//!     and base target count (1) all carry over unchanged (the "+1
//!     creature per three bard levels beyond 15th" scaling lands at bard
//!     level 18, beyond this bounded slice's ceiling).
//!   - Soothing Performance carries over unchanged as a bounded grant-only
//!     identity record.
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing, condition-resolution, fear-resolution, or
//! save/AC-application engine (including Inspire Heroics' own targeting and
//! effect application), or the spontaneous spell posture burden (all stay
//! named-but-unproven, unchanged from levels 1-15); the spontaneous
//! spell-level-access ladder and the base spells-per-day / spells-known
//! table lookups stay at their pre-existing level-10 ceiling exactly as
//! left by the level-11 through level-15 cycles (no 5th-level spell-access
//! threshold is grounded), and it does not ground Bard level 17+. It also
//! preserves the accepted Bard level-1..level-15 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control. No new
//! tier constant, record type, or choice slot is added this slice.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level15_sd18_widening_deterministic_input.txt"
);

const BARD_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level16_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus and good saves genuinely rise; poor Fortitude stays -----

#[test]
fn bard_level16_base_attack_and_good_saves_genuinely_rise() {
    let input = load(BARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Bard level 16 3/4-BAB progression (16 * 3 / 4) must genuinely rise to 12, up from 11 \
         at level 15: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Bard level 16 poor Fortitude (16/3) must stay 5, an integer-division coincidence with \
         level 15"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 10,
        "Bard level 16 good Reflex (16/2+2) must genuinely rise to 10, up from 9 at level 15"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 10,
        "Bard level 16 good Will (16/2+2) must genuinely rise to 10, up from 9 at level 15"
    );
}

// ----- Bardic Knowledge and Bardic Performance rounds both genuinely rise -----

#[test]
fn bard_level16_knowledge_and_rounds_genuinely_rise() {
    let input = load(BARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 8,
        "Bardic Knowledge (max(16/2, 1)) must genuinely rise to 8, up from 7 at level 15"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 37,
        "Bard level 16 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (16 - 1)) must \
         genuinely rise to 37, up from 35 at level 15: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 21,
        "the Fascinate DC (10 + 16/2 + Charisma modifier 3) must genuinely rise to 21, up from \
         20 at level 15"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 6,
        "the Fascinate affected-creature count (1 + (16-1)/3) must genuinely rise to 6, up from \
         5 at level 15"
    );
}

// ----- Frightening Tune's flat DC magnitude genuinely rises -----

#[test]
fn bard_level16_frightening_tune_dc_genuinely_rises() {
    let input = load(BARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, FRIGHTENING_TUNE_DC_ID);
    assert_eq!(
        dc.value, 21,
        "Frightening Tune's Will save DC (10 + 1/2 bard level + Charisma modifier) must \
         genuinely rise to 21 at level 16, the same integer-division rise as the Fascinate DC: \
         {}",
        dc.detail
    );
}

// ----- Inspire Competence stays at its level-15 fourth tier -----

#[test]
fn bard_level16_inspire_competence_stays_fourth_tier() {
    let input = load(BARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 5,
        "Inspire Competence must stay +5 at level 16 (the next tier lands at bard level 19, out \
         of scope): {}",
        inspire_competence.detail
    );
}

// ----- Inspire Courage and Lore Master stay at their level-11 third tier -----

#[test]
fn bard_level16_inspire_courage_and_lore_master_stay_third_tier() {
    let input = load(BARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(
        inspire_courage.value, 3,
        "Inspire Courage must stay +3 at level 16 (the next tier lands at bard level 17, out of \
         scope): {}",
        inspire_courage.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 2,
        "Lore Master's flat take-20 usage count must stay 2/day at level 16 (the next tier \
         lands at bard level 17, out of scope): {}",
        lore_master.detail
    );
}

// ----- Inspire Heroics' flat magnitudes carry over unchanged -----

#[test]
fn bard_level16_inspire_heroics_carries_over_unchanged() {
    let input = load(BARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let save_bonus = explanation(&computation, INSPIRE_HEROICS_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 4,
        "Inspire Heroics' morale bonus on saving throws must stay a flat +4 at level 16: {}",
        save_bonus.detail
    );

    let ac_bonus = explanation(&computation, INSPIRE_HEROICS_AC_BONUS_ID);
    assert_eq!(
        ac_bonus.value, 4,
        "Inspire Heroics' dodge bonus to AC must stay a flat +4 at level 16: {}",
        ac_bonus.detail
    );

    let target_count = explanation(&computation, INSPIRE_HEROICS_TARGET_COUNT_ID);
    assert_eq!(
        target_count.value, 1,
        "Inspire Heroics' base target count must stay 1 at level 16, before the \
         every-three-levels-beyond-15th scaling (lands at level 18, out of scope): {}",
        target_count.detail
    );
}

// ----- Soothing Performance carries over unchanged -----

#[test]
fn bard_level16_soothing_performance_carries_over() {
    let input = load(BARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must carry over as a bounded +0 grant-only identity record at \
         level 16: {}",
        soothing_performance.detail
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 16 -----

#[test]
fn bard_level16_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL16_FIXTURE);
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
                "level-16 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-15 fixture is unaffected by this widening -----

#[test]
fn bard_level15_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 11, "Bard level 15 base attack bonus must stay 11");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 35, "Bard level 15 Bardic Performance rounds must stay 35");

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(dc.value, 20, "Bard level 15 Fascinate DC must stay 20");
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level16_recognition() {
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
fn multiclass_bard_level16_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL16_FIXTURE.replace(
        "class_level=class:bard:16",
        "class_level=class:bard:16\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-16 widening -----

#[test]
fn matrix_bard_row_names_level_16_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level16_widening"),
        "bard row must cite the live SD18 level-16 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 16") || note.contains("level-16"),
        "bard partial note must name the level-16 widening: {note}"
    );
}
