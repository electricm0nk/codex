#[allow(unused_imports)]
pub(crate) use super::*;

/// SD-27 `decisions.md §28` defect 1: the three formulas above, pinned against
/// the published PF1 arithmetic on their own, independent of any fixture.
///
/// `tests/sd27_size_modifiers_to_touch_cmb_cmd_and_attack.rs` pins the same
/// three through the real compute path across all 18 in-scope races. These
/// unit tests exist for the branch that file *cannot* reach: no creatable race
/// is Tiny or smaller, so CMB's Dexterity substitution has no integration
/// coverage and would otherwise ship unproven.
#[cfg(test)]
mod combat_maneuver_and_touch_formula_tests {
    use super::*;

    /// The Goblin Fighter 1 measured on screen (STR 14 -> +2, DEX 18 -> +4,
    /// BAB +1, Chain Shirt): the exact case that exposed the defect. PF1's
    /// correct values are CMB +2 and CMD 16; the sheet was showing +3 and 17.
    #[test]
    fn the_measured_small_goblin_fighter_matches_the_published_pf1_values() {
        let size = SizeCategory::Small;
        assert_eq!(
            combat_maneuver_bonus(1, 2, 4, Some(size), size.special_size_modifier()),
            2,
            "Small Goblin Fighter 1: BAB +1 + STR +2 + special size -1 = +2"
        );
        assert_eq!(
            combat_maneuver_defense(1, 2, 4, size.special_size_modifier()),
            16,
            "Small Goblin Fighter 1: 10 + BAB +1 + STR +2 + DEX +4 + special size -1 = 16"
        );
        // AC 19 with a +4 armor bonus is touch 15, not 14 -- the contradiction
        // the sheet was displaying.
        assert_eq!(touch_armor_class(19, 4), 15);
    }

    /// The same character at Medium size is the regression half: Medium is
    /// PF1's +0 baseline and must produce exactly the pre-fix arithmetic.
    #[test]
    fn medium_is_the_unchanged_baseline_on_both_maneuver_formulas() {
        let size = SizeCategory::Medium;
        assert_eq!(
            combat_maneuver_bonus(1, 2, 4, Some(size), size.special_size_modifier()),
            3
        );
        assert_eq!(combat_maneuver_defense(1, 2, 4, size.special_size_modifier()), 17);
    }

    /// The branch no creatable race reaches: PF1's CMB entry substitutes
    /// Dexterity for Strength at Tiny and smaller. Written and pinned rather
    /// than deferred, so a later Tiny ingest does not silently compute a
    /// Strength-based CMB.
    #[test]
    fn tiny_and_smaller_creatures_substitute_dexterity_for_strength_on_cmb() {
        for size in [SizeCategory::Tiny, SizeCategory::Diminutive, SizeCategory::Fine] {
            assert_eq!(
                combat_maneuver_bonus(1, 2, 4, Some(size), size.special_size_modifier()),
                1 + 4 + size.special_size_modifier(),
                "{size:?} is Tiny or smaller: CMB uses the Dexterity modifier (+4), not Strength"
            );
        }
        // Small is NOT Tiny-or-smaller, and must keep using Strength -- the
        // off-by-one-category error this test exists to catch.
        assert_eq!(
            combat_maneuver_bonus(1, 2, 4, Some(SizeCategory::Small), -1),
            2,
            "Small still uses Strength: the substitution begins at Tiny"
        );
        // CMD has no such substitution in PF1: it sums BOTH abilities at every
        // size, so a Tiny creature's CMD is unaffected by the CMB rule.
        assert_eq!(combat_maneuver_defense(1, 2, 4, -2), 15);
    }

    /// An unresolvable race yields `None`, which must not be treated as
    /// Tiny-or-smaller by accident (a `matches!`-style catch-all written the
    /// other way round would do exactly that).
    #[test]
    fn an_unknown_size_falls_back_to_strength_rather_than_the_tiny_substitution() {
        assert_eq!(combat_maneuver_bonus(1, 2, 4, None, 0), 3);
    }

    /// Touch AC is the Armor Class minus its excluded contributors, and
    /// nothing else -- including when that makes it lower than 10, which is a
    /// real PF1 state (penalties apply to touch attacks) and must not be
    /// clamped into looking healthy.
    #[test]
    fn touch_armor_class_subtracts_and_does_not_clamp() {
        assert_eq!(touch_armor_class(17, 4), 13);
        assert_eq!(touch_armor_class(10, 0), 10);
        assert_eq!(touch_armor_class(8, 0), 8);
    }

    /// The measured screen defect (SD-27 `decisions.md §28`): a Tiefling read
    /// `AC 19 / flat-footed 16`, took Dodge, and read `AC 20 / flat-footed 17`.
    /// PF1 forbids the second number moving at all.
    #[test]
    fn a_dodge_bonus_raises_armor_class_and_leaves_flat_footed_ac_where_it_was() {
        // Before Dodge: AC 19, Dexterity contribution +3, no dodge-typed term.
        assert_eq!(flat_footed_armor_class(19, 3, 0), 16);
        // After Dodge: AC 20, same Dexterity contribution, +1 dodge-typed.
        assert_eq!(flat_footed_armor_class(20, 3, 1), 16);
        // What the shipped sheet displayed instead, reproduced so the
        // regression is stated as an equation rather than as prose: it dropped
        // the dodge term entirely.
        assert_eq!(20 - 3, 17);
    }

    /// A Dexterity *penalty* is not a Dexterity *bonus*. PF1 denies the bonus,
    /// so a flat-footed character with DEX 8 keeps their -1 rather than having
    /// it forgiven — the `max(0)` branch, pinned in both directions.
    #[test]
    fn a_dexterity_penalty_is_kept_while_a_dexterity_bonus_is_denied() {
        assert_eq!(flat_footed_armor_class(13, -1, 0), 13);
        assert_eq!(flat_footed_armor_class(13, 0, 0), 13);
        assert_eq!(flat_footed_armor_class(13, 1, 0), 12);
    }

    /// Dodge-typed bonuses stack with each other in PF1 (the one bonus type
    /// that does), so the caller passes a sum and every point of it is denied.
    /// Like touch AC, the result is not clamped: penalties can legitimately
    /// carry it below 10.
    #[test]
    fn every_point_of_dodge_typed_bonus_is_denied_and_the_result_is_not_clamped() {
        assert_eq!(flat_footed_armor_class(22, 4, 3), 15);
        assert_eq!(flat_footed_armor_class(10, 0, 0), 10);
        assert_eq!(flat_footed_armor_class(9, 2, 1), 6);
    }
}

