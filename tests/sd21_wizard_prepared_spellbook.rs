//! SD-21 Epic 6b (E6b.2 prepared spellbook, E6b.3 Arcane School powers +
//! opposed-school cost) acceptance tests.
//!
//! `tests/sd21_wizard_chassis_computes.rs` (Epic 6's own landed cycle) proved
//! a single-class Human Wizard 3 gets a real BAB/base-save chassis but stays
//! claim-blocked forever on two of its own named burdens:
//! `class_spell.wizard.prepared_spellbook.unsupported` and
//! `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported`.
//! This file grounds both, bounded to wizard levels 1-3 and the canonical
//! Evocation specialization (opposed Necromancy/Transmutation) the rest of
//! this file's Wizard support already requires.
//!
//! Spellbook contents ("which spells are recorded") and the daily
//! preparation selection ("which recorded spells are prepared today") are
//! both grounded via the existing SD-19 `spells_selected` field
//! (`AcquisitionMode::Known` = recorded in the spellbook,
//! `AcquisitionMode::Prepared` = prepared today) -- no `CharacterInput`
//! schema change. Since this compute surface has no corpus access (unlike
//! Epic 2's `spellbook::compute_spellbook_coverage`, which resolves a
//! spell's school/level from the real imported PCGen corpus),
//! `spell_id` here uses a bounded, documented, corpus-free convention this
//! test file owns: `<school>.<level>.<name>`, e.g. `evocation.1.magic_missile`
//! -- reusing the real strict-school enum (`Pf1SchoolId`) rather than
//! inventing a parallel school taxonomy. `magic_missile` (Evocation, 1st
//! level) and `cause_fear` (Necromancy, 1st level) were verified against
//! d20pfsrd.com (both) and a second independent Archives of Nethys mirror
//! (magic_missile) before use here.

use codex::rules_core::character_input::{AcquisitionMode, CharacterInput, SpellSelection};
use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
mod common;
use common::{load, explanation};

const WIZARD_LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level3_sd13_deterministic_input.txt");

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
}

fn wizard_spell(spell_id: &str, mode: AcquisitionMode) -> SpellSelection {
    SpellSelection {
        spell_id: spell_id.to_string(),
        source_class_id: "class:wizard".to_string(),
        acquisition_mode: mode,
    }
}

/// A Human Wizard 3 (Evocation, opposed Necromancy/Transmutation) with a
/// populated spellbook (three recorded spells: two 1st-level Evocation
/// spells and a 2nd-level Evocation spell) and a daily preparation selection
/// that stays within the per-level slot budget: level 1 has base 2 +
/// specialist bonus 1 + Intelligence bonus (INT 17, mod +3: (3-1)/4+1 = 1)
/// = 4 slots, of which 1 is prepared; level 2 has base 1 + specialist bonus
/// 1 + Intelligence bonus ((3-2)/4+1 = 1) = 3 slots, of which 1 is prepared.
fn wizard_level_3_with_populated_spellbook_and_daily_prep() -> CharacterInput {
    let mut input = load(WIZARD_LEVEL_3_FIXTURE);
    input.chosen.spells_selected = vec![
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Known),
        wizard_spell("evocation.1.burning_hands", AcquisitionMode::Known),
        wizard_spell("evocation.2.scorching_ray", AcquisitionMode::Known),
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Prepared),
        wizard_spell("evocation.2.scorching_ray", AcquisitionMode::Prepared),
    ];
    input
}

#[test]
fn wizard_level3_with_populated_spellbook_reaches_a_real_prepared_spells_state() {
    let input = wizard_level_3_with_populated_spellbook_and_daily_prep();
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(
            &computation,
            "class_spell.wizard.prepared_spellbook.unsupported"
        ),
        "a Wizard 3 with a populated spellbook and a daily prep selection within the slot \
         budget must clear the prepared-spellbook diagnostic: {:?}",
        computation.diagnostics
    );

    let spellbook = explanation(&computation, "class_spell.wizard.spellbook_contents");
    assert_eq!(spellbook.value, 3, "3 recorded spells: {spellbook:?}");
    assert!(spellbook.detail.contains("magic_missile"));
    assert!(spellbook.detail.contains("burning_hands"));
    assert!(spellbook.detail.contains("scorching_ray"));

    let prepared = explanation(&computation, "class_spell.wizard.daily_preparation");
    assert_eq!(prepared.value, 2, "2 spells prepared today: {prepared:?}");
    assert!(prepared.detail.contains("magic_missile"));
    assert!(prepared.detail.contains("scorching_ray"));

    let level1_total = explanation(
        &computation,
        "class_spell.wizard.total_spells_per_day.spell_level_1",
    );
    assert_eq!(level1_total.value, 4, "{level1_total:?}");
    let level2_total = explanation(
        &computation,
        "class_spell.wizard.total_spells_per_day.spell_level_2",
    );
    assert_eq!(level2_total.value, 3, "{level2_total:?}");
}

