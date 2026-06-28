//! GE06-E2-F1a deterministic pilot input contract load gate.
//!
//! Proves the rules-core loader can represent and validate the accepted GE-06
//! deterministic PF1 Core Rulebook Human Fighter level 1 pilot input contract as a
//! chosen-input record. This asserts represented selections only; it deliberately
//! does not compute or assert any derived Pathfinder values.

use codex::rules_core::character_input::{
    ActiveState, DiagnosticClass, DiagnosticSeverity, load_character_input_fixture,
};

#[test]
fn loads_ge06_deterministic_pilot_input_contract_as_chosen_input() {
    let fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    let result = load_character_input_fixture(fixture);

    assert!(
        result.diagnostics.is_empty(),
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
    let input = result
        .character_input
        .expect("valid GE-06 deterministic fixture should produce a character input record");

    // Identity (case_id is the GE-06 addition over the GE-04 record shape).
    assert_eq!(input.case_id.as_deref(), Some("pf1-crb-human-fighter-level1"));
    assert_eq!(input.source_package_id, "pf1.core_rulebook");
    assert_eq!(input.chosen.race_id, "race:human");
    assert_eq!(input.chosen.class_levels.len(), 1);
    assert_eq!(input.chosen.class_levels[0].class_id, "class:fighter");
    assert_eq!(input.chosen.class_levels[0].level, 1);

    // Ability scores. GE-06 closes CON at 14 (the GE-04 historical fixture keeps 13).
    assert_eq!(input.chosen.ability_scores.strength, 16);
    assert_eq!(input.chosen.ability_scores.dexterity, 14);
    assert_eq!(input.chosen.ability_scores.constitution, 14);
    assert_eq!(input.chosen.ability_scores.intelligence, 10);
    assert_eq!(input.chosen.ability_scores.wisdom, 12);
    assert_eq!(input.chosen.ability_scores.charisma, 8);

    // Selected feats: Power Attack, Dodge, Weapon Focus.
    for feat in ["feat:power_attack", "feat:dodge", "feat:weapon_focus"] {
        assert!(
            input.chosen.selected_feats.iter().any(|f| f == feat),
            "expected selected feat {feat}, got {:?}",
            input.chosen.selected_feats
        );
    }

    // Choice slots: slot identity must not be lost, and the Weapon Focus weapon
    // selection and Human ability-bonus target must survive.
    let selection_for = |set: &str| {
        input
            .chosen
            .selected_choices
            .iter()
            .find(|c| c.choice_set_id == set)
            .map(|c| c.selection_id.as_str())
    };
    assert_eq!(
        selection_for("choice:level_1_character_feat"),
        Some("feat:power_attack")
    );
    assert_eq!(
        selection_for("choice:human_bonus_feat"),
        Some("feat:dodge")
    );
    assert_eq!(
        selection_for("choice:fighter_bonus_feat"),
        Some("feat:weapon_focus:weapon:longsword")
    );
    assert_eq!(
        selection_for("choice:human_ability_bonus"),
        Some("ability:strength")
    );

    // Skill ranks: Climb 1, Intimidate 1, Swim 1 (chosen ranks, not computed modifiers).
    let ranks_for = |skill: &str| {
        input
            .chosen
            .skill_allocations
            .iter()
            .find(|s| s.skill_id == skill)
            .map(|s| s.ranks)
    };
    assert_eq!(ranks_for("skill:climb"), Some(1));
    assert_eq!(ranks_for("skill:intimidate"), Some(1));
    assert_eq!(ranks_for("skill:swim"), Some(1));

    // Equipment and active states must distinguish equipped/active, absent, and
    // selected-but-inactive (Power Attack inactive for baseline outputs).
    let state_for = |item: &str| {
        input
            .chosen
            .equipment_selections
            .iter()
            .find(|e| e.item_id == item)
    };
    assert_eq!(
        state_for("item:chain_shirt").map(|e| e.active_state),
        Some(ActiveState::EquippedActive)
    );
    assert_eq!(
        state_for("item:longsword").map(|e| e.active_state),
        Some(ActiveState::EquippedActive)
    );
    assert_eq!(
        state_for("item:shield").map(|e| e.active_state),
        Some(ActiveState::Absent)
    );
    assert_eq!(
        state_for("power_attack").map(|e| e.active_state),
        Some(ActiveState::SelectedInactive)
    );

    // Backward-compatible boolean stays consistent with the active-state model.
    assert!(state_for("item:longsword").unwrap().equipped_or_active);
    assert!(!state_for("power_attack").unwrap().equipped_or_active);

    // Provenance retains a reference to the GE-06 deterministic input contract.
    assert!(
        input.selection_provenance.iter().any(|p| {
            p.source_ref
                == "programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md"
        }),
        "expected provenance pointing to the GE-06 contract, got {:?}",
        input.selection_provenance
    );
}

#[test]
fn unsupported_equipment_active_state_returns_claim_blocking_diagnostic() {
    // An unrecognized active-state token must be rejected with a structured
    // diagnostic rather than silently coerced to Absent, matching how every other
    // field parser in the loader treats invalid input.
    let result = load_character_input_fixture(
        "case_id=pf1-crb-human-fighter-level1\n\
         source_package_id=pf1.core_rulebook\n\
         race_id=race:human\n\
         class_level=class:fighter:1\n\
         ability=strength:16\n\
         ability=dexterity:14\n\
         ability=constitution:14\n\
         ability=intelligence:10\n\
         ability=wisdom:12\n\
         ability=charisma:8\n\
         equipment=item:longsword:equiped_primary_active\n",
    );

    assert!(result.character_input.is_none());
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        result.diagnostics[0].class,
        DiagnosticClass::InvalidCharacterInput
    );
    assert_eq!(result.diagnostics[0].severity, DiagnosticSeverity::Error);
    assert_eq!(result.diagnostics[0].subject_ref, "equipment_selections");
    assert!(result.diagnostics[0].claim_blocking);
    assert!(
        result.diagnostics[0]
            .message
            .contains("unsupported state")
    );
}
