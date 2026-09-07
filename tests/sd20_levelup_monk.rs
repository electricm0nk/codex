//! SD-20 Epic 7 (Level Up grant model): Monk — sixth core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, fighter, monk,
//! ...; `scope-draft.md` §1.7, `technical-design.md` §2.6).
//!
//! Uses the identical Human Monk STR 13 / DEX 16 / CON 13 / INT 8 / WIS
//! 17 / CHA 10 deterministic fixture posture already established by
//! `tests/fixtures/rules_core/pf1_human_monk_level1_sd13_deterministic_input.txt`
//! (the exact input shape `pilot_compute.rs`'s `explain_monk_level1_chassis`
//! requires to fire at all) — race_id `"race:human"`, no `selected_choices`
//! (so no bonus-feat-choice explanations fire and cannot spuriously affect
//! any transition asserted here).
//!
//! Expected grants, cross-checked directly against `pilot_compute.rs`'s own
//! grounded monk explanations (the `tests/sd13_monk_level*_progression.rs`
//! family establishes these same numbers independently):
//!
//! - level 1 -> 2: base attack bonus 0 -> 1 (3/4-BAB, `class_tables.rs`),
//!   ALL THREE base saves 2 -> 3 (Monk is unusual in having all three
//!   saves good, unlike Barbarian's single-good-save split), Flurry of
//!   Blows flat attack bonus -1 -> 0, and Evasion newly granted (a
//!   2nd-level Monk class feature, value always 0 whether absent or
//!   granted — the grant-state-change case).
//! - level 3 -> 4: Still Mind stays granted unchanged (must not produce a
//!   duplicate grant), the ki pool newly appears as a `resource_pool_change`
//!   entry (0 -> 5, mirroring Barbarian's rage-rounds-per-day pattern —
//!   the ki pool is a flat resource-count magnitude, not a discrete
//!   automatic-feature grant), and Slow Fall is newly granted.
//! - level 4 -> 5: Purity of Body newly granted.
//! - level 7 -> 8: the unarmed strike damage die steps up 1d8 -> 1d10 AND
//!   the Flurry of Blows attack count rises 2 -> 3 in the same transition
//!   (both verified independently against `tests/sd13_monk_level8_progression.rs`).
//! - level 10 -> 11: Diamond Body newly granted.
//! - level 12 -> 13: task #49 widened `MAX_SUPPORTED_MONK_LEVEL`
//!   (`pilot_compute.rs`) from 12 to 20, so this transition now genuinely
//!   grants Diamond Soul (spell resistance = 10 + monk level) rather than
//!   honestly producing an empty plan -- the old ceiling test is flipped to
//!   a positive assertion below, mirroring how
//!   `tests/sd13_monk_level9_progression.rs`'s own level-10 boundary was
//!   flipped once a later slice widened past it. `capstone_threshold`
//!   still correctly reports false at 13 (< 20).

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::compute_level_up_grants;

