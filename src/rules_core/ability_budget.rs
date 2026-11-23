//! How many picks each Abilities sub-tab grants, pool by pool, and why a pick is refused -- the
//! sibling of `feat_budget.rs` for the sub-tabs that are not Feats (`technical-design.md` §3.2).
//!
//! Rows are `abilities.budget.<subtab>.<pool>.{granted,spent,remaining}`, each pool naming its
//! grant sources with the character level that granted them. The UI renders the rows; it never
//! sums a grant (NR-1).
//!
//! **Traits.** The grants are the rules' own, read through the oracle's variable: every player
//! character has `Pool_Traits` 2 (`apg_abilities.lst:10`, `BONUS:VAR|Pool_Traits|2|TYPE=Base`),
//! Additional Traits adds 2 (`apg_feats.lst:9`, `BONUS:VAR|Pool_Traits|2`), and one drawback buys one
//! more trait (Ultimate Campaign, Drawbacks; [`trait_effects::DRAWBACKS`]). A character takes one trait
//! per category: each converted trait carries that rule as its own gate, `!PREABILITY:1,CATEGORY=Special
//! Ability,TYPE.<Category>Trait` (`trait_acrobat`'s `PREMULT`), so the category a trait is refused over
//! is read from the trait's own record, never from a hand-kept list.
//!
//! Traits are saved in `ChosenCharacterState.selected_traits` (`technical-design.md` §3.2); the
//! `ability_selections` field is the saved home of the sub-tabs that had none.
//!
//! **Class Features.** A pool's size is the rules' own, read the way `feat_budget.rs` reads a feat pool:
//! every held rule whose `target` is `Pool(<pool>)` adds its evaluated value (the Rogue's
//! `rogue_rogue_talents` is `RogueTalentLVL/2`, from `BONUS:ABILITYPOOL|Rogue Talent|RogueTalentLVL/2`),
//! never a per-class table. Feat pools stay with `feat_budget.rs`. The character level each pick opened
//! at is read by evaluating the same rules at every character level up to the character's own.
//! Picks are saved as `ability_selections` rows (`class_features|<pool>|<record>|<level>|`); an option
//! id is `<pool>:<record id>`. The members a pool offers are its grant's `offers` (`offer_selects`).

use std::collections::BTreeMap;

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use crate::rules_core::feat_budget::{feat_pool_of, grant_amount, repeats_the_line};
use crate::rules_core::feat_identity;
use crate::rules_core::ability_prereqs;
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation, PilotBaseChassisComputation};
use crate::rules_core::sheet_rule::{
    display_label, evaluate_applies, held_set, id_slug, offer_selects, sibling_line_gate, split_rule_id, Applies, BonusTarget, CharacterFacts,
    Choice, EvalContext, HeldSeed, Holdable, SheetRule, SheetRulePackage, SheetValue, Subject,
};
use crate::rules_core::sheet_rule_catalog::catalog_prose;
use crate::rules_core::trait_effects::{is_drawback, CREATION_TRAIT_LIMIT, DRAWBACKS};

const EXPLANATION_PREFIX: &str = "abilities.budget";
/// The package kind a character trait is converted under.
const TRAIT_KIND: &str = "trait";
/// The feat that adds two traits (`apg_feats.lst:9`).
const ADDITIONAL_TRAITS_FEAT: &str = "feat:additional_traits";
const ADDITIONAL_TRAITS_GRANT: i64 = 2;
/// Ultimate Campaign: a character may take one drawback.
const DRAWBACK_LIMIT: i64 = 1;

/// The trait pool and the drawback pool of the Traits sub-tab.
pub const TRAITS_POOL: &str = "traits";
pub const DRAWBACKS_POOL: &str = "drawbacks";

/// An Abilities sub-tab with an engine budget. Feats keep `feat_budget.rs`; each later sub-tab
/// joins here with its own card (E3.4-E3.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubTab {
    Traits,
    ClassFeatures,
}

impl SubTab {
    pub const ALL: [SubTab; 2] = [SubTab::Traits, SubTab::ClassFeatures];

    pub fn id(self) -> &'static str {
        match self {
            SubTab::Traits => "traits",
            SubTab::ClassFeatures => "class_features",
        }
    }

    pub fn from_id(id: &str) -> Option<SubTab> {
        SubTab::ALL.into_iter().find(|subtab| subtab.id() == id)
    }
}

/// One pool of picks on a sub-tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityPool {
    pub subtab: SubTab,
    pub id: String,
    pub granted: i64,
    pub spent: i64,
    /// `(character level that granted it, None when the source has no level of its own, words)`.
    pub sources: Vec<(Option<u8>, String)>,
    /// The character level each granted pick opened at, earliest first; empty for a pool whose
    /// grants have no level of their own (Traits).
    pub opened: Vec<u8>,
}

impl AbilityPool {
    pub fn remaining(&self) -> i64 {
        self.granted - self.spent
    }

    /// The character level the next pick taken from this pool is charged to: the level its first
    /// unspent grant opened at. `None` when no grant is left or the pool's grants carry no level.
    pub fn next_pick_level(&self) -> Option<u8> {
        usize::try_from(self.spent).ok().and_then(|spent| self.opened.get(spent)).copied()
    }

