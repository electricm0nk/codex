//! SD18 Bard level-15 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-14 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level14_widening.rs`, the SD18 loop's own
//! prior ceiling) to Bard level 15 — the loop's TENTH §3.2 level-15
//! landing (after Barbarian, Rogue, Fighter, Cleric, Druid, Ranger, Wizard,
//! Paladin, and Sorcerer), and the FINAL landing needed to close the §3.2
//! level-15 sweep at 10 of 10 non-Monk classes — mirroring the sibling
//! per-level-gate idiom (`supported_bard_level` is generalized from
//! `1..=14` to `1..=15` via `MAX_SUPPORTED_BARD_LEVEL = 15`). Both PF1 CRB
//! primary sources (d20pfsrd and the Archives of Nethys aonprd.com mirror)
//! were read directly before writing any code or test, and agree
//! byte-for-byte:
//!
//! - level 15 base attack bonus GENUINELY RISES to +11 (`15 * 3 / 4 = 11`,
//!   up from +10 at level 14) and poor Fortitude GENUINELY RISES to +5
//!   (`15 / 3 = 5`, up from +4), while both good saves (Reflex, Will) STAY
//!   +9 (`15 / 2 + 2 = 9`, an integer-division coincidence with level 14).
//! - Bardic Knowledge STAYS 7 (`max(15 / 2, 1) = 7`, an integer-division
//!   coincidence with level 14); the Bardic Performance rounds-per-day pool
//!   GENUINELY RISES to 34 (`4 + Charisma modifier 2 + 2 x (15 - 1)`, up
//!   from 32); the Fascinate DC STAYS 19 (`10 + 15/2 + Charisma modifier
//!   2`, an integer-division coincidence with level 14); the Fascinate
//!   affected-creature count STAYS 5 (`1 + (15 - 1) / 3`, an
//!   integer-division coincidence with level 14).
//! - Frightening Tune's DC (the same formula shape as the Fascinate DC)
//!   STAYS 19 for the same integer-division-coincidence reason.
//! - the PF1 Core Rulebook Bard class table's level-15 "Special" column
//!   reads "Inspire competence +5, inspire heroics" (verified
//!   independently against both primary sources, checked rather than
//!   assumed, resolving the open question flagged by the level-13 cycle's
//!   own doc comment about whether the Inspire Courage/Lore Master tier
//!   thresholds land at level 15 or 17). Two things are true at once:
//!   - Inspire Competence's flat magnitude GENUINELY RISES from +4 to +5
//!     (a fourth tier on the already-generalized tiered if/else chain, the
//!     same arithmetic-widening idiom as the third-tier addition at level
//!     11 — no new grounding machinery required).
//!   - Inspire Courage and Lore Master do NOT gain a new tier at level 15
//!     (their own next tiers land at bard level 17, confirmed directly
//!     against the rule text "every six bard levels thereafter" starting
//!     from their own 5th/11th-level second-tier grants) — both STAY at
//!     their level-11 third tier (Inspire Courage +3, Lore Master 2/day).
//!   - Inspire Heroics is a wholly new 15th-level Bard class feature
//!     ("A bard of 15th level or higher can inspire tremendous heroism in
//!     himself or a single ally within 30 feet... Inspired creatures gain
//!     a +4 morale bonus on saving throws and a +4 dodge bonus to AC.").
//!     Both magnitude numbers (+4 save bonus, +4 AC bonus) are flat and
//!     non-level-scaled at the level they are gained, mirroring the
//!     Well-Versed flat-magnitude idiom exactly; the base target count (a
//!     single creature at 15th level, before the "+1 creature per three
//!     bard levels beyond 15th" scaling, which lands beyond this bounded
//!     slice's ceiling) is grounded as a flat count, mirroring the
//!     Fascinate affected-creature-count idiom. No targeting, save
//!     resolution, AC application, or performance-state execution is
//!     grounded — those remain named-but-unproven, exactly like Frightening
//!     Tune and Soothing Performance before it.
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing, condition-resolution, fear-resolution, or
//! save/AC-application engine (including Inspire Heroics' own targeting and
//! effect application), or the spontaneous spell posture burden (all stay
//! named-but-unproven, unchanged from levels 1-14); the spontaneous
//! spell-level-access ladder and the base spells-per-day / spells-known
//! table lookups stay at their pre-existing level-10 ceiling exactly as
//! left by the level-11 through level-14 cycles (no 5th-level spell-access
//! threshold is grounded), and it does not ground Bard level 16+. It also
//! preserves the accepted Bard level-1..level-14 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level14_sd18_widening_deterministic_input.txt"
);

const BARD_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level15_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus and poor Fortitude genuinely rise; good saves stay -----

#[test]
fn bard_level15_base_attack_and_fortitude_genuinely_rise() {
    let input = load(BARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 11,
        "Bard level 15 3/4-BAB progression (15 * 3 / 4) must genuinely rise to 11, up from 10 \
         at level 14: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Bard level 15 poor Fortitude (15/3) must genuinely rise to 5, up from 4 at level 14"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 9,
        "Bard level 15 good Reflex (15/2+2) must stay 9, an integer-division coincidence with \
         level 14"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 9,
        "Bard level 15 good Will (15/2+2) must stay 9, an integer-division coincidence with \
         level 14"
    );
}