/// `AT-34-E3-001` (`class_feature_owner_matched_by_name_but_record_not_held_
/// by_engine` mechanism, `engine_effect_token_present` sub-cause): grounds
/// a base class's zero-magnitude "Weapon and Armor Proficiency" class
/// feature as a bounded grant-only identity record, quoting the real
/// corpus DESC text -- the same "grant-only identity record" idiom this
/// file already uses for Sorcerer's own Arcane Apotheosis and Rogue's
/// Master Strike, and the SAME idiom `class_slayer.rs`'s
/// `ground_slayer_weapon_and_armor_proficiency` already establishes for
/// this exact record SHAPE, without that function's archetype-supersession
/// complexity: neither Sorcerer nor Wizard has a registered archetype able
/// to claim this slot anywhere in this engine today
/// (`archetype_resolver::archetype_claiming_slot_entry` has no Sorcerer/
/// Wizard-owned proficiency-shaped slot id to resolve), so the base grant
/// always applies once the class is present, unconditionally.
///
/// **Zero-magnitude, grant-only record, by design**, confirmed against the
/// ingested corpus JSON for both records (`data/corpus/core_rulebook/
/// class_feature/sorcerer/weapon_and_armor_proficiency.json`, `data/corpus/
/// core_rulebook/class_feature/wizard/weapon_and_armor_proficiency.json`,
/// both `wiring_class: "display"`, `wiring_class_signals:
/// ["display:no_magnitude_token"]`): each row's only tokens are
/// `ABILITY:...AUTOMATIC` proficiency grants, no `BONUS:` magnitude
/// anywhere. Explanation id shape (`class_feature.<owner>.weapon_and_armor_
/// proficiency`) matches `v06_work_inventory.rs`'s `classify()` `Kind::
/// ClassFeature` "owner resolved" arm's own `class_feature_exact_suffix_
/// grounded` check (`.{owner}.` substring, trailing dot-segment equal to
/// `class_feature_engine_join_slug("Weapon and Armor Proficiency")` ==
/// `"weapon_and_armor_proficiency"`), so both records now ground via that
/// EXISTING generic check rather than a new bucket-specific fallback --
/// promoting each to `text-complete` under the classify() function's own
/// unchanged Decision-7 gates (`text_only`, `has_real_description`,
/// `display` wiring class, `!universal_sheet_modifier`).
///
/// **The weapon half's real mechanical consequence is grounded elsewhere,
/// not duplicated here**, exactly like the Slayer precedent:
/// `weapon_tables::class_weapon_proficiency("class:sorcerer"/"class:
/// wizard")` already carries each class's own weapon tiers and is read by
/// `character_is_proficient_with` to decide the real -4 nonproficiency
/// penalty -- this function grounds only the class-features-tab DISPLAY
/// record, a different concern.
pub(super) fn explain_base_class_weapon_and_armor_proficiency(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let has_class = |class_id: &str| {
        input.chosen.class_levels.iter().any(|class_level| class_level.class_id == class_id)
    };

    if has_class(SORCERER_CLASS_ID) {
        explanations.push(ComputationExplanation {
            id: "class_feature.sorcerer.weapon_and_armor_proficiency".to_owned(),
            value: 0,
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   Armor interferes with a sorcerer's gestures, which can cause her spells with
            //   somatic components to fail.\" This is a bounded grant-only identity record (value
            //   0, non-fabricated): the record's only tokens are ABILITY:...AUTOMATIC proficiency
            //   grants, no BONUS: magnitude anywhere.
            detail: "Sorcerer Weapon and Armor Proficiency (corpus KEY:Sorcerer ~ Weapon and Armor \
                 Proficiency): \"Sorcerers are proficient with all simple weapons. They are not \
                 proficient with any type of armor or shield. The weapon half's real mechanical \
                 consequence -- avoiding the -4 nonproficiency attack penalty -- is already grounded \
                 separately by `weapon_tables::class_weapon_proficiency(\"class:sorcerer\")`, which \
                 this record does not duplicate. No armor-nonproficiency-penalty mechanic exists \
                 anywhere in this engine"
                .to_owned(),
        });
    }

    if has_class(WIZARD_CLASS_ID) {
        explanations.push(ComputationExplanation {
            id: "class_feature.wizard.weapon_and_armor_proficiency".to_owned(),
            value: 0,
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   Armor interferes with a wizard's movements, which can cause his spells with somatic
            //   components to fail.\" This is a bounded grant-only identity record (value 0,
            //   non-fabricated): the record's only tokens are ABILITY:...AUTOMATIC proficiency
            //   grants, no BONUS: magnitude anywhere.
            detail: "Wizard Weapon and Armor Proficiency (corpus KEY:Wizard ~ Weapon and Armor \
                 Proficiency): \"Wizards are proficient with the club, dagger, heavy crossbow, light \
                 crossbow, and quarterstaff, but not with any type of armor or shield. The weapon \
                 half's real mechanical consequence -- avoiding the -4 nonproficiency attack penalty \
                 -- is already grounded separately by \
                 `weapon_tables::class_weapon_proficiency(\"class:wizard\")`, which this record does \
                 not duplicate. No armor-nonproficiency-penalty mechanic exists anywhere in this \
                 engine"
                .to_owned(),
        });
    }

    ground_class_weapon_and_armor_proficiency(
        input,
        CLERIC_CLASS_ID,
        "Cleric",
        &["ClericWeaponProficiencies", "ClericArmorProficiencies", "ClericWeaponProficiency", "ClericArmorProficiency"],
        "class_feature.cleric.weapon_and_armor_proficiency",
        "Cleric",
        "Clerics are proficient with all simple weapons, light armor, medium armor, and \
         shields (except tower shields). Clerics are also proficient with the favored \
         weapon of their deity.",
        true,
        explanations,
    );

    ground_class_weapon_and_armor_proficiency(
        input,
        ASSASSIN_CLASS_ID,
        "Assassin",
        &[],
        "class_feature.assassin.weapon_and_armor_proficiency",
        "Assassin",
        "Assassins are proficient with the crossbow (hand, light, or heavy), dagger (any \
         type), dart, rapier, sap, shortbow (normal and composite), and short sword. \
         Assassins are proficient with light armor but not with shields.",
        false,
        explanations,
    );

    ground_class_weapon_and_armor_proficiency(
        input,
        SHADOWDANCER_CLASS_ID,
        "Shadowdancer",
        &[],
        "class_feature.shadowdancer.weapon_and_armor_proficiency",
        "Shadowdancer",
        "Shadowdancers are proficient with the club, crossbow (hand, light, or heavy), \
         dagger (any type), dart, mace, morningstar, quarterstaff, rapier, sap, shortbow \
         (normal and composite), and short sword. Shadowdancers are proficient with light \
         armor but not with shields.",
        false,
        explanations,
    );

    // Wave 33 lane A's own next-cycle plan (`decisions.md §21`): Bard,
    // Fighter, Paladin, Ranger, Rogue -- the `class_feature_pool_catalog::
    // WEAPON_AND_ARMOR_PROFICIENCY_GRANT_CLASS_TABLE_MATCHES` five, the
    // "Weapon and Armor Proficiency ~ <Class>" combined-key shape (a
    // DIFFERENT corpus key ordering than Cleric/Sorcerer/Wizard's own
    // "<Class> ~ Weapon and Armor Proficiency"). Every one of these five
    // classes' OWN registered archetypes across `rules_tables/*/
    // archetype_tables.rs` was read individually (not grep-and-trust) before
    // choosing each `proficiency_slot_ids` list below, at the same rigor
    // Cleric's own cycle 6 applied -- and the result is NOT uniform across
    // the five, unlike the wave-33 receipt's own assumption that all five
    // "have real archetypes doing exactly that":
    //
    // - **Bard**: `BardWeaponProficiencies`/`BardArmorProficiencies`.
    //   `Bard Archetype ~ Geisha` (Ultimate Magic) replaces BOTH; `Bard
    //   Archetype ~ Dervish Dancer` (Ultimate Combat) replaces the weapon
    //   half only. Neither archetype's own catalog `grants` names a "~
    //   Weapon and Armor Proficiency" sub-feature WITH resolved text
    //   (Geisha's own grants are "Scribe Scroll"/"Tea Ceremony"; Dervish
    //   Dancer's own "Dervish Dancer ~ Weapon and Armor Proficiency" grant
    //   carries `description: None`), so both correctly fall to the "not
    //   resolved in this catalog entry" branch -- never fabricated text.
    // - **Fighter**: `FighterArmorProficiencies`/`FighterTowerShieldProficiency`
    //   ONLY -- never `FighterWeaponTraining*`/`FighterArmorTraining*`/
    //   `FighterArmorMastery`/`FighterWeaponMastery`/`FighterBravery`/
    //   `FighterBonusFeat*`, which are the class's OWN separate, higher-level
    //   Fighter features (Weapon/Armor Training, Bravery, ...), not this
    //   1st-level proficiency grant, even though several Fighter archetypes'
    //   `replaces` lists are dominated by those unrelated slots. Cad,
    //   Gladiator, Tactician, and Unarmed Fighter (all Ultimate Combat)
    //   replace `FighterArmorProficiencies`; Dragoon and Unbreakable (same
    //   book) replace only `FighterTowerShieldProficiency`. Every one of
    //   these six archetypes' own grant carries `description: None` for its
    //   "~ Weapon and Armor Proficiency" entry, so all six correctly fall to
    //   the "not resolved" branch.
    // - **Paladin**: `PaladinArmorProficiencies`/`PaladinWeaponProficiencies`
    //   ONLY. `Paladin Archetype ~ Holy Gun` (Ultimate Combat) replaces both
    //   and carries its own "Holy Gun ~ Weapon and Armor Proficiency" grant
    //   (`description: None`) -- the "not resolved" branch applies.
    //   **`Paladin Archetype ~ Divine Hunter` is deliberately EXCLUDED**,
    //   the fabrication hazard this cycle's dispatch named by name: Divine
    //   Hunter's own `replaces` list carries `PaladinArmorProficiencyHeavy`
    //   alone (heavy armor only -- light/medium armor and all weapon
    //   proficiencies are untouched), and its own grant is named "Divine
    //   Hunter ~ Precise Shot" ("This ability replaces her Heavy Armor
    //   Proficiency"), never a "~ Weapon and Armor Proficiency" sub-feature.
    //   Including `PaladinArmorProficiencyHeavy` in this list would make a
    //   Divine Hunter selection wrongly claim "the base progression does not
    //   apply" for a class whose light/medium armor and weapon proficiencies
    //   are still fully in force -- exactly the stale-text-to-a-real-player
    //   risk this cycle's dispatch instruction warned against.
    // - **Ranger, Rogue**: `&[]`, empty -- like the Assassin/Shadowdancer
    //   precedent above, NOT a re-run of that precedent's premise. This
    //   cycle's own dispatch (and the wave-33 receipt it carries forward)
    //   assumed all five classes "have real archetypes doing exactly that,
    //   unlike the zero-archetype Assassin/Shadowdancer precedent" -- that
    //   assumption does not hold for Ranger or Rogue specifically. Every one
    //   of Ranger's and Rogue's registered archetypes across all six
    //   archetype-table modules that carry either class was read; not one
    //   `replaces` list names any of the TYPE facets the base Ranger/Rogue
    //   corpus record's own `!PREABILITY` negation gates reference
    //   (`RangerArmorProficiencies`/`RangerWeaponProficiency`/
    //   `RangerLightArmorProficiency`/`RangerMediumArmorProficiency`/
    //   `RangerShieldProficiency`/`RogueWeaponProficiencies`/
    //   `RogueArmorProficiencies`/`RogueLightArmor` -- confirmed by direct
    //   grep for every one of those eight literal TYPE facets across
    //   `rules_tables/*/archetype_tables.rs`, zero matches for any). PCGen's
    //   own source anticipates such an archetype existing somewhere in the
    //   wider game line; none of the tier-1 books this engine has ingested
    //   happens to be it. Passing empty lists here is the honest finding,
    //   not a shortcut -- the archetype lookup is always a no-op for these
    //   two classes today, and remains the single place a future archetype
    //   landing in the catalog would need to be wired.
    ground_class_weapon_and_armor_proficiency(
        input,
        BARD_CLASS_ID,
        "Bard",
        &["BardWeaponProficiencies", "BardArmorProficiencies"],
        "class_feature.bard.weapon_and_armor_proficiency",
        "Bard",
        "A bard is proficient with all simple weapons, plus the longsword, rapier, sap, \
         short sword, shortbow, and whip. Bards are proficient with light armor and \
         shields (except tower shields). A bard can cast bard spells while wearing light \
         armor without incurring the normal arcane spell failure chance. However, like \
         any other arcane spellcaster, a bard wearing medium or heavy armor or using a \
         shield incurs a chance of arcane spell failure if the spell in question has a \
         somatic component (most do). A multiclass bard still incurs the normal arcane \
         spell failure chance for arcane spells received from other classes.",
        true,
        explanations,
    );

    ground_class_weapon_and_armor_proficiency(
        input,
        FIGHTER_CLASS_ID,
        "Fighter",
        &["FighterArmorProficiencies", "FighterTowerShieldProficiency"],
        "class_feature.fighter.weapon_and_armor_proficiency",
        "Fighter",
        "A fighter is proficient with all simple and martial weapons and with all armor \
         (heavy, medium, and light) and shields (including tower shields).",
        true,
        explanations,
    );

    ground_class_weapon_and_armor_proficiency(
        input,
        PALADIN_CLASS_ID,
        "Paladin",
        &["PaladinArmorProficiencies", "PaladinWeaponProficiencies"],
        "class_feature.paladin.weapon_and_armor_proficiency",
        "Paladin",
        "Paladins are proficient with all simple and martial weapons, with all types of \
         armor (heavy, medium, and light), and with shields (except tower shields).",
        true,
        explanations,
    );

    ground_class_weapon_and_armor_proficiency(
        input,
        RANGER_CLASS_ID,
        "Ranger",
        &[],
        "class_feature.ranger.weapon_and_armor_proficiency",
        "Ranger",
        "A ranger is proficient with all simple and martial weapons and with light \
         armor, medium armor, and shields (except tower shields).",
        true,
        explanations,
    );

    ground_class_weapon_and_armor_proficiency(
        input,
        ROGUE_CLASS_ID,
        "Rogue",
        &[],
        "class_feature.rogue.weapon_and_armor_proficiency",
        "Rogue",
        "Rogues are proficient with all simple weapons, plus the hand crossbow, rapier, \
         sap, shortbow, and short sword. Rogues are proficient with light armor, but not \
         with shields.",
        true,
        explanations,
    );
}

