//! SD13-E5 Wizard level-9 progression grounding proof.
//!
//! Widens the accepted Wizard level-1..level-8 prepared-spell-burden baseline
//! (most recently `tests/sd13_wizard_level8_progression.rs`) to Wizard level
//! 9, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=8` to `1..=9` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 9`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Wizard class table and spells-per-day table) were
//! read directly before writing any code or test:
//!
//! - level 9 base attack bonus is +4 (`9 / 2 = 4`, the Wizard's 1/2-BAB
//!   progression, numerically unchanged from level 8 — an integer-division
//!   coincidence) and base saves are +3 Fortitude (poor, `9 / 3 = 3`,
//!   genuinely risen from +2), +3 Reflex (poor, `9 / 3 = 3`, genuinely
//!   risen from +2), and +6 Will (good, `9 / 2 + 2 = 6`, numerically
//!   unchanged from level 8, a coincidence) — confirmed by the same
//!   formulas already grounded at levels 1-8, not re-derived.
//! - the school specialization choice recognition (Evocation chosen,
//!   Necromancy and Transmutation opposed) is not level-gated, so it still
//!   fires at level 9 for the same fixture selections.
//! - the specialist bonus slot flat count GENUINELY RISES to 5 at level 9:
//!   the raw Wizard spells-per-day table (verified independently against
//!   both primary sources) shows the level-9 row is "4/4/4/3/2/1" — the
//!   first row with a non-"—" 5th-level spell column — so a level-9
//!   specialist wizard now gains one Evocation-only bonus slot of each
//!   castable spell level, 1st through 5th, mirroring exactly the level-7
//!   fourth-slot widening idiom (a new
//!   `WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 9` threshold gating a
//!   new `WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_9 = 5` constant).
//! - Intense Spells' bonus-damage magnitude STAYS at 4 at level 9
//!   (`max(9 / 2, 1) = 4`, numerically unchanged from level 8, an
//!   integer-division coincidence — the next rise lands at level 10), via
//!   the same pre-existing formula, not re-derived.
//! - Force Missile's uses-per-day pool (3 + Intelligence modifier) is
//!   level-independent and unchanged at level 9; Scribe Scroll stays
//!   recognized as an already-held 1st-level grant.
//! - the Wizard class table's level-9 "Special" column is genuinely BLANK
//!   (verified independently against both primary sources, checked rather
//!   than assumed away — the Wizard's bonus feats land at levels 5, 10, 15,
//!   and 20), so no new Wizard class feature is gained at 9th level and
//!   this slice grounds no new pillar record beyond widening the specialist
//!   bonus slot pillar to a genuinely new value.
//!
//! It deliberately does not touch the school-power execution machinery, the
//! opposed-school two-slot preparation cost, the prepared spellbook/
//! spells-per-day posture burden, or the 5th-level bonus feat's own
//! selection/execution (all stay named-but-unproven, unchanged from levels
//! 1-8), and it does not ground Wizard level 10+. It also preserves the
//! accepted Wizard level-1..level-8 truth (unchanged), the Fighter negative
//! control, and the multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level8_sd13_deterministic_input.txt");

const WIZARD_LEVEL9_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level9_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus and saves at level 9 -----

#[test]
fn wizard_level9_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(WIZARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 4,
        "Wizard level 9 1/2-BAB progression (9 / 2) must equal 4 — unchanged from level 8, an \
         integer-division coincidence: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 3,
        "Wizard level 9 poor Fortitude (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 3,
        "Wizard level 9 poor Reflex (9/3) must equal 3, genuinely risen from 2 at level 8"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(will.value, 6, "Wizard level 9 good Will (9/2+2) must equal 6");
}

// ----- School specialization choice recognition still fires at level 9 -----

#[test]
fn wizard_level9_still_recognizes_the_school_specialization_choice() {
    let input = load(WIZARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        choice.value, 0,
        "school specialization choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Evocation") || choice.detail.contains("evocation"),
        "school specialization recognition must still name Evocation at level 9: {}",
        choice.detail
    );
}

// ----- Specialist bonus slot count genuinely rises to 5 at level 9 -----

#[test]
fn wizard_level9_specialist_bonus_slot_count_rises_to_five() {
    let input = load(WIZARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 5,
        "Wizard level 9 specialist bonus slot count must rise to 5 — 5th-level wizard spells \
         first appear at 9th per both primary sources' spells-per-day tables (the level-9 row \
         is \"4/4/4/3/2/1\"), so the one-slot-per-castable-spell-level rule now spans 1st \
         through 5th: {}",
        slot.detail
    );
}

// ----- Intense Spells stays 4 at level 9 (integer-division coincidence) -----

#[test]
fn wizard_level9_intense_spells_bonus_damage_stays_four() {
    let input = load(WIZARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus_damage = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        bonus_damage.value, 4,
        "Wizard level 9 Intense Spells bonus damage (max(9/2, 1)) must stay 4 — an \
         integer-division coincidence with level 8; the next rise lands at level 10: {}",
        bonus_damage.detail
    );
}

// ----- Force Missile pool and Scribe Scroll carry over at level 9 -----

#[test]
fn wizard_level9_force_missile_and_scribe_scroll_carry_over() {
    let input = load(WIZARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // CG-03 fix: Intelligence modifier is now +4 (base 17 + 2 Human racial), not +3.
    let uses = explanation(&computation, "class_chassis.wizard.force_missile_uses_per_day");
    assert_eq!(
        uses.value, 7,
        "Wizard level 9 Force Missile pool (3 + Intelligence modifier +4) must stay 7"
    );

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

// ----- The two existing burden diagnostics still fire at level 9 -----

#[test]
fn wizard_level9_still_claim_blocks_school_power_and_prepared_spellbook_burdens() {
    let input = load(WIZARD_LEVEL9_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-9 Wizard must still claim-block on the school-power / opposed-school burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(
            |d| d.id == "class_spell.wizard.prepared_spellbook.unsupported" && d.claim_blocking
        ),
        "level-9 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-8 fixture is unaffected by this widening -----

#[test]
fn wizard_level8_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 4, "Wizard level 8 specialist bonus slot count must stay 4");

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Wizard level 8 poor Fortitude must stay 2");
}

// ----- Level 10 was later widened into the supported tranche by a further slice -----

#[test]
fn wizard_level_10_was_later_widened_into_the_supported_tranche() {
    let level_10 = WIZARD_LEVEL9_FIXTURE.replace("class:wizard:9", "class:wizard:10");
    let input = load(&level_10);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")),
        "level-10 Wizard is now recognized by the later level-10 widening slice \
         (tests/sd13_wizard_level10_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level9_recognition() {
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
fn multiclass_wizard_level9_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL9_FIXTURE.replace(
        "class_level=class:wizard:9",
        "class_level=class:wizard:9\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-9 widening -----

#[test]
fn matrix_wizard_row_names_level_9_widening() {
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
            .contains("sd13_wizard_level9_progression"),
        "wizard row must cite the live SD13-E5 level-9 proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 9") || note.contains("level-9"),
        "wizard partial note must name the level-9 widening: {note}"
    );
}
