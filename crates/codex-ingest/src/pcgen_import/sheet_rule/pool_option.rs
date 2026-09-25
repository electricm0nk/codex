//! SD-36 Epic F3c4b: an ability-category pick row, converted as an OPTION of the choice that
//! picks it.
//!
//! # The oracle
//!
//! - `docs/listfilepages/globalfilestagpages/globalfileschoose.html` (pinned oracle checkout),
//!   `CHOOSE:ABILITYSELECTION|<category>|<criteria>`: "a list of abilities, resolved down to the
//!   internal ability selections, matching the stipulated criteria". A criterion is an ability
//!   name or key, `TYPE=<t>` (dot-joined types all required), `!TYPE=<t>`, or a qualifier `ANY`
//!   (the default), `ALL`, `QUALIFIED`, `PC`, optionally negated (`!PC`) and optionally wrapping
//!   criteria (`QUALIFIED[TYPE=x]`); "Criteria can be logically combined by using the comma (,),
//!   logical AND, and the pipe (|), logical OR. Logical ANDs are evaluated before logical ORs."
//!   `CHOOSE:ABILITY|<category>|...` reads the same (`plugin/lsttokens/choose/AbilityToken.java`
//!   and `AbilitySelectionToken.java` share the ability-category primitive grammar).
//! - `docs/listfilepages/globalfilestagpages/globalfilesother.html`, `ABILITY:`: the example
//!   `ABILITY:Special Ability|AUTOMATIC|%LIST <tab> CHOOSE:ABILITYSELECTION|Special Ability|
//!   TYPE=CuteBunnies` "would grant automatically, a Choice of type CuteBunnies": the record's own
//!   `ABILITY:<category>|<nature>|%LIST` applies the picked row to the character. The picked row
//!   is then an ability the character holds, so every token on it applies -- its `BONUS:VAR`
//!   raises its variables, its `ABILITY:` grants hand out their targets, its `PRE*` gates it.
//! - `pcgen/core/AbilityCategory.java`: an ability category is the pool the pick draws from; the
//!   members are every ability whose `CATEGORY:` is that category.
//!
//! # The rule (one mechanism, no per-pool case)
//!
//! A **pick chooser** is an inventory record whose own closure carries BOTH
//! `CHOOSE:[NUMCHOICES=n|]ABILITYSELECTION|<C>|<criteria>` (or `CHOOSE:...ABILITY|<C>|...`) AND
//! `ABILITY:<C>|<nature>|%LIST` -- a pool pick that applies the picked row. The converter already
//! writes its choice as `offers: Rules { pool: slug(<C>) }` under the record's own id.
//!
//! A **pick row** of that chooser is an oracle ability row (ability file family, outside `_pfs/`,
//! a plain or `.COPY=` row) whose `CATEGORY:` is `<C>` and which the chooser's criteria select,
//! that NO inventory unit stands for: no unit owns the row, and no unit answers its
//! `(<C>, KEY)` / `(<C>, name)` pair. (A row an inventory unit already stands for is untouched:
//! it converted as a record.)
//!
//! Each pick row converts, through the SAME `convert_record` every record goes through, as ONE
//! `pool_option` rule `<book>:pool_option:<pool>_<name slug>` -- a sheet rule, never an inventory
//! unit (the 49,450-unit inventory does not move). So its tokens take the shapes the converter
//! already emits: `BONUS:VAR` becomes `_vars/` contributions from the option, `ABILITY:` grant
//! edges `Granter::Rule(<option>)`, `PRE*` the option's gate, `DEFINE` a declaration. It is
//! granted by `Granter::Choice(<chooser>)` for every pick chooser whose criteria select it --
//! the edge `subclass.rs` and `pool_pick.rs` already write -- so it is held exactly when the
//! character's recorded choice names it.
//!
//! The pick row's `(<C>, KEY)` and `(<C>, name)` pairs are registered in the index BEFORE any
//! record converts (only where no unit answers them), so a record naming the row
//! (`ABILITY:Sorcerer Bloodline|AUTOMATIC|Draconic Bloodline`, `cr_abilities_class.lst:2976`)
//! resolves to the option instead of an unresolved reference.
//!
//! Named, never guessed:
//! - a criterion this reader does not read (`pool-option-criterion-unread`): that alternative
//!   selects nothing;
//! - two pick rows of one pool whose ids collide (`pool-option-id-collision`): the later is
//!   named;
//! - a pick row whose name is product identity, or whose conversion refuses
//!   (`pool-option-unconverted`);
//! - a chooser whose converted rule does not offer the pool under its own id
//!   (`pool-option-chooser-unconverted`): its grant edges are dropped, never pointed at nothing.

