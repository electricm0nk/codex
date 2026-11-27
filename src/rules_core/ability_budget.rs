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
//! A level-1 `ABILITYSELECTION` pool (SD-43 E3.13: `CHOOSE:ABILITYSELECTION|Sorcerer Bloodline|...`) has no `Pool`
//! grant: its size is the count of the chooser's own offer while the chooser is held or offered ([`selection_pool`]).
//! A bloodline chosen at creation (`selected_choices`, `choice:sorcerer_bloodline`) is that pool's pick
//! ([`creation_choice_picks`]): charged and held, so a second bloodline is refused for room; picking another
//! member renames that choice ([`creation_choice_swap`]) instead of adding a second pick. The chooser it was picked
//! through is itself the pick of a held pool whose offer selects it (E3.20: the bloodline implies its kind,
//! `Standard Bloodline`, in the `Bloodline` pool `Sorcerer ~ Standard Bloodline` grants).
//! A held class feature with a parameter choice of its own (E3.26: `CHOOSE:` of a weapon, spell, skill, school, class,
//! language or free text; [`parameter_choice`]) is a pool of its own, `<record slug>_choice`, sized by its offer's count:
//! its one option is the record, and a pick saves the chosen value as its `target` (option id `<pool>:<record>|<value>`),
//! which the character's facts record under the record's choice so its `Chosen` grants read it.
//!
//! **Archetype.** An archetype of a held class is one converted record in the `archetype` pool, named by
//! its class (`rogue_archetype_acrobat`). It is saved as a `SelectedChoice` under
//! [`ARCHETYPE_CHOICE_ID`], the selection being the corpus `KEY:` `archetype_resolver` reads, prefixed to
//! persist (`"archetype:Rogue Archetype ~ Acrobat"`), or the record id for a book that table does not carry. The class features it
//! replaces are its record's tags and that table's `replaces`; a class feature is replaced once, so an
//! archetype replacing one a held archetype already replaces is refused, naming the feature. A held
//! archetype's record is held when the Class Features pools are read, so a pool it shrinks or grants follows.
//!
//! **Favored Class** (Core Rulebook p.31, `decisions.md §19`). One favored class, saved as a `SelectedChoice`
//! under [`FAVORED_CLASS_CHOICE_ID`]; then one bonus per level of it, +1 hit point or +1 skill rank, saved
//! as `ability_selections` rows (`favored_class|favored_class_bonus|<record>|<character level>|`).
//! [`favored_class_bonus_taken`] is what the sheet's maximum hit points and skill point pool add.
//!
//! **Background and Misc** (SD-43 E3.9). PCGen titles a sub-tab with its ability category's display location;
//! `data/ability_categories.json` (`scripts/derive_ability_categories.py`, E0.1's 22 + 219 categories less 1 PI-screened Background one) names
//! each Background and Misc category's pool, its members (parent category slug + type tags) and whether PCGen
//! leaves its size to the player. A pool is granted the way a Class Features pool is (held rules targeting
//! `Pool(<category>)`); an editable pool has no count budget (no rule grants it; the player or GM sets it), so
//! it serves no rows and refuses no pick for room. Picks are `ability_selections` rows
//! (`background|<pool>|<record>|<level>|`), held when pools are read, so a pick that grants a pool grants it
//! (Horror Adventures: the corruption optional rule grants a corruption).

use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;

use crate::rules_core::character_input::{CharacterClassLevel, CharacterInput, SelectedChoice};
use crate::rules_core::feat_budget::{feat_pool_of, grant_amount, repeats_the_line};
use crate::rules_core::feat_identity;
use crate::rules_core::ability_prereqs;
use crate::rules_core::archetype_resolver::{archetype_catalog_entries, archetype_resolve, ARCHETYPE_CHOICE_ID, ARCHETYPE_SELECTION_PREFIX};
use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, ComputationExplanation, PilotBaseChassisComputation};
use crate::rules_core::sheet_rule::{
    chooser_offered, display_label, evaluate_applies, held_set, id_slug, link_path_a_picks, slug, offer_selects, sibling_line_gate, split_rule_id, Applies,
    BonusTarget, CharacterFacts, Choice, Effect, EvalContext, Expr, Fact, Granter, HeldRule, HeldSeed, Holdable, OptionSet, SheetRule, SheetRulePackage, SheetValue, Subject,
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
    Archetype,
    FavoredClass,
    Background,
    Misc,
}

impl SubTab {
    pub const ALL: [SubTab; 6] = [SubTab::Traits, SubTab::ClassFeatures, SubTab::Archetype, SubTab::FavoredClass, SubTab::Background, SubTab::Misc];

    pub fn id(self) -> &'static str {
        match self {
            SubTab::Traits => "traits",
            SubTab::ClassFeatures => "class_features",
            SubTab::Archetype => "archetype",
            SubTab::FavoredClass => "favored_class",
            SubTab::Background => "background",
            SubTab::Misc => "misc",
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
            SubTab::ClassFeatures | SubTab::FavoredClass | SubTab::Background | SubTab::Misc => match option_id.split_once(':') {
                Some((pool, _)) => pool,
                None => return Some(format!("'{option_id}' names no class feature pool.")),
            },
            // An archetype has no count budget; a replaced-feature clash is its option's `unmet`.
            SubTab::Archetype => return None,
        };
        if self.subtab == SubTab::FavoredClass
            && pool_id == FAVORED_CLASS_POOL
            && let Some(held) = self.held.iter().find(|held| held.id.starts_with(FAVORED_CLASS_POOL) && !held.id.starts_with(FAVORED_BONUS_POOL))
        {
            return Some(format!("This character's favored class is already {}; remove it to choose another.", held.label));
        }
        if ability_category(pool_id).is_some_and(|category| category.editable_pool) {
            return None;
        }
        if matches!(self.subtab, SubTab::ClassFeatures | SubTab::FavoredClass | SubTab::Background | SubTab::Misc) && self.pool(pool_id).is_none() {
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
        (self.subtab == SubTab::FavoredClass && option_id.starts_with(FAVORED_BONUS_POOL))
            || matches!(self.subtab, SubTab::ClassFeatures | SubTab::Background | SubTab::Misc)
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
        SubTab::Archetype => archetype_budget(package, input),
        SubTab::FavoredClass => favored_class_budget(package, input, 0),
        SubTab::Background | SubTab::Misc => category_budget(package, input, subtab),
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
        SubTab::Archetype => archetype_budget(package, input),
        SubTab::FavoredClass => favored_class_budget(package, input, 0),
        SubTab::Background | SubTab::Misc => category_budget(package, input, subtab),
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
    class_feature_grants_holding(package, input, &[], SubTab::ClassFeatures)
}

/// [`class_feature_grants`] with `rule_ids` held outright besides what the character holds; the pools of
/// `subtab` (Class Features, Background or Misc).
fn class_feature_grants_holding(
    package: &SheetRulePackage,
    input: &CharacterInput,
    rule_ids: &[String],
    subtab: SubTab,
) -> BTreeMap<String, (i64, Vec<String>)> {
    let base = compute_pilot_base_chassis(input);
    let mut seed = HeldSeed::from_character(input, &base);
    seed.rule_ids.extend(rule_ids.iter().cloned());
    let facts = CharacterFacts::from_character(input, &base).with_linked_picks(package, &seed);
    let held = held_set(package, &seed, &facts);
    let mut pools: BTreeMap<String, (i64, Vec<String>)> = BTreeMap::new();
    let mut counted: Vec<&SheetRule> = Vec::new();
    // A record every character holds (`always_held`: Inner Sea World Guide's `Default`, one region of origin)
    // is not seeded into the held set, but its own `Always` grants count for every character.
    let base_state = HeldRule { holder_class: None, spell_level: None, via: None };
    let always = package.always_held_ids().filter(|id| subtab != SubTab::ClassFeatures && !held.rules.contains_key(*id)).map(|id| (id, &base_state));
    for (id, entry) in held.rules.iter().chain(always) {
        if held.removed.contains(id) {
            continue;
        }
        let Some(rule) = package.rule(id) else { continue };
        let (Some(BonusTarget::Pool(pool)), SheetValue::Number(amount)) = (&rule.target, &rule.value) else { continue };
        let grants = match subtab {
            SubTab::ClassFeatures => is_class_feature_grant(rule),
            _ => rule.subject == Subject::Character && ability_category(pool).is_some_and(|category| category.subtab == subtab.id()),
        };
        if !grants {
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
    if subtab == SubTab::ClassFeatures {
        for rule in package.rules_of_kind("class_feature") {
            if selection_offer(rule).is_none() || held.removed.contains(&rule.id) || !chooser_offered(package, &held, &facts, &rule.id) {
                continue;
            }
            let (Some(pool), Some(offer)) = (selection_pool(package, rule), rule.offers.as_ref()) else { continue };
            let ctx = held.rules.get(&rule.id).map_or_else(EvalContext::default, |entry| EvalContext {
                holder_class: entry.holder_class.clone(),
                spell_level: entry.spell_level.unwrap_or(0),
                item_tags: Vec::new(),
            });
            let amount = grant_amount(&offer.count, &held, package, &facts, ctx);
            let amount = if amount.den == 0 { 0 } else { amount.num / amount.den };
            let row = pools.entry(pool.to_owned()).or_default();
            row.0 += amount;
            if amount != 0 && !row.1.contains(&rule.label) {
                row.1.push(rule.label.clone());
            }
        }
        // A held class feature's parameter choice (E3.26) is a pool of its own, sized by its offer's count, unless a
        // creation choice already answers it (a canonical seed: the Expert's ten class skills).
        for (id, entry) in &held.rules {
            let Some(rule) = package.rule(id).filter(|_| !held.removed.contains(id)) else { continue };
            let Some(offer) = parameter_offer(rule) else { continue };
            if input.chosen.selected_choices.iter().any(|choice| choice.choice_set_id == rule.id) {
                continue;
            }
            let ctx = EvalContext { holder_class: entry.holder_class.clone(), spell_level: entry.spell_level.unwrap_or(0), item_tags: Vec::new() };
            let amount = grant_amount(&offer.count, &held, package, &facts, ctx);
            let amount = if amount.den == 0 { 0 } else { amount.num / amount.den };
            let row = pools.entry(parameter_pool(rule)).or_default();
            row.0 += amount;
            let label = display_label(rule);
            if amount != 0 && !row.1.contains(&label) {
                row.1.push(label);
            }
        }
    }
    pools
}

/// PCGen's own parent ability categories (`Special Ability`, `Internal`, `FEAT`): an offer drawing from one is a
/// typed pick of that broad category, never a pool of its own.
const BASE_CATEGORY_POOLS: [&str; 3] = ["special_ability", "internal", "feat"];

/// The pool a class feature's untagged offer draws from when the feature has no target of its own: the shape the
/// converter gives `CHOOSE:ABILITYSELECTION|<Category>|...` (`sorcerer_standard_bloodline_selection` offers 1 from
/// `sorcerer_bloodline`). The cheap structural half of [`selection_pool`].
fn selection_offer(rule: &SheetRule) -> Option<&str> {
    let Some(Choice { from: OptionSet::Rules { pool, tags, .. }, .. }) = &rule.offers else { return None };
    (rule.target.is_none()
        && rule.subject == Subject::Character
        && !rule.id.contains('#')
        && split_rule_id(&rule.id).1 == "class_feature"
        && tags.is_empty()
        && !BASE_CATEGORY_POOLS.contains(&pool.as_str()))
    .then_some(pool.as_str())
}

/// The level-1 `ABILITYSELECTION` pool `rule` sizes (E3.13): [`selection_offer`]'s pool, when the pool has members
/// and no rule grants it as a `Pool` (a granted pool is sized by its grants; the offer would count it twice).
pub(crate) fn selection_pool<'a>(package: &SheetRulePackage, rule: &'a SheetRule) -> Option<&'a str> {
    let pool = selection_offer(rule)?;
    let members = package.rules.values().any(|other| other.pool == pool);
    let granted = package.rules.values().any(|other| matches!(&other.target, Some(BonusTarget::Pool(p)) if p == pool));
    (members && !granted).then_some(pool)
}

/// The separator between a Class Features option id and the parameter its pick takes (E3.26): `<pool>:<record>|<target>`,
/// the `target` its saved `ability_selection=` line carries.
pub const TARGET_SEPARATOR: char = '|';

/// What a class feature's parameter choice offers (E3.26).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterChoice {
    /// The offer in the rule's own words (`the weapons Longsword, Rapier`), the prompt when the player writes it in.
    pub words: String,
    /// The values the corpus lists for it, sorted; empty when it lists none and the player writes the value in.
    pub values: Vec<String>,
}

