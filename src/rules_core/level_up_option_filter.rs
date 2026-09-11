//! The per-character level-up option filter -- the join over [`SheetRule::applies`].
//!
//! # Why this module exists
//!
//! SD-34 `decisions.md §17` states the operator requirement directly: at level-up the backend
//! filters the valid options for **this** character, rather than handing the player the whole
//! pool and letting them discover at save time that they never qualified. Until this module
//! landed nothing did that join -- `level_up::PickList::candidates` was empty at every one of
//! the eleven per-class modules, and `character_hub::preview_level_up`'s own test pinned the
//! absence: "composing a real candidate list needs PF1 Combat-Feat eligibility filtering plus
//! per-candidate prerequisite evaluation".
//!
//! # The join
//!
//! Prerequisites are [`Applies`], converted from the source's `PRE*` rows at ingest. There is
//! no token on this side: this module reads `SheetRule.applies` and calls the same
//! [`evaluate_applies`] the sheet renderer calls, against the same [`HeldSet`] and
//! [`CharacterFacts`] the sheet is rendered from. One evaluator, several consumers.
//!
//! # What the player sees, under the sheet rule
//!
//! `decisions.md §1`: a rendered sheet line is one final number, dice in final form, or the
//! rule's words. An option whose gate excludes it does not silently vanish -- it moves to
//! [`FilteredOptionPool::refused`] carrying **the words of the requirement it failed**, so the
//! sheet can print "Mobility -- requires Dodge" rather than showing a shorter list for reasons
//! the player cannot see. A term this module cannot put into words is still counted and still
//! named by its option; it is never dropped and never turned into an exemption.
//!
//! A [`Gate::Situational`] option is **eligible**, and its condition prints on the line: that
//! is what `Situational` means everywhere else in the evaluator, and a condition the player
//! resolves at the table is not a prerequisite the engine can refuse.

use crate::rules_core::sheet_rule::{
    evaluate_applies, split_rule_id, Ability, Applies, CharacterFacts, ClassRef, Cmp, EvalContext,
    Expr, Gate, HeldFilter, HeldSet, Holdable, PoolId, ProfRef, RuleId, Save, SheetRule,
    SheetRulePackage, SpellKind, Tag,
};

/// An option this character may take, with the condition that prints on the line when the
/// gate included it situationally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EligibleOption {
    pub id: RuleId,
    pub label: String,
    /// `Some` only for a [`Gate::Situational`] include -- the condition, in the rule's words.
    pub condition: Option<String>,
}

/// An option this character may not take, and the requirement it failed, in words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusedOption {
    pub id: RuleId,
    pub label: String,
    /// The failing requirement, in the rule's words (`"requires Dodge"`,
    /// `"base attack bonus at least 6"`). Never empty.
    pub unmet: String,
}

/// One option pool, split by this character's own prerequisites.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FilteredOptionPool {
    pub pool: PoolId,
    /// Sorted by label, then id -- a stable order the sheet can print directly.
    pub eligible: Vec<EligibleOption>,
    /// Sorted the same way. A count here is a number to report, never an exemption.
    pub refused: Vec<RefusedOption>,
}

impl FilteredOptionPool {
    /// Every option considered, eligible or not.
    pub fn considered(&self) -> usize {
        self.eligible.len() + self.refused.len()
    }
}

/// The pool id every character's own feat selections come from.
pub const FEAT_POOL: &str = "feat";

