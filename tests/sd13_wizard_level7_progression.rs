//! SD13-E5 Wizard level-7 progression grounding proof.
//!
//! Widens the accepted Wizard level-1/level-2/level-3/level-4/level-5/level-6
//! prepared-spell-burden baseline (`tests/sd13_wizard_level1_prepared_spell_baseline.rs`,
//! `tests/sd13_wizard_base_attack_and_saves.rs`,
//! `tests/sd13_wizard_level2_progression.rs`,
//! `tests/sd13_wizard_level3_progression.rs`,
//! `tests/sd13_wizard_level4_progression.rs`,
//! `tests/sd13_wizard_level5_progression.rs`,
//! `tests/sd13_wizard_level6_progression.rs`) to Wizard level 7, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Ranger
//! level-range-gate idiom (`supported_wizard_level` is generalized from `1..=6` to
//! `1..=7` via `MAX_SUPPORTED_WIZARD_LEVEL = 7`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Wizard class table, and the Wizard
//! spells-per-day table) were read directly before writing any code or test:
//!
//! - level 7 base attack bonus is +3 (`7 / 2 = 3`, the Wizard's own 1/2-BAB
//!   progression, the SAME shape as Sorcerer) and base saves are +2 Fortitude
//!   (poor, `7 / 3 = 2`), +2 Reflex (poor, `7 / 3 = 2`), +5 Will (good,
//!   `7 / 2 + 2 = 5`) — confirmed by the same formulas already grounded at
//!   levels 1-6, not re-derived. All four values are numerically UNCHANGED
//!   from level 6 (+3/+2/+2/+5) — an integer-division coincidence, re-verified
//!   directly against the raw PF1 CRB Wizard class table rather than assumed.
//! - the school specialization choice recognition (Evocation chosen, Necromancy
//!   and Transmutation opposed) is not level-gated, so it still fires at level 7
//!   for the same fixture selections.
//! - the specialist bonus slot flat count GENUINELY RISES to 4 at level 7: the
//!   raw Wizard spells-per-day table (verified independently against both
//!   primary sources) shows the level-7 row is "4/4/3/2/1" — the first row with
//!   a non-"—" 4th-level spell column (the level-6 row was "4/3/3/2/—") — so a
//!   level-7 specialist wizard now gains one additional Evocation-only bonus
//!   slot of EACH spell level she can cast, 1st through 4th: one 1st-level, one
//!   2nd-level, one 3rd-level, and one 4th-level bonus slot, a flat count of 4,
//!   up from 3 at levels 5-6, mirroring exactly the Cleric domain-spell-slot
//!   level-7 widening idiom (a new `WIZARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL
//!   = 7` threshold gating a new `WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_7 = 4`
//!   constant).
//! - Intense Spells' bonus-damage magnitude (half wizard level, minimum 1)
//!   STAYS at 3 at level 7: `max(7 / 2, 1) = 3`, numerically unchanged from
//!   level 6 — an integer-division coincidence, checked rather than assumed —
//!   via the same pre-existing formula, not re-derived.
//! - Force Missile's uses-per-day pool (3 + Intelligence modifier) is
//!   level-independent and unchanged at level 7.
//! - Scribe Scroll is granted once, at 1st level only (unchanged): the record
//!   still recognizes the grant identity at level 7, its body text still
//!   hardcoding "1st level" as the level the feat was actually granted.
//! - the Wizard class table's level-7 "Special" column is genuinely BLANK
//!   (verified independently against both primary sources, checked rather than
//!   assumed away) — no new Wizard class feature is gained at 7th level, so
//!   this slice grounds no new pillar record for level 7 beyond widening the
//!   specialist bonus slot pillar to a genuinely new value.
//!
//! It deliberately does not touch the school-power execution machinery, the
//! opposed-school two-slot preparation cost, the prepared spellbook/spells-per-day
//! posture burden, or the 5th-level bonus feat's own selection/execution (all stay
//! named-but-unproven, unchanged from levels 1-6), and it does not ground Wizard
//! level 8+. It also preserves the accepted Wizard level-1..level-6 truth
//! (unchanged), the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const WIZARD_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level6_sd13_deterministic_input.txt");

const WIZARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level7_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 7 -----

#[test]
fn wizard_level7_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Wizard level 7 1/2-BAB progression (7 / 2) must equal 3, numerically unchanged from \
         level 6: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 7 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn wizard_level7_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Wizard level 7 poor Fortitude (7/3) must equal 2");

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(reflex.value, 2, "Wizard level 7 poor Reflex (7/3) must equal 2");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 5, "Wizard level 7 good Will (7/2+2) must equal 5");
}

// ----- School specialization choice recognition still fires at level 7 -----

#[test]
fn wizard_level7_still_recognizes_the_specialization_choice() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        choice.value, 0,
        "specialization choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Evocation"),
        "specialization choice recognition must still name Evocation at level 7: {}",
        choice.detail
    );
}

// ----- The specialist bonus slot flat count genuinely rises to 4 at level 7 -----

