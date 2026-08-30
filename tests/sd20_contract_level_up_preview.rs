//! SD-20 Epic 2-7 -> boundary-contract wiring project, Cycle 6
//! (`contract:level_up_preview`, per `adaptive-squishing-mccarthy.md`).
//!
//! Per the plan's Q1 design decision, Level-Up (Epic 7) is deliberately
//! NOT folded into `PilotReceipt`: it models a level *transition* (needs
//! `from_level`/`to_level`), not current-state snapshot data. Forcing it
//! into `PilotReceipt` would mean either fabricating transition params
//! for every snapshot consumer or contaminating the contract with
//! transition-only fields. So this cycle lands a standalone function,
//! `contract::compute_level_up_preview(character, from_level, to_level)
//! -> LevelUpPlan`, that coexists with `PilotReceipt` rather than being
//! embedded in it -- a thin pass-through to
//! `level_up::compute_level_up_grants`. No `PilotReceipt` field and no
//! `printed_sheet_cell_map` cell is added this cycle.
//!
//! This file proves parity: `compute_level_up_preview` produces
//! byte-identical output (`LevelUpPlan` derives `PartialEq`) to a direct
//! call to `compute_level_up_grants` for the same inputs. Uses the same
//! Fighter tabletop fixture shape (`class:fighter` at level 1, the same
//! ability scores) and the level 1->2 transition that
//! `tests/sd20_tabletop_readiness_integration.rs`'s
//! `epic_probe_level_up_grants_fighter_level_1_to_2` already exercises
//! directly, cross-checking the same concrete grant values that test
//! pins (BAB +2, Fortitude +3, Bravery +1, a level-2 bonus feat slot, no
//! resource pool, not the level-20 capstone) so the parity assertion is
//! not vacuously true over a default `LevelUpPlan`.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::contract::compute_level_up_preview;
use codex::rules_core::level_up::compute_level_up_grants;

/// Mirrors the Fighter tabletop fixture's `input` section
/// (`tests/fixtures/wire/sd20/human_fighter_level_1_tabletop.json`,
/// consumed via `character_input_from_fixture` in
/// `tests/sd20_tabletop_readiness_integration.rs`): a Human Fighter,
/// `class_levels`/`ability_scores` byte-identical to the fixture, plus
/// the `choice:fighter_bonus_feat_2` selection (`feat:toughness`) the
/// fixture also carries -- without it, Fighter's level-2 bonus-feat-slot
/// grant (`pilot_compute.rs::explain_fighter_class_features`) is gated
/// off and this test's cross-check against
/// `epic_probe_level_up_grants_fighter_level_1_to_2` would not exercise
/// the same grant set that test pins.
fn base_fighter_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_level_up_preview_fighter".to_string()),
        source_package_id: "sd20_contract_level_up_preview_fighter".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            selected_feats: vec![
                "feat:dodge".to_string(),
                "feat:weapon_focus".to_string(),
                "feat:power_attack".to_string(),
                "Dodge".to_string(),
                "Weapon Focus".to_string(),
                "Power Attack".to_string(),
            ],
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: vec![
                SelectedChoice {
                    choice_set_id: "choice:level_1_character_feat".to_string(),
                    selection_id: "feat:power_attack".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:human_bonus_feat".to_string(),
                    selection_id: "feat:dodge".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat".to_string(),
                    selection_id: "feat:weapon_focus:weapon:longsword".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:favored_class_bonus".to_string(),
                    selection_id: "bonus:hp".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_2".to_string(),
                    selection_id: "feat:toughness".to_string(),
                },
            ],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Parity: `compute_level_up_preview` must be byte-identical
/// (`LevelUpPlan: PartialEq`) to a direct `compute_level_up_grants` call
/// for the same character and the same level 1->2 transition -- proving
/// the contract function is a thin pass-through, not a reinterpretation.
#[test]
fn compute_level_up_preview_matches_direct_engine_call_fighter_level_1_to_2() {
    let input = base_fighter_input();

    let preview = compute_level_up_preview(&input, 1, 2);
    let direct = compute_level_up_grants(&input, 1, 2);

    assert_eq!(
        preview, direct,
        "compute_level_up_preview must be byte-identical to a direct \
         compute_level_up_grants(character, from_level, to_level) call"
    );
}

/// Cross-check against the concrete values
/// `epic_probe_level_up_grants_fighter_level_1_to_2`
/// (`tests/sd20_tabletop_readiness_integration.rs`) already pins for the
/// same transition -- proves the parity assertion above is not vacuously
/// true over a default/empty `LevelUpPlan`.
#[test]
fn compute_level_up_preview_carries_real_fighter_level_2_grants() {
    let input = base_fighter_input();

    let preview = compute_level_up_preview(&input, 1, 2);

    let grant = |name_fragment: &str| {
        preview
            .automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", preview.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 2, "fighter level 2 full BAB must be +2");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "fighter:2");

    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", preview.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "fighter level 2 good Fortitude save must be +3");

    let bravery_grant = grant("bravery")
        .unwrap_or_else(|| panic!("expected a bravery grant at level 2: {:?}", preview.automatic_features));
    assert_eq!(bravery_grant.effects[0].value, 1);

    let bonus_feat_grant = grant("level 2 bonus feat").unwrap_or_else(|| {
        panic!("expected a level-2 bonus feat slot grant: {:?}", preview.automatic_features)
    });
    assert_eq!(bonus_feat_grant.effects[0].value, 0);

    assert!(
        preview.resource_pool_change.pools.is_empty(),
        "Fighter has no per-level resource pool"
    );
    assert!(!preview.capstone_threshold, "level 2 is not the level-20 capstone");
}
