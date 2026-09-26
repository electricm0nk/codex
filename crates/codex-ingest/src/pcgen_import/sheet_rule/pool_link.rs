//! SD-36 F1c-3, defect D6: a player's pick into an ability pool, linked at ingest to the options
//! the pool offers.
//!
//! PCGen's `BONUS:ABILITYPOOL|<Category>|<n>` hands out `<n>` picks in a child
//! `ABILITYCATEGORY` whose members are every ability of its parent `CATEGORY:` carrying all of its
//! `TYPE:` tags (`cr_abilitycategories.lst:136`, `Simple Weapon Proficiency Choice` ->
//! `CATEGORY:Special Ability TYPE:SingleSimpleWeaponProficiency`). The converter writes the pick
//! as `target: Pool(<category slug>)` and, before this, no link from that pool to its members:
//! the member's converted `pool` is its parent category's slug, so no converted rule named the
//! pick's pool, and a reader could not see what the pick offers.
//!
//! One rule, no per-class case: a pick (non-zero `Pool` target, no `offers` of its own) whose
//! pool is a child category with a `TYPE:` filter, whose filter selects EXACTLY ONE converted
//! record, and whose one member offers a weapon choice resolved at ingest to oracle
//! weapon-proficiency names (`convert::weapon_choice_options`), gets
//! `offers: { id: <pick>, count: <pick value>, from: Rules { pool: <parent slug>, tags: <TYPE
//! tags> } }` -- the same `OptionSet::Rules` shape `CHOOSE:ABILITYSELECTION` already converts
//! to. The pick is then a weapon choice with named options, not an unlinked pool.
//!
//! Pools with several members (a bonus-feat list where one feat grants a proficiency), pools
//! whose member's weapon options depend on the character (`PC`, a deity's weapon) and every
//! non-weapon pool are left as they were, and named by mechanism in the F1c-3 receipt.

use std::collections::BTreeMap;

use codex::rules_core::sheet_rule::{Applies, Choice, Expr, OptionSet, SheetRule, SheetValue};

use super::closure::PinnedTree;
use super::ctx::slug;
use super::weapon_membership::WeaponMembershipIndex;

/// The child-category views the tree declares: pool slug -> (parent pool slug, `TYPE:` tags).
/// Two categories whose names slug alike are left out (never guessed).
pub fn category_views(tree: &PinnedTree) -> BTreeMap<String, (String, Vec<String>)> {
    let mut out: BTreeMap<String, Option<(String, Vec<String>)>> = BTreeMap::new();
    for (name, tags) in &tree.ability_category_type {
        let Some(parent) = tree.ability_category_parent.get(name) else { continue };
        let view = (slug(parent), tags.clone());
        out.entry(slug(name)).and_modify(|v| {
            if v.as_ref() != Some(&view) {
                *v = None;
            }
        }).or_insert(Some(view));
    }
    out.into_iter().filter_map(|(k, v)| v.map(|v| (k, v))).collect()
}

/// Whether `rule` carries every tag in `tags` (case-insensitive, as PCGen matches `TYPE`).
fn carries_all(rule: &SheetRule, tags: &[String]) -> bool {
    tags.iter().all(|t| rule.tags.iter().any(|o| o.eq_ignore_ascii_case(t)))
}

/// A rule that offers a weapon choice resolved to oracle weapon-proficiency names.
pub fn offers_resolved_weapons(rule: &SheetRule, membership: &WeaponMembershipIndex) -> bool {
    match rule.offers.as_ref().map(|o| &o.from) {
        Some(OptionSet::Weapons(names)) => !names.is_empty() && names.iter().all(|n| membership.weapon_named(n).is_some()),
        _ => false,
    }
}

/// Link every qualifying pick in `files` (see the module doc). Returns the ids of the picks
/// linked.
pub fn link_weapon_choice_pools(
    views: &BTreeMap<String, (String, Vec<String>)>,
    membership: &WeaponMembershipIndex,
    files: &mut BTreeMap<String, Vec<SheetRule>>,
) -> Vec<String> {
    // Principal rules by their own pool.
    let mut by_pool: BTreeMap<&str, Vec<&SheetRule>> = BTreeMap::new();
    for rules in files.values() {
        for rule in rules.iter().filter(|r| !r.id.contains('#')) {
            by_pool.entry(rule.pool.as_str()).or_default().push(rule);
        }
    }
    let mut links: BTreeMap<String, Choice> = BTreeMap::new();
    for rules in files.values() {
        for pick in rules {
            let Some(codex::rules_core::sheet_rule::BonusTarget::Pool(pool)) = &pick.target else { continue };
            let SheetValue::Number(count) = &pick.value else { continue };
            if matches!(count, Expr::Const(0)) || pick.offers.is_some() || by_pool.contains_key(pool.as_str()) {
                continue;
            }
            let Some((parent, tags)) = views.get(pool) else { continue };
            let members: Vec<&&SheetRule> =
                by_pool.get(parent.as_str()).into_iter().flatten().filter(|m| carries_all(m, tags)).collect();
            let [member] = members.as_slice() else { continue };
            if !offers_resolved_weapons(member, membership) {
                continue;
            }
            links.insert(
                pick.id.clone(),
                Choice {
                    id: pick.id.clone(),
                    count: count.clone(),
                    from: OptionSet::Rules { pool: parent.clone(), tags: tags.clone(), requires: Applies::Always },
                },
            );
        }
    }
    for rules in files.values_mut() {
        for rule in rules.iter_mut() {
            if let Some(choice) = links.get(&rule.id) {
                rule.offers = Some(choice.clone());
            }
        }
    }
    links.into_keys().collect()
}

