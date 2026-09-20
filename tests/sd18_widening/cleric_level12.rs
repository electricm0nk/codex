//! SD18 Cleric level-12 widening grounding proof.
//!
//! Widens the accepted Cleric level-1..level-11 prepared divine spell-bearing
//! baseline (`tests/sd18_cleric_level11_widening.rs`, the loop's most recent
//! Cleric ceiling) to Cleric level 12 — mirroring the sibling-class
//! level-range-gate idiom (`supported_cleric_level` is generalized from
//! `1..=11` to `1..=12` via `MAX_SUPPORTED_CLERIC_LEVEL = 12`, exactly as
//! `cycle-2026-07-14T1814` widened `MAX_SUPPORTED_BARBARIAN_LEVEL` and
//! `cycle-2026-07-14T2359` widened `MAX_SUPPORTED_BARD_LEVEL`, both from 11
//! to 12). Both PF1 CRB primary sources (d20pfsrd and Archives of Nethys
//! aonprd.com Cleric class table and spells-per-day table) were read
//! directly before writing any code or test:
//!
//! - level 12 base attack bonus is +9 (`12 * 3 / 4 = 9`, genuinely risen
//!   from +8 at level 11) and base saves are +8 Fortitude and +8 Will (both
//!   good, `12 / 2 + 2 = 8`, genuinely risen from +7) and +4 Reflex (poor,
//!   `12 / 3 = 4`, genuinely risen from +3) — confirmed by the same formulas
//!   already grounded at levels 1-11, not re-derived.
//! - the PF1 Core Rulebook Cleric class table's level-12 "Special" column is
//!   genuinely BLANK (verified independently against both primary sources,
//!   checked rather than assumed away) — Channel Energy's die-count rises
//!   land only at odd cleric levels, so no new class feature is gained at
//!   12th and this slice grounds no new pillar record.
//! - Channel Energy's die count STAYS 6d6 (`(12 + 1) / 2 = 6`, its next rise
//!   landing at 13th) and its uses-per-day pool stays the level-independent
//!   3 + Charisma modifier (5).
//! - the domain spell slot count STAYS 6 (a level-12 cleric still casts only
//!   up to 6th-level cleric spells; 7th-level cleric spells first appear at
//!   13th, checked rather than assumed).
//! - Touch of Good's bonus GENUINELY RISES to 6 (`12 / 2`, up from 5 at
//!   level 11, via the same half-cleric-level formula); its uses-per-day
//!   pool and Rebuke Death's uses-per-day pool both stay the
//!   level-independent 3 + Wisdom modifier (6); the domain choice
//!   recognitions (Good, Healing) are not level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-11), and it does not
//! ground Cleric level 13+. It also preserves the accepted Cleric
//! level-1..level-11 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use crate::common::explanation;

const CLERIC_LEVEL11_FIXTURE: &str = include_str!(
    "../fixtures/rules_core/pf1_human_cleric_level11_sd18_widening_deterministic_input.txt"
);

const CLERIC_LEVEL12_FIXTURE: &str = include_str!(
    "../fixtures/rules_core/pf1_human_cleric_level12_sd18_widening_deterministic_input.txt"
);


// ----- Base attack bonus and saves genuinely rise at level 12 -----

#[test]
fn cleric_level12_base_attack_and_saves_genuinely_rise() {
    let computation = crate::support::compute(CLERIC_LEVEL12_FIXTURE);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Cleric level 12 3/4-BAB progression (12 * 3 / 4) must equal 9, genuinely risen from \
         8 at level 11: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 8,
        "Cleric level 12 good Fortitude (12/2+2) must genuinely rise to 8, up from 7 at level 11"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Cleric level 12 poor Reflex (12/3) must genuinely rise to 4, up from 3 at level 11"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 8,
        "Cleric level 12 good Will (12/2+2) must genuinely rise to 8, up from 7 at level 11"
    );
}

// ----- Channel Energy stays 6d6 at level 12 -----

#[test]
fn cleric_level12_channel_energy_stays_at_level11_values() {
    let computation = crate::support::compute(CLERIC_LEVEL12_FIXTURE);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 6,
        "Cleric level 12 Channel Energy die count ((12 + 1) / 2) must stay 6 — the odd-level \
         cadence puts the next rise at 13th: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 12 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );
}

// ----- Domain spell slot count stays 6 at level 12 -----

#[test]
fn cleric_level12_domain_spell_slot_count_stays_six() {
    let computation = crate::support::compute(CLERIC_LEVEL12_FIXTURE);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 6,
        "Cleric level 12 domain spell slot count must stay 6 — 7th-level cleric spells first \
         appear at 13th per both primary sources' spells-per-day tables: {}",
        slot.detail
    );
}

// ----- Touch of Good genuinely rises to 6; other domain facets carry over -----

#[test]
fn cleric_level12_touch_of_good_rises_and_other_facets_carry_over() {
    let computation = crate::support::compute(CLERIC_LEVEL12_FIXTURE);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 6,
        "Touch of Good's bonus (12 / 2) must genuinely rise to 6 at level 12, up from 5 at \
         level 11: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 12");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 12");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The domain-powers and prepared-divine-spell burdens still claim-block at level 12 -----

#[test]
fn cleric_level12_still_claim_blocks_domain_powers_and_prepared_spell_burdens() {
    let computation = crate::support::compute(CLERIC_LEVEL12_FIXTURE);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-12 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: the level-11 fixture is unaffected by this widening -----

#[test]
fn cleric_level11_truth_is_unchanged_by_this_slice() {
    let computation = crate::support::compute(CLERIC_LEVEL11_FIXTURE);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Cleric level 11 base attack bonus must stay 8");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 6, "Cleric level 11 Channel Energy die count must stay 6");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 5, "Cleric level 11 Touch of Good bonus must stay 5");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (Superseded boundary: cycle-2026-07-15T1500 widened MAX_SUPPORTED_CLERIC_LEVEL
// from 12 to 13, then cycle-2026-07-15T2300 widened it from 13 to 14, then
// cycle-2026-07-15T3100 widened it from 14 to 15, then cycle-2026-07-15T5300
// widened it from 15 to 16, then cycle-2026-07-15T9600 widened it from 16 to
// 17, then cycle-2026-07-15T14300 widened it from 17 to 18, then
// cycle-2026-07-16T1100 widened it from 18 to 19, then cycle-2026-07-16T0844
// widened it from 19 to 20 (the final level within PF1's 1-20
// character-level cap), so this file's own negative-control boundary moves
// from 20 to 21, a pure implementation-gate check since PF1 has no 21st
// character level, mirroring the exact same boundary-move idiom applied to
// tests/sd18_ranger_level12_widening.rs when MAX_SUPPORTED_RANGER_LEVEL
// widened from 12 through 14.)

crate::sd18_boundary_neg_control_test!(cleric_level_21_is_not_promoted_by_this_slice, "cleric_level12", CLERIC_LEVEL12_FIXTURE);

// ----- Negative control: the cleric path must not leak onto other classes -----

crate::sd18_fighter_neg_control_test!(fighter_does_not_gain_cleric_level12_recognition, "cleric");

// ----- Negative control: multiclass Cleric is not promoted -----

crate::sd18_multiclass_neg_control_test!(multiclass_cleric_level12_is_not_promoted_by_this_slice, "cleric_level12", CLERIC_LEVEL12_FIXTURE);

// ----- Control plane: the matrix note names the level-12 widening -----

