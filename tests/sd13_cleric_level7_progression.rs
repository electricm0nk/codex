//! SD13-E5 Cleric level-7 progression grounding proof.
//!
//! Widens the accepted Cleric level-1/level-2/level-3/level-4/level-5/level-6
//! prepared divine spell-bearing baseline (`tests/sd13_cleric_level1_spell_baseline.rs`,
//! `tests/sd13_cleric_base_attack_and_saves.rs`, `tests/sd13_cleric_domain_powers.rs`,
//! `tests/sd13_cleric_level2_progression.rs`, `tests/sd13_cleric_level3_progression.rs`,
//! `tests/sd13_cleric_level4_progression.rs`, `tests/sd13_cleric_level5_progression.rs`,
//! `tests/sd13_cleric_level6_progression.rs`) to Cleric level 7, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_cleric_level` is generalized from `1..=6`
//! to `1..=7` via `MAX_SUPPORTED_CLERIC_LEVEL = 7`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Cleric class table, and the Cleric
//! spells-per-day table) were read directly before writing any code or test:
//!
//! - level 7 base attack bonus is +5 (`7 * 3 / 4 = 5`, the Cleric's own
//!   3/4-BAB progression, the same shape as Rogue/Monk/Druid) and base saves
//!   are +5 Fortitude (good, `7/2+2 = 5`), +2 Reflex (poor, `7/3 = 2`), +5
//!   Will (good, `7/2+2 = 5`) — confirmed by the same formulas already
//!   grounded at levels 1-6, not re-derived (Fortitude/Will are numerically
//!   unchanged from level 6, an integer-division coincidence, re-verified
//!   rather than assumed; Reflex is likewise unchanged).
//! - Channel Energy's die count GENUINELY increases to 4d6 at level 7: both
//!   primary sources confirm the level-7 class table's Special column reads
//!   "Channel energy 4d6" (the die count rises every odd cleric level — 1d6
//!   at 1st, 2d6 at 3rd, 3d6 at 5th, 4d6 at 7th), so the pre-existing
//!   `ceil(cleric level / 2)` formula produces the new value without
//!   re-derivation: `ceil(7 / 2) = 4`.
//! - Channel Energy's uses-per-day count (3 + Charisma modifier) is
//!   level-independent and unchanged at level 7.
//! - the domain choice seam still fires at level 7 for the same fixture
//!   selections (Good, Healing), not level-gated.
//! - the domain spell slot count GENUINELY increases to 4 at level 7: the raw
//!   Cleric spells-per-day table's level-7 row (verified independently
//!   against both primary sources) is the first to show a non-"—" 4th-level
//!   spell column, so a level-7 cleric casts 4th-level cleric spells for the
//!   first time — the domain spell slot count (PF1 Core Rulebook Domains:
//!   "one domain spell slot per level of cleric spells she can cast, 1st and
//!   up") becomes exactly 4 (one 1st-level, one 2nd-level, one 3rd-level, one
//!   4th-level domain slot), mirroring exactly the level-3 and level-5
//!   widenings of this same pillar. This grounds only the flat slot count,
//!   not the slot's contents.
//! - the Good domain's Touch of Good sacred bonus stays 3 at level 7: half
//!   cleric level, minimum 1 (`max(7/2, 1) = 3`, integer division), unchanged
//!   from level 6 — verified independently against the PF1 Core Rulebook
//!   Good Domain granted-power rule text — via the same pre-existing formula,
//!   not re-derived; it next increases only at level 8.
//! - both domain powers' uses-per-day counts (3 + Wisdom modifier, for Touch
//!   of Good and Rebuke Death alike) are level-independent and unchanged at
//!   level 7.
//! - the Cleric class table's level-7 "Special" column names only the
//!   Channel Energy die-count increase ("Channel energy 4d6"), verified
//!   independently against both primary sources — no other new Cleric class
//!   feature is gained at 7th level — so this slice adds no new pillar record
//!   for level 7 beyond widening the Channel Energy and domain-spell-slot
//!   pillars to genuinely new values.
//!
//! It deliberately does not touch domain spell-list contents, the prepared
//! divine spell posture burden, or the Rebuke Death heal amount (all three
//! stay named-but-unproven, unchanged from levels 1-6), and it does not
//! ground Cleric level 8+. It also preserves the accepted Cleric level-1/
//! level-2/level-3/level-4/level-5/level-6 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.
//!
//! The prior sibling file's negative control naming level 7 as unrecognized
//! has been renamed and flipped in `tests/sd13_cleric_level6_progression.rs` to
//! `cleric_level_7_was_later_widened_into_the_supported_tranche`, whose coverage
//! moves here now that level 7 is itself promoted into the supported tranche.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const CLERIC_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level6_sd13_deterministic_input.txt");

