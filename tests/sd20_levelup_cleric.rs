//! SD-20 Epic 7 (Level Up grant model): Cleric — third core class per
//! Step 2's stated order (barbarian, bard, cleric, ...;
//! `scope-draft.md` §1.7, `technical-design.md` §2.6).
//!
//! Uses the identical Human Cleric, WIS 17 / CHA 14 deterministic fixture
//! posture `tests/sd13_cleric_level1_spell_baseline.rs` already
//! established (race_id `"race:human"`, ability scores STR 10 / DEX 12 /
//! CON 13 / INT 8 / WIS 17 / CHA 14, and the canonical two-domain
//! selection `choice:cleric_domain -> domain:good` /
//! `choice:cleric_domain -> domain:healing` — the exact input shape
//! `pilot_compute.rs`'s `explain_cleric_level1_spell_baseline` requires to
//! ground its domain-power pillars at all).
//!
//! **Discovered during this cycle**: `rules_tables::crb::class_tables`'s
//! `CLASS_META` row for `ClassId::Cleric` encodes `good_saves.fortitude:
//! false` — but the PF1 Core Rulebook Cleric class table's good saves are
//! Fortitude AND Will (poor Reflex only), independently verified by this
//! codebase's own already-landed `pilot_compute.rs::explain_cleric_level1_spell_baseline`
//! (which primary-source-verified this exact fact against d20pfsrd and
//! legacy.aonprd.com before ever landing — see that function's own doc
//! comment). `class_tables()`'s Druid row shares the identical defect.
//! Composing this class's `LevelUpPlan` with the buggy `class_tables()`
//! row would silently fabricate a wrong (poor-progression) Fortitude-save
//! grant, so `src/rules_core/level_up/cleric.rs` does NOT read
//! `class_tables()` at all — every automatic-feature pillar (base attack
//! bonus, all three base saves, Channel Energy, the domain spell slot
//! count, and the domain-power magnitudes) is composed from
//! `pilot_compute::compute_pilot_base_chassis`'s own already-grounded,
//! already-primary-source-verified Cleric explanations instead, exactly
//! mirroring how Barbarian's own class-specific pillars (which
//! `class_tables()` does not carry at all) are sourced. This test's
//! level-1-to-2 case is the direct regression proof: it asserts BOTH
//! Fortitude and Will rise together (the correct good/good/poor shape),
//! which the buggy `class_tables()` row would get wrong (it would report
//! Fortitude as an unchanging poor save, `1/3 = 0` and `2/3 = 0`).
//!
//! Cleric has no distinct named capstone class feature at 20th level (the
//! class table's level-20 "Special" column is genuinely blank, per
//! `pilot_compute.rs`'s own doc comment on `MAX_SUPPORTED_CLERIC_LEVEL`)
//! — unlike Barbarian's Mighty Rage. `capstone_threshold` still flags
//! `to_level >= 20` (PF1's universal character-level cap), but no
//! separate named grant is fabricated for it: the ordinary base-attack /
//! base-save / Touch-of-Good pillars keep rising through the same generic
//! diff mechanism used at every other level.
//!
//! Cleric's daily-use pools (Channel Energy uses/day, Touch of Good
//! uses/day, Rebuke Death uses/day) are all flat `3 + ability modifier`
//! formulas with no level term, so none of them ever change size between
//! `from_level` and `to_level` for a fixed character — `resource_pool_change`
//! genuinely stays empty for every Cleric level-up transition, unlike
//! Barbarian's rage-rounds-per-day pool (which does scale with level).

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::{compute_level_up_grants, PickCategory};

