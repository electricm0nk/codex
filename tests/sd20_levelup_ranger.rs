//! SD-20 Epic 7 (Level Up grant model): Ranger — eighth core class per
//! Step 2's stated order (`scope-draft.md` §1.7, `technical-design.md`
//! §2.6), landed after barbarian, bard, cleric, druid, fighter, monk, and
//! paladin.
//!
//! Ranger's `class_tables::CLASS_META` row (`good_saves { fortitude: true,
//! reflex: true, will: false }`, `bab: Full`) was spot-checked against
//! `pilot_compute.rs`'s own already-grounded, already-primary-source-
//! verified Ranger formulas
//! (`explain_ranger_level1_chassis_and_class_feature_separation`: full BAB
//! `classlevel`, good Fortitude/Reflex `classlevel/2+2`, poor Will
//! `classlevel/3`, verified against the raw PF1 Core Rulebook Ranger class
//! table level 1-5 rows) before composing with it — CONFIRMED CORRECT, not
//! a second latent `good_saves` defect like the now-fixed Cleric/Druid row
//! (`28b0e88`). `class_tables()` is safe to compose with for Ranger.
//!
//! Expected level 1 -> 2 grants, cross-checked directly against
//! `pilot_compute.rs`'s own grounded Ranger formulas:
//! - base attack bonus 1 -> 2 (full BAB, `class_tables.rs`)
//! - Fortitude save 2 -> 3 and Reflex save 2 -> 3 (good saves,
//!   `classlevel/2+2`; Will stays +0, no grant, poor save
//!   `classlevel/3` = 0 at both levels 1 and 2)
//! - Combat Style Feat newly granted (the style choice AND its first
//!   restricted-list bonus feat are granted together at 2nd level per PF1
//!   Core Rulebook — `pilot_compute.rs`'s own doc comment on this exact
//!   point, correcting an earlier mistaken framing)
//! - Track, Favored Enemy, Endurance, Favored Terrain, Hunter's Bond,
//!   Woodland Stride, Swift Tracker, Quarry, Camouflage, and Master Hunter
//!   must NOT produce spurious grants at this transition: Track's flat
//!   `max(level/2, 1)` formula stays 1 at both levels 1 and 2; Favored
//!   Enemy's flat surface is already granted at level 1 (no change); the
//!   rest are still below their own PF1 Core Rulebook level gates (3rd,
//!   3rd, 4th, 7th, 8th, 11th, 12th, and 20th level respectively).
//!
//! Expected level 19 -> 20 grants, cross-checked directly against
//! `pilot_compute.rs`'s own grounded Ranger formulas:
//! - base attack bonus 19 -> 20 (full BAB)
//! - Fortitude/Reflex save 11 -> 12 (good saves, `20/2+2=12`)
//! - Track's flat Survival bonus 9 -> 10 (`max(level/2, 1)`)
//! - Master Hunter newly granted (the Ranger capstone class feature)
//! - the FIFTH Favored Enemy selection newly granted (PF1 Core Rulebook's
//!   final 20th-level Favored Enemy interval)
//! - Swift Tracker, Woodland Stride, Hunter's Bond, Evasion, Improved
//!   Evasion, Favored Terrain, Quarry/Improved Quarry, Camouflage, and
//!   Hide in Plain Sight all stay unchanged 19 -> 20 (all already granted
//!   at or before level 19; must NOT produce spurious grants) — proving
//!   the diff does not over-fire on every level-gated feature.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::{compute_level_up_grants, PickCategory};

