#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm: QA found `compute_selected_skill_modifiers` applied
/// the Climb/Intimidate/Swim class-skill `+3` bonus unconditionally
/// regardless of the character's actual class -- silently wrong for
/// Wizard (whose real class-skill list includes none of the three; see
/// `selected_skill_class_skill_bonus_applies`'s own doc comment for the
/// corpus citations). Inline here for the same reason as
/// `multiclass_bab_save_stacking_generalization_tests` and
/// `wizard_spell_save_dc_tests` above: `tests/**` is QA's owned surface
/// for this swarm; this uses the exact same public entry point
/// (`compute_pilot_base_chassis`) so it is trivially portable into QA's
/// catalogue.
#[cfg(test)]
mod selected_skill_class_skill_bonus_tests {
    use super::{
        compute_pilot_base_chassis, ARCANIST_CLASS_ID, BRAWLER_CLASS_ID, FIGHTER_CLASS_ID,
        HUNTER_CLASS_ID, INQUISITOR_CLASS_ID, INVESTIGATOR_CLASS_ID, ROGUE_CLASS_ID,
        SKALD_CLASS_ID, SLAYER_CLASS_ID, SWASHBUCKLER_CLASS_ID, WARPRIEST_CLASS_ID,
        WITCH_CLASS_ID, WIZARD_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// This fixture already carries the exact deterministic selected-skill
    /// posture (Climb/Intimidate/Swim rank 1, Chain Shirt equipped)
    /// `unmet_selected_skill_posture_conditions` requires -- swapping only
    /// the class identity in place isolates the class-skill-bonus logic
    /// from everything else that posture gate checks.
    fn with_class(class_id: &str) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(
            result.diagnostics.is_empty(),
            "fixture should load cleanly: {:?}",
            result.diagnostics
        );
        let mut input = result
            .character_input
            .expect("valid fixture should produce a character input record");
        input.chosen.class_levels[0].class_id = class_id.to_owned();
        input
    }

