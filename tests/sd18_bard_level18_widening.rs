//! SD18 Bard level-18 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-17 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level17_widening.rs`, the SD18 loop's own
//! prior ceiling) to Bard level 18 — the loop's NINTH §3.2 level-18
//! landing (after Wizard, Cleric, Paladin, Fighter, Barbarian, Rogue,
//! Ranger, and Sorcerer) and, if it lands, the CLOSE of the §3.2 level-18
//! sweep at 9 of 9 eligible classes (Druid capped at 15, Monk capped at 12
//! as documented structural exceptions) — mirroring the sibling
//! per-level-gate idiom (`supported_bard_level` is generalized from
//! `1..=17` to `1..=18` via `MAX_SUPPORTED_BARD_LEVEL = 18`). TWO primary
//! PF1 CRB sources were read fresh this cycle before writing any code or
//! test: a raw HTML parse of d20pfsrd.com's own class table (bypassing
//! AI-summarization, following the lesson from the Sorcerer level-18
//! cycle) and the Archives of Nethys aonprd.com mirror via
//! `ClassDisplay.aspx`, fetching the levels-16-through-19 block in one
//! pass so the level-18 row's neighbors were visible in context (guards
//! against level-misattribution): level 16 blank, level 17 "Inspire
//! courage +4, lore master 3/day", level 18 "Mass suggestion, versatile
//! performance", level 19 "Inspire competence +6" — both sources agree
//! byte-for-byte on the level-18 row ("+13/+8/+3 | +6 | +11 | +11 | Mass
//! suggestion, versatile performance"), and no source disagreement was
//! found (a third source was not required).
//!
//! - level 18 base attack bonus GENUINELY RISES to +13 (`18 * 3 / 4 = 13`,
//!   up from +12 at level 17) and both good saves (Reflex, Will) GENUINELY
//!   RISE to +11 (`18 / 2 + 2 = 11`, up from +10), while poor Fortitude
//!   GENUINELY RISES to +6 (`18 / 3 = 6`, up from +5) — every base-chassis
//!   pillar genuinely rises at level 18, confirmed directly against both
//!   primary sources rather than assumed.
//! - Bardic Knowledge GENUINELY RISES to 9 (`max(18 / 2, 1) = 9`, up from
//!   8 at level 17).
//! - the Bardic Performance rounds-per-day pool GENUINELY RISES to 40
//!   (`4 + Charisma modifier 2 + 2 x (18 - 1)`, up from 38 at level 17).
//! - the Fascinate DC GENUINELY RISES to 21 (`10 + 18/2 + Charisma
//!   modifier 2`), while the Fascinate affected-creature count STAYS 6
//!   (`1 + (18 - 1) / 3 = 6`, an integer-division coincidence with level
//!   17).
//! - Frightening Tune's DC (the same formula shape as the Fascinate DC)
//!   likewise GENUINELY RISES to 21.
//! - Inspire Courage stays at its level-17 fourth tier (+4; no further
//!   tier is defined within this bounded slice's ceiling).
//! - Inspire Competence stays at its level-15 fourth tier (+5; its next
//!   tier lands at bard level 19, out of scope).
//! - Lore Master stays at its level-17 third tier (3/day take-20 uses; no
//!   further tier is defined within this bounded slice's ceiling).
//! - Inspire Heroics' flat save-bonus (+4) and AC-bonus (+4) magnitudes
//!   stay unchanged, but its base target count GENUINELY RISES from 1 to
//!   2 — the PF1 Core Rulebook's own text ("for every three bard levels
//!   the character attains beyond 15th, he can inspire heroics in one
//!   additional creature") places this exactly at bard level 18, verified
//!   against the rule text directly (fetched fresh this cycle) rather
//!   than assumed from the level-15/level-17 cycles' own "out of scope"
//!   notes, which correctly deferred this exact threshold to the cycle
//!   that widens the gate to 18. This is a genuine arithmetic-pillar
//!   widening on an already-generalized tiered if/else chain, the same
//!   idiom as Inspire Courage's/Inspire Competence's/Lore Master's own
//!   tier additions — mirroring the Fascinate affected-creature-count
//!   idiom's own tiered-threshold shape — needing no new grounding
//!   machinery beyond one new tier constant pair.
//! - Soothing Performance carries over unchanged as a bounded grant-only
//!   identity record.
//!
//! The level-18 "Special" column's two named entries are checked and
//! confirmed to require the SAME already-declined machinery as their own
//! precedents, so NEITHER is grounded as a new record:
//! - "Mass suggestion" is verified against its own PF1 rule text ("This
//!   ability functions just like suggestion, but allows a bard of 18th
//!   level or higher to make a suggestion simultaneously to any number of
//!   creatures that he has already fascinated") to be a strict widening
//!   of the 6th-level Suggestion spell-like ability, which was already
//!   deliberately left named-but-unproven at level 6
//!   (`tests/sd13_bard_level6_progression.rs`,
//!   `bard_level6_does_not_fabricate_suggestion_or_versatile_performance`)
//!   because it requires a fascinated-target prerequisite and the
//!   "suggestion" spell's own effect-resolution engine, neither of which
//!   exists in this codebase. Mass Suggestion inherits the identical
//!   blocker and adds a multi-target dimension on top of it (which would
//!   also require reading the already-grounded Fascinate
//!   affected-creature-count into an actual targeting resolution, itself
//!   unproven) — so it is deliberately left named-but-unproven, with no
//!   explanation or diagnostic record fabricated for it, exactly
//!   mirroring the level-6 Suggestion precedent. This is NOT a new
//!   spell-like-ability-casting engine being declined for the first time;
//!   it is the SAME already-declined engine, re-confirmed.
//! - "Versatile performance" is a REPEAT of the Bard's own 2nd-level
//!   grant (also seen at levels 6, 10, and 14): it was already
//!   deliberately left named-but-unproven at level 2 (requires a
//!   choice-gated skill-substitution engine that does not exist in this
//!   codebase), so this cycle adds no new record for its level-18
//!   reappearance either — checked directly rather than assumed,
//!   confirming it is the same repeat-grant idiom as every prior
//!   Versatile Performance reappearance, not a new-shape feature.
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing, condition-resolution, fear-resolution, spell-like-ability
//! casting/targeting, or save/AC-application engine (including Inspire
//! Courage's own competence/morale-bonus application and Lore Master's own
//! take-10/take-20 skill-check-resolution execution), or the spontaneous
//! spell posture burden (all stay named-but-unproven, unchanged from
//! levels 1-17); the spontaneous spell-level-access ladder and the base
//! spells-per-day / spells-known table lookups stay at their pre-existing
//! level-10 ceiling exactly as left by the level-11 through level-17
//! cycles (no 5th-level spell-access threshold is grounded), and it does
//! not ground Bard level 19+. It also preserves the accepted Bard
//! level-1..level-17 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. No new record type or choice slot is
//! added this slice — only one new tier constant pair on an
//! already-generalized tiered if/else chain (Inspire Heroics' target
//! count).

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level17_sd18_widening_deterministic_input.txt"
);

