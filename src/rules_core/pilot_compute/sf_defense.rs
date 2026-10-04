//! The Starfinder 1e defense totals -- Energy Armor Class, Kinetic Armor Class and initiative --
//! read from the CONVERTED Starfinder package (`data/starfinder-1e/sheet_rules/`), for any
//! character the package can hold. SD-37 E4.2 (`epic-breakdown.md` Epic E4; `decisions.md §5`).
//!
//! # The character the package holds
//!
//! [`SfBuild`] is the E4.1 chassis build plus the picks a sheet total reads: the theme, the worn
//! armour, other held records (a mystic connection, a pool pick), skill ranks and choices.
//! [`held`] turns it into the package's own held set ([`held_set`]: race, class, theme and every
//! record they grant, to a fixpoint) and the [`CharacterFacts`] its gates read. Both defense and
//! skills ([`super::sf_skills`]) fold rows off that one held set with the one evaluator
//! ([`evaluate`]), so a number on the sheet is the number the package's own rows produce.
//!
//! PCGen applies one more template to every character: `First Level Base Class`
//! (`scr_templates.lst:29`, `CHOOSE:TEMPLATE|QUALIFIED[TYPE=BaseClass]`), the `BaseClass`
//! template of the class taken at 1st level. Its variable contributions (`CS_First_<Skill>`)
//! decide a theme's "already a class skill" bonus. The pick is a fact of the build (the first
//! class), so [`held`] holds the `BaseClass` template whose own gate names that class -- found by
//! its tag and gate in the package, never by name.
//!
//! # Where each term comes from
//!
//! | Total | Term | Source |
//! |---|---|---|
//! | EAC / KAC | 10 | system rule, SRD Armor Class (<https://www.aonsrd.com/Rules.aspx?ID=102>) |
//! | EAC / KAC | armour bonus | the worn armour's `Eac` / `Kac` rows (oracle `BONUS:COMBAT\|AC\|n\|TYPE=EAC_Armor`) |
//! | EAC / KAC | Dex modifier, capped by the armour's max Dex | system rule (ID=102); the cap is the armour's `StatBlock "Max Dex bonus"` row (oracle `MAXDEX:`) |
//! | EAC / KAC / initiative | every other held row targeting the total | the package rows, folded by bonus type |
//! | initiative | Dex modifier | system rule, SRD Initiative (<https://www.aonsrd.com/Rules.aspx?ID=96>) |
//!
//! A row that resolves to words, or holds only in a situation, is not added: it is listed in
//! [`SfDefense::not_folded`] and printed by its own line. A term this reader cannot resolve is a
//! named [`SfChassisRefusal`], never a 0.

use std::collections::{BTreeMap, BTreeSet};

use super::sf_chassis::{ability_modifier, SfChassisBuild, SfChassisRefusal, SfTerm, SfTotal};
use crate::rules_core::game_system::GameSystem;
use crate::rules_core::sheet_rule::{
    evaluate, evaluate_applies, sibling_line_gate, split_rule_id, stacking_types, Ability, Applies, BonusTarget, CharacterFacts, Cmp,
    EvalContext, Expr, Gate, HeldSeed, HeldSet, ProseFamily, ProsePiece, RuleId, SheetLineValue, SheetRule, SheetRulePackage,
    StackMode,
};

/// One Starfinder character as the sheet totals read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfBuild {
    /// Classes, race, final ability scores and the key-ability choice (E4.1). The FIRST class is
    /// the one taken at 1st level.
    pub chassis: SfChassisBuild,
    /// The theme's selection record (`core:ability:icon`), if the character has a theme.
    pub theme: Option<RuleId>,
    /// The worn armour's record (`core:equipment:carbon_skin_graphite`).
    pub armor: Option<RuleId>,
    /// Every other record the character holds by choice: a mystic connection
    /// (`core:ability:empath`), a pool pick (`core:ability:2_racial_bonus_to_skill`), a feat.
    pub picks: Vec<RuleId>,
    /// Package skill id (`sense_motive`) -> ranks.
    pub skill_ranks: BTreeMap<String, i64>,
    /// Choice id (the choosing rule's id) -> the options chosen (package skill ids for a skill
    /// choice). A repeatable pick chosen twice lists both options.
    pub choices: BTreeMap<String, Vec<String>>,
}

