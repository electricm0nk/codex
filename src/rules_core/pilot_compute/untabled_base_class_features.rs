#[allow(unused_imports)]
pub(crate) use super::*;

/// Task #82: `defense.total_save.unsupported` and
/// `skill.selected_modifier.unsupported` used to hardcode "only computed
/// from the grounded Fighter levels 1-N or Wizard levels 1-N", even though
/// `has_supported_class_chassis` had long since widened past those two
/// classes to 20 predicates (Skald, Bloodrager, Brawler, Hunter, Cavalier,
/// Alchemist, Inquisitor, Oracle, Arcanist, Warpriest, Slayer,
/// Swashbuckler, Investigator, Witch, Shaman, Summoner, a supported
/// multiclass mix, and a supported generic single class, on top of
/// Fighter/Wizard). These
/// tests prove both diagnostics -- including the second, inner hardcoded
/// copy inside `unmet_selected_skill_posture_conditions` -- now compose
/// their class-list prose from `supported_class_chassis_description`
/// instead, through the real `compute_pilot_base_chassis` pipeline.
#[cfg(test)]
mod chassis_unsupported_diagnostics_name_the_real_gate_tests {
    use super::{
        compute_pilot_base_chassis, PilotBaseChassisComputation, FIGHTER_CLASS_ID,
        MAX_SUPPORTED_FIGHTER_LEVEL, MAX_SUPPORTED_WIZARD_LEVEL, WIZARD_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// A single class id no `has_supported_class_chassis` predicate
    /// recognizes (not Fighter, not Wizard, not any of the 18 other
    /// single-class arms, and length 1 so the multiclass-mix arm cannot
    /// match either) -- guarantees the gate is false regardless of which
    /// classes are currently supported, so this test does not need its own
    /// copy of that list to stay correct as the roster widens further.
    fn unsupported_single_class_input() -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly: {:?}", result.diagnostics);
        let mut input = result
            .character_input
            .expect("valid fixture should produce a character input record");
        input.chosen.class_levels[0].class_id = "class:not_a_recognized_chassis".to_owned();
        input.chosen.class_levels[0].level = 1;
        input
    }

    fn diagnostic_message<'a>(
        computation: &'a PilotBaseChassisComputation,
        id: &str,
    ) -> &'a str {
        computation
            .diagnostics
            .iter()
            .find(|d| d.id == id)
            .unwrap_or_else(|| panic!("expected diagnostic {id} to fire: {:?}", computation.diagnostics))
            .message
            .as_str()
    }

    #[test]
    fn total_save_unsupported_does_not_claim_fighter_or_wizard_only() {
        let input = unsupported_single_class_input();
        let computation = compute_pilot_base_chassis(&input);
        let message = diagnostic_message(&computation, "defense.total_save.unsupported");

        let stale_fighter_or_wizard_only_fragment = format!(
            "only computed from the grounded {FIGHTER_CLASS_ID} levels \
             1-{MAX_SUPPORTED_FIGHTER_LEVEL} or {WIZARD_CLASS_ID} levels \
             1-{MAX_SUPPORTED_WIZARD_LEVEL} base saves"
        );
        assert!(
            !message.contains(&stale_fighter_or_wizard_only_fragment),
            "defense.total_save.unsupported must not hardcode a Fighter-or-Wizard-only \
             framing now that has_supported_class_chassis accepts 20 predicates: {message}"
        );
        // The real current supported set must be named, not a stale pair.
        for supported_class_name in ["Skald", "Bloodrager", "Cavalier", "Shaman", "Witch"] {
            assert!(
                message.contains(supported_class_name),
                "defense.total_save.unsupported should name the real supported class list \
                 (missing {supported_class_name}): {message}"
            );
        }
    }

    #[test]
    fn selected_skill_modifier_unsupported_does_not_claim_fighter_only() {
        let input = unsupported_single_class_input();
        let computation = compute_pilot_base_chassis(&input);
        let message = diagnostic_message(&computation, "skill.selected_modifier.unsupported");

        assert!(
            !message.contains("Fighter level-1 Climb/Intimidate/Swim")
                && !message.contains("missing supported class:fighter levels 1-"),
            "skill.selected_modifier.unsupported must not hardcode a Fighter-only (or \
             Fighter-or-Wizard-only) framing in either the outer message or the inner unmet \
             string composed by unmet_selected_skill_posture_conditions: {message}"
        );
        for supported_class_name in ["Skald", "Bloodrager", "Cavalier", "Shaman", "Witch"] {
            assert!(
                message.contains(supported_class_name),
                "skill.selected_modifier.unsupported should name the real supported class \
                 list (missing {supported_class_name}): {message}"
            );
        }
    }
}

/// SD-32 Epic 3 (`epic-3-class-reachability`, AT-32-E3-001), second half:
/// proves the 20-real-base-classes-without-tables registry really runs
/// through the real `compute_pilot_base_chassis` → `compute_class_chassis`
/// call site (not a direct unit call on `untabled_base_class_chassis`
/// alone), for a real corpus-derived class, `class:kineticist`.
#[cfg(test)]
mod untabled_base_class_chassis_wiring_tests {
    use super::{compute_pilot_base_chassis, PilotBaseChassisComputation};
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn kineticist_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly: {:?}", result.diagnostics);
        let mut input = result
            .character_input
            .expect("valid fixture should produce a character input record");
        input.chosen.class_levels[0].class_id = "class:kineticist".to_owned();
        input.chosen.class_levels[0].level = level;
        input
    }

    fn diagnostic<'a>(
        computation: &'a PilotBaseChassisComputation,
        id: &str,
    ) -> Option<&'a str> {
        computation
            .diagnostics
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.message.as_str())
    }

    /// RED, confirmed manually before this wiring landed: the `else { None
    /// }` arm produced `base_attack_bonus == 0` and a bare
    /// `class_chassis.unsupported` diagnostic for a `class:kineticist`
    /// single-class input -- the class was recognized by nothing at all.
    /// GREEN below: a real chassis magnitude reaches the top-level
    /// computation, and `class_chassis.unsupported` no longer fires for
    /// this class.
    #[test]
    fn kineticist_level_20_reaches_a_real_base_attack_bonus_and_saves() {
        let input = kineticist_input(20);
        let computation = compute_pilot_base_chassis(&input);
        assert_eq!(
            computation.base_attack_bonus, 15,
            "Kineticist level 20 ThreeQuarter BAB: (20*3)/4 = 15, from the real \
             compute_pilot_base_chassis -> compute_class_chassis call site"
        );
        assert_eq!(computation.base_saves.fortitude, 12); // good: 20/2+2
        assert_eq!(computation.base_saves.reflex, 12); // good: 20/2+2
        assert_eq!(computation.base_saves.will, 6); // poor: 20/3
        assert!(
            diagnostic(&computation, "class_chassis.unsupported").is_none(),
            "class_chassis.unsupported must NOT fire once class:kineticist has a real \
             dispatch arm: {:?}",
            computation.diagnostics
        );
    }

    #[test]
    fn kineticist_level_1_matches_the_oracle_at_the_low_end_too() {
        let input = kineticist_input(1);
        let computation = compute_pilot_base_chassis(&input);
        assert_eq!(computation.base_attack_bonus, 0); // ThreeQuarter BAB: (1*3)/4 = 0
        assert_eq!(computation.base_saves.fortitude, 2); // good: 1/2+2 = 2
        assert_eq!(computation.base_saves.reflex, 2); // good: 1/2+2 = 2
        assert_eq!(computation.base_saves.will, 0); // poor: 1/3 = 0
    }

    /// An unregistered class id (neither this registry, nor any of the
    /// other five dispatch families) must still fall through to the
    /// pre-existing `class_chassis.unsupported` diagnostic -- this wiring
    /// must not accidentally become a catch-all.
    #[test]
    fn unregistered_class_id_still_falls_through_to_unsupported() {
        let mut input = kineticist_input(5);
        input.chosen.class_levels[0].class_id = "class:not_a_real_class".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        assert_eq!(computation.base_attack_bonus, 0);
        assert!(diagnostic(&computation, "class_chassis.unsupported").is_some());
    }
}

