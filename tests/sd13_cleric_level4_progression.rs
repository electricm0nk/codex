//! SD13-E5 Cleric level-4 progression grounding proof.
//!
//! Widens the accepted Cleric level-1/level-2/level-3 prepared divine
//! spell-bearing baseline (`tests/sd13_cleric_level1_spell_baseline.rs`,
//! `tests/sd13_cleric_base_attack_and_saves.rs`, `tests/sd13_cleric_domain_powers.rs`,
//! `tests/sd13_cleric_level2_progression.rs`, `tests/sd13_cleric_level3_progression.rs`)
//! to Cleric level 4, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/Sorcerer/Wizard
//! level-range-gate idiom (`supported_cleric_level` is generalized from `1..=3`
//! to `1..=4` via `MAX_SUPPORTED_CLERIC_LEVEL = 4`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Cleric class table, and the Good
//! Domain granted-power rule text) were read directly before writing any code
//! or test:
//!
//! - level 4 base attack bonus is +3 (`4 * 3 / 4 = 3`, the Cleric's own
//!   3/4-BAB progression, the same shape as Rogue/Monk/Druid) and base saves
//!   are +4 Fortitude (good, `4/2+2 = 4`), +1 Reflex (poor, `4/3 = 1`), +4
//!   Will (good, `4/2+2 = 4`) — confirmed by the same formulas already
//!   grounded at levels 1-3, not re-derived.
//! - Channel Energy's die count does NOT change at level 4: the PF1 Core
//!   Rulebook Cleric class table's level-4 "Special" column is blank (verified
//!   independently against both primary sources), and the pre-existing
//!   `ceil(cleric level / 2)` formula confirms this without re-derivation:
//!   `ceil(4 / 2) = 2`, the same 2d6 value as level 3 (it next increases only
//!   at level 5, `ceil(5/2) = 3`).
//! - Channel Energy's uses-per-day count (3 + Charisma modifier) is
//!   level-independent and unchanged at level 4.
//! - the domain choice seam still fires at level 4 for the same fixture
//!   selections (Good, Healing), not level-gated.
//! - the domain spell slot count does NOT change at level 4: the raw Cleric
//!   spells-per-day table rows (verified independently against both primary
//!   sources) show a level-4 cleric's 3rd-level spell column is still "—" (a
//!   level-4 cleric casts 3rd-level cleric spells for the first time only at
//!   level 5), so the count of domain spell slots ("one domain spell slot per
//!   level of cleric spells she can cast, 1st and up", PF1 Core Rulebook
//!   Domains) stays 2 (one 1st-level domain slot plus one 2nd-level domain
//!   slot), unchanged from level 3, via the same pre-existing formula.
//! - the Good domain's Touch of Good sacred bonus genuinely CHANGES at level
//!   4: half cleric level, minimum 1 (`max(4/2, 1) = 2`), up from 1 at levels
//!   1-3 — verified independently against the PF1 Core Rulebook Good Domain
//!   granted-power rule text (d20pfsrd, cross-checked by a second independent
//!   search) — via the same pre-existing formula, not re-derived.
//! - both domain powers' uses-per-day counts (3 + Wisdom modifier, for Touch
//!   of Good and Rebuke Death alike) are level-independent and unchanged at
//!   level 4.
//! - the Cleric class table's level-4 "Special" column is blank (verified
//!   independently against both primary sources: no new Cleric class feature
//!   is gained at 4th level), so this slice adds no new pillar record for
//!   level 4 — only the existing pillars are widened (one of them, the Touch
//!   of Good sacred bonus, widened to a genuinely new value; the rest confirmed
//!   unchanged).
//!
//! It deliberately does not touch domain spell-list contents, the prepared
//! divine spell posture burden, or the Rebuke Death heal amount (all three
//! stay named-but-unproven, unchanged from levels 1-3), and it does not
//! ground Cleric level 5+. It also preserves the accepted Cleric level-1/
//! level-2/level-3 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const CLERIC_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level3_sd13_deterministic_input.txt");

const CLERIC_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level4_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 4 -----

#[test]
fn cleric_level4_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Cleric level 4 3/4-BAB progression (4 * 3 / 4) must equal 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 4 (good Fortitude/Will, poor Reflex) -----

#[test]
fn cleric_level4_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 4, "Cleric level 4 good Fortitude (4/2+2) must equal 4");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 1, "Cleric level 4 poor Reflex (4/3) must equal 1");

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(will.value, 4, "Cleric level 4 good Will (4/2+2) must equal 4");
}

