//! SD18 Cleric level-17 widening grounding proof.
//!
//! Widens the accepted deterministic Human Cleric level-1..level-16 prepared
//! divine spell-bearing chassis (`tests/sd18_cleric_level16_widening.rs`) to
//! Cleric level 17, mirroring the sibling-class level-range-gate idiom
//! (`supported_cleric_level` is generalized from `1..=16` to `1..=17` via
//! `MAX_SUPPORTED_CLERIC_LEVEL = 17`, exactly as Ranger, Bard, Rogue,
//! Fighter, and Wizard widened their own `MAX_SUPPORTED_<CLASS>_LEVEL` from
//! 16 to 17 — the loop's SIXTH §3.2 level-17 landing, after Ranger, Bard,
//! Rogue, Fighter, and Wizard, and the second full 9-level-caster class
//! (after Wizard) to reach level 17). §3.1 race rows and §3.3 interaction
//! rows stay fully exhausted / structurally blocked (cited from the
//! progress doc, not re-derived); §3.4/§3.5 stay structurally blocked for
//! the same documented reason.
//!
//! Two independent primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! fetching the full levels-14-through-18 block in one pass so the level-17
//! row's neighbors were visible in context (guards against
//! level-misattribution), and both agree byte-for-byte on all five rows (no
//! self-contradiction, so a third source was not required):
//!
//! - level 17 base attack bonus STAYS at +12 (`17 * 3 / 4 = 12`, an
//!   integer-division coincidence with level 16) and both good saves
//!   (Fortitude, Will) STAY at +10 (`17 / 2 + 2 = 10`, an integer-division
//!   coincidence with level 16), while poor Reflex STAYS at +5
//!   (`17 / 3 = 5`, also an integer-division coincidence with level 16).
//! - the PF1 Core Rulebook Cleric class table's level-17 "Special" column
//!   reads "Channel energy 9d6" on both primary sources — Channel Energy's
//!   die count GENUINELY RISES to 9d6 (`(17 + 1) / 2 = 9`, up from 8d6 at
//!   level 16) via the same pre-existing formula, not re-derived; this is
//!   the odd-level cadence's next rise, exactly as predicted by the level-16
//!   cycle's own comment. No new class-feature pillar is named — "Channel
//!   energy 9d6" is a tier-rise on the already-grounded Channel Energy dice
//!   pillar, not a new feature.
//! - the flat domain spell slot count GENUINELY RISES to 9 (a level-17
//!   cleric casts 9th-level cleric spells for the first time — the
//!   spells-per-day table's 9th-level column is "—" at level 16 and first
//!   shows "1+1" at level 17 on both primary sources), via the same
//!   one-slot-per-castable-spell-level rule, not re-derived. This mirrors
//!   the Wizard level-17 cycle's own 9th-level spell column opening for the
//!   same class-level reason.
//! - Channel Energy's uses-per-day pool stays the level-independent
//!   3 + Charisma modifier (5); the Good domain's Touch of Good sacred bonus
//!   STAYS at 8 (`17 / 2 = 8`, an integer-division coincidence with level
//!   16) via the pre-existing formula; its uses-per-day pool and Rebuke
//!   Death's uses-per-day pool both stay the level-independent
//!   3 + Wisdom modifier (6); the domain choice recognitions (Good, Healing)
//!   are not level-gated and still fire.
//!
//! It deliberately does not touch the domain-power execution burden (Touch
//! of Good's touch-attack resolution, Rebuke Death's heal amount and
//! hit-point-state gating) or the prepared divine spell posture burden (both
//! stay named-but-unproven, unchanged from levels 1-16), and it does not
//! ground Cleric level 18+. It also preserves the accepted Cleric
//! level-1..level-16 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the brief's lesson about stale
//! negative controls, a targeted grep for `class:cleric:17` found FIVE stale
//! sibling files carrying a "level 17 stays claim-blocked" negative control:
//! `tests/sd13_cleric_level10_progression.rs`,
//! `tests/sd18_cleric_level11_widening.rs`, `tests/sd18_cleric_level12_widening.rs`,
//! `tests/sd18_cleric_level13_widening.rs`, and `tests/sd18_cleric_level14_widening.rs`
//! — this cycle moves all five sibling "level 17 is not promoted" negative
//! controls to a "level 18 is not promoted" boundary in the same commit;
//! `tests/sd18_cleric_level16_widening.rs`'s own "level 17 is not promoted"
//! test is removed rather than moved, since level 17 is now itself the
//! supported/grounded row rather than the out-of-range boundary, mirroring
//! the Ranger/Bard/Rogue/Fighter/Wizard level-17 cycles' identical fix.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const CLERIC_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level16_sd18_widening_deterministic_input.txt"
);

const CLERIC_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_cleric_level17_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and all three base saves stay put at level 17 -----

