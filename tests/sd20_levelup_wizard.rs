//! SD-20 Epic 7 (Level Up grant model): Wizard — eleventh and FINAL core
//! class per Step 2's stated order (barbarian, bard, cleric, druid,
//! fighter, monk, paladin, ranger, rogue, sorcerer, wizard;
//! `scope-draft.md` §1.7, `technical-design.md` §2.6). **This cycle
//! closes Epic 7** — all 11 core classes now have a landed
//! `LevelUpPlan`.
//!
//! Uses the identical Human Wizard, STR 8 / DEX 14 / CON 13 / INT 17 /
//! WIS 12 / CHA 10 deterministic fixture posture
//! `tests/sd13_wizard_level1_prepared_spell_baseline.rs` already
//! established
//! (`tests/fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt`),
//! with the canonical arcane school specialization choice
//! (`choice:wizard_school_specialization -> school:evocation`) and its
//! two opposed schools (`choice:wizard_opposed_schools ->
//! school:necromancy`, `school:transmutation`) — the exact input shape
//! `pilot_compute.rs`'s `explain_wizard_level1_prepared_spell_baseline`
//! requires to ground its specialization-choice-gated pillars at all.
//!
//! `rules_tables::crb::class_tables`'s `CLASS_META` row for
//! `ClassId::Wizard` was spot-checked against `pilot_compute.rs`'s own
//! independently-grounded Wizard formulas before writing any
//! implementation code — the two sources agree at every level (Half
//! BAB, good Will only, matching Sorcerer's shape) — so
//! `src/rules_core/level_up/wizard.rs` composes `class_tables()`
//! directly for the base-attack/base-save pillars, mirroring
//! Barbarian's/Fighter's/Monk's/Paladin's/Ranger's/Rogue's/Sorcerer's
//! own precedent (NOT Cleric's/Druid's deviation). No defect exists for
//! Wizard's `CLASS_META` row.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::compute_level_up_grants;

