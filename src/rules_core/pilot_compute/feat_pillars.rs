#[allow(unused_imports)]
pub(crate) use super::*;

/// Feats a class grants AUTOMATICALLY at a level gate, which therefore
/// never appear in `selected_feats` because a player never picks them
/// (v0.6 alpha swarm, task #20, 2026-07-27).
///
/// This is the missing half of `feat_effects::effective_feats`: that leaf
/// unions chosen and granted feats, but deliberately does not know class
/// or level gates, so the derivation lives here.
///
/// The three real grants, each verified directly against the corpus:
/// - Monk, 1st level: `Improved Unarmed Strike`
///   (`ABILITY:FEAT|AUTOMATIC|Improved Unarmed Strike` on
///   `KEY:Monk ~ Unarmed Strike`) and `Stunning Fist`
///   (`ABILITY:FEAT|VIRTUAL|Stunning Fist` on
///   `KEY:Monk ~ Stunning Fist`) -- granted "even if he does not meet the
///   prerequisites".
/// - Ranger, 3rd level: `Endurance`
///   (`ABILITY:FEAT|AUTOMATIC|Endurance|...|PREVARGTEQ:Ranger_CFP_Level,3`).
///
/// Returns catalog keys, matching the exact string form a player-picked
/// feat carries in `selected_feats`, so producers cannot tell the two
/// apart.
///
/// **Scope honesty**: these three are the grants that matter to the
/// producers that exist today. Other classes grant further bonus feats
/// (Fighter's own bonus-feat slots, Wizard's Scribe Scroll) but those are
/// either player-chosen from a list -- so they DO reach `selected_feats`
/// -- or have no producer keying on them.
pub(super) fn class_granted_feats(input: &CharacterInput) -> Vec<&'static str> {
    let mut granted = Vec::new();
    for class_level in &input.chosen.class_levels {
        if class_level.class_id == MONK_CLASS_ID {
            granted.push("Improved Unarmed Strike");
            granted.push("Stunning Fist");
        }
        if class_level.class_id == RANGER_CLASS_ID
            && class_level.level >= RANGER_ENDURANCE_LEVEL
        {
            granted.push("Endurance");
        }
    }
    granted
}

/// `bonus` multiplied by how many times `feat_key` appears in
/// `selected_feats`.
///
/// COUNTS occurrences rather than testing presence, because all four
/// feats are `STACK:YES MULT:YES` in the corpus -- genuinely repeatable,
/// with each instance stacking. A Barbarian who took Extra Rage twice has
/// 12 extra rounds per day, not 6.
///
/// Folded into each resource's own shared formula rather than applied at
/// the display site, deliberately: Rage's and Bardic Performance's
/// per-day budgets are each read in three places (the displayed total
/// plus two activation-budget checks), and applying the bonus at only
/// one of them would show a character a widened budget while still
/// judging them over the un-widened one. Putting it inside the formula
/// makes all consumers agree by construction.
///
/// No explicit class-ownership gate is needed here, and that is PROVEN
/// rather than assumed (`extra_feats_never_ground_for_a_character_lacking_
/// the_underlying_class_feature` and
/// `extra_feats_do_not_grant_a_resource_before_its_own_class_level_gate`):
/// each formula is only reached inside its own class's chassis seam,
/// which already returns early for the wrong class and below the
/// resource's own level gate. The feat widens a pool the character has
/// already earned; it never conjures one.
pub(super) fn extra_resource_feat_bonus(selected_feats: &[String], feat_key: &str, bonus: i16) -> i16 {
    let taken = feat_identity::count(selected_feats, feat_key);
    bonus.saturating_mul(i16::try_from(taken).unwrap_or(i16::MAX))
}

/// The Resiliency rogue talent's selection id (task #58, v0.6 alpha
/// swarm), used across every numbered Rogue talent choice slot AND
/// Investigator's own separate talent chooser (`INVESTIGATOR_TALENT_
/// CHOICE_ID`) -- the same talent, two classes' pools, distinguished by
/// which `choice_set_id` it is found under, not by a per-class selection
/// string. Matches the pre-existing `talent:resiliency` fixture value
/// already used by `tests/sd25_rogue_level_up_explanation_filter_audit.rs`
/// (slot 5, rogue level 10), so this grounding recognizes that exact
/// fixture rather than inventing a second naming convention.
pub(super) const RESILIENCY_TALENT_SELECTION: &str = "talent:resiliency";

/// v0.6 alpha swarm item 17 widening (2026-07-24): proves
/// `compute_total_saves` actually applies the real feat-derived save bonus
/// for Great Fortitude/Iron Will/Lightning Reflexes, through the real
/// `compute_pilot_base_chassis` pipeline (not a direct unit call on the
/// private function), against the real Fighter level-1 fixture.
#[cfg(test)]
mod save_boosting_feats_widen_total_saves_tests {
    use super::{compute_pilot_base_chassis, PilotBaseChassisComputation};
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn load() -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(
            result.diagnostics.is_empty(),
            "fixture should load cleanly: {:?}",
            result.diagnostics
        );
        result
            .character_input
            .expect("valid fixture should produce a character input record")
    }

    fn compute(input: &CharacterInput) -> PilotBaseChassisComputation {
        compute_pilot_base_chassis(input)
    }

    #[test]
    fn baseline_fighter_gets_no_save_feat_bonus() {
        let input = load();
        let computation = compute(&input);
        // The fixed loadout carries none of the three save feats -- this
        // is the pre-widening baseline every other assertion below diffs
        // against, so a regression that accidentally always adds the
        // bonus would still be caught here.
        let baseline_fortitude = computation.total_saves.fortitude;
        let baseline_reflex = computation.total_saves.reflex;
        let baseline_will = computation.total_saves.will;

        let mut with_great_fortitude = input.clone();
        with_great_fortitude.chosen.selected_feats.push("Great Fortitude".to_owned());
        let with_gf = compute(&with_great_fortitude);
        assert_eq!(
            with_gf.total_saves.fortitude, baseline_fortitude + 2,
            "Great Fortitude's real +2 must land on total Fortitude specifically"
        );
        assert_eq!(with_gf.total_saves.reflex, baseline_reflex, "must not leak onto Reflex");
        assert_eq!(with_gf.total_saves.will, baseline_will, "must not leak onto Will");

        let mut with_lightning_reflexes = input.clone();
        with_lightning_reflexes.chosen.selected_feats.push("Lightning Reflexes".to_owned());
        let with_lr = compute(&with_lightning_reflexes);
        assert_eq!(
            with_lr.total_saves.reflex, baseline_reflex + 2,
            "Lightning Reflexes' real +2 must land on total Reflex specifically"
        );
        assert_eq!(with_lr.total_saves.fortitude, baseline_fortitude, "must not leak onto Fortitude");
        assert_eq!(with_lr.total_saves.will, baseline_will, "must not leak onto Will");

        let mut with_iron_will = input.clone();
        with_iron_will.chosen.selected_feats.push("Iron Will".to_owned());
        let with_iw = compute(&with_iron_will);
        assert_eq!(
            with_iw.total_saves.will, baseline_will + 2,
            "Iron Will's real +2 must land on total Will specifically"
        );
        assert_eq!(with_iw.total_saves.fortitude, baseline_fortitude, "must not leak onto Fortitude");
        assert_eq!(with_iw.total_saves.reflex, baseline_reflex, "must not leak onto Reflex");
    }

    #[test]
    fn all_three_stack_together_on_the_real_pipeline() {
        let mut input = load();
        let baseline = compute(&input).total_saves;
        input.chosen.selected_feats.push("Great Fortitude".to_owned());
        input.chosen.selected_feats.push("Lightning Reflexes".to_owned());
        input.chosen.selected_feats.push("Iron Will".to_owned());

        let widened = compute(&input).total_saves;

        assert_eq!(widened.fortitude, baseline.fortitude + 2);
        assert_eq!(widened.reflex, baseline.reflex + 2);
        assert_eq!(widened.will, baseline.will + 2);
    }
}

/// v0.6 alpha swarm: proves `compute_selected_skill_modifiers` actually
/// applies the real feat-derived skill bonus for Athletic/Persuasive/
/// Intimidating Prowess, through the real `compute_pilot_base_chassis`
/// pipeline (not a direct unit call on `feat_effects::skill_bonuses_from_feats`
/// itself, which already has its own dedicated test module), against the
/// real Fighter level-1 fixture -- the exact same consumer-wiring shape
/// `save_boosting_feats_widen_total_saves_tests` already established for
/// `compute_total_saves`.
#[cfg(test)]
mod skill_boosting_feats_widen_selected_skill_modifiers_tests {
    use super::{compute_pilot_base_chassis, PilotBaseChassisComputation};
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn load() -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(
            result.diagnostics.is_empty(),
            "fixture should load cleanly: {:?}",
            result.diagnostics
        );
        result
            .character_input
            .expect("valid fixture should produce a character input record")
    }

    fn compute(input: &CharacterInput) -> PilotBaseChassisComputation {
        compute_pilot_base_chassis(input)
    }

    #[test]
    fn baseline_fighter_gets_no_feat_derived_skill_bonus() {
        let input = load();
        let computation = compute(&input);
        // The fixed loadout carries none of the three skill feats -- this is
        // the pre-widening baseline every other assertion below diffs
        // against, so a regression that accidentally always adds the bonus
        // would still be caught here.
        let baseline_climb = computation.selected_skill_modifiers.climb;
        let baseline_intimidate = computation.selected_skill_modifiers.intimidate;
        let baseline_swim = computation.selected_skill_modifiers.swim;

        let mut with_athletic = input.clone();
        with_athletic.chosen.selected_feats.push("Athletic".to_owned());
        let with_a = compute(&with_athletic);
        assert_eq!(
            with_a.selected_skill_modifiers.climb, baseline_climb + 2,
            "Athletic's real +2 must land on Climb"
        );
        assert_eq!(
            with_a.selected_skill_modifiers.swim, baseline_swim + 2,
            "Athletic's real +2 must land on Swim"
        );
        assert_eq!(
            with_a.selected_skill_modifiers.intimidate, baseline_intimidate,
            "Athletic must not leak onto Intimidate"
        );

        let mut with_persuasive = input.clone();
        with_persuasive.chosen.selected_feats.push("Persuasive".to_owned());
        let with_p = compute(&with_persuasive);
        assert_eq!(
            with_p.selected_skill_modifiers.intimidate, baseline_intimidate + 2,
            "Persuasive's real +2 must land on Intimidate"
        );
        assert_eq!(with_p.selected_skill_modifiers.climb, baseline_climb, "must not leak onto Climb");
        assert_eq!(with_p.selected_skill_modifiers.swim, baseline_swim, "must not leak onto Swim");

        let mut with_intimidating_prowess = input.clone();
        with_intimidating_prowess.chosen.selected_feats.push("Intimidating Prowess".to_owned());
        let with_ip = compute(&with_intimidating_prowess);
        let strength_modifier = with_ip.ability_modifiers.strength;
        assert_eq!(
            with_ip.selected_skill_modifiers.intimidate, baseline_intimidate + strength_modifier,
            "Intimidating Prowess adds the real (unclamped) Strength modifier to Intimidate"
        );
        assert_eq!(with_ip.selected_skill_modifiers.climb, baseline_climb, "must not leak onto Climb");
        assert_eq!(with_ip.selected_skill_modifiers.swim, baseline_swim, "must not leak onto Swim");
    }

    #[test]
    fn persuasive_and_intimidating_prowess_stack_together_on_intimidate() {
        let mut input = load();
        let baseline = compute(&input).selected_skill_modifiers;
        input.chosen.selected_feats.push("Persuasive".to_owned());
        input.chosen.selected_feats.push("Intimidating Prowess".to_owned());

        let widened = compute(&input).selected_skill_modifiers;
        let strength_modifier = compute(&input).ability_modifiers.strength;

        assert_eq!(widened.intimidate, baseline.intimidate + 2 + strength_modifier);
        assert_eq!(widened.climb, baseline.climb, "must not leak onto Climb");
        assert_eq!(widened.swim, baseline.swim, "must not leak onto Swim");
    }

    #[test]
    fn all_three_feats_stack_together_on_the_real_pipeline() {
        let mut input = load();
        let baseline = compute(&input).selected_skill_modifiers;
        input.chosen.selected_feats.push("Athletic".to_owned());
        input.chosen.selected_feats.push("Persuasive".to_owned());
        input.chosen.selected_feats.push("Intimidating Prowess".to_owned());

        let widened_computation = compute(&input);
        let widened = widened_computation.selected_skill_modifiers;
        let strength_modifier = widened_computation.ability_modifiers.strength;

        assert_eq!(widened.climb, baseline.climb + 2);
        assert_eq!(widened.swim, baseline.swim + 2);
        assert_eq!(widened.intimidate, baseline.intimidate + 2 + strength_modifier);
    }

    #[test]
    fn explanation_detail_names_the_real_feat_source() {
        let mut input = load();
        input.chosen.selected_feats.push("Athletic".to_owned());
        let computation = compute(&input);

        let climb_detail = computation
            .explanations
            .iter()
            .find(|e| e.id == "skill.selected_modifier.climb")
            .expect("climb explanation should be grounded")
            .detail
            .clone();
        assert!(
            climb_detail.contains("Athletic"),
            "climb explanation should name Athletic as the feat source: {climb_detail}"
        );
    }
}