#[test]
fn cleric_level17_base_attack_and_saves_stay_put() {
    let input = load(CLERIC_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Cleric level 17 3/4-BAB progression (17 * 3 / 4) must STAY at 12, an integer-division \
         coincidence with level 16: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.cleric.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Cleric level 17 good Fortitude (17/2+2) must STAY at 10, an integer-division \
         coincidence with level 16"
    );

    let reflex = explanation(&computation, "class_chassis.cleric.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Cleric level 17 poor Reflex (17/3) must STAY at 5 — an integer-division coincidence \
         with level 16"
    );

    let will = explanation(&computation, "class_chassis.cleric.base_save.will");
    assert_eq!(
        will.value, 10,
        "Cleric level 17 good Will (17/2+2) must STAY at 10, an integer-division coincidence \
         with level 16"
    );
}

// ----- Channel Energy genuinely rises at level 17 (the odd-level cadence's next tier) -----

#[test]
fn cleric_level17_channel_energy_genuinely_rises() {
    let input = load(CLERIC_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(
        dice.value, 9,
        "Cleric level 17 Channel Energy die count ((17 + 1) / 2) must GENUINELY RISE to 9, up \
         from 8 at level 16: {}",
        dice.detail
    );

    let uses = explanation(
        &computation,
        "class_chassis.cleric.channel_energy_uses_per_day",
    );
    assert_eq!(
        uses.value, 5,
        "Cleric level 17 Channel Energy uses per day (3 + Charisma modifier 2) must stay 5"
    );
}

// ----- Domain spell slot count genuinely rises at level 17 (9th-level spells open) -----

#[test]
fn cleric_level17_domain_spell_slot_count_genuinely_rises_to_nine() {
    let input = load(CLERIC_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(
        slot.value, 9,
        "Cleric level 17 domain spell slot count must GENUINELY RISE to 9 — a level-17 cleric \
         casts 9th-level cleric spells for the first time: {}",
        slot.detail
    );
}

// ----- Touch of Good stays put at level 17; other domain facets carry over -----

#[test]
fn cleric_level17_touch_of_good_stays_put_and_other_facets_carry_over() {
    let input = load(CLERIC_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(
        bonus.value, 8,
        "Touch of Good's bonus (17 / 2) must STAY at 8 at level 17, an integer-division \
         coincidence with level 16: {}",
        bonus.detail
    );

    let tog_uses = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_uses_per_day",
    );
    assert_eq!(tog_uses.value, 7, "Touch of Good's uses per day must stay 7 at level 17");

    let rebuke_uses = explanation(
        &computation,
        "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day",
    );
    assert_eq!(rebuke_uses.value, 7, "Rebuke Death's uses per day must stay 7 at level 17");

    let domain_choice = explanation(&computation, "class_chassis.cleric.domain_choice");
    assert_eq!(domain_choice.value, 0, "the domain choice seam must still carry no mechanical value");
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn cleric_level17_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(CLERIC_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.cleric"),
        "level-17 Cleric must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported" && d.claim_blocking),
        "level-17 Cleric must still claim-block on the domain powers burden: {:?}",
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

// ----- Negative control: level 16 truth is unchanged by this widening -----

#[test]
fn cleric_level16_truth_is_unchanged_by_this_slice() {
    let input = load(CLERIC_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.cleric.base_attack_bonus");
    assert_eq!(base_attack.value, 12, "Cleric level 16 base attack bonus must stay 12");

    let dice = explanation(&computation, "class_chassis.cleric.channel_energy_dice");
    assert_eq!(dice.value, 8, "Cleric level 16 Channel Energy die count must stay 8");

    let slot = explanation(&computation, "class_chassis.cleric.domain_spell_slot");
    assert_eq!(slot.value, 8, "Cleric level 16 domain spell slot count must stay 8");

    let bonus = explanation(
        &computation,
        "class_feature.domain.good_touch_of_good_bonus",
    );
    assert_eq!(bonus.value, 8, "Cleric level 16 Touch of Good bonus must stay 8");
}

// ----- Negative control: the cleric path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_cleric_level17_recognition() {
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
fn multiclass_cleric_level17_is_not_promoted_by_this_slice() {
    let multiclass = CLERIC_LEVEL17_FIXTURE.replace(
        "class_level=class:cleric:17",
        "class_level=class:cleric:17\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_cleric_row_names_level_17_widening() {
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
        cleric.grounding_ref.contains("sd18_cleric_level17_widening"),
        "cleric row must cite the live SD18 level-17 widening proof surface: {}",
        cleric.grounding_ref
    );
    let note = cleric.blocker_or_lossiness_note;
    assert!(
        note.contains("level 17") || note.contains("level-17"),
        "cleric partial note must name the level-17 widening: {note}"
    );
}
