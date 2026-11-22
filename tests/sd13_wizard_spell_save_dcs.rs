//! v0.6 alpha swarm: Wizard spell-save-DC grounding proof — the fifth spell-DC
//! slice, alongside the existing Paladin/Ranger/Sorcerer/Bard ones
//! (`tests/sd13_{paladin,ranger,sorcerer,bard}_spell_save_dcs.rs`). QA found
//! no `wizard_spell_save_dc` computation anywhere in `pilot_compute.rs`
//! during the v0.6 alpha release swarm's wave-2 survey; backend grounded it
//! in `explain_wizard_level1_prepared_spell_baseline`, verified against both
//! PF1 primary sources and PCGen's real `cr_classes.lst` data
//! (`SPELLSTAT:INT`), which state the rule identically: "The Difficulty
//! Class for a saving throw against a wizard's spell is 10 + the spell
//! level + the wizard's Intelligence modifier" — Intelligence, unlike the
//! Paladin/Sorcerer/Bard's Charisma or the Ranger's Wisdom.
//!
//! One record per ACCESSIBLE spell level (per the already-grounded
//! `WIZARD_<N>TH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` access ladder):
//! `class_chassis.wizard.spell_save_dc.spell_level_<N>` with value
//! `10 + N + Intelligence modifier`. On the deterministic per-level wizard
//! fixtures (Intelligence base 17, +2 Human racial ability-bonus choice
//! targeting Intelligence specifically -> effective 19, modifier +4) that
//! is DC 15/16/17/... for spell levels 1/2/3/... as they become accessible.
//!
//! Unlike the Bard DC slice, this formula is unconditional on school
//! specialization: every wizard, specialist or universalist, has the same
//! access ladder and the same DC formula (verified directly against
//! backend's own inline `wizard_spell_save_dc_tests` module in
//! `pilot_compute.rs`, adopted here into the catalogue with independently
//! chosen fixtures/values rather than transcribed). This grounds only the
//! base DC formula: no saving-throw resolution, no target, no spell
//! selection, and no feat DC modifiers are computed. The
//! `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported`
//! blocker stays claim-blocking at every supported level; unlike Bard's
//! equivalent blocker, it never mentioned spell DCs as a deferred item, so
//! there is nothing for this grounding to stop deferring in its message.
//!
//! Reuses the accepted per-level wizard fixtures (no new fixture) and
//! preserves the Fighter negative control and the multiclass negative
//! control, mirroring the sibling `_spell_save_dcs.rs` files' structure.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::load;

const WIZARD_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt");
const WIZARD_LEVEL3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level3_sd13_deterministic_input.txt");
const WIZARD_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level5_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const DC_PREFIX: &str = "class_chassis.wizard.spell_save_dc.";
const SCHOOL_POWERS_BLOCKER_ID: &str =
    "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported";

fn dc_values(fixture: &str) -> Vec<(String, i16)> {
    let input = load(fixture);
    let computation = compute_pilot_base_chassis(&input);
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(DC_PREFIX))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

fn id(spell_level: u8) -> String {
    format!("{DC_PREFIX}spell_level_{spell_level}")
}

// ----- Level 1: DC 15 for the single accessible spell level -----

#[test]
fn wizard_level1_first_level_dc_is_fifteen() {
    let input = load(WIZARD_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(1))
        .expect("level-1 wizard must carry the 1st-level spell-save-DC record");
    assert_eq!(
        dc.value, 15,
        "DC = 10 + spell level 1 + Intelligence modifier 4 (fixture Intelligence 17 + 2 Human \
         racial, targeting Intelligence specifically) = 15: {}",
        dc.detail
    );
    assert!(
        dc.detail.contains("10 + ") && dc.detail.contains("Intelligence"),
        "the record must state the PF1 DC formula with the Intelligence modifier, not Charisma \
         or Wisdom: {}",
        dc.detail
    );
    assert_eq!(
        dc_values(WIZARD_LEVEL1_FIXTURE).len(),
        1,
        "only the 1st spell level is accessible at level 1"
    );
}

// ----- The DC ladder tracks the access ladder at every step -----

#[test]
fn wizard_spell_save_dcs_track_the_access_ladder() {
    assert_eq!(
        dc_values(WIZARD_LEVEL3_FIXTURE),
        vec![(id(1), 15), (id(2), 16)],
        "level 3 (WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL): spell levels 1-2 \
         accessible, DCs 15/16"
    );
    assert_eq!(
        dc_values(WIZARD_LEVEL5_FIXTURE),
        vec![(id(1), 15), (id(2), 16), (id(3), 17)],
        "level 5 (WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL): spell levels 1-3 \
         accessible, DCs 15/16/17"
    );
}

// ----- The arithmetic is live: a lower Intelligence lowers the DCs -----

