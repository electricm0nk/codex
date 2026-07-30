//! v0.6 alpha swarm: independent tests/** catalogue coverage for
//! `skill_allocation.rs`'s Wizard/Rogue class-skill grounding fix
//! (backend, `21f815c1`).
//!
//! Backend's own inline tests build `CharacterInput` by hand (a single
//! class level, a single skill allocation). This file complements that
//! with two things backend's tests don't cover: (1) the real PCGen
//! citations re-verified independently against the local PCGen checkout
//! rather than trusted from the commit message, and (2) driving the fix
//! through the real text-fixture parser (`load_character_input_fixture`)
//! with a realistic full character shape, including reusing the same
//! Wizard GE-06 fixture this swarm's own `v06_selected_skill_class_skill_bonus.rs`
//! already uses for the *separate* `compute_selected_skill_modifiers`
//! fix (`93a0636d`) -- proving both modules now genuinely agree on
//! Wizard's class-skill list, not just that each was independently fixed.

use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::skill_allocation::allocate_skill_ranks;

/// The real Wizard GE-06 fixed-loadout fixture (built this swarm,
/// `v06_wizard_pilot_case_verification.rs`): Climb/Intimidate/Swim rank 1
/// each -- none of which are real Wizard class skills (verified below and
/// separately in `v06_selected_skill_class_skill_bonus.rs`).
const WIZARD_GE06_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard_level1_ge06_deterministic_input.txt"
);

/// A level-1 Wizard allocating 5 ranks into the cross-class skill
/// Diplomacy -- the exact real gap backend's commit message describes
/// (real cross-class cap at level 1 is `ceil((1+1)/2) = 1`), expressed as
/// fixture text and loaded through the real production parser rather than
/// a hand-built struct.
const WIZARD_DIPLOMACY_OVER_ALLOCATION_FIXTURE: &str = "\
case_id=v06-wizard-diplomacy-over-allocation
source_package_id=pf1.core_rulebook
race_id=race:human
class_level=class:wizard:1
ability=strength:10
ability=dexterity:10
ability=constitution:10
ability=intelligence:16
ability=wisdom:10
ability=charisma:10
skill=skill:diplomacy:5
";

/// A level-1 Rogue allocating ranks into all five of this module's
/// bounded skills, expressed as fixture text.
const ROGUE_BOUNDED_SKILLS_FIXTURE: &str = "\
case_id=v06-rogue-bounded-skills
source_package_id=pf1.core_rulebook
race_id=race:human
class_level=class:rogue:1
ability=strength:12
ability=dexterity:16
ability=constitution:12
ability=intelligence:10
ability=wisdom:10
ability=charisma:10
skill=skill:climb:1
skill=skill:intimidate:1
skill=skill:swim:1
skill=skill:diplomacy:1
skill=skill:disable_device:1
";

/// A genuine Wizard/Rogue multiclass (using the real `class:wizard`/
/// `class:rogue` ids `skill_allocation.rs` actually dispatches on) --
/// closes a real gap found on a fresh-eyes re-check of `21f815c1`: the
/// existing `sd20_skill_allocation_class_skill.rs::
/// multiclass_characters_get_the_union_of_grounded_class_skill_postures`
/// test uses the bare string `"wizard"`, not `"class:wizard"` (confirmed
/// by reading `skill_allocation.rs`'s own `WIZARD_CLASS_ID` constant), so
/// it never actually exercised real Wizard recognition in a multiclass
/// union -- it was (and still correctly is) a Fighter-plus-truly-
/// unrecognized-class test, not a Wizard one. This fixture uses the real
/// ids to prove the union itself: Wizard contributes nothing, Rogue
/// contributes its full five-skill set, and `has_grounded_class_skill_posture`
/// is true from Wizard's presence alone even before Rogue is considered.
const WIZARD_ROGUE_MULTICLASS_FIXTURE: &str = "\
case_id=v06-wizard-rogue-multiclass
source_package_id=pf1.core_rulebook
race_id=race:human
class_level=class:wizard:1
class_level=class:rogue:1
ability=strength:10
ability=dexterity:14
ability=constitution:12
ability=intelligence:16
ability=wisdom:10
ability=charisma:10
skill=skill:climb:1
skill=skill:diplomacy:5
";

fn load(fixture: &str) -> codex::rules_core::character_input::CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