fn human_wizard_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_wizard".to_string()),
        source_package_id: "sd20_levelup_wizard".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:wizard".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 8,
                dexterity: 14,
                constitution: 13,
                intelligence: 17,
                wisdom: 12,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: vec![
                SelectedChoice {
                    choice_set_id: "choice:wizard_school_specialization".to_string(),
                    selection_id: "school:evocation".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:wizard_opposed_schools".to_string(),
                    selection_id: "school:necromancy".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:wizard_opposed_schools".to_string(),
                    selection_id: "school:transmutation".to_string(),
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
fn wizard_level_1_to_2_grants_bab_and_will_save_rises_with_no_spurious_spell_grants() {
    let character = human_wizard_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let bab_grant = grant_by_column(&plan, "base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 1, "wizard level 2 1/2-BAB (2/2) must be +1");
    assert_eq!(bab_grant.source_table.table, "class_tables");
    assert_eq!(bab_grant.source_table.row_key, "wizard:2");

    // Will is Wizard's one good save and must rise 2 -> 3 (1/2+2=2,
    // 2/2+2=3).
    let will_grant = grant_by_column(&plan, "will_save")
        .unwrap_or_else(|| panic!("expected a will_save grant: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 3);

    // Fortitude and Reflex are Wizard's poor saves and must NOT change
    // 1 -> 2 (1/3 = 0, 2/3 = 0).
    assert!(grant_by_column(&plan, "fort_save").is_none());
    assert!(grant_by_column(&plan, "ref_save").is_none());

    // The specialist bonus slot flat count stays exactly 1 through
    // levels 1-2 (2nd-level wizard spells require class level 3) --
    // must NOT produce a grant.
    assert!(grant_by_column(&plan, "class_chassis.wizard.specialist_bonus_slot").is_none());

    // Intense Spells' bonus damage stays exactly 1 through levels 1-2
    // (max(1/2,1)=1, max(2/2,1)=1) -- must NOT produce a grant.
    assert!(grant_by_column(&plan, "class_chassis.wizard.intense_bonus_damage").is_none());

    // Force Missile's uses-per-day pool is level-independent -- must
    // NOT produce a grant.
    assert!(grant_by_column(&plan, "class_chassis.wizard.force_missile_uses_per_day").is_none());

    // The recognition, Scribe Scroll, and specialization-choice
    // records are flat +0 at every supported level -- must NOT produce
    // a grant.
    assert!(grant_by_column(&plan, "class_chassis.spell_baseline.wizard").is_none());
    assert!(grant_by_column(&plan, "class_chassis.wizard.scribe_scroll").is_none());
    assert!(grant_by_column(&plan, "class_chassis.wizard.specialization_choice").is_none());

    assert!(!plan.capstone_threshold);
    assert!(
        plan.pick_from_lists.is_empty(),
        "no spell-list or bonus-feat candidate catalog exists yet; pick_from_lists must not be \
         fabricated"
    );
    assert!(
        plan.resource_pool_change.pools.is_empty(),
        "Wizard has no flat daily-use resource pool that changes on a level-up (Force Missile's \
         pool is level-independent)"
    );
}

#[test]
fn wizard_level_2_to_3_opens_the_second_specialist_bonus_slot_and_poor_saves_rise() {
    let character = human_wizard_input(2);
    let plan = compute_level_up_grants(&character, 2, 3);

    // Base attack bonus and Will stay numerically unchanged 2 -> 3 (an
    // integer-division coincidence: 2/2=1, 3/2=1; 2/2+2=3, 3/2+2=3) --
    // must NOT produce spurious grants.
    assert!(grant_by_column(&plan, "base_attack_bonus").is_none());
    assert!(grant_by_column(&plan, "will_save").is_none());

    // Fortitude and Reflex (Wizard's poor saves) genuinely rise 0 -> 1
    // (2/3=0, 3/3=1).
    let fort_grant = grant_by_column(&plan, "fort_save")
        .unwrap_or_else(|| panic!("expected a fort_save grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 1);
    let ref_grant = grant_by_column(&plan, "ref_save")
        .unwrap_or_else(|| panic!("expected a ref_save grant: {:?}", plan.automatic_features));
    assert_eq!(ref_grant.effects[0].value, 1);

    // The specialist bonus slot flat count genuinely rises 1 -> 2: a
    // level-3 wizard casts 2nd-level spells for the first time, so the
    // specialist gains one bonus slot of each spell level she can cast
    // (1st and 2nd), per pilot_compute.rs's own grounded formula.
    let bonus_slot_grant = grant_by_column(&plan, "class_chassis.wizard.specialist_bonus_slot")
        .unwrap_or_else(|| panic!("expected a specialist bonus slot grant: {:?}", plan.automatic_features));
    assert_eq!(bonus_slot_grant.effects[0].value, 2);

    // Intense Spells' bonus damage stays exactly 1 (max(2/2,1)=1,
    // max(3/2,1)=1) -- must NOT produce a grant.
    assert!(grant_by_column(&plan, "class_chassis.wizard.intense_bonus_damage").is_none());

    assert!(!plan.capstone_threshold);
}

#[test]
fn wizard_level_19_to_20_crosses_the_character_level_cap_with_no_fabricated_named_capstone() {
    let character = human_wizard_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is PF1's universal character-level cap"
    );

    // Wizard has NO grounded distinct named capstone class feature at
    // level 20 (`class_tables.rs`'s `ClassTableRow` carries no
    // "Special" column at all) -- only the ordinary base-attack /
    // base-save / school-power pillars keep rising through the same
    // generic diff, exactly as at every other level. No feature named
    // "capstone" or similar is fabricated.
    assert!(
        !plan
            .automatic_features
            .iter()
            .any(|grant| grant.name.to_lowercase().contains("capstone")),
        "Wizard has no grounded named 20th-level capstone feature; none may be fabricated: {:?}",
        plan.automatic_features
    );

    let bab_grant = grant_by_column(&plan, "base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant at the level cap: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 10, "wizard level 20 1/2-BAB (20/2) must be +10");

    let will_grant = grant_by_column(&plan, "will_save")
        .unwrap_or_else(|| panic!("expected a will_save grant at the level cap: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 12, "wizard level 20 good Will (20/2+2) must be +12");

    // The specialist bonus slot flat count STAYS at 9 (unchanged from
    // level 18/19; the ladder was already fully populated through
    // 9th-level spells at level 17) -- must NOT produce a spurious
    // grant.
    assert!(grant_by_column(&plan, "class_chassis.wizard.specialist_bonus_slot").is_none());

    // Intense Spells' bonus damage genuinely rises 9 -> 10
    // (max(19/2,1)=9, max(20/2,1)=10) -- Wizard's own distinct pillar,
    // still rising all the way to the level cap.
    let intense_grant = grant_by_column(&plan, "class_chassis.wizard.intense_bonus_damage")
        .unwrap_or_else(|| panic!("expected an intense-spells bonus-damage grant at the level cap: {:?}", plan.automatic_features));
    assert_eq!(intense_grant.effects[0].value, 10);
}

#[test]
fn non_wizard_class_returns_an_honestly_empty_plan() {
    // Uses `class:oracle` -- a genuinely unlanded PF1 core-ish class
    // (not one of Epic 7's 11 core classes). Every prior Epic 7 sibling
    // used `class:wizard` as its own negative control while Wizard
    // stayed open; this cycle lands the real Wizard dispatch arm, so
    // `class:wizard` no longer produces an honestly-empty plan and
    // those siblings are fixed forward in the same commit to use
    // `class:oracle` instead.
    let mut character = human_wizard_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);

    // Sanity: dispatch still reaches Wizard correctly (no cross-class
    // leakage introduced by wiring this class's new match arm).
    let wizard = human_wizard_input(1);
    let wizard_plan = compute_level_up_grants(&wizard, 1, 2);
    assert!(
        !wizard_plan.automatic_features.is_empty(),
        "wizard dispatch must still produce real grants"
    );
}
