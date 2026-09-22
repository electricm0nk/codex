//! A class's weapon proficiency, read from the CONVERTED rule package
//! (`data/sheet_rules/`) -- SD-36 Epic F `epic-f-class-completion.md` §3.4 (the reader), §3.4a
//! (the package handle it loads through), review findings 1, 5 and 15.
//!
//! # What this is for
//!
//! `rules_tables::crb::weapon_tables::CLASS_WEAPON_PROFICIENCIES` is 42 hand-typed rows.
//! Ruling 7 keeps those rows in Rust until Starfinder; it does not require new data to be
//! authored as Rust. Every class WITHOUT a static row is answered here instead, from the grants
//! the converter already wrote onto the class's own rules -- never from a new hand-typed row.
//!
//! # Mechanism (one rule, no per-class cases)
//!
//! For `(class slug, class level)`:
//!
//! 1. Load the package through [`sheet_rule_package::package`] (a named `Err` is
//!    [`ProficiencyAnswer::Unknown`], never an empty answer).
//! 2. Seed [`HeldSeed`] with that one class at that level -- the class-level subset of the seed
//!    the desktop builds with `HeldSeed::from_character` -- and run [`held_set`], which already
//!    walks `Granter::Class { at_level }` and `Granter::Rule` edges from the class's principal
//!    rule to a fixpoint. The fixpoint is reused, never re-implemented.
//! 3. Collect every `Fact::Proficiency` a held rule grants. `held_set` itself folds only
//!    `FactDeclare` / `CountsAs` / `Waives` / `Revokes`, never `FactGrant`, which is why the
//!    reader collects proficiency facts itself (§3.4 "Note from the code").
//! 4. A gated grant (`Effect::GatedFactGrant`, review finding 1) counts only when its gate is
//!    decidable from class-level facts ([`gate_is_class_decidable`]) and evaluates true;
//!    decidable and false is skipped; anything else (a race, a feat, a deity, a record the
//!    corpus does not carry, a situational clause) is NOT counted and its words go to
//!    `printed_conditions` -- the paper sheet prints the rule, it does not guess.
//!
//! # Unknown, never "proficient with nothing"
//!
//! The answer is [`ProficiencyAnswer::Unknown`] with its reason when:
//! - the package cannot be loaded, or carries no converted record for the class;
//! - the walk reaches no weapon-proficiency grant at all: §3.4 allows an empty answer only when
//!   the converter attests the closure complete, and the package carries no such attestation;
//! - the walk meets a [`Holdable::MissingRule`] on a grant edge leading to a
//!   weapon-proficiency grant: the closure is incomplete there, so the reader does not guess
//!   which way that edge goes;
//! - a weapon `ProfRef` names a tag that matches none of: a tier (`Simple`/`Martial`/`Exotic`),
//!   a `Weapon Group <x>` tag some converted equipment rule carries, a `WeaponAllOf` whose every
//!   conjunct the weapon table answers (tier, `Melee`, `Ranged`), or a `WeaponSet` with members
//!   (review finding 15 -- a bare `Auto` reaching the reader as a tag is Unknown; the converter
//!   resolves the real `TYPE=Auto` selector to a `WeaponSet` at ingest).
//!
//! Deterministic; no I/O beyond the package handle.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

use crate::rules_core::level_up_option_filter::{describe_gate, describe_prof};
use crate::rules_core::rules_tables::crb::weapon_tables::WeaponProficiency;
use crate::rules_core::sheet_rule::{
    held_set, resolve_gated_fact_grant, split_rule_id, Applies, CharacterFacts, Effect, EvalContext, Expr, Fact,
    GatedFact, Granter, HeldSeed, HeldSet, Holdable, OptionSet, ProfRef, RuleId, SheetRule, SheetRulePackage,
};
use crate::rules_core::sheet_rule_package;

/// One `ProfRef::WeaponSet` as the reader returns it: the selector's own label (display only)
/// and the literal member list the converter resolved at ingest.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeaponSetView {
    pub label: String,
    pub members: Vec<String>,
}

