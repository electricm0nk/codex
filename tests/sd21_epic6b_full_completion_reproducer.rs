//! SD-21 Epic 6b's own acceptance reproducer (supersedes Epic 6's original,
//! now-provably-incomplete one, per `epic-breakdown.md`'s "Epic 6b -- Wizard
//! full-completion" section): a single-class Human Wizard 3 with a chosen
//! Evocation specialization, a populated spellbook, and a daily preparation
//! selection reaches `Status: Computed` with zero claim-blocking diagnostics.
//!
//! This combines all three E6b criteria's own grounding on one input:
//! E6b.1's GE-06 combat/skill posture (`tests/sd21_wizard_chassis_computes.rs`),
//! E6b.2's populated spellbook + daily preparation
//! (`tests/sd21_wizard_prepared_spellbook.rs`), and E6b.3's Evocation school
//! powers + opposed-school cost (grounded as part of the same E6b.2 cycle,
//! since the opposed-school slot-cost rule lives in the same slot-consumption
//! check the spellbook grounding already performs).

use codex::rules_core::character_input::{
    AcquisitionMode,
    ActiveState,
    CharacterInput,
    EquipmentSelection,
    SkillAllocation,
    SpellSelection,
};
use codex::rules_core::pilot_compute::{HeadlessReceiptStatus, build_pilot_headless_receipt};
mod common;
use common::load;

const WIZARD_LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level3_sd13_deterministic_input.txt");

fn wizard_spell(spell_id: &str, mode: AcquisitionMode) -> SpellSelection {
    SpellSelection {
        spell_id: spell_id.to_string(),
        source_class_id: "class:wizard".to_string(),
        acquisition_mode: mode,
    }
}

/// The full Epic 6b reproducer: a single-class Human Wizard 3, Evocation
/// specialization (already in the shared SD13 fixture, unmodified), the
/// GE-06 deterministic combat/skill posture (E6b.1), and a populated
/// spellbook with an in-budget daily preparation selection (E6b.2/E6b.3).
fn epic_6b_reproducer_input() -> CharacterInput {
    let mut input = load(WIZARD_LEVEL_3_FIXTURE);

    // E6b.1: the GE-06 deterministic combat/skill posture.
    input.chosen.selected_feats.push("feat:weapon_focus".to_string());
    input.chosen.equipment_selections.push(EquipmentSelection {
        item_id: "item:longsword".to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    });
    input.chosen.equipment_selections.push(EquipmentSelection {
        item_id: "item:chain_shirt".to_string(),
        equipped_or_active: true,
        active_state: ActiveState::EquippedActive,
        applied_modifiers: Vec::new(),
    });
    input.chosen.equipment_selections.push(EquipmentSelection {
        item_id: "item:shield".to_string(),
        equipped_or_active: false,
        active_state: ActiveState::Absent,
        applied_modifiers: Vec::new(),
    });
    input.chosen.equipment_selections.push(EquipmentSelection {
        item_id: "power_attack".to_string(),
        equipped_or_active: false,
        active_state: ActiveState::SelectedInactive,
        applied_modifiers: Vec::new(),
    });
    for skill_id in ["skill:climb", "skill:intimidate", "skill:swim"] {
        input.chosen.skill_allocations.push(SkillAllocation {
            skill_id: skill_id.to_string(),
            ranks: 1,
        });
    }

    // E6b.2/E6b.3: a populated Evocation-only spellbook (no opposed-school
    // spells, so the reproducer definitely stays within the slot budget) with
    // a daily preparation selection.
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
fn epic_6b_reproducer_reaches_status_computed_with_zero_claim_blocking_diagnostics() {
    let input = epic_6b_reproducer_input();
    let receipt = build_pilot_headless_receipt(&input);

    let claim_blocking: Vec<_> = receipt
        .computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .collect();
    assert!(
        claim_blocking.is_empty(),
        "Epic 6b's own reproducer must carry zero claim-blocking diagnostics: {claim_blocking:?}"
    );
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "Epic 6b's own reproducer must reach Status: Computed: {:?}",
        receipt.computation.diagnostics
    );
}
