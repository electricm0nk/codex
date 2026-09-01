//! SD13-E5 Paladin level-2 lay on hands / divine grace grounding proof.
//!
//! Widens the accepted Paladin level-1 chassis-and-spell-burden separation
//! (`tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs`) to
//! paladin level 2, the PF1 Core Rulebook level gate at which lay on hands and
//! divine grace are actually granted. It proves:
//!
//! - lay on hands is grounded for real at level 2: uses per day = 1/2 paladin
//!   level + Charisma modifier, and the heal amount is stated as a flat,
//!   non-fabricated die-count magnitude (1d6 per two paladin levels), never a
//!   rolled value — mirroring how Smite Evil's damage bonus is a flat scalar,
//!   not a dice-roll execution.
//! - divine grace is grounded for real at level 2: a Charisma-modifier bonus
//!   on all saving throws, applied only if positive — mirroring the "applied
//!   only if positive" idiom already used for Smite Evil's attack bonus.
//! - mercy stays correctly ABSENT at level 2 (a 3rd-level paladin feature),
//!   and the partial-caster spell burden stays claim-blocking and unchanged.
//! - Smite Evil's uses-per-day / attack-bonus / damage-bonus formulas still
//!   scale correctly at level 2 (damage bonus = paladin level = 2).
//!
//! It is intentionally not a healing-execution engine or a
//! saving-throw-resolution engine: it grounds only the flat numeric surface
//! (uses/day, heal-dice count, save-bonus magnitude), exactly like Smite Evil
//! was grounded. It grounds no Paladin level 3+, no mercy selection, no spell
//! slots, no spell source lineage, no spells known or prepared posture, no
//! deity resolution, no domain mechanics, no alignment-driven target
//! resolution, and no healing-resource accounting. It also preserves the
//! accepted Paladin level-1 truth (unchanged), the F6 hybrid baseline, the
//! Ranger negative control, and the Fighter negative control.

use codex::rules_core::character_input::{AcquisitionMode, SpellSelection};
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus,
    PilotBaseChassisComputation,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");

const PALADIN_LEVEL2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level2_sd13_deterministic_input.txt");

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

// The newly grounded level-2 lay on hands / divine grace explanations.
const PALADIN_LAY_ON_HANDS_USES_PER_DAY_ID: &str =
    "class_chassis.paladin.lay_on_hands_uses_per_day";
const PALADIN_LAY_ON_HANDS_HEAL_AMOUNT_ID: &str = "class_chassis.paladin.lay_on_hands_heal_amount";
const PALADIN_DIVINE_GRACE_SAVE_BONUS_ID: &str = "class_chassis.paladin.divine_grace_save_bonus";

// The level-1-only absence records; must not appear at level 2.
const PALADIN_LAY_ON_HANDS_GATE_ID: &str = "class_chassis.paladin.level_gate.lay_on_hands";
const PALADIN_DIVINE_GRACE_GATE_ID: &str = "class_chassis.paladin.level_gate.divine_grace";

// Mercy stays a correctly-absent level-gate record at both level 1 and 2.
const PALADIN_MERCY_GATE_ID: &str = "class_chassis.paladin.level_gate.mercy";

// Smite Evil's grounded numeric explanations, unchanged in shape across levels.
const PALADIN_SMITE_EVIL_USES_PER_DAY_ID: &str = "class_chassis.paladin.smite_evil_uses_per_day";
const PALADIN_SMITE_EVIL_ATTACK_BONUS_ID: &str = "class_chassis.paladin.smite_evil_attack_bonus";
const PALADIN_SMITE_EVIL_DAMAGE_BONUS_ID: &str = "class_chassis.paladin.smite_evil_damage_bonus";

// The remaining claim-blocking spell burden, unchanged by this slice.
const PALADIN_PARTIAL_CASTER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
}

// ----- Lay on hands is grounded for real at level 2 -----

