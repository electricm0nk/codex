//! SD18 Wizard level-16 widening grounding proof.
//!
//! Widens the accepted deterministic Human Wizard level-1..level-15 prepared
//! arcane spell-bearing chassis (`tests/sd18_wizard_level15_widening.rs`) to
//! Wizard level 16, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=15` to `1..=16` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 16`, exactly as the Barbarian and Fighter
//! cycles widened `MAX_SUPPORTED_BARBARIAN_LEVEL` and
//! `MAX_SUPPORTED_FIGHTER_LEVEL` from 15 to 16 — the loop's THIRD §3.2
//! level-16 landing, after Barbarian and Fighter, and the first
//! spell-bearing class to reach level 16 in the level-16 sweep. §3.1 race
//! rows and §3.3 interaction rows stay fully exhausted / structurally
//! blocked (cited from the progress doc, not re-derived); §3.4/§3.5 stay
//! structurally blocked for the same documented reason.
//!
//! Two PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte (no self-contradiction, so no third source
//! was required):
//!
//! - level 16 base attack bonus GENUINELY RISES to +8 (`16/2 = 8`, up from
//!   +7 at level 15) and good Will GENUINELY RISES to +10 (`16/2+2 = 10`,
//!   up from +9 at level 15), while poor Fortitude/Reflex both STAY at +5
//!   (`16/3 = 5`, an integer-division coincidence with level 15).
//! - the PF1 Core Rulebook Wizard class table's level-16 "Special" column
//!   is genuinely BLANK on both primary sources — the Wizard's own next
//!   bonus feat lands at 20th level, not 16th — so this is a pure ceiling
//!   raise: no new pillar record is grounded from the Special column.
//! - the raw Wizard spells-per-day table's level-16 row is
//!   "4/4/4/4/4/4/3/3/2", up from the level-15 row "4/4/4/4/4/4/3/2/1" (the
//!   7th-level column rises from 2 to 3 and the 8th-level column rises from
//!   1 to 2); the 9th-level column STAYS "—" (9th-level wizard spells do
//!   not become available until wizard level 17), so no genuinely new
//!   spell-level column opens and the specialist bonus-slot flat count
//!   (one bonus slot of each spell level she can cast, 1st through 8th)
//!   STAYS at 8, unchanged from level 15 — the pre-existing
//!   `>= WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` branch already
//!   covers level 16, so no new threshold constant is needed.
//! - Intense Spells' bonus-damage magnitude GENUINELY RISES to 8
//!   (`max(16/2, 1) = 8`, up from 7 at level 15) via the pre-existing
//!   formula; Force Missile's uses-per-day pool stays the level-independent
//!   3 + Intelligence modifier (6); Scribe Scroll and the school
//!   specialization choice recognitions are not level-gated and still fire.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven
//! 5th/10th/15th-level "Bonus feat" selection/execution, or the prepared
//! spellbook / spells-prepared / spell-slot posture burden (all stay
//! named-but-unproven, unchanged from levels 1-15), and it does not ground
//! Wizard level 17+. It also preserves the accepted Wizard level-1..
//! level-15 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control. Per the brief's lesson about stale negative
//! controls, a targeted grep for `wizard.*is_not_promoted` and
//! `class:wizard:16` found FIVE stale sibling files carrying a "level 16
//! stays claim-blocked" negative control: `tests/sd13_wizard_level10_progression.rs`,
//! `tests/sd18_wizard_level11_widening.rs`, `tests/sd18_wizard_level12_widening.rs`,
//! `tests/sd18_wizard_level13_widening.rs`, and `tests/sd18_wizard_level14_widening.rs`
//! — this cycle moves all five sibling "level 16 is not promoted" negative
//! controls to a "level 17 is not promoted" boundary in the same commit;
//! `tests/sd18_wizard_level15_widening.rs`'s own "level 16 is not promoted"
//! test is removed rather than moved, since level 16 is now itself the
//! supported/grounded row rather than the out-of-range boundary, mirroring
//! the Barbarian/Fighter level-16 cycles' identical fix.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level15_sd18_widening_deterministic_input.txt"
);

const WIZARD_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level16_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 16 genuinely rises -----

#[test]
fn wizard_level16_base_attack_bonus_is_grounded_and_genuinely_rises() {
    let input = load(WIZARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Wizard level 16 1/2-BAB progression (16 / 2) must GENUINELY RISE to 8, up from 7 at \
         level 15: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 16: poor saves stay put, good Will genuinely rises -----

#[test]
fn wizard_level16_base_saves_are_grounded_poor_saves_stay_will_rises() {
    let input = load(WIZARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Wizard level 16 poor Fortitude (16/3) must STAY at 5 — an integer-division \
         coincidence with level 15"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Wizard level 16 poor Reflex (16/3) must STAY at 5 — an integer-division coincidence \
         with level 15"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 10,
        "Wizard level 16 good Will (16/2+2) must GENUINELY RISE to 10, up from 9 at level 15"
    );
}

// ----- The specialist bonus slot count stays put at level 16 (no new spell level opens) -----

#[test]
fn wizard_level16_specialist_bonus_slot_stays_put_at_eight() {
    let input = load(WIZARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 8,
        "Wizard level 16 specialist bonus slot count must STAY at 8 — the raw spells-per-day \
         table's level-16 row \"4/4/4/4/4/4/3/3/2\" still shows a \"—\" 9th-level column (9th-level \
         wizard spells first become available at level 17): {}",
        slot.detail
    );
}

// ----- Intense Spells' bonus damage genuinely rises at level 16 -----

#[test]
fn wizard_level16_intense_spells_bonus_damage_genuinely_rises() {
    let input = load(WIZARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 8,
        "Intense Spells' bonus-damage magnitude (max(16/2, 1)) must GENUINELY RISE to 8 at \
         level 16, up from 7 at level 15: {}",
        intense.detail
    );
}

// ----- Force Missile, Scribe Scroll, and the specialization choice carry over unchanged -----

#[test]
fn wizard_level16_grants_carry_over_unchanged() {
    let input = load(WIZARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 16"
    );

    let scribe_scroll = explanation(&computation, "class_chassis.wizard.scribe_scroll");
    assert_eq!(scribe_scroll.value, 0, "Scribe Scroll must still carry no mechanical value");

    let specialization = explanation(&computation, "class_chassis.wizard.specialization_choice");
    assert_eq!(
        specialization.value, 0,
        "the school specialization choice seam must still carry no mechanical value"
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn wizard_level16_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(WIZARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "level-16 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-16 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-16 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 15 truth is unchanged by this widening -----

#[test]
fn wizard_level15_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 7, "Wizard level 15 base attack bonus must stay 7");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 8, "Wizard level 15 specialist bonus slot count must stay 8");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 7, "Wizard level 15 Intense Spells bonus damage must stay 7");
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level16_recognition() {
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
fn multiclass_wizard_level16_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL16_FIXTURE.replace(
        "class_level=class:wizard:16",
        "class_level=class:wizard:16\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-16 widening -----

#[test]
fn matrix_wizard_row_names_level_16_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level16_widening"),
        "wizard row must cite the live SD18 level-16 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 16") || note.contains("level-16"),
        "wizard partial note must name the level-16 widening: {note}"
    );
}