/// SD-32 card 11 (T12): proves the generic corpus-derived class-feature
/// roster (`untabled_base_class_feature_roster::roster_for`,
/// `push_untabled_base_class_feature_records`) really runs through the real
/// `compute_pilot_base_chassis` → `compute_class_chassis` →
/// `untabled_base_class_chassis::resolve` dispatch arm, for a real
/// corpus-derived class the fixture has data for (`class:antipaladin`), not
/// a direct unit call on the roster module alone.
#[cfg(test)]
mod untabled_base_class_feature_roster_wiring_tests {
    use super::{compute_pilot_base_chassis, PilotBaseChassisComputation};
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn antipaladin_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly: {:?}", result.diagnostics);
        let mut input = result
            .character_input
            .expect("valid fixture should produce a character input record");
        input.chosen.class_levels[0].class_id = "class:antipaladin".to_owned();
        input.chosen.class_levels[0].level = level;
        input
    }

    fn explanation_ids(computation: &PilotBaseChassisComputation) -> Vec<String> {
        computation.explanations.iter().map(|e| e.id.clone()).collect()
    }

    /// RED, confirmed manually before this wiring landed (and re-confirmed
    /// live by mutating `roster_for` to return an empty `Vec` and re-running
    /// this exact test, which failed for the intended reason): a level-2
    /// Antipaladin's `Touch of Corruption` (min_level 2, per the oracle's
    /// own `PREVARGTEQ:Antipaladin_CFP_Level,2`) produced no
    /// `class_feature.untabled.*` explanation at all. GREEN below: the id
    /// now reaches the top-level computation.
    #[test]
    fn antipaladin_level_2_reaches_touch_of_corruption_via_the_generic_roster() {
        let input = antipaladin_input(2);
        let computation = compute_pilot_base_chassis(&input);
        let ids = explanation_ids(&computation);
        assert!(
            ids.contains(&"class_feature.untabled.antipaladin.corpus_record.touch_of_corruption".to_owned()),
            "level-2 Antipaladin must carry the granted Touch of Corruption roster id; got: {ids:?}"
        );
        // A feature gated to a higher level (Aura of Cowardice, min_level 3)
        // must NOT appear yet -- the same "absent means not yet granted"
        // contract `push_pu_class_feature_records` already uses.
        assert!(
            !ids.contains(&"class_feature.untabled.antipaladin.corpus_record.aura_of_cowardice".to_owned()),
            "level-2 Antipaladin must not yet carry a level-3-gated feature; got: {ids:?}"
        );
    }

    #[test]
    fn antipaladin_level_3_gains_the_level_3_gated_feature() {
        let input = antipaladin_input(3);
        let computation = compute_pilot_base_chassis(&input);
        let ids = explanation_ids(&computation);
        assert!(
            ids.contains(&"class_feature.untabled.antipaladin.corpus_record.aura_of_cowardice".to_owned()),
            "level-3 Antipaladin must carry Aura of Cowardice; got: {ids:?}"
        );
    }

    /// Shape-2 (`CLASS:` level-table row) coverage, proven end-to-end the
    /// same way shape 1's Antipaladin tests are proven above: Cryptic has
    /// NO shape-1 (`.MOD`) data at all -- every one of its rows comes from
    /// the level-table convention added when closing the T12 attribution
    /// gap. A level-1 Cryptic must carry Altered Defense (min_level 1) but
    /// not Hidden Pattern (min_level 2); a level-2 Cryptic gains it.
    #[test]
    fn cryptic_level_1_reaches_altered_defense_via_shape_2_but_not_the_level_2_gated_feature() {
        let mut input = antipaladin_input(1);
        input.chosen.class_levels[0].class_id = "class:cryptic".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        let ids = explanation_ids(&computation);
        assert!(
            ids.contains(&"class_feature.untabled.cryptic.corpus_record.altered_defense".to_owned()),
            "level-1 Cryptic must carry the shape-2 Altered Defense roster id; got: {ids:?}"
        );
        assert!(
            !ids.contains(&"class_feature.untabled.cryptic.corpus_record.hidden_pattern".to_owned()),
            "level-1 Cryptic must not yet carry the level-2-gated Hidden Pattern; got: {ids:?}"
        );
    }

    #[test]
    fn cryptic_level_2_gains_the_level_2_gated_shape_2_feature() {
        let mut input = antipaladin_input(2);
        input.chosen.class_levels[0].class_id = "class:cryptic".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        let ids = explanation_ids(&computation);
        assert!(
            ids.contains(&"class_feature.untabled.cryptic.corpus_record.hidden_pattern".to_owned()),
            "level-2 Cryptic must carry Hidden Pattern; got: {ids:?}"
        );
    }

    /// A registry class the roster fixture has NO `.MOD`/`CLASS:`-row data
    /// for (`kineticist`'s own roster attribution went through shape 2 like
    /// every other T12 class; there is no longer a class the roster fixture
    /// covers zero shapes for at level 20, so this test now names a class
    /// this repo's chassis genuinely does not dispatch at all --
    /// `undine_scion`, a race feature id namespace, never a class id) must
    /// emit zero `class_feature.untabled.*` ids -- confirms the mechanism
    /// never fabricates a row for a class it has no corpus evidence for.
    /// **Superseded example, not superseded assertion:** `psion` was this
    /// test's example through SD-32 card 11 (T12) cycle 4; it is now
    /// covered by shape 3 (`ground_psion_class_features`,
    /// `psion_features`'s own module doc comment) and asserted positively
    /// below (`psion_manifesting_emits_its_own_power_points_magnitude`),
    /// same as Cryptic's shape-2 supersession noted above.
    #[test]
    fn a_class_with_no_roster_data_emits_no_untabled_class_feature_ids() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:undine_scion".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        let ids = explanation_ids(&computation);
        assert!(
            ids.iter().all(|id| !id.starts_with("class_feature.untabled.undine_scion.")),
            "undine_scion is not a dispatched class id; must emit none: {ids:?}"
        );
    }

    /// Proves `ground_antipaladin_class_features` really runs through this
    /// same `compute_pilot_base_chassis` -> `compute_class_chassis` call
    /// site (not a direct unit call on `antipaladin_features` alone), and
    /// that a real Charisma modifier reaches the formula: the fixture's
    /// human Fighter carries a 10 Charisma (modifier 0) by default, so this
    /// overrides it to a 16 (+3) to prove the ability score is actually
    /// read, not a hardcoded stand-in.
    #[test]
    fn antipaladin_level_20_reaches_every_real_magnitude_with_a_real_charisma_score() {
        let mut input = antipaladin_input(20);
        input.chosen.ability_scores.charisma = 16;
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(
            value("class_feature.untabled.antipaladin.touch_of_corruption.uses_per_day"),
            13, // 20/2 + 3
        );
        assert_eq!(
            value("class_feature.untabled.antipaladin.touch_of_corruption.damage_dice"),
            10, // 20/2
        );
        assert_eq!(
            value("class_feature.untabled.antipaladin.unholy_resilience.save_bonus"),
            3,
        );
        assert_eq!(value("class_feature.untabled.antipaladin.cruelty.dc"), 23); // 10+3+10
        assert_eq!(value("class_feature.untabled.antipaladin.cruelty.known"), 6); // capped
        assert_eq!(
            value("class_feature.untabled.antipaladin.channel_negative_energy.dice"),
            10, // (20+1)/2
        );
        assert_eq!(
            value("class_feature.untabled.antipaladin.channel_negative_energy.dc"),
            23, // 10+10+3
        );
        assert_eq!(
            value("class_feature.untabled.antipaladin.fiendish_boon.selections"),
            4, // capped
        );
        assert_eq!(
            value("class_feature.untabled.antipaladin.aura_of_depravity.damage_reduction"),
            5,
        );
        assert_eq!(
            value("class_feature.untabled.antipaladin.unholy_champion.banishment_caster_level"),
            20,
        );
        // SD-32 card 11 (T12) follow-up: three magnitude-bearing records
        // the `psion` cycle's widened census surfaced on this class.
        assert_eq!(value("class_feature.untabled.antipaladin.aura_of_evil.strength_level"), 20);
        assert_eq!(value("class_feature.untabled.antipaladin.detect_good.caster_level"), 20);
        assert_eq!(value("class_feature.untabled.antipaladin.smite_good.uses_per_day"), 7); // capped
        assert_eq!(
            value("class_feature.untabled.antipaladin.smite_good.attack_and_ac_bonus"),
            3,
        );
        assert_eq!(value("class_feature.untabled.antipaladin.smite_good.damage_bonus"), 20);
    }

    /// Below every grant level (a level-1 Antipaladin, one below Touch of
    /// Corruption's own level-2 minimum), none of the seven magnitude ids
    /// fire -- the "absent means not yet granted" contract, proven live at
    /// the real dispatch site.
    #[test]
    fn antipaladin_level_1_has_none_of_the_original_seven_magnitudes_yet() {
        // The ORIGINAL seven Antipaladin magnitudes (Touch of Corruption,
        // Unholy Resilience, Cruelty, Channel Negative Energy, Fiendish
        // Boon, Aura of Depravity, Unholy Champion) are all gated at level
        // >= 2. SD-32 card 11 (T12)'s widened-census follow-up cycle added
        // three MORE magnitudes (Aura of Evil, Detect Good, Smite Good)
        // whose own roster `min_level` is 1 -- those legitimately DO appear
        // at level 1 now, so this test (renamed from
        // `..._has_none_of_the_seven_magnitudes_yet`, which the new grants
        // made stale) asserts the original seven's absence specifically,
        // not a blanket absence of every `antipaladin.` id.
        let input = antipaladin_input(1);
        let computation = compute_pilot_base_chassis(&input);
        let ids = explanation_ids(&computation);
        const ORIGINAL_SEVEN_PREFIXES: [&str; 7] = [
            "class_feature.untabled.antipaladin.touch_of_corruption.",
            "class_feature.untabled.antipaladin.unholy_resilience.",
            "class_feature.untabled.antipaladin.cruelty.",
            "class_feature.untabled.antipaladin.channel_negative_energy.",
            "class_feature.untabled.antipaladin.fiendish_boon.",
            "class_feature.untabled.antipaladin.aura_of_depravity.",
            "class_feature.untabled.antipaladin.unholy_champion.",
        ];
        assert!(
            ids.iter().all(|id| !ORIGINAL_SEVEN_PREFIXES.iter().any(|p| id.starts_with(p))),
            "level-1 Antipaladin must have none of the original seven (all min_level >= 2) \
             magnitude explanations yet: {ids:?}"
        );
        // The three level-1-gated follow-up magnitudes DO appear.
        for id in [
            "class_feature.untabled.antipaladin.aura_of_evil.strength_level",
            "class_feature.untabled.antipaladin.detect_good.caster_level",
            "class_feature.untabled.antipaladin.smite_good.uses_per_day",
            "class_feature.untabled.antipaladin.smite_good.attack_and_ac_bonus",
            "class_feature.untabled.antipaladin.smite_good.damage_bonus",
        ] {
            assert!(ids.contains(&id.to_owned()), "expected {id} at level 1, got: {ids:?}");
        }
    }

    /// SD-32 card 11 (T12), classes 2-6 of the group attempted after
    /// Antipaladin: proves `ground_cryptic_class_features`,
    /// `ground_dread_class_features`, `ground_marksman_class_features`,
    /// `ground_psychic_warrior_class_features`, and
    /// `ground_soulknife_class_features` really run through the real
    /// `compute_pilot_base_chassis` -> `compute_class_chassis` dispatch
    /// site, at level 20 with real ability scores overridden away from the
    /// fixture's default (10, modifier 0).
    #[test]
    fn cryptic_level_20_reaches_every_real_magnitude() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:cryptic".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(
            value("class_feature.untabled.cryptic.altered_defense.damage_reduction"),
            5, // (20+3)/4
        );
        assert_eq!(value("class_feature.untabled.cryptic.disrupt_pattern.range_feet"), 30);
        assert_eq!(
            value("class_feature.untabled.cryptic.enhanced_disruption.bonus_dice"),
            9, // (20-1)/2
        );
        assert_eq!(
            value("class_feature.untabled.cryptic.hidden_pattern.stealth_bonus"),
            6, // capped
        );
        assert_eq!(value("class_feature.untabled.cryptic.trapmaker.bonus"), 20);
        assert_eq!(
            value("class_feature.untabled.cryptic.unchanging_pattern.power_resistance"),
            32, // 12+20
        );
        // SD-32 card 11 (T12) follow-up: `Cryptic Manifesting`'s three
        // shape-3 magnitudes, proven live through the real dispatch site.
        let int_score = input.chosen.ability_scores.intelligence;
        let int_mod = (int_score - 10) / 2;
        assert_eq!(
            value("class_feature.untabled.cryptic.cryptic_manifesting.power_points"),
            12 + (int_mod * 20) / 2,
        );
        assert_eq!(
            value("class_feature.untabled.cryptic.cryptic_manifesting.powers_known"),
            20,
        );
        assert_eq!(
            value("class_feature.untabled.cryptic.cryptic_manifesting.max_power_level"),
            6.min(int_score - 10),
        );
    }

    #[test]
    fn dread_level_20_reaches_every_real_magnitude_with_a_real_charisma_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:dread".to_owned();
        input.chosen.ability_scores.charisma = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(
            value("class_feature.untabled.dread.devastating_touch.bonus_damage"),
            20,
        );
        assert_eq!(value("class_feature.untabled.dread.fearsome_insight.bonus"), 10);
        assert_eq!(
            value("class_feature.untabled.dread.terror.uses_per_day"),
            23, // 20+3
        );
        assert_eq!(value("class_feature.untabled.dread.aura_of_fear.penalty"), -4);
        assert_eq!(
            value("class_feature.untabled.dread.shadow_twin.uses_per_day"),
            3, // charisma modifier only
        );
        assert_eq!(value("class_feature.untabled.dread.fear_incarnate.damage_reduction"), 10);
        // SD-32 card 11 (T12) follow-up: `Dread Manifesting`'s three
        // shape-3 magnitudes, proven live through the real dispatch site.
        let cha_score = input.chosen.ability_scores.charisma;
        assert_eq!(
            value("class_feature.untabled.dread.dread_manifesting.power_points"),
            12 + (3 * 20) / 2,
        );
        assert_eq!(value("class_feature.untabled.dread.dread_manifesting.powers_known"), 20);
        assert_eq!(
            value("class_feature.untabled.dread.dread_manifesting.max_power_level"),
            6.min(cha_score - 10),
        );
    }

    #[test]
    fn marksman_level_20_reaches_every_real_magnitude_with_a_real_dexterity_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:marksman".to_owned();
        input.chosen.ability_scores.dexterity = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.marksman.wind_reader.uses_per_day"), 23);
        assert_eq!(value("class_feature.untabled.marksman.evade_arrows.ac_bonus"), 5);
        assert_eq!(value("class_feature.untabled.marksman.favored_weapon.base_bonus"), 5);
        assert_eq!(
            value("class_feature.untabled.marksman.cover_fire.dc"),
            23, // 10+3+10
        );
        assert_eq!(
            value("class_feature.untabled.marksman.ranged_specialist.critical_multiplier_bonus"),
            1,
        );
        // SD-32 card 11 (T12) follow-up: `Marksman Manifesting`'s three
        // shape-3 magnitudes, proven live through the real dispatch site.
        let wis_score = input.chosen.ability_scores.wisdom; // fixture default 12, modifier +1
        assert_eq!(
            value("class_feature.untabled.marksman.marksman_manifesting.power_points"),
            6 + 20 / 2, // ladder value 6 + (WIS modifier 1 * level)/2
        );
        assert_eq!(
            value("class_feature.untabled.marksman.marksman_manifesting.powers_known"),
            12, // min(9,floor(59/4)=14)=9, + floor(7/2)=3 -> 12
        );
        assert_eq!(
            value("class_feature.untabled.marksman.marksman_manifesting.max_power_level"),
            4.min(wis_score - 10),
        );
    }

    #[test]
    fn psychic_warrior_level_20_reaches_every_real_magnitude() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:psychic_warrior".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.psychic_warrior.warriors_path.level"), 20);
        assert_eq!(
            value("class_feature.untabled.psychic_warrior.pathweaving.uses_per_day"),
            2, // (20-12)/3
        );
        assert_eq!(
            value("class_feature.untabled.psychic_warrior.eternal_warrior.uses_per_day"),
            1,
        );
        // SD-32 card 11 (T12) follow-up: `Psychic Warrior Manifesting`'s
        // three shape-3 magnitudes, proven live through the real dispatch
        // site.
        let wis_score = input.chosen.ability_scores.wisdom; // fixture default 12, modifier +1
        assert_eq!(
            value(
                "class_feature.untabled.psychic_warrior.psychic_warrior_manifesting.\
power_points"
            ),
            12 + 20 / 2, // ladder value 12 + (WIS modifier 1 * level)/2
        );
        assert_eq!(
            value(
                "class_feature.untabled.psychic_warrior.psychic_warrior_manifesting.\
powers_known"
            ),
            20,
        );
        assert_eq!(
            value(
                "class_feature.untabled.psychic_warrior.psychic_warrior_manifesting.\
max_power_level"
            ),
            6.min(wis_score - 10),
        );
    }

    #[test]
    fn soulknife_level_20_reaches_every_real_magnitude() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:soulknife".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.soulknife.form_mind_blade.level"), 20);
        assert_eq!(
            value("class_feature.untabled.soulknife.enhanced_mind_blade.max_enhancement_bonus"),
            5, // capped
        );
        assert_eq!(value("class_feature.untabled.soulknife.psychic_strike.die_size"), 8);
        assert_eq!(value("class_feature.untabled.soulknife.quick_draw.uses_per_round"), 1);
    }

    /// SD-32 card 11 (T12), cycle 3: proves `ground_aegis_class_features`,
    /// `ground_tactician_class_features`, `ground_vitalist_class_features`,
    /// and `ground_wilder_class_features` really run through the real
    /// `compute_pilot_base_chassis` -> `compute_class_chassis` dispatch
    /// site, at level 20 with real ability scores overridden away from the
    /// fixture's default (10, modifier 0) -- closing all nine
    /// magnitude-bearing `ultimate_psionics` classes.
    #[test]
    fn aegis_level_20_reaches_every_real_magnitude_with_a_real_intelligence_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:aegis".to_owned();
        input.chosen.ability_scores.intelligence = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.aegis.astral_repair.hp"), 2);
        assert_eq!(
            value("class_feature.untabled.aegis.damage_reduction.value"),
            8, // (20+4)/3
        );
        assert_eq!(
            value("class_feature.untabled.aegis.form_astral_suit.custom_points"),
            26, // 2+20+4
        );
        assert_eq!(
            value("class_feature.untabled.aegis.craftsman.bonus"),
            5, // (20+2)/4
        );
        assert_eq!(
            value("class_feature.untabled.aegis.reconfigure.times_per_day"),
            9, // (20-1)/2
        );
        assert_eq!(
            value("class_feature.untabled.aegis.augment_suit.duration_rounds"),
            3, // intelligence modifier only
        );
        assert_eq!(
            value("class_feature.untabled.aegis.cannibalize_suit.times_per_day"),
            5, // (20-10)/2
        );
    }

    #[test]
    fn tactician_level_20_reaches_every_real_magnitude_with_real_ability_scores() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:tactician".to_owned();
        input.chosen.ability_scores.intelligence = 16; // +3
        input.chosen.ability_scores.charisma = 14; // +2
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(
            value("class_feature.untabled.tactician.collective.minds"),
            10, // max(3, 20/2=10)
        );
        assert_eq!(
            value("class_feature.untabled.tactician.coordinated_strike.times_per_day"),
            6, // 3+3
        );
        assert_eq!(
            value("class_feature.untabled.tactician.strategy.times_per_day"),
            5, // 3+2
        );
        assert_eq!(
            value("class_feature.untabled.tactician.improved_share.powers"),
            4, // 1+(21/6)
        );
        assert_eq!(
            value("class_feature.untabled.tactician.teamwork_feats.bonus_pool"),
            3, // 20/6
        );
        assert_eq!(
            value("class_feature.untabled.tactician.master_strategist.bonus"),
            3, // intelligence modifier only
        );
        // SD-32 card 11 (T12) follow-up: `Tactician Manifesting`'s three
        // shape-3 magnitudes, proven live through the real dispatch site.
        let int_score = input.chosen.ability_scores.intelligence;
        assert_eq!(
            value("class_feature.untabled.tactician.tactician_manifesting.power_points"),
            32 + (3 * 20) / 2,
        );
        assert_eq!(
            value("class_feature.untabled.tactician.tactician_manifesting.powers_known"),
            20,
        );
        assert_eq!(
            value("class_feature.untabled.tactician.tactician_manifesting.max_power_level"),
            9.min(int_score - 10),
        );
    }

    #[test]
    fn vitalist_level_20_reaches_every_real_magnitude_with_a_real_wisdom_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:vitalist".to_owned();
        input.chosen.ability_scores.wisdom = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(
            value("class_feature.untabled.vitalist.collective.minds"),
            10, // max(20/2=10, 3)
        );
        assert_eq!(
            value("class_feature.untabled.vitalist.transfer_wounds.times_per_day"),
            6, // 3+3
        );
        assert_eq!(value("class_feature.untabled.vitalist.health_sense.level"), 20);
        assert_eq!(
            value("class_feature.untabled.vitalist.steal_health.damage"),
            23, // 20+3
        );
        assert_eq!(
            value("class_feature.untabled.vitalist.request_aid.times_per_day"),
            6, // 3+3
        );
        assert_eq!(
            value("class_feature.untabled.vitalist.steal_life.dc"),
            23, // 10+3+10
        );
        // SD-32 card 11 (T12) follow-up: `Vitalist Manifesting`'s three
        // shape-3 magnitudes, proven live through the real dispatch site.
        let wis_score = input.chosen.ability_scores.wisdom;
        assert_eq!(
            value("class_feature.untabled.vitalist.vitalist_manifesting.power_points"),
            32 + (3 * 20) / 2,
        );
        assert_eq!(
            value("class_feature.untabled.vitalist.vitalist_manifesting.powers_known"),
            11, // 1+(21/2)
        );
        assert_eq!(
            value("class_feature.untabled.vitalist.vitalist_manifesting.max_power_level"),
            9.min(wis_score - 10),
        );
    }

    #[test]
    fn wilder_level_20_reaches_every_real_magnitude() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:wilder".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.wilder.psychic_enervation.percent"), 15);
        assert_eq!(value("class_feature.untabled.wilder.surge_blast.range_feet"), 30);
        assert_eq!(
            value("class_feature.untabled.wilder.wild_surge.bonus"),
            6, // 1+(21/4)
        );
        assert_eq!(
            value("class_feature.untabled.wilder.elude_attack.ac_bonus"),
            5, // (22/4)
        );
        assert_eq!(
            value("class_feature.untabled.wilder.surging_euphoria.duration_rounds"),
            6, // mirrors wild_surge bonus
        );
        // SD-32 card 11 (T12) follow-up: `Wilder Manifesting`'s three
        // shape-3 magnitudes, proven live through the real dispatch site.
        let cha_score = input.chosen.ability_scores.charisma; // fixture default 8, modifier -1
        assert_eq!(
            value("class_feature.untabled.wilder.wilder_manifesting.power_points"),
            32 + -20 / 2, // ladder value 32 + (CHA modifier -1 * level)/2
        );
        assert_eq!(
            value("class_feature.untabled.wilder.wilder_manifesting.powers_known"),
            11, // 1+(20/2)
        );
        assert_eq!(
            value("class_feature.untabled.wilder.wilder_manifesting.max_power_level"),
            9.min(cha_score - 10),
        );
    }

    /// SD-32 card 11 (T12), cycle 4: proves `ground_kineticist_class_features`,
    /// `ground_medium_class_features`, `ground_mesmerist_class_features`,
    /// `ground_occultist_class_features`, `ground_psychic_class_features`,
    /// and `ground_spiritualist_class_features` really run through the real
    /// `compute_pilot_base_chassis` -> `compute_class_chassis` dispatch
    /// site, at level 20 with real ability scores overridden away from the
    /// fixture's default (10, modifier 0) -- closing all six classes
    /// sharing `oa_abilities_class.lst`.
    #[test]
    fn kineticist_level_20_reaches_every_real_magnitude_with_a_real_constitution_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:kineticist".to_owned();
        input.chosen.ability_scores.constitution = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.kineticist.burn.max_points"), 6); // 3+3
        assert_eq!(value("class_feature.untabled.kineticist.elemental_focus.level_base"), 10);
        assert_eq!(value("class_feature.untabled.kineticist.infusion.pool"), 8);
        assert_eq!(value("class_feature.untabled.kineticist.kinetic_blast.range_feet"), 30);
        assert_eq!(value("class_feature.untabled.kineticist.wild_talents.dc"), 23); // 10+10+3
        assert_eq!(value("class_feature.untabled.kineticist.expanded_element.pool"), 2);
    }

    #[test]
    fn medium_level_20_reaches_every_real_magnitude_with_a_real_charisma_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:medium".to_owned();
        input.chosen.ability_scores.charisma = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.medium.spirit.bonus"), 6);
        assert_eq!(value("class_feature.untabled.medium.spirit_surge.dice"), 10);
        assert_eq!(value("class_feature.untabled.medium.haunt_channeler.dice"), 10);
        assert_eq!(value("class_feature.untabled.medium.haunt_channeler.dc"), 30);
        assert_eq!(value("class_feature.untabled.medium.location_channel.duration_rounds"), 20);
        assert_eq!(value("class_feature.untabled.medium.location_channel.dc"), 30);
        assert_eq!(value("class_feature.untabled.medium.ask_the_spirits.dc"), 18); // 15+3
        assert_eq!(value("class_feature.untabled.medium.astral_journey.dc"), 22); // 19+3
        assert_eq!(value("class_feature.untabled.medium.trance_of_three.duration_rounds"), 20);
    }

    #[test]
    fn mesmerist_level_20_reaches_every_real_magnitude_with_a_real_charisma_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:mesmerist".to_owned();
        input.chosen.ability_scores.charisma = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.mesmerist.consummate_liar.bonus"), 10);
        assert_eq!(value("class_feature.untabled.mesmerist.hypnotic_stare.penalty"), 1);
        assert_eq!(value("class_feature.untabled.mesmerist.mesmerist_tricks.uses"), 13); // 10+3
        assert_eq!(value("class_feature.untabled.mesmerist.mesmerist_tricks.range_feet"), 300);
        assert_eq!(value("class_feature.untabled.mesmerist.mesmerist_tricks.dc"), 23); // 10+10+3
        assert_eq!(value("class_feature.untabled.mesmerist.mesmerist_tricks.known"), 11);
        assert_eq!(value("class_feature.untabled.mesmerist.painful_stare.damage"), 10);
        assert_eq!(value("class_feature.untabled.mesmerist.painful_stare.bonus_dice"), 6);
        assert_eq!(value("class_feature.untabled.mesmerist.towering_ego.bonus"), 3);
        assert_eq!(value("class_feature.untabled.mesmerist.bold_stare.known"), 5);
        assert_eq!(value("class_feature.untabled.mesmerist.touch_treatment.uses"), 6); // 3+3
        assert_eq!(value("class_feature.untabled.mesmerist.manifold_tricks.count"), 5);
        assert_eq!(value("class_feature.untabled.mesmerist.mental_potency.bonus"), 4);
        assert_eq!(value("class_feature.untabled.mesmerist.glib_lie.dc"), 35);
    }

    #[test]
    fn occultist_level_20_reaches_every_real_magnitude_with_a_real_intelligence_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:occultist".to_owned();
        input.chosen.ability_scores.intelligence = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.occultist.focus_powers.count"), 10);
        assert_eq!(value("class_feature.untabled.occultist.focus_powers.dc"), 23); // 10+10+3
        assert_eq!(value("class_feature.untabled.occultist.implements.school_count"), 7);
        assert_eq!(value("class_feature.untabled.occultist.mental_focus.value"), 23); // 20+3
        assert_eq!(value("class_feature.untabled.occultist.magic_item_skill.bonus"), 10);
        assert_eq!(value("class_feature.untabled.occultist.outside_contact.count"), 4);
        assert_eq!(value("class_feature.untabled.occultist.binding_circles.dc"), 23); // 10+10+3
    }

    #[test]
    fn psychic_level_20_reaches_every_real_magnitude_with_a_real_charisma_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:psychic".to_owned();
        input.chosen.ability_scores.charisma = 16; // +3
        // Row 17 residual closure: Phrenic Pool's ability term is
        // discipline-dependent (see `psychic_discipline_pool_ability`), so a
        // real magnitude requires a real chosen discipline, same as
        // Sorcerer's bloodline choice.
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:psychic_discipline".to_owned(),
            selection_id: "discipline:rapport".to_owned(),
        });
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.psychic.phrenic_amplifications.count"), 1 + 19 / 4);
        assert_eq!(value("class_feature.untabled.psychic.phrenic_pool.value"), 13); // 10+3, Rapport = CHA
        assert_eq!(value("class_feature.untabled.psychic.psychic_discipline.pool"), 1);
        assert_eq!(value("class_feature.untabled.psychic.major_amplifications.count"), 1 + 9 / 4);
    }

    /// RED, confirmed manually before this wiring landed (mutated
    /// `psychic_discipline_pool_ability` to `Some((ability_modifiers.charisma,
    /// "Charisma"))` unconditionally, re-ran, watched `phrenic_pool.value`
    /// appear with no chosen discipline at all -- then reverted). GREEN
    /// below: with no `choice:psychic_discipline` selection, Phrenic Pool
    /// is correctly left ungrounded rather than defaulting to any ability
    /// score -- the exact provisional-default shape row 17 exists to close,
    /// not reintroduce. Every OTHER psychic magnitude still grounds, since
    /// only Phrenic Pool depends on the discipline choice.
    #[test]
    fn psychic_phrenic_pool_is_ungrounded_with_no_chosen_discipline() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:psychic".to_owned();
        input.chosen.ability_scores.charisma = 16;
        input.chosen.ability_scores.wisdom = 18;
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.untabled.psychic.phrenic_pool.value"),
            "Phrenic Pool must not ground a value with no chosen Psychic Discipline: {:?}",
            computation.explanations
        );
        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.untabled.psychic.phrenic_amplifications.count"),
            "discipline-independent psychic magnitudes must still ground: {:?}",
            computation.explanations
        );
    }

    /// Proves `psychic_discipline_pool_ability` per discipline
    /// (`oa_abilities_class.lst:1188`-1196's own `BONUS:VAR|
    /// PhrenicPoolAbility|<CHA|WIS>` tokens, not domain recall) — all 9,
    /// not just one, per `decisions.md §27b` item 5 ("needs a new
    /// mechanism" is not grounds to under-prove it).
    #[test]
    fn psychic_phrenic_pool_uses_the_real_ability_for_every_discipline() {
        let disciplines_and_expected_ability: [(&str, &str); 9] = [
            ("discipline:abomination", "cha"),
            ("discipline:dream", "cha"),
            ("discipline:pain", "cha"),
            ("discipline:rapport", "cha"),
            ("discipline:faith", "wis"),
            ("discipline:lore", "wis"),
            ("discipline:psychedelia", "wis"),
            ("discipline:self_perfection", "wis"),
            ("discipline:tranquility", "wis"),
        ];
        for (selection_id, expect_ability) in disciplines_and_expected_ability {
            let mut input = antipaladin_input(4);
            input.chosen.class_levels[0].class_id = "class:psychic".to_owned();
            input.chosen.ability_scores.charisma = 14; // +2
            input.chosen.ability_scores.wisdom = 20; // +5
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: "choice:psychic_discipline".to_owned(),
                selection_id: selection_id.to_owned(),
            });
            let computation = compute_pilot_base_chassis(&input);
            let fact = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.untabled.psychic.phrenic_pool.value")
                .unwrap_or_else(|| {
                    panic!("expected Phrenic Pool to ground for {selection_id}: {:?}", computation.explanations)
                });
            let expected = 4 / 2 + if expect_ability == "cha" { 2 } else { 5 };
            assert_eq!(fact.value, expected, "{selection_id} -> {expect_ability}: {:?}", fact);
        }
    }

    /// An unrecognized selection under the right choice-set id must not be
    /// treated as any real discipline (no silent default, `decisions.md
    /// §1a`).
    #[test]
    fn psychic_phrenic_pool_is_ungrounded_for_an_unrecognized_discipline_selection() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:psychic".to_owned();
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:psychic_discipline".to_owned(),
            selection_id: "discipline:not_a_real_discipline".to_owned(),
        });
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.untabled.psychic.phrenic_pool.value"),
            "an unrecognized discipline selection must not ground any value: {:?}",
            computation.explanations
        );
    }

    #[test]
    fn spiritualist_level_20_reaches_every_real_magnitude() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:spiritualist".to_owned();
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.spiritualist.phantom.master_level"), 20);
        assert_eq!(
            value("class_feature.untabled.spiritualist.shared_consciousness.focus_pool"),
            1
        );
        assert_eq!(value("class_feature.untabled.spiritualist.calm_spirit.uses_per_day"), 4);
    }

    /// SD-34 wave 44 (`decisions.md §22`, Piece 2 item 4): the generic
    /// `"Phantom Emotional Focus"` pool this wave wires -- each of the
    /// seven real corpus members grounds its own bare literal `1` when
    /// recorded, and none does when unrecorded (never a fabricated
    /// default).
    #[test]
    fn spiritualist_phantom_emotional_focus_grounds_the_recorded_choice_only() {
        let mut input = antipaladin_input(1);
        input.chosen.class_levels[0].class_id = "class:spiritualist".to_owned();
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:spiritualist_emotional_focus".to_owned(),
            selection_id: "focus:despair".to_owned(),
        });
        let computation = compute_pilot_base_chassis(&input);
        let despair = computation
            .explanations
            .iter()
            .find(|e| {
                e.id.starts_with(
                    "class_feature.occult_adventures.spiritualist.phantom_emotional_focus.generic",
                ) && e.id.contains("despair")
            })
            .unwrap_or_else(|| panic!("expected Despair's own generic-pool explanation, got: {:?}", computation.explanations));
        assert_eq!(despair.value, 1, "{:?}", despair);
        assert!(
            !computation.explanations.iter().any(|e| e.id.contains("zeal")),
            "an unrecorded emotional focus must not ground: {:?}",
            computation.explanations
        );
    }

    /// An unrecognized emotional-focus selection must ground nothing.
    #[test]
    fn spiritualist_phantom_emotional_focus_is_ungrounded_for_an_unrecognized_selection() {
        let mut input = antipaladin_input(1);
        input.chosen.class_levels[0].class_id = "class:spiritualist".to_owned();
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:spiritualist_emotional_focus".to_owned(),
            selection_id: "focus:not_a_real_focus".to_owned(),
        });
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation.explanations.iter().any(|e| {
                e.id.starts_with(
                    "class_feature.occult_adventures.spiritualist.phantom_emotional_focus.generic",
                )
            }),
            "an unrecognized selection must not ground any value: {:?}",
            computation.explanations
        );
    }

    /// SD-32 card 11 (T12), cycle 4: proves `ground_magus_class_features`,
    /// `ground_shifter_class_features`, and `ground_vigilante_class_features`
    /// really run through the real dispatch site -- the three single-class
    /// single-book tails cycle 3 named.
    #[test]
    fn magus_level_20_reaches_every_real_magnitude_with_a_real_intelligence_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:magus".to_owned();
        input.chosen.ability_scores.intelligence = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.magus.arcane_pool.value"), 13); // 10+3
        assert_eq!(value("class_feature.untabled.magus.arcane_pool.enhancement_bonus"), 5);
        assert_eq!(value("class_feature.untabled.magus.armor_proficiency.level"), 20);
        assert_eq!(value("class_feature.untabled.magus.magus_arcana.pool"), 6);
        assert_eq!(value("class_feature.untabled.magus.bonus_feats.pool"), 3);
        assert_eq!(value("class_feature.untabled.magus.fighter_training.level"), 10);
    }

    #[test]
    fn shifter_level_20_reaches_every_real_magnitude_with_a_real_wisdom_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:shifter".to_owned();
        input.chosen.ability_scores.wisdom = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.shifter.shifter_aspect.minutes"), 23);
        assert_eq!(value("class_feature.untabled.shifter.shifter_aspect.count"), 4);
        // Fixture's default race resolves Medium: base 4, +2 at 7, +2 at 11, +2 at 13.
        assert_eq!(value("class_feature.untabled.shifter.shifter_claws.damage"), 10);
        assert_eq!(value("class_feature.untabled.shifter.defensive_instinct.ac_bonus"), 6); // 5+1
        assert_eq!(value("class_feature.untabled.shifter.track.level"), 20);
        assert_eq!(value("class_feature.untabled.shifter.wild_shape.count"), 23); // 20+3
    }

    #[test]
    fn vigilante_level_20_reaches_every_real_magnitude_with_a_real_charisma_score() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:vigilante".to_owned();
        input.chosen.ability_scores.charisma = 16; // +3
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        assert_eq!(value("class_feature.untabled.vigilante.seamless_guise.bonus"), 20);
        assert_eq!(value("class_feature.untabled.vigilante.social_talent.count"), 10);
        assert_eq!(
            value("class_feature.untabled.vigilante.vigilante_specialization.pool"),
            1
        );
        assert_eq!(value("class_feature.untabled.vigilante.vigilante_talent.count"), 10);
        assert_eq!(value("class_feature.untabled.vigilante.vigilante_talent.dc"), 23); // 10+10+3
        assert_eq!(value("class_feature.untabled.vigilante.unshakable.dc_bonus"), 20);
        assert_eq!(
            value("class_feature.untabled.vigilante.frightening_appearance.dc"),
            23
        );
        assert_eq!(value("class_feature.untabled.vigilante.stunning_appearance.dc"), 23);
        assert_eq!(
            value("class_feature.untabled.vigilante.stunning_appearance.hd_threshold"),
            20
        );
    }

    /// Proves `ground_psion_class_features` really runs through
    /// `compute_pilot_base_chassis` -> `compute_class_chassis` (not a
    /// direct unit call on `psion_features` alone), and that a real
    /// Intelligence modifier reaches the formula.
    #[test]
    fn psion_manifesting_emits_its_own_power_points_magnitude() {
        let mut input = antipaladin_input(20);
        input.chosen.class_levels[0].class_id = "class:psion".to_owned();
        input.chosen.ability_scores.intelligence = 20; // +5
        let computation = compute_pilot_base_chassis(&input);
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected explanation {id}, got: {:?}", computation.explanations))
                .value
        };
        // base ladder at level 20 = 32, bonus (5*20)/2 = 50.
        assert_eq!(value("class_feature.untabled.psion.psion_manifesting.power_points"), 82);
    }

    /// Below level 1 (the character's own lowest possible class level in
    /// this fixture shape), Psion Power Points must be absent -- the same
    /// "absent means not yet granted" contract every other class above
    /// proves, spot-checked at level 1 itself since Psion grants this
    /// magnitude from its very first level (no higher gate to under-shoot).
    #[test]
    fn psion_level_1_already_carries_power_points() {
        let mut input = antipaladin_input(1);
        input.chosen.class_levels[0].class_id = "class:psion".to_owned();
        input.chosen.ability_scores.intelligence = 10; // +0
        let computation = compute_pilot_base_chassis(&input);
        let ids = explanation_ids(&computation);
        assert!(
            ids.contains(&"class_feature.untabled.psion.psion_manifesting.power_points".to_owned()),
            "level-1 Psion must already carry Psion Manifesting's power points; got: {ids:?}"
        );
    }

    /// Below each class's own highest-gated magnitude's `min_level`, that
    /// specific id must be absent -- the same "absent means not yet
    /// granted" contract proven for Antipaladin above, spot-checked at
    /// each new class's own highest threshold (several of these five
    /// classes grant one or more magnitudes at level 1 itself, unlike
    /// Antipaladin whose lowest is level 2, so a blanket level-1 check
    /// does not apply generically here).
    #[test]
    fn each_new_class_lacks_its_highest_gated_magnitude_one_level_early() {
        let cases: [(&str, u8, &str); 18] = [
            (
                "class:cryptic",
                18,
                "class_feature.untabled.cryptic.unchanging_pattern.power_resistance",
            ),
            (
                "class:dread",
                20,
                "class_feature.untabled.dread.fear_incarnate.damage_reduction",
            ),
            (
                "class:marksman",
                19,
                "class_feature.untabled.marksman.ranged_specialist.critical_multiplier_bonus",
            ),
            (
                "class:psychic_warrior",
                20,
                "class_feature.untabled.psychic_warrior.eternal_warrior.uses_per_day",
            ),
            (
                "class:soulknife",
                5,
                "class_feature.untabled.soulknife.quick_draw.uses_per_round",
            ),
            (
                "class:aegis",
                12,
                "class_feature.untabled.aegis.cannibalize_suit.times_per_day",
            ),
            (
                "class:tactician",
                20,
                "class_feature.untabled.tactician.master_strategist.bonus",
            ),
            (
                "class:vitalist",
                14,
                "class_feature.untabled.vitalist.steal_life.dc",
            ),
            (
                "class:wilder",
                4,
                "class_feature.untabled.wilder.surging_euphoria.duration_rounds",
            ),
            (
                "class:kineticist",
                7,
                "class_feature.untabled.kineticist.expanded_element.pool",
            ),
            (
                "class:medium",
                15,
                "class_feature.untabled.medium.trance_of_three.duration_rounds",
            ),
            (
                "class:mesmerist",
                11,
                "class_feature.untabled.mesmerist.glib_lie.dc",
            ),
            (
                "class:occultist",
                12,
                "class_feature.untabled.occultist.binding_circles.dc",
            ),
            (
                "class:psychic",
                11,
                "class_feature.untabled.psychic.major_amplifications.count",
            ),
            (
                "class:spiritualist",
                7,
                "class_feature.untabled.spiritualist.calm_spirit.uses_per_day",
            ),
            (
                "class:magus",
                10,
                "class_feature.untabled.magus.fighter_training.level",
            ),
            (
                "class:shifter",
                4,
                "class_feature.untabled.shifter.wild_shape.count",
            ),
            (
                "class:vigilante",
                17,
                "class_feature.untabled.vigilante.stunning_appearance.hd_threshold",
            ),
        ];
        for (class_id, min_level, gated_id) in cases {
            let mut input = antipaladin_input(min_level - 1);
            input.chosen.class_levels[0].class_id = class_id.to_owned();
            let computation = compute_pilot_base_chassis(&input);
            let ids = explanation_ids(&computation);
            assert!(
                !ids.contains(&gated_id.to_owned()),
                "{class_id} at level {} (one below {gated_id}'s min_level {min_level}) must \
                 not yet carry it: {ids:?}",
                min_level - 1
            );

            let mut input_at = antipaladin_input(min_level);
            input_at.chosen.class_levels[0].class_id = class_id.to_owned();
            let computation_at = compute_pilot_base_chassis(&input_at);
            let ids_at = explanation_ids(&computation_at);
            assert!(
                ids_at.contains(&gated_id.to_owned()),
                "{class_id} at level {min_level} must carry {gated_id}: {ids_at:?}"
            );
        }
    }
}

