//! SD18 Wizard level-17 widening grounding proof.
//!
//! Widens the accepted deterministic Human Wizard level-1..level-16 prepared
//! arcane spell-bearing chassis (`tests/sd18_wizard_level16_widening.rs`) to
//! Wizard level 17, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=16` to `1..=17` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 17`, exactly as the Ranger/Bard/Rogue/
//! Fighter cycles widened their own `MAX_SUPPORTED_<CLASS>_LEVEL` from 16 to
//! 17 — the loop's FIFTH §3.2 level-17 landing, after Ranger, Bard, Rogue,
//! and Fighter, and the first spell-bearing class to reach level 17 in the
//! level-17 sweep. §3.1 race rows and §3.3 interaction rows stay fully
//! exhausted / structurally blocked (cited from the progress doc, not
//! re-derived); §3.4/§3.5 stay structurally blocked for the same documented
//! reason.
//!
//! Two PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! fetching the full levels-15-through-18 block in one pass so the level-17
//! row's neighbors were visible in context (guards against
//! level-misattribution), and both agree byte-for-byte on all four rows (no
//! self-contradiction, so no third source was required):
//!
//! - level 15: "+7/+2 | +5 | +5 | +9 | Bonus feat | 4/4/4/4/4/4/3/2/1/—"
//! - level 16: "+8/+3 | +5 | +5 | +10 | — | 4/4/4/4/4/4/3/3/2/—"
//! - level 17: "+8/+3 | +5 | +5 | +10 | — | 4/4/4/4/4/4/4/3/2/1"
//! - level 18: "+9/+4 | +6 | +6 | +11 | — | 4/4/4/4/4/4/4/3/3/2"
//!
//! So at level 17: base attack bonus STAYS at +8 (`17/2 = 8`, an
//! integer-division coincidence with level 16) and good Will STAYS at +10
//! (`17/2+2 = 10`, also an integer-division coincidence with level 16),
//! while poor Fortitude/Reflex both STAY at +5 (`17/3 = 5`, an
//! integer-division coincidence with level 16).
//! - the PF1 Core Rulebook Wizard class table's level-17 "Special" column is
//!   genuinely BLANK on both primary sources — the Wizard's own next bonus
//!   feat lands at 20th level, not 17th — so the Special column itself is a
//!   pure ceiling raise: no new named-feature pillar record is grounded from
//!   it.
//! - the raw Wizard spells-per-day table's level-17 row is
//!   "4/4/4/4/4/4/4/3/2/1", up from the level-16 row "4/4/4/4/4/4/3/3/2/—"
//!   (the 6th-level column rises from 3 to 4) AND a genuinely NEW 9th-level
//!   column opens for the first time (value 1) — so a level-17 specialist
//!   wizard casts 9th-level spells for the first time, and the specialist
//!   bonus-slot flat count (one bonus slot of each spell level she can cast,
//!   1st through 9th) GENUINELY RISES to 9, up from 8 at levels 15-16, via a
//!   new `WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 17` threshold
//!   constant gated exactly like the existing level-3/5/7/9/11/13/15 idiom.
//! - Intense Spells' bonus-damage magnitude STAYS at 8 (`max(17/2, 1) = 8`,
//!   an integer-division coincidence with level 16) via the pre-existing
//!   formula; Force Missile's uses-per-day pool stays the level-independent
//!   3 + Intelligence modifier (6); Scribe Scroll and the school
//!   specialization choice recognitions are not level-gated and still fire.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven
//! 5th/10th/15th-level "Bonus feat" selection/execution, or the prepared
//! spellbook / spells-prepared / spell-slot posture burden (all stay
//! named-but-unproven, unchanged from levels 1-16), and it does not ground
//! Wizard level 18+. It also preserves the accepted Wizard level-1..
//! level-16 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control. Per the brief's lesson about stale negative
//! controls, a targeted grep for `wizard.*is_not_promoted` and
//! `class:wizard:17` found FIVE stale sibling files carrying a "level 17
//! stays claim-blocked" negative control: `tests/sd13_wizard_level10_progression.rs`,
//! `tests/sd18_wizard_level11_widening.rs`, `tests/sd18_wizard_level12_widening.rs`,
//! `tests/sd18_wizard_level13_widening.rs`, and `tests/sd18_wizard_level14_widening.rs`
//! — this cycle moves all five sibling "level 17 is not promoted" negative
//! controls to a "level 18 is not promoted" boundary in the same commit;
//! `tests/sd18_wizard_level16_widening.rs`'s own "level 17 is not promoted"
//! test is removed rather than moved, since level 17 is now itself the
//! supported/grounded row rather than the out-of-range boundary, mirroring
//! the Ranger/Bard/Rogue/Fighter level-17 cycles' identical fix.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level16_sd18_widening_deterministic_input.txt"
);

const WIZARD_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level17_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 17 stays put (integer-division coincidence) -----

#[test]
fn wizard_level17_base_attack_bonus_is_grounded_and_stays_put() {
    let input = load(WIZARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 8,
        "Wizard level 17 1/2-BAB progression (17 / 2) must STAY at 8, an integer-division \
         coincidence with level 16: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 17: all three stay put (integer-division coincidences) -----

#[test]
fn wizard_level17_base_saves_are_grounded_and_stay_put() {
    let input = load(WIZARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Wizard level 17 poor Fortitude (17/3) must STAY at 5 — an integer-division \
         coincidence with level 16"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Wizard level 17 poor Reflex (17/3) must STAY at 5 — an integer-division coincidence \
         with level 16"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 10,
        "Wizard level 17 good Will (17/2+2) must STAY at 10 — an integer-division coincidence \
         with level 16"
    );
}

// ----- The specialist bonus slot count genuinely rises at level 17 (9th column opens) -----

#[test]
fn wizard_level17_specialist_bonus_slot_genuinely_rises_to_nine() {
    let input = load(WIZARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 9,
        "Wizard level 17 specialist bonus slot count must GENUINELY RISE to 9 — the raw \
         spells-per-day table's level-17 row \"4/4/4/4/4/4/4/3/2/1\" shows the first non-\"—\" \
         9th-level column (9th-level wizard spells first become available at level 17): {}",
        slot.detail
    );
}

// ----- Intense Spells' bonus damage stays put at level 17 (integer-division coincidence) -----

#[test]
fn wizard_level17_intense_spells_bonus_damage_stays_put() {
    let input = load(WIZARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 8,
        "Intense Spells' bonus-damage magnitude (max(17/2, 1)) must STAY at 8 at level 17 — an \
         integer-division coincidence with level 16: {}",
        intense.detail
    );
}

// ----- Force Missile, Scribe Scroll, and the specialization choice carry over unchanged -----

#[test]
fn wizard_level17_grants_carry_over_unchanged() {
    let input = load(WIZARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 17"
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
fn wizard_level17_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(WIZARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "level-17 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-17 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-17 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 16 truth is unchanged by this widening -----

#[test]
fn wizard_level16_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Wizard level 16 base attack bonus must stay 8");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 8, "Wizard level 16 specialist bonus slot count must stay 8");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 8, "Wizard level 16 Intense Spells bonus damage must stay 8");
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level17_recognition() {
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
fn multiclass_wizard_level17_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL17_FIXTURE.replace(
        "class_level=class:wizard:17",
        "class_level=class:wizard:17\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_wizard_row_names_level_17_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level17_widening"),
        "wizard row must cite the live SD18 level-17 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 17") || note.contains("level-17"),
        "wizard partial note must name the level-17 widening: {note}"
    );
}