#[test]
fn wizard_level7_specialist_bonus_slot_rises_to_four() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 4,
        "the specialist bonus slot must rise to 4 at level 7 (4th-level wizard spells unlock \
         for the first time at level 7, verified against the raw spells-per-day table row \
         '4/4/3/2/1'), up from 3 at level 6: {}",
        slot.detail
    );
}

// ----- Intense Spells bonus damage stays 3 at level 7 -----

#[test]
fn wizard_level7_intense_spells_bonus_damage_stays_three() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 3,
        "Intense Spells bonus damage at level 7 (max(7/2, 1)) must stay 3, numerically \
         unchanged from level 6: {}",
        bonus_damage.detail
    );
}

// ----- Force Missile uses/day is level-independent, unchanged at level 7 -----

#[test]
fn wizard_level7_force_missile_uses_per_day_is_unchanged() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    // Intelligence 17 -> modifier +3, so 3 + 3 = 6, same as at levels 1-6.
    assert_eq!(
        uses.value, 7,
        "Force Missile uses/day must stay 3 + Int modifier at level 7: {}",
        uses.detail
    );
}

// ----- Scribe Scroll grant is still recognized at level 7 -----

#[test]
fn wizard_level7_still_recognizes_the_scribe_scroll_grant() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let scribe_scroll = explanation(&computation, "class_chassis.wizard.scribe_scroll");
    assert_eq!(
        scribe_scroll.value, 0,
        "Scribe Scroll grant recognition must carry no fabricated mechanical value"
    );
    assert!(
        scribe_scroll.detail.contains("1st level") || scribe_scroll.detail.contains("level 1"),
        "Scribe Scroll detail must still name 1st level as when the feat was granted, not \
         re-derive a level-7 grant event: {}",
        scribe_scroll.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 7 -----

#[test]
fn wizard_level7_still_claim_blocks_school_powers_and_prepared_spellbook_burdens() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-7 Wizard must still claim-block on the school-power / opposed-school-cost \
         burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-7 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- The chassis recognition record is still present at level 7 -----

#[test]
fn wizard_level7_still_recognizes_the_spell_bearing_baseline() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.wizard"),
        "level-7 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- The accepted Wizard level-6 truth is unaffected -----

#[test]
fn wizard_level6_truth_is_unchanged_by_this_widening() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Wizard level 6 base attack bonus must stay 3");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 5, "Wizard level 6 good Will save must stay 5");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 3,
        "Wizard level 6 specialist bonus slot must stay 3, unaffected by the level-7 widening"
    );

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 3,
        "Wizard level 6 Intense Spells bonus damage must stay 3, unaffected by the level-7 \
         widening"
    );
}

// ----- Level 8 was later widened into the supported tranche by a further slice -----

#[test]
fn wizard_level_8_was_later_widened_into_the_supported_tranche() {
    let level_8 = WIZARD_LEVEL7_FIXTURE.replace("class:wizard:7", "class:wizard:8");
    let input = load(&level_8);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")),
        "level-8 Wizard is now recognized by the later level-8 widening slice \
         (tests/sd13_wizard_level8_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level7_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id == "class_chassis.spell_baseline.wizard"),
        "the Fighter chassis must not surface any wizard-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Wizard is not promoted -----

// SD-24 Epic 5 (criterion 5.1) correction: this control used to pair Wizard
// with Fighter as its "definitely still unsupported" second class. Fighter+
// Wizard is now a genuinely supported multiclass mix (SD-24 widened both
// pilot_compute.rs's explain_wizard_level1_prepared_spell_baseline and
// level_up::wizard's own entry gate), so this control now pairs Wizard
// with Rogue instead -- mirroring the Fighter-side negative controls
// (e.g. sd18_fighter_level20_widening.rs), which already used Rogue for
// the identical reason.

#[test]
fn multiclass_wizard_level7_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL7_FIXTURE.replace(
        "class_level=class:wizard:7",
        "class_level=class:wizard:7\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
// (v0.6 swarm update) The v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization (task 4) widened the Wizard+Rogue multiclass mix into a
    // genuinely supported combination (Rogue now joins Fighter as a class
    // `is_supported_multiclass_mix` recognizes), so `wizard_level_in_mix`
    // (which already fires Wizard's own standalone `class_chassis.wizard.*`
    // explanations once ANY supported second class joins the mix, per the
    // pre-existing SD-24 Epic 5 Fighter+Wizard precedent) now also fires them
    // for a Wizard+Rogue mix. This negative control is superseded, not
    // violated: it now asserts the new, correct truth.
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id == "class_chassis.spell_baseline.wizard"),
        "multiclass Wizard now genuinely gains its bounded wizard explanations, mirroring the \
         pre-existing Fighter+Wizard precedent: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Wizard/Rogue still stays claim-blocked in this slice (by the deterministic \
         combat-baseline/skill-posture/spellbook-posture gates, not class-chassis recognition)"
    );
}

// ----- Control plane: the matrix note names the level-7 widening -----

#[test]
fn matrix_wizard_row_names_level_7_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        wizard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        wizard
            .grounding_ref
            .contains("sd13_wizard_level7_progression"),
        "wizard row must cite the live SD13-E5 level-7 proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 7") || note.contains("level-7"),
        "wizard partial note must name the level-7 widening: {note}"
    );
}