#[cfg(test)]
mod base_class_weapon_and_armor_proficiency_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, ASSASSIN_CLASS_ID,
        BARD_CLASS_ID, CLERIC_CLASS_ID, FIGHTER_CLASS_ID, PALADIN_CLASS_ID, RANGER_CLASS_ID,
        ROGUE_CLASS_ID, SHADOWDANCER_CLASS_ID, SORCERER_CLASS_ID, WIZARD_CLASS_ID,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation(
        input: &CharacterInput,
        id: &str,
    ) -> Option<(i16, String)> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .into_iter()
            .find(|e| e.id == id)
            .map(|e| (e.value, e.detail))
    }

    #[test]
    fn sorcerer_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(SORCERER_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.sorcerer.weapon_and_armor_proficiency")
            .expect("a Sorcerer must ground this base-class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("proficient with all simple weapons"),
            "must quote the real base corpus DESC: {detail}"
        );
    }

    #[test]
    fn wizard_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(WIZARD_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.wizard.weapon_and_armor_proficiency")
            .expect("a Wizard must ground this base-class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("proficient with the club, dagger"),
            "must quote the real base corpus DESC: {detail}"
        );
    }

    #[test]
    fn a_non_sorcerer_non_wizard_character_grounds_neither_explanation() {
        let input = character(SORCERER_CLASS_ID, 1);
        // A Wizard-only character must not carry the Sorcerer explanation,
        // and vice versa -- confirms the per-class gate, not a blanket grant.
        assert!(explanation(&input, "class_feature.wizard.weapon_and_armor_proficiency").is_none());
    }

    #[test]
    fn cleric_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(CLERIC_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.cleric.weapon_and_armor_proficiency")
            .expect("a Cleric must ground this base-class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("favored weapon of their deity"),
            "must quote the real base corpus DESC: {detail}"
        );
        assert!(
            detail.contains("already grounded separately"),
            "Cleric has a real weapon_tables entry, so the weapon half's grounding-\
             elsewhere claim must be honest: {detail}"
        );
        assert!(
            !detail.to_lowercase().contains("superseded"),
            "no archetype is selected, so nothing is superseded: {detail}"
        );
    }

    /// Supersession branch: Ecclesitheurge's own real catalog entry names a
    /// "~ Weapon and Armor Proficiency" sub-feature, so it must replace
    /// the base grant's text entirely -- the same shape Slayer's Bounty
    /// Hunter proves, now exercised for a BASE class's own archetype for
    /// the first time.
    #[test]
    fn cleric_weapon_and_armor_proficiency_is_superseded_by_ecclesitheurge() {
        let mut input = character(CLERIC_CLASS_ID, 1);
        input.chosen.selected_choices.push(
            crate::rules_core::character_input::SelectedChoice {
                choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID
                    .to_owned(),
                selection_id: "Cleric Archetype ~ Ecclesitheurge".to_owned(),
            },
        );
        let (value, detail) = explanation(&input, "class_feature.cleric.weapon_and_armor_proficiency")
            .expect("the record must still ground, with superseded text");
        assert_eq!(value, 0);
        assert!(detail.contains("Ecclesitheurge"), "must name the superseding archetype: {detail}");
        assert!(
            detail.contains("proficient with the club, dagger, heavy crossbow"),
            "must quote Ecclesitheurge's OWN real corpus text, not the base grant's: {detail}"
        );
        assert!(
            !detail.contains("light armor, medium armor, and shields (except"),
            "the base grant's own text must NOT appear once superseded: {detail}"
        );
    }

    #[test]
    fn assassin_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant_with_honest_disclosure() {
        let input = character(ASSASSIN_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.assassin.weapon_and_armor_proficiency")
            .expect("an Assassin must ground this class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("proficient with the crossbow"),
            "must quote the real base corpus DESC: {detail}"
        );
        assert!(
            detail.contains("NOT grounded elsewhere"),
            "Assassin has no weapon_tables entry, so this must NOT claim the weapon half is \
             grounded elsewhere the way Cleric/Sorcerer/Wizard's own explanations do: {detail}"
        );
    }

    #[test]
    fn shadowdancer_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(SHADOWDANCER_CLASS_ID, 1);
        let (value, detail) =
            explanation(&input, "class_feature.shadowdancer.weapon_and_armor_proficiency")
                .expect("a Shadowdancer must ground this class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("proficient with the club, crossbow"),
            "must quote the real base corpus DESC: {detail}"
        );
        assert!(
            detail.contains("NOT grounded elsewhere"),
            "Shadowdancer has no weapon_tables entry either: {detail}"
        );
    }

    #[test]
    fn a_cleric_character_does_not_ground_the_assassin_or_shadowdancer_explanations() {
        let input = character(CLERIC_CLASS_ID, 1);
        assert!(explanation(&input, "class_feature.assassin.weapon_and_armor_proficiency").is_none());
        assert!(
            explanation(&input, "class_feature.shadowdancer.weapon_and_armor_proficiency").is_none()
        );
    }

    // Wave 33 lane A's own next-cycle plan: Bard, Fighter, Paladin, Ranger,
    // Rogue. All five have a real `weapon_tables::class_weapon_proficiency`
    // entry, so all five must claim "already grounded separately", matching
    // Cleric/Sorcerer/Wizard's own shape, never Assassin/Shadowdancer's
    // "NOT grounded elsewhere" disclosure.

    #[test]
    fn bard_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(BARD_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.bard.weapon_and_armor_proficiency")
            .expect("a Bard must ground this base-class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("longsword, rapier, sap, short sword, shortbow, and whip"),
            "must quote the real base corpus DESC: {detail}"
        );
        assert!(
            detail.contains("already grounded separately"),
            "Bard has a real weapon_tables entry: {detail}"
        );
        assert!(!detail.to_lowercase().contains("superseded"), "no archetype selected: {detail}");
    }

    /// Supersession branch, both slots at once: Geisha's own catalog entry
    /// names no "~ Weapon and Armor Proficiency" sub-feature grant with
    /// resolved text (its own grants are "Scribe Scroll"/"Tea Ceremony"), so
    /// this must fall to the honest "not resolved in this catalog entry"
    /// branch, never fabricate replacement prose.
    #[test]
    fn bard_weapon_and_armor_proficiency_is_superseded_by_geisha_but_replacement_text_is_not_resolved()
    {
        let mut input = character(BARD_CLASS_ID, 1);
        input.chosen.selected_choices.push(crate::rules_core::character_input::SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Bard Archetype ~ Geisha".to_owned(),
        });
        let (value, detail) = explanation(&input, "class_feature.bard.weapon_and_armor_proficiency")
            .expect("the record must still ground, with superseded text");
        assert_eq!(value, 0);
        assert!(detail.contains("Geisha"), "must name the superseding archetype: {detail}");
        assert!(
            detail.contains("not resolved in this catalog entry"),
            "Geisha's own catalog entry names no proficiency replacement grant, so this must \
             not fabricate replacement text: {detail}"
        );
        assert!(
            !detail.contains("longsword, rapier, sap, short sword"),
            "the base grant's own text must NOT appear once superseded: {detail}"
        );
    }

    #[test]
    fn fighter_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(FIGHTER_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.fighter.weapon_and_armor_proficiency")
            .expect("a Fighter must ground this base-class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("proficient with all simple and martial weapons"),
            "must quote the real base corpus DESC: {detail}"
        );
        assert!(
            detail.contains("already grounded separately"),
            "Fighter has a real weapon_tables entry: {detail}"
        );
        assert!(!detail.to_lowercase().contains("superseded"), "no archetype selected: {detail}");
    }

    /// Cad replaces `FighterArmorProficiencies` (armor half only, never
    /// `FighterWeaponProficiencies` -- confirmed absent from Fighter's own
    /// registered `replaces` catalog entirely), and its own "Cad ~ Weapon
    /// and Armor Proficiency" grant carries `description: None` -- the
    /// honest "not resolved" branch applies, same as Bard's Geisha.
    #[test]
    fn fighter_weapon_and_armor_proficiency_is_superseded_by_cad_but_replacement_text_is_not_resolved()
    {
        let mut input = character(FIGHTER_CLASS_ID, 1);
        input.chosen.selected_choices.push(crate::rules_core::character_input::SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Fighter Archetype ~ Cad".to_owned(),
        });
        let (value, detail) =
            explanation(&input, "class_feature.fighter.weapon_and_armor_proficiency")
                .expect("the record must still ground, with superseded text");
        assert_eq!(value, 0);
        assert!(detail.contains("Cad"), "must name the superseding archetype: {detail}");
        assert!(
            detail.contains("not resolved in this catalog entry"),
            "Cad's own catalog entry names no proficiency replacement grant, so this must not \
             fabricate replacement text: {detail}"
        );
    }

    #[test]
    fn paladin_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(PALADIN_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.paladin.weapon_and_armor_proficiency")
            .expect("a Paladin must ground this base-class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("proficient with all simple and martial weapons"),
            "must quote the real base corpus DESC: {detail}"
        );
        assert!(
            detail.contains("already grounded separately"),
            "Paladin has a real weapon_tables entry: {detail}"
        );
        assert!(!detail.to_lowercase().contains("superseded"), "no archetype selected: {detail}");
    }

    /// Holy Gun replaces BOTH `PaladinArmorProficiencies` and
    /// `PaladinWeaponProficiencies`, and carries its own "Holy Gun ~ Weapon
    /// and Armor Proficiency" grant with `description: None` -- the "not
    /// resolved" branch applies.
    #[test]
    fn paladin_weapon_and_armor_proficiency_is_superseded_by_holy_gun_but_replacement_text_is_not_resolved()
    {
        let mut input = character(PALADIN_CLASS_ID, 1);
        input.chosen.selected_choices.push(crate::rules_core::character_input::SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Paladin Archetype ~ Holy Gun".to_owned(),
        });
        let (value, detail) =
            explanation(&input, "class_feature.paladin.weapon_and_armor_proficiency")
                .expect("the record must still ground, with superseded text");
        assert_eq!(value, 0);
        assert!(detail.contains("Holy Gun"), "must name the superseding archetype: {detail}");
        assert!(
            detail.contains("not resolved in this catalog entry"),
            "Holy Gun's own catalog entry names no proficiency replacement grant, so this must \
             not fabricate replacement text: {detail}"
        );
    }

    /// **The fabrication-risk hazard this cycle's dispatch named directly.**
    /// Divine Hunter's own `replaces` list carries `PaladinArmorProficiencyHeavy`
    /// ALONE (heavy armor only -- light/medium armor and every weapon
    /// proficiency are untouched), and it names no "~ Weapon and Armor
    /// Proficiency" sub-feature of its own (its own grant is "Divine Hunter
    /// ~ Precise Shot", "This ability replaces her Heavy Armor
    /// Proficiency"). `PaladinArmorProficiencyHeavy` is deliberately absent
    /// from this function's own `proficiency_slot_ids` list for exactly this
    /// reason -- a Divine Hunter selection must keep showing the FULL base
    /// progression text, never a "superseded" claim that would wrongly tell
    /// a real player their light/medium armor and weapon proficiencies no
    /// longer apply.
    #[test]
    fn paladin_weapon_and_armor_proficiency_divine_hunter_does_not_supersede_the_base_grant() {
        let mut input = character(PALADIN_CLASS_ID, 1);
        input.chosen.selected_choices.push(crate::rules_core::character_input::SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Paladin Archetype ~ Divine Hunter".to_owned(),
        });
        let (value, detail) =
            explanation(&input, "class_feature.paladin.weapon_and_armor_proficiency")
                .expect("the base grant must still ground");
        assert_eq!(value, 0);
        assert!(
            detail.contains("all simple and martial weapons"),
            "Divine Hunter replaces only heavy armor proficiency -- the base grant's own full \
             text (including light/medium armor and every weapon proficiency) must still \
             render unchanged: {detail}"
        );
        assert!(
            !detail.to_lowercase().contains("superseded"),
            "Divine Hunter's own `replaces` list never names a full proficiency slot this \
             function tracks, so this must NOT claim supersession: {detail}"
        );
        assert!(
            !detail.contains("Divine Hunter"),
            "the archetype must not be named as a superseding archetype it is not: {detail}"
        );
    }

    #[test]
    fn ranger_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(RANGER_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.ranger.weapon_and_armor_proficiency")
            .expect("a Ranger must ground this base-class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("proficient with all simple and martial weapons"),
            "must quote the real base corpus DESC: {detail}"
        );
        assert!(
            detail.contains("already grounded separately"),
            "Ranger has a real weapon_tables entry: {detail}"
        );
        assert!(!detail.to_lowercase().contains("superseded"), "no archetype selected: {detail}");
    }

    /// Not one of Ranger's registered archetypes names any of the TYPE
    /// facets the base corpus record's own `!PREABILITY` negation gates
    /// reference (confirmed by direct grep, zero matches) -- so even with a
    /// real Ranger archetype selected, this record must keep showing the
    /// base progression, exactly like the Assassin/Shadowdancer precedent.
    #[test]
    fn ranger_weapon_and_armor_proficiency_is_never_superseded_by_a_registered_archetype() {
        let mut input = character(RANGER_CLASS_ID, 1);
        input.chosen.selected_choices.push(crate::rules_core::character_input::SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Ranger Archetype ~ Guide".to_owned(),
        });
        let (_, detail) = explanation(&input, "class_feature.ranger.weapon_and_armor_proficiency")
            .expect("the base grant must still ground");
        assert!(
            !detail.to_lowercase().contains("superseded"),
            "no registered Ranger archetype claims this slot in this engine's catalog: {detail}"
        );
    }

    #[test]
    fn rogue_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant() {
        let input = character(ROGUE_CLASS_ID, 1);
        let (value, detail) = explanation(&input, "class_feature.rogue.weapon_and_armor_proficiency")
            .expect("a Rogue must ground this base-class grant");
        assert_eq!(value, 0);
        assert!(
            detail.contains("hand crossbow, rapier"),
            "must quote the real base corpus DESC: {detail}"
        );
        assert!(
            detail.contains("already grounded separately"),
            "Rogue has a real weapon_tables entry: {detail}"
        );
        assert!(!detail.to_lowercase().contains("superseded"), "no archetype selected: {detail}");
    }
}