/// Split `pool`'s options into the ones this character qualifies for and the ones it does not.
///
/// `tags` narrows the pool the way the source's own record-type narrowing does (e.g. `["Combat"]`
/// for a fighter bonus feat); an empty `tags` is the whole pool. An option the character
/// already holds is left out of both lists unless the rule is `repeatable` -- it is not a
/// choice on offer, and it is not a refusal either.
pub fn filter_option_pool(
    package: &SheetRulePackage,
    held: &HeldSet,
    facts: &CharacterFacts,
    pool: &str,
    tags: &[Tag],
) -> FilteredOptionPool {
    let mut out = FilteredOptionPool { pool: pool.to_owned(), ..FilteredOptionPool::default() };
    for rule in package.rules.values() {
        if !is_offerable(rule, pool, tags) {
            continue;
        }
        if !rule.repeatable && held.holds(&rule.id) {
            continue;
        }
        match evaluate_applies(&rule.applies, held, package, facts, EvalContext::default()) {
            Gate::Include => out.eligible.push(EligibleOption {
                id: rule.id.clone(),
                label: rule.label.clone(),
                condition: None,
            }),
            Gate::Situational(condition) => out.eligible.push(EligibleOption {
                id: rule.id.clone(),
                label: rule.label.clone(),
                condition: Some(condition),
            }),
            Gate::Exclude => out.refused.push(RefusedOption {
                id: rule.id.clone(),
                label: rule.label.clone(),
                unmet: unmet_words(package, held, facts, &rule.applies),
            }),
        }
    }
    out.eligible.sort_by(|a, b| (&a.label, &a.id).cmp(&(&b.label, &b.id)));
    out.refused.sort_by(|a, b| (&a.label, &a.id).cmp(&(&b.label, &b.id)));
    out
}

/// A record is on offer when it sits in the pool, is a principal record rather than one of a
/// record file's `#`-suffixed siblings, carries every narrowing tag, and is a character rule
/// rather than an item's.
fn is_offerable(rule: &SheetRule, pool: &str, tags: &[Tag]) -> bool {
    if rule.pool != pool || rule.id.contains('#') || rule.label.is_empty() {
        return false;
    }
    tags.iter().all(|want| rule.tags.iter().any(|have| have.eq_ignore_ascii_case(want)))
}

/// The failing half of a gate, in words. Descends `All` to the terms that actually failed so
/// the line names the one missing prerequisite rather than restating the whole tree.
pub fn unmet_words(
    package: &SheetRulePackage,
    held: &HeldSet,
    facts: &CharacterFacts,
    gate: &Applies,
) -> String {
    let mut terms = Vec::new();
    collect_unmet(package, held, facts, gate, &mut terms);
    terms.dedup();
    if terms.is_empty() {
        // Reached only if the gate excluded without any single term failing -- report the
        // whole gate rather than an empty line.
        return describe_gate(package, gate);
    }
    terms.join("; ")
}

fn collect_unmet(
    package: &SheetRulePackage,
    held: &HeldSet,
    facts: &CharacterFacts,
    gate: &Applies,
    out: &mut Vec<String>,
) {
    match gate {
        Applies::All(terms) => {
            for term in terms {
                if !evaluate_applies(term, held, package, facts, EvalContext::default()).includes()
                {
                    collect_unmet(package, held, facts, term, out);
                }
            }
        }
        other => out.push(describe_gate(package, other)),
    }
}

/// One gate in the rule's own words.
pub fn describe_gate(package: &SheetRulePackage, gate: &Applies) -> String {
    match gate {
        Applies::Always => "no prerequisite".to_owned(),
        Applies::Never => "not available to a character".to_owned(),
        Applies::All(terms) => {
            join(terms.iter().map(|t| describe_gate(package, t)).collect(), " and ")
        }
        Applies::AtLeast { n, of } => format!(
            "at least {n} of: {}",
            join(of.iter().map(|t| describe_gate(package, t)).collect(), ", ")
        ),
        Applies::Not(inner) => format!("not {}", describe_gate(package, inner)),
        Applies::Compare { lhs, op, rhs } => format!(
            "{} {} {}",
            describe_expr(package, lhs),
            describe_cmp(*op),
            describe_expr(package, rhs)
        ),
        Applies::Holds { what, count } => describe_holdable(package, what, *count),
        Applies::Chosen { choice, option } => match option {
            Some(option) => format!("requires {} chosen for {}", pretty(option), pretty(choice)),
            None => format!("requires a choice made for {}", pretty(choice)),
        },
        Applies::ItemHas { tags, n } => {
            format!("an item with at least {n} of: {}", join(tags.clone(), ", "))
        }
        Applies::Situational { text } => text.clone(),
    }
}

