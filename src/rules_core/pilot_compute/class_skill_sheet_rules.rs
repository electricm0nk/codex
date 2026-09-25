//! A class's class skills, read from the CONVERTED rule package (`data/sheet_rules/`) --
//! SD-36 Epic F3b3. The same walk [`class_proficiency_sheet_rules`] makes for weapon
//! proficiency, collecting `Fact::ClassSkill` / `Fact::ClassSkillGroup` instead.
//!
//! # Mechanism (one rule, no per-class cases)
//!
//! For `(class slug, class level)`: seed [`HeldSeed`] with that one class at that level, run
//! [`held_set`] (the class principal's `Granter::Class` / `Granter::Rule` edges to a fixpoint,
//! the class's Path-A canonical member picks recorded as the proficiency reader records them),
//! and collect every class-skill fact a held rule grants. A gated grant counts only when its gate
//! is decidable from class-level facts ([`gate_is_class_decidable`]) and evaluates true.
//!
//! A class whose class skills ARE a choice (Expert, CRB p.450: any ten) answers with its Path-A
//! canonical picks from `class_seeds` (SD-36 F3c2, [`add_canonical_class_skill_picks`]): one rule
//! over the converted chooser (`offers: Skills` + `ClassSkillChosen(<own id>)`).
//!
//! # Unknown, never "no class skills"
//!
//! Every PF1 class has class skills. A walk that reaches no class-skill grant at all, a class
//! with no converted record, or a package that does not load is [`ClassSkillAnswer::Unknown`]
//! with its reason -- never an empty list (which would print every skill without its +3).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

use super::class_proficiency_sheet_rules::gate_is_class_decidable;
use crate::rules_core::sheet_rule::{
    chooser_offered, held_set, resolve_gated_fact_grant, CharacterFacts, Choice, Effect, EvalContext, Fact, GatedFact, Granter, HeldSeed,
    OptionSet, SheetRulePackage,
};
use crate::rules_core::sheet_rule_package;

/// What the converted record says one class's class skills are at one level.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassSkillView {
    /// Skill ids as the package spells them (`"climb"`, `"knowledge_nature"`).
    pub skills: BTreeSet<String>,
    /// Whole skill families (`"Craft"`, `"Knowledge"`).
    pub groups: BTreeSet<String>,
    /// The rules that granted them (display/provenance only).
    pub granted_by: BTreeSet<String>,
}

impl ClassSkillView {
    /// Whether `skill` (a package skill id, `"climb"`) is a class skill in this view: named
    /// directly, or a member of a granted family (`"knowledge_nature"` under `"Knowledge"`).
    pub fn contains(&self, skill: &str) -> bool {
        self.skills.contains(skill)
            || self.groups.iter().any(|group| {
                let family = group.to_ascii_lowercase();
                skill == family || skill.strip_prefix(&family).is_some_and(|rest| rest.starts_with('_'))
            })
    }
}

/// The reader's answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassSkillAnswer {
    Known(ClassSkillView),
    Unknown { reason: String },
}

/// The class's class skills at `class_level`, from the process-wide converted package.
/// Cached per `(class, level)`.
pub fn class_skill_view(class_slug: &str, class_level: u8) -> ClassSkillAnswer {
    cached_view(class_slug, class_level, &[])
}

/// SD-36 F3c4: the class's class skills for THIS character -- [`class_skill_view`] plus the
/// character's own Path-A picks linked to the converted options they name
/// ([`sheet_rule_package::linked_picks`]; a sorcerer's `bloodline:aquatic` holds the Aquatic
/// pick option, whose record grants Swim). One rule for every class and every pick; a pick
/// whose chooser this class does not hold is ignored here. With no linked pick it is exactly
/// [`class_skill_view`].
pub fn class_skill_view_for(
    input: &crate::rules_core::character_input::CharacterInput,
    class_slug: &str,
    class_level: u8,
) -> ClassSkillAnswer {
    let picks: Vec<(String, String)> =
        sheet_rule_package::linked_picks(input).into_iter().map(|l| (l.chooser, l.option)).collect();
    cached_view(class_slug, class_level, &picks)
}

fn cached_view(class_slug: &str, class_level: u8, picks: &[(String, String)]) -> ClassSkillAnswer {
    type Key = (String, u8, Vec<(String, String)>);
    static CACHE: OnceLock<Mutex<BTreeMap<Key, ClassSkillAnswer>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    let key = (class_slug.to_string(), class_level, picks.to_vec());
    if let Ok(guard) = cache.lock()
        && let Some(answer) = guard.get(&key)
    {
        return answer.clone();
    }
    let answer = match sheet_rule_package::package() {
        Ok(package) => class_skill_view_with(package, class_slug, class_level, picks),
        Err(reason) => ClassSkillAnswer::Unknown { reason: format!("the converted rule package did not load: {reason}") },
    };
    if let Ok(mut guard) = cache.lock() {
        guard.insert(key, answer.clone());
    }
    answer
}

