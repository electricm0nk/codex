//! The Starfinder 1e skill totals, read from the CONVERTED Starfinder package
//! (`data/starfinder-1e/sheet_rules/<book>/skill/`), for every skill the package holds. SD-37
//! E4.2 (`epic-breakdown.md` Epic E4; `decisions.md §5`).
//!
//! There is no per-skill or per-class table: the skills are the package's `Base` skill records,
//! and every number is read off the character's held set ([`super::sf_defense::held`]).
//!
//! # Where each term comes from
//!
//! | Term | Source |
//! |---|---|
//! | ranks | the build ([`SfBuild::skill_ranks`]) |
//! | ability modifier | the skill record's key-ability tag (oracle `KEYSTAT:` / `TYPE:Base.<Ability>`) |
//! | +3 trained class skill | system rule, SRD Skills (<https://www.aonsrd.com/Rules.aspx?ID=78>), when a held rule grants the skill as a class skill (class, theme) and it has a rank |
//! | armour check penalty | the worn armour's `StatBlock "Armor check penalty"` row, on a skill tagged `ACHECK` (oracle `ACHECK:YES`) |
//! | every other bonus | every held row targeting the skill (racial trait, theme, class feature, a chosen-skill pick), folded by bonus type |
//!
//! A skill whose `Display ~ <Skill>` record's gate does not admit the character (a trained-only
//! skill with no rank: oracle `PREMULT:1,[PREVAREQ:UseSkillUntrained,1],[PREVARGTEQ:skillinfo(
//! "TOTALRANK", …),1]`) has no total: it cannot be attempted, so nothing prints.
//!
//! # Paper-sheet rule
//!
//! Each total is one number with every term that resolves added in (`decisions.md §5`). A row
//! that resolves to words, or holds only in a situation, is printed by its own line and listed
//! in [`SfSkills::not_folded`], never added.

use super::sf_chassis::{SfChassisRefusal, SfTerm, SfTotal};
use super::sf_defense::{fold, held, held_rows, SfBuild, SfHeld, SfNotFolded};
use crate::rules_core::sheet_rule::{evaluate_applies, split_rule_id, Ability, BonusTarget, Gate, SheetRule, SheetRulePackage};

/// One skill line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfSkill {
    /// Package skill id (`sense_motive`).
    pub skill: String,
    /// The skill's printed name: its `Display ~ <Skill>` record's label (`Sense Motive`,
    /// `Profession (Accountant)`).
    pub label: String,
    pub ability: Ability,
    pub class_skill: bool,
    /// `None`: a trained-only skill without a rank -- no check possible, no total printed.
    pub total: Option<SfTotal>,
}

/// Every skill of one character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfSkills {
    pub skills: Vec<SfSkill>,
    pub not_folded: Vec<SfNotFolded>,
}

impl SfSkills {
    pub fn by_label(&self, label: &str) -> Option<&SfSkill> {
        self.skills.iter().find(|s| s.label == label)
    }
}

pub const REFUSED_SKILL_ABILITY: &str = "sf_skills.key_ability";
pub const REFUSED_SKILL_DISPLAY: &str = "sf_skills.display_record_missing";

const SRD_SKILLS: &str = "SRD Skills (https://www.aonsrd.com/Rules.aspx?ID=78)";

fn refuse(id: &'static str, message: String) -> SfChassisRefusal {
    SfChassisRefusal { id, message }
}

/// The key ability a skill record's tags name (`Dexterity` -> Dex).
fn key_ability(skill: &SheetRule) -> Result<Ability, SfChassisRefusal> {
    let abilities: Vec<Ability> = skill
        .tags
        .iter()
        .filter_map(|t| match t.as_str() {
            "Strength" => Some(Ability::Str),
            "Dexterity" => Some(Ability::Dex),
            "Constitution" => Some(Ability::Con),
            "Intelligence" => Some(Ability::Int),
            "Wisdom" => Some(Ability::Wis),
            "Charisma" => Some(Ability::Cha),
            _ => None,
        })
        .collect();
    match abilities.as_slice() {
        [one] => Ok(*one),
        _ => Err(refuse(REFUSED_SKILL_ABILITY, format!("{}: tags {:?} name {} key abilities", skill.id, skill.tags, abilities.len()))),
    }
}