fn describe_holdable(package: &SheetRulePackage, what: &Holdable, count: u8) -> String {
    let times = if count > 1 { format!(" {count} times") } else { String::new() };
    match what {
        Holdable::Rule(id) | Holdable::Template(id) | Holdable::Spell(id) => {
            format!("requires {}{times}", label_of(package, id))
        }
        Holdable::RuleTag { pool, tag } => {
            format!("requires {count} {} from {}", pretty(tag), pretty(pool))
        }
        Holdable::MissingRule { pool, name } => {
            format!("requires {name} from {} (no record in the corpus)", pretty(pool))
        }
        Holdable::Race(race) => format!("requires the {} race", pretty(race)),
        Holdable::RaceType(tag) => format!("requires a {} creature", pretty(tag)),
        Holdable::RaceSubtype(tag) => format!("requires the {} subtype", pretty(tag)),
        Holdable::Alignment(list) => format!("requires alignment {}", join(list.clone(), " or ")),
        Holdable::AlignmentMatchesDeity => "requires an alignment matching the deity".to_owned(),
        Holdable::Deity(_) => "requires a specific deity".to_owned(),
        Holdable::DeityInPantheon(p) => format!("requires a deity of the {p} pantheon"),
        Holdable::DeityGrantsDomain(id) => {
            format!("requires a deity granting {}", label_of(package, id))
        }
        Holdable::DeityAlignment(list) => {
            format!("requires a deity of alignment {}", join(list.clone(), " or "))
        }
        Holdable::ClassSkill(skill) => format!("requires {} as a class skill", pretty(skill)),
        Holdable::Proficiency(prof) => format!("requires proficiency with {}", describe_prof(prof)),
        Holdable::Language(language) => format!("requires the {language} language"),
        Holdable::Movement { mode, min } => format!("requires a {mode} speed of {min} feet"),
        Holdable::Vision(tag) => format!("requires {}", pretty(tag)),
        Holdable::ClassTag(tag) => format!("requires a {} class", pretty(tag)),
        Holdable::Gender(tag) => format!("requires a {} character", pretty(tag)),
        Holdable::AgeCategory(tag) => format!("requires a {} character", pretty(tag)),
        Holdable::Fact { name, value } => format!("requires {} to be {value}", pretty(name)),
    }
}

fn describe_prof(prof: &ProfRef) -> String {
    match prof {
        ProfRef::Weapon(weapon) => pretty(weapon),
        ProfRef::WeaponGroup(tag) | ProfRef::WeaponTag(tag) => format!("{} weapons", pretty(tag)),
        ProfRef::ArmorGroup(tag) => format!("{} armor", pretty(tag)),
        ProfRef::ShieldGroup(tag) => format!("{} shields", pretty(tag)),
        ProfRef::DeityFavoredWeapon => "the deity's favored weapon".to_owned(),
        ProfRef::Chosen(choice) => format!("the weapon chosen for {}", pretty(choice)),
    }
}

fn describe_cmp(op: Cmp) -> &'static str {
    match op {
        Cmp::Eq => "exactly",
        Cmp::Ne => "other than",
        Cmp::Lt => "below",
        Cmp::Lte => "at most",
        Cmp::Gt => "above",
        Cmp::Gte => "at least",
    }
}