/// The parameter choice a class feature asks for (E3.26, PCGen's `CHOOSE:` of a weapon, spell, skill, school, class,
/// language or free text): its own offer (`offers.id` is the record's id) when that offer is not a pool's members
/// ([`selection_offer`]), a domain, a deity, a template or a number. On a record that sizes a pool the offer describes the
/// pool's members (a favored enemy's creature types), except a weapon list: a pool's members are records, never weapons
/// (the Kensai's `Kensai Weapon Proficiency` grants its `Kensai Chosen Weapon` pool and asks for the weapon). A record
/// whose `CHOOSE:` the corpus screened as product identity keeps its grant of the chosen skill
/// (`ClassSkillChosen(<own id>)`), so its offer is a skill.
fn parameter_offer(rule: &SheetRule) -> Option<Choice> {
    let sizes_a_pool = matches!(rule.target, Some(BonusTarget::Pool(_))) && !matches!(&rule.offers, Some(Choice { from: OptionSet::Weapons(_), .. }));
    if rule.id.contains('#') || split_rule_id(&rule.id).1 != "class_feature" || sizes_a_pool || selection_offer(rule).is_some() {
        return None;
    }
    match &rule.offers {
        Some(offer) => (offer.id == rule.id
            && !matches!(offer.from, OptionSet::Domains | OptionSet::Deities | OptionSet::Templates(_) | OptionSet::Number { .. }))
        .then(|| offer.clone()),
        None => rule
            .grants
            .iter()
            .any(|effect| matches!(effect, Effect::FactGrant(Fact::ClassSkillChosen(choice)) if *choice == rule.id))
            .then(|| Choice { id: rule.id.clone(), count: Expr::Const(1), from: OptionSet::Skills(Vec::new()) }),
    }
}

/// The Class Features pool a record's parameter choice is taken in: `<record slug>_choice`.
pub fn parameter_pool(rule: &SheetRule) -> String {
    format!("{}_choice", split_rule_id(&rule.id).2)
}

/// [`parameter_offer`]'s choice with the values the corpus lists for it: the records of the kind it names (a skill, a
/// language, a class, a race; all of them for an empty list), the equipment records a weapon list names, the spells of
/// the class list and levels it names, the members of a tagged ability list. A list the converter kept no member filter
/// for (a weapon class, `QUALIFIED`, a spellcaster, an untagged base category), a school, an item and free text list no
/// value: the player writes it in.
pub fn parameter_choice(package: &SheetRulePackage, rule: &SheetRule) -> Option<ParameterChoice> {
    let offer = parameter_offer(rule)?;
    let named = |kind: &str, names: &[String], all: bool| -> Vec<String> {
        package
            .rules_of_kind(kind)
            .filter(|record| !record.id.contains('#') && !record.label.is_empty())
            .filter(|record| {
                let label = display_label(record);
                (all && names.is_empty())
                    || names.iter().any(|name| {
                        name.eq_ignore_ascii_case(&label)
                            || name.eq_ignore_ascii_case(&record.label)
                            || id_slug(name) == split_rule_id(&record.id).2
                            // A language list may name a type (`ADD:LANGUAGE|1|TYPE=Spoken`, E3.27): its languages.
                            || (kind == "language" && record.tags.iter().any(|tag| tag.eq_ignore_ascii_case(name)))
                    })
            })
            .map(display_label)
            .collect()
    };
    let mut values = match &offer.from {
        OptionSet::Skills(names) => named("skill", names, true),
        OptionSet::Languages(names) => named("language", names, true),
        OptionSet::Classes(names) => named("class", names, true),
        OptionSet::Races(names) => named("race", names, true),
        OptionSet::Weapons(names) => named("equipment", names, false),
        OptionSet::Spells { class, levels } => package
            .rules_of_kind("spell")
            .filter(|spell| !spell.id.contains('#') && !spell.label.is_empty())
            .filter(|spell| {
                spell.granted_by.iter().any(|grant| {
                    matches!(&grant.by, Granter::ClassSpellList { id, spell_level } if (class.is_empty() || id == class) && (levels.0..=levels.1).contains(spell_level))
                })
            })
            .map(display_label)
            .collect(),
        OptionSet::Rules { pool, tags, .. } if !(tags.is_empty() && BASE_CATEGORY_POOLS.contains(&pool.as_str())) => {
            package.rules.values().filter(|member| offer_selects(&offer, member) && !member.label.is_empty()).map(display_label).collect()
        }
        _ => Vec::new(),
    };
    values.sort();
    values.dedup();
    Some(ParameterChoice { words: crate::rules_core::sheet_rule_catalog::option_set_words(package, &offer.from), values })
}

/// The option id a saved Class Features, Background or Misc pick is held under: `<pool>:<record>`, then
/// `|<target>` when it takes a parameter (E3.26).
pub fn pick_option_id(pick: &crate::rules_core::character_input::AbilitySelection) -> String {
    match &pick.target {
        Some(target) => format!("{}:{}{TARGET_SEPARATOR}{target}", pick.pool, pick.record_key),
        None => format!("{}:{}", pick.pool, pick.record_key),
    }
}

/// `<pool>:<record>|<target>` -> (`<pool>:<record>`, `Some(<target>)`); an id with no target is itself.
pub fn split_target(option_id: &str) -> (&str, Option<&str>) {
    match option_id.split_once(TARGET_SEPARATOR) {
        Some((id, target)) => (id, Some(target)),
        None => (option_id, None),
    }
}

/// The Class Features picks a creation choice fills: each `selected_choices` entry (`choice:sorcerer_bloodline ->
/// bloodline:arcane`) linked to the option a held or offered level-1 `ABILITYSELECTION` chooser offers for it, as
/// `(<pool>:<option id>, that choice)`. It is the pool's pick: charged, held, and removed by removing the choice.
pub fn creation_choice_picks(package: &SheetRulePackage, input: &CharacterInput) -> Vec<(String, SelectedChoice)> {
    let base = compute_pilot_base_chassis(input);
    let seed = HeldSeed::from_character(input, &base);
    let facts = CharacterFacts::from_character(input, &base);
    let held = held_set(package, &seed, &facts);
    link_path_a_picks(package, &held, &facts)
        .into_iter()
        .filter_map(|link| {
            let chooser = package.rule(&link.chooser)?;
            let pool = selection_pool(package, chooser)?;
            let choice = input
                .chosen
                .selected_choices
                .iter()
                .find(|c| c.choice_set_id == link.choice_set_id && c.selection_id.rsplit(':').next() == Some(link.member.as_str()))?;
            // The chooser is itself the pick of a held pool whose offer selects it (E3.20: the held bloodline implies its
            // kind, `Standard Bloodline`, the `Bloodline` pool's member), unless that pick is saved.
            let kinds = held.rules.keys().filter_map(|id| package.rule(id)).filter_map(|grant| match (&grant.target, &grant.offers) {
                (Some(BonusTarget::Pool(kind_pool)), Some(offer)) if is_class_feature_grant(grant) && offer_selects(offer, chooser) => Some(kind_pool.as_str()),
                _ => None,
            });
            let mut kind_picks: Vec<String> = Vec::new();
            for kind_pool in kinds {
                let saved = input.chosen.ability_selections.iter().any(|p| p.subtab == SubTab::ClassFeatures.id() && p.pool == kind_pool && p.record_key == chooser.id);
                let id = format!("{kind_pool}:{}", chooser.id);
                if !saved && !kind_picks.contains(&id) {
                    kind_picks.push(id);
                }
            }
            Some(std::iter::once(format!("{pool}:{}", link.option)).chain(kind_picks).map(|id| (id, choice.clone())).collect::<Vec<_>>())
        })
        .flatten()
        .collect()
}

/// The swap a pick of `option_id` makes when a creation choice fills its level-1 selection pool: that choice with
/// its member renamed to the one `option_id` names, so the pool keeps one pick and every reader of the choice (the
/// sheet's bloodline seam) reads the new member. `(old choice, new choice)`; `None` when no creation choice fills
/// the pool, `option_id` is that choice's own pick, or no member spelling links back to `option_id` (never guessed:
/// the renamed choice must link to exactly `option_id`).
pub fn creation_choice_swap(package: &SheetRulePackage, input: &CharacterInput, option_id: &str) -> Option<(SelectedChoice, SelectedChoice)> {
    let (pool, record) = option_id.split_once(':')?;
    let picks = creation_choice_picks(package, input);
    if picks.iter().any(|(id, _)| id == option_id) {
        return None;
    }
    let (_, old) = picks.into_iter().find(|(id, _)| id.split_once(':').is_some_and(|(p, _)| p == pool))?;
    let legacy = old.choice_set_id.strip_prefix("choice:")?;
    let (namespace, _) = old.selection_id.split_once(':')?;
    package
        .rules
        .values()
        .filter(|rule| rule.granted_by.iter().any(|g| matches!(&g.by, Granter::Rule(by) if by == record)))
        .filter_map(|rule| split_rule_id(&rule.id).2.strip_prefix(legacy)?.strip_prefix('_').map(str::to_owned))
        .find_map(|member| {
            let new = SelectedChoice { choice_set_id: old.choice_set_id.clone(), selection_id: format!("{namespace}:{member}") };
            let mut swapped = input.clone();
            swapped.chosen.selected_choices.iter_mut().filter(|c| **c == old).for_each(|c| *c = new.clone());
            creation_choice_picks(package, &swapped).iter().any(|(id, _)| id == option_id).then(|| (old.clone(), new))
        })
}

/// Record kinds whose pools sit on another sub-tab (see [`class_feature_grants`]).
const NOT_CLASS_FEATURE_KINDS: [&str; 7] = ["race", "race_trait", "template", "monster", "monster_ability", "trait", "equipment"];

/// Whether `rule` grants picks into a Class Features pool (see [`class_feature_grants`]).
pub(crate) fn is_class_feature_grant(rule: &SheetRule) -> bool {
    matches!(rule.target, Some(BonusTarget::Pool(_)))
        && feat_pool_of(rule).is_none()
        && rule.subject == Subject::Character
        && !NOT_CLASS_FEATURE_KINDS.contains(&split_rule_id(&rule.id).1)
        && !matches!(&rule.target, Some(BonusTarget::Pool(pool)) if ability_category(pool).is_some())
}

/// One PCGen ability category shown on the Background or Misc sub-tab (`data/ability_categories.json`).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AbilityCategory {
    /// The category's slug: the `Pool(<pool>)` its grants target.
    pub pool: String,
    pub name: String,
    /// `background` or `misc` ([`SubTab::id`]).
    pub subtab: String,
    /// PCGen leaves the pool's size to the player: no rule grants it, so it has no count budget.
    pub editable_pool: bool,
    /// The members: records of this package pool (the parent category's slug) ...
    pub member_pool: String,
    /// ... carrying one of these tags (case-insensitive), or every record of it when empty.
    pub member_tags: Vec<String>,
}

/// Every Background and Misc category, read once from the generated table.
pub fn ability_categories() -> &'static [AbilityCategory] {
    #[derive(serde::Deserialize)]
    struct File {
        categories: Vec<AbilityCategory>,
    }
    static CATEGORIES: OnceLock<Vec<AbilityCategory>> = OnceLock::new();
    CATEGORIES.get_or_init(|| {
        serde_json::from_str::<File>(include_str!("../../data/ability_categories.json")).expect("data/ability_categories.json parses").categories
    })
}

