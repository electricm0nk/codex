//! SD13-E5 Wizard level-6 progression grounding proof.
//!
//! Widens the accepted Wizard level-1/level-2/level-3/level-4/level-5 prepared-
//! spell-burden baseline (`tests/sd13_wizard_level1_prepared_spell_baseline.rs`,
//! `tests/sd13_wizard_base_attack_and_saves.rs`,
//! `tests/sd13_wizard_level2_progression.rs`,
//! `tests/sd13_wizard_level3_progression.rs`,
//! `tests/sd13_wizard_level4_progression.rs`,
//! `tests/sd13_wizard_level5_progression.rs`) to Wizard level 6, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Ranger
//! level-range-gate idiom (`supported_wizard_level` is generalized from `1..=5` to
//! `1..=6` via `MAX_SUPPORTED_WIZARD_LEVEL = 6`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com) were read directly before writing any code or
//! test:
//!
//! - level 6 base attack bonus is +3 (`6 / 2 = 3`, the Wizard's own 1/2-BAB
//!   progression, the SAME shape as Sorcerer) and base saves are +2 Fortitude
//!   (poor, `6 / 3 = 2`), +2 Reflex (poor, `6 / 3 = 2`), +5 Will (good,
//!   `6 / 2 + 2 = 5`) — confirmed by the same formulas already grounded at
//!   levels 1-5, not re-derived. Every one of these four values is a genuinely
//!   NEW value, up from +2/+1/+1/+4 at level 5.
//! - the school specialization choice recognition (Evocation chosen, Necromancy
//!   and Transmutation opposed) is not level-gated, so it still fires at level 6
//!   for the same fixture selections.
//! - the specialist bonus slot flat count STAYS at 3 at level 6: the raw Wizard
//!   spells-per-day table (verified independently against both primary sources)
//!   shows a level-6 wizard's 3rd-level spell column is still non-"—" but the
//!   4th-level column is still "—" (level 6 row: "4/3/3/2/—"; 4th-level wizard
//!   spells do not become available until wizard level 7, level 7 row:
//!   "4/4/3/2/1"), so a level-6 specialist still casts only 1st-, 2nd-, and
//!   3rd-level spells — the flat count stays exactly 3, an unchanged value, not a
//!   sign the formula stopped scaling.
//! - Intense Spells' bonus-damage magnitude (half wizard level, minimum 1)
//!   genuinely RISES to 3 at level 6: `max(6 / 2, 1) = 3`, up from 2 at level 5 —
//!   the same pre-existing formula, not re-derived.
//! - Force Missile's uses-per-day pool (3 + Intelligence modifier) is
//!   level-independent and unchanged at level 6.
//! - Scribe Scroll is granted once, at 1st level only (unchanged): the record
//!   still recognizes the grant identity at level 6, its body text still
//!   hardcoding "1st level" as the level the feat was actually granted.
//! - the Wizard class table's level-6 "Special" column is genuinely BLANK
//!   (verified independently against both primary sources, checked rather than
//!   assumed away) — UNLIKE the level-5 "Bonus feat" entry, Wizard gains no new
//!   class feature at 6th level, mirroring exactly how the blank level-2/3/4
//!   "Special" columns were left as pure arithmetic widenings — so this slice
//!   grounds no new pillar for level 6 either, only the existing pillars are
//!   widened (one of them, Intense Spells, to a genuinely new value).
//!
//! It deliberately does not touch the school-power execution machinery, the
//! opposed-school two-slot preparation cost, the prepared spellbook/spells-per-day
//! posture burden, or the 5th-level bonus feat's own selection/execution (all stay
//! named-but-unproven, unchanged from levels 1-5), and it does not ground Wizard
//! level 7+. It also preserves the accepted Wizard level-1/level-2/level-3/level-4/
//! level-5 truth (unchanged), the Fighter negative control, and the multiclass
//! negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const WIZARD_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level5_sd13_deterministic_input.txt");

const WIZARD_LEVEL6_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level6_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 6 -----

#[test]
fn wizard_level6_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 3,
        "Wizard level 6 1/2-BAB progression (6 / 2) must equal 3: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 6 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn wizard_level6_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Wizard level 6 poor Fortitude (6/3) must equal 2");

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(reflex.value, 2, "Wizard level 6 poor Reflex (6/3) must equal 2");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 5, "Wizard level 6 good Will (6/2+2) must equal 5");
}