/// What the converted record says one class is proficient with at one level.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassWeaponProficiencyView {
    /// Blanket tiers, in `Simple`, `Martial`, `Exotic` order, each at most once.
    pub tiers: Vec<WeaponProficiency>,
    /// Individually named weapon proficiencies (`ProfRef::Weapon`), by their proficiency name.
    pub named: BTreeSet<String>,
    /// Whole weapon groups, by the group's own name (`"Close"` for `Weapon Group Close`).
    pub groups: BTreeSet<String>,
    /// Conjunctive selectors the weapon table answers directly (`["Martial", "Ranged"]`).
    pub all_of: BTreeSet<Vec<String>>,
    /// Membership selectors, expanded at ingest.
    pub sets: Vec<WeaponSetView>,
    /// A grant the class-level facts cannot decide: printed with its condition, never counted.
    pub printed_conditions: Vec<String>,
}

/// The reader's answer: known (possibly with printed conditions) or unknown with its reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProficiencyAnswer {
    Known(ClassWeaponProficiencyView),
    Unknown { reason: String },
}

impl ProficiencyAnswer {
    pub fn known(&self) -> Option<&ClassWeaponProficiencyView> {
        match self {
            ProficiencyAnswer::Known(view) => Some(view),
            ProficiencyAnswer::Unknown { .. } => None,
        }
    }
}

/// The class's weapon proficiency at `class_level`, read from the process-wide converted
/// package ([`sheet_rule_package::package`]). Cached per `(class, level)`.
pub fn class_weapon_proficiency_view(class_slug: &str, class_level: u8) -> ProficiencyAnswer {
    static CACHE: OnceLock<Mutex<BTreeMap<(String, u8), ProficiencyAnswer>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    let key = (class_slug.to_string(), class_level);
    if let Ok(guard) = cache.lock()
        && let Some(answer) = guard.get(&key)
    {
        return answer.clone();
    }
    let answer = match sheet_rule_package::package() {
        Ok(package) => class_weapon_proficiency_view_in(package, class_slug, class_level),
        Err(reason) => ProficiencyAnswer::Unknown { reason: format!("the converted rule package did not load: {reason}") },
    };
    if let Ok(mut guard) = cache.lock() {
        guard.insert(key, answer.clone());
    }
    answer
}

/// The same answer over an explicit package -- the pure form, for a caller (or a fixture)
/// that already holds one.
pub fn class_weapon_proficiency_view_in(package: &SheetRulePackage, class_slug: &str, class_level: u8) -> ProficiencyAnswer {
    let Some(principal) = package.find("class", class_slug) else {
        return ProficiencyAnswer::Unknown { reason: format!("no converted class record for `{class_slug}`") };
    };
    let level = i64::from(class_level);
    let seed = HeldSeed { classes: vec![(class_slug.to_string(), level)], ..HeldSeed::default() };
    let facts = CharacterFacts { level, class_levels: vec![(class_slug.to_string(), level)], ..CharacterFacts::default() };
    let held = held_set(package, &seed, &facts);
    if !held.rules.contains_key(principal) {
        return ProficiencyAnswer::Unknown { reason: format!("the class principal rule {principal} is not held at level {class_level}") };
    }

    if let Some(reason) = missing_rule_on_a_proficiency_edge(package, &held, class_slug, class_level)
        .or_else(|| unadmitted_class_line_grant(package, &held, class_slug, class_level))
    {
        return ProficiencyAnswer::Unknown { reason };
    }

    let mut acc = Accumulator::default();
    let mut equipment_groups: Option<BTreeSet<String>> = None;
    for (id, entry) in &held.rules {
        if held.removed.contains(id) {
            continue;
        }
        let Some(rule) = package.rule(id) else { continue };
        let ctx = EvalContext { holder_class: entry.holder_class.clone(), ..EvalContext::default() };
        for effect in &rule.grants {
            let (fact, when) = match effect {
                Effect::FactGrant(fact) => (fact, None),
                Effect::GatedFactGrant { fact, when } => (fact, Some(when)),
                _ => continue,
            };
            let Some(words) = weapon_fact_words(rule, fact) else { continue };
            if let Fact::Proficiency(prof) = fact
                && let Err(reason) = recognize(package, prof, &mut equipment_groups)
            {
                return ProficiencyAnswer::Unknown { reason: format!("{id}: {reason}") };
            }
            let Some(when) = when else {
                acc.fold(fact, &words, None);
                continue;
            };
            if !gate_is_class_decidable(when) {
                acc.fold(fact, &words, Some(describe_gate(package, when)));
                continue;
            }
            match resolve_gated_fact_grant(fact, when, package, &held, &facts, ctx.clone()) {
                GatedFact::Granted(fact) => acc.fold(&fact, &words, None),
                GatedFact::Conditional { condition, .. } => acc.fold(fact, &words, Some(condition)),
                GatedFact::Excluded => {}
            }
        }
    }
    let view = acc.finish();
    if view.tiers.is_empty()
        && view.named.is_empty()
        && view.groups.is_empty()
        && view.all_of.is_empty()
        && view.sets.is_empty()
        && view.printed_conditions.is_empty()
    {
        // §3.4 "Known-empty vs unknown": an empty answer is only honest when the converter has
        // attested the closure complete, and the package carries no such attestation -- so an
        // empty walk is a closure the converter did not finish, never "proficient with nothing".
        return ProficiencyAnswer::Unknown {
            reason: format!(
                "the converted closure of `{class_slug}` at level {class_level} grants no weapon proficiency, and the \
                 package carries no closure-complete attestation that none is owed"
            ),
        };
    }
    ProficiencyAnswer::Known(view)
}

