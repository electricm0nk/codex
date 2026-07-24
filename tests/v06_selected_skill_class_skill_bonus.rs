//! v0.6 alpha swarm: catalogue adoption for the class-skill-bonus fix.
//!
//! `compute_selected_skill_modifiers` (`src/rules_core/pilot_compute.rs`)
//! used to apply the Climb/Intimidate/Swim `+3` class-skill bonus
//! unconditionally regardless of the character's actual class — QA found
//! this silently produced a wrong value for Wizard (whose real PF1
//! class-skill list includes none of the three) once the dispatch gate
//! widened from Fighter-only to Fighter/Wizard/Rogue; Rogue happened to be
//! coincidentally correct. Fixed in `93a0636d` via
//! `selected_skill_class_skill_bonus_applies`, which checks whether ANY of
//! the character's classes is Fighter or Rogue (the two of the three whose
//! real class-skill list includes all three skills).
//!
//! Independently re-verified against the real PCGen corpus directly (not
//! transcribed from the fix's own citations): `cr_abilities_class.lst:2835`
//! (`Fighter Core Class Skills`, `CSKILL:Climb|...|Intimidate|...|Swim`),
//! `:2838` (`Rogue Core Class Skills`, same three present), `:2565`
//! (`Wizard ~ Class Skills`, `DESC:The wizard's class skills are Appraise
//! (Int), Craft (Int), Fly (Dex), Knowledge (all) (Int), Linguistics (Int),
//! Profession (Wis), and Spellcraft (Int)` plus a matching
//! `ABILITY:Class Skill|AUTOMATIC|...` list — none of Climb/Intimidate/Swim
//! in either place).
//!
//! Backend's own 4 inline `#[cfg(test)]` tests (same file) cover Fighter
//! solo, Rogue solo, Wizard solo, and a Fighter+Wizard multiclass (bonus via
//! the Fighter side). This file adds the one multiclass angle they didn't:
//! Wizard+Rogue (bonus via the Rogue side, not Fighter), plus asserts the
//! explanation text itself states no bonus applies for the Wizard case
//! (backend's tests only checked the numeric values, not the message).

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{compute_pilot_base_chassis, PilotBaseChassisComputation};

const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

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

fn explanation<'a>(computation: &'a PilotBaseChassisComputation, id: &str) -> &'a codex::rules_core::pilot_compute::ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("expected explanation {id} to be grounded: {computation:?}"))
}

#[test]
fn wizard_rogue_multiclass_gets_the_class_skill_bonus_via_the_rogue_side() {
    // Backend's own multiclass test only exercised Fighter+Wizard (bonus via
    // the Fighter side). This is the other real combination: Wizard alone
    // grants no bonus, but mixing in Rogue (whose real class-skill list does
    // include Climb/Intimidate/Swim) must still grant the bonus, proving the
    // "any class grants it" union rule isn't accidentally Fighter-specific.
    let mut input = with_class("class:wizard");
    let mut rogue_level = input.chosen.class_levels[0].clone();
    rogue_level.class_id = "class:rogue".to_owned();
    rogue_level.level = 1;
    input.chosen.class_levels.push(rogue_level);

    let computation = compute_pilot_base_chassis(&input);

    // Same values as the solo Fighter/Rogue golden path (STR 16+2=18 mod +4,
    // CHA 8 mod -1, level 1, no armor-training ACP reduction): Climb/Swim =
    // rank 1 + STR 4 + class-skill 3 + Chain Shirt ACP -2 = 6; Intimidate =
    // rank 1 + CHA -1 + class-skill 3 = 3.
    assert_eq!(explanation(&computation, "skill.selected_modifier.climb").value, 6);
    assert_eq!(explanation(&computation, "skill.selected_modifier.intimidate").value, 3);
    assert_eq!(explanation(&computation, "skill.selected_modifier.swim").value, 6);
}

#[test]
fn wizard_solo_explanation_text_states_no_class_skill_bonus_applies() {
    // Backend's own inline tests checked only the numeric values for the
    // Wizard case, not the explanation text -- this confirms the message
    // itself honestly states why, not just that the number happens to be
    // lower (matters for anyone reading the explanation record directly,
    // e.g. a future UI surface that renders it).
    let input = with_class("class:wizard");
    let computation = compute_pilot_base_chassis(&input);

    let climb = explanation(&computation, "skill.selected_modifier.climb");
    assert!(
        climb.detail.contains("no class-skill bonus") && climb.detail.contains("not class skills"),
        "the explanation must state no class-skill bonus applies and why: {}",
        climb.detail
    );
    assert_eq!(climb.value, 3, "rank 1 + STR modifier 4 + no class-skill bonus + ACP -2 = 3");
}

#[test]
fn fighter_solo_explanation_text_still_names_the_real_class_skill_bonus() {
    // The positive-path counterpart: for a class that genuinely gets the
    // bonus, the explanation text must still name it as a real, non-zero
    // contribution (not silently drop the class-skill line now that it's
    // conditional).
    let input = with_class("class:fighter");
    let computation = compute_pilot_base_chassis(&input);

    let climb = explanation(&computation, "skill.selected_modifier.climb");
    assert!(
        climb.detail.contains("class-skill bonus (+3)"),
        "the explanation must still name the real +3 class-skill bonus for Fighter: {}",
        climb.detail
    );
}