#[test]
fn wizard_spell_save_dcs_track_the_intelligence_modifier() {
    let lowered = WIZARD_LEVEL5_FIXTURE.replace("ability=intelligence:17", "ability=intelligence:12");
    let input = load(&lowered);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level DC record must exist at level 5");
    // Lowered Intelligence of 12 still receives the +2 Human racial ability-bonus
    // choice before the modifier is derived (12 + 2 = 14, modifier +2), so the DC
    // is 10 + 3 + 2 = 15, not 17.
    assert_eq!(
        dc.value, 15,
        "DC = 10 + spell level 3 + Intelligence modifier 2 (lowered Intelligence 12 + 2 Human \
         racial) = 15 — the formula is live arithmetic over the chosen ability score, not a \
         hardcoded table: {}",
        dc.detail
    );
}

// ----- Base DC formula only; the school-powers blocker persists unaffected -----

#[test]
fn wizard_level5_school_powers_blocker_persists_alongside_the_grounded_dc_arithmetic() {
    let input = load(WIZARD_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc = computation
        .explanations
        .iter()
        .find(|e| e.id == id(3))
        .expect("the 3rd-level DC record must exist at level 5");
    assert!(
        dc.detail.contains("no saving-throw resolution"),
        "the record must state it grounds the base DC formula only: {}",
        dc.detail
    );

    let blocker = computation
        .diagnostics
        .iter()
        .find(|d| d.id == SCHOOL_POWERS_BLOCKER_ID)
        .expect("the school-powers blocker must still exist at level 5");
    assert!(blocker.claim_blocking, "the blocker must stay claim-blocking");
}

// ----- The DC formula is unconditional on school specialization -----

#[test]
fn wizard_spell_save_dc_is_grounded_even_without_the_canonical_specialization_choice() {
    // Strip the canonical wizard_school_specialization / wizard_opposed_schools
    // choices this fixture normally carries -- the specialist bonus slot record
    // must vanish (it's specialization-gated), but the DC record must not.
    let unspecialized = WIZARD_LEVEL1_FIXTURE
        .lines()
        .filter(|line| {
            !line.starts_with("choice=choice:wizard_school_specialization")
                && !line.starts_with("choice=choice:wizard_opposed_schools")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&unspecialized);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.wizard.specialist_bonus_slot"),
        "with the canonical specialization choice stripped, the specialization-gated \
         specialist bonus slot record must be absent: {:?}",
        computation.explanations
    );
    assert!(
        computation.explanations.iter().any(|e| e.id == id(1)),
        "the spell save DC record must be grounded regardless of school specialization: {:?}",
        computation.explanations
    );
}

// ----- Negative control: no leak onto other classes -----

#[test]
fn fighter_does_not_gain_wizard_dc_records() {
    let fighter = load(FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with(DC_PREFIX)),
        "the Fighter chassis must not surface any wizard spell-save-DC record: {:?}",
        computation.explanations
    );
}

// ----- Multiclass Wizard: DC records fire (mirroring the pre-existing SD-24 -----
// ----- precedent for wizard's other standalone explanations), but the      -----
// ----- receipt stays claim-blocked for unrelated reasons                   -----

#[test]
fn multiclass_wizard_still_gains_dc_records_mirroring_the_existing_mix_precedent() {
    // `wizard_level_in_mix` (SD-24 Epic 5) already fires Wizard's standalone
    // class_chassis.wizard.* explanations once ANY supported second class joins
    // the mix (verified directly by running this exact fixture -- initially wrote
    // this test assuming the opposite, absence, and it failed with the DC records
    // genuinely present, so this asserts the real behavior, not a guess). The
    // spell-save-DC records this file grounds share that same gate, so they fire
    // here too, with the same values as the single-class case (the DC formula
    // depends only on Wizard's own sub-level and Intelligence modifier, not on
    // multiclass BAB/save stacking).
    let multiclass = WIZARD_LEVEL5_FIXTURE.replace(
        "class_level=class:wizard:5",
        "class_level=class:wizard:5\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        dc_values(&multiclass),
        vec![(id(1), 15), (id(2), 16), (id(3), 17)],
        "multiclass Wizard 5 / Fighter 1 still grounds the same DC ladder as single-class \
         Wizard 5: {:?}",
        computation.explanations
    );
    // The receipt still stays claim-blocked -- not because the DC records are
    // missing (they're not), but because the school-powers/spellbook-posture
    // burden (SCHOOL_POWERS_BLOCKER_ID) is unrelated to class-mix recognition
    // and persists regardless.
    let blocker = computation
        .diagnostics
        .iter()
        .find(|d| d.id == SCHOOL_POWERS_BLOCKER_ID)
        .expect("the school-powers blocker must still exist for a multiclass Wizard");
    assert!(blocker.claim_blocking, "the blocker must stay claim-blocking");
}