fn human_monk_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_monk".to_string()),
        source_package_id: "sd20_levelup_monk".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:monk".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 13,
                dexterity: 16,
                constitution: 13,
                intelligence: 8,
                wisdom: 17,
                charisma: 10,
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
fn monk_level_1_to_2_grants_bab_all_three_saves_flurry_bonus_and_evasion() {
    let character = human_monk_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 1, "monk level 2 3/4-BAB is 2 * 3 / 4 = 1");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "monk:2");
    assert_eq!(bab_grant.source_table.column_key, "base_attack_bonus");

    // Monk is unusual among the martial classes: all three base saves are
    // good, so all three must rise together (unlike Barbarian's single
    // good save).
    let fort_grant = grant("fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "monk level 2 good Fortitude: 2/2+2 = 3");
    let ref_grant = grant("ref_save")
        .unwrap_or_else(|| panic!("expected a ref_save grant: {:?}", plan.automatic_features));
    assert_eq!(ref_grant.effects[0].value, 3, "monk level 2 good Reflex: 2/2+2 = 3");
    let will_grant = grant("will_save")
        .unwrap_or_else(|| panic!("expected a will_save grant: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 3, "monk level 2 good Will: 2/2+2 = 3");

    let flurry_bonus_grant = grant("flurry of blows attack bonus").unwrap_or_else(|| {
        panic!("expected a flurry_of_blows_attack_bonus grant: {:?}", plan.automatic_features)
    });
    assert_eq!(
        flurry_bonus_grant.effects[0].value, 0,
        "monk level 2 flurry flat attack modifier is monk level - 2 = 2 - 2 = 0"
    );

    // Flurry attack count stays 2 at levels 1-7 — must NOT produce a grant.
    assert!(
        grant("flurry of blows attack count").is_none(),
        "flurry attack count must not change 1 -> 2"
    );
    // Unarmed strike damage die stays 1d6 at levels 1-3 — must NOT change.
    assert!(
        grant("unarmed strike damage die").is_none(),
        "unarmed strike damage die must not change 1 -> 2 (1d6 band spans levels 1-3)"
    );
    // AC Bonus is a flat Wisdom-modifier value, constant across levels for
    // a fixed character — must NOT produce a grant.
    assert!(grant("ac bonus").is_none(), "AC Bonus must not change 1 -> 2 for a fixed Wisdom score");

    let evasion_grant = grant("evasion").unwrap_or_else(|| {
        panic!("expected an evasion grant at level 2: {:?}", plan.automatic_features)
    });
    assert!(
        evasion_grant.effects[0]
            .description
            .contains("granted at monk level 2"),
        "evasion grant must cite the real grounded explanation text: {:?}",
        evasion_grant.effects[0]
    );
    assert_eq!(
        evasion_grant.source_table.table,
        "pilot_compute::explain_monk_level1_chassis"
    );

    // Not yet granted at level 2 — must NOT produce a spurious grant.
    assert!(grant("still mind").is_none(), "still mind is a 3rd-level feature, not 2nd");
    assert!(grant("ki pool").is_none(), "ki pool is a 4th-level feature, not 2nd");
    assert!(grant("slow fall").is_none(), "slow fall is a 4th-level feature, not 2nd");
    assert!(grant("purity of body").is_none(), "purity of body is a 5th-level feature, not 2nd");
    assert!(grant("diamond body").is_none(), "diamond body is an 11th-level feature, not 2nd");

    // No ki pool exists yet at either level 1 or level 2 (both correctly
    // absent), so the resource pool must stay empty.
    assert!(
        plan.resource_pool_change.pools.is_empty(),
        "no ki pool resource change is expected before its 4th-level gate: {:?}",
        plan.resource_pool_change.pools
    );

    assert!(!plan.capstone_threshold);
}

#[test]
fn monk_level_3_to_4_grants_ki_pool_resource_change_and_slow_fall_keeps_still_mind_unchanged() {
    let character = human_monk_input(3);
    let plan = compute_level_up_grants(&character, 3, 4);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    // Still Mind was already granted at level 3; it must NOT produce a
    // duplicate grant at 3 -> 4 since its flat +2 magnitude is unchanged.
    assert!(
        grant("still mind").is_none(),
        "still mind must not re-fire once already granted with an unchanged magnitude"
    );

    let slow_fall_grant = grant("slow fall").unwrap_or_else(|| {
        panic!("expected a slow_fall grant at level 4: {:?}", plan.automatic_features)
    });
    assert!(
        slow_fall_grant.effects[0]
            .description
            .contains("granted at monk level 4"),
        "slow fall grant must cite the real grounded explanation text: {:?}",
        slow_fall_grant.effects[0]
    );

    // The ki pool is a resource-pool magnitude, not a discrete grant.
    assert!(grant("ki pool").is_none(), "ki pool must land in resource_pool_change, not automatic_features");
    assert_eq!(plan.resource_pool_change.pools.len(), 1);
    let ki_pool = &plan.resource_pool_change.pools[0];
    assert_eq!(ki_pool.pool_id, "ki_pool_size");
    assert_eq!(ki_pool.from_value, 0, "ki pool is correctly absent (0) below its 4th-level gate");
    assert_eq!(ki_pool.to_value, 5, "monk level 4 ki pool: 4 / 2 + 3 (Wisdom modifier) = 5");

    assert!(grant("purity of body").is_none(), "purity of body is a 5th-level feature, not 4th");
    assert!(grant("diamond body").is_none(), "diamond body is an 11th-level feature, not 4th");
}

#[test]
fn monk_level_4_to_5_grants_purity_of_body() {
    let character = human_monk_input(4);
    let plan = compute_level_up_grants(&character, 4, 5);

    let purity_grant = plan
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("purity of body"))
        .unwrap_or_else(|| panic!("expected a purity_of_body grant at level 5: {:?}", plan.automatic_features));
    assert!(
        purity_grant.effects[0]
            .description
            .contains("granted at monk level 5"),
        "purity of body grant must cite the real grounded explanation text: {:?}",
        purity_grant.effects[0]
    );

    // Slow Fall was already granted at level 4; must not re-fire (its
    // `.value` stays 0 the whole 4 -> 5 transition).
    assert!(
        !plan
            .automatic_features
            .iter()
            .any(|grant| grant.name.contains("slow fall")),
        "slow fall must not re-fire once already granted"
    );
}

#[test]
fn monk_level_7_to_8_grants_unarmed_die_and_flurry_count_rise_together() {
    let character = human_monk_input(7);
    let plan = compute_level_up_grants(&character, 7, 8);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let die_grant = grant("unarmed strike damage die").unwrap_or_else(|| {
        panic!("expected an unarmed_strike_damage_die grant at level 8: {:?}", plan.automatic_features)
    });
    assert_eq!(die_grant.effects[0].value, 10, "monk level 8 unarmed strike die steps up to 1d10 (die-size facet 10)");

    let count_grant = grant("flurry of blows attack count").unwrap_or_else(|| {
        panic!("expected a flurry_of_blows_attack_count grant at level 8: {:?}", plan.automatic_features)
    });
    assert_eq!(count_grant.effects[0].value, 3, "monk level 8 flurry grants a third attack");

    let bab_grant = grant("base_attack_bonus").unwrap_or_else(|| {
        panic!("expected a base_attack_bonus grant at level 8: {:?}", plan.automatic_features)
    });
    assert_eq!(bab_grant.effects[0].value, 6, "monk level 8 3/4-BAB is 8 * 3 / 4 = 6");
}

#[test]
fn monk_level_10_to_11_grants_diamond_body() {
    let character = human_monk_input(10);
    let plan = compute_level_up_grants(&character, 10, 11);

    let diamond_grant = plan
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("diamond body"))
        .unwrap_or_else(|| panic!("expected a diamond_body grant at level 11: {:?}", plan.automatic_features));
    assert!(
        diamond_grant.effects[0]
            .description
            .contains("granted at monk level 11"),
        "diamond body grant must cite the real grounded explanation text: {:?}",
        diamond_grant.effects[0]
    );
}