/// `Some(words)` when `fact` is a WEAPON proficiency fact (armor and shield proficiencies are
/// not this reader's business), or a player choice this rule offers over weapons.
fn weapon_fact_words(rule: &SheetRule, fact: &Fact) -> Option<String> {
    match fact {
        Fact::Proficiency(ProfRef::ArmorGroup(_) | ProfRef::ShieldGroup(_)) => None,
        Fact::Proficiency(prof) => Some(describe_prof(prof)),
        Fact::Chosen(choice) => match &rule.offers {
            Some(offer) if &offer.id == choice && matches!(offer.from, OptionSet::Weapons(_)) => {
                Some(format!("the weapon chosen for {}", rule.label))
            }
            _ => None,
        },
        _ => None,
    }
}

const TIERS: [(&str, WeaponProficiency); 3] =
    [("Simple", WeaponProficiency::Simple), ("Martial", WeaponProficiency::Martial), ("Exotic", WeaponProficiency::Exotic)];

fn tier_of(tag: &str) -> Option<WeaponProficiency> {
    TIERS.iter().find(|(name, _)| *name == tag).map(|(_, tier)| *tier)
}

/// Review finding 15: a weapon `ProfRef` whose tag the live side cannot answer is an
/// `Err` naming the tag, never a fabricated membership.
fn recognize(package: &SheetRulePackage, prof: &ProfRef, equipment_groups: &mut Option<BTreeSet<String>>) -> Result<(), String> {
    match prof {
        ProfRef::WeaponGroup(tag) | ProfRef::WeaponTag(tag) => {
            if tier_of(tag).is_some() {
                return Ok(());
            }
            if tag.starts_with("Weapon Group ") {
                let groups = equipment_groups.get_or_insert_with(|| equipment_group_tags(package));
                if groups.contains(tag.as_str()) {
                    return Ok(());
                }
                return Err(format!("weapon group tag `{tag}` is carried by no converted equipment rule"));
            }
            Err(format!("proficiency tag `{tag}` is neither a tier, a weapon group, nor an expanded weapon set"))
        }
        ProfRef::WeaponAllOf(tags) => match tags.iter().find(|t| tier_of(t).is_none() && *t != "Melee" && *t != "Ranged") {
            Some(tag) => Err(format!("conjunctive selector {tags:?} carries `{tag}`, which the weapon table cannot answer")),
            None if tags.is_empty() => Err("an empty conjunctive weapon selector".to_string()),
            None => Ok(()),
        },
        ProfRef::WeaponSet { label, members } if members.is_empty() => {
            Err(format!("weapon set `{label}` carries no members"))
        }
        _ => Ok(()),
    }
}

