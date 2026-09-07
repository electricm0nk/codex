//! SD18 Wizard level-12 widening grounding proof.
//!
//! Widens the accepted deterministic Human Wizard level-1..level-11 prepared
//! arcane spell-bearing chassis (`tests/sd18_wizard_level11_widening.rs`) to
//! Wizard level 12, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=11` to `1..=12` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 12`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_MONK_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, `MAX_SUPPORTED_ROGUE_LEVEL`,
//! `MAX_SUPPORTED_RANGER_LEVEL`, and `MAX_SUPPORTED_SORCERER_LEVEL`, all from
//! 11 to 12). §3.1 race rows and §3.3 interaction rows stay fully exhausted /
//! structurally blocked (cited from the progress doc, not re-derived); §3.4/
//! §3.5 stay structurally blocked for the same documented reason.
//!
//! This cycle independently re-verified (not assumed from the prior
//! Sorcerer level-12 cycle's outcome) whether Wizard's own live claim-blocker
//! (`class_spell.wizard.prepared_spellbook.unsupported`, named in the §3.4
//! investigation) structurally gates any further Wizard progression. Reading
//! `explain_wizard_level1_prepared_spell_baseline` directly in
//! `pilot_compute.rs` confirms it does not: the diagnostic is pushed
//! unconditionally alongside the level's other explanations (it is not part
//! of any control-flow guard on `supported_wizard_level` or
//! `MAX_SUPPORTED_WIZARD_LEVEL`), exactly mirroring the pattern of every
//! sibling class's own remaining-burden diagnostics (e.g. Sorcerer's
//! `class_spell.sorcerer.spontaneous.unsupported`, Cleric's prepared-spell
//! burden note). It marks incomplete coverage; it does not block arithmetic
//! widening.
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte:
//!
//! - level 12 base attack bonus genuinely rises to +6 (`12 / 2 = 6`, up from
//!   +5 at level 11) and all three base saves genuinely rise too: +4
//!   Fortitude (poor, `12 / 3 = 4`), +4 Reflex (poor, `12 / 3 = 4`), and +8
//!   Will (good, `12 / 2 + 2 = 8`) — unlike level 11, where all four values
//!   stayed numerically identical to level 10 (integer-division
//!   coincidences), all four genuinely rise at level 12, via the same
//!   formulas already grounded at levels 1-11, not re-derived.
//! - the PF1 Core Rulebook Wizard class table's level-12 "Special" column is
//!   genuinely BLANK on both primary sources (the Wizard's bonus feats land
//!   only at levels 5/10/15/20), so no new pillar is grounded from the
//!   Special column, mirroring the Sorcerer level-12 cycle's own clean
//!   landing.
//! - the raw Wizard spells-per-day table's level-12 row is "4/4/4/4/3/3/2",
//!   with the 6th-level column rising from 1 to 2 but no 7th-level column
//!   appearing at all — 7th-level wizard spells do not become accessible
//!   until level 13, verified rather than assumed, so the specialist
//!   bonus-slot flat count (one bonus slot of each spell level she can cast)
//!   STAYS at 6, an integer-division/threshold-stasis coincidence, not a
//!   sign the pillar stopped scaling.
//! - Intense Spells' bonus-damage magnitude GENUINELY RISES to 6
//!   (`max(12/2, 1) = 6`, up from 5 at level 11), via the pre-existing
//!   half-wizard-level-minimum-1 formula, not re-derived; Force Missile's
//!   uses-per-day pool stays the level-independent 3 + Intelligence modifier
//!   (6); Scribe Scroll and the school specialization choice recognitions
//!   are not level-gated and still fire.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven 5th/10th-
//! level "Bonus feat" selection/execution, or the prepared spellbook /
//! spells-prepared / spell-slot posture burden (all stay named-but-unproven,
//! unchanged from levels 1-11), and it does not ground Wizard level 13+. It
//! also preserves the accepted Wizard level-1..level-11 truth (unchanged),
//! the Fighter negative control, and the multiclass negative control. Per
//! the brief's lesson about stale negative controls, this cycle also moves
//! the sibling "level 12 is not promoted" negative controls in
//! `tests/sd13_wizard_level10_progression.rs` and
//! `tests/sd18_wizard_level11_widening.rs` to a "level 13 is not promoted"
//! boundary in the same commit.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level11_sd18_widening_deterministic_input.txt"
);

