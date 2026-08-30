//! SD-20 Epic 7 (Level Up grant model): Sorcerer — tenth core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, fighter, monk,
//! paladin, ranger, rogue, sorcerer, wizard; `scope-draft.md` §1.7,
//! `technical-design.md` §2.6).
//!
//! Uses the identical Human Sorcerer, STR 8 / DEX 14 / CON 13 / INT 10 /
//! WIS 12 / CHA 17 deterministic fixture posture
//! `tests/sd13_sorcerer_level1_spell_baseline.rs` already established
//! (`tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt`),
//! with the canonical Arcane bloodline selection
//! (`choice:sorcerer_bloodline -> bloodline:arcane`) and its class-skill
//! choice (`choice:sorcerer_bloodline_class_skill -> knowledge:arcana`) —
//! the exact input shape `pilot_compute.rs`'s
//! `explain_sorcerer_level1_spell_baseline` requires to ground its
//! bloodline-choice recognition pillars at all.
//!
//! `rules_tables::crb::class_tables`'s `CLASS_META` row for
//! `ClassId::Sorcerer` was spot-checked against
//! `pilot_compute.rs::explain_sorcerer_level1_spell_baseline`'s own
//! independently-grounded formulas before writing any implementation
//! code — the two sources agree at every level (Half BAB, good Will
//! only) — so `src/rules_core/level_up/sorcerer.rs` composes
//! `class_tables()` directly for the base-attack/base-save pillars,
//! mirroring Barbarian's/Fighter's/Monk's/Paladin's own precedent (NOT
//! Cleric's/Druid's deviation). No second latent `good_saves` defect
//! exists for Sorcerer.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::compute_level_up_grants;

fn human_sorcerer_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_sorcerer".to_string()),
        source_package_id: "sd20_levelup_sorcerer".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:sorcerer".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 8,
                dexterity: 14,
                constitution: 13,
                intelligence: 10,
                wisdom: 12,
                charisma: 17,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: vec![
                SelectedChoice {
                    choice_set_id: "choice:sorcerer_bloodline".to_string(),
                    selection_id: "bloodline:arcane".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:sorcerer_bloodline_class_skill".to_string(),
                    selection_id: "knowledge:arcana".to_string(),
                },
            ],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn grant_by_column<'a>(
    plan: &'a codex::rules_core::level_up::LevelUpPlan,
    column_key: &str,
) -> Option<&'a codex::rules_core::level_up::Grant> {
    plan.automatic_features
        .iter()
        .find(|grant| grant.source_table.column_key == column_key)
}

#[test]
fn sorcerer_level_1_to_2_grants_bab_will_save_and_spells_per_day_rises() {
    let character = human_sorcerer_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let bab_grant = grant_by_column(&plan, "base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 1, "sorcerer level 2 1/2-BAB (2/2) must be +1");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "sorcerer:2");

    // Will is Sorcerer's one good save and must rise 2 -> 3 (1/2+2=2,
    // 2/2+2=3).
    let will_grant = grant_by_column(&plan, "will_save")
        .unwrap_or_else(|| panic!("expected a will_save grant: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 3);

    // Fortitude and Reflex are Sorcerer's poor saves and must NOT change
    // 1 -> 2 (1/3 = 0, 2/3 = 0).
    assert!(grant_by_column(&plan, "fort_save").is_none());
    assert!(grant_by_column(&plan, "ref_save").is_none());

    // Base 1st-level spells per day genuinely rises 3 -> 4.
    let base_spells_grant = grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_1",
    )
    .unwrap_or_else(|| panic!("expected a base spells-per-day grant: {:?}", plan.automatic_features));
    assert_eq!(base_spells_grant.effects[0].value, 4);

    // Cantrips known genuinely rises 4 -> 5.
    let cantrips_known_grant = grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.spells_known.spell_level_0",
    )
    .unwrap_or_else(|| panic!("expected a cantrips-known grant: {:?}", plan.automatic_features));
    assert_eq!(cantrips_known_grant.effects[0].value, 5);

    // 1st-level spells known stays at 2 (no change) -- must NOT produce a
    // grant.
    assert!(grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.spells_known.spell_level_1"
    )
    .is_none());

    // Total 1st-level spells per day genuinely rises (base 3+bonus1=4 ->
    // base 4+bonus1=5).
    let total_spells_grant = grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_1",
    )
    .unwrap_or_else(|| panic!("expected a total spells-per-day grant: {:?}", plan.automatic_features));
    assert_eq!(total_spells_grant.effects[0].value, 5);

    // Spell-level access stays at 1 (2nd-level spells begin only at
    // sorcerer level 4) -- must NOT produce a grant.
    assert!(grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.spell_level_access"
    )
    .is_none());

    // Eschew Materials and the bloodline choice recognitions are flat +0
    // records present at every supported level -- must NOT produce a
    // grant.
    assert!(grant_by_column(&plan, "class_chassis.sorcerer.eschew_materials").is_none());
    assert!(grant_by_column(&plan, "class_chassis.sorcerer.bloodline_choice").is_none());

    assert!(!plan.capstone_threshold);
    assert!(
        plan.pick_from_lists.is_empty(),
        "no spell-list candidate catalog exists yet; pick_from_lists must not be fabricated"
    );
    assert!(
        plan.resource_pool_change.pools.is_empty(),
        "Sorcerer has no flat daily-use resource pool; its entire class-specific mechanic is \
         the spells-per-day ladder, already granted through automatic_features"
    );
}