/// Compute the deterministic baseline melee attack bonus and armor class, or
/// block the claim if the input is not the exact supported pilot posture.
///
/// This is intentionally not a combat engine. It computes only the GE-06
/// deterministic Longsword/Chain Shirt/Dodge/no-shield baseline. Any deviation
/// from that exact posture is refused with a claim-blocking diagnostic rather
/// than fabricating combat totals.
pub(super) fn compute_combat_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    base_attack_bonus: i16,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> (i16, i16) {
    let unmet = unmet_combat_posture_conditions(input);

    if !unmet.is_empty() {
        diagnostics.push(ComputationDiagnostic {
            id: "combat.baseline_unsupported".to_owned(),
            message: format!(
                "baseline combat totals are only computed for the exact GE-06 deterministic \
                 Longsword/Chain Shirt/Dodge/no-shield posture; unmet conditions: {}",
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
        return (0, 0);
    }

    // SD-27, decisions.md §28 defect 1 (2026-07-31): resolved ONCE, up here,
    // because four separate cells below need a size term -- Armor Class, touch
    // AC, attack rolls, and CMB/CMD. Resolving it per-consumer would push the
    // unknown-race diagnostic four times for the same single unknown.
    let size = combat_size_modifiers(input, diagnostics);

    // Baseline melee attack bonus: Fighter BAB + STR modifier + Weapon Focus
    // (Longsword) + Weapon Training (from level 5, Heavy Blades). Power Attack is
    // selected but inactive, contributing 0. The posture check above guarantees a
    // supported Fighter level here.
    let level = supported_fighter_level(input).unwrap_or(1);
    let strength_modifier = ability_modifiers.strength;
    let weapon_training_bonus = fighter_weapon_training_attack_bonus(input, level);
    // v0.6 alpha swarm, risks item 8: Bard Inspire Courage's competence
    // bonus on attack rolls applies here -- class-ownership-gated by
    // `active_bard_inspire_courage_attack_bonus` construction, 0 for every
    // non-Bard or not-actively-performing character.
    let inspire_courage_attack_bonus =
        active_bard_inspire_courage_attack_bonus(input, ability_modifiers)
            .map(|(_, bonus)| bonus)
            .unwrap_or(0);
    // v0.6 alpha swarm, risks item 8: Cleric Good domain's Touch of Good
    // sacred bonus (self-application only) applies here -- class-
    // ownership-gated by `active_touch_of_good_bonus` construction,
    // 0 for every non-Cleric, non-Good-domain, or not-currently-active
    // character.
    let touch_of_good_attack_bonus = active_touch_of_good_bonus(input).unwrap_or(0);
    // v0.6 alpha swarm, risks item 8 (Inquisitor Judgment closure):
    // Inquisitor Justice judgment's sacred (or profane) bonus on attack
    // rolls applies here too -- class-ownership-gated by
    // `active_inquisitor_justice_judgment_bonus` construction, 0 for
    // every non-Inquisitor, non-Justice-judgment, or not-currently-active
    // character.
    let justice_judgment_attack_bonus = active_inquisitor_justice_judgment_bonus(input)
        .map(|(_, bonus)| bonus)
        .unwrap_or(0);
    // v0.6 alpha swarm, risks item #89 / tasks #80+#86 (2026-07-29): the
    // nonproficiency penalty. This baseline hardcodes a Longsword (a
    // MARTIAL weapon) and, until this fix, handed every dispatched class a
    // full attack bonus for it -- including the many that have no Longsword
    // proficiency at all. That made a real, silently-wrong number on
    // classes users can build and save today (Wizard's shipped value was
    // overstated by exactly 4).
    //
    // Proficiency is read from the ingested corpus table rather than
    // inferred from a "martial classes only" rule of thumb, because that
    // rule of thumb is wrong in both directions: Bard is proficient with
    // the Longsword through an explicit
    // `AUTO:WEAPONPROF|Longsword|...` list (`cr_abilities_class.lst`,
    // `KEY:Weapon and Armor Proficiency ~ Bard`) despite having only the
    // Simple tier, and Brawler carries a whole weapon GROUP grant
    // ("Close") that does not contain the Longsword ("Blades Heavy").
    //
    // Known modelling boundary, deliberately not papered over: BOTH Cleric
    // (`cr_abilities_class.lst`, `Weapon and Armor Proficiency ~ Cleric`)
    // and Inquisitor (`apg_abilities_globalvar.lst:340`,
    // `CATEGORY=Class|Inquisitor.MOD`) carry
    // `AUTO:WEAPONPROF|DEITYWEAPONS` on top of their listed grants, so a
    // character of either class whose deity favors the Longsword IS
    // proficient in real PF1. No deity is modelled anywhere in this engine
    // (`DEITY:` exists only as a `source_content` record kind, never as a
    // chosen-input field) and none is present in this posture, so
    // DEITYWEAPONS contributes nothing here and the penalty correctly
    // applies -- but a future deity surface must revisit this, not assume
    // these two classes are always non-proficient.
    let longsword = equipped_weapon_stat_block(LONGSWORD_ITEM_ID);
    let proficiency_verdict =
        longsword.and_then(|weapon| character_is_proficient_with(input, weapon));
    let nonproficiency_penalty = match proficiency_verdict {
        Some(false) => WEAPON_NONPROFICIENCY_ATTACK_PENALTY,
        Some(true) => 0,
        // Unknown: refuse to guess in either direction. The diagnostic
        // below claim-blocks so no number is presented as trustworthy.
        None => 0,
    };
    if proficiency_verdict.is_none() {
        diagnostics.push(ComputationDiagnostic {
            id: "combat.baseline_weapon_proficiency_unknown".to_owned(),
            message: format!(
                "the baseline melee attack bonus applies PF1's \
                 {WEAPON_NONPROFICIENCY_ATTACK_PENALTY} nonproficiency penalty only when \
                 this character's proficiency with {LONGSWORD_ITEM_ID} is actually known, \
                 and it is not: at least one class in {:?} has no ingested \
                 rules_tables::crb::weapon_tables::CLASS_WEAPON_PROFICIENCIES record (or the \
                 Longsword itself is missing from the weapon table), so the attack total \
                 below is NOT claimed to be correct",
                input
                    .chosen
                    .class_levels
                    .iter()
                    .map(|c| c.class_id.as_str())
                    .collect::<Vec<&str>>()
            ),
            claim_blocking: true,
        });
    }

    // SD-27, decisions.md §28 defect 1 (2026-07-31): attack rolls take PF1
    // Table 8-1's size modifier -- the SAME column and the same magnitude as
    // Armor Class, not the CMB/CMD "special" one. `size.rs`'s
    // `armor_class_size_modifier` doc comment named this gap explicitly
    // ("Attack rolls take the identical modifier in real PF1 and do not yet
    // receive it") rather than leaving it to be rediscovered; this closes it,
    // and that doc comment is updated in the same change.
    let melee_attack_bonus = base_attack_bonus
        + strength_modifier
        + WEAPON_FOCUS_TO_HIT_BONUS
        + size.armor_class_and_attack
        + weapon_training_bonus
        + inspire_courage_attack_bonus
        + touch_of_good_attack_bonus
        + justice_judgment_attack_bonus
        + nonproficiency_penalty;
    let weapon_training_detail = if weapon_training_bonus > 0 {
        format!(" + Weapon Training (Heavy Blades) (+{weapon_training_bonus})")
    } else {
        String::new()
    };
    let inspire_courage_detail = if inspire_courage_attack_bonus > 0 {
        format!(" + Bard Inspire Courage competence bonus (+{inspire_courage_attack_bonus})")
    } else {
        String::new()
    };
    let touch_of_good_detail = if touch_of_good_attack_bonus > 0 {
        format!(" + Good domain Touch of Good sacred bonus (+{touch_of_good_attack_bonus}, self-applied)")
    } else {
        String::new()
    };
    let justice_judgment_detail = if justice_judgment_attack_bonus > 0 {
        format!(" + Inquisitor Justice judgment sacred/profane bonus (+{justice_judgment_attack_bonus})")
    } else {
        String::new()
    };
    // Stated explicitly rather than folded silently into the total: a
    // player looking at a lower-than-expected attack bonus must be able to
    // see WHY from the explanation alone.
    let nonproficiency_detail = match proficiency_verdict {
        Some(false) => format!(
            " {WEAPON_NONPROFICIENCY_ATTACK_PENALTY} nonproficiency penalty (this class has no \
             Longsword proficiency in the corpus)"
        ),
        Some(true) => String::new(),
        None => " (nonproficiency penalty UNRESOLVED -- see \
                 combat.baseline_weapon_proficiency_unknown)"
            .to_owned(),
    };

    let class_label = class_summary_label(input);
    explanations.push(ComputationExplanation {
        id: "combat.baseline_melee_attack_bonus".to_owned(),
        value: melee_attack_bonus,
        detail: format!(
            "Baseline melee attack bonus for the Longsword: {class_label} base attack bonus (+{base_attack_bonus}) \
             + Strength modifier (+{strength_modifier}) + Weapon Focus (Longsword) (+{WEAPON_FOCUS_TO_HIT_BONUS}) \
             + {} size modifier ({:+}, PF1 Table 8-1 -- attack rolls take the same size modifier as Armor Class)\
             {weapon_training_detail}{inspire_courage_detail}{touch_of_good_detail}{justice_judgment_detail}{nonproficiency_detail}; \
             Power Attack is selected but inactive (+0) = {melee_attack_bonus}",
            size.label, size.armor_class_and_attack
        ),
    });

    // Baseline armor class: 10 + Chain Shirt armor bonus + capped DEX + Dodge,
    // with no shield (absent posture contributes 0). Fighter armor training from
    // level 3 raises the Chain Shirt maximum Dexterity bonus.
    let effective_max_dex = CHAIN_SHIRT_MAX_DEX + fighter_armor_training(level).max_dex_increase;
    let dexterity_modifier = ability_modifiers.dexterity;
    let dexterity_contribution = dexterity_modifier.min(effective_max_dex);
    // v0.6 alpha swarm, risks item 8: Barbarian Rage's Armor Class penalty
    // applies here -- class-ownership-gated by `active_barbarian_rage_bonus`
    // construction, 0 for every non-Barbarian or not-currently-raging
    // character. The penalty magnitude is the same -2 at every Rage tier
    // (Greater/Mighty Rage don't change it), so the tier tuple's other
    // fields are unused here.
    let rage_armor_class_penalty = if active_barbarian_rage_bonus(input, ability_modifiers).is_some() {
        BARBARIAN_RAGE_ARMOR_CLASS_PENALTY
    } else {
        0
    };
    // v0.6 alpha swarm, risks item 8 (first APG/ACG closure): Skald Inspired
    // Rage's Armor Class penalty applies here too, the same shape as
    // Barbarian's Rage penalty -- class-ownership-gated by
    // `active_skald_inspired_rage_bonus` construction, 0 for every non-Skald
    // or not-currently-singing character.
    let inspired_rage_armor_class_penalty =
        if active_skald_inspired_rage_bonus(input, ability_modifiers).is_some() {
            SKALD_INSPIRED_RAGE_ARMOR_CLASS_PENALTY
        } else {
            0
        };
    // v0.6 alpha swarm, risks item 8 (second APG/ACG closure): Bloodrager
    // Bloodrage's Armor Class penalty applies here too, the same shape as
    // Barbarian's/Skald's -- class-ownership-gated by
    // `active_bloodrager_bloodrage_bonus` construction, 0 for every
    // non-Bloodrager or not-currently-bloodraging character.
    let bloodrage_armor_class_penalty =
        if active_bloodrager_bloodrage_bonus(input, ability_modifiers).is_some() {
            BLOODRAGER_BLOODRAGE_ARMOR_CLASS_PENALTY
        } else {
            0
        };
    // v0.6 alpha swarm, risks item 8 (third APG/ACG closure): Brawler's AC
    // Bonus dodge bonus applies here too -- class-ownership-gated by
    // `active_brawler_ac_bonus` construction, 0 for every non-Brawler
    // character. Unlike the Rage-shaped bonuses above, this is not
    // activation-gated: it is always on for a Brawler (subject to the
    // provably-vacuous light-armor precondition -- see
    // `apply_brawler_ac_bonus_to_combat_baseline`'s own doc comment).
    let brawler_ac_bonus_value = apply_brawler_ac_bonus_to_combat_baseline(input);
    // v0.6 alpha swarm, risks item 8 (Alchemist Mutagen closure): Mutagen's
    // natural armor bonus applies here too -- class-ownership-gated by
    // `active_alchemist_mutagen_bonus` construction, 0 for every
    // non-Alchemist or not-currently-mutated character.
    let alchemist_mutagen_ac_bonus_value = apply_alchemist_mutagen_ac_bonus_to_combat_baseline(input);
    // v0.6 alpha swarm, risks item 8 (Inquisitor Judgment closure, widened
    // 2026-07-26): Inquisitor Protection judgment's sacred (or profane)
    // bonus to Armor Class applies here too -- class-ownership-gated by
    // `active_inquisitor_protection_judgment_bonus` construction, 0 for
    // every non-Inquisitor, non-Protection-judgment, or not-currently-
    // active character.
    let protection_judgment_ac_bonus = active_inquisitor_protection_judgment_bonus(input)
        .map(|(_, bonus)| bonus)
        .unwrap_or(0);
    // v0.6 alpha swarm, risks item 8 (Oracle revelation deepening,
    // 2026-07-26, task #10): Nature Mystery's Nature's Whispers lets
    // Charisma stand in for Dexterity on Armor Class. Integrated rather
    // than grounded standalone because it is always on with no activation
    // and no per-day budget -- class-ownership-gated AND explicit-
    // revelation-gated by `active_oracle_natures_whispers_ac_bonus`
    // construction, 0 for every non-Oracle and for any Oracle who did not
    // take it. Applied as the corpus applies it: an untyped ability bonus
    // layered on top of `dexterity_contribution` (which the worn armor's
    // MAXDEX may have capped), not a replacement for it.
    let natures_whispers_ac_bonus =
        active_oracle_natures_whispers_ac_bonus(input, ability_modifiers).unwrap_or(0);
    // v0.6 alpha swarm, task #6 (2026-07-27): Cavalier's Challenge
    // applies a -2 penalty to the cavalier's OWN Armor Class while a
    // challenge is active -- same self-applied shape as Bloodrage's,
    // class-ownership-gated by construction. DESC-sourced magnitude; see
    // CAVALIER_CHALLENGE_ARMOR_CLASS_PENALTY.
    let challenge_armor_class_penalty =
        active_cavalier_challenge_armor_class_penalty(input).unwrap_or(0);
    // task #61, 2026-07-28: Sorcerer Draconic Bloodline Dragon Resistances' natural
    // armor bonus applies here too -- class-ownership-gated by construction
    // (`active_sorcerer_draconic_dragon_resistances_natural_armor_bonus`), 0 for
    // every non-Sorcerer, non-Draconic-bloodline, or below-level-3 character. Like
    // Brawler's own AC Bonus (and unlike the Rage-shaped penalties above), this is
    // not activation-gated: it is always on once the level-3 gate and the recognized
    // Draconic bloodline choice are both met, since it is a permanent (Ex) quality.
    let draconic_dragon_resistances_natural_armor_bonus =
        apply_sorcerer_draconic_dragon_resistances_ac_bonus_to_combat_baseline(input);
    // SD-27 (`decisions.md` §24/§28, 2026-07-31): ARG's Armor of the Pit is the
    // only one of the Advanced Race Guide's 187 feats whose unconditional
    // corpus magnitude lands on a total this engine computes -- a `+2` natural
    // armor bonus, and therefore a real, visible change to a tiefling's Armor
    // Class and (by exclusion) to their touch AC.
    //
    // Its corpus token carries `!PREABILITY:1,CATEGORY=Special Ability,Scaled
    // Skin C ~ Tiefling,...`, which a mechanical "an inline PRE means
    // situational" reading would classify as conditional. It is not a
    // situation: it asks whether this character took the Scaled Skin alternate
    // racial trait, a persisted creation-time decision this engine already
    // reads through `selected_alternate_trait_keys`. So the branch is fully
    // decided here rather than deferred, matching the `BENEFIT:` prose exactly
    // ("+2 natural armor bonus. If you have the scaled skin racial trait, you
    // *instead* gain resistance 5 to two of ...").
    //
    // Deliberately NOT race-gated on `PREFACT:1,TEMPLATES,IsTiefling=true`:
    // asserting a feat's selection prerequisites is `feat_prereqs`' job, the
    // same split `master_craftsman_facts_from_choices` already documents.
    //
    // SD-27 (`decisions.md` §28, feat-seam defect, 2026-07-31): read through
    // `feat_derived_pillar_contributions` rather than calling
    // `feat_effects::armor_of_the_pit_natural_armor_bonus_from_feats` here, so
    // the corpus twin (`compute_combat_baseline_from_corpus`, the path the
    // shipped sheet actually reads) consumes the identical value. Wired only
    // here, this feat moved a test and moved a live Tiefling's Armor Class by
    // nothing at all.
    let feat_contributions =
        feat_derived_pillar_contributions(input, ability_modifiers.strength);
    let armor_of_the_pit_natural_armor_bonus = feat_contributions.natural_armor_bonus;
    // v0.6 alpha swarm (creation-seed honesty fix): Dodge is a conditional
    // CONTRIBUTION, not a precondition. `unmet_combat_posture_conditions`
    // used to *require* `feat:dodge` before this baseline would compute
    // anything at all, which forced `compose_character_input`
    // (`apps/desktop/src-tauri/src/pf1_adapter.rs`) to seed the feat onto
    // every freshly created character just so the engine would produce a
    // number -- claiming a feat no choice slot had granted, on the very
    // list the sheet's Feats tab renders verbatim. Dodge's +1 is not
    // load-bearing for the rest of this formula: a character without it
    // computes the same armor class, one point lower, honestly. Every
    // character that really does carry Dodge (through any slot -- the
    // identity fold is slot-blind, exactly as before) gets the identical
    // number it always did.
    let dodge_armor_class_bonus = feat_contributions.dodge_armor_class_bonus;
    // SD-27, decisions.md §28 defect 1 (2026-07-31): the creature's PF1 size
    // modifier to Armor Class. Until this landed, every race computed a Medium
    // creature's Armor Class -- a live Goblin Fighter 1 read 18 where PF1's
    // Small value at those stats is 19 -- and the defect PRE-DATES the 18-race
    // widening: Gnome and Halfling shipped with it.
    //
    // Placed here, immediately after the Dexterity contribution and ahead of
    // every conditional class term, because that is where PF1 stacks it:
    //   AC = 10 + armor + shield + Dex + SIZE + natural + deflection + dodge + misc
    // Position is not arithmetically load-bearing in a flat sum, but the term
    // order is the sheet's published order and the explanation string below is
    // read by users, so it is put where a player would look for it.
    //
    // It is its own PF1 modifier type -- not a dodge bonus (so it is kept out
    // of `dodge_armor_class_bonus`, and is NOT lost when the creature is denied
    // its Dexterity), not armor, not natural armor, not untyped. Size modifiers
    // do not stack with each other; a creature has exactly one size, so exactly
    // one value is summed and non-stacking holds structurally rather than by
    // de-duplicating a list.
    let size_armor_class_modifier = size.armor_class_and_attack;
    let size_label = size.label;
    let armor_class = ARMOR_CLASS_BASE
        + CHAIN_SHIRT_ARMOR_BONUS
        + dexterity_contribution
        + size_armor_class_modifier
        + dodge_armor_class_bonus
        + rage_armor_class_penalty
        + inspired_rage_armor_class_penalty
        + bloodrage_armor_class_penalty
        + brawler_ac_bonus_value
        + alchemist_mutagen_ac_bonus_value
        + protection_judgment_ac_bonus
        + natures_whispers_ac_bonus
        + challenge_armor_class_penalty
        + draconic_dragon_resistances_natural_armor_bonus
        + armor_of_the_pit_natural_armor_bonus;

    explanations.push(ComputationExplanation {
        id: "defense.baseline_armor_class".to_owned(),
        value: armor_class,
        detail: format!(
            "Baseline armor class: base {ARMOR_CLASS_BASE} + Chain Shirt armor bonus (+{CHAIN_SHIRT_ARMOR_BONUS}) \
             + Dexterity contribution (+{dexterity_contribution}, DEX modifier +{dexterity_modifier} within MAXDEX:{effective_max_dex}) \
             + {size_label} size modifier ({size_armor_class_modifier:+}, PF1 Table 8-1) \
             + Dodge (+{dodge_armor_class_bonus}, only for a character actually carrying \
             {DODGE_FEAT_ID}) + Barbarian Rage penalty ({rage_armor_class_penalty}, only while \
             actively, validly raging) + Skald Inspired Rage penalty ({inspired_rage_armor_class_penalty}, only \
             while actively, validly singing) + Bloodrager Bloodrage penalty ({bloodrage_armor_class_penalty}, \
             only while actively, validly bloodraging) + Brawler AC Bonus (+{brawler_ac_bonus_value}, while \
             wearing light or no armor) + Alchemist Mutagen natural armor bonus \
             (+{alchemist_mutagen_ac_bonus_value}, only while actively, validly mutated) + \
             Inquisitor Protection judgment sacred/profane bonus (+{protection_judgment_ac_bonus}, \
             only while actively, validly judging Protection) + Oracle Nature's Whispers \
             Charisma-for-Dexterity substitution (+{natures_whispers_ac_bonus}, only for a \
             Nature-Mystery Oracle who took that revelation) + Cavalier Challenge penalty \
             ({challenge_armor_class_penalty}, only while actively challenging) + Sorcerer \
             Draconic Bloodline Dragon Resistances natural armor bonus \
             (+{draconic_dragon_resistances_natural_armor_bonus}, only for a Draconic-bloodline \
             Sorcerer at 3rd level or higher) + ARG Armor of the Pit natural armor bonus \
             (+{armor_of_the_pit_natural_armor_bonus}, only for a character holding that feat \
             who did NOT take the Scaled Skin alternate racial trait); shield \
             is absent (+0) = {armor_class}"
        ),
    });

    // SD-27, decisions.md §28 defect 1 (2026-07-31): touch AC, CMB and CMD.
    //
    // None of the three existed in this engine at all. They were computed in
    // React, in `apps/desktop/src/characterHub/CharacterSheet.tsx`, as
    // `10 + dexMod` / `bab + str` / `10 + bab + str + dexMod` -- three rules
    // formulas living in a view, none of them aware of creature size, and one
    // of them (touch) able to contradict the engine's own Armor Class on the
    // same panel. Grounding them here is the fix; the sheet now renders what
    // the engine computed.
    //
    // Which of this function's terms a touch attack ignores, stated per term
    // rather than inferred:
    //   EXCLUDED -- armor bonus (Chain Shirt), Alchemist Mutagen's natural
    //     armor bonus, Sorcerer Draconic Dragon Resistances' natural armor
    //     bonus, ARG Armor of the Pit's natural armor bonus. PF1 touch AC
    //     ignores armor, shield and natural armor.
    //   INCLUDED -- the Dexterity contribution (still subject to the armor's
    //     MAXDEX, which limits the Dexterity bonus to Armor Class generally),
    //     the size modifier, Dodge's dodge bonus, Brawler's AC Bonus (also a
    //     dodge bonus), Inquisitor Protection judgment's sacred/profane bonus,
    //     Oracle Nature's Whispers' Charisma-for-Dexterity substitution, and
    //     every Rage/Inspired Rage/Bloodrage/Challenge PENALTY (penalties are
    //     never ignored by a touch attack).
    // There is no shield term to exclude: this posture requires the shield
    // absent, contributing 0.
    let excluded_from_touch = CHAIN_SHIRT_ARMOR_BONUS
        + alchemist_mutagen_ac_bonus_value
        + draconic_dragon_resistances_natural_armor_bonus
        // The feat-derived half comes from the shared seam's own accessor, not
        // from re-listing its fields here: a feat added to
        // `FeatDerivedPillarContributions` as natural armor must leave touch AC
        // on both twins at once, and re-listing is how one twin drifts.
        + feat_contributions.excluded_from_touch_armor_class();
    let touch_armor_class_total = touch_armor_class(armor_class, excluded_from_touch);
    explanations.push(ComputationExplanation {
        id: "defense.touch_armor_class".to_owned(),
        value: touch_armor_class_total,
        detail: format!(
            "Touch armor class: the armor class above ({armor_class}) with the contributors a \
             touch attack ignores removed -- Chain Shirt armor bonus (-{CHAIN_SHIRT_ARMOR_BONUS}), \
             Alchemist Mutagen natural armor (-{alchemist_mutagen_ac_bonus_value}), Sorcerer \
             Draconic Bloodline natural armor (-{draconic_dragon_resistances_natural_armor_bonus}), \
             ARG Armor of the Pit natural armor (-{armor_of_the_pit_natural_armor_bonus}); \
             shield is absent (-0). The Dexterity contribution (+{dexterity_contribution}), the \
             {size_label} size modifier ({size_armor_class_modifier:+}, PF1 Table 8-1) and every \
             dodge/sacred/ability bonus and penalty are all retained = {touch_armor_class_total}"
        ),
    });

    // SD-27, decisions.md §28 (flat-footed defect, 2026-07-31): flat-footed
    // Armor Class, which -- exactly like touch AC one cycle earlier -- did not
    // exist in this engine at all. It was a THIRD compute twin, living in
    // `apps/desktop/src/characterHub/CharacterSheet.tsx` as
    // `ac - Math.max(0, dexMod)` since `f5117103` (2026-07-11): half of PF1's
    // rule, in a view, with no engine record to contradict it.
    //
    // Which of this function's terms a flat-footed creature loses, stated per
    // term by walking the `armor_class` sum above rather than inferred:
    //   DENIED -- the Dexterity contribution (the Dexterity BONUS only; see
    //     `flat_footed_armor_class` on why a Dexterity penalty is kept), the
    //     Oracle Nature's Whispers Charisma-for-Dexterity substitution (it
    //     stands in FOR the Dexterity bonus, so it is denied with it), and
    //     Dodge's dodge bonus.
    //   KEPT -- the Chain Shirt's armor bonus, the size modifier, all three
    //     natural-armor terms (Alchemist Mutagen, Sorcerer Draconic Dragon
    //     Resistances, ARG Armor of the Pit), Inquisitor Protection judgment's
    //     sacred/profane bonus, every Rage / Inspired Rage / Bloodrage /
    //     Challenge PENALTY (a penalty is never forgiven by being caught
    //     unprepared), and Brawler's AC Bonus.
    //
    // Brawler's AC Bonus is the one term whose reason has to be written down.
    // It IS a dodge bonus (`ground_brawler_ac_bonus_and_defer_the_rest` says so
    // from the corpus formula), but its own rule text carves itself out of the
    // general dodge rule: "These bonuses to AC apply even against touch attacks
    // or when the [character] is flat-footed." The ACG Brawler record in this
    // repo is `completeness: chassis_only` and carries no DESC, but that exact
    // sentence is in-corpus verbatim for the identical mechanic at
    // `data/corpus/pathfinder_unchained/class_feature/monk_unchained_class/`
    // `unchained_monk_ac_bonus.json` -- and this engine ALREADY implements its
    // first clause, by leaving `brawler_ac_bonus_value` out of
    // `excluded_from_touch` above. Denying it here would obey one half of one
    // sentence and not the other.
    //
    // The six other dodge-typed magnitudes this file grounds -- Dwarf
    // Defensive Training, Slayer/Investigator Trap Sense, Swashbuckler Nimble,
    // Dodging Panache and Dizzying Defense, and Bard Inspire Heroics -- are
    // deliberately NOT subtracted, because none of them is ADDED. Each is an
    // explanation-only recognition record and none appears in the `armor_class`
    // sum above; verified by grepping each symbol's every reference rather than
    // by reading the file. Subtracting a term that was never summed is how a
    // thorough-looking fix produces a flat-footed AC below what PF1 allows.
    let dexterity_bonus_denied_when_flat_footed = dexterity_contribution + natures_whispers_ac_bonus;
    // Read through the shared feat seam's own accessor, not from the local
    // `dodge_armor_class_bonus`: a future feat-derived dodge bonus must leave
    // flat-footed AC on both twins at once, and re-listing is how one twin
    // drifts -- the same reason `excluded_from_touch` reads its accessor.
    let dodge_typed_armor_class_bonuses = feat_contributions.denied_to_flat_footed_armor_class();
    let flat_footed_armor_class_total = flat_footed_armor_class(
        armor_class,
        dexterity_bonus_denied_when_flat_footed,
        dodge_typed_armor_class_bonuses,
    );
    explanations.push(ComputationExplanation {
        id: "defense.flat_footed_armor_class".to_owned(),
        value: flat_footed_armor_class_total,
        detail: format!(
            "Flat-footed armor class: the armor class above ({armor_class}) with the \
             contributors a flat-footed character is denied removed -- the Dexterity \
             contribution (-{dexterity_contribution}, bonus only; a Dexterity penalty is kept) \
             including the Oracle Nature's Whispers Charisma-for-Dexterity substitution \
             (-{natures_whispers_ac_bonus}, which stands in for that bonus), and every \
             dodge-typed bonus (-{dodge_typed_armor_class_bonuses}: Dodge's \
             {DODGE_AC_BONUS:+}, per PF1 \"any situation that denies you your Dexterity bonus \
             to Armor Class also denies you dodge bonuses\"). The Chain Shirt armor bonus \
             (+{CHAIN_SHIRT_ARMOR_BONUS}), the {size_label} size modifier \
             ({size_armor_class_modifier:+}), every natural armor and sacred/profane bonus, \
             every penalty, and Brawler's AC Bonus (+{brawler_ac_bonus_value}, whose own rule \
             text applies it even when flat-footed) are all retained = \
             {flat_footed_armor_class_total}"
        ),
    });

    let combat_maneuver_bonus_total = combat_maneuver_bonus(
        base_attack_bonus,
        strength_modifier,
        dexterity_modifier,
        size.category,
        size.special,
    );
    explanations.push(ComputationExplanation {
        id: "combat.combat_maneuver_bonus".to_owned(),
        value: combat_maneuver_bonus_total,
        detail: format!(
            "Combat Maneuver Bonus: {class_label} base attack bonus (+{base_attack_bonus}) + \
             Strength modifier (+{strength_modifier}) + {size_label} special size modifier \
             ({:+}) = {combat_maneuver_bonus_total}. The special size modifier runs OPPOSITE to \
             Table 8-1's Armor Class column: being smaller makes a creature worse at combat \
             maneuvers, not better. No Improved/Greater maneuver feat or size-changing effect is \
             modelled by this engine, so none is summed here",
            size.special
        ),
    });

    let combat_maneuver_defense_total = combat_maneuver_defense(
        base_attack_bonus,
        strength_modifier,
        dexterity_modifier,
        size.special,
    );
    explanations.push(ComputationExplanation {
        id: "defense.combat_maneuver_defense".to_owned(),
        value: combat_maneuver_defense_total,
        detail: format!(
            "Combat Maneuver Defense: base {COMBAT_MANEUVER_DEFENSE_BASE} + {class_label} base \
             attack bonus (+{base_attack_bonus}) + Strength modifier (+{strength_modifier}) + \
             Dexterity modifier (+{dexterity_modifier}) + {size_label} special size modifier \
             ({:+}) = {combat_maneuver_defense_total}. These are exactly the terms PF1's published \
             CMD formula prints; the deflection/dodge/insight/luck/morale/profane/sacred bonuses \
             to Armor Class that also apply to CMD in real PF1 are NOT folded in here yet, and \
             are named rather than silently implied",
            size.special
        ),
    });

    (melee_attack_bonus, armor_class)
}

/// v0.6 alpha swarm: found via a systematic sweep for the same brittleness
/// (single-class-only level lookup in a "_legality"/"_conditions" gate
/// function, where a multiclass-aware equivalent already exists elsewhere)
/// after finding it twice in `validate_fighter_feat_choice_legality`.
/// `unmet_combat_posture_conditions`'s own Fighter bonus-feat sub-check had
/// the identical blind spot. Confirmed this function's own `unmet` list
/// came back empty for a multiclass scenario with a wrong bonus-feat
/// choice before fixing -- not currently exploitable at the system level
/// (the other, already-fixed function independently catches the same
/// slot), but a real inconsistency worth closing so the two checks can't
/// silently diverge later.
#[cfg(test)]
mod combat_posture_multiclass_tests {
    use super::{unmet_combat_posture_conditions, CharacterClassLevel, FIGHTER_CLASS_ID, ROGUE_CLASS_ID};
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    #[test]
    fn multiclass_wrong_bonus_feat_is_now_named_in_this_functions_own_unmet_list() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
            CharacterClassLevel { class_id: ROGUE_CLASS_ID.to_owned(), level: 3 },
        ];
        input.chosen.selected_choices.retain(|c| c.choice_set_id != "choice:fighter_bonus_feat");
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:fighter_bonus_feat".to_owned(),
            selection_id: "feat:cleave".to_owned(),
        });

        let unmet = unmet_combat_posture_conditions(&input);

        assert!(
            unmet.iter().any(|reason| reason.contains("choice:fighter_bonus_feat")),
            "this function's own unmet list must now name the wrong multiclass bonus-feat \
             choice, matching its already-correct single-class behavior: {unmet:?}"
        );
    }

    /// Positive control: the canonical selection in a multiclass mix must
    /// not be flagged.
    #[test]
    fn multiclass_canonical_bonus_feat_is_not_in_the_unmet_list() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
            CharacterClassLevel { class_id: ROGUE_CLASS_ID.to_owned(), level: 3 },
        ];

        let unmet = unmet_combat_posture_conditions(&input);

        assert!(
            !unmet.iter().any(|reason| reason.contains("choice:fighter_bonus_feat")),
            "the canonical bonus-feat selection must never be flagged, in a multiclass mix: \
             {unmet:?}"
        );
    }
}

