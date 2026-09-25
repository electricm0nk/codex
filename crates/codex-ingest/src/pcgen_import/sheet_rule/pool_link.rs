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
