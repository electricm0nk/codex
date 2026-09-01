//! SD18 Cleric level-11 widening grounding proof.
//!
//! Widens the accepted Cleric level-1..level-10 prepared divine spell-bearing
//! baseline (`tests/sd13_cleric_level10_progression.rs`, the SD13 tranche's
//! declared ceiling) to Cleric level 11 — the third SD-18 §3.2 class-row
//! widening, mirroring the sibling-class level-range-gate idiom
//! (`supported_cleric_level` is generalized from `1..=10` to `1..=11` via
//! `MAX_SUPPORTED_CLERIC_LEVEL = 11`, exactly as `cycle-2026-07-13T1255`
//! widened `MAX_SUPPORTED_BARBARIAN_LEVEL` and `cycle-2026-07-13T1830`
//! widened `MAX_SUPPORTED_BARD_LEVEL`, both from 10 to 11). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Cleric class table and
//! spells-per-day table) were read directly before writing any code or test:
//!
//! - level 11 base attack bonus is +8 (`11 * 3 / 4 = 8`, genuinely risen from
//!   +7 at level 10) and base saves are +7 Fortitude and +7 Will (both good,
//!   `11 / 2 + 2 = 7`, numerically unchanged from level 10, an
//!   integer-division coincidence) and +3 Reflex (poor, `11 / 3 = 3`, also
//!   an integer-division coincidence with level 10) — confirmed by the same
//!   formulas already grounded at levels 1-10, not re-derived.
//! - the PF1 Core Rulebook Cleric class table's level-11 "Special" column
//!   reads "Channel energy 6d6" (verified independently against both
//!   primary sources, checked rather than assumed away): Channel Energy's
//!   die count GENUINELY RISES to 6d6 (`(11 + 1) / 2 = 6`, up from 5d6 at
//!   level 10); its uses-per-day pool stays the level-independent
//!   3 + Charisma modifier (5).
//! - the domain spell slot count GENUINELY RISES to 6 (the level-11
//!   spells-per-day row gains a 6th-level column for the first time —
//!   6th-level cleric spells first appear at 11th caster level, verified
//!   independently against both primary sources' raw spells-per-day table
//!   rows, checked rather than assumed) — one domain spell slot per level of
//!   cleric spells she can cast, 1st through 6th.
//! - Touch of Good's bonus stays 5 (`11 / 2`, an integer-division
//!   coincidence with level 10); its uses-per-day pool and Rebuke Death's
//!   uses-per-day pool both stay the level-independent 3 + Wisdom modifier
//!   (6); the domain choice recognitions (Good, Healing) are not level-gated
//!   and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-10), and it does not
//! ground Cleric level 12+. It also preserves the accepted Cleric
//! level-1..level-10 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level10_sd13_deterministic_input.txt");

const CLERIC_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level11_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and saves at level 11 -----

#[test]
fn cleric_level11_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(CLERIC_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Cleric level 11 3/4-BAB progression (11 * 3 / 4) must equal 8, genuinely risen from \
         7 at level 10: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(fortitude.value, 7, "Cleric level 11 good Fortitude (11/2+2) must stay 7");

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(reflex.value, 3, "Cleric level 11 poor Reflex (11/3) must stay 3");

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(will.value, 7, "Cleric level 11 good Will (11/2+2) must stay 7");
}

// ----- Channel Energy die count genuinely rises to 6d6 -----

#[test]
fn cleric_level11_channel_energy_dice_genuinely_rise() {
    let input = load(CLERIC_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 6,
        "Cleric level 11 Channel Energy die count ((11 + 1) / 2) must genuinely rise to 6, up \
         from 5 at level 10: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 11 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );
}

// ----- Domain spell slot count genuinely rises to 6 -----

#[test]
fn cleric_level11_domain_spell_slot_count_genuinely_rises() {
    let input = load(CLERIC_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 6,
        "Cleric level 11 domain spell slot count must genuinely rise to 6 — 6th-level cleric \
         spells first appear at 11th caster level per both primary sources' spells-per-day \
         tables: {}",
        slot.detail
    );
}

// ----- Touch of Good and Rebuke Death carry over unchanged -----

#[test]
fn cleric_level11_touch_of_good_and_rebuke_death_carry_over() {
    let input = load(CLERIC_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 5,
        "Touch of Good's bonus (11 / 2) must stay 5 at level 11, an integer-division \
         coincidence with level 10: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 11");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 11");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The domain-powers and prepared-divine-spell burdens still claim-block at level 11 -----

#[test]
fn cleric_level11_still_claim_blocks_domain_powers_and_prepared_spell_burdens() {
    let input = load(CLERIC_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-11 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn cleric_level10_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 5, "Cleric level 10 Channel Energy die count must stay 5");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 5, "Cleric level 10 domain spell slot count must stay 5");

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 7, "Cleric level 10 base attack bonus must stay 7");
}

