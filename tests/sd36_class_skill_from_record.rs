//! SD-36 Epic F3b3 (4): the selected-skill check reads each class's class skills from its
//! CONVERTED record (`class_skill_sheet_rules`), not from a hand-kept class list.
//!
//! F3b found Barbarian printed Climb/Swim without the +3 class-skill bonus although its
//! converted record (`core_rulebook:class_feature:class_skills_barbarian`,
//! `cr_abilities_class.lst:2831`, CSKILL Acrobatics|Climb|TYPE=Craft|Handle Animal|Intimidate|
//! Knowledge (Nature)|Perception|Ride|Survival|Swim) grants all three.
//!
//! Hand-worked (the shared census fixture: Human, Str 16 + 2 = 18 (+4), Cha 8 (-1), 1 rank each
//! in Climb / Intimidate / Swim, chain shirt worn): CRB p.87 a class skill with at least 1 rank
//! gets +3; CRB p.150 Table 6-6 chain shirt armor check penalty -2 (Climb and Swim).
//!   Barbarian 1 Climb      = 1 + 4 + 3 - 2 = 6
//!   Barbarian 1 Intimidate = 1 - 1 + 3     = 3
//!   Barbarian 1 Swim       = 1 + 4 + 3 - 2 = 6

use codex::rules_core::class_census::{census, load_sweep_fixture};
use codex::rules_core::class_seeds::input_for;
use codex::rules_core::pilot_compute::class_skill_sheet_rules::{class_skill_view, ClassSkillAnswer};
use codex::rules_core::pilot_compute::{build_pilot_headless_receipt, PilotHeadlessReceipt};

fn value(receipt: &PilotHeadlessReceipt, id: &str) -> Option<i16> {
    receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
}

fn detail(receipt: &PilotHeadlessReceipt, id: &str) -> Option<String> {
    receipt.computation.explanations.iter().find(|e| e.id == id).map(|e| e.detail.clone())
}

#[test]
fn barbarian_gets_the_class_skill_bonus_its_record_grants() {
    let fixture = load_sweep_fixture().expect("fixture");
    let receipt = build_pilot_headless_receipt(&input_for(&fixture, "barbarian", 1));
    assert_eq!(value(&receipt, "skill.selected_modifier.climb"), Some(6), "{:?}", detail(&receipt, "skill.selected_modifier.climb"));
    assert_eq!(value(&receipt, "skill.selected_modifier.intimidate"), Some(3), "{:?}", detail(&receipt, "skill.selected_modifier.intimidate"));
    assert_eq!(value(&receipt, "skill.selected_modifier.swim"), Some(6), "{:?}", detail(&receipt, "skill.selected_modifier.swim"));
}

/// The scan: every census class at level 1 whose converted record answers Known, and whose
/// single-class receipt prints a selected-skill line. For each of Climb / Intimidate / Swim the
/// line carries the class-skill bonus exactly when the record grants the skill. Printed with its
/// denominator; asserted 0 mismatches.
#[test]
fn every_census_class_prints_the_class_skill_bonus_its_record_grants() {
    let fixture = load_sweep_fixture().expect("fixture");
    let mut checked = 0usize;
    let mut known = 0usize;
    let mut mismatches = Vec::new();
    let classes = census();
    for entry in classes.values() {
        let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
        let ClassSkillAnswer::Known(view) = class_skill_view(slug, 1) else { continue };
        known += 1;
        let receipt = build_pilot_headless_receipt(&input_for(&fixture, slug, 1));
        for skill in ["climb", "intimidate", "swim"] {
            let Some(line) = detail(&receipt, &format!("skill.selected_modifier.{skill}")) else { continue };
            checked += 1;
            let printed = line.contains("class-skill bonus (+3)");
            if printed != view.contains(skill) {
                mismatches.push(format!("{} {skill}: record {} / sheet {}", entry.class_id, view.contains(skill), printed));
            }
        }
    }
    println!(
        "census ids {}; record Known at level 1: {known}; (class, skill) lines checked: {checked}; mismatches: {}",
        classes.len(),
        mismatches.len()
    );
    assert!(checked > 0);
    assert!(mismatches.is_empty(), "{} of {checked} lines disagree with the record: {mismatches:#?}", mismatches.len());
}