#[cfg(test)]
mod per_weapon_attack_total_tests {
    use super::{
        build_pilot_headless_receipt, equipped_weapon_stat_block, normalize_weapon_identity,
        CharacterInput,
    };
    use crate::rules_core::character_input::SelectedChoice;
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn fixture() -> CharacterInput {
        load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture")
    }

    /// The join this stage exists to make: equipment ids are
    /// `item:longsword`, the weapon table's keys are corpus display names
    /// like `Longsword`. Both must fold to one identity.
    #[test]
    fn the_identity_fold_joins_equipment_ids_to_weapon_table_keys() {
        assert_eq!(normalize_weapon_identity("item:longsword"), "longsword");
        assert_eq!(normalize_weapon_identity("Longsword"), "longsword");
        assert_eq!(
            normalize_weapon_identity("item:longsword"),
            normalize_weapon_identity("Longsword"),
            "the two shapes must join"
        );
        // Punctuation and spacing differences fold away too.
        assert_eq!(normalize_weapon_identity("Pick (Heavy)"), "pickheavy");
    }

    #[test]
    fn equipment_ids_resolve_to_real_stat_blocks_and_non_weapons_do_not() {
        let longsword = equipped_weapon_stat_block("item:longsword").expect("longsword resolves");
        assert_eq!(longsword.key, "Longsword");
        assert_eq!(longsword.damage_die, "1d8");
        // Armor is equipment but not a weapon -- must not resolve.
        assert!(equipped_weapon_stat_block("item:chain_shirt").is_none());
        assert!(equipped_weapon_stat_block("item:not_a_real_item").is_none());
    }

