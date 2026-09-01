//! SD13-E5 Wizard level-4 progression grounding proof.
//!
//! Widens the accepted Wizard level-1/level-2/level-3 prepared-spell-burden baseline
//! (`tests/sd13_wizard_level1_prepared_spell_baseline.rs`,
//! `tests/sd13_wizard_base_attack_and_saves.rs`,
//! `tests/sd13_wizard_evocation_school_powers.rs`,
//! `tests/sd13_wizard_level2_progression.rs`,
//! `tests/sd13_wizard_level3_progression.rs`) to Wizard level 4, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Ranger
//! level-range-gate idiom (`supported_wizard_level` is generalized from `1..=3` to
//! `1..=4` via `MAX_SUPPORTED_WIZARD_LEVEL = 4`). Both PF1 CRB primary sources
//! (d20pfsrd and legacy.aonprd.com Wizard class table) were read directly before
//! writing any code or test:
//!
//! - level 4 base attack bonus is +2 (`4 / 2 = 2`, the Wizard's own 1/2-BAB
//!   progression, the SAME shape as Sorcerer) and base saves are +1 Fortitude
//!   (poor, `4 / 3 = 1`), +1 Reflex (poor, `4 / 3 = 1`), +4 Will (good,
//!   `4 / 2 + 2 = 4`) — confirmed by the same formulas already grounded at levels
//!   1-3, not re-derived.
//! - the school specialization choice recognition (Evocation chosen, Necromancy and
//!   Transmutation opposed) is not level-gated, so it still fires at level 4 for the
//!   same fixture selections.
//! - the specialist bonus slot flat count stays 2 at level 4 — this was the exact
//!   question this cycle was briefed to verify rather than assume: the raw Wizard
//!   spells-per-day table rows (verified independently against both primary sources)
//!   show a level-4 wizard's 3rd-level spell column is still "—" (level 4 row:
//!   "4/3/2/—/—"); 3rd-level wizard spells do not become available until wizard level
//!   5 (level 5 row: "4/3/2/1/—" — the first non-"—" 3rd-level column). So a level-4
//!   specialist still only casts 1st- and 2nd-level spells, and the flat count stays
//!   exactly 2 (one 1st-level bonus slot plus one 2nd-level bonus slot), unchanged
//!   from level 3 — the pre-existing `level >=
//!   WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` gate already produces this
//!   value correctly with no formula change needed.
//! - Intense Spells' bonus-damage magnitude (half wizard level, minimum 1) genuinely
//!   CHANGES at level 4: `max(4 / 2, 1) = 2`, up from 1 at levels 1-3 — the first
//!   value change this pillar's formula produces since it was grounded, reached
//!   naturally via the pre-existing formula, not re-derived.
//! - Force Missile's uses-per-day pool (3 + Intelligence modifier) is
//!   level-independent and unchanged at level 4.
//! - Scribe Scroll is granted once, at 1st level only (unchanged): the record still
//!   recognizes the grant identity at level 4, its body text still hardcoding "1st
//!   level" as the level the feat was actually granted.
//! - the Wizard class table's level-4 "Special" column is blank (verified
//!   independently against both primary sources: no new Wizard class feature is
//!   gained at 4th level — the next Wizard class feature, a bonus feat, is granted at
//!   5th level, not 4th, verified rather than assumed), so this slice adds no new
//!   pillar record for level 4 — only the existing pillars are widened (one of them,
//!   Intense Spells' bonus-damage magnitude, widened to a genuinely new value).
//!
//! It deliberately does not touch the school-power execution machinery, the
//! opposed-school two-slot preparation cost, or the prepared spellbook/
//! spells-per-day posture burden (all stay named-but-unproven, unchanged from
//! levels 1-3), and it does not ground Wizard level 5+. It also preserves the
//! accepted Wizard level-1/level-2/level-3 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const WIZARD_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level3_sd13_deterministic_input.txt");

const WIZARD_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level4_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 4 -----

#[test]
fn wizard_level4_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 2,
        "Wizard level 4 1/2-BAB progression (4 / 2) must equal 2: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 4 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn wizard_level4_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(fortitude.value, 1, "Wizard level 4 poor Fortitude (4/3) must equal 1");

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(reflex.value, 1, "Wizard level 4 poor Reflex (4/3) must equal 1");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 4, "Wizard level 4 good Will (4/2+2) must equal 4");
}