fn describe_expr(package: &SheetRulePackage, expr: &Expr) -> String {
    match expr {
        Expr::Const(n) => n.to_string(),
        Expr::Level => "character level".to_owned(),
        Expr::ClassLevel(class) => format!("{} level", pretty(class)),
        Expr::CasterLevel(ClassRef::Class(class)) => format!("{} caster level", pretty(class)),
        Expr::CasterLevel(ClassRef::Holder) => "caster level".to_owned(),
        Expr::SpellLevel => "spell level".to_owned(),
        Expr::AbilityMod(ability) => format!("{} modifier", ability_word(*ability)),
        Expr::AbilityScore(ability) => ability_word(*ability).to_owned(),
        Expr::Size => "size".to_owned(),
        Expr::BaseSize => "base size".to_owned(),
        Expr::SizeMod => "size modifier".to_owned(),
        Expr::HitDice => "hit dice".to_owned(),
        Expr::BaseAttack => "base attack bonus".to_owned(),
        Expr::BaseSave(save) => format!("base {} save", save_word(*save)),
        Expr::SkillRanks(skill) => format!("{} ranks", pretty(skill)),
        Expr::SkillTotal(skill) => format!("{} bonus", pretty(skill)),
        Expr::HeldCount { pool, filter } => match filter {
            HeldFilter::Any => format!("{} held", pretty(pool)),
            HeldFilter::Tag(tag) => format!("{} {} held", pretty(tag), pretty(pool)),
            HeldFilter::Rule(id) => format!("{} held", label_of(package, id)),
        },
        Expr::ChallengeRating => "challenge rating".to_owned(),
        Expr::Speed(mode) => format!("{mode} speed"),
        Expr::HighestSpellLevel(kind) => match kind {
            SpellKind::Any => "highest spell level".to_owned(),
            SpellKind::Arcane => "highest arcane spell level".to_owned(),
            SpellKind::Divine => "highest divine spell level".to_owned(),
            SpellKind::Psychic => "highest psychic spell level".to_owned(),
        },
        Expr::MasterLevel => "the master's level".to_owned(),
        Expr::MasterVar(_) => "a value from the master".to_owned(),
        // A corpus variable's id is a content hash, never a word -- naming it would put a
        // token on the sheet. The gate is still reported; only this leaf stays generic.
        Expr::Var(_) => "a rules variable".to_owned(),
        Expr::Sum(terms) => {
            join(terms.iter().map(|t| describe_expr(package, t)).collect(), " plus ")
        }
        Expr::Mul(a, b) => {
            format!("{} times {}", describe_expr(package, a), describe_expr(package, b))
        }
        Expr::Div(a, b) => {
            format!("{} divided by {}", describe_expr(package, a), describe_expr(package, b))
        }
        Expr::Min(a, b) => {
            format!("the lower of {} and {}", describe_expr(package, a), describe_expr(package, b))
        }
        Expr::Max(a, b) => {
            format!("the higher of {} and {}", describe_expr(package, a), describe_expr(package, b))
        }
        Expr::Floor(inner) | Expr::Ceil(inner) => describe_expr(package, inner),
        Expr::Choice(choice) => format!("the choice made for {}", pretty(choice)),
    }
}

fn ability_word(ability: Ability) -> &'static str {
    match ability {
        Ability::Str => "Strength",
        Ability::Dex => "Dexterity",
        Ability::Con => "Constitution",
        Ability::Int => "Intelligence",
        Ability::Wis => "Wisdom",
        Ability::Cha => "Charisma",
    }
}

fn save_word(save: Save) -> &'static str {
    match save {
        Save::Fortitude => "Fortitude",
        Save::Reflex => "Reflex",
        Save::Will => "Will",
    }
}

/// The referenced rule's own display label when the package carries it, else its slug in
/// words. Never the raw id: an id is a locator, not something a player reads.
fn label_of(package: &SheetRulePackage, id: &str) -> String {
    match package.rule(id) {
        Some(rule) if !rule.label.is_empty() => rule.label.clone(),
        _ => pretty(split_rule_id(id).2),
    }
}

/// One converted [`Expr`] in the rule's own words -- [`describe_expr`] for callers outside
/// this module.
///
/// SD-35 `AT-35-E6-003`: `sheet_rule_catalog` renders a rule's prose with no character in
/// hand, where a slot standing on a character term has no number and must print words. That
/// vocabulary already exists here, is already the one the refusal lines use, and is pinned by
/// this module's own tests -- a second describer would be a second place for the sheet's words
/// to drift.
pub fn expr_words(package: &SheetRulePackage, expr: &Expr) -> String {
    describe_expr(package, expr)
}