/// SD-34 wave 33 lane C (`class_modelled_but_no_observed_delta_on_the_
/// rendered_snapshot`, bucket D). The same integration gap
/// `ultimate_combat_chassis_gate_tests` already proved and closed for
/// Gunslinger: `untabled_base_class_chassis`/`crb_untabled_class_chassis`
/// dispatch a real chassis through `compute_class_chassis`, but
/// `has_supported_class_chassis` -- checked independently by
/// `compute_total_saves`/`compute_combat_baseline`/
/// `compute_selected_skill_modifiers` -- never grew a matching arm.
#[cfg(test)]
mod untabled_class_chassis_gate_tests {
    use super::{
        build_pilot_headless_receipt, has_supported_class_chassis, untabled_base_class_chassis,
        crb_untabled_class_chassis, CharacterClassLevel, CharacterInput, HeadlessReceiptStatus,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn single_class(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    /// The gate must recognize every one of the 27 classes the two
    /// registries cover, at every level within each class's own real
    /// `MAXLEVEL` ceiling -- not just level 1.
    #[test]
    fn all_27_untabled_classes_pass_the_chassis_gate_at_every_real_level() {
        for meta in untabled_base_class_chassis::untabled_base_class_registry() {
            for level in 1..=meta.max_level {
                assert!(
                    has_supported_class_chassis(&single_class(&meta.class_id, level)),
                    "{} level {level} must be a supported chassis",
                    meta.class_id
                );
            }
        }
        for meta in crb_untabled_class_chassis::covered_classes() {
            // `crb_untabled_class_chassis::resolve` reads MAXLEVEL from the
            // corpus itself rather than exposing it on the metadata struct;
            // level 1 and level 20 (the sweep's own ceiling) bracket every
            // real NPC/Ex-* class's table, which tops out at 20 like every
            // other class this engine models.
            for level in [1u8, 20] {
                assert!(
                    has_supported_class_chassis(&single_class(&meta.class_id, level)),
                    "{} level {level} must be a supported chassis",
                    meta.class_id
                );
            }
        }
    }

    /// The gate widening did not touch the prestige-entry-gate arm --
    /// prestige classes correctly stay unsupported here, since no BAB/save
    /// chassis exists for them to fold into a total save or combat
    /// baseline.
    #[test]
    fn a_prestige_class_id_still_fails_the_gate() {
        assert!(!has_supported_class_chassis(&single_class("class:arcane_archer", 5)));
    }

    /// The Epic F1 proficiency-reader remainder, read from its committed record
    /// (`reader-remainder.md`, pinned against the reader itself by
    /// `weapon_tables::every_census_class_has_a_known_proficiency_answer`): the non-prestige
    /// class ids that still answer Unknown, each with a named mechanism there.
    fn non_prestige_reader_remainder() -> Vec<String> {
        const REMAINDER: &str = include_str!(
            "../../../docs/release/SD-36-consolidation/artifacts/epic-f/reader-remainder.md"
        );
        REMAINDER
            .lines()
            .filter_map(|line| line.strip_prefix("| class:"))
            .filter_map(|rest| {
                let mut cells = rest.split('|').map(str::trim);
                let slug = cells.next()?;
                (cells.next()? == "base").then(|| format!("class:{slug}"))
            })
            .collect()
    }

    fn claim_blocking_ids(receipt: &super::PilotHeadlessReceipt) -> Vec<String> {
        receipt
            .computation
            .diagnostics
            .iter()
            .filter(|d| d.claim_blocking)
            .map(|d| d.id.clone())
            .collect()
    }

    /// Meaning changed by SD-36 Epic F1 (was
    /// `the_nine_classes_with_a_real_proficiency_row_reach_computed`, which pinned only the nine
    /// classes given a static `CLASS_WEAPON_PROFICIENCIES` row in SD-34). With the converted-record
    /// reader as the fallback, EVERY class both untabled registries cover reaches `Computed` at
    /// level 1 -- except the named non-prestige reader remainder (`reader-remainder.md`), which must
    /// NOT reach it and must be blocked on exactly the proficiency diagnostic. A class leaving the
    /// remainder flips here with no edit; a class that silently stops computing fails by name.
    #[test]
    fn every_untabled_class_outside_the_named_reader_remainder_reaches_computed() {
        let remainder = non_prestige_reader_remainder();
        assert!(
            remainder.iter().all(|id| id == "class:antipaladin" || id == "class:magus" || id == "class:commoner"),
            "the recorded non-prestige remainder moved: {remainder:?}"
        );
        let mut class_ids: Vec<String> = untabled_base_class_chassis::untabled_base_class_registry()
            .iter()
            .map(|meta| meta.class_id.clone())
            .collect();
        class_ids.extend(crb_untabled_class_chassis::covered_classes().into_iter().map(|meta| meta.class_id));
        let mut computed = 0usize;
        let mut held_back = 0usize;
        for class_id in &class_ids {
            let receipt = build_pilot_headless_receipt(&single_class(class_id, 1));
            let blocking = claim_blocking_ids(&receipt);
            if remainder.contains(class_id) {
                held_back += 1;
                assert_ne!(receipt.status, HeadlessReceiptStatus::Computed, "{class_id} is in the recorded remainder");
                assert_eq!(
                    blocking,
                    vec!["combat.baseline_weapon_proficiency_unknown".to_string()],
                    "{class_id} (recorded remainder) must be blocked on the proficiency answer alone"
                );
            } else {
                computed += 1;
                assert_eq!(
                    receipt.status,
                    HeadlessReceiptStatus::Computed,
                    "{class_id} level 1 must reach Computed, blockers: {blocking:?}"
                );
            }
        }
        // Denominator: the classes the two untabled registries cover (27 + the CRB NPC/Ex-* set).
        assert_eq!(computed + held_back, class_ids.len());
        assert!(computed > 9, "more than the SD-34 nine must compute now: {computed} of {}", class_ids.len());
    }

    /// Replaces `a_class_without_a_new_proficiency_row_still_reports_proficiency_unknown`: the
    /// "unknown stays unknown" contract lives on. Synthetic half (always has a case, even once the
    /// real remainder is empty): a class with no converted record reads Unknown from the reader,
    /// never an empty "proficient with nothing". Real half: every non-prestige class in the
    /// recorded reader remainder (its closure is incomplete -- `reader-remainder.md` names the
    /// mechanism) keeps the claim-blocking `combat.baseline_weapon_proficiency_unknown`, while the
    /// chassis-gate blockers stay gone.
    #[test]
    fn a_class_whose_converted_closure_is_incomplete_still_reports_proficiency_unknown() {
        use crate::rules_core::pilot_compute::class_proficiency_sheet_rules::{
            class_weapon_proficiency_view, ProficiencyAnswer,
        };
        match class_weapon_proficiency_view("fixture_class_with_no_record", 1) {
            ProficiencyAnswer::Unknown { reason } => {
                assert!(reason.contains("no converted class record"), "{reason}")
            }
            ProficiencyAnswer::Known(view) => panic!("a class with no record must be Unknown, got {view:?}"),
        }

        for class_id in non_prestige_reader_remainder() {
            let receipt = build_pilot_headless_receipt(&single_class(&class_id, 1));
            assert_ne!(receipt.status, HeadlessReceiptStatus::Computed, "{class_id}");
            assert!(
                receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id == "combat.baseline_weapon_proficiency_unknown" && d.claim_blocking),
                "{class_id}: {:?}",
                receipt.computation.diagnostics
            );
            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.claim_blocking
                        && (d.id == "class_chassis.unsupported"
                            || d.id == "combat.baseline_unsupported"
                            || d.id == "defense.total_save.unsupported"
                            || d.id == "skill.selected_modifier.unsupported")),
                "{class_id}: the chassis-gate blockers must stay gone: {:?}",
                receipt.computation.diagnostics
            );
        }
    }
}