use std::collections::{BTreeMap, BTreeSet};

use super::closure::{row_identity, tokenize_row, Closure, FileFamily, PinnedTree, RowRef, RowShape};
use super::convert::{convert_record, Converted};
use super::ctx::{slug, split_gates, CorpusIndex, RecordRef};
use super::prose::pi_hit;
use codex::rules_core::sheet_rule::*;

/// The kind directory (and rule-id kind) a converted pick row is written under.
pub const POOL_OPTION_KIND: &str = "pool_option";

/// One criterion primitive (an AND term).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// `ANY` / `ALL` / `QUALIFIED` / `PC` / `!PC`: no membership constraint (`QUALIFIED` is the
    /// row's own gate, which the option carries; `PC` / `!PC` are held-state at pick time).
    Any,
    /// `TYPE=a.b`: the row carries every type.
    Types(Vec<String>),
    /// `!TYPE=a.b`: the row carries not all of them.
    NotTypes(Vec<String>),
    /// A literal ability name or key.
    Named(String),
}

/// A chooser's criteria: OR over alternatives, each an AND of terms.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Criteria {
    pub alternatives: Vec<Vec<Term>>,
    /// Alternatives with a primitive this reader does not read (verbatim).
    pub unread: Vec<String>,
}

/// A pick chooser (module doc).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickChooser {
    /// The inventory record (its converted principal offers the choice).
    pub record: RuleId,
    /// The category picked from (upper).
    pub category: String,
    /// `slug(<category>)`, the pool the converted choice offers.
    pub pool: String,
    pub criteria: Criteria,
    /// The `CHOOSE:` row, cited.
    pub cite: String,
}

/// One pick row, with the choosers that select it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolOptionDecl {
    pub id: RuleId,
    pub row: RowRef,
    pub book: String,
    pub rel_path: String,
    pub category: String,
    pub key: String,
    pub name: String,
    pub type_facet: String,
    pub pool: String,
    pub choosers: Vec<RuleId>,
}

/// Everything the declaration pass found.
#[derive(Debug, Clone, Default)]
pub struct PoolOptionScan {
    pub choosers: Vec<PickChooser>,
    pub options: Vec<PoolOptionDecl>,
    /// Defect rows by `_defects/<kind>.json`.
    pub defects: BTreeMap<String, Vec<String>>,
}

fn parse_term(raw: &str) -> Option<Term> {
    let t = raw.trim();
    let upper = t.to_ascii_uppercase();
    if matches!(upper.as_str(), "ANY" | "ALL" | "QUALIFIED" | "PC" | "!PC") {
        return Some(Term::Any);
    }
    let types = |s: &str| s.split('.').map(|x| x.trim().to_ascii_uppercase()).filter(|x| !x.is_empty()).collect::<Vec<_>>();
    if let Some(rest) = upper.strip_prefix("!TYPE=").or_else(|| upper.strip_prefix("!TYPE.")) {
        return Some(Term::NotTypes(types(rest)));
    }
    if let Some(rest) = upper.strip_prefix("TYPE=").or_else(|| upper.strip_prefix("TYPE.")) {
        return Some(Term::Types(types(rest)));
    }
    if t.contains('[') || t.contains(']') || t.contains('=') || t.contains('%') || t.is_empty() {
        return None;
    }
    Some(Term::Named(upper))
}

/// Split one `|`-alternative on top-level commas (a `[...]` wrapper keeps its own commas).
fn and_parts(alt: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for ch in alt.chars() {
        match ch {
            '[' => {
                depth += 1;
                cur.push(ch);
            }
            ']' => {
                depth -= 1;
                cur.push(ch);
            }
            ',' if depth == 0 => out.push(std::mem::take(&mut cur)),
            _ => cur.push(ch),
        }
    }
    out.push(cur);
    out
}