/// The held set and facts of one [`SfBuild`]: what every Starfinder total reads.
pub struct SfHeld {
    pub held: HeldSet,
    pub facts: CharacterFacts,
    /// Class skills granted by held rules (class, theme), package skill ids.
    pub class_skills: BTreeSet<String>,
    /// Skill families granted as class skills (`Profession`).
    pub class_skill_groups: BTreeSet<String>,
}

/// A held row that targets a total but is not added to it, and why (printed on its own line).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfNotFolded {
    pub rule_id: RuleId,
    pub reason: String,
}

/// The defense totals of one Starfinder character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfDefense {
    pub eac: SfTotal,
    pub kac: SfTotal,
    pub initiative: SfTotal,
    /// The worn armour's armour check penalty (0 with no armour); skills read it.
    pub armor_check_penalty: i64,
    pub not_folded: Vec<SfNotFolded>,
}

pub const REFUSED_ARMOR_NOT_ARMOR: &str = "sf_defense.armor_not_armor";
pub const REFUSED_ARMOR_STAT: &str = "sf_defense.armor_stat";
pub const REFUSED_FIRST_CLASS_TEMPLATE: &str = "sf_defense.first_class_template";
pub const REFUSED_NOT_HELD: &str = "sf_defense.pick_not_in_package";

const SRD_AC: &str = "SRD Armor Class (https://www.aonsrd.com/Rules.aspx?ID=102)";
const SRD_INITIATIVE: &str = "SRD Initiative (https://www.aonsrd.com/Rules.aspx?ID=96)";