// ----- Negative control: level 14 stays unrecognized by this slice -----
// (Cleric levels 12, 13, 14, and 15 were widened into scope by later SD18
// slices — tests/sd18_cleric_level12_widening.rs,
// tests/sd18_cleric_level13_widening.rs, tests/sd18_cleric_level14_widening.rs,
// and tests/sd18_cleric_level15_widening.rs — so this negative control's
// boundary moves from 12 to 16, mirroring the exact same boundary-move
// idiom applied to tests/sd18_ranger_level12_widening.rs when
// MAX_SUPPORTED_RANGER_LEVEL widened from 12 through 14; cycle-2026-07-15T5300
// moved this boundary again, from 16 to 17, since level 16 was then itself
// Cleric's supported/grounded row; cycle-2026-07-15T9600 moved this boundary
// again, from 17 to 18, since level 17 was then itself Cleric's
// supported/grounded row; cycle-2026-07-15T14300 moved this boundary again,
// from 18 to 19, since level 18 was then itself Cleric's supported/grounded
// row; cycle-2026-07-16T1100 moved this boundary again, from 19 to 20, since
// level 19 was then itself Cleric's supported/grounded row;
// cycle-2026-07-16T0844 moves this boundary again, from 20 to 21, since
// level 20 is now itself Cleric's supported/grounded row — and the final
// level within PF1's 1-20 character-level cap, so this boundary check is now
// a pure implementation-gate check with no further real level to move to.)

#[test]
fn cleric_level_21_is_not_promoted_by_this_slice() {
    let level_21 = CLERIC_LEVEL11_FIXTURE.replace("class:cleric:11", "class:cleric:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.cleric.")
                || e.id.starts_with("class_feature.cleric.")
                || e.id == "class_chassis.spell_baseline.cleric")
                // (v0.6 alpha swarm, risks item 8, Good domain closure)
                // Touch of Good's not-active explanation is checked
                // unconditionally, regardless of level bound or
                // single-class status (mirrors every other class's
                // gate-ordering fix)
                && e.id != "class_feature.domain.good_touch_of_good_not_active"),
        "level-21 Cleric must not gain any bounded cleric explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level11_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.cleric.")
                || e.id.starts_with("class_feature.cleric.")),
        "the Fighter chassis must not surface any cleric-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Cleric is not promoted -----

#[test]
fn multiclass_cleric_level11_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL11_FIXTURE.replace(
        "class_level=class:cleric:11",
        "class_level=class:cleric:11\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.cleric.")
                || e.id.starts_with("class_feature.cleric."))
                // (v0.6 alpha swarm, risks item 8, Good domain closure)
                // Touch of Good's not-active explanation is checked
                // unconditionally, regardless of level bound or
                // single-class status (mirrors every other class's
                // gate-ordering fix)
                && e.id != "class_feature.domain.good_touch_of_good_not_active"),
        "multiclass Cleric must not gain any bounded cleric explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Cleric must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_cleric_row_names_level_11_widening() {
    let matrix = seeded_current_truth();
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric progression_and_spell_burden row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class Progression
    // Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(cleric.support_state, SupportState::Supported);
    assert_eq!(cleric.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(cleric.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        cleric.grounding_ref.contains("sd18_cleric_level11_widening"),
        "cleric row must cite the live SD18 level-11 widening proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "cleric partial note must name the level-11 widening: {note}"
    );
}