#[test]
fn wizard_level3_school_powers_and_opposed_school_cost_diagnostic_clears_too() {
    // SD-21 E6b.3: the school-power / opposed-school-cost diagnostic clears
    // alongside the prepared-spellbook one -- Intense Spells and Force
    // Missile's flat magnitudes were already grounded by Epic 6's own cycle,
    // and the opposed-school double-slot-cost rule is now genuinely applied
    // by the same slot-budget check (proven in isolation by
    // `preparing_an_opposed_school_spell_costs_two_slots` below).
    let input = wizard_level_3_with_populated_spellbook_and_daily_prep();
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(
            &computation,
            "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
        ),
        "{:?}",
        computation.diagnostics
    );
    // Intense Spells / Force Missile stay grounded exactly as Epic 6 left them.
    let intense = explanation(&computation, "class_chassis.wizard.intense_bonus_damage");
    assert_eq!(intense.value, 1);
    let force_missile = explanation(
        &computation,
        "class_chassis.wizard.force_missile_uses_per_day",
    );
    // CG-03 fix: this fixture's Human ability-bonus choice targets Intelligence, so
    // the +2 racial bonus now applies before the modifier is derived (base 17 -> 19,
    // modifier +3 -> +4), raising Force Missile uses/day (3 + Int modifier) from 6 to 7.
    assert_eq!(force_missile.value, 7);
}

#[test]
fn preparing_an_opposed_school_spell_costs_two_slots() {
    // SD-21 E6b.3: an opposed-school spell (`cause_fear`, Necromancy, one of
    // this fixture's two canonical opposed schools) occupies TWO prepared
    // slots, not one (PF1 Core Rulebook arcane school opposition rule). At
    // level 3, level-1 slots total 4 (base 2 + specialist 1 + Int bonus 1);
    // preparing one opposed-school 1st-level spell alone consumes 2 of them,
    // correctly leaving 2, not fabricating a 1-slot cost.
    let mut input = wizard_level_3_with_populated_spellbook_and_daily_prep();
    input.chosen.spells_selected.push(wizard_spell(
        "necromancy.1.cause_fear",
        AcquisitionMode::Known,
    ));
    input.chosen.spells_selected.push(wizard_spell(
        "necromancy.1.cause_fear",
        AcquisitionMode::Prepared,
    ));
    // Remove the other level-1 prepared spell so the arithmetic is legible:
    // level-1 consumption is now exactly the opposed-school spell's cost (2).
    input
        .chosen
        .spells_selected
        .retain(|s| !(s.spell_id == "evocation.1.magic_missile" && s.acquisition_mode == AcquisitionMode::Prepared));
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(
            &computation,
            "class_spell.wizard.prepared_spellbook.unsupported"
        ),
        "2 of 4 level-1 slots consumed by the opposed-school spell must still fit the budget: \
         {:?}",
        computation.diagnostics
    );

    // Now over-prepare: add a second opposed-school spell at level 1, pushing
    // consumption to 2 + 2 = 4 (exactly the budget) -- still fits.
    input.chosen.spells_selected.push(wizard_spell(
        "transmutation.1.enlarge_person",
        AcquisitionMode::Known,
    ));
    input.chosen.spells_selected.push(wizard_spell(
        "transmutation.1.enlarge_person",
        AcquisitionMode::Prepared,
    ));
    let exact_budget_computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_diagnostic(
            &exact_budget_computation,
            "class_spell.wizard.prepared_spellbook.unsupported"
        ),
        "2 opposed-school spells costing 2 slots each = 4, exactly the level-1 budget: {:?}",
        exact_budget_computation.diagnostics
    );

    // One more opposed-school spell tips it over (4 + 2 = 6 > 4 available):
    // the diagnostic must fire again -- the cost is genuinely enforced, not
    // a cosmetic pass-through.
    input.chosen.spells_selected.push(wizard_spell(
        "necromancy.1.ray_of_enfeeblement",
        AcquisitionMode::Known,
    ));
    input.chosen.spells_selected.push(wizard_spell(
        "necromancy.1.ray_of_enfeeblement",
        AcquisitionMode::Prepared,
    ));
    let over_budget_computation = compute_pilot_base_chassis(&input);
    assert!(
        has_diagnostic(
            &over_budget_computation,
            "class_spell.wizard.prepared_spellbook.unsupported"
        ),
        "over-preparing opposed-school spells beyond the level-1 budget must re-block: {:?}",
        over_budget_computation.diagnostics
    );
}

#[test]
fn preparing_a_spell_not_recorded_in_the_spellbook_stays_blocked() {
    // The prepared-vs-known distinction (SD-21 E6b.2's own named burden): a
    // wizard may only prepare from what she has recorded.
    let mut input = wizard_level_3_with_populated_spellbook_and_daily_prep();
    input.chosen.spells_selected.push(wizard_spell(
        "evocation.1.shocking_grasp",
        AcquisitionMode::Prepared,
    ));
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_diagnostic(
            &computation,
            "class_spell.wizard.prepared_spellbook.unsupported"
        ),
        "preparing an unrecorded spell must stay blocked: {:?}",
        computation.diagnostics
    );
}

#[test]
fn an_empty_spellbook_stays_blocked_exactly_as_before() {
    // Negative control / regression: the unmodified fixture (no
    // `spells_selected` at all) is untouched by this grounding.
    let input = load(WIZARD_LEVEL_3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_diagnostic(
            &computation,
            "class_spell.wizard.prepared_spellbook.unsupported"
        ),
        "an empty spellbook must stay blocked: {:?}",
        computation.diagnostics
    );
    assert!(
        has_diagnostic(
            &computation,
            "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
        ),
        "{:?}",
        computation.diagnostics
    );
}
