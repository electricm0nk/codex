//! SD13-E5 Wizard level-2 progression grounding proof.
//!
//! Widens the accepted Wizard level-1 prepared-spell-burden baseline
//! (`tests/sd13_wizard_level1_prepared_spell_baseline.rs`,
//! `tests/sd13_wizard_base_attack_and_saves.rs`,
//! `tests/sd13_wizard_evocation_school_powers.rs`) to Wizard level 2, mirroring the
//! Fighter `supported_fighter_level` / Paladin `supported_paladin_level` / Rogue
//! `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
//! `supported_monk_level` / Cleric `supported_cleric_level` / Bard
//! `supported_bard_level` / Druid `supported_druid_level` / Sorcerer
//! `supported_sorcerer_level` level-range-gate idiom (the level-1-only gate
//! `is_single_class_wizard_level1` is generalized to `supported_wizard_level`, an
//! `Option<u8>` helper gated by `MAX_SUPPORTED_WIZARD_LEVEL = 2`). Both PF1 CRB
//! primary sources (d20pfsrd and legacy.aonprd.com Wizard class table) were read
//! directly before writing any code or test:
//!
//! - level 2 base attack bonus is +1 (`2 / 2 = 1`, the Wizard's own 1/2-BAB
//!   progression, the SAME shape as Sorcerer, UNLIKE the 3/4 BAB shared by
//!   Rogue/Monk/Druid/Cleric/Bard) and base saves are +0 Fortitude (poor, `2 / 3 = 0`),
//!   +0 Reflex (poor, `2 / 3 = 0`), +3 Will (good, `2 / 2 + 2 = 3`) — confirmed by the
//!   same formulas already grounded at level 1, not re-derived.
//! - the school specialization choice recognition (Evocation chosen, Necromancy and
//!   Transmutation opposed) is not level-gated (a wizard's chosen school does not
//!   change by level), so it still fires at level 2 for the same fixture selections.
//! - the specialist bonus slot flat count stays exactly 1 at level 2: verified
//!   independently against both primary sources that a level-2 wizard still only
//!   casts 1st-level wizard spells (2nd-level wizard spells require caster level 3,
//!   per the raw spells-per-day table rows — level 2 shows "4/2/—/—" and level 3
//!   shows "4/2/1/—"), so the specialist's "one additional spell slot of each spell
//!   level she can cast" is still exactly one 1st-level slot, unchanged from level 1.
//! - Intense Spells' bonus-damage magnitude (half wizard level, minimum 1) stays 1 at
//!   level 2 (`max(2 / 2, 1) = 1`), reached naturally via the formula rather than via
//!   the level-1 floor, mirroring the Cleric Touch of Good precedent.
//! - Force Missile's uses-per-day pool (3 + Intelligence modifier) is level-independent
//!   and unchanged at level 2.
//! - Scribe Scroll is granted once, at 1st level only (confirmed against both primary
//!   sources: it appears exactly once, in the level-1 "Special" column, never
//!   re-granted at 2nd level). Since the wizard keeps the feat once granted, this seam
//!   still recognizes the grant identity at level 2 (mirroring the Sorcerer Eschew
//!   Materials idiom: the record's header cites the character's current level, but its
//!   body text hardcodes "at 1st level" as the level the feat was actually granted,
//!   never re-deriving a level-2 grant event).
//! - the Wizard class table's level-2 "Special" column is blank: verified
//!   independently against both primary sources (d20pfsrd and legacy.aonprd.com) that
//!   Wizard gains no new class feature at 2nd level (unlike Rogue/Monk/Druid's
//!   Evasion/Woodland Stride, but like Cleric/Sorcerer), so no new pillar burden is
//!   added this slice — only the existing pillars are widened.
//!
//! It deliberately does not touch the school-power execution machinery, the
//! opposed-school two-slot preparation cost, or the prepared spellbook/spells-per-day
//! posture burden (all stay named-but-unproven, unchanged from level 1), and it does
//! not ground Wizard level 3+. It also preserves the accepted Wizard level-1 truth
//! (unchanged), the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const WIZARD_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt");

const WIZARD_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level2_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 2 -----

#[test]
fn wizard_level2_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 1,
        "Wizard level 2 1/2-BAB progression (2 / 2) must equal 1: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 2 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn wizard_level2_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(fortitude.value, 0, "Wizard level 2 poor Fortitude (2/3) must equal 0");

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(reflex.value, 0, "Wizard level 2 poor Reflex (2/3) must equal 0");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 3, "Wizard level 2 good Will (2/2+2) must equal 3");
}

// ----- School specialization choice recognition still fires at level 2 -----

