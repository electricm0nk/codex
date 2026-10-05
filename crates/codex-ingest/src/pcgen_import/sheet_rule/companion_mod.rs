//! SD-37 E5.4: a Starfinder `COMPANIONMOD:` row, converted as a rule its follower's race holds.
//!
//! # The oracle
//!
//! - `docs/listfilepages/datafilestagpages/datafilescompanionmodifiers.html` (pinned oracle
//!   checkout): `FOLLOWER:x,x=y` opens a companion modifier, "True if the CompanionLevel variable
//!   is 5 or greater" (`FOLLOWER:CompanionLevel=5`) -- `x` a class name or a variable of the
//!   master; it applies to the followers of the row's `TYPE:` (the follower role). Every other
//!   token on the row is a global token applied to the follower.
//! - The master offers a follower of a role by `COMPANIONLIST:<role>|<race,...>` on an object
//!   it holds (`scr_abilities.lst:1369`, the mechanic's AI selection `Drone`:
//!   `FOLLOWERS:Drone|1 COMPANIONLIST:Drone|Drone`).
//! - The Starfinder drone: `FOLLOWER:DroneCompanionLVL=1 TYPE:Drone` (`scr_companionmods.lst:13`)
//!   grants the drone's special abilities (`ABILITY:Internal|AUTOMATIC|Drone Special
//!   Abilities`), its chassis / skill unit / feat / mod picks (`BONUS:ABILITYPOOL|...`) and its
//!   level variables from the master's (`BONUS:VAR|DroneMasterLVL|MASTERVAR("DroneCompanionLVL")`).
//!
//! # The rule
//!
//! Each `FOLLOWER:` row of a converted Starfinder book whose role some ability row offers
//! (`COMPANIONLIST:<role>|...`) converts, through the SAME `convert_record` every record goes
//! through, as ONE rule `<book>:companion_mod:<role slug>` -- a sheet rule, never an inventory
//! unit (the inventory's 8,582 units do not move). Every line it writes carries the row's own
//! gate, `MasterVar(<variable>) >= <n>`, and the rule is granted by each race the role's
//! `COMPANIONLIST` names (`Granter::Rule(<race>)`): a follower of that race holds it. Its
//! `BONUS:ABILITYPOOL` picks are scanned with the records' (`pool_option::scan`), so the
//! `CATEGORY:Internal` members they pick from become pool options.
//!
//! Named, never guessed (`_defects/companion-mod-*.json`):
//! - a `FOLLOWER:` row whose role no ability row offers (the `.pcc`'s own `COMPANIONLIST:SpyDrone|Drone`
//!   and `COMPANIONLIST:Follower|ANY` name roles no record of the converted books hands out);
//! - a row whose head does not read `FOLLOWER:<variable>=<integer>` with one variable that is
//!   not a class name (a class-level or several-name head is not read), or that states no `TYPE:`;
//! - a role no converted record offers a race for (`companion-mod-race-unresolved`).
//!
//! Pathfinder is unchanged: the scan runs for Starfinder only.

use std::collections::{BTreeMap, BTreeSet};

use super::closure::{tokenize_row, Closure, FileFamily, PinnedTree, RowRef};
use super::ctx::{slug, RecordRef};
use codex::rules_core::game_system::GameSystem;
use codex::rules_core::sheet_rule::*;

/// The kind directory (and rule-id kind) a converted companion modifier is written under.
pub const COMPANION_MOD_KIND: &str = "companion_mod";

/// One `FOLLOWER:` row of a converted book.
#[derive(Debug, Clone)]
pub struct CompanionModDecl {
    pub id: RuleId,
    pub row: RowRef,
    pub book: String,
    pub rel_path: String,
    /// The follower role (`TYPE:`), as written.
    pub role: String,
    /// The master's variable the row is keyed on (`DroneCompanionLVL`).
    pub master_var: String,
    /// The least value of `master_var` the modifier applies at.
    pub min: i32,
    pub closure: Closure,
}

impl CompanionModDecl {
    /// The record the row converts as (`convert_record`'s input).
    pub fn record(&self) -> RecordRef {
        RecordRef {
            id: self.id.clone(),
            book: self.book.clone(),
            kind: COMPANION_MOD_KIND.into(),
            name: format!("{} companion", self.role),
            key: format!("FOLLOWER:{}={}", self.master_var, self.min),
            category: String::new(),
            type_facet: self.role.clone(),
            rel_path: self.rel_path.clone(),
            line: self.row.line,
            shipped_tokens: None,
            prerequisites: Vec::new(),
            copy_base_key: None,
            license_pi: false,
            pi_fields: Vec::new(),
            description: None,
            class_name: None,
            class_selection_of: None,
            joined: true,
        }
    }