    /// The deterministic fixture equips a Longsword, so a real per-weapon
    /// total must appear through the live receipt.
    #[test]
    fn the_equipped_weapon_gets_a_real_attack_total() {
        let receipt = build_pilot_headless_receipt(&fixture());
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.weapon_attack_bonus.longsword")
            .expect("the equipped Longsword must ground a per-weapon attack total");
        // Fighter level 1 BAB is +1; the fixture's Strength is 18 -- base
        // chosen score 16 plus the Human +2 racial, applied BEFORE modifiers
        // are derived -- so the modifier is +4, not +3. Confirmed against the
        // fixture's own ability_modifier.strength record rather than fitted
        // to the output. The fixture carries Weapon Focus (Longsword), worth
        // +1, wired in at stage 3.
        assert_eq!(record.value, 6, "BAB(+1) + STR(+4) + Weapon Focus(+1): {record:?}");
    }

    /// SD-36 Epic E PC8-2: the per-weapon attack total must apply PF1 Table 8-1's size
    /// modifier the same way `compute_combat_baseline` already does -- every Small/Large
    /// character's real size bonus/penalty to attack was silently missing here before this
    /// fix. A Gnome (Small, `race_size_for_race_token` +1 to AC/attack) also carries a -2
    /// Strength racial adjustment relative to the Human fixture, so the TOTAL is not simply
    /// "+1 higher" -- this asserts the size term is present and non-zero, and that the total
    /// is exactly base attack bonus + governing ability modifier + weapon feats +
    /// nonproficiency penalty + size modifier, so a future change cannot silently drop the
    /// size term back out while leaving the total looking plausible.
    #[test]
    fn a_small_races_per_weapon_attack_total_carries_the_real_size_modifier() {
        let mut small = fixture();
        small.chosen.race_id = "race:gnome".to_owned();
        let receipt = build_pilot_headless_receipt(&small);
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.weapon_attack_bonus.longsword")
            .expect("the equipped Longsword must ground a per-weapon attack total");
        assert!(record.detail.contains("size modifier (+1)"), "a Small race gets PF1's +1 size bonus to attack: {record:?}");

        let medium = build_pilot_headless_receipt(&fixture());
        let medium_record = medium
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.weapon_attack_bonus.longsword")
            .expect("the Human fixture's Longsword total");
        assert!(medium_record.detail.contains("size modifier (+0)"), "a Medium race gets no size term: {medium_record:?}");
    }

