//! SD13-E5 Wizard level-8 progression grounding proof.
//!
//! Widens the accepted Wizard level-1..level-7 prepared-spell-burden baseline
//! (`tests/sd13_wizard_level1_prepared_spell_baseline.rs`,
//! `tests/sd13_wizard_base_attack_and_saves.rs`,
//! `tests/sd13_wizard_level2_progression.rs`,
//! `tests/sd13_wizard_level3_progression.rs`,
//! `tests/sd13_wizard_level4_progression.rs`,
//! `tests/sd13_wizard_level5_progression.rs`,
//! `tests/sd13_wizard_level6_progression.rs`,
//! `tests/sd13_wizard_level7_progression.rs`) to Wizard level 8, mirroring the
//! Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Ranger
//! level-range-gate idiom (`supported_wizard_level` is generalized from `1..=7`
//! to `1..=8` via `MAX_SUPPORTED_WIZARD_LEVEL = 8`). Both PF1 CRB primary
//! sources (d20pfsrd and legacy.aonprd.com Wizard class table, and the Wizard
//! spells-per-day table) were read directly before writing any code or test:
//!
//! - level 8 base attack bonus is +4 (`8 / 2 = 4`, the Wizard's own 1/2-BAB
//!   progression, genuinely risen from +3 at level 7) and base saves are +2
//!   Fortitude (poor, `8 / 3 = 2`, numerically unchanged from level 7, an
//!   integer-division coincidence), +2 Reflex (poor, `8 / 3 = 2`, likewise
//!   unchanged), and +6 Will (good, `8 / 2 + 2 = 6`, genuinely risen from +5)
//!   — confirmed by the same formulas already grounded at levels 1-7, not
//!   re-derived.
//! - the school specialization choice recognition (Evocation chosen,
//!   Necromancy and Transmutation opposed) is not level-gated, so it still
//!   fires at level 8 for the same fixture selections.
//! - the specialist bonus slot flat count STAYS at 4 at level 8: the raw
//!   Wizard spells-per-day table (verified independently against both primary
//!   sources) shows the level-8 row is "4/4/3/3/2" with the 5th-level spell
//!   column still "—" — 5th-level spells first appear at level 9 — so a
//!   level-8 specialist wizard still gains one Evocation-only bonus slot of
//!   each castable spell level, 1st through 4th, a flat count of 4, unchanged
//!   from level 7 (a threshold stasis, checked rather than assumed: the next
//!   rise lands at level 9 when 5th-level spells arrive, not at level 8).
//! - Intense Spells' bonus-damage magnitude (half wizard level, minimum 1)
//!   GENUINELY RISES to 4 at level 8: `max(8 / 2, 1) = 4`, up from 3 at
//!   levels 6-7, via the same pre-existing formula, not re-derived.
//! - Force Missile's uses-per-day pool (3 + Intelligence modifier) is
//!   level-independent and unchanged at level 8.
//! - Scribe Scroll is granted once, at 1st level only (unchanged): the record
//!   still recognizes the grant identity at level 8, its body text still
//!   hardcoding "1st level" as the level the feat was actually granted.
//! - the Wizard class table's level-8 "Special" column is genuinely BLANK
//!   (verified independently against both primary sources, checked rather
//!   than assumed away) — the Wizard's bonus feats land at levels 5, 10, 15,
//!   and 20, so no new Wizard class feature is gained at 8th level and this
//!   slice grounds no new pillar record for level 8.
//!
//! It deliberately does not touch the school-power execution machinery, the
//! opposed-school two-slot preparation cost, the prepared spellbook/spells-per-day
//! posture burden, or the 5th-level bonus feat's own selection/execution (all stay
//! named-but-unproven, unchanged from levels 1-7), and it does not ground Wizard
//! level 9+. It also preserves the accepted Wizard level-1..level-7 truth
//! (unchanged), the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const WIZARD_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level7_sd13_deterministic_input.txt");

const WIZARD_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level8_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 8 -----

#[test]
fn wizard_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 4,
        "Wizard level 8 1/2-BAB progression (8 / 2) must equal 4, genuinely risen from 3 at \
         level 7: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 (good Will only, poor Fortitude, poor Reflex) -----