/// Parse the criteria after the category (`args` are the `|` fields).
pub fn parse_criteria(args: &[String]) -> Criteria {
    let mut c = Criteria::default();
    let args: Vec<&String> = args.iter().filter(|a| !a.trim().is_empty()).collect();
    if args.is_empty() {
        // `ANY` is the default qualifier.
        c.alternatives.push(vec![Term::Any]);
        return c;
    }
    for alt in args {
        let mut terms = Vec::new();
        let mut ok = true;
        for part in and_parts(alt) {
            let p = part.trim();
            // `Q[inner]`: the qualifier AND its inner criteria.
            if let Some(open) = p.find('[')
                && p.ends_with(']')
            {
                let q = &p[..open];
                let inner = &p[open + 1..p.len() - 1];
                match parse_term(q) {
                    Some(Term::Any) => {}
                    _ => {
                        ok = false;
                        break;
                    }
                }
                for ip in and_parts(inner) {
                    match parse_term(&ip) {
                        Some(t) => terms.push(t),
                        None => ok = false,
                    }
                }
                continue;
            }
            match parse_term(p) {
                Some(t) => terms.push(t),
                None => ok = false,
            }
        }
        if ok {
            c.alternatives.push(terms);
        } else {
            c.unread.push(alt.to_string());
        }
    }
    c
}

impl Criteria {
    /// Whether the criteria select a row with this key, name and types (upper).
    pub fn selects(&self, key: &str, name: &str, types: &BTreeSet<String>) -> bool {
        self.alternatives.iter().any(|alt| {
            alt.iter().all(|t| match t {
                Term::Any => true,
                Term::Types(ts) => ts.iter().all(|x| types.contains(x)),
                Term::NotTypes(ts) => !ts.iter().all(|x| types.contains(x)),
                Term::Named(n) => n == key || n == name,
            })
        })
    }
}

/// Every pick chooser among the index's records (module doc).
pub fn pick_choosers(tree: &PinnedTree, records: &[RecordRef], closures: &[Closure], defects: &mut BTreeMap<String, Vec<String>>) -> Vec<PickChooser> {
    let mut out = Vec::new();
    for (r, closure) in records.iter().zip(closures.iter()) {
        if !r.joined || r.rel_path.is_empty() {
            continue;
        }
        let mut choose: Option<(String, Vec<String>, String)> = None;
        let mut applies: BTreeSet<String> = BTreeSet::new();
        for row in &closure.rows {
            for (k, v) in &row.tokens {
                match k.as_str() {
                    "CHOOSE" if choose.is_none() => {
                        let (fields, _) = split_gates(v);
                        let mut fields: Vec<String> = fields.into_iter().map(|f| f.trim().to_string()).collect();
                        if fields.first().is_some_and(|f| f.to_ascii_uppercase().starts_with("NUMCHOICES=")) {
                            fields.remove(0);
                        }
                        let sel = fields.first().map(|s| s.to_ascii_uppercase()).unwrap_or_default();
                        if (sel == "ABILITYSELECTION" || sel == "ABILITY") && fields.len() >= 2 {
                            choose = Some((fields[1].to_ascii_uppercase(), fields[2..].to_vec(), row.cite.clone()));
                        }
                    }
                    "ABILITY" => {
                        let (fields, _) = split_gates(v);
                        if fields.len() >= 3 && fields.iter().skip(2).any(|t| t.trim() == "%LIST") {
                            applies.insert(fields[0].trim().to_ascii_uppercase());
                        }
                    }
                    _ => {}
                }
            }
        }
        let Some((category, args, cite)) = choose else { continue };
        if !applies.contains(&category) {
            continue;
        }
        let criteria = parse_criteria(&args);
        for u in &criteria.unread {
            defects.entry("pool-option-criterion-unread".into()).or_default().push(format!("{}: {u} ({cite})", r.id));
        }
        let _ = tree;
        out.push(PickChooser { record: r.id.clone(), pool: slug(&category), category, criteria, cite });
    }
    out
}

pub fn option_id(book: &str, pool: &str, name: &str) -> RuleId {
    format!("{book}:{POOL_OPTION_KIND}:{pool}_{}", slug(name))
}