    fn sources_words(&self) -> String {
        let words: Vec<String> = self
            .sources
            .iter()
            .map(|(level, words)| match level {
                Some(level) => format!("{words} (level {level})"),
                None => words.clone(),
            })
            .collect();
        words.join(", ")
    }
}

/// One held pick and the category the book limits to one pick.
#[derive(Debug, Clone)]
struct HeldAbility {
    id: String,
    label: String,
    category: Option<String>,
}

/// A sub-tab's pools with every held pick charged.
#[derive(Debug, Clone)]
pub struct AbilityBudget {
    subtab: SubTab,
    pools: Vec<AbilityPool>,
    held: Vec<HeldAbility>,
}

impl AbilityBudget {
    pub fn subtab(&self) -> SubTab {
        self.subtab
    }

    pub fn pools(&self) -> &[AbilityPool] {
        &self.pools
    }

    pub fn pool(&self, id: &str) -> Option<&AbilityPool> {
        self.pools.iter().find(|pool| pool.id == id)
    }

    /// `abilities.budget.<subtab>.<pool>.{granted,spent,remaining}` for every pool.
    pub fn explanation_rows(&self) -> Vec<ComputationExplanation> {
        let row = |pool: &AbilityPool, field: &str, value: i64, detail: String| ComputationExplanation {
            id: format!("{EXPLANATION_PREFIX}.{}.{}.{field}", self.subtab.id(), pool.id),
            value: i16::try_from(value).unwrap_or(i16::MAX),
            detail,
        };
        self.pools
            .iter()
            .flat_map(|pool| {
                [
                    row(pool, "granted", pool.granted, format!("Granted by {}", pool.sources_words())),
                    row(pool, "spent", pool.spent, format!("Held picks charged to the {} pool", pool.id)),
                    row(pool, "remaining", pool.remaining(), format!("Granted minus spent in the {} pool", pool.id)),
                ]
            })
            .collect()
    }

    /// Why this character may not take `option_id` now, naming the pool or the category; `None`
    /// when the budget has room. Prerequisites are [`ability_options`]' `unmet`, refused by
    /// [`ability_prereqs::refusal`].
    pub fn refusal(&self, package: &SheetRulePackage, option_id: &str) -> Option<String> {
        if self.held.iter().any(|held| held.id == option_id) && !self.repeatable(package, option_id) {
            return Some(format!("This character already holds '{option_id}'."));
        }
        let pool_id = match self.subtab {
            SubTab::Traits => pool_of(option_id),
            SubTab::ClassFeatures => match option_id.split_once(':') {
                Some((pool, _)) => pool,
                None => return Some(format!("'{option_id}' names no class feature pool.")),
            },
        };
        if self.subtab == SubTab::ClassFeatures && self.pool(pool_id).is_none() {
            return Some(format!("This character has no {} pool.", pool_words(pool_id)));
        }
        if let Some(pool) = self.pool(pool_id)
            && pool.remaining() <= 0
        {
            return Some(format!(
                "No room in the {} pool: {} of {} spent (granted by {}).",
                pool.id,
                pool.spent,
                pool.granted,
                pool.sources_words()
            ));
        }
        if self.subtab != SubTab::Traits {
            return None;
        }
        let category = trait_category(package, option_id)?;
        let clash = self.held.iter().find(|held| held.category.as_deref() == Some(category.as_str()))?;
        Some(format!(
            "A character takes one trait of each category: {} is already this character's {} trait.",
            clash.label,
            category_words(&category)
        ))
    }
}

impl AbilityBudget {
    /// Whether the record an option names may be taken more than once (PCGen `MULT:YES`).
    fn repeatable(&self, package: &SheetRulePackage, option_id: &str) -> bool {
        self.subtab == SubTab::ClassFeatures
            && option_id.split_once(':').and_then(|(_, record)| package.rule(record)).is_some_and(|rule| rule.repeatable)
    }
}

/// `rogue_talent` -> `rogue talent`.
pub fn pool_words(pool: &str) -> String {
    pool.replace('_', " ")
}

fn pool_of(option_id: &str) -> &'static str {
    if is_drawback(option_id) { DRAWBACKS_POOL } else { TRAITS_POOL }
}

/// The budget of one sub-tab for this character.
pub fn ability_budget(package: &SheetRulePackage, input: &CharacterInput, subtab: SubTab) -> AbilityBudget {
    match subtab {
        SubTab::Traits => trait_budget(package, input),
        SubTab::ClassFeatures => class_feature_budget(package, input),
    }
}

/// [`ability_budget`] read at the character's current level only: the same pools and the same
/// granted / spent / remaining, but a Class Features grant names its rules without the level each
/// pick opened at, so it costs one held set instead of one per character level. For a surface that
/// shows the rows (the sheet's load); the sub-tab and the add path use [`ability_budget`].
pub fn ability_budget_now(package: &SheetRulePackage, input: &CharacterInput, subtab: SubTab) -> AbilityBudget {
    match subtab {
        SubTab::Traits => trait_budget(package, input),
        SubTab::ClassFeatures => class_feature_budget_from(package, input, false),
    }
}

