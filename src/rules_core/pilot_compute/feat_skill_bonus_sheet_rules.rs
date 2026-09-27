//! SD-36 Epic F6b: the skill bonuses a character's held FEAT records grant, folded per skill --
//! read off the converted records, never a per-feat table.
//!
//! # Mechanism (one rule, no per-feat cases)
//!
//! Input: the character's rendered sheet lines (`render_sheet` via
//! [`PilotBaseChassisComputation::with_sheet_rules`](super::PilotBaseChassisComputation::with_sheet_rules)),
//! so the fold reads exactly the lines the sheet prints, evaluated by the one evaluator. A line
//! folds into a skill total when:
//!
//! - it is a feat line (`kind == "feat"`) whose converted rule targets
//!   `BonusTarget::Skill(<skill>)` or `BonusTarget::SkillGroup(<family>)`, and
//! - it resolved to a number (`SheetLineValue::Resolved`), and
//! - it carries no situational condition (a conditional bonus is printed on its own line and
//!   not added to the always-on total -- "print the rule text once").
//!
//! Per skill, contributions fold by bonus type the way the package's own `Var` fold does:
//! untyped, negative and `STACKING_TYPES` contributions sum; two of any other type take the max.
//!
//! A held feat line that targets a skill but evaluated to words is Unknown by name
//! ([`FeatSkillBonuses::unknown`]); a conditional one is listed in
//! [`FeatSkillBonuses::situational`]. Neither is added.
//!
//! Not in this fold (named remainder): `BonusTarget::Chosen` feats whose target is the
//! character's pick (Skill Focus), `SkillSituation` targets (printed with their situation), and
//! skill bonuses from non-feat records (traits, racial traits, class features).

use std::collections::BTreeMap;

use crate::rules_core::sheet_rule::{BonusTarget, SheetLine, SheetLineValue, SheetRulePackage, STACKING_TYPES};

/// One feat line's contribution to a skill (or a skill family).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatSkillContribution {
    /// The package skill id (`perception`, `sense_motive`), or the family (`knowledge`) for a
    /// `SkillGroup` target.
    pub skill: String,
    /// `true` for a `SkillGroup` target: the bonus applies to every skill of the family.
    pub group: bool,
    pub value: i32,
    /// The feat line's rule id (`core_rulebook:feat:alertness#bonus1`).
    pub rule_id: String,
    /// The feat line's label, as the sheet prints it.
    pub label: String,
    /// The bonus type's name, when typed.
    pub bonus_type: Option<String>,
}

/// A feat line that targets a skill but is not added to its total, and why.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatSkillNotFolded {
    pub skill: String,
    pub rule_id: String,
    pub label: String,
    /// The situational condition, or the reason the value is Unknown.
    pub reason: String,
}

/// The fold's answer for one character.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatSkillBonuses {
    /// Folded total per skill id (exact targets).
    pub skills: BTreeMap<String, i32>,
    /// Folded total per skill family (`SkillGroup` targets), applied to every family member.
    pub groups: BTreeMap<String, i32>,
    /// Every folded contribution, in line order.
    pub contributions: Vec<FeatSkillContribution>,
    /// Conditional lines: printed with their condition, not added.
    pub situational: Vec<FeatSkillNotFolded>,
    /// Lines whose value the evaluator left as words: Unknown, not added.
    pub unknown: Vec<FeatSkillNotFolded>,
}

/// The skill (or family) a feat rule targets, if it targets one.
fn skill_target(target: Option<&BonusTarget>) -> Option<(String, bool)> {
    match target? {
        BonusTarget::Skill(skill) => Some((skill.clone(), false)),
        BonusTarget::SkillGroup(family) => Some((family.to_lowercase(), true)),
        _ => None,
    }
}

/// Folds `contributions` by bonus type (see the module doc).
fn fold(contributions: &[&FeatSkillContribution]) -> i32 {
    let mut summed = 0;
    let mut typed_max: BTreeMap<&str, i32> = BTreeMap::new();
    for c in contributions {
        match c.bonus_type.as_deref() {
            None => summed += c.value,
            Some(name) if c.value < 0 || STACKING_TYPES.contains(&name) => summed += c.value,
            Some(name) => {
                let e = typed_max.entry(name).or_insert(c.value);
                *e = (*e).max(c.value);
            }
        }
    }
    summed + typed_max.values().sum::<i32>()
}