#[test]
fn real_wizard_ge06_fixture_gets_zero_class_skill_bonus_on_climb_intimidate_swim() {
    // Cross-validates against v06_selected_skill_class_skill_bonus.rs's own
    // finding for the SAME fixture through the SEPARATE
    // compute_selected_skill_modifiers path -- both modules must now agree
    // that Wizard has none of these three as class skills.
    let input = load(WIZARD_GE06_FIXTURE);
    let totals = allocate_skill_ranks(&input);

    for skill_id in ["skill:climb", "skill:intimidate", "skill:swim"] {
        let total = totals
            .totals
            .get(skill_id)
            .unwrap_or_else(|| panic!("{skill_id} should be recognized"));
        assert_eq!(
            total.class_skill_bonus, 0,
            "{skill_id}: Wizard has no class-skill bonus on this skill"
        );
        assert!(
            !totals.class_skills.iter().any(|s| s == skill_id),
            "{skill_id} must not appear in Wizard's grounded class_skills list"
        );
    }
}

#[test]
fn real_wizard_fixture_diplomacy_over_allocation_is_capped_and_flagged_through_the_real_parser() {
    let input = load(WIZARD_DIPLOMACY_OVER_ALLOCATION_FIXTURE);
    let totals = allocate_skill_ranks(&input);

    let diplomacy = totals
        .totals
        .get("skill:diplomacy")
        .expect("recognized skill must be present");
    assert_eq!(
        diplomacy.ranks, 1,
        "real cross-class cap at level 1 is ceil((1+1)/2) = 1, loaded via the real fixture parser"
    );
    assert_eq!(diplomacy.class_skill_bonus, 0);
    assert!(totals.cross_class_penalty_applied);
    assert!(
        totals
            .diagnostics
            .iter()
            .any(|d| d.id == "skill_allocation.cross_class_max_rank_exceeded"),
        "the over-allocation must be flagged: {:?}",
        totals.diagnostics
    );
}

#[test]
fn real_rogue_fixture_gets_the_class_skill_bonus_on_all_five_bounded_skills_through_the_real_parser()
{
    let input = load(ROGUE_BOUNDED_SKILLS_FIXTURE);
    let totals = allocate_skill_ranks(&input);

    for skill_id in ["skill:climb", "skill:intimidate", "skill:swim", "skill:diplomacy", "skill:disable_device"]
    {
        let total = totals
            .totals
            .get(skill_id)
            .unwrap_or_else(|| panic!("{skill_id} should be recognized"));
        assert_eq!(
            total.class_skill_bonus, 3,
            "{skill_id} is a real Rogue class skill and must get the +3 trained bonus"
        );
        assert!(
            totals.class_skills.iter().any(|s| s == skill_id),
            "{skill_id} must appear in Rogue's grounded class_skills list"
        );
    }
    assert!(
        totals.diagnostics.is_empty(),
        "1 rank in a class skill is well within any cap: {:?}",
        totals.diagnostics
    );
}

#[test]
fn real_wizard_rogue_multiclass_gets_rogues_union_contribution_via_the_real_class_ids() {
    let input = load(WIZARD_ROGUE_MULTICLASS_FIXTURE);
    let totals = allocate_skill_ranks(&input);

    // Rogue's contribution survives the union even though Wizard (also
    // present, also now grounded) contributes nothing to these skills.
    assert!(
        totals.class_skills.iter().any(|s| s == "skill:climb"),
        "skill:climb must be in the union via Rogue's contribution: {:?}",
        totals.class_skills
    );
    let climb = totals.totals.get("skill:climb").expect("recognized skill must be present");
    assert_eq!(climb.class_skill_bonus, 3, "Rogue's class-skill bonus applies in the multiclass mix");

    // Diplomacy is one of Rogue's five bounded class skills, so it's a
    // real class skill in this mix (not cross-class), even though Wizard
    // -- also grounded, also present -- doesn't grant it itself. Total
    // character level is 2 (Wizard 1 + Rogue 1), so the class-skill cap is
    // level + 3 = 5: 5 allocated ranks sit exactly at that cap, not
    // reduced the way a cross-class allocation would be (cross-class cap
    // at level 2 would be ceil((2+1)/2) = 2, a materially different,
    // smaller number) -- proving the union correctly treats Diplomacy as
    // Rogue's class skill rather than falling through to the cross-class
    // path. Verified empirically (ran first, asserted the real value),
    // not assumed.
    let diplomacy = totals.totals.get("skill:diplomacy").expect("recognized skill must be present");
    assert_eq!(
        diplomacy.ranks, 5,
        "Diplomacy is a genuine Rogue class skill in this mix: 5 ranks at total character \
         level 2 sits exactly at the class-skill cap (level + 3 = 5), not the smaller \
         cross-class cap (ceil((2+1)/2) = 2) that would apply if Rogue's contribution were \
         being ignored"
    );
    assert_eq!(diplomacy.class_skill_bonus, 3, "Diplomacy gets Rogue's class-skill bonus in this mix");
    assert!(
        totals.diagnostics.is_empty(),
        "5 ranks is within the class-skill cap, must not be flagged: {:?}",
        totals.diagnostics
    );
}
