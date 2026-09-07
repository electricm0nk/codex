//! SD18 Wizard level-19 widening grounding proof.
//!
//! Widens the accepted deterministic Human Wizard level-1..level-18 prepared
//! arcane spell-bearing chassis (`tests/sd18_wizard_level18_widening.rs`) to
//! Wizard level 19, mirroring the sibling-class level-range-gate idiom
//! (`supported_wizard_level` is generalized from `1..=18` to `1..=19` via
//! `MAX_SUPPORTED_WIZARD_LEVEL = 19`, exactly as the Barbarian/Cleric/
//! Fighter/Bard/Paladin/Ranger/Rogue/Sorcerer cycles widened their own
//! `MAX_SUPPORTED_<CLASS>_LEVEL` from 18 to 19 — this is the loop's NINTH
//! landing in the §3.2 level-19 sweep and the LAST of the 9 eligible
//! classes, fully closing the level-19 sweep. §3.1 race rows and §3.3
//! interaction rows stay fully exhausted / structurally blocked (cited from
//! the progress doc, not re-derived); §3.4/§3.5 stay structurally blocked
//! for the same documented reason.
//!
//! Two PF1 CRB primary sources (d20pfsrd.com and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! fetching the full levels-16-through-20 block in one pass so the level-19
//! row's neighbors were visible in context (guards against
//! level-misattribution), and both agree byte-for-byte on all five rows (no
//! self-contradiction, so no third source was required):
//!
//! - level 16: "+8/+3 | +5 | +5 | +10 | — | 4/4/4/4/4/4/3/3/2/—"
//! - level 17: "+8/+3 | +5 | +5 | +10 | — | 4/4/4/4/4/4/4/3/2/1"
//! - level 18: "+9/+4 | +6 | +6 | +11 | — | 4/4/4/4/4/4/4/3/3/2"
//! - level 19: "+9/+4 | +6 | +6 | +11 | — | 4/4/4/4/4/4/4/4/3/3"
//! - level 20: "+10/+5 | +6 | +6 | +12 | Bonus feat | 4/4/4/4/4/4/4/4/4/4"
//!
//! So at level 19: base attack bonus STAYS at +9 (`19/2 = 9`, an
//! integer-division coincidence with level 18) and good Will STAYS at +11
//! (`19/2+2 = 11`, also an integer-division coincidence with level 18),
//! while poor Fortitude/Reflex both STAY at +6 (`19/3 = 6`, an
//! integer-division coincidence with level 18).
//! - the PF1 Core Rulebook Wizard class table's level-19 "Special" column is
//!   genuinely BLANK on both primary sources — the Wizard's own next bonus
//!   feat lands at 20th level, not 19th (confirmed directly rather than
//!   assumed: the wizard's bonus-feat cadence is 5/10/15/20, so 19 is NOT a
//!   bonus-feat level) — so the Special column itself is a pure ceiling
//!   raise: no new named-feature pillar record is grounded from it.
//! - the raw Wizard spells-per-day table's level-19 row is
//!   "4/4/4/4/4/4/4/4/3/3", up from the level-18 row "4/4/4/4/4/4/4/3/3/2"
//!   (the 7th-level column rises from 3 to 4 and the 9th-level column rises
//!   from 2 to 3) — but NO genuinely new spell-level column opens (9th is
//!   already the highest wizard spell level in PF1, first opened at level
//!   17), so the specialist bonus-slot flat count (one bonus slot of each
//!   spell level she can cast, 1st through 9th) STAYS at 9, unchanged from
//!   levels 17-18.
//! - Intense Spells' bonus-damage magnitude STAYS at 9 (`max(19/2, 1) = 9`,
//!   an integer-division coincidence with level 18) via the pre-existing
//!   formula; Force Missile's uses-per-day pool stays the level-independent
//!   3 + Intelligence modifier (6); Scribe Scroll and the school
//!   specialization choice recognitions are not level-gated and still fire.
//!
//! It deliberately does not touch the school-power execution burden
//! (Intense Spells' damage application, Force Missile's casting execution),
//! the opposed-school preparation-cost burden, the still-unproven
//! 5th/10th/15th/20th-level "Bonus feat" selection/execution, or the
//! prepared spellbook / spells-prepared / spell-slot posture burden (all
//! stay named-but-unproven, unchanged from levels 1-18), and it does not
//! ground Wizard level 20+. It also preserves the accepted Wizard level-1..
//! level-18 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control. Per the brief's lesson about stale negative
//! controls, a targeted grep for `wizard.*is_not_promoted` and
//! `class:wizard:19` found FIVE stale sibling files carrying a "level 19
//! stays claim-blocked" negative control that this cycle fixes in the same
//! commit before running the full suite: `tests/sd18_wizard_level11_widening.rs`,
//! `tests/sd18_wizard_level12_widening.rs`, `tests/sd18_wizard_level13_widening.rs`,
//! and `tests/sd18_wizard_level14_widening.rs`, all moved from a level-19 to
//! a level-20 negative-control boundary, plus
//! `tests/sd18_wizard_level18_widening.rs`'s own level-19 negative-control
//! test removed rather than moved, since level 19 is now itself the
//! supported row — mirroring the exact fix pattern used for every prior
//! level-N cycle's own siblings (Barbarian, Cleric, Fighter, Bard, Paladin,
//! Ranger, Rogue, Sorcerer).

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level18_sd18_widening_deterministic_input.txt"
);