    /// The row's own gate: the master's variable is at least `min`.
    pub fn gate(&self) -> Applies {
        Applies::Compare {
            lhs: Expr::MasterVar(super::ctx::var_id(&self.master_var)),
            op: Cmp::Gte,
            rhs: Expr::Const(self.min),
        }
    }
}

/// The follower roles an ability row offers (`COMPANIONLIST:<role>|...`), upper-cased.
fn offered_roles(tree: &PinnedTree) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for file in tree.files.iter().filter(|f| f.family == FileFamily::Ability && !f.is_pfs) {
        for raw in &file.lines {
            if raw.trim_start().starts_with('#') {
                continue;
            }
            for (k, v) in tokenize_row(raw).1 {
                if k.trim() == "COMPANIONLIST"
                    && let Some(role) = v.split('|').next().map(str::trim).filter(|r| !r.is_empty())
                {
                    out.insert(role.to_ascii_uppercase());
                }
            }
        }
    }
    out
}

/// Every converted Starfinder book's `FOLLOWER:` rows (module doc), with the defects named.
pub fn scan(tree: &PinnedTree) -> (Vec<CompanionModDecl>, BTreeMap<String, Vec<String>>) {
    let mut out = Vec::new();
    let mut defects: BTreeMap<String, Vec<String>> = BTreeMap::new();
    if tree.system != GameSystem::Starfinder1e {
        return (out, defects);
    }
    let roles = offered_roles(tree);
    let mut seen: BTreeMap<RuleId, String> = BTreeMap::new();
    for (fi, file) in tree.files.iter().enumerate() {
        if file.family != FileFamily::Companion || file.is_pfs {
            continue;
        }
        for (li, raw) in file.lines.iter().enumerate() {
            let row = RowRef { file: fi, line: li + 1 };
            let trimmed = raw.trim_start();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let (head, tokens) = tokenize_row(raw);
            let Some(rest) = head.strip_prefix("FOLLOWER:") else { continue };
            let cite = tree.cite(row);
            let parsed = rest.split_once('=').and_then(|(var, n)| n.trim().parse::<i32>().ok().map(|n| (var.trim().to_string(), n)));
            let is_variable = |var: &str| !var.is_empty() && !var.contains(',') && !tree.class_rows.contains_key(&var.to_ascii_uppercase());
            let Some((master_var, min)) = parsed.filter(|(var, _)| is_variable(var)) else {
                defects.entry("companion-mod-unread".into()).or_default().push(format!("{cite}: head {head:?}"));
                continue;
            };
            let Some(role) = tokens.iter().find(|(k, _)| k.trim() == "TYPE").map(|(_, v)| v.trim().to_string()).filter(|r| !r.is_empty()) else {
                defects.entry("companion-mod-unread".into()).or_default().push(format!("{cite}: no TYPE"));
                continue;
            };
            if !roles.contains(&role.to_ascii_uppercase()) {
                defects.entry("companion-mod-role-unoffered".into()).or_default().push(format!("{cite}: role {role}"));
                continue;
            }
            let id = format!("{}:{COMPANION_MOD_KIND}:{}", file.book, slug(&role));
            if let Some(first) = seen.get(&id) {
                defects.entry("companion-mod-id-collision".into()).or_default().push(format!("{id}: {cite} (first {first})"));
                continue;
            }
            seen.insert(id.clone(), cite);
            let closure = tree.closure(&file.rel_path, row.line, None, "", "", None);
            out.push(CompanionModDecl {
                id,
                row,
                book: file.book.clone(),
                rel_path: file.rel_path.clone(),
                role,
                master_var,
                min,
                closure,
            });
        }
    }
    (out, defects)
}

/// role (upper) -> the races a converted record offers a follower of that role from: a rule
/// granting `Fact::CompanionSlots { role }` whose `offers` are `OptionSet::Races`.
pub fn role_races(files: &BTreeMap<String, Vec<SheetRule>>) -> BTreeMap<String, BTreeSet<RuleId>> {
    let mut out: BTreeMap<String, BTreeSet<RuleId>> = BTreeMap::new();
    for rule in files.values().flatten() {
        let Some(Choice { from: OptionSet::Races(races), .. }) = &rule.offers else { continue };
        for effect in &rule.grants {
            if let Effect::FactGrant(Fact::CompanionSlots { role, .. }) = effect {
                out.entry(role.to_ascii_uppercase()).or_default().extend(races.iter().cloned());
            }
        }
    }
    out
}
