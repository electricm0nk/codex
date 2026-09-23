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
//! - the walk reaches no weapon-proficiency grant at all, unless the converter attests the class's
//!   closure complete (`SheetRule::closure_complete`, SD-36 F1c-3 D4) AND no rule the class line
//!   reaches at or below the level leads to a weapon grant -- then the answer is Known and empty
//!   (the class grants none);
//! - the walk meets a [`Holdable::MissingRule`] on a grant edge leading to a
//!   weapon-proficiency grant: the closure is incomplete there, so the reader does not guess
//!   which way that edge goes;
//! - a weapon `ProfRef` names a tag that matches none of: a tier (`Simple`/`Martial`/`Exotic`),
//!   a `Weapon Group <x>` tag some converted equipment rule carries, a `WeaponAllOf` whose every
//!   conjunct the weapon table answers (tier, `Melee`, `Ranged`), or a `WeaponSet` with members
//!   (review finding 15 -- a bare `Auto` reaching the reader as a tag is Unknown; the converter
//!   resolves the real `TYPE=Auto` selector to a `WeaponSet` at ingest).
//!
//! # A pick of one weapon from a named list
//!
//! A held pick the converter linked to its pool's one member (SD-36 F1c-3, D6: `offers: Rules`
//! on the pick, the member offering a weapon list resolved at ingest -- the Commoner's one
//! Simple weapon) is carried in [`ClassWeaponProficiencyView::weapon_picks`] with its choice id
//! and options. The class level cannot say which weapon the player took; the character's own
//! recorded choice decides it, and the sheet prints the choice.
//!
//! # A pick the reader cannot resolve
//!
//! A held player's pick that could grant a weapon proficiency (Commoner's one simple weapon) is
//! carried in [`ClassWeaponProficiencyView::unresolved_picks`] ([`unresolved_weapon_pick`]
//! names the rule). The answer stays Known for what the counted grants cover, and the caller
//! answers Unknown for every other weapon -- never Known(false) from a closure known to be
//! incomplete.
//!
//! Deterministic; no I/O beyond the package handle.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

use crate::rules_core::level_up_option_filter::{describe_gate, describe_prof};
use crate::rules_core::rules_tables::crb::weapon_tables::WeaponProficiency;
use crate::rules_core::sheet_rule::{
    held_set, resolve_gated_fact_grant, split_rule_id, Applies, BonusTarget, CharacterFacts, Choice, Effect, EvalContext, Expr,
    Fact, GatedFact, Granter, HeldSeed, HeldSet, Holdable, OptionSet, ProfRef, RuleId, SheetRule, SheetRulePackage,
    SheetValue,
};
use crate::rules_core::sheet_rule_package;

/// One `ProfRef::WeaponSet` as the reader returns it: the selector's own label (display only)
/// and the literal member list the converter resolved at ingest.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeaponSetView {
    pub label: String,
    pub members: Vec<String>,
}

