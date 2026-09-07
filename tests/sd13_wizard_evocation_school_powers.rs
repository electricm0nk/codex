//! SD13-E5 Wizard Evocation school-power grounding proof (Intense Spells bonus-damage
//! magnitude, Force Missile uses-per-day pool), narrowing the
//! `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported` blocker.
//!
//! The pre-existing blocker message already named two specific Evocation school
//! powers by name ("intense spells, and the force missile pool of 3 + Int-mod uses
//! per day") without either claim having been independently verified. Per the loop's
//! standing "verify before coding" discipline — which has already caught wrong
//! inherited/briefed rules text four separate times this session (Paladin mercy's
//! level gate, Ranger combat style's level gate, Bard Fascinate's affected-creature
//! formula, and the Sorcerer bloodline class-skill grant) — both claims were checked
//! against the legacy Paizo PRD mirror (`legacy.aonprd.com`) before writing any code:
//!
//! - Intense Spells (Su): "Whenever you cast an evocation spell that deals hit point
//!   damage, add 1/2 your wizard level to the damage (minimum +1)." A real, 1st-level
//!   Evocation school power. Its bonus-damage magnitude is a flat, non-dice formula
//!   (half wizard level, minimum 1), so it grounds for real, mirroring the Cleric
//!   Touch of Good sacred-bonus idiom.
//! - Force Missile (Sp): "As a standard action you can unleash a force missile that
//!   automatically strikes a foe, as magic missile. The force missile deals 1d4
//!   points of damage plus the damage from your intense spells evocation power...
//!   You can use this ability a number of times per day equal to 3 + your
//!   Intelligence modifier." Also a real, 1st-level Evocation school power — checked
//!   with deliberate skepticism (a name that could plausibly have been confused with
//!   non-core material) but confirmed genuinely core, with exactly the "3 + Int-mod"
//!   pool the pre-existing blocker text already claimed. Only the flat uses-per-day
//!   count is a non-dice formula; the 1d4 damage roll and the automatic-hit casting
//!   execution are not flat, so they stay unproven.
//!
//! Both prior claims turned out correct on independent verification (unlike the
//! Paladin/Ranger/Bard/Sorcerer corrections), so this slice grounds two new flat
//! numeric magnitudes rather than correcting wrong wording. No evocation
//! spell-damage-application engine, no force-missile casting-execution engine, no
//! automatic-hit targeting resolution, and no action-economy or per-use consumption
//! tracking is implemented for either power. The opposed-school two-slot preparation
//! cost and the prepared spellbook / spells-prepared / spell-slot posture burden
//! remain untouched and out of scope for this slice.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};
mod common;
use common::{load, explanation, has_explanation};

const WIZARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt");

const INTENSE_SPELLS_BONUS_DAMAGE_ID: &str = "class_chassis.wizard.intense_bonus_damage";
const FORCE_MISSILE_USES_PER_DAY_ID: &str = "class_chassis.wizard.force_missile_uses_per_day";
const SCHOOL_POWERS_BLOCKER_ID: &str =
    "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported";
const PREPARED_BLOCKER_ID: &str = "class_spell.wizard.prepared_spellbook.unsupported";

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

// ----- Grounded for real: Intense Spells bonus-damage magnitude -----

#[test]
fn wizard_level1_grounds_intense_spells_bonus_damage_magnitude() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Evocation School: half wizard level, minimum 1.
    // At level 1: max(1 / 2, 1) = 1.
    let bonus = explanation(&computation, INTENSE_SPELLS_BONUS_DAMAGE_ID);
    assert_eq!(
        bonus.value, 1,
        "wizard level 1 Intense Spells must ground a +1 bonus-damage magnitude (half level, minimum 1)"
    );
    assert!(
        bonus.detail.contains("Intense Spells"),
        "intense spells explanation must name Intense Spells by name: {}",
        bonus.detail
    );
    assert!(
        bonus.detail.contains("spell-damage-application"),
        "intense spells explanation must disclaim any spell-damage-application engine: {}",
        bonus.detail
    );
}

// ----- Grounded for real: Force Missile uses-per-day pool -----