/// Find every pick row (module doc). `answered(cat, key_or_name)` says whether an inventory unit
/// already answers the pair; `owned(row)` whether a unit owns the row.
pub fn scan(tree: &PinnedTree, records: &[RecordRef], closures: &[Closure], owned: &dyn Fn(RowRef) -> bool, answered: &dyn Fn(&str, &str) -> bool) -> PoolOptionScan {
    let mut s = PoolOptionScan::default();
    s.choosers = pick_choosers(tree, records, closures, &mut s.defects);
    let categories: BTreeSet<&str> = s.choosers.iter().map(|c| c.category.as_str()).collect();
    let mut seen_ids: BTreeMap<RuleId, String> = BTreeMap::new();
    for (fi, file) in tree.files.iter().enumerate() {
        if file.family != FileFamily::Ability || file.is_pfs {
            continue;
        }
        for (li, raw) in file.lines.iter().enumerate() {
            let row = RowRef { file: fi, line: li + 1 };
            let ident = row_identity(raw);
            if !matches!(ident.shape, RowShape::Plain | RowShape::Copy(_)) || !categories.contains(ident.category.as_str()) {
                continue;
            }
            let (head, tokens) = tokenize_row(raw);
            // A category declaration (`ABILITYCATEGORY:<name> ... CATEGORY:<parent>`) is not an
            // ability row.
            if head.trim_start().to_ascii_uppercase().starts_with("ABILITYCATEGORY:") {
                continue;
            }
            let name = head.split(".COPY=").next().unwrap_or(&head).trim().to_string();
            let name_u = name.to_ascii_uppercase();
            if owned(row) || answered(&ident.category, &ident.key) || answered(&ident.category, &name_u) {
                continue;
            }
            let type_facet = tokens.iter().filter(|(k, _)| k == "TYPE").map(|(_, v)| v.clone()).next_back().unwrap_or_default();
            let types: BTreeSet<String> = type_facet.split('.').map(|t| t.trim().to_ascii_uppercase()).filter(|t| !t.is_empty()).collect();
            let choosers: Vec<RuleId> = s
                .choosers
                .iter()
                .filter(|c| c.category == ident.category && c.criteria.selects(&ident.key, &name_u, &types))
                .map(|c| c.record.clone())
                .collect();
            if choosers.is_empty() {
                continue;
            }
            let pool = slug(&ident.category);
            let id = option_id(&file.book, &pool, &name);
            if let Some(first) = seen_ids.get(&id) {
                s.defects.entry("pool-option-id-collision".into()).or_default().push(format!("{id}: {} (first {first})", tree.cite(row)));
                continue;
            }
            seen_ids.insert(id.clone(), tree.cite(row));
            s.options.push(PoolOptionDecl {
                id,
                row,
                book: file.book.clone(),
                rel_path: file.rel_path.clone(),
                category: ident.category.clone(),
                key: ident.key.clone(),
                name,
                type_facet,
                pool,
                choosers,
            });
        }
    }
    s
}