/// The same answer over an explicit package.
pub fn class_skill_view_in(package: &SheetRulePackage, class_slug: &str, class_level: u8) -> ClassSkillAnswer {
    class_skill_view_with(package, class_slug, class_level, &[])
}

/// [`class_skill_view_in`] with the character's linked picks `(chooser, option)`: a pick counts
/// only when this class's own walk offers its chooser.
pub fn class_skill_view_with(
    package: &SheetRulePackage,
    class_slug: &str,
    class_level: u8,
    picks: &[(String, String)],
) -> ClassSkillAnswer {
    let Some(principal) = package.find("class", class_slug) else {
        return ClassSkillAnswer::Unknown { reason: format!("no converted class record for `{class_slug}`") };
    };
    let level = i64::from(class_level);
    let seed = HeldSeed { classes: vec![(class_slug.to_string(), level)], ..HeldSeed::default() };
    let mut facts = CharacterFacts { level, class_levels: vec![(class_slug.to_string(), level)], ..CharacterFacts::default() };
    for (choice, member) in canonical_member_picks(package, class_slug) {
        facts.choices.entry(choice).or_default().push((member.clone(), member));
    }
    let mut held = held_set(package, &seed, &facts);
    let own: Vec<&(String, String)> =
        picks.iter().filter(|(chooser, _)| chooser_offered(package, &held, &facts, chooser)).collect();
    if !own.is_empty() {
        for (chooser, option) in own {
            let entry = facts.choices.entry(chooser.clone()).or_default();
            if !entry.iter().any(|(o, _)| o == option) {
                entry.push((option.clone(), option.clone()));
            }
        }
        held = held_set(package, &seed, &facts);
    }
    if !held.rules.contains_key(principal) {
        return ClassSkillAnswer::Unknown {
            reason: format!("the class principal rule {principal} is not held at level {class_level}"),
        };
    }
    let mut view = ClassSkillView::default();
    for (id, entry) in &held.rules {
        if held.removed.contains(id) {
            continue;
        }
        let Some(rule) = package.rule(id) else { continue };
        let ctx = EvalContext { holder_class: entry.holder_class.clone(), ..EvalContext::default() };
        for effect in &rule.grants {
            let fact = match effect {
                Effect::FactGrant(fact) => Some(fact.clone()),
                Effect::GatedFactGrant { fact, when } if gate_is_class_decidable(when) => {
                    match resolve_gated_fact_grant(fact, when, package, &held, &facts, ctx.clone()) {
                        GatedFact::Granted(fact) => Some(fact),
                        GatedFact::Conditional { .. } | GatedFact::Excluded => None,
                    }
                }
                _ => None,
            };
            match fact {
                Some(Fact::ClassSkill(skill)) => {
                    view.skills.insert(skill);
                    view.granted_by.insert(id.clone());
                }
                Some(Fact::ClassSkillGroup(group)) => {
                    view.groups.insert(group);
                    view.granted_by.insert(id.clone());
                }
                _ => {}
            }
        }
    }
    if let Err(reason) = add_canonical_class_skill_picks(package, class_slug, &mut view) {
        return ClassSkillAnswer::Unknown { reason };
    }
    if view.skills.is_empty() && view.groups.is_empty() {
        return ClassSkillAnswer::Unknown {
            reason: format!(
                "the converted closure of `{class_slug}` at level {class_level} reaches no class-skill grant \
                 (every PF1 class has class skills, so an empty walk is an incomplete closure, not an empty list)"
            ),
        };
    }
    ClassSkillAnswer::Known(view)
}

