//! SD18 Bard level-13 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-12 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level12_widening.rs`, the SD18 loop's own
//! prior ceiling) to Bard level 13 — the loop's seventh §3.2 level-13
//! landing (after Rogue, Barbarian, Fighter, Ranger, Cleric, and Druid),
//! and the first level-13 landing on a spontaneous (non-9-level) caster —
//! mirroring the sibling per-level-gate idiom (`supported_bard_level` is
//! generalized from `1..=12` to `1..=13` via `MAX_SUPPORTED_BARD_LEVEL =
//! 13`). All three PF1 CRB primary sources (d20pfsrd, the Archives of
//! Nethys aonprd.com mirror, and legacy.aonprd.com) were read directly
//! before writing any code or test, and agree byte-for-byte:
//!
//! - level 13 base attack bonus STAYS +9 (`13 * 3 / 4 = 9`, an
//!   integer-division coincidence with level 12) and all three base saves
//!   STAY unchanged: Fortitude +4 (poor, `13 / 3 = 4`), Reflex +8 and Will
//!   +8 (both good, `13 / 2 + 2 = 8`) — confirmed by the same formulas
//!   already grounded at levels 1-12, not re-derived.
//! - Bardic Knowledge STAYS 6 (`max(13 / 2, 1) = 6`, an integer-division
//!   coincidence with level 12); the Bardic Performance rounds-per-day
//!   pool GENUINELY RISES to 30 (`4 + Charisma modifier 2 + 2 x (13 - 1)`);
//!   the Fascinate DC STAYS 18 (`10 + 13 / 2 + Charisma modifier 2`, an
//!   integer-division coincidence with level 12, since `13 / 2 == 12 / 2
//!   == 6`); the Fascinate affected-creature count GENUINELY RISES to 5
//!   (`1 + (13 - 1) / 3`, up from 4 at level 12).
//! - the PF1 Core Rulebook Bard class table's level-13 "Special" column is
//!   BLANK (verified independently against all three primary sources,
//!   checked rather than assumed): no new named class feature is granted
//!   at 13th level, so this is a pure arithmetic-pillar widening — no new
//!   subsystem, and no new grant-only identity record is added.
//! - Inspire Courage, Inspire Competence, Lore Master, Well-Versed,
//!   Jack-of-All-Trades, and Soothing Performance all stay unchanged at
//!   their level-12 tiers (their own next tiers land at bard level 15 or
//!   17, out of scope).
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing or condition-resolution engine, or the spontaneous spell
//! posture burden (all stay named-but-unproven, unchanged from levels
//! 1-12); the spontaneous spell-level-access ladder and the base
//! spells-per-day / spells-known table lookups stay at their pre-existing
//! level-10 ceiling exactly as left by the level-11 and level-12 cycles
//! (neither table's match arm is widened by this slice — no 5th-level
//! spell-access threshold is grounded), and it does not ground Bard level
//! 14+. It also preserves the accepted Bard level-1..level-12 truth
//! (unchanged), the Fighter negative control, and the multiclass negative
//! control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level12_sd18_widening_deterministic_input.txt"
);

const BARD_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level13_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";
const JACK_OF_ALL_TRADES_ID: &str = "class_feature.bard.jack_of_all_trades";
const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const SOOTHING_PERFORMANCE_ID: &str = "class_feature.bard.soothing_performance";

// ----- Base attack bonus and saves stay unchanged at level 13 (integer-division coincidences) -----