fn equipment_group_tags(package: &SheetRulePackage) -> BTreeSet<String> {
    package
        .rules_of_kind("equipment")
        .flat_map(|r| r.tags.iter())
        .filter(|t| t.starts_with("Weapon Group "))
        .cloned()
        .collect()
}

#[derive(Default)]
struct Accumulator {
    view: ClassWeaponProficiencyView,
    sets: BTreeSet<WeaponSetView>,
    printed: BTreeSet<String>,
}

impl Accumulator {
    /// `condition == None`: the fact is granted. `Some(words)`: it is printed, never counted.
    fn fold(&mut self, fact: &Fact, words: &str, condition: Option<String>) {
        if let Some(condition) = condition {
            self.printed.insert(format!("{words} -- only when: {condition}"));
            return;
        }
        let Fact::Proficiency(prof) = fact else {
            // A player's weapon pick is settled at play, never at class level.
            self.printed.insert(format!("{words} -- chosen by the player"));
            return;
        };
        match prof {
            ProfRef::Weapon(name) => {
                self.view.named.insert(name.clone());
            }
            ProfRef::WeaponGroup(tag) | ProfRef::WeaponTag(tag) => match tier_of(tag) {
                Some(tier) => {
                    if !self.view.tiers.contains(&tier) {
                        self.view.tiers.push(tier);
                    }
                }
                None => {
                    let group = tag.strip_prefix("Weapon Group ").unwrap_or(tag);
                    self.view.groups.insert(group.to_string());
                }
            },
            ProfRef::WeaponAllOf(tags) => {
                self.view.all_of.insert(tags.clone());
            }
            ProfRef::WeaponSet { label, members } => {
                self.sets.insert(WeaponSetView { label: label.clone(), members: members.clone() });
            }
            ProfRef::DeityFavoredWeapon => {
                self.printed.insert(format!("{words} -- depends on the character's deity"));
            }
            ProfRef::Chosen(_) => {
                self.printed.insert(format!("{words} -- chosen by the player"));
            }
            ProfRef::ArmorGroup(_) | ProfRef::ShieldGroup(_) => {}
        }
    }

    fn finish(mut self) -> ClassWeaponProficiencyView {
        self.view.tiers.sort_by_key(|t| TIERS.iter().position(|(_, x)| x == t));
        self.view.sets = self.sets.into_iter().collect();
        self.view.printed_conditions = self.printed.into_iter().collect();
        self.view
    }
}

/// Whether `gate` can be decided from class-level facts alone: the class's own level, total
/// level, converted variables folded over the class-held rules, and what the class walk itself
/// holds (a class/class-feature/archetype rule, an archetype tag, a declared fact). The reader
/// answers the UNARCHETYPED class -- the same contract the static table states -- so an
/// archetype is held only if the class walk grants one. Anything about the rest of the
/// character (race, alignment, deity, feats, a skill, a choice, a record the corpus does not
/// carry, a situational clause) is not decidable here.
pub fn gate_is_class_decidable(gate: &Applies) -> bool {
    match gate {
        Applies::Always | Applies::Never => true,
        Applies::All(terms) | Applies::AtLeast { of: terms, .. } => terms.iter().all(gate_is_class_decidable),
        Applies::Not(inner) => gate_is_class_decidable(inner),
        Applies::Compare { lhs, rhs, .. } => expr_is_class_decidable(lhs) && expr_is_class_decidable(rhs),
        Applies::Holds { what, .. } => match what {
            Holdable::Rule(id) => matches!(split_rule_id(id).1, "class" | "class_feature" | "archetype"),
            Holdable::RuleTag { pool, .. } => pool == "archetype",
            Holdable::Fact { .. } => true,
            _ => false,
        },
        Applies::Chosen { .. } | Applies::ItemHas { .. } | Applies::Situational { .. } => false,
    }
}

