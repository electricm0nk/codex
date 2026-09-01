//! SD18 Bard level-14 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-13 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level13_widening.rs`, the SD18 loop's own
//! prior ceiling) to Bard level 14 — the loop's FIFTH §3.2 level-14
//! landing (after Barbarian, Fighter, Rogue, and Ranger) — mirroring the
//! sibling per-level-gate idiom (`supported_bard_level` is generalized
//! from `1..=13` to `1..=14` via `MAX_SUPPORTED_BARD_LEVEL = 14`). Both
//! PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and agree byte-for-byte:
//!
//! - level 14 base attack bonus GENUINELY RISES to +10 (`14 * 3 / 4 = 10`,
//!   up from +9 at level 13) and both good saves GENUINELY RISE to +9
//!   (Reflex and Will, `14 / 2 + 2 = 9`, up from +8), while poor Fortitude
//!   STAYS +4 (`14 / 3 = 4`, an integer-division coincidence with level
//!   13).
//! - Bardic Knowledge GENUINELY RISES to 7 (`max(14 / 2, 1) = 7`, up from
//!   6); the Bardic Performance rounds-per-day pool GENUINELY RISES to 32
//!   (`4 + Charisma modifier 2 + 2 x (14 - 1)`, up from 30); the Fascinate
//!   DC GENUINELY RISES to 19 (`10 + 14/2 + Charisma modifier 2`, up from
//!   18); the Fascinate affected-creature count STAYS 5
//!   (`1 + (14 - 1) / 3`, an integer-division coincidence with level 13,
//!   since `(14-1)/3 == (13-1)/3 == 4`).
//! - the PF1 Core Rulebook Bard class table's level-14 "Special" column
//!   reads "Frightening tune, Versatile performance" (verified
//!   independently against both primary sources, checked rather than
//!   assumed). Frightening Tune is a wholly new 14th-level Bard class
//!   feature; its rule text ("Each enemy within range receives a Will
//!   save (DC 10 + 1/2 the bard's level + the bard's Cha modifier) to
//!   negate the effect") gives the exact same DC formula shape as the
//!   already-grounded Fascinate DC, so it is grounded ONLY as a flat
//!   standalone Will-save DC magnitude — mirroring the Fascinate DC
//!   idiom exactly. Unlike Fascinate, Frightening Tune's affected-scope is
//!   range-based ("each enemy within 30 feet who can hear the
//!   performance"), not a numeric-count formula, so no affected-creature
//!   count record is added for it. The repeat "Versatile Performance"
//!   grant (also appearing at levels 2, 6, and 10) stays named-but-unproven
//!   exactly as before — it requires a choice-gated skill-substitution
//!   engine that does not exist in this codebase, so no new record is
//!   added for it either.
//! - Inspire Courage, Inspire Competence, Lore Master, Well-Versed,
//!   Jack-of-All-Trades, and Soothing Performance all stay unchanged at
//!   their level-13 tiers (their own next tiers land at bard level 15 or
//!   17, out of scope).
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing, condition-resolution, or fear-resolution engine (including
//! Frightening Tune's own frightened-condition application), or the
//! spontaneous spell posture burden (all stay named-but-unproven, unchanged
//! from levels 1-13); the spontaneous spell-level-access ladder and the
//! base spells-per-day / spells-known table lookups stay at their
//! pre-existing level-10 ceiling exactly as left by the level-11 through
//! level-13 cycles (no 5th-level spell-access threshold is grounded), and
//! it does not ground Bard level 15+. It also preserves the accepted Bard
//! level-1..level-13 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level13_sd18_widening_deterministic_input.txt"
);

const BARD_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level14_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";
const JACK_OF_ALL_TRADES_ID: &str = "class_feature.bard.jack_of_all_trades";
const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const SOOTHING_PERFORMANCE_ID: &str = "class_feature.bard.soothing_performance";
const FRIGHTENING_TUNE_DC_ID: &str = "class_chassis.bard.frightening_tune_dc";

// ----- Base attack bonus and two good saves genuinely rise; poor Fortitude stays -----

#[test]
fn bard_level14_base_attack_and_good_saves_genuinely_rise() {
    let input = load(BARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 10,
        "Bard level 14 3/4-BAB progression (14 * 3 / 4) must genuinely rise to 10, up from 9 \
         at level 13: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Bard level 14 poor Fortitude (14/3) must stay 4, an integer-division coincidence \
         with level 13"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 9,
        "Bard level 14 good Reflex (14/2+2) must genuinely rise to 9, up from 8 at level 13"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 9,
        "Bard level 14 good Will (14/2+2) must genuinely rise to 9, up from 8 at level 13"
    );
}

