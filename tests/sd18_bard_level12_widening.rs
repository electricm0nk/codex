//! SD18 Bard level-12 widening grounding proof.
//!
//! Widens the accepted Bard level-1..level-11 spontaneous spell-bearing
//! baseline (`tests/sd18_bard_level11_inspire_widening.rs`, the SD18 loop's
//! own prior ceiling) to Bard level 12 — the loop's second §3.2 level-12
//! widening (after Barbarian) and the first level-12 widening on a
//! spell-bearing class — mirroring the sibling per-level-gate idiom
//! (`supported_bard_level` is generalized from `1..=11` to `1..=12` via
//! `MAX_SUPPORTED_BARD_LEVEL = 12`). Both PF1 CRB primary sources
//! (d20pfsrd and the Archives of Nethys aonprd.com mirror) were read
//! directly before writing any code or test, and agree byte-for-byte:
//!
//! - level 12 base attack bonus is +9 (`12 * 3 / 4 = 9`, genuinely risen
//!   from +8) and base saves are +4 Fortitude (poor, `12 / 3 = 4`,
//!   genuinely risen from +3), +8 Reflex and +8 Will (both good,
//!   `12 / 2 + 2 = 8`, genuinely risen from +7) — confirmed by the same
//!   formulas already grounded at levels 1-11, not re-derived.
//! - Bardic Knowledge GENUINELY RISES to 6 (`max(12 / 2, 1)`); the Bardic
//!   Performance rounds-per-day pool GENUINELY RISES to 28
//!   (`4 + Charisma modifier 2 + 2 x (12 - 1)`); the Fascinate DC GENUINELY
//!   RISES to 18 (`10 + 12 / 2 + Charisma modifier 2`); the Fascinate
//!   affected-creature count stays 4 (`1 + (12 - 1) / 3`, an
//!   integer-division coincidence with level 11).
//! - the PF1 Core Rulebook Bard class table's level-12 "Special" column
//!   reads "Soothing performance" only (verified independently against
//!   both primary sources, checked rather than assumed): a wholly new
//!   12th-level Bard class feature ("a bard of 12th level or higher can
//!   use his performance to help heal the wounds of his allies... this
//!   ability functions as mass cure serious wounds... also removes the
//!   fatigued, sickened, and shaken conditions"). This is grounded ONLY as
//!   a bounded grant-only identity record (value 0, non-fabricated),
//!   mirroring the Monk Diamond Body / Paladin Aura of Justice idiom
//!   exactly: no healing-application engine and no condition-removal
//!   engine exist anywhere in this codebase, so neither is fabricated.
//! - Inspire Courage, Inspire Competence, and Lore Master's flat magnitudes
//!   all stay unchanged at their level-11 third tier (the next tier for
//!   each lands at bard level 15 or 17, out of scope); Well-Versed stays
//!   the flat +4; Jack-of-All-Trades carries over unchanged as a +0
//!   identity record.
//!
//! It deliberately does not touch the bardic performance-execution engine,
//! any healing or condition-resolution engine, or the spontaneous spell
//! posture burden (all stay named-but-unproven, unchanged from levels
//! 1-11), and it does not ground Bard level 13+. It also preserves the
//! accepted Bard level-1..level-11 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const BARD_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level11_sd18_inspire_widening_deterministic_input.txt"
);

const BARD_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_bard_level12_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const INSPIRE_COMPETENCE_ID: &str = "class_feature.bard.inspire_competence";
const LORE_MASTER_ID: &str = "class_feature.bard.lore_master";
const JACK_OF_ALL_TRADES_ID: &str = "class_feature.bard.jack_of_all_trades";
const WELL_VERSED_ID: &str = "class_feature.bard.well_versed";
const SOOTHING_PERFORMANCE_ID: &str = "class_feature.bard.soothing_performance";

// ----- Base attack bonus and saves genuinely rise at level 12 -----

#[test]
fn bard_level12_base_attack_and_saves_genuinely_rise() {
    let input = load(BARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Bard level 12 3/4-BAB progression (12 * 3 / 4) must equal 9, genuinely risen from 8 \
         at level 11: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.bard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Bard level 12 poor Fortitude (12/3) must equal 4, genuinely risen from 3"
    );

    let reflex = explanation(&computation, "class_chassis.bard.base_save.reflex");
    assert_eq!(
        reflex.value, 8,
        "Bard level 12 good Reflex (12/2+2) must equal 8, genuinely risen from 7"
    );

    let will = explanation(&computation, "class_chassis.bard.base_save.will");
    assert_eq!(will.value, 8, "Bard level 12 good Will (12/2+2) must equal 8, genuinely risen from 7");
}

// ----- Bardic Knowledge, performance rounds, and Fascinate DC genuinely rise -----