#[test]
fn wizard_level1_grounds_force_missile_uses_per_day() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Evocation School: 3 + Intelligence modifier.
    // Fixture Intelligence is 17 + 2 Human racial (CG-03 fix) -> modifier +4 -> 3 + 4 = 7.
    let uses = explanation(&computation, FORCE_MISSILE_USES_PER_DAY_ID);
    assert_eq!(
        uses.value, 7,
        "wizard level 1 with INT 17 (+4) must ground 3 + 4 = 7 Force Missile uses per day"
    );
    assert!(
        uses.detail.contains("Force Missile") && uses.detail.contains("Intelligence"),
        "force missile explanation must name Force Missile and Intelligence: {}",
        uses.detail
    );
    assert!(
        uses.detail.contains("casts no force missile")
            && uses.detail.contains("1d4 damage roll")
            && uses.detail.contains("action economy"),
        "force missile explanation must disclaim the casting execution, the 1d4 damage roll, \
         and any action-economy tracking: {}",
        uses.detail
    );

    assert_eq!(computation.ability_modifiers.intelligence, 4);
}

// ----- Grounding the school-power magnitudes must not fabricate execution or the opposed-school cost -----

#[test]
fn wizard_level1_school_power_grounding_does_not_fabricate_execution_or_opposed_cost() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No spell-damage-application or force-missile-casting execution explanation may
    // exist under any id: only the two flat magnitudes above are grounded.
    assert!(
        !has_explanation(&computation, "class_chassis.wizard.force_missile_damage"),
        "grounding the uses-per-day pool must not fabricate the 1d4 damage roll"
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("opposed_school") && e.id.contains("cost")),
        "grounding the school powers must not fabricate the opposed-school two-slot preparation cost: {:?}",
        computation.explanations
    );

    // The school-powers blocker narrows but stays claim-blocking (execution machinery
    // and the opposed-school preparation cost remain unproven).
    let school_powers = claim_blocking(&computation, SCHOOL_POWERS_BLOCKER_ID);
    for token in ["intense spells", "force missile", "opposed", "two prepared"] {
        assert!(
            school_powers.message.contains(token),
            "wizard school-powers blocker must still name the '{token}' burden: {}",
            school_powers.message
        );
    }
    assert!(
        school_powers.message.contains("now grounded as flat numbers"),
        "wizard school-powers blocker must acknowledge the newly grounded flat magnitudes: {}",
        school_powers.message
    );

    // The prepared spellbook posture burden is untouched by this slice.
    claim_blocking(&computation, PREPARED_BLOCKER_ID);

    // Exactly two wizard-specific claim-blocking diagnostics remain (school-power
    // execution + opposed-school cost, and prepared spellbook posture) — grounding
    // narrows messages and adds explanation records, not diagnostic count.
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("wizard"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "wizard must still leave exactly two class-specific claim-blocking diagnostics: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: without the canonical specialization choice, nothing is fabricated -----

#[test]
fn wizard_without_canonical_specialization_choices_gains_no_school_power_grounding() {
    let stripped: String = WIZARD_FIXTURE
        .lines()
        .filter(|line| {
            !line.contains("choice:wizard_school_specialization")
                && !line.contains("choice:wizard_opposed_schools")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&stripped);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, INTENSE_SPELLS_BONUS_DAMAGE_ID),
        "a wizard without the canonical specialization choice must not gain the Intense Spells grounding"
    );
    assert!(
        !has_explanation(&computation, FORCE_MISSILE_USES_PER_DAY_ID),
        "a wizard without the canonical specialization choice must not gain the Force Missile grounding"
    );
    claim_blocking(&computation, SCHOOL_POWERS_BLOCKER_ID);
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

// ----- Control plane: the matrix row stays Partial and names the narrowed burden -----

#[test]
fn matrix_wizard_row_names_school_power_magnitudes_grounded_and_execution_unproven() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class Progression
    // Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(wizard.support_state, SupportState::Supported);
    assert!(
        wizard.blocker_or_lossiness_note.contains("Intense Spells")
            && wizard.blocker_or_lossiness_note.contains("Force Missile"),
        "wizard blocker note must name both grounded school powers: {}",
        wizard.blocker_or_lossiness_note
    );
    assert!(
        wizard.blocker_or_lossiness_note.contains("opposed-school"),
        "wizard blocker note must still name the unproven opposed-school preparation cost: {}",
        wizard.blocker_or_lossiness_note
    );
}
