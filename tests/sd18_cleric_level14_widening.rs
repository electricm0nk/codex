//! SD18 Cleric level-14 widening grounding proof.
//!
//! Widens the accepted Cleric level-1..level-13 prepared divine spell-bearing
//! baseline (`tests/sd18_cleric_level13_widening.rs`, the loop's most recent
//! Cleric ceiling) to Cleric level 14 — mirroring the sibling-class
//! level-range-gate idiom (`supported_cleric_level` is generalized from
//! `1..=13` to `1..=14` via `MAX_SUPPORTED_CLERIC_LEVEL = 14`, exactly as
//! `cycle-2026-07-15T1900` widened `MAX_SUPPORTED_BARBARIAN_LEVEL`,
//! `cycle-2026-07-15T2000` widened `MAX_SUPPORTED_FIGHTER_LEVEL` and
//! `MAX_SUPPORTED_ROGUE_LEVEL`, and `cycle-2026-07-15T2100` widened
//! `MAX_SUPPORTED_RANGER_LEVEL`, all from 13 to 14). All three primary
//! sources (d20pfsrd, Archives of Nethys aonprd.com, and legacy.aonprd.com
//! Cleric class table and spells-per-day table) were read directly before
//! writing any code or test, byte-for-byte agreement across all three:
//!
//! - level 14 base attack bonus GENUINELY RISES to +10 (`14 * 3 / 4 = 10`,
//!   up from +9 at level 13) and both good saves GENUINELY RISE to +9
//!   (`14 / 2 + 2 = 9`, up from +8), while poor Reflex STAYS +4 (`14 / 3 =
//!   4`, an integer-division coincidence with level 13), checked rather
//!   than assumed.
//! - the PF1 Core Rulebook Cleric class table's level-14 "Special" column
//!   is genuinely BLANK (verified independently against all three primary
//!   sources) — no new class feature is granted at 14th level, and
//!   Channel Energy's die count STAYS 7d6 (`(14 + 1) / 2 = 7`, an
//!   integer-division coincidence with level 13, since the die count rises
//!   only every odd cleric level, next rising at 15th) via the same
//!   pre-existing formula, not re-derived; its uses-per-day pool stays the
//!   level-independent 3 + Charisma modifier (5).
//! - level 14 is NOT when a cleric first casts 8th-level cleric spells
//!   (verified independently against all three primary sources' raw
//!   spells-per-day table rows, which all show the 8th-level spell column
//!   still "—" at level 14, first opening only at level 15), so the flat
//!   domain spell slot count STAYS 7, an integer-division-adjacent
//!   coincidence of the underlying threshold table, unchanged from level
//!   13.
//! - Touch of Good's bonus GENUINELY RISES to 7 (`14 / 2 = 7`, up from 6 at
//!   level 13) via the same pre-existing formula, not re-derived; its
//!   uses-per-day pool and Rebuke Death's uses-per-day pool both stay the
//!   level-independent 3 + Wisdom modifier (6); the domain choice
//!   recognitions (Good, Healing) are not level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-13), and it does not
//! ground Cleric level 15+. It also preserves the accepted Cleric
//! level-1..level-13 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level13_sd18_widening_deterministic_input.txt"
);

const CLERIC_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level14_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and good saves genuinely rise at level 14 -----

#[test]
fn cleric_level14_base_attack_and_good_saves_genuinely_rise() {
    let input = load(CLERIC_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 10,
        "Cleric level 14 3/4-BAB progression (14 * 3 / 4) must genuinely rise to 10, up from 9 \
         at level 13: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Cleric level 14 good Fortitude (14/2+2) must genuinely rise to 9, up from 8 at level 13"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Cleric level 14 poor Reflex (14/3) must stay 4, unchanged from level 13"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 9,
        "Cleric level 14 good Will (14/2+2) must genuinely rise to 9, up from 8 at level 13"
    );
}

// ----- Channel Energy stays 7d6 at level 14 (odd-level cadence coincidence) -----