const CLERIC_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level7_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 7 -----

#[test]
fn cleric_level7_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Cleric level 7 3/4-BAB progression (7 * 3 / 4) must equal 5: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 (good Fortitude/Will, poor Reflex) -----

#[test]
fn cleric_level7_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 5, "Cleric level 7 good Fortitude (7/2+2) must equal 5");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 2, "Cleric level 7 poor Reflex (7/3) must equal 2");

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(will.value, 5, "Cleric level 7 good Will (7/2+2) must equal 5");
}

// ----- Channel Energy dice genuinely rises to 4d6 at level 7 -----

#[test]
fn cleric_level7_channel_energy_dice_rises_to_four_d6() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 4,
        "Cleric level 7 Channel Energy die count must genuinely rise to 4 (i.e. 4d6), up from \
         3 at level 6 (PF1 CRB class table's level-7 Special column reads \"Channel energy \
         4d6\"): {}",
        dice.detail
    );
}

#[test]
fn cleric_level7_channel_energy_uses_per_day_is_unchanged() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 7 Channel Energy uses per day (3 + Charisma modifier 2) must equal 5: {}",
        uses.detail
    );
}

// ----- Domain choice seam still fires; domain spell slot count genuinely rises to 4 -----

#[test]
fn cleric_level7_domain_choice_is_recognized() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.cleric.domain_choice"),
        "level-7 Cleric must still recognize the domain choice seam: {:?}",
        computation.explanations
    );
}

#[test]
fn cleric_level7_domain_spell_slot_count_rises_to_four() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 4,
        "Cleric level 7 domain spell slot count must genuinely rise to 4 (the raw \
         spells-per-day table's 4th-level spell column is non-\"—\" for the first time at \
         level 7, so a level-7 cleric casts 4th-level cleric spells for the first time), up \
         from 3 at level 6: {}",
        slot.detail
    );
}

// ----- Domain powers at level 7: Touch of Good stays unchanged, Rebuke Death unchanged -----

#[test]
fn cleric_level7_touch_of_good_bonus_stays_three() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 3,
        "Cleric level 7 Touch of Good sacred bonus (half cleric level, minimum 1: max(3,1)) \
         must stay 3, unchanged from level 6 (it next increases only at level 8): {}",
        bonus.detail
    );

    let uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 7 Touch of Good uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

#[test]
fn cleric_level7_grounds_rebuke_death_uses_per_day() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(
        uses.value, 7,
        "Cleric level 7 Rebuke Death uses per day (3 + Wisdom modifier 4) must equal 7: {}",
        uses.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 7 -----

#[test]
fn cleric_level7_still_claim_blocks_domain_powers_and_prepared_divine_burdens() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-7 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- The chassis recognition record is still present at level 7 -----

#[test]
fn cleric_level7_still_recognizes_the_spell_bearing_baseline() {
    let input = load(CLERIC_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.cleric"),
        "level-7 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Level 8 was later widened into the supported tranche; coverage moved to
// ----- tests/sd13_cleric_level8_progression.rs -----

#[test]
fn cleric_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = CLERIC_LEVEL7_FIXTURE.replace("class:cleric:7", "class:cleric:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.cleric"),
        "level-8 Cleric was widened into the supported tranche by a later SD13-E5 slice; the \
         full level-8 coverage now lives in tests/sd13_cleric_level8_progression.rs: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-6 fixture is unaffected by this widening -----

#[test]
fn cleric_level6_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 4, "Cleric level 6 base attack bonus must stay 4");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 3, "Cleric level 6 Channel Energy die count must stay 3");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 3,
        "Cleric level 6 domain spell slot count must stay 3, unaffected by the level-7 widening"
    );

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 3,
        "Cleric level 6 Touch of Good bonus must stay 3, unaffected by the level-7 widening"
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level7_recognition() {
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
fn multiclass_cleric_level7_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL7_FIXTURE.replace(
        "class_level=class:cleric:7",
        "class_level=class:cleric:7\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_cleric_row_names_level_7_widening_and_channel_energy_and_domain_slot_increase() {
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
        cleric.grounding_ref.contains("sd13_cleric_level7_progression"),
        "cleric row must cite the live SD13-E5 level-7 proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 7") || note.contains("level-7"),
        "cleric partial note must name the level-7 widening: {note}"
    );
    assert!(
        note.contains("4d6") || note.contains("four"),
        "cleric partial note must name the level-7 Channel Energy die-count increase: {note}"
    );
}