/// The record a pick row converts as.
pub fn record_of(d: &PoolOptionDecl, category_label: &str) -> RecordRef {
    RecordRef {
        id: d.id.clone(),
        book: d.book.clone(),
        kind: POOL_OPTION_KIND.into(),
        name: d.name.clone(),
        key: d.key.clone(),
        category: category_label.to_string(),
        type_facet: d.type_facet.clone(),
        rel_path: d.rel_path.clone(),
        line: d.row.line,
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

/// One converted pick row.
#[derive(Debug)]
pub struct PoolOption {
    pub rule: SheetRule,
    pub siblings: Vec<SheetRule>,
    pub grants_out: Vec<(RuleId, Grant)>,
    pub converted: Converted,
    /// The own rows of its closure (for `_vars` provenance).
    pub own_rows: BTreeSet<RowRef>,
}

/// Convert every declared pick row. `offered(chooser, pool)` says whether the chooser's converted
/// rule offers `Rules { pool }` under its own id.
pub fn convert_options(tree: &PinnedTree, index: &CorpusIndex, offered: &dyn Fn(&str, &str) -> bool) -> (Vec<PoolOption>, BTreeMap<String, Vec<String>>) {
    let mut out = Vec::new();
    let mut defects: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut bad_choosers: BTreeSet<RuleId> = BTreeSet::new();
    for d in &index.pool_options {
        let cite = tree.cite(d.row);
        if pi_hit(&d.name).is_some() {
            defects.entry("pool-option-unconverted".into()).or_default().push(format!("{}: name withheld (product identity) ({cite})", d.id));
            continue;
        }
        let raw_category = tokenize_row(tree.row_text(d.row)).1.into_iter().find(|(k, _)| k == "CATEGORY").map(|(_, v)| v.trim().to_string()).unwrap_or_else(|| d.category.clone());
        let rec = record_of(d, &raw_category);
        let copy_base = match row_identity(tree.row_text(d.row)).shape {
            RowShape::Copy(b) => Some(b),
            _ => None,
        };
        let closure = tree.closure(&d.rel_path, d.row.line, None, &raw_category, &d.key, copy_base.as_deref());
        let mut c = convert_record(tree, index, &rec, &closure);
        if !c.refusals.is_empty() {
            let r: Vec<String> = c.refusals.iter().cloned().collect();
            defects.entry("pool-option-unconverted".into()).or_default().push(format!("{}: refused {} ({cite})", d.id, r.join(", ")));
            continue;
        }
        let mut rules = std::mem::take(&mut c.rules);
        if rules.is_empty() {
            defects.entry("pool-option-unconverted".into()).or_default().push(format!("{}: no rule ({cite})", d.id));
            continue;
        }
        let mut principal = rules.remove(0);
        for ch in &d.choosers {
            if offered(ch, &d.pool) {
                principal.granted_by.push(Grant { by: Granter::Choice(ch.clone()), when: Applies::Always });
            } else {
                bad_choosers.insert(ch.clone());
            }
        }
        if principal.granted_by.is_empty() {
            defects.entry("pool-option-unconverted".into()).or_default().push(format!("{}: no converted chooser offers it ({cite})", d.id));
            continue;
        }
        let grants_out = std::mem::take(&mut c.grants_out);
        out.push(PoolOption { rule: principal, siblings: rules, grants_out, converted: c, own_rows: closure.own_rows.clone() });
    }
    for ch in bad_choosers {
        defects.entry("pool-option-chooser-unconverted".into()).or_default().push(ch);
    }
    (out, defects)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn criteria_read_the_oracle_grammar() {
        let types = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<BTreeSet<_>>();
        // `!PC,QUALIFIED[TYPE=SorcererBloodlineChoice]` (cr_abilities_class.lst:2329).
        let c = parse_criteria(&s(&["!PC,QUALIFIED[TYPE=SorcererBloodlineChoice]"]));
        assert!(c.unread.is_empty());
        assert!(c.selects("DRACONIC BLOODLINE", "DRACONIC BLOODLINE", &types(&["SORCERERBLOODLINECHOICE"])));
        assert!(!c.selects("X", "X", &types(&["BLOODRAGERBLOODLINECHOICE"])));
        // Named alternatives OR'd (`arg_abilities_race.lst` Trailblazer).
        let c = parse_criteria(&s(&["Heart of the Mountain", "Heart of the Sea"]));
        assert!(c.selects("HEART OF THE SEA", "HEART OF THE SEA", &types(&[])));
        assert!(!c.selects("HEART OF THE FIELDS", "HEART OF THE FIELDS", &types(&[])));
        // Default is ANY.
        assert!(parse_criteria(&[]).selects("A", "A", &types(&[])));
        // TYPE dot-join is AND; !TYPE negates.
        let c = parse_criteria(&s(&["TYPE=A.B"]));
        assert!(c.selects("K", "K", &types(&["A", "B"])) && !c.selects("K", "K", &types(&["A"])));
        let c = parse_criteria(&s(&["ANY,!TYPE=A"]));
        assert!(!c.selects("K", "K", &types(&["A"])) && c.selects("K", "K", &types(&["B"])));
        // An unread primitive is named, never guessed.
        let c = parse_criteria(&s(&["CATEGORY=FEAT|x"]));
        assert_eq!(c.unread.len(), 1);
    }
}
