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

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::feat_identity;
use crate::rules_core::feat_prereqs::converted_gate::{verdicts, TermVerdict};
use crate::rules_core::pilot_compute::{ComputationExplanation, PilotBaseChassisComputation};
use crate::rules_core::sheet_rule::{
    display_label, held_set, id_slug, Applies, CharacterFacts, HeldSeed, Holdable, SheetRule, SheetRulePackage,
};
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
}

impl SubTab {
    pub const ALL: [SubTab; 1] = [SubTab::Traits];

    pub fn id(self) -> &'static str {
        match self {
            SubTab::Traits => "traits",
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
}

impl AbilityPool {
    pub fn remaining(&self) -> i64 {
        self.granted - self.spent
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
    /// when the budget has room. Prerequisites are [`ability_options`]' `unmet`.
    pub fn refusal(&self, package: &SheetRulePackage, option_id: &str) -> Option<String> {
        if self.held.iter().any(|held| held.id == option_id) {
            return Some(format!("This character already holds '{option_id}'."));
        }
        let pool_id = pool_of(option_id);
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
        let category = trait_category(package, option_id)?;
        let clash = self.held.iter().find(|held| held.category.as_deref() == Some(category.as_str()))?;
        Some(format!(
            "A character takes one trait of each category: {} is already this character's {} trait.",
            clash.label,
            category_words(&category)
        ))
    }
}

fn pool_of(option_id: &str) -> &'static str {
    if is_drawback(option_id) { DRAWBACKS_POOL } else { TRAITS_POOL }
}

/// The budget of one sub-tab for this character.
pub fn ability_budget(package: &SheetRulePackage, input: &CharacterInput, subtab: SubTab) -> AbilityBudget {
    match subtab {
        SubTab::Traits => trait_budget(package, input),
    }
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
        AbilityPool { subtab: SubTab::Traits, id: TRAITS_POOL.to_owned(), granted, spent: traits_held, sources },
        AbilityPool {
            subtab: SubTab::Traits,
            id: DRAWBACKS_POOL.to_owned(),
            granted: DRAWBACK_LIMIT,
            spent: drawbacks_held,
            sources: vec![(Some(1), format!("every character ({DRAWBACK_LIMIT})"))],
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
    }
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
            unmet.extend(verdicts(package, &held, &facts, &rule.applies).into_iter().filter_map(|verdict| match verdict {
                TermVerdict::Unmet(words) => Some(words),
                _ => None,
            }));
            AbilityOption {
                id,
                label: display_label(rule),
                pool: TRAITS_POOL.to_owned(),
                category: category.as_deref().map(|c| category_words(c).to_owned()),
                qualified: unmet.is_empty(),
                unmet,
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
}
