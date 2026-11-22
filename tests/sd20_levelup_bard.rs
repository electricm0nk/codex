//! SD-20 Epic 7 (Level Up grant model): Bard — second core class per
//! Step 2's stated order (barbarian already landed;
//! `scope-draft.md` §1.7, `technical-design.md` §2.6).
//!
//! Mirrors `tests/sd20_levelup_barbarian.rs`'s shape exactly. Uses the
//! identical deterministic Human Bard, ability-score posture already
//! established by `tests/sd13_bard_level2_progression.rs` /
//! `tests/sd18_bard_level20_widening.rs`'s own fixtures (STR 10, DEX 14,
//! CON 12, INT 13, WIS 8, CHA 15 -> +2 Charisma modifier; those tests
//! already pin the exact numbers this file cross-checks against), advancing
//! level 1 -> level 2 and level 19 -> level 20.
//!
//! Expected level-2 grants, cross-checked directly against
//! `pilot_compute.rs`'s own grounded bard explanations
//! (`tests/sd13_bard_level2_progression.rs` establishes these same numbers
//! independently):
//! - base attack bonus 0 -> 1 (3/4 BAB, `class_tables.rs`)
//! - Reflex save 2 -> 3 and Will save 2 -> 3 (both good saves;
//!   Fortitude stays 0, poor save, no grant)
//! - Bardic Performance rounds per day 6 -> 8 (4 + Charisma modifier (+2) +
//!   2 * (level - 1)), landing in `resource_pool_change`, not
//!   `automatic_features`
//! - Fascinate DC 12 -> 13 (10 + level/2 + Charisma modifier), a magnitude
//!   rise, landing as an `automatic_features` grant
//! - Well-Versed newly granted (a 2nd-level Bard class feature, absent
//!   below level 2 by the same "correctly absent" grant-state-change
//!   idiom `barbarian.rs`'s Uncanny Dodge uses)
//!
//! Not yet at level 2 (must NOT produce a grant): Bardic Knowledge (stays
//! at its level-1 floor of 1), Inspire Courage (stays flat +1, first rises
//! at level 5), Fascinate affected-creature count (stays 1).
//!
//! Expected level-20 capstone transition, cross-checked directly against
//! `tests/sd18_bard_level20_widening.rs`'s own grounded numbers:
//! - base attack bonus 14 -> 15, Reflex/Will 11 -> 12 (Fortitude stays 6)
//! - Deadly Performance DC newly granted at 22 (10 + level/2 + Charisma
//!   modifier) — the Bard's 20th-level PF1 Core Rulebook class capstone
//! - `capstone_threshold` true

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::{compute_level_up_grants, PickCategory};