fn human_cleric_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_cleric".to_string()),
        source_package_id: "sd20_levelup_cleric".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:cleric".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 12,
                constitution: 13,
                intelligence: 8,
                wisdom: 17,
                charisma: 14,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: vec![
                SelectedChoice {
                    choice_set_id: "choice:cleric_domain".to_string(),
                    selection_id: "domain:good".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:cleric_domain".to_string(),
                    selection_id: "domain:healing".to_string(),
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
fn cleric_level_1_to_2_grants_bab_rise_and_both_good_saves_rise_together() {
    let character = human_cleric_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let bab_grant = grant_by_column(&plan, "class_chassis.cleric.base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 1, "cleric level 2 3/4-BAB (2*3/4) must be +1");
    assert_eq!(bab_grant.source_table.table, "pilot_compute::explain_cleric_level1_spell_baseline");
    assert_eq!(bab_grant.source_table.row_key, "cleric:2");

    // The regression proof: BOTH Fortitude and Will are good saves for
    // Cleric (real PF1 rule) and must rise together 2 -> 3. A cycle that
    // wrongly composed with the buggy `class_tables()` row would report
    // Fortitude as an unchanging poor save (0 -> 0) instead.
    let fort_grant = grant_by_column(&plan, "class_chassis.cleric.base_save.fortitude")
        .unwrap_or_else(|| panic!("expected a Fortitude grant (Cleric has a GOOD Fortitude save): {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "cleric level 2 good Fortitude (2/2+2) must be +3");

    let will_grant = grant_by_column(&plan, "class_chassis.cleric.base_save.will")
        .unwrap_or_else(|| panic!("expected a Will grant: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 3, "cleric level 2 good Will (2/2+2) must be +3");

    // Reflex is Cleric's one poor save and must NOT rise 1 -> 2 (1/3 = 0,
    // 2/3 = 0).
    assert!(
        grant_by_column(&plan, "class_chassis.cleric.base_save.reflex").is_none(),
        "cleric poor Reflex save must not change 1 -> 2"
    );

    // Channel Energy dice stays 1d6 at levels 1-2 (ceil(1/2) = ceil(2/2) = 1)
    // and the domain spell slot count stays 1 (2nd-level cleric spells
    // begin only at level 3) -- neither should produce a grant.
    assert!(
        grant_by_column(&plan, "class_chassis.cleric.channel_energy_dice").is_none(),
        "channel energy dice must not change 1 -> 2"
    );
    assert!(
        grant_by_column(&plan, "class_chassis.cleric.domain_spell_slot").is_none(),
        "domain spell slot count must not change 1 -> 2"
    );

    assert!(!plan.capstone_threshold);
    assert!(
        plan.pick_from_lists.is_empty(),
        "no domain-spell or feat candidate catalog exists yet; pick_from_lists must not be fabricated"
    );
    assert!(
        plan.resource_pool_change.pools.is_empty(),
        "Cleric's daily-use pools are flat ability-modifier formulas with no level term, so none \
         genuinely change size on a level-up"
    );
}

#[test]
fn cleric_level_2_to_3_grants_bab_reflex_channel_energy_and_domain_slot_rises() {
    let character = human_cleric_input(2);
    let plan = compute_level_up_grants(&character, 2, 3);

    let bab_grant = grant_by_column(&plan, "class_chassis.cleric.base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 2, "cleric level 3 3/4-BAB (3*3/4) must be +2");

    // Fortitude and Will stay +3 at level 3 (3/2+2 = 3, an integer-division
    // coincidence with level 2) -- must NOT produce a spurious grant.
    assert!(grant_by_column(&plan, "class_chassis.cleric.base_save.fortitude").is_none());
    assert!(grant_by_column(&plan, "class_chassis.cleric.base_save.will").is_none());

    // Reflex genuinely rises 0 -> 1 at level 3 (2/3 = 0, 3/3 = 1).
    let reflex_grant = grant_by_column(&plan, "class_chassis.cleric.base_save.reflex")
        .unwrap_or_else(|| panic!("expected a Reflex grant at level 3: {:?}", plan.automatic_features));
    assert_eq!(reflex_grant.effects[0].value, 1);

    // Channel Energy dice genuinely rises to 2d6 at level 3 (ceil(3/2) = 2,
    // the class table's level-3 "Special" column reads "Channel energy
    // 2d6").
    let channel_energy_grant = grant_by_column(&plan, "class_chassis.cleric.channel_energy_dice")
        .unwrap_or_else(|| panic!("expected a channel_energy_dice grant at level 3: {:?}", plan.automatic_features));
    assert_eq!(channel_energy_grant.effects[0].value, 2);

    // The domain spell slot count genuinely rises to 2 at level 3 (a
    // level-3 cleric casts 2nd-level cleric spells for the first time).
    let domain_slot_grant = grant_by_column(&plan, "class_chassis.cleric.domain_spell_slot")
        .unwrap_or_else(|| panic!("expected a domain_spell_slot grant at level 3: {:?}", plan.automatic_features));
    assert_eq!(domain_slot_grant.effects[0].value, 2);

    assert!(!plan.capstone_threshold);
}

#[test]
fn cleric_level_19_to_20_crosses_the_character_level_cap_with_no_fabricated_named_capstone() {
    let character = human_cleric_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    assert!(
        plan.capstone_threshold,
        "level 20 is PF1's universal character-level cap"
    );

    // Cleric has NO distinct named capstone class feature at level 20 (the
    // class table's level-20 "Special" column is genuinely blank) -- only
    // the ordinary base-attack / base-save / Touch-of-Good pillars keep
    // rising through the same generic diff, exactly as at every other
    // level. No feature named "capstone" or similar is fabricated.
    assert!(
        !plan
            .automatic_features
            .iter()
            .any(|grant| grant.name.to_lowercase().contains("capstone")),
        "Cleric has no named 20th-level capstone feature; none may be fabricated: {:?}",
        plan.automatic_features
    );

    let bab_grant = grant_by_column(&plan, "class_chassis.cleric.base_attack_bonus")
        .unwrap_or_else(|| panic!("expected a base_attack_bonus grant at the level cap: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects[0].value, 15, "cleric level 20 3/4-BAB (20*3/4) must be +15");

    let touch_of_good_grant =
        grant_by_column(&plan, "class_feature.domain.good_touch_of_good_bonus")
            .unwrap_or_else(|| panic!("expected a Touch of Good bonus grant at level 20: {:?}", plan.automatic_features));
    assert_eq!(touch_of_good_grant.effects[0].value, 10, "Touch of Good bonus (20/2) must be 10 at level 20");
}

#[test]
fn non_cleric_class_returns_an_honestly_empty_plan() {
    // Uses `class:oracle` — a genuinely unlanded PF1 class (not one of
    // Epic 7's 11 core classes). Originally used `class:wizard`
    // (matching Barbarian's and Bard's own negative controls at the
    // time), not `class:fighter` — Epic 7's fifth cycle
    // (cycle-2026-07-18, `fighter.rs`) landed a real Fighter dispatch arm,
    // so `class:fighter` no longer produced an honestly-empty plan and
    // would no longer prove this test's own claim. Epic 7's eleventh and
    // final cycle (`wizard.rs`) landed a real Wizard dispatch arm too, so
    // `class:wizard` was fixed forward to `class:oracle` in that same
    // commit (Epic 7's per-class order was barbarian, bard, cleric,
    // druid, fighter, monk, paladin, ranger, rogue, sorcerer, wizard —
    // all 11 are now landed, so no core class id can serve as an
    // "unlanded" negative control anymore).
    let mut character = human_cleric_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);

    // Sanity: dispatch still reaches Barbarian correctly (no cross-class
    // leakage introduced by wiring Cleric's new match arm).
    let barbarian = human_cleric_input(1);
    let mut barbarian = barbarian;
    barbarian.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:barbarian".to_string(),
        level: 1,
    }];
    let barbarian_plan = compute_level_up_grants(&barbarian, 1, 2);
    assert!(
        barbarian_plan
            .automatic_features
            .iter()
            .any(|grant| grant.source_table.table == "class_tables"),
        "Barbarian's own dispatch arm must be unaffected by Cleric's new match arm: {:?}",
        barbarian_plan.automatic_features
    );

    let _ = PickCategory::Spell;
}

#[test]
fn cleric_without_domain_selections_still_grounds_bab_saves_and_channel_energy() {
    // Absent domain-choice slots must not be fabricated (mirroring the
    // Fighter bonus-feat choice-slot precedent), but the class-generic
    // pillars (base attack bonus, base saves, Channel Energy, the flat
    // domain spell slot count) are independent of which domains were
    // chosen and must still ground.
    let mut character = human_cleric_input(1);
    character.chosen.selected_choices = Vec::new();
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(grant_by_column(&plan, "class_chassis.cleric.base_attack_bonus").is_some());
    assert!(grant_by_column(&plan, "class_chassis.cleric.base_save.fortitude").is_some());
    assert!(
        grant_by_column(&plan, "class_feature.domain.good_touch_of_good_bonus").is_none(),
        "Touch of Good must not be fabricated when the Good domain was never chosen"
    );
}