const BARD_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level18_sd18_widening_deterministic_input.txt"
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

// ----- Base attack bonus and all three base saves genuinely rise -----

#[test]
fn bard_level18_base_attack_and_saves_genuinely_rise() {
    let input = load(BARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 13,
        "Bard level 18 3/4-BAB progression (18 * 3 / 4) must genuinely rise to 13, up from 12 \
         at level 17: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Bard level 18 poor Fortitude (18/3) must genuinely rise to 6, up from 5 at level 17"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 11,
        "Bard level 18 good Reflex (18/2+2) must genuinely rise to 11, up from 10 at level 17"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 11,
        "Bard level 18 good Will (18/2+2) must genuinely rise to 11, up from 10 at level 17"
    );
}

// ----- Bardic Knowledge and Bardic Performance rounds both genuinely rise -----

#[test]
fn bard_level18_knowledge_and_rounds_genuinely_rise() {
    let input = load(BARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 9,
        "Bardic Knowledge (max(18/2, 1)) must genuinely rise to 9, up from 8 at level 17"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 41,
        "Bard level 18 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (18 - 1)) must \
         genuinely rise to 41, up from 39 at level 17: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 22,
        "the Fascinate DC (10 + 18/2 + Charisma modifier 3) must genuinely rise to 22, up from \
         21 at level 17: {}",
        dc.detail
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 6,
        "the Fascinate affected-creature count (1 + (18-1)/3) must stay 6, an integer-division \
         coincidence with level 17"
    );
}

// ----- Frightening Tune's flat DC magnitude genuinely rises -----