/// v0.6 alpha swarm: turnkey consumer wiring for the feat-effects agent's
/// own `standalone_skill_facts_from_feats`/`skill_focus_facts_from_choices`
/// (both already built and independently tested in `feat_effects.rs`) --
/// proves each grounds a real, standalone explanation record through the
/// actual `compute_pilot_base_chassis` pipeline, not just `feat_effects.rs`'s
/// own unit-level tests, mirroring `skill_boosting_feats_widen_selected_skill_modifiers_tests`'s
/// own end-to-end shape.
#[cfg(test)]
mod standalone_feat_skill_facts_consumer_wiring_tests {
    use super::{compute_pilot_base_chassis, PilotBaseChassisComputation};
    use crate::rules_core::character_input::{
        load_character_input_fixture, CharacterInput, SelectedChoice,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn load() -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(
            result.diagnostics.is_empty(),
            "fixture should load cleanly: {:?}",
            result.diagnostics
        );
        result
            .character_input
            .expect("valid fixture should produce a character input record")
    }

    fn compute(input: &CharacterInput) -> PilotBaseChassisComputation {
        compute_pilot_base_chassis(input)
    }

    #[test]
    fn baseline_fighter_grounds_no_standalone_feat_skill_facts() {
        let input = load();
        let computation = compute(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("feat.standalone_skill_bonus.")
                    || e.id.starts_with("feat.skill_focus_bonus.")),
            "the fixed loadout carries none of the grounding feats: {:?}",
            computation.explanations
        );
    }

    #[test]
    fn acrobatic_grounds_its_two_uncomputed_skills_as_standalone_facts() {
        let mut input = load();
        input.chosen.selected_feats.push("Acrobatic".to_owned());
        let computation = compute(&input);

        let acrobatics = computation
            .explanations
            .iter()
            .find(|e| e.id == "feat.standalone_skill_bonus.acrobatics")
            .expect("expected the Acrobatics standalone fact to be grounded");
        assert_eq!(acrobatics.value, 2, "{:?}", acrobatics);

        let fly = computation
            .explanations
            .iter()
            .find(|e| e.id == "feat.standalone_skill_bonus.fly")
            .expect("expected the Fly standalone fact to be grounded");
        assert_eq!(fly.value, 2, "{:?}", fly);
    }

    #[test]
    fn persuasive_grounds_only_its_diplomacy_half_as_a_standalone_fact() {
        // Persuasive's Intimidate half is already wired into the real
        // Intimidate total by `skill_bonuses_from_feats` -- only its
        // Diplomacy half (an uncomputed skill) should appear here, with no
        // double-counted Intimidate standalone fact.
        let mut input = load();
        input.chosen.selected_feats.push("Persuasive".to_owned());
        let computation = compute(&input);

        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "feat.standalone_skill_bonus.diplomacy" && e.value == 2),
            "expected the Diplomacy standalone fact: {:?}",
            computation.explanations
        );
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == "feat.standalone_skill_bonus.intimidate"),
            "Intimidate must not be double-grounded as a standalone fact: {:?}",
            computation.explanations
        );
    }

    /// The APG's two `BONUS:SKILL` feats (2026-07-29 widening) reach the
    /// same live consumer the CRB table already does.
    #[test]
    fn the_apg_skill_feats_ground_standalone_records_through_the_real_pipeline() {
        for (feat, ids) in [
            ("Master Alchemist", vec!["feat.standalone_skill_bonus.craft_(alchemy)"]),
            (
                "Breadth of Experience",
                vec![
                    "feat.standalone_skill_bonus.all_knowledge_skills",
                    "feat.standalone_skill_bonus.all_profession_skills",
                ],
            ),
        ] {
            let mut input = load();
            input.chosen.selected_feats.push(feat.to_owned());
            let computation = compute(&input);
            for id in ids {
                assert!(
                    computation.explanations.iter().any(|e| e.id == id && e.value == 2),
                    "expected {feat} to ground {id} at +2: {:?}",
                    computation
                        .explanations
                        .iter()
                        .map(|e| &e.id)
                        .collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn skill_focus_grounds_nothing_without_an_explicit_target_choice() {
        let mut input = load();
        input.chosen.selected_feats.push("Skill Focus".to_owned());
        let computation = compute(&input);

        assert!(
            !computation.explanations.iter().any(|e| e.id.starts_with("feat.skill_focus_bonus.")),
            "Skill Focus with no target choice must ground nothing (no fabricated canonical \
             skill): {:?}",
            computation.explanations
        );
    }

    #[test]
    fn skill_focus_grounds_the_chosen_skill_when_a_real_target_choice_is_present() {
        let mut input = load();
        input.chosen.selected_feats.push("Skill Focus".to_owned());
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:skill_focus_target".to_owned(),
            selection_id: "skill:Stealth".to_owned(),
        });
        let computation = compute(&input);

        let fact = computation
            .explanations
            .iter()
            .find(|e| e.id == "feat.skill_focus_bonus.stealth")
            .expect("expected the Stealth Skill Focus fact to be grounded");
        assert_eq!(fact.value, 3, "{:?}", fact);
    }

    #[test]
    fn skill_focus_grounds_one_fact_per_target_when_taken_more_than_once() {
        let mut input = load();
        input.chosen.selected_feats.push("Skill Focus".to_owned());
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:skill_focus_target".to_owned(),
            selection_id: "skill:Stealth".to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:skill_focus_target".to_owned(),
            selection_id: "skill:Perception".to_owned(),
        });
        let computation = compute(&input);

        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "feat.skill_focus_bonus.stealth" && e.value == 3),
            "{:?}",
            computation.explanations
        );
        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "feat.skill_focus_bonus.perception" && e.value == 3),
            "{:?}",
            computation.explanations
        );
    }

    /// Master Craftsman's consumer wiring. Fourth chooser feat to reach a
    /// real consumer, and the shape is Skill Focus's exactly: Craft and
    /// Profession are not among the three skills
    /// `compute_selected_skill_modifiers` tracks, so the `+2` grounds as
    /// a standalone flat record with no total to layer onto.
    #[test]
    fn master_craftsman_grounds_its_chosen_craft_skill_as_a_standalone_fact() {
        let mut input = load();
        input.chosen.selected_feats.push("Master Craftsman".to_owned());
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:master_craftsman_target".to_owned(),
            selection_id: "skill:Craft (armor)".to_owned(),
        });
        let computation = compute(&input);

        let fact = computation
            .explanations
            .iter()
            .find(|e| e.id == "feat.master_craftsman_bonus.craft_armor")
            .expect("expected the Craft (armor) Master Craftsman fact to be grounded");
        assert_eq!(fact.value, 2, "{:?}", fact);
    }

    /// `STACK:NO MULT:YES` -- repeatable across different Craft/Profession
    /// skills, never stacking on one.
    #[test]
    fn master_craftsman_grounds_one_fact_per_distinct_target() {
        let mut input = load();
        input.chosen.selected_feats.push("Master Craftsman".to_owned());
        for target in ["skill:Craft (armor)", "skill:Profession (siege engineer)"] {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: "choice:master_craftsman_target".to_owned(),
                selection_id: target.to_owned(),
            });
        }
        let computation = compute(&input);

        assert!(
            computation.explanations.iter().any(
                |e| e.id == "feat.master_craftsman_bonus.craft_armor" && e.value == 2
            ),
            "{:?}",
            computation.explanations
        );
        assert!(
            computation.explanations.iter().any(|e| e.id
                == "feat.master_craftsman_bonus.profession_siege_engineer"
                && e.value == 2),
            "{:?}",
            computation.explanations
        );
    }

    /// No-silent-seeding: the feat alone, with no recorded target, grounds
    /// nothing rather than inventing a canonical Craft skill.
    #[test]
    fn master_craftsman_without_a_recorded_target_grounds_nothing() {
        let mut input = load();
        input.chosen.selected_feats.push("Master Craftsman".to_owned());
        let computation = compute(&input);

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("feat.master_craftsman_bonus.")),
            "{:?}",
            computation.explanations
        );
    }
}

