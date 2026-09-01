//! SD18 Cleric level-18 widening grounding proof.
//!
//! Widens the accepted deterministic Human Cleric level-1..level-17 prepared
//! divine spell-bearing chassis (`tests/sd18_cleric_level17_widening.rs`) to
//! Cleric level 18, mirroring the sibling-class level-range-gate idiom
//! (`supported_cleric_level` is generalized from `1..=17` to `1..=18` via
//! `MAX_SUPPORTED_CLERIC_LEVEL = 18`, exactly as Wizard widened its own
//! `MAX_SUPPORTED_WIZARD_LEVEL` from 17 to 18 — the loop's SECOND §3.2
//! level-18 landing, after Wizard, and the first full 9-level-caster class
//! after Wizard to reach level 18). §3.1 race rows and §3.3 interaction rows
//! stay fully exhausted / structurally blocked (cited from the progress
//! doc, not re-derived); §3.4/§3.5 stay structurally blocked for the same
//! documented reason.
//!
//! Two independent primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! fetching the full levels-16-through-19 block in one pass so the level-18
//! row's neighbors were visible in context (guards against
//! level-misattribution), and both agree byte-for-byte on all four rows (no
//! self-contradiction, so a third source was not required):
//!
//! - level 18 base attack bonus GENUINELY RISES to +13 (`18 * 3 / 4 = 13`,
//!   up from 12 at level 17) and both good saves (Fortitude, Will)
//!   GENUINELY RISE to +11 (`18 / 2 + 2 = 11`, up from 10 at level 17),
//!   while poor Reflex GENUINELY RISES to +6 (`18 / 3 = 6`, up from 5 at
//!   level 17).
//! - the PF1 Core Rulebook Cleric class table's level-18 "Special" column
//!   is genuinely BLANK on both primary sources — a pure ceiling raise,
//!   exactly mirroring the Wizard level-18 cycle's own pure ceiling raise.
//! - Channel Energy's die count STAYS at 9d6 (`(18 + 1) / 2 = 9`, an
//!   integer-division coincidence with level 17, the odd-level cadence's
//!   next rise landing at 19th).
//! - the flat domain spell slot count STAYS at 9 (a level-18 cleric still
//!   casts only up to 9th-level cleric spells — the highest cleric spell
//!   level in PF1, so no further domain-slot-count rise is possible; this
//!   mirrors the Wizard level-18 cycle's own specialist-bonus-slot top arm
//!   already covering level 18 with zero code change).
//! - Touch of Good's sacred bonus GENUINELY RISES to 9 (`18 / 2 = 9`, up
//!   from 8 at level 17) via the pre-existing formula, not re-derived; its
//!   uses-per-day pool and Rebuke Death's uses-per-day pool both stay the
//!   level-independent 3 + Wisdom modifier (6); the domain choice
//!   recognitions (Good, Healing) are not level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-17), and it does not
//! ground Cleric level 19+. It also preserves the accepted Cleric
//! level-1..level-17 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the brief's lesson about stale
//! negative controls, a targeted grep for `class:cleric:18` found FIVE stale
//! sibling files carrying a "level 18 stays claim-blocked" negative control:
//! `tests/sd13_cleric_level10_progression.rs`,
//! `tests/sd18_cleric_level11_widening.rs`, `tests/sd18_cleric_level12_widening.rs`,
//! `tests/sd18_cleric_level13_widening.rs`, and `tests/sd18_cleric_level14_widening.rs`
//! — this cycle moves all five sibling "level 18 is not promoted" negative
//! controls to a "level 19 is not promoted" boundary in the same commit;
//! `tests/sd18_cleric_level17_widening.rs`'s own "level 18 is not promoted"
//! test is removed rather than moved, since level 18 is now itself the
//! supported/grounded row rather than the out-of-range boundary, mirroring
//! the Wizard level-18 cycle's identical fix.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level17_sd18_widening_deterministic_input.txt"
);

const CLERIC_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level18_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and all three base saves genuinely rise at level 18 -----

#[test]
fn cleric_level18_base_attack_and_saves_genuinely_rise() {
    let input = load(CLERIC_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 13,
        "Cleric level 18 3/4-BAB progression (18 * 3 / 4) must GENUINELY RISE to 13, up from 12 \
         at level 17: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 11,
        "Cleric level 18 good Fortitude (18/2+2) must GENUINELY RISE to 11, up from 10 at level \
         17"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Cleric level 18 poor Reflex (18/3) must GENUINELY RISE to 6, up from 5 at level 17"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 11,
        "Cleric level 18 good Will (18/2+2) must GENUINELY RISE to 11, up from 10 at level 17"
    );
}

// ----- Channel Energy and domain spell slot count both stay put at level 18 -----

#[test]
fn cleric_level18_channel_energy_and_domain_slot_stay_put() {
    let input = load(CLERIC_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 9,
        "Cleric level 18 Channel Energy die count ((18 + 1) / 2) must STAY at 9, an \
         integer-division coincidence with level 17: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 18 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 9,
        "Cleric level 18 domain spell slot count must STAY at 9 — a level-18 cleric still casts \
         only up to 9th-level cleric spells, the highest cleric spell level in PF1: {}",
        slot.detail
    );
}

// ----- Touch of Good genuinely rises at level 18; other domain facets carry over -----

#[test]
fn cleric_level18_touch_of_good_genuinely_rises_and_other_facets_carry_over() {
    let input = load(CLERIC_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 9,
        "Touch of Good's bonus (18 / 2) must GENUINELY RISE to 9 at level 18, up from 8 at level \
         17: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 18");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 18");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn cleric_level18_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(CLERIC_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.cleric"),
        "level-18 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-18 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: level 17 truth is unchanged by this widening -----

#[test]
fn cleric_level17_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 12, "Cleric level 17 base attack bonus must stay 12");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 9, "Cleric level 17 Channel Energy die count must stay 9");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 9, "Cleric level 17 domain spell slot count must stay 9");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 8, "Cleric level 17 Touch of Good bonus must stay 8");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (cycle-2026-07-16T1100 moved this file's own boundary from 19 to 20, since
// level 19 was then itself Cleric's supported/grounded row;
// cycle-2026-07-16T0844 moves this boundary again, from 20 to 21, since
// level 20 is now itself Cleric's supported/grounded row — and the final
// level within PF1's 1-20 character-level cap, so this boundary check is now
// a pure implementation-gate check with no further real level to move to.)

#[test]
fn cleric_level_21_is_not_promoted_by_this_slice() {
    let level_21 = CLERIC_LEVEL18_FIXTURE.replace("class:cleric:18", "class:cleric:21");
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
fn fighter_does_not_gain_cleric_level18_recognition() {
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
fn multiclass_cleric_level18_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL18_FIXTURE.replace(
        "class_level=class:cleric:18",
        "class_level=class:cleric:18\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-18 widening -----

#[test]
fn matrix_cleric_row_names_level_18_widening() {
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
        cleric.grounding_ref.contains("sd18_cleric_level18_widening"),
        "cleric row must cite the live SD18 level-18 widening proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 18") || note.contains("level-18"),
        "cleric partial note must name the level-18 widening: {note}"
    );
}