const WIZARD_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level19_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

// ----- Base attack bonus stays flat at level 19 (integer-division coincidence) -----

#[test]
fn wizard_level19_base_attack_bonus_stays_flat() {
    let input = load(WIZARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Wizard level 19 1/2-BAB progression (19 / 2) must STAY at 9, an integer-division \
         coincidence with level 18: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 19: all three stay flat (integer-division coincidence) -----

#[test]
fn wizard_level19_base_saves_stay_flat() {
    let input = load(WIZARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.wizard.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Wizard level 19 poor Fortitude (19/3) must STAY at 6, an integer-division coincidence \
         with level 18"
    );

    let reflex = explanation(&computation, "class_chassis.wizard.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Wizard level 19 poor Reflex (19/3) must STAY at 6, an integer-division coincidence \
         with level 18"
    );

    let will = explanation(&computation, "class_chassis.wizard.base_save.will");
    assert_eq!(
        will.value, 11,
        "Wizard level 19 good Will (19/2+2) must STAY at 11, an integer-division coincidence \
         with level 18"
    );
}

// ----- The specialist bonus slot count stays flat at level 19 (no new column opens) -----

#[test]
fn wizard_level19_specialist_bonus_slot_stays_flat_at_nine() {
    let input = load(WIZARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(
        slot.value, 9,
        "Wizard level 19 specialist bonus slot count must STAY at 9 — the raw spells-per-day \
         table's level-19 row \"4/4/4/4/4/4/4/4/3/3\" does not open any spell-level column \
         beyond the 9th (already the highest wizard spell level in PF1, first opened at level \
         17): {}",
        slot.detail
    );
}

// ----- Intense Spells' bonus damage stays flat at level 19 (integer-division coincidence) -----

#[test]
fn wizard_level19_intense_spells_bonus_damage_stays_flat() {
    let input = load(WIZARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(
        intense.value, 9,
        "Intense Spells' bonus-damage magnitude (max(19/2, 1)) must STAY at 9 at level 19, an \
         integer-division coincidence with level 18: {}",
        intense.detail
    );
}

// ----- Force Missile, Scribe Scroll, and the specialization choice carry over unchanged -----

#[test]
fn wizard_level19_grants_carry_over_unchanged() {
    let input = load(WIZARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    assert_eq!(
        force_missile.value, 7,
        "Force Missile's uses per day (3 + Intelligence modifier 4) must stay 7 at level 19"
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
fn wizard_level19_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(WIZARD_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.wizard"),
        "level-19 Wizard must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
            && d.claim_blocking),
        "level-19 Wizard must still claim-block on the school-power execution and \
         opposed-school preparation-cost burden: {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"
                && d.claim_blocking),
        "level-19 Wizard must still claim-block on the prepared spellbook posture burden: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 18 truth is unchanged by this widening -----

#[test]
fn wizard_level18_truth_is_unchanged_by_this_slice() {
    let input = load(WIZARD_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.wizard.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "Wizard level 18 base attack bonus must stay 9");

    let slot = explanation(&computation, "class_chassis.wizard.specialist_bonus_slot");
    assert_eq!(slot.value, 9, "Wizard level 18 specialist bonus slot count must stay 9");

    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 9, "Wizard level 18 Intense Spells bonus damage must stay 9");
}

// ----- Negative control: the wizard path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_level19_recognition() {
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
fn multiclass_wizard_level19_is_not_promoted_by_this_slice() {
    let multiclass = WIZARD_LEVEL19_FIXTURE.replace(
        "class_level=class:wizard:19",
        "class_level=class:wizard:19\nclass_level=class:rogue:1",
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

// ----- Control plane: the matrix note names the level-19 widening -----

#[test]
fn matrix_wizard_row_names_level_19_widening() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard progression_and_spell_burden row must exist");

    assert_eq!(wizard.support_state, SupportState::Supported); // Later promoted to Supported/ProductVisible by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(wizard.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        wizard.grounding_ref.contains("sd18_wizard_level19_widening"),
        "wizard row must cite the live SD18 level-19 widening proof surface: {}",
        wizard.grounding_ref
    );
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        note.contains("level 19") || note.contains("level-19"),
        "wizard partial note must name the level-19 widening: {note}"
    );
}
