//! SD-36 F1c-5, defect D8: a pick into an ability category whose pool a variable sizes.
//!
//! # The oracle rule
//!
//! PCGen sizes an `ABILITYCATEGORY` by its `POOL:` formula. Every `POOL:` in the pinned tree is
//! one variable (`apg_abilitycategories.lst:267`, `ABILITYCATEGORY:Summoner Class Selection ...
//! POOL:Pool_Summoner_Class_Selection CATEGORY:Class TYPE:Summoner Class Selection`). A record
//! that raises that variable (`apg_abilities_class.lst:739`, the Summoner ability's
//! `BONUS:VAR|Pool_Summoner_Class_Selection|1|TYPE=Base`) hands the player picks among the
//! category's members: every ability of the category's `CATEGORY:` carrying all of its `TYPE:`
//! tags (`:741` Summoner ~ Standard Class, `pu_abilities_class.lst:117` Summoner ~ Unchained
//! Class).
//!
//! # What the converter writes (one rule, no record named)
//!
//! A record whose own rows raise a pool variable (a `BONUS:VAR` amount that is not negative; a
//! negative amount spends a pick the record fills itself) offers the choice
//! `{ id: <record>, count: Var(<pool variable>), from: Rules { pool: <parent slug>, tags: <TYPE
//! tags> } }` -- the `OptionSet::Rules` shape `CHOOSE:ABILITYSELECTION` and D6 already write. The
//! count is the pool variable itself, folded over every held contribution with its own gate, as
//! PCGen sizes the pool. Each member the category selects gets the edge
//! `Granter::Choice(<record>)`, the edge `ABILITY:<category>|NORMAL|<key>` already writes: the
//! member is held when the character's recorded choice names it.
//!
//! Left out, and named by mechanism in the F1c-5 receipt, never guessed:
//! - a pool variable more than one category shares (a points budget spread over several
//!   categories, e.g. the race builder's `ARG_RaceBuilderPoints`);
//! - a category that selects by `ABILITYLIST:` or by no `TYPE:` (no member filter this module
//!   reads);
//! - a record that already offers another choice (one choice per rule).
//!
//! An oracle member row no converted record stands for is the named defect
//! `pool-member-unconverted`.

use std::collections::{BTreeMap, BTreeSet};

use codex::rules_core::sheet_rule::RuleId;

use super::closure::{row_identity, tokenize_row, FileFamily, PinnedTree, RowRef, RowShape};
use super::ctx::{resolve_rule_in_checked, slug, CorpusIndex, RuleLookup};

/// One ability category sized by a variable pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariablePool {
    /// The category's own name, as declared (upper).
    pub category: String,
    /// The `ABILITYCATEGORY:` row, cited.
    pub decl: String,
    /// The members' `CATEGORY:` (upper).
    pub parent: String,
    /// The members' `TYPE:` tags (all required).
    pub tags: Vec<String>,
    /// The converted pool id the members carry (`slug(parent)`).
    pub pool: String,
}

/// A variable pool's members: the converted records it selects, and every oracle member row no
/// converted record stands for.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PoolMembers {
    pub resolved: Vec<RuleId>,
    pub unresolved: Vec<String>,
}

/// Why a declared `POOL:` category is not linked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unlinked {
    /// The variable sizes several categories at once.
    SharedVariable { var: String, categories: Vec<String> },
    /// The category names no `TYPE:` member filter this module reads (an `ABILITYLIST:`, no
    /// `TYPE:`, or an ambiguous declaration).
    NoTypeFilter { category: String },
}

/// Every variable pool the tree declares, by pool variable (upper), and the declared `POOL:`
/// categories left unlinked, each with its mechanism.
pub fn variable_pools(tree: &PinnedTree) -> (BTreeMap<String, VariablePool>, Vec<Unlinked>) {
    let mut by_var: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (category, (var, _)) in &tree.ability_category_pool {
        by_var.entry(var.clone()).or_default().push(category.clone());
    }
    let mut out = BTreeMap::new();
    let mut unlinked = Vec::new();
    for (var, categories) in by_var {
        if categories.len() > 1 {
            unlinked.push(Unlinked::SharedVariable { var, categories });
            continue;
        }
        let category = categories.into_iter().next().unwrap_or_default();
        let (Some(parent), Some(tags)) = (tree.ability_category_parent.get(&category), tree.ability_category_type.get(&category)) else {
            unlinked.push(Unlinked::NoTypeFilter { category });
            continue;
        };
        let decl = tree.cite(tree.ability_category_pool[&category].1);
        out.insert(var, VariablePool { category: category.clone(), decl, parent: parent.clone(), tags: tags.clone(), pool: slug(parent) });
    }
    (out, unlinked)
}

fn carries_all(own: &[String], tags: &[String]) -> bool {
    tags.iter().all(|t| own.iter().any(|o| o.eq_ignore_ascii_case(t)))
}