// ----- Bardic Knowledge stays, Bardic Performance rounds genuinely rise -----

#[test]
fn bard_level15_knowledge_stays_and_rounds_genuinely_rise() {
    let input = load(BARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 7,
        "Bardic Knowledge (max(15/2, 1)) must stay 7, an integer-division coincidence with \
         level 14"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 35,
        "Bard level 15 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (15 - 1)) must \
         genuinely rise to 35, up from 33 at level 14: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 20,
        "the Fascinate DC (10 + 15/2 + Charisma modifier 3) must stay 20, an integer-division \
         coincidence with level 14"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 5,
        "the Fascinate affected-creature count (1 + (15-1)/3) must stay 5, an integer-division \
         coincidence with level 14"
    );
}

// ----- Frightening Tune's flat DC magnitude carries over unchanged -----

#[test]
fn bard_level15_frightening_tune_dc_carries_over_unchanged() {
    let input = load(BARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, FRIGHTENING_TUNE_DC_ID);
    assert_eq!(
        dc.value, 20,
        "Frightening Tune's Will save DC (10 + 1/2 bard level + Charisma modifier) must stay 20 \
         at level 15, the same integer-division coincidence as the Fascinate DC: {}",
        dc.detail
    );
}

// ----- Inspire Competence genuinely rises to its fourth tier -----

#[test]
fn bard_level15_inspire_competence_fourth_tier_is_newly_grounded() {
    let input = load(BARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 5,
        "Inspire Competence must genuinely rise to +5 at level 15 (the fourth tier of the \
         already-generalized tiered magnitude, up from +4 at level 14; the next tier lands at \
         bard level 19, out of scope): {}",
        inspire_competence.detail
    );
}

#[test]
fn bard_level14_inspire_competence_stays_fourth_tier_unaffected() {
    let input = load(BARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 4,
        "Bard level 14 Inspire Competence must stay +4, unaffected by this slice's level-15 \
         fourth-tier addition"
    );
}

// ----- Inspire Courage and Lore Master do NOT gain a new tier at level 15 -----

#[test]
fn bard_level15_inspire_courage_and_lore_master_stay_third_tier() {
    let input = load(BARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(
        inspire_courage.value, 3,
        "Inspire Courage must stay +3 at level 15 (the next tier lands at bard level 17, out of \
         scope): {}",
        inspire_courage.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 2,
        "Lore Master's flat take-20 usage count must stay 2/day at level 15 (the next tier lands \
         at bard level 17, out of scope): {}",
        lore_master.detail
    );
}

// ----- Inspire Heroics is newly grounded as flat, non-level-scaled magnitudes -----

#[test]
fn bard_level15_inspire_heroics_is_newly_grounded() {
    let input = load(BARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let save_bonus = explanation(&computation, INSPIRE_HEROICS_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 4,
        "Inspire Heroics' morale bonus on saving throws must be a flat +4 at level 15: {}",
        save_bonus.detail
    );

    let ac_bonus = explanation(&computation, INSPIRE_HEROICS_AC_BONUS_ID);
    assert_eq!(
        ac_bonus.value, 4,
        "Inspire Heroics' dodge bonus to AC must be a flat +4 at level 15: {}",
        ac_bonus.detail
    );

    let target_count = explanation(&computation, INSPIRE_HEROICS_TARGET_COUNT_ID);
    assert_eq!(
        target_count.value, 1,
        "Inspire Heroics' base target count (himself or a single ally) must be 1 at level 15, \
         before the every-three-levels-beyond-15th scaling, which is out of scope: {}",
        target_count.detail
    );
}

#[test]
fn bard_level14_has_no_inspire_heroics() {
    let input = load(BARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == INSPIRE_HEROICS_SAVE_BONUS_ID
                || e.id == INSPIRE_HEROICS_AC_BONUS_ID
                || e.id == INSPIRE_HEROICS_TARGET_COUNT_ID),
        "level-14 Bard must not carry any Inspire Heroics record: {:?}",
        computation.explanations
    );
}

// ----- Soothing Performance carries over unchanged -----

#[test]
fn bard_level15_soothing_performance_carries_over() {
    let input = load(BARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must carry over as a bounded +0 grant-only identity record at \
         level 15: {}",
        soothing_performance.detail
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 15 -----

#[test]
fn bard_level15_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL15_FIXTURE);
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
                "level-15 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-14 fixture is unaffected by this widening -----

#[test]
fn bard_level14_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 10, "Bard level 14 base attack bonus must stay 10");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 33, "Bard level 14 Bardic Performance rounds must stay 33");

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(dc.value, 20, "Bard level 14 Fascinate DC must stay 20");
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level15_recognition() {
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
fn multiclass_bard_level15_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL15_FIXTURE.replace(
        "class_level=class:bard:15",
        "class_level=class:bard:15\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_bard_row_names_level_15_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level15_widening"),
        "bard row must cite the live SD18 level-15 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "bard partial note must name the level-15 widening: {note}"
    );
}