const WIZARD_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level12_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus at level 12 genuinely rises -----

#[test]
fn wizard_level12_base_attack_bonus_is_grounded_and_rises() {
    let input = load(WIZARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 6,
        "Wizard level 12 1/2-BAB progression (12 / 2) must equal 6 — genuinely risen from 5 at \
         level 11: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 12 genuinely rise (all three) -----

#[test]
fn wizard_level12_base_saves_are_grounded_and_rise() {
    let input = load(WIZARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Wizard level 12 poor Fortitude (12/3) must equal 4 — genuinely risen from 3 at level 11"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Wizard level 12 poor Reflex (12/3) must equal 4 — genuinely risen from 3 at level 11"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 8,
        "Wizard level 12 good Will (12/2+2) must equal 8 — genuinely risen from 7 at level 11"
    );
}

// ----- The specialist bonus slot count stays at 6 (threshold stasis, not a regression) -----

#[test]
fn wizard_level12_specialist_bonus_slot_stays_at_six() {
    let input = load(WIZARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 6,
        "Wizard level 12 specialist bonus slot count must stay 6 — 7th-level wizard spells do \
         not become accessible until level 13, verified rather than assumed against the raw \
         spells-per-day table row \"4/4/4/4/3/3/2\": {}",
        slot.detail
    );
}

// ----- Intense Spells' bonus damage genuinely rises at level 12 -----

#[test]
fn wizard_level12_intense_spells_bonus_damage_genuinely_rises() {
    let input = load(WIZARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 6,
        "Intense Spells' bonus-damage magnitude (max(12/2, 1)) must genuinely rise to 6 at \
         level 12, up from 5 at level 11: {}",
        intense.detail
    );
}

// ----- Force Missile, Scribe Scroll, and the specialization choice carry over unchanged -----

#[test]
fn wizard_level12_grants_carry_over_unchanged() {
    let input = load(WIZARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 12"
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
fn wizard_level12_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(WIZARD_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "level-12 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-12 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-12 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 11 truth is unchanged by this widening -----

#[test]
fn wizard_level11_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 5, "Wizard level 11 base attack bonus must stay 5");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 6, "Wizard level 11 specialist bonus slot count must stay 6");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 5, "Wizard level 11 Intense Spells bonus damage must stay 5");
}

// ----- Negative control: level 21 stays unrecognized by this slice -----
// (widened from level 14 all the way to level 20 by the SD18
// wizard-level14-widening through wizard-level20-widening cycles, which
// genuinely promote levels 14-20 — see tests/sd18_wizard_level14_widening.rs
// through tests/sd18_wizard_level20_widening.rs — mirroring the exact same
// boundary move the Cleric level-20 widening cycle made for its own sibling
// widening tests. PF1 has no 21st character level; this is a pure
// implementation-gate check only.)

#[test]
fn wizard_level_21_is_not_promoted_by_this_slice() {
    let level_21 = WIZARD_LEVEL12_FIXTURE.replace("class:wizard:12", "class:wizard:21");
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
fn fighter_does_not_gain_wizard_level12_recognition() {
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
fn multiclass_wizard_level12_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL12_FIXTURE.replace(
        "class_level=class:wizard:12",
        "class_level=class:wizard:12\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_wizard_row_names_level_12_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level12_widening"),
        "wizard row must cite the live SD18 level-12 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "wizard partial note must name the level-12 widening: {note}"
    );
}