#[test]
fn paladin_level2_grounds_lay_on_hands_uses_per_day_and_heal_amount() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture is a level-2 Human Paladin with Charisma 14 (modifier +2).
    // PF1 Core Rulebook: uses/day = 1/2 paladin level + Charisma modifier
    // = 2/2 + 2 = 3.
    let uses_per_day = explanation(&computation, PALADIN_LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 4,
        "lay on hands uses per day must be 1/2 paladin level + Charisma modifier: {uses_per_day:?}"
    );
    assert!(
        uses_per_day.detail.contains("Charisma modifier"),
        "lay on hands uses-per-day detail must name the Charisma-modifier formula: {}",
        uses_per_day.detail
    );

    // Heal amount is a flat non-fabricated die-count magnitude (1d6 per two
    // paladin levels), never a rolled value: at level 2, 2/2 = 1d6.
    let heal_amount = explanation(&computation, PALADIN_LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 1,
        "lay on hands heal amount must be paladin level / 2 d6 dice: {heal_amount:?}"
    );
    assert!(
        heal_amount.detail.contains("1d6"),
        "lay on hands heal-amount detail must name the 1d6-per-two-levels formula: {}",
        heal_amount.detail
    );
    assert!(
        !heal_amount.detail.to_lowercase().contains("rolled"),
        "lay on hands heal amount must be a flat magnitude, not a rolled value: {}",
        heal_amount.detail
    );
}

// ----- Divine grace is grounded for real at level 2 -----

#[test]
fn paladin_level2_grounds_divine_grace_save_bonus_applied_only_if_positive() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture Charisma modifier is +2, so the save bonus is +2.
    let save_bonus = explanation(&computation, PALADIN_DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "divine grace save bonus must equal the positive Charisma modifier: {save_bonus:?}"
    );
    assert!(
        save_bonus.detail.contains("saving throw"),
        "divine grace detail must name the saving-throw bonus: {}",
        save_bonus.detail
    );
    assert!(
        save_bonus.detail.contains("if positive"),
        "divine grace detail must name the 'applied only if positive' idiom: {}",
        save_bonus.detail
    );
}

// ----- Level-1-only absence records must not leak into the level-2 surface -----

#[test]
fn paladin_level2_retires_lay_on_hands_and_divine_grace_absence_gates() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [PALADIN_LAY_ON_HANDS_GATE_ID, PALADIN_DIVINE_GRACE_GATE_ID] {
        assert!(
            !has_explanation(&computation, id),
            "level-1 absence record '{id}' must not appear once lay on hands / divine grace are \
             grounded for real at level 2"
        );
    }
}

// ----- Mercy stays correctly absent at level 2 (3rd-level paladin feature) -----

#[test]
fn paladin_level2_mercy_remains_a_correct_absence() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let mercy = explanation(&computation, PALADIN_MERCY_GATE_ID);
    assert_eq!(
        mercy.value, 0,
        "mercy must remain a correct absence (value 0) at level 2: {mercy:?}"
    );
    assert!(
        mercy.detail.contains("3rd-level"),
        "mercy gate must name the 3rd-level PF1 CRB grant: {}",
        mercy.detail
    );
    assert!(
        mercy.detail.contains("correctly absent at level 2"),
        "mercy gate must name the correct absence at level 2: {}",
        mercy.detail
    );
}

// ----- Smite Evil scales correctly to level 2 -----

#[test]
fn paladin_level2_smite_evil_scales_correctly() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, PALADIN_SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 1,
        "smite evil uses per day stays 1/day below level 4: {uses_per_day:?}"
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let attack_bonus = explanation(&computation, PALADIN_SMITE_EVIL_ATTACK_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 3,
        "smite evil attack bonus must equal the Charisma modifier (+3 for CHA 14 + 2 Human \
         racial): {attack_bonus:?}"
    );

    let damage_bonus = explanation(&computation, PALADIN_SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 2,
        "smite evil damage bonus must equal paladin level (2 at level 2): {damage_bonus:?}"
    );
}