fn refuse(id: &'static str, message: String) -> SfChassisRefusal {
    SfChassisRefusal { id, message }
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

/// The `BaseClass` template PCGen's `First Level Base Class` choice applies for `class_slug`:
/// the package template tagged `BaseClass` whose own gate is `ClassLevel(<class>) >= 1`.
fn first_class_template(package: &SheetRulePackage, class_slug: &str) -> Result<RuleId, SfChassisRefusal> {
    let found: Vec<&SheetRule> = package
        .rules_of_kind("template")
        .filter(|r| !r.id.contains('#') && r.tags.iter().any(|t| t == "BaseClass"))
        .filter(|r| {
            matches!(&r.applies, Applies::Compare { lhs: Expr::ClassLevel(c), op: Cmp::Gte, rhs: Expr::Const(1) } if c == class_slug)
        })
        .collect();
    match found.as_slice() {
        [one] => Ok(one.id.clone()),
        [] => Err(refuse(REFUSED_FIRST_CLASS_TEMPLATE, format!("no BaseClass template is gated on {class_slug} level 1"))),
        many => Err(refuse(
            REFUSED_FIRST_CLASS_TEMPLATE,
            format!("{} BaseClass templates are gated on {class_slug} level 1: {:?}", many.len(), many.iter().map(|r| &r.id).collect::<Vec<_>>()),
        )),
    }
}

/// The package's held set for `build`, and the facts its gates read.
pub fn held(package: &SheetRulePackage, build: &SfBuild) -> Result<SfHeld, SfChassisRefusal> {
    let chassis = &build.chassis;
    let classes: Vec<(String, i64)> =
        chassis.classes.iter().map(|(id, l)| (split_rule_id(id).2.to_string(), i64::from(*l))).collect();
    let level: i64 = classes.iter().map(|(_, l)| l).sum();
    let mut rule_ids: Vec<RuleId> = vec![chassis.race.clone()];
    if let Some((first, _)) = classes.first() {
        rule_ids.push(first_class_template(package, first)?);
    }
    rule_ids.extend(build.theme.iter().cloned());
    rule_ids.extend(build.armor.iter().cloned());
    rule_ids.extend(build.picks.iter().cloned());
    for id in &rule_ids {
        if package.rule(id).is_none() {
            return Err(refuse(REFUSED_NOT_HELD, format!("{id}: no such record in the Starfinder package")));
        }
    }
    let seed = HeldSeed { classes: classes.clone(), rule_ids, ..HeldSeed::default() };
    let mut facts = CharacterFacts {
        level,
        class_levels: classes,
        ability_scores: chassis.ability_scores,
        ability_mods: chassis.ability_scores.map(ability_modifier),
        race: Some(split_rule_id(&chassis.race).2.to_string()),
        theme: build.theme.as_deref().map(|t| split_rule_id(t).2.to_string()),
        skill_ranks: build.skill_ranks.clone(),
        choices: build
            .choices
            .iter()
            .map(|(c, options)| (c.clone(), options.iter().map(|o| (o.clone(), o.clone())).collect()))
            .collect(),
        ..CharacterFacts::default()
    };
    // The class-skill facts gate some rows (`Holdable::ClassSkill`), and they come from held
    // rules: hold, read the class skills, hold again with them.
    let mut held = crate::rules_core::sheet_rule::held_set(package, &seed, &facts);
    let (mut class_skills, mut class_skill_groups) = class_skills_of(package, &held, &facts);
    for _ in 0..4 {
        facts.class_skills = class_skills.clone();
        held = crate::rules_core::sheet_rule::held_set(package, &seed, &facts);
        let (next, next_groups) = class_skills_of(package, &held, &facts);
        if next == class_skills && next_groups == class_skill_groups {
            break;
        }
        class_skills = next;
        class_skill_groups = next_groups;
    }
    Ok(SfHeld { held, facts, class_skills, class_skill_groups })
}

/// Every class skill a held rule grants (`FactGrant(ClassSkill)`, a gated grant whose gate
/// includes, a chosen class skill), and every skill family granted whole.
fn class_skills_of(package: &SheetRulePackage, held: &HeldSet, facts: &CharacterFacts) -> (BTreeSet<String>, BTreeSet<String>) {
    use crate::rules_core::sheet_rule::{resolve_gated_fact_grant, Effect, Fact, GatedFact};
    let mut skills = BTreeSet::new();
    let mut groups = BTreeSet::new();
    for id in held.rules.keys().filter(|id| !held.removed.contains(*id)) {
        let Some(rule) = package.rule(id) else { continue };
        for effect in &rule.grants {
            let fact = match effect {
                Effect::FactGrant(f) => Some(f.clone()),
                Effect::GatedFactGrant { fact, when } => {
                    match resolve_gated_fact_grant(fact, when, package, held, facts, ctx_for(held, id)) {
                        GatedFact::Granted(f) => Some(f),
                        _ => None,
                    }
                }
                _ => None,
            };
            match fact {
                Some(Fact::ClassSkill(s)) => {
                    skills.insert(s);
                }
                Some(Fact::ClassSkillGroup(g)) => {
                    groups.insert(g);
                }
                Some(Fact::ClassSkillChosen(c)) => {
                    skills.extend(facts.choices.get(&c).into_iter().flatten().map(|(o, _)| o.clone()));
                }
                _ => {}
            }
        }
    }
    (skills, groups)
}

pub(crate) fn ctx_for(held: &HeldSet, id: &str) -> EvalContext {
    let entry = held.rules.get(id).cloned().unwrap_or_default();
    EvalContext { holder_class: entry.holder_class, spell_level: entry.spell_level.unwrap_or(0), item_tags: Vec::new() }
}

/// One held row's contribution to a total, before the bonus-type fold.
#[derive(Debug, Clone)]
pub(crate) struct Contribution {
    pub term: SfTerm,
    pub bonus_type: Option<(String, StackMode)>,
}

/// Every held, open row whose target `wants` accepts, evaluated by the one evaluator: a number
/// is a contribution; words, dice or a situational gate are not added ([`SfNotFolded`]).
/// A `Chosen` target is offered to `wants` once per option the character chose.
pub(crate) fn held_rows(
    package: &SheetRulePackage,
    sf: &SfHeld,
    wants: &dyn Fn(&BonusTarget) -> bool,
    not_folded: &mut Vec<SfNotFolded>,
) -> Vec<Contribution> {
    let mut out = Vec::new();
    for id in sf.held.rules.keys().filter(|id| !sf.held.removed.contains(*id)) {
        let Some(rule) = package.rule(id) else { continue };
        let Some(target) = &rule.target else { continue };
        let targets: Vec<BonusTarget> = match target {
            BonusTarget::Chosen(c) => sf
                .facts
                .choices
                .get(c)
                .into_iter()
                .flatten()
                .map(|(option, _)| BonusTarget::Skill(option.clone()))
                .collect(),
            t => vec![t.clone()],
        };
        let hits = targets.iter().filter(|t| wants(t)).count();
        if hits == 0 {
            continue;
        }
        let ctx = ctx_for(&sf.held, id);
        let gate = if rule.id.contains('#') {
            sibling_line_gate(package, &sf.held, &sf.facts, rule, ctx.clone())
        } else {
            evaluate_applies(&rule.applies, &sf.held, package, &sf.facts, ctx.clone())
        };
        match gate {
            Gate::Exclude => continue,
            Gate::Situational(text) => {
                not_folded.push(SfNotFolded { rule_id: id.clone(), reason: format!("situational: {text}") });
                continue;
            }
            Gate::Include => {}
        }
        let line = evaluate(rule, &sf.held, package, &sf.facts, ctx);
        let value = match line.value {
            SheetLineValue::Resolved(n) => i64::from(n),
            SheetLineValue::Dice(d) => {
                not_folded.push(SfNotFolded { rule_id: id.clone(), reason: format!("dice, rolled: {d}") });
                continue;
            }
            SheetLineValue::Words => {
                not_folded.push(SfNotFolded { rule_id: id.clone(), reason: "resolves to words, not a number".into() });
                continue;
            }
        };
        for _ in 0..hits {
            out.push(Contribution {
                term: SfTerm { label: rule.label.clone(), value, source: id.clone() },
                bonus_type: rule.bonus_type.as_ref().map(|t| (t.name.clone(), t.mode)),
            });
        }
    }
    out
}

/// The package's bonus-type fold (the `Var` fold's rule, `technical-design.md` §2): untyped,
/// `Stack`, negative and stacking-type contributions all add; of the rest, each type keeps its
/// largest (a `Replace` of a type competes with that type's plain largest). Returns the terms
/// that count.
pub(crate) fn fold(contributions: Vec<Contribution>) -> Vec<SfTerm> {
    let stacking = stacking_types(GameSystem::Starfinder1e);
    let mut out = Vec::new();
    let mut best: BTreeMap<String, SfTerm> = BTreeMap::new();
    for c in contributions {
        match &c.bonus_type {
            None => out.push(c.term),
            Some((name, mode)) if *mode == StackMode::Stack || c.term.value < 0 || stacking.contains(&name.as_str()) => out.push(c.term),
            Some((name, _)) => {
                let keep = best.get(name).is_none_or(|b| c.term.value > b.value);
                if keep {
                    best.insert(name.clone(), c.term);
                }
            }
        }
    }
    out.extend(best.into_values());
    out
}

/// A worn armour's `StatBlock "<label>"` row as an integer (`"-4"`, `"1"`).
fn armor_stat(armor: &SheetRule, label: &str) -> Result<i64, SfChassisRefusal> {
    let text: Option<String> = armor.prose.iter().find_map(|seg| match &seg.family {
        ProseFamily::StatBlock(l) if l == label => {
            seg.pieces.iter().map(|p| if let ProsePiece::Text(s) = p { Some(s.as_str()) } else { None }).collect()
        }
        _ => None,
    });
    let text = text.ok_or_else(|| refuse(REFUSED_ARMOR_STAT, format!("{}: the armour states no {label:?}", armor.id)))?;
    text.trim()
        .parse::<i64>()
        .map_err(|_| refuse(REFUSED_ARMOR_STAT, format!("{}: {label} {text:?} is not a number", armor.id)))
}

/// The worn armour's record, checked to be armour (`ARMOR` tag).
fn worn_armor<'p>(package: &'p SheetRulePackage, build: &SfBuild) -> Result<Option<&'p SheetRule>, SfChassisRefusal> {
    let Some(id) = &build.armor else { return Ok(None) };
    let armor = package.rule(id).ok_or_else(|| refuse(REFUSED_NOT_HELD, format!("{id}: no such record in the Starfinder package")))?;
    if !armor.tags.iter().any(|t| t == "ARMOR") {
        return Err(refuse(REFUSED_ARMOR_NOT_ARMOR, format!("{id}: not tagged ARMOR")));
    }
    Ok(Some(armor))
}