/// SD-36 Epic F3c2 (1): the two classes F3b3 left Blocked on
/// `skill.selected_modifier.class_skill_unknown`.
///
/// **Expert** (CRB p.450: "any 10 skills" are class skills). Its converted record carries the
/// pick (`core_rulebook:class:expert#bonus4`, `BONUS:ABILITYPOOL|Expert Class Skills|10`,
/// cr_classes.lst:549) and the chooser (`core_rulebook:class_feature:expert_class_skills`,
/// cr_abilities_class.lst:2735, `CHOOSE:SKILL|ALL`, `CSKILL:LIST` -> `ClassSkillChosen`). The
/// class's ten picks are a choice, so its Path-A canonical default is seeded through
/// `class_seeds` like every other choice: the first ten single skills of the CRB skill list
/// (Acrobatics, Appraise, Bluff, Climb, Diplomacy, Disable Device, Disguise, Escape Artist, Fly,
/// Handle Animal). Hand-worked on the shared census fixture (Human, Str 18 (+4), Cha 8 (-1),
/// 1 rank each, chain shirt ACP -2, CRB p.150):
///   Expert 1 Climb      = 1 + 4 + 3 - 2 = 6   (a class skill: the +3, CRB p.87)
///   Expert 1 Intimidate = 1 - 1         = 0   (not picked)
///   Expert 1 Swim       = 1 + 4 - 2     = 3   (not picked)
///
/// (F3c2 also pinned Psion as still refused by name here. F3c3 moved that pin: the converter now
/// carries Psion's discipline `SUBCLASS:` lines as a class choice, and Psion answers through its
/// canonical discipline -- `psion_class_skills_answer_through_its_canonical_discipline` below.)
#[test]
fn expert_prints_its_canonical_class_skill_picks() {
    let fixture = load_sweep_fixture().expect("fixture");
    let ClassSkillAnswer::Known(view) = class_skill_view("expert", 1) else {
        panic!("expert: {:?}", class_skill_view("expert", 1));
    };
    for skill in ["acrobatics", "appraise", "bluff", "climb", "diplomacy", "disable_device", "disguise", "escape_artist", "fly", "handle_animal"] {
        assert!(view.contains(skill), "{skill}: {view:?}");
    }
    assert_eq!(view.skills.len(), 10, "CRB p.450: ten skills: {view:?}");
    assert!(view.groups.is_empty() && !view.contains("intimidate") && !view.contains("swim"), "{view:?}");
    for level in [1u8, 20] {
        let receipt = build_pilot_headless_receipt(&input_for(&fixture, "expert", level));
        assert_eq!(value(&receipt, "skill.selected_modifier.climb"), Some(6), "{:?}", detail(&receipt, "skill.selected_modifier.climb"));
        assert_eq!(value(&receipt, "skill.selected_modifier.intimidate"), Some(0), "{:?}", detail(&receipt, "skill.selected_modifier.intimidate"));
        assert_eq!(value(&receipt, "skill.selected_modifier.swim"), Some(3), "{:?}", detail(&receipt, "skill.selected_modifier.swim"));
        assert!(
            !receipt.computation.diagnostics.iter().any(|d| d.id == "skill.selected_modifier.class_skill_unknown"),
            "expert {level}: {:?}",
            receipt.computation.diagnostics
        );
    }

}