#[test]
fn bard_level18_frightening_tune_dc_genuinely_rises() {
    let input = load(BARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = explanation(&computation, FRIGHTENING_TUNE_DC_ID);
    assert_eq!(
        dc.value, 22,
        "Frightening Tune's Will save DC (10 + 1/2 bard level + Charisma modifier) must \
         genuinely rise to 22 at level 18, the same formula shape as the Fascinate DC: {}",
        dc.detail
    );
}

// ----- Inspire Courage, Inspire Competence, and Lore Master all stay at their level-17 tier -----

#[test]
fn bard_level18_inspire_and_lore_master_stay_unchanged() {
    let input = load(BARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(
        inspire_courage.value, 4,
        "Inspire Courage must stay +4 at level 18 (no further tier is defined within this \
         bounded slice's ceiling): {}",
        inspire_courage.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 5,
        "Inspire Competence must stay +5 at level 18 (the next tier lands at bard level 19, out \
         of scope): {}",
        inspire_competence.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 3,
        "Lore Master's flat take-20 usage count must stay 3/day at level 18 (no further tier is \
         defined within this bounded slice's ceiling): {}",
        lore_master.detail
    );
}

// ----- Inspire Heroics' save/AC bonuses carry over, but its target count genuinely rises -----

#[test]
fn bard_level18_inspire_heroics_target_count_genuinely_rises() {
    let input = load(BARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let save_bonus = explanation(&computation, INSPIRE_HEROICS_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 4,
        "Inspire Heroics' morale bonus on saving throws must stay a flat +4 at level 18: {}",
        save_bonus.detail
    );

    let ac_bonus = explanation(&computation, INSPIRE_HEROICS_AC_BONUS_ID);
    assert_eq!(
        ac_bonus.value, 4,
        "Inspire Heroics' dodge bonus to AC must stay a flat +4 at level 18: {}",
        ac_bonus.detail
    );

    let target_count = explanation(&computation, INSPIRE_HEROICS_TARGET_COUNT_ID);
    assert_eq!(
        target_count.value, 2,
        "Inspire Heroics' base target count must genuinely rise to 2 at level 18 (PF1 Core \
         Rulebook: \"for every three bard levels the character attains beyond 15th, he can \
         inspire heroics in one additional creature\", landing exactly at level 18): {}",
        target_count.detail
    );
}

// ----- Soothing Performance carries over unchanged -----

#[test]
fn bard_level18_soothing_performance_carries_over() {
    let input = load(BARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must carry over as a bounded +0 grant-only identity record at \
         level 18: {}",
        soothing_performance.detail
    );
}

// ----- Mass Suggestion and the level-18 Versatile Performance repeat: checked, confirmed NOT flat, deliberately unproven -----

#[test]
fn bard_level18_does_not_fabricate_mass_suggestion_or_versatile_performance() {
    let input = load(BARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("suggestion")
                // AT-34-E3-001 option-pool-with-magnitude cycle 5 (`b4eadc9cbf`, 2026-08-28)
                // grounded Suggestion's and Mass Suggestion's flat Will-save DC magnitude
                // (real corpus-cited `BONUS:VAR|...DC|10+(BardicPerformanceLVL/2)+CHA`
                // formula) -- explicitly NOT the full effect: no range/audible-performance
                // checking, no Will-save resolution, no suggestion-effect application. The
                // DC number is real, tested, non-fabricated content; only the execution
                // burden this test's own name still correctly guards stays unproven.
                && e.id != "class_feature.bard.suggestion_dc"
                && e.id != "class_feature.bard.mass_suggestion_dc"),
        "Mass Suggestion (the PF1 CRB's 18th-level Bard spell-like ability, \"functions just \
         like suggestion, but allows a bard of 18th level or higher to make a suggestion \
         simultaneously to any number of creatures that he has already fascinated\") inherits \
         the same fascinated-target prerequisite and the \"suggestion\" spell's own \
         effect-resolution engine already declined at level 6, neither of which exists in this \
         codebase; no explanation record must be fabricated for it BEYOND the two grounded DC \
         magnitudes: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("suggestion")),
        "no diagnostic record should be fabricated for Mass Suggestion either, since this slice \
         deliberately declines to ground it, mirroring the level-6 Suggestion precedent: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("versatile_performance")),
        "the 18th-level Versatile Performance grant is the same choice-gated \
         skill-substitution engine already deliberately left named-but-unproven at 2nd, 6th, \
         10th, and 14th level; no explanation record must be fabricated for it: {:?}",
        computation.explanations
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 18 -----

#[test]
fn bard_level18_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL18_FIXTURE);
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
                "level-18 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-17 fixture is unaffected by this widening -----

#[test]
fn bard_level17_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, INSPIRE_COURAGE_ID);
    assert_eq!(inspire_courage.value, 4, "Bard level 17 Inspire Courage must stay +4");

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(lore_master.value, 3, "Bard level 17 Lore Master must stay 3/day");

    let target_count = explanation(&computation, INSPIRE_HEROICS_TARGET_COUNT_ID);
    assert_eq!(
        target_count.value, 1,
        "Bard level 17 Inspire Heroics target count must stay 1, before the level-18 threshold"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 39, "Bard level 17 Bardic Performance rounds must stay 39");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
//
// SD18 (tests/sd18_bard_level19_widening.rs) further widened the bounded
// tranche from level 18 to level 19 (Inspire Competence's fifth tier, +6),
// so this negative control moved to sit just above that bound (level 20)
// rather than at level 19; the SD18 bard-level20-widening cycle then
// widened the tranche to level 20 (Deadly Performance's flat Will-save DC),
// so this negative control moves again to level 21 (a pure
// implementation-gate check, since PF1 has no 21st character level).

#[test]
fn bard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARD_LEVEL18_FIXTURE.replace("class:bard:18", "class:bard:21");
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
fn fighter_does_not_gain_bard_level18_recognition() {
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
fn multiclass_bard_level18_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL18_FIXTURE.replace(
        "class_level=class:bard:18",
        "class_level=class:bard:18\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-18 widening -----

#[test]
fn matrix_bard_row_names_level_18_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level18_widening"),
        "bard row must cite the live SD18 level-18 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 18") || note.contains("level-18"),
        "bard partial note must name the level-18 widening: {note}"
    );
}
