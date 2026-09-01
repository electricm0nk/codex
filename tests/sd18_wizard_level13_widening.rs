//! SD18 Wizard level-13 widening grounding proof.
//!
//! Widens the accepted deterministic Human Wizard level-1..level-12 prepared
//! arcane spell-bearing chassis (`tests/sd18_wizard_level12_widening.rs`) to
//! Wizard level 13, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=12` to `1..=13` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 13`, exactly as prior cycles widened
//! `MAX_SUPPORTED_ROGUE_LEVEL`, `MAX_SUPPORTED_BARBARIAN_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_RANGER_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_BARD_LEVEL`, `MAX_SUPPORTED_PALADIN_LEVEL`, and
//! `MAX_SUPPORTED_SORCERER_LEVEL`, all from 12 to 13 — the tenth §3.2
//! level-13 landing, after Rogue, Barbarian, Fighter, Ranger, Cleric, Druid,
//! Bard, Paladin, and Sorcerer, and the FOURTH spell-bearing class (after
//! Ranger, Bard, and Sorcerer) to reach it, and the LAST of the 11 §3.2
//! classes to reach level 13 (Monk excluded — confirmed dead end at level 13,
//! Diamond Soul needs spell resistance). §3.1 race rows and §3.3 interaction
//! rows stay fully exhausted / structurally blocked (cited from the progress
//! doc, not re-derived); §3.4/§3.5 stay structurally blocked for the same
//! documented reason.
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte:
//!
//! - level 13 base attack bonus and all three base saves STAY numerically
//!   unchanged from level 12 (`13/2 = 6`, `13/3 = 4`, `13/3 = 4`,
//!   `13/2+2 = 8`) — integer-division coincidences, re-verified rather than
//!   assumed, not a sign any formula stopped scaling.
//! - the PF1 Core Rulebook Wizard class table's level-13 "Special" column is
//!   genuinely BLANK on both primary sources (the Wizard's bonus feats land
//!   only at levels 5/10/15/20), so no new pillar is grounded from the
//!   Special column.
//! - the raw Wizard spells-per-day table's level-13 row is "4/4/4/4/4/3/2/1",
//!   the first non-"—" 7th-level column, up from the level-12 row
//!   "4/4/4/4/3/3/2" whose 7th-level column does not exist at all — so a
//!   level-13 specialist wizard casts 7th-level spells for the first time,
//!   and the specialist bonus-slot flat count (one bonus slot of each spell
//!   level she can cast) GENUINELY RISES to 7, from 6 at level 12.
//! - Intense Spells' bonus-damage magnitude STAYS at 6 (`max(13/2, 1) = 6`,
//!   another integer-division coincidence); Force Missile's uses-per-day
//!   pool stays the level-independent 3 + Intelligence modifier (6); Scribe
//!   Scroll and the school specialization choice recognitions are not
//!   level-gated and still fire.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven 5th/10th-
//! level "Bonus feat" selection/execution, or the prepared spellbook /
//! spells-prepared / spell-slot posture burden (all stay named-but-unproven,
//! unchanged from levels 1-12), and it does not ground Wizard level 14+. It
//! also preserves the accepted Wizard level-1..level-12 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control. Per
//! the brief's lesson about stale negative controls, this cycle also moves
//! the sibling "level 13 is not promoted" negative controls in
//! `tests/sd13_wizard_level10_progression.rs`,
//! `tests/sd18_wizard_level11_widening.rs`, and
//! `tests/sd18_wizard_level12_widening.rs` to a "level 14 is not promoted"
//! boundary in the same commit.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level12_sd18_widening_deterministic_input.txt"
);

const WIZARD_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level13_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 13 stays unchanged (integer-division coincidence) -----

#[test]
fn wizard_level13_base_attack_bonus_is_grounded_and_stays_unchanged() {
    let input = load(WIZARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Wizard level 13 1/2-BAB progression (13 / 2) must equal 6 — numerically unchanged from \
         level 12, an integer-division coincidence: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 13 stay unchanged (all three, integer-division coincidences) -----

#[test]
fn wizard_level13_base_saves_are_grounded_and_stay_unchanged() {
    let input = load(WIZARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Wizard level 13 poor Fortitude (13/3) must stay 4 — numerically unchanged from level 12"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Wizard level 13 poor Reflex (13/3) must stay 4 — numerically unchanged from level 12"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 8,
        "Wizard level 13 good Will (13/2+2) must stay 8 — numerically unchanged from level 12"
    );
}

// ----- The specialist bonus slot count genuinely rises to 7 (7th-level spells open) -----

#[test]
fn wizard_level13_specialist_bonus_slot_genuinely_rises_to_seven() {
    let input = load(WIZARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 7,
        "Wizard level 13 specialist bonus slot count must genuinely rise to 7 — 7th-level \
         wizard spells become accessible for the first time at level 13, verified against the \
         raw spells-per-day table row \"4/4/4/4/4/3/2/1\": {}",
        slot.detail
    );
}

// ----- Intense Spells' bonus damage stays unchanged at level 13 -----

#[test]
fn wizard_level13_intense_spells_bonus_damage_stays_unchanged() {
    let input = load(WIZARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 6,
        "Intense Spells' bonus-damage magnitude (max(13/2, 1)) must stay 6 at level 13, \
         unchanged from level 12, an integer-division coincidence: {}",
        intense.detail
    );
}

// ----- Force Missile, Scribe Scroll, and the specialization choice carry over unchanged -----

#[test]
fn wizard_level13_grants_carry_over_unchanged() {
    let input = load(WIZARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 13"
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
fn wizard_level13_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(WIZARD_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "level-13 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-13 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-13 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 12 truth is unchanged by this widening -----

#[test]
fn wizard_level12_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 6, "Wizard level 12 base attack bonus must stay 6");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 6, "Wizard level 12 specialist bonus slot count must stay 6");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 6, "Wizard level 12 Intense Spells bonus damage must stay 6");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (widened from level 15 all the way to level 20 by the SD18
// wizard-level15-widening through wizard-level20-widening cycles, which
// genuinely promote levels 15-20 — see tests/sd18_wizard_level15_widening.rs
// through tests/sd18_wizard_level20_widening.rs — mirroring the exact same
// boundary move the Cleric level-20 widening cycle made for its own sibling
// widening tests. PF1 has no 21st character level; this is a pure
// implementation-gate check only.)

#[test]
fn wizard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = WIZARD_LEVEL13_FIXTURE.replace("class:wizard:13", "class:wizard:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.wizard.")
                || e.id.starts_with("class_feature.wizard.")
                || e.id == "class_chassis.spell_baseline.wizard"),
        "level-21 Wizard must not gain any bounded wizard explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level13_recognition() {
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
fn multiclass_wizard_level13_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL13_FIXTURE.replace(
        "class_level=class:wizard:13",
        "class_level=class:wizard:13\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-13 widening -----

#[test]
fn matrix_wizard_row_names_level_13_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level13_widening"),
        "wizard row must cite the live SD18 level-13 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 13") || note.contains("level-13"),
        "wizard partial note must name the level-13 widening: {note}"
    );
}