/// A held player's pick of ONE weapon from a list the converter resolved at ingest (SD-36
/// F1c-3, D6: the pick's pool linked to its one member's weapon options). The class-level walk
/// cannot know which weapon the player takes; the character's own recorded choice under
/// `choice` decides it, and the sheet prints it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeaponPickView {
    /// The choice id a character records its pick under (`SelectedChoice::choice_set_id`).
    pub choice: String,
    /// The picking rule's words (`Weapon and Armor Proficiency`).
    pub label: String,
    /// Every weapon-proficiency name the pick may take, as the oracle spells it.
    pub options: Vec<String>,
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
    /// A player's pick the walk holds that could grant a weapon proficiency, but whose options
    /// the package cannot resolve (see [`unresolved_weapon_pick`]). The closure is incomplete
    /// there: every weapon the counted grants do not cover is Unknown, never Known(false).
    pub unresolved_picks: Vec<String>,
    /// A held weapon pick whose options the package names ([`WeaponPickView`]): decided by the
    /// character's recorded choice, never by the class level.
    pub weapon_picks: Vec<WeaponPickView>,
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
    let mut pool_reaches_weapon: BTreeMap<String, Option<bool>> = BTreeMap::new();
    for (id, entry) in &held.rules {
        if held.removed.contains(id) {
            continue;
        }
        let Some(rule) = package.rule(id) else { continue };
        match linked_pick(package, rule) {
            Some(LinkedPick::Weapon(pick)) => {
                acc.picks.insert(pick);
            }
            Some(LinkedPick::NotAProficiency) => {}
            None => {
                if let Some(pick) = unresolved_weapon_pick(package, rule, &mut pool_reaches_weapon) {
                    acc.unresolved.insert(pick);
                }
            }
        }
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
        && view.unresolved_picks.is_empty()
        && view.weapon_picks.is_empty()
    {
        // §3.4 "Known-empty vs unknown" (SD-36 F1c-3, D4): an empty answer is honest only when
        // the converter attests the closure complete (`closure_complete` on the class principal)
        // AND no rule the class line reaches at or below this level leads to a weapon grant --
        // otherwise the walk is empty because a gate the class-level facts cannot open shut it,
        // not because the class grants nothing.
        let attested = package.rule(principal).is_some_and(|r| r.closure_complete);
        if !attested {
            return ProficiencyAnswer::Unknown {
                reason: format!(
                    "the converted closure of `{class_slug}` at level {class_level} grants no weapon proficiency, and the \
                     package carries no closure-complete attestation that none is owed"
                ),
            };
        }
        if let Some(reached) = class_line_reaches_a_weapon_grant(package, principal, class_slug, class_level) {
            return ProficiencyAnswer::Unknown {
                reason: format!(
                    "the converted closure of `{class_slug}` is attested complete, but {reached} leads to a weapon \
                     proficiency grant the level-{class_level} walk did not admit"
                ),
            };
        }
    }
    ProficiencyAnswer::Known(view)
}

