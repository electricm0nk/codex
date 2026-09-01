//! SD13-E5 Cleric level-2 progression grounding proof.
//!
//! Widens the accepted Cleric level-1 prepared divine spell-bearing baseline
//! (`tests/sd13_cleric_level1_spell_baseline.rs`, `tests/sd13_cleric_base_attack_and_saves.rs`,
//! `tests/sd13_cleric_domain_powers.rs`) to Cleric level 2, mirroring the Fighter
//! `supported_fighter_level` / Paladin `supported_paladin_level` / Rogue
//! `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
//! `supported_monk_level` level-range-gate idiom (the level-1-only gate
//! `is_single_class_cleric_level1` is generalized to `supported_cleric_level`, an
//! `Option<u8>` helper gated by `MAX_SUPPORTED_CLERIC_LEVEL = 2`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Cleric class table) were read directly
//! before writing any code or test:
//!
//! - level 2 base attack bonus is +1, base Fortitude/Will are +3 (good), base Reflex
//!   is +0 (poor) — confirmed by the same formulas already grounded at level 1
//!   (`classlevel * 3 / 4` and `classlevel/2+2` / `classlevel/3`), not re-derived.
//! - Channel Energy stays 1d6 at level 2 (`ceil(2 / 2) = 1`) — the PF1 Core Rulebook
//!   Cleric class table's Channel Energy die count next increases only at level 3
//!   (to 2d6), confirmed via the same formula, not a new record.
//! - Channel Energy uses per day (3 + Charisma modifier) and both domain powers'
//!   uses per day (3 + Wisdom modifier, for Touch of Good and Rebuke Death alike) are
//!   level-independent formulas, confirmed unchanged at level 2 via the same formula.
//! - the domain spell slot count stays exactly 1 at level 2: verified against the PF1
//!   Core Rulebook Cleric spells-per-day table (d20pfsrd and legacy.aonprd.com both
//!   list 1st-level spells as "2+1" and 2nd-level spells as "—" at level 2) that a
//!   level-2 cleric still only casts 1st-level cleric spells — 2nd-level cleric spells
//!   begin only at caster level 3 — so the domain slot count formula ("one domain
//!   spell slot per level of cleric spells she can cast") still evaluates to exactly
//!   one 1st-level slot, confirmed via the same formula, not a new record.
//! - the Good domain's Touch of Good sacred bonus (half cleric level, minimum 1)
//!   evaluates to 1 at level 2 (`2 / 2 = 1`), the same value as level 1's floor-forced
//!   1, but reached naturally this time rather than via the floor, confirmed via the
//!   same formula.
//! - the Cleric class table's level-2 "Special" column is blank: verified
//!   independently against both primary sources that Cleric gains no new class
//!   feature at 2nd level (unlike Rogue/Monk's Evasion), so no new pillar burden is
//!   added this slice — only the existing pillars are widened.
//!
//! It deliberately does not touch domain spell-list contents, the prepared divine
//! spell posture burden, or the Rebuke Death heal amount (all three stay
//! named-but-unproven, unchanged from level 1), and it does not ground Cleric level
//! 3+. It also preserves the accepted Cleric level-1 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const CLERIC_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level1_sd13_deterministic_input.txt");

const CLERIC_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level2_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 2 -----

#[test]
fn cleric_level2_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 1,
        "Cleric level 2 3/4-BAB progression (2 * 3 / 4) must equal 1: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 2 (good Fortitude/Will, poor Reflex) -----

#[test]
fn cleric_level2_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Cleric level 2 good Fortitude (2/2+2) must equal 3");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 0, "Cleric level 2 poor Reflex (2/3) must equal 0");

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(will.value, 3, "Cleric level 2 good Will (2/2+2) must equal 3");
}

// ----- Channel Energy stays 1d6, uses/day stays 3 + Cha modifier -----

#[test]
fn cleric_level2_channel_energy_dice_stays_one_d6() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 1,
        "Cleric level 2 Channel Energy die count must stay 1 (i.e. 1d6), not a new record: {}",
        dice.detail
    );
}

#[test]
fn cleric_level2_channel_energy_uses_per_day_is_unchanged() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 2 Channel Energy uses per day (3 + Charisma modifier 2) must equal 5: {}",
        uses.detail
    );
}

// ----- Domain choice seam and flat domain spell slot count at level 2 -----

#[test]
fn cleric_level2_domain_choice_is_recognized() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.cleric.domain_choice"),
        "level-2 Cleric must still recognize the domain choice seam: {:?}",
        computation.explanations
    );
}

#[test]
fn cleric_level2_domain_spell_slot_count_stays_one() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 1,
        "Cleric level 2 domain spell slot count must stay 1, not a new record: {}",
        slot.detail
    );
    assert!(
        slot.detail.contains("caster level 3"),
        "domain spell slot explanation must name why the count stays 1 at level 2 (2nd-level \
         cleric spells begin at caster level 3): {}",
        slot.detail
    );
}

// ----- Domain powers at level 2: Touch of Good and Rebuke Death -----

#[test]
fn cleric_level2_grounds_touch_of_good_bonus_and_uses_per_day() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 1,
        "Cleric level 2 Touch of Good sacred bonus (half cleric level, minimum 1: max(1,1)) must \
         equal 1: {}",
        bonus.detail
    );

    let uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 2 Touch of Good uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

#[test]
fn cleric_level2_grounds_rebuke_death_uses_per_day() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 2 Rebuke Death uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 2 -----

#[test]
fn cleric_level2_still_claim_blocks_domain_powers_and_prepared_divine_burdens() {
    let input = load(CLERIC_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-2 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- The accepted Cleric level-1 truth is unaffected -----

#[test]
fn cleric_level1_truth_is_unchanged_by_this_widening() {
    let input = load(CLERIC_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 0, "Cleric level 1 base attack bonus must stay 0");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 1, "Cleric level 1 Channel Energy die count must stay 1");
}

// ----- Cleric level 3 was later widened into the supported tranche -----

#[test]
fn cleric_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_cleric_level3_progression.rs) widened the level-range gate to
    // level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/
    // Sorcerer/Wizard level-range gate idiom) and extended every formula, changing
    // both Channel Energy's die count (1 -> 2) and the domain spell slot count
    // (1 -> 2) for real; this negative control is superseded, not violated — pin
    // the new truth here too so this file stays internally consistent.
    let level_3 = CLERIC_LEVEL2_FIXTURE.replace("class:cleric:2", "class:cleric:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")
                || e.id == "class_chassis.spell_baseline.cleric"),
        "level-3 Cleric is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control (was: level 4 stays unrecognized). Level 4 was later
// widened into the supported tranche by tests/sd13_cleric_level4_progression.rs;
// coverage for that level moved there. -----

#[test]
fn cleric_level_4_was_later_widened_into_the_supported_tranche() {
    let level_4 = CLERIC_LEVEL2_FIXTURE.replace("class:cleric:2", "class:cleric:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.cleric"),
        "level-4 Cleric is now supported by a later SD13-E5 slice \
         (tests/sd13_cleric_level4_progression.rs): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level2_recognition() {
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
fn multiclass_cleric_level2_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL2_FIXTURE.replace(
        "class_level=class:cleric:2",
        "class_level=class:cleric:2\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-2 widening -----

#[test]
fn matrix_cleric_row_names_level_2_widening() {
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
        cleric.grounding_ref.contains("sd13_cleric_level2_progression"),
        "cleric row must cite the live SD13-E5 level-2 proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 2") || note.contains("level-2"),
        "cleric partial note must name the level-2 widening: {note}"
    );
}