// ----- Partial-caster spell burden stays claim-blocking and unchanged -----

#[test]
fn paladin_level2_partial_caster_spell_burden_stays_claim_blocking() {
    // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25)
    // PALADIN_PARTIAL_CASTER_ID is no longer unconditional: at level 2 no
    // paladin spell level is accessible yet (spells begin at level 4), so a
    // bare fixture with zero prepared spells has a genuinely valid (empty)
    // posture and the blocker would not fire at all. This test is
    // specifically about the blocker staying claim-blocking, so a genuinely
    // invalid preparation (Bless, a real paladin spell, but not yet
    // accessible at level 2) is added to make it fire for real.
    let mut input = load(PALADIN_LEVEL2_FIXTURE);
    input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Bless".to_owned(),
        source_class_id: "class:paladin".to_owned(),
        acquisition_mode: AcquisitionMode::Prepared,
    });
    let computation = compute_pilot_base_chassis(&input);

    let spell = computation
        .diagnostics
        .iter()
        .find(|d| d.id == PALADIN_PARTIAL_CASTER_ID)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{PALADIN_PARTIAL_CASTER_ID}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        spell.claim_blocking,
        "partial-caster spell burden must remain claim-blocking at level 2: {spell:?}"
    );
}

// ----- The integrated posture stays Blocked at level 2, never fakes Computed -----

#[test]
fn paladin_level2_still_yields_blocked_headless_receipt_and_view_model() {
    let input = load(PALADIN_LEVEL2_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Blocked,
        "grounded lay on hands / divine grace must keep the integrated paladin posture Blocked \
         (the spell burden and the non-Fighter chassis path both still claim-block)"
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked level-2 paladin posture must not emit a computed snapshot"
    );
}

// ----- Sibling preservation: level 1, Ranger, and Fighter must not regress -----

#[test]
fn paladin_level1_absence_gates_remain_unaffected_by_the_level2_widening() {
    let input = load(PALADIN_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Level 1 must keep emitting the absence gates, not the grounded records.
    for id in [PALADIN_LAY_ON_HANDS_GATE_ID, PALADIN_DIVINE_GRACE_GATE_ID] {
        let gate = explanation(&computation, id);
        assert_eq!(
            gate.value, 0,
            "level-1 gate '{id}' must remain a correct absence: {gate:?}"
        );
    }
    for id in [
        PALADIN_LAY_ON_HANDS_USES_PER_DAY_ID,
        PALADIN_LAY_ON_HANDS_HEAL_AMOUNT_ID,
        PALADIN_DIVINE_GRACE_SAVE_BONUS_ID,
    ] {
        assert!(
            !has_explanation(&computation, id),
            "level-1 paladin must not gain the level-2 grounded record '{id}'"
        );
    }
}

#[test]
fn paladin_level2_grounded_records_do_not_leak_onto_ranger_or_fighter() {
    let ranger_input = load(RANGER_FIXTURE);
    let ranger_computation = compute_pilot_base_chassis(&ranger_input);
    let fighter_input = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter_input);

    for id in [
        PALADIN_LAY_ON_HANDS_USES_PER_DAY_ID,
        PALADIN_LAY_ON_HANDS_HEAL_AMOUNT_ID,
        PALADIN_DIVINE_GRACE_SAVE_BONUS_ID,
    ] {
        assert!(
            !has_explanation(&ranger_computation, id),
            "ranger must not gain a Paladin-only level-2 grounded explanation '{id}'"
        );
        assert!(
            !has_explanation(&fighter_computation, id),
            "fighter must not gain a Paladin-only level-2 grounded explanation '{id}'"
        );
        assert!(
            !has_diagnostic(&ranger_computation, id),
            "ranger must not gain a Paladin-only level-2 diagnostic '{id}'"
        );
        assert!(
            !has_diagnostic(&fighter_computation, id),
            "fighter must not gain a Paladin-only level-2 diagnostic '{id}'"
        );
    }
}