#[test]
fn wizard_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Wizard level 8 poor Fortitude (8/3) must equal 2");

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(reflex.value, 2, "Wizard level 8 poor Reflex (8/3) must equal 2");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 6,
        "Wizard level 8 good Will (8/2+2) must equal 6, genuinely risen from 5 at level 7"
    );
}

// ----- School specialization choice recognition still fires at level 8 -----

#[test]
fn wizard_level8_still_recognizes_the_school_specialization_choice() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        choice.value, 0,
        "school specialization choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Evocation") || choice.detail.contains("evocation"),
        "school specialization recognition must still name Evocation at level 8: {}",
        choice.detail
    );
}

// ----- Specialist bonus slot count stays 4 at level 8 (threshold stasis) -----

#[test]
fn wizard_level8_specialist_bonus_slot_count_stays_four() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 4,
        "Wizard level 8 specialist bonus slot count must stay 4 (one per castable spell level \
         1st-4th; 5th-level spells first arrive at level 9 per both primary sources): {}",
        slot.detail
    );
}

// ----- Intense Spells bonus damage genuinely rises to 4 at level 8 -----

#[test]
fn wizard_level8_intense_spells_bonus_damage_rises_to_four() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 4,
        "Wizard level 8 Intense Spells bonus damage (max(8/2, 1)) must equal 4, genuinely \
         risen from 3 at level 7: {}",
        bonus_damage.detail
    );
}

// ----- Force Missile uses/day is level-independent and unchanged at level 8 -----

#[test]
fn wizard_level8_force_missile_pool_is_unchanged() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses = explanation(&computation, "class_chassis.wizard.force_missile_uses_per_day");
    // CG-03 fix: Intelligence modifier is now +4 (base 17 + 2 Human racial), not +3.
    assert_eq!(
        uses.value, 7,
        "Wizard level 8 Force Missile pool (3 + Intelligence modifier +4) must stay 7: {}",
        uses.detail
    );
}

// ----- Scribe Scroll grant recognition still fires at level 8 -----

#[test]
fn wizard_level8_still_recognizes_the_scribe_scroll_grant() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let scribe_scroll = explanation(&computation, "class_chassis.wizard.scribe_scroll");
    assert_eq!(
        scribe_scroll.value, 0,
        "Scribe Scroll grant recognition must carry no fabricated mechanical value"
    );
    assert!(
        scribe_scroll.detail.contains("1st level"),
        "Scribe Scroll recognition must still name 1st level as the grant level: {}",
        scribe_scroll.detail
    );
}

// ----- The two existing burden diagnostics still fire at level 8 -----

#[test]
fn wizard_level8_still_claim_blocks_school_power_and_prepared_spellbook_burdens() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-8 Wizard must still claim-block on the school-power / opposed-school burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_spell.wizard.prepared_spellbook.unsupported" && d.claim_blocking
        ),
        "level-8 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- The chassis recognition record is still present at level 8 -----

#[test]
fn wizard_level8_still_recognizes_the_spell_bearing_baseline() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.spell_baseline.wizard"),
        "level-8 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the level-7 fixture is unaffected by this widening -----

#[test]
fn wizard_level7_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 3, "Wizard level 7 base attack bonus must stay 3");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 5, "Wizard level 7 good Will save must stay 5");

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 3,
        "Wizard level 7 Intense Spells bonus damage must stay 3"
    );
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn wizard_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = WIZARD_LEVEL8_FIXTURE.replace("class:wizard:8", "class:wizard:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")),
        "level-9 Wizard is now recognized by the later level-9 widening slice \
         (tests/sd13_wizard_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level8_recognition() {
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
fn multiclass_wizard_level8_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL8_FIXTURE.replace(
        "class_level=class:wizard:8",
        "class_level=class:wizard:8\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_wizard_row_names_level_8_widening() {
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
            .contains("sd13_wizard_level8_progression"),
        "wizard row must cite the live SD13-E5 level-8 proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 8") || note.contains("level-8"),
        "wizard partial note must name the level-8 widening: {note}"
    );
}