/// `input` as it stood at character level `level`: its class levels taken in saved order.
fn at_character_level(input: &CharacterInput, level: u8) -> CharacterInput {
    let mut left = level;
    let mut at = input.clone();
    at.chosen.class_levels = input
        .chosen
        .class_levels
        .iter()
        .filter_map(|class| {
            let taken = class.level.min(left);
            left -= taken;
            (taken > 0).then(|| CharacterClassLevel { level: taken, ..class.clone() })
        })
        .collect();
    at.chosen.ability_selections.retain(|pick| pick.level <= level);
    at
}

/// Every class-feature pool the rules `input` holds grant picks into, with what it grants and the
/// labels of the granting rules. The same reading as `feat_budget`'s: a held rule targeting
/// `Pool(<pool>)` adds its evaluated value; a repeated converter line counts once. Feat pools stay on
/// the Feats sub-tab, a pool on an item is the item's, and a pool a race, racial trait, template,
/// monster record or character trait grants (a human's +2 to one ability, an ethnicity) belongs to the
/// Racial Abilities or Traits sub-tab. A choice about a companion (its status, a trick) is the
/// character's pick, so it is here.
fn class_feature_grants(package: &SheetRulePackage, input: &CharacterInput) -> BTreeMap<String, (i64, Vec<String>)> {
    class_feature_grants_holding(package, input, &picked_rules(input))
}

/// The records of the Class Features picks `input` saves. A pick is held: a member whose prerequisite is
/// another pick (Major Magic after Minor Magic) reads it, and a pick that grants a pool grants it.
fn picked_rules(input: &CharacterInput) -> Vec<String> {
    input
        .chosen
        .ability_selections
        .iter()
        .filter(|pick| pick.subtab == SubTab::ClassFeatures.id())
        .map(|pick| pick.record_key.clone())
        .collect()
}

/// [`class_feature_grants`] with `rule_ids` held outright besides what the character holds.
fn class_feature_grants_holding(package: &SheetRulePackage, input: &CharacterInput, rule_ids: &[String]) -> BTreeMap<String, (i64, Vec<String>)> {
    let base = compute_pilot_base_chassis(input);
    let mut seed = HeldSeed::from_character(input, &base);
    seed.rule_ids.extend(rule_ids.iter().cloned());
    let facts = CharacterFacts::from_character(input, &base).with_linked_picks(package, &seed);
    let held = held_set(package, &seed, &facts);
    let mut pools: BTreeMap<String, (i64, Vec<String>)> = BTreeMap::new();
    let mut counted: Vec<&SheetRule> = Vec::new();
    for (id, entry) in &held.rules {
        if held.removed.contains(id) {
            continue;
        }
        let Some(rule) = package.rule(id) else { continue };
        let (Some(BonusTarget::Pool(pool)), SheetValue::Number(amount)) = (&rule.target, &rule.value) else { continue };
        if !is_class_feature_grant(rule) {
            continue;
        }
        if counted.iter().any(|earlier| repeats_the_line(rule, earlier)) {
            continue;
        }
        counted.push(rule);
        let ctx = EvalContext { holder_class: entry.holder_class.clone(), spell_level: entry.spell_level.unwrap_or(0), item_tags: Vec::new() };
        let gate = if id.contains('#') {
            sibling_line_gate(package, &held, &facts, rule, ctx.clone())
        } else {
            evaluate_applies(&rule.applies, &held, package, &facts, ctx.clone())
        };
        if !gate.includes() {
            continue;
        }
        let amount = grant_amount(amount, &held, package, &facts, ctx);
        let amount = if amount.den == 0 { 0 } else { amount.num / amount.den };
        let row = pools.entry(pool.clone()).or_default();
        row.0 += amount;
        if amount != 0 && !row.1.contains(&rule.label) {
            row.1.push(rule.label.clone());
        }
    }
    pools
}

/// Record kinds whose pools sit on another sub-tab (see [`class_feature_grants`]).
const NOT_CLASS_FEATURE_KINDS: [&str; 7] = ["race", "race_trait", "template", "monster", "monster_ability", "trait", "equipment"];

/// Whether `rule` grants picks into a Class Features pool (see [`class_feature_grants`]).
pub(crate) fn is_class_feature_grant(rule: &SheetRule) -> bool {
    matches!(rule.target, Some(BonusTarget::Pool(_)))
        && feat_pool_of(rule).is_none()
        && rule.subject == Subject::Character
        && !NOT_CLASS_FEATURE_KINDS.contains(&split_rule_id(&rule.id).1)
}

fn class_feature_budget(package: &SheetRulePackage, input: &CharacterInput) -> AbilityBudget {
    class_feature_budget_from(package, input, true)
}

