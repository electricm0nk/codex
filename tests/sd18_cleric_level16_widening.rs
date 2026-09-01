//! SD18 Cleric level-16 widening grounding proof.
//!
//! Widens the accepted deterministic Human Cleric level-1..level-15 prepared
//! divine spell-bearing chassis (`tests/sd18_cleric_level15_widening.rs`) to
//! Cleric level 16, mirroring the sibling-class level-range-gate idiom
//! (`supported_cleric_level` is generalized from `1..=15` to `1..=16` via
//! `MAX_SUPPORTED_CLERIC_LEVEL = 16`, exactly as Barbarian, Fighter, Wizard,
//! and Rogue widened their own `MAX_SUPPORTED_<CLASS>_LEVEL` from 15 to 16 —
//! the loop's FIFTH §3.2 level-16 landing, after Barbarian, Fighter, Wizard,
//! and Rogue, and the second full 9-level-caster class (after Wizard) to
//! reach level 16 in the level-16 sweep). §3.1 race rows and §3.3
//! interaction rows stay fully exhausted / structurally blocked (cited from
//! the progress doc, not re-derived); §3.4/§3.5 stay structurally blocked
//! for the same documented reason.
//!
//! Two independent primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte (no self-contradiction, so a third source
//! was not required):
//!
//! - level 16 base attack bonus GENUINELY RISES to +12 (`16 * 3 / 4 = 12`,
//!   up from +11 at level 15) and both good saves (Fortitude, Will)
//!   GENUINELY RISE to +10 (`16 / 2 + 2 = 10`, up from +9 at level 15),
//!   while poor Reflex STAYS at +5 (`16 / 3 = 5`, an integer-division
//!   coincidence with level 15).
//! - the PF1 Core Rulebook Cleric class table's level-16 "Special" column is
//!   genuinely BLANK on both primary sources — the next Channel Energy
//!   die-count rise (the odd-cleric-level cadence already grounded at every
//!   prior odd tier) does not land until 17th level — so this is a pure
//!   ceiling raise: no new pillar record is grounded from the Special
//!   column, exactly mirroring the Wizard level-16 cycle's own pure ceiling
//!   raise.
//! - Channel Energy's die count STAYS 8d6 (`(16 + 1) / 2 = 8`, an
//!   integer-division coincidence with level 15) via the same pre-existing
//!   formula, not re-derived; its uses-per-day pool stays the
//!   level-independent 3 + Charisma modifier (5).
//! - the flat domain spell slot count STAYS 8 (a level-16 cleric still casts
//!   only up to 8th-level cleric spells; 9th-level cleric spells are out of
//!   this bounded ceiling's scope) — the pre-existing
//!   `>= CLERIC_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` branch already
//!   covers level 16, so no new threshold constant is needed.
//! - the Good domain's Touch of Good sacred bonus GENUINELY RISES to 8
//!   (`16 / 2 = 8`, up from 7 at level 15) via the pre-existing formula; its
//!   uses-per-day pool and Rebuke Death's uses-per-day pool both stay the
//!   level-independent 3 + Wisdom modifier (6); the domain choice
//!   recognitions (Good, Healing) are not level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-15), and it does not
//! ground Cleric level 17+. It also preserves the accepted Cleric
//! level-1..level-15 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the brief's lesson about stale
//! negative controls, a targeted grep for `cleric.*is_not_promoted` and
//! `class:cleric:16` found FIVE stale sibling files carrying a "level 16
//! stays claim-blocked" negative control: `tests/sd13_cleric_level10_progression.rs`,
//! `tests/sd18_cleric_level11_widening.rs`, `tests/sd18_cleric_level12_widening.rs`,
//! `tests/sd18_cleric_level13_widening.rs`, and `tests/sd18_cleric_level14_widening.rs`
//! — this cycle moves all five sibling "level 16 is not promoted" negative
//! controls to a "level 17 is not promoted" boundary in the same commit;
//! `tests/sd18_cleric_level15_widening.rs`'s own "level 16 is not promoted"
//! test is removed rather than moved, since level 16 is now itself the
//! supported/grounded row rather than the out-of-range boundary, mirroring
//! the Barbarian/Fighter/Wizard/Rogue level-16 cycles' identical fix.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level15_sd18_widening_deterministic_input.txt"
);

const CLERIC_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level16_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and good saves genuinely rise at level 16; poor Reflex stays -----

#[test]
fn cleric_level16_base_attack_and_good_saves_genuinely_rise() {
    let input = load(CLERIC_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Cleric level 16 3/4-BAB progression (16 * 3 / 4) must GENUINELY RISE to 12, up from 11 \
         at level 15: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Cleric level 16 good Fortitude (16/2+2) must GENUINELY RISE to 10, up from 9 at level 15"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Cleric level 16 poor Reflex (16/3) must STAY at 5 — an integer-division coincidence \
         with level 15"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 10,
        "Cleric level 16 good Will (16/2+2) must GENUINELY RISE to 10, up from 9 at level 15"
    );
}

// ----- Channel Energy stays put at level 16 (die count rises only at odd levels) -----

#[test]
fn cleric_level16_channel_energy_stays_put() {
    let input = load(CLERIC_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 8,
        "Cleric level 16 Channel Energy die count ((16 + 1) / 2) must STAY at 8 — an \
         integer-division coincidence with level 15: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 16 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );
}

// ----- Domain spell slot count stays put at level 16 (no new spell level opens) -----

#[test]
fn cleric_level16_domain_spell_slot_count_stays_put_at_eight() {
    let input = load(CLERIC_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 8,
        "Cleric level 16 domain spell slot count must STAY at 8 — a level-16 cleric still casts \
         only up to 8th-level cleric spells, out of this bounded ceiling's scope: {}",
        slot.detail
    );
}

// ----- Touch of Good genuinely rises at level 16; other domain facets carry over -----

#[test]
fn cleric_level16_touch_of_good_genuinely_rises_and_other_facets_carry_over() {
    let input = load(CLERIC_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 8,
        "Touch of Good's bonus (16 / 2) must GENUINELY RISE to 8 at level 16, up from 7 at level \
         15: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 16");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 16");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn cleric_level16_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(CLERIC_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.cleric"),
        "level-16 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-16 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: level 15 truth is unchanged by this widening -----

#[test]
fn cleric_level15_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 11, "Cleric level 15 base attack bonus must stay 11");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 8, "Cleric level 15 Channel Energy die count must stay 8");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 8, "Cleric level 15 domain spell slot count must stay 8");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 7, "Cleric level 15 Touch of Good bonus must stay 7");
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level16_recognition() {
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
fn multiclass_cleric_level16_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL16_FIXTURE.replace(
        "class_level=class:cleric:16",
        "class_level=class:cleric:16\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-16 widening -----

#[test]
fn matrix_cleric_row_names_level_16_widening() {
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
        cleric.grounding_ref.contains("sd18_cleric_level16_widening"),
        "cleric row must cite the live SD18 level-16 widening proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 16") || note.contains("level-16"),
        "cleric partial note must name the level-16 widening: {note}"
    );
}
