//! SD18 Cleric level-20 widening grounding proof.
//!
//! Widens the accepted deterministic Human Cleric level-1..level-19 prepared
//! divine spell-bearing chassis (`tests/sd18_cleric_level19_widening.rs`) to
//! Cleric level 20, mirroring the sibling-class level-range-gate idiom
//! (`supported_cleric_level` is generalized from `1..=19` to `1..=20` via
//! `MAX_SUPPORTED_CLERIC_LEVEL = 20`). Level 20 is the final level within
//! PF1's 1-20 character-level cap for this class row. §3.1 race rows and
//! §3.3 interaction rows stay fully exhausted / structurally blocked (cited
//! from the progress doc, not re-derived); §3.4/§3.5 stay structurally
//! blocked for the same documented reason.
//!
//! Two independent primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly (raw `curl` fetch + a small Python
//! tag-stripper, not AI-summarized) before writing any code or test, fetching
//! the full levels-16-through-20 block in one pass so the level-20 row's
//! neighbors were visible in context (guards against level-misattribution),
//! and both agree byte-for-byte on all five rows (no self-contradiction, so a
//! third source was not required):
//!
//! - the PF1 Core Rulebook Cleric class table's level-20 "Special" column is
//!   genuinely BLANK on both primary sources — Cleric has NO named capstone
//!   class feature at 20th level (unlike Barbarian's Mighty Rage, Fighter's
//!   Weapon Mastery, Rogue's Master Strike, Paladin's Holy Champion, or
//!   Ranger's Master Hunter). This is a pure ceiling raise, exactly
//!   mirroring the level-16 and level-18 cycles' own pure ceiling raises.
//! - level 20 base attack bonus GENUINELY RISES to +15 (`20 * 3 / 4 = 15`,
//!   up from 14 at level 19).
//! - both good saves (Fortitude, Will) GENUINELY RISE to +12
//!   (`20 / 2 + 2 = 12`, up from 11 at level 19).
//! - poor Reflex STAYS at +6 (`20 / 3 = 6`, an integer-division coincidence
//!   with level 19).
//! - Channel Energy's die count STAYS at 10d6 (`(20 + 1) / 2 = 10`,
//!   unchanged from level 19 — the odd-level rise cadence's last rise landed
//!   at level 19, and PF1 character levels do not go past 20, so no further
//!   rise is possible via this formula).
//! - the flat domain spell slot count STAYS at 9 (a level-20 cleric still
//!   casts only up to 9th-level cleric spells — the highest cleric spell
//!   level in PF1, so no further domain-slot-count rise is possible; the
//!   top `domain_spell_slot_count` arm, gated on `level >=
//!   CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` (17), already covers
//!   level 20 with zero code change).
//! - Touch of Good's sacred bonus GENUINELY RISES to 10 (`20 / 2 = 10`, up
//!   from 9 at level 19) via the pre-existing formula, not re-derived; its
//!   uses-per-day pool and Rebuke Death's uses-per-day pool both stay the
//!   level-independent 3 + Wisdom modifier (6); the domain choice
//!   recognitions (Good, Healing) are not level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-19). PF1 character
//! levels cap at 20, so this closes the per-level arithmetic-widening
//! frontier for the Cleric row entirely — the row stays Partial, not
//! Supported, because the domain-power execution and prepared-spell-posture
//! burdens remain unproven, not because any further level exists to widen
//! into. It also preserves the accepted Cleric level-1..level-19 truth
//! (unchanged), the Fighter negative control, and the multiclass negative
//! control. Per the brief's lesson about stale negative controls, a
//! targeted grep for `class:cleric:20` found SEVEN stale sibling files
//! carrying a "level 20 stays claim-blocked" negative control:
//! `tests/sd13_cleric_level10_progression.rs`,
//! `tests/sd18_cleric_level11_widening.rs`, `tests/sd18_cleric_level12_widening.rs`,
//! `tests/sd18_cleric_level13_widening.rs`, `tests/sd18_cleric_level14_widening.rs`,
//! `tests/sd18_cleric_level18_widening.rs`, and `tests/sd18_cleric_level19_widening.rs`
//! — this cycle moves all seven sibling "level 20 is not promoted" negative
//! controls to a "level 21 is not promoted" boundary in the same commit,
//! purely as an implementation-gate check (PF1 does not have a 21st
//! character level; this only verifies the code's own range gate does not
//! overshoot the newly raised ceiling).

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level19_sd18_widening_deterministic_input.txt"
);

const CLERIC_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level20_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack and good saves genuinely rise at level 20; poor Reflex stays put -----

#[test]
fn cleric_level20_base_attack_and_good_saves_rise() {
    let input = load(CLERIC_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 15,
        "Cleric level 20 3/4-BAB progression (20 * 3 / 4) must GENUINELY RISE to 15, up from 14 \
         at level 19: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 12,
        "Cleric level 20 good Fortitude (20/2+2) must GENUINELY RISE to 12, up from 11 at level \
         19"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Cleric level 20 poor Reflex (20/3) must STAY at 6, an integer-division coincidence \
         with level 19"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 12,
        "Cleric level 20 good Will (20/2+2) must GENUINELY RISE to 12, up from 11 at level 19"
    );
}

// ----- Channel Energy and domain spell slot count both stay put at level 20 -----

#[test]
fn cleric_level20_channel_energy_and_domain_slot_stay_put() {
    let input = load(CLERIC_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 10,
        "Cleric level 20 Channel Energy die count ((20 + 1) / 2) must STAY at 10, unchanged \
         from level 19 — the class table's level-20 Special column is genuinely blank: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 20 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 9,
        "Cleric level 20 domain spell slot count must STAY at 9 — a level-20 cleric still casts \
         only up to 9th-level cleric spells, the highest cleric spell level in PF1: {}",
        slot.detail
    );
}

// ----- Touch of Good genuinely rises at level 20; other domain facets stay put -----

#[test]
fn cleric_level20_touch_of_good_rises_and_other_facets_stay_put() {
    let input = load(CLERIC_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 10,
        "Touch of Good's bonus (20 / 2) must GENUINELY RISE to 10 at level 20, up from 9 at \
         level 19: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 20");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 20");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn cleric_level20_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(CLERIC_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.cleric"),
        "level-20 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-20 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: level 19 truth is unchanged by this slice -----

#[test]
fn cleric_level19_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 14, "Cleric level 19 base attack bonus must stay 14");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 10, "Cleric level 19 Channel Energy die count must stay 10");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 9, "Cleric level 19 domain spell slot count must stay 9");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 9, "Cleric level 19 Touch of Good bonus must stay 9");
}

// ----- Negative control: level 21 stays unrecognized by this slice (implementation-gate -----
// ----- check only; PF1 has no 21st character level) -----

#[test]
fn cleric_level_21_is_not_promoted_by_this_slice() {
    let level_21 = CLERIC_LEVEL20_FIXTURE.replace("class:cleric:20", "class:cleric:21");
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
fn fighter_does_not_gain_cleric_level20_recognition() {
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
fn multiclass_cleric_level20_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL20_FIXTURE.replace(
        "class_level=class:cleric:20",
        "class_level=class:cleric:20\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_cleric_row_names_level_20_widening() {
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
        cleric.grounding_ref.contains("sd18_cleric_level20_widening"),
        "cleric row must cite the live SD18 level-20 widening proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 20") || note.contains("level-20"),
        "cleric partial note must name the level-20 widening: {note}"
    );
}