fn human_ranger_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_ranger".to_string()),
        source_package_id: "sd20_levelup_ranger".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:ranger".to_string(),
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
            // `tests/fixtures/rules_core/pf1_human_ranger_level20_sd18_fifth_favored_enemy_and_master_hunter_deterministic_input.txt`
            // (every ranger favored-enemy/favored-terrain/combat-style/
            // hunter's-bond/quarry slot through level 20), so every
            // explanation seam this module reads is exercised, not just
            // the level-2 subset — the level-gate checks inside
            // `explain_ranger_level1_chassis_and_class_feature_separation`
            // themselves bound which of these fire at a given probed
            // level.
            selected_choices: vec![
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_enemy".to_string(),
                    selection_id: "enemy:humanoid_orc".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_enemy_2".to_string(),
                    selection_id: "enemy:undead".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_enemy_bonus_increase_target".to_string(),
                    selection_id: "enemy:first".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_enemy_4".to_string(),
                    selection_id: "enemy:construct".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_enemy_bonus_increase_target_3".to_string(),
                    selection_id: "enemy:fourth".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_enemy_5".to_string(),
                    selection_id: "enemy:dragon".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_enemy_bonus_increase_target_4".to_string(),
                    selection_id: "enemy:second".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_combat_style".to_string(),
                    selection_id: "style:archery".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_combat_style_bonus_feat".to_string(),
                    selection_id: "feat:point_blank_shot".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_combat_style_bonus_feat_2".to_string(),
                    selection_id: "feat:manyshot".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_combat_style_bonus_feat_3".to_string(),
                    selection_id: "feat:shot_on_the_run".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_combat_style_bonus_feat_4".to_string(),
                    selection_id: "feat:improved_precise_shot".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_combat_style_bonus_feat_5".to_string(),
                    selection_id: "feat:improved_precise_shot".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_terrain".to_string(),
                    selection_id: "terrain:forest".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_terrain_2".to_string(),
                    selection_id: "terrain:mountain".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_terrain_bonus_increase_target".to_string(),
                    selection_id: "terrain:first".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_terrain_3".to_string(),
                    selection_id: "terrain:swamp".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_terrain_bonus_increase_target_2".to_string(),
                    selection_id: "terrain:first".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_terrain_4".to_string(),
                    selection_id: "terrain:urban".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_favored_terrain_bonus_increase_target_3".to_string(),
                    selection_id: "terrain:first".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_hunters_bond".to_string(),
                    selection_id: "form:bond".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:ranger_quarry_target".to_string(),
                    selection_id: "enemy:humanoid_orc".to_string(),
                },
            ],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn ranger_level_1_to_2_grants_bab_rise_saves_rise_and_combat_style() {
    let character = human_ranger_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects.len(), 1);
    assert_eq!(bab_grant.effects[0].value, 2, "ranger level 2 full BAB must be +2");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "ranger:2");
    assert_eq!(bab_grant.source_table.column_key, "base_attack_bonus");

    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "ranger level 2 good Fortitude save must be +3");

    let ref_grant = grant("ref_save")
        .unwrap_or_else(|| panic!("expected a ref_save grant: {:?}", plan.automatic_features));
    assert_eq!(ref_grant.effects[0].value, 3, "ranger level 2 good Reflex save must be +3");

    // Will is a poor save (classlevel/3 = 0 at both level 1 and level 2) —
    // must NOT produce a grant.
    assert!(grant("will_save").is_none(), "will save must not change 1 -> 2");

    let combat_style_grant = grant("combat style choice").unwrap_or_else(|| {
        panic!(
            "expected a combat style choice grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(
        combat_style_grant.effects[0].value, 0,
        "the combat style choice seam grounds the choice slot only, contributing no mechanical value"
    );
    assert_eq!(
        combat_style_grant.source_table.table,
        "pilot_compute::explain_ranger_level1_chassis_and_class_feature_separation"
    );

    let combat_style_feat_grant = grant("combat style bonus feat choice").unwrap_or_else(|| {
        panic!(
            "expected a combat style bonus feat choice grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(combat_style_feat_grant.effects[0].value, 0);

    // Not yet granted at level 2 — must NOT produce a spurious grant.
    assert!(grant("track").is_none(), "track stays max(level/2,1)=1 unchanged 1 -> 2");
    assert!(
        grant("favored enemy").is_none(),
        "favored enemy's flat surface is already granted at level 1, unchanged at level 2"
    );
    assert!(grant("endurance").is_none(), "endurance is a 3rd-level feature, not 2nd");
    assert!(grant("favored terrain").is_none(), "favored terrain is a 3rd-level feature, not 2nd");
    assert!(grant("hunters bond").is_none(), "hunter's bond is a 4th-level feature, not 2nd");
    assert!(grant("woodland stride").is_none(), "woodland stride is a 7th-level feature, not 2nd");
    assert!(grant("swift tracker").is_none(), "swift tracker is an 8th-level feature, not 2nd");
    assert!(grant("quarry").is_none(), "quarry is an 11th-level feature, not 2nd");
    assert!(grant("camouflage").is_none(), "camouflage is a 12th-level feature, not 2nd");
    assert!(grant("master hunter").is_none(), "master hunter is the 20th-level capstone, not 2nd");

    // Ranger has no per-level resource pool on this compute surface.
    assert!(plan.resource_pool_change.pools.is_empty());

    // Not yet a capstone level.
    assert!(!plan.capstone_threshold);

    // No Ranger combat-style-feat/favored-enemy/favored-terrain candidate
    // catalog is composed by this cycle — mirrors barbarian.rs's/fighter.rs's
    // own documented `pick_from_lists` scope note: fabricating a candidate
    // list would be exactly the counterfeit-completion risk `AGENTS.md`
    // rules out.
    assert!(
        plan.pick_from_lists.is_empty(),
        "pick_from_lists must not be fabricated this cycle"
    );
}

#[test]
fn ranger_level_19_to_20_crosses_the_capstone_threshold_with_master_hunter() {
    let character = human_ranger_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is the Ranger's PF1 Core Rulebook capstone (Master Hunter)"
    );

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 20, "ranger level 20 full BAB must be +20");

    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 12, "ranger level 20 good Fortitude (20/2+2) must be +12");

    let ref_grant = grant("ref_save")
        .unwrap_or_else(|| panic!("expected a ref_save grant: {:?}", plan.automatic_features));
    assert_eq!(ref_grant.effects[0].value, 12, "ranger level 20 good Reflex (20/2+2) must be +12");

    let track_grant = grant("track")
        .unwrap_or_else(|| panic!("expected a track grant: {:?}", plan.automatic_features));
    assert_eq!(
        track_grant.effects[0].value, 10,
        "Track's Survival bonus at ranger level 20 (max(20/2,1)) must be +10"
    );

    let master_hunter_grant = grant("master hunter").unwrap_or_else(|| {
        panic!(
            "expected a master hunter grant at the Ranger capstone: {:?}",
            plan.automatic_features
        )
    });
    // SD-32 Epic 1 (compute-library wiring): `human_ranger_input`'s Wisdom 12
    // (modifier +1) now resolves through the corpus's own
    // `BONUS:VAR|MasterHunterDC|10+(MasterHunterLVL/2)+WIS` formula: 10 + 20/2 + 1 = 21.
    assert_eq!(master_hunter_grant.effects[0].value, 21);

    let fifth_favored_enemy_grant = grant("favored enemy 5").unwrap_or_else(|| {
        panic!(
            "expected a fifth favored enemy grant at ranger level 20 (the FINAL 20th-level \
             Favored Enemy interval): {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(fifth_favored_enemy_grant.effects[0].value, 0);

    // Integer-division coincidences / already-granted-earlier features 19 ->
    // 20 — must NOT produce spurious grants.
    assert!(grant("swift tracker").is_none(), "swift tracker carries over unchanged 19 -> 20");
    assert!(grant("woodland stride").is_none(), "woodland stride carries over unchanged 19 -> 20");
    assert!(grant("hunters bond").is_none(), "hunter's bond carries over unchanged 19 -> 20");
    assert!(grant("improved evasion").is_none(), "improved evasion carries over unchanged 19 -> 20");
    assert!(
        grant("favored terrain").is_none(),
        "favored terrain's fourth interval (18th level) already fired before 19 -> 20"
    );
    assert!(
        grant("improved quarry").is_none(),
        "improved quarry (19th level) is already granted at from_level 19, unchanged at 20"
    );
    assert!(grant("camouflage").is_none(), "camouflage carries over unchanged 19 -> 20");
    assert!(grant("hide in plain sight").is_none(), "hide in plain sight carries over unchanged 19 -> 20");

    // Ranger has no per-level resource pool on this compute surface.
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(
        plan.pick_from_lists.is_empty(),
        "pick_from_lists must not be fabricated this cycle"
    );
}

#[test]
fn non_ranger_class_returns_an_honestly_empty_plan() {
    let mut character = human_ranger_input(1);
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

    // Sanity: dispatch does not disturb Ranger's own already-landed arm.
    let mut ranger = human_ranger_input(1);
    ranger.chosen.selected_choices = Vec::new();
    let ranger_plan = compute_level_up_grants(&ranger, 1, 2);
    assert!(!ranger_plan.automatic_features.is_empty());

    let _ = PickCategory::Feat;
}