/// SD-36 Epic F3c3 (1): Psion's class skills answer through its canonical discipline.
///
/// The converter now carries PCGen's `SUBCLASS:` lines as a class choice
/// (`ultimate_psionics:class:psion#subclass`) whose options carry each line's grants. The
/// canonical default is the first option in oracle order (`SubClassApplication.checkForSubClass`
/// offers the class's `SUB_CLASS` list in load order; the oracle marks no default, and Psion's
/// `ALLOWBASECLASS:NO`, `up_classes.lst:216`, makes a pick mandatory): Egoist (`:221`), seeded
/// through `class_seeds` like every other pick. Its class skills (UP p.49 and p.52):
/// Autohypnosis, Craft, Knowledge, Profession, Spellcraft (`:221` `CSKILL`) plus Acrobatics and
/// Heal (`Psychometabolism Class Skills`, `up_abilities_class.lst:409`, granted at level 1 by
/// `:222`). Climb, Intimidate and Swim are none of these.
///
/// Hand-worked on the shared census fixture (Human, Str 18 (+4), Cha 8 (-1), 1 rank each, chain
/// shirt ACP -2, CRB p.150), no class-skill +3 on any of the three (CRB p.87):
///   Psion 1 Climb      = 1 + 4 - 2 = 3
///   Psion 1 Intimidate = 1 - 1     = 0
///   Psion 1 Swim       = 1 + 4 - 2 = 3
#[test]
fn psion_class_skills_answer_through_its_canonical_discipline() {
    let fixture = load_sweep_fixture().expect("fixture");
    let ClassSkillAnswer::Known(view) = class_skill_view("psion", 1) else {
        panic!("psion: {:?}", class_skill_view("psion", 1));
    };
    for skill in ["autohypnosis", "spellcraft", "acrobatics", "heal", "craft_alchemy", "knowledge_psionics", "profession_sailor"] {
        assert!(view.contains(skill), "{skill}: {view:?}");
    }
    for skill in ["climb", "intimidate", "swim", "bluff", "disable_device"] {
        assert!(!view.contains(skill), "{skill} is not an Egoist class skill: {view:?}");
    }
    assert!(view.granted_by.contains("ultimate_psionics:subclass:psion_egoist"), "{view:?}");
    for level in [1u8, 20] {
        let receipt = build_pilot_headless_receipt(&input_for(&fixture, "psion", level));
        assert_eq!(value(&receipt, "skill.selected_modifier.climb"), Some(3), "{:?}", detail(&receipt, "skill.selected_modifier.climb"));
        assert_eq!(value(&receipt, "skill.selected_modifier.intimidate"), Some(0), "{:?}", detail(&receipt, "skill.selected_modifier.intimidate"));
        assert_eq!(value(&receipt, "skill.selected_modifier.swim"), Some(3), "{:?}", detail(&receipt, "skill.selected_modifier.swim"));
        assert!(
            !receipt.computation.diagnostics.iter().any(|d| d.id == "skill.selected_modifier.class_skill_unknown"),
            "psion {level}: {:?}",
            receipt.computation.diagnostics
        );
    }
}