// ----- School specialization choice recognition still fires at level 6 -----

#[test]
fn wizard_level6_still_recognizes_the_specialization_choice() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        choice.value, 0,
        "specialization choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Evocation"),
        "specialization choice recognition must still name Evocation at level 6: {}",
        choice.detail
    );
}

// ----- The specialist bonus slot flat count stays 3 at level 6 -----

#[test]
fn wizard_level6_specialist_bonus_slot_stays_three() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 3,
        "the specialist bonus slot must stay 3 at level 6 (4th-level wizard spells do not \
         unlock until level 7, verified against the raw spells-per-day table, so a level-6 \
         specialist still casts only 1st-, 2nd-, and 3rd-level spells): {}",
        slot.detail
    );
}

// ----- Intense Spells bonus damage rises to 3 at level 6 -----

#[test]
fn wizard_level6_intense_spells_bonus_damage_rises_to_three() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 3,
        "Intense Spells bonus damage at level 6 (max(6/2, 1)) must rise to 3, up from 2 at \
         level 5: {}",
        bonus_damage.detail
    );
}

// ----- Force Missile uses/day is level-independent, unchanged at level 6 -----

#[test]
fn wizard_level6_force_missile_uses_per_day_is_unchanged() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    // Intelligence 17 + 2 Human racial (CG-03 fix) -> modifier +4, so 3 + 4 = 7, same as
    // at levels 1-5.
    assert_eq!(
        uses.value, 7,
        "Force Missile uses/day must stay 3 + Int modifier at level 6: {}",
        uses.detail
    );
}

// ----- Scribe Scroll grant is still recognized at level 6 -----

#[test]
fn wizard_level6_still_recognizes_the_scribe_scroll_grant() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let scribe_scroll = explanation(&computation, "class_chassis.wizard.scribe_scroll");
    assert_eq!(
        scribe_scroll.value, 0,
        "Scribe Scroll grant recognition must carry no fabricated mechanical value"
    );
    assert!(
        scribe_scroll.detail.contains("1st level") || scribe_scroll.detail.contains("level 1"),
        "Scribe Scroll detail must still name 1st level as when the feat was granted, not \
         re-derive a level-6 grant event: {}",
        scribe_scroll.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 6 -----

#[test]
fn wizard_level6_still_claim_blocks_school_powers_and_prepared_spellbook_burdens() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-6 Wizard must still claim-block on the school-power / opposed-school-cost \
         burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-6 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- The chassis recognition record is still present at level 6 -----

#[test]
fn wizard_level6_still_recognizes_the_spell_bearing_baseline() {
    let input = load(WIZARD_LEVEL6_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.wizard"),
        "level-6 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- The accepted Wizard level-5 truth is unaffected -----

#[test]
fn wizard_level5_truth_is_unchanged_by_this_widening() {
    let input = load(WIZARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 2, "Wizard level 5 base attack bonus must stay 2");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 4, "Wizard level 5 good Will save must stay 4");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 3,
        "Wizard level 5 specialist bonus slot must stay 3, unaffected by the level-6 widening"
    );

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 2,
        "Wizard level 5 Intense Spells bonus damage must stay 2, unaffected by the level-6 \
         widening"
    );
}

// ----- Negative control (later widened): level 7 was subsequently promoted -----

#[test]
fn wizard_level_7_was_later_widened_into_the_supported_tranche() {
    let level_7 = WIZARD_LEVEL6_FIXTURE.replace("class:wizard:6", "class:wizard:7");
    let input = load(&level_7);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.wizard.base_attack_bonus"),
        "level-7 Wizard was later widened into the supported tranche by \
         tests/sd13_wizard_level7_progression.rs — the negative-control coverage for level 8 now \
         lives there: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level6_recognition() {
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
fn multiclass_wizard_level6_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL6_FIXTURE.replace(
        "class_level=class:wizard:6",
        "class_level=class:wizard:6\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-6 widening -----

#[test]
fn matrix_wizard_row_names_level_6_widening() {
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
            .contains("sd13_wizard_level6_progression"),
        "wizard row must cite the live SD13-E5 level-6 proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 6") || note.contains("level-6"),
        "wizard partial note must name the level-6 widening: {note}"
    );
}