/// SD-36 F3c2: a class whose class skills are a CHOICE (Expert, CRB p.450) answers with its
/// Path-A canonical picks, seeded through `class_seeds` like every other choice. One rule: a
/// canonical seed whose choice id is a converted rule that offers `OptionSet::Skills` under its
/// own id AND grants `ClassSkillChosen(<that id>)` (PCGen's `CHOOSE:SKILL` + `CSKILL:LIST`) makes
/// the picked skill a class skill. A pick the chooser does not admit, or that names no converted
/// skill, is `Err` (the class then answers Unknown by name) -- never dropped, never guessed.
/// A class with no such seed is untouched: its answer stays what its record's walk says.
fn add_canonical_class_skill_picks(package: &SheetRulePackage, class_slug: &str, view: &mut ClassSkillView) -> Result<(), String> {
    let (choices, _) = crate::rules_core::class_seeds::canonical_seeds_for(class_slug);
    for c in choices {
        let Some(picker) = package.rule(&c.choice_set_id) else { continue };
        let Some(Choice { id, from: OptionSet::Skills(options), .. }) = &picker.offers else { continue };
        if id != &c.choice_set_id
            || !picker.grants.iter().any(|e| matches!(e, Effect::FactGrant(Fact::ClassSkillChosen(ch)) if ch == id))
        {
            continue;
        }
        let skill = c.selection_id.strip_prefix("skill:").unwrap_or(&c.selection_id);
        let admitted = options.iter().any(|o| o == "all" || o == "any" || o == skill);
        if !admitted || package.find("skill", skill).is_none() {
            return Err(format!(
                "`{class_slug}`'s canonical class-skill pick `{}` under {id} is not a converted skill the chooser \
                 admits (options {options:?})",
                c.selection_id
            ));
        }
        view.skills.insert(skill.to_string());
        view.granted_by.insert(id.clone());
    }
    Ok(())
}

/// The class's Path-A canonical member picks (the proficiency reader's rule, SD-36 F1c-5 D8):
/// `(choice id, selected member id)` where the choice is a converted rule offering
/// `OptionSet::Rules` under its own id and the member is granted by that choice.
fn canonical_member_picks(package: &SheetRulePackage, class_slug: &str) -> Vec<(String, String)> {
    let (choices, _) = crate::rules_core::class_seeds::canonical_seeds_for(class_slug);
    choices
        .into_iter()
        .filter(|c| {
            package.rule(&c.choice_set_id).is_some_and(|picker| {
                matches!(&picker.offers, Some(Choice { id, from: OptionSet::Rules { .. }, .. }) if id == &c.choice_set_id)
            }) && package.rule(&c.selection_id).is_some_and(|member| {
                member.granted_by.iter().any(|g| matches!(&g.by, Granter::Choice(ch) if ch == &c.choice_set_id))
            })
        })
        .map(|c| (c.choice_set_id, c.selection_id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::pilot_compute::{unread_record_selected_class_skill_ids, unread_record_selected_class_skills};

    fn known(slug: &str) -> ClassSkillView {
        match class_skill_view(slug, 1) {
            ClassSkillAnswer::Known(view) => view,
            ClassSkillAnswer::Unknown { reason } => panic!("{slug}: {reason}"),
        }
    }

    #[test]
    fn barbarian_reads_its_core_class_skill_record() {
        // cr_abilities_class.lst:2831 CSKILL:Acrobatics|Climb|TYPE=Craft|Handle Animal|Intimidate|
        // Knowledge (Nature)|Perception|Ride|Survival|Swim
        let view = known("barbarian");
        for skill in ["acrobatics", "climb", "handle_animal", "intimidate", "knowledge_nature", "perception", "ride", "survival", "swim"] {
            assert!(view.contains(skill), "{skill}: {view:?}");
        }
        assert!(view.contains("craft_alchemy"), "TYPE=Craft covers each Craft skill: {view:?}");
        assert!(!view.contains("diplomacy") && !view.contains("knowledge_arcana"), "{view:?}");
    }

    #[test]
    fn wizard_reads_none_of_the_three_selected_skills() {
        // cr_abilities_class.lst:2565: Appraise, Craft, Fly, Knowledge (all), Linguistics,
        // Profession, Spellcraft.
        let view = known("wizard");
        for skill in ["climb", "intimidate", "swim"] {
            assert!(!view.contains(skill), "{skill}: {view:?}");
        }
        assert!(view.contains("knowledge_arcana") && view.contains("spellcraft"), "{view:?}");
    }

    #[test]
    fn a_class_with_no_converted_record_is_unknown_never_empty() {
        assert!(matches!(class_skill_view("no_such_class", 1), ClassSkillAnswer::Unknown { .. }));
    }

    /// The oracle fallback rows exist only for classes the reader cannot answer. When the
    /// converter closes a class's `Class|<Class>` edge, its row starts shadowing nothing and this
    /// test names it for deletion.
    #[test]
    fn every_fallback_row_is_a_class_the_reader_cannot_answer() {
        let mut rows = 0;
        for class_id in unread_record_selected_class_skill_ids() {
            rows += 1;
            let slug = class_id.strip_prefix("class:").unwrap_or(class_id);
            for level in 1..=20 {
                assert!(
                    matches!(class_skill_view(slug, level), ClassSkillAnswer::Unknown { .. }),
                    "{class_id} {level}: the reader now answers; delete its fallback row"
                );
            }
            assert!(unread_record_selected_class_skills(class_id).is_some());
        }
        assert_eq!(rows, 9);
    }
}