/// v0.6 alpha swarm, task #19 (feat-effects Category B): the four
/// "Extra <resource>" General feats each add a flat amount to a per-day
/// resource total that this engine ALREADY computes, rather than
/// introducing a new mechanism.
///
/// Every magnitude was verified twice before these tests were written --
/// against this repo's own `feat_data/general.rs` catalog and against the
/// raw PCGen `cr_feats.lst` record -- and they agree:
/// `BONUS:VAR|RageDuration|6`, `BONUS:VAR|BardicPerformanceDuration|6`,
/// `BONUS:VAR|KiPoints|2`, `BONUS:VAR|LayOnHandsTimes|2`.
///
/// Each test differences the SAME character with and without the feat, so
/// no assertion can pass on a coincidental absolute value, and each
/// checks that sibling resources are untouched.
#[cfg(test)]
mod extra_resource_feat_tests {
    use super::{ActiveState, build_pilot_headless_receipt, CharacterClassLevel, CharacterInput};
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation,
    };

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

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// Drives one Extra-<resource> feat end to end: the same character
    /// with and without the feat, asserting the named total rises by
    /// exactly the corpus magnitude.
    fn assert_feat_adds(class_id: &str, level: u8, feat: &str, total_id: &str, expected: i16) {
        let without = character(class_id, level);
        let base = explanation_value(&without, total_id)
            .unwrap_or_else(|| panic!("{total_id} must be computed for a bare {class_id}"));

        let mut with = without.clone();
        with.chosen.selected_feats.push(feat.to_owned());
        let raised = explanation_value(&with, total_id)
            .unwrap_or_else(|| panic!("{total_id} must still be computed with {feat}"));

        assert_eq!(
            raised - base,
            expected,
            "{feat} must raise {total_id} by exactly {expected} (base {base}, with feat {raised})"
        );
    }

    #[test]
    fn extra_rage_adds_six_rounds_to_the_real_rage_total() {
        assert_feat_adds(
            "class:barbarian",
            1,
            "Extra Rage",
            "class_chassis.barbarian.rage_rounds_per_day",
            6,
        );
    }

    #[test]
    fn extra_performance_adds_six_rounds_to_the_real_bardic_performance_total() {
        assert_feat_adds(
            "class:bard",
            1,
            "Extra Performance",
            "class_chassis.bard.bardic_performance_rounds_per_day",
            6,
        );
    }

    #[test]
    fn extra_ki_adds_two_points_to_the_real_ki_pool_total() {
        // Monk's Ki Pool is a 4th-level class feature.
        assert_feat_adds("class:monk", 4, "Extra Ki", "class_chassis.monk.ki_pool_size", 2);
    }

    #[test]
    fn extra_lay_on_hands_adds_two_uses_to_the_real_lay_on_hands_total() {
        // Paladin's Lay On Hands is a 2nd-level class feature.
        assert_feat_adds(
            "class:paladin",
            2,
            "Extra Lay On Hands",
            "class_chassis.paladin.lay_on_hands_uses_per_day",
            2,
        );
    }

    /// None of the four may leak onto a resource it does not name. Each
    /// feat is applied to a character who owns a DIFFERENT resource, and
    /// that resource must not move.
    #[test]
    fn each_extra_feat_leaves_every_sibling_resource_untouched() {
        let cases = [
            ("class:barbarian", 1u8, "class_chassis.barbarian.rage_rounds_per_day"),
            ("class:bard", 1, "class_chassis.bard.bardic_performance_rounds_per_day"),
            ("class:monk", 4, "class_chassis.monk.ki_pool_size"),
            ("class:paladin", 2, "class_chassis.paladin.lay_on_hands_uses_per_day"),
        ];
        let feats = ["Extra Rage", "Extra Performance", "Extra Ki", "Extra Lay On Hands"];

        for (index, (class_id, level, total_id)) in cases.iter().enumerate() {
            let baseline = character(class_id, *level);
            let base = explanation_value(&baseline, total_id)
                .unwrap_or_else(|| panic!("{total_id} must be computed"));

            for (feat_index, feat) in feats.iter().enumerate() {
                if feat_index == index {
                    continue;
                }
                let mut other = baseline.clone();
                other.chosen.selected_feats.push((*feat).to_owned());
                assert_eq!(
                    explanation_value(&other, total_id),
                    Some(base),
                    "{feat} must not move {total_id}"
                );
            }
        }
    }

    /// All four feats are `STACK:YES MULT:YES` in the corpus -- genuinely
    /// repeatable, with each instance stacking. Taking Extra Rage twice
    /// grants 12 rounds, not 6, so the bonus must COUNT occurrences in
    /// `selected_feats` rather than merely test for presence.
    ///
    /// Missed on the first pass (2026-07-27) because the corpus record
    /// was read through a `grep -E "^(BONUS|PRE)"` filter that discarded
    /// the `STACK:`/`MULT:` lines entirely -- a filter chosen around the
    /// fields expected to matter, which hid one that did.
    #[test]
    fn repeated_extra_feats_stack_because_all_four_are_stack_yes_mult_yes() {
        let cases = [
            ("class:barbarian", 1u8, "Extra Rage", "class_chassis.barbarian.rage_rounds_per_day", 6),
            (
                "class:bard",
                1,
                "Extra Performance",
                "class_chassis.bard.bardic_performance_rounds_per_day",
                6,
            ),
            ("class:monk", 4, "Extra Ki", "class_chassis.monk.ki_pool_size", 2),
            (
                "class:paladin",
                2,
                "Extra Lay On Hands",
                "class_chassis.paladin.lay_on_hands_uses_per_day",
                2,
            ),
        ];

        for (class_id, level, feat, total_id, per_instance) in cases {
            let base = character(class_id, level);
            let baseline = explanation_value(&base, total_id).expect("total must compute");

            for instances in 1..=3i16 {
                let mut input = base.clone();
                for _ in 0..instances {
                    input.chosen.selected_feats.push(feat.to_owned());
                }
                let raised = explanation_value(&input, total_id).expect("total must compute");
                assert_eq!(
                    raised - baseline,
                    per_instance * instances,
                    "{feat} taken {instances}x must grant {per_instance} each ({class_id})"
                );
            }
        }
    }

    /// Each feat carries `PREABILITY:1,CATEGORY=Special Ability,TYPE.<X>`
    /// -- it requires the underlying class feature. A character without
    /// that class must never have the feat grounded against some other
    /// class's total, and must not gain the resource record at all.
    #[test]
    fn extra_feats_never_ground_for_a_character_lacking_the_underlying_class_feature() {
        let totals = [
            "class_chassis.barbarian.rage_rounds_per_day",
            "class_chassis.bard.bardic_performance_rounds_per_day",
            "class_chassis.monk.ki_pool_size",
            "class_chassis.paladin.lay_on_hands_uses_per_day",
        ];
        let mut fighter = character("class:fighter", 1);
        for feat in ["Extra Rage", "Extra Performance", "Extra Ki", "Extra Lay On Hands"] {
            fighter.chosen.selected_feats.push(feat.to_owned());
        }

        let receipt = build_pilot_headless_receipt(&fighter);
        for total_id in totals {
            assert!(
                !receipt.computation.explanations.iter().any(|e| e.id == total_id),
                "a Fighter holding every Extra feat must not gain {total_id}: {:?}",
                receipt.computation.explanations.iter().map(|e| &e.id).collect::<Vec<_>>()
            );
        }
    }

    /// A Monk below the Ki Pool's own 4th-level gate, and a Paladin below
    /// Lay On Hands' 2nd-level gate, must not gain the resource from the
    /// feat alone -- the feat widens an existing pool, it does not create
    /// one the character has not yet earned.
    #[test]
    fn extra_feats_do_not_grant_a_resource_before_its_own_class_level_gate() {
        let mut monk = character("class:monk", 1);
        monk.chosen.selected_feats.push("Extra Ki".to_owned());
        let ki = explanation_value(&monk, "class_chassis.monk.ki_pool_size");
        assert!(
            ki.is_none() || ki == Some(0),
            "a level-1 Monk has no ki pool for Extra Ki to widen, got {ki:?}"
        );

        let mut paladin = character("class:paladin", 1);
        paladin.chosen.selected_feats.push("Extra Lay On Hands".to_owned());
        let loh = explanation_value(&paladin, "class_chassis.paladin.lay_on_hands_uses_per_day");
        assert!(
            loh.is_none() || loh == Some(0),
            "a level-1 Paladin has no lay on hands for the feat to widen, got {loh:?}"
        );
    }

    /// The load-bearing consistency check. Rage's and Bardic
    /// Performance's per-day budgets are read in THREE separate places
    /// each (the displayed total plus two activation-budget checks), so
    /// raising only the displayed total would let a character be shown 13
    /// rounds while still being judged over budget at 8 -- the same
    /// two-spot divergence the Rage/Charmed Life budget work already hit
    /// once.
    ///
    /// A Barbarian with Extra Rage who has consumed more than the BASE
    /// budget but no more than the EXTENDED one must still be validly
    /// raging, which is only true if the budget check sees the feat too.
    #[test]
    fn extra_rage_widens_the_activation_budget_not_merely_the_displayed_total() {
        let mut input = character("class:barbarian", 1);
        input.chosen.selected_feats.push("Extra Rage".to_owned());
        // Fixture Constitution 14 (+2): base budget is 4 + 2 = 6 rounds,
        // extended to 12 by Extra Rage. 9 is over base, within extended.
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: "rage".to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(9),
        });

        let receipt = build_pilot_headless_receipt(&input);
        let over_budget = receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("rounds_exceeded") || d.id.contains("budget"));

        assert!(
            !over_budget,
            "9 rounds is within the Extra-Rage-extended budget; the activation check must see \
             the feat, not just the displayed total: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Same three-spot property for Bardic Performance, whose displayed
    /// total is computed INLINE while its two budget checks call the
    /// shared formula -- the likeliest place for the two to drift apart.
    #[test]
    fn extra_performance_widens_the_activation_budget_not_merely_the_displayed_total() {
        let mut input = character("class:bard", 1);
        input.chosen.selected_feats.push("Extra Performance".to_owned());
        // Fixture Charisma 8 (-1): base budget is 4 + (-1) = 3 rounds,
        // extended to 9. 5 is over base, within extended.
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: "bardic_performance".to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(5),
        });

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("rounds_exceeded")),
            "5 rounds is within the Extra-Performance-extended budget; the activation check must \
             see the feat: {:?}",
            receipt.computation.diagnostics
        );
    }

    // ---------------------------------------------------------------
    // v0.6 alpha swarm, `BONUS:VAR`/`BONUS:DC`/`BONUS:ABILITYPOOL`
    // triage slice (2026-07-29): the five APG/ACG Extra-<resource>
    // feats whose pool this engine ALREADY computes, plus the ACG
    // `.MOD` that extends the already-shipped Extra Performance onto
    // Skald's Raging Song.
    //
    // Every magnitude below is the corpus `BONUS:VAR` token AND the
    // record's own `BENEFIT:` prose in agreement -- the disagreement
    // shape that deferred Extra Channel (`ABILITYPOOL|1` vs "two
    // additional times per day") does not occur in any of these six.
    // ---------------------------------------------------------------

    /// APG `Extra Bombs`: `BONUS:VAR|AlchemistBombTimes|2`, prose "You
    /// can throw two additional bombs per day."
    #[test]
    fn extra_bombs_adds_two_uses_to_the_real_alchemist_bomb_total() {
        assert_feat_adds(
            "class:alchemist",
            1,
            "Extra Bombs",
            "class_feature.apg.alchemist.bomb_uses_per_day",
            2,
        );
    }

    /// ACG `Extra Inspiration`: `BONUS:VAR|InvestigatorInspirationPool\
    /// Bonus|3`, prose "You gain three extra use per day of inspiration
    /// in your inspiration pool" (and its own Special clause repeats
    /// "three extra uses"). The base formula clamps at `max(1, ...)`;
    /// the feat's `BONUS:VAR` is a separate additive contribution to
    /// the same variable, so it lands OUTSIDE that clamp.
    #[test]
    fn extra_inspiration_adds_three_uses_to_the_real_investigator_pool() {
        assert_feat_adds(
            "class:investigator",
            1,
            "Extra Inspiration",
            "class_feature.acg.investigator.inspiration_pool_size",
            3,
        );
    }

    /// ACG `Extra Martial Flexibility`:
    /// `BONUS:VAR|BrawlerMartialFlexibilityTimes|3`, prose "You can use
    /// your martial flexibility ability three additional times per day."
    #[test]
    fn extra_martial_flexibility_adds_three_uses_to_the_real_brawler_pool() {
        assert_feat_adds(
            "class:brawler",
            1,
            "Extra Martial Flexibility",
            "class_feature.acg.brawler.martial_flexibility_uses_per_day",
            3,
        );
    }

    /// ACG `Extra Panache`: `BONUS:VAR|Panache_Cap|2` (alongside
    /// `BONUS:VAR|PanachePoints|2`), prose "your maximum panache
    /// increases by two."
    #[test]
    fn extra_panache_adds_two_points_to_the_real_swashbuckler_maximum() {
        assert_feat_adds(
            "class:swashbuckler",
            1,
            "Extra Panache",
            "class_feature.acg.swashbuckler.panache_max",
            2,
        );
    }

    /// ACG `Extra Reservoir`: `BONUS:VAR|MaxArcanistReservoirSize|3`,
    /// prose "the maximum number of points in your arcane reservoir
    /// increases by that amount."
    #[test]
    fn extra_reservoir_adds_three_points_to_the_real_arcanist_maximum() {
        assert_feat_adds(
            "class:arcanist",
            1,
            "Extra Reservoir",
            "class_feature.acg.arcanist.arcane_reservoir_max",
            3,
        );
    }

    /// The corpus-decided half of Extra Reservoir, and the reason this
    /// feat needed reading rather than paraphrasing.
    ///
    /// Its prose opens "You gain three more points in your arcane
    /// reservoir, AND the maximum ... increases by that amount", which
    /// reads like both the daily fill and the cap rise. The corpus
    /// settles it: `acg_abilities_class.lst`'s own `Arcanist ~ Arcane
    /// Reservoir` record carries TWO distinct variables --
    /// `BONUS:VAR|ArcanistReservoirSize|3+ArcanistLVL/2` (the daily
    /// fill) and `BONUS:VAR|MaxArcanistReservoirSize|3+ArcanistLVL`
    /// (the cap) -- matching this engine's own
    /// `arcanist_reservoir_daily_fill`/`arcanist_reservoir_max` exactly.
    /// Extra Reservoir names ONLY `MaxArcanistReservoirSize`. So the cap
    /// rises by 3 and the daily fill does not move, and asserting that
    /// negative is what keeps the prose's first clause from being
    /// silently double-counted.
    #[test]
    fn extra_reservoir_raises_only_the_cap_never_the_daily_fill() {
        let without = character("class:arcanist", 1);
        let base_fill =
            explanation_value(&without, "class_feature.acg.arcanist.arcane_reservoir_daily_fill")
                .expect("daily fill must be computed for a bare Arcanist");

        let mut with = without.clone();
        with.chosen.selected_feats.push("Extra Reservoir".to_owned());
        assert_eq!(
            explanation_value(&with, "class_feature.acg.arcanist.arcane_reservoir_daily_fill"),
            Some(base_fill),
            "Extra Reservoir names only MaxArcanistReservoirSize; the daily fill must not move"
        );
    }

    /// The already-shipped CRB `Extra Performance` reaches a SECOND
    /// pool that nothing here read before.
    ///
    /// `acg_feats.lst` carries three `CATEGORY=FEAT|Extra
    /// Performance.MOD` records: `PRE:.CLEAR`, then
    /// `BONUS:VAR|SkaldRagingSongRoundsPerDay|6`, then a re-widened
    /// `PREMULT:1,[...TYPE.SkaldRagingSong],[...TYPE.Bardic
    /// Performance]`. Read only against `cr_feats.lst`, this feat looks
    /// Bard-only -- exactly the `.MOD`-carries-the-real-value trap, in
    /// its additive direction. A Skald holding Extra Performance got
    /// nothing before this test.
    #[test]
    fn extra_performance_also_adds_six_rounds_to_the_real_skald_raging_song_total() {
        assert_feat_adds(
            "class:skald",
            1,
            "Extra Performance",
            "class_feature.acg.skald.raging_song_rounds_per_day",
            6,
        );
    }

    /// Skald's Raging Song budget is read in THREE places (the
    /// displayed total plus two activation-budget checks), the same
    /// shape that already bit Rage and Bardic Performance. Fixture
    /// Charisma 8 (-1): base budget is 3 + (-1) = 2 rounds, extended to
    /// 8 by Extra Performance. 5 is over base, within extended.
    #[test]
    fn extra_performance_widens_the_skald_activation_budget_not_merely_the_total() {
        let mut input = character("class:skald", 1);
        input.chosen.selected_feats.push("Extra Performance".to_owned());
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: "inspired_rage".to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(5),
        });

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            !receipt.computation.diagnostics.iter().any(|d| d.id.contains("rounds_exceeded")),
            "5 rounds is within the Extra-Performance-extended Skald budget; the activation \
             check must see the feat: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Four of the five new feats are `STACK:YES MULT:YES` with an
    /// explicit "you can take this feat multiple times" Special clause,
    /// so each instance must add again -- the same counting contract
    /// the original four established.
    #[test]
    fn repeated_new_extra_feats_stack_where_the_corpus_says_stack_yes() {
        let cases = [
            (
                "class:alchemist",
                1u8,
                "Extra Bombs",
                "class_feature.apg.alchemist.bomb_uses_per_day",
                2i16,
            ),
            (
                "class:investigator",
                1,
                "Extra Inspiration",
                "class_feature.acg.investigator.inspiration_pool_size",
                3,
            ),
            (
                "class:swashbuckler",
                1,
                "Extra Panache",
                "class_feature.acg.swashbuckler.panache_max",
                2,
            ),
            (
                "class:arcanist",
                1,
                "Extra Reservoir",
                "class_feature.acg.arcanist.arcane_reservoir_max",
                3,
            ),
        ];

        for (class_id, level, feat, total_id, per_instance) in cases {
            let base = character(class_id, level);
            let baseline = explanation_value(&base, total_id).expect("total must compute");

            for instances in 1..=3i16 {
                let mut input = base.clone();
                for _ in 0..instances {
                    input.chosen.selected_feats.push(feat.to_owned());
                }
                let raised = explanation_value(&input, total_id).expect("total must compute");
                assert_eq!(
                    raised - baseline,
                    per_instance * instances,
                    "{feat} taken {instances}x must grant {per_instance} each ({class_id})"
                );
            }
        }
    }

    /// The one that breaks the pattern, and the reason each record was
    /// read whole rather than swept for `BONUS:VAR`.
    ///
    /// `Extra Martial Flexibility` carries NO `STACK:`, NO `MULT:` and
    /// NO `CHOOSE:` token, and its `BENEFIT:` has no "Special: you can
    /// take this feat multiple times" clause -- unique among all nine
    /// Extra-<resource> feats this engine grounds. It is not
    /// repeatable, so it must test for PRESENCE, not count occurrences.
    /// Reusing the shared counting helper here would silently grant a
    /// Brawler +9 for a build the rules forbid.
    #[test]
    fn extra_martial_flexibility_does_not_stack_because_it_is_not_mult_yes() {
        let base = character("class:brawler", 1);
        let total_id = "class_feature.acg.brawler.martial_flexibility_uses_per_day";
        let baseline = explanation_value(&base, total_id).expect("total must compute");

        for instances in 1..=3 {
            let mut input = base.clone();
            for _ in 0..instances {
                input.chosen.selected_feats.push("Extra Martial Flexibility".to_owned());
            }
            assert_eq!(
                explanation_value(&input, total_id),
                Some(baseline + 3),
                "Extra Martial Flexibility is not MULT:YES; {instances} copies must still be +3"
            );
        }
    }

    /// Each new feat carries a `PREABILITY`/`PREMULT` on its own class
    /// resource. A Fighter holding all five must gain none of the five
    /// totals -- the feat widens a pool the character earned, it never
    /// conjures one.
    #[test]
    fn new_extra_feats_never_ground_for_a_character_lacking_the_class_feature() {
        let totals = [
            "class_feature.apg.alchemist.bomb_uses_per_day",
            "class_feature.acg.investigator.inspiration_pool_size",
            "class_feature.acg.brawler.martial_flexibility_uses_per_day",
            "class_feature.acg.swashbuckler.panache_max",
            "class_feature.acg.arcanist.arcane_reservoir_max",
            "class_feature.acg.skald.raging_song_rounds_per_day",
        ];
        let mut fighter = character("class:fighter", 1);
        for feat in [
            "Extra Bombs",
            "Extra Inspiration",
            "Extra Martial Flexibility",
            "Extra Panache",
            "Extra Reservoir",
            "Extra Performance",
        ] {
            fighter.chosen.selected_feats.push(feat.to_owned());
        }

        let receipt = build_pilot_headless_receipt(&fighter);
        for total_id in totals {
            assert!(
                !receipt.computation.explanations.iter().any(|e| e.id == total_id),
                "a Fighter holding every new Extra feat must not gain {total_id}"
            );
        }
    }

    /// No new feat may leak onto a sibling pool it does not name.
    #[test]
    fn each_new_extra_feat_leaves_every_sibling_resource_untouched() {
        let cases = [
            ("class:alchemist", 1u8, "class_feature.apg.alchemist.bomb_uses_per_day"),
            ("class:investigator", 1, "class_feature.acg.investigator.inspiration_pool_size"),
            ("class:brawler", 1, "class_feature.acg.brawler.martial_flexibility_uses_per_day"),
            ("class:swashbuckler", 1, "class_feature.acg.swashbuckler.panache_max"),
            ("class:arcanist", 1, "class_feature.acg.arcanist.arcane_reservoir_max"),
            ("class:skald", 1, "class_feature.acg.skald.raging_song_rounds_per_day"),
        ];
        let feats = [
            "Extra Bombs",
            "Extra Inspiration",
            "Extra Martial Flexibility",
            "Extra Panache",
            "Extra Reservoir",
            "Extra Performance",
        ];

        for (index, (class_id, level, total_id)) in cases.iter().enumerate() {
            let baseline = character(class_id, *level);
            let base = explanation_value(&baseline, total_id)
                .unwrap_or_else(|| panic!("{total_id} must be computed"));

            for (feat_index, feat) in feats.iter().enumerate() {
                if feat_index == index {
                    continue;
                }
                let mut other = baseline.clone();
                other.chosen.selected_feats.push((*feat).to_owned());
                assert_eq!(
                    explanation_value(&other, total_id),
                    Some(base),
                    "{feat} must not move {total_id}"
                );
            }
        }
    }
}

