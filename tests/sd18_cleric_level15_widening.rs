//! SD18 Cleric level-15 widening grounding proof.
//!
//! Widens the accepted Cleric level-1..level-14 prepared divine spell-bearing
//! baseline (`tests/sd18_cleric_level14_widening.rs`, the loop's most recent
//! Cleric ceiling) to Cleric level 15 — mirroring the sibling-class
//! level-range-gate idiom (`supported_cleric_level` is generalized from
//! `1..=14` to `1..=15` via `MAX_SUPPORTED_CLERIC_LEVEL = 15`, exactly as
//! `cycle-2026-07-15T2800` widened `MAX_SUPPORTED_BARBARIAN_LEVEL`,
//! `cycle-2026-07-15T2900` widened `MAX_SUPPORTED_ROGUE_LEVEL`, and
//! `cycle-2026-07-15T3000` widened `MAX_SUPPORTED_FIGHTER_LEVEL`, all from 14
//! to 15, and the first §3.2 level-15 landing on a full 9-level-caster
//! class). Two independent primary sources (d20pfsrd and the Archives of
//! Nethys aonprd.com mirror) were read directly before writing any code or
//! test and agree byte-for-byte on the level-15 class-table row; a third
//! source (legacy.aonprd.com's raw spells-per-day table) was consulted
//! specifically to settle the 8th-level domain-spell-slot threshold question,
//! since two summarized single-row fetches disagreed with each other
//! (one claimed level 16, one claimed level 17) in a way that broke the
//! established every-other-odd-level cadence already grounded in this
//! codebase (1st@1, 2nd@3, 3rd@5, 4th@7, 5th@9, 6th@11, 7th@13) — both
//! single-row fetches were rejected as tool artifacts, not genuine source
//! conflicts, and legacy.aonprd.com's full multi-row table extraction
//! (levels 14-17 side by side, internally consistent and matching the
//! established odd-level cadence) was treated as authoritative:
//!
//! - level 15 base attack bonus GENUINELY RISES to +11 (`15 * 3 / 4 = 11`,
//!   up from +10 at level 14) and base Reflex (poor save) GENUINELY RISES to
//!   +5 (`15 / 3 = 5`, up from +4), while both good saves (Fortitude, Will)
//!   STAY +9 (`15 / 2 + 2 = 9`, an integer-division coincidence with level
//!   14), checked rather than assumed.
//! - the PF1 Core Rulebook Cleric class table's level-15 "Special" column
//!   reads "Channel energy 8d6" (verified independently against both
//!   d20pfsrd and the Archives of Nethys aonprd.com mirror, byte-for-byte
//!   agreement) — Channel Energy's die count GENUINELY RISES to 8d6
//!   (`(15 + 1) / 2 = 8`, up from 7d6 at level 14) via the same
//!   pre-existing formula, not re-derived; its uses-per-day pool stays the
//!   level-independent 3 + Charisma modifier (5). This is a widening of the
//!   already-grounded Channel Energy die-count pillar, not a new class
//!   feature — no new pillar record is added.
//! - level 15 IS when a cleric first casts 8th-level cleric spells (verified
//!   against legacy.aonprd.com's raw spells-per-day table rows, which show
//!   the 8th-level spell column as "—" through level 14 and first
//!   non-"—" ("1+1") at level 15), so the flat domain spell slot count
//!   GENUINELY RISES to 8 (one each of 1st through 8th-level domain slots)
//!   via the same one-slot-per-castable-spell-level rule, gated on a new
//!   `CLERIC_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 15` threshold
//!   constant, no new formula invented.
//! - Touch of Good's bonus STAYS 7 (`15 / 2 = 7`, an integer-division
//!   coincidence with level 14) via the same pre-existing formula; its
//!   uses-per-day pool and Rebuke Death's uses-per-day pool both stay the
//!   level-independent 3 + Wisdom modifier (6); the domain choice
//!   recognitions (Good, Healing) are not level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-14), and it does not
//! ground Cleric level 16+. It also preserves the accepted Cleric
//! level-1..level-14 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level14_sd18_widening_deterministic_input.txt"
);

const CLERIC_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level15_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and poor Reflex genuinely rise at level 15 -----

#[test]
fn cleric_level15_base_attack_and_poor_reflex_genuinely_rise() {
    let input = load(CLERIC_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 11,
        "Cleric level 15 3/4-BAB progression (15 * 3 / 4) must genuinely rise to 11, up from 10 \
         at level 14: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Cleric level 15 good Fortitude (15/2+2) must stay 9, unchanged from level 14"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Cleric level 15 poor Reflex (15/3) must genuinely rise to 5, up from 4 at level 14"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 9,
        "Cleric level 15 good Will (15/2+2) must stay 9, unchanged from level 14"
    );
}

// ----- Channel Energy genuinely rises to 8d6 at level 15 -----

#[test]
fn cleric_level15_channel_energy_genuinely_rises() {
    let input = load(CLERIC_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 8,
        "Cleric level 15 Channel Energy die count ((15 + 1) / 2) must genuinely rise to 8, up \
         from 7 at level 14: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 15 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );
}

// ----- Domain spell slot count genuinely rises to 8 at level 15 -----

#[test]
fn cleric_level15_domain_spell_slot_count_genuinely_rises() {
    let input = load(CLERIC_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 8,
        "Cleric level 15 domain spell slot count must genuinely rise to 8 — 8th-level cleric \
         spells first appear at 15th per legacy.aonprd.com's raw spells-per-day table: {}",
        slot.detail
    );
}

// ----- Touch of Good stays; other domain facets carry over -----

#[test]
fn cleric_level15_touch_of_good_stays_and_other_facets_carry_over() {
    let input = load(CLERIC_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 7,
        "Touch of Good's bonus (15 / 2) must stay 7 at level 15, unchanged from level 14: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 15");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 15");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The domain-powers and prepared-divine-spell burdens still claim-block at level 15 -----

#[test]
fn cleric_level15_still_claim_blocks_domain_powers_and_prepared_spell_burdens() {
    let input = load(CLERIC_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-15 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: the level-14 fixture is unaffected by this widening -----

#[test]
fn cleric_level14_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 10, "Cleric level 14 base attack bonus must stay 10");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 7, "Cleric level 14 Channel Energy die count must stay 7");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 7, "Cleric level 14 domain spell slot count must stay 7");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 7, "Cleric level 14 Touch of Good bonus must stay 7");
}

// (This file's own "level 16 is not promoted" negative control is removed
// rather than moved, since cycle-2026-07-15T5300 promotes level 16 to the
// supported/grounded row — see tests/sd18_cleric_level16_widening.rs for the
// level-17 negative control that now supersedes it.)

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level15_recognition() {
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
fn multiclass_cleric_level15_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL15_FIXTURE.replace(
        "class_level=class:cleric:15",
        "class_level=class:cleric:15\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_cleric_row_names_level_15_widening() {
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
        cleric.grounding_ref.contains("sd18_cleric_level15_widening"),
        "cleric row must cite the live SD18 level-15 widening proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "cleric partial note must name the level-15 widening: {note}"
    );
}