fn human_bard_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_bard".to_string()),
        source_package_id: "sd20_levelup_bard".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:bard".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 14,
                constitution: 12,
                intelligence: 13,
                wisdom: 8,
                charisma: 15,
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
fn bard_level_1_to_2_grants_bab_rise_good_saves_rise_fascinate_dc_rise_and_well_versed() {
    let character = human_bard_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects.len(), 1);
    assert_eq!(bab_grant.effects[0].value, 1, "bard level 2 3/4-BAB must be +1");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "bard:2");
    assert_eq!(bab_grant.source_table.column_key, "base_attack_bonus");

    let ref_grant = grant("ref_save")
        .unwrap_or_else(|| panic!("expected a ref_save grant: {:?}", plan.automatic_features));
    assert_eq!(ref_grant.effects[0].value, 3, "bard level 2 good Reflex save must be +3");

    let will_grant = grant("will_save")
        .unwrap_or_else(|| panic!("expected a will_save grant: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 3, "bard level 2 good Will save must be +3");

    // Fortitude stays +0 at level 2 (poor save, level / 3 = 0 at both 1 and
    // 2) — must NOT produce a grant.
    assert!(grant("fort_save").is_none(), "fortitude save must not change 1 -> 2");

    let well_versed_grant = grant("well versed").unwrap_or_else(|| {
        panic!(
            "expected a well_versed grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert!(
        well_versed_grant.effects[0].value == 4,
        "well versed grant must cite the real grounded +4 magnitude: {:?}",
        well_versed_grant.effects[0]
    );
    assert_eq!(
        well_versed_grant.source_table.table,
        "pilot_compute::explain_bard_level1_spell_baseline"
    );

    let fascinate_dc_grant = grant("fascinate dc").unwrap_or_else(|| {
        panic!(
            "expected a fascinate dc grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(fascinate_dc_grant.effects[0].value, 13, "bard level 2 fascinate DC must rise to 13");

    // Not yet changed at level 2 — must NOT produce a spurious grant.
    assert!(
        grant("bardic knowledge").is_none(),
        "bardic knowledge stays at its level-1 floor of 1 through level 2"
    );
    assert!(
        grant("inspire courage").is_none(),
        "inspire courage stays flat +1 through level 2 (first rises at level 5)"
    );
    assert!(
        grant("fascinate affected creatures").is_none(),
        "fascinate affected-creature count stays 1 through level 2"
    );

    // Bardic performance rounds per day is a resource pool change, not a
    // discrete automatic-feature grant.
    assert!(
        grant("bardic_performance_rounds_per_day").is_none(),
        "bardic performance rounds per day must land in resource_pool_change, not automatic_features"
    );
    assert_eq!(plan.resource_pool_change.pools.len(), 1);
    let rounds_pool = &plan.resource_pool_change.pools[0];
    assert_eq!(rounds_pool.pool_id, "bardic_performance_rounds_per_day");
    assert_eq!(rounds_pool.from_value, 6, "level 1 bardic performance rounds per day: 4 + 2 (CHA mod)");
    assert_eq!(rounds_pool.to_value, 8, "level 2 bardic performance rounds per day: 4 + 2 + 2 * (2 - 1)");

    // Not yet a capstone level.
    assert!(!plan.capstone_threshold);

    // No candidate catalog exists for any open-ended bard pick list yet
    // (mirrors barbarian's Rage Power precedent) — pick_from_lists must
    // stay honestly empty rather than fabricated.
    assert!(
        plan.pick_from_lists.is_empty(),
        "no bard pick-list candidate catalog exists yet; pick_from_lists must not be fabricated"
    );
}

#[test]
fn bard_level_19_to_20_crosses_the_capstone_threshold_and_grants_deadly_performance() {
    let character = human_bard_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is the Bard's PF1 Core Rulebook capstone (Deadly Performance)"
    );

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 15, "bard level 20 3/4-BAB must rise to +15");

    let ref_grant = grant("ref_save")
        .unwrap_or_else(|| panic!("expected a ref_save grant: {:?}", plan.automatic_features));
    assert_eq!(ref_grant.effects[0].value, 12, "bard level 20 good Reflex save must rise to +12");

    let will_grant = grant("will_save")
        .unwrap_or_else(|| panic!("expected a will_save grant: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 12, "bard level 20 good Will save must rise to +12");

    assert!(
        grant("fort_save").is_none(),
        "fortitude stays +6 at level 20, an integer-division coincidence with level 19"
    );

    // Deadly Performance's magnitude rise (a genuinely new grant at the
    // capstone level) must be grounded as a real grant, not silently
    // dropped.
    let deadly_performance_grant = plan
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("deadly performance"))
        .unwrap_or_else(|| {
            panic!(
                "expected a deadly_performance_dc grant at the capstone level: {:?}",
                plan.automatic_features
            )
        });
    assert_eq!(deadly_performance_grant.effects[0].value, 22);
}

#[test]
fn non_bard_class_returns_an_honestly_empty_plan() {
    let mut character = human_bard_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);

    // Sanity: the type is actually reachable from this test file too.
    let _ = PickCategory::Spell;
}