    fn skill_value(computation: &super::PilotBaseChassisComputation, id: &str) -> i16 {
        computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("expected explanation {id} to be grounded: {computation:?}"))
            .value
    }

    #[test]
    fn fighter_still_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(FIGHTER_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        // Unchanged golden-path values: Fighter's real class-skill list
        // includes Climb, Intimidate, and Swim (cr_abilities_class.lst:2835).
        // Fixture: STR 16 + 2 human bonus = 18 (mod +4), CHA 8 (mod -1),
        // level 1 (no armor-training ACP reduction, Chain Shirt ACP -2).
        // Climb/Swim = rank 1 + STR 4 + class-skill 3 + ACP -2 = 6.
        // Intimidate = rank 1 + CHA -1 + class-skill 3 = 3.
        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }

    #[test]
    fn rogue_still_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(ROGUE_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        // Rogue's real class-skill list also includes all three
        // (cr_abilities_class.lst:2838) -- same values as Fighter, since
        // this fixture's ability scores/equipment/armor-training math is
        // otherwise class-identical for this bounded posture.
        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }

    /// Warpriest's own real class-skill list (`acg_abilities_class.lst`'s
    /// `KEY:Warpriest ~ Class Skills`: Climb, Craft, Diplomacy, Handle
    /// Animal, Heal, Intimidate, Knowledge (Engineering), Knowledge
    /// (Religion), Profession, Ride, Sense Motive, Spellcraft, Survival,
    /// Swim) genuinely includes all three of Climb/Intimidate/Swim (v0.6
    /// alpha swarm, risks item 8, Warpriest full-build closure): the
    /// mirror-image of the Wizard bug this test module's own name
    /// documents (false positive there, false negative here) -- before
    /// this widening, a Warpriest would have silently gotten a false
    /// ZERO bonus on all three despite genuinely earning one.
    #[test]
    fn warpriest_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(WARPRIEST_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }

    /// Slayer's own real class-skill list (`acg_abilities_class.lst`'s
    /// `KEY:Slayer ~ Class Skills`: Acrobatics, Bluff, Climb, Craft,
    /// Disguise, Heal, Intimidate, Knowledge (Dungeoneering/Geography/
    /// Local), Profession, Ride, Sense Motive, Stealth, Survival, Swim)
    /// genuinely includes all three of Climb/Intimidate/Swim (v0.6 alpha
    /// swarm, risks item 8, Slayer full-build closure) -- the THIRD
    /// class needing this exact widening, same shape as Warpriest.
    /// Written first, per the lead's own instruction, to prove the bug
    /// exists before fixing it.
    #[test]
    fn slayer_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(SLAYER_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }

    /// Swashbuckler's own real class-skill list
    /// (`acg_abilities_class.lst`'s `KEY:Swashbuckler ~ Class Skills`:
    /// Acrobatics, Bluff, Climb, Craft, Diplomacy, Escape Artist,
    /// Intimidate, Knowledge (Local/Nobility), Perception, Perform,
    /// Profession, Ride, Sense Motive, Sleight of Hand, Swim) genuinely
    /// includes all three of Climb/Intimidate/Swim (v0.6 alpha swarm,
    /// risks item 8, Swashbuckler full-build closure) -- the FOURTH class
    /// needing this exact widening, same shape as Warpriest/Slayer.
    /// Written first, per the lead's own established instruction, to
    /// prove the bug exists before fixing it.
    #[test]
    fn swashbuckler_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(SWASHBUCKLER_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }

    /// Investigator's own real class-skill list (`acg_abilities_class.lst`'s
    /// `KEY:Investigator ~ Class Skills`: Acrobatics, Appraise, Bluff,
    /// Climb, Craft, Diplomacy, Disable Device, Disguise, Escape Artist,
    /// Heal, Intimidate, Knowledge (all), Linguistics, Perception,
    /// Perform, Profession, Sense Motive, Sleight of Hand, Spellcraft,
    /// Stealth, Use Magic Device) is a genuine 2-of-3 PARTIAL match --
    /// Climb and Intimidate present, Swim absent -- the first partial
    /// match on the whole roster (v0.6 alpha swarm, risks item 8,
    /// Investigator full-build closure). This is exactly why
    /// `selected_skill_class_skill_bonus_applies`'s single scalar had to
    /// split into three independent per-skill functions
    /// (`selected_skill_climb_is_class_skill`/`..._intimidate_.../
    /// ..._swim_...`): no single boolean could represent Climb/Intimidate
    /// true while Swim is false for the same character.
    #[test]
    fn investigator_gets_the_class_skill_bonus_on_climb_and_intimidate_but_not_swim() {
        let input = with_class(INVESTIGATOR_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(
            skill_value(&computation, "skill.selected_modifier.swim"),
            3,
            "Swim is NOT a real Investigator class skill -- must match the no-bonus value, not \
             the class-skill-bonus value"
        );
    }

    /// Witch's own real class-skill list (`apg_abilities_class.lst`'s
    /// `KEY:Witch ~ Class Skills`: Craft, Fly, Heal, Intimidate,
    /// Knowledge (Arcana/History/Nature/Planes), Profession, Spellcraft,
    /// Use Magic Device) is ANOTHER genuine partial match -- Intimidate
    /// present, Climb and Swim both absent (v0.6 alpha swarm, risks item
    /// 8, Witch full-build closure). A different partial shape than
    /// Investigator's own (Climb+Intimidate yes, Swim no), proving the
    /// per-skill split generalizes correctly to more than one partial
    /// pattern, not just Investigator's specific one.
    #[test]
    fn witch_gets_the_class_skill_bonus_on_intimidate_only() {
        let input = with_class(WITCH_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(
            skill_value(&computation, "skill.selected_modifier.climb"),
            3,
            "Climb is NOT a real Witch class skill: {:?}",
            computation
        );
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(
            skill_value(&computation, "skill.selected_modifier.swim"),
            3,
            "Swim is NOT a real Witch class skill: {:?}",
            computation
        );
    }

    /// Brawler's own real class-skill list (`acg_abilities_class.lst`'s
    /// `KEY:Brawler ~ Class Skills`: Acrobatics, Climb, Craft, Escape
    /// Artist, Handle Animal, Intimidate, Knowledge (Dungeoneering/
    /// Local), Perception, Profession, Ride, Sense Motive, Swim)
    /// genuinely includes all three of Climb/Intimidate/Swim (v0.6 alpha
    /// swarm, risks item 8, Brawler deepening, 2026-07-26) -- the SIXTH
    /// class needing this exact widening, same shape as Warpriest/
    /// Slayer/Swashbuckler. Found while deepening Brawler's own closure
    /// for an unrelated reason (Cunning/Strike), not the original task --
    /// written first, per the lead's own established instruction, to
    /// prove the bug exists before fixing it.
    #[test]
    fn brawler_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(BRAWLER_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }

    #[test]
    fn wizard_gets_no_class_skill_bonus_on_any_of_the_three_skills() {
        let input = with_class(WIZARD_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        // The real bug: Wizard's real class-skill list
        // (cr_abilities_class.lst:2565) includes NONE of Climb/Intimidate/
        // Swim, so these must be exactly 3 lower than Fighter/Rogue's
        // values above (no +3 class-skill bonus).
        // Climb/Swim = rank 1 + STR 4 + 0 + ACP -2 = 3.
        // Intimidate = rank 1 + CHA -1 + 0 = 0.
        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 0);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 3);
    }

    /// Arcanist's own class-skill list (`acg_abilities_class.lst`'s
    /// `KEY:Arcanist ~ Class Skills`: Appraise, Craft, Fly, Knowledge
    /// (all), Linguistics, Profession, Spellcraft, Use Magic Device) also
    /// includes NONE of Climb/Intimidate/Swim -- same shape as Wizard, so
    /// this needed no new wiring in `selected_skill_class_skill_bonus_applies`
    /// at all once the chassis gate admitted Arcanist (v0.6 alpha swarm,
    /// risks item 8, Arcanist full-build closure): the existing generic
    /// mechanism already produces the correct answer for a class outside
    /// its own hardcoded Fighter/Rogue bonus-eligibility set.
    #[test]
    fn arcanist_gets_no_class_skill_bonus_on_any_of_the_three_skills() {
        let input = with_class(ARCANIST_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 0);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 3);
    }

    /// PF1's real multiclass rule: the class-skill bonus applies if ANY of
    /// the character's classes grants it. A Fighter/Wizard mix must still
    /// get the bonus (from the Fighter side), proving this isn't a
    /// single-class-only check.
    #[test]
    fn multiclass_fighter_wizard_still_gets_the_bonus_via_the_fighter_side() {
        let mut input = with_class(FIGHTER_CLASS_ID);
        let mut wizard_level = input.chosen.class_levels[0].clone();
        wizard_level.class_id = WIZARD_CLASS_ID.to_owned();
        wizard_level.level = 1;
        input.chosen.class_levels.push(wizard_level);

        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
    }

    /// Corpus gap found and independently re-verified by the lead and a
    /// scout teammate against the raw PCGen corpus (v0.6 alpha swarm):
    /// Inquisitor's real class-skill list (`apg_abilities_class.lst`'s
    /// `KEY:Inquisitor ~ Class Skills`) genuinely includes Climb,
    /// Intimidate, AND Swim, but none of the three
    /// `selected_skill_*_is_class_skill` predicates had an Inquisitor arm
    /// -- silently zero class-skill bonus for a real Inquisitor. Written
    /// first, per the lead's own established instruction, to prove the
    /// bug exists before fixing it. Intimidate's expected value already
    /// carries Inquisitor's OWN unconditional Stern Gaze morale bonus
    /// (`active_inquisitor_stern_gaze_bonus`, +1 at level 1) on top of the
    /// class-skill bonus this fix adds -- the two are independent, both
    /// genuinely apply, and must both show up in the same total: rank 1 +
    /// CHA -1 + class-skill 3 + Stern Gaze 1 = 4.
    #[test]
    fn inquisitor_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(INQUISITOR_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 4);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }

    /// Same corpus gap as Inquisitor above, independently re-verified
    /// against Hunter's real class-skill list (`acg_abilities_class.lst`'s
    /// `KEY:Hunter ~ Class Skills`), which also genuinely includes all
    /// three of Climb/Intimidate/Swim. No other Hunter-specific bonus
    /// touches any of these three totals in this engine (unlike
    /// Inquisitor's Stern Gaze), so the expected values are the plain
    /// Fighter/Rogue shape.
    #[test]
    fn hunter_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(HUNTER_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }

    /// Same corpus gap as Inquisitor/Hunter above, independently
    /// re-verified against Skald's real class-skill list
    /// (`acg_abilities_class.lst`'s `KEY:Skald ~ Class Skills`), which
    /// also genuinely includes all three of Climb/Intimidate/Swim. This
    /// fixture's character is not actively raging/singing, so Raging
    /// Climber/Raging Swimmer's `active_rage_powers_level` term is 0 --
    /// only the class-skill bonus this fix adds is exercised here.
    #[test]
    fn skald_gets_the_class_skill_bonus_on_all_three_skills() {
        let input = with_class(SKALD_CLASS_ID);
        let computation = compute_pilot_base_chassis(&input);

        assert_eq!(skill_value(&computation, "skill.selected_modifier.climb"), 6);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.intimidate"), 3);
        assert_eq!(skill_value(&computation, "skill.selected_modifier.swim"), 6);
    }
}