/// The Class Features budget; `by_level` reads every character level to name where each pick opened.
fn class_feature_budget_from(package: &SheetRulePackage, input: &CharacterInput, by_level: bool) -> AbilityBudget {
    let level: u8 = input.chosen.class_levels.iter().map(|class| class.level).sum();
    let now = class_feature_grants(package, input);
    let mut pools: BTreeMap<String, AbilityPool> = BTreeMap::new();
    // A character no rule grants a pool to needs no level-by-level reading (one held set, not one per level).
    let first = if !by_level || now.values().all(|(granted, _)| *granted == 0) { level } else { 1 };
    for at in first..=level {
        let grants = if at == level { now.clone() } else { class_feature_grants(package, &at_character_level(input, at)) };
        let ids: Vec<String> = pools.keys().chain(grants.keys()).cloned().collect();
        for id in ids {
            let (granted, labels) = grants.get(&id).cloned().unwrap_or_default();
            let pool = pools.entry(id.clone()).or_insert_with(|| AbilityPool {
                subtab: SubTab::ClassFeatures,
                id,
                granted: 0,
                spent: 0,
                sources: Vec::new(),
                opened: Vec::new(),
            });
            let gained = granted - pool.granted;
            if gained == 0 {
                continue;
            }
            let words = if labels.is_empty() { "no rule".to_owned() } else { labels.join(" + ") };
            pool.sources.push((by_level.then_some(at), format!("{words} {gained:+}")));
            if gained > 0 {
                pool.opened.extend(std::iter::repeat_n(at, usize::try_from(gained).unwrap_or(0)));
            } else {
                pool.opened.truncate(usize::try_from(granted.max(0)).unwrap_or(0));
            }
            pool.granted = granted;
        }
    }
    let picks: Vec<_> = input.chosen.ability_selections.iter().filter(|pick| pick.subtab == SubTab::ClassFeatures.id()).collect();
    for pick in &picks {
        pools
            .entry(pick.pool.clone())
            .or_insert_with(|| AbilityPool {
                subtab: SubTab::ClassFeatures,
                id: pick.pool.clone(),
                granted: 0,
                spent: 0,
                sources: Vec::new(),
                opened: Vec::new(),
            })
            .spent += 1;
    }
    let held = picks
        .iter()
        .map(|pick| HeldAbility {
            id: format!("{}:{}", pick.pool, pick.record_key),
            label: package.rule(&pick.record_key).map_or_else(|| pick.record_key.clone(), display_label),
            category: None,
        })
        .collect();
    pools.retain(|_, pool| pool.granted > 0 || pool.spent > 0);
    AbilityBudget { subtab: SubTab::ClassFeatures, pools: pools.into_values().collect(), held }
}

fn trait_budget(package: &SheetRulePackage, input: &CharacterInput) -> AbilityBudget {
    let chosen = &input.chosen;
    let held: Vec<HeldAbility> = chosen
        .selected_traits
        .iter()
        .map(|id| HeldAbility { id: id.clone(), label: option_label(package, id), category: trait_category(package, id) })
        .collect();
    let drawbacks_held = chosen.selected_traits.iter().filter(|id| is_drawback(id)).count() as i64;
    let traits_held = chosen.selected_traits.len() as i64 - drawbacks_held;

    let mut sources = vec![(Some(1), format!("every character ({CREATION_TRAIT_LIMIT})"))];
    let mut granted = CREATION_TRAIT_LIMIT as i64;
    if feat_identity::count(&chosen.selected_feats, ADDITIONAL_TRAITS_FEAT) > 0 {
        granted += ADDITIONAL_TRAITS_GRANT;
        sources.push((None, format!("Additional Traits ({ADDITIONAL_TRAITS_GRANT})")));
    }
    let bought = drawbacks_held.min(DRAWBACK_LIMIT);
    if bought > 0 {
        granted += bought;
        sources.push((Some(1), format!("a drawback ({bought})")));
    }
    let pools = vec![
        AbilityPool { subtab: SubTab::Traits, id: TRAITS_POOL.to_owned(), granted, spent: traits_held, sources, opened: Vec::new() },
        AbilityPool {
            subtab: SubTab::Traits,
            id: DRAWBACKS_POOL.to_owned(),
            granted: DRAWBACK_LIMIT,
            spent: drawbacks_held,
            sources: vec![(Some(1), format!("every character ({DRAWBACK_LIMIT})"))],
            opened: Vec::new(),
        },
    ];
    AbilityBudget { subtab: SubTab::Traits, pools, held }
}

/// The converted trait record a wire id (`trait:trait_acrobat`) names.
fn trait_rule<'a>(package: &'a SheetRulePackage, option_id: &str) -> Option<&'a SheetRule> {
    if is_drawback(option_id) {
        return None;
    }
    package.find(TRAIT_KIND, &id_slug(option_id)).and_then(|id| package.rule(id))
}

fn option_label(package: &SheetRulePackage, option_id: &str) -> String {
    if let Some(drawback) = DRAWBACKS.iter().find(|drawback| drawback.drawback_id == option_id) {
        return drawback.name.to_owned();
    }
    trait_rule(package, option_id).map_or_else(|| option_id.to_owned(), display_label)
}

/// The category tag a trait is limited to one of: the tag its own gate refuses a second holder of
/// (`Not(Holds(RuleTag))`), else its one `…Trait` tag below `Trait`/`BasicTrait` (a converted record
/// whose gate dropped the clause, as 34 `ReligionTrait` records do). `None` for a drawback, an unknown
/// id, or a record whose tags name no category.
pub fn trait_category(package: &SheetRulePackage, option_id: &str) -> Option<String> {
    let rule = trait_rule(package, option_id)?;
    excluded_tag(&rule.applies, false).or_else(|| {
        rule.tags
            .iter()
            .find(|tag| tag.ends_with("Trait") && !matches!(tag.as_str(), "Trait" | "BasicTrait") && !tag.contains(' '))
            .cloned()
    })
}