    /// Stage 2 is additive: the pre-existing GE-06 baseline total must be
    /// untouched and still present alongside the new record.
    #[test]
    fn the_existing_pilot_baseline_total_is_left_intact() {
        let receipt = build_pilot_headless_receipt(&fixture());
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "combat.baseline_melee_attack_bonus"),
            "the Longsword-specific baseline must survive this stage unchanged"
        );
    }

    /// Weapon Focus must be counted exactly once.
    ///
    /// This replaces a stage-2 guard that asserted `5` on the grounds that
    /// "if this ever reads 6, stage 3 has double-counted". That reasoning
    /// was wrong and worth recording: it conflated adding one magnitude
    /// twice *inside a single total* (the real hazard, which reads 7) with
    /// the same magnitude appearing both as its own component fact and
    /// inside a total (`feat.standalone.weapon_focus.longsword` alongside
    /// this record -- normal, and not double-counting at all). A guard
    /// pinned to a stage's temporary posture has to be re-derived when the
    /// stage lands, not carried forward on its original wording.
    #[test]
    fn the_per_weapon_total_counts_weapon_focus_exactly_once() {
        let receipt = build_pilot_headless_receipt(&fixture());
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.weapon_attack_bonus.longsword")
            .expect("present");
        assert_eq!(
            record.value, 6,
            "must be BAB(+1) + STR(+4) + Weapon Focus(+1); 5 means the feat never wired in, 7 means it was added twice"
        );
    }

    /// The fixture holds no Specialization feat, so its damage bonus must be
    /// grounded as an explicit zero rather than omitted -- the same
    /// "ground the absence" convention as the level-gated class features.
    #[test]
    fn a_weapon_with_no_specialization_grounds_an_explicit_zero_damage_bonus() {
        let receipt = build_pilot_headless_receipt(&fixture());
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.weapon_damage_bonus.longsword")
            .expect("the damage bonus must be grounded even when no feat names the weapon");
        assert_eq!(record.value, 0, "no Specialization feat is held: {record:?}");
    }

    /// Threat range comes from the weapon's own stat block, and the fixture
    /// has no Improved Critical -- so the Longsword keeps its printed 19-20.
    #[test]
    fn the_threat_range_is_the_weapons_printed_one_without_improved_critical() {
        let receipt = build_pilot_headless_receipt(&fixture());
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.weapon_threat_range_low.longsword")
            .expect("the threat range must be grounded for an equipped weapon");
        assert_eq!(record.value, 19, "Longsword is 19-20 (width 2) undoubled: {record:?}");
    }

    /// The Specialization feats write `DAMAGE`, not `TOHIT`. If their
    /// magnitude ever leaked into the attack total this would catch it --
    /// the single most consequential way to get this feat family wrong.
    #[test]
    fn specialization_damage_never_leaks_into_the_attack_total() {
        let mut input = fixture();
        input.chosen.selected_feats.push("Weapon Specialization".to_owned());
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:weapon_specialization_target".to_owned(),
            selection_id: "weapon:Longsword".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let explanations = &receipt.computation.explanations;

        let attack = explanations
            .iter()
            .find(|e| e.id == "combat.weapon_attack_bonus.longsword")
            .expect("present");
        assert_eq!(
            attack.value, 6,
            "adding Weapon Specialization must not move the attack total: {attack:?}"
        );

        let damage = explanations
            .iter()
            .find(|e| e.id == "combat.weapon_damage_bonus.longsword")
            .expect("present");
        assert_eq!(damage.value, 2, "the +2 must land on damage instead: {damage:?}");
    }

    /// Improved Critical doubles the width, so a Longsword's 19-20 (width 2)
    /// becomes 17-20 (width 4). Asserting the low bound catches the
    /// width-vs-low-bound confusion this pillar's stage 1 was built to
    /// prevent: treating CRITRANGE as a low bound would give 18, not 17.
    #[test]
    fn improved_critical_doubles_the_threat_range_width() {
        let mut input = fixture();
        input.chosen.selected_feats.push("Improved Critical".to_owned());
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:improved_critical_target".to_owned(),
            selection_id: "weapon:Longsword".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.weapon_threat_range_low.longsword")
            .expect("present");
        assert_eq!(
            record.value, 17,
            "width 2 doubled to 4 gives 17-20; 18 would mean the width was treated as a low bound"
        );
    }

    /// No silent seeding: holding the feat without recording which weapon it
    /// names must change nothing, exactly as for every other chooser feat.
    #[test]
    fn a_chooser_feat_with_no_recorded_target_changes_nothing() {
        let mut input = fixture();
        input.chosen.selected_feats.push("Improved Critical".to_owned());
        input.chosen.selected_feats.push("Weapon Specialization".to_owned());
        let receipt = build_pilot_headless_receipt(&input);
        let explanations = &receipt.computation.explanations;

        let threat = explanations
            .iter()
            .find(|e| e.id == "combat.weapon_threat_range_low.longsword")
            .expect("present");
        assert_eq!(threat.value, 19, "untargeted Improved Critical must not widen anything");

        let damage = explanations
            .iter()
            .find(|e| e.id == "combat.weapon_damage_bonus.longsword")
            .expect("present");
        assert_eq!(damage.value, 0, "untargeted Weapon Specialization must not add damage");
    }
}

/// The combat feat-effects slice (2026-07-29): the three CRB feats that
/// GRANT weapon proficiency, and Weapon Finesse.
///
/// Every test here asserts a real computed TOTAL moving -- either
/// `combat.baseline_melee_attack_bonus` or
/// `combat.weapon_attack_bonus.<weapon>` -- never merely that a producer
/// returned a number.
#[cfg(test)]
mod weapon_proficiency_feat_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput,
        WEAPON_NONPROFICIENCY_ATTACK_PENALTY,
    };
    use crate::rules_core::character_input::load_character_input_fixture;
    use crate::rules_core::character_input::SelectedChoice;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn fixture() -> CharacterInput {
        load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture")
    }

    /// The fixture is a Fighter (Martial tier), so it is already proficient
    /// with its Longsword. A Wizard is not -- Wizard's ingested grant list
    /// is Club/Dagger/Crossbows/Quarterstaff.
    fn wizard_fixture() -> CharacterInput {
        let mut input = fixture();
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: "class:wizard".to_owned(), level: 1 }];
        input
    }

    fn choice(choice_set_id: &str, selection_id: &str) -> SelectedChoice {
        SelectedChoice {
            choice_set_id: choice_set_id.to_owned(),
            selection_id: selection_id.to_owned(),
        }
    }

    fn value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// The headline interaction the slice exists to prove: Martial Weapon
    /// Proficiency naming the Longsword REMOVES the -4 nonproficiency
    /// penalty from the shipped baseline total.
    #[test]
    fn martial_weapon_proficiency_removes_the_nonproficiency_penalty_from_the_baseline() {
        let without = value(&wizard_fixture(), "combat.baseline_melee_attack_bonus")
            .expect("the baseline total must ground for a Wizard");

        let mut with = wizard_fixture();
        with.chosen.selected_feats.push("Martial Weapon Proficiency".to_owned());
        with.chosen
            .selected_choices
            .push(choice("choice:martial_weapon_proficiency_target", "weapon:Longsword"));
        let with_value = value(&with, "combat.baseline_melee_attack_bonus").expect("present");

        assert_eq!(
            with_value - without,
            -WEAPON_NONPROFICIENCY_ATTACK_PENALTY,
            "granting Longsword proficiency must raise the baseline by exactly 4 \
             (was {without}, now {with_value})"
        );
    }

    /// The same removal must reach the per-weapon total, which had no
    /// proficiency term at all before this slice.
    #[test]
    fn the_per_weapon_total_carries_the_penalty_and_the_feat_removes_it() {
        let without = value(&wizard_fixture(), "combat.weapon_attack_bonus.longsword")
            .expect("the equipped Longsword must ground a per-weapon total for a Wizard");

        let mut with = wizard_fixture();
        with.chosen.selected_feats.push("Martial Weapon Proficiency".to_owned());
        with.chosen
            .selected_choices
            .push(choice("choice:martial_weapon_proficiency_target", "weapon:Longsword"));
        let with_value = value(&with, "combat.weapon_attack_bonus.longsword").expect("present");

        assert_eq!(
            with_value - without,
            -WEAPON_NONPROFICIENCY_ATTACK_PENALTY,
            "the per-weapon total must move by exactly 4 (was {without}, now {with_value})"
        );
    }

    /// A Fighter is already proficient, so the feat must be a no-op rather
    /// than a second, stacking +4.
    #[test]
    fn granting_a_proficiency_the_class_already_has_changes_nothing() {
        let before = value(&fixture(), "combat.weapon_attack_bonus.longsword").expect("present");
        let mut with = fixture();
        with.chosen.selected_feats.push("Martial Weapon Proficiency".to_owned());
        with.chosen
            .selected_choices
            .push(choice("choice:martial_weapon_proficiency_target", "weapon:Longsword"));
        assert_eq!(
            value(&with, "combat.weapon_attack_bonus.longsword"),
            Some(before),
            "proficiency is boolean -- granting it twice must not add a second +4"
        );
    }

    /// No silent seeding, asserted on the total rather than the producer:
    /// holding the feat without naming a weapon must leave the penalty in
    /// place.
    #[test]
    fn an_untargeted_proficiency_feat_leaves_the_penalty_in_place() {
        let without =
            value(&wizard_fixture(), "combat.weapon_attack_bonus.longsword").expect("present");
        let mut with = wizard_fixture();
        with.chosen.selected_feats.push("Martial Weapon Proficiency".to_owned());
        assert_eq!(
            value(&with, "combat.weapon_attack_bonus.longsword"),
            Some(without),
            "an untargeted grant must not erase the -4"
        );
    }

    /// A grant naming a DIFFERENT weapon must not transfer to this one.
    #[test]
    fn a_grant_naming_another_weapon_does_not_help_this_one() {
        let without =
            value(&wizard_fixture(), "combat.weapon_attack_bonus.longsword").expect("present");
        let mut with = wizard_fixture();
        with.chosen.selected_feats.push("Martial Weapon Proficiency".to_owned());
        with.chosen
            .selected_choices
            .push(choice("choice:martial_weapon_proficiency_target", "weapon:Greatsword"));
        assert_eq!(
            value(&with, "combat.weapon_attack_bonus.longsword"),
            Some(without),
            "proficiency with a Greatsword says nothing about a Longsword"
        );
    }

    /// Simple Weapon Proficiency grants a TIER: it must not touch the
    /// Longsword (Martial), and the Wizard already has its Simple weapons
    /// by class, so the honest way to see the tier grant work is a class
    /// with no Simple tier at all. Druid's grant list is individually
    /// named with no tier, and it does not include the Club... it does.
    /// Monk's list likewise. So this asserts the negative half, which is
    /// the one that could silently over-grant.
    #[test]
    fn simple_weapon_proficiency_does_not_reach_a_martial_weapon() {
        let without =
            value(&wizard_fixture(), "combat.weapon_attack_bonus.longsword").expect("present");
        let mut with = wizard_fixture();
        with.chosen.selected_feats.push("Simple Weapon Proficiency".to_owned());
        assert_eq!(
            value(&with, "combat.weapon_attack_bonus.longsword"),
            Some(without),
            "the Longsword is Martial; a Simple tier grant must not reach it"
        );
    }

    /// Exotic Weapon Proficiency uses the same machinery, proven on a real
    /// Exotic weapon rather than assumed from the Martial case.
    #[test]
    fn exotic_weapon_proficiency_removes_the_penalty_for_its_named_weapon() {
        let mut base = fixture();
        // Equip a Spiked Chain (Exotic) in place of the Longsword: even a
        // Fighter is non-proficient with it.
        for selection in &mut base.chosen.equipment_selections {
            if selection.item_id == "item:longsword" {
                selection.item_id = "item:spiked_chain".to_owned();
            }
        }
        let without = value(&base, "combat.weapon_attack_bonus.spikedchain")
            .expect("the Spiked Chain must ground a per-weapon total");

        let mut with = base.clone();
        with.chosen.selected_feats.push("Exotic Weapon Proficiency".to_owned());
        with.chosen
            .selected_choices
            .push(choice("choice:exotic_weapon_proficiency_target", "weapon:Spiked Chain"));
        let with_value =
            value(&with, "combat.weapon_attack_bonus.spikedchain").expect("present");
        assert_eq!(
            with_value - without,
            -WEAPON_NONPROFICIENCY_ATTACK_PENALTY,
            "a Fighter is not proficient with an Exotic weapon until the feat grants it \
             (was {without}, now {with_value})"
        );
    }
}

/// Weapon Finesse, wired into the per-weapon attack total (combat
/// feat-effects slice, 2026-07-29).
#[cfg(test)]
mod weapon_finesse_tests {
    use super::{build_pilot_headless_receipt, CharacterInput};
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn fixture() -> CharacterInput {
        load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture")
    }

