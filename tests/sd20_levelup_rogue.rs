//! SD-20 Epic 7 (Level Up grant model): Rogue — ninth core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, fighter, monk,
//! paladin already landed; `scope-draft.md` §1.7, `technical-design.md`
//! §2.6).
//!
//! Mirrors `tests/sd20_levelup_bard.rs`'s / `tests/sd20_levelup_barbarian.rs`'s
//! shape exactly. Uses a deterministic Human Rogue posture (ability scores
//! do not affect any grounded Rogue pillar this module composes with —
//! base attack, base saves, sneak attack, trapfinding, Evasion, Trap
//! Sense, Uncanny Dodge, Improved Uncanny Dodge, and Master Strike are all
//! either pure level formulas or flat identity records), advancing level
//! 1 -> level 2 and level 19 -> level 20, cross-checked directly against
//! `tests/sd18_rogue_level20_widening.rs`'s own already-grounded numbers:
//!
//! - level 1: BAB +0, Fort +0, Ref +2, Will +0, sneak attack 1d6,
//!   trapfinding +1
//! - level 2: BAB +1, Fort +0, Ref +3, Will +0, sneak attack 1d6 (no
//!   change), trapfinding +1 (no change), Evasion newly granted, the
//!   first Rogue Talent slot newly granted (a `choice:rogue_talent`
//!   selection is supplied)
//! - level 19: BAB +14, Fort +6, Ref +11, Will +6, sneak attack 10d6,
//!   trap sense +6, trapfinding +9
//! - level 20: BAB +15, Fort +6 (no change), Ref +12, Will +6 (no
//!   change), sneak attack 10d6 (no change), trap sense +6 (no change),
//!   trapfinding +10, Master Strike newly granted (the 20th-level Rogue
//!   capstone), the tenth Rogue Talent slot newly granted (a
//!   `choice:rogue_talent_10` selection is supplied)

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::{compute_level_up_grants, PickCategory};