/// The armour check penalty of the worn armour (`StatBlock "Armor check penalty"`, oracle
/// `ACCHECK:`), 0 with none.
pub fn armor_check_penalty(package: &SheetRulePackage, build: &SfBuild) -> Result<i64, SfChassisRefusal> {
    match worn_armor(package, build)? {
        Some(armor) => armor_stat(armor, "Armor check penalty"),
        None => Ok(0),
    }
}

/// EAC, KAC and initiative of `build`, read from `package` (the Starfinder package).
pub fn compute(package: &SheetRulePackage, build: &SfBuild) -> Result<SfDefense, SfChassisRefusal> {
    let sf = held(package, build)?;
    compute_with(package, build, &sf)
}

/// [`compute`] over an already-held build.
pub fn compute_with(package: &SheetRulePackage, build: &SfBuild, sf: &SfHeld) -> Result<SfDefense, SfChassisRefusal> {
    let dex = sf.facts.ability_mods[ability_index(Ability::Dex)];
    let armor = worn_armor(package, build)?;
    let dex_term = match armor {
        Some(a) => {
            let cap = armor_stat(a, "Max Dex bonus")?;
            SfTerm { label: format!("Dexterity modifier (max {cap} in {})", a.label), value: dex.min(cap), source: a.id.clone() }
        }
        None => SfTerm { label: "Dexterity modifier".into(), value: dex, source: SRD_AC.into() },
    };
    let mut not_folded = Vec::new();
    let mut ac = |target: BonusTarget| -> SfTotal {
        let rows = held_rows(package, sf, &|t| *t == target, &mut not_folded);
        let mut terms = vec![SfTerm { label: "base".into(), value: 10, source: SRD_AC.into() }];
        terms.extend(fold(rows));
        terms.push(dex_term.clone());
        SfTotal { total: terms.iter().map(|t| t.value).sum(), terms }
    };
    let eac = ac(BonusTarget::Eac);
    let kac = ac(BonusTarget::Kac);
    let mut init_terms = vec![SfTerm { label: "Dexterity modifier".into(), value: dex, source: SRD_INITIATIVE.into() }];
    init_terms.extend(fold(held_rows(package, sf, &|t| *t == BonusTarget::Initiative, &mut not_folded)));
    let initiative = SfTotal { total: init_terms.iter().map(|t| t.value).sum(), terms: init_terms };
    let armor_check_penalty = match armor {
        Some(a) => armor_stat(a, "Armor check penalty")?,
        None => 0,
    };
    Ok(SfDefense { eac, kac, initiative, armor_check_penalty, not_folded })
}