fn excluded_tag(gate: &Applies, negated: bool) -> Option<String> {
    match gate {
        Applies::Not(inner) => excluded_tag(inner, !negated),
        Applies::All(terms) | Applies::AtLeast { of: terms, .. } => terms.iter().find_map(|term| excluded_tag(term, negated)),
        Applies::Holds { what: Holdable::RuleTag { tag, .. }, .. } if negated => Some(tag.clone()),
        _ => None,
    }
}

/// `SocialTrait` -> `Social`.
pub fn category_words(tag: &str) -> &str {
    tag.strip_suffix("Trait").filter(|words| !words.is_empty()).unwrap_or(tag)
}

/// One pick a sub-tab offers this character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityOption {
    /// The wire id the add command takes and the saved field holds.
    pub id: String,
    pub label: String,
    pub pool: String,
    /// The category words the one-per-category rule reads (`Social`), if any.
    pub category: Option<String>,
    /// `false` when a prerequisite is definitively unmet or the category is already taken.
    pub qualified: bool,
    /// The unmet reasons, in the rule's own words.
    pub unmet: Vec<String>,
    /// The record's rule text when the converted record carries it (Class Features); `None` for a
    /// sub-tab whose screen reads its prose elsewhere.
    pub description: Option<String>,
}

/// Every pick the sub-tab offers, each with whether this character qualifies and why not.
pub fn ability_options(
    package: &SheetRulePackage,
    input: &CharacterInput,
    base: &PilotBaseChassisComputation,
    subtab: SubTab,
) -> Vec<AbilityOption> {
    match subtab {
        SubTab::Traits => trait_options(package, input, base),
        SubTab::ClassFeatures => class_feature_options(package, input, base),
    }
}

