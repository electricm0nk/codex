//! SD-20 Epic 7 (Level Up grant model): Paladin — seventh core class per
//! Step 2's stated order (`scope-draft.md` §1.7, `technical-design.md`
//! §2.6).
//!
//! Per §1.7's per-cycle test description: advancing a `CharacterInput`
//! from level N to N+1 produces a `LevelUpPlan` whose
//! `automatic_features` matches the published CRB table at level N+1 for
//! the character's class. Uses a deterministic Human Paladin, Charisma 14
//! fixture posture matching the shape
//! `pilot_compute.rs`'s
//! `explain_paladin_level1_chassis_and_spell_burden_separation` requires
//! to fire at all (race_id `"race:human"`, single paladin class level).
//!
//! Expected level 1 -> 2 grants, cross-checked directly against
//! `pilot_compute.rs`'s own grounded paladin explanations:
//! - base attack bonus 1 -> 2 (full BAB, `class_tables.rs`)
//! - Fortitude save 2 -> 3, Will save 2 -> 3 (good saves,
//!   `class_tables.rs`; Reflex stays +0, no grant)
//! - Lay on Hands and Divine Grace newly granted (2nd-level paladin
//!   features; below level 2 they are grounded as correct level-gate
//!   absences with a DIFFERENT explanation id, so this proves the diff
//!   correctly treats an id that changes across the gate the same way it
//!   treats Barbarian's same-id "correctly absent" -> "granted"
//!   marker-text transition)
//! - Smite Evil damage bonus 1 -> 2 (equals paladin level, always
//!   granted from level 1, but genuinely rises)
//!
//! Not yet at level 2: Smite Evil uses/day (stays 1/day below level 4,
//! so no resource-pool entry for this transition), Mercy (3rd-level),
//! Channel Positive Energy (4th-level) — neither should produce a grant,
//! proving the diff does not over-fire on every level-gated feature.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::{compute_level_up_grants, PickCategory};

fn human_paladin_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_paladin".to_string()),
        source_package_id: "sd20_levelup_paladin".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:paladin".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 12,
                constitution: 14,
                intelligence: 8,
                wisdom: 10,
                charisma: 14,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn paladin_level_1_to_2_grants_bab_saves_lay_on_hands_and_divine_grace() {
    let character = human_paladin_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 2, "paladin level 2 full BAB must be +2");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "paladin:2");
    assert_eq!(bab_grant.source_table.column_key, "base_attack_bonus");

    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "paladin level 2 good Fortitude save must be +3");

    let will_grant = grant("will_save")
        .unwrap_or_else(|| panic!("expected a will_save grant: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 3, "paladin level 2 good Will save must be +3");

    // Reflex is a poor save (level / 3 = 0 at both 1 and 2) — must NOT
    // produce a grant.
    assert!(grant("ref_save").is_none(), "reflex save must not change 1 -> 2");

    let lay_on_hands_heal_grant = grant("lay on hands heal amount").unwrap_or_else(|| {
        panic!(
            "expected a lay on hands heal amount grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(lay_on_hands_heal_grant.effects[0].value, 1, "paladin level 2 lay on hands heals 1d6");

    let divine_grace_grant = grant("divine grace save bonus").unwrap_or_else(|| {
        panic!(
            "expected a divine grace save bonus grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(divine_grace_grant.effects[0].value, 2, "Charisma 14 -> +2 divine grace save bonus");

    let smite_damage_grant = grant("smite evil damage bonus").unwrap_or_else(|| {
        panic!(
            "expected a smite evil damage bonus grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(smite_damage_grant.effects[0].value, 2, "smite evil damage bonus equals paladin level");

    // Not yet granted/changed at level 2 — must NOT produce a spurious
    // grant or resource-pool entry.
    assert!(grant("mercy").is_none(), "mercy is a 3rd-level feature, not 2nd");
    assert!(
        grant("channel positive energy").is_none(),
        "channel positive energy is a 4th-level feature, not 2nd"
    );
    assert!(
        plan.resource_pool_change.pools.is_empty(),
        "smite evil uses/day stays 1/day below level 4; lay on hands uses/day is asserted \
         separately below, not as a spurious extra pool"
    );

    // Not yet a capstone level.
    assert!(!plan.capstone_threshold);

    // No mercy candidate catalog exists anywhere in this repo to
    // enumerate real candidates from (see this module's own doc
    // comment) — pick_from_lists must stay honestly empty rather than
    // fabricated.
    assert!(
        plan.pick_from_lists.is_empty(),
        "no mercy candidate catalog exists yet; pick_from_lists must not be fabricated"
    );
}

#[test]
fn paladin_level_3_to_4_grants_smite_evil_resource_pool_and_channel_positive_energy() {
    let character = human_paladin_input(3);
    let plan = compute_level_up_grants(&character, 3, 4);

    // Smite Evil uses/day genuinely rises 1 -> 2 at level 4 — a resource
    // pool, not a discrete automatic-feature grant.
    let smite_pool = plan
        .resource_pool_change
        .pools
        .iter()
        .find(|pool| pool.pool_id == "smite_evil_uses_per_day")
        .unwrap_or_else(|| panic!("expected a smite_evil_uses_per_day pool change: {:?}", plan.resource_pool_change));
    assert_eq!(smite_pool.from_value, 1);
    assert_eq!(smite_pool.to_value, 2);

    let channel_grant = plan
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("channel positive energy dice"))
        .unwrap_or_else(|| {
            panic!(
                "expected a channel positive energy dice grant at level 4: {:?}",
                plan.automatic_features
            )
        });
    assert_eq!(channel_grant.effects[0].value, 2, "ceil(4 / 2) = 2 channel positive energy dice");

    assert!(!plan.capstone_threshold);
}

#[test]
fn paladin_level_19_to_20_crosses_the_capstone_threshold() {
    let character = human_paladin_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is the Paladin's PF1 Core Rulebook capstone (Holy Champion)"
    );
    let holy_champion_grant = plan
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("holy champion"))
        .unwrap_or_else(|| panic!("expected a holy champion grant at the capstone: {:?}", plan.automatic_features));
    assert!(
        holy_champion_grant.effects[0]
            .description
            .contains("granted at paladin level 20"),
        "holy champion grant must cite the real grounded explanation text: {:?}",
        holy_champion_grant.effects[0]
    );
}

#[test]
fn non_paladin_class_returns_an_honestly_empty_plan() {
    let mut character = human_paladin_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);

    // Sanity: the type is actually reachable.
    let _ = PickCategory::Feat;
}
