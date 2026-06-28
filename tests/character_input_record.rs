use codex::rules_core::character_input::{
    DiagnosticClass, DiagnosticSeverity, load_character_input_fixture,
};

#[test]
fn loads_pilot_fixture_as_chosen_input_without_deriving_outputs() {
    let fixture =
        include_str!("fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt");

    let result = load_character_input_fixture(fixture);

    assert!(
        result.diagnostics.is_empty(),
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
    let input = result
        .character_input
        .expect("valid pilot fixture should produce a character input record");

    assert_eq!(input.source_package_id, "pf1.core_rulebook");
    assert_eq!(input.chosen.race_id, "race:human");
    assert_eq!(input.chosen.class_levels.len(), 1);
    assert_eq!(input.chosen.class_levels[0].class_id, "class:fighter");
    assert_eq!(input.chosen.class_levels[0].level, 1);
    assert_eq!(input.chosen.ability_scores.strength, 16);
    assert_eq!(input.chosen.ability_scores.dexterity, 14);
    assert_eq!(input.chosen.ability_scores.constitution, 13);
    assert_eq!(input.chosen.ability_scores.intelligence, 10);
    assert_eq!(input.chosen.ability_scores.wisdom, 12);
    assert_eq!(input.chosen.ability_scores.charisma, 8);
    assert_eq!(input.chosen.selected_feats, vec!["feat:power_attack"]);
    assert_eq!(input.chosen.skill_allocations[0].skill_id, "skill:climb");
    assert_eq!(input.chosen.skill_allocations[0].ranks, 1);
    assert_eq!(
        input.chosen.equipment_selections[0].item_id,
        "item:longsword"
    );
    assert!(input.chosen.equipment_selections[0].equipped_or_active);
    assert_eq!(
        input.chosen.selected_choices[0].choice_set_id,
        "choice:bonus_feat"
    );
    assert_eq!(
        input.chosen.selected_choices[0].selection_id,
        "feat:power_attack"
    );
    assert_eq!(
        input.selection_provenance[0].source_ref,
        "fixture:pf1-human-fighter-level1"
    );
}

#[test]
fn invalid_character_input_returns_structured_claim_blocking_diagnostics() {
    let result = load_character_input_fixture(
        "ability=strength:16\n\
         ability=dexterity:14\n\
         ability=constitution:13\n\
         ability=intelligence:10\n\
         ability=wisdom:12\n\
         ability=charisma:8\n",
    );

    assert!(result.character_input.is_none());
    assert_eq!(result.diagnostics.len(), 3);

    let subjects: Vec<&str> = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.subject_ref.as_str())
        .collect();
    assert!(subjects.contains(&"source_package_id"));
    assert!(subjects.contains(&"race_id"));
    assert!(subjects.contains(&"class_levels"));

    for diagnostic in result.diagnostics {
        assert_eq!(diagnostic.class, DiagnosticClass::InvalidCharacterInput);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert!(diagnostic.claim_blocking);
        assert!(diagnostic.message.contains("missing"));
    }
}

#[test]
fn missing_required_ability_score_returns_structured_diagnostic() {
    let result = load_character_input_fixture(
        "source_package_id=pf1.core_rulebook\n\
         race_id=race:human\n\
         class_level=class:fighter:1\n\
         ability=strength:16\n\
         ability=dexterity:14\n\
         ability=constitution:13\n\
         ability=intelligence:10\n\
         ability=wisdom:12\n",
    );

    assert!(result.character_input.is_none());
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        result.diagnostics[0].class,
        DiagnosticClass::InvalidCharacterInput
    );
    assert_eq!(result.diagnostics[0].severity, DiagnosticSeverity::Error);
    assert_eq!(result.diagnostics[0].subject_ref, "ability_scores.charisma");
    assert!(result.diagnostics[0].claim_blocking);
    assert!(
        result.diagnostics[0]
            .message
            .contains("missing character input ability score")
    );
}