#[test]
fn monk_level_12_to_13_grants_diamond_soul_now_that_the_ceiling_is_widened() {
    // Task #49 widened pilot_compute.rs's own MAX_SUPPORTED_MONK_LEVEL from
    // 12 to 20 (and class_tables.rs's mirrored Monk max_supported_level to
    // match), so this transition is no longer past the grounded ceiling:
    // Diamond Soul (13th-level, spell resistance = 10 + monk level) is a
    // real new grant.
    let character = human_monk_input(12);
    let plan = compute_level_up_grants(&character, 12, 13);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let diamond_soul_grant = grant("diamond soul").unwrap_or_else(|| {
        panic!("expected a diamond_soul_spell_resistance grant at level 13: {:?}", plan.automatic_features)
    });
    assert_eq!(
        diamond_soul_grant.effects[0].value, 23,
        "monk level 13 Diamond Soul spell resistance is 10 + 13 = 23"
    );
    assert!(
        diamond_soul_grant.effects[0]
            .description
            .contains("granted at monk level 13"),
        "diamond soul grant must cite the real grounded explanation text: {:?}",
        diamond_soul_grant.effects[0]
    );

    assert!(!plan.capstone_threshold, "level 13 has not yet crossed the level-20 capstone");
}

#[test]
fn monk_level_19_to_20_grants_perfect_self_dr_and_crosses_the_capstone() {
    // Perfect Self's DR was previously provably dead code under the old
    // MAX_SUPPORTED_MONK_LEVEL = 12 ceiling (level 20 was never reachable at
    // all); task #49 widens the ceiling to 20, making this a real,
    // independently re-derived grant rather than a fabricated one.
    let character = human_monk_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let perfect_self_grant = grant("perfect self damage reduction").unwrap_or_else(|| {
        panic!("expected a perfect_self_damage_reduction grant at level 20: {:?}", plan.automatic_features)
    });
    assert_eq!(
        perfect_self_grant.effects[0].value, 10,
        "monk level 20 Perfect Self grants DR 10/chaotic"
    );

    assert!(plan.capstone_threshold, "level 20 must cross the universal PF1 capstone");
}

#[test]
fn non_monk_class_returns_an_honestly_empty_plan() {
    let mut character = human_monk_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);
}