// ----- School specialization choice recognition still fires at level 4 -----

#[test]
fn wizard_level4_still_recognizes_the_specialization_choice() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        choice.value, 0,
        "specialization choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Evocation"),
        "specialization choice recognition must still name Evocation at level 4: {}",
        choice.detail
    );
}

// ----- The specialist bonus slot flat count stays 2 at level 4 -----

#[test]
fn wizard_level4_specialist_bonus_slot_stays_two() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 2,
        "the specialist bonus slot must stay 2 at level 4 (3rd-level wizard spells do not \
         unlock until level 5, verified against the raw spells-per-day table, so a level-4 \
         specialist still only casts 1st- and 2nd-level spells): {}",
        slot.detail
    );
}

// ----- Intense Spells bonus damage becomes 2 at level 4 -----

#[test]
fn wizard_level4_intense_spells_bonus_damage_becomes_two() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 2,
        "Intense Spells bonus damage at level 4 (max(4/2, 1)) must equal 2, up from 1 at \
         levels 1-3: {}",
        bonus_damage.detail
    );
}

// ----- Force Missile uses/day is level-independent, unchanged at level 4 -----

#[test]
fn wizard_level4_force_missile_uses_per_day_is_unchanged() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    // Intelligence 17 -> modifier +3, so 3 + 3 = 6, same as at levels 1-3.
    assert_eq!(
        uses.value, 7,
        "Force Missile uses/day must stay 3 + Int modifier at level 4: {}",
        uses.detail
    );
}

// ----- Scribe Scroll grant is still recognized at level 4 -----

#[test]
fn wizard_level4_still_recognizes_the_scribe_scroll_grant() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let scribe_scroll = explanation(&computation, "class_chassis.wizard.scribe_scroll");
    assert_eq!(
        scribe_scroll.value, 0,
        "Scribe Scroll grant recognition must carry no fabricated mechanical value"
    );
    assert!(
        scribe_scroll.detail.contains("1st level") || scribe_scroll.detail.contains("level 1"),
        "Scribe Scroll detail must still name 1st level as when the feat was granted, not \
         re-derive a level-4 grant event: {}",
        scribe_scroll.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 4 -----

#[test]
fn wizard_level4_still_claim_blocks_school_powers_and_prepared_spellbook_burdens() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-4 Wizard must still claim-block on the school-power / opposed-school-cost \
         burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-4 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- The chassis recognition record is still present at level 4 -----

#[test]
fn wizard_level4_still_recognizes_the_spell_bearing_baseline() {
    let input = load(WIZARD_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.wizard"),
        "level-4 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- The accepted Wizard level-3 truth is unaffected -----

#[test]
fn wizard_level3_truth_is_unchanged_by_this_widening() {
    let input = load(WIZARD_LEVEL3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 1, "Wizard level 3 base attack bonus must stay 1");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 3, "Wizard level 3 good Will save must stay 3");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 2,
        "Wizard level 3 specialist bonus slot must stay 2, unaffected by the level-4 widening"
    );

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 1,
        "Wizard level 3 Intense Spells bonus damage must stay 1, unaffected by the level-4 \
         widening"
    );
}

// ----- Negative control: level 5 was later widened into the supported tranche -----

#[test]
fn wizard_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was out of scope and stayed
    // unrecognized. A later SD13-E5 slice (tests/sd13_wizard_level5_progression.rs)
    // widened the level-range gate to level 5; this negative control is superseded,
    // not violated — pin the new truth here too so this file stays internally
    // consistent.
    let level_5 = WIZARD_LEVEL4_FIXTURE.replace("class:wizard:4", "class:wizard:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id == "class_chassis.spell_baseline.wizard"),
        "level-5 Wizard is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level4_recognition() {
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
fn multiclass_wizard_level4_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL4_FIXTURE.replace(
        "class_level=class:wizard:4",
        "class_level=class:wizard:4\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-4 widening -----

#[test]
fn matrix_wizard_row_names_level_4_widening() {
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
            .contains("sd13_wizard_level4_progression"),
        "wizard row must cite the live SD13-E5 level-4 proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 4") || note.contains("level-4"),
        "wizard partial note must name the level-4 widening: {note}"
    );
}