/// The offer that lists a pool's members: a held grant's first, else any converted grant's.
fn pool_offer<'a>(package: &'a SheetRulePackage, held_rules: &[&'a SheetRule], pool: &str) -> Option<&'a Choice> {
    let targets = |rule: &&SheetRule| matches!(&rule.target, Some(BonusTarget::Pool(p)) if p == pool);
    held_rules
        .iter()
        .copied()
        .filter(targets)
        .find_map(|rule| rule.offers.as_ref())
        .or_else(|| package.rules.values().filter(targets).find_map(|rule| rule.offers.as_ref()))
}

/// Every member of every class-feature pool this character holds, each with whether the record's own
/// gate is met (the converted-gate verdicts the Traits sub-tab reads). A pool whose grants carry no
/// offer lists no members. Two printings of one name are offered once, the first by record id.
fn class_feature_options(package: &SheetRulePackage, input: &CharacterInput, base: &PilotBaseChassisComputation) -> Vec<AbilityOption> {
    let mut pools: Vec<String> = class_feature_grants(package, input).into_iter().filter(|(_, (n, _))| *n > 0).map(|(pool, _)| pool).collect();
    for pick in input.chosen.ability_selections.iter().filter(|pick| pick.subtab == SubTab::ClassFeatures.id()) {
        if !pools.contains(&pick.pool) {
            pools.push(pick.pool.clone());
        }
    }
    let mut seed = HeldSeed::from_character(input, base);
    seed.rule_ids.extend(picked_rules(input));
    let facts = CharacterFacts::from_character(input, base).with_linked_picks(package, &seed);
    let held = held_set(package, &seed, &facts);
    let held_rules: Vec<&SheetRule> = held.rules.keys().filter_map(|id| package.rule(id)).collect();
    let mut options = Vec::new();
    for pool in &pools {
        let Some(offer) = pool_offer(package, &held_rules, pool) else { continue };
        let mut labels: Vec<String> = Vec::new();
        for rule in package.rules.values().filter(|rule| offer_selects(offer, rule) && !rule.label.is_empty()) {
            let label = display_label(rule);
            if labels.contains(&label) {
                continue;
            }
            labels.push(label.clone());
            let unmet = ability_prereqs::unmet(package, &held, &facts, &rule.applies);
            let prose = catalog_prose(package, rule);
            options.push(AbilityOption {
                id: format!("{pool}:{}", rule.id),
                label,
                pool: pool.clone(),
                category: None,
                qualified: unmet.is_empty(),
                unmet,
                description: (!prose.trim().is_empty()).then_some(prose),
            });
        }
    }
    options.sort_by(|a, b| (&a.pool, &a.label, &a.id).cmp(&(&b.pool, &b.label, &b.id)));
    options
}

fn trait_options(package: &SheetRulePackage, input: &CharacterInput, base: &PilotBaseChassisComputation) -> Vec<AbilityOption> {
    let budget = trait_budget(package, input);
    let facts = CharacterFacts::from_character(input, base);
    let held = held_set(package, &HeldSeed::from_character(input, base), &facts);
    let mut options: Vec<AbilityOption> = package
        .rules_of_kind(TRAIT_KIND)
        .filter(|rule| !rule.id.contains('#') && !rule.label.is_empty())
        .map(|rule| {
            let id = format!("{TRAIT_KIND}:{}", id_slug(&rule.id));
            let category = trait_category(package, &id);
            let mut unmet = Vec::new();
            if let Some(category) = &category
                && let Some(clash) = budget.held.iter().find(|h| h.id != id && h.category.as_ref() == Some(category))
            {
                unmet.push(format!("one {} trait only: {} is held", category_words(category), clash.label));
            }
            unmet.extend(ability_prereqs::unmet(package, &held, &facts, &rule.applies));
            AbilityOption {
                id,
                label: display_label(rule),
                pool: TRAITS_POOL.to_owned(),
                category: category.as_deref().map(|c| category_words(c).to_owned()),
                qualified: unmet.is_empty(),
                unmet,
                description: None,
            }
        })
        .collect();
    options.extend(DRAWBACKS.iter().map(|drawback| AbilityOption {
        id: drawback.drawback_id.to_owned(),
        label: drawback.name.to_owned(),
        pool: DRAWBACKS_POOL.to_owned(),
        category: None,
        qualified: true,
        unmet: Vec::new(),
        description: None,
    }));
    // Two books printing one slug resolve to one record (`SheetRulePackage::find`): offer it once.
    options.sort_by(|a, b| a.id.cmp(&b.id));
    options.dedup_by(|a, b| a.id == b.id);
    options.sort_by(|a, b| (&a.pool, &a.label, &a.id).cmp(&(&b.pool, &b.label, &b.id)));
    options
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::class_census::load_sweep_fixture;
    use crate::rules_core::class_seeds::input_for;
    use crate::rules_core::corpus_loader::load_sheet_rules;
    use std::sync::OnceLock;

    fn package() -> &'static SheetRulePackage {
        static PACKAGE: OnceLock<SheetRulePackage> = OnceLock::new();
        PACKAGE.get_or_init(|| load_sheet_rules(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/sheet_rules")).package)
    }

    fn level_one_with(traits: &[&str]) -> CharacterInput {
        let mut input = input_for(&load_sweep_fixture().expect("census fixture"), "fighter", 1);
        input.chosen.selected_traits = traits.iter().map(|t| (*t).to_owned()).collect();
        input
    }

    fn row(budget: &AbilityBudget, id: &str) -> i16 {
        budget.explanation_rows().iter().find(|row| row.id == id).unwrap_or_else(|| panic!("no row {id}")).value
    }

    #[test]
    fn ability_budget_traits_two_at_level_one_leave_none_and_a_third_is_refused_naming_the_pool() {
        let input = level_one_with(&["trait:trait_acrobat", "trait:trait_axe_to_grind"]);
        let budget = ability_budget(package(), &input, SubTab::Traits);
        assert_eq!(row(&budget, "abilities.budget.traits.traits.granted"), 2);
        assert_eq!(row(&budget, "abilities.budget.traits.traits.spent"), 2);
        assert_eq!(row(&budget, "abilities.budget.traits.traits.remaining"), 0);
        let refusal = budget.refusal(package(), "trait:trait_air_touched").expect("a third trait is refused");
        assert!(refusal.contains("traits pool"), "{refusal}");
    }

    #[test]
    fn ability_budget_traits_two_of_one_category_are_refused_naming_the_category() {
        let input = level_one_with(&["trait:trait_acrobat"]);
        let budget = ability_budget(package(), &input, SubTab::Traits);
        assert_eq!(row(&budget, "abilities.budget.traits.traits.remaining"), 1);
        let refusal = budget.refusal(package(), "trait:trait_ambitious").expect("a second Social trait is refused");
        assert!(refusal.contains("Social"), "{refusal}");
        assert_eq!(budget.refusal(package(), "trait:trait_air_touched"), None, "a Magic trait has room");
    }

    #[test]
    fn ability_budget_traits_a_drawback_and_additional_traits_each_grant_more() {
        let mut input = level_one_with(&["trait:trait_acrobat", "trait:trait_axe_to_grind", "drawback:vain"]);
        let budget = ability_budget(package(), &input, SubTab::Traits);
        assert_eq!(row(&budget, "abilities.budget.traits.traits.granted"), 3);
        assert_eq!(row(&budget, "abilities.budget.traits.drawbacks.remaining"), 0);
        assert!(budget.refusal(package(), "drawback:pride").expect("a second drawback").contains("drawbacks pool"));
        input.chosen.selected_feats.push("feat:additional_traits".to_owned());
        assert_eq!(row(&ability_budget(package(), &input, SubTab::Traits), "abilities.budget.traits.traits.granted"), 5);
    }

    #[test]
    fn ability_budget_traits_options_carry_qualified_and_the_unmet_reason() {
        let input = level_one_with(&["trait:trait_acrobat"]);
        let base = crate::rules_core::pilot_compute::compute_pilot_base_chassis(&input);
        let options = ability_options(package(), &input, &base, SubTab::Traits);
        let find = |id: &str| options.iter().find(|o| o.id == id).unwrap_or_else(|| panic!("no option {id}"));
        let ambitious = find("trait:trait_ambitious");
        assert!(!ambitious.qualified);
        assert!(ambitious.unmet.iter().any(|reason| reason.contains("Social") && reason.contains("Acrobat")), "{:?}", ambitious.unmet);
        assert!(find("trait:trait_air_touched").qualified);
        assert_eq!(find("drawback:vain").pool, DRAWBACKS_POOL);
        assert_eq!(options.iter().filter(|o| o.pool == DRAWBACKS_POOL).count(), DRAWBACKS.len());
        let mut ids: Vec<&str> = options.iter().map(|o| o.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), options.len(), "a slug two books print is offered once");
    }

    fn class_features_at(class: &str, level: u8) -> AbilityBudget {
        ability_budget(package(), &input_for(&load_sweep_fixture().expect("census fixture"), class, level), SubTab::ClassFeatures)
    }

    // The expected sizes are the books' class text, not the engine's read path: "at 2nd level, and every 2 levels
    // thereafter, a rogue gains one rogue talent" (Core Rulebook, Rogue); "at 2nd level, a barbarian gains a rage
    // power ... and an additional rage power for every two levels" (Core Rulebook, Barbarian); a ninja trick "at 2nd
    // level and every 2 levels thereafter" (Ultimate Combat, Ninja).
    #[test]
    fn ability_budget_class_features_rogue_6_is_granted_three_rogue_talents_at_levels_2_4_6() {
        let budget = class_features_at("rogue", 6);
        assert_eq!(row(&budget, "abilities.budget.class_features.rogue_talent.granted"), 3);
        assert_eq!(row(&budget, "abilities.budget.class_features.rogue_talent.remaining"), 3);
        let pool = budget.pool("rogue_talent").expect("rogue talent pool");
        let levels: Vec<Option<u8>> = pool.sources.iter().map(|(level, _)| *level).collect();
        assert_eq!(levels, [Some(2), Some(4), Some(6)], "{:?}", pool.sources);
    }

    /// E0.1's census of class-feature pools (`artifacts/epic_0/abilities-census.json`).
    fn census() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/abilities-census.json");
        serde_json::from_str(&std::fs::read_to_string(path).expect("E0.1 census")).expect("census JSON")
    }

    /// The class a converted class feature belongs to: its `<Class>ClassFeatures` tag, else the longest
    /// class slug its own slug starts with (`investigator_alchemist_discovery` -> `investigator`).
    fn class_of(rule: &SheetRule, classes: &[&str]) -> Option<String> {
        let slug = crate::rules_core::sheet_rule::split_rule_id(&rule.id).2;
        let slug = slug.split('#').next().unwrap_or(slug);
        rule.tags
            .iter()
            .filter_map(|tag| tag.strip_suffix("ClassFeatures"))
            .map(str::to_lowercase)
            .find(|class| classes.contains(&class.as_str()))
            .or_else(|| {
                classes
                    .iter()
                    .filter(|class| slug == **class || slug.starts_with(&format!("{class}_")))
                    .max_by_key(|class| class.len())
                    .map(|class| (*class).to_owned())
            })
    }

    /// The rules declaring a variable a grant line's own gate reads (a kineticist's `Element Tracker Air`
    /// is declared by the primary and expanded Air element picks): holding them is making that choice.
    fn rules_the_gate_reads(rule: &SheetRule) -> Vec<String> {
        fn vars(value: &serde_json::Value, out: &mut Vec<String>) {
            match value {
                serde_json::Value::Object(map) => {
                    for (key, inner) in map {
                        match (key.as_str(), inner) {
                            ("Var", serde_json::Value::String(var)) => out.push(var.clone()),
                            _ => vars(inner, out),
                        }
                    }
                }
                serde_json::Value::Array(items) => items.iter().for_each(|item| vars(item, out)),
                _ => {}
            }
        }
        let mut names = Vec::new();
        vars(&serde_json::to_value(&rule.applies).expect("applies serialize"), &mut names);
        names.iter().filter_map(|var| package().vars.get(var)).flat_map(|table| table.declared_by.iter().cloned()).collect()
    }

    /// Runs `work` over `items` on 8 threads (each held set takes about a second in a debug build).
    fn in_parallel<T: Sync, R: Send>(items: &[T], work: impl Fn(&T) -> R + Sync) -> Vec<R> {
        let chunk = items.len().div_ceil(8).max(1);
        std::thread::scope(|scope| {
            let handles: Vec<_> = items.chunks(chunk).map(|part| scope.spawn(|| part.iter().map(&work).collect::<Vec<R>>())).collect();
            handles.into_iter().flat_map(|handle| handle.join().expect("census thread")).collect()
        })
    }

    /// Acceptance census (E3.4): every pool E0.1 counts with a class among its granting records yields
    /// a grant row, or is a Feats-sub-tab pool, or is on E0.1's data-gap list, or is a shape this card
    /// split off to a discovered card. A grant row is proved by evaluation: the pool is granted > 0 to
    /// some class at level 20 (its canonical seed), or, for a pool granted through a further choice (an
    /// oracle's mystery), to that class at 20 holding the pool's own converted grant rule.
    #[test]
    fn ability_budget_class_features_census_every_class_owned_pool_yields_a_grant_row_or_is_a_listed_gap() {
        use crate::rules_core::sheet_rule::split_rule_id;
        use std::collections::BTreeSet;
        let census = census();
        let fixture = load_sweep_fixture().expect("census fixture");
        let mut classes: Vec<&str> =
            package().rules_of_kind("class").filter(|rule| !rule.id.contains('#')).map(|rule| split_rule_id(&rule.id).2).collect();
        classes.sort_unstable();
        classes.dedup();
        let granted_to = |grants: BTreeMap<String, (i64, Vec<String>)>| grants.into_iter().filter(|(_, (n, _))| *n > 0).map(|(pool, _)| pool);
        let granted: BTreeSet<String> =
            in_parallel(&classes, |class| granted_to(class_feature_grants(package(), &input_for(&fixture, class, 20))).collect::<Vec<_>>())
                .into_iter()
                .flatten()
                .collect();

        let gaps: Vec<&str> = census["data_gap_list"].as_array().expect("gap list").iter().filter_map(|gap| gap["pool"].as_str()).collect();
        let grant_rules = |slug: &str| -> Vec<&SheetRule> {
            package().rules.values().filter(|rule| matches!(&rule.target, Some(BonusTarget::Pool(p)) if p == slug)).collect()
        };
        let mut rows = 0;
        let mut feat_pools = 0;
        let mut listed = 0;
        let mut selection: Vec<String> = Vec::new();
        let mut through_a_choice: Vec<(String, String, Vec<String>)> = Vec::new();
        let mut unexplained: Vec<String> = Vec::new();
        for (name, pool) in census["class_feature_pools"]["pools"].as_object().expect("pools") {
            if pool["granted_by"].get("class").is_none() {
                continue;
            }
            let slug = crate::rules_core::sheet_rule::id_slug(name);
            let rules = grant_rules(&slug);
            if granted.contains(&slug) {
                rows += 1;
            } else if slug == FEAT_POOL_ID || rules.iter().any(|rule| feat_pool_of(rule).is_some()) {
                feat_pools += 1;
            } else if gaps.contains(&name.as_str()) {
                listed += 1;
            } else if pool["mechanisms"] == serde_json::json!(["abilityselection"]) {
                selection.push(name.clone());
            } else {
                let mut principals: Vec<String> = rules
                    .iter()
                    .flat_map(|rule| std::iter::once(rule.id.split('#').next().unwrap_or(&rule.id).to_owned()).chain(rules_the_gate_reads(rule)))
                    .collect();
                principals.sort();
                principals.dedup();
                match rules.iter().find_map(|rule| class_of(rule, &classes)) {
                    Some(class) if !principals.is_empty() => through_a_choice.push((slug, class, principals)),
                    _ => unexplained.push(name.clone()),
                }
            }
        }
        let proved = in_parallel(&through_a_choice, |(slug, class, principals)| {
            (slug.clone(), granted_to(class_feature_grants_holding(package(), &input_for(&fixture, class, 20), principals)).any(|p| p == *slug))
        });
        let not_proved: Vec<String> = proved.iter().filter(|(_, ok)| !ok).map(|(slug, _)| slug.clone()).collect();
        eprintln!(
            "census: {} classes; class-owned pools: grant row on a class seed {rows}, grant row through a choice {}, Feats sub-tab {feat_pools}, data gap {listed}, ABILITYSELECTION (E3.13) {}; not proved {not_proved:?}; unexplained {unexplained:?}",
            classes.len(),
            proved.len() - not_proved.len(),
            selection.len()
        );
        assert_eq!(selection, ABILITY_SELECTION_POOLS_SPLIT_OFF, "the ABILITYSELECTION shape this card split off (discovered card E3.13)");
        assert!(not_proved.is_empty() && unexplained.is_empty(), "not proved {not_proved:?}; unexplained {unexplained:?}");
    }

    /// The pools E0.1 counts whose size is a level-1 `ABILITYSELECTION` pick, not a `Pool` grant: split off
    /// to discovered card E3.13 (E3.4's split rule).
    const ABILITY_SELECTION_POOLS_SPLIT_OFF: [&str; 5] =
        ["Blood of Dragons Bloodline", "Bloodrager Bloodline", "Crossblooded Bloodline", "Crossblooded Rager Bloodline", "Sorcerer Bloodline"];
    const FEAT_POOL_ID: &str = "feat";

    #[test]
    fn ability_budget_class_features_now_serves_the_same_numbers_without_levels() {
        let input = input_for(&load_sweep_fixture().expect("census fixture"), "rogue", 6);
        let numbers = |budget: &AbilityBudget| budget.explanation_rows().into_iter().map(|row| (row.id, row.value)).collect::<Vec<_>>();
        let now = ability_budget_now(package(), &input, SubTab::ClassFeatures);
        assert_eq!(numbers(&now), numbers(&ability_budget(package(), &input, SubTab::ClassFeatures)));
        assert!(now.pool("rogue_talent").expect("pool").sources.iter().all(|(level, _)| level.is_none()));
    }

    #[test]
    fn ability_budget_class_features_barbarian_4_is_granted_two_rage_powers() {
        assert_eq!(row(&class_features_at("barbarian", 4), "abilities.budget.class_features.rage_power.granted"), 2);
    }

    #[test]
    fn ability_budget_class_features_ninja_2_is_granted_one_ninja_trick_and_ninja_1_none() {
        assert_eq!(row(&class_features_at("ninja", 2), "abilities.budget.class_features.ninja_trick.granted"), 1);
        assert!(class_features_at("ninja", 1).pool("ninja_trick").is_none(), "no trick before 2nd level");
    }
}