fn ability_category(pool: &str) -> Option<&'static AbilityCategory> {
    ability_categories().iter().find(|category| category.pool == pool)
}

/// Whether `rule` is a member a category offers.
fn category_member(category: &AbilityCategory, rule: &SheetRule) -> bool {
    rule.pool == category.member_pool
        && (category.member_tags.is_empty() || rule.tags.iter().any(|tag| category.member_tags.iter().any(|t| t.eq_ignore_ascii_case(tag))))
}

/// The Background or Misc budget: a pool per category the held rules (and picks) grant into, an editable
/// pool none, every pick charged to its pool.
fn category_budget(package: &SheetRulePackage, input: &CharacterInput, subtab: SubTab) -> AbilityBudget {
    let grants = class_feature_grants_holding(package, input, &[], subtab);
    let picks: Vec<_> = input.chosen.ability_selections.iter().filter(|pick| pick.subtab == subtab.id()).collect();
    let mut pools: BTreeMap<String, AbilityPool> = BTreeMap::new();
    for (id, (granted, labels)) in grants {
        if granted <= 0 {
            continue;
        }
        let words = format!("{} {granted:+}", labels.join(" + "));
        pools.insert(id.clone(), AbilityPool { subtab, id, granted, spent: 0, sources: vec![(None, words)], opened: Vec::new() });
    }
    for pick in &picks {
        if ability_category(&pick.pool).is_some_and(|category| category.editable_pool) {
            continue;
        }
        pools
            .entry(pick.pool.clone())
            .or_insert_with(|| AbilityPool { subtab, id: pick.pool.clone(), granted: 0, spent: 0, sources: Vec::new(), opened: Vec::new() })
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
    AbilityBudget { subtab, pools: pools.into_values().collect(), held }
}

/// The members of every Background or Misc category this character has a pool in (granted, editable, or
/// holding a pick), each with whether its own gate is met. Two printings of one name are offered once.
fn category_options(package: &SheetRulePackage, input: &CharacterInput, base: &PilotBaseChassisComputation, subtab: SubTab) -> Vec<AbilityOption> {
    let grants = class_feature_grants_holding(package, input, &[], subtab);
    let shown: Vec<&AbilityCategory> = ability_categories()
        .iter()
        .filter(|category| category.subtab == subtab.id())
        .filter(|category| {
            category.editable_pool
                || grants.get(&category.pool).is_some_and(|(granted, _)| *granted > 0)
                || input.chosen.ability_selections.iter().any(|pick| pick.subtab == subtab.id() && pick.pool == category.pool)
        })
        .collect();
    let mut by_member_pool: HashMap<&str, Vec<&AbilityCategory>> = HashMap::new();
    for category in &shown {
        by_member_pool.entry(category.member_pool.as_str()).or_default().push(category);
    }
    let seed = HeldSeed::from_character(input, base);
    let facts = CharacterFacts::from_character(input, base).with_linked_picks(package, &seed);
    let held = held_set(package, &seed, &facts);
    let mut seen: Vec<(String, String)> = Vec::new();
    let mut options = Vec::new();
    for rule in package.rules.values().filter(|rule| !rule.id.contains('#') && !rule.label.is_empty()) {
        let Some(categories) = by_member_pool.get(rule.pool.as_str()) else { continue };
        for category in categories.iter().filter(|category| category_member(category, rule)) {
            let label = display_label(rule);
            let key = (category.pool.clone(), label.clone());
            if seen.contains(&key) {
                continue;
            }
            seen.push(key);
            let unmet = ability_prereqs::unmet(package, &held, &facts, &rule.applies);
            let prose = catalog_prose(package, rule);
            options.push(AbilityOption {
                id: format!("{}:{}", category.pool, rule.id),
                label,
                pool: category.pool.clone(),
                category: Some(category.name.clone()),
                qualified: unmet.is_empty(),
                unmet,
                parameter: None,
                description: (!prose.trim().is_empty()).then_some(prose),
            });
        }
    }
    options.sort_by(|a, b| (&a.pool, &a.label, &a.id).cmp(&(&b.pool, &b.label, &b.id)));
    options
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
    // The saved picks, then the creation choices that fill a level-1 selection pool: `(pool, record)`.
    // `(pool, record, the parameter it takes)`.
    let picks: Vec<(String, String, Option<String>)> = input
        .chosen
        .ability_selections
        .iter()
        .filter(|pick| pick.subtab == SubTab::ClassFeatures.id())
        .map(|pick| (pick.pool.clone(), pick.record_key.clone(), pick.target.clone()))
        .chain(
            creation_choice_picks(package, input)
                .into_iter()
                .filter_map(|(id, _)| id.split_once(':').map(|(pool, record)| (pool.to_owned(), record.to_owned(), None))),
        )
        .collect();
    for (pool, _, _) in &picks {
        pools
            .entry(pool.clone())
            .or_insert_with(|| AbilityPool {
                subtab: SubTab::ClassFeatures,
                id: pool.clone(),
                granted: 0,
                spent: 0,
                sources: Vec::new(),
                opened: Vec::new(),
            })
            .spent += 1;
    }
    let held = picks
        .iter()
        .map(|(pool, record, target)| {
            let label = package.rule(record).map_or_else(|| record.clone(), display_label);
            match target {
                Some(target) => HeldAbility { id: format!("{pool}:{record}{TARGET_SEPARATOR}{target}"), label: format!("{label} ({target})"), category: None },
                None => HeldAbility { id: format!("{pool}:{record}"), label, category: None },
            }
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
    /// The parameter its pick names (E3.26: a class feature's weapon, spell, skill...); `None` when it takes none.
    pub parameter: Option<ParameterChoice>,
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
        SubTab::Archetype => archetype_options(package, input, base),
        SubTab::FavoredClass => favored_class_options(package, input),
        SubTab::Background | SubTab::Misc => category_options(package, input, base, subtab),
    }
}

/// The package pool every archetype record is converted under.
const ARCHETYPE_POOL: &str = "archetype";

/// The corpus `KEY:` of each archetype `archetype_resolver` carries, by its record's slug
/// (`rogue_archetype_acrobat` -> `"Rogue Archetype ~ Acrobat"`).
fn archetype_keys() -> BTreeMap<String, &'static str> {
    archetype_catalog_entries()
        .iter()
        .map(|entry| (format!("{}_archetype_{}", slug(entry.subject), slug(entry.archetype_name)), entry.key))
        .collect()
}

/// `(selection id, record)` for every archetype record in the package: the selection is the prefixed
/// corpus key (`archetype:Rogue Archetype ~ Acrobat`) when `archetype_resolver` carries it, else the record id.
fn archetype_records<'a>(package: &'a SheetRulePackage, keys: &BTreeMap<String, &'static str>) -> Vec<(String, &'a SheetRule)> {
    package
        .rules_of_kind("class_feature")
        .filter(|rule| rule.pool == ARCHETYPE_POOL && !rule.id.contains('#') && rule.id.contains("_archetype_"))
        .map(|rule| (keys.get(split_rule_id(&rule.id).2).map_or_else(|| rule.id.clone(), |key| format!("{ARCHETYPE_SELECTION_PREFIX}{key}")), rule))
        .collect()
}

/// The record an archetype selection names: a record id as is, a (prefixed) corpus key through its record's slug.
pub(crate) fn archetype_record<'a>(package: &'a SheetRulePackage, selection: &str) -> Option<&'a SheetRule> {
    if let Some(rule) = package.rule(selection) {
        return Some(rule);
    }
    let entry = archetype_resolve(selection)?;
    let slug = format!("{}_archetype_{}", slug(entry.subject), slug(entry.archetype_name));
    package.rules_of_kind("class_feature").find(|rule| rule.pool == ARCHETYPE_POOL && split_rule_id(&rule.id).2 == slug)
}

/// The archetypes `input` holds, `(selection id, record)`, in saved order.
fn held_archetypes<'a>(package: &'a SheetRulePackage, input: &CharacterInput) -> Vec<(String, &'a SheetRule)> {
    input
        .chosen
        .selected_choices
        .iter()
        .filter(|choice| choice.choice_set_id == ARCHETYPE_CHOICE_ID)
        .filter_map(|choice| archetype_record(package, &choice.selection_id).map(|rule| (choice.selection_id.clone(), rule)))
        .collect()
}

/// The class an archetype record belongs to (`rogue_archetype_acrobat` -> `rogue`).
fn archetype_class(rule: &SheetRule) -> &str {
    split_rule_id(&rule.id).2.split("_archetype_").next().unwrap_or("")
}

/// The class features an archetype replaces: its record's tags (less the archetype and society tags)
/// and `archetype_resolver`'s `replaces`, as printed (`RogueTrapfinding`).
fn replaced_features(rule: &SheetRule, selection: &str) -> Vec<String> {
    let mut slots: Vec<String> = rule
        .tags
        .iter()
        .filter(|tag| !tag.ends_with("Archetype") && !tag.to_ascii_uppercase().starts_with("PFS") && !tag.starts_with('['))
        .cloned()
        .collect();
    for slot in archetype_resolve(selection).and_then(|entry| entry.replaces).unwrap_or(&[]) {
        if !slots.iter().any(|s| s.eq_ignore_ascii_case(slot)) {
            slots.push((*slot).to_owned());
        }
    }
    slots
}

/// Whether two replaced-feature names name one feature: equal ignoring case, or one the other's
/// numbered step (`RogueTrapSense` or `FighterArmorTraining_ALL` and `RogueTrapSense1`).
fn same_feature(a: &str, b: &str) -> bool {
    let whole = |s: &str| {
        let s = s.to_ascii_lowercase();
        s.strip_suffix("_all").map_or_else(|| s.clone(), str::to_owned)
    };
    let (a, b) = (whole(a), whole(b));
    if a == b {
        return true;
    }
    let (short, long) = if a.len() <= b.len() { (&a, &b) } else { (&b, &a) };
    long.strip_prefix(short.as_str()).is_some_and(|rest| rest.chars().all(|c| c.is_ascii_digit()))
        && !short.ends_with(|c: char| c.is_ascii_digit())
}

/// `RogueTrapSense1` of a rogue -> `Trap Sense 1`.
fn feature_words(slot: &str, class: &str) -> String {
    let prefix = class.replace('_', "");
    let rest = match (slot.get(..prefix.len()), slot.get(prefix.len()..)) {
        (Some(head), Some(rest)) if !rest.is_empty() && head.eq_ignore_ascii_case(&prefix) => rest,
        _ => slot,
    };
    let rest = rest.strip_prefix("CF_").unwrap_or(rest);
    let (rest, all) = match rest.strip_suffix("_ALL") {
        Some(rest) => (rest, " (all)"),
        None => (rest, ""),
    };
    let mut words = String::new();
    let mut prev: Option<char> = None;
    for c in rest.chars() {
        if let Some(p) = prev
            && ((c.is_ascii_uppercase() && p.is_ascii_lowercase()) || (c.is_ascii_digit() && !p.is_ascii_digit()))
        {
            words.push(' ');
        }
        words.push(c);
        prev = Some(c);
    }
    words + all
}

/// The held archetypes, as the sub-tab lists them; no pool counts an archetype.
fn archetype_budget(package: &SheetRulePackage, input: &CharacterInput) -> AbilityBudget {
    let held = held_archetypes(package, input)
        .into_iter()
        .map(|(id, rule)| HeldAbility { id, label: display_label(rule), category: None })
        .collect();
    AbilityBudget { subtab: SubTab::Archetype, pools: Vec::new(), held }
}