/// v0.6 alpha swarm, task #20 (2026-07-27): class-granted feats must
/// reach the `feat_effects` producers.
///
/// Three feats are granted automatically by class level and never appear
/// in `selected_feats`, because a player never picks them: Monk's
/// Improved Unarmed Strike and Stunning Fist at 1st level, and Ranger's
/// Endurance at 3rd. Every producer keys on `selected_feats`, so those
/// characters were invisible to every feat-presence check.
#[cfg(test)]
mod class_granted_feat_tests {
    use super::{class_granted_feats, CharacterClassLevel, CharacterInput};
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    /// Monk gains BOTH Improved Unarmed Strike and Stunning Fist at 1st
    /// level ("even if he does not meet the prerequisites"), verified
    /// against `cr_abilities_class.lst`'s own
    /// `ABILITY:FEAT|AUTOMATIC|Improved Unarmed Strike` and
    /// `ABILITY:FEAT|VIRTUAL|Stunning Fist`.
    #[test]
    fn a_monk_is_granted_improved_unarmed_strike_and_stunning_fist_from_first_level() {
        for level in [1u8, 4, 20] {
            let granted = class_granted_feats(&character("class:monk", level));
            assert!(
                granted.contains(&"Improved Unarmed Strike"),
                "level {level} Monk must be granted Improved Unarmed Strike: {granted:?}"
            );
            assert!(
                granted.contains(&"Stunning Fist"),
                "level {level} Monk must be granted Stunning Fist: {granted:?}"
            );
        }
    }

