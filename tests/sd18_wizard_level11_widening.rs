//! SD18 Wizard level-11 widening grounding proof.
//!
//! Widens the accepted Wizard level-1..level-10 prepared arcane spell-bearing
//! baseline (`tests/sd13_wizard_level10_progression.rs`, the SD13 tranche's
//! declared ceiling) to Wizard level 11 — a fresh, fully-untouched §3.2
//! class-row widening (Elf and Half-Elf's remaining §3.1 race-row families
//! were re-checked directly against their raw LST corpus this cycle and both
//! confirmed genuine new-subsystem risk: Elf's weapon familiarity needs a
//! CHANGEPROF proficiency-category-reclassification engine and its bonus
//! languages need a CHOOSE-list selection mechanism; Half-Elf's Adaptability
//! needs a bonus-feat-plus-skill-selection mechanism and its Multitalented
//! needs a dual-favored-class bookkeeping mechanism — so §3.1 is genuinely
//! exhausted under the current seam shape and this cycle pivots to a
//! first-touch §3.2 class row instead, mirroring the level-11 widening
//! pattern already landed for Barbarian/Bard/Cleric/Druid/Fighter/Monk/
//! Paladin). Both PF1 CRB primary sources (d20pfsrd and the Archives of
//! Nethys aonprd.com mirror) were read directly before writing any code or
//! test:
//!
//! - level 11 base attack bonus is +5 (`11 / 2 = 5`) and base saves are +3
//!   Fortitude (poor, `11 / 3 = 3`), +3 Reflex (poor, `11 / 3 = 3`), and +7
//!   Will (good, `11 / 2 + 2 = 7`) — all four values numerically IDENTICAL to
//!   level 10, integer-division coincidences, not a sign any formula stopped
//!   scaling, confirmed by the same formulas already grounded at levels
//!   1-10, not re-derived.
//! - the PF1 Core Rulebook Wizard class table's level-11 "Special" column is
//!   genuinely blank on both primary sources (the Wizard's bonus feats land
//!   only at levels 5/10/15/20), so no new pillar is added at level 11.
//! - the raw Wizard spells-per-day table's level-11 row is "4/4/4/4/3/2/1" —
//!   the first non-"-" 6th-level column, up from the level-10 row
//!   "4/4/4/3/3/2" whose 6th-level column is still "-" (verified
//!   independently against both primary sources' raw table rows, checked
//!   rather than assumed) — so a level-11 specialist wizard casts 6th-level
//!   spells for the first time and the specialist bonus-slot flat count
//!   GENUINELY RISES to 6 (one bonus slot of each spell level 1st through
//!   6th), up from 5 at levels 9-10.
//! - Intense Spells' bonus-damage magnitude stays 5 (`max(11 / 2, 1) = 5`,
//!   another integer-division coincidence with level 10); Force Missile's
//!   uses-per-day pool stays the level-independent 3 + Intelligence modifier
//!   (6); Scribe Scroll and the school specialization choice recognitions
//!   are not level-gated and still fire.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven 5th/10th-
//! level "Bonus feat" selection/execution, or the prepared spellbook /
//! spells-prepared / spell-slot posture burden (all stay named-but-unproven,
//! unchanged from levels 1-10), and it does not ground Wizard level 12+. It
//! also preserves the accepted Wizard level-1..level-10 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level10_sd13_deterministic_input.txt");

const WIZARD_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level11_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and saves at level 11 stay numerically unchanged -----

#[test]
fn wizard_level11_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(WIZARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 5,
        "Wizard level 11 1/2-BAB progression (11 / 2) must stay 5, an integer-division \
         coincidence with level 10: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(fortitude.value, 3, "Wizard level 11 poor Fortitude (11/3) must stay 3");

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(reflex.value, 3, "Wizard level 11 poor Reflex (11/3) must stay 3");

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 7, "Wizard level 11 good Will (11/2+2) must stay 7");
}