fn human_rogue_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_rogue".to_string()),
        source_package_id: "sd20_levelup_rogue".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:rogue".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 12,
                dexterity: 16,
                constitution: 12,
                intelligence: 13,
                wisdom: 10,
                charisma: 8,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: vec![
                SelectedChoice {
                    choice_set_id: "choice:rogue_talent".to_string(),
                    selection_id: "talent:finesse_rogue".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:rogue_talent_10".to_string(),
                    selection_id: "talent:another_day".to_string(),
                },
            ],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn rogue_level_1_to_2_grants_bab_rise_reflex_rise_evasion_and_first_talent_slot() {
    let character = human_rogue_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects.len(), 1);
    assert_eq!(bab_grant.effects[0].value, 1, "rogue level 2 3/4-BAB must be +1");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "rogue:2");
    assert_eq!(bab_grant.source_table.column_key, "base_attack_bonus");

    let ref_grant = grant("ref_save")
        .unwrap_or_else(|| panic!("expected a ref_save grant: {:?}", plan.automatic_features));
    assert_eq!(ref_grant.effects[0].value, 3, "rogue level 2 good Reflex save must be +3");

    // Fortitude and Will both stay +0 at level 2 (poor saves, level / 3 = 0
    // at both 1 and 2) — must NOT produce a grant.
    assert!(grant("fort_save").is_none(), "fortitude save must not change 1 -> 2");
    assert!(grant("will_save").is_none(), "will save must not change 1 -> 2");

    let evasion_grant = grant("evasion").unwrap_or_else(|| {
        panic!("expected an evasion grant at level 2: {:?}", plan.automatic_features)
    });
    assert_eq!(evasion_grant.effects[0].value, 0, "Evasion is a bounded identity record (+0)");
    assert_eq!(
        evasion_grant.source_table.table,
        "pilot_compute::explain_rogue_level1_chassis"
    );

    let talent_grant = grant("talent choice").unwrap_or_else(|| {
        panic!(
            "expected the first rogue talent choice-slot grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(talent_grant.effects[0].value, 0, "talent choice-slot recognition is +0");

    // Not yet changed at level 2 — must NOT produce a spurious grant.
    assert!(
        grant("sneak_attack").is_none(),
        "sneak attack die count stays 1d6 through level 2"
    );
    assert!(
        grant("trapfinding").is_none(),
        "trapfinding stays +1 through level 2"
    );
    assert!(grant("trap_sense").is_none(), "trap sense is not yet granted below level 3");
    assert!(grant("uncanny_dodge").is_none(), "uncanny dodge is not yet granted below level 4");
    assert!(grant("master_strike").is_none(), "master strike is not the level 2 capstone");

    // No resource pool is composed for Rogue (no per-day pool exists).
    assert!(
        plan.resource_pool_change.pools.is_empty(),
        "rogue has no named per-day resource pool"
    );

    // Not yet a capstone level.
    assert!(!plan.capstone_threshold);

    // No candidate catalog exists for any open-ended rogue talent pick
    // list yet (mirrors barbarian's Rage Power precedent) — pick_from_lists
    // must stay honestly empty rather than fabricated.
    assert!(
        plan.pick_from_lists.is_empty(),
        "no rogue talent candidate catalog exists yet; pick_from_lists must not be fabricated"
    );
}

#[test]
fn rogue_level_19_to_20_crosses_the_capstone_threshold_and_grants_master_strike() {
    let character = human_rogue_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is the Rogue's PF1 Core Rulebook capstone (Master Strike)"
    );

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 15, "rogue level 20 3/4-BAB must rise to +15");

    let ref_grant = grant("ref_save")
        .unwrap_or_else(|| panic!("expected a ref_save grant: {:?}", plan.automatic_features));
    assert_eq!(ref_grant.effects[0].value, 12, "rogue level 20 good Reflex save must rise to +12");

    assert!(
        grant("fort_save").is_none(),
        "fortitude stays +6 at level 20, an integer-division coincidence with level 19"
    );
    assert!(
        grant("will_save").is_none(),
        "will stays +6 at level 20, an integer-division coincidence with level 19"
    );
    assert!(
        grant("sneak_attack").is_none(),
        "sneak attack stays 10d6 at level 20, an integer-division coincidence with level 19"
    );
    assert!(
        grant("trap_sense").is_none(),
        "trap sense stays +6 at level 20, an integer-division coincidence with level 19"
    );

    let trapfinding_grant = grant("trapfinding").unwrap_or_else(|| {
        panic!(
            "expected a trapfinding grant at level 20: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(trapfinding_grant.effects[0].value, 10, "rogue level 20 trapfinding must rise to +10");

    // Master Strike's magnitude rise (a genuinely new grant at the
    // capstone level) must be grounded as a real grant, not silently
    // dropped.
    let master_strike_grant = plan
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("master strike"))
        .unwrap_or_else(|| {
            panic!(
                "expected a master_strike grant at the capstone level: {:?}",
                plan.automatic_features
            )
        });
    // SD-32 Epic 1 (compute-library wiring): `human_rogue_input`'s Intelligence 13
    // (modifier +1) now resolves through the corpus's own
    // `BONUS:VAR|MasterStrikeDC|10+(MasterStrikeLVL/2)+INT` formula: 10 + 20/2 + 1 = 21.
    assert_eq!(master_strike_grant.effects[0].value, 21);

    let tenth_talent_grant = grant("talent 10 choice").unwrap_or_else(|| {
        panic!(
            "expected the tenth rogue talent choice-slot grant at level 20: {:?}",
            plan.automatic_features
        )
    });
    assert_eq!(tenth_talent_grant.effects[0].value, 0);
}

#[test]
fn non_rogue_class_returns_an_honestly_empty_plan() {
    let mut character = human_rogue_input(1);
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
    let _ = PickCategory::Feat;
}