    /// Ranger's Endurance is gated at 3rd level
    /// (`PREVARGTEQ:Ranger_CFP_Level,3`), so a level 1-2 Ranger must NOT
    /// be credited with it.
    #[test]
    fn a_ranger_is_granted_endurance_only_from_third_level() {
        for level in [1u8, 2] {
            let granted = class_granted_feats(&character("class:ranger", level));
            assert!(
                !granted.contains(&"Endurance"),
                "level {level} Ranger has not gained Endurance yet: {granted:?}"
            );
        }
        for level in [3u8, 5, 20] {
            let granted = class_granted_feats(&character("class:ranger", level));
            assert!(
                granted.contains(&"Endurance"),
                "level {level} Ranger genuinely has Endurance: {granted:?}"
            );
        }
    }

    /// No other class grants any of these, and a Fighter grants none.
    #[test]
    fn a_class_without_automatic_feat_grants_is_credited_with_none() {
        for class_id in ["class:fighter", "class:wizard", "class:barbarian"] {
            assert!(
                class_granted_feats(&character(class_id, 20)).is_empty(),
                "{class_id} grants no automatic feats in this codebase"
            );
        }
    }

    /// A multiclass Monk/Ranger gets both classes' grants, each at its
    /// own level gate.
    #[test]
    fn a_multiclass_character_gets_each_classs_own_grants_at_its_own_gate() {
        let mut input = character("class:monk", 1);
        input.chosen.class_levels.push(CharacterClassLevel {
            class_id: "class:ranger".to_owned(),
            level: 3,
        });
        let granted = class_granted_feats(&input);
        assert!(granted.contains(&"Stunning Fist"));
        assert!(granted.contains(&"Improved Unarmed Strike"));
        assert!(granted.contains(&"Endurance"));
    }

    /// The granted feats genuinely reach the `feat_effects` producers
    /// through `effective_feats` -- the whole point of the wiring. Proven
    /// against the real union rather than the derivation alone.
    #[test]
    fn granted_feats_reach_the_producers_through_effective_feats() {
        let input = character("class:ranger", 3);
        let effective = crate::rules_core::feat_effects::effective_feats(
            &input.chosen.selected_feats,
            &class_granted_feats(&input),
        );
        assert!(
            effective.iter().any(|f| f == "Endurance"),
            "a level-3 Ranger's Endurance must be visible to every producer: {effective:?}"
        );
        assert!(
            crate::rules_core::feat_effects::endurance_check_bonus_from_feats(&effective) > 0,
            "the producer that keys on Endurance must now see it"
        );
    }

    /// A Ranger who ALSO picked Endurance from the catalog must not hold
    /// it twice -- `effective_feats` appends only when absent, and that
    /// matters because other producers count duplicates for STACK:YES
    /// feats.
    #[test]
    fn a_granted_feat_already_chosen_is_not_duplicated() {
        let mut input = character("class:ranger", 3);
        input.chosen.selected_feats.push("Endurance".to_owned());
        let effective = crate::rules_core::feat_effects::effective_feats(
            &input.chosen.selected_feats,
            &class_granted_feats(&input),
        );
        assert_eq!(
            effective.iter().filter(|f| *f == "Endurance").count(),
            1,
            "Endurance must appear exactly once: {effective:?}"
        );
    }
}