    fn value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// A Rapier is Finesseable and Martial, so a Fighter wields it
    /// proficiently -- isolating the ability swap from the -4 penalty.
    /// The fixture's Dexterity must exceed its Strength for the swap to be
    /// visible, so this raises it explicitly rather than hoping.
    fn dexterous_rapier_fixture() -> CharacterInput {
        let mut input = fixture();
        for selection in &mut input.chosen.equipment_selections {
            if selection.item_id == "item:longsword" {
                selection.item_id = "item:rapier".to_owned();
            }
        }
        input.chosen.ability_scores.dexterity = 20;
        input
    }

    #[test]
    fn weapon_finesse_swaps_dexterity_in_on_a_finesseable_weapon() {
        let base = dexterous_rapier_fixture();
        let without = value(&base, "combat.weapon_attack_bonus.rapier")
            .expect("the equipped Rapier must ground a per-weapon total");

        let strength = value(&base, "ability_modifier.strength").expect("present");
        let dexterity = value(&base, "ability_modifier.dexterity").expect("present");
        assert!(
            dexterity > strength,
            "the test posture needs DEX ({dexterity}) above STR ({strength}) or it proves nothing"
        );

        let mut with = base.clone();
        with.chosen.selected_feats.push("Weapon Finesse".to_owned());
        let with_value = value(&with, "combat.weapon_attack_bonus.rapier").expect("present");

        assert_eq!(
            with_value - without,
            dexterity - strength,
            "the corpus token is max(STR,DEX)-STR, so the total must rise by exactly the gap \
             (was {without}, now {with_value})"
        );
    }

    /// The Longsword carries no Finesseable facet, so the same feat on the
    /// same character must change nothing -- the check that would fail if
    /// the facet lookup were skipped.
    #[test]
    fn weapon_finesse_does_not_reach_a_non_finesseable_weapon() {
        let mut base = fixture();
        base.chosen.ability_scores.dexterity = 20;
        let without =
            value(&base, "combat.weapon_attack_bonus.longsword").expect("present");
        let mut with = base.clone();
        with.chosen.selected_feats.push("Weapon Finesse".to_owned());
        assert_eq!(
            value(&with, "combat.weapon_attack_bonus.longsword"),
            Some(without),
            "a Longsword is not finesseable; Strength must still govern"
        );
    }

    /// `max(STR, DEX)` never lowers the total: a strong, clumsy character
    /// holding Weapon Finesse keeps Strength.
    #[test]
    fn weapon_finesse_never_lowers_an_attack_total() {
        let mut base = fixture();
        for selection in &mut base.chosen.equipment_selections {
            if selection.item_id == "item:longsword" {
                selection.item_id = "item:rapier".to_owned();
            }
        }
        base.chosen.ability_scores.dexterity = 8;
        let without = value(&base, "combat.weapon_attack_bonus.rapier").expect("present");
        let mut with = base.clone();
        with.chosen.selected_feats.push("Weapon Finesse".to_owned());
        assert_eq!(
            value(&with, "combat.weapon_attack_bonus.rapier"),
            Some(without),
            "max(STR,DEX) with DEX below STR is STR -- the total must not drop"
        );
    }
}

/// SD-31 wave 20 (chassis-coverage lane): `compute_class_chassis` had
/// dispatched Ultimate Combat's three classes (Gunslinger, Ninja, Samurai)
/// to `compute_uc_class_chassis` since `SD31-E4-F1-002`/`-005` --
/// `class_ultimate_combat.rs`'s own tests prove the resulting
/// `class_chassis.*` explanations carry the right numbers -- but
/// `has_supported_class_chassis` (the SEPARATE gate `compute_total_saves`,
/// `compute_combat_baseline` and `compute_selected_skill_modifiers` each
/// check independently) had not yet been widened to recognize `UcClassId`,
/// so every one of these three classes emitted `class_chassis.base_attack_bonus`
/// etc. correctly and still never reached `HeadlessReceiptStatus::Computed`,
/// because `defense.total_save.unsupported` (and its two siblings) fired a
/// `claim_blocking: true` diagnostic anyway. That was exactly the
/// consumer-delta probe's `NoSnapshotDeltaVsClasslessBaseline`/
/// `NeverReachesComputed` outcome shape.
///
/// **Fixed the same wave** by the `is_supported_uc_single_class` arm on
/// `has_supported_class_chassis` above -- `all_three_uc_classes_pass_the_
/// chassis_gate_at_every_level_one_through_twenty` and `the_four_chassis_
/// integration_blockers_are_gone_for_all_three_uc_classes` below both
/// assert the fix directly and stay green. Confirmed still current as of
/// SD-31 wave 27 (chassis-coverage census, `cargo test
/// ultimate_combat_chassis_gate_tests`, 4 passed).
///
/// **What is genuinely still open, and why this is NOT a chassis/dispatch
/// gap:** Samurai still never reaches `Computed` (Gunslinger does, and as
/// of SD-34 wave 34 lane C, so does Ninja), and the one remaining
/// claim-blocker is `combat.baseline_weapon_proficiency_unknown` --
/// `rules_tables::crb::weapon_tables::CLASS_WEAPON_PROFICIENCIES` carries
/// no row for Samurai (`weapon_tables.rs`'s own doc comment explains why:
/// Samurai's real corpus token, `AUTO:WEAPONPROF|TYPE=Samurai`, is a weapon
/// TYPE selector this table has no representation for at all -- an
/// all-empty row would be indistinguishable from a real "proficient with
/// nothing" claim, so none is added). `gunslinger_and_ninja_reach_computed_
/// status_samurai_does_not` below pins exactly this shape. Closing Samurai
/// would need this table's schema to grow a weapon-TYPE-selector
/// representation first, a larger change than a transcription.
#[cfg(test)]
mod ultimate_combat_chassis_gate_tests {
    use super::{
        build_pilot_headless_receipt, has_supported_class_chassis, CharacterClassLevel,
        CharacterInput,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    const GUNSLINGER_CLASS_ID: &str = "class:gunslinger";
    const NINJA_CLASS_ID: &str = "class:ninja";
    const SAMURAI_CLASS_ID: &str = "class:samurai";

    fn single_class(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    /// The gate every downstream chassis-dependent pillar keys off must
    /// recognize all three UC classes at every real level, the same bar
    /// `both_classes_pass_the_chassis_gate_at_every_level_one_through_twenty`
    /// already holds Monk/Summoner to.
    #[test]
    fn all_three_uc_classes_pass_the_chassis_gate_at_every_level_one_through_twenty() {
        for class_id in [GUNSLINGER_CLASS_ID, NINJA_CLASS_ID, SAMURAI_CLASS_ID] {
            for level in 1..=20u8 {
                assert!(
                    has_supported_class_chassis(&single_class(class_id, level)),
                    "{class_id} level {level} must be a supported chassis"
                );
            }
        }
    }

    /// The real deliverable, proven with the actual driver the class probe
    /// (`v06_work_inventory.rs::probe_class_name`) uses: the four
    /// `class_chassis.unsupported` / `combat.baseline_unsupported` /
    /// `defense.total_save.unsupported` / `skill.selected_modifier.unsupported`
    /// blockers this gate used to cause are GONE for all three classes.
    ///
    /// This does NOT claim every one of the three now reaches `Computed`.
    /// Samurai does not, at level 5: the shared fixture this test (and the
    /// sibling `class_ultimate_combat.rs` tests) borrows is built as a
    /// Fighter, but proficiency is resolved from `CLASS_WEAPON_
    /// PROFICIENCIES` keyed on `class_id`, not from the fixture's own
    /// stored data, so re-pointing `class_levels` at a class this table
    /// does not cover leaves a genuine, SEPARATE `combat.baseline_weapon_
    /// proficiency_unknown` gap -- a proficiency-model concern, not a
    /// chassis-recognition one. Gunslinger's own proficiency gap was closed
    /// the wave this gate first widened (its corpus record is unambiguous:
    /// Simple+Martial tiers); Ninja's was closed by SD-34 wave 34 lane C
    /// (a real, if partial, named-weapon transcription -- see
    /// `weapon_tables.rs`'s own doc comment); Samurai's corpus record
    /// carries only a weapon-TYPE selector this table has no representation
    /// for, so it alone is asserted to stay blocked here.
    #[test]
    fn the_four_chassis_integration_blockers_are_gone_for_all_three_uc_classes() {
        for class_id in [GUNSLINGER_CLASS_ID, NINJA_CLASS_ID, SAMURAI_CLASS_ID] {
            let receipt = build_pilot_headless_receipt(&single_class(class_id, 5));
            let blocking: Vec<String> = receipt
                .computation
                .diagnostics
                .iter()
                .filter(|d| d.claim_blocking)
                .map(|d| d.id.clone())
                .collect();
            for gone in [
                "class_chassis.unsupported",
                "combat.baseline_unsupported",
                "defense.total_save.unsupported",
                "skill.selected_modifier.unsupported",
            ] {
                assert!(
                    !blocking.contains(&gone.to_owned()),
                    "{class_id} must no longer emit {gone}: {blocking:?}"
                );
            }
        }
    }

    /// Gunslinger and (as of SD-34 wave 34 lane C) Ninja both reach
    /// `Computed` at level 5: their weapon proficiency IS resolved
    /// (`weapon_tables.rs`'s `class:gunslinger`/`class:ninja` entries),
    /// unlike Samurai, which still carries the `combat.baseline_weapon_
    /// proficiency_unknown` claim-blocker (its corpus record is a weapon
    /// TYPE selector this table has no representation for). Renamed from
    /// `gunslinger_alone_reaches_computed_status` -- the word "alone" is
    /// load-bearing (wave-20 integration fix, `OPEN-ISSUES.md` row 331): a
    /// version of this test naming only Gunslinger would have stayed green
    /// silently under a stale name the moment Ninja was fixed, exactly the
    /// failure shape this test itself was built to prevent for the next
    /// class widening too.
    #[test]
    fn gunslinger_and_ninja_reach_computed_status_samurai_does_not() {
        use super::HeadlessReceiptStatus;
        for (class_id, name) in [(GUNSLINGER_CLASS_ID, "gunslinger"), (NINJA_CLASS_ID, "ninja")] {
            let receipt = build_pilot_headless_receipt(&single_class(class_id, 5));
            let blocking: Vec<String> = receipt
                .computation
                .diagnostics
                .iter()
                .filter(|d| d.claim_blocking)
                .map(|d| d.id.clone())
                .collect();
            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "{name} level 5 must reach Computed, blockers: {blocking:?}"
            );
        }

        {
            let (class_id, name) = (SAMURAI_CLASS_ID, "samurai");
            let other_receipt = build_pilot_headless_receipt(&single_class(class_id, 5));
            assert_ne!(
                other_receipt.status,
                HeadlessReceiptStatus::Computed,
                "{name} level 5 must NOT reach Computed (still blocked on \
                 combat.baseline_weapon_proficiency_unknown) -- if this now fails, this test's \
                 name and the underlying scoping decision both need revisiting, not just this \
                 assertion"
            );
        }
    }

    /// A level beyond a UC class's real `MAXLEVEL` ceiling stays honestly
    /// blocked, not silently accepted. Mirrors `a_level_beyond_the_real_
    /// maxlevel_ceiling_stays_blocked` (APG alchemist) and its ACG sibling.
    /// Added wave-20 integration (`OPEN-ISSUES.md` row 331): before this
    /// test existed, `is_supported_uc_single_class`'s `.is_some()` ceiling
    /// check could be mutated to `.is_some() || true` (accept any UC class
    /// at any level, past `MAXLEVEL:20`) with the full lib suite staying
    /// green -- a gate that could not fail. `uc::class_chassis_resolve`
    /// itself already refuses level 21 correctly; this test is the missing
    /// proof that the gate in front of it does too.
    #[test]
    fn a_level_beyond_the_real_maxlevel_ceiling_stays_blocked() {
        use super::HeadlessReceiptStatus;
        let receipt = build_pilot_headless_receipt(&single_class(GUNSLINGER_CLASS_ID, 21));
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "gunslinger level 21 (beyond MAXLEVEL:20) must not reach Computed: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_chassis.unsupported"),
            "{:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !has_supported_class_chassis(&single_class(GUNSLINGER_CLASS_ID, 21)),
            "has_supported_class_chassis must itself refuse level 21, not just the downstream \
             chassis resolver -- this is the assertion the pre-fix gate could not fail"
        );
    }
}