#[test]
fn bard_level12_knowledge_rounds_and_fascinate_dc_genuinely_rise() {
    let input = load(BARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(
        knowledge.value, 6,
        "Bardic Knowledge (max(12/2, 1)) must genuinely rise to 6, up from 5 at level 11"
    );

    let rounds = explanation(
        &computation,
        "class_chassis.bard.bardic_performance_rounds_per_day",
    );
    // CG-03 fix: Charisma modifier is now +3 (base 15 + 2 Human racial), not +2.
    assert_eq!(
        rounds.value, 29,
        "Bard level 12 Bardic Performance rounds (4 + Charisma modifier 3 + 2 x (12 - 1)) must \
         equal 29, genuinely risen from 27: {}",
        rounds.detail
    );

    let dc = explanation(&computation, "class_chassis.bard.fascinate_dc");
    assert_eq!(
        dc.value, 19,
        "the Fascinate DC (10 + 12/2 + Charisma modifier 3) must genuinely rise to 19, up from \
         18 at level 11"
    );

    let count = explanation(&computation, "class_chassis.bard.fascinate_affected_creatures");
    assert_eq!(
        count.value, 4,
        "the Fascinate affected-creature count (1 + (12-1)/3) must stay 4, an integer-division \
         coincidence with level 11"
    );
}

// ----- Inspire Courage, Inspire Competence, Lore Master, and Well-Versed carry over -----

#[test]
fn bard_level12_third_tier_magnitudes_carry_over_unchanged() {
    let input = load(BARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let inspire_courage = explanation(&computation, "class_chassis.bard.inspire_courage_bonus");
    assert_eq!(
        inspire_courage.value, 3,
        "Inspire Courage must stay +3 at level 12 (the next tier lands at bard level 17, out \
         of scope): {}",
        inspire_courage.detail
    );

    let inspire_competence = explanation(&computation, INSPIRE_COMPETENCE_ID);
    assert_eq!(
        inspire_competence.value, 4,
        "Inspire Competence must stay +4 at level 12 (the next tier lands at bard level 15, \
         out of scope): {}",
        inspire_competence.detail
    );

    let lore_master = explanation(&computation, LORE_MASTER_ID);
    assert_eq!(
        lore_master.value, 2,
        "Lore Master's flat take-20 usage count must stay 2/day at level 12 (the next tier \
         lands at bard level 17, out of scope): {}",
        lore_master.detail
    );

    let well_versed = explanation(&computation, WELL_VERSED_ID);
    assert_eq!(well_versed.value, 4, "Well-Versed must stay the flat +4 at level 12");
}

// ----- Soothing Performance is newly granted as a grant-only identity record -----

#[test]
fn bard_level12_soothing_performance_is_a_grant_only_identity_record() {
    let input = load(BARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Soothing Performance must be a bounded +0 grant-only identity record, non-fabricated: \
         {}",
        soothing_performance.detail
    );
    assert!(
        soothing_performance.detail.to_lowercase().contains("soothing performance"),
        "the Soothing Performance record must name the PF1 Core Rulebook rule text: {}",
        soothing_performance.detail
    );
}

// ----- Jack-of-All-Trades carries over unchanged -----

#[test]
fn bard_level12_jack_of_all_trades_carries_over() {
    let input = load(BARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let joat = explanation(&computation, JACK_OF_ALL_TRADES_ID);
    assert_eq!(
        joat.value, 0,
        "Jack-of-All-Trades must carry over as a +0 identity/recognition record at level 12: {}",
        joat.detail
    );
}

// ----- The bardic performance-execution burden still claim-blocks at level 12 -----

#[test]
fn bard_level12_still_claim_blocks_the_performance_execution_burden() {
    let input = load(BARD_LEVEL12_FIXTURE);
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
                "level-12 Bard must ground an honest not-performing record when no \
                 bardic-performance posture violation exists: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- Negative control: the level-11 fixture is unaffected by this widening -----

#[test]
fn bard_level11_truth_is_unchanged_by_this_slice() {
    let input = load(BARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.bard.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Bard level 11 base attack bonus must stay 8");

    let knowledge = explanation(&computation, "class_chassis.bard.bardic_knowledge");
    assert_eq!(knowledge.value, 5, "Bard level 11 Bardic Knowledge must stay 5");

    let soothing_performance = explanation(&computation, SOOTHING_PERFORMANCE_ID);
    assert_eq!(
        soothing_performance.value, 0,
        "Bard level 11 Soothing Performance must be a correctly-absent +0 record"
    );
    assert!(
        soothing_performance.detail.contains("correctly absent"),
        "Bard level 11 Soothing Performance record must say it is correctly absent: {}",
        soothing_performance.detail
    );
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (Bard levels 13 through 18 were widened into scope by later SD18 slices —
// up through tests/sd18_bard_level18_widening.rs — so this negative
// control's boundary moves from 13 to 19, mirroring the exact same
// boundary-move idiom applied to every prior sibling class's own level
// widening cycle, then to 20 by the SD18 bard-level19-widening cycle, then
// to 21 (a pure implementation-gate check, since PF1 has no 21st character
// level) by the SD18 bard-level20-widening cycle.)

#[test]
fn bard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = BARD_LEVEL12_FIXTURE.replace("class:bard:12", "class:bard:21");
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
fn fighter_does_not_gain_bard_level12_recognition() {
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
fn multiclass_bard_level12_is_not_promoted_by_this_slice() {
    let multiclass = BARD_LEVEL12_FIXTURE.replace(
        "class_level=class:bard:12",
        "class_level=class:bard:12\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_bard_row_names_level_12_widening() {
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard progression_and_spell_burden row must exist");

    assert_eq!(bard.support_state, SupportState::Supported);
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(bard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        bard.grounding_ref.contains("sd18_bard_level12_widening"),
        "bard row must cite the live SD18 level-12 widening proof surface: {}",
        bard.grounding_ref
    );
    let note = bard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "bard partial note must name the level-12 widening: {note}"
    );
}
