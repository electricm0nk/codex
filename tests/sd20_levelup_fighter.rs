//! SD-20 Epic 7 (Level Up grant model): Fighter — fifth core class per
//! Step 2's stated order (`scope-draft.md` §1.7, `technical-design.md`
//! §2.6), landed after barbarian, bard, cleric, and druid.
//!
//! Fighter is a strong structural analog to Barbarian's own already-landed
//! cycle: full BAB, and a from-level/to-level diff over
//! `pilot_compute::compute_pilot_base_chassis`'s already-grounded
//! Fighter-specific explanations (Bravery, the ten Bonus Feat slots at
//! levels 1/2/4/6/8/10/12/14/16/18/20, Armor Training, Weapon Training,
//! Armor Mastery, Weapon Mastery) composed with
//! `rules_tables::crb::class_tables::class_tables()`'s class-generic
//! BAB/save progression (Fighter's `CLASS_META` row was spot-checked
//! against the PF1 Core Rulebook before use: good Fortitude only, full
//! BAB — confirmed correct, unlike the now-fixed Cleric/Druid
//! `good_saves.fortitude` defect `28b0e88` corrected).
//!
//! Expected level 1 -> 2 grants, cross-checked directly against
//! `tests/sd13_fighter_level2_level3_progression.rs`'s own independently
//! grounded numbers:
//! - base attack bonus 1 -> 2 (full BAB, `class_tables.rs`)
//! - Fortitude save 2 -> 3 (good save, `class_tables.rs`; Reflex/Will stay
//!   +0, no grant)
//! - Bravery newly granted (+1 Will vs. fear, a 2nd-level Fighter class
//!   feature; unlike Barbarian's Uncanny Dodge, Fighter's own
//!   `explain_fighter_class_features` carries no explicit "correctly
//!   absent" marker text below the level gate — the explanation record is
//!   simply entirely absent, which the identical from-level/to-level diff
//!   algorithm treats identically to Bard's Deadly Performance capstone
//!   case, per the progress doc's own note that no new diff-algorithm
//!   branch was needed there)
//! - the level-2 Bonus Feat slot newly granted (value 0 — the seam names
//!   the slot only, not a general feat-effect engine)
//!
//! Not yet at level 2: Armor Training (3rd-level), Weapon Training
//! (5th-level) — neither should produce a grant for this transition,
//! proving the diff does not over-fire on every level-gated feature.
//!
//! Expected level 19 -> 20 grants, cross-checked directly against
//! `tests/sd18_fighter_level20_widening.rs`'s own independently grounded
//! numbers:
//! - base attack bonus 19 -> 20 (full BAB)
//! - Fortitude save 11 -> 12 (good save, `20/2+2=12`)
//! - Weapon Mastery newly granted (the Fighter capstone class feature,
//!   critical-multiplier-increase magnitude 1)
//! - the level-20 Bonus Feat slot newly granted
//! - Bravery, Armor Training, and Weapon Training all stay unchanged
//!   (integer-division coincidences at levels 19/20 per the sd18 proof) —
//!   must NOT produce spurious grants.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::{compute_level_up_grants, PickCategory};