fn ability_index(a: Ability) -> usize {
    match a {
        Ability::Str => 0,
        Ability::Dex => 1,
        Ability::Con => 2,
        Ability::Int => 3,
        Ability::Wis => 4,
        Ability::Cha => 5,
    }
}

/// The package's skills: every `Base`-tagged skill principal, one per skill id (the first book
/// in id order that states it).
pub fn skill_records(package: &SheetRulePackage) -> Vec<&SheetRule> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out: Vec<&SheetRule> = package
        .rules_of_kind("skill")
        .filter(|r| !r.id.contains('#') && r.tags.iter().any(|t| t == "Base"))
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out.retain(|r| seen.insert(split_rule_id(&r.id).2.to_string()));
    out
}

/// Every skill total of `build`, read from `package` (the Starfinder package).
pub fn compute(package: &SheetRulePackage, build: &SfBuild) -> Result<SfSkills, SfChassisRefusal> {
    let sf = held(package, build)?;
    compute_with(package, build, &sf)
}

/// [`compute`] over an already-held build.
pub fn compute_with(package: &SheetRulePackage, build: &SfBuild, sf: &SfHeld) -> Result<SfSkills, SfChassisRefusal> {
    let acp = super::sf_defense::armor_check_penalty(package, build)?;
    let mut not_folded = Vec::new();
    let mut skills = Vec::new();
    for record in skill_records(package) {
        let (book, _, slug) = split_rule_id(&record.id);
        let ability = key_ability(record)?;
        let display_id = format!("{book}:skill:display_{slug}");
        let display = package
            .rule(&display_id)
            .ok_or_else(|| refuse(REFUSED_SKILL_DISPLAY, format!("{}: no {display_id} record", record.id)))?;
        let class_skill =
            sf.class_skills.contains(slug) || record.tags.iter().any(|t| sf.class_skill_groups.contains(t));
        let ranks = build.skill_ranks.get(slug).copied().unwrap_or(0);
        let open = evaluate_applies(&display.applies, &sf.held, package, &sf.facts, Default::default());
        if matches!(open, Gate::Exclude) {
            skills.push(SfSkill { skill: slug.to_string(), label: display.label.clone(), ability, class_skill, total: None });
            continue;
        }
        let mut terms = Vec::new();
        if ranks != 0 {
            terms.push(SfTerm { label: "ranks".into(), value: ranks, source: record.id.clone() });
        }
        terms.push(SfTerm {
            label: format!("{ability:?} modifier"),
            value: sf.facts.ability_mods[ability_index(ability)],
            source: record.id.clone(),
        });
        if class_skill && ranks >= 1 {
            terms.push(SfTerm { label: "trained class skill".into(), value: 3, source: SRD_SKILLS.into() });
        }
        if record.tags.iter().any(|t| t == "ACHECK") && acp != 0 {
            terms.push(SfTerm {
                label: "armor check penalty".into(),
                value: acp,
                source: build.armor.clone().unwrap_or_default(),
            });
        }
        let wants = |t: &BonusTarget| match t {
            BonusTarget::Skill(s) => s == slug,
            BonusTarget::SkillGroup(g) => g == "All" || record.tags.iter().any(|x| x == g),
            _ => false,
        };
        terms.extend(fold(held_rows(package, sf, &wants, &mut not_folded)));
        skills.push(SfSkill {
            skill: slug.to_string(),
            label: display.label.clone(),
            ability,
            class_skill,
            total: Some(SfTotal { total: terms.iter().map(|t| t.value).sum(), terms }),
        });
    }
    Ok(SfSkills { skills, not_folded })
}