/// `Some(rule id)` when the class principal, or any rule the class line grants at or below
/// `class_level` (a class-selection class's base class line included), leads to a weapon grant.
fn class_line_reaches_a_weapon_grant(package: &SheetRulePackage, principal: &str, class_slug: &str, class_level: u8) -> Option<RuleId> {
    let mut memo = BTreeMap::new();
    if reaches_a_weapon_grant(package, principal, &mut memo) {
        return Some(principal.to_string());
    }
    let lines = package.class_lines_held(&[(class_slug.to_string(), i64::from(class_level))]);
    for (class, _) in &lines {
        if class != class_slug
            && let Some(base) = package.find("class", class)
            && reaches_a_weapon_grant(package, base, &mut memo)
        {
            return Some(base.clone());
        }
    }
    package
        .rules
        .values()
        .filter(|rule| {
            rule.granted_by.iter().any(|g| {
                matches!(&g.by, Granter::Class { id, at_level } if lines.iter().any(|(c, _)| c == id) && *at_level <= class_level)
            })
        })
        .find(|rule| reaches_a_weapon_grant(package, &rule.id, &mut memo))
        .map(|rule| rule.id.clone())
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

enum LinkedPick {
    Weapon(WeaponPickView),
    /// The pick's one member offers weapons but grants no proficiency with the pick (a Weapon
    /// Focus-style choice): not this reader's business.
    NotAProficiency,
}

/// A pick the converter linked to its pool's members (`offers: Rules { pool, tags }` on a
/// `target: Pool(_)` rule, `pool_link.rs`): `Some` when exactly one member answers, and that
/// member offers a weapon list; `None` for any other rule (the unlinked path decides it).
fn linked_pick(package: &SheetRulePackage, rule: &SheetRule) -> Option<LinkedPick> {
    let Some(BonusTarget::Pool(_)) = &rule.target else { return None };
    let Some(Choice { from: OptionSet::Rules { pool, tags, .. }, .. }) = &rule.offers else { return None };
    let mut members = package.rules.values().filter(|r| {
        !r.id.contains('#') && &r.pool == pool && tags.iter().all(|t| r.tags.iter().any(|o| o.eq_ignore_ascii_case(t)))
    });
    let (Some(member), None) = (members.next(), members.next()) else { return None };
    let Some(Choice { id: choice, from: OptionSet::Weapons(options), .. }) = &member.offers else { return None };
    let grants_the_pick = member.grants.iter().any(|e| {
        matches!(e, Effect::FactGrant(Fact::Proficiency(ProfRef::Chosen(c))) | Effect::FactGrant(Fact::Chosen(c)) if c == choice)
    });
    if !grants_the_pick {
        return Some(LinkedPick::NotAProficiency);
    }
    Some(LinkedPick::Weapon(WeaponPickView { choice: choice.clone(), label: rule.label.clone(), options: options.clone() }))
}

/// `Some(words)` when `rule` is a held player's pick (`target: Pool(p)` with a non-zero count)
/// that could grant a weapon proficiency and whose options the reader cannot resolve.
///
/// One rule, two branches by what the package carries (reader batch blocker 2):
/// - the pool has member rules in the package: the pick is a weapon pick when any member reaches
///   a weapon-proficiency grant ([`reaches_a_weapon_grant`]); the member the player took is not
///   a class-level fact, so it is unresolved either way;
/// - the pool has NO member rule (measured: 1,090 of the 1,092 pools a converted rule picks
///   into; the converter does not carry PCGen's `ABILITYCATEGORY` `TYPE` link from a pool to its
///   members): the package cannot say what the pick covers. The pick is then treated as a weapon
///   pick exactly when the picking record's own name or its pool's name says it grants a
///   proficiency (`proficien`, any case) -- the only statement the converted package makes
///   about that pick's subject. Commoner's `Weapon and Armor Proficiency` ->
///   `simple_weapon_proficiency_choice` is one; Marksman's `Marksman Combat Style` is not.
///
/// A pick the reader cannot resolve is never guessed in either direction: the caller answers
/// Unknown for every weapon the counted grants do not cover.
fn unresolved_weapon_pick(
    package: &SheetRulePackage,
    rule: &SheetRule,
    pool_reaches_weapon: &mut BTreeMap<String, Option<bool>>,
) -> Option<String> {
    let Some(BonusTarget::Pool(pool)) = &rule.target else { return None };
    let count = match &rule.value {
        SheetValue::Number(Expr::Const(0)) => return None,
        SheetValue::Number(Expr::Const(n)) => n.to_string(),
        SheetValue::Number(_) => "a computed number of".to_string(),
        _ => return None,
    };
    let linked = *pool_reaches_weapon.entry(pool.clone()).or_insert_with(|| {
        let mut memo = BTreeMap::new();
        let members: Vec<&RuleId> = package.rules.values().filter(|r| &r.pool == pool).map(|r| &r.id).collect();
        if members.is_empty() {
            None
        } else {
            Some(members.into_iter().any(|id| reaches_a_weapon_grant(package, id, &mut memo)))
        }
    });
    let names_proficiency = |text: &str| text.to_ascii_lowercase().contains("proficien");
    let weapon_pick = match linked {
        Some(reaches) => reaches,
        None => names_proficiency(&rule.label) || names_proficiency(pool),
    };
    if !weapon_pick {
        return None;
    }
    let pool_words = pool.replace('_', " ");
    Some(format!(
        "{} ({}) picks {count} from the {pool_words} pool, whose options the converted package does not {}",
        rule.label,
        rule.id,
        if linked.is_some() { "decide at class level" } else { "link (no converted rule is a member of it)" }
    ))
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
    unresolved: BTreeSet<String>,
    picks: BTreeSet<WeaponPickView>,
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
        self.view.unresolved_picks = self.unresolved.into_iter().collect();
        self.view.weapon_picks = self.picks.into_iter().collect();
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
/// guess which one is right. (Measured shape, until SD-36 F1c-2 fixed it in the converter: a
/// token-level PRE of a different row hoisted onto the granted record's own `applies` --
/// `fighter_class` carried the level-20 Weapon Mastery pool's `PREVARGTEQ`, so levels 1-19 would
/// otherwise have read "no fighter proficiency". The guard stays for any closure it still
/// catches.) A
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
            closure_complete: false,
            always_held: false,
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

    fn attested_package(rules: Vec<SheetRule>) -> SheetRulePackage {
        let mut package = SheetRulePackage::new();
        let mut principal = rule("fx_book:class:fx", Vec::new(), Applies::Always, Vec::new());
        principal.closure_complete = true;
        package.insert_rule(principal);
        for r in rules {
            package.insert_rule(r);
        }
        package.finish();
        package
    }

    /// D4: the converter attests the closure complete and the walk finds no weapon grant --
    /// the class grants none: Known, empty.
    #[test]
    fn a_complete_closure_with_no_weapon_grant_answers_known_empty() {
        let armor = Effect::FactGrant(Fact::Proficiency(ProfRef::ArmorGroup("Light".into())));
        let package = attested_package(vec![rule("fx_book:class_feature:fx_armor", class_line(1), Applies::Always, vec![armor])]);
        for level in [1, 10] {
            let answer = class_weapon_proficiency_view_in(&package, "fx", level);
            assert_eq!(answer, ProficiencyAnswer::Known(ClassWeaponProficiencyView::default()), "level {level}");
        }
    }

    /// D4: without the attestation (or with a closure rule that reaches a weapon grant the walk
    /// did not admit) an empty walk stays Unknown.
    #[test]
    fn an_incomplete_closure_still_answers_unknown() {
        let unattested = package_of(vec![rule("fx_book:class_feature:fx_misc", class_line(1), Applies::Always, Vec::new())]);
        let answer = class_weapon_proficiency_view_in(&unattested, "fx", 1);
        assert!(matches!(answer, ProficiencyAnswer::Unknown { ref reason } if reason.contains("closure-complete")), "{answer:?}");
        // Attested, but a rule the closure reaches grants a weapon behind a gate the class-level
        // facts shut (a variable no held rule contributes): not "proficient with nothing".
        let shut = Applies::Compare { lhs: Expr::Var("v_outside".into()), op: crate::rules_core::sheet_rule::Cmp::Eq, rhs: Expr::Const(1) };
        let standard = rule("fx_book:class_feature:fx_standard", Vec::new(), shut, vec![Effect::FactGrant(dagger())]);
        let mut standard = standard;
        standard.granted_by = vec![crate::rules_core::sheet_rule::Grant { by: Granter::Rule("fx_book:class_feature:fx_misc".into()), when: Applies::Always }];
        let gated = attested_package(vec![rule("fx_book:class_feature:fx_misc", class_line(1), Applies::Always, Vec::new()), standard]);
        let answer = class_weapon_proficiency_view_in(&gated, "fx", 1);
        assert!(matches!(answer, ProficiencyAnswer::Unknown { .. }), "{answer:?}");
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

    fn pick(id: &str, pool: &str, count: i32) -> SheetRule {
        let mut r = rule(id, class_line(1), Applies::Always, Vec::new());
        r.value = crate::rules_core::sheet_rule::SheetValue::Number(Expr::Const(count));
        r.target = Some(crate::rules_core::sheet_rule::BonusTarget::Pool(pool.to_string()));
        r
    }

    /// Reader batch blocker 2: a held pick into a pool the package links no member to, on a
    /// record whose own name (or pool) says it grants a proficiency, is an unresolved pick --
    /// the view carries it, so the caller answers Unknown for every weapon not otherwise
    /// covered. A pick of zero, a pick whose record/pool names no proficiency, and a pick into a
    /// linked pool whose members reach no weapon grant are not.
    #[test]
    fn an_unlinked_proficiency_pick_is_an_unresolved_pick() {
        let club = || Effect::FactGrant(Fact::Proficiency(ProfRef::Weapon("Club".into())));
        let mut member = rule("fx_book:class_feature:fx_style_a", Vec::new(), Applies::Always, Vec::new());
        member.pool = "fx_linked_style".into();
        let package = package_of(vec![
            rule("fx_book:class_feature:fx_autos", class_line(1), Applies::Always, vec![club()]),
            pick("fx_book:class_feature:fx_weapon_and_armor_proficiency", "fx_simple_weapon_proficiency_choice", 1),
            pick("fx_book:class_feature:fx_combat_style", "fx_combat_style", 1),
            pick("fx_book:class_feature:fx_zero_proficiency", "fx_other_proficiency_choice", 0),
            pick("fx_book:class_feature:fx_linked", "fx_linked_style", 1),
            member,
        ]);
        let view = class_weapon_proficiency_view_in(&package, "fx", 1);
        let view = view.known().expect("known: the Club grant is counted");
        assert!(view.named.contains("Club"), "{view:?}");
        assert_eq!(view.unresolved_picks.len(), 1, "{view:?}");
        assert!(view.unresolved_picks[0].contains("fx simple weapon proficiency choice"), "{view:?}");
    }

    /// D6: a pick the converter linked to its pool's one member (`offers: Rules` on the pick;
    /// the member offers a weapon list and grants the chosen proficiency) is a weapon pick with
    /// its choice id and options -- not an unresolved pick, and not a counted grant.
    #[test]
    fn a_linked_weapon_pick_is_a_weapon_pick_with_its_options() {
        let mut linked = pick("fx_book:class_feature:fx_weapon_and_armor_proficiency", "fx_simple_weapon_proficiency_choice", 1);
        linked.offers = Some(Choice {
            id: linked.id.clone(),
            count: Expr::Const(1),
            from: OptionSet::Rules { pool: "special_ability".into(), tags: vec!["FxSingleSimple".into()], requires: Applies::Always },
        });
        let member_id = "fx_book:class_feature:fx_single_simple_weapon_proficiency";
        let mut member = rule(member_id, Vec::new(), Applies::Always, vec![Effect::FactGrant(Fact::Proficiency(ProfRef::Chosen(member_id.into())))]);
        member.pool = "special_ability".into();
        member.tags = vec!["FxSingleSimple".into()];
        member.offers = Some(Choice { id: member_id.into(), count: Expr::Const(1), from: OptionSet::Weapons(vec!["Club".into(), "Dagger".into()]) });
        let package = package_of(vec![linked, member]);
        let answer = class_weapon_proficiency_view_in(&package, "fx", 1);
        let view = answer.known().expect("a linked pick is Known with its pick");
        assert!(view.unresolved_picks.is_empty(), "{view:?}");
        assert_eq!(
            view.weapon_picks,
            vec![WeaponPickView { choice: member_id.into(), label: "fx_book:class_feature:fx_weapon_and_armor_proficiency".into(), options: vec!["Club".into(), "Dagger".into()] }]
        );
        assert!(view.named.is_empty() && view.tiers.is_empty(), "the pick is never a counted grant: {view:?}");
    }

    /// SD-36 F1c-4 (D7), on the real package: the Summoner's own gates read variables set only
    /// by the global Internal `Default` ability every character holds (`cr__stats.lst:4` grants
    /// it; `apg_abilities_class.lst:715` declares CLASS_SummonerAllowed / StandardSummonerAllowed
    /// 0 and sets CLASS_SummonerAllowed from StandardSummonerAllowed, `:717` sets
    /// StandardSummonerAllowed 1). With `Default` attested always held, a summoner walk starts
    /// from that base state: the class's own BAB line (`applies` CLASS_SummonerAllowed == 1,
    /// `apg_classes.lst:141`) and the Standard Class selection's gate
    /// (`PREVAREQ:StandardSummonerAllowed,1`, `apg_abilities_class.lst:741`) both open. Before
    /// D7 both read 0 and were shut for every summoner.
    ///
    /// The reader still answers Unknown for the Summoner: the Standard Class is a player's pick
    /// from the Summoner Class Selection pool (`apg_abilities_class.lst:739`
    /// `BONUS:VAR|Pool_Summoner_Class_Selection|1`, `apg_abilitycategories.lst:267`
    /// `POOL:Pool_Summoner_Class_Selection`), which the converter writes as neither a pick nor a
    /// grant edge -- a separate mechanism (D8), not a gate.
    #[test]
    fn summoner_standard_summoner_gate_opens_from_the_always_held_base_state() {
        use crate::rules_core::sheet_rule::{evaluate_applies, Gate};
        let package = sheet_rule_package::package().as_ref().expect("package loads");
        assert!(package.is_always_held("advanced_players_guide:class_feature:default"), "Default must be attested always held");
        let seed = HeldSeed { classes: vec![("summoner".into(), 1)], ..HeldSeed::default() };
        let facts = CharacterFacts { level: 1, class_levels: vec![("summoner".into(), 1)], ..CharacterFacts::default() };
        let held = held_set(package, &seed, &facts);
        for id in ["advanced_players_guide:class_feature:summoner_standard_class", "advanced_players_guide:class:summoner#bonus0"] {
            let rule = package.rule(id).unwrap_or_else(|| panic!("{id} is converted"));
            let gate = evaluate_applies(&rule.applies, &held, package, &facts, EvalContext { holder_class: Some("summoner".into()), ..EvalContext::default() });
            assert_eq!(gate, Gate::Include, "{id}: {}", describe_gate(package, &rule.applies));
        }
    }

    fn var_table(var: &str, declarer: &str, contributor: &str) -> crate::rules_core::sheet_rule::VarTable {
        crate::rules_core::sheet_rule::VarTable {
            var: var.into(),
            label: var.into(),
            declared_by: vec![declarer.into()],
            contributions: vec![crate::rules_core::sheet_rule::VarContribution {
                rule_id: contributor.into(),
                expr: Expr::Const(1),
                bonus_type: None,
                when: Applies::Always,
            }],
            provenance: Default::default(),
        }
    }

    /// A class line whose weapon grant sits behind `v_global == 1`, where `v_global` is declared
    /// and set to 1 by `global` -- a record nothing grants. `always_held` says whether the
    /// converter attested `global` as held by every character.
    fn global_gate_package(always_held: bool, table: Option<crate::rules_core::sheet_rule::VarTable>) -> SheetRulePackage {
        let gate = Applies::Compare { lhs: Expr::Var("v_global".into()), op: crate::rules_core::sheet_rule::Cmp::Eq, rhs: Expr::Const(1) };
        let mut standard = rule("fx_book:class_feature:fx_standard", Vec::new(), gate, vec![Effect::FactGrant(dagger())]);
        standard.granted_by =
            vec![crate::rules_core::sheet_rule::Grant { by: Granter::Rule("fx_book:class_feature:fx_selection".into()), when: Applies::Always }];
        let mut global = rule("fx_book:class_feature:default", Vec::new(), Applies::Always, Vec::new());
        global.always_held = always_held;
        let mut package = SheetRulePackage::new();
        let mut principal = rule("fx_book:class:fx", Vec::new(), Applies::Always, Vec::new());
        principal.closure_complete = true;
        package.insert_rule(principal);
        package.insert_rule(rule("fx_book:class_feature:fx_selection", class_line(1), Applies::Always, Vec::new()));
        package.insert_rule(standard);
        package.insert_rule(global);
        if let Some(table) = table {
            package.insert_var(table);
        }
        package.finish();
        package
    }

    /// D7, synthetic: an always-held global's unconditional variable setting is part of the base
    /// state every walk starts from -- the gate it opens opens for the held-set fixpoint and for
    /// the reader alike.
    #[test]
    fn an_always_held_global_seeds_every_walk() {
        let global = "fx_book:class_feature:default";
        let package = global_gate_package(true, Some(var_table("v_global", global, global)));
        let seed = HeldSeed { classes: vec![("fx".into(), 1)], ..HeldSeed::default() };
        let facts = CharacterFacts { level: 1, class_levels: vec![("fx".into(), 1)], ..CharacterFacts::default() };
        let held = held_set(&package, &seed, &facts);
        assert!(held.rules.contains_key("fx_book:class_feature:fx_standard"), "the fixpoint must open the global's gate");
        assert!(!held.rules.contains_key(global), "an always-held global seeds variables, it is not a held sheet line");
        let answer = class_weapon_proficiency_view_in(&package, "fx", 1);
        let view = answer.known().unwrap_or_else(|| panic!("Known through the global's gate: {answer:?}"));
        assert!(view.named.contains("Dagger"), "{view:?}");
    }

    /// D7, the converse: a gate on a variable nobody the character holds sets -- set only by a
    /// record NOT attested always-held, or with no variable table at all -- stays shut, and the
    /// attested-complete class whose line reaches the shut grant answers Unknown, never empty.
    #[test]
    fn a_gate_on_a_variable_nobody_sets_stays_unknown() {
        let global = "fx_book:class_feature:default";
        for package in [global_gate_package(false, Some(var_table("v_global", global, global))), global_gate_package(true, None)] {
            let answer = class_weapon_proficiency_view_in(&package, "fx", 1);
            assert!(matches!(answer, ProficiencyAnswer::Unknown { .. }), "{answer:?}");
        }
        // Attested always-held, but its only contribution is conditional: not base state.
        let mut table = var_table("v_global", global, global);
        table.contributions[0].when = Applies::Holds { what: Holdable::Race("elf".into()), count: 1 };
        let answer = class_weapon_proficiency_view_in(&global_gate_package(true, Some(table)), "fx", 1);
        assert!(matches!(answer, ProficiencyAnswer::Unknown { .. }), "{answer:?}");
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