#[test]
fn sorcerer_level_3_to_4_opens_second_level_spell_access() {
    let character = human_sorcerer_input(3);
    let plan = compute_level_up_grants(&character, 3, 4);

    let bab_grant = grant_by_column(&plan, "base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 2, "sorcerer level 4 1/2-BAB (4/2) must be +2");

    // 2nd-level sorcerer spells first become accessible at level 4 (a
    // genuine None -> Some transition, proving the from-side-miss idiom
    // handles a newly ACCESSIBLE spell level with zero special-casing).
    let access_grant = grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.spell_level_access",
    )
    .unwrap_or_else(|| panic!("expected a spell_level_access grant: {:?}", plan.automatic_features));
    assert_eq!(access_grant.effects[0].value, 2);

    let base_spells_level2_grant = grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_2",
    )
    .unwrap_or_else(|| panic!("expected a newly-accessible 2nd-level base spells-per-day grant: {:?}", plan.automatic_features));
    assert_eq!(base_spells_level2_grant.effects[0].value, 3);

    let known_level2_grant = grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.spells_known.spell_level_2",
    )
    .unwrap_or_else(|| panic!("expected a newly-accessible 2nd-level spells-known grant: {:?}", plan.automatic_features));
    assert_eq!(known_level2_grant.effects[0].value, 1);

    let save_dc_level2_grant = grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_2",
    )
    .unwrap_or_else(|| panic!("expected a newly-accessible 2nd-level spell save DC grant: {:?}", plan.automatic_features));
    // 10 + spell level 2 + Charisma modifier 3 = 15.
    assert_eq!(save_dc_level2_grant.effects[0].value, 15);

    assert!(!plan.capstone_threshold);
}

#[test]
fn sorcerer_level_19_to_20_crosses_the_character_level_cap_with_no_fabricated_named_capstone() {
    let character = human_sorcerer_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is PF1's universal character-level cap"
    );

    // Sorcerer has NO grounded distinct named capstone class feature at
    // level 20 (the class table's level-19/20 "Special" column names
    // bloodline-specific text that pilot_compute.rs leaves
    // named-but-unproven) -- only the ordinary base-attack / base-save /
    // spell-progression pillars keep rising through the same generic
    // diff, exactly as at every other level. No feature named "capstone"
    // or similar is fabricated.
    assert!(
        !plan
            .automatic_features
            .iter()
            .any(|grant| grant.name.to_lowercase().contains("capstone")),
        "Sorcerer has no grounded named 20th-level capstone feature; none may be fabricated: {:?}",
        plan.automatic_features
    );

    let bab_grant = grant_by_column(&plan, "base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant at the level cap: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 10, "sorcerer level 20 1/2-BAB (20/2) must be +10");

    let will_grant = grant_by_column(&plan, "will_save")
        .unwrap_or_else(|| panic!("expected a will_save grant at the level cap: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 12, "sorcerer level 20 good Will (20/2+2) must be +12");

    // Spell-level access STAYS at 9 (unchanged from level 18/19; the
    // ladder was already fully populated through 9th-level spells) -- must
    // NOT produce a spurious grant.
    assert!(grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.spell_level_access"
    )
    .is_none());

    // 9th-level base spells per day genuinely rises 4 -> 6.
    let ninth_level_spells_grant = grant_by_column(
        &plan,
        "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_9",
    )
    .unwrap_or_else(|| panic!("expected a 9th-level base spells-per-day grant at the level cap: {:?}", plan.automatic_features));
    assert_eq!(ninth_level_spells_grant.effects[0].value, 6);
}

#[test]
fn non_sorcerer_class_returns_an_honestly_empty_plan() {
    // Uses `class:oracle` — a genuinely unlanded PF1 class (not one of
    // Epic 7's 11 core classes). Originally used `class:wizard`
    // (matching every prior Epic 7 sibling's own negative control at the
    // time, since wizard was last in Epic 7's per-class order and still
    // open). Epic 7's eleventh and final cycle (`wizard.rs`) landed a
    // real Wizard dispatch arm, so `class:wizard` was fixed forward to
    // `class:oracle` in that same commit — no core class id can serve as
    // an "unlanded" negative control anymore now that all 11 are landed.
    let mut character = human_sorcerer_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);

    // Sanity: dispatch still reaches Sorcerer correctly (no cross-class
    // leakage introduced by wiring this class's new match arm).
    let sorcerer = human_sorcerer_input(1);
    let sorcerer_plan = compute_level_up_grants(&sorcerer, 1, 2);
    assert!(
        !sorcerer_plan.automatic_features.is_empty(),
        "sorcerer dispatch must still produce real grants"
    );
}
