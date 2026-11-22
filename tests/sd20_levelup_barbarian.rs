//! SD-20 Epic 7 (Level Up grant model): Barbarian — first core class per
//! Step 2's stated order (`scope-draft.md` §1.7, `technical-design.md`
//! §2.6).
//!
//! Per §1.7's per-cycle test description: advancing a `CharacterInput`
//! from level N to N+1 produces a `LevelUpPlan` whose
//! `automatic_features` matches the published CRB table at level N+1 for
//! the character's class. Uses the identical Human Barbarian, Con 16
//! deterministic fixture posture `tests/sd13_barbarian_level1_chassis_baseline.rs`
//! already established (race_id `"race:human"`, `ability=constitution:16`
//! — the exact input shape `pilot_compute.rs`'s
//! `explain_barbarian_level1_chassis` requires to fire at all), advancing
//! level 1 -> level 2.
//!
//! Expected level-2 grants, cross-checked directly against
//! `pilot_compute.rs`'s own grounded barbarian explanations
//! (`tests/sd13_barbarian_level1_chassis_baseline.rs` and its level-2
//! sibling establish these same numbers independently):
//! - base attack bonus 1 -> 2 (full BAB, `class_tables.rs`)
//! - Fortitude save 2 -> 3 (good save, `class_tables.rs`; Reflex/Will
//!   stay +0, no grant)
//! - Uncanny Dodge newly granted (a 2nd-level Barbarian class feature,
//!   value always 0 whether absent or granted — this is the
//!   grant-state-change case, not a value-change case)
//! - rage rounds per day 7 -> 9 (4 + Constitution modifier (+3) + 2 *
//!   (level - 1)), landing in `resource_pool_change`, not
//!   `automatic_features`
//!
//! Not yet at level 2: Trap Sense (3rd-level), Improved Uncanny Dodge
//! (5th-level) — neither should produce a grant for this transition,
//! proving the diff does not over-fire on every level-gated feature.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::{compute_level_up_grants, PickCategory};

fn human_barbarian_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_barbarian".to_string()),
        source_package_id: "sd20_levelup_barbarian".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:barbarian".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 16,
                intelligence: 8,
                wisdom: 10,
                charisma: 8,
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
fn barbarian_level_1_to_2_grants_bab_rise_fortitude_rise_and_uncanny_dodge() {
    let character = human_barbarian_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects.len(), 1);
    assert_eq!(bab_grant.effects[0].value, 2, "barbarian level 2 full BAB must be +2");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "barbarian:2");
    assert_eq!(bab_grant.source_table.column_key, "base_attack_bonus");

    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "barbarian level 2 good Fortitude save must be +3");

    // Reflex/Will stay +0 at level 2 (poor save, level / 3 = 0 at both 1
    // and 2) — must NOT produce a grant.
    assert!(grant("ref_save").is_none(), "reflex save must not change 1 -> 2");
    assert!(grant("will_save").is_none(), "will save must not change 1 -> 2");

    let uncanny_dodge_grant = grant("uncanny dodge").unwrap_or_else(|| {
        panic!(
            "expected an uncanny_dodge grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert!(
        uncanny_dodge_grant.effects[0]
            .description
            .contains("granted at barbarian level 2"),
        "uncanny dodge grant must cite the real grounded explanation text: {:?}",
        uncanny_dodge_grant.effects[0]
    );
    assert_eq!(
        uncanny_dodge_grant.source_table.table,
        "pilot_compute::explain_barbarian_level1_chassis"
    );

    // Not yet granted at level 2 — must NOT produce a spurious grant.
    assert!(grant("trap sense").is_none(), "trap sense is a 3rd-level feature, not 2nd");
    assert!(
        grant("improved uncanny dodge").is_none(),
        "improved uncanny dodge is a 5th-level feature, not 2nd"
    );

    // Rage rounds per day is a resource pool change, not a discrete
    // automatic-feature grant.
    assert!(
        grant("rage_rounds_per_day").is_none(),
        "rage rounds per day must land in resource_pool_change, not automatic_features"
    );
    assert_eq!(plan.resource_pool_change.pools.len(), 1);
    let rage_pool = &plan.resource_pool_change.pools[0];
    assert_eq!(rage_pool.pool_id, "rage_rounds_per_day");
    assert_eq!(rage_pool.from_value, 7, "level 1 rage rounds per day: 4 + 3 (Con mod)");
    assert_eq!(rage_pool.to_value, 9, "level 2 rage rounds per day: 4 + 3 + 2 * (2 - 1)");

    // Not yet a capstone level.
    assert!(!plan.capstone_threshold);

    // No Rage Power catalog exists anywhere in this repo to enumerate
    // real candidates from (see barbarian.rs's module doc comment) —
    // pick_from_lists must stay honestly empty rather than fabricated.
    assert!(
        plan.pick_from_lists.is_empty(),
        "no Rage Power candidate catalog exists yet; pick_from_lists must not be fabricated"
    );
}

#[test]
fn barbarian_level_19_to_20_crosses_the_capstone_threshold() {
    let character = human_barbarian_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is the Barbarian's PF1 Core Rulebook capstone (Mighty Rage)"
    );
    // Mighty Rage's magnitude rise (Strength/Constitution morale bonus
    // +6 -> +8, Will-save morale bonus +3 -> +4) must be grounded as a
    // real grant, not silently dropped.
    let mighty_rage_grant = plan
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("strength morale bonus"))
        .unwrap_or_else(|| panic!("expected a strength_morale_bonus grant at the Mighty Rage tier rise: {:?}", plan.automatic_features));
    assert_eq!(mighty_rage_grant.effects[0].value, 8);
}

#[test]
fn non_barbarian_class_returns_an_honestly_empty_plan() {
    let mut character = human_barbarian_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);

    // Sanity: the type is actually reachable and PickCategory has the
    // variant this module's doc comment names for a future cycle.
    let _ = PickCategory::RagePower;
}