#[test]
fn wizard_level2_still_recognizes_the_specialization_choice() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        choice.value, 0,
        "specialization choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Evocation"),
        "specialization choice recognition must still name Evocation at level 2: {}",
        choice.detail
    );
}

// ----- The specialist bonus slot flat count stays 1 at level 2 -----

#[test]
fn wizard_level2_specialist_bonus_slot_stays_one() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 1,
        "the specialist bonus slot must stay exactly 1 at level 2 (a level-2 wizard still only \
         casts 1st-level spells): {}",
        slot.detail
    );
}

// ----- Intense Spells bonus damage stays 1 at level 2 (reached naturally) -----

#[test]
fn wizard_level2_intense_spells_bonus_damage_stays_one() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 1,
        "Intense Spells bonus damage at level 2 (max(2/2, 1)) must equal 1: {}",
        bonus_damage.detail
    );
}

// ----- Force Missile uses/day is level-independent, unchanged at level 2 -----

#[test]
fn wizard_level2_force_missile_uses_per_day_is_unchanged() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    // Intelligence 17 + 2 Human racial (CG-03 fix) -> modifier +4, so 3 + 4 = 7, same as
    // at level 1.
    assert_eq!(
        uses.value, 7,
        "Force Missile uses/day must stay 3 + Int modifier at level 2: {}",
        uses.detail
    );
}

// ----- Scribe Scroll grant is still recognized at level 2 -----

#[test]
fn wizard_level2_still_recognizes_the_scribe_scroll_grant() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let scribe_scroll = explanation(&computation, "class_chassis.wizard.scribe_scroll");
    assert_eq!(
        scribe_scroll.value, 0,
        "Scribe Scroll grant recognition must carry no fabricated mechanical value"
    );
    assert!(
        scribe_scroll.detail.contains("1st level") || scribe_scroll.detail.contains("level 1"),
        "Scribe Scroll detail must still name 1st level as when the feat was granted, not \
         re-derive a level-2 grant event: {}",
        scribe_scroll.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 2 -----

#[test]
fn wizard_level2_still_claim_blocks_school_powers_and_prepared_spellbook_burdens() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-2 Wizard must still claim-block on the school-power / opposed-school-cost \
         burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-2 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- The chassis recognition record is still present at level 2 -----

#[test]
fn wizard_level2_still_recognizes_the_spell_bearing_baseline() {
    let input = load(WIZARD_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.wizard"),
        "level-2 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- The accepted Wizard level-1 truth is unaffected -----

#[test]
fn wizard_level1_truth_is_unchanged_by_this_widening() {
    let input = load(WIZARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 0, "Wizard level 1 base attack bonus must stay 0");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 2, "Wizard level 1 good Will save must stay 2");

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(bonus_damage.value, 1, "Wizard level 1 Intense Spells bonus damage must stay 1");
}

// ----- Wizard level 3 was later widened into the supported tranche -----

#[test]
fn wizard_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_wizard_level3_progression.rs) widened the level-range gate to
    // level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/
    // Sorcerer level-range gate idiom) and extended every formula, changing the
    // specialist bonus slot count for real (1 -> 2); this negative control is
    // superseded, not violated — pin the new truth here too so this file stays
    // internally consistent.
    let level_3 = WIZARD_LEVEL2_FIXTURE.replace("class:wizard:2", "class:wizard:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id == "class_chassis.spell_baseline.wizard"),
        "level-3 Wizard is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
}

// ----- Wizard level 4 was later widened into the supported tranche -----

#[test]
fn wizard_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_wizard_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/
    // Sorcerer level-range gate idiom) and extended every formula (Intense Spells'
    // bonus-damage magnitude changes for real, 1 -> 2); this negative control is
    // superseded, not violated — pin the new truth here too so this file stays
    // internally consistent.
    let level_4 = WIZARD_LEVEL2_FIXTURE.replace("class:wizard:2", "class:wizard:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id == "class_chassis.spell_baseline.wizard"),
        "level-4 Wizard is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
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
    let level_5 = WIZARD_LEVEL2_FIXTURE.replace("class:wizard:2", "class:wizard:5");
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
fn fighter_does_not_gain_wizard_level2_recognition() {
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
fn multiclass_wizard_level2_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL2_FIXTURE.replace(
        "class_level=class:wizard:2",
        "class_level=class:wizard:2\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-2 widening -----

#[test]
fn matrix_wizard_row_names_level_2_widening() {
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
            .contains("sd13_wizard_level2_progression"),
        "wizard row must cite the live SD13-E5 level-2 proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 2") || note.contains("level-2"),
        "wizard partial note must name the level-2 widening: {note}"
    );
}