// ----- Channel Energy dice stays 2d6 at level 4 -----

#[test]
fn cleric_level4_channel_energy_dice_stays_two_d6() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 2,
        "Cleric level 4 Channel Energy die count must stay 2 (i.e. 2d6), unchanged from level 3: {}",
        dice.detail
    );
}

#[test]
fn cleric_level4_channel_energy_uses_per_day_is_unchanged() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 4 Channel Energy uses per day (3 + Charisma modifier 2) must equal 5: {}",
        uses.detail
    );
}

// ----- Domain choice seam still fires; domain spell slot count stays 2 -----

#[test]
fn cleric_level4_domain_choice_is_recognized() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.cleric.domain_choice"),
        "level-4 Cleric must still recognize the domain choice seam: {:?}",
        computation.explanations
    );
}

#[test]
fn cleric_level4_domain_spell_slot_count_stays_two() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 2,
        "Cleric level 4 domain spell slot count must stay 2 (a level-4 cleric still casts only \
         1st and 2nd-level cleric spells; 3rd-level cleric spells begin at level 5), unchanged \
         from level 3: {}",
        slot.detail
    );
}

// ----- Domain powers at level 4: Touch of Good becomes 2, Rebuke Death unchanged -----

#[test]
fn cleric_level4_touch_of_good_bonus_becomes_two() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 2,
        "Cleric level 4 Touch of Good sacred bonus (half cleric level, minimum 1: max(2,1)) must \
         become 2, up from 1 at levels 1-3: {}",
        bonus.detail
    );

    let uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 4 Touch of Good uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

#[test]
fn cleric_level4_grounds_rebuke_death_uses_per_day() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 4 Rebuke Death uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 4 -----

#[test]
fn cleric_level4_still_claim_blocks_domain_powers_and_prepared_divine_burdens() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-4 Cleric must still claim-block on the domain powers burden: {:?}",
        computation.diagnostics
    );
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.cleric.prepared_divine.unsupported")
    {
        Some(blocker) => assert!(blocker.claim_blocking, "if the blocker fires, it must be claim-blocking"),
        None => {
            let prepared_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.cleric.daily_preparation")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                prepared_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- The chassis recognition record is still present at level 4 -----

#[test]
fn cleric_level4_still_recognizes_the_spell_bearing_baseline() {
    let input = load(CLERIC_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.cleric"),
        "level-4 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- The accepted Cleric level-3 truth is unaffected -----

#[test]
fn cleric_level3_truth_is_unchanged_by_this_widening() {
    let input = load(CLERIC_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 2, "Cleric level 3 base attack bonus must stay 2");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 2, "Cleric level 3 Channel Energy die count must stay 2");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 1,
        "Cleric level 3 Touch of Good bonus must stay 1, unaffected by the level-4 widening"
    );
}

// ----- Negative control: level 5 was later widened into the supported tranche -----

#[test]
fn cleric_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_cleric_level5_progression.rs) widened the level-range gate to
    // level 5 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/
    // Sorcerer/Wizard/Ranger level-range gate idiom) and confirmed Channel
    // Energy's die count and the domain spell slot count both change for real
    // at level 5, while Touch of Good's sacred bonus stays unchanged from
    // level 4; this negative control is superseded, not violated — pin the
    // new truth here too so this file stays internally consistent.
    let level_5 = CLERIC_LEVEL4_FIXTURE.replace("class:cleric:4", "class:cleric:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.cleric"),
        "level-5 Cleric is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-5 Cleric must stay claim-blocked in this slice"
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level4_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")
                || e.id == "class_chassis.spell_baseline.cleric"),
        "the Fighter chassis must not surface any cleric-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Cleric is not promoted -----

#[test]
fn multiclass_cleric_level4_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL4_FIXTURE.replace(
        "class_level=class:cleric:4",
        "class_level=class:cleric:4\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")
                || e.id == "class_chassis.spell_baseline.cleric"),
        "multiclass Cleric must not gain any bounded cleric chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Cleric must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-4 widening -----

#[test]
fn matrix_cleric_row_names_level_4_widening() {
    let matrix = seeded_current_truth();
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric progression_and_spell_burden row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class Progression
    // Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(cleric.support_state, SupportState::Supported);
    assert_eq!(cleric.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        cleric.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        cleric.grounding_ref.contains("sd13_cleric_level4_progression"),
        "cleric row must cite the live SD13-E5 level-4 proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 4") || note.contains("level-4"),
        "cleric partial note must name the level-4 widening: {note}"
    );
}