// ----- The specialist bonus slot count genuinely rises to 6 -----

#[test]
fn wizard_level11_specialist_bonus_slot_genuinely_rises() {
    let input = load(WIZARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 6,
        "Wizard level 11 specialist bonus slot count must genuinely rise to 6 — 6th-level \
         wizard spells first appear at 11th caster level per both primary sources' raw \
         spells-per-day table rows: {}",
        slot.detail
    );
}

// ----- Intense Spells, Force Missile, Scribe Scroll, and the specialization choice carry over -----

#[test]
fn wizard_level11_school_powers_and_grants_carry_over() {
    let input = load(WIZARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 5,
        "Intense Spells' bonus-damage magnitude (max(11/2, 1)) must stay 5 at level 11, an \
         integer-division coincidence with level 10: {}",
        intense.detail
    );

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 11"
    );

    let scribe_scroll = explanation(&computation, "class_chassis.wizard.scribe_scroll");
    assert_eq!(scribe_scroll.value, 0, "Scribe Scroll must still carry no mechanical value");

    let specialization = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        specialization.value, 0,
        "the school specialization choice seam must still carry no mechanical value"
    );
}

// ----- The school-power execution and prepared-spellbook burdens still claim-block at level 11 -----

#[test]
fn wizard_level11_still_claim_blocks_school_power_and_prepared_spellbook_burdens() {
    let input = load(WIZARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-11 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );

    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported" && d.claim_blocking),
        "level-11 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-10 fixture is unaffected by this widening -----

#[test]
fn wizard_level10_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 5, "Wizard level 10 specialist bonus slot count must stay 5");

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 5, "Wizard level 10 base attack bonus must stay 5");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 5, "Wizard level 10 Intense Spells bonus damage must stay 5");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (widened from level 12 all the way to level 20 by the SD18
// wizard-level12-widening through wizard-level20-widening cycles, which
// genuinely promote levels 12-20 — see tests/sd18_wizard_level12_widening.rs
// through tests/sd18_wizard_level20_widening.rs — mirroring the exact same
// boundary move the Cleric level-20 widening cycle made for its own sibling
// widening tests. PF1 has no 21st character level; this is a pure
// implementation-gate check only.)

#[test]
fn wizard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = WIZARD_LEVEL11_FIXTURE.replace("class:wizard:11", "class:wizard:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.wizard.")
                || e.id.starts_with("class_feature.wizard.")
                || e.id == "class_chassis.spell_baseline.wizard")
                // SD-34 decisions.md section 18 (`bfe90f020a`, 2026-08-29) widened the
                // anti-fabrication gate BY CONSTRUCTION for Wizard: class_feature_grant_
                // consumer now emits a real, citation-backed class_feature.wizard.
                // corpus_record.* id for any grant fact with a renderable corpus record,
                // at any Wizard level -- that commit widened the sd13_* acceptance tests
                // it named but never reached these later sd18_* widening siblings. Same
                // carve-out, same reasoning, applied here.
                && !e.id.starts_with("class_feature.wizard.corpus_record.")
                // AT-34-E3-001 cycle 6 (`49d72f5e03`, 2026-08-28) grounded Wizard Weapon
                // and Armor Proficiency unconditionally (real PF1 content, any level) --
                // pre-existing, already-tested, not promotion by this slice.
                && e.id != "class_feature.wizard.weapon_and_armor_proficiency"),
        "level-21 Wizard must not gain any bounded wizard explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level11_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id.starts_with("class_feature.wizard.")),
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
fn multiclass_wizard_level11_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL11_FIXTURE.replace(
        "class_level=class:wizard:11",
        "class_level=class:wizard:11\nclass_level=class:rogue:1",
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
                || e.id.starts_with("class_feature.wizard.")),
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

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_wizard_row_names_level_11_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level11_widening"),
        "wizard row must cite the live SD18 level-11 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "wizard partial note must name the level-11 widening: {note}"
    );
}