#[cfg(test)]
mod sf_seed {
    use super::super::sf_defense::seed_support::*;
    use super::*;

    /// Every skill row of `seed-hand-values.md` (20 skills x 4 seeds), against the engine. The
    /// row `Profession` stands for every `Profession (…)` skill the package holds.
    #[test]
    fn sf_seed_every_skill_total_matches_the_srd_hand_values() {
        let hand = hand_values();
        let mut bad = Vec::new();
        let mut checked = 0;
        for (seed, build) in seeds() {
            let skills = compute(package(), &build).unwrap_or_else(|r| panic!("{seed}: {r:?}"));
            for ((s, field), want) in hand.iter().filter(|((s, f), _)| s == seed && f.starts_with("Skill: ")) {
                let name = field.trim_start_matches("Skill: ");
                let got: Vec<(&str, Hand)> = if name == "Profession" {
                    skills
                        .skills
                        .iter()
                        .filter(|k| k.label.starts_with("Profession ("))
                        .map(|k| (k.label.as_str(), k.total.as_ref().map_or(Hand::Untrained, |t| Hand::Value(t.total))))
                        .collect()
                } else {
                    let k = skills.by_label(name).unwrap_or_else(|| panic!("{s}: no skill labelled {name:?}"));
                    vec![(k.label.as_str(), k.total.as_ref().map_or(Hand::Untrained, |t| Hand::Value(t.total)))]
                };
                assert!(!got.is_empty(), "{s} {field}: no package skill");
                checked += 1;
                for (label, g) in got {
                    if g != *want {
                        let terms = skills.by_label(label).and_then(|k| k.total.clone());
                        bad.push(format!("{s} {label}: engine {g:?}, SRD {want:?} -- {terms:?}"));
                    }
                }
            }
        }
        assert_eq!(checked, 80, "4 seeds x 20 skills");
        assert!(bad.is_empty(), "{} skill totals mismatch:\n{}", bad.len(), bad.join("\n"));
    }

    /// The criterion names it: every Envoy 3 skill total (`decisions.md §9`, the skill-heavy seed),
    /// each term named.
    #[test]
    fn sf_seed_every_envoy_3_skill_total_names_its_terms() {
        let (_, envoy) = seeds().remove(3);
        let skills = compute(package(), &envoy).unwrap();
        let ranked: Vec<&SfSkill> = skills.skills.iter().filter(|k| envoy.skill_ranks.contains_key(&k.skill)).collect();
        assert_eq!(ranked.len(), 9, "the envoy ranks 9 skills (seed-builds.md §4)");
        for k in skills.skills.iter().filter_map(|k| k.total.as_ref().map(|t| (k, t))) {
            let (k, t) = k;
            for term in &t.terms {
                assert!(term.source.starts_with("core:") || term.source.starts_with("SRD "), "{}: {term:?}", k.label);
            }
        }
        // Scrounger (+2 racial, Engineering/Stealth/Survival) is read from the racial trait.
        for label in ["Engineering", "Stealth", "Survival"] {
            let t = skills.by_label(label).and_then(|k| k.total.clone()).unwrap();
            assert!(t.terms.iter().any(|x| x.source.starts_with("core:ability:ysoki_default_scrounger") && x.value == 2), "{label}: {t:?}");
        }
    }

    /// A trained-only skill is attempted only with a rank: the same skill with one rank gets a
    /// total (a planted rank, not a seed value).
    #[test]
    fn sf_seed_a_trained_only_skill_opens_with_a_rank() {
        let (_, mut soldier) = seeds().remove(0);
        let before = compute(package(), &soldier).unwrap();
        assert_eq!(before.by_label("Computers").unwrap().total, None);
        soldier.skill_ranks.insert("computers".into(), 1);
        let after = compute(package(), &soldier).unwrap();
        let t = after.by_label("Computers").unwrap().total.clone().unwrap();
        // 1 rank + Int 0; Computers is not a soldier class skill, so no +3.
        assert_eq!(t.total, 1, "{t:?}");
    }
}