fn human_fighter_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_fighter".to_string()),
        source_package_id: "sd20_levelup_fighter".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            // Mirrors the canonical choices already proven by
            // `tests/fixtures/rules_core/pf1_human_fighter_level20_sd18_widening_deterministic_input.txt`
            // (every fighter bonus-feat and weapon-training-group slot
            // through level 20), so every explanation seam this module
            // reads is exercised, not just the level-2 subset — the
            // level-gate checks inside `explain_fighter_class_features`
            // themselves bound which of these fire at a given probed
            // level.
            selected_choices: vec![
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat".to_string(),
                    selection_id: "feat:weapon_focus:weapon:longsword".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_2".to_string(),
                    selection_id: "feat:toughness".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_4".to_string(),
                    selection_id: "feat:cleave".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_weapon_training_group".to_string(),
                    selection_id: "group:heavy_blades".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_6".to_string(),
                    selection_id: "feat:combat_reflexes".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_8".to_string(),
                    selection_id: "feat:improved_critical".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_weapon_training_group_2".to_string(),
                    selection_id: "group:bows".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_10".to_string(),
                    selection_id: "feat:greater_weapon_focus".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_12".to_string(),
                    selection_id: "feat:weapon_specialization".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_weapon_training_group_3".to_string(),
                    selection_id: "group:polearms".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_14".to_string(),
                    selection_id: "feat:greater_weapon_specialization".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_16".to_string(),
                    selection_id: "feat:critical_focus".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_weapon_training_group_4".to_string(),
                    selection_id: "group:hammers".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_18".to_string(),
                    selection_id: "feat:staggering_critical".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:fighter_bonus_feat_20".to_string(),
                    selection_id: "feat:critical_mastery".to_string(),
                },
            ],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn fighter_level_1_to_2_grants_bab_rise_fortitude_rise_bravery_and_bonus_feat_slot() {
    let character = human_fighter_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects.len(), 1);
    assert_eq!(bab_grant.effects[0].value, 2, "fighter level 2 full BAB must be +2");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "fighter:2");
    assert_eq!(bab_grant.source_table.column_key, "base_attack_bonus");

    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "fighter level 2 good Fortitude save must be +3");

    // Reflex/Will stay +0 at level 2 (poor save, level / 3 = 0 at both 1
    // and 2) — must NOT produce a grant.
    assert!(grant("ref_save").is_none(), "reflex save must not change 1 -> 2");
    assert!(grant("will_save").is_none(), "will save must not change 1 -> 2");

    let bravery_grant = grant("bravery")
        .unwrap_or_else(|| panic!("expected a bravery grant at level 2: {:?}", plan.automatic_features));
    assert_eq!(
        bravery_grant.effects[0].value, 1,
        "Bravery's Will-vs-fear bonus at level 2 (1 + (2-2)/4) must be +1"
    );
    assert_eq!(
        bravery_grant.source_table.table,
        "pilot_compute::explain_fighter_class_features"
    );

    let bonus_feat_grant = grant("level 2 bonus feat").unwrap_or_else(|| {
        panic!(
            "expected a level_2_bonus_feat slot grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(
        bonus_feat_grant.effects[0].value, 0,
        "the bonus-feat slot seam grounds the choice slot only, contributing no mechanical value"
    );

    // Not yet granted at level 2 — must NOT produce a spurious grant.
    assert!(grant("armor training").is_none(), "armor training is a 3rd-level feature, not 2nd");
    assert!(grant("weapon training").is_none(), "weapon training is a 5th-level feature, not 2nd");

    // Fighter has no per-level resource pool on this compute surface.
    assert!(plan.resource_pool_change.pools.is_empty());

    // Not yet a capstone level.
    assert!(!plan.capstone_threshold);

    // No Fighter bonus-feat candidate catalog is composed by this cycle —
    // see fighter.rs's own module doc comment on why `pick_from_lists`
    // stays honestly empty even though a real feat catalog now exists in
    // `rules_tables::crb::feats` (a documented forward-pointing scope
    // note, not a blocker on this cycle's `LevelUpPlan`).
    assert!(
        plan.pick_from_lists.is_empty(),
        "pick_from_lists must not be fabricated this cycle"
    );
}

#[test]
fn fighter_level_19_to_20_crosses_the_capstone_threshold_with_weapon_mastery() {
    let character = human_fighter_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is the Fighter's PF1 Core Rulebook capstone (Weapon Mastery)"
    );

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 20, "fighter level 20 full BAB must be +20");

    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 12, "fighter level 20 good Fortitude (20/2+2) must be +12");

    let weapon_mastery_grant = grant("weapon mastery").unwrap_or_else(|| {
        panic!(
            "expected a weapon_mastery grant at the Fighter capstone: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(
        weapon_mastery_grant.effects[0].value, 1,
        "Weapon Mastery's critical-multiplier-increase magnitude must be +1"
    );

    let bonus_feat_grant = grant("level 20 bonus feat").unwrap_or_else(|| {
        panic!(
            "expected a level_20_bonus_feat slot grant at the capstone: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(bonus_feat_grant.effects[0].value, 0);

    // Integer-division coincidences at levels 19/20 (per the already-proven
    // `tests/sd18_fighter_level20_widening.rs`) — must NOT produce spurious
    // grants.
    assert!(grant("bravery").is_none(), "bravery stays +5 unchanged from level 19 to 20");
    assert!(grant("armor training").is_none(), "armor training stays rank 4 unchanged 19 -> 20");
    assert!(grant("weapon training").is_none(), "weapon training stays rank 4 unchanged 19 -> 20");
    assert!(grant("armor mastery").is_none(), "armor mastery carries over unchanged 19 -> 20");
}

#[test]
fn non_fighter_class_returns_an_honestly_empty_plan() {
    let mut character = human_fighter_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    character.chosen.selected_choices = Vec::new();
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);

    // Sanity: dispatch does not disturb Barbarian's own already-landed arm.
    let barbarian = human_fighter_input(1);
    let mut barbarian = barbarian;
    barbarian.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:barbarian".to_string(),
        level: 1,
    }];
    barbarian.chosen.selected_choices = Vec::new();
    let barbarian_plan = compute_level_up_grants(&barbarian, 1, 2);
    assert!(!barbarian_plan.automatic_features.is_empty());

    let _ = PickCategory::Feat;
}
