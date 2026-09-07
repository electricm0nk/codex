//! SD18 Wizard level-15 widening grounding proof.
//!
//! Widens the accepted deterministic Human Wizard level-1..level-14 prepared
//! arcane spell-bearing chassis (`tests/sd18_wizard_level14_widening.rs`) to
//! Wizard level 15, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=14` to `1..=15` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 15`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_ROGUE_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_CLERIC_LEVEL`,
//! `MAX_SUPPORTED_DRUID_LEVEL`, and `MAX_SUPPORTED_RANGER_LEVEL`, all from
//! 14 to 15 — the loop's SEVENTH §3.2 level-15 landing, after Barbarian,
//! Rogue, Fighter, Cleric, Druid, and Ranger, and the first pure 1/2-BAB
//! spell-bearing class to reach level 15 in the sweep. §3.1 race rows and
//! §3.3 interaction rows stay fully exhausted / structurally blocked (cited
//! from the progress doc, not re-derived); §3.4/§3.5 stay structurally
//! blocked for the same documented reason.
//!
//! Two PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte (no self-contradiction, so no third source
//! was required):
//!
//! - level 15 base attack bonus STAYS at +7 (`15/2 = 7`, an
//!   integer-division coincidence with level 14) and good Will STAYS at +9
//!   (`15/2+2 = 9`, another integer-division coincidence with level 14),
//!   while poor Fortitude/Reflex both GENUINELY RISE to +5 (`15/3 = 5`, up
//!   from +4 at level 14).
//! - the PF1 Core Rulebook Wizard class table's level-15 "Special" column
//!   reads "Bonus feat" on both primary sources — the SAME genuinely
//!   open-ended metamagic/item-creation/Spell-Mastery choice already left
//!   named-but-unproven at levels 5 and 10, NOT a new type of class
//!   feature, so no new pillar record is grounded from the Special column.
//! - the raw Wizard spells-per-day table's level-15 row is
//!   "4/4/4/4/4/4/3/2/1", up from the level-14 row "4/4/4/4/4/3/3/2" — the
//!   5th-level column rises from 3 to 4 AND a genuinely NEW 8th-level
//!   column appears for the first time (value 1) — so a level-15
//!   specialist wizard casts 8th-level spells for the first time, and the
//!   specialist bonus-slot flat count (one bonus slot of each spell level
//!   she can cast, 1st through 8th) GENUINELY RISES to 8, up from 7 at
//!   level 14, via a new `WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL =
//!   15` threshold constant gated exactly like the existing level-3/5/7/9/
//!   11/13 idiom.
//! - Intense Spells' bonus-damage magnitude STAYS at 7 (`max(15/2, 1) = 7`,
//!   an integer-division coincidence with level 14); Force Missile's
//!   uses-per-day pool stays the level-independent 3 + Intelligence
//!   modifier (6); Scribe Scroll and the school specialization choice
//!   recognitions are not level-gated and still fire.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven
//! 5th/10th/15th-level "Bonus feat" selection/execution, or the prepared
//! spellbook / spells-prepared / spell-slot posture burden (all stay
//! named-but-unproven, unchanged from levels 1-14), and it does not ground
//! Wizard level 16+. It also preserves the accepted Wizard level-1..
//! level-14 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control. Per the brief's lesson about stale negative
//! controls, a targeted grep for `wizard.*is_not_promoted` and
//! `class:wizard:15` found FIVE stale sibling files carrying a "level 15
//! stays claim-blocked" negative control (one more than the four usual
//! suspects, because this cycle's own immediately-preceding sibling,
//! `tests/sd18_wizard_level14_widening.rs`, also carried its own "level 15
//! is not promoted" control written when level 14 was that cycle's own
//! target) — this cycle moves all five sibling "level 15 is not promoted"
//! negative controls in `tests/sd13_wizard_level10_progression.rs`,
//! `tests/sd18_wizard_level11_widening.rs`,
//! `tests/sd18_wizard_level12_widening.rs`,
//! `tests/sd18_wizard_level13_widening.rs`, and
//! `tests/sd18_wizard_level14_widening.rs` to a "level 16 is not promoted"
//! boundary in the same commit.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level14_sd18_widening_deterministic_input.txt"
);

const WIZARD_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level15_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 15 stays put (integer-division coincidence) -----

#[test]
fn wizard_level15_base_attack_bonus_is_grounded_and_stays_put() {
    let input = load(WIZARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Wizard level 15 1/2-BAB progression (15 / 2) must equal 7 — an integer-division \
         coincidence with level 14: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 15: poor saves rise, good Will stays put -----

#[test]
fn wizard_level15_base_saves_are_grounded_poor_saves_rise_will_stays() {
    let input = load(WIZARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 5,
        "Wizard level 15 poor Fortitude (15/3) must genuinely rise to 5, up from 4 at level 14"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Wizard level 15 poor Reflex (15/3) must genuinely rise to 5, up from 4 at level 14"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 9,
        "Wizard level 15 good Will (15/2+2) must stay 9 — an integer-division coincidence with \
         level 14"
    );
}

// ----- The specialist bonus slot count genuinely rises to 8 (8th spell level opens) -----

#[test]
fn wizard_level15_specialist_bonus_slot_genuinely_rises_to_eight() {
    let input = load(WIZARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 8,
        "Wizard level 15 specialist bonus slot count must GENUINELY RISE to 8 — the raw \
         spells-per-day table's level-15 row \"4/4/4/4/4/4/3/2/1\" shows a genuinely NEW \
         non-\"—\" 8th-level column for the first time: {}",
        slot.detail
    );
}

// ----- Intense Spells' bonus damage stays put at level 15 (integer-division coincidence) -----

#[test]
fn wizard_level15_intense_spells_bonus_damage_stays_put() {
    let input = load(WIZARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 7,
        "Intense Spells' bonus-damage magnitude (max(15/2, 1)) must STAY at 7 at level 15 — an \
         integer-division coincidence with level 14: {}",
        intense.detail
    );
}

// ----- Force Missile, Scribe Scroll, and the specialization choice carry over unchanged -----

#[test]
fn wizard_level15_grants_carry_over_unchanged() {
    let input = load(WIZARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 15"
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
fn wizard_level15_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(WIZARD_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "level-15 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-15 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-15 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 14 truth is unchanged by this widening -----

#[test]
fn wizard_level14_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 7, "Wizard level 14 base attack bonus must stay 7");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 7, "Wizard level 14 specialist bonus slot count must stay 7");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 7, "Wizard level 14 Intense Spells bonus damage must stay 7");
}

// Note: this file's own "level 16 is not promoted" negative control was
// removed by the SD18 wizard-level16-widening cycle (see
// tests/sd18_wizard_level16_widening.rs), since level 16 is now itself the
// supported/grounded row rather than the out-of-range boundary — mirroring
// the exact same fix the Barbarian/Fighter level-16 widening cycles each
// made for their own immediately-preceding level-15 sibling.

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level15_recognition() {
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
fn multiclass_wizard_level15_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL15_FIXTURE.replace(
        "class_level=class:wizard:15",
        "class_level=class:wizard:15\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_wizard_row_names_level_15_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level15_widening"),
        "wizard row must cite the live SD18 level-15 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "wizard partial note must name the level-15 widening: {note}"
    );
}