/// A slug or an internal id as words -- [`pretty`] for callers outside this module.
pub fn words_of_id(value: &str) -> String {
    pretty(value)
}

/// A slug or an internal id as words: `"power_attack"` -> `"power attack"`.
fn pretty(value: &str) -> String {
    let words = value.replace(['_', '-'], " ");
    let words = words.trim();
    if words.is_empty() { value.to_owned() } else { words.to_owned() }
}

fn join(parts: Vec<String>, sep: &str) -> String {
    parts.join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::sheet_rule::{
        held_set, Applies, Cmp, Expr, HeldSeed, Provenance, SheetRule, SheetValue, Subject,
    };

    fn option_rule(slug: &str, label: &str, applies: Applies) -> SheetRule {
        SheetRule {
            id: format!("core_rulebook:feat:{slug}"),
            label: label.to_owned(),
            value: SheetValue::Text,
            also: Vec::new(),
            prose: Vec::new(),
            applies,
            target: None,
            bonus_type: None,
            print: true,
            pool: FEAT_POOL.to_owned(),
            tags: Vec::new(),
            subject: Subject::Character,
            repeatable: false,
            granted_by: Vec::new(),
            offers: None,
            grants: Vec::new(),
            provenance: Provenance::default(),
        }
    }

    /// The package under test: one ungated option, one gated on an ability score, one gated on
    /// holding another option.
    fn package() -> SheetRulePackage {
        let mut package = SheetRulePackage::new();
        package.insert_rule(option_rule("improved_initiative", "Improved Initiative", Applies::Always));
        package.insert_rule(option_rule(
            "power_attack",
            "Power Attack",
            Applies::Compare {
                lhs: Expr::AbilityScore(Ability::Str),
                op: Cmp::Gte,
                rhs: Expr::Const(13),
            },
        ));
        package.insert_rule(option_rule(
            "mobility",
            "Mobility",
            Applies::Holds {
                what: Holdable::Rule("core_rulebook:feat:dodge".to_owned()),
                count: 1,
            },
        ));
        package.insert_rule(option_rule("dodge", "Dodge", Applies::Always));
        package.finish();
        package
    }

    fn facts_with_strength(strength: i64) -> CharacterFacts {
        let mut facts = CharacterFacts { level: 3, ..CharacterFacts::default() };
        facts.ability_scores[0] = strength;
        facts
    }

    #[test]
    fn an_option_whose_prerequisite_is_unmet_is_refused_with_the_requirement_in_words() {
        let package = package();
        let facts = facts_with_strength(16);
        let held = held_set(&package, &HeldSeed::default(), &facts);

        let filtered = filter_option_pool(&package, &held, &facts, FEAT_POOL, &[]);

        let mobility = filtered
            .refused
            .iter()
            .find(|option| option.label == "Mobility")
            .expect("Mobility must be refused: this character does not hold Dodge");
        assert_eq!(mobility.unmet, "requires Dodge");
        assert!(
            !filtered.eligible.iter().any(|option| option.label == "Mobility"),
            "a refused option must not also be offered"
        );
    }

    #[test]
    fn an_option_whose_prerequisite_is_met_is_offered() {
        let package = package();
        let facts = facts_with_strength(16);
        let held = held_set(&package, &HeldSeed::default(), &facts);

        let filtered = filter_option_pool(&package, &held, &facts, FEAT_POOL, &[]);

        let labels: Vec<&str> = filtered.eligible.iter().map(|o| o.label.as_str()).collect();
        assert!(labels.contains(&"Improved Initiative"), "ungated option must be offered: {labels:?}");
        assert!(
            labels.contains(&"Power Attack"),
            "Strength 16 meets Power Attack's Strength 13: {labels:?}"
        );
    }

    /// The same ability gate, one point short, moves the option to the refused list carrying
    /// the number the player has to reach -- not a silent disappearance.
    #[test]
    fn an_ability_score_one_point_short_refuses_and_names_the_score() {
        let package = package();
        let facts = facts_with_strength(12);
        let held = held_set(&package, &HeldSeed::default(), &facts);

        let filtered = filter_option_pool(&package, &held, &facts, FEAT_POOL, &[]);

        let power_attack = filtered
            .refused
            .iter()
            .find(|option| option.label == "Power Attack")
            .expect("Strength 12 must refuse Power Attack");
        assert_eq!(power_attack.unmet, "Strength at least 13");
    }

    /// Every option lands in exactly one of the two lists: a refusal is a number to report,
    /// never a record that quietly leaves the pool.
    #[test]
    fn every_pool_option_is_either_offered_or_refused() {
        let package = package();
        let facts = facts_with_strength(16);
        let held = held_set(&package, &HeldSeed::default(), &facts);

        let filtered = filter_option_pool(&package, &held, &facts, FEAT_POOL, &[]);

        assert_eq!(filtered.considered(), 4, "all four pool records must be accounted for");
        assert_eq!(filtered.pool, FEAT_POOL);
    }

    /// A `Situational` gate includes and prints its condition -- it is a condition the player
    /// resolves at the table, not a prerequisite the engine can refuse.
    #[test]
    fn a_situational_gate_is_offered_and_carries_its_condition() {
        let mut package = SheetRulePackage::new();
        package.insert_rule(option_rule(
            "shield_stance",
            "Shield Stance",
            Applies::Situational { text: "while wielding a shield".to_owned() },
        ));
        package.finish();
        let facts = facts_with_strength(16);
        let held = held_set(&package, &HeldSeed::default(), &facts);

        let filtered = filter_option_pool(&package, &held, &facts, FEAT_POOL, &[]);

        assert_eq!(filtered.refused.len(), 0);
        assert_eq!(
            filtered.eligible[0].condition.as_deref(),
            Some("while wielding a shield")
        );
    }

    /// `All` reports only the terms that actually failed -- the line names the one missing
    /// prerequisite rather than restating a gate the character already half-satisfies.
    #[test]
    fn a_compound_gate_names_only_the_failing_term() {
        let mut package = SheetRulePackage::new();
        package.insert_rule(option_rule("dodge", "Dodge", Applies::Always));
        package.insert_rule(option_rule(
            "greater_mobility",
            "Greater Mobility",
            Applies::All(vec![
                Applies::Compare {
                    lhs: Expr::AbilityScore(Ability::Str),
                    op: Cmp::Gte,
                    rhs: Expr::Const(13),
                },
                Applies::Holds {
                    what: Holdable::Rule("core_rulebook:feat:dodge".to_owned()),
                    count: 1,
                },
            ]),
        ));
        package.finish();
        let facts = facts_with_strength(16);
        let held = held_set(&package, &HeldSeed::default(), &facts);

        let filtered = filter_option_pool(&package, &held, &facts, FEAT_POOL, &[]);

        let greater = filtered
            .refused
            .iter()
            .find(|option| option.label == "Greater Mobility")
            .expect("the Dodge half is unmet");
        assert_eq!(
            greater.unmet, "requires Dodge",
            "the met Strength half must not be restated"
        );
    }

    /// A narrowing tag bounds the pool the way the source's own record-type narrowing does.
    #[test]
    fn a_narrowing_tag_bounds_the_pool() {
        let mut package = SheetRulePackage::new();
        let mut combat = option_rule("power_attack", "Power Attack", Applies::Always);
        combat.tags = vec!["Combat".to_owned()];
        package.insert_rule(combat);
        package.insert_rule(option_rule("skill_focus", "Skill Focus", Applies::Always));
        package.finish();
        let facts = facts_with_strength(16);
        let held = held_set(&package, &HeldSeed::default(), &facts);

        let filtered =
            filter_option_pool(&package, &held, &facts, FEAT_POOL, &["combat".to_owned()]);

        assert_eq!(filtered.considered(), 1);
        assert_eq!(filtered.eligible[0].label, "Power Attack");
    }
}