/// The `BonusTarget::Other` word `convert.rs` writes for `BONUS:DOMAIN|NUMBER|<n>`.
pub const DOMAIN_COUNT_TARGET: &str = "domains";

/// What [`link_pool_choices`] linked, by the oracle token that hands out the pick.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PoolChoices {
    /// `BONUS:ABILITYPOOL|<C>|<n>` picks now offering the members of `<C>`.
    pub ability_pools: Vec<String>,
    /// `BONUS:DOMAIN|NUMBER|<n>` lines now offering the domains.
    pub domain_counts: Vec<String>,
}

/// SD-36 F4pre (FS-21): a selection pool the oracle fills becomes a converted CHOICE.
///
/// The oracle hands out a pick among a category's objects by two tokens:
///
/// - `BONUS:ABILITYPOOL|<C>|<n>` -- `<n>` picks in the ability category `<C>`, whose members are
///   every ability of its parent `CATEGORY:` carrying all of its `TYPE:` tags
///   (`acg_abilities_class.lst:1386`, `Shaman ~ Spirit`: `BONUS:ABILITYPOOL|Shaman Spirit|1`;
///   `acg_abilitycategories.lst:100`, `Shaman Spirit` = `CATEGORY:Special Ability
///   TYPE:ShamanSpirit`);
/// - `BONUS:DOMAIN|NUMBER|<n>` -- `<n>` picks among the domains (`cr_classes.lst:55`,
///   `CLASS:Cleric ... BONUS:DOMAIN|NUMBER|ClericDomainCount`, `BONUS:VAR|ClericDomainCount|2`).
///
/// ONE rule over both, D6's shape without its single-member-weapon restriction: every such pick
/// (a non-zero count, no `offers` of its own) gets `offers: { id: <pick>, count: <pick value>,
/// from: <the category's members> }` -- `Rules { pool: <parent slug>, tags: <TYPE tags> }` for an
/// ability category with a `TYPE:` filter that selects at least one converted record,
/// `Domains` for the domain count. The options are the member records themselves, which carry
/// their own grants (the Battle spirit's `ABILITY:` edges, the Air domain's `Core Domain ~ Air
/// Domain`), so the engine holds a recorded option under a held choice
/// (`sheet_rule::held_set`) and each member line at the level the record states.
///
/// No member edge is written: the option set names the members, and a category the size of the
/// feat list would otherwise carry one `Granter::Choice` edge per member per pick (measured in
/// the F4pre receipt). Left out, as before: a pool whose name is itself a parent category, a
/// category with no `TYPE:` view (or two that slug alike), a filter selecting no converted
/// record, a negative count (a record spending a pick it fills itself).
pub fn link_pool_choices(views: &BTreeMap<String, (String, Vec<String>)>, files: &mut BTreeMap<String, Vec<SheetRule>>) -> PoolChoices {
    let mut by_pool: BTreeMap<&str, Vec<&SheetRule>> = BTreeMap::new();
    for rules in files.values() {
        for rule in rules.iter().filter(|r| !r.id.contains('#')) {
            by_pool.entry(rule.pool.as_str()).or_default().push(rule);
        }
    }
    let mut out = PoolChoices::default();
    let mut links: BTreeMap<String, Choice> = BTreeMap::new();
    for rules in files.values() {
        for pick in rules {
            if pick.offers.is_some() {
                continue;
            }
            let SheetValue::Number(count) = &pick.value else { continue };
            if matches!(count, Expr::Const(n) if *n <= 0) {
                continue;
            }
            let from = match &pick.target {
                Some(codex::rules_core::sheet_rule::BonusTarget::Pool(pool)) => {
                    if by_pool.contains_key(pool.as_str()) {
                        continue;
                    }
                    let Some((parent, tags)) = views.get(pool) else { continue };
                    if !by_pool.get(parent.as_str()).into_iter().flatten().any(|m| carries_all(m, tags)) {
                        continue;
                    }
                    out.ability_pools.push(pick.id.clone());
                    OptionSet::Rules { pool: parent.clone(), tags: tags.clone(), requires: Applies::Always }
                }
                Some(codex::rules_core::sheet_rule::BonusTarget::Other(t)) if t == DOMAIN_COUNT_TARGET => {
                    out.domain_counts.push(pick.id.clone());
                    OptionSet::Domains
                }
                _ => continue,
            };
            links.insert(pick.id.clone(), Choice { id: pick.id.clone(), count: count.clone(), from });
        }
    }
    for rules in files.values_mut() {
        for rule in rules.iter_mut() {
            if let Some(choice) = links.get(&rule.id) {
                rule.offers = Some(choice.clone());
            }
        }
    }
    out.ability_pools.sort();
    out.domain_counts.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::sheet_rule::{BonusTarget, Subject};

    fn rule(id: &str, pool: &str, tags: &[&str], value: SheetValue, target: Option<BonusTarget>) -> SheetRule {
        SheetRule {
            id: id.into(),
            label: id.into(),
            value,
            also: Vec::new(),
            prose: Vec::new(),
            applies: Applies::Always,
            target,
            bonus_type: None,
            print: true,
            pool: pool.into(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
            subject: Subject::Character,
            repeatable: false,
            granted_by: Vec::new(),
            offers: None,
            grants: Vec::new(),
            closure_complete: false,
            always_held: false,
            provenance: Default::default(),
        }
    }

    /// The F4pre rule over a toy package: an `ABILITYPOOL` pick into a child category with a
    /// member offers `Rules`; a domain count offers `Domains`; a pick with no member, a zero or
    /// negative count, a pick that already offers, and a pool named after a parent category do
    /// not.
    #[test]
    fn ability_pool_picks_and_domain_counts_offer_their_members() {
        let n = |v: i32| SheetValue::Number(Expr::Const(v));
        let pool = |p: &str| Some(BonusTarget::Pool(p.into()));
        let mut files: BTreeMap<String, Vec<SheetRule>> = BTreeMap::new();
        let mut put = |r: SheetRule| {
            files.insert(r.id.clone(), vec![r]);
        };
        put(rule("b:class_feature:spirit_pick", "special_ability", &[], n(1), pool("fx_spirit")));
        put(rule("b:class_feature:empty_pick", "special_ability", &[], n(1), pool("fx_empty")));
        put(rule("b:class_feature:zero_pick", "special_ability", &[], n(0), pool("fx_spirit")));
        put(rule("b:class_feature:spend_pick", "special_ability", &[], n(-1), pool("fx_spirit")));
        put(rule("b:class_feature:parent_pick", "special_ability", &[], n(1), pool("special_ability")));
        let mut offering = rule("b:class_feature:offering_pick", "special_ability", &[], n(1), pool("fx_spirit"));
        offering.offers = Some(Choice { id: offering.id.clone(), count: Expr::Const(1), from: OptionSet::FreeText });
        put(offering);
        put(rule("b:class_feature:fx_spirit_battle", "special_ability", &["FxSpirit", "SpecialQuality"], SheetValue::Text, None));
        put(rule("b:class:fx#bonus1", "", &[], n(2), Some(BonusTarget::Other(DOMAIN_COUNT_TARGET.into()))));
        let views: BTreeMap<String, (String, Vec<String>)> = [
            ("fx_spirit".to_string(), ("special_ability".to_string(), vec!["fxspirit".to_string()])),
            ("fx_empty".to_string(), ("special_ability".to_string(), vec!["NoSuchTag".to_string()])),
        ]
        .into_iter()
        .collect();
        let out = link_pool_choices(&views, &mut files);
        assert_eq!(out.ability_pools, vec!["b:class_feature:spirit_pick".to_string()]);
        assert_eq!(out.domain_counts, vec!["b:class:fx#bonus1".to_string()]);
        let offers = |id: &str| files[id][0].offers.clone();
        assert_eq!(
            offers("b:class_feature:spirit_pick"),
            Some(Choice {
                id: "b:class_feature:spirit_pick".into(),
                count: Expr::Const(1),
                from: OptionSet::Rules { pool: "special_ability".into(), tags: vec!["fxspirit".into()], requires: Applies::Always },
            })
        );
        assert_eq!(offers("b:class:fx#bonus1").map(|c| c.from), Some(OptionSet::Domains));
        for id in ["b:class_feature:empty_pick", "b:class_feature:zero_pick", "b:class_feature:spend_pick", "b:class_feature:parent_pick"] {
            assert_eq!(offers(id), None, "{id}");
        }
        assert_eq!(offers("b:class_feature:offering_pick").map(|c| c.from), Some(OptionSet::FreeText));
    }
}