/// Test support shared by the E4.2 seed tests: the four SD-37 Starfinder seed builds
/// (`seed-builds.md` §1–§4) and the SRD hand values (`seed-hand-values.md`, E0.4, plus
/// E4.2's initiative supplement).
#[cfg(test)]
pub(crate) mod seed_support {
    use super::*;
    use crate::rules_core::corpus_loader::live_sheet_rules_for;

    pub const HAND_VALUES: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-hand-values.md";
    pub const INITIATIVE_HAND_VALUES: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.2-initiative-hand-values.md";

    pub fn package() -> &'static SheetRulePackage {
        live_sheet_rules_for(GameSystem::Starfinder1e).expect("the Starfinder package loads (data/starfinder-1e/sheet_rules)")
    }

    /// One hand value: a number, or "untrained (trained only)" -- no total printed.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Hand {
        Value(i64),
        Untrained,
    }

    /// `(seed, field) -> value` from every `| SF-<seed> | <field> | <value> | <source> |` row of
    /// `files`. A row without an `https://` source fails.
    pub fn hand_values() -> BTreeMap<(String, String), Hand> {
        let mut out = BTreeMap::new();
        for rel in [HAND_VALUES, INITIATIVE_HAND_VALUES] {
            let path = crate::support::paths::repo_root().join(rel);
            let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            for line in text.lines() {
                if !line.starts_with('|') {
                    continue;
                }
                let cells: Vec<&str> = line.split('|').map(str::trim).collect();
                if cells.len() < 6 || !cells[1].starts_with("SF-") {
                    continue;
                }
                assert!(cells[4].contains("https://"), "fixture row without a source URL: {line}");
                let value = if cells[3].starts_with("untrained") {
                    Hand::Untrained
                } else {
                    match cells[3].replace('\u{2212}', "-").trim_start_matches('+').parse::<i64>() {
                        Ok(v) => Hand::Value(v),
                        Err(_) => continue,
                    }
                };
                out.insert((cells[1].to_string(), cells[2].to_string()), value);
            }
        }
        out
    }

    fn ranks(pairs: &[(&str, i64)]) -> BTreeMap<String, i64> {
        pairs.iter().map(|(s, n)| (s.to_string(), *n)).collect()
    }

    /// The four seeds, `seed-builds.md` §1–§4: race, class, final scores and key ability (as
    /// E4.1), theme, worn armour, connection / pool picks, skill ranks and skill choices.
    pub fn seeds() -> Vec<(&'static str, SfBuild)> {
        let chassis = |class: &str, level: u8, race: &str, scores: [i64; 6], choice: Option<Ability>| SfChassisBuild {
            classes: vec![(format!("core:class:{class}"), level)],
            race: format!("core:race:{race}"),
            ability_scores: scores,
            key_ability_choice: choice,
        };
        let s = |x: &str| x.to_string();
        vec![
            (
                "SF-Soldier-3",
                SfBuild {
                    chassis: chassis("soldier", 3, "human", [16, 14, 12, 11, 10, 10], Some(Ability::Str)),
                    theme: Some(s("core:ability:mercenary")),
                    armor: Some(s("core:equipment:defiance_series_squad")),
                    picks: vec![],
                    skill_ranks: ranks(&[("athletics", 3), ("intimidate", 3), ("medicine", 3), ("piloting", 3), ("survival", 3)]),
                    choices: BTreeMap::new(),
                },
            ),
            (
                "SF-Mystic-5",
                SfBuild {
                    chassis: chassis("mystic", 5, "lashunta", [10, 14, 8, 14, 19, 15], None),
                    theme: Some(s("core:ability:priest")),
                    armor: Some(s("core:equipment:lashunta_tempweave_basic")),
                    // Empath connection; Lashunta Student's two picks of "+2 Racial Bonus to Skill"
                    // (Diplomacy, Medicine).
                    picks: vec![s("core:ability:empath"), s("core:ability:2_racial_bonus_to_skill")],
                    skill_ranks: ranks(&[
                        ("bluff", 5),
                        ("culture", 5),
                        ("diplomacy", 5),
                        ("life_science", 5),
                        ("medicine", 5),
                        ("mysticism", 5),
                        ("perception", 5),
                        ("sense_motive", 5),
                    ]),
                    choices: BTreeMap::from([(s("core:ability:2_racial_bonus_to_skill"), vec![s("diplomacy"), s("medicine")])]),
                },
            ),
            (
                "SF-Technomancer-5",
                SfBuild {
                    chassis: chassis("technomancer", 5, "android", [10, 16, 14, 19, 13, 8], None),
                    theme: Some(s("core:ability:scholar")),
                    armor: Some(s("core:equipment:d_suit_i")),
                    picks: vec![],
                    skill_ranks: ranks(&[
                        ("computers", 5),
                        ("engineering", 5),
                        ("life_science", 5),
                        ("mysticism", 5),
                        ("physical_science", 5),
                        ("piloting", 5),
                        ("sleight_of_hand", 5),
                        ("perception", 5),
                    ]),
                    // Scholar theme knowledge: Physical Science chosen (seed-builds.md §3).
                    choices: BTreeMap::from([(
                        s("core:ability:scholar_theme_benefit_theme_knowledge"),
                        vec![s("core:pool_option:scholar_theme_chosen_skill_physical_science")],
                    )]),
                },
            ),
            (
                "SF-Envoy-3",
                SfBuild {
                    chassis: chassis("envoy", 3, "ysoki", [8, 13, 12, 12, 10, 18], None),
                    theme: Some(s("core:ability:icon")),
                    armor: Some(s("core:equipment:carbon_skin_graphite")),
                    picks: vec![],
                    skill_ranks: ranks(&[
                        ("bluff", 3),
                        ("computers", 3),
                        ("culture", 3),
                        ("diplomacy", 3),
                        ("engineering", 3),
                        ("intimidate", 3),
                        ("perception", 3),
                        ("sense_motive", 3),
                        ("stealth", 3),
                    ]),
                    choices: BTreeMap::new(),
                },
            ),
        ]
    }
}

