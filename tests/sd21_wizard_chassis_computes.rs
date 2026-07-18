//! SD-21 Epic 6 (single-class coverage completion, Wizard-first): E6.25 per-class
//! chassis dispatch + E6.26 `compute_wizard_chassis` BAB/save progression proof.
//!
//! Confirmed live 2026-07-18 (pre-launch verification): `compute_pilot_base_chassis`
//! called only `compute_fighter_chassis` for the base-attack-bonus / base-save
//! pillar, so any single-class non-Fighter input (Wizard included) got a fabricated
//! `base_attack_bonus: 0` / `base_saves: BaseSaves::default()` plus a claim-blocking
//! `class_chassis.unsupported` diagnostic, even though Wizard's own
//! `supported_wizard_level` already gated levels 1-20 for its (until now, standalone
//! and NOT integrated) spell-baseline explanation records.
//!
//! This slice refactors `compute_pilot_base_chassis` into a per-class dispatch (single-
//! class only; multiclass stays deferred to Epic 7) and adds `compute_wizard_chassis`,
//! which composes `rules_tables::crb::class_tables::class_tables()`'s already-verified
//! Wizard `BabProgression::Half` / good-Will-only `GoodSaves` row (spot-checked against
//! `pilot_compute.rs`'s own Wizard formulas by `level_up/wizard.rs`'s SD-20 cycle) rather
//! than re-deriving the progression. The acceptance reproducer is a single-class Human
//! Wizard 3: `base_attack_bonus` / `base_saves` / `total_saves` are now real computed
//! numbers, not fabricated zeros, and the universal `class_chassis.unsupported` /
//! `defense.total_save.unsupported` diagnostics no longer fire for it. The receipt's
//! overall `HeadlessReceiptStatus` stays `Blocked` regardless: Wizard's own
//! `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported` and
//! `class_spell.wizard.prepared_spellbook.unsupported` diagnostics (SD13, unaffected by
//! this slice) remain claim-blocking, since this slice grounds no spellbook / spell-slot
//! / school-power math — that is explicitly out of E6.25-E6.27's bounded scope.
//!
//! This slice does NOT touch `compute_combat_baseline` or
//! `compute_selected_skill_modifiers`, which independently gate on
//! `supported_fighter_level` too; those stay claim-blocked for Wizard and are a
//! remaining Epic 6 sub-feature, not this cycle's.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    BaseSaves, ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};

const WIZARD_LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level3_sd13_deterministic_input.txt");
const FIGHTER_LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
}

#[test]
fn wizard_level3_gets_real_base_attack_bonus_and_base_saves() {
    let input = load(WIZARD_LEVEL_3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Wizard class table, level 3: 1/2 BAB (3 / 2 = 1), poor
    // Fortitude/Reflex (3 / 3 = 1), good Will (3 / 2 + 2 = 3).
    assert_eq!(
        computation.base_attack_bonus, 1,
        "Wizard level 3 base attack bonus must be genuinely computed, not fabricated: {computation:?}"
    );
    assert_eq!(
        computation.base_saves,
        BaseSaves {
            fortitude: 1,
            reflex: 1,
            will: 3,
        },
        "Wizard level 3 base saves must be genuinely computed, not fabricated"
    );

    let base_attack = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(base_attack.value, 1);
    let fortitude = explanation(&computation, "class_chassis.base_save.fortitude");
    assert_eq!(fortitude.value, 1);
    let reflex = explanation(&computation, "class_chassis.base_save.reflex");
    assert_eq!(reflex.value, 1);
    let will = explanation(&computation, "class_chassis.base_save.will");
    assert_eq!(will.value, 3);
}

#[test]
fn wizard_level3_no_longer_trips_the_universal_class_chassis_unsupported_diagnostic() {
    let input = load(WIZARD_LEVEL_3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(&computation, "class_chassis.unsupported"),
        "a single-class Human Wizard 3 is now a dispatch-supported chassis: {:?}",
        computation.diagnostics
    );
}

#[test]
fn wizard_level3_total_saves_fold_in_ability_modifiers_for_real() {
    let input = load(WIZARD_LEVEL_3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture ability scores: STR 8 (-1), DEX 14 (+2), CON 13 (+1), INT 17 (+3),
    // WIS 12 (+1), CHA 10 (+0). Total save = base save + matching ability modifier.
    assert_eq!(
        computation.total_saves,
        BaseSaves {
            fortitude: 2,
            reflex: 3,
            will: 4,
        },
        "Wizard level 3 total saves must fold in ability modifiers for real: {computation:?}"
    );
    assert!(
        !has_diagnostic(&computation, "defense.total_save.unsupported"),
        "total saves are now supported for a single-class Wizard: {:?}",
        computation.diagnostics
    );
}

#[test]
fn wizard_level3_still_stays_blocked_on_its_own_spell_burdens() {
    let input = load(WIZARD_LEVEL_3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // This slice grounds no spellbook / spell-slot / school-power math: Wizard's own
    // SD13 burdens remain exactly as claim-blocking as before.
    for id in [
        "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported",
        "class_spell.wizard.prepared_spellbook.unsupported",
    ] {
        let diag = computation
            .diagnostics
            .iter()
            .find(|d| d.id == id)
            .unwrap_or_else(|| panic!("expected diagnostic '{id}' to still fire"));
        assert!(diag.claim_blocking, "'{id}' must remain claim_blocking");
    }
}

#[test]
fn fighter_chassis_dispatch_is_unaffected() {
    let input = load(FIGHTER_LEVEL_1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(&computation, "class_chassis.unsupported"),
        "Fighter must remain a genuinely supported dispatch target"
    );
    let base_attack = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(
        base_attack.value, computation.base_attack_bonus,
        "the dispatch refactor must not change Fighter's own chassis result"
    );
}

#[test]
fn multiclass_inputs_are_now_dispatch_supported_by_epic_7() {
    // Superseded, not violated, by SD-21 Epic 7's E7.28 multiclass dispatch
    // extension (`compute_multiclass_base_chassis`): this test used to pin the
    // pre-Epic-7 deferral of length-2+ `class_levels` inputs as expected
    // behavior. A Wizard 3 / Fighter 1 mix is now genuinely computed, mirroring
    // this same file's own `wizard_level3_gets_real_base_attack_bonus_and_base_saves`
    // level-widening negative-control idiom.
    let mut input = load(WIZARD_LEVEL_3_FIXTURE);
    let mut second = input.chosen.class_levels[0].clone();
    second.class_id = "class:fighter".to_string();
    second.level = 1;
    input.chosen.class_levels.push(second);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_diagnostic(&computation, "class_chassis.unsupported"),
        "a Wizard 3 / Fighter 1 multiclass mix is now a dispatch-supported chassis \
         (SD-21 E7.28): {:?}",
        computation.diagnostics
    );
    // Wizard 3 (Half BAB): 3/2 = 1. Fighter 1 (Full BAB): 1. Summed: 2.
    assert_eq!(computation.base_attack_bonus, 2);
    // Fort: Wizard3 poor (3/3=1) + Fighter1 good (1/2+2=2) = 3.
    // Ref: Wizard3 poor (3/3=1) + Fighter1 poor (1/3=0) = 1.
    // Will: Wizard3 good (3/2+2=3) + Fighter1 poor (1/3=0) = 3.
    assert_eq!(
        computation.base_saves,
        BaseSaves {
            fortitude: 3,
            reflex: 1,
            will: 3,
        }
    );
}