fn expr_is_class_decidable(e: &Expr) -> bool {
    match e {
        Expr::Const(_) | Expr::Level | Expr::ClassLevel(_) | Expr::Var(_) => true,
        Expr::Sum(terms) => terms.iter().all(expr_is_class_decidable),
        Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
            expr_is_class_decidable(a) && expr_is_class_decidable(b)
        }
        Expr::Floor(a) | Expr::Ceil(a) => expr_is_class_decidable(a),
        _ => false,
    }
}

fn applies_names_a_missing_rule(a: &Applies) -> Option<String> {
    match a {
        Applies::All(terms) | Applies::AtLeast { of: terms, .. } => terms.iter().find_map(applies_names_a_missing_rule),
        Applies::Not(inner) => applies_names_a_missing_rule(inner),
        Applies::Holds { what: Holdable::MissingRule { pool, name }, .. } => Some(format!("{name} ({pool})")),
        _ => None,
    }
}

/// Whether `id`, or anything its `Granter::Rule` edges reach, grants a weapon proficiency.
fn reaches_a_weapon_grant(package: &SheetRulePackage, id: &str, memo: &mut BTreeMap<RuleId, bool>) -> bool {
    let mut stack = vec![id.to_string()];
    let mut seen = BTreeSet::new();
    while let Some(next) = stack.pop() {
        if let Some(known) = memo.get(&next) {
            if *known {
                memo.insert(id.to_string(), true);
                return true;
            }
            continue;
        }
        if !seen.insert(next.clone()) {
            continue;
        }
        if let Some(rule) = package.rule(&next) {
            let grants_weapon = rule.grants.iter().any(|e| match e {
                Effect::FactGrant(f) | Effect::GatedFactGrant { fact: f, .. } => weapon_fact_words(rule, f).is_some(),
                _ => false,
            });
            if grants_weapon {
                memo.insert(id.to_string(), true);
                return true;
            }
        }
        stack.extend(package.granted_from(&next).iter().cloned());
    }
    memo.insert(id.to_string(), false);
    false
}

/// The walk's edges are: every `Granter::Class` edge of this class at or below `class_level`,
/// and every `Granter::Rule` edge out of a held rule. An edge whose own gate (`Grant.when`) or
/// whose target's own `applies` names a record the corpus does not carry, and whose target
/// leads to a weapon-proficiency grant, is a hole in the closure: `Some(reason)`.
fn missing_rule_on_a_proficiency_edge(
    package: &SheetRulePackage,
    held: &HeldSet,
    class_slug: &str,
    class_level: u8,
) -> Option<String> {
    let mut memo = BTreeMap::new();
    let mut check = |target: &SheetRule, when: &Applies, from: &str| -> Option<String> {
        let missing = applies_names_a_missing_rule(when).or_else(|| applies_names_a_missing_rule(&target.applies))?;
        if !reaches_a_weapon_grant(package, &target.id, &mut memo) {
            return None;
        }
        Some(format!(
            "the grant edge {from} -> {} depends on {missing}, a record the corpus does not carry, and leads to a weapon proficiency grant",
            target.id
        ))
    };
    for rule in package.rules.values() {
        for grant in &rule.granted_by {
            if let Granter::Class { id, at_level } = &grant.by
                && id == class_slug
                && *at_level <= class_level
                && let Some(reason) = check(rule, &grant.when, &format!("class {class_slug} level {at_level}"))
            {
                return Some(reason);
            }
        }
    }
    for id in held.rules.keys() {
        if held.removed.contains(id) {
            continue;
        }
        for target_id in package.granted_from(id) {
            let Some(target) = package.rule(target_id) else { continue };
            for grant in &target.granted_by {
                if matches!(&grant.by, Granter::Rule(g) if g == id)
                    && let Some(reason) = check(target, &grant.when, id)
                {
                    return Some(reason);
                }
            }
        }
    }
    None
}