#[cfg(test)]
mod sf_seed {
    use super::seed_support::*;
    use super::*;

    #[test]
    fn sf_seed_eac_kac_initiative_match_the_srd_hand_values() {
        let hand = hand_values();
        let mut bad = Vec::new();
        let mut checked = 0;
        for (seed, build) in seeds() {
            let d = compute(package(), &build).unwrap_or_else(|r| panic!("{seed}: {r:?}"));
            for (field, got) in [("EAC", d.eac.total), ("KAC", d.kac.total), ("Initiative", d.initiative.total)] {
                let want = hand.get(&(seed.to_string(), field.to_string())).unwrap_or_else(|| panic!("{seed} {field}: no hand value"));
                checked += 1;
                if *want != Hand::Value(got) {
                    bad.push(format!("{seed} {field}: engine {got}, SRD {want:?} -- terms {:?}", [&d.eac, &d.kac, &d.initiative]));
                }
            }
        }
        assert_eq!(checked, 12, "4 seeds x EAC, KAC, initiative");
        assert!(bad.is_empty(), "{} of 12 mismatch:\n{}", bad.len(), bad.join("\n"));
    }

    /// The soldier's max-Dex cap binds (`decisions.md §9`): Dex +2, Defiance Series Squad max +1.
    #[test]
    fn sf_seed_the_soldier_armour_caps_dex() {
        let (_, soldier) = seeds().remove(0);
        let d = compute(package(), &soldier).unwrap();
        let dex = d.eac.terms.iter().find(|t| t.label.starts_with("Dexterity")).unwrap();
        assert_eq!(dex.value, 1, "{:?}", d.eac);
        assert_eq!(d.armor_check_penalty, -4);
        let mut unarmored = soldier.clone();
        unarmored.armor = None;
        let d = compute(package(), &unarmored).unwrap();
        assert_eq!((d.eac.total, d.kac.total, d.armor_check_penalty), (12, 12, 0), "10 + Dex 2, no armour");
    }

    /// Every armour term is read from the armour record the build wears.
    #[test]
    fn sf_seed_every_ac_term_names_its_source() {
        for (seed, build) in seeds() {
            let d = compute(package(), &build).unwrap();
            let armor = build.armor.clone().unwrap();
            for total in [&d.eac, &d.kac] {
                assert!(total.terms.iter().any(|t| t.source.starts_with(&armor) && t.label != "base" && !t.label.starts_with("Dexterity")), "{seed}: {total:?}");
                for t in &total.terms {
                    assert!(t.source.starts_with("core:") || t.source.starts_with("SRD "), "{seed}: {t:?}");
                }
            }
        }
    }

    #[test]
    fn sf_seed_a_record_that_is_not_armour_is_refused() {
        let (_, mut soldier) = seeds().remove(0);
        soldier.armor = Some("core:class:soldier".into());
        assert_eq!(compute(package(), &soldier).unwrap_err().id, REFUSED_ARMOR_NOT_ARMOR);
        soldier.armor = Some("core:equipment:no_such_armour".into());
        assert_eq!(compute(package(), &soldier).unwrap_err().id, REFUSED_NOT_HELD);
    }
}