#[test]
fn cleric_level14_channel_energy_stays() {
    let input = load(CLERIC_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 7,
        "Cleric level 14 Channel Energy die count ((14 + 1) / 2) must stay 7, unchanged from \
         level 13 (the die count rises only every odd cleric level): {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 14 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );
}

// ----- Domain spell slot count stays 7 at level 14 -----

#[test]
fn cleric_level14_domain_spell_slot_count_stays() {
    let input = load(CLERIC_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 7,
        "Cleric level 14 domain spell slot count must stay 7 — 8th-level cleric spells first \
         appear only at 15th per all three primary sources' spells-per-day tables: {}",
        slot.detail
    );
}

// ----- Touch of Good genuinely rises; other domain facets carry over -----

#[test]
fn cleric_level14_touch_of_good_genuinely_rises_and_other_facets_carry_over() {
    let input = load(CLERIC_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 7,
        "Touch of Good's bonus (14 / 2) must genuinely rise to 7 at level 14, up from 6 at \
         level 13: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 14");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 14");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The domain-powers and prepared-divine-spell burdens still claim-block at level 14 -----

#[test]
fn cleric_level14_still_claim_blocks_domain_powers_and_prepared_spell_burdens() {
    let input = load(CLERIC_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-14 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: the level-13 fixture is unaffected by this widening -----

#[test]
fn cleric_level13_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Cleric level 13 base attack bonus must stay 9");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 7, "Cleric level 13 Channel Energy die count must stay 7");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 7, "Cleric level 13 domain spell slot count must stay 7");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 6, "Cleric level 13 Touch of Good bonus must stay 6");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (Superseded boundary: cycle-2026-07-15T3100 widened MAX_SUPPORTED_CLERIC_LEVEL
// from 14 to 15, then cycle-2026-07-15T5300 widened it from 15 to 16, then
// cycle-2026-07-15T9600 widened it from 16 to 17, then cycle-2026-07-15T14300
// widened it from 17 to 18, then cycle-2026-07-16T1100 widened it from 18 to
// 19, then cycle-2026-07-16T0844 widened it from 19 to 20 (the final level
// within PF1's 1-20 character-level cap), so this file's own
// negative-control boundary moves from 20 to 21, a pure implementation-gate
// check since PF1 has no 21st character level, mirroring the exact same
// boundary-move idiom applied to tests/sd18_fighter_level14_widening.rs when
// MAX_SUPPORTED_FIGHTER_LEVEL widened from 14 to 15.)

#[test]
fn cleric_level_21_is_not_promoted_by_this_slice() {
    let level_21 = CLERIC_LEVEL14_FIXTURE.replace("class:cleric:14", "class:cleric:21");
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
                && e.id != "class_feature.domain.good_touch_of_good_not_active"
                // AT-34-E3-001 cycle 6 (`49d72f5e03`, 2026-08-28) grounded Cleric
                // Weapon and Armor Proficiency unconditionally (real PF1 content,
                // any Cleric level, any multiclass mix -- not gated the way this
                // widening slice is), and the generic domain-power pass grounds
                // Healing domain's Rebuke Death uses-per-day the same way Good
                // domain's Touch of Good already is above (both domains are
                // selected in this fixture and "not level-gated, still fire" per
                // this file's own doc comment). Neither is promotion by THIS
                // slice's widening; both are pre-existing, already-tested closures.
                && e.id != "class_feature.cleric.weapon_and_armor_proficiency"
                && e.id != "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes"),
        "level-21 Cleric must not gain any bounded cleric explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level14_recognition() {
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
fn multiclass_cleric_level14_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL14_FIXTURE.replace(
        "class_level=class:cleric:14",
        "class_level=class:cleric:14\nclass_level=class:fighter:1",
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
                && e.id != "class_feature.domain.good_touch_of_good_not_active"
                // AT-34-E3-001 cycle 6 (`49d72f5e03`, 2026-08-28) grounded Cleric
                // Weapon and Armor Proficiency unconditionally (real PF1 content,
                // any Cleric level, any multiclass mix -- not gated the way this
                // widening slice is), and the generic domain-power pass grounds
                // Healing domain's Rebuke Death uses-per-day the same way Good
                // domain's Touch of Good already is above (both domains are
                // selected in this fixture and "not level-gated, still fire" per
                // this file's own doc comment). Neither is promotion by THIS
                // slice's widening; both are pre-existing, already-tested closures.
                && e.id != "class_feature.cleric.weapon_and_armor_proficiency"
                && e.id != "class_feature.cleric.domain.generic.healing_domain.rebuke_death.rebukedeathtimes"),
        "multiclass Cleric must not gain any bounded cleric explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Cleric must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_cleric_row_names_level_14_widening() {
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
        cleric.grounding_ref.contains("sd18_cleric_level14_widening"),
        "cleric row must cite the live SD18 level-14 widening proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 14") || note.contains("level-14"),
        "cleric partial note must name the level-14 widening: {note}"
    );
}