/// A grant the CLASS LINE itself makes at or below `class_level` (`Granter::Class`) whose target
/// leads to a weapon-proficiency grant, but which the fixpoint did not admit: the closure the
/// class-level facts reach is not the closure the class line grants, so the reader does not
/// guess which one is right. (Measured shape: a token-level PRE of a different row hoisted onto
/// the granted record's own `applies` -- `fighter_class` carries the level-20 Weapon Mastery
/// pool's `PREVARGTEQ`, so levels 1-19 would otherwise read "no fighter proficiency".) A
/// `Granter::Rule` edge out of a held rule is NOT held to this: those legitimately stay shut on
/// an optional rule or a feat the class does not grant (the firearm grant behind Simple Weapon
/// Proficiency).
fn unadmitted_class_line_grant(
    package: &SheetRulePackage,
    held: &HeldSet,
    class_slug: &str,
    class_level: u8,
) -> Option<String> {
    let mut memo = BTreeMap::new();
    for rule in package.rules.values() {
        if held.rules.contains_key(&rule.id) || held.removed.contains(&rule.id) {
            continue;
        }
        for grant in &rule.granted_by {
            if let Granter::Class { id, at_level } = &grant.by
                && id == class_slug
                && *at_level <= class_level
                && reaches_a_weapon_grant(package, &rule.id, &mut memo)
            {
                return Some(format!(
                    "the class line grants {} at level {at_level}, which leads to a weapon proficiency grant, and the \
                     class-level fixpoint did not admit it (grant gate: {}; the record's own gate: {})",
                    rule.id,
                    describe_gate(package, &grant.when),
                    describe_gate(package, &rule.applies)
                ));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wizard() -> ClassWeaponProficiencyView {
        match class_weapon_proficiency_view("wizard", 1) {
            ProficiencyAnswer::Known(view) => view,
            ProficiencyAnswer::Unknown { reason } => panic!("wizard must be Known from the converted package: {reason}"),
        }
    }

    /// Wizard's record grants five named weapons and no blanket tier (`CLASS_WEAPON_PROFICIENCIES`
    /// states the same, from the same corpus record).
    #[test]
    fn wizard_reads_five_named_weapons_and_no_tier() {
        let view = wizard();
        assert!(view.tiers.is_empty(), "wizard tiers: {:?}", view.tiers);
        let expected: BTreeSet<String> =
            ["Club", "Dagger", "Crossbow (Heavy)", "Crossbow (Light)", "Quarterstaff"].iter().map(|s| s.to_string()).collect();
        assert_eq!(view.named, expected);
    }

    /// A class slug the package does not carry is Unknown with a reason, never an empty Known.
    #[test]
    fn an_unconverted_class_is_unknown_not_empty() {
        let answer = class_weapon_proficiency_view("no_such_class_anywhere", 1);
        assert!(matches!(answer, ProficiencyAnswer::Unknown { ref reason } if reason.contains("no_such_class_anywhere")), "{answer:?}");
    }

    fn rule(id: &str, granted_by: Vec<crate::rules_core::sheet_rule::Grant>, applies: Applies, grants: Vec<Effect>) -> SheetRule {
        SheetRule {
            id: id.to_string(),
            label: id.to_string(),
            value: crate::rules_core::sheet_rule::SheetValue::Text,
            also: Vec::new(),
            prose: Vec::new(),
            applies,
            target: None,
            bonus_type: None,
            print: true,
            pool: String::new(),
            tags: Vec::new(),
            subject: crate::rules_core::sheet_rule::Subject::Character,
            repeatable: false,
            granted_by,
            offers: None,
            grants,
            provenance: Default::default(),
        }
    }

    fn class_line(level: u8) -> Vec<crate::rules_core::sheet_rule::Grant> {
        vec![crate::rules_core::sheet_rule::Grant { by: Granter::Class { id: "fx".into(), at_level: level }, when: Applies::Always }]
    }

    fn package_of(rules: Vec<SheetRule>) -> SheetRulePackage {
        let mut package = SheetRulePackage::new();
        package.insert_rule(rule("fx_book:class:fx", Vec::new(), Applies::Always, Vec::new()));
        for r in rules {
            package.insert_rule(r);
        }
        package.finish();
        package
    }

    fn dagger() -> Fact {
        Fact::Proficiency(ProfRef::Weapon("Dagger".into()))
    }

    /// §3.4: no weapon grant anywhere in the walk is Unknown -- the package carries no
    /// closure-complete attestation that makes an empty answer honest.
    #[test]
    fn an_empty_walk_is_unknown_not_proficient_with_nothing() {
        let answer = class_weapon_proficiency_view_in(&package_of(Vec::new()), "fx", 1);
        assert!(matches!(answer, ProficiencyAnswer::Unknown { ref reason } if reason.contains("no weapon proficiency")), "{answer:?}");
    }

    /// A class-line grant at or below the level that leads to a weapon grant but is shut by its
    /// own record gate is a hole in the closure: Unknown, naming the record.
    #[test]
    fn an_unadmitted_class_line_weapon_grant_is_unknown() {
        let shut = Applies::Compare { lhs: Expr::ClassLevel("fx".into()), op: crate::rules_core::sheet_rule::Cmp::Gte, rhs: Expr::Const(20) };
        let package = package_of(vec![
            rule("fx_book:class_feature:fx_open", class_line(1), Applies::Always, vec![Effect::FactGrant(Fact::Proficiency(ProfRef::Weapon("Club".into())))]),
            rule("fx_book:class_feature:fx_shut", class_line(1), shut, vec![Effect::FactGrant(dagger())]),
        ]);
        let answer = class_weapon_proficiency_view_in(&package, "fx", 1);
        assert!(matches!(answer, ProficiencyAnswer::Unknown { ref reason } if reason.contains("fx_shut")), "{answer:?}");
        // At level 20 the same record is admitted and both grants count.
        let view = class_weapon_proficiency_view_in(&package, "fx", 20);
        let view = view.known().expect("admitted at 20");
        assert!(view.named.contains("Dagger") && view.named.contains("Club"), "{view:?}");
    }

    /// Gated facts: decidable-true counts, decidable-false is skipped, undecidable prints.
    #[test]
    fn gated_facts_count_skip_or_print_by_decidability() {
        let level_gate = |n: i32| Applies::Compare { lhs: Expr::ClassLevel("fx".into()), op: crate::rules_core::sheet_rule::Cmp::Gte, rhs: Expr::Const(n) };
        let race_gate = Applies::Holds { what: Holdable::Race("elf".into()), count: 1 };
        let package = package_of(vec![rule(
            "fx_book:class_feature:fx_profs",
            class_line(1),
            Applies::Always,
            vec![
                Effect::GatedFactGrant { fact: dagger(), when: level_gate(1) },
                Effect::GatedFactGrant { fact: Fact::Proficiency(ProfRef::Weapon("Longsword".into())), when: level_gate(5) },
                Effect::GatedFactGrant { fact: Fact::Proficiency(ProfRef::Weapon("Longbow".into())), when: race_gate },
            ],
        )]);
        let answer = class_weapon_proficiency_view_in(&package, "fx", 1);
        let view = answer.known().expect("known");
        assert!(view.named.contains("Dagger"), "{view:?}");
        assert!(!view.named.contains("Longsword"), "decidable-false must be skipped: {view:?}");
        assert!(!view.named.contains("Longbow"), "undecidable must not count: {view:?}");
        assert_eq!(view.printed_conditions.len(), 1, "{view:?}");
        assert!(view.printed_conditions[0].contains("Longbow") && view.printed_conditions[0].contains("elf"), "{view:?}");
    }

    #[test]
    fn a_race_or_missing_record_gate_is_not_class_decidable_but_an_archetype_off_gate_is() {
        let archetype_off = Applies::Not(Box::new(Applies::Holds {
            what: Holdable::RuleTag { pool: "archetype".into(), tag: "X".into() },
            count: 1,
        }));
        assert!(gate_is_class_decidable(&archetype_off));
        let missing = Applies::Holds { what: Holdable::MissingRule { pool: "special_ability".into(), name: "Y".into() }, count: 1 };
        assert!(!gate_is_class_decidable(&missing));
        let race = Applies::Holds { what: Holdable::Race("elf".into()), count: 1 };
        assert!(!gate_is_class_decidable(&race));
        let feat = Applies::Holds { what: Holdable::Rule("core_rulebook:feat:power_attack".into()), count: 1 };
        assert!(!gate_is_class_decidable(&feat));
    }
}
