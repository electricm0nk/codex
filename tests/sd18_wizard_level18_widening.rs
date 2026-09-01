//! SD18 Wizard level-18 widening grounding proof.
//!
//! Widens the accepted deterministic Human Wizard level-1..level-17 prepared
//! arcane spell-bearing chassis (`tests/sd18_wizard_level17_widening.rs`) to
//! Wizard level 18, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=17` to `1..=18` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 18`, exactly as the Ranger/Bard/Rogue/
//! Fighter/Cleric/Paladin/Barbarian/Sorcerer cycles widened their own
//! `MAX_SUPPORTED_<CLASS>_LEVEL` from 16 to 17 — this is the loop's FIRST
//! landing in the §3.2 level-18 sweep, opening it. §3.1 race rows and §3.3
//! interaction rows stay fully exhausted / structurally blocked (cited from
//! the progress doc, not re-derived); §3.4/§3.5 stay structurally blocked
//! for the same documented reason.
//!
//! Two PF1 CRB primary sources (d20pfsrd.com and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! fetching the full levels-16-through-19 block in one pass so the level-18
//! row's neighbors were visible in context (guards against
//! level-misattribution), and both agree byte-for-byte on all four rows (no
//! self-contradiction, so no third source was required):
//!
//! - level 16: "+8/+3 | +5 | +5 | +10 | — | 4/4/4/4/4/4/3/3/2/—"
//! - level 17: "+8/+3 | +5 | +5 | +10 | — | 4/4/4/4/4/4/4/3/2/1"
//! - level 18: "+9/+4 | +6 | +6 | +11 | — | 4/4/4/4/4/4/4/3/3/2"
//! - level 19: "+9/+4 | +6 | +6 | +11 | — | 4/4/4/4/4/4/4/4/3/3"
//!
//! So at level 18: base attack bonus GENUINELY RISES to +9 (`18/2 = 9`, up
//! from +8 at level 17) and good Will GENUINELY RISES to +11 (`18/2+2 = 11`,
//! up from +10 at level 17), while poor Fortitude/Reflex both GENUINELY RISE
//! to +6 (`18/3 = 6`, up from +5 at level 17).
//! - the PF1 Core Rulebook Wizard class table's level-18 "Special" column is
//!   genuinely BLANK on both primary sources — the Wizard's own next bonus
//!   feat lands at 20th level, not 18th — so the Special column itself is a
//!   pure ceiling raise: no new named-feature pillar record is grounded from
//!   it.
//! - the raw Wizard spells-per-day table's level-18 row is
//!   "4/4/4/4/4/4/4/3/3/2", up from the level-17 row "4/4/4/4/4/4/4/3/2/1"
//!   (the 8th-level column rises from 2 to 3 and the 9th-level column rises
//!   from 1 to 2) — but NO genuinely new spell-level column opens (9th is
//!   already the highest wizard spell level in PF1), so the specialist
//!   bonus-slot flat count (one bonus slot of each spell level she can cast,
//!   1st through 9th) STAYS at 9, an integer-division-free but genuinely
//!   flat coincidence, not a new column opening, checked rather than assumed.
//! - Intense Spells' bonus-damage magnitude GENUINELY RISES to 9
//!   (`max(18/2, 1) = 9`, up from 8 at level 17) via the pre-existing
//!   formula; Force Missile's uses-per-day pool stays the level-independent
//!   3 + Intelligence modifier (6); Scribe Scroll and the school
//!   specialization choice recognitions are not level-gated and still fire.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven
//! 5th/10th/15th-level "Bonus feat" selection/execution, or the prepared
//! spellbook / spells-prepared / spell-slot posture burden (all stay
//! named-but-unproven, unchanged from levels 1-17), and it does not ground
//! Wizard level 19+. It also preserves the accepted Wizard level-1..
//! level-17 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control. Per the brief's lesson about stale negative
//! controls, a targeted grep for `wizard.*is_not_promoted` and
//! `class:wizard:18` found ONE stale sibling file carrying a "level 18 stays
//! claim-blocked" negative control: `tests/sd18_wizard_level17_widening.rs`
//! — this cycle removes that test's "level 18 is not promoted" negative
//! control rather than moving it, since level 18 is now itself the
//! supported/grounded row rather than the out-of-range boundary, mirroring
//! the Ranger/Bard/Rogue/Fighter/Wizard/Cleric/Paladin/Barbarian/Sorcerer
//! level-17 cycles' identical fix pattern one level further out.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level17_sd18_widening_deterministic_input.txt"
);

const WIZARD_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level18_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus genuinely rises at level 18 -----

#[test]
fn wizard_level18_base_attack_bonus_genuinely_rises() {
    let input = load(WIZARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Wizard level 18 1/2-BAB progression (18 / 2) must GENUINELY RISE to 9, up from 8 at \
         level 17: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 18: all three genuinely rise -----

#[test]
fn wizard_level18_base_saves_genuinely_rise() {
    let input = load(WIZARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Wizard level 18 poor Fortitude (18/3) must GENUINELY RISE to 6, up from 5 at level 17"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Wizard level 18 poor Reflex (18/3) must GENUINELY RISE to 6, up from 5 at level 17"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 11,
        "Wizard level 18 good Will (18/2+2) must GENUINELY RISE to 11, up from 10 at level 17"
    );
}

// ----- The specialist bonus slot count stays flat at level 18 (no new column opens) -----

#[test]
fn wizard_level18_specialist_bonus_slot_stays_flat_at_nine() {
    let input = load(WIZARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 9,
        "Wizard level 18 specialist bonus slot count must STAY at 9 — the raw spells-per-day \
         table's level-18 row \"4/4/4/4/4/4/4/3/3/2\" does not open any spell-level column beyond \
         the 9th (already the highest wizard spell level in PF1): {}",
        slot.detail
    );
}

// ----- Intense Spells' bonus damage genuinely rises at level 18 -----

#[test]
fn wizard_level18_intense_spells_bonus_damage_genuinely_rises() {
    let input = load(WIZARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 9,
        "Intense Spells' bonus-damage magnitude (max(18/2, 1)) must GENUINELY RISE to 9 at \
         level 18, up from 8 at level 17: {}",
        intense.detail
    );
}

// ----- Force Missile, Scribe Scroll, and the specialization choice carry over unchanged -----

#[test]
fn wizard_level18_grants_carry_over_unchanged() {
    let input = load(WIZARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 18"
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
fn wizard_level18_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(WIZARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "level-18 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-18 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-18 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 17 truth is unchanged by this widening -----

#[test]
fn wizard_level17_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Wizard level 17 base attack bonus must stay 8");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 9, "Wizard level 17 specialist bonus slot count must stay 9");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 8, "Wizard level 17 Intense Spells bonus damage must stay 8");
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level18_recognition() {
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
fn multiclass_wizard_level18_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL18_FIXTURE.replace(
        "class_level=class:wizard:18",
        "class_level=class:wizard:18\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-18 widening -----

#[test]
fn matrix_wizard_row_names_level_18_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level18_widening"),
        "wizard row must cite the live SD18 level-18 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 18") || note.contains("level-18"),
        "wizard partial note must name the level-18 widening: {note}"
    );
}
