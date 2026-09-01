//! SD18 Cleric level-19 widening grounding proof.
//!
//! Widens the accepted deterministic Human Cleric level-1..level-18 prepared
//! divine spell-bearing chassis (`tests/sd18_cleric_level18_widening.rs`) to
//! Cleric level 19, mirroring the sibling-class level-range-gate idiom
//! (`supported_cleric_level` is generalized from `1..=18` to `1..=19` via
//! `MAX_SUPPORTED_CLERIC_LEVEL = 19` — the loop's SECOND §3.2 level-19
//! landing, after Barbarian, opening the level-19 sweep for the 9-level-caster
//! classes). §3.1 race rows and §3.3 interaction rows stay fully exhausted /
//! structurally blocked (cited from the progress doc, not re-derived);
//! §3.4/§3.5 stay structurally blocked for the same documented reason.
//!
//! Two independent primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly (raw `curl` fetch + a small Python
//! tag-stripper, not AI-summarized) before writing any code or test, fetching
//! the full levels-16-through-20 block in one pass so the level-19 row's
//! neighbors were visible in context (guards against level-misattribution),
//! and both agree byte-for-byte on all five rows (no self-contradiction, so a
//! third source was not required):
//!
//! - level 19 base attack bonus GENUINELY RISES to +14 (`19 * 3 / 4 = 14`,
//!   up from 13 at level 18), while both good saves (Fortitude, Will) STAY
//!   at +11 (`19 / 2 + 2 = 11`, an integer-division coincidence with level
//!   18) and poor Reflex STAYS at +6 (`19 / 3 = 6`, also an integer-division
//!   coincidence with level 18).
//! - the PF1 Core Rulebook Cleric class table's level-19 "Special" column
//!   reads "Channel energy 10d6" on both primary sources — Channel Energy's
//!   die count GENUINELY RISES to 10d6 (`(19 + 1) / 2 = 10`, up from 9d6 at
//!   level 18) via the same pre-existing formula, not re-derived. This names
//!   only a tier-rise on the already-grounded Channel Energy dice pillar,
//!   not a new class feature, mirroring the level-17 cycle's identical
//!   "Channel energy 9d6" finding, so no new pillar record is added.
//! - the flat domain spell slot count STAYS at 9 (a level-19 cleric still
//!   casts only up to 9th-level cleric spells — the highest cleric spell
//!   level in PF1, so no further domain-slot-count rise is possible; the
//!   top `domain_spell_slot_count` arm, gated on `level >=
//!   CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` (17), already covers
//!   level 19 with zero code change).
//! - Touch of Good's sacred bonus STAYS at 9 (`19 / 2 = 9`, an
//!   integer-division coincidence with level 18) via the pre-existing
//!   formula, not re-derived; its uses-per-day pool and Rebuke Death's
//!   uses-per-day pool both stay the level-independent 3 + Wisdom modifier
//!   (6); the domain choice recognitions (Good, Healing) are not
//!   level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-18), and it does not
//! ground Cleric level 20+. It also preserves the accepted Cleric
//! level-1..level-18 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the brief's lesson about stale
//! negative controls, a targeted grep for `class:cleric:19` found SIX stale
//! sibling files carrying a "level 19 stays claim-blocked" negative control:
//! `tests/sd13_cleric_level10_progression.rs`,
//! `tests/sd18_cleric_level11_widening.rs`, `tests/sd18_cleric_level12_widening.rs`,
//! `tests/sd18_cleric_level13_widening.rs`, `tests/sd18_cleric_level14_widening.rs`,
//! and `tests/sd18_cleric_level18_widening.rs`
//! — this cycle moves all six sibling "level 19 is not promoted" negative
//! controls to a "level 20 is not promoted" boundary in the same commit;
//! `tests/sd18_cleric_level18_widening.rs`'s own "level 19 is not promoted"
//! test is moved (not removed, since level 19 is a real new supported row
//! but level 20 remains unrecognized).

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level18_sd18_widening_deterministic_input.txt"
);

const CLERIC_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level19_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack genuinely rises at level 19; all three base saves stay put -----

#[test]
fn cleric_level19_base_attack_rises_and_saves_stay_put() {
    let input = load(CLERIC_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 14,
        "Cleric level 19 3/4-BAB progression (19 * 3 / 4) must GENUINELY RISE to 14, up from 13 \
         at level 18: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 11,
        "Cleric level 19 good Fortitude (19/2+2) must STAY at 11, an integer-division \
         coincidence with level 18"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Cleric level 19 poor Reflex (19/3) must STAY at 6, an integer-division coincidence \
         with level 18"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 11,
        "Cleric level 19 good Will (19/2+2) must STAY at 11, an integer-division coincidence \
         with level 18"
    );
}

// ----- Channel Energy genuinely rises at level 19; domain spell slot count stays put -----

#[test]
fn cleric_level19_channel_energy_rises_and_domain_slot_stays_put() {
    let input = load(CLERIC_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 10,
        "Cleric level 19 Channel Energy die count ((19 + 1) / 2) must GENUINELY RISE to 10, up \
         from 9 at level 18: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 19 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 9,
        "Cleric level 19 domain spell slot count must STAY at 9 — a level-19 cleric still casts \
         only up to 9th-level cleric spells, the highest cleric spell level in PF1: {}",
        slot.detail
    );
}

// ----- Touch of Good and other domain facets all stay put at level 19 -----

#[test]
fn cleric_level19_touch_of_good_and_other_facets_stay_put() {
    let input = load(CLERIC_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 9,
        "Touch of Good's bonus (19 / 2) must STAY at 9 at level 19, an integer-division \
         coincidence with level 18: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 19");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 19");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn cleric_level19_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(CLERIC_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.cleric"),
        "level-19 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-19 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: level 18 truth is unchanged by this slice -----

#[test]
fn cleric_level18_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 13, "Cleric level 18 base attack bonus must stay 13");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 9, "Cleric level 18 Channel Energy die count must stay 9");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 9, "Cleric level 18 domain spell slot count must stay 9");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 9, "Cleric level 18 Touch of Good bonus must stay 9");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (cycle-2026-07-16T0844 moves this file's own boundary from 20 to 21, since
// level 20 is now itself Cleric's supported/grounded row — and the final
// level within PF1's 1-20 character-level cap, so this boundary check is now
// a pure implementation-gate check with no further real level to move to.)

#[test]
fn cleric_level_21_is_not_promoted_by_this_slice() {
    let level_21 = CLERIC_LEVEL19_FIXTURE.replace("class:cleric:19", "class:cleric:21");
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
fn fighter_does_not_gain_cleric_level19_recognition() {
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
fn multiclass_cleric_level19_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL19_FIXTURE.replace(
        "class_level=class:cleric:19",
        "class_level=class:cleric:19\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-19 widening -----

#[test]
fn matrix_cleric_row_names_level_19_widening() {
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
        cleric.grounding_ref.contains("sd18_cleric_level19_widening"),
        "cleric row must cite the live SD18 level-19 widening proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 19") || note.contains("level-19"),
        "cleric partial note must name the level-19 widening: {note}"
    );
}