/// Every archetype of every class `input` holds, its pool the class; one replacing a class feature a held
/// archetype already replaces is refused, naming the feature and that archetype. An archetype whose
/// record names no replaced feature is read through its own converted gate instead (the held archetypes
/// held), which carries the corpus's exclusions and prerequisites.
fn archetype_options(package: &SheetRulePackage, input: &CharacterInput, base: &PilotBaseChassisComputation) -> Vec<AbilityOption> {
    let seed = HeldSeed::from_character(input, base);
    let facts = CharacterFacts::from_character(input, base).with_linked_picks(package, &seed);
    let held_set = held_set(package, &seed, &facts);
    let classes: Vec<String> = input.chosen.class_levels.iter().map(|class| id_slug(&class.class_id)).collect();
    let held: Vec<(String, &SheetRule, Vec<String>)> = held_archetypes(package, input)
        .into_iter()
        .map(|(id, rule)| {
            let slots = replaced_features(rule, &id);
            (id, rule, slots)
        })
        .collect();
    let mut options: Vec<AbilityOption> = archetype_records(package, &archetype_keys())
        .into_iter()
        .filter(|(_, rule)| classes.iter().any(|class| class == archetype_class(rule)))
        .map(|(id, rule)| {
            let class = archetype_class(rule);
            let slots = replaced_features(rule, &id);
            let mut unmet = held
                .iter()
                .filter(|(held_id, held_rule, _)| held_id != &id && archetype_class(held_rule) == class)
                .flat_map(|(_, held_rule, held_slots)| {
                    slots.iter().filter(|slot| held_slots.iter().any(|h| same_feature(slot, h))).map(move |slot| {
                        format!(
                            "replaces {}, which {} already replaces; a class feature is replaced once",
                            feature_words(slot, class),
                            display_label(held_rule)
                        )
                    })
                })
                .collect::<Vec<_>>();
            if unmet.is_empty() && !held.iter().any(|(held_id, _, _)| held_id == &id) {
                unmet = ability_prereqs::unmet(package, &held_set, &facts, &rule.applies);
            }
            let words: Vec<String> = slots.iter().map(|slot| feature_words(slot, class)).collect();
            let prose = catalog_prose(package, rule);
            AbilityOption {
                id,
                label: display_label(rule),
                pool: class.to_owned(),
                category: (!words.is_empty()).then(|| format!("Replaces {}", words.join(", "))),
                qualified: unmet.is_empty(),
                unmet,
                parameter: None,
                description: (!prose.trim().is_empty()).then_some(prose),
            }
        })
        .collect();
    options.sort_by(|a, b| (&a.pool, &a.label, &a.id).cmp(&(&b.pool, &b.label, &b.id)));
    options
}

/// The offer that lists a pool's members: a held grant's first, else any converted grant's. Only an offer of
/// records lists members: the Ranger's own `Favored Enemy` grant offers a number (its +2 to +10 bonus), not an enemy.
fn pool_offer<'a>(package: &'a SheetRulePackage, held_rules: &[&'a SheetRule], pool: &str) -> Option<&'a Choice> {
    let targets = |rule: &&SheetRule| matches!(&rule.target, Some(BonusTarget::Pool(p)) if p == pool);
    fn members(rule: &SheetRule) -> Option<&Choice> {
        rule.offers.as_ref().filter(|offer| matches!(offer.from, OptionSet::Rules { .. } | OptionSet::Domains))
    }
    held_rules
        .iter()
        .copied()
        .filter(targets)
        .find_map(members)
        .or_else(|| package.rules.values().filter(targets).find_map(members))
        // A level-1 `ABILITYSELECTION` pool's members are its chooser's offer (E3.13).
        .or_else(|| package.rules_of_kind("class_feature").filter(|rule| selection_offer(rule) == Some(pool)).find(|rule| selection_pool(package, rule).is_some()).and_then(|rule| rule.offers.as_ref()))
}

/// The member filter the converter keeps for a granted pool whose grants carry no offer
/// (`data/sheet_rules/_pool_members.json`, SD-43 E3.14): a `TYPE:A.B` category takes either type, an
/// `ABILITYLIST:` category the rules it names.
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct PoolMembers {
    pub(crate) member_pool: String,
    pub(crate) any_tags: Vec<String>,
    pub(crate) rules: Vec<String>,
}

pub(crate) fn pool_members(pool: &str) -> Option<&'static PoolMembers> {
    static MEMBERS: OnceLock<BTreeMap<String, PoolMembers>> = OnceLock::new();
    MEMBERS
        .get_or_init(|| serde_json::from_str(include_str!("../../data/sheet_rules/_pool_members.json")).expect("data/sheet_rules/_pool_members.json parses"))
        .get(pool)
}

/// Whether `rule` is a member of `pool`: its grant's offer selects it, else the converter's member filter does.
type MemberSelector<'a> = Box<dyn Fn(&SheetRule) -> bool + 'a>;

fn pool_member_selector<'a>(package: &'a SheetRulePackage, held_rules: &[&'a SheetRule], pool: &str) -> Option<MemberSelector<'a>> {
    if let Some(offer) = pool_offer(package, held_rules, pool) {
        return Some(Box::new(move |rule| offer_selects(offer, rule)));
    }
    let members = pool_members(pool)?;
    Some(Box::new(move |rule| {
        !rule.id.contains('#')
            && (members.rules.contains(&rule.id)
                || (rule.pool == members.member_pool && rule.tags.iter().any(|tag| members.any_tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))))
    }))
}

/// Every member of every class-feature pool this character holds, each with whether the record's own
/// gate is met (the converted-gate verdicts the Traits sub-tab reads). A pool lists the members its grant's
/// offer selects, else its category's ([`pool_member_selector`]). Two printings of one name are offered once,
/// the first by record id that qualifies, else the first by record id.
fn class_feature_options(package: &SheetRulePackage, input: &CharacterInput, base: &PilotBaseChassisComputation) -> Vec<AbilityOption> {
    let mut pools: Vec<String> = class_feature_grants(package, input).into_iter().filter(|(_, (n, _))| *n > 0).map(|(pool, _)| pool).collect();
    for pick in input.chosen.ability_selections.iter().filter(|pick| pick.subtab == SubTab::ClassFeatures.id()) {
        if !pools.contains(&pick.pool) {
            pools.push(pick.pool.clone());
        }
    }
    let seed = HeldSeed::from_character(input, base);
    let facts = CharacterFacts::from_character(input, base).with_linked_picks(package, &seed);
    let held = held_set(package, &seed, &facts);
    let held_rules: Vec<&SheetRule> = held.rules.keys().filter_map(|id| package.rule(id)).collect();
    let mut options: Vec<AbilityOption> = Vec::new();
    for pool in &pools {
        // A parameter choice's pool (E3.26) offers its one record, with the values its pick names as the target.
        if let Some((rule, parameter)) =
            held_rules.iter().find(|rule| parameter_pool(rule) == *pool).and_then(|rule| parameter_choice(package, rule).map(|parameter| (*rule, parameter)))
        {
            let prose = catalog_prose(package, rule);
            options.push(AbilityOption {
                id: format!("{pool}:{}", rule.id),
                label: display_label(rule),
                pool: pool.clone(),
                category: None,
                qualified: true,
                unmet: Vec::new(),
                description: (!prose.trim().is_empty()).then_some(prose),
                parameter: Some(parameter),
            });
            continue;
        }
        let Some(selects) = pool_member_selector(package, &held_rules, pool) else { continue };
        // label -> index in `options` of the record kept for it.
        let mut labels: HashMap<String, usize> = HashMap::new();
        for rule in package.rules.values().filter(|rule| selects(rule) && !rule.label.is_empty()) {
            let label = display_label(rule);
            let kept = labels.get(&label).copied();
            // Two records of one name are offered once: the first by id, unless a later one qualifies where it does
            // not (E3.20: the bloodrager's `Standard Bloodline` sorts before the sorcerer's, gated on another class).
            if kept.is_some_and(|at| options[at].qualified) {
                continue;
            }
            let unmet = ability_prereqs::unmet(package, &held, &facts, &rule.applies);
            if kept.is_some() && !unmet.is_empty() {
                continue;
            }
            let prose = catalog_prose(package, rule);
            let option = AbilityOption {
                id: format!("{pool}:{}", rule.id),
                label,
                pool: pool.clone(),
                category: None,
                qualified: unmet.is_empty(),
                unmet,
                parameter: None,
                description: (!prose.trim().is_empty()).then_some(prose),
            };
            match kept {
                Some(at) => options[at] = option,
                None => {
                    labels.insert(option.label.clone(), options.len());
                    options.push(option);
                }
            }
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
                parameter: None,
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
        parameter: None,
        description: None,
    }));
    // Two books printing one slug resolve to one record (`SheetRulePackage::find`): offer it once.
    options.sort_by(|a, b| a.id.cmp(&b.id));
    options.dedup_by(|a, b| a.id == b.id);
    options.sort_by(|a, b| (&a.pool, &a.label, &a.id).cmp(&(&b.pool, &b.label, &b.id)));
    options
}

/// The saved favored class: a `SelectedChoice` naming a class id (`class:fighter`).
pub const FAVORED_CLASS_CHOICE_ID: &str = "choice:favored_class";
/// The one-pick pool of the favored class and the per-level pool of its bonuses.
pub const FAVORED_CLASS_POOL: &str = "favored_class";
pub const FAVORED_BONUS_POOL: &str = "favored_class_bonus";
/// The two Core Rulebook bonuses (p.31), as the package converts them (`cr_abilities_class.lst:93-94`).
pub const FAVORED_HIT_POINT: &str = "core_rulebook:class_feature:favored_class_bonus_hit_point";
pub const FAVORED_SKILL_RANK: &str = "core_rulebook:class_feature:favored_class_bonus_skill_rank";
/// A held bonus's option id is its pool and record, then `@` and the character level it was taken at.
const PICK_LEVEL_SEPARATOR: char = '@';
/// The race alternates the corpus lacks (`artifacts/epic_3/E3.8/figures.py`: the oracle types 703
/// favored class bonus options, the package converts the 2 core ones).
const RACE_ALTERNATES_GAP: &str = "The corpus holds none of the race alternate favored class options (PCGen's Advanced Player's \
     Guide / Advanced Race Guide lists name 703 favored class bonus options; the converted books carry only the Core Rulebook's \
     +1 hit point and +1 skill rank), so only those two can be taken.";

/// The class id `input` saves as its favored class (Core Rulebook p.31: one, chosen at 1st level).
pub fn favored_class(input: &CharacterInput) -> Option<&str> {
    input.chosen.selected_choices.iter().find(|choice| choice.choice_set_id == FAVORED_CLASS_CHOICE_ID).map(|choice| choice.selection_id.as_str())
}

/// The character levels at which `input` took a level of its favored class, classes in saved order.
fn favored_levels(input: &CharacterInput) -> Vec<u8> {
    let Some(favored) = favored_class(input) else { return Vec::new() };
    let mut before = 0u8;
    let mut levels = Vec::new();
    for class in &input.chosen.class_levels {
        if class.class_id == favored {
            levels.extend((1..=class.level).map(|n| before.saturating_add(n)));
        }
        before = before.saturating_add(class.level);
    }
    levels
}

/// The favored class bonuses `input` holds for a level of its favored class, `(hit points, skill ranks)`:
/// the sheet's maximum hit points and skill point pool each add theirs (Core Rulebook p.31).
pub fn favored_class_bonus_taken(input: &CharacterInput) -> (i64, i64) {
    let levels = favored_levels(input);
    let picks = input.chosen.ability_selections.iter().filter(|pick| pick.subtab == SubTab::FavoredClass.id() && levels.contains(&pick.level));
    picks.fold((0, 0), |(hp, skill), pick| match pick.record_key.as_str() {
        FAVORED_HIT_POINT => (hp + 1, skill),
        FAVORED_SKILL_RANK => (hp, skill + 1),
        _ => (hp, skill),
    })
}

fn class_words(class_id: &str) -> String {
    let words = pool_words(class_id.strip_prefix("class:").unwrap_or(class_id));
    let mut chars = words.chars();
    chars.next().map_or_else(String::new, |first| first.to_uppercase().chain(chars).collect())
}