/// The feat skill-bonus fold over a character's rendered `lines`.
pub fn feat_skill_bonuses(package: &SheetRulePackage, lines: &[SheetLine]) -> FeatSkillBonuses {
    let mut out = FeatSkillBonuses::default();
    for line in lines.iter().filter(|l| l.kind == "feat") {
        let Some(rule) = package.rule(&line.id) else { continue };
        let Some((skill, group)) = skill_target(rule.target.as_ref()) else { continue };
        if let Some(condition) = &line.condition {
            out.situational.push(FeatSkillNotFolded {
                skill,
                rule_id: line.id.clone(),
                label: line.label.clone(),
                reason: condition.clone(),
            });
            continue;
        }
        match &line.value {
            SheetLineValue::Resolved(value) => out.contributions.push(FeatSkillContribution {
                skill,
                group,
                value: *value,
                rule_id: line.id.clone(),
                label: line.label.clone(),
                bonus_type: rule.bonus_type.as_ref().map(|t| t.name.clone()),
            }),
            other => out.unknown.push(FeatSkillNotFolded {
                skill,
                rule_id: line.id.clone(),
                label: line.label.clone(),
                reason: format!("the feat line's value did not resolve to a number ({other:?})"),
            }),
        }
    }
    let mut keys: Vec<(String, bool)> = out.contributions.iter().map(|c| (c.skill.clone(), c.group)).collect();
    keys.sort();
    keys.dedup();
    for (skill, group) in keys {
        let of: Vec<&FeatSkillContribution> =
            out.contributions.iter().filter(|c| c.skill == skill && c.group == group).collect();
        let total = fold(&of);
        if group {
            out.groups.insert(skill, total);
        } else {
            out.skills.insert(skill, total);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::character_input::{CharacterInput, SelectedChoice};
    use crate::rules_core::class_census::load_sweep_fixture;
    use crate::rules_core::sheet_rule_package;

    fn package() -> &'static SheetRulePackage {
        sheet_rule_package::package().as_ref().expect("the converted package loads")
    }

    /// The census fixture (Human Fighter 1) with `feat` held as its level-1 character feat.
    fn fixture_with_feat(feat_slug: &str) -> CharacterInput {
        let fixture = load_sweep_fixture().expect("census fixture");
        let mut input = crate::rules_core::class_seeds::input_for(&fixture, "fighter", 1);
        let feat_id = format!("feat:{feat_slug}");
        input.chosen.selected_feats.retain(|f| f != "feat:power_attack");
        input.chosen.selected_feats.push(feat_id.clone());
        let mut replaced = 0;
        for choice in input.chosen.selected_choices.iter_mut() {
            if choice.choice_set_id == "choice:level_1_character_feat" {
                *choice = SelectedChoice { choice_set_id: choice.choice_set_id.clone(), selection_id: feat_id.clone() };
                replaced += 1;
            }
        }
        assert_eq!(replaced, 1, "the fixture records its level-1 character feat pick once");
        input
    }

    fn fold_for(input: &CharacterInput) -> FeatSkillBonuses {
        let computed = crate::rules_core::pilot_compute::compute_pilot_base_chassis(input).with_sheet_rules(input, package(), &[]);
        feat_skill_bonuses(package(), &computed.sheet_lines)
    }

    /// CRB p.117, Alertness: "You get a +2 bonus on Perception and Sense Motive skill checks. If
    /// you have 10 or more ranks in one of these skills, the bonus increases to +4 for that
    /// skill." Level 1 (at most 1 rank): +2 on each.
    #[test]
    fn alertness_adds_two_to_perception_and_sense_motive_at_level_one() {
        let bonuses = fold_for(&fixture_with_feat("alertness"));
        assert_eq!(bonuses.skills.get("perception"), Some(&2), "{bonuses:#?}");
        assert_eq!(bonuses.skills.get("sense_motive"), Some(&2), "{bonuses:#?}");
        assert_eq!(bonuses.skills.len(), 2, "Alertness touches exactly two skills: {bonuses:#?}");
    }

    /// Negative control: the fixture's own feats (Power Attack, Dodge, Weapon Focus) grant no
    /// skill bonus, so nothing folds.
    #[test]
    fn the_fixture_s_own_feats_fold_nothing() {
        let fixture = load_sweep_fixture().expect("census fixture");
        let input = crate::rules_core::class_seeds::input_for(&fixture, "fighter", 1);
        let bonuses = fold_for(&input);
        assert!(bonuses.contributions.is_empty() && bonuses.groups.is_empty(), "{bonuses:#?}");
    }

    /// Typed bonuses of one type take the max; untyped and stacking types sum.
    #[test]
    fn the_fold_stacks_by_bonus_type() {
        let c = |value, bonus_type: Option<&str>| FeatSkillContribution {
            skill: "stealth".into(),
            group: false,
            value,
            rule_id: String::new(),
            label: String::new(),
            bonus_type: bonus_type.map(str::to_owned),
        };
        let (a, b, d, e, f) = (c(2, None), c(1, None), c(2, Some("Competence")), c(3, Some("Competence")), c(1, Some("Racial")));
        assert_eq!(fold(&[&a, &b, &d, &e, &f]), 2 + 1 + 3 + 1);
    }

    /// Population measure: every converted feat record carrying a skill bonus (a principal or
    /// `#bonusN` rule targeting `Skill` / `SkillGroup`), held alone as the census fixture's
    /// level-1 character feat. Each is counted as folding (at least one of its skill rules adds
    /// to a total), situational only, Unknown, or not rendered (no skill line printed: the rule's
    /// own gate is false on this fixture -- e.g. a bonus that needs ranks, a class or a trait).
    /// Denominator: the feats the package holds with such a rule. Printed with
    /// `--nocapture`; the counts are pinned so a converter or evaluator change shows here.
    #[test]
    fn every_skill_bonus_feat_is_counted() {
        let package = package();
        let mut feats: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (id, rule) in &package.rules {
            let mut parts = id.split(':');
            let (Some(_book), Some("feat"), Some(slug)) = (parts.next(), parts.next(), parts.next()) else { continue };
            if skill_target(rule.target.as_ref()).is_some() {
                let slug = slug.split('#').next().unwrap_or(slug).to_owned();
                feats.entry(slug).or_default().push(id.clone());
            }
        }
        let rules: usize = feats.values().map(Vec::len).sum();
        let (mut folds, mut situational, mut unknown, mut not_rendered) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for slug in feats.keys() {
            let bonuses = fold_for(&fixture_with_feat(slug));
            if !bonuses.contributions.is_empty() {
                folds.push(slug.clone());
            } else if !bonuses.unknown.is_empty() {
                unknown.push(format!("{slug}: {:?}", bonuses.unknown.iter().map(|u| &u.reason).collect::<Vec<_>>()));
            } else if !bonuses.situational.is_empty() {
                situational.push(slug.clone());
            } else {
                not_rendered.push(slug.clone());
            }
        }
        eprintln!("feat slugs with a skill-bonus rule: {} ({rules} rules)", feats.len());
        eprintln!("  fold into a skill total: {} {folds:?}", folds.len());
        eprintln!("  situational only (printed, not added): {} {situational:?}", situational.len());
        eprintln!("  Unknown (words): {} {unknown:?}", unknown.len());
        eprintln!("  no skill line rendered on the fixture: {} {not_rendered:?}", not_rendered.len());
        assert!(folds.iter().any(|f| f == "alertness"));
        assert_eq!(
            (feats.len(), folds.len(), situational.len(), unknown.len(), not_rendered.len()),
            (43, 40, 2, 0, 1),
            "pin the measured counts (feat slugs; fold; situational only; Unknown; no line on the fixture)"
        );
    }
}