/// v0.6 alpha swarm, Tier 0 of the opponent-interaction architecture
/// work (2026-07-27, risks item 52): the four target-conditioned
/// features whose formulas turn out to read only the character's OWN
/// level, needing no opponent-tracking pillar at all.
#[cfg(test)]
mod opponent_conditioned_tier_zero_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput,
        ActiveState, CAVALIER_CHALLENGE_ABILITY_ID, CAVALIER_CLASS_ID, INVESTIGATOR_CLASS_ID,
        INVESTIGATOR_STUDIED_DEFENSE_CHOICE_ID, INVESTIGATOR_STUDIED_DEFENSE_SELECTION,
        SLAYER_CLASS_ID};
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SelectedChoice,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// Studied Target: `SlayerLVL/5 + 1`, and the simultaneous-target
    /// count `(L>0)+(L>6)` -- one target from 1st, two from 7th.
    #[test]
    fn slayer_studied_target_bonus_and_target_count_match_the_corpus() {
        for (level, want) in [(1u8, 1i16), (4, 1), (5, 2), (9, 2), (10, 3), (20, 5)] {
            assert_eq!(super::slayer_studied_target_bonus(level), want, "level {level}");
        }
        for (level, want) in [(1u8, 1i16), (6, 1), (7, 2), (20, 2)] {
            assert_eq!(super::slayer_studied_target_count(level), want, "level {level}");
        }
        assert_eq!(value(&character(SLAYER_CLASS_ID, 5), "class_feature.acg.slayer.studied_target_bonus"), Some(2));
        assert_eq!(value(&character(SLAYER_CLASS_ID, 7), "class_feature.acg.slayer.studied_target_count"), Some(2));
    }

    /// Stalker follows a TWO-HOP corpus variable chain
    /// (`SlayerStalkerBonus = SlayerStalkerLVL/5+1`, `SlayerStalkerLVL =
    /// SlayerStudiedTargetLVL`, `SlayerStudiedTargetLVL = SlayerLVL`).
    /// Stopping at the first hop leaves `SlayerStalkerLVL` at its
    /// `DEFINE` default of 0 and produces a flat +1 forever -- which is
    /// why this asserts a level where the answer is NOT 1.
    #[test]
    fn slayer_stalker_follows_the_full_variable_chain_not_its_define_default() {
        for (level, want) in [(7u8, 2i16), (9, 2), (10, 3), (14, 3), (15, 4), (20, 5)] {
            assert_eq!(super::slayer_stalker_bonus(level), want, "level {level}");
        }
        assert_eq!(value(&character(SLAYER_CLASS_ID, 6), "class_feature.acg.slayer.stalker_bonus"), None);
        assert_eq!(value(&character(SLAYER_CLASS_ID, 7), "class_feature.acg.slayer.stalker_bonus"), Some(2));
        assert_eq!(value(&character(SLAYER_CLASS_ID, 20), "class_feature.acg.slayer.stalker_bonus"), Some(5));
    }

    /// Slayer's Advance is `1+(SlayerLVL>16)` -- a boolean step, not a
    /// division. It is 1/day across its whole 13-16 band and 2/day from
    /// 17, and must never scale smoothly.
    #[test]
    fn slayers_advance_is_a_boolean_step_at_seventeen_not_a_progression() {
        for (level, want) in [(13u8, 1i16), (16, 1), (17, 2), (20, 2)] {
            assert_eq!(super::slayer_advance_uses_per_day(level), want, "level {level}");
        }
        assert_eq!(value(&character(SLAYER_CLASS_ID, 12), "class_feature.acg.slayer.advance_uses_per_day"), None);
        assert_eq!(value(&character(SLAYER_CLASS_ID, 13), "class_feature.acg.slayer.advance_uses_per_day"), Some(1));
        assert_eq!(value(&character(SLAYER_CLASS_ID, 17), "class_feature.acg.slayer.advance_uses_per_day"), Some(2));
    }

    /// Quarry and Improved Quarry are MUTUALLY EXCLUSIVE in the corpus:
    /// Improved Quarry's TYPE tag suppresses the Quarry Output record.
    /// Emitting both would let a reader sum them to +6.
    #[test]
    fn improved_quarry_supersedes_quarry_rather_than_stacking() {
        let quarry = "class_feature.acg.slayer.quarry_attack_bonus";
        let improved = "class_feature.acg.slayer.improved_quarry_attack_bonus";

        assert_eq!(value(&character(SLAYER_CLASS_ID, 13), quarry), None);
        assert_eq!(value(&character(SLAYER_CLASS_ID, 14), quarry), Some(2));
        assert_eq!(value(&character(SLAYER_CLASS_ID, 18), quarry), Some(2));
        assert_eq!(value(&character(SLAYER_CLASS_ID, 18), improved), None);

        // From 19 the base record must DISAPPEAR, not coexist.
        assert_eq!(value(&character(SLAYER_CLASS_ID, 19), quarry), None);
        assert_eq!(value(&character(SLAYER_CLASS_ID, 19), improved), Some(4));
        assert_eq!(value(&character(SLAYER_CLASS_ID, 20), quarry), None);
        assert_eq!(value(&character(SLAYER_CLASS_ID, 20), improved), Some(4));
    }

    /// Master Slayer's DC keys off INTELLIGENCE, not a casting stat --
    /// Slayer has none -- and off the modifier, not the score.
    #[test]
    fn master_slayer_dc_uses_the_intelligence_modifier() {
        assert_eq!(super::slayer_master_slayer_dc(20, 3), 23);
        assert_eq!(super::slayer_master_slayer_dc(20, 0), 20);
        assert_eq!(super::slayer_master_slayer_dc(20, -1), 19);
        // A score-instead-of-modifier bug would land near 10+10+13.
        assert!(super::slayer_master_slayer_dc(20, 3) < 30);

        assert_eq!(value(&character(SLAYER_CLASS_ID, 19), "class_feature.acg.slayer.master_slayer_dc"), None);
        assert!(value(&character(SLAYER_CLASS_ID, 20), "class_feature.acg.slayer.master_slayer_dc").is_some());
    }

    /// Swift Tracker is a zero-magnitude record: the -5/-10/-20 in its
    /// text are the normal penalties it waives, not derived quantities.
    /// It grounds as a bounded grant-only identity record quoting the
    /// real corpus DESC, and must cite its own namespaced key so it can
    /// never be confused with `KEY:Hunter ~ Swift Tracker`.
    #[test]
    fn slayer_swift_tracker_grounds_as_a_zero_magnitude_identity_record() {
        let id = "class_feature.acg.slayer.swift_tracker_grant";
        assert_eq!(value(&character(SLAYER_CLASS_ID, 10), id), None);
        assert_eq!(value(&character(SLAYER_CLASS_ID, 11), id), Some(0));

        let receipt = build_pilot_headless_receipt(&character(SLAYER_CLASS_ID, 11));
        let detail = &receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .expect("Swift Tracker grounds at 11")
            .detail;
        assert!(
            detail.contains("KEY:Slayer ~ Swift Tracker") && detail.contains("Hunter ~ Swift Tracker"),
            "the record must cite its own namespaced key and disambiguate from Hunter's: {detail}"
        );
    }




    /// The shared Witch hex save DC: `10 + INT + WitchLVL/2`. Every one
    /// of the 53 hex records aliases this single variable
    /// (`WitchHexDC_<Name>|WitchHexDC`), so it is ONE mechanism, not 27.
    #[test]
    fn the_witch_hex_save_dc_is_one_shared_formula_across_every_hex() {
        for (level, intelligence, want) in
            [(1u8, 0i16, 10i16), (1, 4, 14), (2, 4, 15), (10, 4, 19), (20, 5, 25)]
        {
            assert_eq!(super::witch_hex_save_dc(level, intelligence), want, "level {level}");
        }
        // Fixture INT 10 -> modifier 0; level 6 -> 10 + 0 + 3 = 13.
        assert_eq!(value(&character("class:witch", 6), "class_feature.apg.witch.hex_save_dc"), Some(13));
    }

    /// Cauldron and Flight are the only two hexes carrying a magnitude
    /// distinct from the shared DC. Each needs an explicit pick.
    #[test]
    fn cauldron_and_flight_ground_their_own_distinct_magnitudes_when_chosen() {
        let bare = character("class:witch", 6);
        assert_eq!(value(&bare, "class_feature.apg.witch.cauldron_hex.craft_alchemy_bonus"), None);

        let mut cauldron = bare.clone();
        cauldron.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: super::CAULDRON_HEX_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&cauldron, "class_feature.apg.witch.cauldron_hex.craft_alchemy_bonus"),
            Some(4)
        );
    }

    /// Flight's `+4 Swim` is the standout: Swim IS one of the three
    /// skills this engine computes, so it INTEGRATES into the real
    /// total. Proven by differencing the same character with and without
    /// the hex, so the assertion cannot pass on a standalone record.
    #[test]
    fn the_flight_hex_actually_raises_the_real_computed_swim_total() {
        let without = character("class:witch", 6);
        let base = value(&without, "skill.selected_modifier.swim")
            .expect("swim modifier must be computed for a Witch");

        let mut with = without.clone();
        with.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: super::FLIGHT_HEX_SELECTION.to_owned(),
        });
        let raised = value(&with, "skill.selected_modifier.swim").expect("still computed");

        assert_eq!(raised - base, 4, "the Flight hex is worth 4 real Swim");
    }

    /// Neither hex leaks onto a non-Witch, and Flight must not move a
    /// non-Witch's Swim total.
    #[test]
    fn the_hexes_never_apply_to_a_non_witch() {
        let mut fighter = character("class:fighter", 6);
        let clean_swim = value(&fighter, "skill.selected_modifier.swim");
        fighter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: super::FLIGHT_HEX_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&fighter, "skill.selected_modifier.swim"),
            clean_swim,
            "a Fighter's Swim must be untouched by a spoofed Witch hex"
        );
        assert_eq!(value(&fighter, "class_feature.apg.witch.hex_save_dc"), None);
    }

    /// The familiar's master benefit grounds only on an explicit species
    /// pick, and only for a class that actually gets a familiar.
    #[test]
    fn the_familiar_benefit_needs_both_a_familiar_class_and_an_explicit_pick() {
        let mut witch = character("class:witch", 5);
        assert_eq!(
            value(&witch, "class_feature.familiar.master_hit_point_bonus"),
            None,
            "no familiar is seeded"
        );
        witch.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::FAMILIAR_CHOICE_ID.to_owned(),
            selection_id: super::FAMILIAR_TOAD_SELECTION.to_owned(),
        });
        assert_eq!(value(&witch, "class_feature.familiar.master_hit_point_bonus"), Some(3));

        // Same pick on a class with no familiar grounds nothing.
        let mut fighter = character("class:fighter", 5);
        fighter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::FAMILIAR_CHOICE_ID.to_owned(),
            selection_id: super::FAMILIAR_TOAD_SELECTION.to_owned(),
        });
        assert_eq!(value(&fighter, "class_feature.familiar.master_hit_point_bonus"), None);
    }

    /// One implementation closes the familiar slot for BOTH classes --
    /// the corpus routes Shaman's Spirit Animal through the same
    /// `Standard Familiar List` as Witch's Familiar (corpus-first, per
    /// the lead's ruling on the RAW-vs-corpus divergence).
    #[test]
    fn shaman_and_witch_share_one_familiar_implementation() {
        for class_id in ["class:witch", "class:shaman"] {
            let mut input = character(class_id, 5);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: super::FAMILIAR_CHOICE_ID.to_owned(),
                selection_id: super::FAMILIAR_TOAD_SELECTION.to_owned(),
            });
            assert_eq!(
                value(&input, "class_feature.familiar.master_hit_point_bonus"),
                Some(3),
                "{class_id} must get the same familiar benefit"
            );
            assert_eq!(super::character_familiar_hp_bonus(&input), 3, "{class_id} HP bonus");
        }
    }

    /// Arcanist's own Familiar Exploit (`KEY:Arcanist Exploit ~ Familiar`,
    /// task #56) carries the identical `BONUS:VAR|FamiliarMasterLVL|
    /// ArcanistLVL` token as Witch's and Shaman's own familiar grants --
    /// no Arcane Reservoir cost, no PRE gate, available at 1st level -- so
    /// it dispatches into the same shared, class-agnostic
    /// `ground_familiar_master_benefit` rather than a new mechanism.
    #[test]
    fn arcanist_grounds_the_shared_familiar_benefit_too() {
        let mut arcanist = character("class:arcanist", 5);
        assert_eq!(
            value(&arcanist, "class_feature.familiar.master_hit_point_bonus"),
            None,
            "no familiar is seeded"
        );
        arcanist.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::FAMILIAR_CHOICE_ID.to_owned(),
            selection_id: super::FAMILIAR_TOAD_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&arcanist, "class_feature.familiar.master_hit_point_bonus"),
            Some(3),
            "Arcanist must get the same familiar benefit as Witch/Shaman"
        );
        assert_eq!(super::character_familiar_hp_bonus(&arcanist), 3, "Arcanist HP bonus");
    }

    /// Guard for the one shared-mechanism counting hazard this codebase
    /// actually has (added 2026-07-27 after the sweep that followed
    /// `66b8aae9`).
    ///
    /// The criterion the sweep produced: a `named_features_wired`
    /// asymmetry is only possible when ONE feature-grounding helper is
    /// shared by TWO OR MORE classes that BOTH have coverage rows -- i.e.
    /// both APG/ACG, since CRB has no coverage matrix at all. Exactly one
    /// helper in the codebase meets that: `ground_familiar_master_benefit`,
    /// shared by Witch, Shaman, and (task #56) Arcanist. It was genuinely
    /// wrong for a while (Shaman credited the slot, Witch did not) and
    /// stayed invisible because nothing compares the rows.
    ///
    /// Arcanist joined the set 2026-07-28 (task #56): `KEY:Arcanist
    /// Exploit ~ Familiar` carries the identical `BONUS:VAR|
    /// FamiliarMasterLVL|ArcanistLVL` token as Witch's/Shaman's own
    /// familiar grants, so it dispatches into the same shared,
    /// class-agnostic helper rather than a new mechanism -- an
    /// eligibility-check extension, not a new pillar.
    ///
    /// So this enumerates every APG and ACG class rather than checking
    /// the known ones: if a further class ever picks up the shared
    /// `Standard Familiar List` -- a Spirit Summoner archetype, say --
    /// it lands here as a failure, forcing a deliberate decision about
    /// its own count instead of silently landing credited for one row
    /// and not another.
    #[test]
    fn every_class_grounding_the_shared_familiar_benefit_credits_it_in_its_coverage_row() {
        use crate::rules_core::rules_tables::{acg, apg};

        let mut grounding: Vec<String> = Vec::new();
        for id in apg::ApgClassId::ALL {
            let class_id = format!("class:{}", id.name());
            if grounds_familiar_benefit(&class_id) {
                grounding.push(class_id);
            }
        }
        for id in acg::AcgClassId::ALL {
            let class_id = format!("class:{}", id.name());
            if grounds_familiar_benefit(&class_id) {
                grounding.push(class_id);
            }
        }
        grounding.sort();

        assert_eq!(
            grounding,
            vec![
                "class:arcanist".to_string(),
                "class:shaman".to_string(),
                "class:witch".to_string(),
            ],
            "a class started (or stopped) grounding the shared familiar benefit. Every class \
             in this list must credit the familiar slot in its own named_features_wired entry \
             -- decide that deliberately, then update this assertion"
        );

        // All three credit it: Witch = Hex slot + Familiar, Shaman =
        // Spirit slot + Spirit Animal, Arcanist = Arcane Reservoir +
        // Spells Prepared + Familiar. Equal counts across Witch/Shaman
        // are a coincidence of both having exactly one other slot, not a
        // rule -- what matters is that none of the three is missing the
        // familiar.
        assert_eq!(
            apg::class_coverage(apg::ApgClassId::Witch).named_features_wired,
            2,
            "Witch must credit Hex slot + Familiar"
        );
        assert_eq!(
            acg::class_coverage(acg::AcgClassId::Arcanist).named_features_wired,
            3,
            "Arcanist must credit Arcane Reservoir + Spells Prepared + Familiar"
        );
        assert_eq!(
            acg::class_coverage(acg::AcgClassId::Shaman).named_features_wired,
            2,
            "Shaman must credit Spirit slot + Spirit Animal"
        );
    }

    /// Whether a single-class character of `class_id` grounds the shared
    /// familiar master benefit when a familiar is selected.
    fn grounds_familiar_benefit(class_id: &str) -> bool {
        // Level 1 deliberately: the familiar master benefit is a flat
        // level-independent magnitude, and level 1 is the only level
        // every class's own bounded slices agree is in scope (Hunter's
        // and Cavalier's companion/mount paths assert on higher levels).
        let mut input = character(class_id, 1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::FAMILIAR_CHOICE_ID.to_owned(),
            selection_id: super::FAMILIAR_TOAD_SELECTION.to_owned(),
        });
        value(&input, "class_feature.familiar.master_hit_point_bonus").is_some()
    }

    /// The bonus reaching a real max-HP total is the whole reason Toad
    /// was picked as canonical, so the consumer-facing helper is
    /// differenced directly rather than trusted via the record.
    #[test]
    fn the_consumer_helper_reports_zero_without_a_familiar_and_three_with_one() {
        let bare = character("class:witch", 5);
        assert_eq!(super::character_familiar_hp_bonus(&bare), 0);

        let mut bonded = bare.clone();
        bonded.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::FAMILIAR_CHOICE_ID.to_owned(),
            selection_id: super::FAMILIAR_TOAD_SELECTION.to_owned(),
        });
        assert_eq!(super::character_familiar_hp_bonus(&bonded) - super::character_familiar_hp_bonus(&bare), 3);
    }

    /// Slayer's talent count: `SlayerLVL/2` -- one at 2nd, ten by 20th.
    ///
    /// Ten `-1` deductions against this pool exist in the corpus, each
    /// gated on a `Slayer_CF_TalentN` flag whose only setters are Slayer
    /// ARCHETYPE `.MOD` records (Bounty Hunter, Cleaner, Cutthroat, and
    /// others). This repo ingests only the base `slayer.json`, so all
    /// ten are provably vacuous -- the same check that cleared Brawler's
    /// seven and Cavalier's three.
    #[test]
    fn slayer_talent_count_matches_the_corpus_and_ignores_archetype_deductions() {
        for (level, want) in [(1u8, 0i16), (2, 1), (3, 1), (4, 2), (10, 5), (19, 9), (20, 10)] {
            assert_eq!(super::slayer_talent_count(level), want, "level {level}");
        }
        assert_eq!(
            value(&character(SLAYER_CLASS_ID, 1), "class_feature.acg.slayer.talent_count"),
            None,
            "a level-1 Slayer has no talents yet"
        );
        assert_eq!(
            value(&character(SLAYER_CLASS_ID, 6), "class_feature.acg.slayer.talent_count"),
            Some(3)
        );
    }

    /// Foil Scrutiny is the one canonical talent grounded, narrowed the
    /// same way Order of the Sword and Animal Focus's Bull were. It
    /// requires an explicit recorded pick: a talent chooser's entire
    /// value IS which talent was taken, so seeding one would assert a
    /// specific checkable falsehood, per the ratified Skill Focus line.
    #[test]
    fn foil_scrutiny_grounds_only_when_explicitly_chosen() {
        let bare = character(SLAYER_CLASS_ID, 6);
        assert_eq!(
            value(&bare, "class_feature.acg.slayer.talent.foil_scrutiny_bonus"),
            None,
            "no talent is seeded"
        );

        let mut chosen = bare.clone();
        chosen.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: super::SLAYER_TALENT_FOIL_SCRUTINY_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&chosen, "class_feature.acg.slayer.talent.foil_scrutiny_bonus"),
            Some(2)
        );
    }

    /// A talent cannot be held before any talent exists to spend.
    #[test]
    fn a_talent_choice_grounds_nothing_before_the_first_talent_is_gained() {
        let mut too_early = character(SLAYER_CLASS_ID, 1);
        too_early.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: super::SLAYER_TALENT_FOIL_SCRUTINY_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&too_early, "class_feature.acg.slayer.talent.foil_scrutiny_bonus"),
            None,
            "a level-1 Slayer has no talent slot to have spent"
        );
    }

    /// SD-32 T12 Epic 8 cycle 3: the generic pool-choice resolver, applied
    /// to Slayer Talent, closes a flat-constant talent end-to-end. `Slayer
    /// Talent ~ Deadly Range Output` (real corpus record) carries only
    /// `BONUS:VAR|DeadlyRangeDistance|30` -- a self-contained constant
    /// needing no class-level or ability-modifier binding at all.
    #[test]
    fn slayer_deadly_range_output_talent_resolves_generically_as_a_flat_constant() {
        let mut input = character(SLAYER_CLASS_ID, 6);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: "talent:deadly_range_output".to_owned(),
        });
        assert_eq!(
            value(&input, "class_feature.acg.slayer.talent.generic.deadly_range_output.deadlyrangedistance"),
            Some(30)
        );
    }

    /// SD-32 T12 Epic 8 cycle 4: `Slayer Talent ~ Assassinate`'s DC formula
    /// (`10+(SlayerAssassinateLVL/2)+INT`, chained through
    /// `SlayerAssassinateLVL|SlayerTalentLVL`) previously could not resolve
    /// because the header record that defines `SlayerTalentLVL`
    /// (`"Slayer ~ Slayer Talents"`, plural) did not byte-match the
    /// `"Slayer Talent"` singular member-key prefix this call site passes
    /// as `pool_group` -- the exact `Spirit`-shaped name mismatch cycle 2
    /// flagged and cycle 3 confirmed general (`decisions.md §17a`).
    /// `pool_header_record_by_normalized_suffix` (this cycle) now finds the
    /// header by a singular/plural-normalized suffix match, so the chain
    /// binds: at Slayer level 6, `SlayerTalentLVL` = `SlayerLVL` = 6,
    /// `SlayerAssassinateLVL` = 6, DC = `10 + (6/2) + 0` (no INT bonus on
    /// this fixture) = 13 -- a real value derived from the corpus formula
    /// chain, not a guess.
    #[test]
    fn slayer_assassinate_dc_resolves_generically_once_the_header_binds_slayertalentlvl() {
        let mut input = character(SLAYER_CLASS_ID, 6);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: "talent:assassinate".to_owned(),
        });
        assert_eq!(
            value(&input, "class_feature.acg.slayer.talent.generic.assassinate.slayerassassinatedc"),
            Some(13),
            "10 + (SlayerTalentLVL=6 / 2) + INT(0) = 13, derived from the real corpus chain"
        );
    }

    /// The same header-suffix fix closes `Slowing Strike` too -- same
    /// `SlayerXLVL|SlayerTalentLVL` -> `10+(X/2)+INT` shape as Assassinate,
    /// a second independent record proving this is a real class of members
    /// closing, not a one-off.
    #[test]
    fn slayer_slowing_strike_dc_resolves_generically_once_the_header_binds_slayertalentlvl() {
        let mut input = character(SLAYER_CLASS_ID, 6);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: "talent:slowing_strike".to_owned(),
        });
        assert_eq!(
            value(&input, "class_feature.acg.slayer.talent.generic.slowing_strike.slayerslowingstrikedc"),
            Some(13),
            "10 + (SlayerTalentLVL=6 / 2) + INT(0) = 13, same shape as Assassinate"
        );
    }

    /// `Hard to Fool`'s formula (`SlayerTalentLVL/5+1`) references
    /// `SlayerTalentLVL` DIRECTLY (no intermediate per-talent LVL variable
    /// the way Assassinate/Slowing Strike have) -- proves the header-suffix
    /// fix also closes a record with no intermediate chain hop.
    #[test]
    fn slayer_hard_to_fool_times_resolves_generically_once_the_header_binds_slayertalentlvl() {
        let mut input = character(SLAYER_CLASS_ID, 6);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: "talent:hard_to_fool".to_owned(),
        });
        assert_eq!(
            value(&input, "class_feature.acg.slayer.talent.generic.hard_to_fool.slayerhardtofooltimes"),
            Some(2),
            "SlayerTalentLVL=6 / 5 + 1 = 2 (floor division), derived from the real corpus chain"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 20 correction (`§17a`): this test originally pinned
    /// `resolve_pool_member_sole_magnitude`'s single-value "more than one terminal -> ground
    /// nothing" refusal, at a time when that was the ONLY multi-terminal capability this
    /// codebase had. Cycle 20 built the real capability the refusal was a placeholder for:
    /// `push_generic_pool_choice_magnitude` now resolves through `resolve_pool_member_all_
    /// magnitudes`, which reports EVERY independent terminal a record carries rather than
    /// refusing the whole record. `Slayer Talent ~ Combat Style I` carries real, independent,
    /// unconditional, corpus-verified constant-1 targets -- re-derived directly against the
    /// real record rather than assumed: `CombatStyleLVL` plus SEVEN
    /// `RangerCombatStyle<Option>Allowed` flags (Archery, Crossbow, MountedCombat,
    /// NaturalWeapon, TwoHandedWeapon, TwoWeapon, WeaponAndShield), none of which references any
    /// other, all of them now correctly grounding at their real value (1), restating the same
    /// never-guess safety property for the real capability rather than a blanket refusal.
    #[test]
    fn slayer_combat_style_i_resolves_every_independent_terminal_not_a_guess() {
        let mut input = character(SLAYER_CLASS_ID, 6);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: "talent:combat_style_i".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let mut combat_style_i_values: Vec<(String, i16)> = receipt
            .computation
            .explanations
            .iter()
            .filter(|e| e.id.contains("combat_style_i"))
            .map(|e| (e.id.clone(), e.value))
            .collect();
        combat_style_i_values.sort();
        let expected_targets = [
            "combatstylelvl",
            "rangercombatstylearcheryallowed",
            "rangercombatstylecrossbowallowed",
            "rangercombatstylemountedcombatallowed",
            "rangercombatstylenaturalweaponallowed",
            "rangercombatstyletwohandedweaponallowed",
            "rangercombatstyletwoweaponallowed",
            "rangercombatstyleweaponandshieldallowed",
        ];
        let mut expected: Vec<(String, i16)> = expected_targets
            .iter()
            .map(|target| {
                (
                    format!("class_feature.acg.slayer.talent.generic.combat_style_i.{target}"),
                    1,
                )
            })
            .collect();
        expected.sort();
        assert_eq!(
            combat_style_i_values, expected,
            "every one of Combat Style I's independent terminals must ground, each at its real \
             corpus-verified constant value (1), neither guessed nor dropped"
        );
    }

    /// An invented Slayer Talent selection never grounds a generic
    /// magnitude (mirrors the Rage Power / Discovery precedent).
    #[test]
    fn an_invented_slayer_talent_selection_never_grounds_a_generic_magnitude() {
        let mut input = character(SLAYER_CLASS_ID, 6);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: "talent:not_a_real_talent".to_owned(),
        });
        assert!(build_pilot_headless_receipt(&input)
            .computation
            .explanations
            .iter()
            .all(|e| !e.id.starts_with("class_feature.acg.slayer.talent.generic.")));
    }

    /// Below the pool's own grant level (talents start at 2nd), a
    /// recorded selection grounds no generic magnitude either -- mirrors
    /// Alchemist Discovery's own level-gate proof.
    #[test]
    fn slayer_generic_talent_resolver_stays_silent_below_the_talent_grant_level() {
        let mut input = character(SLAYER_CLASS_ID, 1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: super::SLAYER_TALENT_CHOICE_ID.to_owned(),
            selection_id: "talent:deadly_range_output".to_owned(),
        });
        assert_eq!(
            value(&input, "class_feature.acg.slayer.talent.generic.deadly_range_output.deadlyrangedistance"),
            None
        );
    }

    /// Studied Combat's insight bonus is `InvestigatorLVL/2`, and its
    /// duration `max(1, INT)` rounds grounds as a fact but is explicitly
    /// UNENFORCED -- this engine has no round clock.
    #[test]
    fn investigator_studied_combat_grounds_its_bonus_and_an_unenforced_duration() {
        for (level, want) in [(1u8, 0i16), (2, 1), (7, 3), (20, 10)] {
            assert_eq!(super::investigator_studied_combat_bonus(level), want, "level {level}");
        }
        // Fixture INT 10 -> modifier 0 -> duration max(1, 0) = 1.
        assert_eq!(super::investigator_studied_combat_duration(0), 1);
        assert_eq!(super::investigator_studied_combat_duration(4), 4);

        let input = character(INVESTIGATOR_CLASS_ID, 6);
        assert_eq!(value(&input, "class_feature.acg.investigator.studied_combat_bonus"), Some(3));
        let duration = build_pilot_headless_receipt(&input)
            .computation
            .explanations
            .into_iter()
            .find(|e| e.id == "class_feature.acg.investigator.studied_combat_duration")
            .expect("duration must ground");
        assert!(
            duration.detail.to_lowercase().contains("not enforced"),
            "the duration must be named as unenforced: {duration:?}"
        );
    }

    /// Studied Strike: `min(9, (level-2)/2)` d6, genuinely absent below
    /// 4th and reaching 9d6 at 20th.
    ///
    /// Note the corpus clamp is NOT meaningfully covered here and cannot
    /// be: `(20-2)/2` is already exactly 9 and Rust's truncating
    /// division keeps the low end at 0, so neither bound ever binds.
    /// Asserting the reachable values is the most this test can honestly
    /// do -- the same limit as Brawler's Flurry cap.
    #[test]
    fn investigator_studied_strike_dice_cap_at_nine_and_start_at_fourth() {
        for level in 1..4u8 {
            assert_eq!(super::investigator_studied_strike_dice(level), 0, "level {level}");
        }
        for (level, want) in [(4u8, 1i16), (6, 2), (12, 5), (19, 8), (20, 9)] {
            assert_eq!(super::investigator_studied_strike_dice(level), want, "level {level}");
        }
        assert_eq!(value(&character(INVESTIGATOR_CLASS_ID, 1), "class_feature.acg.investigator.studied_strike_dice"), None);
        assert_eq!(value(&character(INVESTIGATOR_CLASS_ID, 6), "class_feature.acg.investigator.studied_strike_dice"), Some(2));
    }

    /// Studied Defense is a mechanism-B explicit choice: it redirects
    /// the SAME magnitude from attack rolls to AC. One magnitude, two
    /// destinations, player's pick -- never a silently seeded default.
    #[test]
    fn studied_defense_redirects_the_bonus_only_when_explicitly_chosen() {
        let without = character(INVESTIGATOR_CLASS_ID, 10);
        assert_eq!(
            value(&without, "class_feature.acg.investigator.studied_defense_ac_bonus"),
            None,
            "nothing is seeded without an explicit choice"
        );

        let mut with = without.clone();
        with.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INVESTIGATOR_STUDIED_DEFENSE_CHOICE_ID.to_owned(),
            selection_id: INVESTIGATOR_STUDIED_DEFENSE_SELECTION.to_owned(),
        });
        // Level 10 -> bonus 5, redirected to AC.
        assert_eq!(value(&with, "class_feature.acg.investigator.studied_defense_ac_bonus"), Some(5));
    }

    /// The talent gate is real: Studied Defense requires
    /// `InvestigatorTalentLVL >= 9`, and that variable IS properly set
    /// from `InvestigatorLVL` (unlike Swashbuckler's deed gate, which is
    /// never set -- checked tree-wide rather than assumed by analogy).
    #[test]
    fn studied_defense_respects_its_ninth_level_talent_gate() {
        let mut early = character(INVESTIGATOR_CLASS_ID, 8);
        early.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INVESTIGATOR_STUDIED_DEFENSE_CHOICE_ID.to_owned(),
            selection_id: INVESTIGATOR_STUDIED_DEFENSE_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&early, "class_feature.acg.investigator.studied_defense_ac_bonus"),
            None,
            "the talent is not available below investigator level 9"
        );
    }

    /// Cavalier's Challenge grants `+CavalierLVL` damage against the
    /// challenge target, and it applies only while actually challenging.
    #[test]
    fn cavalier_challenge_damage_grounds_only_while_challenging() {
        let idle = character(CAVALIER_CLASS_ID, 1);
        assert_eq!(value(&idle, "class_feature.apg.cavalier.challenge_damage_bonus"), None);

        let mut challenging = idle.clone();
        challenging.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: CAVALIER_CHALLENGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });
        assert_eq!(
            value(&challenging, "class_feature.apg.cavalier.challenge_damage_bonus"),
            Some(1)
        );
    }

    /// The two Challenge scopes are INVERSE and must not be conflated:
    /// the damage bonus applies ONLY against the challenge target, while
    /// the -2 AC penalty applies against everyone EXCEPT that target.
    #[test]
    fn the_two_challenge_scopes_are_described_as_inverses_not_conflated() {
        let mut challenging = character(CAVALIER_CLASS_ID, 1);
        challenging.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: CAVALIER_CHALLENGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });
        let receipt = build_pilot_headless_receipt(&challenging);
        let find = |id: &str| {
            receipt.computation.explanations.iter().find(|e| e.id == id).cloned()
        };
        let damage = find("class_feature.apg.cavalier.challenge_damage_bonus").expect("damage");
        let penalty =
            find("class_feature.apg.cavalier.challenge_armor_class_penalty").expect("penalty");
        assert!(
            damage.detail.to_lowercase().contains("against the target"),
            "damage is scoped TO the target: {damage:?}"
        );
        assert!(
            penalty.detail.to_lowercase().contains("except against"),
            "the AC penalty is scoped to everyone EXCEPT the target: {penalty:?}"
        );
    }

    /// SD31-E4-F1-001: Slayer's Weapon and Armor Proficiency, base case --
    /// no archetype selected, so the base ACG progression grounds with the
    /// real corpus DESC text and value 0 (zero-magnitude, grant-only).
    #[test]
    fn slayer_weapon_and_armor_proficiency_grounds_the_base_grant_with_no_archetype() {
        let base = character(SLAYER_CLASS_ID, 1);
        let receipt = build_pilot_headless_receipt(&base);
        let explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.slayer.weapon_and_armor_proficiency")
            .expect("the base grant must ground for a Slayer with no archetype selected");
        assert_eq!(explanation.value, 0);
        assert!(
            explanation.detail.contains("light armor, medium armor, and shields"),
            "must quote the real base corpus DESC: {explanation:?}"
        );
        assert!(
            !explanation.detail.to_lowercase().contains("superseded"),
            "no archetype is selected, so nothing is superseded: {explanation:?}"
        );
    }

    /// The supersession branch, Bounty Hunter shape: Bounty Hunter's own
    /// FACT-set names the SPLIT WeaponProficiencies+ArmorProficiencies
    /// pair, and its own "~ Weapon and Armor Proficiency" sub-feature text
    /// (naming the aklys/bolas/dan bong/lasso/net additions) replaces the
    /// base grant's text entirely.
    #[test]
    fn slayer_weapon_and_armor_proficiency_is_superseded_by_bounty_hunter() {
        let mut input = character(SLAYER_CLASS_ID, 1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Slayer Archetype ~ Bounty Hunter".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.slayer.weapon_and_armor_proficiency")
            .expect("the record must still ground, with superseded text");
        assert_eq!(explanation.value, 0);
        assert!(
            explanation.detail.contains("Bounty Hunter"),
            "must name the superseding archetype: {explanation:?}"
        );
        assert!(
            explanation.detail.contains("aklys"),
            "must quote Bounty Hunter's OWN real corpus text, not the base grant's: \
             {explanation:?}"
        );
        assert!(
            !explanation.detail.contains("light armor, medium armor, and shields (except"),
            "the base grant's own text must NOT appear once superseded: {explanation:?}"
        );
    }

    /// The supersession branch, Stygian Slayer shape: this archetype's own
    /// PREMULT clause names the GENERIC `Proficiencies` fact rather than
    /// the split pair -- a real, verified corpus authoring inconsistency
    /// between Slayer's own three archetypes, and proof the primitive
    /// checks all three named slot ids, not only the split pair.
    #[test]
    fn slayer_weapon_and_armor_proficiency_is_superseded_by_stygian_slayer_via_the_generic_slot() {
        let mut input = character(SLAYER_CLASS_ID, 1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Slayer Archetype ~ Stygian Slayer".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);
        let explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.slayer.weapon_and_armor_proficiency")
            .expect("the record must still ground, with superseded text");
        assert!(explanation.detail.contains("Stygian Slayer"));
        assert!(
            explanation.detail.contains("not with medium armor, heavy armor"),
            "must quote Stygian Slayer's own real corpus text: {explanation:?}"
        );
    }

    /// Nothing leaks onto a non-Slayer: a Fighter must never emit the
    /// Slayer-namespaced explanation id at all.
    #[test]
    fn weapon_and_armor_proficiency_does_not_ground_for_a_non_slayer() {
        let fighter = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        assert!(
            !build_pilot_headless_receipt(&fighter)
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.slayer.weapon_and_armor_proficiency"),
            "a Fighter must not ground Slayer's Weapon and Armor Proficiency"
        );
    }
}