/// The members of `pool`: every converted record whose accumulated `(CATEGORY, TYPE)`
/// (`CorpusIndex::facets`, the fold the converter applies) selects it -- one per PCGen ability,
/// the record a by-name grant of its key reaches -- and every oracle ability base row the category
/// selects (its own `TYPE:` plus its `.MOD` rows') that no selected record stands for: either no
/// record of the index owns the row, or its owner's accumulated facets disagree with the row
/// (both named, never guessed into a member). `ABILITYCATEGORY:` declaration files are not
/// ability rows.
pub fn pool_members(tree: &PinnedTree, index: &CorpusIndex, pool: &VariablePool) -> PoolMembers {
    let mut resolved: Vec<RuleId> = Vec::new();
    let mut idents: BTreeSet<String> = BTreeSet::new();
    let mut unresolved: Vec<String> = Vec::new();
    let selects = |id: &str| index.facets.get(id).is_some_and(|(cat, tags)| cat.eq_ignore_ascii_case(&pool.parent) && carries_all(tags, &pool.tags));
    for r in &index.records {
        if !selects(&r.id) {
            continue;
        }
        let ident = if r.key.trim().is_empty() { r.name.trim() } else { r.key.trim() }.to_ascii_uppercase();
        if !idents.insert(ident.clone()) {
            continue;
        }
        // The record a by-name grant of the key reaches, when that record is itself selected by
        // the category (a reprint of the same ability); otherwise this record. A by-name join that
        // lands on a record the category does not select (a Mythic feat sharing the key of a
        // Combat feat) is never taken.
        let id = match resolve_rule_in_checked(tree, index, &pool.parent, &ident) {
            RuleLookup::Found(id) if selects(&id) => id,
            _ => r.id.clone(),
        };
        if !resolved.contains(&id) {
            resolved.push(id);
        }
    }
    for (fi, file) in tree.files.iter().enumerate() {
        if file.is_pfs || file.family != FileFamily::Ability || file.rel_path.to_ascii_lowercase().contains("abilitycategor") {
            continue;
        }
        for (li, raw) in file.lines.iter().enumerate() {
            let row = RowRef { file: fi, line: li + 1 };
            let id = row_identity(raw);
            if id.shape != RowShape::Plain || !id.category.eq_ignore_ascii_case(&pool.parent) || idents.contains(&id.key) {
                continue;
            }
            let owner = index.row_owner.get(&row);
            if owner.is_some_and(|o| selects(o)) {
                continue;
            }
            let mut tags: Vec<String> = type_tags(raw);
            for m in tree.mod_index.get(&(FileFamily::Ability, id.category.clone(), id.key.clone())).into_iter().flatten() {
                tags.extend(type_tags(tree.row_text(*m)));
            }
            if !carries_all(&tags, &pool.tags) {
                continue;
            }
            match owner {
                None => unresolved.push(format!("{} ({}): no converted record stands for this member", id.key, tree.cite(row))),
                Some(o) => {
                    let tags = index.facets.get(o).map(|(_, t)| t.clone()).unwrap_or_default();
                    if tags.iter().any(|t| t.starts_with("[redacted")) {
                        unresolved.push(format!(
                            "{} ({}): its converted record {o} is product-identity redacted (its category and tags are withheld)",
                            id.key,
                            tree.cite(row)
                        ));
                    } else {
                        unresolved.push(format!(
                            "{} ({}): its converted record {o} is not in the pool (its tags: {})",
                            id.key,
                            tree.cite(row),
                            tags.join(", ")
                        ));
                    }
                }
            }
        }
    }
    unresolved.sort();
    unresolved.dedup();
    PoolMembers { resolved, unresolved }
}

fn type_tags(row: &str) -> Vec<String> {
    let (_, tokens) = tokenize_row(row);
    tokens
        .iter()
        .filter(|(k, _)| k == "TYPE")
        .flat_map(|(_, v)| v.split('.').map(str::trim).filter(|t| !t.is_empty() && !t.eq_ignore_ascii_case("CLEAR")).map(str::to_string).collect::<Vec<_>>())
        .collect()
}

/// Every variable pool a record of the index fills, with its members. Keyed by pool variable
/// (upper).
pub fn filled_pools(tree: &PinnedTree, index: &CorpusIndex) -> BTreeMap<String, (VariablePool, PoolMembers)> {
    let (pools, _) = variable_pools(tree);
    let filled: BTreeSet<&String> = index.own_var_contribs.values().flat_map(|m| m.keys()).collect();
    pools
        .into_iter()
        .filter(|(var, _)| filled.contains(var))
        .map(|(var, pool)| {
            let members = pool_members(tree, index, &pool);
            (var, (pool, members))
        })
        .collect()
}

/// The `pool-member-unconverted` defect rows: one per oracle member row of a filled pool that no
/// converted record stands for.
pub fn unconverted_member_defects(filled: &BTreeMap<String, (VariablePool, PoolMembers)>) -> Vec<String> {
    let mut out = Vec::new();
    for (pool, members) in filled.values() {
        for m in &members.unresolved {
            out.push(format!("pool:{} ({}): member {m}", slug(&pool.category), pool.decl));
        }
    }
    out
}

/// Whether a `BONUS:VAR` amount raises the pool (a negative literal spends a pick instead).
pub fn raises_the_pool(formula: &str) -> bool {
    !formula.trim_start().starts_with('-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_negative_amount_spends_a_pick_and_offers_none() {
        assert!(raises_the_pool("1"));
        assert!(raises_the_pool("MagusArcanaLVL/3"));
        assert!(raises_the_pool("(OccultistLVL+2)/4"));
        assert!(!raises_the_pool("-1"));
        assert!(!raises_the_pool(" -((OccultistLVL+2)/4)"));
    }
}