/// SD-36 Epic F3c3 (1): the double-grant scan. A seeded sub-class pick now holds its option rule
/// and everything the option grants. A class whose bespoke chassis module already grounds one of
/// those records (an explanation `rule_for_explanation` joins to the same converted rule) would
/// print it twice unless the join de-duplicates it. Scanned over every census class, at levels 1
/// and 20: the classes whose canonical seeds carry a `#subclass` pick, the rules held only
/// because of that pick, and how many of those the bespoke module's explanations also join to.
/// Printed with its denominators.
#[test]
fn a_seeded_subclass_pick_is_scanned_for_double_grants_against_the_bespoke_join() {
    use codex::rules_core::class_seeds::canonical_seeds_for;
    use codex::rules_core::sheet_line_join::{rule_for_explanation, JoinResult};
    use codex::rules_core::sheet_rule::{held_set, CharacterFacts, HeldSeed};
    use std::collections::BTreeSet;

    let package = codex::rules_core::sheet_rule_package::package().as_ref().expect("package loads");
    let fixture = load_sweep_fixture().expect("fixture");
    let classes = census();
    let mut seeded: Vec<String> = Vec::new();
    let mut via_pick_total = 0usize;
    let mut joined_overlap: Vec<String> = Vec::new();
    for entry in classes.values() {
        let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
        let (choices, _) = canonical_seeds_for(slug, entry.max_level);
        if !choices.iter().any(|c| c.choice_set_id.ends_with("#subclass")) {
            continue;
        }
        seeded.push(slug.to_string());
        for level in [1u8, 20] {
            let input = input_for(&fixture, slug, level);
            let receipt = build_pilot_headless_receipt(&input);
            let seed = HeldSeed::from_character(&input, &receipt.computation);
            let facts = CharacterFacts::from_character(&input, &receipt.computation);
            let mut without = facts.clone();
            without.choices.retain(|choice, _| !choice.ends_with("#subclass"));
            let bare = HeldSeed { class_features: Vec::new(), ..seed.clone() };
            let with_pick: BTreeSet<String> = held_set(package, &bare, &facts).rules.into_keys().collect();
            let without_pick: BTreeSet<String> = held_set(package, &bare, &without).rules.into_keys().collect();
            let via_pick: BTreeSet<String> = with_pick.difference(&without_pick).cloned().collect();
            assert!(!via_pick.is_empty(), "{slug} {level}: the pick holds its option");
            via_pick_total += via_pick.len();
            let joined: BTreeSet<String> = seed
                .class_features
                .iter()
                .filter_map(|(c, e)| match rule_for_explanation(package, c, e) {
                    JoinResult::Matched(id) => Some(id),
                    _ => None,
                })
                .collect();
            for id in via_pick.intersection(&joined) {
                joined_overlap.push(format!("{slug} {level}: {id}"));
            }
        }
    }
    println!(
        "census ids {}; classes seeding a subclass pick: {} {seeded:?}; rules held only through the pick (levels 1 + 20): {via_pick_total}; \
         of those also joined from a bespoke explanation: {} {joined_overlap:?}",
        classes.len(),
        seeded.len(),
        joined_overlap.len()
    );
    // Every class carrying a converted subclass choice, seeded or not: the records its options
    // grant directly (a `Granter::Rule(<option>)` edge) that its bespoke module already joins to
    // at levels 1 and 20 -- what a pick of that option would meet in the join.
    let mut choice_classes: Vec<String> = Vec::new();
    let mut option_edges = 0usize;
    let mut would_meet: Vec<String> = Vec::new();
    for entry in classes.values() {
        let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
        let Some(principal) = package.find("class", slug) else { continue };
        let chooser = format!("{principal}#subclass");
        if package.rule(&chooser).is_none() {
            continue;
        }
        choice_classes.push(slug.to_string());
        let options: Vec<&String> = package
            .rules
            .values()
            .filter(|r| r.granted_by.iter().any(|g| g.by == codex::rules_core::sheet_rule::Granter::Choice(chooser.clone())))
            .map(|r| &r.id)
            .collect();
        let granted: BTreeSet<String> = package
            .rules
            .values()
            .filter(|r| r.granted_by.iter().any(|g| matches!(&g.by, codex::rules_core::sheet_rule::Granter::Rule(o) if options.contains(&o))))
            .map(|r| r.id.clone())
            .collect();
        option_edges += granted.len();
        for level in [1u8, 20] {
            let input = input_for(&fixture, slug, level);
            let receipt = build_pilot_headless_receipt(&input);
            let seed = HeldSeed::from_character(&input, &receipt.computation);
            for (c, e) in &seed.class_features {
                if let JoinResult::Matched(id) = rule_for_explanation(package, c, e)
                    && granted.contains(&id)
                {
                    would_meet.push(format!("{slug} {level}: {e} -> {id}"));
                }
            }
        }
    }
    println!(
        "classes carrying a converted subclass choice: {choice_classes:?}; records their options grant: {option_edges}; \
         of those the bespoke module already joins to: {} {would_meet:?}",
        would_meet.len()
    );
    assert_eq!(seeded, vec!["psion".to_string()], "the one class seeding a subclass pick");
    assert!(joined_overlap.is_empty(), "a rule both the pick and the bespoke module grant: {joined_overlap:#?}");
}