#[test]
fn bard_level13_base_attack_and_saves_stay_unchanged() {
    let input = load(BARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Bard level 13 3/4-BAB progression (13 * 3 / 4) must equal 9, an integer-division \
         coincidence with level 12: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Bard level 13 poor Fortitude (13/3) must stay 4, an integer-division coincidence \
         with level 12"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 8,
        "Bard level 13 good Reflex (13/2+2) must stay 8, an integer-division coincidence \
         with level 12"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(
        will.value, 8,
        "Bard level 13 good Will (13/2+2) must stay 8, an integer-division coincidence with \
         level 12"
    );
}

// ----- Bardic Performance rounds and the Fascinate affected-creature count genuinely rise -----

#[test]
fn bard_level13_performance_rounds_and_fascinate_count_genuinely_rise() {
    let input = load(BARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 6,
        "Bardic Knowledge (max(13/2, 1)) must stay 6, an integer-division coincidence with \
         level 12"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 31,
        "Bard level 13 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (13 - 1)) \
         must genuinely rise to 31, up from 29 at level 12: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 19,
        "the Fascinate DC (10 + 13/2 + Charisma modifier 3) must stay 19, an \
         integer-division coincidence with level 12 (13/2 == 12/2 == 6)"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 5,
        "the Fascinate affected-creature count (1 + (13-1)/3) must genuinely rise to 5, up \
         from 4 at level 12"
    );
}

// ----- Inspire Courage, Inspire Competence, Lore Master, Well-Versed carry over -----

#[test]
fn bard_level13_third_tier_magnitudes_carry_over_unchanged() {
    let input = load(BARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 3,
        "Inspire Courage must stay +3 at level 13 (the next tier lands at bard level 17, out \
         of scope): {}",
        inspire_courage.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 4,
        "Inspire Competence must stay +4 at level 13 (the next tier lands at bard level 15, \
         out of scope): {}",
        inspire_competence.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 2,
        "Lore Master's flat take-20 usage count must stay 2/day at level 13 (the next tier \
         lands at bard level 17, out of scope): {}",
        lore_master.detail
    );

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(well_versed.value, 4, "Well-Versed must stay the flat +4 at level 13");
}

// ----- Soothing Performance and Jack-of-All-Trades carry over unchanged -----

#[test]
fn bard_level13_soothing_performance_and_jack_of_all_trades_carry_over() {
    let input = load(BARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must carry over as a bounded +0 grant-only identity record at \
         level 13: {}",
        soothing_performance.detail
    );

    let joat = explanation(&computation, JACK_OF_ALL_TRADES_ID);
    assert_eq!(
        joat.value, 0,
        "Jack-of-All-Trades must carry over as a +0 identity/recognition record at level 13: {}",
        joat.detail
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 13 -----

#[test]
fn bard_level13_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL13_FIXTURE);
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
                "level-13 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-12 fixture is unaffected by this widening -----

#[test]
fn bard_level12_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Bard level 12 base attack bonus must stay 9");

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    assert_eq!(rounds.value, 29, "Bard level 12 Bardic Performance rounds must stay 29");

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(count.value, 4, "Bard level 12 Fascinate affected-creature count must stay 4");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (Bard levels 14 through 18 were widened into scope by later SD18 slices —
// up through tests/sd18_bard_level18_widening.rs — so this negative
// control's boundary moves from 14 to 19, mirroring the exact same
// boundary-move idiom applied to every prior sibling class's own level
// widening cycle, then to 20 by the SD18 bard-level19-widening cycle, then
// to 21 (a pure implementation-gate check, since PF1 has no 21st character
// level) by the SD18 bard-level20-widening cycle.)

#[test]
fn bard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARD_LEVEL13_FIXTURE.replace("class:bard:13", "class:bard:21");
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
                && e.id != "class_feature.bard.bardic_performance_execution.not_performing"
                // SD-34 wave 34 lane A (`docs/release/SD-34-book-completion/artifacts/
                // bucket-d-mining/wave34_laneA_weapon_and_armor_proficiency_cycle_
                // receipt.md`): Bard's own Weapon and Armor Proficiency identity
                // grant is now genuinely grounded as a level-independent, always-on
                // +0 record (true since level 1, mirrors the same "no gate to lift"
                // idiom as Jack-of-All-Trades) -- not a bounded, level-gated feature
                // this slice's negative control is checking for.
                && e.id != "class_feature.bard.weapon_and_armor_proficiency"),
        "level-21 Bard must not gain any bounded bard explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the bard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_bard_level13_recognition() {
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
fn multiclass_bard_level13_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL13_FIXTURE.replace(
        "class_level=class:bard:13",
        "class_level=class:bard:13\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-13 widening -----

#[test]
fn matrix_bard_row_names_level_13_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level13_widening"),
        "bard row must cite the live SD18 level-13 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 13") || note.contains("level-13"),
        "bard partial note must name the level-13 widening: {note}"
    );
}