/// The Favored Class budget: one favored class, then one bonus per level of it. `imported` is the
/// bonuses a PCGen import already took as Custom grants (the desktop reads them from `custom.json`);
/// they spend the bonus pool so the same levels are not paid twice.
pub fn favored_class_budget(package: &SheetRulePackage, input: &CharacterInput, imported: i64) -> AbilityBudget {
    let favored = favored_class(input);
    let levels = favored_levels(input);
    let picks: Vec<_> = input.chosen.ability_selections.iter().filter(|pick| pick.subtab == SubTab::FavoredClass.id()).collect();
    let mut held: Vec<HeldAbility> = favored
        .map(|class| HeldAbility { id: format!("{FAVORED_CLASS_POOL}:{class}"), label: class_words(class), category: None })
        .into_iter()
        .collect();
    held.extend(picks.iter().map(|pick| HeldAbility {
        id: format!("{FAVORED_BONUS_POOL}:{}{PICK_LEVEL_SEPARATOR}{}", pick.record_key, pick.level),
        label: package.rule(&pick.record_key).map_or_else(|| pick.record_key.clone(), display_label),
        category: None,
    }));
    let mut bonus_sources: Vec<(Option<u8>, String)> = Vec::new();
    if let Some(class) = favored {
        bonus_sources.push((None, format!("{} levels ({})", class_words(class), levels.len())));
    }
    if imported > 0 {
        bonus_sources.push((None, format!("{imported} already taken by the PCGen import as Custom grants")));
    }
    let pools = vec![
        AbilityPool {
            subtab: SubTab::FavoredClass,
            id: FAVORED_CLASS_POOL.to_owned(),
            granted: 1,
            spent: i64::from(favored.is_some()),
            sources: vec![(Some(1), "every character (1)".to_owned())],
            opened: vec![1],
        },
        AbilityPool {
            subtab: SubTab::FavoredClass,
            id: FAVORED_BONUS_POOL.to_owned(),
            granted: levels.len() as i64,
            spent: picks.len() as i64 + imported,
            sources: bonus_sources,
            opened: levels,
        },
    ];
    AbilityBudget { subtab: SubTab::FavoredClass, pools, held }
}

/// The level the next favored class bonus is charged to: the first favored-class level with no bonus,
/// after the ones a PCGen import took (`imported`, the earliest levels).
pub fn favored_next_level(input: &CharacterInput, imported: i64) -> Option<u8> {
    let taken: Vec<u8> = input.chosen.ability_selections.iter().filter(|pick| pick.subtab == SubTab::FavoredClass.id()).map(|pick| pick.level).collect();
    favored_levels(input).into_iter().skip(usize::try_from(imported).unwrap_or(0)).find(|level| !taken.contains(level))
}

/// `(record id, level)` of a held bonus's option id (`favored_class_bonus:<record>@<level>`).
pub fn favored_pick_of(option_id: &str) -> Option<(&str, u8)> {
    let (record, level) = option_id.strip_prefix(FAVORED_BONUS_POOL)?.strip_prefix(':')?.rsplit_once(PICK_LEVEL_SEPARATOR)?;
    Some((record, level.parse().ok()?))
}