/// Task #58 (v0.6 alpha swarm): the Resiliency rogue talent, grounded for
/// BOTH classes that can take it -- Rogue's own copy (`cr_abilities_class
/// .lst`, `KEY:Rogue Talent ~ Resiliency`) and Investigator's separate
/// copy drawn from its own explicit 40-record `KEY:Investigator ~ Rogue
/// Talent ~ *` whitelist (`acg_abilities_class.lst`). Two distinct corpus
/// records, two distinct level variables (`RogueTalentLVL` vs
/// `InvestigatorLVL`), so two distinct grounding paths, not a shared
/// formula.
#[cfg(test)]
mod resiliency_talent_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, FIGHTER_CLASS_ID,
        INVESTIGATOR_CLASS_ID, INVESTIGATOR_TALENT_CHOICE_ID, RESILIENCY_TALENT_SELECTION,
        ROGUE_CLASS_ID, ROGUE_TALENT_CHOICE_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .into_iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// Nothing is seeded without an explicit choice -- Resiliency is a
    /// pick among many rogue talents, never a silent default.
    #[test]
    fn rogue_resiliency_grounds_only_when_explicitly_selected() {
        let without = character(ROGUE_CLASS_ID, 10);
        assert_eq!(
            value(&without, "class_feature.rogue.resiliency_temp_hp"),
            None,
            "nothing is seeded without an explicit talent choice"
        );

        let mut with = without.clone();
        with.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ROGUE_TALENT_CHOICE_ID.to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&with, "class_feature.rogue.resiliency_temp_hp"),
            Some(10),
            "level 10 Rogue: ResiliencyHitPoints = RogueTalentLVL = 10"
        );
    }

    /// Recognized across EVERY numbered talent slot, not just the first
    /// -- a rogue could take Resiliency at any of her ten possible slots.
    #[test]
    fn rogue_resiliency_is_recognized_in_any_numbered_talent_slot() {
        let mut input = character(ROGUE_CLASS_ID, 20);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:rogue_talent_10".to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&input, "class_feature.rogue.resiliency_temp_hp"),
            Some(20),
            "slot 10 (level-20 grant) must be recognized just like slot 1"
        );
    }

    /// A slot filled with Resiliency BEFORE that slot's own grant level is
    /// reached must not ground -- a genuinely inconsistent input, not a
    /// valid early activation. Slot 10 grants at level 20; this input is
    /// only level 19.
    #[test]
    fn rogue_resiliency_respects_its_own_slots_grant_level() {
        let mut input = character(ROGUE_CLASS_ID, 19);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:rogue_talent_10".to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&input, "class_feature.rogue.resiliency_temp_hp"),
            None,
            "slot 10 has not been granted yet at level 19"
        );
    }

    /// A non-Rogue never grounds this explanation, even carrying the
    /// exact matching choice/selection pair.
    #[test]
    fn non_rogue_never_grounds_rogue_resiliency() {
        let mut input = character(FIGHTER_CLASS_ID, 10);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ROGUE_TALENT_CHOICE_ID.to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        assert_eq!(value(&input, "class_feature.rogue.resiliency_temp_hp"), None);
    }

    /// Investigator's own separate copy: `ResiliencyHitPoints =
    /// InvestigatorLVL`, gated on its own talent chooser
    /// (`choice:investigator_talent`), starting at investigator level 3
    /// (the class's real first talent-grant level -- 3/5/7/9..., NOT
    /// Rogue's own 2/4/6/8... cadence).
    #[test]
    fn investigator_resiliency_grounds_only_when_explicitly_selected() {
        let without = character(INVESTIGATOR_CLASS_ID, 10);
        assert_eq!(
            value(&without, "class_feature.acg.investigator.resiliency_temp_hp"),
            None,
            "nothing is seeded without an explicit talent choice"
        );

        let mut with = without.clone();
        with.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INVESTIGATOR_TALENT_CHOICE_ID.to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&with, "class_feature.acg.investigator.resiliency_temp_hp"),
            Some(10),
            "level 10 Investigator: ResiliencyHitPoints = InvestigatorLVL = 10, NOT RogueTalentLVL"
        );
    }

    /// The investigator talent chooser is not available before level 3
    /// (Investigator's own first talent-grant level -- distinct from
    /// Rogue's level-2 cadence).
    #[test]
    fn investigator_resiliency_respects_its_own_level_3_talent_gate() {
        let mut input = character(INVESTIGATOR_CLASS_ID, 2);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INVESTIGATOR_TALENT_CHOICE_ID.to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&input, "class_feature.acg.investigator.resiliency_temp_hp"),
            None,
            "investigator talents are not available before level 3"
        );
    }

    /// A non-Investigator never grounds the Investigator-prefixed
    /// explanation, even carrying the exact matching choice/selection.
    #[test]
    fn non_investigator_never_grounds_investigator_resiliency() {
        let mut input = character(FIGHTER_CLASS_ID, 10);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: INVESTIGATOR_TALENT_CHOICE_ID.to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        assert_eq!(value(&input, "class_feature.acg.investigator.resiliency_temp_hp"), None);
    }

    /// The two classes' copies are genuinely independent records: an
    /// Investigator carrying a selection under Rogue's OWN choice id does
    /// not leak into the Investigator-side grounding -- each side only
    /// reads its own `choice_set_id`.
    #[test]
    fn the_two_copies_do_not_cross_react_on_choice_set_id() {
        let mut investigator = character(INVESTIGATOR_CLASS_ID, 10);
        investigator.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ROGUE_TALENT_CHOICE_ID.to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        assert_eq!(
            value(&investigator, "class_feature.acg.investigator.resiliency_temp_hp"),
            None,
            "an Investigator's Rogue-choice-id selection must not ground the Investigator record"
        );
    }
}