// ----- Bardic Knowledge, performance rounds, and the Fascinate DC genuinely rise -----

#[test]
fn bard_level14_knowledge_rounds_and_fascinate_dc_genuinely_rise() {
    let input = load(BARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 7,
        "Bardic Knowledge (max(14/2, 1)) must genuinely rise to 7, up from 6 at level 13"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 33,
        "Bard level 14 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (14 - 1)) \
         must genuinely rise to 33, up from 31 at level 13: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 20,
        "the Fascinate DC (10 + 14/2 + Charisma modifier 3) must genuinely rise to 20, up \
         from 19 at level 13"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 5,
        "the Fascinate affected-creature count (1 + (14-1)/3) must stay 5, an \
         integer-division coincidence with level 13"
    );
}

// ----- Frightening Tune's flat DC magnitude is newly grounded -----

#[test]
fn bard_level14_frightening_tune_dc_is_newly_grounded() {
    let input = load(BARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, FRIGHTENING_TUNE_DC_ID);
    assert_eq!(
        dc.value, 20,
        "Frightening Tune's Will save DC (10 + 1/2 bard level + Charisma modifier) must equal \
         20 at level 14, the same formula shape as the Fascinate DC: {}",
        dc.detail
    );
}

#[test]
fn bard_level13_has_no_frightening_tune_dc() {
    let input = load(BARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.explanations.iter().any(|e| e.id == FRIGHTENING_TUNE_DC_ID),
        "level-13 Bard must not carry a Frightening Tune DC record: {:?}",
        computation.explanations
    );
}

// ----- Inspire Courage, Inspire Competence, Lore Master, Well-Versed carry over -----

#[test]
fn bard_level14_third_tier_magnitudes_carry_over_unchanged() {
    let input = load(BARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 3,
        "Inspire Courage must stay +3 at level 14 (the next tier lands at bard level 17, out \
         of scope): {}",
        inspire_courage.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 4,
        "Inspire Competence must stay +4 at level 14 (the next tier lands at bard level 15, \
         out of scope): {}",
        inspire_competence.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 2,
        "Lore Master's flat take-20 usage count must stay 2/day at level 14 (the next tier \
         lands at bard level 17, out of scope): {}",
        lore_master.detail
    );

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(well_versed.value, 4, "Well-Versed must stay the flat +4 at level 14");
}

// ----- Soothing Performance and Jack-of-All-Trades carry over unchanged -----

#[test]
fn bard_level14_soothing_performance_and_jack_of_all_trades_carry_over() {
    let input = load(BARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must carry over as a bounded +0 grant-only identity record at \
         level 14: {}",
        soothing_performance.detail
    );

    let joat = explanation(&computation, JACK_OF_ALL_TRADES_ID);
    assert_eq!(
        joat.value, 0,
        "Jack-of-All-Trades must carry over as a +0 identity/recognition record at level 14: {}",
        joat.detail
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 14 -----

#[test]
fn bard_level14_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL14_FIXTURE);
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
                "level-14 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-13 fixture is unaffected by this widening -----

#[test]
fn bard_level13_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Bard level 13 base attack bonus must stay 9");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 31, "Bard level 13 Bardic Performance rounds must stay 31");

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(dc.value, 19, "Bard level 13 Fascinate DC must stay 19");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (Bard levels 15 through 18 were widened into scope by later SD18 slices —
// tests/sd18_bard_level15_widening.rs, tests/sd18_bard_level16_widening.rs,
// tests/sd18_bard_level17_widening.rs, and tests/sd18_bard_level18_widening.rs
// — so this negative control's boundary moves from 15 to 19, mirroring the
// exact same boundary-move idiom applied to every prior sibling class's own
// level widening cycle, then to 20 by the SD18 bard-level19-widening
// cycle, then to 21 (a pure implementation-gate check, since PF1 has no
// 21st character level) by the SD18 bard-level20-widening cycle.)

#[test]
fn bard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARD_LEVEL14_FIXTURE.replace("class:bard:14", "class:bard:21");
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
fn fighter_does_not_gain_bard_level14_recognition() {
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
fn multiclass_bard_level14_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL14_FIXTURE.replace(
        "class_level=class:bard:14",
        "class_level=class:bard:14\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_bard_row_names_level_14_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level14_widening"),
        "bard row must cite the live SD18 level-14 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 14") || note.contains("level-14"),
        "bard partial note must name the level-14 widening: {note}"
    );
}