/// The favored class choices: each held class, the two core bonuses, each held bonus (its own id, so
/// the Selected list names it with its level), and the race alternates the corpus lacks, as a sentence.
fn favored_class_options(package: &SheetRulePackage, input: &CharacterInput) -> Vec<AbilityOption> {
    let favored = favored_class(input);
    let mut options: Vec<AbilityOption> = input
        .chosen
        .class_levels
        .iter()
        .map(|class| {
            let unmet: Vec<String> = favored
                .filter(|held| *held != class.class_id)
                .map(|held| format!("This character's favored class is already {}; remove it to choose another.", class_words(held)))
                .into_iter()
                .collect();
            AbilityOption {
                id: format!("{FAVORED_CLASS_POOL}:{}", class.class_id),
                label: class_words(&class.class_id),
                pool: FAVORED_CLASS_POOL.to_owned(),
                category: Some(format!("{} levels held", class.level)),
                qualified: unmet.is_empty(),
                unmet,
                parameter: None,
                description: Some(
                    "Core Rulebook p.31: each character chooses one favored class; whenever the character gains a level in it, \
                     the character receives either +1 hit point or +1 skill rank."
                        .to_owned(),
                ),
            }
        })
        .collect();
    let no_class = favored.is_none().then(|| "Choose a favored class first: a bonus is taken for a level of the favored class.".to_owned());
    for (record, words) in [(FAVORED_HIT_POINT, "+1 hit point: the sheet's maximum hit points add it."), (FAVORED_SKILL_RANK, "+1 skill rank: the skill point pool adds it.")] {
        let label = package.rule(record).map_or_else(|| record.to_owned(), display_label);
        options.push(AbilityOption {
            id: format!("{FAVORED_BONUS_POOL}:{record}"),
            label,
            pool: FAVORED_BONUS_POOL.to_owned(),
            category: None,
            qualified: no_class.is_none(),
            unmet: no_class.clone().into_iter().collect(),
            parameter: None,
            description: Some(format!("For one level of the favored class: {words}")),
        });
    }
    for pick in input.chosen.ability_selections.iter().filter(|pick| pick.subtab == SubTab::FavoredClass.id()) {
        options.push(AbilityOption {
            id: format!("{FAVORED_BONUS_POOL}:{}{PICK_LEVEL_SEPARATOR}{}", pick.record_key, pick.level),
            label: format!("{} (level {})", package.rule(&pick.record_key).map_or_else(|| pick.record_key.clone(), display_label), pick.level),
            pool: FAVORED_BONUS_POOL.to_owned(),
            category: None,
            qualified: true,
            unmet: Vec::new(),
            parameter: None,
            description: None,
        });
    }
    options.push(AbilityOption {
        id: format!("{FAVORED_BONUS_POOL}:race_alternates"),
        label: "Race alternate options".to_owned(),
        pool: FAVORED_BONUS_POOL.to_owned(),
        category: Some("not in the corpus".to_owned()),
        qualified: false,
        unmet: vec![RACE_ALTERNATES_GAP.to_owned()],
        parameter: None,
        description: Some(RACE_ALTERNATES_GAP.to_owned()),
    });
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

    /// The archetype tables spell one feature several ways: case, a numbered step, a whole `_ALL`.
    #[test]
    fn ability_budget_archetype_replaced_features_match_across_spellings_and_read_as_words() {
        assert!(same_feature("RogueTrapFinding", "RogueTrapfinding"));
        assert!(same_feature("RogueTrapSense", "RogueTrapSense3"));
        assert!(same_feature("FighterArmorTraining_ALL", "FighterArmorTraining3"));
        assert!(same_feature("FighterWeaponTraining_ALL", "FighterWeaponTraining"));
        assert!(!same_feature("RogueTalent2", "RogueTalent4"));
        assert!(same_feature("RogueTalent2", "rogueTalent2"));
        assert!(!same_feature("FighterBonusFeat1", "FighterBonusFeats"));
        assert_eq!(feature_words("RogueTrapSense1", "rogue"), "Trap Sense 1");
        assert_eq!(feature_words("FighterArmorTraining_ALL", "fighter"), "Armor Training (all)");
        assert_eq!(feature_words("CF_WitchHex", "witch"), "Witch Hex");
    }

    /// A Wizard 1 / Fighter 2 favoring Fighter: its bonuses open at character levels 2 and 3; a bonus
    /// saved at level 1 (a Wizard level) counts for nothing, and the race alternates are a sentence.
    #[test]
    fn ability_budget_favored_class_counts_only_levels_of_the_favored_class() {
        use crate::rules_core::character_input::{AbilitySelection, SelectedChoice};
        let mut input = level_one_with(&[]);
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: "class:wizard".to_owned(), level: 1 },
            CharacterClassLevel { class_id: "class:fighter".to_owned(), level: 2 },
        ];
        input.chosen.selected_choices.push(SelectedChoice { choice_set_id: FAVORED_CLASS_CHOICE_ID.to_owned(), selection_id: "class:fighter".to_owned() });
        let pick = |record: &str, level: u8| AbilitySelection { subtab: "favored_class".to_owned(), pool: FAVORED_BONUS_POOL.to_owned(), record_key: record.to_owned(), level, target: None };
        input.chosen.ability_selections = vec![pick(FAVORED_HIT_POINT, 1), pick(FAVORED_HIT_POINT, 2), pick(FAVORED_SKILL_RANK, 3)];
        assert_eq!(favored_levels(&input), [2, 3]);
        assert_eq!(favored_class_bonus_taken(&input), (1, 1));
        let budget = favored_class_budget(package(), &input, 0);
        assert_eq!(row(&budget, "abilities.budget.favored_class.favored_class_bonus.granted"), 2);
        assert_eq!(favored_next_level(&input, 0), None);
        assert_eq!(favored_pick_of("favored_class_bonus:core_rulebook:class_feature:favored_class_bonus_hit_point@2"), Some((FAVORED_HIT_POINT, 2)));
        let options = favored_class_options(package(), &input);
        let gap = options.iter().find(|o| o.id == "favored_class_bonus:race_alternates").expect("the gap sentence");
        assert!(!gap.qualified && gap.unmet[0].contains("703"), "{:?}", gap.unmet);
        let wizard = options.iter().find(|o| o.id == "favored_class:class:wizard").expect("wizard offered");
        assert!(!wizard.qualified && wizard.unmet[0].contains("Fighter"), "{:?}", wizard.unmet);
    }

    /// E0.1's mapping (`artifacts/epic_0/abilities-census.json` `subtabs.<tab>.pcgen_categories_player_editable`):
    /// Background 22 categories, Misc 219; the engine's category table holds exactly those, less the ones the
    /// generator's PI screen dropped (E3.24: Background 1), which the file counts.
    #[test]
    fn ability_budget_background_and_misc_categories_are_e0_1s() {
        let census = census();
        let file: serde_json::Value = serde_json::from_str(include_str!("../../data/ability_categories.json")).expect("parses");
        for (tab, subtab) in [("Background", SubTab::Background), ("Misc", SubTab::Misc)] {
            let want = census["subtabs"][tab]["pcgen_categories_player_editable"].as_u64().expect("census count");
            let have = ability_categories().iter().filter(|c| c.subtab == subtab.id()).count() as u64;
            let screened = file["pi_screened"][subtab.id()].as_u64().expect("screened count");
            assert_eq!(have + screened, want, "{tab}");
        }
    }

    /// E3.24: no category the engine ships (its pool, name or a member tag) carries a live PI blacklist term
    /// (`pi_screening::classify_field`, the screen the sibling generators apply).
    #[test]
    fn no_ability_category_carries_a_blacklist_term() {
        use crate::rules_core::{pi_screening::classify_field, shape_b_v1::License};
        let live: Vec<&str> = ability_categories()
            .iter()
            .flat_map(|c| [&c.pool, &c.name].into_iter().chain(&c.member_tags))
            .filter(|value| classify_field("ability_category", value).0 == License::PiRedacted)
            .map(String::as_str)
            .collect();
        assert!(live.is_empty(), "{} ability category value(s) carry a blacklist term: {live:?}", live.len());
    }

    fn pick(subtab: SubTab, option_id: &str) -> crate::rules_core::character_input::AbilitySelection {
        let (pool, record) = option_id.split_once(':').expect("pool:record");
        crate::rules_core::character_input::AbilitySelection {
            subtab: subtab.id().to_owned(),
            pool: pool.to_owned(),
            record_key: record.to_owned(),
            level: 1,
            target: None,
        }
    }

    /// Inner Sea World Guide: every character has one region of origin (`iswg_abilities.lst:6`, converted as
    /// `inner_sea_world_guide:ability:default`, `Pool(region_of_origin)` 1, always held). E0.1: Region of Origin
    /// is a Background category with 43 corpus records. It is a Background pool, not a Class Features one.
    #[test]
    fn ability_budget_background_every_character_takes_one_region_of_origin_and_a_second_is_refused() {
        let mut input = level_one_with(&[]);
        let budget = ability_budget(package(), &input, SubTab::Background);
        assert_eq!(row(&budget, "abilities.budget.background.region_of_origin.granted"), 1);
        assert_eq!(row(&budget, "abilities.budget.background.region_of_origin.remaining"), 1);
        assert!(ability_budget(package(), &input, SubTab::ClassFeatures).pool("region_of_origin").is_none(), "not a class feature pool");
        let base = crate::rules_core::pilot_compute::compute_pilot_base_chassis(&input);
        let options = ability_options(package(), &input, &base, SubTab::Background);
        let regions: Vec<&AbilityOption> = options.iter().filter(|o| o.pool == "region_of_origin" && o.qualified).collect();
        assert!(regions.len() >= 30, "{} qualified regions", regions.len());
        // An editable pool (PCGen leaves its size to the player: Monastery Reward) is offered with no count budget.
        assert!(options.iter().any(|o| o.pool == "monastery_reward"), "monastery rewards offered");
        assert!(budget.pool("monastery_reward").is_none(), "no count row for an editable pool");

        input.chosen.ability_selections.push(pick(SubTab::Background, &regions[0].id));
        let budget = ability_budget(package(), &input, SubTab::Background);
        assert_eq!(row(&budget, "abilities.budget.background.region_of_origin.spent"), 1);
        let refusal = budget.refusal(package(), &regions[1].id).expect("a second region refused");
        assert!(refusal.contains("region_of_origin pool"), "{refusal}");
        let monastery = options.iter().find(|o| o.pool == "monastery_reward").expect("a monastery reward");
        assert_eq!(budget.refusal(package(), &monastery.id), None, "an editable pool has no count to refuse on");
    }

    /// Horror Adventures: corruption is an optional rule (Misc, Optional Rules Systems, an editable pool);
    /// using it grants one corruption (`Corruption Type Choice`, E0.1: 11 corpus records), so the pool follows the pick.
    #[test]
    fn ability_budget_misc_an_optional_rule_picked_grants_the_corruption_type_choice() {
        let mut input = level_one_with(&[]);
        let base = crate::rules_core::pilot_compute::compute_pilot_base_chassis(&input);
        let options = ability_options(package(), &input, &base, SubTab::Misc);
        let rule = "optional_rules_systems:horror_adventures:ability:rules_use_corruption_standard";
        assert!(options.iter().any(|o| o.id == rule && o.qualified), "the corruption optional rule offered");
        for pool in ["afflictions", "gm_awards"] {
            assert!(options.iter().any(|o| o.pool == pool), "{pool} offered");
        }
        assert!(ability_budget(package(), &input, SubTab::Misc).pool("corruption_type_choice").is_none());

        input.chosen.ability_selections.push(pick(SubTab::Misc, rule));
        let budget = ability_budget(package(), &input, SubTab::Misc);
        assert_eq!(row(&budget, "abilities.budget.misc.corruption_type_choice.granted"), 1);
        let options = ability_options(package(), &input, &base, SubTab::Misc);
        let corruptions = options.iter().filter(|o| o.pool == "corruption_type_choice").count();
        assert!(corruptions >= 10, "{corruptions} corruptions offered");
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
    /// a grant row, or is a Feats-sub-tab pool, or is on E0.1's data-gap list; a level-1 `ABILITYSELECTION`
    /// pool (E3.13) is a grant row like any other. A grant row is proved by evaluation: the pool is granted > 0 to
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
        // A pool's grant rules: the rules targeting `Pool(<pool>)`, and the level-1 `ABILITYSELECTION` choosers
        // whose offer draws from it (E3.13).
        let grant_rules = |slug: &str| -> Vec<&SheetRule> {
            package()
                .rules
                .values()
                .filter(|rule| matches!(&rule.target, Some(BonusTarget::Pool(p)) if p == slug) || selection_pool(package(), rule) == Some(slug))
                .collect()
        };
        let mut rows = 0;
        let mut feat_pools = 0;
        let mut listed = 0;
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
            (slug.clone(), granted_to(class_feature_grants_holding(package(), &input_for(&fixture, class, 20), principals, SubTab::ClassFeatures)).any(|p| p == *slug))
        });
        let not_proved: Vec<String> = proved.iter().filter(|(_, ok)| !ok).map(|(slug, _)| slug.clone()).collect();
        eprintln!(
            "census: {} classes; class-owned pools: grant row on a class seed {rows}, grant row through a choice {}, Feats sub-tab {feat_pools}, data gap {listed}; not proved {not_proved:?}; unexplained {unexplained:?}",
            classes.len(),
            proved.len() - not_proved.len(),
        );
        assert!(not_proved.is_empty() && unexplained.is_empty(), "not proved {not_proved:?}; unexplained {unexplained:?}");
    }

    const FEAT_POOL_ID: &str = "feat";

    #[test]
    fn ability_budget_class_features_now_serves_the_same_numbers_without_levels() {
        let input = input_for(&load_sweep_fixture().expect("census fixture"), "rogue", 6);
        let numbers = |budget: &AbilityBudget| budget.explanation_rows().into_iter().map(|row| (row.id, row.value)).collect::<Vec<_>>();
        let now = ability_budget_now(package(), &input, SubTab::ClassFeatures);
        assert_eq!(numbers(&now), numbers(&ability_budget(package(), &input, SubTab::ClassFeatures)));
        assert!(now.pool("rogue_talent").expect("pool").sources.iter().all(|(level, _)| level.is_none()));
    }

    // "At 1st level, a sorcerer chooses a bloodline" (Core Rulebook, Sorcerer). The seeded creation choice
    // (`choice:sorcerer_bloodline -> bloodline:arcane`) is that one pick: it fills the pool, lists as held, and a
    // second bloodline is refused for room, so a bloodline is never held twice.
    #[test]
    fn sorcerer_bloodline_pool_is_granted_one_at_level_one() {
        let input = input_for(&load_sweep_fixture().expect("census fixture"), "sorcerer", 1);
        let budget = ability_budget(package(), &input, SubTab::ClassFeatures);
        assert_eq!(row(&budget, "abilities.budget.class_features.sorcerer_bloodline.granted"), 1);
        assert_eq!(budget.pool("sorcerer_bloodline").expect("pool").opened, [1], "granted at 1st level");
        assert_eq!(row(&budget, "abilities.budget.class_features.sorcerer_bloodline.spent"), 1, "the creation choice is the pick");
        let arcane = "sorcerer_bloodline:core_rulebook:pool_option:sorcerer_bloodline_arcane_bloodline";
        let draconic = "sorcerer_bloodline:core_rulebook:pool_option:sorcerer_bloodline_draconic_bloodline";
        let picks = creation_choice_picks(package(), &input);
        let kind = "bloodline:core_rulebook:class_feature:sorcerer_standard_bloodline_selection";
        assert_eq!(picks.iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(), [arcane, kind], "the bloodline and its kind (E3.20)");
        assert_eq!(picks[0].1.choice_set_id, "choice:sorcerer_bloodline");
        let options = ability_options(package(), &input, &compute_pilot_base_chassis(&input), SubTab::ClassFeatures);
        assert!(options.iter().any(|o| o.id == draconic), "the pool's members are the offer's");
        assert!(budget.refusal(package(), arcane).expect("held").contains("already holds"));
        assert!(budget.refusal(package(), draconic).expect("full").contains("sorcerer_bloodline pool"));
        let (old, new) = creation_choice_swap(package(), &input, draconic).expect("draconic renames the creation choice");
        assert_eq!((old.selection_id.as_str(), new.selection_id.as_str()), ("bloodline:arcane", "bloodline:draconic"));
        assert!(creation_choice_swap(package(), &input, arcane).is_none(), "the held pick swaps nothing");

        let mut picked = input.clone();
        picked.chosen.selected_choices.retain(|c| c.choice_set_id != "choice:sorcerer_bloodline");
        picked.chosen.ability_selections.push(pick(SubTab::ClassFeatures, draconic));
        let budget = ability_budget(package(), &picked, SubTab::ClassFeatures);
        assert_eq!(row(&budget, "abilities.budget.class_features.sorcerer_bloodline.remaining"), 0);
        assert!(creation_choice_picks(package(), &picked).is_empty());
    }

    // "A sorcerer must pick one bloodline upon taking her first level of sorcerer" (Core Rulebook, Sorcerer). PCGen's
    // `Sorcerer ~ Standard Bloodline` grants one `Bloodline` pick, the kind of bloodline (`Standard Bloodline`, whose own
    // chooser offers the bloodline itself). The held bloodline implies its kind: the pool is filled by the chooser it
    // was picked through, and that kind's option reads the verdict of the gate that opens it (`chooser_offered`).
    #[test]
    fn sorcerer_bloodline_kind_pool_is_filled_by_the_held_bloodline_and_its_kind_qualifies() {
        let input = input_for(&load_sweep_fixture().expect("census fixture"), "sorcerer", 1);
        let budget = ability_budget(package(), &input, SubTab::ClassFeatures);
        assert_eq!(row(&budget, "abilities.budget.class_features.bloodline.granted"), 1);
        assert_eq!(row(&budget, "abilities.budget.class_features.bloodline.remaining"), 0, "the held bloodline implies its kind");
        let standard = "bloodline:core_rulebook:class_feature:sorcerer_standard_bloodline_selection";
        assert!(creation_choice_picks(package(), &input).iter().any(|(id, _)| id == standard));
        // The kind record is `MULT:YES`, so a second one is refused for room, not as already held.
        assert!(budget.refusal(package(), standard).expect("full").contains("bloodline pool"));

        let base = compute_pilot_base_chassis(&input);
        let seed = HeldSeed::from_character(&input, &base);
        let facts = CharacterFacts::from_character(&input, &base).with_linked_picks(package(), &seed);
        let held = held_set(package(), &seed, &facts);
        let options = ability_options(package(), &input, &base, SubTab::ClassFeatures);
        let option = options.iter().find(|o| o.id == standard).expect("Standard Bloodline is the sorcerer's option");
        assert!(option.qualified, "{:?}", option.unmet);
        // No option whose own gate includes against the held set reads unmet.
        let disagree: Vec<(&str, &Vec<String>)> = options
            .iter()
            .filter(|o| !o.qualified)
            .filter(|o| package().rule(o.id.split_once(':').expect("pool:record").1).is_some_and(|r| evaluate_applies(&r.applies, &held, package(), &facts, EvalContext::default()) == crate::rules_core::sheet_rule::Gate::Include))
            .map(|o| (o.id.as_str(), &o.unmet))
            .collect();
        assert!(disagree.is_empty(), "{disagree:?}");
    }

    // "An oracle's mystery grants ... revelations" (Advanced Player's Guide, Oracle). PCGen's `Flame Mystery Revelation`
    // takes any ability typed `FlameRevelation` or `GeneralRevelation` (`apg_abilitycategories.lst:54`, `TYPE:A.B` is
    // either type, `AbilityCategory.populate`), so the pool lists both kinds.
    #[test]
    fn oracle_flame_mystery_lists_revelation_options() {
        let mut input = input_for(&load_sweep_fixture().expect("census fixture"), "oracle", 1);
        input.chosen.selected_choices.retain(|c| c.choice_set_id != "choice:oracle_mystery");
        input.chosen.ability_selections.push(pick(SubTab::ClassFeatures, "oracle_mystery:advanced_players_guide:class_feature:oracle_flame_mystery"));
        assert!(row(&ability_budget(package(), &input, SubTab::ClassFeatures), "abilities.budget.class_features.flame_mystery_revelation.granted") >= 1);
        let options = ability_options(package(), &input, &compute_pilot_base_chassis(&input), SubTab::ClassFeatures);
        let flame: Vec<&AbilityOption> = options.iter().filter(|o| o.pool == "flame_mystery_revelation").collect();
        let typed = |tag: &str| flame.iter().any(|o| package().rule(o.id.split_once(':').expect("pool:record").1).is_some_and(|r| r.tags.iter().any(|t| t == tag)));
        assert!(typed("FlameRevelation") && typed("GeneralRevelation"), "Flame revelation options: {:?}", flame.iter().map(|o| &o.label).collect::<Vec<_>>());
    }

    // "At 1st level, a ranger selects a creature type from the ranger favored enemies table" (Core Rulebook, Ranger).
    // The Ranger's own grant offers the bonus number, so the pool lists the enemies another grant offers.
    #[test]
    fn ranger_1_lists_favored_enemy_options() {
        let input = input_for(&load_sweep_fixture().expect("census fixture"), "ranger", 1);
        let options = ability_options(package(), &input, &compute_pilot_base_chassis(&input), SubTab::ClassFeatures);
        assert!(options.iter().any(|o| o.pool == "favored_enemy"), "no favored enemy option");
    }

    /// Acceptance census (E3.14): every pool E0.1 counts with a class among its granting records, granted by a
    /// converted `Pool` rule and not a Feats-sub-tab pool, lists at least one member, or is on E0.1's data-gap list.
    #[test]
    fn ability_budget_class_features_census_every_granted_class_owned_pool_lists_a_member_or_is_a_listed_gap() {
        let census = census();
        let gaps: Vec<&str> = census["data_gap_list"].as_array().expect("gap list").iter().filter_map(|gap| gap["pool"].as_str()).collect();
        let (mut pools, mut listed, mut gap) = (0, 0, 0);
        let mut memberless: Vec<String> = Vec::new();
        for (name, pool) in census["class_feature_pools"]["pools"].as_object().expect("pools") {
            if pool["granted_by"].get("class").is_none() {
                continue;
            }
            let slug = crate::rules_core::sheet_rule::id_slug(name);
            let grants: Vec<&SheetRule> =
                package().rules.values().filter(|rule| matches!(&rule.target, Some(BonusTarget::Pool(p)) if *p == slug)).collect();
            if grants.is_empty() || slug == FEAT_POOL_ID || grants.iter().any(|rule| feat_pool_of(rule).is_some()) {
                continue;
            }
            pools += 1;
            let members = pool_member_selector(package(), &[], &slug)
                .map_or(0, |selects| package().rules.values().filter(|rule| selects(rule) && !rule.label.is_empty()).count());
            if members > 0 {
                listed += 1;
            } else if gaps.contains(&name.as_str()) {
                gap += 1;
            } else {
                memberless.push(name.clone());
            }
        }
        eprintln!("census: granted class-owned class-feature pools {pools}; listing members {listed}; data gap {gap}; memberless {memberless:?}");
        assert!(memberless.is_empty(), "pools listing no member: {memberless:?}");
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

    /// The 29 class-granted pools E3.12 gave corpus members: each Class Features pool lists at least one member to
    /// a character holding its grants (the Feats-sub-tab pools are `feat_budget`'s test). A grant offering a number
    /// (the Lantern Bearer's +2 to +4) lists no member, so its pool is read through its category's member filter.
    #[test]
    fn ability_budget_class_features_every_member_typed_pool_lists_a_member_to_a_holder_of_its_grant() {
        const POOLS: [&str; 29] = [
            "Advanced Masked Performer Talent", "Autumn Witch Hex Choice", "Bloody Jake Terrain", "Cloaked Wolf Bonus Feat",
            "Death Slayer Bonus Feat", "Druid Bear Shaman Feat", "Druid Eagle Shaman Feat", "Druid Lion Shaman Feat",
            "Druid Serpent Shaman Feat", "Druid Wolf Shaman Feat", "Finesse Marksman Bonus Feat", "Hexenhammer Major Hex Choice",
            "Hexenhammer Minor Hex Choice", "Lantern Bearer Favored Enemy", "Lantern Bearer Favored Enemy Bonus",
            "Life Channeler Domain", "Mammoth Rider Born Survivor Feat", "Masked Performer Talent", "Psionic Fighter Starting Feat",
            "Sniper Marksman Bonus Feat", "Spring Witch Hex Choice", "Summer Witch Hex Choice", "Swashbuckler Weapon Proficiency",
            "Tempest Druid Domain Choice", "Undead Master Arcane Bond", "Vexing Daredevil Bonus Stare Feat",
            "Volley Marksman Bonus Feat", "Winter Witch Hex Choice", "Winter Witch Patron",
        ];
        let (mut class_features, mut memberless) = (0, Vec::new());
        for name in POOLS {
            let slug = crate::rules_core::sheet_rule::id_slug(name);
            let grants: Vec<&SheetRule> =
                package().rules.values().filter(|rule| matches!(&rule.target, Some(BonusTarget::Pool(p)) if *p == slug)).collect();
            assert!(!grants.is_empty(), "{name}: no converted grant");
            if grants.iter().any(|rule| feat_pool_of(rule).is_some()) {
                continue;
            }
            class_features += 1;
            let members = pool_member_selector(package(), &grants, &slug)
                .map_or(0, |selects| package().rules.values().filter(|rule| selects(rule) && !rule.label.is_empty()).count());
            if members == 0 {
                memberless.push(name);
            }
        }
        assert!(memberless.is_empty(), "{} of {class_features} Class Features pools list no member: {memberless:?}", memberless.len());

        // End to end: "a lantern bearer selects a creature type" at 2nd level (Adventurer's Guide, Lantern Bearer).
        let input = input_for(&load_sweep_fixture().expect("census fixture"), "lantern_bearer", 2);
        let options = ability_options(package(), &input, &compute_pilot_base_chassis(&input), SubTab::ClassFeatures);
        let enemies: Vec<&str> = options.iter().filter(|o| o.pool == "lantern_bearer_favored_enemy").map(|o| o.label.as_str()).collect();
        assert!(enemies.contains(&"Aberration"), "Lantern Bearer 2 favored enemy options: {enemies:?}");
    }

    /// One named site of E3.26's population: Minor Magic (Core Rulebook, Rogue Talents): "a rogue with this talent
    /// gains the ability to cast a 0-level spell from the sorcerer/wizard spell list" (`CHOOSE:SPELLS`). The Class
    /// Features sub-tab offers the spell, saves it as the pick's target, and the character's facts read it.
    #[test]
    fn ability_budget_class_features_minor_magic_takes_its_spell_as_the_picks_target() {
        const MINOR_MAGIC: &str = "core_rulebook:class_feature:rogue_talent_minor_magic";
        let mut input = input_for(&load_sweep_fixture().expect("census fixture"), "rogue", 2);
        input.chosen.ability_selections.push(crate::rules_core::character_input::AbilitySelection {
            subtab: SubTab::ClassFeatures.id().to_owned(),
            pool: "rogue_talent".to_owned(),
            record_key: MINOR_MAGIC.to_owned(),
            level: 2,
            target: None,
        });
        let pool = "rogue_talent_minor_magic_choice";
        assert_eq!(row(&ability_budget(package(), &input, SubTab::ClassFeatures), &format!("abilities.budget.class_features.{pool}.granted")), 1);
        let base = compute_pilot_base_chassis(&input);
        let options = ability_options(package(), &input, &base, SubTab::ClassFeatures);
        let option = options.iter().find(|option| option.pool == pool).unwrap_or_else(|| panic!("no {pool} option"));
        assert_eq!(option.id, format!("{pool}:{MINOR_MAGIC}"));
        let values = &option.parameter.as_ref().expect("Minor Magic takes a spell").values;
        assert!(values.contains(&"Acid Splash".to_owned()) && !values.contains(&"Fireball".to_owned()), "{values:?}");

        input.chosen.ability_selections.push(crate::rules_core::character_input::AbilitySelection {
            subtab: SubTab::ClassFeatures.id().to_owned(),
            pool: pool.to_owned(),
            record_key: MINOR_MAGIC.to_owned(),
            level: 2,
            target: Some("Acid Splash".to_owned()),
        });
        let budget = ability_budget(package(), &input, SubTab::ClassFeatures);
        assert_eq!(row(&budget, &format!("abilities.budget.class_features.{pool}.remaining")), 0);
        let held = format!("{pool}:{MINOR_MAGIC}{TARGET_SEPARATOR}Acid Splash");
        assert!(budget.held.iter().any(|pick| pick.id == held && pick.label == "Minor Magic (Acid Splash)"), "{:?}", budget.held);
        assert!(budget.refusal(package(), &held).is_some(), "the same spell twice is refused");
        let facts = CharacterFacts::from_character(&input, &compute_pilot_base_chassis(&input));
        assert_eq!(facts.choices.get(MINOR_MAGIC).map(Vec::as_slice), Some(&[("acid_splash".to_owned(), "Acid Splash".to_owned())][..]));
    }

    /// One named site of E3.27's population: Master Caster (Adventurer's Guide, Aspis Agency Secret,
    /// `ADD:SPELLCASTER|Bard,Inquisitor,Mesmerist`): the agent's spellcasting advances in one of three classes. The
    /// Class Features sub-tab offers the three as the pick's target; a typed language list (`Raven Master ~ Companion
    /// Abilities`, `ADD:LANGUAGE|1|TYPE=Spoken`) offers the spoken languages.
    #[test]
    fn ability_budget_class_features_an_add_list_is_a_parameter_pick() {
        let master_caster = package().rule("adventurers_guide:class_feature:aspis_agency_secret_master_caster").expect("Master Caster");
        let classes = parameter_choice(package(), master_caster).expect("Master Caster takes a class").values;
        assert_eq!(classes, ["Bard", "Inquisitor", "Mesmerist"]);
        let raven = package().rule("adventurers_guide:class_feature:raven_master_companion_abilities").expect("Raven Master");
        let languages = parameter_choice(package(), raven).expect("Raven Master takes a language").values;
        assert!(languages.contains(&"Aquan".to_owned()) && languages.contains(&"Common".to_owned()), "{languages:?}");
    }

    /// How a converted `ADD:<kind>` naming one choice grants it (E3.27), by the site's kind: `ABILITY` / `VFEAT` / `FEAT`
    /// grant another rule held with this one, `LANGUAGE` / `EQUIP` grant the language or item fact, `SPELLCASTER` prints
    /// its spellcasting class. `None` for a record that offers a choice or carries no such grant.
    fn one_option_add(rule: &SheetRule, kind: &str) -> Option<&'static str> {
        if rule.offers.is_some() {
            return None;
        }
        let fact = |matches: fn(&Fact) -> bool| rule.grants.iter().any(|effect| matches!(effect, Effect::FactGrant(fact) if matches(fact)));
        match kind {
            "ABILITY" | "VFEAT" | "FEAT" => package()
                .rules
                .values()
                .any(|other| other.granted_by.iter().any(|grant| grant.by == Granter::Rule(rule.id.clone())))
                .then_some("the one ability is held with the record"),
            "LANGUAGE" => fact(|f| matches!(f, Fact::Language(_))).then_some("the one language is granted"),
            "EQUIP" => fact(|f| matches!(f, Fact::Equipment(_))).then_some("the one item prints as granted"),
            "SPELLCASTER" => rule
                .prose
                .iter()
                .flat_map(|segment| &segment.pieces)
                .any(|piece| matches!(piece, crate::rules_core::sheet_rule::ProsePiece::Text(text) if text.starts_with("Casts spells as a ")))
                .then_some("the one spellcasting class prints"),
            _ => None,
        }
    }

    /// The kanban cards that own each cause a class-granted choice site is not reachable for (E3.MC, intake §22c), by
    /// the cause's opening words.
    const UNREACHABLE_CARDS: &[(&str, &str)] = &[
        ("domain_count:", "E3.28"),
        ("abilityselection of PCGen's base category", "E3.29"),
        ("abilitypool: the granting record carries no converted Pool line", "E3.29"),
        ("no class of the granting record to seed", "E3.30"),
        ("the pool is not granted", "E3.30"),
        ("the pool is granted but lists no member", "E3.30"),
        ("a member pick is not held after save and reload", "E3.30"),
    ];

    /// Acceptance census (E3.MC, intake §22c): every one of E0.1's class-granted choice sites
    /// (`artifacts/epic_3/E3.MC/choice_sites.json`, written from E0.1's instrument by `choice_sites.py`) is reachable,
    /// sits on its own sub-tab (a feat pool on Feats, the favored class bonus on Favored Class), is on E0.1's data-gap
    /// list, or is unreachable for a cause a kanban card names. Reachable: a character of the record's class at level 20
    /// holding the granting record's rules is granted the pool, the pool lists a member, and a pick of that member saved
    /// through `SavedCharacterStore` and loaded again is held by the Class Features budget.
    #[test]
    fn ability_budget_class_features_census_every_class_granted_choice_site_is_reachable_or_a_named_card() {
        use crate::rules_core::sheet_rule::split_rule_id;
        use crate::saved_character::local_store::SavedCharacterStore;
        use crate::saved_character::{SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION};
        use std::collections::BTreeSet;
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.MC");
        let sites: Vec<serde_json::Value> =
            serde_json::from_str(&std::fs::read_to_string(dir.join("choice_sites.json")).expect("choice sites")).expect("choice sites JSON");
        let census = census();
        assert_eq!(sites.len() as u64, census["class_granted_choices"]["total"].as_u64().expect("E0.1 total"), "E0.1's site count");
        let gaps: Vec<&str> = census["data_gap_list"].as_array().expect("gap list").iter().filter_map(|gap| gap["pool"].as_str()).collect();
        let fixture = load_sweep_fixture().expect("census fixture");
        let mut classes: Vec<&str> =
            package().rules_of_kind("class").filter(|rule| !rule.id.contains('#')).map(|rule| split_rule_id(&rule.id).2).collect();
        classes.sort_unstable();
        classes.dedup();
        let archetypes = archetype_records(package(), &archetype_keys());
        let scratch = std::env::temp_dir().join(format!("codex-choice-site-census-{}", std::process::id()));
        let offer_shape = |rules: &[&SheetRule]| -> String {
            rules
                .iter()
                .find_map(|rule| rule.offers.as_ref())
                .and_then(|offer| serde_json::to_value(&offer.from).ok())
                .map(|from| from.as_object().and_then(|o| o.keys().next().cloned()).or_else(|| from.as_str().map(str::to_owned)).unwrap_or_default())
                .unwrap_or_else(|| "nothing".to_owned())
        };
        let verdict = |(index, site): &(usize, &serde_json::Value)| -> (String, String) {
            let mechanism = site["mechanism"].as_str().expect("mechanism");
            let target = site["target"].as_str().expect("target");
            let rules: Vec<&SheetRule> = site["rules"].as_array().expect("rules").iter().filter_map(|id| package().rule(id.as_str()?)).collect();
            let targets = |rule: &SheetRule, pool: &str| matches!(&rule.target, Some(BonusTarget::Pool(p)) if p == pool);
            // A parameter choice (E3.26) is taken in its record's own pool, its pick naming one of the offered values.
            // An `ADD:` of a list (E3.27) is the same: its record's own offer, taken as the pick's target.
            let parameter = rules
                .iter()
                .find_map(|rule| parameter_choice(package(), rule).map(|choice| (*rule, choice)))
                .filter(|_| ["parameter_choose", "add_token"].contains(&mechanism));
            // An `ADD:` naming one choice is PCGen's automatic pick, a grant (E3.27, decisions.md §32): the record holds
            // the ability, grants the language or item, or prints its spellcasting class.
            if mechanism == "add_token"
                && parameter.is_none()
                && let Some(how) = rules.iter().find_map(|rule| one_option_add(rule, target))
            {
                return ("granted (one option)".to_owned(), how.to_owned());
            }
            let pools: Vec<String> = match mechanism {
                "parameter_choose" | "add_token" => parameter.iter().map(|(rule, _)| parameter_pool(rule)).collect(),
                "abilitypool" => Some(id_slug(target)).filter(|slug| rules.iter().any(|rule| targets(rule, slug))).into_iter().collect(),
                "domain_count" => rules
                    .iter()
                    .filter_map(|rule| match (&rule.target, &rule.offers) {
                        (Some(BonusTarget::Pool(p)), offer) if p.contains("domain") || offer.as_ref().is_some_and(|o| matches!(o.from, OptionSet::Domains)) => Some(p.clone()),
                        _ => None,
                    })
                    .collect(),
                _ => rules.iter().filter_map(|rule| selection_pool(package(), rule)).map(str::to_owned).collect(),
            };
            let unreachable = |cause: String| ("unreachable".to_owned(), cause);
            if pools.is_empty() {
                return match mechanism {
                    "abilitypool" => unreachable("abilitypool: the granting record carries no converted Pool line for the pool".to_owned()),
                    "abilityselection" if ["FEAT", "Internal", "Special Ability"].contains(&target) => {
                        unreachable(format!("abilityselection of PCGen's base category {target}: no Class Features pool"))
                    }
                    _ => unreachable(format!("{mechanism}: no Class Features pool (the record offers {})", offer_shape(&rules))),
                };
            }
            let mut last = unreachable(String::new());
            for pool in &pools {
                let grants: Vec<&SheetRule> = rules.iter().copied().filter(|rule| targets(rule, pool) || selection_pool(package(), rule) == Some(pool.as_str())).collect();
                if pool == FEAT_POOL_ID || grants.iter().any(|rule| feat_pool_of(rule).is_some()) {
                    return ("own sub-tab".to_owned(), "Feats".to_owned());
                }
                if pool == FAVORED_BONUS_POOL {
                    return ("own sub-tab".to_owned(), "Favored Class".to_owned());
                }
                if gaps.contains(&target) {
                    return ("data gap".to_owned(), target.to_owned());
                }
                // The seed: an archetype the record's owner names is held on its class; else the record's class.
                let owner = id_slug(site["owner"].as_str().unwrap_or_default());
                let archetype = archetypes.iter().find(|(_, rule)| split_rule_id(&rule.id).2.split("_archetype_").nth(1) == Some(owner.as_str()));
                let class = archetype
                    .map(|(_, rule)| archetype_class(rule).to_owned())
                    .filter(|class| classes.contains(&class.as_str()))
                    .or_else(|| rules.iter().find_map(|rule| class_of(rule, &classes)))
                    .or_else(|| classes.contains(&owner.as_str()).then_some(owner.clone()));
                let Some(class) = class else {
                    last = unreachable("no class of the granting record to seed".to_owned());
                    continue;
                };
                let mut principals: Vec<String> =
                    rules.iter().flat_map(|rule| std::iter::once(rule.id.split('#').next().unwrap_or(&rule.id).to_owned()).chain(rules_the_gate_reads(rule))).collect();
                principals.extend(archetype.map(|(_, rule)| rule.id.clone()));
                principals.sort();
                principals.dedup();
                let mut input = input_for(&fixture, &class, 20);
                if let Some((selection, _)) = archetype {
                    input.chosen.selected_choices.push(SelectedChoice { choice_set_id: ARCHETYPE_CHOICE_ID.to_owned(), selection_id: selection.clone() });
                }
                let granted = class_feature_grants_holding(package(), &input, &principals, SubTab::ClassFeatures).get(pool).is_some_and(|(n, _)| *n > 0);
                if !granted {
                    last = unreachable("the pool is not granted to its class at 20 holding the granting record".to_owned());
                    continue;
                }
                let member = match &parameter {
                    Some((rule, _)) => Some(rule.id.clone()),
                    None => pool_member_selector(package(), &grants, pool)
                        .and_then(|selects| package().rules.values().filter(|rule| selects(rule) && !rule.label.is_empty()).map(|rule| rule.id.clone()).min()),
                };
                // The first value the corpus lists, else a value the player writes in.
                let target = parameter.as_ref().map(|(_, choice)| choice.values.first().cloned().unwrap_or_else(|| "Written in".to_owned()));
                let Some(member) = member else {
                    last = unreachable("the pool is granted but lists no member".to_owned());
                    continue;
                };
                let mut picked = input.clone();
                picked.chosen.ability_selections.push(crate::rules_core::character_input::AbilitySelection {
                    subtab: SubTab::ClassFeatures.id().to_owned(),
                    pool: pool.clone(),
                    record_key: member.clone(),
                    level: 20,
                    target: target.clone(),
                });
                let root = scratch.join(index.to_string());
                let envelope = SavedCharacterEnvelope {
                    character_id: format!("choice-site-{index}"),
                    revision_id: format!("choice-site-{index}.rev.1"),
                    revision_kind: SavedCharacterRevisionKind::Authoritative,
                    saved_at: "2026-11-25T00:00:00Z".to_owned(),
                    schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
                    app_or_runtime_version: "codex-dev".to_owned(),
                    content_or_rules_provenance: "pf1.core_rulebook".to_owned(),
                    game_system: "pf1".to_owned(),
                    latest_authoritative_revision_ref: format!("choice-site-{index}.rev.1"),
                    display_label: "Choice site census".to_owned(),
                    character_input: picked,
                };
                SavedCharacterStore::save(&envelope, &root).expect("save");
                let reloaded = SavedCharacterStore::load(&root).expect("load").character_input;
                let _ = std::fs::remove_dir_all(&root);
                let id = match &target {
                    Some(target) => format!("{pool}:{member}{TARGET_SEPARATOR}{target}"),
                    None => format!("{pool}:{member}"),
                };
                if ability_budget_now(package(), &reloaded, SubTab::ClassFeatures).held.iter().any(|held| held.id == id) {
                    return ("reachable".to_owned(), String::new());
                }
                last = unreachable("a member pick is not held after save and reload".to_owned());
            }
            last
        };
        let indexed: Vec<(usize, &serde_json::Value)> = sites.iter().enumerate().collect();
        let verdicts = in_parallel(&indexed, verdict);
        let _ = std::fs::remove_dir_all(&scratch);
        let mut table: BTreeMap<(String, String, String), usize> = BTreeMap::new();
        let mut examples: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for ((_, site), (state, cause)) in indexed.iter().zip(&verdicts) {
            let mechanism = site["mechanism"].as_str().unwrap_or_default().to_owned();
            *table.entry((mechanism, state.clone(), cause.clone())).or_default() += 1;
            let seen = examples.entry(cause.clone()).or_default();
            if state == "unreachable" && seen.len() < 3 {
                seen.push(format!("{} -> {}", site["path"].as_str().unwrap_or_default(), site["target"].as_str().unwrap_or_default()));
            }
        }
        let mut uncovered: BTreeSet<String> = BTreeSet::new();
        eprintln!("choice-site census ({} sites): mechanism | state | cause | sites | card", sites.len());
        for ((mechanism, state, cause), n) in &table {
            let card = UNREACHABLE_CARDS.iter().find(|(c, _)| !cause.is_empty() && cause.starts_with(c)).map(|(_, card)| *card);
            if state == "unreachable" && card.is_none() {
                uncovered.insert(cause.clone());
            }
            eprintln!("| {mechanism} | {state} | {cause} | {n} | {} |", card.unwrap_or("-"));
        }
        for cause in &uncovered {
            eprintln!("uncovered cause {cause:?}, e.g. {:?}", examples.get(cause));
        }
        assert_eq!(table.values().sum::<usize>(), sites.len());
        let kanban = std::fs::read_to_string(dir.join("../../../kanban.md")).expect("kanban.md");
        let missing: Vec<&str> = UNREACHABLE_CARDS.iter().map(|(_, card)| *card).filter(|card| !kanban.contains(&format!("\n| {card} |"))).collect();
        assert!(missing.is_empty(), "cards the census names have no kanban.md row: {missing:?}");
        assert!(uncovered.is_empty(), "{} unreachable cause(s) no card names: {uncovered:?}", uncovered.len());
    }
}
